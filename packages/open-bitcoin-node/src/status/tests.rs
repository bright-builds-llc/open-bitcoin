// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    BestKnownTipStatus, BuildProvenance, ChainTipStatus, ConfigStatus, FieldAvailability,
    HealthSignal, HealthSignalLevel, INBOUND_STATUS_UNAVAILABLE_REASON,
    InboundAddressDecisionEvent, InboundAddressEvidenceEntry, InboundHandshakeStatusCounts,
    InboundPeerServingStatus, LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON,
    LAST_USEFUL_WORK_UNAVAILABLE_REASON, MempoolStatus, NO_PROGRESS_DIAGNOSIS_UNAVAILABLE_REASON,
    NO_PROGRESS_NEXT_ACTION_UNAVAILABLE_REASON, NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON,
    NoProgressDiagnosis, NoProgressThresholdEvidence, NoProgressThresholdState, NodeRuntimeState,
    NodeStatus, OpenBitcoinStatusSnapshot, PROGRESS_CREDIT_UNAVAILABLE_REASON,
    PeerContributionEvidence, PeerContributionKind, PeerCounts, PeerStatus, PeerTelemetry,
    ProgressCreditEvidence, ProgressCreditKind, ProgressWindowEvidence,
    RESOURCE_BOUND_STOP_PERCENT, RESOURCE_BOUND_WARNING_PERCENT, RejectedProgressActivity,
    RejectedProgressActivityKind, ResourceBoundEntry, ResourceBoundKind, ResourceBoundSnapshot,
    ResourceBoundUnit, ResourcePressureState, STALL_DIAGNOSIS_UNAVAILABLE_REASON,
    ServiceLifecycleStatus, ServicePriorShutdownStatus, ServiceRestartResumeStatus,
    ServiceStaleInflightStatus, ServiceStatus, StallDiagnosisConfidence, StallDiagnosisEvidence,
    StalledSubsystem, StayCurrentStatus, SyncAttemptCounters, SyncConfiguredTargets, SyncLagStatus,
    SyncLifecycleState, SyncProgress, SyncProgressSignal, SyncReconcileProgressStatus,
    SyncReorgEvidence, SyncResourcePressure, SyncStatus, SyncStopReasonStatus, WalletFreshness,
    WalletScanProgress, WalletStatus, classify_budget_pressure,
    classify_snapshot_against_disk_budget, usage_against_budget,
};
use crate::recovery::{
    LockEvidence, LockEvidenceKind, RECOVERY_EVIDENCE_UNAVAILABLE_REASON, RecoveryActionClass,
    RecoveryCause, RecoveryEvidenceSnapshot,
};
use crate::{LogStatus, MetricsStatus};

#[test]
fn unavailable_field_serializes_with_reason() {
    // Arrange
    let value = FieldAvailability::<String>::unavailable("node stopped");

    // Act
    let encoded = serde_json::to_value(&value).expect("availability json");

    // Assert
    assert_eq!(encoded["state"], "unavailable");
    assert_eq!(encoded["value"]["reason"], "node stopped");
}

#[test]
fn unavailable_build_provenance_keeps_missing_fields_visible() {
    // Arrange / Act
    let provenance = BuildProvenance::unavailable();
    let encoded = serde_json::to_value(provenance).expect("provenance json");

    // Assert
    assert_eq!(encoded["commit"]["state"], "unavailable");
    assert_eq!(encoded["build_time"]["state"], "unavailable");
    assert_eq!(encoded["target"]["state"], "unavailable");
}

