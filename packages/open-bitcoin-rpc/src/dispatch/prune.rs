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

//! Prune facts for `getblockchaininfo` and the operator status response.

mod status;

#[cfg(test)]
mod tests;

pub(in crate::dispatch) use status::{configured_last_pruned_height, operator_prune_status};

use serde_json::{Value, json};

use open_bitcoin_node::ChainstateStore;
use open_bitcoin_node::core::chainstate::CoinsView;

use super::{ManagedRpcContext, RpcFailure};
use crate::method::{ClearPruneLockRequest, MethodCall, PruneLockEntry, SetPruneLockRequest};

/// Dispatches prune-lock RPC methods. `pruneblockchain` arrives in the next task.
pub(in crate::dispatch) fn dispatch_prune<S, V>(
    context: &mut ManagedRpcContext<S, V>,
    call: MethodCall,
) -> Result<Value, RpcFailure>
where
    S: ChainstateStore,
    V: CoinsView,
{
    match call {
        MethodCall::ListPruneLocks(_) => {
            let locks = context.list_prune_locks()?;
            let entries: Vec<PruneLockEntry> = locks
                .into_iter()
                .map(|lock| PruneLockEntry {
                    name: lock.name,
                    height_first: lock.height_first,
                    height_last: lock.height_last,
                })
                .collect();
            serde_json::to_value(entries)
                .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::SetPruneLock(SetPruneLockRequest {
            name,
            height_first,
            height_last,
        }) => {
            let record = context.replace_prune_lock(name, height_first, height_last)?;
            serde_json::to_value(PruneLockEntry {
                name: record.name,
                height_first: record.height_first,
                height_last: record.height_last,
            })
            .map_err(|error| RpcFailure::internal_error(error.to_string()))
        }
        MethodCall::ClearPruneLock(ClearPruneLockRequest { name }) => {
            let removed = context.clear_named_prune_lock(&name)?;
            if removed {
                Ok(json!({ "success": true }))
            } else {
                Ok(json!({ "success": false }))
            }
        }
        _ => Err(RpcFailure::internal_error("unsupported prune method")),
    }
}
