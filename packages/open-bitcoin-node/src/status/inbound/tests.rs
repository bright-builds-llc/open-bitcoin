// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    INBOUND_ADDRESS_DECISION_UNAVAILABLE_REASON, INBOUND_PEER_POLICY_DECISION_UNAVAILABLE_REASON,
    INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON, INBOUND_RESOURCE_DECISION_UNAVAILABLE_REASON,
    INBOUND_STATUS_UNAVAILABLE_REASON, InboundAddressDecisionEvent, InboundAddressEvidenceEntry,
    InboundAdmissionEvent, InboundHandshakeStatusCounts, InboundPeerPolicyEvent,
    InboundPeerServingStatus, InboundPermissionDecisionEvent, InboundResourceGovernanceEvent,
};
use crate::network::ManagedResourceGovernanceInfo;
use crate::status::{FieldAvailability, PeerCounts, PeerStatus};
use open_bitcoin_network::InboundResourceEvent;

#[test]
fn inbound_status_default_serializes_unavailable_reason() {
    // Arrange
    let peers = PeerStatus {
        peer_counts: FieldAvailability::unavailable("node stopped"),
        recent_peers: FieldAvailability::unavailable("node stopped"),
        inbound: FieldAvailability::unavailable(INBOUND_STATUS_UNAVAILABLE_REASON),
    };

    // Act
    let encoded = serde_json::to_value(peers).expect("peer status json");

    // Assert
    assert_eq!(encoded["inbound"]["state"], "unavailable");
    assert_eq!(
        encoded["inbound"]["value"]["reason"],
        INBOUND_STATUS_UNAVAILABLE_REASON
    );
}

#[test]
fn inbound_status_defaults_legacy_peer_status_json() {
    // Arrange
    let legacy_json = serde_json::json!({
        "peer_counts": {
            "state": "available",
            "value": {
                "inbound": 0,
                "outbound": 2
            }
        },
        "recent_peers": {
            "state": "unavailable",
            "value": {
                "reason": "peer telemetry unavailable"
            }
        }
    });

    // Act
    let peers: PeerStatus = serde_json::from_value(legacy_json).expect("legacy peer status json");

    // Assert
    assert_eq!(
        peers.inbound,
        FieldAvailability::<InboundPeerServingStatus>::unavailable(
            INBOUND_STATUS_UNAVAILABLE_REASON
        )
    );
}

