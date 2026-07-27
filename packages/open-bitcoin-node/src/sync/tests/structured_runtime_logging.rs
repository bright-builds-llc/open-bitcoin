// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn write_block_relay_log_emits_when_status_available() {
    // Arrange
    let path = temp_store_path("block-relay-log-available");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&log_dir).expect("create log dir");
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let block_relay = block_relay_status_for_metrics();
    let snapshot = crate::network::BlockRelayRuntimeEvidenceSnapshot {
        status: block_relay,
        served_count: 9,
    };
    let mut summary = runtime.snapshot_summary();

    // Act
    runtime.write_block_relay_log(&mut summary, Some(&snapshot), 1_777_225_305);

    // Assert
    let records = load_structured_log_records(&log_dir);
    let maybe_block_relay = records
        .iter()
        .find(|record| record.source == BLOCK_RELAY_LOG_SOURCE);
    let record = maybe_block_relay.expect("block_relay log record");
    assert!(record.message.contains("outcome=projected"));
    assert!(record.message.contains("cause=status_projection"));
    assert!(record.message.contains("label=block_relay"));

    remove_dir_if_exists(&path);
}

#[test]
fn write_block_relay_log_omits_when_status_unavailable() {
    // Arrange
    let path = temp_store_path("block-relay-log-unavailable");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&log_dir).expect("create log dir");
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let mut summary = runtime.snapshot_summary();

    // Act
    runtime.write_block_relay_log(&mut summary, None, 1_777_225_306);

    // Assert
    let records = load_structured_log_records(&log_dir);
    assert!(
        !records
            .iter()
            .any(|record| record.source == BLOCK_RELAY_LOG_SOURCE)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn write_block_relay_log_omits_sensitive_markers() {
    // Arrange
    let path = temp_store_path("block-relay-log-leakage");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&log_dir).expect("create log dir");
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let block_relay = block_relay_status_for_metrics();
    let snapshot = crate::network::BlockRelayRuntimeEvidenceSnapshot {
        status: block_relay,
        served_count: 0,
    };
    let mut summary = runtime.snapshot_summary();

    // Act
    runtime.write_block_relay_log(&mut summary, Some(&snapshot), 1_777_225_307);

    // Assert
    let records = load_structured_log_records(&log_dir);
    let maybe_block_relay = records
        .iter()
        .find(|record| record.source == BLOCK_RELAY_LOG_SOURCE);
    let record = maybe_block_relay.expect("block_relay log record");
    for raw in [
        "127.0.0.1",
        "peer_id",
        "permission_string",
        "credential",
        "cookie",
        "secret",
        "0123456789abcdef",
    ] {
        assert!(!record.message.contains(raw), "leaked {raw}");
    }

    remove_dir_if_exists(&path);
}

#[test]
fn sync_status_and_log_records_include_message_header_block_counters() {
    // Arrange
    let path = temp_store_path("counter-logs");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let genesis_hash = block_hash(&genesis.header);
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone()],
        }),
        WireNetworkMessage::Block(genesis),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_099)
        .expect("sync");

    // Assert
    assert_eq!(summary.messages_processed, 4);
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 1);
    assert_eq!(
        summary.sync_status(SyncNetwork::Regtest).sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 0,
            block_height: 0,
            downloaded_block_height: 0,
            connected_block_height: 0,
            validated_active_chain_height: 0,
            maybe_downloaded_block_hash: Some(block_hash_hex(genesis_hash)),
            maybe_connected_block_hash: Some(block_hash_hex(genesis_hash)),
            maybe_validated_active_chain_hash: Some(block_hash_hex(genesis_hash)),
            maybe_validated_active_chain_work: Some("1".to_string()),
            progress_ratio: 1.0,
            messages_processed: 4,
            headers_received: 1,
            blocks_received: 1,
        })
    );
    let records = load_structured_log_records(&log_dir);
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Info
            && record.source == "sync"
            && record.message.contains("messages_processed=4")
            && record.message.contains("headers_received=1")
            && record.message.contains("blocks_received=1")
            && record.message.contains("header=0")
            && record.message.contains("downloaded=0")
            && record.message.contains("connected=0")
            && record.message.contains("progress_signal=block_progress")
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn peer_contribution_counts_only_accepted_headers_and_blocks() {
    // Arrange
    let path = temp_store_path("peer-contribution-accepted");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone()],
        }),
        WireNetworkMessage::Block(genesis),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_101)
        .expect("sync");

    // Assert
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 1);
    assert_eq!(summary.peer_outcomes.len(), 1);
    assert_eq!(summary.peer_outcomes[0].contribution.messages_processed, 4);
    assert_eq!(summary.peer_outcomes[0].contribution.headers_received, 1);
    assert_eq!(summary.peer_outcomes[0].contribution.blocks_received, 1);
    assert_eq!(
        summary.peer_outcomes[0].maybe_last_activity_unix_seconds,
        Some(1_777_225_101)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn stalled_peer_emits_warning_health_signal_and_log_record() {
    // Arrange
    let path = temp_store_path("stalled-peer");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![Vec::new()]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_111)
        .expect("sync");

    // Assert
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Stalled);
    assert!(summary.health_signals.iter().any(|signal| {
        signal.level == HealthSignalLevel::Warn
            && signal.source == "sync"
            && signal.message.contains("peer stalled")
    }));
    let log_status = load_log_status(&log_dir, LogRetentionPolicy::default(), 10);
    assert!(log_status.recent_signals.iter().any(|signal| {
        signal.level == StructuredLogLevel::Warn
            && signal.source == "sync"
            && signal.message.contains("peer stalled")
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn connect_retries_preserve_attempt_count() {
    // Arrange
    let path = temp_store_path("connect-retries");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_peer_retries: 2,
            maybe_log_dir: Some(log_dir.clone()),
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::failing();

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_122)
        .expect("sync");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Failed);
    assert_eq!(summary.peer_outcomes[0].attempts, 3);
    assert!(summary.health_signals.iter().any(|signal| {
        signal.source == "network" && signal.message.contains("sync I/O failure")
    }));
    let log_status = load_log_status(&log_dir, LogRetentionPolicy::default(), 10);
    assert!(log_status.recent_signals.iter().any(|signal| {
        signal.source == "network" && signal.message.contains("sync I/O failure")
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn storage_failure_projects_storage_health_signal() {
    // Arrange
    let error = SyncRuntimeError::Storage(StorageError::BackendFailure {
        namespace: StorageNamespace::Metrics,
        message: "/tmp/open-bitcoin/private-store".to_string(),
        action: crate::StorageRecoveryAction::Restart,
    });

    // Act
    let signal = error.health_signal();
    let records = SyncRunSummary {
        target_outbound_peers: 0,
        maybe_target_header_height: None,
        attempted_peers: 0,
        connected_peers: 0,
        failed_peers: 0,
        messages_processed: 0,
        headers_received: 0,
        blocks_received: 0,
        best_header_height: 0,
        downloaded_block_height: 0,
        best_block_height: 0,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: Vec::new(),
        health_signals: vec![signal.clone()],
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    }
    .structured_log_records(1_777_225_133);

    // Assert
    assert_eq!(signal.level, HealthSignalLevel::Error);
    assert_eq!(signal.source, "storage");
    assert!(
        signal
            .message
            .contains("storage backend failure in metrics")
    );
    assert!(!signal.message.contains("/tmp/"));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Error
            && record.source == "storage"
            && record.message == signal.message
    }));
}
