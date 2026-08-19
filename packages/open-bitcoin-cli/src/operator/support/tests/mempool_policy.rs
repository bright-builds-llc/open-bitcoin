// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

const POISONED_RECOVERY_TXID: &str =
    "cafebabecafebabecafebabecafebabecafebabecafebabecafebabecafebabe";

fn available_retry_group() -> MempoolRetryGroup {
    MempoolRetryGroup {
        eligible: 1,
        queued: 2,
        attempted: 0,
        emitted: 3,
        requested: 4,
        served: 5,
        suppressed: 6,
        relay_disabled: 0,
        cleared: 7,
    }
}

fn snapshot_with_available_policy_counts() -> OpenBitcoinStatusSnapshot {
    let mut status = phase105_status_with_relay_evidence();
    status.mempool.resources = FieldAvailability::available(MempoolResourcesGroup {
        virtual_size: 2_048,
        accounted_usage: 4_096,
        accounted_capacity: 300_000_000,
        transaction_count: 12,
    });
    status.mempool.fee_floors = FieldAvailability::available(MempoolFeeFloorsGroup {
        static_relay_floor: 1_000,
        rolling_mempool_floor: 2_000,
        effective_admission_floor: 2_000,
        incremental_relay_fee: 1_000,
    });
    status.mempool.pressure = FieldAvailability::available(MempoolPressureGroup {
        pressure_removal_count: 3,
        decay_half_life_label: "half_life_12h".to_string(),
    });
    status.mempool.eviction = FieldAvailability::available(MempoolEvictionGroup {
        pressure_removal_count: 3,
    });
    status.mempool.checkpoint = FieldAvailability::available(MempoolCheckpointGroup {
        outcome: "succeeded".to_string(),
        overdue: false,
        persistence_strength: "sync".to_string(),
        age_seconds: Some(12),
        loss_bound_seconds: Some(30),
        dirty_generation_present: false,
    });
    status.mempool.recovery = FieldAvailability::available(MempoolRecoveryGroup {
        recovered_count: 1,
        dropped_confirmed_count: 2,
        dropped_duplicate_count: 3,
        dropped_missing_parent_count: 4,
        dropped_policy_incompatible_count: 5,
        dropped_expired_count: 6,
        dropped_evicted_count: 7,
    });
    status.mempool.retry = FieldAvailability::available(available_retry_group());
    status.mempool.admission = FieldAvailability::available(MempoolAdmissionGroup {
        accepted: 8,
        still_present: 9,
        cleared: 10,
    });
    status
}

#[test]
fn support_bundle_redaction_keeps_retry_counts() {
    // Arrange
    let status = snapshot_with_available_policy_counts();
    let expected_retry = available_retry_group();

    // Act
    let redacted = support_status_for_bundle(status);

    // Assert
    let FieldAvailability::Available(retry) = redacted.mempool.retry else {
        panic!("retry counts should stay available");
    };
    assert_eq!(retry, expected_retry);
    let FieldAvailability::Available(recovery) = &redacted.mempool.recovery else {
        panic!("recovery counts should stay available");
    };
    assert_eq!(recovery.recovered_count, 1);
    assert_eq!(recovery.dropped_evicted_count, 7);
}

#[test]
fn support_bundle_redaction_strips_txid_from_group_unavailable_reason() {
    // Arrange
    let mut status = snapshot_with_available_policy_counts();
    status.mempool.retry = FieldAvailability::unavailable(format!(
        "retry unavailable for txid={POISONED_RECOVERY_TXID} peer 198.51.100.109:8333"
    ));

    // Act
    let redacted = support_status_for_bundle(status);

    // Assert
    let FieldAvailability::Unavailable { reason } = redacted.mempool.retry else {
        panic!("retry should stay unavailable after redaction");
    };
    assert_eq!(reason, "redacted_relay_mempool_evidence");
}

#[test]
fn support_bundle_omitted_list_mentions_package_fingerprints() {
    // Arrange
    let summary = redaction_summary();

    // Act
    let mentions_package_fingerprints = summary
        .omitted
        .iter()
        .any(|entry| entry.contains("package fingerprints"));

    // Assert
    assert!(
        mentions_package_fingerprints,
        "omitted list must mention package fingerprints: {:?}",
        summary.omitted
    );
}

#[test]
fn support_bundle_json_has_no_64_hex_after_poisoned_recovery_reason() {
    // Arrange
    let mut status = snapshot_with_available_policy_counts();
    status.mempool.recovery = FieldAvailability::unavailable(POISONED_RECOVERY_TXID);

    // Act
    let redacted = support_status_for_bundle(status);
    let json = serde_json::to_string(&redacted).expect("redacted snapshot json");

    // Assert
    assert!(
        !json.contains(POISONED_RECOVERY_TXID),
        "poisoned recovery txid leaked into shareable snapshot json"
    );
}
