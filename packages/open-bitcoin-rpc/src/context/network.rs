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

use open_bitcoin_network::{
    InboundAdmissionDecision, InboundAdmissionPolicy, InboundAdmissionRequest,
    InboundListenerConfig, InboundPermissionDecision, ReconnectSuppressionInput,
};
use open_bitcoin_node::core::chainstate::ChainstateSnapshot;
use open_bitcoin_node::core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_node::core::mempool::PolicyConfig;
use open_bitcoin_node::core::mempool::{AdmissionResult, MempoolOutcome};
use open_bitcoin_node::core::network::{LocalPeerConfig, ServiceFlags, WireNetworkMessage};
use open_bitcoin_node::core::primitives::{Block, NetworkAddress, NetworkMagic, Transaction};
use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_node::network::{
    LocalRelaySubmissionEvidence, ManagedInboundAdmissionInfo, ManagedMempoolInfo,
    ManagedNetworkInfo,
};
use open_bitcoin_node::status::relay_evidence::RelayEvidenceStatus;
use open_bitcoin_node::{DurableSyncState, FjallNodeStore, MetricRetentionPolicy, MetricsStatus};
use open_bitcoin_node::{
    ManagedNetworkError, ManagedPeerNetwork, ManagedWallet, MemoryChainstateStore,
    MemoryWalletStore,
};

use crate::{config::RuntimeConfig, inbound_listener::InboundListenerEvidence};

use super::ManagedRpcContext;
use super::address_boundary::local_advertisement_decisions;
use super::wallet_state::build_wallet_state_with_store;

impl ManagedRpcContext {
    pub fn new(
        chain: AddressNetwork,
        consensus_params: ConsensusParams,
        verify_flags: ScriptVerifyFlags,
        network: ManagedPeerNetwork<MemoryChainstateStore>,
        wallet: ManagedWallet<MemoryWalletStore>,
    ) -> Self {
        Self {
            chain,
            consensus_params,
            verify_flags,
            network,
            permission_classes: Default::default(),
            inbound_permission_validation_failures: 0,
            inbound_listener_config: InboundListenerConfig::default(),
            maybe_inbound_listener_evidence: None,
            maybe_resource_governance_log_dir: None,
            resource_governance_log_retention: Default::default(),
            resource_governance_log_write_failures: 0,
            maybe_metrics_store: None,
            maybe_durable_sync_state: None,
            maybe_daemon_sync_control: None,
            wallet_state: super::wallet_state::WalletState::Local(wallet),
        }
    }

    pub fn from_runtime_config(config: &RuntimeConfig) -> Self {
        Self::from_runtime_config_with_store(config, None)
    }

    pub fn from_runtime_config_with_store(
        config: &RuntimeConfig,
        maybe_store: Option<FjallNodeStore>,
    ) -> Self {
        let consensus_params = ConsensusParams {
            coinbase_maturity: config.wallet.coinbase_maturity,
            ..ConsensusParams::default()
        };
        let local_config = LocalPeerConfig {
            magic: network_magic(config.chain),
            services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
            address: NetworkAddress {
                services: 0,
                address_bytes: [0_u8; 16],
                port: default_p2p_port(config.chain),
            },
            nonce: 0,
            relay: true,
            user_agent: "/open-bitcoin:0.1.0/".to_string(),
        };
        let mut managed_network = ManagedPeerNetwork::new_with_relay_activation(
            MemoryChainstateStore::default(),
            local_config,
            PolicyConfig::default(),
            config.relay,
            config.inbound.enabled,
        );
        managed_network.set_inbound_admission_policy(InboundAdmissionPolicy::new(
            config.inbound.max_peers,
            config.inbound.reserved_slots,
        ));
        let maybe_resource_governance_log_dir =
            config.maybe_data_dir.as_ref().map(|dir| dir.join("logs"));
        match build_wallet_state_with_store(config, maybe_store.clone()) {
            super::wallet_state::WalletState::Local(wallet) => Self {
                chain: config.chain,
                consensus_params,
                verify_flags: default_verify_flags(),
                network: managed_network,
                permission_classes: config.inbound.permission_classes.clone(),
                inbound_permission_validation_failures: config
                    .inbound_permission_validation_failures,
                inbound_listener_config: config.inbound.clone(),
                maybe_inbound_listener_evidence: None,
                maybe_resource_governance_log_dir: maybe_resource_governance_log_dir.clone(),
                resource_governance_log_retention: Default::default(),
                resource_governance_log_write_failures: 0,
                maybe_metrics_store: maybe_store.clone(),
                maybe_durable_sync_state: load_durable_sync_state(config, maybe_store.as_ref()),
                maybe_daemon_sync_control: None,
                wallet_state: super::wallet_state::WalletState::Local(wallet),
            },
            super::wallet_state::WalletState::DurableNamedRegistry {
                store,
                maybe_request_wallet_name,
            } => Self {
                chain: config.chain,
                consensus_params,
                verify_flags: default_verify_flags(),
                network: managed_network,
                permission_classes: config.inbound.permission_classes.clone(),
                inbound_permission_validation_failures: config
                    .inbound_permission_validation_failures,
                inbound_listener_config: config.inbound.clone(),
                maybe_inbound_listener_evidence: None,
                maybe_resource_governance_log_dir,
                resource_governance_log_retention: Default::default(),
                resource_governance_log_write_failures: 0,
                maybe_metrics_store: Some(store.clone()),
                maybe_durable_sync_state: store
                    .load_runtime_metadata()
                    .ok()
                    .flatten()
                    .and_then(|metadata| metadata.maybe_sync_state),
                maybe_daemon_sync_control: None,
                wallet_state: super::wallet_state::WalletState::DurableNamedRegistry {
                    store,
                    maybe_request_wallet_name,
                },
            },
        }
    }

