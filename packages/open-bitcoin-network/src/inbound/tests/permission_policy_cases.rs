// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use super::*;

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
fn all_permission_emits_scoped_relay_policy_effects_and_keeps_filters_inactive() {
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
    let relay_labels: Vec<&str> = set
        .relay_permission_effects()
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
        vec!["inactive_bloomfilter", "inactive_blockfilters"],
    );
    assert_eq!(
        relay_labels,
        vec![
            "transaction_relay_policy_input",
            "force_relay_policy_input",
            "mempool_policy_input",
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
    assert!(class
        .permissions()
        .contains_token(PeerPermissionToken::Download));
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
    assert!(decision
        .active_effects()
        .iter()
        .any(|effect| effect.as_str() == "admission_protected"));
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
fn permissioned_admission_record_preserves_scoped_relay_policy_effect_evidence() {
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
    let relay_labels = relay_permission_labels(&record.permission_decision);
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
    assert_eq!(
        relay_labels,
        vec!["transaction_relay_policy_input", "mempool_policy_input",],
    );
    assert!(inactive_labels.is_empty());
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
    assert!(record
        .permission_decision
        .active_effects()
        .iter()
        .any(|effect| effect.as_str() == "admission_protected"));
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
