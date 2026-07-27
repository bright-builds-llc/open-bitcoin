// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use super::*;

#[test]
fn legacy_inactive_relay_like_effect_labels_remain_stable() {
    // Arrange
    let labels = [
        InactivePermissionEffectLabel::Relay,
        InactivePermissionEffectLabel::ForceRelay,
        InactivePermissionEffectLabel::Mempool,
    ];

    // Act
    let serialized: Vec<&str> = labels.into_iter().map(|label| label.as_str()).collect();

    // Assert
    assert_eq!(
        serialized,
        vec!["inactive_relay", "inactive_forcerelay", "inactive_mempool"],
    );
}

#[test]
fn unmatched_inbound_peer_stays_ordinary_and_never_uses_reserved_capacity() {
    // Arrange
    let class = match ParsedPeerPermissionClass::parse(
        "download-address",
        ["203.0.113.9"],
        ["in", "download", "addr"],
    ) {
        Ok(class) => class,
        Err(error) => panic!("expected permissioned class to parse: {error:?}"),
    };
    let registry = PeerPermissionClassRegistry::new([class]);

    // Act
    let decision = registry.resolve_inbound(test_ip("203.0.113.10"));

    // Assert
    assert_eq!(
        decision.connection_class(),
        PeerConnectionClass::OrdinaryInbound
    );
    assert_eq!(decision.connection_class().as_str(), "ordinary_inbound");
    assert_eq!(decision.slot_class(), InboundAdmissionSlotClass::Ordinary);
    assert!(decision.active_effects().is_empty());
    assert!(decision.inactive_effects().is_empty());
}

#[test]
fn admission_rejection_labels_are_stable() {
    // Arrange
    let reasons = [
        (InboundAdmissionRejectionReason::CapReached, "cap_reached"),
        (
            InboundAdmissionRejectionReason::ReservedSlotUnavailable,
            "reserved_slot_unavailable",
        ),
        (
            InboundAdmissionRejectionReason::DuplicateEndpoint,
            "duplicate_endpoint",
        ),
        (
            InboundAdmissionRejectionReason::DuplicatePeerId,
            "duplicate_peer_id",
        ),
        (
            InboundAdmissionRejectionReason::SelfConnection,
            "self_connection",
        ),
        (InboundAdmissionRejectionReason::Shutdown, "shutdown"),
    ];

    // Act
    let labels: Vec<&str> = reasons
        .into_iter()
        .map(|(reason, _label)| reason.as_str())
        .collect();

    // Assert
    assert_eq!(
        labels,
        vec![
            "cap_reached",
            "reserved_slot_unavailable",
            "duplicate_endpoint",
            "duplicate_peer_id",
            "self_connection",
            "shutdown",
        ],
    );
}

#[test]
fn malformed_endpoint_returns_field_specific_invalid_endpoint() {
    // Arrange
    let config = enabled_config(vec!["not-a-socket"]);

    // Act
    let plan = classify_inbound_preflight(&config);

    // Assert
    let diagnostic = &plan.diagnostics()[0];
    assert_eq!(diagnostic.reason, InboundPreflightReason::InvalidEndpoint);
    assert_eq!(diagnostic.maybe_endpoint.as_deref(), Some("not-a-socket"));
    assert_eq!(diagnostic.field, "inbound.listen_addresses");
    assert!(diagnostic.next_action.contains("host:port"));
}

#[test]
fn wildcard_or_non_loopback_endpoint_requires_public_acknowledgement() {
    // Arrange
    let wildcard = enabled_config(vec!["0.0.0.0:8333"]);
    let public = enabled_config(vec!["192.0.2.10:8333"]);

    // Act
    let wildcard_plan = classify_inbound_preflight(&wildcard);
    let public_plan = classify_inbound_preflight(&public);

    // Assert
    assert_eq!(
        wildcard_plan.diagnostics()[0].reason,
        InboundPreflightReason::UnsafeEndpoint,
    );
    assert_eq!(
        public_plan.diagnostics()[0].reason,
        InboundPreflightReason::UnsafeEndpoint,
    );
    assert!(!wildcard_plan.should_attempt_bind());
    assert!(!public_plan.should_attempt_bind());
}

#[test]
fn allow_public_accepts_non_loopback_endpoint() {
    // Arrange
    let mut config = enabled_config(vec!["192.0.2.10:8333"]);
    config.allow_public = true;

    // Act
    let plan = classify_inbound_preflight(&config);

    // Assert
    assert_eq!(plan.reason(), InboundPreflightReason::Ready);
    assert!(plan.should_attempt_bind());
    assert_eq!(plan.ready_endpoints()[0].normalized, "192.0.2.10:8333");
}

