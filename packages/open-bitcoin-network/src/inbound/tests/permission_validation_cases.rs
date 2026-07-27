// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use super::*;

#[test]
fn class_addresses_accept_only_literal_ip_values() {
    // Arrange
    let invalid_addresses = ["203.0.113.0/24", "peer.example", "203.0.113.7:8333"];

    for invalid_address in invalid_addresses {
        // Act
        let parsed =
            ParsedPeerPermissionClass::parse("bad-address", [invalid_address], ["in", "download"]);

        // Assert
        let Err(error) = parsed else {
            panic!("expected invalid literal IP rejection");
        };
        assert_eq!(error.field(), "inbound.permission_classes[].addresses[]");
        assert_eq!(error.reason(), "invalid_literal_ip_address");
        assert_eq!(error.token(), invalid_address);
    }
}

#[test]
fn connection_class_labels_and_slot_mapping_are_stable() {
    // Arrange
    let classes = [
        (
            PeerConnectionClass::OrdinaryInbound,
            "ordinary_inbound",
            InboundAdmissionSlotClass::Ordinary,
        ),
        (
            PeerConnectionClass::PermissionedInbound,
            "permissioned_inbound",
            InboundAdmissionSlotClass::Ordinary,
        ),
        (
            PeerConnectionClass::ProtectedInbound,
            "protected_inbound",
            InboundAdmissionSlotClass::Reserved,
        ),
        (
            PeerConnectionClass::Outbound,
            "outbound",
            InboundAdmissionSlotClass::Ordinary,
        ),
        (
            PeerConnectionClass::ManualConfigured,
            "manual_configured",
            InboundAdmissionSlotClass::Ordinary,
        ),
    ];

    // Act
    let labels: Vec<&str> = classes
        .iter()
        .map(|(connection_class, _label, _slot)| connection_class.as_str())
        .collect();

    // Assert
    assert_eq!(
        labels,
        vec![
            "ordinary_inbound",
            "permissioned_inbound",
            "protected_inbound",
            "outbound",
            "manual_configured",
        ],
    );
    for (connection_class, _label, slot_class) in classes {
        assert_eq!(connection_class.slot_class(), slot_class);
    }
}

#[test]
fn admission_slot_class_labels_are_stable() {
    // Arrange
    let slot_classes = [
        (InboundAdmissionSlotClass::Ordinary, "ordinary"),
        (InboundAdmissionSlotClass::Reserved, "reserved"),
    ];

    // Act
    let labels: Vec<&str> = slot_classes
        .into_iter()
        .map(|(slot_class, _label)| slot_class.as_str())
        .collect();

    // Assert
    assert_eq!(labels, vec!["ordinary", "reserved"]);
}
