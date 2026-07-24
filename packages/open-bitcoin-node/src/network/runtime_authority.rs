// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use std::{
    fmt,
    net::IpAddr,
    sync::{Arc, Mutex},
};

use open_bitcoin_core::{
    chainstate::{AnchoredBlock, ChainPosition, ChainTransition, ChainstateSnapshot},
    consensus::{ConsensusParams, ScriptVerifyFlags},
    mempool::{AdmissionResult, MempoolOutcome},
    primitives::{Block, BlockHash, NetworkAddress, NetworkMagic, Transaction},
};
use open_bitcoin_mempool::{PolicyConfig, RelayIntent, ReorgLifecycleContext};
use open_bitcoin_network::{
    BanDecision, BanScope, BlockRelayActivationPolicy, HeaderEntry, InboundAdmissionDecision,
    InboundAdmissionPolicy, InboundAdmissionRequest, InboundResourceEvent,
    LocalAdvertisementDecision, LocalPeerConfig, MisbehaviorDecision, PeerBanEntry, PeerId,
    PeerManager, ReconnectSuppressionInput, RelayActivationConfig, ServiceFlags, UnbanDecision,
    WireNetworkMessage,
};

use crate::{
    MemoryChainstateStore, StorageError,
    status::{BlockRelayEvidenceStatus, SyncRecoveryCategory, relay_evidence::RelayEvidenceStatus},
    storage::MempoolSnapshot,
    sync::SyncRuntimeError,
};

use super::{
    AnnouncementPreparationOutcome, BlockConnectDisposition, LocalRelaySubmissionEvidence,
    ManagedAddressBoundaryInfo, ManagedBlockServeCompletion, ManagedInboundAdmissionInfo,
    ManagedMempoolInfo, ManagedMempoolRecoverySummary, ManagedNetworkError, ManagedNetworkInfo,
    ManagedNetworkOperatorSnapshot, ManagedPeerNetwork, ManagedPeerPolicyInfo,
    ManagedResourceGovernanceInfo, ManagedSyncMessageResult, PeerEmissionReceipt,
    PeerOutboxSnapshot,
};

type AuthoritativeNetwork = ManagedPeerNetwork<MemoryChainstateStore>;

#[derive(Debug)]
pub enum ManagedNetworkAuthorityError {
    Poisoned,
    Operation(ManagedNetworkError),
}

impl fmt::Display for ManagedNetworkAuthorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Poisoned => formatter.write_str("authoritative network state is unavailable"),
            Self::Operation(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ManagedNetworkAuthorityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Poisoned => None,
            Self::Operation(error) => Some(error),
        }
    }
}

impl From<ManagedNetworkError> for ManagedNetworkAuthorityError {
    fn from(value: ManagedNetworkError) -> Self {
        Self::Operation(value)
    }
}

