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

use open_bitcoin_node::MemoryChainstateStore;
use open_bitcoin_node::core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_node::{DurableSyncState, ManagedPeerNetwork};

mod network;
mod rescan;
#[cfg(test)]
mod tests;
mod wallet_state;

pub use rescan::{WalletFreshnessKind, WalletFreshnessView, WalletRescanExecution};
use wallet_state::WalletState;

pub struct ManagedRpcContext {
    chain: AddressNetwork,
    consensus_params: ConsensusParams,
    verify_flags: ScriptVerifyFlags,
    network: ManagedPeerNetwork<MemoryChainstateStore>,
    maybe_durable_sync_state: Option<DurableSyncState>,
    wallet_state: WalletState,
}

impl core::fmt::Debug for ManagedRpcContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let wallet_mode = match &self.wallet_state {
            WalletState::Local(_) => "local",
            WalletState::DurableNamedRegistry { .. } => "durable",
        };
        f.debug_struct("ManagedRpcContext")
            .field("chain", &self.chain)
            .field("consensus_params", &self.consensus_params)
            .field("verify_flags", &self.verify_flags)
            .field(
                "has_durable_sync_state",
                &self.maybe_durable_sync_state.is_some(),
            )
            .field("wallet_mode", &wallet_mode)
            .finish()
    }
}
