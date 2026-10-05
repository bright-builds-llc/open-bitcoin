// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/validation.h

use super::{ClassifiedHeight, classify_prune_height};
use crate::chainstate::{FlushPersistSink, MemoryChainstateStore, PruneProtectionSnapshot};
use crate::storage::fjall_store::PairedDeleteOutcome;
use crate::storage::{StorageError, StorageNamespace, StorageRecoveryAction};
use open_bitcoin_core::chainstate::{BlockUndo, PrunePlan};
use open_bitcoin_core::primitives::Block;
use open_bitcoin_core::{
    chainstate::{ChainPosition, IndexInputProtection, PruneLockInfo},
    primitives::{BlockHash, BlockHeader, MerkleRoot},
};
use open_bitcoin_network::HeaderEntry;

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
            block_hash: expected_hash
        }
    );
    assert_ne!(expected_hash, side_chain_hash);
}

#[test]
fn required_genesis_and_height_one_skip_without_lock_or_window_confounders() {
    for height in [0, 1] {
        // Arrange
        let active = [position(height), position(400)];
        // Act
        let classified = super::classify_protected_prune_height(
            height,
            &active,
            &[],
            Some(IndexInputProtection::FromHeight(0)),
        );
        // Assert
        assert_eq!(classified, ClassifiedHeight::SkippedLock { height });
    }
}

struct UnsupportedSink;

impl FlushPersistSink for UnsupportedSink {
    fn persist_block(&mut self, _block: &Block) -> Result<(), StorageError> {
        Ok(())
    }
    fn persist_undo(&mut self, _hash: BlockHash, _undo: &BlockUndo) -> Result<(), StorageError> {
        Ok(())
    }
    fn persist_header_entries(&mut self, _entries: &[HeaderEntry]) -> Result<(), StorageError> {
        Ok(())
    }
    fn persist_chain_meta(&mut self, _chain: &[ChainPosition]) -> Result<(), StorageError> {
        Ok(())
    }
}

#[test]
fn unsupported_snapshot_is_refusal_not_no_index_authority() {
    // Arrange
    let mut sink = UnsupportedSink;
    let mut deleted = Vec::new();

    // Act
    let result = super::apply_prune_plan(
        &mut sink,
        &PrunePlan { heights: vec![1] },
        &[],
        &[position(1), position(400)],
        &mut |hash| deleted.push(hash),
    );

    // Assert
    assert!(matches!(
        result,
        Err(StorageError::UnavailableNamespace {
            namespace: StorageNamespace::BlockIndex
        })
    ));
    assert!(deleted.is_empty());
}

#[test]
fn explicit_no_index_already_absent_never_earns_deletion_cleanup() {
    // Arrange
    let mut sink = MemoryChainstateStore::default();
    let mut deleted = Vec::new();

    // Act
    let result = super::apply_prune_plan(
        &mut sink,
        &PrunePlan { heights: vec![1] },
        &[],
        &[position(1), position(400)],
        &mut |hash| deleted.push(hash),
    );

    // Assert
    assert!(result.expect("explicit no-index").is_empty());
    assert!(deleted.is_empty());
    assert!(
        sink.load_prune_protection()
            .expect("explicit contract")
            .maybe_owner()
            .is_none()
    );
}

struct ReceiptSink {
    recorded: Vec<u32>,
    calls: Vec<u32>,
}

impl FlushPersistSink for ReceiptSink {
    fn persist_block(&mut self, _block: &Block) -> Result<(), StorageError> {
        Ok(())
    }
    fn persist_undo(&mut self, _hash: BlockHash, _undo: &BlockUndo) -> Result<(), StorageError> {
        Ok(())
    }
    fn persist_header_entries(&mut self, _entries: &[HeaderEntry]) -> Result<(), StorageError> {
        Ok(())
    }
    fn persist_chain_meta(&mut self, _chain: &[ChainPosition]) -> Result<(), StorageError> {
        Ok(())
    }
    fn load_prune_protection(&self) -> Result<PruneProtectionSnapshot, StorageError> {
        Ok(PruneProtectionSnapshot::no_index(Vec::new()))
    }
    fn commit_paired_unlink(
        &mut self,
        height: u32,
        _hash: BlockHash,
    ) -> Result<PairedDeleteOutcome, StorageError> {
        self.calls.push(height);
        if height == 2 {
            return Err(StorageError::Corruption {
                namespace: StorageNamespace::BlockIndex,
                detail: "later candidate failed".to_owned(),
                action: StorageRecoveryAction::Repair,
            });
        }
        Ok(PairedDeleteOutcome::DeletedLiveMate)
    }
    fn record_successful_prune_batch(&mut self, heights: &[u32]) -> Result<(), StorageError> {
        self.recorded.extend_from_slice(heights);
        Ok(())
    }
}

#[test]
fn later_candidate_failure_preserves_only_cleanup_earned_by_live_receipt() {
    // Arrange
    let mut sink = ReceiptSink {
        recorded: Vec::new(),
        calls: Vec::new(),
    };
    let eligible = position(1);
    let expected = eligible.block_hash;
    let mut deleted = Vec::new();

    // Act
    let result = super::apply_prune_plan(
        &mut sink,
        &PrunePlan {
            heights: vec![1, 2],
        },
        &[],
        &[eligible, position(2), position(400)],
        &mut |hash| deleted.push(hash),
    );

    // Assert
    assert!(result.is_err());
    assert_eq!(deleted, vec![expected]);
    assert_eq!(sink.calls, vec![1, 2]);
    assert!(
        sink.recorded.is_empty(),
        "existing partial-batch summary advisory remains unchanged"
    );
}

#[test]
fn caller_restrictions_only_strengthen_fresh_no_index_snapshot() {
    // Arrange
    let mut sink = ReceiptSink {
        recorded: Vec::new(),
        calls: Vec::new(),
    };
    let locks = [PruneLockInfo {
        name: "caller-only".to_owned(),
        height_first: 20,
        height_last: 20,
    }];

    // Act
    let result = super::apply_prune_plan(
        &mut sink,
        &PrunePlan { heights: vec![20] },
        &locks,
        &[position(20), position(400)],
        &mut |_| panic!("protected height cannot earn cleanup"),
    );

    // Assert
    assert!(result.expect("caller lock skip").is_empty());
    assert!(sink.calls.is_empty());
}
