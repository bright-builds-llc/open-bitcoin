// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use core::net::IpAddr;
use std::collections::BTreeSet;

use crate::PeerId;

use super::{
    InboundAdmissionCounters, InboundAdmissionDecision, InboundAdmissionPolicy,
    InboundAdmissionRejectionReason, InboundAdmissionRequest, InboundAdmissionSlotClass,
    InboundHandshakeState, InboundListenerActivationDiagnostic, InboundListenerConfig,
    InboundPermissionDecision, InboundPreflightReason, ParsedPeerPermissionClass,
    PeerConnectionClass, PeerPermissionClassRegistry, PeerPermissionDirection, PeerPermissionSet,
    PeerPermissionToken, classify_inbound_preflight,
};

fn enabled_config(addresses: Vec<&str>) -> InboundListenerConfig {
    InboundListenerConfig {
        enabled: true,
        listen_addresses: addresses.into_iter().map(str::to_string).collect(),
        max_peers: 8,
        reserved_slots: 2,
        allow_public: false,
        permission_classes: PeerPermissionClassRegistry::default(),
    }
}

fn admission_request(
    peer_id: PeerId,
    remote_endpoint: &str,
    slot_class: InboundAdmissionSlotClass,
    counters: InboundAdmissionCounters,
) -> InboundAdmissionRequest {
    let permission_decision = match slot_class {
        InboundAdmissionSlotClass::Ordinary => InboundPermissionDecision::ordinary(),
        InboundAdmissionSlotClass::Reserved => protected_permission_decision(),
    };
    let mut request = InboundAdmissionRequest::from_permission_decision(
        peer_id,
        remote_endpoint,
        permission_decision,
    );
    request.counters = counters;
    request.existing_endpoint_keys = BTreeSet::new();
    request.existing_peer_ids = BTreeSet::new();
    request.local_nonce = 99;
    request.maybe_remote_nonce = Some(101);
    request
}

fn test_ip(raw: &str) -> IpAddr {
    match raw.parse() {
        Ok(address) => address,
        Err(error) => panic!("test IP address should parse: {error}"),
    }
}

fn permission_decision(permissions: &[&str]) -> InboundPermissionDecision {
    let class = match ParsedPeerPermissionClass::parse("test-class", ["203.0.113.7"], permissions) {
        Ok(class) => class,
        Err(error) => panic!("expected test permission class to parse: {error:?}"),
    };
    PeerPermissionClassRegistry::new([class]).resolve_inbound(test_ip("203.0.113.7"))
}

fn protected_permission_decision() -> InboundPermissionDecision {
    permission_decision(&["in", "noban", "forceinbound"])
}

#[test]
fn permission_tokens_accept_exact_knots_anchored_vocabulary() {
    // Arrange
    let tokens = [
        "bloomfilter",
        "blockfilters",
        "noban",
        "forcerelay",
        "relay",
        "mempool",
        "download",
        "addr",
        "forceinbound",
        "in",
        "out",
        "all",
    ];

    // Act
    let parsed = PeerPermissionSet::parse("inbound.permission_classes[].permissions[]", tokens);

    // Assert
    let Ok(set) = parsed else {
        panic!("expected exact permission vocabulary to parse");
    };
    assert!(set.contains_token(PeerPermissionToken::BloomFilter));
    assert!(set.contains_token(PeerPermissionToken::BlockFilters));
    assert!(set.contains_token(PeerPermissionToken::NoBan));
    assert!(set.contains_token(PeerPermissionToken::ForceRelay));
    assert!(set.contains_token(PeerPermissionToken::Relay));
    assert!(set.contains_token(PeerPermissionToken::Mempool));
    assert!(set.contains_token(PeerPermissionToken::Download));
    assert!(set.contains_token(PeerPermissionToken::Addr));
    assert!(set.contains_token(PeerPermissionToken::ForceInbound));
    assert!(set.has_direction(PeerPermissionDirection::Inbound));
    assert!(set.has_direction(PeerPermissionDirection::Outbound));
}

