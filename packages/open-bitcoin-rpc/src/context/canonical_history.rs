// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/rpc/mining.cpp

use super::{
    CanonicalContextError, CanonicalSubmissionBridge, ManagedRpcContext, wallet_state::WalletState,
};
use crate::config::{BasicFilterIndexSetting, RuntimeConfig};
use open_bitcoin_network::InboundAdmissionPolicy;
use open_bitcoin_node::chainstate::BasicFilterStartupMode;
use open_bitcoin_node::core::chainstate::engine::canonical_genesis::consensus_params;
use open_bitcoin_node::core::consensus::chain_spec::CanonicalGenesisProof;
use open_bitcoin_node::storage::coins_view::FjallCoinsView;
use open_bitcoin_node::{DurableSyncRuntime, FjallChainstateStore};

impl ManagedRpcContext<FjallChainstateStore, FjallCoinsView> {
    /// Build the authenticated context for an explicitly configured, loopback-only pruned Regtest owner.
    pub fn from_pruned_regtest_runtime_config(
        config: &RuntimeConfig,
        archive: impl AsRef<std::path::Path>,
    ) -> Result<(Self, DurableSyncRuntime), CanonicalContextError> {
        let chain = super::canonical_chain::selected_chain(config)?;
        validate_pruned_services(config, chain)?;
        let directory = config
            .maybe_data_dir
            .as_ref()
            .ok_or(CanonicalContextError::InvalidGenesisConfiguration)?;
        let spec = chain.spec();
        let genesis = CanonicalGenesisProof::prove(spec, &spec.genesis_block())
            .map_err(|_| CanonicalContextError::InvalidGenesisConfiguration)?;
        let runtime = DurableSyncRuntime::reopen_pruned_regtest_with_archive(
            directory,
            archive,
            config.sync.runtime.clone(),
            config.block_serving.block_serving,
            config.inbound.enabled,
        )
        .map_err(CanonicalContextError::GenesisRuntime)?;
        let context = assemble(config, &runtime, consensus_params(&genesis), None)?;
        Ok((context, runtime))
    }

    /// Acquire one safe selected owner and bind only its paired submission command.
    pub fn from_canonical_history_runtime_config(
        config: &RuntimeConfig,
    ) -> Result<(Self, DurableSyncRuntime), CanonicalContextError> {
        use open_bitcoin_node::core::{chainstate::PruneMode, consensus::CanonicalChain};
        let chain = preflight(config)?;
        if chain == CanonicalChain::Mainnet
            && (config.block_filter_index != BasicFilterIndexSetting::Disabled
                || config.prune_mode != PruneMode::Disabled
                || config.peer_block_filters
                || !config.rpc_server.bind_address.ip().is_loopback())
        {
            return Err(CanonicalContextError::InvalidGenesisConfiguration);
        }
        let Some(directory) = config.maybe_data_dir.as_ref() else {
            return Err(CanonicalContextError::InvalidGenesisConfiguration);
        };
        let (runtime, maybe_command) = DurableSyncRuntime::open_canonical_submission_configured(
            directory,
            config.sync.runtime.clone(),
            startup_mode(config.block_filter_index),
        )
        .map_err(CanonicalContextError::GenesisRuntime)?;
        let maybe_bridge = maybe_command.map(|command| {
            let command = std::sync::Arc::new(command);
            let reader = std::sync::Arc::clone(&command);
            #[cfg(test)]
            let observer = std::sync::Arc::clone(&command);
            CanonicalSubmissionBridge {
                active_tip: Box::new(move || {
                    // The ordinary observer returns detached facts after its guards drop.
                    reader
                        .observe_accepted_state(&[])
                        .map_err(|error| crate::RpcFailure::verify_error(error.to_string()))?
                        .fold(|(snapshot, _, _), _, _, _, _| {
                            snapshot.tip().cloned().ok_or_else(|| {
                                crate::RpcFailure::verify_error(
                                    "canonical accepted tip unavailable",
                                )
                            })
                        })
                }),
                submit: Box::new(move |wire| {
                    command
                        .submit(wire)
                        .map(|outcome| {
                            outcome.fold(
                                |_| serde_json::Value::Null,
                                |_| serde_json::json!("duplicate"),
                                |reason| {
                                    serde_json::json!(if reason.is_empty() {
                                        "rejected".to_owned()
                                    } else {
                                        reason
                                    })
                                },
                                || serde_json::json!("inconclusive"),
                            )
                        })
                        .map_err(|error| crate::RpcFailure::verify_error(error.to_string()))
                }),
                #[cfg(test)]
                observe: Box::new(move |hashes| {
                    observer
                        .observe_accepted_state(hashes)
                        .map(|facts| {
                            facts.fold(
                                |coins, maybe_progress, maybe_stored_basic, rows, provenance| {
                                    super::CanonicalObservation {
                                        coins,
                                        maybe_progress,
                                        maybe_stored_basic,
                                        rows,
                                        provenance,
                                    }
                                },
                            )
                        })
                        .map_err(|error| crate::RpcFailure::verify_error(error.to_string()))
                }),
            }
        });
        let spec = chain.spec();
        let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block())
            .map_err(|_| CanonicalContextError::InvalidGenesisConfiguration)?;
        let context = assemble(config, &runtime, consensus_params(&proof), maybe_bridge)?;
        Ok((context, runtime))
    }
}

