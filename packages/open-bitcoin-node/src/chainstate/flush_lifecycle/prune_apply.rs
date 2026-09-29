// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/validation.h

//! Re-check planned heights against the active chain, locks, and keep window.

use open_bitcoin_core::{
    chainstate::{
        ChainPosition, PruneLockInfo, PrunePlan, height_forbidden_by_any_lock,
        height_inside_keep_window,
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
    let classified = classify_prune_height(height, active_chain, locks);
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
mod tests {
    use open_bitcoin_core::{
        chainstate::{ChainPosition, PruneLockInfo},
        primitives::{BlockHash, BlockHeader, MerkleRoot},
    };

    use super::{ClassifiedHeight, classify_prune_height};

    fn position(height: u32) -> ChainPosition {
        let header = BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0; 32]),
            merkle_root: MerkleRoot::from_byte_array([height as u8; 32]),
            time: 1_700_000_000,
            bits: 0x207f_ffff,
            nonce: height,
        };
        ChainPosition::new(header, height, u128::from(height), 1)
    }

    #[test]
    fn empty_chain_is_skipped_unresolved() {
        // Arrange
        let locks = [];

        // Act
        let classified = classify_prune_height(4, &[], &locks);

        // Assert
        assert_eq!(
            classified,
            ClassifiedHeight::SkippedUnresolved { height: 4 }
        );
    }

    #[test]
    fn lock_wins_over_keep_window() {
        // Arrange
        let tip = position(10);
        let locks = [PruneLockInfo {
            name: "rescan".to_string(),
            height_first: 10,
            height_last: 10,
        }];

        // Act
        let classified = classify_prune_height(10, &[tip], &locks);

        // Assert
        assert_eq!(classified, ClassifiedHeight::SkippedLock { height: 10 });
    }

    #[test]
    fn tip_height_is_inside_the_keep_window() {
        // Arrange
        let older = position(1);
        let tip = position(10);

        // Act
        let classified = classify_prune_height(10, &[older, tip], &[]);

        // Assert
        assert_eq!(
            classified,
            ClassifiedHeight::SkippedKeepWindow { height: 10 }
        );
    }

    #[test]
    fn ready_uses_the_active_chain_hash() {
        // Arrange
        let eligible = position(1);
        let tip = position(400);
        let expected_hash = eligible.block_hash;
        let side_chain_hash = BlockHash::from_byte_array([0xab; 32]);

        // Act
        let classified = classify_prune_height(1, &[eligible, tip], &[]);

        // Assert
        assert_eq!(
            classified,
            ClassifiedHeight::Ready {
                height: 1,
                block_hash: expected_hash,
            }
        );
        assert_ne!(expected_hash, side_chain_hash);
    }
}
