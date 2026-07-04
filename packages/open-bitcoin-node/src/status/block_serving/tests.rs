// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON, BlockServingActivationEvidence,
    BlockServingEligibilityCounters, BlockServingEvidenceStatus, BlockServingStatusCounters,
};
use serde_json::Value;
use std::collections::BTreeSet;

#[test]
fn block_serving_default_unavailable_serializes_fixed_reason_and_zero_counters() {
    // Arrange
    let status = BlockServingEvidenceStatus::default_unavailable();

    // Act
    let encoded = serde_json::to_value(status).expect("block serving evidence json");
    let keys = collect_json_keys(&encoded);

    // Assert
    assert_eq!(encoded["activation"]["state"], "unavailable");
    assert_eq!(
        encoded["activation"]["value"]["reason"],
        BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON,
    );
    assert_eq!(encoded["eligibility"]["state"], "available");
    assert_eq!(encoded["eligibility"]["value"]["eligible_peer_count"], 0);
    assert_eq!(
        encoded["eligibility"]["value"]["permission_effect_inactive_count"],
        0
    );
    assert_eq!(encoded["status"]["state"], "available");
    assert_eq!(encoded["status"]["value"]["validated_count"], 0);
    assert_eq!(encoded["status"]["value"]["suppressed_count"], 0);
    assert_eq!(keys, expected_default_public_keys());
}

#[test]
fn block_serving_evidence_serializes_activation_eligibility_and_status_counters() {
    // Arrange
    let status = BlockServingEvidenceStatus::with_activation_eligibility_and_status(
        BlockServingActivationEvidence {
            block_serving_enabled: true,
            compact_relay_enabled: true,
        },
        BlockServingEligibilityCounters {
            eligible_peer_count: 1,
            ineligible_peer_count: 2,
            disabled_count: 3,
            activation_required_count: 4,
            inbound_serving_required_count: 5,
            permission_required_count: 6,
            protected_not_serving_count: 7,
            status_unavailable_count: 8,
            permission_effect_inactive_count: 9,
        },
        BlockServingStatusCounters {
            validated_count: 10,
            available_count: 11,
            stale_count: 12,
            side_chain_count: 13,
            pruned_count: 14,
            unavailable_count: 15,
            unvalidated_count: 16,
            unknown_count: 17,
            suppressed_count: 18,
        },
    );

    // Act
    let encoded = serde_json::to_value(status).expect("block serving evidence json");

    // Assert
    assert_eq!(encoded["activation"]["state"], "available");
    assert_eq!(
        encoded["activation"]["value"]["block_serving_enabled"],
        true
    );
    assert_eq!(
        encoded["activation"]["value"]["compact_relay_enabled"],
        true
    );
    assert_eq!(encoded["eligibility"]["value"]["eligible_peer_count"], 1);
    assert_eq!(encoded["eligibility"]["value"]["ineligible_peer_count"], 2);
    assert_eq!(encoded["eligibility"]["value"]["disabled_count"], 3);
    assert_eq!(
        encoded["eligibility"]["value"]["activation_required_count"],
        4
    );
    assert_eq!(
        encoded["eligibility"]["value"]["inbound_serving_required_count"],
        5
    );
    assert_eq!(
        encoded["eligibility"]["value"]["permission_required_count"],
        6
    );
    assert_eq!(
        encoded["eligibility"]["value"]["protected_not_serving_count"],
        7
    );
    assert_eq!(
        encoded["eligibility"]["value"]["status_unavailable_count"],
        8
    );
    assert_eq!(
        encoded["eligibility"]["value"]["permission_effect_inactive_count"],
        9
    );
    assert_eq!(encoded["status"]["value"]["validated_count"], 10);
    assert_eq!(encoded["status"]["value"]["available_count"], 11);
    assert_eq!(encoded["status"]["value"]["stale_count"], 12);
    assert_eq!(encoded["status"]["value"]["side_chain_count"], 13);
    assert_eq!(encoded["status"]["value"]["pruned_count"], 14);
    assert_eq!(encoded["status"]["value"]["unavailable_count"], 15);
    assert_eq!(encoded["status"]["value"]["unvalidated_count"], 16);
    assert_eq!(encoded["status"]["value"]["unknown_count"], 17);
    assert_eq!(encoded["status"]["value"]["suppressed_count"], 18);
}

#[test]
fn block_serving_evidence_defaults_legacy_json_to_safe_contract() {
    // Arrange
    let legacy_json = serde_json::json!({});

    // Act
    let status: BlockServingEvidenceStatus =
        serde_json::from_value(legacy_json).expect("legacy block serving evidence json");
    let encoded = serde_json::to_value(status).expect("block serving evidence json");

    // Assert
    assert_eq!(encoded["activation"]["state"], "unavailable");
    assert_eq!(encoded["eligibility"]["state"], "available");
    assert_eq!(encoded["status"]["state"], "available");
    assert_eq!(encoded["eligibility"]["value"]["eligible_peer_count"], 0);
    assert_eq!(encoded["status"]["value"]["available_count"], 0);
}

fn collect_json_keys(value: &Value) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    collect_json_keys_inner(value, &mut keys);
    keys
}

fn collect_json_keys_inner(value: &Value, keys: &mut BTreeSet<String>) {
    if let Value::Object(object) = value {
        for (key, value) in object {
            keys.insert(key.clone());
            collect_json_keys_inner(value, keys);
        }
    }
}

fn expected_default_public_keys() -> BTreeSet<String> {
    [
        "activation",
        "activation_required_count",
        "available_count",
        "disabled_count",
        "eligible_peer_count",
        "eligibility",
        "inbound_serving_required_count",
        "ineligible_peer_count",
        "permission_effect_inactive_count",
        "permission_required_count",
        "protected_not_serving_count",
        "pruned_count",
        "reason",
        "side_chain_count",
        "stale_count",
        "state",
        "status",
        "status_unavailable_count",
        "suppressed_count",
        "unknown_count",
        "unavailable_count",
        "unvalidated_count",
        "validated_count",
        "value",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