#[test]
fn phase62_sync_truth_contract() {
    // Arrange
    let sync = SyncStatus {
        network: FieldAvailability::available("mainnet".to_string()),
        chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
        sync_progress: FieldAvailability::unavailable("sync progress unavailable"),
        lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
        phase: FieldAvailability::available("headers".to_string()),
        configured_targets: FieldAvailability::available(SyncConfiguredTargets {
            target_outbound_peers: 4,
            maybe_target_header_height: Some(840_123),
        }),
        attempt_counters: FieldAvailability::available(SyncAttemptCounters {
            attempted_peers: 3,
            connected_peers: 2,
            failed_peers: 1,
            max_sync_rounds: 8,
        }),
        progress_signal: FieldAvailability::available(SyncProgressSignal::HeaderProgress),
        lag: FieldAvailability::available(SyncLagStatus {
            headers_remaining: 0,
            blocks_remaining: 12,
        }),
        last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_000),
        progress_credit: FieldAvailability::unavailable(PROGRESS_CREDIT_UNAVAILABLE_REASON),
        expected_progress_window: FieldAvailability::unavailable(
            super::EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON,
        ),
        no_progress_threshold: FieldAvailability::unavailable(
            NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON,
        ),
        last_useful_work: FieldAvailability::unavailable(LAST_USEFUL_WORK_UNAVAILABLE_REASON),
        last_peer_contribution: FieldAvailability::unavailable(
            LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON,
        ),
        stall_diagnosis: FieldAvailability::unavailable(STALL_DIAGNOSIS_UNAVAILABLE_REASON),
        latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
            label: "target_header_reached".to_string(),
            message: "sync header target reached".to_string(),
        }),
        last_error: FieldAvailability::unavailable("no sync error recorded"),
        recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
        recovery_action: FieldAvailability::unavailable("no recovery action required"),
        resource_pressure: FieldAvailability::unavailable("resource pressure unavailable"),
        best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
            "best-known tip evidence unavailable",
        ),
        stay_current: FieldAvailability::unavailable("stay-current state unavailable"),
        stay_current_next_action: FieldAvailability::unavailable(
            "stay-current next action unavailable",
        ),
        no_progress_diagnosis: FieldAvailability::unavailable(
            NO_PROGRESS_DIAGNOSIS_UNAVAILABLE_REASON,
        ),
        no_progress_next_action: FieldAvailability::unavailable(
            NO_PROGRESS_NEXT_ACTION_UNAVAILABLE_REASON,
        ),
        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
        reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
    };

    // Act
    let encoded = serde_json::to_value(&sync).expect("sync status json");
    let legacy_sync: SyncStatus = serde_json::from_value(serde_json::json!({
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
    }))
    .expect("legacy sync status json");

    // Assert
    assert_eq!(encoded["configured_targets"]["state"], "available");
    assert_eq!(
        encoded["configured_targets"]["value"]["target_outbound_peers"],
        4
    );
    assert_eq!(
        encoded["configured_targets"]["value"]["maybe_target_header_height"],
        840_123
    );
    assert_eq!(encoded["attempt_counters"]["value"]["attempted_peers"], 3);
    assert_eq!(encoded["attempt_counters"]["value"]["max_sync_rounds"], 8);
    assert_eq!(
        encoded["latest_stop_reason"]["value"]["label"],
        "target_header_reached"
    );
    assert_eq!(encoded["progress_credit"]["state"], "unavailable");
    assert_eq!(encoded["expected_progress_window"]["state"], "unavailable");
    assert_eq!(encoded["no_progress_threshold"]["state"], "unavailable");
    assert_eq!(encoded["last_useful_work"]["state"], "unavailable");
    assert_eq!(encoded["last_peer_contribution"]["state"], "unavailable");
    assert_eq!(encoded["stall_diagnosis"]["state"], "unavailable");
    assert_eq!(encoded["best_known_tip"]["state"], "unavailable");
    assert_eq!(encoded["stay_current"]["state"], "unavailable");
    assert_eq!(encoded["stay_current_next_action"]["state"], "unavailable");
    assert_eq!(encoded["no_progress_diagnosis"]["state"], "unavailable");
    assert_eq!(encoded["no_progress_next_action"]["state"], "unavailable");
    assert_eq!(
        legacy_sync.configured_targets,
        FieldAvailability::unavailable("configured targets unavailable")
    );
    assert_eq!(
        legacy_sync.attempt_counters,
        FieldAvailability::unavailable("attempt counters unavailable")
    );
    assert_eq!(
        legacy_sync.latest_stop_reason,
        FieldAvailability::unavailable("latest stop reason unavailable")
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
            super::EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON
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
    assert_eq!(
        legacy_sync.best_known_tip,
        FieldAvailability::unavailable("best-known tip evidence unavailable")
    );
    assert_eq!(
        legacy_sync.stay_current,
        FieldAvailability::<StayCurrentStatus>::unavailable("stay-current state unavailable")
    );
    assert_eq!(
        legacy_sync.stay_current_next_action,
        FieldAvailability::unavailable("stay-current next action unavailable")
    );
    assert_eq!(
        legacy_sync.no_progress_diagnosis,
        FieldAvailability::<NoProgressDiagnosis>::unavailable(
            NO_PROGRESS_DIAGNOSIS_UNAVAILABLE_REASON
        )
    );
    assert_eq!(
        legacy_sync.no_progress_next_action,
        FieldAvailability::unavailable(NO_PROGRESS_NEXT_ACTION_UNAVAILABLE_REASON)
    );
}

