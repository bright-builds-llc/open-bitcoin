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