#[test]
fn permission_tokens_reject_unsupported_knots_aliases_with_field_and_token() {
    // Arrange
    let aliases = ["bloom", "compactfilters", "cfilters"];

    for alias in aliases {
        // Act
        let parsed =
            PeerPermissionSet::parse("inbound.permission_classes[].permissions[]", [alias]);

        // Assert
        let Err(error) = parsed else {
            panic!("expected unsupported alias rejection for {alias}");
        };
        assert_eq!(error.field(), "inbound.permission_classes[].permissions[]");
        assert_eq!(error.token(), alias);
        assert_eq!(error.reason(), "unsupported_token");
    }
}

#[test]
fn all_permission_keeps_bounded_effects_active_and_relay_like_effects_inactive() {
    // Arrange
    let set = match PeerPermissionSet::parse("inbound.permission_classes[].permissions[]", ["all"])
    {
        Ok(set) => set,
        Err(error) => panic!("expected all to parse: {error:?}"),
    };

    // Act
    let active_labels: Vec<&str> = set
        .active_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect();
    let inactive_labels: Vec<&str> = set
        .inactive_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect();

    // Assert
    assert_eq!(
        active_labels,
        vec![
            "admission_protected",
            "eviction_policy_protected",
            "misbehavior_policy_protected",
            "address_response_policy_input",
            "download_serving_policy_input",
        ],
    );
    assert_eq!(
        inactive_labels,
        vec![
            "inactive_relay",
            "inactive_forcerelay",
            "inactive_mempool",
            "inactive_bloomfilter",
            "inactive_blockfilters",
        ],
    );
    assert!(set.contains_token(PeerPermissionToken::All));
}

#[test]
fn permission_tokens_and_directions_use_knots_labels() {
    // Arrange
    let tokens = [
        (PeerPermissionToken::BloomFilter, "bloomfilter"),
        (PeerPermissionToken::BlockFilters, "blockfilters"),
        (PeerPermissionToken::NoBan, "noban"),
        (PeerPermissionToken::ForceRelay, "forcerelay"),
        (PeerPermissionToken::Relay, "relay"),
        (PeerPermissionToken::Mempool, "mempool"),
        (PeerPermissionToken::Download, "download"),
        (PeerPermissionToken::Addr, "addr"),
        (PeerPermissionToken::ForceInbound, "forceinbound"),
        (PeerPermissionToken::All, "all"),
    ];
    let directions = [
        (PeerPermissionDirection::Inbound, "in"),
        (PeerPermissionDirection::Outbound, "out"),
    ];

    // Act
    let token_labels: Vec<&str> = tokens
        .into_iter()
        .map(|(token, _label)| token.as_str())
        .collect();
    let direction_labels: Vec<&str> = directions
        .into_iter()
        .map(|(direction, _label)| direction.as_str())
        .collect();

    // Assert
    assert_eq!(
        token_labels,
        vec![
            "bloomfilter",
            "blockfilters",
            "noban",
            "forcerelay",
            "relay",
            "mempool",
            "download",
            "addr",
            "forceinbound",
            "all",
        ],
    );
    assert_eq!(direction_labels, vec!["in", "out"]);
}

#[test]
fn permission_class_accessors_preserve_sanitized_domain_values() {
    // Arrange
    let parsed = ParsedPeerPermissionClass::parse(
        "  trusted-download  ",
        ["203.0.113.20"],
        ["in", "download"],
    );

    // Act
    let class = match parsed {
        Ok(class) => class,
        Err(error) => panic!("expected permission class to parse: {error:?}"),
    };

    // Assert
    assert_eq!(class.name().as_str(), "trusted-download");
    assert_eq!(class.addresses(), &[test_ip("203.0.113.20")]);
    assert!(
        class
            .permissions()
            .contains_token(PeerPermissionToken::Download)
    );
}

#[test]
fn permission_class_rejects_empty_name_and_empty_address_list() {
    // Arrange
    let empty_name = "";
    let empty_addresses: [&str; 0] = [];

    // Act
    let name_result =
        ParsedPeerPermissionClass::parse(empty_name, ["203.0.113.21"], ["in", "download"]);
    let address_result =
        ParsedPeerPermissionClass::parse("missing-address", empty_addresses, ["in", "download"]);

    // Assert
    let Err(name_error) = name_result else {
        panic!("expected empty class name rejection");
    };
    assert_eq!(name_error.field(), "inbound.permission_classes[].name");
    assert_eq!(name_error.reason(), "empty_class_name");
    let Err(address_error) = address_result else {
        panic!("expected empty address list rejection");
    };
    assert_eq!(
        address_error.field(),
        "inbound.permission_classes[].addresses[]",
    );
    assert_eq!(address_error.reason(), "empty_address_list");
}

