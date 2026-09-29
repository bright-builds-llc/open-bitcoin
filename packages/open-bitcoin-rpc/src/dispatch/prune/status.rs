// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp

//! Active-chain walk and the live operator prune object.

use open_bitcoin_node::core::{
    chainstate::{ChainstateSnapshot, PruneMode, get_prune_height, project_prune_status},
    primitives::BlockHash,
};
use open_bitcoin_node::status::{
    FieldAvailability, ManualPruneSurface, PruneLockRow, PruneOperatorStatus,
};

use super::super::{ManagedRpcContext, RpcFailure, network_authority_error_to_failure};

const PRUNE_HEIGHT_PRESENCE_UNAVAILABLE: &str = "prune height block presence is unavailable";
const PRUNE_LOCKS_UNAVAILABLE: &str = "prune locks are unavailable";
const PRUNE_SUPPORT_COUNTS_UNAVAILABLE: &str = "prune support counts are unavailable";

/// Last pruned height for an enabled mode. Disabled skips the chain walk.
pub(in crate::dispatch) fn configured_last_pruned_height<S, V>(
    context: &ManagedRpcContext<S, V>,
    mode: PruneMode,
) -> Result<Option<u32>, RpcFailure>
where
    S: open_bitcoin_node::ChainstateStore,
    V: open_bitcoin_node::core::chainstate::CoinsView,
{
    if matches!(mode, PruneMode::Disabled) {
        return Ok(None);
    }

    let maybe_tip = context
        .maybe_chain_tip()
        .map_err(network_authority_error_to_failure)?;
    let Some(tip) = maybe_tip else {
        return Ok(get_prune_height(0, &[]));
    };
    if tip.height == 0 {
        return Ok(get_prune_height(0, &[]));
    }

    let snapshot = context
        .blockchain_snapshot()
        .map_err(network_authority_error_to_failure)?;
    let mut complete_from_height_one = vec![false; tip.height as usize];
    for position in snapshot
        .active_chain
        .iter()
        .filter(|position| position.height >= 1 && position.height <= tip.height)
    {
        let index = (position.height - 1) as usize;
        complete_from_height_one[index] =
            block_payload_and_undo_present(context, position.block_hash, &snapshot)?;
    }
    Ok(get_prune_height(tip.height, &complete_from_height_one))
}

/// Live operator prune facts. Manual outcome stays none until Plan 05 stores one.
pub(in crate::dispatch) fn operator_prune_status<S, V>(
    context: &ManagedRpcContext<S, V>,
) -> Result<PruneOperatorStatus, RpcFailure>
where
    S: open_bitcoin_node::ChainstateStore,
    V: open_bitcoin_node::core::chainstate::CoinsView,
{
    let mode = context
        .prune_mode()
        .map_err(network_authority_error_to_failure)?;
    let projection = project_prune_status(mode, configured_last_pruned_height(context, mode)?);
    let pruneheight = if projection.pruned {
        FieldAvailability::available(projection.maybe_pruneheight)
    } else {
        FieldAvailability::available(None)
    };
    let locks = match context.load_prune_locks() {
        Ok(locks) => FieldAvailability::available(
            locks
                .into_iter()
                .map(|lock| PruneLockRow {
                    name: lock.name,
                    height_first: lock.height_first,
                    height_last: lock.height_last,
                })
                .collect(),
        ),
        Err(_) => FieldAvailability::unavailable(PRUNE_LOCKS_UNAVAILABLE),
    };
    let support_counts = match context.load_prune_support_counts() {
        Ok(counts) => FieldAvailability::available(counts),
        Err(_) => FieldAvailability::unavailable(PRUNE_SUPPORT_COUNTS_UNAVAILABLE),
    };
    Ok(PruneOperatorStatus::from_projection(
        projection,
        pruneheight,
        locks,
        FieldAvailability::available(ManualPruneSurface::None),
        support_counts,
    ))
}

fn block_payload_and_undo_present<S, V>(
    context: &ManagedRpcContext<S, V>,
    block_hash: BlockHash,
    snapshot: &ChainstateSnapshot,
) -> Result<bool, RpcFailure>
where
    S: open_bitcoin_node::ChainstateStore,
    V: open_bitcoin_node::core::chainstate::CoinsView,
{
    if !block_payload_present(context, block_hash)? {
        return Ok(false);
    }
    if snapshot.undo_by_block.contains_key(&block_hash) {
        return Ok(true);
    }
    context
        .durable_undo_present(block_hash)
        .map_err(|_| RpcFailure::internal_error(PRUNE_HEIGHT_PRESENCE_UNAVAILABLE))
}

fn block_payload_present<S, V>(
    context: &ManagedRpcContext<S, V>,
    block_hash: BlockHash,
) -> Result<bool, RpcFailure>
where
    S: open_bitcoin_node::ChainstateStore,
    V: open_bitcoin_node::core::chainstate::CoinsView,
{
    if context
        .cached_block_present(block_hash)
        .map_err(network_authority_error_to_failure)?
    {
        return Ok(true);
    }
    context
        .durable_block_present(block_hash)
        .map_err(|_| RpcFailure::internal_error(PRUNE_HEIGHT_PRESENCE_UNAVAILABLE))
}
