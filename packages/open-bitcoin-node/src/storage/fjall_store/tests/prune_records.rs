// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h

use std::path::{Path, PathBuf};

use open_bitcoin_core::chainstate::PruneLockInfo;

use super::super::prune::{PRUNE_LOCKS_KEY, PRUNE_SUMMARY_KEY};
use super::*;

fn open_store(test_name: &str) -> (PathBuf, FjallNodeStore) {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    (path, store)
}

fn lock(name: &str, height_first: u32, height_last: u32) -> PruneLockInfo {
    PruneLockInfo {
        name: name.to_owned(),
        height_first,
        height_last,
    }
}

fn block_index_has_key(store: &FjallNodeStore, key: &str) -> bool {
    store
        .block_index
        .contains_key(key)
        .expect("block index contains")
}

fn tree_contains_component(root: &Path, name: &str) -> bool {
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            if entry.file_name() == name {
                return true;
            }
            if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                pending.push(entry.path());
            }
        }
    }
    false
}

#[test]
fn absent_keys_load_as_empty_locks_and_a_zero_summary() {
    // Arrange
    let (path, store) = open_store("prune-records-absent");

    // Act
    let locks = store.load_prune_locks().expect("load locks");
    let summary = store.load_prune_support_summary().expect("load summary");

    // Assert
    assert!(locks.is_empty());
    assert_eq!(summary.successful_batch_count, 0);
    assert_eq!(summary.pruned_height_count, 0);
    assert_eq!(summary.maybe_last_prune_height, None);
    assert!(!block_index_has_key(&store, PRUNE_LOCKS_KEY));
    assert!(!block_index_has_key(&store, PRUNE_SUMMARY_KEY));
    remove_dir_if_exists(&path);
}

#[test]
fn synced_locks_reopen_with_the_same_inclusive_heights() {
    // Arrange
    let (path, store) = open_store("prune-records-reopen");
    let locks = vec![lock("wallet", 20, 30), lock("rescans", 40, 50)];
    store.sync_prune_locks(&locks).expect("sync locks");
    drop(store);

    // Act
    let reopened = FjallNodeStore::open(&path).expect("reopen");
    let loaded = reopened.load_prune_locks().expect("load locks");

    // Assert
    assert_eq!(loaded, locks);
    assert!(!block_index_has_key(&reopened, "wallet"));
    assert!(!block_index_has_key(&reopened, "rescans"));
    assert!(!tree_contains_component(&path, "wallet"));
    assert!(!tree_contains_component(&path, "rescans"));
    remove_dir_if_exists(&path);
}

#[test]
fn writing_the_same_name_replaces_only_that_range() {
    // Arrange
    let (path, store) = open_store("prune-records-replace");
    store
        .sync_prune_locks(&[lock("wallet", 20, 30), lock("rescans", 40, 50)])
        .expect("sync locks");

    // Act
    store
        .sync_prune_locks(&[lock("wallet", 21, 31), lock("rescans", 40, 50)])
        .expect("replace wallet");
    let loaded = store.load_prune_locks().expect("load locks");

    // Assert
    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[0], lock("wallet", 21, 31));
    assert_eq!(loaded[1], lock("rescans", 40, 50));
    remove_dir_if_exists(&path);
}

#[test]
fn removing_one_name_leaves_the_other_and_empty_sync_clears_the_map() {
    // Arrange
    let (path, store) = open_store("prune-records-clear");
    store
        .sync_prune_locks(&[lock("wallet", 20, 30), lock("rescans", 40, 50)])
        .expect("sync locks");

    // Act
    store
        .sync_prune_locks(&[lock("rescans", 40, 50)])
        .expect("drop wallet");
    let remaining = store.load_prune_locks().expect("load remaining");
    store.sync_prune_locks(&[]).expect("sync empty");
    let cleared = store.load_prune_locks().expect("load cleared");

    // Assert
    assert_eq!(remaining, vec![lock("rescans", 40, 50)]);
    assert!(cleared.is_empty());
    assert!(block_index_has_key(&store, PRUNE_LOCKS_KEY));
    remove_dir_if_exists(&path);
}

#[test]
fn empty_successful_batch_does_not_create_the_summary_key() {
    // Arrange
    let (path, store) = open_store("prune-records-empty-batch");

    // Act
    store
        .record_successful_prune_batch(&[])
        .expect("empty batch");

    // Assert
    assert!(!block_index_has_key(&store, PRUNE_SUMMARY_KEY));
    assert!(!store.load_have_pruned().expect("have_pruned"));
    let summary = store.load_prune_support_summary().expect("load summary");
    assert_eq!(summary.successful_batch_count, 0);
    assert_eq!(summary.maybe_last_prune_height, None);
    remove_dir_if_exists(&path);
}

#[test]
fn recorded_batches_count_heights_and_keep_only_this_batch_max() {
    // Arrange
    let (path, store) = open_store("prune-records-batches");

    // Act
    store
        .record_successful_prune_batch(&[4, 9])
        .expect("first batch");
    store
        .record_successful_prune_batch(&[2])
        .expect("second batch");
    let summary = store.load_prune_support_summary().expect("load summary");

    // Assert
    assert_eq!(summary.successful_batch_count, 2);
    assert_eq!(summary.pruned_height_count, 3);
    assert_eq!(summary.maybe_last_prune_height, Some(2));
    assert!(!store.load_have_pruned().expect("have_pruned"));
    remove_dir_if_exists(&path);
}

#[test]
fn lock_name_longer_than_1024_bytes_is_refused_on_write() {
    // Arrange
    let (path, store) = open_store("prune-records-long-name");
    let long_name = "a".repeat(1025);

    // Act
    let error = store
        .sync_prune_locks(&[lock(&long_name, 1, 2)])
        .expect_err("long name is refused");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::BlockIndex,
            action: StorageRecoveryAction::Repair,
            ..
        }
    ));
    assert!(!block_index_has_key(&store, PRUNE_LOCKS_KEY));
    assert!(store.load_prune_locks().expect("load locks").is_empty());
    remove_dir_if_exists(&path);
}

#[test]
fn lock_name_longer_than_1024_bytes_is_corruption_on_load() {
    // Arrange
    let (path, store) = open_store("prune-records-long-name-load");
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes.extend_from_slice(&1025_u16.to_le_bytes());
    bytes.extend(std::iter::repeat_n(b'a', 1025));
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes.extend_from_slice(&2_u32.to_le_bytes());
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, PRUNE_LOCKS_KEY, bytes)
        .expect("plant long name");

    // Act
    let error = store.load_prune_locks().expect_err("long name is corrupt");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::BlockIndex,
            action: StorageRecoveryAction::Repair,
            ..
        }
    ));
    remove_dir_if_exists(&path);
}

#[test]
fn zero_batch_count_with_a_last_height_is_corruption() {
    // Arrange
    let (path, store) = open_store("prune-records-zero-batch-flag");
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&0_u64.to_le_bytes());
    bytes.extend_from_slice(&0_u64.to_le_bytes());
    bytes.push(1);
    bytes.extend_from_slice(&4_u32.to_le_bytes());
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, PRUNE_SUMMARY_KEY, bytes)
        .expect("plant summary");

    // Act
    let error = store
        .load_prune_support_summary()
        .expect_err("zero batch with height is corrupt");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::BlockIndex,
            ..
        }
    ));
    remove_dir_if_exists(&path);
}
