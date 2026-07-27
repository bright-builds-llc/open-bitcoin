// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::network_fixtures::*;
use super::*;

#[test]
fn permission_context_defaults_to_ordinary_decision_without_configured_classes() {
    // Arrange
    let context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    let remote_addr = "127.0.0.1:50000".parse().expect("remote address");

    // Act
    let decision = context.permission_decision_for_remote_addr(remote_addr);

    // Assert
    assert_eq!(
        decision.connection_class(),
        PeerConnectionClass::OrdinaryInbound
    );
    assert!(decision.active_effects().is_empty());
    assert!(decision.inactive_effects().is_empty());
}

#[test]
fn permission_context_resolves_permissioned_literal_ip_from_runtime_config() {
    // Arrange
    let context = permission_context(vec![parsed_permission_class(
        "loopback-download",
        "127.0.0.1",
        &["in", "download", "addr", "relay"],
    )]);
    let remote_addr = "127.0.0.1:50000".parse().expect("remote address");

    // Act
    let decision = context.permission_decision_for_remote_addr(remote_addr);

    // Assert
    assert_eq!(
        decision.connection_class(),
        PeerConnectionClass::PermissionedInbound
    );
    assert_eq!(decision.slot_class(), InboundAdmissionSlotClass::Ordinary);
    assert!(
        decision
            .active_effects()
            .contains(&PermissionEffectLabel::DownloadServingPolicyInput)
    );
    assert!(
        decision
            .active_effects()
            .contains(&PermissionEffectLabel::AddressResponsePolicyInput)
    );
    assert!(
        decision
            .relay_permission_effects()
            .contains(&RelayPermissionEffectLabel::TransactionRelayPolicyInput)
    );
    assert!(decision.inactive_effects().is_empty());
}

#[test]
fn permission_context_resolves_protected_literal_ip_without_raw_class_leak() {
    // Arrange
    let context = permission_context(vec![parsed_permission_class(
        "operator-loopback-secret-name",
        "127.0.0.1",
        &["in", "noban", "forceinbound"],
    )]);
    let matched_addr = "127.0.0.1:50000".parse().expect("matched address");
    let unmatched_addr = "127.0.0.2:50000".parse().expect("unmatched address");

    // Act
    let matched = context.permission_decision_for_remote_addr(matched_addr);
    let unmatched = context.permission_decision_for_remote_addr(unmatched_addr);
    let debug = format!("{context:?}");

    // Assert
    assert_eq!(
        matched.connection_class(),
        PeerConnectionClass::ProtectedInbound
    );
    assert_eq!(matched.slot_class(), InboundAdmissionSlotClass::Reserved);
    assert!(
        matched
            .active_effects()
            .contains(&PermissionEffectLabel::AdmissionProtected)
    );
    assert!(
        matched
            .active_effects()
            .contains(&PermissionEffectLabel::EvictionPolicyProtected)
    );
    assert_eq!(
        unmatched.connection_class(),
        PeerConnectionClass::OrdinaryInbound
    );
    assert!(!debug.contains("operator-loopback-secret-name"));
}
