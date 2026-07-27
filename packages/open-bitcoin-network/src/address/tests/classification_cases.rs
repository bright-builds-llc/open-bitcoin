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

use super::*;

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

#[test]
fn local_advertisement_uses_runtime_bound_address_before_configured_listener() {
    // Arrange
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    let inputs = [LocalAdvertisementInput {
        listener_endpoint: listener_endpoint("127.0.0.1:8333"),
        maybe_bound_addr: Some(socket_addr("8.8.8.8:18444")),
        services,
        allow_public: true,
    }];

    // Act
    let decisions = select_local_advertisement_candidates(&inputs);

    // Assert
    assert_eq!(decisions.len(), 1);
    let decision = &decisions[0];
    assert_eq!(decision.label, AddressDecisionLabel::AdvertiseCandidate);
    assert_eq!(decision.reason, AddressDecisionReason::PolicyAccepted);
    assert_eq!(decision.source, AddressSourceKind::LocalListener);
    assert_eq!(decision.network_kind, AddressNetworkKind::Ipv4);
    assert_eq!(decision.routability, RoutabilityClass::PubliclyRoutable);
    assert_eq!(decision.services_bits, services.bits());
    assert_eq!(decision.port, 18444);

    let wire_address = decision
        .maybe_wire_address
        .as_ref()
        .expect("candidate should have a wire address");
    assert_eq!(wire_address.services, services.bits());
    assert_eq!(wire_address.port, 18444);
    assert_eq!(
        wire_address.address_bytes,
        ipv4_mapped_address_bytes([8, 8, 8, 8]),
    );
    assert_eq!(
        maybe_version_sender_address(&decisions),
        decision.maybe_wire_address.clone(),
    );
}

#[test]
fn local_private_documentation_and_multicast_listener_addresses_are_not_advertised() {
    // Arrange
    let inputs: Vec<_> = [
        "127.0.0.1:8333",
        "10.0.0.1:8333",
        "172.16.0.1:8333",
        "192.168.0.1:8333",
        "0.0.0.0:8333",
        "224.0.0.1:8333",
        "192.0.2.1:8333",
        "203.0.113.1:8333",
        "[::1]:8333",
        "[::]:8333",
        "[ff02::1]:8333",
        "[fc00::1]:8333",
        "[fe80::1]:8333",
        "[2001:db8::1]:8333",
    ]
    .into_iter()
    .map(|raw_endpoint| LocalAdvertisementInput {
        listener_endpoint: listener_endpoint(raw_endpoint),
        maybe_bound_addr: None,
        services: ServiceFlags::NETWORK,
        allow_public: true,
    })
    .collect();

    // Act
    let decisions = select_local_advertisement_candidates(&inputs);

    // Assert
    assert_eq!(decisions.len(), inputs.len());
    assert!(decisions.iter().all(|decision| {
        decision.label == AddressDecisionLabel::AdvertiseSuppressed
            && decision.reason == AddressDecisionReason::NotPubliclyRoutable
            && decision.source == AddressSourceKind::LocalListener
            && decision.routability == RoutabilityClass::NotPubliclyRoutable
            && decision.maybe_wire_address.is_none()
    }));
}

#[test]
fn public_listener_with_public_acknowledgement_is_advertised_and_used_for_version_sender() {
    // Arrange
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    let inputs = [LocalAdvertisementInput {
        listener_endpoint: listener_endpoint("8.8.4.4:8333"),
        maybe_bound_addr: None,
        services,
        allow_public: true,
    }];

    // Act
    let decisions = select_local_advertisement_candidates(&inputs);
    let maybe_sender = maybe_version_sender_address(&decisions);

    // Assert
    assert_eq!(decisions.len(), 1);
    let decision = &decisions[0];
    assert_eq!(decision.label, AddressDecisionLabel::AdvertiseCandidate);
    assert_eq!(decision.reason, AddressDecisionReason::PolicyAccepted);
    assert_eq!(decision.source, AddressSourceKind::LocalListener);
    assert_eq!(decision.network_kind, AddressNetworkKind::Ipv4);
    assert_eq!(decision.routability, RoutabilityClass::PubliclyRoutable);
    assert_eq!(decision.services_bits, services.bits());
    assert_eq!(decision.port, 8333);
    assert_eq!(maybe_sender, decision.maybe_wire_address.clone());
}

#[test]
fn version_sender_address_stays_empty_without_advertisement_candidate() {
    // Arrange
    let inputs = [
        LocalAdvertisementInput {
            listener_endpoint: listener_endpoint("127.0.0.1:8333"),
            maybe_bound_addr: None,
            services: ServiceFlags::NETWORK,
            allow_public: true,
        },
        LocalAdvertisementInput {
            listener_endpoint: listener_endpoint("8.8.8.8:8333"),
            maybe_bound_addr: None,
            services: ServiceFlags::NETWORK,
            allow_public: false,
        },
    ];

    // Act
    let decisions = select_local_advertisement_candidates(&inputs);
    let maybe_sender = maybe_version_sender_address(&decisions);

    // Assert
    assert_eq!(decisions.len(), 2);
    assert_eq!(
        decisions
            .iter()
            .map(|decision| decision.label)
            .collect::<Vec<_>>(),
        vec![
            AddressDecisionLabel::AdvertiseSuppressed,
            AddressDecisionLabel::AdvertiseSuppressed,
        ],
    );
    assert_eq!(
        decisions
            .iter()
            .map(|decision| decision.reason)
            .collect::<Vec<_>>(),
        vec![
            AddressDecisionReason::NotPubliclyRoutable,
            AddressDecisionReason::PermissionPolicyDenied,
        ],
    );
    assert_eq!(maybe_sender, None);
}

#[test]
fn learned_public_ipv4_and_ipv6_from_inbound_addr_are_persistence_eligible() {
    // Arrange
    let mut book = LearnedAddressBook::default();
    let now_unix_seconds = 1_700_000_000;
    let announcements = [
        address_announcement(
            now_unix_seconds,
            public_ipv4_network_address(8, 8, 8, 8, 8333),
        ),
        address_announcement(
            now_unix_seconds,
            public_ipv6_network_address("2606:4700:4700::1111", 8333),
        ),
    ];

    // Act
    let batch = book.learn_batch(
        &announcements,
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    );

    // Assert
    assert_eq!(batch.decisions.len(), 2);
    assert_eq!(batch.accepted_count, 2);
    assert_eq!(batch.rejected_count, 0);
    assert!(batch.decisions.iter().all(|decision| {
        decision.label == AddressDecisionLabel::LearnedAccepted
            && decision.reason == AddressDecisionReason::PolicyAccepted
            && decision
                .maybe_entry
                .as_ref()
                .is_some_and(|entry| entry.persistence_eligible)
    }));
    assert_eq!(book.entries().len(), 2);
    assert!(book.entries().iter().all(|entry| {
        entry.source == AddressSourceKind::InboundAddr
            && entry.routability == RoutabilityClass::PubliclyRoutable
            && entry.persistence_eligible
            && entry.first_seen_unix_seconds == now_unix_seconds
            && entry.last_seen_unix_seconds == now_unix_seconds
    }));
}
