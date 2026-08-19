// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_mempool::{
    MempoolCapacityEnforcement, MempoolCapacityStatus, RollingFeeParityStatus,
};

use super::*;
use crate::network::ManagedMempoolInfo;

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
    let status: MempoolStatus =
        serde_json::from_value(legacy).expect("legacy mempool status json");

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