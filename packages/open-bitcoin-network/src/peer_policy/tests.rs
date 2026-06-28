// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/banman.h
// - packages/bitcoin-knots/src/banman.cpp
// - packages/bitcoin-knots/src/net_permissions.cpp

use core::net::IpAddr;

use crate::inbound::PermissionEffectLabel;

use super::{
    BanDecision, BanReason, BanScope, EvictionCandidateInput, EvictionDecision, EvictionReason,
    MAX_PEER_POLICY_RUNTIME_DECISIONS, MisbehaviorDecision, MisbehaviorKind,
    MisbehaviorObservation, MisbehaviorPolicy, MisbehaviorResponse, PeerBanBook, PeerBanEntry,
    PeerPolicyRuntimeState, UnbanDecision, select_eviction_candidate,
};

fn peer(label: &str) -> EvictionCandidateInput {
    EvictionCandidateInput {
        peer_label: label.to_string(),
        handshake_state: crate::InboundHandshakeState::Handshaking,
        connection_class: "ordinary_inbound",
        slot_class: "ordinary",
        requested_inventory_count: 0,
        active_permission_effects: Vec::new(),
        diversity_group: "198.51.100.0/24".to_string(),
    }
}

#[test]
fn eviction_reason_labels_are_stable() {
    // Arrange
    let labels = [
        (EvictionReason::CapPressure, "cap_pressure"),
        (EvictionReason::HandshakeStalled, "handshake_stalled"),
        (EvictionReason::LowActivity, "low_activity"),
        (EvictionReason::DiversityPressure, "diversity_pressure"),
        (EvictionReason::ProtectedPeer, "protected_peer"),
        (EvictionReason::NoCandidate, "no_eviction_candidate"),
    ];

    // Act
    let rendered = labels.map(|(reason, label)| (reason.as_str(), label));

    // Assert
    assert!(rendered.iter().all(|(actual, expected)| actual == expected));
}

#[test]
fn eviction_selects_highest_unprotected_score() {
    // Arrange
    let low_score = EvictionCandidateInput {
        handshake_state: crate::InboundHandshakeState::Established,
        requested_inventory_count: 1,
        diversity_group: "203.0.113.0/24".to_string(),
        ..peer("low")
    };
    let high_score = peer("high");

    // Act
    let decision = select_eviction_candidate(&[low_score, high_score]);

    // Assert
    assert_eq!(decision.outcome_label(), "eviction_candidate_selected");
    let mut maybe_candidate = None;
    for observed_decision in [decision, select_eviction_candidate(&[])] {
        if let EvictionDecision::Select(candidate) = observed_decision {
            maybe_candidate = Some(candidate);
        }
    }
    let candidate = maybe_candidate.expect("expected selected candidate");
    assert_eq!(candidate.peer_label, "high");
    assert_eq!(candidate.reason.as_str(), "handshake_stalled");
    assert!(candidate.score > 0);
}

#[test]
fn eviction_scoring_records_diversity_pressure() {
    // Arrange
    let first = EvictionCandidateInput {
        peer_label: "first".to_string(),
        handshake_state: crate::InboundHandshakeState::Established,
        slot_class: "reserved",
        requested_inventory_count: 1,
        diversity_group: "shared-group".to_string(),
        ..peer("first")
    };
    let second = EvictionCandidateInput {
        peer_label: "second".to_string(),
        handshake_state: crate::InboundHandshakeState::Established,
        slot_class: "reserved",
        requested_inventory_count: 1,
        diversity_group: "shared-group".to_string(),
        ..peer("second")
    };

    // Act
    let decision = select_eviction_candidate(&[first, second]);

    // Assert
    assert_eq!(decision.outcome_label(), "eviction_candidate_selected");
    let mut maybe_candidate = None;
    for observed_decision in [decision, select_eviction_candidate(&[])] {
        if let EvictionDecision::Select(candidate) = observed_decision {
            maybe_candidate = Some(candidate);
        }
    }
    let candidate = maybe_candidate.expect("expected selected candidate");
    assert_eq!(candidate.reason, EvictionReason::DiversityPressure);
    assert!(candidate.components.iter().any(|component| {
        component.label == EvictionReason::DiversityPressure.as_str() && component.points == 20
    }));
}

#[test]
fn eviction_suppresses_protected_peers() {
    // Arrange
    let protected = EvictionCandidateInput {
        active_permission_effects: vec![PermissionEffectLabel::EvictionPolicyProtected],
        ..peer("protected")
    };

    // Act
    let decision = select_eviction_candidate(&[protected]);

    // Assert
    assert_eq!(decision.outcome_label(), "eviction_suppressed");
    assert!(matches!(
        decision,
        EvictionDecision::Suppress {
            protected_peer_count: 1,
            ..
        }
    ));
}

