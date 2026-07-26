// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use crate::address::{
    GetAddrRequestState, GetAddrResponseDecision, LearnedAddressBook, LearnedAddressDecision,
    LocalAdvertisementDecision,
};
use crate::block_serving::BlockRelayActivationPolicy;
use crate::compact_download::CompactDownloadPeerState;
use crate::error::{DisconnectReason, NetworkError, PeerId};
use crate::header_store::HeaderStore;
use crate::inbound::{InboundAdmissionRejectionReason, InboundHandshakeState, InboundPeerRecord};
use crate::message::{HeadersMessage, InventoryList, LocalPeerConfig, WireNetworkMessage};
use crate::peer_policy::{
    EvictionCandidateInput, EvictionDecision, MisbehaviorDecision, MisbehaviorKind,
    MisbehaviorObservation, MisbehaviorPolicy, PeerPolicyRuntimeState, select_eviction_candidate,
};
use crate::resource::InboundResourceEvent;
use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_consensus::{
    block_hash, build_compact_block_payload, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{
    Block, BlockHash, InventoryType, InventoryVector, Transaction, Txid, Wtxid,
};
use std::collections::{BTreeMap, BTreeSet};

mod address_boundary;
mod compact_download_state;
mod compact_relay;
mod inbound_state;
mod inventory_state;
mod message_dispatch;
mod policy_state;
mod relay_download;
mod transaction_relay;

pub use address_boundary::{PeerAddressBoundaryDecision, PeerAddressBoundaryEvidence};
pub use compact_download_state::CompactBlockReceiveFacts;
pub use compact_relay::{
    CompactAnnouncementAction, CompactAnnouncementDecision, CompactAnnouncementEligibility,
    CompactAnnouncementEligibilityReason, CompactAnnouncementInput, CompactAnnouncementProvenance,
    CompactAnnouncementReason, CompactBlockTransactionsRequest, CompactRelayCapability,
    CompactRelayNegotiationOutcome, CompactRelayNegotiationReason, CompactRelayPeerState,
    CompactRelayPreference, LocalCompactRelayOfferState, MAX_COMPACT_ANNOUNCEMENT_PROVENANCE,
    PeerCompactAnnouncementInput, decide_compact_announcement,
};
use policy_state::{eviction_candidate_input, peer_policy_label, peer_policy_protected};
pub use relay_download::RelayDownloadPolicy;
pub use transaction_relay::{
    HardRejectEvidence, OrphanAction, OrphanEvidenceLabel, OrphanPolicy,
    OrphanReconsiderationCandidate, OrphanReconsiderationStatus, OrphanStageInput,
    PHASE101_GETDATA_TX_INTERVAL_SECONDS, PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER,
    PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER, PHASE101_NONPREF_PEER_TX_DELAY_SECONDS,
    PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS, PHASE101_TXID_RELAY_DELAY_SECONDS,
    PHASE102_MAX_ORPHAN_TRANSACTIONS, PHASE102_MAX_ORPHANS_PER_PEER,
    PHASE102_MAX_RECONSIDERATIONS_PER_PARENT, PHASE102_ORPHAN_TTL_SECONDS,
    PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER, PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER,
    PHASE104_TX_FANOUT_MIN_INTERVAL_SECONDS, PHASE133_REJECT_FILTER_CAPACITY,
    PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE, ReconsiderableEvidenceKey,
    ReconsiderableRejectEvidence, RejectEvidenceConfigError, RejectEvidenceTweak,
    RetryDecisionContext, RetryJitterRangeError, RetryJitterSeconds, TxAnnouncementInput,
    TxDownloadAction, TxDownloadLocalFacts, TxDownloadPolicy, TxDownloadScheduler,
    TxDownloadSnapshot, TxDownloadSuppressionReason, TxFanoutAction, TxFanoutAdmission,
    TxFanoutAdmissionOutcome, TxFanoutCleanupReason, TxFanoutPeerInput, TxFanoutPolicy,
    TxFanoutQueue, TxFanoutSnapshot, TxFanoutSuppressionReason, TxOrphanage, TxParentRequestInput,
    TxPeerRequestSnapshot, TxRelayId, TxRelayIdentityError, TxRelayPeerMode, TxServeDecision,
    TxServeOutcomeLabel, TxServingRecordStatus, classify_tx_serve_request, defer_local_rebroadcast,
};
pub const DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionRole {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderSyncPolicy {
    HeadersOnly,
    HeadersAndBlocks,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerAction {
    Send(WireNetworkMessage),
    ServeInventory(Vec<InventoryVector>),
    TransactionRelay(TxDownloadAction),
    ReceivedTransaction(Transaction),
    ReceivedBlock(Block),
    ServeCompactBlockTransactions(CompactBlockTransactionsRequest),
    Disconnect(DisconnectReason),
    ResourceGovernanceDisconnect(InboundResourceEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerState {
    pub role: ConnectionRole,
    pub remote_start_height: i32,
    pub remote_services_bits: u64,
    pub remote_user_agent: String,
    pub remote_wtxidrelay: bool,
    pub remote_prefers_headers: bool,
    pub maybe_remote_protocol_version: Option<i32>,
    pub local_compact_relay_offer: LocalCompactRelayOfferState,
    pub compact_relay: CompactRelayPeerState,
    pub compact_announcements: CompactAnnouncementProvenance,
    pub remote_version_received: bool,
    pub remote_verack_received: bool,
    pub local_version_sent: bool,
    pub local_verack_sent: bool,
    pub sync_started: bool,
    pub getheaders_in_flight: bool,
    pub requested_blocks: BTreeSet<BlockHash>,
    pub last_ping_nonce: Option<u64>,
    pub getaddr_request_state: GetAddrRequestState,
    pub maybe_inbound_record: Option<InboundPeerRecord>,
    pub maybe_inbound_rejection_reason: Option<InboundAdmissionRejectionReason>,
}

impl PeerState {
    fn new(role: ConnectionRole) -> Self {
        Self {
            role,
            remote_start_height: -1,
            remote_services_bits: 0,
            remote_user_agent: String::new(),
            remote_wtxidrelay: false,
            remote_prefers_headers: false,
            maybe_remote_protocol_version: None,
            local_compact_relay_offer: LocalCompactRelayOfferState::default(),
            compact_relay: CompactRelayPeerState::default(),
            compact_announcements: CompactAnnouncementProvenance::default(),
            remote_version_received: false,
            remote_verack_received: false,
            local_version_sent: false,
            local_verack_sent: false,
            sync_started: false,
            getheaders_in_flight: false,
            requested_blocks: BTreeSet::new(),
            last_ping_nonce: None,
            getaddr_request_state: GetAddrRequestState::default(),
            maybe_inbound_record: None,
            maybe_inbound_rejection_reason: None,
        }
    }

    fn from_inbound_record(mut record: InboundPeerRecord) -> Self {
        record.handshake_state = InboundHandshakeState::Handshaking;
        let mut state = Self::new(ConnectionRole::Inbound);
        state.maybe_inbound_record = Some(record);
        state
    }

    fn handshake_established(&self) -> bool {
        self.remote_version_received && self.local_verack_sent && self.remote_verack_received
    }
}

#[derive(Debug, Clone)]
pub struct PeerManager {
    local_config: LocalPeerConfig,
    headers: HeaderStore,
    peers: BTreeMap<PeerId, PeerState>,
    known_blocks: BTreeSet<BlockHash>,
    known_txids: BTreeSet<Txid>,
    known_wtxids: BTreeSet<Wtxid>,
    known_wtxids_by_txid: BTreeMap<Txid, Wtxid>,
    tx_download: TxDownloadScheduler,
    hard_reject_evidence: HardRejectEvidence,
    reconsiderable_reject_evidence: ReconsiderableRejectEvidence,
    mempool_known: BTreeSet<TxRelayId>,
    relay_download_policy: RelayDownloadPolicy,
    max_blocks_in_flight_per_peer: usize,
    learned_addresses: LearnedAddressBook,
    local_address_decisions: Vec<LocalAdvertisementDecision>,
    getaddr_responses_served: Vec<GetAddrResponseDecision>,
    getaddr_requests_suppressed: Vec<GetAddrResponseDecision>,
    learned_address_rejections: Vec<LearnedAddressDecision>,
    learned_address_rejection_count: usize,
    maybe_latest_address_decision: Option<PeerAddressBoundaryDecision>,
    peer_policy_runtime_state: PeerPolicyRuntimeState,
    block_relay_activation: BlockRelayActivationPolicy,
    compact_download_states: BTreeMap<PeerId, CompactDownloadPeerState>,
}

impl PeerManager {
    fn peer_mut(
        peers: &mut BTreeMap<PeerId, PeerState>,
        peer_id: PeerId,
    ) -> Result<&mut PeerState, NetworkError> {
        peers
            .get_mut(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))
    }

    pub fn new(local_config: LocalPeerConfig) -> Self {
        Self::with_max_blocks_in_flight(local_config, DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER)
    }

    pub fn with_max_blocks_in_flight(
        local_config: LocalPeerConfig,
        max_blocks_in_flight_per_peer: usize,
    ) -> Self {
        Self {
            local_config,
            headers: HeaderStore::default(),
            peers: BTreeMap::new(),
            known_blocks: BTreeSet::new(),
            known_txids: BTreeSet::new(),
            known_wtxids: BTreeSet::new(),
            known_wtxids_by_txid: BTreeMap::new(),
            tx_download: TxDownloadScheduler::new(TxDownloadPolicy::default()),
            hard_reject_evidence: HardRejectEvidence::new(RejectEvidenceTweak::new(0)),
            reconsiderable_reject_evidence: ReconsiderableRejectEvidence::new(
                RejectEvidenceTweak::new(0),
            ),
            mempool_known: BTreeSet::new(),
            relay_download_policy: RelayDownloadPolicy::default(),
            max_blocks_in_flight_per_peer,
            learned_addresses: LearnedAddressBook::default(),
            local_address_decisions: Vec::new(),
            getaddr_responses_served: Vec::new(),
            getaddr_requests_suppressed: Vec::new(),
            learned_address_rejections: Vec::new(),
            learned_address_rejection_count: 0,
            maybe_latest_address_decision: None,
            peer_policy_runtime_state: PeerPolicyRuntimeState::default(),
            block_relay_activation: BlockRelayActivationPolicy::default(),
            compact_download_states: BTreeMap::new(),
        }
    }

    pub fn seed_local_chain(&mut self, active_chain: &[ChainPosition]) {
        self.headers.seed_from_chain(active_chain);
        self.known_blocks.clear();
        for position in active_chain {
            self.known_blocks.insert(position.block_hash);
        }
    }

    pub fn seed_header_store(&mut self, headers: HeaderStore) {
        self.headers = headers;
    }

    pub fn note_local_position(&mut self, position: &ChainPosition) {
        self.headers.record_position(position);
        self.known_blocks.insert(position.block_hash);
    }

    pub fn note_local_block_hash(&mut self, block_hash: BlockHash) {
        self.known_blocks.insert(block_hash);
    }

    pub fn note_local_transaction(
        &mut self,
        transaction: &Transaction,
    ) -> Result<(), NetworkError> {
        let txid = transaction_txid(transaction)?;
        let wtxid = transaction_wtxid(transaction)?;
        self.known_txids.insert(txid);
        self.known_wtxids.insert(wtxid);
        self.known_wtxids_by_txid.insert(txid, wtxid);
        Ok(())
    }

    pub fn record_hard_reject(&mut self, wtxid: Wtxid) {
        self.hard_reject_evidence.record(wtxid);
    }

    pub fn hard_reject_contains(&self, wtxid: Wtxid) -> bool {
        self.hard_reject_evidence.contains(wtxid)
    }

    pub fn record_reconsiderable_transaction(&mut self, wtxid: Wtxid) {
        self.reconsiderable_reject_evidence
            .record(ReconsiderableEvidenceKey::Transaction(wtxid));
    }

    pub fn reconsiderable_transaction_contains(&self, wtxid: Wtxid) -> bool {
        self.reconsiderable_reject_evidence
            .contains(ReconsiderableEvidenceKey::Transaction(wtxid))
    }

    pub fn record_reconsiderable_package(&mut self, fingerprint: [u8; 32]) {
        self.reconsiderable_reject_evidence
            .record(ReconsiderableEvidenceKey::Package(fingerprint));
    }

    pub fn reconsiderable_package_contains(&self, fingerprint: [u8; 32]) -> bool {
        self.reconsiderable_reject_evidence
            .contains(ReconsiderableEvidenceKey::Package(fingerprint))
    }

    pub fn header_store(&self) -> &HeaderStore {
        &self.headers
    }

    pub const fn max_blocks_in_flight_per_peer(&self) -> usize {
        self.max_blocks_in_flight_per_peer
    }

    pub fn peer_state(&self, peer_id: PeerId) -> Option<&PeerState> {
        self.peers.get(&peer_id)
    }

    pub fn maybe_schedule_local_compact_offer(
        &mut self,
        peer_id: PeerId,
    ) -> Result<Option<open_bitcoin_codec::SendCompactMessage>, NetworkError> {
        let activation = self.block_relay_activation;
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        let handshake_established = peer.handshake_established();
        Ok(compact_relay::maybe_schedule_local_compact_offer(
            &mut peer.local_compact_relay_offer,
            activation,
            handshake_established,
            peer.maybe_remote_protocol_version,
        ))
    }

    pub fn record_compact_block_announcement(
        &mut self,
        peer_id: PeerId,
        block_hash: BlockHash,
    ) -> Result<(), NetworkError> {
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        peer.compact_announcements.record(block_hash);
        Ok(())
    }

    pub fn decide_compact_announcement_for_peer(
        &mut self,
        peer_id: PeerId,
        input: PeerCompactAnnouncementInput,
    ) -> Result<CompactAnnouncementDecision, NetworkError> {
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        let decision = compact_relay::decide_compact_announcement(CompactAnnouncementInput {
            activation: input.activation,
            peer_state: peer.compact_relay,
            peer_prefers_headers: peer.remote_prefers_headers,
            peer_has_previous_header: input.peer_has_previous_header,
            peer_has_current_header: input.peer_has_current_header,
            status: input.status,
            resource_gate: input.resource_gate,
        });
        peer.compact_relay.record_announcement_decision(&decision);
        Ok(decision)
    }

    pub fn peer_ids(&self) -> BTreeSet<PeerId> {
        self.peers.keys().copied().collect()
    }

    pub fn identities(&self) -> BTreeSet<PeerId> {
        self.peer_ids()
    }

    pub fn eviction_candidate_inputs(&self) -> Vec<EvictionCandidateInput> {
        self.peers
            .iter()
            .filter(|(_peer_id, peer)| peer.role == ConnectionRole::Inbound)
            .map(|(peer_id, peer)| {
                eviction_candidate_input(
                    *peer_id,
                    peer,
                    self.tx_download.peer_snapshot(*peer_id).in_flight_count,
                )
            })
            .collect()
    }

    pub fn eviction_decision(&self) -> EvictionDecision {
        let inputs = self.eviction_candidate_inputs();
        select_eviction_candidate(&inputs)
    }

    pub fn peer_policy_runtime_state(&self) -> &PeerPolicyRuntimeState {
        &self.peer_policy_runtime_state
    }

    pub fn peer_policy_runtime_state_mut(&mut self) -> &mut PeerPolicyRuntimeState {
        &mut self.peer_policy_runtime_state
    }

    pub fn misbehavior_decision(
        &self,
        peer_id: PeerId,
        kind: MisbehaviorKind,
        points: u32,
        now_unix_seconds: i64,
    ) -> Result<MisbehaviorDecision, NetworkError> {
        let peer = self
            .peers
            .get(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;
        let protected = peer_policy_protected(peer);
        let observation = MisbehaviorObservation {
            peer_label: peer_policy_label(peer_id),
            kind,
            points,
            prior_score: 0,
            protected,
        };
        let _ = now_unix_seconds;
        Ok(MisbehaviorPolicy::default().decide(observation))
    }

    pub fn peer_requested_blocks(&self, peer_id: PeerId) -> Result<Vec<BlockHash>, NetworkError> {
        let peer = self
            .peers
            .get(&peer_id)
            .ok_or(NetworkError::UnknownPeer(peer_id))?;
        Ok(peer.requested_blocks.iter().copied().collect())
    }

    pub fn add_outbound_peer(
        &mut self,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        if self.peers.contains_key(&peer_id) {
            return Err(NetworkError::PeerAlreadyExists(peer_id));
        }
        let mut state = PeerState::new(ConnectionRole::Outbound);
        state.local_version_sent = true;
        self.peers.insert(peer_id, state);
        Ok(vec![PeerAction::Send(WireNetworkMessage::Version(
            self.local_config
                .version_message(timestamp, self.headers.best_height()),
        ))])
    }

    pub fn request_ping(
        &mut self,
        peer_id: PeerId,
        nonce: u64,
    ) -> Result<WireNetworkMessage, NetworkError> {
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        peer.last_ping_nonce = Some(nonce);
        Ok(WireNetworkMessage::Ping { nonce })
    }

    pub fn announce_block(
        &self,
        peer_id: PeerId,
        block: &Block,
    ) -> Result<Option<WireNetworkMessage>, NetworkError> {
        let Some(peer) = self.peers.get(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        let action = if peer.remote_prefers_headers {
            CompactAnnouncementAction::AnnounceHeaders
        } else {
            CompactAnnouncementAction::AnnounceInventory
        };
        self.announce_block_with_action(peer_id, block, action, 0)
    }

    pub fn announce_block_with_action(
        &self,
        peer_id: PeerId,
        block: &Block,
        action: CompactAnnouncementAction,
        compact_nonce: u64,
    ) -> Result<Option<WireNetworkMessage>, NetworkError> {
        let Some(peer) = self.peers.get(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };

        match action {
            CompactAnnouncementAction::Suppress => Ok(None),
            CompactAnnouncementAction::AnnounceHeaders => {
                Ok(Some(WireNetworkMessage::Headers(HeadersMessage {
                    headers: vec![block.header.clone()],
                })))
            }
            CompactAnnouncementAction::AnnounceInventory => {
                let block_hash = block_hash(&block.header);
                Ok(Some(WireNetworkMessage::Inv(InventoryList::new(vec![
                    InventoryVector {
                        inventory_type: InventoryType::Block,
                        object_hash: block_hash.into(),
                    },
                ]))))
            }
            CompactAnnouncementAction::AnnounceCompactBlock => {
                match build_compact_block_payload(block, compact_nonce) {
                    Ok(payload) => Ok(Some(WireNetworkMessage::CompactBlock(payload))),
                    Err(_) => {
                        if peer.remote_prefers_headers {
                            Ok(Some(WireNetworkMessage::Headers(HeadersMessage {
                                headers: vec![block.header.clone()],
                            })))
                        } else {
                            let block_hash = block_hash(&block.header);
                            Ok(Some(WireNetworkMessage::Inv(InventoryList::new(vec![
                                InventoryVector {
                                    inventory_type: InventoryType::Block,
                                    object_hash: block_hash.into(),
                                },
                            ]))))
                        }
                    }
                }
            }
        }
    }

    pub fn announce_transaction(
        &self,
        peer_id: PeerId,
        transaction: &Transaction,
    ) -> Result<Option<WireNetworkMessage>, NetworkError> {
        let Some(peer) = self.peers.get(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        let txid = transaction_txid(transaction)?;
        let wtxid = transaction_wtxid(transaction)?;
        let inventory = if peer.remote_wtxidrelay {
            InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: wtxid.into(),
            }
        } else {
            InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }
        };
        Ok(Some(WireNetworkMessage::Inv(InventoryList::new(vec![
            inventory,
        ]))))
    }
}
#[cfg(test)]
mod tests;
