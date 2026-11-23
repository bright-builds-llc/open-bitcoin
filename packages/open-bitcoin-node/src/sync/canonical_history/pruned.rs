// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::{
    BasicFilterStartupMode, DurableSyncRuntime, ManagedChainstate, ManagedPeerNetwork, SyncNetwork,
    SyncRuntimeConfig, SyncRuntimeError, invalid, progress,
};

impl DurableSyncRuntime {
    /// Reopen a pruned Regtest store using a separately held complete replay archive.
    /// The runtime owns only the proved retained store after startup.
    pub fn reopen_pruned_regtest_with_archive(
        retained: impl AsRef<std::path::Path>,
        archive: impl AsRef<std::path::Path>,
        config: SyncRuntimeConfig,
        block_serving: open_bitcoin_network::BlockServingActivationConfig,
        inbound_enabled: bool,
    ) -> Result<Self, SyncRuntimeError> {
        use open_bitcoin_core::{chainstate::PruneMode, consensus::CanonicalChain};
        if config.network != SyncNetwork::Regtest {
            return Err(invalid("pruned replay archive requires Regtest"));
        }
        let (store, owners) =
            crate::FjallNodeStore::open_pruned_regtest_with_archive(retained, archive)
                .and_then(|opened| opened.into_parts())
                .map_err(|error| invalid(&error.to_string()))?;
        let (managed, genesis, headers) = ManagedChainstate::from_replayed_history(
            store.clone(),
            owners,
            BasicFilterStartupMode::Enabled,
        )?;
        let params =
            open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&genesis);
        let mut network = ManagedPeerNetwork::from_initialized_chainstate(
            managed,
            progress::local_peer_config(&config),
            open_bitcoin_mempool::PolicyConfig::default(),
            config.max_blocks_in_flight_per_peer,
            open_bitcoin_network::RelayActivationConfig::default(),
            open_bitcoin_network::BlockRelayActivationPolicy {
                block_serving,
                ..Default::default()
            },
            inbound_enabled,
        )
        .into_canonical_scope(CanonicalChain::Regtest);
        network.peer_manager_mut().seed_header_store(headers);
        network.record_basic_indexing_configuration(
            store.resolved_basic_filter_indexing_after_startup()?,
        );
        network.set_prune_mode(PruneMode::ManualOnly);
        network.set_serving_have_pruned(true);
        let mut runtime = Self::finish_initialized_runtime(store, network, config, params)?;
        runtime.network.set_prune_network(SyncNetwork::Regtest)?;
        runtime.pruned_regtest_serving = true;
        Ok(runtime)
    }
}