impl From<ManagedNetworkAuthorityError> for SyncRuntimeError {
    fn from(value: ManagedNetworkAuthorityError) -> Self {
        match value {
            ManagedNetworkAuthorityError::Poisoned => Self::Network {
                message: "authoritative network state is unavailable".to_string(),
            },
            ManagedNetworkAuthorityError::Operation(error) => Self::from(error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManagedNetworkHandle {
    authority: Arc<Mutex<AuthoritativeNetwork>>,
}

impl ManagedNetworkHandle {
    pub(crate) fn new(network: AuthoritativeNetwork) -> Self {
        Self {
            authority: Arc::new(Mutex::new(network)),
        }
    }

    /// Wraps an explicitly constructed in-memory network for tests and benchmarks.
    pub fn from_network_fixture(network: AuthoritativeNetwork) -> Self {
        Self::new(network)
    }

    /// Creates a transient production authority when no durable store is configured.
    pub fn transient_runtime(
        magic: NetworkMagic,
        port: u16,
        relay_activation: RelayActivationConfig,
        block_relay_activation: BlockRelayActivationPolicy,
        inbound_enabled: bool,
    ) -> Self {
        let local_config = LocalPeerConfig {
            magic,
            services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
            address: NetworkAddress {
                services: 0,
                address_bytes: [0_u8; 16],
                port,
            },
            nonce: 0,
            relay: true,
            user_agent: "/open-bitcoin:0.1.0/".to_string(),
        };
        let network = ManagedPeerNetwork::new_with_block_relay_activation(
            MemoryChainstateStore::default(),
            local_config,
            PolicyConfig::default(),
            relay_activation,
            block_relay_activation,
            inbound_enabled,
        );
        Self::new(network)
    }

    fn read<T>(
        &self,
        snapshot: impl FnOnce(&AuthoritativeNetwork) -> T,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        let network = self
            .authority
            .lock()
            .map_err(|_| ManagedNetworkAuthorityError::Poisoned)?;
        Ok(snapshot(&network))
    }

    fn mutate<T>(
        &self,
        command: impl FnOnce(&mut AuthoritativeNetwork) -> T,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        let mut network = self
            .authority
            .lock()
            .map_err(|_| ManagedNetworkAuthorityError::Poisoned)?;
        Ok(command(&mut network))
    }

    fn try_mutate<T>(
        &self,
        command: impl FnOnce(&mut AuthoritativeNetwork) -> Result<T, ManagedNetworkError>,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        self.mutate(command)?.map_err(Into::into)
    }

    fn try_read<T>(
        &self,
        snapshot: impl FnOnce(&AuthoritativeNetwork) -> Result<T, ManagedNetworkError>,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        self.read(snapshot)?.map_err(Into::into)
    }

    pub fn chainstate_snapshot(&self) -> Result<ChainstateSnapshot, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::chainstate_snapshot)
    }

    pub fn maybe_chain_tip(&self) -> Result<Option<ChainPosition>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::maybe_chain_tip)
    }

