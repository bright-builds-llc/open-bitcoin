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

use super::*;

#[test]
fn authoritative_network_sync_mutation_is_visible_to_rpc() {
    // Arrange
    let data_dir = test_data_dir("authoritative-network");
    let store = FjallNodeStore::open(&data_dir).expect("open authoritative store");
    let sync_runtime = DurableSyncRuntime::open(
        store.clone(),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            dns_seeds: Vec::new(),
            ..SyncRuntimeConfig::default()
        },
    )
    .expect("open authoritative sync runtime");
    let sync_network = sync_runtime.network_handle();
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &RuntimeConfig {
            chain: AddressNetwork::Regtest,
            ..RuntimeConfig::default()
        },
        sync_runtime.network_handle(),
        Some(store),
    )
    .expect("construct RPC from authoritative handle");

    // Act
    sync_network
        .connect_outbound_peer(42, 1_777_225_210)
        .expect("sync mutation should succeed");
    let network_info = context
        .network_info()
        .expect("RPC should snapshot the shared authority");

    // Assert
    assert_eq!(network_info.outbound_peers, 1);
    fs::remove_dir_all(data_dir).expect("remove authoritative store");
}

#[test]
fn managed_rpc_context_builds_from_runtime_config() {
    // Arrange
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        ..RuntimeConfig::default()
    };

    // Act
    let context = ManagedRpcContext::from_runtime_config(&runtime);
    let network_info = context.network_info().expect("authoritative network info");
    let wallet_info = context.wallet_info();
    let snapshot = context
        .blockchain_snapshot()
        .expect("authoritative chainstate snapshot");

    // Assert
    assert_eq!(context.chain(), AddressNetwork::Regtest);
    assert_eq!(network_info.connected_peers, 0);
    assert!(
        !context
            .network_info()
            .expect("authoritative network info")
            .relay
    );
    assert_eq!(wallet_info.network, AddressNetwork::Regtest);
    assert!(snapshot.active_chain.is_empty());
}

#[test]
fn managed_rpc_context_builds_from_runtime_config_with_enabled_relay_activation() {
    // Arrange
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        relay: RelayActivationConfig { enabled: true },
        inbound: open_bitcoin_network::InboundListenerConfig {
            enabled: true,
            ..Default::default()
        },
        ..RuntimeConfig::default()
    };

    // Act
    let context = ManagedRpcContext::from_runtime_config(&runtime);

    // Assert
    assert!(
        context
            .network_info()
            .expect("authoritative network info")
            .relay
    );
}
