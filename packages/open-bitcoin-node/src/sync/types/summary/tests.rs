// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp

use crate::{
    FieldAvailability, MetricKind, MetricSample, SyncStopReason,
    status::{PeerCounts, SyncProgressSignal, SyncRecoveryCategory},
    sync::{
        PeerContribution, PeerFailureReason, PeerSyncOutcome, PeerSyncState, SyncNetwork,
        SyncPeerAddress, SyncRunSummary,
    },
};

#[test]
fn phase62_sync_truth_contract_projects_summary_fields() {
    // Arrange
    let mut summary = SyncRunSummary::empty(840_123, 840_120, 4);
    summary.maybe_target_header_height = Some(840_123);
    summary.attempted_peers = 3;
    summary.connected_peers = 2;
    summary.failed_peers = 1;
    summary.maybe_stop_reason = Some(SyncStopReason::TargetHeaderReached {
        target_header_height: 840_123,
        best_header_height: 840_123,
    });

    // Act
    let status = summary.sync_status(SyncNetwork::Mainnet);

    // Assert
    let FieldAvailability::Available(configured_targets) = status.configured_targets else {
        panic!("configured targets should be available");
    };
    assert_eq!(configured_targets.target_outbound_peers, 4);
    assert_eq!(configured_targets.maybe_target_header_height, Some(840_123));
    let FieldAvailability::Available(attempt_counters) = status.attempt_counters else {
        panic!("attempt counters should be available");
    };
    assert_eq!(attempt_counters.attempted_peers, 3);
    assert_eq!(attempt_counters.connected_peers, 2);
    assert_eq!(attempt_counters.failed_peers, 1);
    assert_eq!(attempt_counters.max_sync_rounds, 0);
    let FieldAvailability::Available(stop_reason) = status.latest_stop_reason else {
        panic!("latest stop reason should be available");
    };
    assert_eq!(stop_reason.label, "target_header_reached");
    assert!(stop_reason.message.contains("target_header_height=840123"));
}

#[test]
fn sync_summary_projects_consistent_operator_evidence_fields() {
    // Arrange
    let latest_error = "peer stalled before block connect";
    let summary = SyncRunSummary {
        target_outbound_peers: 4,
        maybe_target_header_height: None,
        attempted_peers: 2,
        connected_peers: 2,
        failed_peers: 0,
        messages_processed: 7,
        headers_received: 3,
        blocks_received: 1,
        best_header_height: 840_100,
        downloaded_block_height: 840_006,
        best_block_height: 840_004,
        maybe_downloaded_block_hash: Some("22".repeat(32)),
        maybe_connected_block_hash: Some("11".repeat(32)),
        maybe_validated_active_chain_work: Some("840005".to_string()),
        peer_outcomes: vec![PeerSyncOutcome {
            peer: SyncPeerAddress::manual("seed.bitcoin.sipa.be", 8_333),
            maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
            network: SyncNetwork::Mainnet,
            state: PeerSyncState::Stalled,
            attempts: 1,
            contribution: PeerContribution {
                messages_processed: 7,
                headers_received: 3,
                blocks_received: 1,
            },
            maybe_tip_height: None,
            maybe_tip_hash: None,
            maybe_tip_work: None,
            maybe_last_activity_unix_seconds: Some(1_717_000_000),
            maybe_capabilities: None,
            maybe_failure_reason: Some(PeerFailureReason::Stall),
            maybe_error: Some(latest_error.to_string()),
        }],
        health_signals: Vec::new(),
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    };

    // Act
    let status = summary.sync_status(SyncNetwork::Mainnet);
    let peer_status = summary.peer_status();
    let samples = summary.metric_samples(1_717_000_000);
    let records = summary.structured_log_records(1_717_000_000);

    // Assert
    let FieldAvailability::Available(progress) = status.sync_progress else {
        panic!("sync progress should be available");
    };
    assert_eq!(progress.header_height, 840_100);
    assert_eq!(progress.downloaded_block_height, 840_006);
    assert_eq!(progress.connected_block_height, 840_004);
    assert_eq!(progress.block_height, 840_004);
    assert_eq!(progress.validated_active_chain_height, 840_004);
    assert_eq!(progress.maybe_downloaded_block_hash, Some("22".repeat(32)));
    assert_eq!(progress.maybe_connected_block_hash, Some("11".repeat(32)));
    assert_eq!(
        progress.maybe_validated_active_chain_hash,
        Some("11".repeat(32))
    );
    assert_eq!(
        progress.maybe_validated_active_chain_work,
        Some("840005".to_string())
    );
    assert_eq!(
        status.progress_signal,
        FieldAvailability::available(SyncProgressSignal::BlockProgress)
    );
    assert_eq!(
        status.last_error,
        FieldAvailability::available(latest_error.to_string())
    );
    assert_eq!(
        summary.latest_recovery_category(),
        Some(SyncRecoveryCategory::PublicNetworkUnreachable)
    );
    assert_eq!(
        status.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::PublicNetworkUnreachable)
    );
    assert_eq!(
        peer_status.peer_counts,
        FieldAvailability::available(PeerCounts {
            inbound: 0,
            outbound: 2,
        })
    );
    assert_eq!(
        samples,
        vec![
            MetricSample::new(MetricKind::HeaderHeight, 840_100.0, 1_717_000_000),
            MetricSample::new(MetricKind::DownloadedBlockHeight, 840_006.0, 1_717_000_000,),
            MetricSample::new(MetricKind::ConnectedBlockHeight, 840_004.0, 1_717_000_000,),
            MetricSample::new(
                MetricKind::ValidatedActiveChainHeight,
                840_004.0,
                1_717_000_000,
            ),
            MetricSample::new(MetricKind::SyncHeight, 840_004.0, 1_717_000_000),
            MetricSample::new(MetricKind::PeerCount, 2.0, 1_717_000_000),
        ]
    );
    assert!(records.iter().any(|record| {
        record.message.contains(
            "header=840100 downloaded=840006 connected=840004 progress_signal=block_progress",
        )
    }));
    assert!(records.iter().any(|record| {
        record
            .message
            .contains("recovery_category=public_network_unreachable")
    }));
    assert!(
        records
            .iter()
            .any(|record| record.message.contains("peer stalled"))
    );
}

