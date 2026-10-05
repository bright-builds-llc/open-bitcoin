// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/validation.h

//! Re-check planned heights against the active chain, locks, and keep window.

use open_bitcoin_core::{
    chainstate::{
        ChainPosition, IndexInputProtection, PruneLockInfo, PrunePlan,
        height_forbidden_by_any_lock, height_inside_keep_window,
    },
    primitives::BlockHash,
};

use super::FlushPersistSink;
use crate::storage::{StorageError, fjall_store::PairedDeleteOutcome};

/// Why one planned height was deleted, already gone, or left in place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HeightPruneOutcome {
    DeletedLiveMate { height: u32, block_hash: BlockHash },
    AlreadyAbsent { height: u32, block_hash: BlockHash },
    SkippedLock { height: u32 },
    SkippedKeepWindow { height: u32 },
    SkippedUnresolved { height: u32 },
}

/// Classification before any store call. The hash is an active-chain position only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ClassifiedHeight {
    Ready { height: u32, block_hash: BlockHash },
    SkippedLock { height: u32 },
    SkippedKeepWindow { height: u32 },
    SkippedUnresolved { height: u32 },
}

/// Resolves `height` to one active-chain hash, or a skip that does not fail the plan.
///
/// Locks win over the keep window. Tip height is the last active-chain position,
/// not the slice length. Header entries are not consulted.
pub(crate) fn classify_prune_height(
    height: u32,
    active_chain: &[ChainPosition],
    locks: &[PruneLockInfo],
) -> ClassifiedHeight {
    let Some(tip) = active_chain.last() else {
        return ClassifiedHeight::SkippedUnresolved { height };
    };
    if height_forbidden_by_any_lock(height, locks) {
        return ClassifiedHeight::SkippedLock { height };
    }
    if height_inside_keep_window(tip.height, height) {
        return ClassifiedHeight::SkippedKeepWindow { height };
    }
    let Some(position) = active_chain
        .iter()
        .find(|position| position.height == height)
    else {
        return ClassifiedHeight::SkippedUnresolved { height };
    };
    ClassifiedHeight::Ready {
        height,
        block_hash: position.block_hash,
    }
}

fn classify_protected_prune_height(
    height: u32,
    active_chain: &[ChainPosition],
    locks: &[PruneLockInfo],
    maybe_protection: Option<IndexInputProtection>,
) -> ClassifiedHeight {
    if maybe_protection.is_some_and(|protection| protection.check_prune_intent(height).is_err()) {
        return ClassifiedHeight::SkippedLock { height };
    }
    classify_prune_height(height, active_chain, locks)
}

pub(super) fn apply_prune_plan<S: FlushPersistSink>(
    sink: &mut S,
    plan: &PrunePlan,
    locks: &[PruneLockInfo],
    active_chain: &[ChainPosition],
    on_deleted: &mut dyn FnMut(BlockHash),
) -> Result<Vec<BlockHash>, StorageError> {
    let mut deleted_block_hashes = Vec::new();
    let mut deleted_heights = Vec::new();
    for height in &plan.heights {
        let outcome = unlink_classified_height(sink, *height, active_chain, locks)?;
        if let HeightPruneOutcome::DeletedLiveMate {
            height: deleted_height,
            block_hash,
        } = outcome
        {
            deleted_heights.push(deleted_height);
            deleted_block_hashes.push(block_hash);
            on_deleted(block_hash);
        }
    }
    sink.record_successful_prune_batch(&deleted_heights)?;
    Ok(deleted_block_hashes)
}

fn unlink_classified_height<S: FlushPersistSink>(
    sink: &mut S,
    height: u32,
    active_chain: &[ChainPosition],
    locks: &[PruneLockInfo],
) -> Result<HeightPruneOutcome, StorageError> {
    let current = sink.load_prune_protection()?;
    let maybe_protection = current
        .maybe_owner()
        .and_then(|owner| owner.maybe_effective_protection());
    let mut current_locks = current.locks().to_vec();
    current_locks.extend_from_slice(locks);
    let classified =
        classify_protected_prune_height(height, active_chain, &current_locks, maybe_protection);
    match classified {
        ClassifiedHeight::SkippedLock { height } => Ok(HeightPruneOutcome::SkippedLock { height }),
        ClassifiedHeight::SkippedKeepWindow { height } => {
            Ok(HeightPruneOutcome::SkippedKeepWindow { height })
        }
        ClassifiedHeight::SkippedUnresolved { height } => {
            Ok(HeightPruneOutcome::SkippedUnresolved { height })
        }
        ClassifiedHeight::Ready { height, block_hash } => {
            match sink.commit_paired_unlink(height, block_hash)? {
                PairedDeleteOutcome::DeletedLiveMate => {
                    Ok(HeightPruneOutcome::DeletedLiveMate { height, block_hash })
                }
                PairedDeleteOutcome::AlreadyAbsent => {
                    Ok(HeightPruneOutcome::AlreadyAbsent { height, block_hash })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
