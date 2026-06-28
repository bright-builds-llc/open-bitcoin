// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h

use std::collections::{BTreeMap, BTreeSet};

mod header_sync;
mod inbound;
mod inventory;
mod peer_policy;

use open_bitcoin_core::{
    chainstate::{
        AnchoredBlock, ChainPosition, ChainTransition, ChainstateError, ChainstateSnapshot,
    },
    codec::CodecError,
    consensus::{ConsensusParams, ScriptVerifyFlags, block_hash, transaction_txid},
    primitives::{Block, BlockHash, NetworkMagic, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{AdmissionResult, MempoolError, PolicyConfig};
use open_bitcoin_network::{
    ConnectionRole, DisconnectReason, HeaderEntry, HeaderStore, HeaderSyncPolicy, HeadersMessage,
    InboundAdmissionPolicy, InboundResourceEvent, InventoryList, LocalAdvertisementDecision,
    LocalPeerConfig, NetworkError, PROTOCOL_VERSION, ParsedNetworkMessage, PeerAction, PeerId,
    PeerManager, WireNetworkMessage,
};

use crate::{ChainstateStore, ManagedChainstate, ManagedMempool};
use header_sync::validate_header_for_sync;
use inbound::{default_inbound_admission_policy, is_active_inbound_peer};

pub use inbound::{
    ManagedAddressBoundaryInfo, ManagedInboundAdmissionInfo, ManagedInboundPermissionDecisionInfo,
    ManagedPeerPolicyInfo, ManagedResourceGovernanceInfo,
};

#[derive(Debug)]
pub enum ManagedNetworkError {
    Network(NetworkError),
    Chainstate(ChainstateError),
    Mempool(MempoolError),
}

impl core::fmt::Display for ManagedNetworkError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Network(error) => error.fmt(f),
            Self::Chainstate(error) => error.fmt(f),
            Self::Mempool(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for ManagedNetworkError {}

impl From<NetworkError> for ManagedNetworkError {
    fn from(value: NetworkError) -> Self {
        Self::Network(value)
    }
}

impl From<ChainstateError> for ManagedNetworkError {
    fn from(value: ChainstateError) -> Self {
        Self::Chainstate(value)
    }
}

impl From<MempoolError> for ManagedNetworkError {
    fn from(value: MempoolError) -> Self {
        Self::Mempool(value)
    }
}

impl From<CodecError> for ManagedNetworkError {
    fn from(value: CodecError) -> Self {
        Self::Network(NetworkError::from(value))
    }
}

type ManagedResult<T> = Result<T, ManagedNetworkError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedMempoolInfo {
    pub transaction_count: usize,
    pub total_virtual_size: usize,
    pub total_fee_sats: i64,
    pub min_relay_feerate_sats_per_kvb: i64,
    pub incremental_relay_feerate_sats_per_kvb: i64,
    pub max_mempool_virtual_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedNetworkInfo {
    pub network_magic: NetworkMagic,
    pub protocol_version: i32,
    pub user_agent: String,
    pub local_services_bits: u64,
    pub relay: bool,
    pub connected_peers: usize,
    pub inbound_peers: usize,
    pub outbound_peers: usize,
    pub wtxidrelay_peers: usize,
    pub header_preferring_peers: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockConnectDisposition {
    Connected(ChainPosition),
    Duplicate(BlockHash),
    NonExtending {
        block_hash: BlockHash,
        previous_block_hash: BlockHash,
    },
    Disconnected {
        block_hash: BlockHash,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedSyncMessageResult {
    pub outbound: Vec<WireNetworkMessage>,
    pub maybe_block_disposition: Option<BlockConnectDisposition>,
}

#[derive(Debug, Clone)]
pub struct ManagedPeerNetwork<S> {
    chainstate: ManagedChainstate<S>,
    mempool: ManagedMempool,
    peer_manager: PeerManager,
    known_peers: BTreeSet<PeerId>,
    inbound_admission_policy: InboundAdmissionPolicy,
    inbound_admission_info: ManagedInboundAdmissionInfo,
    resource_governance_info: ManagedResourceGovernanceInfo,
    local_config: LocalPeerConfig,
    blocks_by_hash: BTreeMap<BlockHash, Block>,
    transactions_by_txid: BTreeMap<Txid, Transaction>,
    transactions_by_wtxid: BTreeMap<Wtxid, Transaction>,
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn new(store: S, local_config: LocalPeerConfig, mempool_config: PolicyConfig) -> Self {
        let chainstate = ManagedChainstate::from_store(store);
        let mut peer_manager = PeerManager::new(local_config.clone());
        peer_manager.seed_local_chain(&chainstate.chainstate().snapshot().active_chain);

        Self {
            chainstate,
            mempool: ManagedMempool::new(mempool_config),
            peer_manager,
            known_peers: BTreeSet::new(),
            inbound_admission_policy: default_inbound_admission_policy(),
            inbound_admission_info: ManagedInboundAdmissionInfo::default(),
            resource_governance_info: ManagedResourceGovernanceInfo::default(),
            local_config,
            blocks_by_hash: BTreeMap::new(),
            transactions_by_txid: BTreeMap::new(),
            transactions_by_wtxid: BTreeMap::new(),
        }
    }

    pub fn with_sync_limits(
        store: S,
        local_config: LocalPeerConfig,
        mempool_config: PolicyConfig,
        max_blocks_in_flight_per_peer: usize,
    ) -> Self {
        let chainstate = ManagedChainstate::from_store(store);
        let mut peer_manager = PeerManager::with_max_blocks_in_flight(
            local_config.clone(),
            max_blocks_in_flight_per_peer,
        );
        peer_manager.seed_local_chain(&chainstate.chainstate().snapshot().active_chain);

        Self {
            chainstate,
            mempool: ManagedMempool::new(mempool_config),
            peer_manager,
            known_peers: BTreeSet::new(),
            inbound_admission_policy: default_inbound_admission_policy(),
            inbound_admission_info: ManagedInboundAdmissionInfo::default(),
            resource_governance_info: ManagedResourceGovernanceInfo::default(),
            local_config,
            blocks_by_hash: BTreeMap::new(),
            transactions_by_txid: BTreeMap::new(),
            transactions_by_wtxid: BTreeMap::new(),
        }
    }

    #[rustfmt::skip]
    pub fn chainstate(&self) -> &ManagedChainstate<S> { &self.chainstate }

    #[rustfmt::skip]
    pub fn mempool(&self) -> &ManagedMempool { &self.mempool }

    #[rustfmt::skip]
    pub fn peer_manager(&self) -> &PeerManager { &self.peer_manager }

    pub fn set_local_address_decisions(&mut self, decisions: Vec<LocalAdvertisementDecision>) {
        self.peer_manager.set_local_address_decisions(decisions);
    }

    pub fn address_boundary_info(&self) -> ManagedAddressBoundaryInfo {
        self.peer_manager.address_boundary_evidence().into()
    }

    #[rustfmt::skip]
    pub fn resource_governance_info(&self) -> ManagedResourceGovernanceInfo { self.resource_governance_info.clone() }

    #[rustfmt::skip]
    pub fn record_resource_governance_event(&mut self, event: InboundResourceEvent) { self.resource_governance_info.record_event(event); }

    pub fn disconnect_peer(&mut self, peer_id: PeerId) -> Result<(), ManagedNetworkError> {
        self.peer_manager.remove_peer(peer_id)?;
        self.known_peers.remove(&peer_id);
        Ok(())
    }

    #[rustfmt::skip]
    pub fn seed_header_store(&mut self, header_store: HeaderStore) { self.peer_manager.seed_header_store(header_store); }

    pub fn header_entries(&self) -> Vec<HeaderEntry> {
        self.peer_manager
            .header_store()
            .entries()
            .cloned()
            .collect()
    }

    #[rustfmt::skip]
    pub fn best_chain_entries(&self) -> Vec<HeaderEntry> { self.peer_manager.header_store().best_chain_entries() }

    pub fn chainstate_snapshot(&self) -> ChainstateSnapshot {
        self.chainstate.chainstate().snapshot()
    }

    #[rustfmt::skip]
    pub fn maybe_chain_tip(&self) -> Option<ChainPosition> { self.chainstate.chainstate().tip().cloned() }

    #[rustfmt::skip]
    pub fn note_local_block_hash(&mut self, block_hash: BlockHash) { self.peer_manager.note_local_block_hash(block_hash); }

    #[rustfmt::skip]
    pub fn peer_requested_blocks(&self, peer_id: PeerId) -> ManagedResult<Vec<BlockHash>> { self.peer_manager.peer_requested_blocks(peer_id).map_err(ManagedNetworkError::from) }

    #[rustfmt::skip]
    pub fn request_missing_blocks(&mut self, peer_id: PeerId, block_hashes: &[BlockHash]) -> ManagedResult<Vec<WireNetworkMessage>> {
        let maybe_message = self.peer_manager.request_missing_blocks(peer_id, block_hashes)?;
        Ok(maybe_message.into_iter().collect())
    }

    pub fn mempool_info(&self) -> ManagedMempoolInfo {
        let entries = self.mempool.mempool().entries();
        let total_fee_sats = entries.values().map(|entry| entry.fee_sats()).sum();
        let config = self.mempool.mempool().config();

        ManagedMempoolInfo {
            transaction_count: entries.len(),
            total_virtual_size: self.mempool.mempool().total_virtual_size(),
            total_fee_sats,
            min_relay_feerate_sats_per_kvb: config.min_relay_feerate.sats_per_kvb(),
            incremental_relay_feerate_sats_per_kvb: config.incremental_relay_feerate.sats_per_kvb(),
            max_mempool_virtual_size: config.max_mempool_virtual_size,
        }
    }

    pub fn network_info(&self) -> ManagedNetworkInfo {
        let mut connected_peers = 0;
        let mut inbound_peers = 0;
        let mut outbound_peers = 0;
        let mut wtxidrelay_peers = 0;
        let mut header_preferring_peers = 0;

        for peer_id in &self.known_peers {
            let Some(peer) = self.peer_manager.peer_state(*peer_id) else {
                continue;
            };

            match peer.role {
                ConnectionRole::Inbound => {
                    if !is_active_inbound_peer(peer) {
                        continue;
                    }
                    connected_peers += 1;
                    inbound_peers += 1;
                }
                ConnectionRole::Outbound => {
                    connected_peers += 1;
                    outbound_peers += 1;
                }
            }
            if peer.remote_wtxidrelay {
                wtxidrelay_peers += 1;
            }
            if peer.remote_prefers_headers {
                header_preferring_peers += 1;
            }
        }

        ManagedNetworkInfo {
            network_magic: self.local_config.magic,
            protocol_version: PROTOCOL_VERSION,
            user_agent: self.local_config.user_agent.clone(),
            local_services_bits: self.local_config.services.bits(),
            relay: self.local_config.relay,
            connected_peers,
            inbound_peers,
            outbound_peers,
            wtxidrelay_peers,
            header_preferring_peers,
        }
    }

    pub fn connect_outbound_peer(
        &mut self,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let actions = self.peer_manager.add_outbound_peer(peer_id, timestamp)?;
        self.known_peers.insert(peer_id);
        self.collect_outbound(actions)
    }

    pub fn receive_message(
        &mut self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let actions = self
            .peer_manager
            .handle_message(peer_id, message, timestamp)?;
        Ok(self
            .process_actions(peer_id, actions, timestamp, verify_flags, consensus_params)?
            .outbound)
    }

    pub fn receive_sync_message(
        &mut self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkError> {
        let actions = match message {
            WireNetworkMessage::Headers(headers_message) => {
                self.handle_headers_message(peer_id, headers_message, timestamp, consensus_params)?
            }
            other => self
                .peer_manager
                .handle_message(peer_id, other, timestamp)?,
        };
        self.process_actions(peer_id, actions, timestamp, verify_flags, consensus_params)
    }

    pub fn receive_wire_message(
        &mut self,
        peer_id: PeerId,
        bytes: &[u8],
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        let parsed = ParsedNetworkMessage::decode_wire(bytes)?;
        self.receive_message(
            peer_id,
            parsed.message,
            timestamp,
            verify_flags,
            consensus_params,
        )
    }

    pub fn encode_messages(
        &self,
        messages: &[WireNetworkMessage],
    ) -> Result<Vec<Vec<u8>>, ManagedNetworkError> {
        messages
            .iter()
            .map(|message| message.encode_wire(self.local_config.magic))
            .collect::<Result<Vec<_>, _>>()
            .map_err(ManagedNetworkError::from)
    }

    pub fn connect_local_block(
        &mut self,
        block: &Block,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, ManagedNetworkError> {
        let position = self.chainstate.connect_block(
            block,
            self.next_chain_work(),
            verify_flags,
            consensus_params,
        )?;
        self.blocks_by_hash
            .insert(position.block_hash, block.clone());
        self.peer_manager.note_local_position(&position);
        Ok(position)
    }

    pub fn submit_local_transaction(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, ManagedNetworkError> {
        let result = self.mempool.submit_transaction(
            &self.chainstate,
            transaction.clone(),
            verify_flags,
            consensus_params,
        )?;
        self.store_transaction(transaction)?;
        Ok(result)
    }

    pub fn announce_block(
        &self,
        peer_id: PeerId,
        block: &Block,
    ) -> Result<Option<WireNetworkMessage>, ManagedNetworkError> {
        self.peer_manager
            .announce_block(peer_id, block)
            .map_err(ManagedNetworkError::from)
    }

    pub fn announce_transaction(
        &self,
        peer_id: PeerId,
        transaction: &Transaction,
    ) -> Result<Option<WireNetworkMessage>, ManagedNetworkError> {
        self.peer_manager
            .announce_transaction(peer_id, transaction)
            .map_err(ManagedNetworkError::from)
    }

    pub fn connect_stored_block(
        &mut self,
        block: &Block,
        chain_work: u128,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> ManagedResult<BlockConnectDisposition> {
        let block_hash = block_hash(&block.header);
        if self
            .chainstate
            .chainstate()
            .snapshot()
            .active_chain
            .iter()
            .any(|position| position.block_hash == block_hash)
        {
            self.blocks_by_hash.insert(block_hash, block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
            return Ok(BlockConnectDisposition::Duplicate(block_hash));
        }

        let maybe_tip = self.chainstate.chainstate().tip().cloned();
        let extends_tip = maybe_tip
            .as_ref()
            .is_none_or(|tip| tip.block_hash == block.header.previous_block_hash);
        let is_genesis = block.header.previous_block_hash.to_byte_array() == [0_u8; 32];
        if maybe_tip.is_some() && !extends_tip {
            self.blocks_by_hash.insert(block_hash, block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
            return Ok(BlockConnectDisposition::NonExtending {
                block_hash,
                previous_block_hash: block.header.previous_block_hash,
            });
        }
        if maybe_tip.is_none() && !is_genesis {
            self.blocks_by_hash.insert(block_hash, block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
            return Ok(BlockConnectDisposition::Disconnected { block_hash });
        }

        let position = self.chainstate.connect_block_with_current_time(
            block,
            chain_work,
            timestamp,
            verify_flags,
            consensus_params,
        )?;
        self.blocks_by_hash.insert(block_hash, block.clone());
        self.peer_manager.note_local_position(&position);
        Ok(BlockConnectDisposition::Connected(position))
    }

    pub fn reorg_to_branch(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> ManagedResult<ChainTransition> {
        let transition = self.chainstate.reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        for anchored_block in replacement_branch {
            let block_hash = block_hash(&anchored_block.block.header);
            self.blocks_by_hash
                .insert(block_hash, anchored_block.block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
        }
        for position in &transition.connected {
            self.peer_manager.note_local_position(position);
        }
        Ok(transition)
    }

    fn collect_outbound(
        &mut self,
        actions: Vec<PeerAction>,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        Ok(actions
            .into_iter()
            .filter_map(|action| match action {
                PeerAction::Send(message) => Some(message),
                _ => None,
            })
            .collect())
    }

    fn handle_headers_message(
        &mut self,
        peer_id: PeerId,
        headers_message: HeadersMessage,
        timestamp: i64,
        consensus_params: ConsensusParams,
    ) -> Result<Vec<PeerAction>, ManagedNetworkError> {
        self.peer_manager
            .handle_headers_with_policy(
                peer_id,
                headers_message,
                HeaderSyncPolicy::HeadersOnly,
                |header_store, header| {
                    validate_header_for_sync(header_store, header, timestamp, consensus_params)?;
                    header_store.insert_header(header.clone())
                },
            )
            .map_err(ManagedNetworkError::from)
    }

    fn process_actions(
        &mut self,
        peer_id: PeerId,
        actions: Vec<PeerAction>,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkError> {
        let mut outbound = Vec::new();
        let mut maybe_block_disposition = None;

        for action in actions {
            match action {
                PeerAction::Send(message) => outbound.push(message),
                PeerAction::ServeInventory(requests) => {
                    let (messages, missing) = self.serve_inventory(requests);
                    outbound.extend(messages);
                    if !missing.is_empty() {
                        outbound.push(WireNetworkMessage::NotFound(InventoryList::new(missing)));
                    }
                }
                PeerAction::ReceivedTransaction(transaction) => {
                    let txid = transaction_txid(&transaction)?;
                    if !self.transactions_by_txid.contains_key(&txid) {
                        self.mempool.submit_transaction(
                            &self.chainstate,
                            transaction.clone(),
                            verify_flags,
                            consensus_params,
                        )?;
                        self.store_transaction(transaction)?;
                    }
                }
                PeerAction::ReceivedBlock(block) => {
                    maybe_block_disposition = Some(self.connect_stored_block(
                        &block,
                        self.next_chain_work(),
                        timestamp,
                        verify_flags,
                        consensus_params,
                    )?);
                }
                PeerAction::Disconnect(reason) => {
                    if reason == DisconnectReason::SelfConnection {
                        self.record_runtime_self_connection_rejection(peer_id);
                    }
                    self.disconnect_peer(peer_id)?;
                    return Err(inventory::disconnect_network_error(peer_id, reason).into());
                }
                PeerAction::ResourceGovernanceDisconnect(event) => {
                    return self.disconnect_for_resource_governance(peer_id, event);
                }
            }
        }

        Ok(ManagedSyncMessageResult {
            outbound,
            maybe_block_disposition,
        })
    }
}

#[cfg(test)]
mod tests;
