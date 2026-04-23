use open_bitcoin_node::core::wallet::AddressNetwork;

use super::ManagedRpcContext;

#[test]
fn managed_rpc_context_composes_network_and_wallet_facades() {
    // Arrange
    let context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);

    // Act
    let network_info = context.network_info();
    let wallet_info = context.wallet_info();
    let snapshot = context.blockchain_snapshot();

    // Assert
    assert_eq!(network_info.connected_peers, 0);
    assert_eq!(wallet_info.network, AddressNetwork::Regtest);
    assert!(snapshot.active_chain.is_empty());
}
