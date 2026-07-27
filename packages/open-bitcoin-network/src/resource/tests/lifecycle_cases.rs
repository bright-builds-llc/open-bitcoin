// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_invalid_messages.py

use super::*;

#[test]
fn slow_handshake_timeout_disconnects_with_stable_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = ResourceTimeoutInput {
        handshake_state: InboundHandshakeState::Handshaking,
        connected_at_unix_seconds: 100,
        last_activity_unix_seconds: 100,
        now_unix_seconds: 100 + PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS + 1,
    };

    // Act
    let decision = policy.decide_timeout(input);

    // Assert
    assert_lifecycle_event(
        decision,
        ResourceLifecycleLabel::SlowHandshake,
        "timeout_disconnect",
    );
}

#[test]
fn established_idle_timeout_disconnects_with_stable_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = ResourceTimeoutInput {
        handshake_state: InboundHandshakeState::Established,
        connected_at_unix_seconds: 100,
        last_activity_unix_seconds: 200,
        now_unix_seconds: 200 + policy.idle_peer_timeout_seconds + 1,
    };

    // Act
    let decision = policy.decide_timeout(input);

    // Assert
    assert_lifecycle_event(
        decision,
        ResourceLifecycleLabel::IdlePeer,
        "timeout_disconnect",
    );
}

#[test]
fn connection_churn_window_rejects_above_configured_cap() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = ConnectionChurnInput {
        window_started_unix_seconds: 300,
        now_unix_seconds: 300,
        connection_attempts_in_window: PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW + 1,
    };

    // Act
    let decision = policy.decide_churn(input);

    // Assert
    assert_lifecycle_event(
        decision,
        ResourceLifecycleLabel::ConnectionChurnLimited,
        "churn_rejected",
    );
}

#[test]
fn repeated_failure_window_rejects_above_configured_cap() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RepeatedFailureInput {
        window_started_unix_seconds: 400,
        now_unix_seconds: 400 + PHASE94_REPEATED_FAILURE_WINDOW_SECONDS,
        failures_in_window: PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW + 1,
    };

    // Act
    let decision = policy.decide_repeated_failure(input);

    // Assert
    assert_lifecycle_event(
        decision,
        ResourceLifecycleLabel::RepeatedFailureLimited,
        "churn_rejected",
    );
}

#[test]
fn active_ban_and_discouraged_reconnect_are_suppressed() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let banned_input = ReconnectSuppressionInput {
        banned: true,
        discouraged: true,
    };
    let discouraged_input = ReconnectSuppressionInput {
        banned: false,
        discouraged: true,
    };

    // Act
    let banned_decision = policy.decide_reconnect(banned_input);
    let discouraged_decision = policy.decide_reconnect(discouraged_input);

    // Assert
    assert_lifecycle_event(
        banned_decision,
        ResourceLifecycleLabel::ReconnectSuppressedBanned,
        "reconnect_suppressed",
    );
    assert_lifecycle_event(
        discouraged_decision,
        ResourceLifecycleLabel::ReconnectSuppressedDiscouraged,
        "reconnect_suppressed",
    );
}

#[test]
fn lifecycle_policy_accepts_inputs_at_configured_caps() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let handshake_input = ResourceTimeoutInput {
        handshake_state: InboundHandshakeState::Handshaking,
        connected_at_unix_seconds: 500,
        last_activity_unix_seconds: 500,
        now_unix_seconds: 500 + policy.slow_handshake_timeout_seconds,
    };
    let idle_input = ResourceTimeoutInput {
        handshake_state: InboundHandshakeState::Established,
        connected_at_unix_seconds: 500,
        last_activity_unix_seconds: 600,
        now_unix_seconds: 600 + policy.idle_peer_timeout_seconds,
    };
    let churn_input = ConnectionChurnInput {
        window_started_unix_seconds: 700,
        now_unix_seconds: 700 + policy.connection_churn_window_seconds,
        connection_attempts_in_window: policy.max_connections_per_churn_window,
    };
    let expired_failure_input = RepeatedFailureInput {
        window_started_unix_seconds: 800,
        now_unix_seconds: 800 + policy.repeated_failure_window_seconds + 1,
        failures_in_window: policy.max_repeated_failures_per_window + 1,
    };
    let reconnect_input = ReconnectSuppressionInput {
        banned: false,
        discouraged: false,
    };

    // Act
    let handshake_decision = policy.decide_timeout(handshake_input);
    let idle_decision = policy.decide_timeout(idle_input);
    let churn_decision = policy.decide_churn(churn_input);
    let failure_decision = policy.decide_repeated_failure(expired_failure_input);
    let reconnect_decision = policy.decide_reconnect(reconnect_input);

    // Assert
    assert_eq!(handshake_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(idle_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(churn_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(failure_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(reconnect_decision, ResourceGovernanceDecision::Accept);
}

#[test]
fn lifecycle_label_strings_cover_phase94_contract() {
    // Arrange
    let labels = [
        (ResourceLifecycleLabel::SlowHandshake, "slow_handshake"),
        (ResourceLifecycleLabel::IdlePeer, "idle_peer"),
        (
            ResourceLifecycleLabel::ConnectionChurnLimited,
            "connection_churn_limited",
        ),
        (
            ResourceLifecycleLabel::RepeatedFailureLimited,
            "repeated_failure_limited",
        ),
        (
            ResourceLifecycleLabel::ReconnectSuppressedBanned,
            "reconnect_suppressed_banned",
        ),
        (
            ResourceLifecycleLabel::ReconnectSuppressedDiscouraged,
            "reconnect_suppressed_discouraged",
        ),
    ];

    // Act
    let label_strings = labels.map(|(label, _)| label.as_str());

    // Assert
    assert_eq!(
        label_strings,
        labels.map(|(_, expected_label)| expected_label)
    );
}

#[test]
fn malformed_header_parse_failures_are_resource_labels() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let short_header = [0_u8; 3];
    let mut bad_padding_header = [0_u8; INBOUND_MESSAGE_HEADER_LEN];
    bad_padding_header[..4].copy_from_slice(NetworkMagic::MAINNET.as_bytes());
    bad_padding_header[4] = b'a';
    bad_padding_header[6] = b'b';

    // Act
    let short_decision = policy.evaluate_header(&short_header);
    let bad_padding_decision = policy.evaluate_header(&bad_padding_header);

    // Assert
    assert_rejection(short_decision, ResourceViolationLabel::MalformedHeader);
    assert_rejection(
        bad_padding_decision,
        ResourceViolationLabel::MalformedHeader,
    );
}
