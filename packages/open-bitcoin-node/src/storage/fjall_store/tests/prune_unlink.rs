// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/validation.h

use std::path::PathBuf;

use open_bitcoin_core::{
    chainstate::{BlockUndo, PrunePlan},
    primitives::BlockHash,
};

use super::super::PairedDeleteOutcome;
use super::super::prune::{PRUNE_INTENT_KEY, PruneIntent};
use super::*;
use crate::storage::coins_codec::{encode_best_block_key, encode_best_block_value};
use crate::storage::coins_view::FjallCoinsView;

fn open_store(test_name: &str) -> (PathBuf, FjallNodeStore) {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    (path, store)
}

fn plant_block(store: &FjallNodeStore, nonce: u32) -> BlockHash {
    let fixture = block(BlockHash::from_byte_array([0_u8; 32]), nonce);
    store
        .save_block(&fixture, PersistMode::Sync)
        .expect("save block")
}

fn plant_undo(store: &FjallNodeStore, block_hash: BlockHash) {
    store
        .save_undo(block_hash, &BlockUndo::default(), PersistMode::Sync)
        .expect("save undo");
}

#[test]
fn paired_delete_removes_both_mates_and_records_have_pruned() {
    // Arrange
    let (path, store) = open_store("paired-delete-both");
    let block_hash = plant_block(&store, 3);
    plant_undo(&store, block_hash);

    // Act
    let outcome = store
        .commit_paired_delete(100, block_hash)
        .expect("paired delete");

    // Assert
    assert_eq!(outcome, PairedDeleteOutcome::DeletedLiveMate);
    assert!(!store.has_block(block_hash).expect("has_block"));
    assert!(!store.has_undo(block_hash).expect("has_undo"));
    assert!(store.load_have_pruned().expect("have_pruned"));
    assert_eq!(store.maybe_prune_intent().expect("intent"), None);
    remove_dir_if_exists(&path);
}

#[test]
fn reopen_keeps_have_pruned_and_absent_mates() {
    // Arrange
    let (path, store) = open_store("paired-delete-reopen");
    let block_hash = plant_block(&store, 4);
    plant_undo(&store, block_hash);
    store
        .commit_paired_delete(101, block_hash)
        .expect("paired delete");
    drop(store);

    // Act
    let reopened = FjallNodeStore::open(&path).expect("reopen");

    // Assert
    assert!(reopened.load_have_pruned().expect("have_pruned"));
    assert!(!reopened.has_block(block_hash).expect("has_block"));
    assert!(!reopened.has_undo(block_hash).expect("has_undo"));
    remove_dir_if_exists(&path);
}

#[test]
fn paired_delete_removes_only_the_undo_when_the_block_is_absent() {
    // Arrange
    let (path, store) = open_store("paired-delete-undo-only");
    let block_hash = BlockHash::from_byte_array([0x11; 32]);
    plant_undo(&store, block_hash);

    // Act
    let outcome = store
        .commit_paired_delete(102, block_hash)
        .expect("paired delete");

    // Assert
    assert_eq!(outcome, PairedDeleteOutcome::DeletedLiveMate);
    assert!(!store.has_block(block_hash).expect("has_block"));
    assert!(!store.has_undo(block_hash).expect("has_undo"));
    assert!(store.load_have_pruned().expect("have_pruned"));
    remove_dir_if_exists(&path);
}

#[test]
fn paired_delete_removes_only_the_block_when_the_undo_is_absent() {
    // Arrange
    let (path, store) = open_store("paired-delete-block-only");
    let block_hash = plant_block(&store, 5);

    // Act
    let outcome = store
        .commit_paired_delete(103, block_hash)
        .expect("paired delete");

    // Assert
    assert_eq!(outcome, PairedDeleteOutcome::DeletedLiveMate);
    assert!(!store.has_block(block_hash).expect("has_block"));
    assert!(!store.has_undo(block_hash).expect("has_undo"));
    assert!(store.load_have_pruned().expect("have_pruned"));
    remove_dir_if_exists(&path);
}

#[test]
fn both_absent_mates_return_already_absent_without_have_pruned() {
    // Arrange
    let (path, store) = open_store("paired-delete-absent");
    let block_hash = BlockHash::from_byte_array([0x22; 32]);

    // Act
    let outcome = store
        .commit_paired_delete(104, block_hash)
        .expect("paired delete");

    // Assert
    assert_eq!(outcome, PairedDeleteOutcome::AlreadyAbsent);
    assert!(!store.load_have_pruned().expect("have_pruned"));
    assert_eq!(store.maybe_prune_intent().expect("intent"), None);
    remove_dir_if_exists(&path);
}