#[test]
fn permission_parse_error_exposes_operator_safe_message_and_display_text() {
    // Arrange
    let invalid_address = "peer.example";

    // Act
    let parsed =
        ParsedPeerPermissionClass::parse("bad-address", [invalid_address], ["in", "download"]);

    // Assert
    let Err(error) = parsed else {
        panic!("expected invalid address rejection");
    };
    assert_eq!(
        error.message(),
        "peer permission class address must be a literal IP address: peer.example",
    );
    assert_eq!(
        format!("{error}"),
        "inbound.permission_classes[].addresses[] contains invalid token peer.example: peer permission class address must be a literal IP address: peer.example",
    );
}

#[test]
fn protected_literal_ip_class_uses_reserved_admission_capacity() {
    // Arrange
    let class = match ParsedPeerPermissionClass::parse(
        "trusted-protected",
        ["203.0.113.7"],
        ["in", "noban", "forceinbound"],
    ) {
        Ok(class) => class,
        Err(error) => panic!("expected protected class to parse: {error:?}"),
    };
    let registry = PeerPermissionClassRegistry::new([class]);

    // Act
    let decision = registry.resolve_inbound(test_ip("203.0.113.7"));

    // Assert
    assert_eq!(
        decision.connection_class(),
        PeerConnectionClass::ProtectedInbound
    );
    assert_eq!(decision.connection_class().as_str(), "protected_inbound");
    assert_eq!(decision.slot_class(), InboundAdmissionSlotClass::Reserved);
    assert!(
        decision
            .active_effects()
            .iter()
            .any(|effect| effect.as_str() == "admission_protected")
    );
}

#[test]
fn permissioned_literal_ip_class_keeps_ordinary_admission_slot() {
    // Arrange
    let class = match ParsedPeerPermissionClass::parse(
        "download-address",
        ["203.0.113.8"],
        ["in", "download", "addr"],
    ) {
        Ok(class) => class,
        Err(error) => panic!("expected permissioned class to parse: {error:?}"),
    };
    let registry = PeerPermissionClassRegistry::new([class]);

    // Act
    let decision = registry.resolve_inbound(test_ip("203.0.113.8"));
    let active_labels: Vec<&str> = decision
        .active_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect();

    // Assert
    assert_eq!(
        decision.connection_class(),
        PeerConnectionClass::PermissionedInbound,
    );
    assert_eq!(decision.connection_class().as_str(), "permissioned_inbound");
    assert_eq!(decision.slot_class(), InboundAdmissionSlotClass::Ordinary);
    assert_eq!(
        active_labels,
        vec![
            "address_response_policy_input",
            "download_serving_policy_input",
        ],
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
fn ordinary_admission_request_defaults_to_empty_permission_evidence() {
    // Arrange
    let request = InboundAdmissionRequest::ordinary(5, "127.0.0.1:20005");

    // Act
    let active_effects = request.permission_decision.active_effects();
    let inactive_effects = request.permission_decision.inactive_effects();

    // Assert
    assert_eq!(
        request.connection_class,
        PeerConnectionClass::OrdinaryInbound
    );
    assert_eq!(request.slot_class, InboundAdmissionSlotClass::Ordinary);
    assert_eq!(
        request.effective_slot_class(),
        InboundAdmissionSlotClass::Ordinary,
    );
    assert!(active_effects.is_empty());
    assert!(inactive_effects.is_empty());
}

#[test]
fn admission_request_rebinds_slot_class_when_permission_decision_changes() {
    // Arrange
    let mut request = InboundAdmissionRequest::ordinary(17, "127.0.0.1:20017");

    // Act
    request.set_permission_decision(protected_permission_decision());

    // Assert
    assert_eq!(
        request.connection_class,
        PeerConnectionClass::ProtectedInbound,
    );
    assert_eq!(request.slot_class, InboundAdmissionSlotClass::Reserved);
    assert_eq!(
        request.effective_slot_class(),
        InboundAdmissionSlotClass::Reserved,
    );
}

#[test]
fn permissioned_admission_record_preserves_active_and_inactive_effect_evidence() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(3, 1);
    let permission_decision = permission_decision(&["in", "download", "addr", "relay", "mempool"]);
    let mut request = InboundAdmissionRequest::from_permission_decision(
        6,
        "127.0.0.1:20006",
        permission_decision,
    );
    request.local_nonce = 99;
    request.maybe_remote_nonce = Some(101);

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Admit(record) = decision else {
        panic!("expected permissioned inbound admission");
    };
    let active_labels: Vec<&str> = record
        .permission_decision
        .active_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect();
    let inactive_labels: Vec<&str> = record
        .permission_decision
        .inactive_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect();
    assert_eq!(
        record.connection_class,
        PeerConnectionClass::PermissionedInbound,
    );
    assert_eq!(record.slot_class, InboundAdmissionSlotClass::Ordinary);
    assert_eq!(
        active_labels,
        vec![
            "address_response_policy_input",
            "download_serving_policy_input",
        ],
    );
    assert_eq!(inactive_labels, vec!["inactive_relay", "inactive_mempool"]);
}

#[test]
fn protected_admission_record_preserves_reserved_slot_and_permission_evidence() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(3, 1);
    let mut request = InboundAdmissionRequest::from_permission_decision(
        16,
        "127.0.0.1:20016",
        protected_permission_decision(),
    );
    request.counters = InboundAdmissionCounters {
        current_inbound_peers: 2,
        current_outbound_peers: 4,
        current_reserved_inbound_peers: 0,
    };
    request.local_nonce = 99;
    request.maybe_remote_nonce = Some(101);

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Admit(record) = decision else {
        panic!("expected protected inbound admission");
    };
    assert_eq!(
        record.connection_class,
        PeerConnectionClass::ProtectedInbound
    );
    assert_eq!(record.slot_class, InboundAdmissionSlotClass::Reserved);
    assert_eq!(record.observed_outbound_peers, 4);
    assert!(
        record
            .permission_decision
            .active_effects()
            .iter()
            .any(|effect| effect.as_str() == "admission_protected")
    );
}