#[test]
fn ban_scope_and_reason_labels_are_stable() {
    // Arrange
    let address_scope = BanScope::Address(IpAddr::from([198, 51, 100, 10]));
    let subnet_scope = BanScope::Subnet {
        network: IpAddr::from([198, 51, 100, 0]),
        prefix_bits: 24,
    };
    let reasons = [
        (
            BanReason::MisbehaviorThreshold,
            "misbehavior_threshold_reached",
        ),
        (BanReason::Manual, "manual_ban"),
        (BanReason::InvalidAddressAbuse, "invalid_address_abuse"),
    ];

    // Act
    let rendered_reasons = reasons.map(|(reason, label)| (reason.as_str(), label));

    // Assert
    assert_eq!(address_scope.as_str(), "address");
    assert_eq!(subnet_scope.as_str(), "subnet");
    assert!(
        rendered_reasons
            .iter()
            .all(|(actual, expected)| actual == expected)
    );
}

#[test]
fn ban_scope_matches_ipv6_and_zero_prefix_subnets() {
    // Arrange
    let ipv6_scope = BanScope::Subnet {
        network: "2001:db8:1::".parse().expect("test IPv6 network"),
        prefix_bits: 64,
    };
    let ipv6_zero_scope = BanScope::Subnet {
        network: "::".parse().expect("test IPv6 zero network"),
        prefix_bits: 0,
    };
    let ipv4_zero_scope = BanScope::Subnet {
        network: IpAddr::from([0, 0, 0, 0]),
        prefix_bits: 0,
    };
    let invalid_prefix_scope = BanScope::Subnet {
        network: IpAddr::from([192, 0, 2, 0]),
        prefix_bits: 33,
    };

    // Act
    let ipv6_match = ipv6_scope.matches_ip("2001:db8:1::f00d".parse().expect("test IPv6 peer"));
    let ipv6_mismatch = ipv6_scope.matches_ip("2001:db8:2::f00d".parse().expect("test IPv6 peer"));
    let ipv6_zero_match =
        ipv6_zero_scope.matches_ip("2001:db8:ffff::1".parse().expect("test IPv6 peer"));
    let ipv4_zero_match = ipv4_zero_scope.matches_ip(IpAddr::from([203, 0, 113, 99]));
    let family_mismatch =
        ipv4_zero_scope.matches_ip("2001:db8::1".parse().expect("test IPv6 peer"));
    let invalid_prefix_match = invalid_prefix_scope.matches_ip(IpAddr::from([192, 0, 2, 1]));

    // Assert
    assert!(ipv6_match);
    assert!(!ipv6_mismatch);
    assert!(ipv6_zero_match);
    assert!(ipv4_zero_match);
    assert!(!family_mismatch);
    assert!(!invalid_prefix_match);
}

#[test]
fn ban_book_expires_and_unbans_scoped_entries() {
    // Arrange
    let mut book = PeerBanBook::default();
    let scope = BanScope::Address(IpAddr::from([203, 0, 113, 7]));
    let entry = PeerBanEntry {
        scope: scope.clone(),
        reason: BanReason::MisbehaviorThreshold,
        created_at_unix_seconds: 100,
        expires_at_unix_seconds: 200,
        source: "misbehavior_policy",
    };

    // Act
    let active = book.ban(entry, 150);
    let expired = book.maybe_ban_decision(&scope, 250);
    let unban = book.unban(&scope, 150);

    // Assert
    assert!(matches!(active, BanDecision::Active(_)));
    assert_eq!(active.outcome_label(), "ban_active");
    assert!(matches!(expired, Some(BanDecision::Expired(_))));
    assert_eq!(unban.outcome_label(), "unbanned");
}

