// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::listener_fixtures::*;
use super::*;

#[test]
fn reconnect_suppression_uses_matching_remote_policy_state() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .record_peer_policy_ban(
            peer_policy_entry(BanScope::Address(IpAddr::from([127, 0, 0, 2])), 300),
            150,
        )
        .expect("authoritative peer policy");

    // Act
    let reconnect = context
        .reconnect_suppression_input_for_remote_addr(
            "127.0.0.2:18444".parse().expect("valid remote addr"),
            150,
        )
        .expect("authoritative reconnect state");

    // Assert
    assert!(reconnect.banned);
    assert!(!reconnect.discouraged);
}

#[test]
fn reconnect_suppression_ignores_non_matching_remote_policy_state() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .record_peer_policy_ban(
            peer_policy_entry(BanScope::Address(IpAddr::from([127, 0, 0, 2])), 300),
            150,
        )
        .expect("authoritative peer policy");

    // Act
    let reconnect = context
        .reconnect_suppression_input_for_remote_addr(
            "127.0.0.3:18444".parse().expect("valid remote addr"),
            150,
        )
        .expect("authoritative reconnect state");

    // Assert
    assert!(!reconnect.banned);
    assert!(!reconnect.discouraged);
}

#[test]
fn listener_records_scoped_banned_reconnect_suppression() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .record_peer_policy_ban(
            peer_policy_entry(BanScope::Address(IpAddr::from([127, 0, 0, 1])), 300),
            150,
        )
        .expect("authoritative peer policy");
    let mut evidence = listener_evidence(&["127.0.0.1:18444"]);
    let reconnect = context
        .reconnect_suppression_input_for_remote_addr(
            "127.0.0.1:18444".parse().expect("valid remote addr"),
            150,
        )
        .expect("authoritative reconnect state");
    let event = match ResourceGovernancePolicy::default().decide_reconnect(reconnect) {
        ResourceGovernanceDecision::Disconnect(event) => event,
        other => panic!("expected reconnect_suppressed_banned event, got {other:?}"),
    };

    // Act
    evidence.record_resource_event(event);

    // Assert
    assert_eq!(evidence.reconnect_suppressions, 1);
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .expect("latest resource event")
            .label,
        "reconnect_suppressed_banned"
    );
}

#[test]
fn listener_records_scoped_discouraged_reconnect_suppression() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .record_peer_policy_discouragement(
            peer_policy_entry(BanScope::Address(IpAddr::from([127, 0, 0, 1])), 300),
            150,
        )
        .expect("authoritative peer policy");
    let mut evidence = listener_evidence(&["127.0.0.1:18444"]);
    let reconnect = context
        .reconnect_suppression_input_for_remote_addr(
            "127.0.0.1:18444".parse().expect("valid remote addr"),
            150,
        )
        .expect("authoritative reconnect state");
    let event = match ResourceGovernancePolicy::default().decide_reconnect(reconnect) {
        ResourceGovernanceDecision::Backpressure(event) => event,
        other => panic!("expected reconnect_suppressed_discouraged event, got {other:?}"),
    };

    // Act
    evidence.record_resource_event(event);

    // Assert
    assert_eq!(evidence.reconnect_suppressions, 1);
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .expect("latest resource event")
            .label,
        "reconnect_suppressed_discouraged"
    );
}