    pub fn for_local_operator(network: AddressNetwork) -> Self {
        Self::from_runtime_config(&RuntimeConfig {
            chain: network,
            ..RuntimeConfig::default()
        })
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

    pub fn blockchain_snapshot(&self) -> ChainstateSnapshot {
        self.network.chainstate_snapshot()
    }

    pub fn maybe_chain_tip(&self) -> Option<open_bitcoin_node::core::chainstate::ChainPosition> {
        self.network.maybe_chain_tip()
    }

    pub fn maybe_durable_sync_state(&self) -> Option<&DurableSyncState> {
        self.maybe_durable_sync_state.as_ref()
    }

    pub fn set_metrics_store(&mut self, store: FjallNodeStore) {
        self.maybe_metrics_store = Some(store);
    }

    pub fn mempool_info(&self) -> ManagedMempoolInfo {
        self.network.mempool_info()
    }

    pub fn network_info(&self) -> ManagedNetworkInfo {
        self.network.network_info()
    }

    pub fn relay_evidence_status(&self) -> RelayEvidenceStatus {
        self.network.relay_evidence_status()
    }

    pub fn inbound_admission_info(&self) -> ManagedInboundAdmissionInfo {
        self.network.inbound_admission_info().clone()
    }

    pub fn set_inbound_listener_evidence(&mut self, evidence: InboundListenerEvidence) {
        let services = ServiceFlags::from_bits(self.network_info().local_services_bits);
        let decisions =
            local_advertisement_decisions(&self.inbound_listener_config, &evidence, services);
        self.network.set_local_address_decisions(decisions);
        self.maybe_inbound_listener_evidence = Some(evidence);
    }

    pub fn reconnect_suppression_input_for_remote_addr(
        &self,
        remote_addr: SocketAddr,
        now_unix_seconds: i64,
    ) -> ReconnectSuppressionInput {
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
    ) -> InboundAdmissionDecision {
        let mut request = InboundAdmissionRequest::ordinary(peer_id, remote_endpoint);
        request.is_shutdown_requested = is_shutdown_requested;
        self.network.admit_inbound_peer(request)
    }

    pub fn record_inbound_admission_for_remote_addr(
        &mut self,
        peer_id: u64,
        remote_addr: SocketAddr,
        is_shutdown_requested: bool,
    ) -> InboundAdmissionDecision {
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

    pub fn receive_inbound_wire_message(
        &mut self,
        peer_id: u64,
        message: WireNetworkMessage,
        timestamp: i64,
    ) -> Result<Vec<Vec<u8>>, ManagedNetworkError> {
        let responses = self.receive_network_message(peer_id, message, timestamp)?;
        self.network.encode_messages(&responses)
    }

    pub fn add_inbound_peer(&mut self, peer_id: u64) -> Result<(), ManagedNetworkError> {
        self.network.add_inbound_peer(peer_id)
    }

    pub fn disconnect_peer(&mut self, peer_id: u64) -> Result<(), ManagedNetworkError> {
        self.network.disconnect_peer(peer_id)
    }

    pub fn connect_outbound_peer(
        &mut self,
        peer_id: u64,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        self.network.connect_outbound_peer(peer_id, timestamp)
    }

    pub fn receive_network_message(
        &mut self,
        peer_id: u64,
        message: WireNetworkMessage,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        self.network.receive_message(
            peer_id,
            message,
            timestamp,
            self.verify_flags,
            self.consensus_params,
        )
    }

    pub fn connect_local_block(
        &mut self,
        block: &Block,
    ) -> Result<open_bitcoin_node::core::chainstate::ChainPosition, ManagedNetworkError> {
        self.network
            .connect_local_block(block, self.verify_flags, self.consensus_params)
    }

    pub fn submit_local_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<AdmissionResult, ManagedNetworkError> {
        self.network
            .submit_local_transaction(transaction, self.verify_flags, self.consensus_params)
    }

    pub fn submit_local_transaction_with_relay_evidence(
        &mut self,
        transaction: Transaction,
    ) -> Result<MempoolOutcome, ManagedNetworkError> {
        self.network.submit_local_transaction_outcome(
            transaction,
            self.verify_flags,
            self.consensus_params,
        )
    }

    pub fn latest_local_submission_evidence(&self) -> Option<LocalRelaySubmissionEvidence> {
        self.network.latest_local_submission_evidence()
    }
}

fn load_durable_sync_state(
    config: &RuntimeConfig,
    maybe_store: Option<&FjallNodeStore>,
) -> Option<DurableSyncState> {
    let store;
    let store = match maybe_store {
        Some(store) => store,
        None => {
            let data_dir = config.maybe_data_dir.as_ref()?;
            store = FjallNodeStore::open(data_dir).ok()?;
            &store
        }
    };
    let metadata = store.load_runtime_metadata().ok()??;
    metadata.maybe_sync_state
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
