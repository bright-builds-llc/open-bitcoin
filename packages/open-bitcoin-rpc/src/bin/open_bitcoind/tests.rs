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
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use open_bitcoin_network::{
    InboundListenerConfig, InboundPreflightReason, VersionMessage, WireNetworkMessage,
};
use open_bitcoin_node::{
    DurableSyncRuntime, FieldAvailability, FjallNodeStore, MetricKind, ResolvedSyncPeerAddress,
    SyncLifecycleState, SyncPeerAddress, SyncPeerReceiveOutcome, SyncPeerSession, SyncRunSummary,
    SyncRuntimeConfig, SyncRuntimeError, SyncStopReason, SyncTransport,
};
use open_bitcoin_rpc::inbound_listener::InboundListenerState;
use open_bitcoin_rpc::{
    ManagedRpcContext,
    config::{DaemonSyncConfig, RuntimeConfig},
};

use super::{
    DaemonSyncLoopDecision, DaemonSyncLoopPolicy, DaemonSyncPreflight,
    daemon_sync_preflight_message, daemon_sync_worker_with_transport,
    inbound_listener_startup_message, preflight_daemon_sync, run_daemon_sync_loop_cycle,
    start_inbound_listener_for_runtime, start_inbound_listener_for_runtime_with_context,
    start_inbound_metrics_worker,
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

#[derive(Debug)]
struct SilentPeerTransport {
    receive_calls: Arc<AtomicUsize>,
}

#[derive(Debug)]
struct SilentPeerSession {
    receive_calls: Arc<AtomicUsize>,
}

impl SyncTransport for SilentPeerTransport {
    type Session = SilentPeerSession;

    fn connect(
        &mut self,
        _peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        Ok(SilentPeerSession {
            receive_calls: Arc::clone(&self.receive_calls),
        })
    }
}

impl SyncPeerSession for SilentPeerSession {
    fn send(
        &mut self,
        _message: &WireNetworkMessage,
        _magic: open_bitcoin_node::core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_node::core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        let call = self.receive_calls.fetch_add(1, Ordering::SeqCst);
        Ok(match call {
            0 => SyncPeerReceiveOutcome::Message(WireNetworkMessage::Version(
                VersionMessage::default(),
            )),
            1 => SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
            _ => {
                thread::sleep(Duration::from_millis(25));
                SyncPeerReceiveOutcome::Idle
            }
        })
    }
}

fn silent_peer_sync_runtime(label: &str) -> DurableSyncRuntime {
    let data_dir = temp_store_path(label);
    remove_dir_if_exists(&data_dir);
    let store = FjallNodeStore::open(&data_dir).expect("test store");
    DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("127.0.0.1", 18_444)],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            max_peer_retries: 0,
            retry_backoff_ms: 25,
            ..SyncRuntimeConfig::default()
        },
    )
    .expect("silent-peer sync runtime")
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
fn phase123_daemon_shutdown_cancels_live_silent_peer_session() {
    // Arrange
    let runtime = silent_peer_sync_runtime("daemon-live-silent-shutdown");
    let receive_calls = Arc::new(AtomicUsize::new(0));
    let transport = SilentPeerTransport {
        receive_calls: Arc::clone(&receive_calls),
    };
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();
    let (done_sender, done_receiver) = mpsc::channel();
    let worker = thread::spawn(move || {
        daemon_sync_worker_with_transport(runtime, transport, shutdown_receiver);
        done_sender.send(()).expect("report worker completion");
    });
    let wait_started = std::time::Instant::now();
    while receive_calls.load(Ordering::SeqCst) < 3 {
        assert!(
            wait_started.elapsed() < Duration::from_secs(1),
            "silent peer did not reach its first idle receive"
        );
        thread::yield_now();
    }

    // Act
    shutdown_sender.send(()).expect("request worker shutdown");
    done_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("silent peer worker should observe shutdown");
    worker.join().expect("join silent peer worker");

    // Assert
    assert_eq!(receive_calls.load(Ordering::SeqCst), 3);
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
            permission_classes: Default::default(),
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
async fn open_bitcoind_inbound_listener_evidence_reaches_shared_rpc_status() {
    // Arrange
    let runtime = RuntimeConfig {
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:0".to_string()],
            max_peers: 2,
            reserved_slots: 0,
            allow_public: false,
            permission_classes: Default::default(),
        },
        ..RuntimeConfig::default()
    };
    let shared_context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_runtime_config(&runtime),
    ));

    // Act
    let mut listener =
        start_inbound_listener_for_runtime_with_context(&runtime, Arc::clone(&shared_context))
            .await;
    let inbound_status = shared_context.lock().await.current_inbound_status();

    // Assert
    let FieldAvailability::Available(status) = inbound_status else {
        panic!("listener activation should publish inbound status evidence");
    };
    assert_eq!(status.listener_state, "listening");
    assert_eq!(status.preflight_reason, "ready");
    assert_eq!(status.bound_endpoints, listener.bound_endpoints);
    assert_eq!(status.admitted_inbound_peers, 0);
    assert_eq!(status.rejected_inbound_peers, 0);
    listener.shutdown().await;
}

#[tokio::test]
async fn open_bitcoind_inbound_metrics_worker_persists_sync_disabled_inbound_samples() {
    // Arrange
    let data_dir = temp_store_path("inbound-metrics-worker");
    remove_dir_if_exists(&data_dir);
    let runtime = RuntimeConfig {
        maybe_data_dir: Some(data_dir.clone()),
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:0".to_string()],
            max_peers: 2,
            reserved_slots: 0,
            allow_public: false,
            permission_classes: Default::default(),
        },
        ..RuntimeConfig::default()
    };
    let metrics_store = FjallNodeStore::open(&data_dir).expect("metrics store");
    let shared_context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_runtime_config_with_store(&runtime, Some(metrics_store.clone())),
    ));
    let mut listener =
        start_inbound_listener_for_runtime_with_context(&runtime, Arc::clone(&shared_context))
            .await;
    let worker =
        start_inbound_metrics_worker(&runtime, Arc::clone(&shared_context), Some(metrics_store))
            .expect("start inbound metrics worker")
            .expect("inbound metrics worker");
    shared_context
        .lock()
        .await
        .set_metrics_store(worker.metrics_store.clone());

    // Act
    let metrics = wait_for_inbound_metric_sample(&worker.metrics_store);
    worker.shutdown();
    let status = shared_context.lock().await.metrics_status();
    listener.shutdown().await;

    // Assert
    assert!(metrics);
    assert!(status.samples.iter().any(|sample| {
        sample.kind == MetricKind::InboundAdmittedPeerCount && sample.timestamp_unix_seconds > 0
    }));
    assert!(status.samples.iter().any(|sample| {
        sample.kind == MetricKind::RelayAcceptedCount && sample.timestamp_unix_seconds > 0
    }));
    remove_dir_if_exists(&data_dir);
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
            permission_classes: Default::default(),
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

fn wait_for_inbound_metric_sample(store: &FjallNodeStore) -> bool {
    for _ in 0..40 {
        let maybe_has_sample =
            store
                .load_metrics_snapshot()
                .ok()
                .flatten()
                .is_some_and(|snapshot| {
                    let has_inbound = snapshot
                        .samples
                        .iter()
                        .any(|sample| matches!(sample.kind, MetricKind::InboundAdmittedPeerCount));
                    let has_relay = snapshot
                        .samples
                        .iter()
                        .any(|sample| matches!(sample.kind, MetricKind::RelayAcceptedCount));
                    has_inbound && has_relay
                });
        if maybe_has_sample {
            return true;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    false
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