#[test]
fn phase70_no_progress_status_contract_serializes_exact_labels() {
    // Arrange
    let cases = [
        (
            NoProgressDiagnosis::CurrentAtBestKnownTip,
            "current_at_best_known_tip",
        ),
        (
            NoProgressDiagnosis::BehindAwaitingHeaders,
            "behind_awaiting_headers",
        ),
        (
            NoProgressDiagnosis::AwaitingBlockBodies,
            "awaiting_block_bodies",
        ),
        (
            NoProgressDiagnosis::StaleInflightCleanup,
            "stale_inflight_cleanup",
        ),
        (NoProgressDiagnosis::PeerBackoff, "peer_backoff"),
        (NoProgressDiagnosis::PeerStalled, "peer_stalled"),
        (
            NoProgressDiagnosis::PeerFailuresExhausted,
            "peer_failures_exhausted",
        ),
        (
            NoProgressDiagnosis::BranchCompetitionAwaitingBodies,
            "branch_competition_awaiting_bodies",
        ),
        (
            NoProgressDiagnosis::RecoveringFromReorgOrStorage,
            "recovering_from_reorg_or_storage",
        ),
        (
            NoProgressDiagnosis::StorageOrResourceBlocked,
            "storage_or_resource_blocked",
        ),
    ];

    // Act / Assert
    for (diagnosis, expected_label) in cases {
        let encoded = serde_json::to_value(diagnosis).expect("diagnosis json");
        assert_eq!(encoded, expected_label);
    }
}

#[test]
fn phase70_no_progress_status_contract_defaults_legacy_json() {
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
        sync.no_progress_diagnosis,
        FieldAvailability::<NoProgressDiagnosis>::unavailable(
            NO_PROGRESS_DIAGNOSIS_UNAVAILABLE_REASON
        )
    );
    assert_eq!(
        sync.no_progress_next_action,
        FieldAvailability::unavailable(NO_PROGRESS_NEXT_ACTION_UNAVAILABLE_REASON)
    );
}

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
            super::EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON
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

#[test]
fn phase63_service_lifecycle_status_contract_serializes_labels() {
    // Arrange
    let installed_stopped = ServiceLifecycleStatus::InstalledStopped;
    let unavailable_manager = ServiceLifecycleStatus::UnavailableManager;

    // Act
    let installed_stopped_json =
        serde_json::to_value(installed_stopped).expect("installed-stopped status json");
    let unavailable_manager_json =
        serde_json::to_value(unavailable_manager).expect("unavailable-manager status json");

    // Assert
    assert_eq!(installed_stopped.as_str(), "installed-stopped");
    assert_eq!(unavailable_manager.as_str(), "unavailable-manager");
    assert_eq!(installed_stopped_json, "installed-stopped");
    assert_eq!(unavailable_manager_json, "unavailable-manager");
}

#[test]
fn phase63_service_lifecycle_status_contract_defaults_legacy_json() {
    // Arrange
    let legacy_json = serde_json::json!({
        "manager": { "state": "available", "value": "launchd" },
        "installed": { "state": "available", "value": true },
        "enabled": { "state": "available", "value": true },
        "running": { "state": "available", "value": false }
    });

    // Act
    let service: ServiceStatus =
        serde_json::from_value(legacy_json).expect("legacy service status json");

    // Assert
    assert_eq!(
        service.lifecycle,
        FieldAvailability::unavailable("service lifecycle unavailable")
    );
    assert_eq!(
        service.service_file_path,
        FieldAvailability::unavailable("service file path unavailable")
    );
    assert_eq!(
        service.log_path,
        FieldAvailability::unavailable("service log path unavailable")
    );
    assert_eq!(
        service.diagnostics,
        FieldAvailability::unavailable("service diagnostics unavailable")
    );
}

#[test]
fn service_restart_resume_status_contract_serializes_labels() {
    // Arrange
    let clean_shutdown = ServicePriorShutdownStatus::Clean;
    let unclean_shutdown = ServicePriorShutdownStatus::Unclean;
    let stale_inflight = ServiceStaleInflightStatus::StaleRequestsRecorded;

    // Act
    let clean_json = serde_json::to_value(clean_shutdown).expect("clean shutdown json");
    let unclean_json = serde_json::to_value(unclean_shutdown).expect("unclean shutdown json");
    let stale_json = serde_json::to_value(stale_inflight).expect("stale in-flight json");

    // Assert
    assert_eq!(clean_shutdown.as_str(), "clean");
    assert_eq!(unclean_shutdown.as_str(), "unclean");
    assert_eq!(stale_inflight.as_str(), "stale_requests_recorded");
    assert_eq!(clean_json, "clean");
    assert_eq!(unclean_json, "unclean");
    assert_eq!(stale_json, "stale_requests_recorded");
}

#[test]
fn service_restart_resume_status_contract_defaults_legacy_json() {
    // Arrange
    let legacy_json = serde_json::json!({
        "manager": { "state": "available", "value": "launchd" },
        "lifecycle": { "state": "available", "value": "running" },
        "installed": { "state": "available", "value": true },
        "enabled": { "state": "available", "value": true },
        "running": { "state": "available", "value": true },
        "service_file_path": { "state": "available", "value": "/tmp/open-bitcoin-node.service" },
        "log_path": { "state": "available", "value": "/tmp/logs/open-bitcoin.log" },
        "diagnostics": { "state": "unavailable", "value": { "reason": "service diagnostics unavailable" } }
    });

    // Act
    let service: ServiceStatus =
        serde_json::from_value(legacy_json).expect("legacy service status json");

    // Assert
    assert_eq!(
        service.restart_resume,
        FieldAvailability::<ServiceRestartResumeStatus>::unavailable(
            "service restart/resume evidence unavailable"
        )
    );
}