#[test]
fn ban_book_reports_expired_missing_and_active_counts() {
    // Arrange
    let mut book = PeerBanBook::default();
    let active_scope = BanScope::Address(IpAddr::from([203, 0, 113, 8]));
    let expired_scope = BanScope::Subnet {
        network: IpAddr::from([203, 0, 113, 0]),
        prefix_bits: 24,
    };
    let active_entry = PeerBanEntry {
        scope: active_scope.clone(),
        reason: BanReason::Manual,
        created_at_unix_seconds: 100,
        expires_at_unix_seconds: 300,
        source: "manual_ban",
    };
    let expired_entry = PeerBanEntry {
        scope: expired_scope.clone(),
        reason: BanReason::InvalidAddressAbuse,
        created_at_unix_seconds: 100,
        expires_at_unix_seconds: 150,
        source: "ban_policy",
    };

    // Act
    let expired_ban = book.ban(expired_entry, 200);
    let active_ban = book.ban(active_entry, 200);
    let active_lookup = book.maybe_ban_decision(&active_scope, 200);
    let active_count = book.active_count(200);
    let already_expired = book.unban(&expired_scope, 200);
    let not_found = book.unban(&expired_scope, 200);

    // Assert
    assert!(matches!(expired_ban, BanDecision::Expired(_)));
    assert_eq!(expired_ban.outcome_label(), "ban_expired");
    assert!(matches!(active_ban, BanDecision::Active(_)));
    assert!(matches!(active_lookup, Some(BanDecision::Active(_))));
    assert_eq!(active_count, 1);
    assert!(matches!(already_expired, UnbanDecision::AlreadyExpired(_)));
    assert_eq!(already_expired.outcome_label(), "unban_already_expired");
    assert!(matches!(not_found, UnbanDecision::NotFound(_)));
    assert_eq!(not_found.outcome_label(), "unban_not_found");
}

#[test]
fn misbehavior_labels_are_stable() {
    // Arrange
    let kinds = [
        (MisbehaviorKind::MalformedMessage, "malformed_message"),
        (MisbehaviorKind::DuplicateVersion, "duplicate_version"),
        (MisbehaviorKind::InvalidAddress, "invalid_address"),
        (
            MisbehaviorKind::UnsupportedCommandAbuse,
            "unsupported_command_abuse",
        ),
        (MisbehaviorKind::HeaderViolation, "header_violation"),
    ];
    let responses = [
        (MisbehaviorResponse::ObserveOnly, "misbehavior_observed"),
        (MisbehaviorResponse::Disconnect, "disconnect_requested"),
        (MisbehaviorResponse::Discourage, "discouraged"),
        (MisbehaviorResponse::Ban, "ban_active"),
        (
            MisbehaviorResponse::ProtectedNoAction,
            "protected_no_action",
        ),
    ];

    // Act
    let rendered_kinds = kinds.map(|(kind, label)| (kind.as_str(), label));
    let rendered_responses = responses.map(|(response, label)| (response.as_str(), label));

    // Assert
    assert!(
        rendered_kinds
            .iter()
            .all(|(actual, expected)| actual == expected)
    );
    assert!(
        rendered_responses
            .iter()
            .all(|(actual, expected)| actual == expected)
    );
}

#[test]
fn misbehavior_policy_thresholds_select_ordered_responses() {
    // Arrange
    let policy = MisbehaviorPolicy::default();
    let cases = [
        (1, MisbehaviorResponse::ObserveOnly),
        (10, MisbehaviorResponse::Disconnect),
        (50, MisbehaviorResponse::Discourage),
        (100, MisbehaviorResponse::Ban),
    ];

    // Act
    let decisions = cases.map(|(points, expected_response)| {
        let decision = policy.decide(MisbehaviorObservation {
            peer_label: format!("peer-{points}"),
            kind: MisbehaviorKind::DuplicateVersion,
            points,
            prior_score: 0,
            protected: false,
        });
        (decision, expected_response)
    });

    // Assert
    assert!(
        decisions
            .iter()
            .all(|(decision, expected_response)| decision.response == *expected_response)
    );
}

#[test]
fn protected_misbehavior_records_no_action() {
    // Arrange
    let policy = MisbehaviorPolicy::default();
    let observation = MisbehaviorObservation {
        peer_label: "protected".to_string(),
        kind: MisbehaviorKind::MalformedMessage,
        points: 500,
        prior_score: 0,
        protected: true,
    };

    // Act
    let decision = policy.decide(observation);

    // Assert
    assert_eq!(decision.response, MisbehaviorResponse::ProtectedNoAction);
    assert_eq!(decision.response.as_str(), "protected_no_action");
    assert_eq!(decision.kind.as_str(), "malformed_message");
}

fn runtime_ban_entry(scope: BanScope, expires_at_unix_seconds: i64) -> PeerBanEntry {
    PeerBanEntry {
        scope,
        reason: BanReason::Manual,
        created_at_unix_seconds: 100,
        expires_at_unix_seconds,
        source: "runtime_test",
    }
}

#[test]
fn runtime_state_scopes_address_bans_to_matching_remote() {
    // Arrange
    let mut state = PeerPolicyRuntimeState::default();
    let banned_ip = IpAddr::from([127, 0, 0, 2]);
    let unrelated_ip = IpAddr::from([127, 0, 0, 3]);

    // Act
    state.record_ban(runtime_ban_entry(BanScope::Address(banned_ip), 300), 150);
    let banned = state.reconnect_suppression_input_for_ip(banned_ip, 150);
    let unrelated = state.reconnect_suppression_input_for_ip(unrelated_ip, 150);

    // Assert
    assert!(banned.banned);
    assert!(!banned.discouraged);
    assert!(!unrelated.banned);
    assert!(!unrelated.discouraged);
}

