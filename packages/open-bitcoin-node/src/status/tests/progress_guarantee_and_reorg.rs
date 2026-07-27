// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use crate::status::EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON;

#[test]
fn phase78_progress_guarantee_status_contract() {
    // Arrange
    let progress_credit = ProgressCreditEvidence {
        kind: ProgressCreditKind::ValidatedDurableActiveChain,
        credited_validated_active_chain_height: 840_001,
        credited_validated_active_chain_hash: "active-tip".to_string(),
        credited_validated_active_chain_work: "00000000000000000000000000000042".to_string(),
        source_unix_seconds: 1_717_000_000,
        rejected_activity: vec![RejectedProgressActivity {
            kind: RejectedProgressActivityKind::HeaderDownload,
            observed_count: 24,
            reason: "headers do not prove durable active-chain progress".to_string(),
        }],
    };
    let current_credit = ProgressCreditKind::CurrentAtBestKnownTip;
    let progress_window = ProgressWindowEvidence {
        retry_backoff_seconds: 30,
        max_sync_rounds: 8,
        expected_progress_window_seconds: 240,
        tip_freshness_threshold_seconds: 600,
    };
    let threshold = NoProgressThresholdEvidence {
        threshold_seconds: 240,
        elapsed_since_last_useful_work_seconds: 120,
        state: NoProgressThresholdState::WithinWindow,
        evaluated_at_unix_seconds: 1_717_000_120,
    };
    let peer_contribution = PeerContributionEvidence {
        peer: "seed.bitcoin.sipa.be:8333".to_string(),
        maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
        kind: PeerContributionKind::HeadersAndBlocks,
        messages_processed: 6,
        headers_received: 2,
        blocks_received: 1,
        maybe_last_activity_unix_seconds: Some(1_717_000_110),
        maybe_failure_reason_label: None,
    };
    let stall_diagnosis = StallDiagnosisEvidence {
        stalled_subsystem: StalledSubsystem::StorageOrResourcePressure,
        confidence: StallDiagnosisConfidence::High,
        evidence_basis: vec!["resource pressure stopped useful validation".to_string()],
        next_action: "free disk or raise the configured storage budget".to_string(),
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::StorageOrResourceBlocked),
        maybe_recovery_category: None,
        maybe_latest_stop_reason_label: Some("storage_pressure".to_string()),
        source_unix_seconds: 1_717_000_130,
    };
    let legacy_json = serde_json::json!({
        "network": { "state": "available", "value": "mainnet" },
        "chain_tip": { "state": "unavailable", "value": { "reason": "chain tip unavailable" } },
        "sync_progress": { "state": "unavailable", "value": { "reason": "sync progress unavailable" } },
        "lifecycle": { "state": "available", "value": "active" },
        "phase": { "state": "available", "value": "headers" },
        "progress_signal": { "state": "available", "value": "header_progress" },
        "lag": {
            "state": "available",
            "value": { "headers_remaining": 0, "blocks_remaining": 12 }
        },
        "last_successful_progress_unix_seconds": { "state": "available", "value": 1717000000 },
        "last_error": { "state": "unavailable", "value": { "reason": "no sync error recorded" } },
        "recovery_category": { "state": "unavailable", "value": { "reason": "no recovery category recorded" } },
        "recovery_action": { "state": "unavailable", "value": { "reason": "no recovery action required" } },
        "resource_pressure": { "state": "unavailable", "value": { "reason": "resource pressure unavailable" } }
    });

    // Act
    let credit_labels = serde_json::to_value([
        ProgressCreditKind::ValidatedDurableActiveChain,
        ProgressCreditKind::CurrentAtBestKnownTip,
    ])
    .expect("credit labels json");
    let rejected_labels = serde_json::to_value([
        RejectedProgressActivityKind::HeaderDownload,
        RejectedProgressActivityKind::BlockDownload,
        RejectedProgressActivityKind::InFlightRequest,
        RejectedProgressActivityKind::PeerMessage,
        RejectedProgressActivityKind::ReportProjection,
        RejectedProgressActivityKind::Retry,
    ])
    .expect("rejected labels json");
    let threshold_labels = serde_json::to_value([
        NoProgressThresholdState::WithinWindow,
        NoProgressThresholdState::Exceeded,
    ])
    .expect("threshold labels json");
    let peer_labels = serde_json::to_value([
        PeerContributionKind::HeadersOnly,
        PeerContributionKind::BlocksOnly,
        PeerContributionKind::HeadersAndBlocks,
        PeerContributionKind::MessagesOnly,
        PeerContributionKind::NoUsefulContribution,
        PeerContributionKind::Failure,
    ])
    .expect("peer contribution labels json");
    let stalled_labels = serde_json::to_value([
        StalledSubsystem::PublicNetworkReachability,
        StalledSubsystem::IncompatiblePeers,
        StalledSubsystem::SlowOrStalledPeers,
        StalledSubsystem::PeerFailuresExhausted,
        StalledSubsystem::StaleInflightCleanup,
        StalledSubsystem::BranchCompetitionAwaitingBodies,
        StalledSubsystem::Validation,
        StalledSubsystem::StorageOrResourcePressure,
        StalledSubsystem::AtTipWaiting,
        StalledSubsystem::OperatorStop,
        StalledSubsystem::LocalShutdown,
        StalledSubsystem::Unknown,
    ])
    .expect("stalled subsystem labels json");
    let confidence_labels = serde_json::to_value([
        StallDiagnosisConfidence::High,
        StallDiagnosisConfidence::Medium,
        StallDiagnosisConfidence::Low,
    ])
    .expect("stall confidence labels json");
    let encoded_credit = serde_json::to_value(&progress_credit).expect("progress credit json");
    let encoded_current_credit = serde_json::to_value(current_credit).expect("current credit json");
    let encoded_window = serde_json::to_value(progress_window).expect("progress window json");
    let encoded_threshold = serde_json::to_value(threshold).expect("threshold json");
    let encoded_peer = serde_json::to_value(peer_contribution).expect("peer contribution json");
    let encoded_stall = serde_json::to_value(stall_diagnosis).expect("stall diagnosis json");
    let unavailable_credit = FieldAvailability::<ProgressCreditEvidence>::unavailable(
        PROGRESS_CREDIT_UNAVAILABLE_REASON,
    );
    let encoded_unavailable =
        serde_json::to_value(unavailable_credit).expect("unavailable credit json");
    let legacy_sync: SyncStatus = serde_json::from_value(legacy_json).expect("legacy sync json");

    // Assert
    assert_eq!(
        credit_labels,
        serde_json::json!([
            "validated_durable_active_chain",
            "current_at_best_known_tip"
        ])
    );
    for rejected_only_label in [
        "header_download",
        "block_download",
        "in_flight_request",
        "report_projection",
        "retry",
    ] {
        assert!(
            !credit_labels
                .as_array()
                .expect("credit labels array")
                .iter()
                .any(|value| value == rejected_only_label)
        );
    }
    assert_eq!(
        rejected_labels,
        serde_json::json!([
            "header_download",
            "block_download",
            "in_flight_request",
            "peer_message",
            "report_projection",
            "retry"
        ])
    );
    assert_eq!(
        threshold_labels,
        serde_json::json!(["within_window", "exceeded"])
    );
    assert_eq!(
        peer_labels,
        serde_json::json!([
            "headers_only",
            "blocks_only",
            "headers_and_blocks",
            "messages_only",
            "no_useful_contribution",
            "failure"
        ])
    );
    assert_eq!(
        stalled_labels,
        serde_json::json!([
            "public_network_reachability",
            "incompatible_peers",
            "slow_or_stalled_peers",
            "peer_failures_exhausted",
            "stale_inflight_cleanup",
            "branch_competition_awaiting_bodies",
            "validation",
            "storage_or_resource_pressure",
            "at_tip_waiting",
            "operator_stop",
            "local_shutdown",
            "unknown"
        ])
    );
    assert_eq!(
        confidence_labels,
        serde_json::json!(["high", "medium", "low"])
    );
    assert_eq!(encoded_credit["kind"], "validated_durable_active_chain");
    assert_eq!(encoded_current_credit, "current_at_best_known_tip");
    assert_eq!(
        encoded_credit["rejected_activity"][0]["kind"],
        "header_download"
    );
    assert_eq!(encoded_window["expected_progress_window_seconds"], 240);
    assert_eq!(encoded_threshold["state"], "within_window");
    assert_eq!(encoded_peer["kind"], "headers_and_blocks");
    assert_eq!(
        encoded_stall["stalled_subsystem"],
        "storage_or_resource_pressure"
    );
    assert_eq!(encoded_stall["confidence"], "high");
    assert_eq!(encoded_unavailable["state"], "unavailable");
    assert_eq!(
        encoded_unavailable["value"]["reason"],
        PROGRESS_CREDIT_UNAVAILABLE_REASON
    );
    assert_eq!(
        legacy_sync.progress_credit,
        FieldAvailability::<ProgressCreditEvidence>::unavailable(
            PROGRESS_CREDIT_UNAVAILABLE_REASON
        )
    );
    assert_eq!(
        legacy_sync.expected_progress_window,
        FieldAvailability::<ProgressWindowEvidence>::unavailable(
            EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON
        )
    );
    assert_eq!(
        legacy_sync.no_progress_threshold,
        FieldAvailability::<NoProgressThresholdEvidence>::unavailable(
            NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON
        )
    );
    assert_eq!(
        legacy_sync.last_useful_work,
        FieldAvailability::<ProgressCreditEvidence>::unavailable(
            LAST_USEFUL_WORK_UNAVAILABLE_REASON
        )
    );
    assert_eq!(
        legacy_sync.last_peer_contribution,
        FieldAvailability::<PeerContributionEvidence>::unavailable(
            LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON
        )
    );
    assert_eq!(
        legacy_sync.stall_diagnosis,
        FieldAvailability::<StallDiagnosisEvidence>::unavailable(
            STALL_DIAGNOSIS_UNAVAILABLE_REASON
        )
    );
}

