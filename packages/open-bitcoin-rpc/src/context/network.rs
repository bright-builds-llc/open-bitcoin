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
    InboundPermissionDecision,
};
use open_bitcoin_node::core::chainstate::ChainstateSnapshot;
use open_bitcoin_node::core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_node::core::mempool::PolicyConfig;
use open_bitcoin_node::core::network::{LocalPeerConfig, ServiceFlags, WireNetworkMessage};
use open_bitcoin_node::core::primitives::{Block, NetworkAddress, NetworkMagic, Transaction};
use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_node::network::{
    ManagedInboundAdmissionInfo, ManagedMempoolInfo, ManagedNetworkInfo,
};
use open_bitcoin_node::status::{
    FieldAvailability, INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON, InboundAdmissionEvent,
    InboundHandshakeStatusCounts, InboundPeerServingStatus, InboundPermissionDecisionEvent,
    InboundPermissionEvidence, inbound_status_unavailable,
};
use open_bitcoin_node::{DurableSyncState, FjallNodeStore};
use open_bitcoin_node::{
    ManagedNetworkError, ManagedPeerNetwork, ManagedWallet, MemoryChainstateStore,
    MemoryWalletStore,
};

use crate::{config::RuntimeConfig, inbound_listener::InboundListenerEvidence};

