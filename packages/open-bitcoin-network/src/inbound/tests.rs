// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use std::collections::BTreeSet;

use crate::PeerId;

use super::{
    InboundAdmissionCounters, InboundAdmissionDecision, InboundAdmissionPolicy,
    InboundAdmissionRejectionReason, InboundAdmissionRequest, InboundAdmissionSlotClass,
    InboundListenerActivationDiagnostic, InboundListenerConfig, InboundPreflightReason,
    classify_inbound_preflight,
};

fn enabled_config(addresses: Vec<&str>) -> InboundListenerConfig {
    InboundListenerConfig {
        enabled: true,
        listen_addresses: addresses.into_iter().map(str::to_string).collect(),
        max_peers: 8,
        reserved_slots: 2,
        allow_public: false,
    }
}

fn admission_request(
    peer_id: PeerId,
    remote_endpoint: &str,
    slot_class: InboundAdmissionSlotClass,
    counters: InboundAdmissionCounters,
) -> InboundAdmissionRequest {
    InboundAdmissionRequest {
        peer_id,
        remote_endpoint: remote_endpoint.to_string(),
        slot_class,
        counters,
        existing_endpoint_keys: BTreeSet::new(),
        existing_peer_ids: BTreeSet::new(),
        local_nonce: 99,
        maybe_remote_nonce: Some(101),
        is_shutdown_requested: false,
    }
}

#[test]
fn preflight_reason_labels_are_stable() {
    // Arrange
    let reasons = [
        (InboundPreflightReason::Disabled, "disabled"),
        (
            InboundPreflightReason::NoListenAddresses,
            "no_listen_addresses",
        ),
        (InboundPreflightReason::InvalidEndpoint, "invalid_endpoint"),
        (InboundPreflightReason::UnsafeEndpoint, "unsafe_endpoint"),
        (InboundPreflightReason::BindUnavailable, "bind_unavailable"),
        (InboundPreflightReason::AlreadyBound, "already_bound"),
        (InboundPreflightReason::Ready, "ready"),
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
            "disabled",
            "no_listen_addresses",
            "invalid_endpoint",
            "unsafe_endpoint",
            "bind_unavailable",
            "already_bound",
            "ready",
        ],
    );
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
fn disabled_preflight_does_not_attempt_bind() {
    // Arrange
    let config = InboundListenerConfig::default();

    // Act
    let plan = classify_inbound_preflight(&config);

    // Assert
    assert_eq!(plan.reason(), InboundPreflightReason::Disabled);
    assert!(!plan.should_attempt_bind());
    assert!(plan.ready_endpoints().is_empty());
    assert_eq!(plan.diagnostics()[0].maybe_endpoint, None);
}

#[test]
fn enabled_preflight_requires_listen_addresses() {
    // Arrange
    let config = enabled_config(Vec::new());

    // Act
    let plan = classify_inbound_preflight(&config);

    // Assert
    let diagnostic = &plan.diagnostics()[0];
    assert_eq!(diagnostic.reason, InboundPreflightReason::NoListenAddresses);
    assert_eq!(diagnostic.field, "inbound.listen_addresses");
    assert!(!plan.should_attempt_bind());
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
fn loopback_preflight_returns_ready_normalized_endpoints() {
    // Arrange
    let config = enabled_config(vec!["127.0.0.1:18444", "[::1]:18444"]);

    // Act
    let plan = classify_inbound_preflight(&config);

    // Assert
    assert_eq!(plan.reason(), InboundPreflightReason::Ready);
    assert!(plan.should_attempt_bind());
    assert_eq!(plan.ready_endpoints()[0].normalized, "127.0.0.1:18444");
    assert_eq!(plan.ready_endpoints()[1].normalized, "[::1]:18444");
}

#[test]
fn activation_diagnostics_represent_os_observed_bind_results() {
    // Arrange
    let config = enabled_config(vec!["127.0.0.1:18444"]);
    let plan = classify_inbound_preflight(&config);
    let endpoint = &plan.ready_endpoints()[0];

    // Act
    let bind_unavailable =
        InboundListenerActivationDiagnostic::bind_unavailable(endpoint, "address unavailable");
    let already_bound =
        InboundListenerActivationDiagnostic::already_bound(endpoint, "address in use");

    // Assert
    assert_eq!(
        bind_unavailable.reason,
        InboundPreflightReason::BindUnavailable,
    );
    assert_eq!(already_bound.reason, InboundPreflightReason::AlreadyBound);
    assert_eq!(
        already_bound.maybe_endpoint.as_deref(),
        Some("127.0.0.1:18444"),
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
    assert_eq!(rejection.reason, InboundAdmissionRejectionReason::CapReached);
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

#[test]
fn shutdown_request_rejects_admission() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(4, 1);
    let counters = InboundAdmissionCounters::default();
    let mut request = admission_request(
        15,
        "127.0.0.1:20015",
        InboundAdmissionSlotClass::Ordinary,
        counters,
    );
    request.is_shutdown_requested = true;

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Reject(rejection) = decision else {
        panic!("expected shutdown rejection");
    };
    assert_eq!(rejection.reason, InboundAdmissionRejectionReason::Shutdown);
}
