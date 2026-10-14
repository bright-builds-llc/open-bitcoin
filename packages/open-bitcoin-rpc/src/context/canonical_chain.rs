// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_node::{
    FjallNodeStore, ManagedNetworkAuthorityError, StorageError,
    core::{
        consensus::{CanonicalChain, UnsupportedValidationScope},
        wallet::AddressNetwork,
    },
    storage::chain_compatibility::CompatibilityRefusal,
};

use super::ManagedRpcContext;
use super::{
    network::{default_p2p_port, network_magic},
    wallet_state::{WalletState, build_fixture_wallet_state_with_store},
};
use crate::config::{RuntimeConfig, selected_canonical_chain};
use open_bitcoin_network::InboundListenerConfig;
use open_bitcoin_node::ManagedNetworkHandle;
use open_bitcoin_node::core::{
    consensus::{ConsensusParams, ScriptVerifyFlags},
    mempool::PolicyConfig,
    network::{LocalPeerConfig, ServiceFlags},
    primitives::NetworkAddress,
};
use open_bitcoin_node::{
    ManagedPeerNetwork, ManagedWallet, MemoryChainstateStore, MemoryWalletStore,
};

/// Ordinary context failures retain typed causes and display bounded diagnostics.
#[derive(Debug)]
pub enum CanonicalContextError {
    CompatibilityRefusal(CompatibilityRefusal),
    UnsupportedValidationScope(UnsupportedValidationScope),
    UnprovedOwner,
    ChainMismatch,
    Storage(StorageError),
    Authority(ManagedNetworkAuthorityError),
}

impl core::fmt::Display for CanonicalContextError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CompatibilityRefusal(error) => error.fmt(formatter),
            Self::UnsupportedValidationScope(error) => error.fmt(formatter),
            Self::UnprovedOwner => {
                formatter.write_str("unproved_owner: canonical ownership is unavailable")
            }
            Self::ChainMismatch => formatter
                .write_str("chain_mismatch: configured selectors and canonical owner must agree"),
            Self::Storage(_) => formatter.write_str("context_storage_unavailable"),
            Self::Authority(_) => formatter.write_str("context_authority_unavailable"),
        }
    }
}

impl std::error::Error for CanonicalContextError {}

impl From<StorageError> for CanonicalContextError {
    fn from(error: StorageError) -> Self {
        Self::Storage(error)
    }
}

impl From<ManagedNetworkAuthorityError> for CanonicalContextError {
    fn from(error: ManagedNetworkAuthorityError) -> Self {
        Self::Authority(error)
    }
}

impl From<CompatibilityRefusal> for CanonicalContextError {
    fn from(error: CompatibilityRefusal) -> Self {
        Self::CompatibilityRefusal(error)
    }
}

pub(super) fn selected_chain(
    config: &RuntimeConfig,
) -> Result<CanonicalChain, CanonicalContextError> {
    selected_canonical_chain(config).map_err(|_| CanonicalContextError::ChainMismatch)
}

pub(super) fn require_ordinary_scope(chain: CanonicalChain) -> Result<(), CanonicalContextError> {
    chain
        .spec()
        .require_ordinary_history()
        .map_err(CanonicalContextError::UnsupportedValidationScope)
}

impl ManagedRpcContext {
    /// Construct an ordinary context only after canonical validation qualification.
    pub fn try_from_runtime_config(config: &RuntimeConfig) -> Result<Self, CanonicalContextError> {
        Self::try_from_runtime_config_with_store(config, None)
    }

    /// Supplied raw storage is not evidence of ordinary canonical support.
    pub fn try_from_runtime_config_with_store(
        config: &RuntimeConfig,
        maybe_store: Option<FjallNodeStore>,
    ) -> Result<Self, CanonicalContextError> {
        let chain = selected_chain(config)?;
        require_ordinary_scope(chain)?;
        let wallet_state =
            super::wallet_state::build_wallet_state_with_store(config, maybe_store.clone())?;
        let (network, maybe_store) =
            Self::build_context_network(config, maybe_store, &wallet_state)?;
        let network =
            open_bitcoin_node::ManagedNetworkHandle::from_canonical_network(network, chain);
        Self::from_runtime_config_with_network_handle(config, network, maybe_store)
    }