#[test]
fn phase72_summary_metrics_and_logs_carry_full_sync_truth_dimensions() {
    // Arrange
    let summary = SyncRunSummary {
        target_outbound_peers: 4,
        maybe_target_header_height: Some(840_004),
        attempted_peers: 4,
        connected_peers: 3,
        failed_peers: 1,
        messages_processed: 128,
        headers_received: 4,
        blocks_received: 4,
        best_header_height: 840_004,
        downloaded_block_height: 840_004,
        best_block_height: 840_004,
        maybe_downloaded_block_hash: Some("11".repeat(32)),
        maybe_connected_block_hash: Some("11".repeat(32)),
        maybe_validated_active_chain_work: Some("840005".to_string()),
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
        maybe_stop_reason: Some(SyncStopReason::TargetHeaderReached {
            target_header_height: 840_004,
            best_header_height: 840_004,
        }),
        maybe_reconcile_progress: None,
    };

    // Act
    let samples = summary.metric_samples(1_717_000_000);
    let records = summary.structured_log_records(1_717_000_000);

    // Assert
    assert!(samples.iter().any(|sample| {
        sample.kind == MetricKind::ValidatedActiveChainHeight
            && sample.value == 840_004.0
            && sample.timestamp_unix_seconds == 1_717_000_000
    }));
    let combined = records
        .iter()
        .map(|record| record.message.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for expected in [
        "validated_active_chain_height=840004",
        "validated_active_chain_work=840005",
        "resource_pressure_blocks_in_flight=0",
        "resource_pressure_target_outbound_peers=4",
        "peer_contribution_connected=3",
        "peer_contribution_failed=1",
        "latest_stop_reason=target_header_reached",
        "recovery_category=unavailable",
    ] {
        assert!(combined.contains(expected), "missing {expected}");
    }
}

#[test]
fn stop_reason_projection_includes_operator_pause_and_shutdown_labels() {
    // Arrange
    let mut paused_summary = SyncRunSummary::empty(0, 0, 1);
    paused_summary.maybe_stop_reason = Some(SyncStopReason::OperatorPaused);
    paused_summary
        .health_signals
        .push(SyncStopReason::OperatorPaused.health_signal());
    let mut stopped_summary = SyncRunSummary::empty(0, 0, 1);
    stopped_summary.maybe_stop_reason = Some(SyncStopReason::ShutdownRequested);
    stopped_summary
        .health_signals
        .push(SyncStopReason::ShutdownRequested.health_signal());

    // Act
    let paused_status = paused_summary.sync_status(SyncNetwork::Mainnet);
    let stopped_status = stopped_summary.sync_status(SyncNetwork::Mainnet);
    let paused_records = paused_summary.structured_log_records(1_717_000_001);
    let stopped_records = stopped_summary.structured_log_records(1_717_000_002);

    // Assert
    assert_eq!(
        paused_status.phase,
        FieldAvailability::available("operator_paused".to_string())
    );
    assert_eq!(
        stopped_status.phase,
        FieldAvailability::available("shutdown_requested".to_string())
    );
    assert_eq!(
        paused_status.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::OperatorCancellation)
    );
    assert_eq!(
        stopped_status.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::OperatorCancellation)
    );
    assert!(paused_records.iter().any(|record| {
        record
            .message
            .contains("recovery_category=operator_cancellation")
    }));
    assert!(
        paused_records
            .iter()
            .any(|record| { record.message.contains("sync stop reason=operator_paused") })
    );
    assert!(stopped_records.iter().any(|record| {
        record
            .message
            .contains("recovery_category=operator_cancellation")
    }));
    assert!(stopped_records.iter().any(|record| {
        record
            .message
            .contains("sync stop reason=shutdown_requested")
    }));
}

#[test]
fn sync_summary_structured_logs_mark_recovery_category_unavailable() {
    // Arrange
    let summary = SyncRunSummary::empty(840_000, 839_999, 2);

    // Act
    let status = summary.sync_status(SyncNetwork::Mainnet);
    let records = summary.structured_log_records(1_717_000_003);

    // Assert
    assert_eq!(
        status.recovery_category,
        FieldAvailability::unavailable("no recovery category recorded")
    );
    assert!(
        records
            .iter()
            .any(|record| { record.message.contains("recovery_category=unavailable") })
    );
}