#[test]
fn runtime_state_scopes_subnet_bans_to_matching_remote() {
    // Arrange
    let mut state = PeerPolicyRuntimeState::default();
    let scope = BanScope::Subnet {
        network: IpAddr::from([192, 0, 2, 0]),
        prefix_bits: 24,
    };

    // Act
    state.record_ban(runtime_ban_entry(scope, 300), 150);
    let matching = state.reconnect_suppression_input_for_ip(IpAddr::from([192, 0, 2, 10]), 150);
    let unrelated = state.reconnect_suppression_input_for_ip(IpAddr::from([198, 51, 100, 10]), 150);

    // Assert
    assert!(matching.banned);
    assert!(!matching.discouraged);
    assert!(!unrelated.banned);
    assert!(!unrelated.discouraged);
}

#[test]
fn runtime_state_ignores_expired_bans_for_reconnect() {
    // Arrange
    let mut state = PeerPolicyRuntimeState::default();
    let remote_ip = IpAddr::from([203, 0, 113, 20]);

    // Act
    state.record_ban(runtime_ban_entry(BanScope::Address(remote_ip), 120), 150);
    let reconnect = state.reconnect_suppression_input_for_ip(remote_ip, 150);

    // Assert
    assert!(!reconnect.banned);
    assert!(!reconnect.discouraged);
}

#[test]
fn runtime_state_records_discouraged_reconnects_separately() {
    // Arrange
    let mut state = PeerPolicyRuntimeState::default();
    let remote_ip = IpAddr::from([203, 0, 113, 21]);

    // Act
    state.record_discouragement(runtime_ban_entry(BanScope::Address(remote_ip), 300), 150);
    let reconnect = state.reconnect_suppression_input_for_ip(remote_ip, 150);

    // Assert
    assert!(!reconnect.banned);
    assert!(reconnect.discouraged);
    assert!(state.ban_decisions().is_empty());
}

#[test]
fn runtime_state_ignores_expired_discouragement_for_reconnect() {
    // Arrange
    let mut state = PeerPolicyRuntimeState::default();
    let remote_ip = IpAddr::from([203, 0, 113, 23]);

    // Act
    let decision =
        state.record_discouragement(runtime_ban_entry(BanScope::Address(remote_ip), 120), 150);
    let reconnect = state.reconnect_suppression_input_for_ip(remote_ip, 150);

    // Assert
    assert!(matches!(decision, BanDecision::Expired(_)));
    assert!(!reconnect.banned);
    assert!(!reconnect.discouraged);
}

#[test]
fn runtime_state_records_unban_and_misbehavior_decisions() {
    // Arrange
    let mut state = PeerPolicyRuntimeState::default();
    let scope = BanScope::Address(IpAddr::from([203, 0, 113, 22]));
    let decision = MisbehaviorDecision {
        peer_label: "peer-protected".to_string(),
        kind: MisbehaviorKind::MalformedMessage,
        score: 500,
        response: MisbehaviorResponse::ProtectedNoAction,
    };

    // Act
    state.record_ban(runtime_ban_entry(scope.clone(), 300), 150);
    let unban = state.record_unban(&scope, 160);
    state.record_misbehavior(decision);

    // Assert
    assert!(matches!(unban, UnbanDecision::Unbanned(_)));
    assert_eq!(state.ban_decisions().len(), 1);
    assert_eq!(state.unban_decisions().len(), 1);
    assert_eq!(state.misbehavior_decisions().len(), 1);
    assert_eq!(
        state.misbehavior_decisions()[0].response,
        MisbehaviorResponse::ProtectedNoAction
    );
}

#[test]
fn runtime_state_bounds_recorded_decision_history() {
    // Arrange
    let mut state = PeerPolicyRuntimeState::default();

    // Act
    for score in 0..=MAX_PEER_POLICY_RUNTIME_DECISIONS {
        state.record_misbehavior(MisbehaviorDecision {
            peer_label: format!("peer-{score}"),
            kind: MisbehaviorKind::MalformedMessage,
            score: score as u32,
            response: MisbehaviorResponse::Disconnect,
        });
    }

    // Assert
    assert_eq!(
        state.misbehavior_decisions().len(),
        MAX_PEER_POLICY_RUNTIME_DECISIONS
    );
    assert_eq!(state.misbehavior_decisions()[0].peer_label, "peer-1");
}
