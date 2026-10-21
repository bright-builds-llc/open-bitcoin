// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use std::net::SocketAddr;

use open_bitcoin_mempool::{PackageReport, SubmittedPackageResult};
use open_bitcoin_network::{
    InboundAdmissionDecision, InboundAdmissionPolicy, InboundAdmissionRequest,
    InboundPermissionDecision, ReconnectSuppressionInput,
};
use open_bitcoin_node::core::chainstate::ChainstateSnapshot;
use open_bitcoin_node::core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_node::core::mempool::PolicyConfig;
use open_bitcoin_node::core::mempool::{
    AdmissionResult, MempoolEntryMetadata, MempoolOutcome, RelayIntent,
};
use open_bitcoin_node::core::network::{ServiceFlags, WireNetworkMessage};
use open_bitcoin_node::core::primitives::{Block, NetworkMagic, Transaction, Txid};
use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_node::network::{
    LocalRelaySubmissionEvidence, ManagedInboundAdmissionInfo, ManagedMempoolInfo,
    ManagedNetworkInfo,
};
use open_bitcoin_node::status::{BlockRelayEvidenceStatus, relay_evidence::RelayEvidenceStatus};
use open_bitcoin_node::{
    DurableSyncState, FjallNodeStore, MetricRetentionPolicy, MetricsStatus, PersistMode,
    StorageError,
};
use open_bitcoin_node::{ManagedNetworkAuthorityError, ManagedNetworkHandle};

use super::CanonicalContextError;
#[cfg(test)]
use super::EncodedWireResponse;
use super::canonical_chain::{require_ordinary_scope, selected_chain};
use super::mempool_recovery::{
    recover_fixture_mempool_snapshot_from_store_handle, recover_mempool_snapshot_from_store_handle,
};
use super::wallet_state::{
    WalletState, build_fixture_wallet_state_with_store, build_wallet_state_with_store,
};
use super::{ManagedRpcContext, address_boundary::local_advertisement_decisions};
use crate::{config::RuntimeConfig, inbound_listener::InboundListenerEvidence};

fn effective_context_store(
    maybe_store: Option<FjallNodeStore>,
    wallet_state: &WalletState,
) -> Option<FjallNodeStore> {
    match wallet_state {
        WalletState::Unavailable => maybe_store,
        WalletState::Local(_) => maybe_store,
        WalletState::DurableNamedRegistry { store, .. } => Some(store.clone()),
    }
}

fn context_consensus_params(config: &RuntimeConfig) -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: config.wallet.coinbase_maturity,
        ..ConsensusParams::default()
    }
}

fn fixture_authority_storage_error(_: ManagedNetworkAuthorityError) -> StorageError {
    StorageError::BackendFailure {
        namespace: open_bitcoin_node::storage::StorageNamespace::Runtime,
        message: "fixture context authority unavailable".to_string(),
        action: open_bitcoin_node::storage::StorageRecoveryAction::RestoreFromBackup,
    }
}

