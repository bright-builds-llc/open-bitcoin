// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Configured local bootstrap consumes proof; activation still requires history.

use super::*;
#[cfg(test)]
use open_bitcoin_node::chainstate::BasicFilterStartupMode;
use open_bitcoin_node::{
    core::consensus::CanonicalChain,
    storage::{
        OpenedCanonicalGenesis,
        chain_compatibility::{
            CompatibilityRefusal, DatadirObservation, inspect_datadir_without_backend,
        },
    },
};

fn compatibility_refusal(
    runtime: &RuntimeConfig,
    chain: CanonicalChain,
) -> DaemonSyncPreflightError {
    let observation = runtime
        .maybe_data_dir
        .as_ref()
        .map_or(DatadirObservation::Missing, |directory| {
            inspect_datadir_without_backend(directory)
        });
    DaemonSyncPreflightError::new(
        CompatibilityRefusal::from_observation(chain, observation).to_string(),
    )
}

fn preflight(runtime: &RuntimeConfig) -> Result<CanonicalChain, DaemonSyncPreflightError> {
    runtime
        .validate_peer_block_filters()
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
    let chain = selected_canonical_chain(runtime)
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
    let activation = runtime.sync.is_enabled()
        || runtime.inbound.enabled
        || runtime.relay.enabled
        || runtime.block_serving.block_serving.enabled
        || runtime.block_serving.compact_relay.enabled;
    if activation {
        chain
            .spec()
            .require_ordinary_history()
            .map_err(|_| compatibility_refusal(runtime, chain))?;
    }
    Ok(chain)
}

pub(super) fn open_runtime_store(
    runtime: &RuntimeConfig,
) -> Result<Option<OpenedCanonicalGenesis>, DaemonSyncPreflightError> {
    let chain = preflight(runtime)?;
    let has_durable_trigger = runtime.sync.is_enabled()
        || runtime.inbound.enabled
        || runtime.prune_mode != open_bitcoin_node::core::chainstate::PruneMode::Disabled
        || runtime.block_filter_index.is_explicit();
    if !has_durable_trigger {
        return Ok(None);
    }
    let Some(data_dir) = runtime.maybe_data_dir.as_ref() else {
        return Err(compatibility_refusal(runtime, chain));
    };
    FjallNodeStore::open_canonical_genesis(data_dir, chain)
        .map(Some)
        .map_err(|_| compatibility_refusal(runtime, chain))
}

pub(super) fn open_authoritative_network_runtime(
    runtime: &RuntimeConfig,
    maybe_opened: Option<OpenedCanonicalGenesis>,
) -> Result<OpenedAuthoritativeRuntime, DaemonSyncPreflightError> {
    let chain = preflight(runtime)?;
    let Some(opened) = maybe_opened else {
        // There is no earned transient canonical owner in this phase.
        return Err(compatibility_refusal(runtime, chain));
    };
    let (context, sync_runtime) =
        ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)
            .map_err(|_| compatibility_refusal(runtime, chain))?;
    match assemble_durable_runtime(runtime, sync_runtime)? {
        OpenedAuthoritativeRuntime::Durable(mut authoritative) => {
            authoritative.maybe_prebuilt_context = Some(context);
            Ok(OpenedAuthoritativeRuntime::Durable(authoritative))
        }
        #[cfg(test)]
        OpenedAuthoritativeRuntime::Transient(_) => Err(compatibility_refusal(runtime, chain)),
    }
}

#[cfg(test)]
pub(super) fn basic_filter_startup_mode(runtime: &RuntimeConfig) -> BasicFilterStartupMode {
    if runtime.block_filter_index.is_enabled() {
        BasicFilterStartupMode::Enabled
    } else {
        BasicFilterStartupMode::Disabled
    }
}

pub(super) fn assemble_durable_runtime(
    runtime: &RuntimeConfig,
    sync_runtime: DurableSyncRuntime,
) -> Result<OpenedAuthoritativeRuntime, DaemonSyncPreflightError> {
    let mut network = sync_runtime.network_handle();
    network
        .set_prune_network(runtime.sync.runtime.network)
        .map_err(|error| {
            DaemonSyncPreflightError::new(format!("failed to configure prune network: {error}"))
        })?;
    network
        .set_prune_mode(runtime.prune_mode)
        .map_err(|error| {
            DaemonSyncPreflightError::new(format!("failed to configure prune mode: {error}"))
        })?;
    network
        .configure_basic_peer_serving(runtime.peer_block_filters)
        .map_err(|error| {
            DaemonSyncPreflightError::new(format!(
                "failed to configure BASIC peer serving: {error}"
            ))
        })?;
    Ok(OpenedAuthoritativeRuntime::Durable(
        AuthoritativeNetworkRuntime {
            maybe_prebuilt_context: None,
            network,
            peer_identity_authority: sync_runtime.peer_identity_authority(),
            announcement_outboxes: sync_runtime.announcement_outboxes(),
            maybe_sync_runtime: Some(sync_runtime),
        },
    ))
}
