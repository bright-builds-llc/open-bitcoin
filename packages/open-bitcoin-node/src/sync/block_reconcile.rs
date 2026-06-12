// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::collections::BTreeSet;

use open_bitcoin_core::{
    chainstate::{AnchoredBlock, ChainPosition, ChainstateError},
    consensus::block_hash,
    primitives::{BlockHash, InventoryType},
};
use open_bitcoin_network::{PeerId, WireNetworkMessage};

use super::{DurableSyncRuntime, SyncRuntimeError, tip, types::SyncReconcileProgress};
use crate::{
    ManagedNetworkError, StorageError, StorageNamespace, StorageRecoveryAction,
    network::BlockConnectDisposition,
};

pub(super) fn validate_block_limits(runtime: &DurableSyncRuntime) -> Result<(), SyncRuntimeError> {
    if runtime.config.max_blocks_in_flight_per_peer == 0 {
        return Err(SyncRuntimeError::ResourceLimit {
            message: "max_blocks_in_flight_per_peer is 0; increase the per-peer block budget to continue sync".to_string(),
        });
    }
    if runtime.config.max_blocks_in_flight_total == 0 {
        return Err(SyncRuntimeError::ResourceLimit {
            message:
                "max_blocks_in_flight_total is 0; increase the global block budget to continue sync"
                    .to_string(),
        });
    }
    Ok(())
}

pub(super) fn request_missing_blocks(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
    let available_global = runtime
        .config
        .max_blocks_in_flight_total
        .saturating_sub(runtime.inflight_blocks.len());
    if available_global == 0 {
        return Ok(Vec::new());
    }

    let active_chain_hashes = runtime
        .network
        .chainstate_snapshot()
        .active_chain
        .into_iter()
        .map(|position| position.block_hash)
        .collect::<BTreeSet<_>>();
    let mut requested = Vec::new();
    for entry in runtime.network.best_chain_entries() {
        if requested.len() >= available_global
            || active_chain_hashes.contains(&entry.block_hash)
            || runtime.inflight_blocks.contains(&entry.block_hash)
        {
            continue;
        }
        if runtime.store.load_block(entry.block_hash)?.is_some() {
            runtime.network.note_local_block_hash(entry.block_hash);
            continue;
        }
        requested.push(entry.block_hash);
    }

    let outbound = runtime
        .network
        .request_missing_blocks(peer_id, &requested)?;
    for message in &outbound {
        if let WireNetworkMessage::GetData(inventory) = message {
            for item in &inventory.inventory {
                if matches!(
                    item.inventory_type,
                    InventoryType::Block | InventoryType::WitnessBlock
                ) {
                    runtime
                        .inflight_blocks
                        .insert(BlockHash::from(item.object_hash));
                }
            }
        }
    }
    Ok(outbound)
}

pub(super) fn release_inflight_for_message(
    runtime: &mut DurableSyncRuntime,
    message: &WireNetworkMessage,
) {
    match message {
        WireNetworkMessage::Block(block) => {
            runtime.inflight_blocks.remove(&block_hash(&block.header));
        }
        WireNetworkMessage::NotFound(inventory) => {
            for item in &inventory.inventory {
                if matches!(
                    item.inventory_type,
                    InventoryType::Block | InventoryType::WitnessBlock
                ) {
                    runtime
                        .inflight_blocks
                        .remove(&BlockHash::from(item.object_hash));
                }
            }
        }
        _ => {}
    }
}

pub(super) fn reconcile_and_persist_best_chain(
    runtime: &mut DurableSyncRuntime,
    timestamp: i64,
) -> Result<SyncReconcileProgress, SyncRuntimeError> {
    let progress = reconcile_best_chain(runtime, timestamp)?;
    if progress.should_persist_progress() {
        runtime.persist_progress()?;
    }
    runtime.record_reconcile_progress(progress.clone());
    Ok(progress)
}