    pub fn header_entries(&self) -> Result<Vec<HeaderEntry>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::header_entries)
    }

    pub fn best_chain_entries(&self) -> Result<Vec<HeaderEntry>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::best_chain_entries)
    }

    pub fn peer_manager_snapshot(&self) -> Result<PeerManager, ManagedNetworkAuthorityError> {
        self.read(|network| network.peer_manager().clone())
    }

    pub fn network_info(&self) -> Result<ManagedNetworkInfo, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::network_info)
    }

    #[rustfmt::skip]
    pub fn operator_snapshot(&self) -> Result<ManagedNetworkOperatorSnapshot, ManagedNetworkAuthorityError> { self.read(ManagedPeerNetwork::operator_snapshot) }

    pub fn mempool_info(&self) -> Result<ManagedMempoolInfo, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::mempool_info)
    }

    pub fn relay_evidence_status(
        &self,
    ) -> Result<RelayEvidenceStatus, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::relay_evidence_status)
    }

    pub fn inbound_admission_info(
        &self,
    ) -> Result<ManagedInboundAdmissionInfo, ManagedNetworkAuthorityError> {
        self.read(|network| network.inbound_admission_info().clone())
    }

    pub fn address_boundary_info(
        &self,
    ) -> Result<ManagedAddressBoundaryInfo, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::address_boundary_info)
    }

    pub fn peer_policy_info(&self) -> Result<ManagedPeerPolicyInfo, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::peer_policy_info)
    }

    pub fn resource_governance_info(
        &self,
    ) -> Result<ManagedResourceGovernanceInfo, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::resource_governance_info)
    }

    pub fn record_resource_governance_event(
        &self,
        event: InboundResourceEvent,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.record_resource_governance_event(event))
    }

    pub fn record_peer_policy_ban(
        &self,
        entry: PeerBanEntry,
        now_unix_seconds: i64,
    ) -> Result<BanDecision, ManagedNetworkAuthorityError> {
        self.mutate(|network| network.record_peer_policy_ban(entry, now_unix_seconds))
    }

    pub fn record_peer_policy_discouragement(
        &self,
        entry: PeerBanEntry,
        now_unix_seconds: i64,
    ) -> Result<BanDecision, ManagedNetworkAuthorityError> {
        self.mutate(|network| network.record_peer_policy_discouragement(entry, now_unix_seconds))
    }

    pub fn record_peer_policy_unban(
        &self,
        scope: &BanScope,
        now_unix_seconds: i64,
    ) -> Result<UnbanDecision, ManagedNetworkAuthorityError> {
        self.mutate(|network| network.record_peer_policy_unban(scope, now_unix_seconds))
    }

    pub fn record_peer_policy_misbehavior(
        &self,
        decision: MisbehaviorDecision,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.record_peer_policy_misbehavior(decision))
    }

    pub fn set_inbound_admission_policy(
        &self,
        policy: InboundAdmissionPolicy,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.set_inbound_admission_policy(policy))
    }

    pub fn set_local_address_decisions(
        &self,
        decisions: Vec<LocalAdvertisementDecision>,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.set_local_address_decisions(decisions))
    }

    pub fn reconnect_suppression_input_for_ip(
        &self,
        remote_ip: IpAddr,
        now_unix_seconds: i64,
    ) -> Result<ReconnectSuppressionInput, ManagedNetworkAuthorityError> {
        self.read(|network| network.reconnect_suppression_input_for_ip(remote_ip, now_unix_seconds))
    }

    pub fn admit_inbound_peer(
        &self,
        request: InboundAdmissionRequest,
    ) -> Result<InboundAdmissionDecision, ManagedNetworkAuthorityError> {
        self.mutate(|network| network.admit_inbound_peer(request))
    }

    pub fn add_inbound_peer(&self, peer_id: PeerId) -> Result<(), ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.add_inbound_peer(peer_id))
    }

    pub fn connect_outbound_peer(
        &self,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.connect_outbound_peer(peer_id, timestamp))
    }

    pub fn receive_sync_message(
        &self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.receive_sync_message(
                peer_id,
                message,
                timestamp,
                verify_flags,
                consensus_params,
            )
        })
    }

    pub fn receive_message(
        &self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.receive_message(peer_id, message, timestamp, verify_flags, consensus_params)
        })
    }

    pub fn receive_message_for_durable_serving(
        &self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.receive_message_for_durable_serving(
                peer_id,
                message,
                timestamp,
                verify_flags,
                consensus_params,
            )
        })
    }

    pub fn encode_messages(
        &self,
        messages: &[WireNetworkMessage],
    ) -> Result<Vec<Vec<u8>>, ManagedNetworkAuthorityError> {
        self.try_read(|network| network.encode_messages(messages))
    }

    pub fn expire_compact_download_timeouts(
        &self,
        timestamp: i64,
    ) -> Result<Vec<(PeerId, WireNetworkMessage)>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.expire_compact_download_timeouts(timestamp))
    }

    pub fn peer_requested_blocks(
        &self,
        peer_id: PeerId,
    ) -> Result<Vec<BlockHash>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.peer_requested_blocks(peer_id))
    }

    pub fn request_missing_blocks(
        &self,
        peer_id: PeerId,
        block_hashes: &[BlockHash],
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.request_missing_blocks(peer_id, block_hashes))
    }

    pub fn disconnect_peer(&self, peer_id: PeerId) -> Result<(), ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.disconnect_peer(peer_id))
    }

    pub fn acknowledge_wire_message_written(
        &self,
        message: &WireNetworkMessage,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.acknowledge_wire_message_written(message))
    }

    pub fn complete_block_serve(
        &self,
        completion: &ManagedBlockServeCompletion,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| {
            network.complete_block_serve(completion);
        })
    }

    pub fn connect_stored_block(
        &self,
        block: &Block,
        chain_work: u128,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<BlockConnectDisposition, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.connect_stored_block(
                block,
                chain_work,
                timestamp,
                verify_flags,
                consensus_params,
            )
        })
    }

    pub fn reorg_to_branch(
        &self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        context: ReorgLifecycleContext,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainTransition, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.reorg_to_branch(
                disconnect_blocks,
                replacement_branch,
                context,
                verify_flags,
                consensus_params,
            )
        })
    }

    pub fn note_local_block_hash(
        &self,
        block_hash: BlockHash,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.note_local_block_hash(block_hash))
    }

    pub fn block_relay_evidence_status(
        &self,
    ) -> Result<BlockRelayEvidenceStatus, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::block_relay_evidence_status)
    }

    pub fn block_served_write_count(&self) -> Result<u64, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::block_served_write_count)
    }

    pub fn connect_local_block(
        &self,
        block: &Block,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.connect_local_block(block, verify_flags, consensus_params)
        })
    }

    /// Fail-closed no-time admission retained for intermediate workspace compatibility.
    ///
    /// Plan 130-06 migrates node callers. Plan 130-11 migrates the final RPC caller
    /// and removes this adapter.
    #[deprecated(
        note = "Plan 130-06 migrates node callers; Plan 130-11 migrates the final RPC caller and removes this fail-closed adapter"
    )]
    #[allow(deprecated)]
    pub fn submit_local_transaction(
        &self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.submit_local_transaction(transaction, verify_flags, consensus_params)
        })
    }

    /// Fail-closed no-time outcome retained for intermediate workspace compatibility.
    ///
    /// Plan 130-06 migrates node callers. Plan 130-11 migrates the final RPC caller
    /// and removes this adapter.
    #[deprecated(
        note = "Plan 130-06 migrates node callers; Plan 130-11 migrates the final RPC caller and removes this fail-closed adapter"
    )]
    #[allow(deprecated)]
    pub fn submit_local_transaction_outcome(
        &self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<MempoolOutcome, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.submit_local_transaction_outcome(transaction, verify_flags, consensus_params)
        })
    }

    /// Submits a local transaction with shell-sampled time and resolved relay intent.
    pub fn submit_local_transaction_outcome_at(
        &self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        now_unix_seconds: i64,
        relay_intent: RelayIntent,
    ) -> Result<MempoolOutcome, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.submit_local_transaction_outcome_at(
                transaction,
                verify_flags,
                consensus_params,
                now_unix_seconds,
                relay_intent,
            )
        })
    }

    pub fn latest_local_submission_evidence(
        &self,
    ) -> Result<Option<LocalRelaySubmissionEvidence>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::latest_local_submission_evidence)
    }

    pub fn recover_mempool_snapshot(
        &self,
        snapshot: &MempoolSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedMempoolRecoverySummary, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.recover_mempool_snapshot(snapshot, verify_flags, consensus_params)
        })
    }

    pub fn record_mempool_recovery_storage_error(
        &self,
        error: &StorageError,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.record_mempool_recovery_storage_error(error))
    }

    pub fn record_mempool_recovery_unavailable(
        &self,
        category: SyncRecoveryCategory,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.record_mempool_recovery_unavailable(category))
    }

    pub fn latest_mempool_recovery_summary(
        &self,
    ) -> Result<Option<ManagedMempoolRecoverySummary>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::latest_mempool_recovery_summary)
    }

    pub fn latest_mempool_recovery_storage_error(
        &self,
    ) -> Result<Option<SyncRecoveryCategory>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::latest_mempool_recovery_storage_error)
    }

    pub fn announce_block(
        &self,
        peer_id: PeerId,
        block: &Block,
    ) -> Result<Option<WireNetworkMessage>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.announce_block(peer_id, block))
    }

    #[rustfmt::skip]
    pub fn prepare_block_announcements(&self, block: &Block, outboxes: &[PeerOutboxSnapshot]) -> Result<Vec<AnnouncementPreparationOutcome>, ManagedNetworkAuthorityError> { let compact_nonces = super::announcement_transport::compact_nonces(outboxes); self.mutate(|network| network.prepare_block_announcements(block, outboxes, &compact_nonces)) }

    #[rustfmt::skip]
    pub fn complete_peer_emission(&self, receipt: PeerEmissionReceipt) -> Result<(), ManagedNetworkAuthorityError> { self.try_mutate(|network| network.complete_peer_emission(receipt)) }

    #[cfg(test)]
    fn poison_for_test(&self) {
        let authority = Arc::clone(&self.authority);
        let result = std::thread::spawn(move || {
            let _network = authority.lock().expect("test authority should lock");
            panic!("poison authoritative network for test");
        })
        .join();
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests;
