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

use super::*;

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