#[test]
fn stopped_node_snapshot_keeps_unavailable_live_fields_explicit() {
    // Arrange / Act
    let snapshot = stopped_snapshot();
    let encoded = serde_json::to_value(&snapshot).expect("snapshot json");

    // Assert
    assert_eq!(snapshot.node.state, NodeRuntimeState::Stopped);
    assert_eq!(encoded["sync"]["network"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["chain_tip"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["sync_progress"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["lifecycle"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["phase"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["configured_targets"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["attempt_counters"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["progress_signal"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["lag"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["last_successful_progress_unix_seconds"]["state"],
        "unavailable"
    );
    assert_eq!(
        encoded["sync"]["latest_stop_reason"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["last_error"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["recovery_category"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["recovery_action"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["resource_pressure"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["peer_counts"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["recent_peers"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["inbound"]["state"], "unavailable");
    assert_eq!(encoded["mempool"]["transactions"]["state"], "unavailable");
    assert_eq!(
        encoded["wallet"]["trusted_balance_sats"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["wallet"]["freshness"]["state"], "unavailable");
    assert_eq!(encoded["wallet"]["scan_progress"]["state"], "unavailable");
    assert_eq!(encoded["config"]["datadir"]["state"], "available");
    assert_eq!(encoded["logs"]["retention"]["max_files"], 14);
    assert_eq!(
        encoded["metrics"]["retention"]["sample_interval_seconds"],
        30
    );
    assert_eq!(encoded["resource_bounds"]["state"], "unavailable");
}

#[test]
fn resource_bounds_classify_thresholds_and_full_kind_set() {
    // Arrange
    let kinds = ResourceBoundKind::ALL
        .into_iter()
        .map(ResourceBoundKind::as_str)
        .collect::<Vec<_>>();

    // Act
    let normal = classify_budget_pressure(79, 100);
    let warning = classify_budget_pressure(80, 100);
    let stop_required = classify_budget_pressure(95, 100);
    let zero_limit = classify_budget_pressure(0, 0);

    // Assert
    assert_eq!(RESOURCE_BOUND_WARNING_PERCENT, 80);
    assert_eq!(RESOURCE_BOUND_STOP_PERCENT, 95);
    assert_eq!(
        kinds,
        vec![
            "disk",
            "file",
            "cache",
            "queue",
            "peer",
            "in_flight",
            "log",
            "metric",
            "support_bundle"
        ]
    );
    assert_eq!(normal, ResourcePressureState::Normal);
    assert_eq!(warning, ResourcePressureState::Warning);
    assert_eq!(stop_required, ResourcePressureState::StopRequired);
    assert_eq!(zero_limit, ResourcePressureState::StopRequired);
}

#[test]
fn resource_bounds_snapshot_aggregates_pressure_and_disk_budget() {
    // Arrange
    let snapshot = ResourceBoundSnapshot::new(vec![
        ResourceBoundEntry::available(
            ResourceBoundKind::Disk,
            "datadir disk budget",
            usage_against_budget(
                95,
                100,
                ResourceBoundUnit::Bytes,
                "Free disk space before continuing.",
            ),
        ),
        ResourceBoundEntry::available(
            ResourceBoundKind::Log,
            "structured log retention",
            usage_against_budget(10, 100, ResourceBoundUnit::Bytes, "Review log retention."),
        ),
    ]);

    // Act
    let encoded = serde_json::to_value(&snapshot).expect("resource bounds json");

    // Assert
    assert_eq!(snapshot.overall_level, ResourcePressureState::StopRequired);
    assert_eq!(
        classify_snapshot_against_disk_budget(&snapshot, 100),
        ResourcePressureState::StopRequired
    );
    assert_eq!(encoded["overall_level"], "stop_required");
    assert_eq!(encoded["entries"][0]["kind"], "disk");
    assert_eq!(
        encoded["entries"][0]["usage"]["value"]["state"],
        "stop_required"
    );
}

#[test]
fn recovery_evidence_contract_action_classes_serialize_stable_labels() {
    // Arrange
    let cases = [
        (RecoveryActionClass::SafeRetry, "safe_retry"),
        (
            RecoveryActionClass::ReadOnlyInspection,
            "read_only_inspection",
        ),
        (
            RecoveryActionClass::BackupThenRebuild,
            "backup_then_rebuild",
        ),
        (RecoveryActionClass::StopAndEscalate, "stop_and_escalate"),
    ];

    // Act / Assert
    for (action_class, expected_label) in cases {
        assert_eq!(
            serde_json::to_value(action_class).expect("action class json"),
            expected_label
        );
    }
}

#[test]
fn recovery_evidence_contract_causes_serialize_stable_labels() {
    // Arrange
    let cases = [
        (RecoveryCause::SchemaMismatch, "schema_mismatch"),
        (RecoveryCause::CorruptionMarker, "corruption_marker"),
        (RecoveryCause::CorruptRecord, "corrupt_record"),
        (RecoveryCause::PartialWrite, "partial_write"),
        (RecoveryCause::UnreadableNamespace, "unreadable_namespace"),
        (RecoveryCause::BackendOpenFailure, "backend_open_failure"),
        (RecoveryCause::ActiveLock, "active_lock"),
        (RecoveryCause::StaleLockEvidence, "stale_lock_evidence"),
        (
            RecoveryCause::ConcurrentDatadirUse,
            "concurrent_datadir_use",
        ),
        (RecoveryCause::ResourcePressure, "resource_pressure"),
    ];

    // Act / Assert
    for (cause, expected_label) in cases {
        assert_eq!(
            serde_json::to_value(cause).expect("recovery cause json"),
            expected_label
        );
    }
}

#[test]
fn recovery_evidence_contract_lock_evidence_serializes_plan_77_02_shape() {
    // Arrange
    let cases = [
        (LockEvidenceKind::NoLockArtifact, "no_lock_artifact"),
        (LockEvidenceKind::ActiveContention, "active_contention"),
        (LockEvidenceKind::StaleLockEvidence, "stale_lock_evidence"),
        (LockEvidenceKind::ProbeUnavailable, "probe_unavailable"),
    ];

    // Act / Assert
    for (kind, expected_label) in cases {
        let evidence = LockEvidence {
            kind,
            lock_path: "/tmp/open-bitcoin/lock".to_string(),
            detail: format!("{expected_label} detail"),
        };
        let encoded = serde_json::to_value(&evidence).expect("lock evidence json");
        let decoded: LockEvidence =
            serde_json::from_value(encoded.clone()).expect("lock evidence round-trip");

        assert_eq!(encoded["kind"], expected_label);
        assert_eq!(encoded["lock_path"], "/tmp/open-bitcoin/lock");
        assert_eq!(encoded["detail"], format!("{expected_label} detail"));
        assert_eq!(decoded, evidence);
    }
}

#[test]
fn status_recovery_evidence_legacy_snapshot_defaults_unavailable() {
    // Arrange
    let mut legacy_json = serde_json::to_value(stopped_snapshot()).expect("legacy snapshot json");
    let serde_json::Value::Object(fields) = &mut legacy_json else {
        panic!("snapshot must serialize to an object");
    };
    fields.remove("recovery_evidence");

    // Act
    let snapshot: OpenBitcoinStatusSnapshot =
        serde_json::from_value(legacy_json).expect("legacy status snapshot json");

    // Assert
    assert_eq!(
        snapshot.recovery_evidence,
        FieldAvailability::<RecoveryEvidenceSnapshot>::unavailable(
            RECOVERY_EVIDENCE_UNAVAILABLE_REASON
        )
    );
}

#[test]
fn status_recovery_evidence_snapshot_json_keeps_top_level_field_visible() {
    // Arrange
    let snapshot = stopped_snapshot();

    // Act
    let encoded = serde_json::to_value(snapshot).expect("snapshot json");

    // Assert
    assert_eq!(encoded["recovery_evidence"]["state"], "unavailable");
    assert_eq!(
        encoded["recovery_evidence"]["value"]["reason"],
        RECOVERY_EVIDENCE_UNAVAILABLE_REASON
    );
}

#[test]
fn populated_snapshot_serializes_obs_01_fields() {
    // Arrange
    let snapshot = OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec!["/tmp/open-bitcoin/bitcoin.conf".to_string()],
        },
        service: ServiceStatus {
            manager: FieldAvailability::available("launchd".to_string()),
            lifecycle: FieldAvailability::available(ServiceLifecycleStatus::Running),
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::available(true),
            running: FieldAvailability::available(true),
            service_file_path: FieldAvailability::available(
                "/tmp/open-bitcoin-node.service".to_string(),
            ),
            log_path: FieldAvailability::available("/tmp/logs/open-bitcoin.log".to_string()),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::unavailable(
                "service restart/resume evidence unavailable",
            ),
        },
        sync: SyncStatus {
            network: FieldAvailability::available("mainnet".to_string()),
            chain_tip: FieldAvailability::available(ChainTipStatus {
                height: 840_000,
                block_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                    .to_string(),
            }),
            sync_progress: FieldAvailability::available(SyncProgress {
                header_height: 840_001,
                block_height: 840_000,
                downloaded_block_height: 840_000,
                connected_block_height: 840_000,
                validated_active_chain_height: 840_000,
                maybe_downloaded_block_hash: Some(
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                ),
                maybe_connected_block_hash: Some(
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                ),
                maybe_validated_active_chain_hash: Some(
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                ),
                maybe_validated_active_chain_work: Some("840001".to_string()),
                progress_ratio: 0.99,
                messages_processed: 12,
                headers_received: 1,
                blocks_received: 1,
            }),
            lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
            phase: FieldAvailability::available("block_download".to_string()),
            configured_targets: FieldAvailability::available(SyncConfiguredTargets {
                target_outbound_peers: 4,
                maybe_target_header_height: Some(840_001),
            }),
            attempt_counters: FieldAvailability::available(SyncAttemptCounters {
                attempted_peers: 2,
                connected_peers: 2,
                failed_peers: 0,
                max_sync_rounds: 8,
            }),
            progress_signal: FieldAvailability::available(SyncProgressSignal::BlockProgress),
            lag: FieldAvailability::available(SyncLagStatus {
                headers_remaining: 0,
                blocks_remaining: 1,
            }),
            last_successful_progress_unix_seconds: FieldAvailability::available(1_715_000_000),
            progress_credit: FieldAvailability::unavailable(PROGRESS_CREDIT_UNAVAILABLE_REASON),
            expected_progress_window: FieldAvailability::unavailable(
                super::EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON,
            ),
            no_progress_threshold: FieldAvailability::unavailable(
                NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON,
            ),
            last_useful_work: FieldAvailability::unavailable(LAST_USEFUL_WORK_UNAVAILABLE_REASON),
            last_peer_contribution: FieldAvailability::unavailable(
                LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON,
            ),
            stall_diagnosis: FieldAvailability::unavailable(STALL_DIAGNOSIS_UNAVAILABLE_REASON),
            latest_stop_reason: FieldAvailability::unavailable("no stop reason recorded"),
            last_error: FieldAvailability::unavailable("no sync error recorded"),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable("no recovery action required"),
            resource_pressure: FieldAvailability::available(SyncResourcePressure {
                blocks_in_flight: 1,
                max_header_requests_in_flight_per_peer: 1,
                max_headers_per_message: 2_000,
                max_blocks_in_flight_per_peer: 16,
                max_blocks_in_flight_total: 64,
                max_messages_per_peer: 64,
                max_sync_rounds: 8,
                outbound_peers: 2,
                target_outbound_peers: 4,
            }),
            best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
                "best-known tip evidence unavailable",
            ),
            stay_current: FieldAvailability::unavailable("stay-current state unavailable"),
            stay_current_next_action: FieldAvailability::unavailable(
                "stay-current next action unavailable",
            ),
            no_progress_diagnosis: FieldAvailability::unavailable(
                NO_PROGRESS_DIAGNOSIS_UNAVAILABLE_REASON,
            ),
            no_progress_next_action: FieldAvailability::unavailable(
                NO_PROGRESS_NEXT_ACTION_UNAVAILABLE_REASON,
            ),
            latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
            reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 8,
            }),
            recent_peers: FieldAvailability::available(vec![PeerTelemetry {
                peer: "seed.bitcoin.sipa.be:8333".to_string(),
                source: "dns_seed".to_string(),
                state: "connected".to_string(),
                network: "mainnet".to_string(),
                attempts: 1,
                maybe_resolved_endpoint: FieldAvailability::available(
                    "203.0.113.10:8333".to_string(),
                ),
                capabilities: FieldAvailability::available("services=9 prefs=headers".to_string()),
                headers_received: 1,
                blocks_received: 1,
                maybe_last_activity_unix_seconds: FieldAvailability::available(1_715_000_000),
                failure_reason: FieldAvailability::unavailable("peer healthy"),
                error: FieldAvailability::unavailable("peer healthy"),
            }]),
            inbound: FieldAvailability::unavailable(INBOUND_STATUS_UNAVAILABLE_REASON),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(12),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::available(25_000),
            freshness: FieldAvailability::available(WalletFreshness::Fresh),
            scan_progress: FieldAvailability::unavailable("wallet already fresh"),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::default(),
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
        health_signals: vec![HealthSignal {
            level: HealthSignalLevel::Info,
            source: "status".to_string(),
            message: "node healthy".to_string(),
        }],
        build: BuildProvenance::unavailable(),
    };

    // Act
    let encoded = serde_json::to_value(&snapshot).expect("snapshot json");

    // Assert
    assert_eq!(encoded["config"]["datadir"]["state"], "available");
    assert_eq!(
        encoded["config"]["config_paths"][0],
        "/tmp/open-bitcoin/bitcoin.conf"
    );
    assert_eq!(encoded["sync"]["network"]["value"], "mainnet");
    assert_eq!(encoded["sync"]["chain_tip"]["value"]["height"], 840_000);
    assert_eq!(
        encoded["sync"]["sync_progress"]["value"]["header_height"],
        840_001
    );
    assert_eq!(encoded["sync"]["lifecycle"]["value"], "active");
    assert_eq!(encoded["sync"]["phase"]["value"], "block_download");
    assert_eq!(
        encoded["sync"]["configured_targets"]["value"]["target_outbound_peers"],
        4
    );
    assert_eq!(
        encoded["sync"]["attempt_counters"]["value"]["attempted_peers"],
        2
    );
    assert_eq!(
        encoded["sync"]["progress_signal"]["value"],
        "block_progress"
    );
    assert_eq!(encoded["sync"]["lag"]["value"]["blocks_remaining"], 1);
    assert_eq!(
        encoded["sync"]["last_successful_progress_unix_seconds"]["value"],
        1_715_000_000
    );
    assert_eq!(encoded["sync"]["progress_credit"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["expected_progress_window"]["state"],
        "unavailable"
    );
    assert_eq!(
        encoded["sync"]["no_progress_threshold"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["last_useful_work"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["last_peer_contribution"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["stall_diagnosis"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["latest_stop_reason"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["recovery_category"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["recovery_action"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["peer_counts"]["value"]["outbound"], 8);
    assert_eq!(
        encoded["peers"]["recent_peers"]["value"][0]["source"],
        "dns_seed"
    );
    assert_eq!(encoded["wallet"]["freshness"]["value"], "fresh");
    assert_eq!(encoded["wallet"]["scan_progress"]["state"], "unavailable");
    assert_eq!(encoded["health_signals"][0]["message"], "node healthy");
}

#[test]
fn wallet_freshness_states_serialize_distinctly_in_snapshot() {
    // Arrange
    let states = [
        (
            WalletFreshness::Fresh,
            FieldAvailability::unavailable("wallet already fresh"),
            "fresh",
        ),
        (
            WalletFreshness::Stale,
            FieldAvailability::unavailable("wallet scan not running"),
            "stale",
        ),
        (
            WalletFreshness::Partial,
            FieldAvailability::available(WalletScanProgress {
                scanned_through_height: 40,
                target_tip_height: 100,
            }),
            "partial",
        ),
        (
            WalletFreshness::Scanning,
            FieldAvailability::available(WalletScanProgress {
                scanned_through_height: 60,
                target_tip_height: 100,
            }),
            "scanning",
        ),
    ];

    // Act
    let encoded = states
        .into_iter()
        .map(|(freshness, scan_progress, expected)| {
            let mut snapshot = stopped_snapshot();
            snapshot.wallet = WalletStatus {
                trusted_balance_sats: FieldAvailability::available(25_000),
                freshness: FieldAvailability::available(freshness),
                scan_progress,
            };
            let encoded = serde_json::to_value(snapshot).expect("snapshot json");
            (encoded, expected)
        })
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(encoded[0].0["wallet"]["freshness"]["value"], encoded[0].1);
    assert_eq!(encoded[1].0["wallet"]["freshness"]["value"], encoded[1].1);
    assert_eq!(encoded[2].0["wallet"]["freshness"]["value"], encoded[2].1);
    assert_eq!(encoded[3].0["wallet"]["freshness"]["value"], encoded[3].1);
    assert_eq!(
        encoded[2].0["wallet"]["scan_progress"]["value"]["scanned_through_height"],
        40
    );
    assert_eq!(
        encoded[3].0["wallet"]["scan_progress"]["value"]["target_tip_height"],
        100
    );
}

#[test]
fn inbound_status_snapshot_serializes_address_boundary_evidence_under_peers_inbound() {
    // Arrange
    let mut snapshot = stopped_snapshot();
    snapshot.peers.inbound = FieldAvailability::available(InboundPeerServingStatus {
        listener_state: "ready".to_string(),
        bound_endpoints: Vec::new(),
        preflight_reason: "ready".to_string(),
        admitted_inbound_peers: 1,
        rejected_inbound_peers: 0,
        handshake: InboundHandshakeStatusCounts::default(),
        duplicate_rejects: 0,
        self_connection_rejects: 0,
        cap_rejects: 0,
        reserved_slot_rejects: 0,
        latest_admission_event: FieldAvailability::unavailable("no admission decision recorded"),
        permissioned_inbound_peers: 0,
        protected_inbound_peers: 0,
        permission_class: "ordinary_inbound".to_string(),
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        latest_permission_decision: FieldAvailability::unavailable(
            "inbound permission decision evidence unavailable",
        ),
        local_advertisement_candidates: vec![InboundAddressEvidenceEntry {
            source: "source_local_listener".to_string(),
            network_kind: "ipv4".to_string(),
            routability: "publicly_routable".to_string(),
            freshness: "fresh".to_string(),
            services_bits: 1,
            port: 18_444,
            persistence_eligible: true,
        }],
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: 1,
        getaddr_requests_suppressed: 0,
        learned_address_entries: 1,
        learned_address_rejections: 0,
        latest_address_decision: FieldAvailability::available(InboundAddressDecisionEvent {
            outcome: "accepted".to_string(),
            reason: "empty_response_cache".to_string(),
            label: "learned_accepted".to_string(),
            source: "source_inbound_addr".to_string(),
            message: "learned address accepted".to_string(),
        }),
    });

    // Act
    let encoded = serde_json::to_value(snapshot).expect("status snapshot json");

    // Assert
    assert_eq!(
        encoded["peers"]["inbound"]["value"]["local_advertisement_candidates"][0]["source"],
        "source_local_listener"
    );
    assert_eq!(
        encoded["peers"]["inbound"]["value"]["local_advertisement_candidates"][0]["port"],
        18_444
    );
    assert_eq!(
        encoded["peers"]["inbound"]["value"]["getaddr_responses_served"],
        1
    );
    assert_eq!(
        encoded["peers"]["inbound"]["value"]["latest_address_decision"]["value"]["source"],
        "source_inbound_addr"
    );
}

fn stopped_snapshot() -> OpenBitcoinStatusSnapshot {
    let unavailable = "node stopped";
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Stopped,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec!["/tmp/open-bitcoin/bitcoin.conf".to_string()],
        },
        service: ServiceStatus {
            manager: FieldAvailability::unavailable("service manager not inspected"),
            lifecycle: FieldAvailability::unavailable("service manager not inspected"),
            installed: FieldAvailability::unavailable("service manager not inspected"),
            enabled: FieldAvailability::unavailable("service manager not inspected"),
            running: FieldAvailability::unavailable("service manager not inspected"),
            service_file_path: FieldAvailability::unavailable("service file path unavailable"),
            log_path: FieldAvailability::unavailable("service log path unavailable"),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::unavailable(
                "service restart/resume evidence unavailable",
            ),
        },
        sync: SyncStatus {
            network: FieldAvailability::unavailable(unavailable),
            chain_tip: FieldAvailability::unavailable(unavailable),
            sync_progress: FieldAvailability::unavailable(unavailable),
            lifecycle: FieldAvailability::unavailable(unavailable),
            phase: FieldAvailability::unavailable(unavailable),
            configured_targets: FieldAvailability::unavailable(unavailable),
            attempt_counters: FieldAvailability::unavailable(unavailable),
            progress_signal: FieldAvailability::unavailable(unavailable),
            lag: FieldAvailability::unavailable(unavailable),
            last_successful_progress_unix_seconds: FieldAvailability::unavailable(unavailable),
            progress_credit: FieldAvailability::unavailable(PROGRESS_CREDIT_UNAVAILABLE_REASON),
            expected_progress_window: FieldAvailability::unavailable(
                super::EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON,
            ),
            no_progress_threshold: FieldAvailability::unavailable(
                NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON,
            ),
            last_useful_work: FieldAvailability::unavailable(LAST_USEFUL_WORK_UNAVAILABLE_REASON),
            last_peer_contribution: FieldAvailability::unavailable(
                LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON,
            ),
            stall_diagnosis: FieldAvailability::unavailable(STALL_DIAGNOSIS_UNAVAILABLE_REASON),
            latest_stop_reason: FieldAvailability::unavailable(unavailable),
            last_error: FieldAvailability::unavailable(unavailable),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable(unavailable),
            resource_pressure: FieldAvailability::unavailable(unavailable),
            best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(unavailable),
            stay_current: FieldAvailability::unavailable(unavailable),
            stay_current_next_action: FieldAvailability::unavailable(unavailable),
            no_progress_diagnosis: FieldAvailability::unavailable(
                NO_PROGRESS_DIAGNOSIS_UNAVAILABLE_REASON,
            ),
            no_progress_next_action: FieldAvailability::unavailable(
                NO_PROGRESS_NEXT_ACTION_UNAVAILABLE_REASON,
            ),
            latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
            reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::unavailable(unavailable),
            recent_peers: FieldAvailability::unavailable(unavailable),
            inbound: FieldAvailability::unavailable(INBOUND_STATUS_UNAVAILABLE_REASON),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable(unavailable),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::unavailable(unavailable),
            freshness: FieldAvailability::unavailable(unavailable),
            scan_progress: FieldAvailability::unavailable(unavailable),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::default(),
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
        health_signals: Vec::new(),
        build: BuildProvenance::unavailable(),
    }
}