#[test]
fn inbound_status_serializes_listener_and_admission_evidence() {
    // Arrange
    let peers = PeerStatus {
        peer_counts: FieldAvailability::available(PeerCounts {
            inbound: 2,
            outbound: 4,
        }),
        recent_peers: FieldAvailability::available(Vec::new()),
        inbound: FieldAvailability::available(InboundPeerServingStatus {
            listener_state: "ready".to_string(),
            bound_endpoints: vec!["127.0.0.1:18444".to_string()],
            preflight_reason: "ready".to_string(),
            admitted_inbound_peers: 2,
            rejected_inbound_peers: 5,
            handshake: InboundHandshakeStatusCounts {
                awaiting_version: 1,
                awaiting_verack: 1,
                established: 2,
                disconnected: 0,
            },
            duplicate_rejects: 2,
            self_connection_rejects: 1,
            cap_rejects: 1,
            reserved_slot_rejects: 1,
            latest_admission_event: FieldAvailability::available(InboundAdmissionEvent {
                outcome: "rejected".to_string(),
                reason: "cap_reached".to_string(),
                slot_class: "ordinary".to_string(),
                message: "inbound peer cap has been reached".to_string(),
            }),
            permissioned_inbound_peers: 1,
            protected_inbound_peers: 1,
            permission_class: "protected_inbound".to_string(),
            active_permission_effects: vec![
                "admission_protected".to_string(),
                "eviction_policy_protected".to_string(),
            ],
            inactive_permission_effects: vec![
                "inactive_relay".to_string(),
                "inactive_mempool".to_string(),
            ],
            inactive_permission_effect_observations: 2,
            permission_validation_failures: 0,
            latest_permission_decision: FieldAvailability::available(
                InboundPermissionDecisionEvent {
                    outcome: "admitted".to_string(),
                    reason: "admitted".to_string(),
                    permission_class: "protected_inbound".to_string(),
                    active_permission_effects: vec!["admission_protected".to_string()],
                    inactive_permission_effects: vec!["inactive_relay".to_string()],
                    message: "inbound permission decision admitted as protected_inbound"
                        .to_string(),
                },
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
            suppressed_advertisements: vec![InboundAddressDecisionEvent {
                outcome: "suppressed".to_string(),
                reason: "not_inbound".to_string(),
                label: "advertise_suppressed".to_string(),
                source: "source_local_listener".to_string(),
                message: "local listener advertisement suppressed".to_string(),
            }],
            getaddr_responses_served: 1,
            getaddr_requests_suppressed: 2,
            learned_address_entries: 3,
            learned_address_rejections: 4,
            latest_address_decision: FieldAvailability::available(InboundAddressDecisionEvent {
                outcome: "served".to_string(),
                reason: "empty_response_cache".to_string(),
                label: "getaddr_served".to_string(),
                source: "source_inbound_addr".to_string(),
                message: "bounded getaddr response served".to_string(),
            }),
            eviction_candidates_evaluated: 2,
            disconnects_requested: 1,
            discouraged_peers: 1,
            active_bans: 1,
            expired_bans: 1,
            manual_unbans: 1,
            misbehavior_observations: 3,
            protected_no_actions: 1,
            latest_peer_policy_decision: FieldAvailability::available(InboundPeerPolicyEvent {
                outcome: "selected".to_string(),
                reason: "low_activity".to_string(),
                label: "eviction_candidate_selected".to_string(),
                source: "source_eviction_policy".to_string(),
                message: "peer eviction decision eviction_candidate_selected: low_activity"
                    .to_string(),
            }),
            resource_pressure_events: 1,
            read_queue_pressure_events: 1,
            write_queue_pressure_events: 1,
            request_cap_events: 1,
            payload_rejections: 1,
            timeout_disconnects: 1,
            churn_rejections: 1,
            reconnect_suppressions: 1,
            latest_resource_governance_decision: FieldAvailability::available(
                InboundResourceGovernanceEvent {
                    outcome: "resource_governance".to_string(),
                    reason: "payload rejected".to_string(),
                    label: "invalid_checksum".to_string(),
                    source: "source_envelope_gate".to_string(),
                    message: "inbound_message_resource_governance".to_string(),
                    next_action: "payload_rejected".to_string(),
                },
            ),
        }),
    };

    // Act
    let encoded = serde_json::to_value(peers).expect("peer status json");

    // Assert
    assert_eq!(encoded["inbound"]["state"], "available");
    assert_eq!(encoded["inbound"]["value"]["listener_state"], "ready");
    assert_eq!(encoded["inbound"]["value"]["preflight_reason"], "ready");
    assert_eq!(
        encoded["inbound"]["value"]["bound_endpoints"],
        serde_json::json!(["127.0.0.1:18444"])
    );
    assert_eq!(encoded["inbound"]["value"]["admitted_inbound_peers"], 2);
    assert_eq!(encoded["inbound"]["value"]["rejected_inbound_peers"], 5);
    assert_eq!(
        encoded["inbound"]["value"]["handshake"]["awaiting_version"],
        1
    );
    assert_eq!(
        encoded["inbound"]["value"]["handshake"]["awaiting_verack"],
        1
    );
    assert_eq!(encoded["inbound"]["value"]["handshake"]["established"], 2);
    assert_eq!(encoded["inbound"]["value"]["handshake"]["disconnected"], 0);
    assert_eq!(encoded["inbound"]["value"]["duplicate_rejects"], 2);
    assert_eq!(encoded["inbound"]["value"]["self_connection_rejects"], 1);
    assert_eq!(encoded["inbound"]["value"]["cap_rejects"], 1);
    assert_eq!(encoded["inbound"]["value"]["reserved_slot_rejects"], 1);
    assert_eq!(
        encoded["inbound"]["value"]["latest_admission_event"]["value"]["outcome"],
        "rejected"
    );
    assert_eq!(
        encoded["inbound"]["value"]["latest_admission_event"]["value"]["reason"],
        "cap_reached"
    );
    assert_eq!(
        encoded["inbound"]["value"]["latest_admission_event"]["value"]["slot_class"],
        "ordinary"
    );
    assert_eq!(encoded["inbound"]["value"]["permissioned_inbound_peers"], 1);
    assert_eq!(encoded["inbound"]["value"]["protected_inbound_peers"], 1);
    assert_eq!(
        encoded["inbound"]["value"]["permission_class"],
        "protected_inbound"
    );
    assert_eq!(
        encoded["inbound"]["value"]["active_permission_effects"],
        serde_json::json!(["admission_protected", "eviction_policy_protected"])
    );
    assert_eq!(
        encoded["inbound"]["value"]["inactive_permission_effects"],
        serde_json::json!(["inactive_relay", "inactive_mempool"])
    );
    assert_eq!(
        encoded["inbound"]["value"]["latest_permission_decision"]["value"]["permission_class"],
        "protected_inbound"
    );
    assert_eq!(
        encoded["inbound"]["value"]["latest_permission_decision"]["value"]["inactive_permission_effects"],
        serde_json::json!(["inactive_relay"])
    );
    assert_eq!(
        encoded["inbound"]["value"]["local_advertisement_candidates"][0]["source"],
        "source_local_listener"
    );
    assert_eq!(
        encoded["inbound"]["value"]["local_advertisement_candidates"][0]["network_kind"],
        "ipv4"
    );
    assert_eq!(
        encoded["inbound"]["value"]["local_advertisement_candidates"][0]["routability"],
        "publicly_routable"
    );
    assert_eq!(
        encoded["inbound"]["value"]["local_advertisement_candidates"][0]["freshness"],
        "fresh"
    );
    assert_eq!(
        encoded["inbound"]["value"]["local_advertisement_candidates"][0]["services_bits"],
        1
    );
    assert_eq!(
        encoded["inbound"]["value"]["local_advertisement_candidates"][0]["port"],
        18_444
    );
    assert_eq!(
        encoded["inbound"]["value"]["local_advertisement_candidates"][0]["persistence_eligible"],
        true
    );
    assert_eq!(
        encoded["inbound"]["value"]["suppressed_advertisements"][0]["reason"],
        "not_inbound"
    );
    assert_eq!(
        encoded["inbound"]["value"]["suppressed_advertisements"][0]["label"],
        "advertise_suppressed"
    );
    assert_eq!(encoded["inbound"]["value"]["getaddr_responses_served"], 1);
    assert_eq!(
        encoded["inbound"]["value"]["getaddr_requests_suppressed"],
        2
    );
    assert_eq!(encoded["inbound"]["value"]["learned_address_entries"], 3);
    assert_eq!(encoded["inbound"]["value"]["learned_address_rejections"], 4);
    assert_eq!(
        encoded["inbound"]["value"]["latest_address_decision"]["value"]["label"],
        "getaddr_served"
    );
    assert_eq!(
        encoded["inbound"]["value"]["eviction_candidates_evaluated"],
        2
    );
    assert_eq!(encoded["inbound"]["value"]["disconnects_requested"], 1);
    assert_eq!(encoded["inbound"]["value"]["discouraged_peers"], 1);
    assert_eq!(encoded["inbound"]["value"]["active_bans"], 1);
    assert_eq!(encoded["inbound"]["value"]["expired_bans"], 1);
    assert_eq!(encoded["inbound"]["value"]["manual_unbans"], 1);
    assert_eq!(encoded["inbound"]["value"]["misbehavior_observations"], 3);
    assert_eq!(encoded["inbound"]["value"]["protected_no_actions"], 1);
    assert_eq!(
        encoded["inbound"]["value"]["latest_peer_policy_decision"]["value"]["label"],
        "eviction_candidate_selected"
    );
    assert_eq!(encoded["inbound"]["value"]["resource_pressure_events"], 1);
    assert_eq!(encoded["inbound"]["value"]["read_queue_pressure_events"], 1);
    assert_eq!(
        encoded["inbound"]["value"]["write_queue_pressure_events"],
        1
    );
    assert_eq!(encoded["inbound"]["value"]["request_cap_events"], 1);
    assert_eq!(encoded["inbound"]["value"]["payload_rejections"], 1);
    assert_eq!(encoded["inbound"]["value"]["timeout_disconnects"], 1);
    assert_eq!(encoded["inbound"]["value"]["churn_rejections"], 1);
    assert_eq!(encoded["inbound"]["value"]["reconnect_suppressions"], 1);
    assert_eq!(
        encoded["inbound"]["value"]["latest_resource_governance_decision"]["value"]["next_action"],
        "payload_rejected"
    );
}

#[test]
fn inbound_status_permission_fields_default_for_legacy_status_json() {
    // Arrange
    let legacy_status = serde_json::json!({
        "listener_state": "listening",
        "bound_endpoints": [],
        "preflight_reason": "ready",
        "admitted_inbound_peers": 1,
        "rejected_inbound_peers": 0,
        "handshake": {
            "awaiting_version": 0,
            "awaiting_verack": 0,
            "established": 1,
            "disconnected": 0
        },
        "duplicate_rejects": 0,
        "self_connection_rejects": 0,
        "cap_rejects": 0,
        "reserved_slot_rejects": 0,
        "latest_admission_event": {
            "state": "available",
            "value": {
                "outcome": "admitted",
                "reason": "admitted",
                "slot_class": "ordinary",
                "message": "inbound peer admitted"
            }
        }
    });

    // Act
    let status: InboundPeerServingStatus =
        serde_json::from_value(legacy_status).expect("legacy inbound status");

    // Assert
    assert_eq!(status.permissioned_inbound_peers, 0);
    assert_eq!(status.protected_inbound_peers, 0);
    assert_eq!(status.permission_class, "ordinary_inbound");
    assert!(status.active_permission_effects.is_empty());
    assert!(status.inactive_permission_effects.is_empty());
    assert_eq!(
        status.latest_permission_decision,
        FieldAvailability::<InboundPermissionDecisionEvent>::unavailable(
            INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON
        )
    );
    assert!(status.local_advertisement_candidates.is_empty());
    assert!(status.suppressed_advertisements.is_empty());
    assert_eq!(status.getaddr_responses_served, 0);
    assert_eq!(status.getaddr_requests_suppressed, 0);
    assert_eq!(status.learned_address_entries, 0);
    assert_eq!(status.learned_address_rejections, 0);
    assert_eq!(
        status.latest_address_decision,
        FieldAvailability::<InboundAddressDecisionEvent>::unavailable(
            INBOUND_ADDRESS_DECISION_UNAVAILABLE_REASON
        )
    );
    assert_eq!(status.eviction_candidates_evaluated, 0);
    assert_eq!(status.disconnects_requested, 0);
    assert_eq!(status.discouraged_peers, 0);
    assert_eq!(status.active_bans, 0);
    assert_eq!(status.expired_bans, 0);
    assert_eq!(status.manual_unbans, 0);
    assert_eq!(status.misbehavior_observations, 0);
    assert_eq!(status.protected_no_actions, 0);
    assert_eq!(
        status.latest_peer_policy_decision,
        FieldAvailability::<InboundPeerPolicyEvent>::unavailable(
            INBOUND_PEER_POLICY_DECISION_UNAVAILABLE_REASON
        )
    );
    assert_eq!(status.resource_pressure_events, 0);
    assert_eq!(status.read_queue_pressure_events, 0);
    assert_eq!(status.write_queue_pressure_events, 0);
    assert_eq!(status.request_cap_events, 0);
    assert_eq!(status.payload_rejections, 0);
    assert_eq!(status.timeout_disconnects, 0);
    assert_eq!(status.churn_rejections, 0);
    assert_eq!(status.reconnect_suppressions, 0);
    assert_eq!(
        status.latest_resource_governance_decision,
        FieldAvailability::<InboundResourceGovernanceEvent>::unavailable(
            INBOUND_RESOURCE_DECISION_UNAVAILABLE_REASON
        )
    );
}

#[test]
fn inbound_status_address_entries_exclude_raw_peer_and_address_details() {
    // Arrange
    let status = InboundPeerServingStatus {
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
            INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON,
        ),
        local_advertisement_candidates: vec![InboundAddressEvidenceEntry {
            source: "source_local_listener".to_string(),
            network_kind: "ipv6".to_string(),
            routability: "publicly_routable".to_string(),
            freshness: "fresh".to_string(),
            services_bits: 1,
            port: 8_333,
            persistence_eligible: true,
        }],
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: 0,
        getaddr_requests_suppressed: 0,
        learned_address_entries: 1,
        learned_address_rejections: 0,
        latest_address_decision: FieldAvailability::available(InboundAddressDecisionEvent {
            outcome: "accepted".to_string(),
            reason: "permission_policy_denied".to_string(),
            label: "learned_accepted".to_string(),
            source: "source_inbound_addr".to_string(),
            message: "learned address evidence accepted".to_string(),
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
            INBOUND_PEER_POLICY_DECISION_UNAVAILABLE_REASON,
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
            INBOUND_RESOURCE_DECISION_UNAVAILABLE_REASON,
        ),
    };

    // Act
    let encoded = serde_json::to_value(status).expect("inbound address evidence json");
    let encoded_text = encoded.to_string();

    // Assert
    assert_eq!(
        encoded["local_advertisement_candidates"][0]["source"],
        "source_local_listener"
    );
    assert_eq!(
        encoded["latest_address_decision"]["value"]["reason"],
        "permission_policy_denied"
    );
    assert!(!encoded_text.contains("address_bytes"));
    assert!(!encoded_text.contains("peer_id"));
    assert!(!encoded_text.contains("raw_permission"));
    assert!(!encoded_text.contains("raw_config"));
    assert!(!encoded_text.contains("class_name"));
}

#[test]
fn inbound_status_address_decision_labels_cover_boundary_contract() {
    // Arrange
    let labels = [
        "advertise_candidate",
        "advertise_suppressed",
        "getaddr_served",
        "getaddr_suppressed",
        "learned_accepted",
        "learned_rejected",
    ];

    // Act
    let events: Vec<_> = labels
        .into_iter()
        .map(|label| InboundAddressDecisionEvent {
            outcome: "recorded".to_string(),
            reason: "policy_accepted".to_string(),
            label: label.to_string(),
            source: "source_inbound_addr".to_string(),
            message: format!("address boundary decision {label}"),
        })
        .collect();

    // Assert
    assert_eq!(events.len(), 6);
    assert_eq!(events[0].label, "advertise_candidate");
    assert_eq!(events[3].label, "getaddr_suppressed");
    assert_eq!(events[5].label, "learned_rejected");
}

#[test]
fn managed_resource_governance_payload_rejected_sets_latest_decision() {
    // Arrange
    let event = resource_event("payload_rejected");
    let mut info = ManagedResourceGovernanceInfo::default();

    // Act
    info.record_event(event);

    // Assert
    assert_eq!(info.payload_rejections, 1);
    assert_eq!(
        info.maybe_latest_resource_governance_decision
            .expect("latest resource governance event")
            .next_action,
        "payload_rejected"
    );
}

#[test]
fn managed_resource_governance_maps_next_actions_to_separate_counters() {
    // Arrange
    let mut info = ManagedResourceGovernanceInfo::default();
    let actions = [
        "resource_pressure_active",
        "read_queue_pressure",
        "write_queue_pressure",
        "request_cap_reached",
        "payload_rejected",
        "timeout_disconnect",
        "churn_rejected",
        "reconnect_suppressed",
    ];

    // Act
    for action in actions {
        info.record_event(resource_event(action));
    }

    // Assert
    assert_eq!(info.resource_pressure_events, 1);
    assert_eq!(info.read_queue_pressure_events, 1);
    assert_eq!(info.write_queue_pressure_events, 1);
    assert_eq!(info.request_cap_events, 1);
    assert_eq!(info.payload_rejections, 1);
    assert_eq!(info.timeout_disconnects, 1);
    assert_eq!(info.churn_rejections, 1);
    assert_eq!(info.reconnect_suppressions, 1);
    assert_eq!(
        info.maybe_latest_resource_governance_decision
            .expect("latest resource governance event")
            .next_action,
        "reconnect_suppressed"
    );
}

fn resource_event(next_action: &str) -> InboundResourceEvent {
    InboundResourceEvent {
        outcome: "resource_governance".to_string(),
        reason: format!("{next_action} reason"),
        label: next_action.to_string(),
        source: "source_runtime_read".to_string(),
        message: "inbound_resource_governance".to_string(),
        next_action: next_action.to_string(),
    }
}