fn validate_pruned_services(
    config: &RuntimeConfig,
    chain: open_bitcoin_node::core::consensus::CanonicalChain,
) -> Result<(), CanonicalContextError> {
    use open_bitcoin_node::core::{chainstate::PruneMode, consensus::CanonicalChain};
    config
        .validate_peer_block_filters()
        .map_err(|_| CanonicalContextError::InvalidGenesisConfiguration)?;
    if chain != CanonicalChain::Regtest
        || config.prune_mode != PruneMode::ManualOnly
        || config.block_filter_index == BasicFilterIndexSetting::Disabled
        || !config.rpc_server.bind_address.ip().is_loopback()
        || config.inbound.allow_public
        || config.relay.enabled
        || config.block_serving.compact_relay.enabled
        || !config.sync.runtime.dns_seeds.is_empty()
        || config.sync.runtime.manual_peers.iter().any(|peer| {
            !peer
                .host
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
        })
        || (config.inbound.enabled
            && config.inbound.listen_addresses.iter().any(|address| {
                !address
                    .parse::<std::net::SocketAddr>()
                    .is_ok_and(|endpoint| endpoint.ip().is_loopback())
            }))
    {
        return Err(CanonicalContextError::InvalidGenesisConfiguration);
    }
    Ok(())
}

pub(super) fn preflight(
    config: &RuntimeConfig,
) -> Result<open_bitcoin_node::core::consensus::CanonicalChain, CanonicalContextError> {
    let chain = super::canonical_chain::selected_chain(config)?;
    config
        .validate_peer_block_filters()
        .map_err(|_| CanonicalContextError::InvalidGenesisConfiguration)?;
    if config.sync.is_enabled()
        || config.inbound.enabled
        || config.relay.enabled
        || config.block_serving.block_serving.enabled
        || config.block_serving.compact_relay.enabled
    {
        super::canonical_chain::require_ordinary_scope(chain)?;
    }
    Ok(chain)
}

pub(super) fn startup_mode(setting: BasicFilterIndexSetting) -> BasicFilterStartupMode {
    match setting {
        BasicFilterIndexSetting::Basic => BasicFilterStartupMode::Enabled,
        BasicFilterIndexSetting::Disabled => BasicFilterStartupMode::Disabled,
        BasicFilterIndexSetting::Unspecified => BasicFilterStartupMode::PreserveSaved,
    }
}

pub(super) fn assemble(
    config: &RuntimeConfig,
    runtime: &DurableSyncRuntime,
    params: open_bitcoin_node::core::consensus::ConsensusParams,
    maybe_bridge: Option<CanonicalSubmissionBridge>,
) -> Result<ManagedRpcContext<FjallChainstateStore, FjallCoinsView>, CanonicalContextError> {
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
    let mut context = ManagedRpcContext::assemble_context(
        &bound_config,
        network,
        Some(runtime.store().clone()),
        WalletState::Unavailable,
    );
    context.consensus_params = params;
    context.maybe_canonical_submission = maybe_bridge;
    Ok(context)
}

#[cfg(test)]
mod tests;