#[test]
fn class_definitions_reject_direction_only_missing_in_and_outbound_rules() {
    // Arrange
    let cases = [
        (["in"].as_slice(), "direction_only", "in"),
        (
            ["download", "addr"].as_slice(),
            "missing_inbound_direction",
            "in",
        ),
        (
            ["in", "out", "download"].as_slice(),
            "outbound_direction_unsupported",
            "out",
        ),
    ];

    for (tokens, reason, token) in cases {
        // Act
        let parsed = ParsedPeerPermissionClass::parse("bad-class", ["203.0.113.11"], tokens);

        // Assert
        let Err(error) = parsed else {
            panic!("expected invalid class definition rejection");
        };
        assert_eq!(error.field(), "inbound.permission_classes[].permissions[]");
        assert_eq!(error.reason(), reason);
        assert_eq!(error.token(), token);
    }
}

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
fn activation_diagnostic_converts_to_preflight_shape() {
    // Arrange
    let config = enabled_config(vec!["127.0.0.1:18444"]);
    let plan = classify_inbound_preflight(&config);
    let endpoint = &plan.ready_endpoints()[0];
    let activation = InboundListenerActivationDiagnostic::already_bound(endpoint, "address in use");

    // Act
    let diagnostic = activation.into_preflight_diagnostic();

    // Assert
    assert_eq!(diagnostic.reason, InboundPreflightReason::AlreadyBound);
    assert_eq!(
        diagnostic.maybe_endpoint.as_deref(),
        Some("127.0.0.1:18444"),
    );
    assert_eq!(diagnostic.field, "inbound.listen_addresses");
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
    let admitted = InboundAdmissionDecision::Admit(super::InboundPeerRecord {
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
    let rejected = InboundAdmissionDecision::Reject(super::InboundAdmissionRejection {
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
    let record = super::InboundPeerRecord {
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
    let self_connection = super::InboundAdmissionRejection::runtime_self_connection(&record);
    let duplicate_identity = super::InboundAdmissionRejection::duplicate_identity(&record);

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