use super::ManagedRpcContext;
use super::wallet_state::build_wallet_state;

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
            maybe_inbound_listener_evidence: None,
            maybe_durable_sync_state: None,
            maybe_daemon_sync_control: None,
            wallet_state: super::wallet_state::WalletState::Local(wallet),
        }
    }

    pub fn from_runtime_config(config: &RuntimeConfig) -> Self {
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
        let mut managed_network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config,
            PolicyConfig::default(),
        );
        managed_network.set_inbound_admission_policy(InboundAdmissionPolicy::new(
            config.inbound.max_peers,
            config.inbound.reserved_slots,
        ));
        match build_wallet_state(config) {
            super::wallet_state::WalletState::Local(wallet) => Self {
                chain: config.chain,
                consensus_params,
                verify_flags: default_verify_flags(),
                network: managed_network,
                permission_classes: config.inbound.permission_classes.clone(),
                maybe_inbound_listener_evidence: None,
                maybe_durable_sync_state: load_durable_sync_state(config),
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
                maybe_inbound_listener_evidence: None,
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

    pub fn mempool_info(&self) -> ManagedMempoolInfo {
        self.network.mempool_info()
    }

    pub fn network_info(&self) -> ManagedNetworkInfo {
        self.network.network_info()
    }

    pub fn inbound_admission_info(&self) -> ManagedInboundAdmissionInfo {
        self.network.inbound_admission_info().clone()
    }

    pub fn set_inbound_listener_evidence(&mut self, evidence: InboundListenerEvidence) {
        self.maybe_inbound_listener_evidence = Some(evidence);
    }

    pub fn current_inbound_status(&self) -> FieldAvailability<InboundPeerServingStatus> {
        let admission = self.inbound_admission_info();
        let maybe_listener_evidence = self.maybe_inbound_listener_evidence.as_ref();
        if admission.admitted_inbound_peers == 0
            && admission.rejected_inbound_peers == 0
            && maybe_listener_evidence.is_none()
        {
            return inbound_status_unavailable();
        }

        let network_info = self.network_info();
        let permission_evidence = inbound_permission_evidence(&admission);
        FieldAvailability::available(InboundPeerServingStatus {
            listener_state: listener_state(&admission, maybe_listener_evidence),
            bound_endpoints: bound_endpoints(maybe_listener_evidence),
            preflight_reason: preflight_reason(&admission, maybe_listener_evidence),
            admitted_inbound_peers: usize_to_u32(admission.admitted_inbound_peers),
            rejected_inbound_peers: usize_to_u32(admission.rejected_inbound_peers),
            handshake: InboundHandshakeStatusCounts {
                awaiting_version: 0,
                awaiting_verack: 0,
                established: usize_to_u32(network_info.inbound_peers),
                disconnected: 0,
            },
            duplicate_rejects: usize_to_u32(
                admission.duplicate_endpoint_rejections + admission.duplicate_peer_id_rejections,
            ),
            self_connection_rejects: usize_to_u32(admission.self_connection_rejections),
            cap_rejects: usize_to_u32(admission.cap_rejections),
            reserved_slot_rejects: usize_to_u32(admission.reserved_slot_rejections),
            latest_admission_event: latest_inbound_admission_event(
                &admission,
                maybe_listener_evidence,
            ),
            permissioned_inbound_peers: usize_to_u32(admission.permissioned_inbound_admits),
            protected_inbound_peers: usize_to_u32(admission.protected_inbound_admits),
            permission_class: permission_evidence.permission_class,
            active_permission_effects: permission_evidence.active_permission_effects,
            inactive_permission_effects: permission_evidence.inactive_permission_effects,
            latest_permission_decision: latest_inbound_permission_decision(&admission),
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
    ) -> Result<open_bitcoin_node::core::mempool::AdmissionResult, ManagedNetworkError> {
        self.network
            .submit_local_transaction(transaction, self.verify_flags, self.consensus_params)
    }
}

fn listener_state(
    admission: &ManagedInboundAdmissionInfo,
    maybe_listener_evidence: Option<&InboundListenerEvidence>,
) -> String {
    maybe_listener_evidence
        .map(|evidence| evidence.listener_state.clone())
        .unwrap_or_else(|| {
            if admission.admitted_inbound_peers > 0 || admission.rejected_inbound_peers > 0 {
                "listening".to_string()
            } else {
                "unavailable".to_string()
            }
        })
}

fn bound_endpoints(maybe_listener_evidence: Option<&InboundListenerEvidence>) -> Vec<String> {
    maybe_listener_evidence
        .map(|evidence| evidence.bound_endpoints.clone())
        .unwrap_or_default()
}

fn preflight_reason(
    admission: &ManagedInboundAdmissionInfo,
    maybe_listener_evidence: Option<&InboundListenerEvidence>,
) -> String {
    maybe_listener_evidence
        .map(|evidence| evidence.preflight_reason.clone())
        .unwrap_or_else(|| {
            if admission.admitted_inbound_peers > 0 || admission.rejected_inbound_peers > 0 {
                "ready".to_string()
            } else {
                "unavailable".to_string()
            }
        })
}

fn latest_inbound_admission_event(
    admission: &ManagedInboundAdmissionInfo,
    maybe_listener_evidence: Option<&InboundListenerEvidence>,
) -> FieldAvailability<InboundAdmissionEvent> {
    if let Some(reason) = admission.maybe_latest_rejection_reason {
        let reason = reason.as_str().to_string();
        return FieldAvailability::available(InboundAdmissionEvent {
            outcome: "rejected".to_string(),
            reason: reason.clone(),
            slot_class: "ordinary".to_string(),
            message: format!("inbound admission rejected: {reason}"),
        });
    }

    if admission.admitted_inbound_peers > 0 {
        return FieldAvailability::available(InboundAdmissionEvent {
            outcome: "admitted".to_string(),
            reason: "admitted".to_string(),
            slot_class: "ordinary".to_string(),
            message: "inbound peer admitted".to_string(),
        });
    }

    if let Some(event) = maybe_listener_evidence
        .and_then(|evidence| evidence.maybe_latest_admission_event.as_deref())
    {
        return FieldAvailability::available(InboundAdmissionEvent {
            outcome: "listener".to_string(),
            reason: event.to_string(),
            slot_class: "ordinary".to_string(),
            message: format!("inbound listener event: {event}"),
        });
    }

    FieldAvailability::unavailable("no inbound admission event recorded")
}

fn inbound_permission_evidence(
    admission: &ManagedInboundAdmissionInfo,
) -> InboundPermissionEvidence {
    let mut evidence = InboundPermissionEvidence::ordinary();
    if let Some(permission_decision) = &admission.maybe_latest_permission_decision {
        evidence.permission_class = permission_decision.connection_class.as_str().to_string();
    }
    evidence.active_permission_effects = admission
        .observed_active_permission_effects
        .iter()
        .map(|effect| effect.as_str().to_string())
        .collect();
    evidence.inactive_permission_effects = admission
        .observed_inactive_permission_effects
        .iter()
        .map(|effect| effect.as_str().to_string())
        .collect();
    evidence
}

fn latest_inbound_permission_decision(
    admission: &ManagedInboundAdmissionInfo,
) -> FieldAvailability<InboundPermissionDecisionEvent> {
    let Some(permission_decision) = &admission.maybe_latest_permission_decision else {
        return FieldAvailability::unavailable(INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON);
    };

    let permission_class = permission_decision.connection_class.as_str().to_string();
    FieldAvailability::available(InboundPermissionDecisionEvent {
        outcome: "admitted".to_string(),
        reason: "admitted".to_string(),
        permission_class: permission_class.clone(),
        active_permission_effects: permission_decision
            .active_effects
            .iter()
            .map(|effect| effect.as_str().to_string())
            .collect(),
        inactive_permission_effects: permission_decision
            .inactive_effects
            .iter()
            .map(|effect| effect.as_str().to_string())
            .collect(),
        message: format!("inbound permission decision admitted as {permission_class}"),
    })
}

fn usize_to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

fn load_durable_sync_state(config: &RuntimeConfig) -> Option<DurableSyncState> {
    let data_dir = config.maybe_data_dir.as_ref()?;
    let store = FjallNodeStore::open(data_dir).ok()?;
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
