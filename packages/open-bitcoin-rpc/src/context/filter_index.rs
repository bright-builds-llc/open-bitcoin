// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/node.cpp
// - packages/bitcoin-knots/src/index/base.cpp

use open_bitcoin_node::core::chainstate::CoinsView;
use open_bitcoin_node::{ChainstateStore, ManagedNetworkHandle};

use super::ManagedRpcContext;

impl<S: ChainstateStore, V: CoinsView> ManagedRpcContext<S, V> {
    /// Clone the configured shared authority without opening another index/store.
    pub(crate) fn basic_filter_network_handle(&self) -> ManagedNetworkHandle<S, V> {
        self.network.clone()
    }
}
