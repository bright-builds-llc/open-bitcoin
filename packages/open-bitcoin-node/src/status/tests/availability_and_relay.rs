// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

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
fn relay_evidence_status_serializes_fixed_counter_contract_once() {
    // Arrange
    let status = RelayEvidenceStatus::with_counters(RelayEvidenceCounters {
        accepted_count: 1,
        rejected_count: 2,
        orphaned_count: 3,
        requested_count: 4,
        served_count: 5,
        announced_count: 6,
        suppressed_count: 7,
        evicted_count: 8,
        expired_count: 9,
        rebroadcast_deferred_count: 10,
    });

    // Act
    let encoded = serde_json::to_value(status).expect("relay evidence json");
    let object = encoded
        .as_object()
        .expect("relay evidence status serializes to object");
    let counters = encoded["outcome_counters"]["value"]
        .as_object()
        .expect("outcome counters serialize to object");

    // Assert
    assert_eq!(encoded["outcome_counters"]["state"], "implemented");
    assert_eq!(counters["accepted_count"], 1);
    assert_eq!(counters["rejected_count"], 2);
    assert_eq!(counters["orphaned_count"], 3);
    assert_eq!(counters["requested_count"], 4);
    assert_eq!(counters["served_count"], 5);
    assert_eq!(counters["announced_count"], 6);
    assert_eq!(counters["suppressed_count"], 7);
    assert_eq!(counters["evicted_count"], 8);
    assert_eq!(counters["expired_count"], 9);
    assert_eq!(counters["rebroadcast_deferred_count"], 10);
    assert_eq!(
        object.keys().filter(|key| key.ends_with("_count")).count(),
        0
    );
    assert_eq!(
        counters
            .keys()
            .filter(|key| key.ends_with("_count"))
            .count(),
        10
    );
}

#[test]
fn relay_evidence_status_serializes_activation_and_download_eligibility_contract() {
    // Arrange
    let status = RelayEvidenceStatus::with_activation_and_counters(
        RelayActivationEvidence { enabled: true },
        RelayDownloadEligibilityCounters {
            eligible_peer_count: 1,
            ineligible_peer_count: 2,
            relay_disabled_count: 3,
            not_relay_eligible_count: 4,
            inbound_serving_required_count: 5,
            permission_required_count: 6,
            protected_not_relay_count: 7,
        },
        RelayEvidenceCounters::default(),
    );

    // Act
    let encoded = serde_json::to_value(status).expect("relay evidence json");
    let eligibility = encoded["download_eligibility"]["value"]
        .as_object()
        .expect("download eligibility counters");

    // Assert
    assert_eq!(encoded["activation"]["state"], "implemented");
    assert_eq!(encoded["activation"]["value"]["enabled"], true);
    assert_eq!(encoded["download_eligibility"]["state"], "implemented");
    assert_eq!(eligibility["eligible_peer_count"], 1);
    assert_eq!(eligibility["ineligible_peer_count"], 2);
    assert_eq!(eligibility["relay_disabled_count"], 3);
    assert_eq!(eligibility["not_relay_eligible_count"], 4);
    assert_eq!(eligibility["inbound_serving_required_count"], 5);
    assert_eq!(eligibility["permission_required_count"], 6);
    assert_eq!(eligibility["protected_not_relay_count"], 7);
    assert_eq!(
        eligibility
            .keys()
            .filter(|key| key.ends_with("_count"))
            .count(),
        7
    );
    assert!(encoded.get("reason").is_none());
}

#[test]
fn relay_evidence_status_projects_recovery_counters() {
    // Arrange
    let status = RelayEvidenceStatus::with_activation_recovery_and_counters(
        RelayActivationEvidence { enabled: true },
        RelayDownloadEligibilityCounters::default(),
        RelayRecoveryCounters {
            recovered_count: 1,
            dropped_confirmed_count: 2,
            dropped_duplicate_count: 3,
            dropped_missing_parent_count: 4,
            dropped_policy_incompatible_count: 5,
            dropped_evicted_count: 6,
        },
        RelayEvidenceCounters::default(),
    );

    // Act
    let encoded = serde_json::to_value(status).expect("relay evidence json");
    let counters = encoded["recovery_counters"]["value"]
        .as_object()
        .expect("recovery counters serialize to object");

    // Assert
    assert_eq!(encoded["recovery_counters"]["state"], "implemented");
    assert_eq!(counters["recovered_count"], 1);
    assert_eq!(counters["dropped_confirmed_count"], 2);
    assert_eq!(counters["dropped_duplicate_count"], 3);
    assert_eq!(counters["dropped_missing_parent_count"], 4);
    assert_eq!(counters["dropped_policy_incompatible_count"], 5);
    assert_eq!(counters["dropped_evicted_count"], 6);
    assert_eq!(
        counters
            .keys()
            .filter(|key| key.ends_with("_count"))
            .count(),
        6
    );
}