#[test]
fn inbound_small_helpers_cover_status_and_counter_branches() {
    // Arrange
    let counters = InboundAdmissionCounters {
        current_inbound_peers: 2,
        current_outbound_peers: 5,
        current_reserved_inbound_peers: 1,
    };

    // Act
    let ordinary = counters.after_admitted(InboundAdmissionSlotClass::Ordinary);
    let reserved = counters.after_admitted(InboundAdmissionSlotClass::Reserved);
    let admitted = InboundAdmissionDecision::Admit(super::super::InboundPeerRecord {
        peer_id: 21,
        remote_endpoint: "127.0.0.1:20021".to_string(),
        slot_class: InboundAdmissionSlotClass::Ordinary,
        connection_class: PeerConnectionClass::OrdinaryInbound,
        permission_decision: InboundPermissionDecision::ordinary(),
        handshake_state: InboundHandshakeState::Accepted,
        maybe_remote_nonce: None,
        observed_inbound_peers: 2,
        observed_outbound_peers: 5,
    });
    let rejected = InboundAdmissionDecision::Reject(super::super::InboundAdmissionRejection {
        reason: InboundAdmissionRejectionReason::Shutdown,
        peer_id: 22,
        slot_class: InboundAdmissionSlotClass::Reserved,
        maybe_endpoint: None,
        message: "shutdown requested".to_string(),
        next_action: "retry after shutdown completes".to_string(),
    });

    // Assert
    assert_eq!(InboundHandshakeState::Accepted.as_str(), "accepted");
    assert_eq!(InboundHandshakeState::Handshaking.as_str(), "handshaking");
    assert_eq!(InboundHandshakeState::Established.as_str(), "established");
    assert_eq!(InboundHandshakeState::Disconnected.as_str(), "disconnected");
    assert_eq!(ordinary.current_inbound_peers, 3);
    assert_eq!(ordinary.current_outbound_peers, 5);
    assert_eq!(ordinary.current_reserved_inbound_peers, 1);
    assert_eq!(reserved.current_inbound_peers, 3);
    assert_eq!(reserved.current_reserved_inbound_peers, 2);
    assert!(admitted.is_admitted());
    assert!(!rejected.is_admitted());
    assert_eq!(
        InboundAdmissionPolicy::new(2, 5).effective_reserved_slots(),
        2,
    );
}

#[test]
fn admission_identity_helpers_project_existing_record_fields() {
    // Arrange
    let mut identities = BTreeSet::new();
    identities.insert(31);
    identities.insert(32);
    let mut request = InboundAdmissionRequest::ordinary(30, "127.0.0.1:20030");
    let record = super::super::InboundPeerRecord {
        peer_id: 31,
        remote_endpoint: "127.0.0.1:20031".to_string(),
        slot_class: InboundAdmissionSlotClass::Reserved,
        connection_class: PeerConnectionClass::ProtectedInbound,
        permission_decision: protected_permission_decision(),
        handshake_state: InboundHandshakeState::Accepted,
        maybe_remote_nonce: None,
        observed_inbound_peers: 2,
        observed_outbound_peers: 1,
    };

    // Act
    request.set_existing_identities(identities.clone());
    let self_connection = super::super::InboundAdmissionRejection::runtime_self_connection(&record);
    let duplicate_identity = super::super::InboundAdmissionRejection::duplicate_identity(&record);

    // Assert
    assert_eq!(request.existing_peer_ids, identities);
    assert_eq!(record.identity(), 31);
    assert_eq!(
        self_connection.reason,
        InboundAdmissionRejectionReason::SelfConnection
    );
    assert_eq!(self_connection.peer_id, 31);
    assert_eq!(
        self_connection.slot_class,
        InboundAdmissionSlotClass::Reserved
    );
    assert_eq!(
        self_connection.maybe_endpoint.as_deref(),
        Some("127.0.0.1:20031")
    );
    assert_eq!(
        duplicate_identity.reason,
        InboundAdmissionRejectionReason::DuplicatePeerId
    );
    assert_eq!(duplicate_identity.peer_id, 31);
    assert_eq!(
        duplicate_identity.next_action,
        "allocate a fresh peer id before retrying admission"
    );
}

