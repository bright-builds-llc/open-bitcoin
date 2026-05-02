#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented,
        clippy::panic_in_result_fn,
    )
)]
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

use std::{
    error::Error,
    path::PathBuf,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{DurableSyncRuntime, FjallNodeStore, SyncLifecycleState, TcpPeerTransport};
use open_bitcoin_rpc::{
    ManagedRpcContext,
    config::{DaemonSyncMode, RuntimeConfig, load_runtime_config},
    http,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = load_runtime_config()?;
    if !runtime.rpc_server.enabled {
        return Err("RPC server is disabled by configuration".into());
    }
    if let Some(preflight) = preflight_daemon_sync(&runtime)? {
        report_daemon_sync_preflight(&preflight);
    }
    let _maybe_sync_worker = start_daemon_sync_worker(&runtime)?;

    let bind_address = runtime.rpc_server.bind_address;
    let auth = runtime.rpc_server.auth.clone();
    let context = ManagedRpcContext::from_runtime_config(&runtime);
    let state = http::build_http_state(auth, context)?;
    let listener = tokio::net::TcpListener::bind(bind_address).await?;

    axum::serve(listener, http::router(state)).await?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DaemonSyncPreflight {
    mode: DaemonSyncMode,
    data_dir: PathBuf,
    best_header_height: u64,
    best_block_height: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DaemonSyncPreflightError {
    message: String,
}

impl DaemonSyncPreflightError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl core::fmt::Display for DaemonSyncPreflightError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for DaemonSyncPreflightError {}

fn preflight_daemon_sync(
    runtime: &RuntimeConfig,
) -> Result<Option<DaemonSyncPreflight>, DaemonSyncPreflightError> {
    if !runtime.sync.is_enabled() {
        return Ok(None);
    }

    let Some(data_dir) = runtime.maybe_data_dir.as_ref() else {
        return Err(DaemonSyncPreflightError::new(
            "open-bitcoind mainnet sync activation requires an existing datadir; set -datadir=<path> or create the default Bitcoin datadir before enabling -openbitcoinsync=mainnet-ibd.",
        ));
    };
    let store = FjallNodeStore::open(data_dir).map_err(|error| {
        DaemonSyncPreflightError::new(format!(
            "open-bitcoind mainnet sync preflight failed to open durable store at \"{}\": {error}",
            data_dir.display()
        ))
    })?;
    let sync_runtime =
        DurableSyncRuntime::open(store, runtime.sync.runtime.clone()).map_err(|error| {
            DaemonSyncPreflightError::new(format!(
                "open-bitcoind mainnet sync preflight failed to construct durable sync runtime: {error}"
            ))
        })?;
    let summary = sync_runtime.snapshot_summary();

    Ok(Some(DaemonSyncPreflight {
        mode: runtime.sync.mode,
        data_dir: data_dir.clone(),
        best_header_height: summary.best_header_height,
        best_block_height: summary.best_block_height,
    }))
}

fn report_daemon_sync_preflight(preflight: &DaemonSyncPreflight) {
    eprintln!(
        "open-bitcoind mainnet sync preflight enabled: mode={}, datadir=\"{}\", best_header_height={}, best_block_height={}; peer transport and unattended full IBD are not started by this phase.",
        preflight.mode,
        preflight.data_dir.display(),
        preflight.best_header_height,
        preflight.best_block_height
    );
}

fn start_daemon_sync_worker(
    runtime: &RuntimeConfig,
) -> Result<Option<thread::JoinHandle<()>>, DaemonSyncPreflightError> {
    if !runtime.sync.is_enabled() {
        return Ok(None);
    }

    let Some(data_dir) = runtime.maybe_data_dir.as_ref() else {
        return Err(DaemonSyncPreflightError::new(
            "open-bitcoind mainnet sync activation requires an existing datadir; set -datadir=<path> or create the default Bitcoin datadir before enabling -openbitcoinsync=mainnet-ibd.",
        ));
    };
    let store = FjallNodeStore::open(data_dir).map_err(|error| {
        DaemonSyncPreflightError::new(format!(
            "open-bitcoind daemon sync failed to open durable store at \"{}\": {error}",
            data_dir.display()
        ))
    })?;
    let sync_runtime =
        DurableSyncRuntime::open(store, runtime.sync.runtime.clone()).map_err(|error| {
            DaemonSyncPreflightError::new(format!(
                "open-bitcoind daemon sync failed to construct durable sync runtime: {error}"
            ))
        })?;
    seed_initial_sync_state(&sync_runtime)?;

    Ok(Some(thread::spawn(move || {
        daemon_sync_worker(sync_runtime)
    })))
}

fn seed_initial_sync_state(
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

fn daemon_sync_worker(mut sync_runtime: DurableSyncRuntime) {
    let mut transport = TcpPeerTransport;
    let sleep_duration = Duration::from_millis(sync_runtime.config().retry_backoff_ms.max(1_000));

    loop {
        let paused = match sync_runtime.load_sync_control() {
            Ok(value) => value.paused,
            Err(error) => {
                eprintln!("open-bitcoind daemon sync control read failed: {error}");
                thread::sleep(sleep_duration);
                continue;
            }
        };
        if paused {
            if let Ok(state) = sync_runtime.durable_sync_state(
                SyncLifecycleState::Paused,
                None,
                current_timestamp_unix_seconds(),
            ) {
                let _ = sync_runtime.persist_durable_sync_state(state);
            }
            thread::sleep(sleep_duration);
            continue;
        }

        let lifecycle = match sync_runtime.store().load_recovery_marker() {
            Ok(Some(_)) => SyncLifecycleState::Recovering,
            Ok(None) => SyncLifecycleState::Active,
            Err(error) => {
                eprintln!("open-bitcoind daemon sync recovery-marker read failed: {error}");
                thread::sleep(sleep_duration);
                continue;
            }
        };
        if let Ok(state) =
            sync_runtime.durable_sync_state(lifecycle, None, current_timestamp_unix_seconds())
        {
            let _ = sync_runtime.persist_durable_sync_state(state);
        }

        let round_result = {
            let timestamp = current_timestamp_unix_seconds();
            sync_runtime
                .sync_until_idle(&mut transport, timestamp)
                .and_then(|summary| {
                    let state = sync_runtime.durable_sync_state_for_summary(
                        &summary,
                        SyncLifecycleState::Active,
                        None,
                        timestamp,
                    )?;
                    sync_runtime.persist_durable_sync_state(state)
                })
        };
        match round_result {
            Ok(()) => thread::sleep(sleep_duration),
            Err(error) => {
                eprintln!("open-bitcoind daemon sync round failed: {error}");
                if let Ok(state) = sync_runtime.durable_sync_state(
                    SyncLifecycleState::Failed,
                    Some(error.to_string()),
                    current_timestamp_unix_seconds(),
                ) {
                    let _ = sync_runtime.persist_durable_sync_state(state);
                }
                thread::sleep(sleep_duration);
            }
        }
    }
}

fn current_timestamp_unix_seconds() -> i64 {
    let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return 0;
    };
    i64::try_from(duration.as_secs()).unwrap_or(i64::MAX)
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    use open_bitcoin_rpc::config::{DaemonSyncConfig, RuntimeConfig};

    use super::preflight_daemon_sync;

    static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

    fn temp_store_path(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "open-bitcoind-sync-preflight-{label}-{}",
            NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).expect("test store directory");
        path
    }

    #[test]
    fn disabled_sync_skips_daemon_preflight() {
        // Arrange
        let runtime = RuntimeConfig::default();

        // Act
        let preflight = preflight_daemon_sync(&runtime).expect("disabled preflight");

        // Assert
        assert_eq!(preflight, None);
    }

    #[test]
    fn enabled_sync_opens_durable_runtime_without_starting_transport() {
        // Arrange
        let data_dir = temp_store_path("enabled");
        let runtime = RuntimeConfig {
            maybe_data_dir: Some(data_dir.clone()),
            sync: DaemonSyncConfig::mainnet_ibd(),
            ..RuntimeConfig::default()
        };

        // Act
        let preflight = preflight_daemon_sync(&runtime)
            .expect("enabled preflight")
            .expect("preflight summary");

        // Assert
        assert_eq!(preflight.data_dir, data_dir);
        assert_eq!(preflight.mode, runtime.sync.mode);
        assert_eq!(preflight.best_header_height, 0);
        assert_eq!(preflight.best_block_height, 0);
    }

    #[test]
    fn enabled_sync_requires_datadir_before_daemon_binds_rpc() {
        // Arrange
        let runtime = RuntimeConfig {
            sync: DaemonSyncConfig::mainnet_ibd(),
            ..RuntimeConfig::default()
        };

        // Act
        let error = preflight_daemon_sync(&runtime).expect_err("missing datadir should fail");

        // Assert
        assert_eq!(
            error.to_string(),
            "open-bitcoind mainnet sync activation requires an existing datadir; set -datadir=<path> or create the default Bitcoin datadir before enabling -openbitcoinsync=mainnet-ibd."
        );
    }
}
