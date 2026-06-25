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
    fs, io,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{
    DurableSyncRuntime, FieldAvailability, FjallNodeStore, SyncLifecycleState, SyncRunSummary,
    SyncRuntimeConfig, SyncRuntimeError, SyncStopReason,
};
use open_bitcoin_network::{InboundListenerConfig, InboundPreflightReason};
use open_bitcoin_rpc::config::{DaemonSyncConfig, RuntimeConfig};
use open_bitcoin_rpc::inbound_listener::InboundListenerState;

use super::{
    DaemonSyncLoopDecision, DaemonSyncLoopPolicy, DaemonSyncPreflight,
    daemon_sync_preflight_message, inbound_listener_startup_message, preflight_daemon_sync,
    run_daemon_sync_loop_cycle, start_inbound_listener_for_runtime,
};

fn temp_store_path(label: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!(
        "open-bitcoind-sync-preflight-{label}-{}-{timestamp}",
        std::process::id()
    ))
}

fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

fn test_sync_runtime(label: &str) -> DurableSyncRuntime {
    let data_dir = temp_store_path(label);
    remove_dir_if_exists(&data_dir);
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
    remove_dir_if_exists(&data_dir);
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

#[tokio::test]
async fn open_bitcoind_inbound_default_runtime_reports_disabled_without_worker() {
    // Arrange
    let runtime = RuntimeConfig::default();

    // Act
    let listener = start_inbound_listener_for_runtime(&runtime).await;

    // Assert
    assert_eq!(listener.state, InboundListenerState::Disabled);
    assert_eq!(listener.preflight_reason, InboundPreflightReason::Disabled);
    assert!(listener.bound_endpoints.is_empty());
    assert!(listener.maybe_worker.is_none());
}

#[tokio::test]
async fn open_bitcoind_inbound_loopback_runtime_binds_before_rpc_serving() {
    // Arrange
    let runtime = RuntimeConfig {
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:0".to_string()],
            max_peers: 2,
            reserved_slots: 0,
            allow_public: false,
        },
        ..RuntimeConfig::default()
    };

    // Act
    let mut listener = start_inbound_listener_for_runtime(&runtime).await;

    // Assert
    assert_eq!(listener.state, InboundListenerState::Listening);
    assert_eq!(listener.preflight_reason, InboundPreflightReason::Ready);
    assert_eq!(listener.bound_endpoints.len(), 1);
    assert!(listener.bound_endpoints[0].starts_with("127.0.0.1:"));
    assert!(listener.maybe_worker.is_some());
    listener.shutdown().await;
}

#[tokio::test]
async fn open_bitcoind_inbound_shutdown_closes_listener_without_sync_shutdown_regression() {
    // Arrange
    let runtime = RuntimeConfig {
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:0".to_string()],
            max_peers: 2,
            reserved_slots: 0,
            allow_public: false,
        },
        ..RuntimeConfig::default()
    };
    let mut listener = start_inbound_listener_for_runtime(&runtime).await;
    let endpoint = listener.bound_endpoints[0]
        .parse()
        .expect("bound endpoint socket address");
    std::net::TcpStream::connect(endpoint).expect("listener accepts before shutdown");
    let mut sync_runtime = test_sync_runtime("inbound-shutdown-sync");
    let policy = DaemonSyncLoopPolicy::from_runtime(&sync_runtime);

    // Act
    listener.shutdown().await;
    let reconnect = std::net::TcpStream::connect_timeout(&endpoint, Duration::from_millis(100));
    let sync_decision = run_daemon_sync_loop_cycle(
        &mut sync_runtime,
        policy,
        1_777_225_194,
        true,
        |_runtime, _| panic!("shutdown cycle must not run network work"),
    );

    // Assert
    assert!(reconnect.is_err());
    assert_eq!(sync_decision, DaemonSyncLoopDecision::Stopped);
}

#[test]
fn open_bitcoind_inbound_startup_message_uses_stable_labels_without_scope_creep() {
    // Arrange
    let listener = super::InboundDaemonListener {
        state: InboundListenerState::Listening,
        preflight_reason: InboundPreflightReason::Ready,
        bound_endpoints: vec!["127.0.0.1:18444".to_string()],
        diagnostics: Vec::new(),
        maybe_worker: None,
    };

    // Act
    let message = inbound_listener_startup_message(&listener);

    // Assert
    assert!(message.contains("inbound_listener_state=listening"));
    assert!(message.contains("inbound_preflight_reason=ready"));
    assert!(message.contains("bound_endpoint=127.0.0.1:18444"));
    assert!(message.contains("admission_reject_reason=unavailable"));
    assert!(message.contains("opt-in inbound listener/admission"));
    assert!(message.contains("deferred network participation remains out of scope"));
}