#[test]
fn admission_rejects_cap_reached_without_using_outbound_count() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(2, 0);
    let counters = InboundAdmissionCounters {
        current_inbound_peers: 2,
        current_outbound_peers: 99,
        current_reserved_inbound_peers: 0,
    };
    let request = admission_request(
        7,
        "127.0.0.1:20007",
        InboundAdmissionSlotClass::Ordinary,
        counters,
    );

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Reject(rejection) = decision else {
        panic!("expected cap rejection");
    };
    assert_eq!(
        rejection.reason,
        InboundAdmissionRejectionReason::CapReached
    );
    assert_eq!(counters.current_inbound_peers, 2);
    assert_eq!(counters.current_outbound_peers, 99);
}

#[test]
fn ordinary_peer_cannot_consume_reserved_slot() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(3, 1);
    let counters = InboundAdmissionCounters {
        current_inbound_peers: 2,
        current_outbound_peers: 0,
        current_reserved_inbound_peers: 0,
    };
    let request = admission_request(
        8,
        "127.0.0.1:20008",
        InboundAdmissionSlotClass::Ordinary,
        counters,
    );

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Reject(rejection) = decision else {
        panic!("expected reserved slot rejection");
    };
    assert_eq!(
        rejection.reason,
        InboundAdmissionRejectionReason::ReservedSlotUnavailable,
    );
}

#[test]
fn reserved_candidate_can_use_available_reserved_capacity() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(3, 1);
    let counters = InboundAdmissionCounters {
        current_inbound_peers: 2,
        current_outbound_peers: 0,
        current_reserved_inbound_peers: 0,
    };
    let request = admission_request(
        9,
        "127.0.0.1:20009",
        InboundAdmissionSlotClass::Reserved,
        counters,
    );

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Admit(record) = decision else {
        panic!("expected reserved candidate admission");
    };
    assert_eq!(record.peer_id, 9);
    assert_eq!(record.slot_class, InboundAdmissionSlotClass::Reserved);
}

#[test]
fn reserved_candidate_rejects_when_reserved_pool_is_full() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(3, 1);
    let counters = InboundAdmissionCounters {
        current_inbound_peers: 3,
        current_outbound_peers: 0,
        current_reserved_inbound_peers: 1,
    };
    let request = admission_request(
        10,
        "127.0.0.1:20010",
        InboundAdmissionSlotClass::Reserved,
        counters,
    );

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Reject(rejection) = decision else {
        panic!("expected reserved pool rejection");
    };
    assert_eq!(
        rejection.reason,
        InboundAdmissionRejectionReason::ReservedSlotUnavailable,
    );
}

#[test]
fn duplicate_endpoint_rejects_before_admission() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(4, 1);
    let counters = InboundAdmissionCounters::default();
    let mut request = admission_request(
        11,
        "127.0.0.1:20011",
        InboundAdmissionSlotClass::Ordinary,
        counters,
    );
    request
        .existing_endpoint_keys
        .insert("127.0.0.1:20011".to_string());

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Reject(rejection) = decision else {
        panic!("expected duplicate endpoint rejection");
    };
    assert_eq!(
        rejection.reason,
        InboundAdmissionRejectionReason::DuplicateEndpoint,
    );
}

#[test]
fn duplicate_peer_id_rejects_before_admission() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(4, 1);
    let counters = InboundAdmissionCounters::default();
    let mut request = admission_request(
        12,
        "127.0.0.1:20012",
        InboundAdmissionSlotClass::Ordinary,
        counters,
    );
    request.existing_peer_ids.insert(12);

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Reject(rejection) = decision else {
        panic!("expected duplicate peer id rejection");
    };
    assert_eq!(
        rejection.reason,
        InboundAdmissionRejectionReason::DuplicatePeerId,
    );
}

#[test]
fn matching_remote_nonce_rejects_self_connection() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(4, 1);
    let counters = InboundAdmissionCounters::default();
    let mut request = admission_request(
        13,
        "127.0.0.1:20013",
        InboundAdmissionSlotClass::Ordinary,
        counters,
    );
    request.maybe_remote_nonce = Some(99);

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Reject(rejection) = decision else {
        panic!("expected self connection rejection");
    };
    assert_eq!(
        rejection.reason,
        InboundAdmissionRejectionReason::SelfConnection,
    );
}

#[test]
fn outbound_count_does_not_block_inbound_admission() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(1, 0);
    let counters = InboundAdmissionCounters {
        current_inbound_peers: 0,
        current_outbound_peers: 100,
        current_reserved_inbound_peers: 0,
    };
    let request = admission_request(
        14,
        "127.0.0.1:20014",
        InboundAdmissionSlotClass::Ordinary,
        counters,
    );

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Admit(record) = decision else {
        panic!("expected inbound admission despite outbound count");
    };
    assert_eq!(record.observed_outbound_peers, 100);
}
