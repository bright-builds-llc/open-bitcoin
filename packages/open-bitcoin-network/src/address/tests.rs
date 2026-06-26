// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/protocol.cpp
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netaddress.cpp
// - packages/bitcoin-knots/src/net.h
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/addrman.h
// - packages/bitcoin-knots/src/addrman.cpp
// - packages/bitcoin-knots/src/addrdb.h
// - packages/bitcoin-knots/src/addrdb.cpp

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::ServiceFlags;

use super::{
    AddressDecisionLabel, AddressDecisionReason, AddressNetworkKind, AddressSourceKind,
    RoutabilityClass, classify_network_address, privacy_network_deferred_classification,
    unsupported_future_network_classification,
};

#[test]
fn stable_address_boundary_labels_match_phase92_contract() {
    // Arrange
    let network_kinds = [
        AddressNetworkKind::Ipv4,
        AddressNetworkKind::Ipv6,
        AddressNetworkKind::UnsupportedPrivacyNetwork,
        AddressNetworkKind::UnsupportedFutureNetwork,
    ];
    let routability_classes = [
        RoutabilityClass::PubliclyRoutable,
        RoutabilityClass::NotPubliclyRoutable,
        RoutabilityClass::Invalid,
        RoutabilityClass::PrivacyNetworkDeferred,
    ];
    let source_kinds = [
        AddressSourceKind::LocalListener,
        AddressSourceKind::InboundAddr,
    ];
    let decision_labels = [
        AddressDecisionLabel::AdvertiseCandidate,
        AddressDecisionLabel::AdvertiseSuppressed,
        AddressDecisionLabel::LearnedAccepted,
        AddressDecisionLabel::LearnedRejected,
        AddressDecisionLabel::GetAddrServed,
        AddressDecisionLabel::GetAddrSuppressed,
        AddressDecisionLabel::FullRelayDeferred,
    ];
    let decision_reasons = [
        AddressDecisionReason::PolicyAccepted,
        AddressDecisionReason::NotPubliclyRoutable,
        AddressDecisionReason::PrivacyNetworkDeferred,
        AddressDecisionReason::UnsupportedAddressNetwork,
        AddressDecisionReason::InvalidPort,
        AddressDecisionReason::StaleOrFuture,
        AddressDecisionReason::DuplicateAddress,
        AddressDecisionReason::OverCapBatch,
        AddressDecisionReason::NotInbound,
        AddressDecisionReason::PermissionPolicyDenied,
        AddressDecisionReason::AlreadyServed,
        AddressDecisionReason::EmptyResponseCache,
    ];

    // Act
    let network_kind_labels: Vec<&str> = network_kinds
        .into_iter()
        .map(AddressNetworkKind::as_str)
        .collect();
    let routability_labels: Vec<&str> = routability_classes
        .into_iter()
        .map(RoutabilityClass::as_str)
        .collect();
    let source_labels: Vec<&str> = source_kinds
        .into_iter()
        .map(AddressSourceKind::as_str)
        .collect();
    let decision_label_values: Vec<&str> = decision_labels
        .into_iter()
        .map(AddressDecisionLabel::as_str)
        .collect();
    let decision_reason_labels: Vec<&str> = decision_reasons
        .into_iter()
        .map(AddressDecisionReason::as_str)
        .collect();

    // Assert
    assert_eq!(
        network_kind_labels,
        vec![
            "ipv4",
            "ipv6",
            "unsupported_privacy_network",
            "unsupported_future_network",
        ],
    );
    assert_eq!(
        routability_labels,
        vec![
            "publicly_routable",
            "not_publicly_routable",
            "invalid",
            "privacy_network_deferred",
        ],
    );
    assert_eq!(
        source_labels,
        vec!["source_local_listener", "source_inbound_addr"]
    );
    assert_eq!(
        decision_label_values,
        vec![
            "advertise_candidate",
            "advertise_suppressed",
            "learned_accepted",
            "learned_rejected",
            "getaddr_served",
            "getaddr_suppressed",
            "full_relay_deferred",
        ],
    );
    assert_eq!(
        decision_reason_labels,
        vec![
            "policy_accepted",
            "not_publicly_routable",
            "privacy_network_deferred",
            "unsupported_address_network",
            "invalid_port",
            "stale_or_future",
            "duplicate_address",
            "over_cap_batch",
            "not_inbound",
            "permission_policy_denied",
            "already_served",
            "empty_response_cache",
        ],
    );
}

