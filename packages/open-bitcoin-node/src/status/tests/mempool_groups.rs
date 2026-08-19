// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_mempool::{
    MempoolCapacityEnforcement, MempoolCapacityStatus, RollingFeeParityStatus,
};

use open_bitcoin_core::primitives::Txid;

use crate::network::{
    CheckpointEvidenceSnapshot, CheckpointOutcome, CheckpointPersistenceStrength,
    ManagedMempoolInfo, ManagedMempoolRecoverySummary,
};
use crate::status::relay_evidence::RelayEvidenceStatus;
use crate::status::{
    FieldAvailability, MempoolPressureGroup, MempoolResourcesGroup, MempoolStatus,
    checkpoint_group_from_evidence, decay_half_life_label, fee_floors_from_managed_info,
    recovery_group_from_summary, resources_from_managed_info,
};
use crate::storage::{MempoolRecoveryRecord, MempoolRecoveryStatus};

fn managed_info_with_distinct_resource_and_fee_roles() -> ManagedMempoolInfo {
    ManagedMempoolInfo {
        transaction_count: 7,
        total_virtual_size: 1_234,
        accounted_memory: 5_678,
        mempool_capacity: 9_000,
        total_fee_sats: 100,
        static_relay_fee_rate_sats_per_kvb: 1_000,
        incremental_relay_fee_rate_sats_per_kvb: 500,
        rolling_mempool_fee_rate_sats_per_kvb: 2_500,
        effective_admission_fee_rate_sats_per_kvb: 2_500,
        capacity_status: MempoolCapacityStatus::UnderCapacity,
        capacity_enforcement: MempoolCapacityEnforcement::AccountedMemory,
        rolling_fee_parity: RollingFeeParityStatus::Active,
    }
}

fn forbidden_identity_or_knots_alias_keys(object: &serde_json::Map<String, serde_json::Value>) {
    for key in object.keys() {
        assert!(
            !key.contains("txid")
                && !key.contains("wtxid")
                && !key.contains("fingerprint")
                && key != "hex"
                && key != "bytes"
                && key != "usage"
                && key != "maxmempool"
                && key != "mempoolminfee",
            "group JSON leaked identifier or Knots alias key {key}"
        );
    }
}

#[test]
fn resources_group_json_uses_open_bitcoin_names_not_knots_aliases() {
    // Arrange
    let group = MempoolResourcesGroup {
        virtual_size: 100,
        accounted_usage: 200,
        accounted_capacity: 300,
        transaction_count: 4,
    };

    // Act
    let encoded = serde_json::to_value(&group).expect("resources group json");
    let object = encoded
        .as_object()
        .expect("resources group serializes to object");

    // Assert
    assert_eq!(encoded["virtual_size"], 100);
    assert_eq!(encoded["accounted_usage"], 200);
    assert_eq!(encoded["accounted_capacity"], 300);
    assert_eq!(encoded["transaction_count"], 4);
    assert_eq!(object.get("bytes"), None);
    assert_eq!(object.get("usage"), None);
    assert_eq!(object.get("maxmempool"), None);
    assert_eq!(object.get("mempoolminfee"), None);
    forbidden_identity_or_knots_alias_keys(object);
}

#[test]
fn fee_floors_keep_four_distinct_values_when_rolling_exceeds_static() {
    // Arrange
    let info = managed_info_with_distinct_resource_and_fee_roles();

    // Act
    let group = fee_floors_from_managed_info(&info);
    let encoded = serde_json::to_value(&group).expect("fee floors json");
    let object = encoded
        .as_object()
        .expect("fee floors group serializes to object");

    // Assert
    assert_eq!(group.static_relay_floor, 1_000);
    assert_eq!(group.rolling_mempool_floor, 2_500);
    assert_eq!(group.effective_admission_floor, 2_500);
    assert_eq!(group.incremental_relay_fee, 500);
    assert_eq!(encoded["static_relay_floor"], 1_000);
    assert_eq!(encoded["rolling_mempool_floor"], 2_500);
    assert_eq!(encoded["effective_admission_floor"], 2_500);
    assert_eq!(encoded["incremental_relay_fee"], 500);
    assert_eq!(object.get("mempoolminfee"), None);
    assert_ne!(group.static_relay_floor, group.rolling_mempool_floor);
    assert_ne!(group.incremental_relay_fee, group.effective_admission_floor);
    forbidden_identity_or_knots_alias_keys(object);
}

#[test]
fn mempool_status_deserializes_missing_groups_as_unavailable() {
    // Arrange
    let legacy = serde_json::json!({
        "transactions": { "state": "available", "value": 3 }
    });

    // Act
    let status: MempoolStatus = serde_json::from_value(legacy).expect("legacy mempool status json");

    // Assert
    assert_eq!(
        status.resources,
        FieldAvailability::unavailable("mempool group unavailable")
    );
    assert_eq!(
        status.fee_floors,
        FieldAvailability::unavailable("mempool group unavailable")
    );
    assert_eq!(status.relay, RelayEvidenceStatus::default());
    assert_eq!(status.transactions, FieldAvailability::available(3));
}