impl<S: open_bitcoin_node::ChainstateStore, V: open_bitcoin_node::core::chainstate::CoinsView>
    ManagedRpcContext<S, V>
{
    /// Validate selected canonical ownership and support before any startup mutation.
    pub fn from_runtime_config_with_network_handle(
        config: &RuntimeConfig,
        mut network: ManagedNetworkHandle<S, V>,
        maybe_store: Option<FjallNodeStore>,
    ) -> Result<Self, CanonicalContextError> {
        let chain = selected_chain(config)?;
        let Some(owner_chain) = network.maybe_canonical_chain()? else {
            return Err(CanonicalContextError::UnprovedOwner);
        };
        if owner_chain != chain {
            return Err(CanonicalContextError::ChainMismatch);
        }
        require_ordinary_scope(chain)?;
        Self::configure_context_network(config, &mut network)?;
        let wallet_state = build_wallet_state_with_store(config, maybe_store.clone())?;
        let effective_store = effective_context_store(maybe_store, &wallet_state);
        recover_mempool_snapshot_from_store_handle(
            config,
            effective_store.as_ref(),
            &network,
            &PolicyConfig::default(),
            default_verify_flags(),
            context_consensus_params(config),
        )?;
        Ok(Self::assemble_context(
            config,
            network,
            effective_store,
            wallet_state,
        ))
    }

    /// Synthetic/source-derived regression evidence only; never ordinary canonical authorization.
    pub fn from_fixture_runtime_config_with_network_handle(
        config: &RuntimeConfig,
        network: ManagedNetworkHandle<S, V>,
        maybe_store: Option<FjallNodeStore>,
    ) -> Result<Self, StorageError> {
        let wallet_state = build_fixture_wallet_state_with_store(config, maybe_store.clone())?;
        Self::assemble_fixture_context(config, network, maybe_store, wallet_state)
    }

    pub(super) fn assemble_fixture_context(
        config: &RuntimeConfig,
        mut network: ManagedNetworkHandle<S, V>,
        maybe_store: Option<FjallNodeStore>,
        wallet_state: WalletState,
    ) -> Result<Self, StorageError> {
        Self::configure_context_network(config, &mut network)
            .map_err(fixture_authority_storage_error)?;
        let effective_store = effective_context_store(maybe_store, &wallet_state);
        recover_fixture_mempool_snapshot_from_store_handle(
            config,
            effective_store.as_ref(),
            &network,
            &PolicyConfig::default(),
            default_verify_flags(),
            context_consensus_params(config),
        )
        .map_err(fixture_authority_storage_error)?;
        Ok(Self::assemble_context(
            config,
            network,
            effective_store,
            wallet_state,
        ))
    }

    fn configure_context_network(
        config: &RuntimeConfig,
        network: &mut ManagedNetworkHandle<S, V>,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        network.set_prune_mode(config.prune_mode)?;
        network.set_inbound_admission_policy(InboundAdmissionPolicy::new(
            config.inbound.max_peers,
            config.inbound.reserved_slots,
        ))?;
        Ok(())
    }

    pub(super) fn assemble_context(
        config: &RuntimeConfig,
        network: ManagedNetworkHandle<S, V>,
        effective_store: Option<FjallNodeStore>,
        wallet_state: WalletState,
    ) -> Self {
        let consensus_params = context_consensus_params(config);
        let maybe_resource_governance_log_dir =
            config.maybe_data_dir.as_ref().map(|dir| dir.join("logs"));
        Self {
            chain: config.chain,
            consensus_params,
            verify_flags: default_verify_flags(),
            network,
            permission_classes: config.inbound.permission_classes.clone(),
            inbound_permission_validation_failures: config.inbound_permission_validation_failures,
            inbound_listener_config: config.inbound.clone(),
            maybe_inbound_listener_evidence: None,
            maybe_resource_governance_log_dir,
            resource_governance_log_retention: Default::default(),
            resource_governance_log_write_failures: 0,
            maybe_block_source: super::durable_block_source(effective_store.clone()),
            maybe_metrics_store: effective_store.clone(),
            maybe_runtime_metadata_source: effective_store,
            maybe_daemon_sync_control: None,
            manual_prune: open_bitcoin_node::status::ManualPruneSurface::None,
            wallet_state,
        }
    }

    pub fn chain(&self) -> AddressNetwork {
        self.chain
    }

    pub fn chain_name(&self) -> &'static str {
        match self.chain {
            AddressNetwork::Mainnet => "main",
            AddressNetwork::Testnet => "test",
            AddressNetwork::Signet => "signet",
            AddressNetwork::Regtest => "regtest",
        }
    }

    pub fn consensus_params(&self) -> ConsensusParams {
        self.consensus_params
    }

    pub fn verify_flags(&self) -> ScriptVerifyFlags {
        self.verify_flags
    }

    pub fn coinbase_maturity(&self) -> u32 {
        self.consensus_params.coinbase_maturity
    }

    pub fn blockchain_snapshot(&self) -> Result<ChainstateSnapshot, ManagedNetworkAuthorityError> {
        self.network.chainstate_snapshot()
    }

    pub fn maybe_chain_tip(
        &self,
    ) -> Result<
        Option<open_bitcoin_node::core::chainstate::ChainPosition>,
        ManagedNetworkAuthorityError,
    > {
        self.network.maybe_chain_tip()
    }

    pub fn current_durable_sync_state(
        &self,
    ) -> Result<Option<DurableSyncState>, open_bitcoin_node::StorageError> {
        let Some(source) = self.maybe_runtime_metadata_source.as_ref() else {
            return Ok(None);
        };

        source
            .load_runtime_metadata()
            .map(|maybe_metadata| maybe_metadata.and_then(|metadata| metadata.maybe_sync_state))
    }

    pub fn set_metrics_store(&mut self, store: FjallNodeStore) {
        self.maybe_metrics_store = Some(store);
    }

    pub fn mempool_info(&self) -> Result<ManagedMempoolInfo, ManagedNetworkAuthorityError> {
        self.network.mempool_info()
    }

    pub fn network_info(&self) -> Result<ManagedNetworkInfo, ManagedNetworkAuthorityError> {
        self.network.network_info()
    }

    pub fn relay_evidence_status(
        &self,
    ) -> Result<RelayEvidenceStatus, ManagedNetworkAuthorityError> {
        self.network.relay_evidence_status()
    }

    pub fn block_relay_evidence_status(
        &self,
    ) -> Result<BlockRelayEvidenceStatus, ManagedNetworkAuthorityError> {
        self.network.block_relay_evidence_status()
    }

    pub fn inbound_admission_info(
        &self,
    ) -> Result<ManagedInboundAdmissionInfo, ManagedNetworkAuthorityError> {
        self.network.inbound_admission_info()
    }

    pub fn set_inbound_listener_evidence(
        &mut self,
        evidence: InboundListenerEvidence,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        let services = ServiceFlags::from_bits(self.network_info()?.local_services_bits);
        let decisions =
            local_advertisement_decisions(&self.inbound_listener_config, &evidence, services);
        self.network.set_local_address_decisions(decisions)?;
        self.maybe_inbound_listener_evidence = Some(evidence);
        Ok(())
    }

    pub fn reconnect_suppression_input_for_remote_addr(
        &self,
        remote_addr: SocketAddr,
        now_unix_seconds: i64,
    ) -> Result<ReconnectSuppressionInput, ManagedNetworkAuthorityError> {
        self.network
            .reconnect_suppression_input_for_ip(remote_addr.ip(), now_unix_seconds)
    }

    #[cfg(test)]
    pub(crate) fn maybe_inbound_listener_evidence(&self) -> Option<&InboundListenerEvidence> {
        self.maybe_inbound_listener_evidence.as_ref()
    }

    pub fn metrics_status(&self) -> MetricsStatus {
        let Some(store) = self.maybe_metrics_store.as_ref() else {
            return MetricsStatus::default();
        };
        store
            .load_metrics_status(MetricRetentionPolicy::default())
            .unwrap_or_else(|error| {
                MetricsStatus::unavailable(
                    MetricRetentionPolicy::default(),
                    format!("metrics history unavailable: {error}"),
                )
            })
    }

    pub fn record_inbound_admission(
        &mut self,
        peer_id: u64,
        remote_endpoint: String,
        is_shutdown_requested: bool,
    ) -> Result<InboundAdmissionDecision, ManagedNetworkAuthorityError> {
        let mut request = InboundAdmissionRequest::ordinary(peer_id, remote_endpoint);
        request.is_shutdown_requested = is_shutdown_requested;
        self.network.admit_inbound_peer(request)
    }

    pub fn record_inbound_admission_for_remote_addr(
        &mut self,
        peer_id: u64,
        remote_addr: SocketAddr,
        is_shutdown_requested: bool,
    ) -> Result<InboundAdmissionDecision, ManagedNetworkAuthorityError> {
        let permission_decision = self.permission_decision_for_remote_addr(remote_addr);
        let mut request = InboundAdmissionRequest::from_permission_decision(
            peer_id,
            remote_addr.to_string(),
            permission_decision,
        );
        request.is_shutdown_requested = is_shutdown_requested;
        self.network.admit_inbound_peer(request)
    }

    pub fn permission_decision_for_remote_addr(
        &self,
        remote_addr: SocketAddr,
    ) -> InboundPermissionDecision {
        self.permission_classes.resolve_inbound(remote_addr.ip())
    }

    #[cfg(test)]
    pub(crate) fn receive_inbound_wire_message(
        &mut self,
        peer_id: u64,
        message: WireNetworkMessage,
        timestamp: i64,
    ) -> Result<Vec<EncodedWireResponse>, ManagedNetworkAuthorityError> {
        let responses = self.receive_network_message(peer_id, message, timestamp)?;
        self.encode_wire_responses(responses)
    }

    #[cfg(test)]
    pub(crate) fn encode_wire_responses(
        &self,
        responses: Vec<WireNetworkMessage>,
    ) -> Result<Vec<EncodedWireResponse>, ManagedNetworkAuthorityError> {
        responses
            .into_iter()
            .map(|message| {
                Ok(EncodedWireResponse {
                    bytes: self
                        .network
                        .encode_pending_network_message(0, &message)
                        .map_err(|error| {
                            ManagedNetworkAuthorityError::LifecycleEffect(error.to_string())
                        })?,
                    message,
                    maybe_block_serve_intent: None,
                    maybe_tx_write_capability: None,
                })
            })
            // Phase 123 carrier-shape anchor: EncodedWireResponse { message, bytes }
            .collect()
    }

    pub(crate) fn acknowledge_wire_message_written(
        &self,
        message: &WireNetworkMessage,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.network.acknowledge_wire_message_written(message)
    }

    #[cfg(test)]
    pub(crate) fn block_served_write_count(&self) -> Result<u64, ManagedNetworkAuthorityError> {
        self.network.block_served_write_count()
    }

    pub fn add_inbound_peer(&mut self, peer_id: u64) -> Result<(), ManagedNetworkAuthorityError> {
        self.network.add_inbound_peer(peer_id)
    }

    pub fn disconnect_peer(&mut self, peer_id: u64) -> Result<(), ManagedNetworkAuthorityError> {
        self.network.disconnect_peer(peer_id)
    }

    pub fn connect_outbound_peer(
        &mut self,
        peer_id: u64,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkAuthorityError> {
        self.network.connect_outbound_peer(peer_id, timestamp)
    }

    pub fn receive_network_message(
        &mut self,
        peer_id: u64,
        message: WireNetworkMessage,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkAuthorityError> {
        Ok(self
            .network
            .receive_message(
                peer_id,
                message,
                timestamp,
                self.verify_flags,
                self.consensus_params,
            )?
            .outbound)
    }

    pub fn connect_local_block(
        &mut self,
        block: &Block,
    ) -> Result<open_bitcoin_node::core::chainstate::ChainPosition, ManagedNetworkAuthorityError>
    {
        let position =
            self.network
                .connect_local_block(block, self.verify_flags, self.consensus_params)?;
        if let WalletState::DurableNamedRegistry { store, .. } = &self.wallet_state {
            store
                .save_block(block, PersistMode::Sync)
                .map_err(|error| {
                    ManagedNetworkAuthorityError::LifecycleEffect(error.to_string())
                })?;
        }
        Ok(position)
    }

    #[allow(deprecated)] // Wallet still uses the AdmissionResult path.
    pub fn submit_local_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<AdmissionResult, ManagedNetworkAuthorityError> {
        self.network
            .submit_local_transaction(transaction, self.verify_flags, self.consensus_params)
    }

    /// Explicit-time local submission; relay intent follows activation.
    pub fn submit_local_transaction_with_relay_evidence_at(
        &mut self,
        transaction: Transaction,
        now_unix_seconds: i64,
    ) -> Result<MempoolOutcome, ManagedNetworkAuthorityError> {
        let relay_intent = if self.network.relay_activation_enabled()? {
            RelayIntent::Requested
        } else {
            RelayIntent::NotRequested
        };
        let outcome = self.network.submit_local_transaction_outcome_at(
            transaction,
            self.verify_flags,
            self.consensus_params,
            now_unix_seconds,
            relay_intent,
        )?;
        if matches!(
            outcome,
            MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. }
        ) {
            for emission in self.network.drain_tx_fanout_emissions(now_unix_seconds)? {
                let capability = emission.into_parts().2;
                let _ignored = self.network.abort_peer_emission(capability);
            }
        }
        Ok(outcome)
    }

    /// Local package dry-run; relay intent follows activation.
    pub fn dry_run_local_package(
        &self,
        transactions: Vec<Transaction>,
        now_unix_seconds: i64,
    ) -> Result<PackageReport, ManagedNetworkAuthorityError> {
        let relay_intent = self.local_relay_intent()?;
        self.network.dry_run_local_package(
            transactions,
            self.verify_flags,
            self.consensus_params,
            now_unix_seconds,
            relay_intent,
        )
    }

    /// Local package submit; relay intent follows activation.
    pub fn submit_local_package(
        &self,
        transactions: Vec<Transaction>,
        now_unix_seconds: i64,
    ) -> Result<SubmittedPackageResult, ManagedNetworkAuthorityError> {
        let relay_intent = self.local_relay_intent()?;
        self.network.submit_local_package(
            transactions,
            self.verify_flags,
            self.consensus_params,
            now_unix_seconds,
            relay_intent,
        )
    }

    fn local_relay_intent(&self) -> Result<RelayIntent, ManagedNetworkAuthorityError> {
        if self.network.relay_activation_enabled()? {
            Ok(RelayIntent::Requested)
        } else {
            Ok(RelayIntent::NotRequested)
        }
    }

    pub(crate) fn package_relay_disabled(&self) -> Result<bool, ManagedNetworkAuthorityError> {
        Ok(
            matches!(self.local_relay_intent()?, RelayIntent::NotRequested)
                || !self.network_info()?.relay,
        )
    }

    pub fn mempool_entry_metadata(
        &self,
        txid: &Txid,
    ) -> Result<Option<MempoolEntryMetadata>, ManagedNetworkAuthorityError> {
        self.network.mempool_entry_metadata(txid)
    }

    pub fn latest_local_submission_evidence(
        &self,
    ) -> Result<Option<LocalRelaySubmissionEvidence>, ManagedNetworkAuthorityError> {
        self.network.latest_local_submission_evidence()
    }
}

