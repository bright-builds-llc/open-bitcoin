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

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use open_bitcoin_primitives::NetworkAddress;

use crate::{InboundListenerEndpoint, PermissionEffectLabel, ServiceFlags};

use super::{
    AddressAnnouncement, AddressDecisionLabel, AddressDecisionReason, AddressNetworkKind,
    AddressResponseCache, AddressResponseEntryEvidence, AddressSourceKind, GetAddrPeerEligibility,
    GetAddrRequestState, GetAddrResponseDecision, LearnedAddressBook, LocalAdvertisementInput,
    PHASE92_GETADDR_RESPONSE_LIMIT, PHASE92_LEARNED_ADDR_BATCH_LIMIT, PHASE92_MAX_ADDR_AGE_SECONDS,
    PHASE92_MAX_FUTURE_SKEW_SECONDS, RoutabilityClass, classify_network_address,
    maybe_version_sender_address, privacy_network_deferred_classification, select_getaddr_response,
    select_local_advertisement_candidates, unsupported_future_network_classification,
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

#[test]
fn learned_addresses_reject_invalid_freshness_duplicate_and_unroutable_inputs() {
    // Arrange
    let mut book = LearnedAddressBook::default();
    let now_unix_seconds = 1_700_000_000;
    let first = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(8, 8, 4, 4, 8333),
    );
    let stale = address_announcement(
        now_unix_seconds - PHASE92_MAX_ADDR_AGE_SECONDS - 1,
        public_ipv4_network_address(8, 8, 8, 8, 8333),
    );
    let future = address_announcement(
        now_unix_seconds + PHASE92_MAX_FUTURE_SKEW_SECONDS + 1,
        public_ipv4_network_address(1, 1, 1, 1, 8333),
    );
    let invalid_port =
        address_announcement(now_unix_seconds, public_ipv4_network_address(8, 8, 8, 8, 0));
    let loopback = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(127, 0, 0, 1, 8333),
    );
    let private = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(10, 0, 0, 1, 8333),
    );
    let documentation = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(192, 0, 2, 1, 8333),
    );

    // Act
    let accepted = book.learn_batch(
        core::slice::from_ref(&first),
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    );
    let rejected = book.learn_batch(
        &[
            invalid_port,
            stale,
            future,
            first,
            loopback,
            private,
            documentation,
        ],
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    );

    // Assert
    assert_eq!(accepted.accepted_count, 1);
    assert_eq!(rejected.accepted_count, 0);
    assert_eq!(
        rejected
            .decisions
            .iter()
            .map(|decision| decision.label)
            .collect::<Vec<_>>(),
        vec![
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionLabel::LearnedRejected,
        ],
    );
    assert_eq!(
        rejected
            .decisions
            .iter()
            .map(|decision| decision.reason)
            .collect::<Vec<_>>(),
        vec![
            AddressDecisionReason::InvalidPort,
            AddressDecisionReason::StaleOrFuture,
            AddressDecisionReason::StaleOrFuture,
            AddressDecisionReason::DuplicateAddress,
            AddressDecisionReason::NotPubliclyRoutable,
            AddressDecisionReason::NotPubliclyRoutable,
            AddressDecisionReason::NotPubliclyRoutable,
        ],
    );
    assert!(
        rejected
            .decisions
            .iter()
            .all(|decision| decision.maybe_entry.is_none())
    );
    assert_eq!(book.entries().len(), 1);
}

#[test]
fn learned_address_batches_above_phase92_limit_are_rejected_without_partial_inserts() {
    // Arrange
    let mut book = LearnedAddressBook::default();
    let now_unix_seconds = 1_700_000_000;
    let announcements: Vec<_> = (0..=PHASE92_LEARNED_ADDR_BATCH_LIMIT)
        .map(|index| {
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(8, 8, 8, index as u8, 8333),
            )
        })
        .collect();

    // Act
    let batch = book.learn_batch(
        &announcements,
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    );

    // Assert
    assert_eq!(batch.label, AddressDecisionLabel::LearnedRejected);
    assert_eq!(batch.reason, AddressDecisionReason::OverCapBatch);
    assert_eq!(batch.accepted_count, 0);
    assert_eq!(batch.rejected_count, announcements.len());
    assert!(batch.decisions.is_empty());
    assert!(book.entries().is_empty());
}