    /// Fallible ordinary alias; fixture callers must explicitly select their evidence scope.
    pub fn from_runtime_config(config: &RuntimeConfig) -> Result<Self, CanonicalContextError> {
        Self::try_from_runtime_config(config)
    }

    /// Fallible ordinary alias preserving typed compatibility and storage failures.
    pub fn from_runtime_config_with_store(
        config: &RuntimeConfig,
        maybe_store: Option<FjallNodeStore>,
    ) -> Result<Self, CanonicalContextError> {
        Self::try_from_runtime_config_with_store(config, maybe_store)
    }

    /// Ordinary local operator construction obeys both configured selectors and support.
    pub fn for_local_operator(network: AddressNetwork) -> Result<Self, CanonicalContextError> {
        Self::try_from_runtime_config(&RuntimeConfig {
            chain: network,
            ..RuntimeConfig::default()
        })
    }
}

#[cfg(test)]
mod tests;

impl ManagedRpcContext {
    /// Low-level synthetic/source-derived composition; never ordinary canonical authorization.
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
            network: ManagedNetworkHandle::from_network_fixture(network),
            permission_classes: Default::default(),
            inbound_permission_validation_failures: 0,
            inbound_listener_config: InboundListenerConfig::default(),
            maybe_inbound_listener_evidence: None,
            maybe_resource_governance_log_dir: None,
            resource_governance_log_retention: Default::default(),
            resource_governance_log_write_failures: 0,
            maybe_block_source: None,
            maybe_metrics_store: None,
            maybe_runtime_metadata_source: None,
            maybe_daemon_sync_control: None,
            manual_prune: open_bitcoin_node::status::ManualPruneSurface::None,
            wallet_state: super::wallet_state::WalletState::Local(wallet),
        }
    }

    #[allow(
        clippy::expect_used,
        reason = "explicit memory fixture construction has no durable migration boundary"
    )]
    /// Synthetic/source-derived regression evidence only; never ordinary canonical authorization.
    pub fn from_fixture_runtime_config(config: &RuntimeConfig) -> Self {
        Self::from_fixture_runtime_config_with_store(config, None)
            .expect("memory-only context has no durable migration boundary")
    }

    /// Synthetic/source-derived regression evidence only; never ordinary canonical authorization.
    pub fn from_fixture_runtime_config_with_store(
        config: &RuntimeConfig,
        maybe_store: Option<FjallNodeStore>,
    ) -> Result<Self, StorageError> {
        let wallet_state = build_fixture_wallet_state_with_store(config, maybe_store.clone())?;
        let (network, maybe_store) =
            Self::build_context_network(config, maybe_store, &wallet_state)?;
        Self::assemble_fixture_context(
            config,
            ManagedNetworkHandle::from_network_fixture(network),
            maybe_store,
            wallet_state,
        )
    }

    pub(super) fn build_context_network(
        config: &RuntimeConfig,
        maybe_store: Option<FjallNodeStore>,
        wallet_state: &WalletState,
    ) -> Result<
        (
            ManagedPeerNetwork<MemoryChainstateStore>,
            Option<FjallNodeStore>,
        ),
        StorageError,
    > {
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
        let policy = PolicyConfig::default();
        let effective_store = match wallet_state {
            super::wallet_state::WalletState::Local(_) => maybe_store,
            super::wallet_state::WalletState::DurableNamedRegistry { store, .. } => {
                Some(store.clone())
            }
        };
        let durable_chainstate = match effective_store.as_ref() {
            Some(store) => store.wallet_scan_chainstate_snapshot()?,
            None => None,
        };
        let chainstate_store = durable_chainstate.map_or_else(
            MemoryChainstateStore::default,
            MemoryChainstateStore::from_snapshot,
        );
        let managed_network = ManagedPeerNetwork::new_with_block_relay_activation(
            chainstate_store,
            local_config,
            policy,
            config.relay,
            config.block_serving,
            config.inbound.enabled,
        );
        Ok((managed_network, effective_store))
    }

    /// Synthetic/source-derived regression evidence only; never ordinary canonical authorization.
    pub fn for_fixture_local_operator(network: AddressNetwork) -> Self {
        Self::from_fixture_runtime_config(&RuntimeConfig {
            chain: network,
            ..RuntimeConfig::default()
        })
    }
}
