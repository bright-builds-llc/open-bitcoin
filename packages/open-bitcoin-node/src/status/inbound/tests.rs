// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON, INBOUND_STATUS_UNAVAILABLE_REASON,
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
}
