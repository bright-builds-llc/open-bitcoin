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

use open_bitcoin_network::InboundResourceEvent;
use open_bitcoin_node::{core::wallet::AddressNetwork, status::FieldAvailability};

use crate::config::RuntimeConfig;

use super::ManagedRpcContext;

#[test]
fn managed_rpc_context_builds_from_runtime_config() {
    // Arrange
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        ..RuntimeConfig::default()
    };

    // Act
    let context = ManagedRpcContext::from_runtime_config(&runtime);
    let network_info = context.network_info();
    let wallet_info = context.wallet_info();
    let snapshot = context.blockchain_snapshot();

    // Assert
    assert_eq!(context.chain(), AddressNetwork::Regtest);
    assert_eq!(network_info.connected_peers, 0);
    assert_eq!(wallet_info.network, AddressNetwork::Regtest);
    assert!(snapshot.active_chain.is_empty());
}

#[test]
fn record_inbound_resource_event_projects_current_inbound_status() {
    // Arrange
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);
    let event = InboundResourceEvent {
        outcome: "rejected".to_string(),
        reason: "payload checksum did not match message header".to_string(),
        label: "invalid_checksum".to_string(),
        source: "source_envelope_gate".to_string(),
        message: "inbound_message_resource_governance".to_string(),
        next_action: "payload_rejected".to_string(),
    };

    // Act
    context.record_inbound_resource_event(event);
    let status = context.current_inbound_status();

    // Assert
    let FieldAvailability::Available(inbound) = status else {
        panic!("resource governance event should make inbound status available");
    };
    assert_eq!(inbound.payload_rejections, 1);
    assert_eq!(inbound.resource_pressure_events, 0);
    let FieldAvailability::Available(decision) = inbound.latest_resource_governance_decision else {
        panic!("latest resource decision should be available");
    };
    assert_eq!(decision.next_action, "payload_rejected");
}
