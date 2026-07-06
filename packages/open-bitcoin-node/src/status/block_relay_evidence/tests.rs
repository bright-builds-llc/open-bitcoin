// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::collections::BTreeSet;

use serde_json::Value;

use crate::status::block_serving::{
    BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON, BlockServingActivationEvidence,
    BlockServingEligibilityCounters, BlockServingEvidenceStatus, BlockServingStatusCounters,
};

use super::{
    BlockRelayEvidenceStatus, CompactRelayAnnouncementCounters, CompactRelayCleanupCounters,
    CompactRelayFallbackCounters, CompactRelayInFlightCounters,
    CompactRelayMissingTransactionCounters, CompactRelayNegotiationCounters,
    CompactRelayReconstructionCounters, block_serving_default_available_counters,
};

#[test]
fn block_relay_default_unavailable_serializes_fixed_reason_and_zero_counters() {
    // Arrange
    let status = BlockRelayEvidenceStatus::default_unavailable();

    // Act
    let encoded = serde_json::to_value(status).expect("block relay evidence json");
    let keys = collect_json_keys(&encoded);

    // Assert
    assert_eq!(
        encoded["block_serving"]["activation"]["state"],
        "unavailable"
    );
    assert_eq!(
        encoded["block_serving"]["activation"]["value"]["reason"],
        BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON
    );
    assert_eq!(
        encoded["block_serving"]["eligibility"]["state"],
        "available"
    );
    assert_eq!(encoded["block_serving"]["status"]["state"], "available");
    assert_eq!(encoded["negotiation"]["state"], "available");
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        0
    );
    assert_eq!(
        encoded["reconstruction"]["value"]["compact_malformed_count"],
        0
    );
    assert_eq!(
        encoded["missing_transaction"]["value"]["compact_missing_tx_requested_count"],
        0
    );
    assert_eq!(encoded["fallback"]["value"]["compact_fallback_count"], 0);
    assert_eq!(encoded["in_flight"]["value"]["in_flight_count"], 0);
    assert_eq!(encoded["cleanup"]["value"]["compact_cleanup_count"], 0);
    assert_eq!(keys, expected_default_public_keys());
}

#[test]
fn block_relay_evidence_serializes_composed_block_and_compact_counters() {
    // Arrange
    let status = BlockRelayEvidenceStatus::with_components(
        BlockServingEvidenceStatus::with_activation_eligibility_and_status(
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
        ),
        CompactRelayNegotiationCounters {
            version2_high_bandwidth_count: 19,
            version2_low_bandwidth_count: 20,
            unsupported_version_count: 21,
        },
        CompactRelayAnnouncementCounters {
            compact_announced_count: 22,
            compact_headers_fallback_count: 23,
            compact_inventory_fallback_count: 24,
            compact_suppressed_count: 25,
        },
        CompactRelayReconstructionCounters {
            compact_reconstructed_count: 26,
            compact_reconstruction_failed_count: 27,
            compact_malformed_count: 28,
        },
        CompactRelayMissingTransactionCounters {
            compact_missing_tx_requested_count: 29,
            compact_missing_tx_suppressed_count: 30,
        },
        CompactRelayFallbackCounters {
            compact_fallback_count: 31,
            compact_timeout_count: 32,
        },
        CompactRelayInFlightCounters {
            in_flight_count: 33,
            getblocktxn_in_flight_count: 34,
            peers_with_in_flight_count: 35,
        },
        CompactRelayCleanupCounters {
            compact_cleanup_count: 36,
            compact_download_peer_disconnect_count: 37,
            compact_download_timeout_count: 38,
            compact_download_reorg_count: 39,
            compact_download_restart_count: 40,
            compact_download_block_connected_count: 41,
        },
    );

    // Act
    let encoded = serde_json::to_value(status).expect("block relay evidence json");

    // Assert
    assert_eq!(
        encoded["block_serving"]["activation"]["value"]["block_serving_enabled"],
        true
    );
    assert_eq!(
        encoded["block_serving"]["activation"]["value"]["compact_relay_enabled"],
        true
    );
    assert_eq!(
        encoded["block_serving"]["eligibility"]["value"]["permission_effect_inactive_count"],
        9
    );
    assert_eq!(
        encoded["block_serving"]["status"]["value"]["suppressed_count"],
        18
    );
    assert_eq!(
        encoded["negotiation"]["value"]["version2_high_bandwidth_count"],
        19
    );
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        22
    );
    assert_eq!(
        encoded["reconstruction"]["value"]["compact_reconstruction_failed_count"],
        27
    );
    assert_eq!(
        encoded["missing_transaction"]["value"]["compact_missing_tx_requested_count"],
        29
    );
    assert_eq!(encoded["fallback"]["value"]["compact_timeout_count"], 32);
    assert_eq!(
        encoded["in_flight"]["value"]["getblocktxn_in_flight_count"],
        34
    );
    assert_eq!(
        encoded["cleanup"]["value"]["compact_download_block_connected_count"],
        41
    );
}

