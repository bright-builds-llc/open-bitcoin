// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_consensus::{block_hash, check_block_header, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Block, BlockHash, BlockHeader, InventoryType, InventoryVector, Transaction, Txid, Wtxid,
};

use crate::address::{
    GetAddrRequestState, GetAddrResponseDecision, LearnedAddressBook, LearnedAddressDecision,
    LocalAdvertisementDecision, maybe_version_sender_address,
};
use crate::error::{DisconnectReason, NetworkError, PeerId};
use crate::header_store::{HeaderStore, InsertedHeader};
use crate::inbound::{InboundAdmissionRejectionReason, InboundHandshakeState, InboundPeerRecord};
use crate::message::{HeadersMessage, InventoryList, LocalPeerConfig, WireNetworkMessage};
use crate::peer_policy::{
    EvictionCandidateInput, EvictionDecision, MisbehaviorDecision, MisbehaviorKind,
    MisbehaviorObservation, MisbehaviorPolicy, select_eviction_candidate,
};
use crate::resource::InboundResourceEvent;

mod address_boundary;
mod inbound_state;
mod inventory_state;
mod policy_state;

pub use address_boundary::{PeerAddressBoundaryDecision, PeerAddressBoundaryEvidence};
use inbound_state::reject_self_connection;
use inventory_state::forget_requested_inventory;
use policy_state::{eviction_candidate_input, peer_policy_label, peer_policy_protected};

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
    ReceivedTransaction(Transaction),
    ReceivedBlock(Block),
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
    pub remote_version_received: bool,
    pub remote_verack_received: bool,
    pub local_version_sent: bool,
    pub local_verack_sent: bool,
    pub sync_started: bool,
    pub getheaders_in_flight: bool,
    pub requested_blocks: BTreeSet<BlockHash>,
    pub requested_txids: BTreeSet<Txid>,
    pub requested_wtxids: BTreeSet<Wtxid>,
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
            remote_version_received: false,
            remote_verack_received: false,
            local_version_sent: false,
            local_verack_sent: false,
            sync_started: false,
            getheaders_in_flight: false,
            requested_blocks: BTreeSet::new(),
            requested_txids: BTreeSet::new(),
            requested_wtxids: BTreeSet::new(),
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
}

