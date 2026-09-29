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

use std::collections::BTreeSet;

use serde_json::{Value, json};

use open_bitcoin_node::ChainstateStore;
use open_bitcoin_node::core::chainstate::{
    CoinsView, ManualPruneArgument, ManualPruneArgumentError, ManualPruneInput, ManualPruneRefusal,
    PruneMode, plan_manual_prune, resolve_manual_prune_argument,
};
use open_bitcoin_node::core::primitives::BlockHash;
use open_bitcoin_node::status::{ManualPruneRefusalCode, ManualPruneSurface};

use super::{ManagedRpcContext, RpcFailure, network_authority_error_to_failure};
use crate::error::{RpcErrorCode, RpcErrorDetail, RpcFailureKind};
use crate::method::{
    ClearPruneLockRequest, MethodCall, PruneBlockchainRequest, PruneLockEntry, SetPruneLockRequest,
};

const DISABLED_PRUNE: &str = "Cannot prune blocks because node is not in prune mode.";
const CHAIN_TOO_SHORT: &str = "Blockchain is too short for pruning.";
const TARGET_ABOVE_TIP: &str = "Blockchain is shorter than the attempted prune height.";
const KEEP_WINDOW: &str = "target is inside the 288-block keep window";
const NEGATIVE_HEIGHT: &str = "Negative block height.";
const TIMESTAMP_NOT_FOUND: &str = "block at the requested timestamp was not found";

/// Dispatches prune lock and `pruneblockchain` RPC methods.
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
        MethodCall::PruneBlockchain(request) => prune_blockchain(context, request),
        _ => Err(RpcFailure::internal_error("unsupported prune method")),
    }
}

fn prune_blockchain<S, V>(
    context: &mut ManagedRpcContext<S, V>,
    request: PruneBlockchainRequest,
) -> Result<Value, RpcFailure>
where
    S: ChainstateStore,
    V: CoinsView,
{
    let mode = context
        .prune_mode()
        .map_err(network_authority_error_to_failure)?;
    if matches!(mode, PruneMode::Disabled) {
        return refuse_manual_prune(
            context,
            ManualPruneRefusalCode::Disabled,
            misc_error(DISABLED_PRUNE),
        );
    }

    let header_times = header_times_by_height(context)?;
    let resolved = match resolve_manual_prune_argument(request.height, &header_times) {
        Ok(argument) => argument,
        Err(ManualPruneArgumentError::Negative) => {
            return refuse_manual_prune(
                context,
                ManualPruneRefusalCode::NegativeHeight,
                invalid_parameter(NEGATIVE_HEIGHT),
            );
        }
        Err(ManualPruneArgumentError::TimestampNotFound) => {
            return refuse_manual_prune(
                context,
                ManualPruneRefusalCode::TimestampNotFound,
                invalid_parameter(TIMESTAMP_NOT_FOUND),
            );
        }
    };
    let ManualPruneArgument::Height(target_height) = resolved else {
        let height = json!(0);
        context.record_manual_prune(ManualPruneSurface::Height { height: 0 });
        return Ok(height);
    };

    let locks = context.list_prune_locks()?;
    let input = ManualPruneInput {
        tip: chain_tip_height(context)?,
        prune_after_height: context.prune_after_height(),
        mode,
        target_height,
        maybe_present_heights: Some(present_payload_heights(context)?),
        locks: locks.clone(),
    };
    let plan = match plan_manual_prune(&input) {
        Ok(plan) => plan,
        Err(refusal) => return refuse_manual_plan(context, refusal),
    };
    context
        .flush_applying_prune_plan(&plan, &locks)
        .map_err(network_authority_error_to_failure)?;
    let height = configured_last_pruned_height(context, mode)?
        .map(i64::from)
        .unwrap_or(-1);
    context.record_manual_prune(ManualPruneSurface::Height { height });
    Ok(json!(height))
}

fn refuse_manual_plan<S, V>(
    context: &mut ManagedRpcContext<S, V>,
    refusal: ManualPruneRefusal,
) -> Result<Value, RpcFailure>
where
    S: ChainstateStore,
    V: CoinsView,
{
    let (reason, failure) = match refusal {
        ManualPruneRefusal::Disabled => {
            (ManualPruneRefusalCode::Disabled, misc_error(DISABLED_PRUNE))
        }
        ManualPruneRefusal::ChainTooShort { .. } => (
            ManualPruneRefusalCode::ChainTooShort,
            misc_error(CHAIN_TOO_SHORT),
        ),
        ManualPruneRefusal::TargetAboveTip { .. } => (
            ManualPruneRefusalCode::TargetAboveTip,
            invalid_parameter(TARGET_ABOVE_TIP),
        ),
        ManualPruneRefusal::TargetInsideKeepWindow { .. } => (
            ManualPruneRefusalCode::KeepWindow,
            invalid_parameter(KEEP_WINDOW),
        ),
    };
    refuse_manual_prune(context, reason, failure)
}

fn refuse_manual_prune<S, V>(
    context: &mut ManagedRpcContext<S, V>,
    reason: ManualPruneRefusalCode,
    failure: RpcFailure,
) -> Result<Value, RpcFailure>
where
    S: ChainstateStore,
    V: CoinsView,
{
    context.record_manual_prune(ManualPruneSurface::Refused { reason });
    Err(failure)
}

fn chain_tip_height<S, V>(context: &ManagedRpcContext<S, V>) -> Result<u32, RpcFailure>
where
    S: ChainstateStore,
    V: CoinsView,
{
    let maybe_tip = context
        .maybe_chain_tip()
        .map_err(network_authority_error_to_failure)?;
    Ok(maybe_tip.map(|tip| tip.height).unwrap_or(0))
}

fn header_times_by_height<S, V>(
    context: &ManagedRpcContext<S, V>,
) -> Result<Vec<(u32, i64)>, RpcFailure>
where
    S: ChainstateStore,
    V: CoinsView,
{
    let snapshot = context
        .blockchain_snapshot()
        .map_err(network_authority_error_to_failure)?;
    Ok(snapshot
        .active_chain
        .iter()
        .map(|position| (position.height, i64::from(position.header.time)))
        .collect())
}

fn present_payload_heights<S, V>(
    context: &ManagedRpcContext<S, V>,
) -> Result<BTreeSet<u32>, RpcFailure>
where
    S: ChainstateStore,
    V: CoinsView,
{
    let snapshot = context
        .blockchain_snapshot()
        .map_err(network_authority_error_to_failure)?;
    let mut present = BTreeSet::new();
    for position in &snapshot.active_chain {
        if payload_present(context, position.block_hash)? {
            present.insert(position.height);
        }
    }
    Ok(present)
}

fn payload_present<S, V>(
    context: &ManagedRpcContext<S, V>,
    block_hash: BlockHash,
) -> Result<bool, RpcFailure>
where
    S: ChainstateStore,
    V: CoinsView,
{
    if context
        .cached_block_present(block_hash)
        .map_err(network_authority_error_to_failure)?
    {
        return Ok(true);
    }
    context
        .durable_block_present(block_hash)
        .map_err(|_| RpcFailure::internal_error("prune height block presence is unavailable"))
}

fn misc_error(message: &str) -> RpcFailure {
    RpcFailure::new(
        RpcFailureKind::InternalError,
        Some(RpcErrorDetail::new(RpcErrorCode::MiscError, message)),
    )
}

fn invalid_parameter(message: &str) -> RpcFailure {
    RpcFailure::invalid_parameter(message)
}
