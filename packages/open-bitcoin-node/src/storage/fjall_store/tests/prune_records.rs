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

fn index_fixture(store: &FjallNodeStore) -> Vec<ChainPosition> {
    let mut genesis = block(BlockHash::default(), 0);
    let mut coinbase = mempool_transaction(0);
    coinbase.inputs[0].previous_output = OutPoint::null();
    genesis.transactions.push(coinbase);
    genesis.header.merkle_root =
        open_bitcoin_core::consensus::block_merkle_root(&genesis.transactions)
            .expect("merkle")
            .0;
    let position = ChainPosition::new(genesis.header.clone(), 0, 1, i64::from(genesis.header.time));
    let snapshot = ChainstateSnapshot::new(vec![position], Default::default(), Default::default());
    store.seed_coins_from_snapshot(&snapshot).expect("coins");
    store
        .save_block(&genesis, PersistMode::Sync)
        .expect("genesis body");
    let fence = open_bitcoin_core::chainstate::VerifiedChainstateFence::new(
        Some(snapshot.active_chain[0].block_hash),
        Some(&snapshot.active_chain),
    )
    .expect("fence");
    store
        .enable_basic_filter_index(&fence)
        .expect("internal lifecycle");
    snapshot.active_chain
}

fn publish_genesis(store: &FjallNodeStore, positions: &[ChainPosition], first: u32) {
    use crate::storage::filter_index::StoredFilterRecord;
    use open_bitcoin_core::chainstate::{
        BasicFilterInputs, FilterCheckpoint, IndexInputProtection, IndexPrefix,
        VerifiedChainstateFence,
    };
    let genesis = store
        .load_block(positions[0].block_hash)
        .expect("body")
        .expect("genesis");
    let inputs = BasicFilterInputs::from_historical(&genesis, &positions[0], None).expect("inputs");
    let record = StoredFilterRecord::generate(&inputs, &positions[0], None).expect("record");
    let fence = VerifiedChainstateFence::new(Some(positions[0].block_hash), Some(positions))
        .expect("fence");
    let work = store
        .maybe_basic_filter_work(&fence)
        .expect("work")
        .expect("active");
    store
        .publish_basic_filter_checkpoint(
            &work,
            &fence,
            FilterCheckpoint::new(IndexPrefix::Committed(record.identity())),
            IndexInputProtection::FromHeight(first),
            &[record],
        )
        .expect("safe progress");
}

fn index_bytes(store: &FjallNodeStore) -> Vec<(Vec<u8>, Vec<u8>)> {
    store
        .block_index
        .iter()
        .map(|entry| {
            let (key, value) = entry.into_inner().expect("entry");
            (key.to_vec(), value.to_vec())
        })
        .collect()
}

#[test]
fn prune_map_reserved_creation_without_owner_refuses_without_write() {
    // Arrange
    let (path, store) = open_store("prune-forged-owner");
    let before = index_bytes(&store);
    let forged = lock(
        open_bitcoin_core::chainstate::BASIC_INDEX_PRUNE_LOCK,
        0,
        100,
    );

    // Act
    let result = store.sync_prune_locks(&[forged]);

    // Assert
    assert!(result.is_err());
    assert_eq!(index_bytes(&store), before);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen");
    assert_eq!(index_bytes(&reopened), before);
    drop(reopened);
    remove_dir_if_exists(&path);
}

#[test]
fn prune_map_requires_exact_reserved_entry_and_preserves_unrelated_updates() {
    // Arrange
    let (path, store) = open_store("prune-exact-owner");
    index_fixture(&store);
    let reserved = store.load_prune_locks().expect("locks")[0].clone();
    let before = index_bytes(&store);
    let mut changed = reserved.clone();
    changed.height_first += 1;
    let mut changed_last = reserved.clone();
    changed_last.height_last -= 1;

    // Act / Assert: every proposed ownership mutation leaves all durable rows intact.
    for proposed in [
        vec![],
        vec![changed],
        vec![changed_last],
        vec![reserved.clone(), reserved.clone()],
    ] {
        assert!(store.sync_prune_locks(&proposed).is_err());
        assert_eq!(index_bytes(&store), before);
    }
    let expected = vec![reserved, lock("ordinary", 20, 30)];
    store
        .sync_prune_locks(&expected)
        .expect("exact preservation");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen");
    assert_eq!(reopened.load_prune_locks().expect("locks"), expected);
    drop(reopened);
    remove_dir_if_exists(&path);
}

#[test]
fn prune_map_clone_prepared_before_disable_cannot_resurrect_reserved_entry() {
    // Arrange
    let (path, store) = open_store("prune-stale-disable");
    index_fixture(&store);
    let clone = store.clone();
    let (prepared_tx, prepared_rx) = std::sync::mpsc::channel();
    let (apply_tx, apply_rx) = std::sync::mpsc::channel();
    let thread = std::thread::spawn(move || {
        let stale = clone.load_prune_locks().expect("prepared map");
        prepared_tx.send(()).expect("prepared signal");
        apply_rx.recv().expect("apply signal");
        clone.sync_prune_locks(&stale)
    });
    prepared_rx.recv().expect("prepared");
    store
        .disable_basic_filter_index()
        .expect("authorized disable");
    let before = index_bytes(&store);

    // Act
    apply_tx.send(()).expect("apply");
    let result = thread.join().expect("writer");

    // Assert
    assert!(result.is_err());
    assert_eq!(index_bytes(&store), before);
    store
        .sync_prune_locks(&[lock("ordinary", 20, 30)])
        .expect("disabled ordinary lock");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen");
    assert_eq!(
        reopened.load_prune_locks().expect("locks"),
        vec![lock("ordinary", 20, 30)]
    );
    assert!(matches!(
        reopened
            .maybe_basic_filter_lifecycle_for_test()
            .expect("owner"),
        Some(
            open_bitcoin_core::chainstate::filter_index::lifecycle::IndexLifecycle::Disabled { .. }
        )
    ));
    drop(reopened);
    remove_dir_if_exists(&path);
}