#[test]
fn block_relay_evidence_defaults_legacy_json_to_safe_contract() {
    // Arrange
    let legacy_json = serde_json::json!({});

    // Act
    let status: BlockRelayEvidenceStatus =
        serde_json::from_value(legacy_json).expect("legacy block relay evidence json");
    let encoded = serde_json::to_value(status).expect("block relay evidence json");

    // Assert
    assert_eq!(
        encoded["block_serving"]["activation"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["negotiation"]["state"], "available");
    assert_eq!(encoded["cleanup"]["state"], "available");
    assert_eq!(
        encoded["missing_transaction"]["value"]["compact_missing_tx_suppressed_count"],
        0
    );
}

#[test]
fn block_relay_evidence_omits_sensitive_material_by_construction() {
    // Arrange
    let status = BlockRelayEvidenceStatus::with_components(
        block_serving_default_available_counters(),
        CompactRelayNegotiationCounters {
            version2_high_bandwidth_count: 1,
            version2_low_bandwidth_count: 0,
            unsupported_version_count: 0,
        },
        CompactRelayAnnouncementCounters {
            compact_announced_count: 1,
            compact_headers_fallback_count: 0,
            compact_inventory_fallback_count: 0,
            compact_suppressed_count: 0,
        },
        CompactRelayReconstructionCounters::default(),
        CompactRelayMissingTransactionCounters::default(),
        CompactRelayFallbackCounters::default(),
        CompactRelayInFlightCounters {
            in_flight_count: 2,
            getblocktxn_in_flight_count: 1,
            peers_with_in_flight_count: 1,
        },
        CompactRelayCleanupCounters::default(),
    );

    // Act
    let encoded = serde_json::to_string(&status).expect("block relay json");

    // Assert
    for forbidden in [
        "\"peer_id\"",
        "\"peer_ids\"",
        "\"endpoint\"",
        "\"endpoints\"",
        "\"credentials\"",
        "\"cookie\"",
        "\"block_hash\"",
        "\"block_hashes\"",
        "\"txid\"",
        "\"txids\"",
        "\"wtxid\"",
        "\"wtxids\"",
        "\"indexes\"",
        "\"index\"",
        "\"labels\"",
        "\"permission_strings\"",
    ] {
        assert!(!encoded.contains(forbidden));
    }
}

#[test]
fn block_relay_evidence_keeps_zeroed_counters_available_when_activation_is_unavailable() {
    // Arrange
    let status = block_serving_default_available_counters();

    // Act
    let encoded = serde_json::to_value(status).expect("block serving evidence json");

    // Assert
    assert_eq!(encoded["activation"]["state"], "unavailable");
    assert_eq!(encoded["eligibility"]["state"], "available");
    assert_eq!(encoded["eligibility"]["value"]["eligible_peer_count"], 0);
    assert_eq!(encoded["status"]["state"], "available");
    assert_eq!(encoded["status"]["value"]["validated_count"], 0);
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
        "announcement",
        "available_count",
        "block_serving",
        "cleanup",
        "compact_announced_count",
        "compact_cleanup_count",
        "compact_download_block_connected_count",
        "compact_download_peer_disconnect_count",
        "compact_download_reorg_count",
        "compact_download_restart_count",
        "compact_download_timeout_count",
        "compact_fallback_count",
        "compact_headers_fallback_count",
        "compact_inventory_fallback_count",
        "compact_malformed_count",
        "compact_missing_tx_requested_count",
        "compact_missing_tx_suppressed_count",
        "compact_reconstructed_count",
        "compact_reconstruction_failed_count",
        "compact_suppressed_count",
        "compact_timeout_count",
        "disabled_count",
        "eligible_peer_count",
        "eligibility",
        "fallback",
        "getblocktxn_in_flight_count",
        "in_flight",
        "in_flight_count",
        "inbound_serving_required_count",
        "ineligible_peer_count",
        "missing_transaction",
        "negotiation",
        "peers_with_in_flight_count",
        "permission_effect_inactive_count",
        "permission_required_count",
        "protected_not_serving_count",
        "pruned_count",
        "reason",
        "reconstruction",
        "side_chain_count",
        "stale_count",
        "state",
        "status",
        "status_unavailable_count",
        "suppressed_count",
        "unavailable_count",
        "unknown_count",
        "unsupported_version_count",
        "unvalidated_count",
        "validated_count",
        "value",
        "version2_high_bandwidth_count",
        "version2_low_bandwidth_count",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
