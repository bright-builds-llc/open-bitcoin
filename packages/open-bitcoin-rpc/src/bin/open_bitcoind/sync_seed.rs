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

use std::path::PathBuf;

use open_bitcoin_node::{DurableSyncRuntime, SyncLifecycleState};
use open_bitcoin_rpc::config::{DaemonSyncMode, RuntimeConfig};

use super::{DaemonSyncPreflightError, current_timestamp_unix_seconds};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DaemonSyncPreflight {
    pub(super) mode: DaemonSyncMode,
    pub(super) data_dir: PathBuf,
    pub(super) best_header_height: u64,
    pub(super) best_block_height: u64,
}

pub(super) fn preflight_daemon_sync(
    runtime: &RuntimeConfig,
    maybe_sync_runtime: Option<&DurableSyncRuntime>,
) -> Result<Option<DaemonSyncPreflight>, DaemonSyncPreflightError> {
    if !runtime.sync.is_enabled() {
        return Ok(None);
    }

    let Some(data_dir) = runtime.maybe_data_dir.as_ref() else {
        return Err(DaemonSyncPreflightError::new(
            "open-bitcoind mainnet sync activation requires an existing datadir; set -datadir=<path> or create the default Bitcoin datadir before enabling -openbitcoinsync=mainnet-ibd.",
        ));
    };
    let Some(sync_runtime) = maybe_sync_runtime else {
        return Err(DaemonSyncPreflightError::new(
            "open-bitcoind mainnet sync preflight requires the authoritative durable sync runtime",
        ));
    };
    let summary = sync_runtime.snapshot_summary();

    Ok(Some(DaemonSyncPreflight {
        mode: runtime.sync.mode,
        data_dir: data_dir.clone(),
        best_header_height: summary.best_header_height,
        best_block_height: summary.best_block_height,
    }))
}

pub(super) fn report_daemon_sync_preflight(preflight: &DaemonSyncPreflight) {
    eprintln!("{}", daemon_sync_preflight_message(preflight));
}

pub(super) fn daemon_sync_preflight_message(preflight: &DaemonSyncPreflight) -> String {
    format!(
        "open-bitcoind mainnet sync preflight opened durable store: mode={}, datadir=\"{}\", best_header_height={}, best_block_height={}; enabled startup will run the explicit opt-in bounded unattended review loop with stop, retry, and backoff policy. This is not unattended production-node operation and is not a packaged-service guarantee.",
        preflight.mode,
        preflight.data_dir.display(),
        preflight.best_header_height,
        preflight.best_block_height
    )
}

pub(super) fn seed_initial_sync_state(
    sync_runtime: &DurableSyncRuntime,
) -> Result<(), DaemonSyncPreflightError> {
    let timestamp = current_timestamp_unix_seconds();
    let lifecycle = if sync_runtime
        .load_sync_control()
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?
        .paused
    {
        SyncLifecycleState::Paused
    } else if sync_runtime
        .store()
        .load_recovery_marker()
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?
        .is_some()
    {
        SyncLifecycleState::Recovering
    } else {
        SyncLifecycleState::Active
    };
    let state = sync_runtime
        .durable_sync_state(lifecycle, None, timestamp)
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
    sync_runtime
        .persist_durable_sync_state(state)
        .map_err(|error| DaemonSyncPreflightError::new(error.to_string()))
}