#[test]
fn prune_map_clone_prepared_before_progress_cannot_replace_fresh_reserved_entry() {
    // Arrange
    let (path, store) = open_store("prune-stale-progress");
    let positions = index_fixture(&store);
    let clone = store.clone();
    let (prepared_tx, prepared_rx) = std::sync::mpsc::channel();
    let (apply_tx, apply_rx) = std::sync::mpsc::channel();
    let thread = std::thread::spawn(move || {
        let stale = clone.load_prune_locks().expect("stale map");
        prepared_tx.send(()).expect("prepared");
        apply_rx.recv().expect("apply");
        clone.sync_prune_locks(&stale)
    });
    prepared_rx.recv().expect("prepared");
    publish_genesis(&store, &positions, 1);
    let before = index_bytes(&store);

    // Act
    apply_tx.send(()).expect("apply");
    let result = thread.join().expect("writer");

    // Assert
    assert!(result.is_err());
    assert_eq!(index_bytes(&store), before);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen");
    assert_eq!(index_bytes(&reopened), before);
    assert_eq!(
        reopened.load_prune_locks().expect("fresh lock")[0].height_first,
        1
    );
    drop(reopened);
    remove_dir_if_exists(&path);
}

#[test]
fn prune_map_preserves_stronger_valid_and_disabled_retained_entries_exactly() {
    use crate::storage::filter_index::ownership::{OWNER_KEY, encode_owner};
    use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};
    for disabled in [false, true] {
        // Arrange: stronger protection than the committed genesis requires.
        let (path, store) = open_store("prune-strong-owner");
        let positions = index_fixture(&store);
        publish_genesis(&store, &positions, 1);
        let reserved = open_bitcoin_core::chainstate::IndexInputProtection::FromHeight(0)
            .maybe_prune_lock()
            .expect("stronger lock");
        store
            .write_raw_for_test(
                StorageNamespace::BlockIndex,
                PRUNE_LOCKS_KEY,
                super::super::prune::encode_prune_locks(std::slice::from_ref(&reserved))
                    .expect("map"),
            )
            .expect("stronger persisted recovery protection");
        if disabled {
            store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    OWNER_KEY,
                    encode_owner(IndexLifecycle::Disabled {
                        generation: IndexGeneration::new(1),
                    })
                    .to_vec(),
                )
                .expect("conservative Disabled recovery fixture");
        }
        let before = index_bytes(&store);
        let mut weaker = reserved.clone();
        weaker.height_first = 1;

        // Act / Assert
        assert!(store.sync_prune_locks(&[weaker]).is_err());
        assert!(store.sync_prune_locks(&[]).is_err());
        assert_eq!(index_bytes(&store), before);
        store
            .sync_prune_locks(&[reserved.clone(), lock("ordinary", 20, 30)])
            .expect("exact stronger preservation");
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("reopen");
        assert_eq!(
            reopened.load_prune_locks().expect("locks"),
            vec![reserved, lock("ordinary", 20, 30)]
        );
        drop(reopened);
        remove_dir_if_exists(&path);
    }
}

#[test]
fn prune_map_validates_legacy_and_corrupt_current_owner_before_unrelated_write() {
    use crate::storage::filter_index::ownership::OWNER_KEY;
    for corrupt in [false, true] {
        // Arrange: raw mutation represents persisted compatibility/corruption only.
        let (path, store) = open_store("prune-legacy-or-corrupt");
        index_fixture(&store);
        if corrupt {
            store
                .write_raw_for_test(StorageNamespace::BlockIndex, OWNER_KEY, vec![255])
                .expect("corrupt owner fixture");
        } else {
            store.block_index.remove(OWNER_KEY).expect("legacy fixture");
            store
                .db
                .persist(fjall::PersistMode::SyncAll)
                .expect("durable legacy");
        }
        let original = store.load_prune_locks().expect("locks");
        let mut proposed = original.clone();
        proposed.push(lock("ordinary", 20, 30));
        let before = index_bytes(&store);

        // Act
        let result = store.sync_prune_locks(&proposed);

        // Assert
        if corrupt {
            assert!(result.is_err());
            assert_eq!(index_bytes(&store), before);
        } else {
            result.expect("valid legacy preservation");
            assert!(
                store
                    .maybe_basic_filter_lifecycle_for_test()
                    .expect("legacy absence")
                    .is_none()
            );
            assert!(store.sync_prune_locks(&[]).is_err());
        }
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("reopen");
        assert_eq!(
            reopened.load_prune_locks().expect("locks"),
            if corrupt { original } else { proposed }
        );
        drop(reopened);
        remove_dir_if_exists(&path);
    }
}
