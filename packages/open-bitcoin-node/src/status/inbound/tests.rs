// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    INBOUND_STATUS_UNAVAILABLE_REASON, InboundAdmissionEvent, InboundHandshakeStatusCounts,
    InboundPeerServingStatus,
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
}