#[test]
fn phase70_sync_reorg_evidence_defaults_legacy_sync_status_json() {
    // Arrange
    let legacy_json = serde_json::json!({
        "network": { "state": "available", "value": "mainnet" },
        "chain_tip": { "state": "unavailable", "value": { "reason": "chain tip unavailable" } },
        "sync_progress": { "state": "unavailable", "value": { "reason": "sync progress unavailable" } },
        "lifecycle": { "state": "available", "value": "active" },
        "phase": { "state": "available", "value": "headers" },
        "progress_signal": { "state": "available", "value": "header_progress" },
        "lag": {
            "state": "available",
            "value": { "headers_remaining": 0, "blocks_remaining": 12 }
        },
        "last_successful_progress_unix_seconds": { "state": "available", "value": 1717000000 },
        "last_error": { "state": "unavailable", "value": { "reason": "no sync error recorded" } },
        "recovery_category": { "state": "unavailable", "value": { "reason": "no recovery category recorded" } },
        "recovery_action": { "state": "unavailable", "value": { "reason": "no recovery action required" } },
        "resource_pressure": { "state": "unavailable", "value": { "reason": "resource pressure unavailable" } }
    });

    // Act
    let sync: SyncStatus = serde_json::from_value(legacy_json).expect("legacy sync status json");

    // Assert
    assert_eq!(
        sync.latest_reorg,
        FieldAvailability::<SyncReorgEvidence>::unavailable("no reorg evidence recorded")
    );
    assert_eq!(
        sync.reconcile_progress,
        FieldAvailability::<SyncReconcileProgressStatus>::unavailable(
            "reconcile progress unavailable"
        )
    );
}