#[test]
fn getaddr_response_combines_local_and_learned_candidates_under_phase92_limit() {
    // Arrange
    let now_unix_seconds = 1_700_000_000;
    let local_inputs = [
        LocalAdvertisementInput {
            listener_endpoint: listener_endpoint("8.8.8.8:8333"),
            maybe_bound_addr: None,
            services: ServiceFlags::NETWORK,
            allow_public: true,
        },
        LocalAdvertisementInput {
            listener_endpoint: listener_endpoint("1.1.1.1:8333"),
            maybe_bound_addr: None,
            services: ServiceFlags::NETWORK,
            allow_public: true,
        },
    ];
    let local_decisions = select_local_advertisement_candidates(&local_inputs);
    let mut book = LearnedAddressBook::default();
    let learned_announcements: Vec<_> = (1..=10)
        .map(|index| {
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(9, 9, 9, index, 8333),
            )
        })
        .collect();
    let learned_batch = book.learn_batch(
        &learned_announcements,
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    );
    let cache =
        AddressResponseCache::from_sources(&local_decisions, book.entries(), now_unix_seconds);
    let eligibility = GetAddrPeerEligibility::from_permission_effects(
        true,
        &[PermissionEffectLabel::AddressResponsePolicyInput],
    );
    let mut request_state = GetAddrRequestState::default();

    // Act
    let decision =
        select_getaddr_response(eligibility, &mut request_state, &cache, now_unix_seconds);

    // Assert
    assert_eq!(PHASE92_GETADDR_RESPONSE_LIMIT, 8);
    assert_eq!(learned_batch.accepted_count, 10);
    assert_eq!(cache.entries().len(), 12);
    let entries = served_response_entries(&decision);
    assert_eq!(entries.len(), PHASE92_GETADDR_RESPONSE_LIMIT);
    assert_eq!(
        entries.iter().map(|entry| entry.source).collect::<Vec<_>>(),
        vec![
            AddressSourceKind::LocalListener,
            AddressSourceKind::LocalListener,
            AddressSourceKind::InboundAddr,
            AddressSourceKind::InboundAddr,
            AddressSourceKind::InboundAddr,
            AddressSourceKind::InboundAddr,
            AddressSourceKind::InboundAddr,
            AddressSourceKind::InboundAddr,
        ],
    );
    assert!(request_state.served);
}

#[test]
fn getaddr_response_suppression_reasons_are_stable() {
    // Arrange
    let now_unix_seconds = 1_700_000_000;
    let fresh_cache = AddressResponseCache::from_entries(vec![response_entry(
        public_ipv4_network_address(8, 8, 4, 4, 8333),
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    )]);
    let empty_cache = AddressResponseCache::default();
    let stale_cache = AddressResponseCache::from_entries(vec![response_entry(
        public_ipv4_network_address(8, 8, 8, 8, 8333),
        AddressSourceKind::InboundAddr,
        now_unix_seconds - PHASE92_MAX_ADDR_AGE_SECONDS - 1,
    )]);
    let suppressed_local_decisions =
        select_local_advertisement_candidates(&[LocalAdvertisementInput {
            listener_endpoint: listener_endpoint("127.0.0.1:8333"),
            maybe_bound_addr: None,
            services: ServiceFlags::NETWORK,
            allow_public: true,
        }]);
    let suppressed_local_cache =
        AddressResponseCache::from_sources(&suppressed_local_decisions, &[], now_unix_seconds);
    let permitted_inbound = GetAddrPeerEligibility::from_permission_effects(
        true,
        &[PermissionEffectLabel::AddressResponsePolicyInput],
    );
    let outbound = GetAddrPeerEligibility::from_permission_effects(
        false,
        &[PermissionEffectLabel::AddressResponsePolicyInput],
    );
    let missing_policy_input = GetAddrPeerEligibility::from_permission_effects(true, &[]);

    // Act
    let outbound_decision = select_getaddr_response(
        outbound,
        &mut GetAddrRequestState::default(),
        &fresh_cache,
        now_unix_seconds,
    );
    let missing_policy_decision = select_getaddr_response(
        missing_policy_input,
        &mut GetAddrRequestState::default(),
        &fresh_cache,
        now_unix_seconds,
    );
    let empty_decision = select_getaddr_response(
        permitted_inbound,
        &mut GetAddrRequestState::default(),
        &empty_cache,
        now_unix_seconds,
    );
    let stale_decision = select_getaddr_response(
        permitted_inbound,
        &mut GetAddrRequestState::default(),
        &stale_cache,
        now_unix_seconds,
    );
    let suppressed_local_decision = select_getaddr_response(
        permitted_inbound,
        &mut GetAddrRequestState::default(),
        &suppressed_local_cache,
        now_unix_seconds,
    );
    let mut served_state = GetAddrRequestState { served: false };
    let first_decision = select_getaddr_response(
        permitted_inbound,
        &mut served_state,
        &fresh_cache,
        now_unix_seconds,
    );
    let second_decision = select_getaddr_response(
        permitted_inbound,
        &mut served_state,
        &fresh_cache,
        now_unix_seconds,
    );

    // Assert
    assert_suppressed_reason(&outbound_decision, AddressDecisionReason::NotInbound);
    assert_suppressed_reason(
        &missing_policy_decision,
        AddressDecisionReason::PermissionPolicyDenied,
    );
    assert_suppressed_reason(&empty_decision, AddressDecisionReason::EmptyResponseCache);
    assert_suppressed_reason(&stale_decision, AddressDecisionReason::EmptyResponseCache);
    assert_suppressed_reason(
        &suppressed_local_decision,
        AddressDecisionReason::EmptyResponseCache,
    );
    assert_eq!(served_response_entries(&first_decision).len(), 1);
    assert_suppressed_reason(&second_decision, AddressDecisionReason::AlreadyServed);
}

