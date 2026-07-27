// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use crate::status::EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON;

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
            EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON,
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
