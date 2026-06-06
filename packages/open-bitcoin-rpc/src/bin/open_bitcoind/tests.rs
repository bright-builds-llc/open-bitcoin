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
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use open_bitcoin_node::{
    DurableSyncRuntime, FieldAvailability, FjallNodeStore, SyncLifecycleState, SyncRunSummary,
    SyncRuntimeConfig, SyncRuntimeError, SyncStopReason,
};
use open_bitcoin_rpc::config::{DaemonSyncConfig, RuntimeConfig};

use super::{
    DaemonSyncLoopDecision, DaemonSyncLoopPolicy, DaemonSyncPreflight,
    daemon_sync_preflight_message, preflight_daemon_sync, run_daemon_sync_loop_cycle,
};

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

fn temp_store_path(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "open-bitcoind-sync-preflight-{label}-{}",
        NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&path).expect("test store directory");
    path
}

fn test_sync_runtime(label: &str) -> DurableSyncRuntime {
    let data_dir = temp_store_path(label);
    let store = FjallNodeStore::open(&data_dir).expect("test store");
    DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            dns_seeds: Vec::new(),
            manual_peers: Vec::new(),
            retry_backoff_ms: 25,
            ..SyncRuntimeConfig::default()
        },
    )
    .expect("test sync runtime")
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
fn enabled_sync_preflight_opens_durable_runtime_before_worker_startup() {
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
fn enabled_sync_preflight_message_describes_opt_in_worker_without_production_claim() {
    // Arrange
    let preflight = DaemonSyncPreflight {
        mode: DaemonSyncConfig::mainnet_ibd().mode,
        data_dir: PathBuf::from("/tmp/open-bitcoin-mainnet"),
        best_header_height: 12,
        best_block_height: 3,
    };

    // Act
    let message = daemon_sync_preflight_message(&preflight);

    // Assert
    assert!(message.contains("opened durable store"));
    assert!(message.contains("explicit opt-in bounded unattended review loop"));
    assert!(message.contains("stop, retry, and backoff policy"));
    assert!(message.contains("not unattended production-node operation"));
    assert!(message.contains("not a packaged-service guarantee"));
    assert!(message.contains("mode=mainnet-ibd"));
    assert!(message.contains("datadir=\"/tmp/open-bitcoin-mainnet\""));
    assert!(message.contains("best_header_height=12"));
    assert!(message.contains("best_block_height=3"));
    assert!(!message.contains("peer transport and unattended full IBD"));
    assert!(!message.contains("not started by this phase"));
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

#[test]
fn daemon_sync_loop_policy_uses_bounded_minimum_backoff() {
    // Arrange
    let runtime = test_sync_runtime("daemon-loop-policy");

    // Act
    let policy = DaemonSyncLoopPolicy::from_runtime(&runtime);

    // Assert
    assert_eq!(policy.sleep_duration, Duration::from_millis(1_000));
}

#[test]
fn daemon_sync_loop_paused_cycle_persists_durable_stop_reason() {
    // Arrange
    let mut runtime = test_sync_runtime("daemon-loop-paused");
    runtime.set_sync_paused(true).expect("pause sync");
    let policy = DaemonSyncLoopPolicy::from_runtime(&runtime);

    // Act
    let decision =
        run_daemon_sync_loop_cycle(&mut runtime, policy, 1_777_225_190, false, |_runtime, _| {
            panic!("paused daemon_sync_loop cycle must not run network work");
        });
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("runtime metadata")
        .expect("metadata saved");
    let state = metadata.maybe_sync_state.expect("sync state saved");

    // Assert
    assert_eq!(
        decision,
        DaemonSyncLoopDecision::Paused(policy.sleep_duration)
    );
    assert_eq!(
        state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Paused)
    );
    assert_eq!(
        state.sync.phase,
        FieldAvailability::available("paused".to_string())
    );
    assert_eq!(
        state.sync.last_error,
        FieldAvailability::available("operator paused unattended sync loop".to_string())
    );
    assert!(state.health_signals.iter().any(|signal| {
        signal
            .message
            .contains("operator paused unattended sync loop")
    }));
}

#[test]
fn daemon_sync_loop_shutdown_cycle_persists_stopped_state() {
    // Arrange
    let mut runtime = test_sync_runtime("daemon-loop-shutdown");
    let policy = DaemonSyncLoopPolicy::from_runtime(&runtime);

    // Act
    let decision =
        run_daemon_sync_loop_cycle(&mut runtime, policy, 1_777_225_191, true, |_runtime, _| {
            panic!("shutdown daemon_sync_loop cycle must not run network work");
        });
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("runtime metadata")
        .expect("metadata saved");
    let state = metadata.maybe_sync_state.expect("sync state saved");

    // Assert
    assert_eq!(decision, DaemonSyncLoopDecision::Stopped);
    assert_eq!(
        state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Stopped)
    );
    assert_eq!(
        state.sync.phase,
        FieldAvailability::available("stopped".to_string())
    );
    assert_eq!(
        state.sync.last_error,
        FieldAvailability::available(
            "daemon shutdown requested for unattended sync loop".to_string()
        )
    );
    assert!(state.health_signals.iter().any(|signal| {
        signal
            .message
            .contains("daemon shutdown requested for unattended sync loop")
    }));
}

#[test]
fn daemon_sync_loop_failed_cycle_persists_failure_guidance() {
    // Arrange
    let mut runtime = test_sync_runtime("daemon-loop-failed");
    let policy = DaemonSyncLoopPolicy::from_runtime(&runtime);

    // Act
    let decision =
        run_daemon_sync_loop_cycle(&mut runtime, policy, 1_777_225_192, false, |_runtime, _| {
            Err(SyncRuntimeError::Network {
                message: "scripted cycle failure".to_string(),
            })
        });
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("runtime metadata")
        .expect("metadata saved");
    let state = metadata.maybe_sync_state.expect("sync state saved");

    // Assert
    assert_eq!(
        decision,
        DaemonSyncLoopDecision::Failed(policy.sleep_duration)
    );
    assert_eq!(
        state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Failed)
    );
    assert_eq!(
        state.sync.phase,
        FieldAvailability::available("failed".to_string())
    );
    assert_eq!(
        state.sync.last_error,
        FieldAvailability::available("sync network failure: scripted cycle failure".to_string())
    );
}

#[test]
fn daemon_sync_loop_successful_cycle_preserves_summary_stop_reason() {
    // Arrange
    let mut runtime = test_sync_runtime("daemon-loop-success");
    let policy = DaemonSyncLoopPolicy::from_runtime(&runtime);

    // Act
    let decision =
        run_daemon_sync_loop_cycle(&mut runtime, policy, 1_777_225_193, false, |runtime, _| {
            let mut summary: SyncRunSummary = runtime.snapshot_summary();
            summary.maybe_stop_reason = Some(SyncStopReason::MaxRoundsReached { max_rounds: 1 });
            summary
                .health_signals
                .push(SyncStopReason::MaxRoundsReached { max_rounds: 1 }.health_signal());
            Ok(summary)
        });
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("runtime metadata")
        .expect("metadata saved");
    let state = metadata.maybe_sync_state.expect("sync state saved");

    // Assert
    assert_eq!(
        decision,
        DaemonSyncLoopDecision::RetryAfter(policy.sleep_duration)
    );
    assert_eq!(
        state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Active)
    );
    assert_eq!(
        state.sync.phase,
        FieldAvailability::available("max_rounds_reached".to_string())
    );
}
