// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn sync_summary_projects_metric_samples() {
    // Arrange
    let summary = SyncRunSummary {
        target_outbound_peers: 1,
        maybe_target_header_height: None,
        attempted_peers: 2,
        connected_peers: 1,
        failed_peers: 1,
        messages_processed: 7,
        headers_received: 3,
        blocks_received: 2,
        best_header_height: 42,
        downloaded_block_height: 41,
        best_block_height: 40,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    };

    // Act
    let samples = summary.metric_samples(1_777_225_022);

    // Assert
    assert_eq!(
        samples,
        vec![
            MetricSample::new(MetricKind::HeaderHeight, 42.0, 1_777_225_022),
            MetricSample::new(MetricKind::DownloadedBlockHeight, 41.0, 1_777_225_022),
            MetricSample::new(MetricKind::ConnectedBlockHeight, 40.0, 1_777_225_022),
            MetricSample::new(MetricKind::ValidatedActiveChainHeight, 40.0, 1_777_225_022,),
            MetricSample::new(MetricKind::SyncHeight, 40.0, 1_777_225_022),
            MetricSample::new(MetricKind::PeerCount, 1.0, 1_777_225_022),
        ]
    );
}

#[test]
fn sync_summary_projects_progress_signal_and_last_successful_timestamp() {
    // Arrange
    let mut outcome = peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_444),
        PeerSyncState::Connected,
        1,
        None,
        None,
    );
    outcome.contribution.headers_received = 2;
    outcome.maybe_last_activity_unix_seconds = Some(1_777_225_099);
    let summary = SyncRunSummary {
        target_outbound_peers: 1,
        maybe_target_header_height: None,
        attempted_peers: 1,
        connected_peers: 1,
        failed_peers: 0,
        messages_processed: 3,
        headers_received: 2,
        blocks_received: 0,
        best_header_height: 42,
        downloaded_block_height: 0,
        best_block_height: 0,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: vec![outcome],
        health_signals: Vec::new(),
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    };

    // Act
    let sync_status = summary.sync_status(SyncNetwork::Regtest);
    let records = summary.structured_log_records(1_777_225_100);

    // Assert
    assert_eq!(
        sync_status.progress_signal,
        FieldAvailability::available(SyncProgressSignal::HeaderProgress)
    );
    assert_eq!(
        sync_status.last_successful_progress_unix_seconds,
        FieldAvailability::available(1_777_225_099)
    );
    assert!(records.iter().any(|record| {
        record.message.contains("progress_signal=header_progress")
            && record.message.contains("last_progress=1777225099")
    }));
}

#[test]
fn sync_summary_projects_structured_log_records() {
    // Arrange
    let summary = SyncRunSummary {
        target_outbound_peers: 2,
        maybe_target_header_height: None,
        attempted_peers: 3,
        connected_peers: 2,
        failed_peers: 1,
        messages_processed: 9,
        headers_received: 4,
        blocks_received: 2,
        best_header_height: 44,
        downloaded_block_height: 44,
        best_block_height: 43,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: vec![
            peer_outcome(
                SyncPeerAddress::manual("127.0.0.1", 18_444),
                PeerSyncState::Stalled,
                1,
                Some(PeerFailureReason::Stall),
                None,
            ),
            peer_outcome(
                SyncPeerAddress::manual("203.0.113.10", 18_444),
                PeerSyncState::Failed,
                3,
                Some(PeerFailureReason::Network),
                Some("scripted network failure".to_string()),
            ),
            peer_outcome(
                SyncPeerAddress::manual("198.51.100.9", 18_444),
                PeerSyncState::Connected,
                2,
                None,
                None,
            ),
        ],
        health_signals: vec![
            HealthSignal {
                level: HealthSignalLevel::Warn,
                source: "sync".to_string(),
                message: "headers stalled".to_string(),
            },
            HealthSignal {
                level: HealthSignalLevel::Error,
                source: "storage".to_string(),
                message: "metrics persistence unavailable".to_string(),
            },
        ],
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    };

    // Act
    let records = summary.structured_log_records(1_777_225_099);

    // Assert
    let summary_record = records
        .iter()
        .find(|record| {
            record.level == StructuredLogLevel::Info
                && record.source == "sync"
                && record.message.contains("messages_processed=9")
        })
        .expect("sync summary log record");
    assert!(summary_record.message.contains("headers_received=4"));
    assert!(summary_record.message.contains("blocks_received=2"));
    assert!(summary_record.message.contains("header=44"));
    assert!(summary_record.message.contains("downloaded=44"));
    assert!(summary_record.message.contains("connected=43"));
    assert!(
        summary_record
            .message
            .contains("progress_signal=block_progress")
    );
    assert!(summary_record.message.contains("last_progress=unavailable"));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Warn
            && record.source == "sync"
            && record.message.contains("peer stalled")
    }));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Error
            && record.source == "sync"
            && record.message.contains("peer failed")
    }));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Warn
            && record.source == "sync"
            && record.message.contains("retry attempts=2")
    }));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Error
            && record.source == "storage"
            && record.message == "metrics persistence unavailable"
    }));
    assert!(records.iter().all(|record| record.message.len() <= 192));
    assert!(records.iter().all(|record| {
        !record.message.contains("127.0.0.1")
            && !record.message.contains("203.0.113")
            && !record.message.contains("cookie")
            && !record.message.contains("/tmp/")
    }));
}

