// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    INBOUND_ADDRESS_DECISION_UNAVAILABLE_REASON, INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON,
    INBOUND_STATUS_UNAVAILABLE_REASON, InboundAddressDecisionEvent, InboundAddressEvidenceEntry,
    InboundAdmissionEvent, InboundHandshakeStatusCounts, InboundPeerServingStatus,
    InboundPermissionDecisionEvent,
};
use crate::status::{FieldAvailability, PeerCounts, PeerStatus};

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