#[test]
fn relay_evidence_status_default_reports_truthful_unavailable_and_deferred_states() {
    // Arrange / Act
    let status = RelayEvidenceStatus::default_unavailable();
    let encoded = serde_json::to_value(status).expect("relay evidence json");

    // Assert
    assert_eq!(encoded["outcome_counters"]["state"], "implemented");
    assert_eq!(encoded["outcome_counters"]["value"]["accepted_count"], 0);
    assert_eq!(encoded["recovery_counters"]["state"], "implemented");
    assert_eq!(encoded["recovery_counters"]["value"]["recovered_count"], 0);
    assert_eq!(
        encoded["recovery_counters"]["value"]["dropped_evicted_count"],
        0
    );
    assert_eq!(encoded["activation"]["state"], "implemented");
    assert_eq!(encoded["activation"]["value"]["enabled"], false);
    assert_eq!(encoded["download_eligibility"]["state"], "implemented");
    assert_eq!(
        encoded["download_eligibility"]["value"]["eligible_peer_count"],
        0
    );
    assert_eq!(
        encoded["download_eligibility"]["value"]["protected_not_relay_count"],
        0
    );
    assert_eq!(encoded["mempool_admission"]["state"], "unavailable");
    assert_eq!(
        encoded["mempool_admission"]["value"]["reason"],
        "mempool admission evidence unavailable"
    );
    assert_eq!(encoded["local_submission"]["state"], "deferred");
    assert_eq!(
        encoded["local_submission"]["value"]["reason"],
        "local submission relay evidence not projected"
    );
    assert_eq!(encoded["fanout"]["state"], "deferred");
    assert_eq!(encoded["serving"]["state"], "deferred");
    assert_eq!(encoded["rebroadcast"]["state"], "deferred");
    assert_eq!(encoded["public_relay"]["state"], "intentionally_different");
    assert_eq!(
        encoded["public_relay"]["value"]["reason"],
        "public relay readiness is intentionally not claimed"
    );
}

#[test]
fn relay_evidence_status_classification_labels_are_exhaustive_and_safe() {
    // Arrange
    let fields = [
        RelayEvidenceField::implemented(RelayCapabilityEvidence::new(
            RelayEvidenceCapability::MempoolAdmission,
        )),
        RelayEvidenceField::<RelayCapabilityEvidence>::unavailable("not wired"),
        RelayEvidenceField::<RelayCapabilityEvidence>::deferred("future surface"),
        RelayEvidenceField::<RelayCapabilityEvidence>::intentionally_different("not claimed"),
    ];

    // Act
    let encoded = serde_json::to_value(fields).expect("relay evidence field json");

    // Assert
    assert_eq!(encoded[0]["state"], "implemented");
    assert_eq!(encoded[0]["value"]["capability"], "mempool_admission");
    assert_eq!(encoded[1]["state"], "unavailable");
    assert_eq!(encoded[1]["value"]["reason"], "not wired");
    assert_eq!(encoded[2]["state"], "deferred");
    assert_eq!(encoded[2]["value"]["reason"], "future surface");
    assert_eq!(encoded[3]["state"], "intentionally_different");
    assert_eq!(encoded[3]["value"]["reason"], "not claimed");
}

#[test]
fn relay_evidence_status_default_omits_sensitive_material_by_construction() {
    // Arrange / Act
    let encoded =
        serde_json::to_string(&RelayEvidenceStatus::default_unavailable()).expect("relay json");

    // Assert
    for forbidden in [
        "tx_hex",
        "raw_tx",
        "txid",
        "wtxid",
        "peer_id",
        "endpoint",
        "permission_string",
        "credential",
        "label",
    ] {
        assert!(!encoded.contains(forbidden));
    }
}