#[test]
fn phase62_structured_logs_keep_bounded_cycle_facts() {
    // Arrange
    let mut summary = SyncRunSummary {
        target_outbound_peers: 4,
        maybe_target_header_height: Some(840_123),
        attempted_peers: 3,
        connected_peers: 2,
        failed_peers: 1,
        messages_processed: 9,
        headers_received: 4,
        blocks_received: 2,
        best_header_height: 840_123,
        downloaded_block_height: 840_120,
        best_block_height: 840_119,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
        maybe_stop_reason: Some(SyncStopReason::NoProgress {
            rounds_completed: 2,
        }),
        maybe_reconcile_progress: None,
    };
    summary.peer_outcomes.push(peer_outcome(
        SyncPeerAddress::manual("198.51.100.44", 18_444),
        PeerSyncState::Connected,
        1,
        None,
        None,
    ));

    // Act
    let records = summary.structured_log_records(1_777_225_100);
    let summary_text = records
        .iter()
        .filter(|record| record.source == "sync" && record.level == StructuredLogLevel::Info)
        .map(|record| record.message.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    // Assert
    for expected in [
        "target_outbound_peers=4",
        "target_header_height=840123",
        "attempted_peers=3",
        "connected_peers=2",
        "failed_peers=1",
        "messages_processed=9",
        "headers_received=4",
        "blocks_received=2",
        "header=840123",
        "downloaded=840120",
        "connected=840119",
        "progress_signal=block_progress",
        "last_progress=unavailable",
        "latest_stop_reason=no_progress",
        "recovery_category=unavailable",
    ] {
        assert!(
            summary_text.contains(expected),
            "missing structured log fact: {expected}"
        );
    }
    assert!(records.iter().all(|record| record.message.len() <= 192));
}

#[test]
fn sync_summary_logs_stop_reason_when_available() {
    // Arrange
    let mut summary = SyncRunSummary::empty(0, 0, 1);
    summary.maybe_stop_reason = Some(SyncStopReason::NoProgress {
        rounds_completed: 2,
    });

    // Act
    let records = summary.structured_log_records(1_777_225_101);

    // Assert
    assert!(
        records
            .iter()
            .any(|record| record.message == "sync stop reason=no_progress")
    );
    assert!(records.iter().all(|record| record.message.len() <= 192));
}

#[test]
fn phase62_status_and_structured_logs_agree_on_configured_targets() {
    // Arrange
    let path = temp_store_path("phase62-target-agreement");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            maybe_target_header_height: Some(840_123),
            maybe_log_dir: Some(log_dir.clone()),
            max_rounds: 5,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut summary = SyncRunSummary::empty(840_123, 840_120, 4);
    summary.maybe_stop_reason = Some(SyncStopReason::TargetHeaderReached {
        target_header_height: 840_123,
        best_header_height: 840_123,
    });

    // Act
    runtime.write_summary_logs(&mut summary, 1_777_225_102);
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_102)
        .expect("durable status");
    let records = load_structured_log_records(&log_dir);
    let summary_text = records
        .iter()
        .filter(|record| record.source == "sync")
        .map(|record| record.message.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    // Assert
    let FieldAvailability::Available(configured_targets) = state.sync.configured_targets else {
        panic!("configured targets should be available");
    };
    assert_eq!(configured_targets.maybe_target_header_height, Some(840_123));
    assert_eq!(configured_targets.target_outbound_peers, 4);
    let FieldAvailability::Available(stop_reason) = state.sync.latest_stop_reason else {
        panic!("latest stop reason should be available");
    };
    assert_eq!(stop_reason.label, "target_header_reached");
    assert!(summary_text.contains("target_header_height=840123"));
    assert!(summary_text.contains("latest_stop_reason=target_header_reached"));

    remove_dir_if_exists(&path);
}
