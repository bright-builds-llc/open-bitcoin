// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/wallet/wallet.cpp

use super::{CanonicalContextError, ManagedRpcContext, wallet_state::WalletState};
use crate::config::{BasicFilterIndexSetting, RuntimeConfig};
use open_bitcoin_network::InboundAdmissionPolicy;
use open_bitcoin_node::chainstate::BasicFilterStartupMode;
use open_bitcoin_node::core::chainstate::engine::canonical_genesis::consensus_params;
use open_bitcoin_node::storage::{OpenedCanonicalGenesis, coins_view::FjallCoinsView};
use open_bitcoin_node::{DurableSyncRuntime, FjallChainstateStore};

impl ManagedRpcContext<FjallChainstateStore, FjallCoinsView> {
    /// Consume proved genesis storage without granting ordinary or wallet history.
    pub fn from_canonical_genesis_runtime_config(
        config: &RuntimeConfig,
        opened: OpenedCanonicalGenesis,
    ) -> Result<(Self, DurableSyncRuntime), CanonicalContextError> {
        preflight(config, &opened)?;
        let params = consensus_params(opened.genesis());
        #[cfg(test)]
        INITIALIZATIONS.set(INITIALIZATIONS.get() + 1);
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            config.sync.runtime.clone(),
            startup_mode(config.block_filter_index),
        )
        .map_err(CanonicalContextError::GenesisRuntime)?;
        let mut network = runtime.network_handle();
        network.set_prune_network(config.sync.runtime.network)?;
        network.set_prune_mode(config.prune_mode)?;
        network.set_inbound_admission_policy(InboundAdmissionPolicy::new(
            config.inbound.max_peers,
            config.inbound.reserved_slots,
        ))?;
        network.configure_basic_peer_serving(config.peer_block_filters)?;
        let mut bound_config = config.clone();
        bound_config.maybe_data_dir = Some(runtime.store().datadir().to_path_buf());
        let mut context = Self::assemble_context(
            &bound_config,
            network,
            Some(runtime.store().clone()),
            WalletState::Unavailable,
        );
        context.consensus_params = params;
        Ok((context, runtime))
    }
}

#[cfg(test)]
thread_local! {
    static INITIALIZATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn preflight(
    config: &RuntimeConfig,
    opened: &OpenedCanonicalGenesis,
) -> Result<(), CanonicalContextError> {
    let chain = super::canonical_chain::selected_chain(config)?;
    config
        .validate_peer_block_filters()
        .map_err(|_| CanonicalContextError::InvalidGenesisConfiguration)?;
    if opened.genesis().chain() != chain {
        return Err(CanonicalContextError::ChainMismatch);
    }
    if config.sync.is_enabled()
        || config.inbound.enabled
        || config.relay.enabled
        || config.block_serving.block_serving.enabled
        || config.block_serving.compact_relay.enabled
    {
        super::canonical_chain::require_ordinary_scope(chain)?;
    }
    Ok(())
}

fn startup_mode(setting: BasicFilterIndexSetting) -> BasicFilterStartupMode {
    match setting {
        BasicFilterIndexSetting::Basic => BasicFilterStartupMode::Enabled,
        BasicFilterIndexSetting::Disabled => BasicFilterStartupMode::Disabled,
        BasicFilterIndexSetting::Unspecified => BasicFilterStartupMode::PreserveSaved,
    }
}

#[cfg(test)]
mod tests;
