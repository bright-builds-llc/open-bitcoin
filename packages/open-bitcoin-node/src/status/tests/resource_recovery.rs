// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

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