#[test]
fn getaddr_response_preserves_source_freshness_and_service_evidence() {
    // Arrange
    let now_unix_seconds = 1_700_000_000;
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    let learned_address = public_ipv4_network_address_with_services(8, 8, 8, 8, 18444, services);
    let mut book = LearnedAddressBook::default();
    let learned_batch = book.learn_batch(
        &[address_announcement(
            now_unix_seconds - 42,
            learned_address.clone(),
        )],
        AddressSourceKind::InboundAddr,
        now_unix_seconds,
    );
    let cache = AddressResponseCache::from_sources(&[], book.entries(), now_unix_seconds);
    let eligibility = GetAddrPeerEligibility::from_permission_effects(
        true,
        &[PermissionEffectLabel::AddressResponsePolicyInput],
    );
    let mut request_state = GetAddrRequestState::default();

    // Act
    let decision =
        select_getaddr_response(eligibility, &mut request_state, &cache, now_unix_seconds);

    // Assert
    assert_eq!(learned_batch.accepted_count, 1);
    let entry = served_response_entries(&decision)
        .first()
        .expect("fresh learned entry should be served");
    assert_eq!(entry.address, learned_address);
    assert_eq!(entry.source, AddressSourceKind::InboundAddr);
    assert_eq!(entry.first_seen_unix_seconds, now_unix_seconds - 42);
    assert_eq!(entry.last_seen_unix_seconds, now_unix_seconds - 42);
    assert_eq!(entry.services_bits, services.bits());
    assert_eq!(entry.port, 18444);
    assert_eq!(entry.routability, RoutabilityClass::PubliclyRoutable);
    assert!(entry.persistence_eligible);
}

fn listener_endpoint(raw_endpoint: &str) -> InboundListenerEndpoint {
    let address = socket_addr(raw_endpoint);
    InboundListenerEndpoint {
        raw: raw_endpoint.to_string(),
        normalized: address.to_string(),
        address,
    }
}

fn socket_addr(raw_endpoint: &str) -> SocketAddr {
    raw_endpoint
        .parse()
        .expect("test listener endpoint should parse")
}

fn ipv4_mapped_address_bytes(octets: [u8; 4]) -> [u8; 16] {
    let mut bytes = [0_u8; 16];
    bytes[10] = 0xff;
    bytes[11] = 0xff;
    bytes[12..].copy_from_slice(&octets);
    bytes
}

fn address_announcement(time_unix_seconds: u64, address: NetworkAddress) -> AddressAnnouncement {
    AddressAnnouncement {
        time_unix_seconds: u32::try_from(time_unix_seconds)
            .expect("test timestamp should fit in legacy addr timestamp"),
        address,
    }
}

fn public_ipv4_network_address(a: u8, b: u8, c: u8, d: u8, port: u16) -> NetworkAddress {
    public_ipv4_network_address_with_services(a, b, c, d, port, ServiceFlags::NETWORK)
}

fn public_ipv4_network_address_with_services(
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    port: u16,
    services: ServiceFlags,
) -> NetworkAddress {
    NetworkAddress {
        services: services.bits(),
        address_bytes: ipv4_mapped_address_bytes([a, b, c, d]),
        port,
    }
}

fn public_ipv6_network_address(raw_address: &str, port: u16) -> NetworkAddress {
    NetworkAddress {
        services: ServiceFlags::NETWORK.bits(),
        address_bytes: raw_address
            .parse::<Ipv6Addr>()
            .expect("test IPv6 address should parse")
            .octets(),
        port,
    }
}

fn response_entry(
    address: NetworkAddress,
    source: AddressSourceKind,
    last_seen_unix_seconds: u64,
) -> AddressResponseEntryEvidence {
    AddressResponseEntryEvidence {
        network_kind: AddressNetworkKind::Ipv4,
        source,
        first_seen_unix_seconds: last_seen_unix_seconds,
        last_seen_unix_seconds,
        services_bits: address.services,
        port: address.port,
        routability: RoutabilityClass::PubliclyRoutable,
        persistence_eligible: source == AddressSourceKind::InboundAddr,
        address,
    }
}

fn served_response_entries(decision: &GetAddrResponseDecision) -> &[AddressResponseEntryEvidence] {
    let GetAddrResponseDecision::Served {
        label,
        reason,
        entries,
    } = decision
    else {
        panic!("expected getaddr_served decision, got {decision:?}");
    };
    assert_eq!(*label, AddressDecisionLabel::GetAddrServed);
    assert_eq!(*reason, AddressDecisionReason::PolicyAccepted);
    entries
}

fn assert_suppressed_reason(
    decision: &GetAddrResponseDecision,
    expected_reason: AddressDecisionReason,
) {
    let GetAddrResponseDecision::Suppressed { label, reason } = decision else {
        panic!("expected getaddr_suppressed decision, got {decision:?}");
    };
    assert_eq!(*label, AddressDecisionLabel::GetAddrSuppressed);
    assert_eq!(*reason, expected_reason);
}
