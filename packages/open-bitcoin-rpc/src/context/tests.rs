use open_bitcoin_node::core::wallet::AddressNetwork;

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