pub(super) fn reconcile_best_chain(
    runtime: &mut DurableSyncRuntime,
    timestamp: i64,
) -> Result<SyncReconcileProgress, SyncRuntimeError> {
    let active_chain = runtime.network.chainstate_snapshot().active_chain;
    let best_chain = runtime.network.best_chain_entries();
    if best_chain.is_empty() {
        return Ok(SyncReconcileProgress::NoChange);
    }

    let mut common_prefix_len = 0_usize;
    while common_prefix_len < active_chain.len()
        && common_prefix_len < best_chain.len()
        && active_chain[common_prefix_len].block_hash == best_chain[common_prefix_len].block_hash
    {
        common_prefix_len += 1;
    }

    if common_prefix_len == active_chain.len() {
        let mut connected_count = 0_u64;
        for entry in best_chain.iter().skip(common_prefix_len) {
            let Some(block) = runtime.store.load_block(entry.block_hash)? else {
                break;
            };
            runtime.network.note_local_block_hash(entry.block_hash);
            let disposition = runtime.network.connect_stored_block(
                &block,
                entry.chain_work,
                timestamp,
                runtime.verify_flags,
                runtime.consensus_params,
            )?;
            if matches!(disposition, BlockConnectDisposition::Connected(_)) {
                connected_count = connected_count.saturating_add(1);
                continue;
            }
            break;
        }
        if connected_count > 0 {
            return Ok(SyncReconcileProgress::ExtendedActiveChain { connected_count });
        }
        return Ok(SyncReconcileProgress::NoChange);
    }

    let mut replacement_branch = Vec::new();
    let mut maybe_first_missing = None;
    let mut missing_count = 0_u64;
    for entry in best_chain.iter().skip(common_prefix_len) {
        let Some(block) = runtime.store.load_block(entry.block_hash)? else {
            missing_count = missing_count.saturating_add(1);
            if maybe_first_missing.is_none() {
                maybe_first_missing = Some((entry.height, entry.block_hash));
            }
            continue;
        };
        runtime.network.note_local_block_hash(entry.block_hash);
        replacement_branch.push(AnchoredBlock {
            block,
            chain_work: entry.chain_work,
        });
    }
    if let Some((first_missing_height, first_missing_hash)) = maybe_first_missing {
        return Ok(SyncReconcileProgress::BranchCompetitionAwaitingBodies {
            missing_count,
            first_missing_height: u64::from(first_missing_height),
            first_missing_hash: tip::block_hash_hex(first_missing_hash),
        });
    }
    if replacement_branch.is_empty() {
        return Ok(SyncReconcileProgress::NoChange);
    }

    let Some(current_tip) = active_chain.last() else {
        return Ok(SyncReconcileProgress::NoChange);
    };
    let candidate_entry = &best_chain[common_prefix_len + replacement_branch.len() - 1];
    let candidate_outranks = candidate_entry.chain_work > current_tip.chain_work
        || (candidate_entry.chain_work == current_tip.chain_work
            && (candidate_entry.height > current_tip.height
                || (candidate_entry.height == current_tip.height
                    && candidate_entry.block_hash > current_tip.block_hash)));
    if !candidate_outranks {
        return Ok(SyncReconcileProgress::SideBranchPreserved);
    }

    let mut disconnect_blocks = Vec::new();
    for position in active_chain.iter().skip(common_prefix_len).rev() {
        let Some(block) = runtime.store.load_block(position.block_hash)? else {
            return Err(SyncRuntimeError::Storage(crate::StorageError::Corruption {
                namespace: StorageNamespace::BlockIndex,
                detail: format!(
                    "missing durable block body for active chain block {:?}",
                    position.block_hash
                ),
                action: StorageRecoveryAction::Repair,
            }));
        };
        disconnect_blocks.push(block);
    }

    let transition = runtime
        .network
        .reorg_to_branch(
            &disconnect_blocks,
            &replacement_branch,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .map_err(reorg_runtime_error)?;
    let common_ancestor = active_chain
        .get(common_prefix_len.saturating_sub(1))
        .or_else(|| active_chain.first());
    let Some(common_ancestor) = common_ancestor else {
        return Ok(SyncReconcileProgress::NoChange);
    };
    let Some(final_active_tip) = runtime.network.maybe_chain_tip() else {
        return Ok(SyncReconcileProgress::NoChange);
    };

    Ok(SyncReconcileProgress::ReorgPersisted(reorg_evidence(
        common_ancestor,
        &transition,
        &final_active_tip,
    )))
}

fn reorg_runtime_error(error: ManagedNetworkError) -> SyncRuntimeError {
    match error {
        ManagedNetworkError::Chainstate(ChainstateError::MissingUndo { block_hash }) => {
            SyncRuntimeError::Storage(StorageError::Corruption {
                namespace: StorageNamespace::Chainstate,
                detail: format!("missing undo data for active chain block {block_hash:?}"),
                action: StorageRecoveryAction::Repair,
            })
        }
        other => other.into(),
    }
}

fn reorg_evidence(
    common_ancestor: &ChainPosition,
    transition: &open_bitcoin_core::chainstate::ChainTransition,
    final_active_tip: &ChainPosition,
) -> crate::status::SyncReorgEvidence {
    crate::status::SyncReorgEvidence {
        common_ancestor_height: u64::from(common_ancestor.height),
        common_ancestor_hash: tip::block_hash_hex(common_ancestor.block_hash),
        disconnected_count: transition.disconnected.len() as u64,
        connected_count: transition.connected.len() as u64,
        final_active_height: u64::from(final_active_tip.height),
        final_active_hash: tip::block_hash_hex(final_active_tip.block_hash),
        fully_persisted: true,
    }
}