#[derive(Debug, Clone)]
pub struct PeerManager {
    local_config: LocalPeerConfig,
    headers: HeaderStore,
    peers: BTreeMap<PeerId, PeerState>,
    known_blocks: BTreeSet<BlockHash>,
    known_txids: BTreeSet<Txid>,
    known_wtxids: BTreeSet<Wtxid>,
    max_blocks_in_flight_per_peer: usize,
    learned_addresses: LearnedAddressBook,
    local_address_decisions: Vec<LocalAdvertisementDecision>,
    getaddr_responses_served: Vec<GetAddrResponseDecision>,
    getaddr_requests_suppressed: Vec<GetAddrResponseDecision>,
    learned_address_rejections: Vec<LearnedAddressDecision>,
    learned_address_rejection_count: usize,
    maybe_latest_address_decision: Option<PeerAddressBoundaryDecision>,
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
            max_blocks_in_flight_per_peer,
            learned_addresses: LearnedAddressBook::default(),
            local_address_decisions: Vec::new(),
            getaddr_responses_served: Vec::new(),
            getaddr_requests_suppressed: Vec::new(),
            learned_address_rejections: Vec::new(),
            learned_address_rejection_count: 0,
            maybe_latest_address_decision: None,
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
        self.known_txids.insert(transaction_txid(transaction)?);
        self.known_wtxids.insert(transaction_wtxid(transaction)?);
        Ok(())
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
            .map(|(peer_id, peer)| eviction_candidate_input(*peer_id, peer))
            .collect()
    }

    pub fn eviction_decision(&self) -> EvictionDecision {
        let inputs = self.eviction_candidate_inputs();
        select_eviction_candidate(&inputs)
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

    pub fn remove_peer(&mut self, peer_id: PeerId) -> Result<(), NetworkError> {
        let Some(_) = self.peers.remove(&peer_id) else {
            return Err(NetworkError::UnknownPeer(peer_id));
        };
        Ok(())
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
        let block_hash = block_hash(&block.header);
        if peer.remote_prefers_headers {
            return Ok(Some(WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![block.header.clone()],
            })));
        }
        Ok(Some(WireNetworkMessage::Inv(InventoryList::new(vec![
            InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: block_hash.into(),
            },
        ]))))
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

    pub fn handle_message(
        &mut self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        match message {
            WireNetworkMessage::Version(version) => {
                self.handle_version(peer_id, version, timestamp)
            }
            WireNetworkMessage::Verack => self.handle_verack(peer_id),
            WireNetworkMessage::WtxidRelay => {
                let peer = Self::peer_mut(&mut self.peers, peer_id)?;
                peer.remote_wtxidrelay = true;
                Ok(Vec::new())
            }
            WireNetworkMessage::SendHeaders => {
                let peer = Self::peer_mut(&mut self.peers, peer_id)?;
                peer.remote_prefers_headers = true;
                Ok(Vec::new())
            }
            WireNetworkMessage::Ping { nonce } => {
                Ok(vec![PeerAction::Send(WireNetworkMessage::Pong { nonce })])
            }
            WireNetworkMessage::Pong { nonce } => {
                let peer = Self::peer_mut(&mut self.peers, peer_id)?;
                if peer.last_ping_nonce == Some(nonce) {
                    peer.last_ping_nonce = None;
                }
                Ok(Vec::new())
            }
            WireNetworkMessage::Inv(inventory) => self.handle_inventory(peer_id, inventory),
            WireNetworkMessage::GetHeaders { locator, stop_hash } => {
                self.handle_getheaders(peer_id, locator, stop_hash)
            }
            WireNetworkMessage::Headers(message) => self.handle_headers(peer_id, message),
            WireNetworkMessage::GetAddr => self.handle_getaddr(peer_id, timestamp),
            WireNetworkMessage::Addr(addresses) => self.handle_addr(peer_id, addresses, timestamp),
            WireNetworkMessage::GetData(inventory) => self.handle_getdata(peer_id, inventory),
            WireNetworkMessage::NotFound(inventory) => {
                let peer = Self::peer_mut(&mut self.peers, peer_id)?;
                for item in inventory.inventory {
                    forget_requested_inventory(peer, item.inventory_type, item.object_hash);
                }
                Ok(Vec::new())
            }
            WireNetworkMessage::Tx(transaction) => self.handle_transaction(peer_id, transaction),
            WireNetworkMessage::Block(block) => self.handle_block(peer_id, block),
        }
    }

    fn handle_version(
        &mut self,
        peer_id: PeerId,
        version: crate::VersionMessage,
        timestamp: i64,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        let best_height = self.headers.best_height();
        let local_nonce = self.local_config.nonce;
        let maybe_sender = maybe_version_sender_address(&self.local_address_decisions);
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        if peer.remote_version_received {
            return Ok(vec![PeerAction::Disconnect(
                DisconnectReason::DuplicateVersion,
            )]);
        }

        if peer.role == ConnectionRole::Inbound && version.nonce == local_nonce {
            return Ok(vec![reject_self_connection(peer, version.nonce)]);
        }

        peer.remote_version_received = true;
        peer.remote_start_height = version.start_height;
        peer.remote_services_bits = version.services.bits();
        peer.remote_user_agent = version.user_agent.clone();
        if let Some(record) = peer.maybe_inbound_record.as_mut() {
            record.maybe_remote_nonce = Some(version.nonce);
            record.handshake_state = InboundHandshakeState::Handshaking;
        }

        let mut actions = Vec::new();
        if !peer.local_version_sent {
            peer.local_version_sent = true;
            actions.push(PeerAction::Send(WireNetworkMessage::Version(
                self.local_config.version_message_with_sender_policy(
                    timestamp,
                    best_height,
                    maybe_sender,
                ),
            )));
        }
        if !peer.local_verack_sent {
            peer.local_verack_sent = true;
            actions.push(PeerAction::Send(WireNetworkMessage::WtxidRelay));
            actions.push(PeerAction::Send(WireNetworkMessage::Verack));
            actions.push(PeerAction::Send(WireNetworkMessage::SendHeaders));
        }
        Ok(actions)
    }

    fn handle_verack(&mut self, peer_id: PeerId) -> Result<Vec<PeerAction>, NetworkError> {
        let locator = self.headers.locator();
        let best_height = self.headers.best_height();
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        peer.remote_verack_received = true;
        if let Some(record) = peer.maybe_inbound_record.as_mut()
            && peer.remote_version_received
            && peer.local_verack_sent
        {
            record.handshake_state = InboundHandshakeState::Established;
        }

        if peer.remote_start_height > best_height && !peer.getheaders_in_flight {
            peer.getheaders_in_flight = true;
            peer.sync_started = true;
            return Ok(vec![PeerAction::Send(WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            })]);
        }
        Ok(Vec::new())
    }

    fn handle_headers(
        &mut self,
        peer_id: PeerId,
        headers_message: HeadersMessage,
    ) -> Result<Vec<PeerAction>, NetworkError> {
        self.handle_headers_with_policy(
            peer_id,
            headers_message,
            HeaderSyncPolicy::HeadersAndBlocks,
            |headers: &mut HeaderStore, header: &BlockHeader| {
                check_block_header(header).map_err(|error| NetworkError::InvalidHeader {
                    reject_reason: error.reject_reason.to_string(),
                    maybe_debug_message: error.debug_message.clone(),
                })?;
                headers.insert_header(header.clone())
            },
        )
    }

    pub fn handle_headers_with_policy<F>(
        &mut self,
        peer_id: PeerId,
        headers_message: HeadersMessage,
        policy: HeaderSyncPolicy,
        mut validate_and_insert: F,
    ) -> Result<Vec<PeerAction>, NetworkError>
    where
        F: FnMut(&mut HeaderStore, &BlockHeader) -> Result<InsertedHeader, NetworkError>,
    {
        let previous_best_height = self.headers.best_height();
        let header_count = headers_message.headers.len();
        let mut requested_inventory = Vec::new();
        for header in headers_message.headers {
            let inserted = validate_and_insert(&mut self.headers, &header)?;
            if !self.known_blocks.contains(&inserted.block_hash) {
                requested_inventory.push(InventoryVector {
                    inventory_type: InventoryType::Block,
                    object_hash: inserted.block_hash.into(),
                });
            }
        }

        let best_height = self.headers.best_height();
        let locator = self.headers.locator();
        let max_blocks_in_flight_per_peer = self
            .max_blocks_in_flight_per_peer
            .min(crate::PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER);
        let peer = Self::peer_mut(&mut self.peers, peer_id)?;
        peer.getheaders_in_flight = false;

        let header_progressed = best_height > previous_best_height;
        let should_request_more_headers = header_count > 0
            && header_progressed
            && (header_count == crate::MAX_HEADERS_RESULTS
                || peer.remote_start_height > best_height);

        let mut actions = Vec::new();
        if should_request_more_headers {
            peer.getheaders_in_flight = true;
            peer.sync_started = true;
            actions.push(PeerAction::Send(WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            }));
        }

        if policy == HeaderSyncPolicy::HeadersAndBlocks {
            let available_slots =
                max_blocks_in_flight_per_peer.saturating_sub(peer.requested_blocks.len());
            requested_inventory.truncate(available_slots);
            for item in &requested_inventory {
                peer.requested_blocks
                    .insert(BlockHash::from(item.object_hash));
            }
            if !requested_inventory.is_empty() {
                actions.push(PeerAction::Send(WireNetworkMessage::GetData(
                    InventoryList::new(requested_inventory),
                )));
            }
        }

        Ok(actions)
    }
}

#[cfg(test)]
mod tests;
