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
    MisbehaviorKind, MisbehaviorObservation, MisbehaviorPolicy, MisbehaviorResponse, PeerBanBook,
    PeerBanEntry, UnbanDecision, select_eviction_candidate,
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
