// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

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
        inactive_permission_effect_observations: 0,
        permission_validation_failures: 0,
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
        eviction_candidates_evaluated: 0,
        disconnects_requested: 0,
        discouraged_peers: 0,
        active_bans: 0,
        expired_bans: 0,
        manual_unbans: 0,
        misbehavior_observations: 0,
        protected_no_actions: 0,
        latest_peer_policy_decision: FieldAvailability::unavailable(
            "inbound peer policy evidence unavailable",
        ),
        resource_pressure_events: 0,
        read_queue_pressure_events: 0,
        write_queue_pressure_events: 0,
        request_cap_events: 0,
        payload_rejections: 0,
        timeout_disconnects: 0,
        churn_rejections: 0,
        reconnect_suppressions: 0,
        latest_resource_governance_decision: FieldAvailability::unavailable(
            "inbound resource governance evidence unavailable",
        ),
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

#[test]
fn status_metrics_json_preserves_retained_inbound_samples_without_dynamic_labels() {
    // Arrange
    let mut snapshot = stopped_snapshot();
    snapshot.metrics = MetricsStatus::available_with_samples(
        MetricRetentionPolicy::default(),
        vec![MetricSample::new(
            MetricKind::InboundResourcePressureActiveCount,
            16.0,
            1_777_225_022,
        )],
    );

    // Act
    let encoded = serde_json::to_value(snapshot).expect("status snapshot json");
    let sample = encoded["metrics"]["samples"][0]
        .as_object()
        .expect("metric sample object");

    // Assert
    assert_eq!(
        sample.get("kind").expect("metric kind"),
        "inbound_resource_pressure_active_count"
    );
    assert_eq!(sample.get("value").expect("metric value"), 16.0);
    assert_eq!(
        sample
            .get("timestamp_unix_seconds")
            .expect("metric timestamp"),
        1_777_225_022
    );
    for forbidden in ["peer_id", "endpoint", "permission_class", "label"] {
        assert!(!sample.contains_key(forbidden));
    }
}