pub(super) fn default_verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::STRICTENC
        | ScriptVerifyFlags::DERSIG
        | ScriptVerifyFlags::LOW_S
        | ScriptVerifyFlags::NULLDUMMY
        | ScriptVerifyFlags::SIGPUSHONLY
        | ScriptVerifyFlags::MINIMALDATA
        | ScriptVerifyFlags::CLEANSTACK
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
        | ScriptVerifyFlags::WITNESS
        | ScriptVerifyFlags::MINIMALIF
        | ScriptVerifyFlags::NULLFAIL
        | ScriptVerifyFlags::WITNESS_PUBKEYTYPE
        | ScriptVerifyFlags::TAPROOT
}

pub(super) fn network_magic(chain: AddressNetwork) -> NetworkMagic {
    match chain {
        AddressNetwork::Mainnet => NetworkMagic::MAINNET,
        AddressNetwork::Testnet => NetworkMagic::from_bytes([0x0b, 0x11, 0x09, 0x07]),
        AddressNetwork::Signet => NetworkMagic::from_bytes([0x0a, 0x03, 0xcf, 0x40]),
        AddressNetwork::Regtest => NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
    }
}

pub(super) fn default_p2p_port(chain: AddressNetwork) -> u16 {
    match chain {
        AddressNetwork::Mainnet => 8333,
        AddressNetwork::Testnet => 18_333,
        AddressNetwork::Signet => 38_333,
        AddressNetwork::Regtest => 18_444,
    }
}
