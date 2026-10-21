// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Explicit synthetic regression startup. This module exists only under cfg(test).
//! Fixture success never establishes canonical startup or ordinary store compatibility.

use crate::{
    AuthoritativeNetworkRuntime, DaemonSyncPreflightError, DurableSyncRuntime, FjallNodeStore,
    ManagedNetworkHandle, OpenedAuthoritativeRuntime, PeerIdentityAuthority,
    assemble_durable_runtime, basic_filter_startup_mode,
};
use open_bitcoin_rpc::config::RuntimeConfig;

pub(crate) fn fixture_open_authoritative_network_runtime(
    runtime: &RuntimeConfig,
    maybe_store: Option<FjallNodeStore>,
) -> Result<OpenedAuthoritativeRuntime, DaemonSyncPreflightError> {
    runtime
        .validate_peer_block_filters()
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
    if let Some(store) = maybe_store {
        let sync_runtime = DurableSyncRuntime::open_with_configured_runtime_activation(
            store,
            runtime.sync.runtime.clone(),
            runtime.relay,
            runtime.block_serving,
            runtime.inbound.enabled,
            basic_filter_startup_mode(runtime),
        )
        .map_err(|error| {
            DaemonSyncPreflightError::new(format!(
                "open-bitcoind failed to construct authoritative network runtime: {error}"
            ))
        })?;
        return assemble_durable_runtime(runtime, sync_runtime);
    }
    if runtime.block_filter_index.is_enabled() {
        return Err(DaemonSyncPreflightError::new(
            "open-bitcoind BASIC indexing requires an existing datadir and retained validated genesis history; set -datadir=<path> before enabling -blockfilterindex.",
        ));
    }
    if runtime.sync.is_enabled() {
        return Err(DaemonSyncPreflightError::new(
            "open-bitcoind mainnet sync activation requires an existing datadir; set -datadir=<path> or create the default Bitcoin datadir before enabling -openbitcoinsync=mainnet-ibd.",
        ));
    }
    let network = ManagedNetworkHandle::transient_runtime(
        runtime.sync.runtime.network.magic(),
        runtime.sync.runtime.network.default_port(),
        runtime.relay,
        runtime.block_serving,
        runtime.inbound.enabled,
    );
    network
        .configure_basic_peer_serving(runtime.peer_block_filters)
        .map_err(|error| {
            DaemonSyncPreflightError::new(format!(
                "failed to configure BASIC peer serving: {error}"
            ))
        })?;
    Ok(OpenedAuthoritativeRuntime::Transient(
        AuthoritativeNetworkRuntime {
            maybe_prebuilt_context: None,
            network,
            peer_identity_authority: PeerIdentityAuthority::default(),
            announcement_outboxes: Default::default(),
            maybe_sync_runtime: None,
        },
    ))
}

pub(crate) fn fixture_open_runtime_store(
    runtime: &RuntimeConfig,
) -> Result<Option<FjallNodeStore>, DaemonSyncPreflightError> {
    runtime
        .validate_peer_block_filters()
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
    let has_valid_datadir = runtime
        .maybe_data_dir
        .as_ref()
        .is_some_and(|dir| dir.is_dir());
    if runtime.block_filter_index.is_enabled() && !has_valid_datadir {
        return Err(DaemonSyncPreflightError::new(
            "open-bitcoind BASIC indexing requires an existing datadir and retained validated genesis history; set -datadir=<path> before enabling -blockfilterindex.",
        ));
    }
    let has_durable_trigger = runtime.sync.is_enabled()
        || runtime.inbound.enabled
        || runtime.prune_mode != open_bitcoin_node::core::chainstate::PruneMode::Disabled
        || (runtime.block_filter_index.is_explicit() && has_valid_datadir);
    if !has_durable_trigger {
        return Ok(None);
    }
    let Some(data_dir) = runtime.maybe_data_dir.as_ref() else {
        return Ok(None);
    };
    FjallNodeStore::open(data_dir).map(Some).map_err(|error| {
        DaemonSyncPreflightError::new(format!(
            "open-bitcoind runtime failed to open durable store at \"{}\": {error}",
            data_dir.display()
        ))
    })
}

pub(super) fn fixture_start_inbound_metrics_worker<S, V>(
    runtime: &RuntimeConfig,
    shared_context: std::sync::Arc<tokio::sync::Mutex<open_bitcoin_rpc::ManagedRpcContext<S, V>>>,
    maybe_store: Option<FjallNodeStore>,
) -> Result<Option<crate::inbound_metrics::InboundMetricsWorker>, DaemonSyncPreflightError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    if !runtime.inbound.enabled || runtime.sync.is_enabled() {
        return Ok(None);
    }
    let Some(data_dir) = runtime.maybe_data_dir.as_ref() else {
        return Ok(None);
    };
    let store = match maybe_store {
        Some(store) => store,
        None => FjallNodeStore::open(data_dir)
            .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?,
    };
    Ok(Some(crate::inbound_metrics::spawn_inbound_metrics_worker(
        runtime,
        shared_context,
        store,
    )))
}