#[test]
fn public_ipv4_and_ipv6_inputs_classify_as_publicly_routable() {
    // Arrange
    let cases = [
        IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
        IpAddr::V6(
            "2606:4700:4700::1111"
                .parse::<Ipv6Addr>()
                .expect("test IPv6 should parse"),
        ),
    ];

    // Act
    let classifications: Vec<_> = cases
        .into_iter()
        .map(|address| classify_network_address(address, 8333, ServiceFlags::NETWORK))
        .collect();

    // Assert
    assert_eq!(
        classifications
            .iter()
            .map(|classification| classification.routability)
            .collect::<Vec<_>>(),
        vec![
            RoutabilityClass::PubliclyRoutable,
            RoutabilityClass::PubliclyRoutable,
        ],
    );
    assert!(classifications.iter().all(|classification| {
        classification.maybe_wire_address.is_some()
            && classification.reason == AddressDecisionReason::PolicyAccepted
    }));
}

#[test]
fn local_private_documentation_and_multicast_inputs_are_not_publicly_routable() {
    // Arrange
    let cases = [
        "127.0.0.1",
        "10.0.0.1",
        "172.16.0.1",
        "192.168.0.1",
        "0.0.0.0",
        "224.0.0.1",
        "192.0.2.1",
        "203.0.113.1",
        "169.254.0.1",
        "::1",
        "::",
        "ff02::1",
        "fc00::1",
        "fe80::1",
        "2001:db8::1",
    ];

    // Act
    let classifications: Vec<_> = cases
        .into_iter()
        .map(|raw_address| {
            classify_network_address(
                raw_address.parse().expect("test IP address should parse"),
                8333,
                ServiceFlags::NETWORK,
            )
        })
        .collect();

    // Assert
    assert!(classifications.iter().all(|classification| {
        classification.routability == RoutabilityClass::NotPubliclyRoutable
            && classification.reason == AddressDecisionReason::NotPubliclyRoutable
            && classification.maybe_wire_address.is_none()
    }));
}

#[test]
fn unsupported_privacy_networks_are_deferred_without_wire_address_bytes() {
    // Arrange
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;

    // Act
    let classification = privacy_network_deferred_classification(services);

    // Assert
    assert_eq!(
        classification.network_kind,
        AddressNetworkKind::UnsupportedPrivacyNetwork,
    );
    assert_eq!(
        classification.routability,
        RoutabilityClass::PrivacyNetworkDeferred,
    );
    assert_eq!(
        classification.reason,
        AddressDecisionReason::PrivacyNetworkDeferred,
    );
    assert_eq!(classification.services_bits, services.bits());
    assert_eq!(classification.maybe_wire_address, None);
}

#[test]
fn zero_port_inputs_are_invalid_without_wire_address_bytes() {
    // Arrange
    let address = IpAddr::V4(Ipv4Addr::new(8, 8, 4, 4));

    // Act
    let classification = classify_network_address(address, 0, ServiceFlags::NETWORK);

    // Assert
    assert_eq!(classification.network_kind, AddressNetworkKind::Ipv4);
    assert_eq!(classification.routability, RoutabilityClass::Invalid);
    assert_eq!(classification.reason, AddressDecisionReason::InvalidPort);
    assert_eq!(classification.port, 0);
    assert_eq!(classification.maybe_wire_address, None);
}

#[test]
fn unsupported_future_networks_are_rejected_without_wire_address_bytes() {
    // Arrange
    let services = ServiceFlags::NETWORK;

    // Act
    let classification = unsupported_future_network_classification(services);

    // Assert
    assert_eq!(
        classification.network_kind,
        AddressNetworkKind::UnsupportedFutureNetwork,
    );
    assert_eq!(classification.routability, RoutabilityClass::Invalid);
    assert_eq!(
        classification.reason,
        AddressDecisionReason::UnsupportedAddressNetwork,
    );
    assert_eq!(classification.services_bits, services.bits());
    assert_eq!(classification.maybe_wire_address, None);
}