#[test]
fn have_pruned_stays_false_without_a_paired_delete() {
    // Arrange
    let (path, store) = open_store("have-pruned-without-delete");
    let plan = PrunePlan {
        heights: vec![700, 701],
    };

    // Act
    let have_pruned = store.load_have_pruned().expect("have_pruned");

    // Assert
    assert!(!plan.heights.is_empty());
    assert!(!have_pruned);
    assert!(
        !include_str!("../prune.rs").contains("PruneMode"),
        "the delete store must not read prune mode"
    );
    remove_dir_if_exists(&path);
}

#[test]
fn paired_delete_leaves_coins_best_block_bytes_unchanged() {
    // Arrange
    let (path, store) = open_store("paired-delete-best-block");
    let block_hash = plant_block(&store, 6);
    plant_undo(&store, block_hash);
    let best_hash = BlockHash::from_byte_array([0x33; 32]);
    let best_value = encode_best_block_value(best_hash);
    FjallCoinsView::from_store(&store)
        .write_raw_bytes(&encode_best_block_key(), best_value.clone())
        .expect("plant best block");

    // Act
    store
        .commit_paired_delete(105, block_hash)
        .expect("paired delete");

    // Assert
    let key = encode_best_block_key();
    let key_text = std::str::from_utf8(&key).expect("best-block key is utf-8");
    let after = store
        .get_bytes(StorageNamespace::Coins, key_text)
        .expect("read best block");
    assert_eq!(after.as_deref(), Some(best_value.as_slice()));
    remove_dir_if_exists(&path);
}

#[test]
fn has_undo_probes_contains_key_without_decoding() {
    // Arrange
    let (path, store) = open_store("has-undo-probe");
    let present_hash = BlockHash::from_byte_array([0x44; 32]);
    let missing_hash = BlockHash::from_byte_array([0x55; 32]);
    plant_undo(&store, present_hash);
    let source = include_str!("../coins.rs");
    let start = source.find("fn has_undo").expect("has_undo");
    let body = &source[start..];
    let end = body.find("\n    pub fn ").expect("method after has_undo");
    let function = &body[..end];

    // Act
    let present = store.has_undo(present_hash).expect("present undo");
    let missing = store.has_undo(missing_hash).expect("missing undo");

    // Assert
    assert!(present);
    assert!(!missing);
    assert!(function.contains("contains_key"));
    assert!(!function.contains("load_undo"));
    assert!(!function.contains("decode_block_undo"));
    remove_dir_if_exists(&path);
}

#[test]
fn have_pruned_is_inserted_only_in_the_tombstone_batch() {
    // Arrange
    let source = include_str!("../prune.rs");

    // Act
    let inserting: Vec<&str> = source
        .split("fn ")
        .skip(1)
        .filter(|body| body.contains("HAVE_PRUNED_KEY") && body.contains(".insert("))
        .collect();

    // Assert
    assert_eq!(inserting.len(), 1);
    assert!(inserting[0].contains("block_key"));
    assert!(inserting[0].contains("undo_key"));
    assert!(inserting[0].contains(".remove("));
    assert!(!source.contains("remove_bytes"));
    assert!(!source.contains("save_block"));
    assert!(!source.contains("save_undo"));
    assert!(!source.contains("encode_best_block_key"));
    assert!(!source.contains("save_chainstate_snapshot"));
    assert!(!source.contains("RuntimeMetadata"));
    assert!(!source.contains("prefix("));
}

#[test]
fn sync_prune_intent_does_not_insert_have_pruned() {
    // Arrange
    let (path, store) = open_store("prune-intent-only");
    let block_hash = BlockHash::from_byte_array([0x66; 32]);
    let intent = PruneIntent {
        height: 106,
        block_hash,
    };

    // Act
    store.sync_prune_intent(intent).expect("sync intent");

    // Assert
    assert!(!store.load_have_pruned().expect("have_pruned"));
    assert_eq!(store.maybe_prune_intent().expect("intent"), Some(intent));
    remove_dir_if_exists(&path);
}

#[test]
fn truncated_prune_intent_is_corruption_with_repair() {
    // Arrange
    let (path, store) = open_store("prune-intent-truncated");
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            PRUNE_INTENT_KEY,
            vec![1, 2, 3, 4],
        )
        .expect("write short intent");

    // Act
    let error = store
        .maybe_prune_intent()
        .expect_err("short intent is corrupt");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::BlockIndex,
            action: StorageRecoveryAction::Repair,
            ..
        }
    ));
    assert_eq!(error.recovery_action(), Some(StorageRecoveryAction::Repair));
    assert_ne!(
        error.recovery_action(),
        Some(StorageRecoveryAction::Reindex)
    );
    remove_dir_if_exists(&path);
}