#[test]
fn resources_from_managed_info_does_not_swap_vsize_and_accounted_usage() {
    // Arrange
    let info = managed_info_with_distinct_resource_and_fee_roles();

    // Act
    let group = resources_from_managed_info(&info);

    // Assert
    assert_eq!(group.virtual_size, 1_234);
    assert_eq!(group.accounted_usage, 5_678);
    assert_eq!(group.accounted_capacity, 9_000);
    assert_eq!(group.transaction_count, 7);
    assert_ne!(group.virtual_size, group.accounted_usage);
}

#[test]
fn decay_label_is_not_decaying_when_gate_closed() {
    // Arrange
    let usage = 10;
    let capacity = 100;
    let decay_gate_open = false;

    // Act
    let label = decay_half_life_label(usage, capacity, decay_gate_open);

    // Assert
    assert_eq!(label, "not_decaying");
}

#[test]
fn decay_label_is_3h_when_usage_below_quarter_capacity() {
    // Arrange
    let usage = 10;
    let capacity = 100;
    let decay_gate_open = true;

    // Act
    let label = decay_half_life_label(usage, capacity, decay_gate_open);

    // Assert
    assert_eq!(label, "half_life_3h");
}

#[test]
fn recovery_group_from_summary_drops_txid_records() {
    // Arrange
    let leaked_hex = "abababababababababababababababababababababababababababababababab";
    let summary = ManagedMempoolRecoverySummary {
        recovered_count: 2,
        dropped_confirmed_count: 1,
        dropped_duplicate_count: 3,
        dropped_missing_parent_count: 4,
        dropped_policy_incompatible_count: 5,
        dropped_expired_count: 6,
        dropped_evicted_count: 7,
        records: vec![MempoolRecoveryRecord {
            txid: Txid::from_byte_array([0xab; 32]),
            status: MempoolRecoveryStatus::Recovered,
        }],
    };

    // Act
    let group = recovery_group_from_summary(&summary);
    let encoded = serde_json::to_string(&group).expect("recovery group json");

    // Assert
    assert_eq!(group.recovered_count, 2);
    assert_eq!(group.dropped_confirmed_count, 1);
    assert_eq!(group.dropped_duplicate_count, 3);
    assert_eq!(group.dropped_missing_parent_count, 4);
    assert_eq!(group.dropped_policy_incompatible_count, 5);
    assert_eq!(group.dropped_expired_count, 6);
    assert_eq!(group.dropped_evicted_count, 7);
    assert!(!encoded.contains(leaked_hex));
    assert!(!regex_contains_txid_hex(&encoded));
    let object = serde_json::from_str::<serde_json::Value>(&encoded)
        .expect("recovery json value")
        .as_object()
        .expect("recovery group serializes to object")
        .clone();
    assert_eq!(object.get("records"), None);
    forbidden_identity_or_knots_alias_keys(&object);
}

#[test]
fn checkpoint_group_exposes_dirty_generation_present_not_generation_id() {
    // Arrange
    let evidence = CheckpointEvidenceSnapshot {
        current_generation: 9,
        maybe_dirty_generation: Some(42),
        maybe_in_flight_generation: None,
        maybe_last_durable_generation: Some(8),
        maybe_captured_at: None,
        maybe_completed_at: None,
        maybe_failed_at: None,
        maybe_trigger: None,
        maybe_persistence_strength: Some(CheckpointPersistenceStrength::Sync),
        outcome: CheckpointOutcome::Succeeded,
        maybe_failure: None,
        overdue: false,
        checkpoint_age_seconds: Some(11),
        maybe_loss_bound_seconds: Some(30),
        maybe_generation_loss_range: None,
    };

    // Act
    let group = checkpoint_group_from_evidence(&evidence);
    let encoded = serde_json::to_value(&group).expect("checkpoint group json");
    let object = encoded
        .as_object()
        .expect("checkpoint group serializes to object");

    // Assert
    assert!(group.dirty_generation_present);
    assert_eq!(group.outcome, "succeeded");
    assert!(!group.overdue);
    assert_eq!(group.persistence_strength, "sync");
    assert_eq!(group.age_seconds, Some(11));
    assert_eq!(group.loss_bound_seconds, Some(30));
    assert_eq!(object.get("dirty_generation"), None);
    assert_eq!(object.get("current_generation"), None);
    assert_ne!(encoded["dirty_generation_present"], 42);
    forbidden_identity_or_knots_alias_keys(object);
}

#[test]
fn pressure_group_json_has_no_evicted_count_or_rebroadcast_deferred() {
    // Arrange
    let group = MempoolPressureGroup {
        pressure_removal_count: 4,
        decay_half_life_label: "half_life_12h".to_string(),
    };

    // Act
    let encoded = serde_json::to_value(&group).expect("pressure group json");
    let object = encoded
        .as_object()
        .expect("pressure group serializes to object");

    // Assert
    assert_eq!(encoded["pressure_removal_count"], 4);
    assert_eq!(encoded["decay_half_life_label"], "half_life_12h");
    assert_eq!(object.get("evicted_count"), None);
    assert_eq!(object.get("rebroadcast_deferred_count"), None);
    assert_eq!(object.get("rebroadcast_deferred"), None);
    forbidden_identity_or_knots_alias_keys(object);
}

fn regex_contains_txid_hex(encoded: &str) -> bool {
    encoded
        .as_bytes()
        .windows(64)
        .any(|window| window.iter().all(|byte| byte.is_ascii_hexdigit()))
}
