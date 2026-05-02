// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::collections::BTreeSet;

use open_bitcoin_core::{
    chainstate::AnchoredBlock,
    consensus::block_hash,
    primitives::{BlockHash, InventoryType},
};
use open_bitcoin_network::{PeerId, WireNetworkMessage};

use super::{DurableSyncRuntime, SyncRuntimeError};
use crate::{StorageNamespace, StorageRecoveryAction};

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

pub(super) fn reconcile_best_chain(
    runtime: &mut DurableSyncRuntime,
    timestamp: i64,
) -> Result<bool, SyncRuntimeError> {
    let active_chain = runtime.network.chainstate_snapshot().active_chain;
    let best_chain = runtime.network.best_chain_entries();
    if best_chain.is_empty() {
        return Ok(false);
    }

    let mut common_prefix_len = 0_usize;
    while common_prefix_len < active_chain.len()
        && common_prefix_len < best_chain.len()
        && active_chain[common_prefix_len].block_hash == best_chain[common_prefix_len].block_hash
    {
        common_prefix_len += 1;
    }

    if common_prefix_len == active_chain.len() {
        let mut progressed = false;
        for entry in best_chain.iter().skip(common_prefix_len) {
            let Some(block) = runtime.store.load_block(entry.block_hash)? else {
                break;
            };
            runtime.network.note_local_block_hash(entry.block_hash);
            let connected = runtime.network.connect_stored_block(
                &block,
                entry.chain_work,
                timestamp,
                runtime.verify_flags,
                runtime.consensus_params,
            )?;
            if connected.is_some() {
                progressed = true;
                continue;
            }
            break;
        }
        return Ok(progressed);
    }

    let mut replacement_branch = Vec::new();
    for entry in best_chain.iter().skip(common_prefix_len) {
        let Some(block) = runtime.store.load_block(entry.block_hash)? else {
            break;
        };
        runtime.network.note_local_block_hash(entry.block_hash);
        replacement_branch.push(AnchoredBlock {
            block,
            chain_work: entry.chain_work,
        });
    }
    if replacement_branch.is_empty() {
        return Ok(false);
    }

    let Some(current_tip) = active_chain.last() else {
        return Ok(false);
    };
    let candidate_entry = &best_chain[common_prefix_len + replacement_branch.len() - 1];
    let candidate_outranks = candidate_entry.chain_work > current_tip.chain_work
        || (candidate_entry.chain_work == current_tip.chain_work
            && (candidate_entry.height > current_tip.height
                || (candidate_entry.height == current_tip.height
                    && candidate_entry.block_hash > current_tip.block_hash)));
    if !candidate_outranks {
        return Ok(false);
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

    runtime.network.reorg_to_branch(
        &disconnect_blocks,
        &replacement_branch,
        runtime.verify_flags,
        runtime.consensus_params,
    )?;
    Ok(true)
}