#[test]
fn phase70_sync_reorg_evidence_serializes_bounded_field_names() {
    // Arrange
    let evidence = SyncReorgEvidence {
        common_ancestor_height: 840_000,
        common_ancestor_hash: "ancestor".to_string(),
        disconnected_count: 2,
        connected_count: 3,
        final_active_height: 840_003,
        final_active_hash: "final-active".to_string(),
        fully_persisted: true,
    };

    // Act
    let encoded = serde_json::to_value(evidence).expect("reorg evidence json");

    // Assert
    assert_eq!(encoded["common_ancestor_height"], 840_000);
    assert_eq!(encoded["common_ancestor_hash"], "ancestor");
    assert_eq!(encoded["disconnected_count"], 2);
    assert_eq!(encoded["connected_count"], 3);
    assert_eq!(encoded["final_active_height"], 840_003);
    assert_eq!(encoded["final_active_hash"], "final-active");
    assert_eq!(encoded["fully_persisted"], true);
}

#[test]
fn phase70_sync_reorg_evidence_reconcile_progress_omits_raw_payloads() {
    // Arrange
    let progress = SyncReconcileProgressStatus::BranchCompetitionAwaitingBodies {
        common_ancestor_height: 840_000,
        common_ancestor_hash: "ancestor".to_string(),
        branch_tip_height: 840_004,
        branch_tip_hash: "branch-tip".to_string(),
        missing_block_count: 4,
    };

    // Act
    let encoded = serde_json::to_value(progress).expect("reconcile progress json");
    let encoded_text = encoded.to_string();

    // Assert
    assert_eq!(encoded["state"], "branch_competition_awaiting_bodies");
    assert_eq!(encoded["details"]["common_ancestor_height"], 840_000);
    assert_eq!(encoded["details"]["common_ancestor_hash"], "ancestor");
    assert_eq!(encoded["details"]["branch_tip_height"], 840_004);
    assert_eq!(encoded["details"]["branch_tip_hash"], "branch-tip");
    assert_eq!(encoded["details"]["missing_block_count"], 4);
    assert!(!encoded_text.contains("undo"));
    assert!(!encoded_text.contains("block_body"));
    assert!(!encoded_text.contains("raw"));
}
