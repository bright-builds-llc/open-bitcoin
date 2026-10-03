// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use open_bitcoin_core::{
    chainstate::BlockUndo,
    primitives::{Block, BlockHash, BlockHeader, MerkleRoot},
};

use super::*;
use crate::storage::{PersistMode, snapshot_codec::encode_block_undo};

static NEXT_PATH: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    store: FjallNodeStore,
    path: TempPath,
}

struct TempPath(PathBuf);

impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "open-bitcoin-payload-usage-{}-{}",
            std::process::id(),
            NEXT_PATH.fetch_add(1, Ordering::Relaxed)
        ));
        let store = FjallNodeStore::open(&path).expect("fresh fixture opens");
        Self {
            store,
            path: TempPath(path),
        }
    }

    fn raw(&self, namespace: StorageNamespace, key: &str, bytes: &[u8]) {
        self.store
            .write_raw_for_test(namespace, key, bytes.to_vec())
            .expect("fixture write succeeds");
    }
}

impl Drop for TempPath {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("fixture directory removed");
    }
}

fn position(height: u32) -> ChainPosition {
    ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0; 32]),
            merkle_root: MerkleRoot::from_byte_array([0; 32]),
            time: 1,
            bits: 0x207f_ffff,
            nonce: height,
        },
        height,
        1,
        1,
    )
}

#[test]
fn payload_usage_counts_actual_encoded_block_and_undo_values() {
    // Arrange
    let fixture = Fixture::new();
    let active = position(300);
    let block = Block {
        header: active.header.clone(),
        transactions: Vec::new(),
    };
    let undo = BlockUndo::default();
    fixture
        .store
        .save_block(&block, PersistMode::Sync)
        .expect("block saved");
    fixture
        .store
        .save_undo(active.block_hash, &undo, PersistMode::Sync)
        .expect("undo saved");
    let expected = (open_bitcoin_core::codec::encode_block(&block)
        .expect("block encodes")
        .len()
        + encode_block_undo(&undo).expect("undo encodes").len()) as u64;
    // Act
    let usage = fixture
        .store
        .retained_payload_usage(&[active])
        .expect("usage measured");
    // Assert
    assert_eq!(usage.current_usage_bytes, expected);
    assert_eq!(usage.height_sizes, BTreeMap::from([(300, expected)]));
    assert!(usage.revision.is_reusable());
    assert_eq!(
        fixture.store.payload_usage_revision().expect("revision"),
        usage.revision
    );
}

#[test]
fn payload_usage_total_includes_nonactive_records_but_omits_support_values() {
    // Arrange
    let fixture = Fixture::new();
    let active = position(300);
    let nonactive = position(20);
    fixture.raw(
        StorageNamespace::BlockIndex,
        &block_key(active.block_hash),
        &[1; 3],
    );
    fixture.raw(
        StorageNamespace::Chainstate,
        &undo_key(active.block_hash),
        &[2; 5],
    );
    fixture.raw(
        StorageNamespace::BlockIndex,
        &block_key(nonactive.block_hash),
        &[3; 11],
    );
    fixture.raw(
        StorageNamespace::Chainstate,
        &undo_key(nonactive.block_hash),
        &[4; 7],
    );
    fixture.raw(StorageNamespace::BlockIndex, "snapshot", &[0; 99]);
    fixture.raw(StorageNamespace::BlockIndex, "prune_summary", &[0; 99]);
    fixture.raw(StorageNamespace::Chainstate, "snapshot", &[0; 99]);
    fixture.raw(StorageNamespace::Chainstate, "chain_meta", &[0; 99]);
    fixture.raw(StorageNamespace::Coins, "coin", &[0; 99]);
    // Act
    let usage = fixture
        .store
        .retained_payload_usage(&[active])
        .expect("raw payload sizes do not decode");
    // Assert
    assert_eq!(usage.current_usage_bytes, 26);
    assert_eq!(usage.height_sizes, BTreeMap::from([(300, 8)]));
}

#[test]
fn payload_usage_candidates_include_half_pairs_and_zero_length_mates() {
    // Arrange
    let fixture = Fixture::new();
    let active: Vec<_> = (1..=4).map(position).collect();
    fixture.raw(
        StorageNamespace::BlockIndex,
        &block_key(active[0].block_hash),
        &[0; 3],
    );
    fixture.raw(
        StorageNamespace::Chainstate,
        &undo_key(active[1].block_hash),
        &[0; 5],
    );
    fixture.raw(
        StorageNamespace::BlockIndex,
        &block_key(active[2].block_hash),
        &[],
    );
    // Act
    let usage = fixture
        .store
        .retained_payload_usage(&active)
        .expect("usage measured");
    // Assert
    assert_eq!(usage.current_usage_bytes, 8);
    assert_eq!(usage.height_sizes, BTreeMap::from([(1, 3), (2, 5), (3, 0)]));
}

#[test]
fn payload_usage_clone_equal_size_replacement_invalidates_prior_facts() {
    // Arrange
    let fixture = Fixture::new();
    let active = position(1);
    let key = block_key(active.block_hash);
    fixture.raw(StorageNamespace::BlockIndex, &key, &[1; 3]);
    let before = fixture
        .store
        .retained_payload_usage(std::slice::from_ref(&active))
        .expect("before");
    let clone = fixture.store.clone();
    // Act
    clone
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, vec![2; 3])
        .expect("replacement");
    let revision = fixture.store.payload_usage_revision().expect("revision");
    let after = fixture
        .store
        .retained_payload_usage(&[active])
        .expect("after");
    // Assert
    assert!(!revision.is_reusable());
    assert_ne!(before.revision, after.revision);
    assert_eq!(before.current_usage_bytes, after.current_usage_bytes);
}

#[test]
fn payload_usage_nonpayload_write_preserves_measured_revision() {
    // Arrange
    let fixture = Fixture::new();
    let before = fixture.store.retained_payload_usage(&[]).expect("before");
    // Act
    fixture.raw(StorageNamespace::Chainstate, "chain_meta", &[7]);
    fixture.raw(StorageNamespace::BlockIndex, "prune_locks", &[0; 4]);
    // Assert
    assert_eq!(
        fixture.store.payload_usage_revision().expect("revision"),
        before.revision
    );
}

#[test]
fn payload_usage_typed_and_migration_writers_invalidate_prior_facts() {
    use open_bitcoin_core::chainstate::ChainstateSnapshot;
    use std::collections::HashMap;
    // Arrange
    let fixture = Fixture::new();
    let active = position(1);
    let block = Block {
        header: active.header.clone(),
        transactions: Vec::new(),
    };
    let undo = BlockUndo::default();
    let snapshot = ChainstateSnapshot::new(
        vec![active.clone()],
        HashMap::new(),
        HashMap::from([(active.block_hash, undo.clone())]),
    );
    let before = fixture
        .store
        .retained_payload_usage(std::slice::from_ref(&active))
        .expect("before");
    // Act
    fixture
        .store
        .save_block(&block, PersistMode::Sync)
        .expect("typed block write");
    let block_revision = fixture
        .store
        .retained_payload_usage(std::slice::from_ref(&active))
        .expect("block measured")
        .revision;
    fixture
        .store
        .save_undo(active.block_hash, &undo, PersistMode::Sync)
        .expect("typed undo write");
    let undo_revision = fixture
        .store
        .retained_payload_usage(std::slice::from_ref(&active))
        .expect("undo measured")
        .revision;
    fixture
        .store
        .seed_coins_from_snapshot(&snapshot)
        .expect("migration seeding");
    let migration_revision = fixture
        .store
        .retained_payload_usage(&[active])
        .expect("migration measured")
        .revision;
    // Assert
    assert_ne!(before.revision, block_revision);
    assert_ne!(block_revision, undo_revision);
    assert_ne!(undo_revision, migration_revision);
}

#[test]
fn payload_usage_raw_remove_and_paired_delete_invalidate_measurement() {
    // Arrange
    let fixture = Fixture::new();
    let active = position(1);
    fixture.raw(
        StorageNamespace::BlockIndex,
        &block_key(active.block_hash),
        &[1; 3],
    );
    fixture.raw(
        StorageNamespace::Chainstate,
        &undo_key(active.block_hash),
        &[2; 5],
    );
    let before = fixture
        .store
        .retained_payload_usage(std::slice::from_ref(&active))
        .expect("before");
    // Act
    fixture
        .store
        .remove_bytes(
            StorageNamespace::BlockIndex,
            &block_key(active.block_hash),
            PersistMode::Sync,
        )
        .expect("raw removal");
    let removed = fixture
        .store
        .retained_payload_usage(std::slice::from_ref(&active))
        .expect("half pair");
    fixture
        .store
        .commit_paired_delete(1, active.block_hash)
        .expect("paired delete");
    let after = fixture
        .store
        .retained_payload_usage(&[active])
        .expect("after");
    // Assert
    assert_ne!(before.revision, removed.revision);
    assert_ne!(removed.revision, after.revision);
    assert_eq!(removed.current_usage_bytes, 5);
    assert_eq!(after.current_usage_bytes, 0);
}

#[test]
fn payload_usage_reopen_requires_new_measurement_identity() {
    // Arrange
    let Fixture { store, path } = Fixture::new();
    let before = store.retained_payload_usage(&[]).expect("before");
    drop(store);
    let reopened = FjallNodeStore::open(&path.0).expect("reopen");
    // Act
    let revision = reopened.payload_usage_revision().expect("revision");
    let after = reopened.retained_payload_usage(&[]).expect("after");
    // Assert
    assert!(!revision.is_reusable());
    assert_ne!(before.revision, after.revision);
    drop(reopened);
}

#[test]
fn payload_usage_overflow_never_publishes_reusable_revision() {
    // Arrange
    let fixture = Fixture::new();
    fixture
        .store
        .payload_guard()
        .expect("guard")
        .maybe_generation = Some(u64::MAX);
    // Act
    fixture.raw(StorageNamespace::BlockIndex, "block:overflow", &[1]);
    let usage = fixture
        .store
        .retained_payload_usage(&[])
        .expect("fresh measurement allowed");
    // Assert
    assert_eq!(usage.current_usage_bytes, 1);
    assert_eq!(usage.revision, PayloadUsageRevision::Invalid);
}

fn injected_persist_error() -> StorageError {
    StorageError::BackendFailure {
        namespace: StorageNamespace::BlockIndex,
        message: "injected persistence failure after live insertion".to_owned(),
        action: super::super::StorageRecoveryAction::Restart,
    }
}

#[test]
fn payload_usage_checked_sum_refuses_overflow() {
    // Arrange
    let sizes = [Ok(u64::MAX), Ok(1)];
    // Act
    let result = sum_sizes(sizes);
    // Assert
    assert!(matches!(result, Err(StorageError::Corruption { .. })));
}

#[test]
fn payload_usage_backend_size_error_propagates_instead_of_zero_total() {
    // Arrange
    let sizes = [Ok(3), Err(injected_persist_error())];
    // Act
    let result = sum_sizes(sizes);
    // Assert
    assert!(matches!(result, Err(StorageError::BackendFailure { .. })));
}

fn paused_writer_measurement(fail_persist: bool, replacement_size: usize) {
    use std::{
        sync::{TryLockError, mpsc},
        thread,
    };
    // Arrange
    let fixture = Fixture::new();
    let active = position(1);
    let key = block_key(active.block_hash);
    fixture.raw(StorageNamespace::BlockIndex, &key, &[1; 3]);
    let before = fixture
        .store
        .retained_payload_usage(std::slice::from_ref(&active))
        .expect("before");
    let writer_store = fixture.store.clone();
    let reader_store = fixture.store.clone();
    let revision_store = fixture.store.clone();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let (measured_tx, measured_rx) = mpsc::channel();
    let (revision_tx, revision_rx) = mpsc::channel();
    let (reader_started_tx, reader_started_rx) = mpsc::channel();
    let (revision_started_tx, revision_started_rx) = mpsc::channel();
    // Act
    thread::scope(|scope| {
        let writer = scope.spawn(move || {
            writer_store.with_payload_mutation(|| {
                entered_tx
                    .send(())
                    .expect("writer signaled guard acquired and invalidated");
                release_rx.recv().expect("writer released");
                writer_store
                    .block_index
                    .insert(&key, vec![2; replacement_size])
                    .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
                if fail_persist {
                    return Err(injected_persist_error());
                }
                writer_store.persist(StorageNamespace::BlockIndex, PersistMode::Sync)
            })
        });
        entered_rx.recv().expect("writer has guard");
        let reader = scope.spawn(move || {
            reader_started_tx.send(()).expect("measurement started");
            measured_tx
                .send(reader_store.retained_payload_usage(&[active]))
                .expect("measurement result sent");
        });
        let revision_reader = scope.spawn(move || {
            revision_started_tx.send(()).expect("reuse check started");
            revision_tx
                .send(revision_store.payload_usage_revision())
                .expect("revision result sent");
        });
        reader_started_rx
            .recv()
            .expect("reader attempted measurement");
        revision_started_rx
            .recv()
            .expect("reader attempted idle revision check");
        // Assert: the held writer guard prohibits both old-value measurement and reuse.
        let guard_blocked = matches!(
            fixture.store.payload_usage.try_lock(),
            Err(TryLockError::WouldBlock)
        );
        let early_measurement = measured_rx.try_recv();
        let early_revision = revision_rx.try_recv();
        release_tx.send(()).expect("release writer");
        let result = writer.join().expect("writer did not unwind");
        assert_eq!(result.is_err(), fail_persist);
        reader.join().expect("reader finished");
        revision_reader.join().expect("revision reader finished");
        assert!(guard_blocked);
        assert!(matches!(early_measurement, Err(mpsc::TryRecvError::Empty)));
        assert!(matches!(early_revision, Err(mpsc::TryRecvError::Empty)));
    });
    let after = measured_rx
        .recv()
        .expect("measurement response")
        .expect("fresh measured facts");
    let revision = revision_rx
        .recv()
        .expect("revision response")
        .expect("reuse check");
    assert_eq!(after.current_usage_bytes, replacement_size as u64);
    assert_ne!(after.revision, before.revision);
    assert_ne!(revision, before.revision);
}

#[test]
fn payload_usage_paused_writer_blocks_measurement_until_completed_write() {
    paused_writer_measurement(false, 7);
}

#[test]
fn payload_usage_paused_equal_size_writer_cannot_reuse_old_facts() {
    paused_writer_measurement(false, 3);
}

#[test]
fn payload_usage_paused_writer_error_after_live_change_invalidates_old_facts() {
    paused_writer_measurement(true, 7);
}

#[test]
fn payload_usage_overlapping_clone_writers_serialize_complete_attempts() {
    use std::{
        sync::{TryLockError, mpsc},
        thread,
    };
    // Arrange
    let fixture = Fixture::new();
    let first = fixture.store.clone();
    let second = fixture.store.clone();
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let (attempted_tx, attempted_rx) = mpsc::channel();
    let (second_entered_tx, second_entered_rx) = mpsc::channel();
    // Act
    thread::scope(|scope| {
        let writer_one = scope.spawn(move || {
            first.with_payload_mutation(|| {
                entered_tx.send(()).expect("first entered");
                release_rx.recv().expect("first released");
                first
                    .block_index
                    .insert("block:first", vec![1; 3])
                    .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
                first.persist(StorageNamespace::BlockIndex, PersistMode::Sync)
            })
        });
        entered_rx.recv().expect("first holds guard");
        let writer_two = scope.spawn(move || {
            attempted_tx.send(()).expect("second attempted");
            second.with_payload_mutation(|| {
                second_entered_tx.send(()).expect("second entered");
                second
                    .block_index
                    .insert("block:second", vec![2; 5])
                    .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
                second.persist(StorageNamespace::BlockIndex, PersistMode::Sync)
            })
        });
        attempted_rx.recv().expect("second attempted guard");
        // Assert
        let guard_blocked = matches!(
            fixture.store.payload_usage.try_lock(),
            Err(TryLockError::WouldBlock)
        );
        let early_second_writer = second_entered_rx.try_recv();
        release_tx.send(()).expect("release first");
        writer_one
            .join()
            .expect("first no unwind")
            .expect("first succeeds");
        writer_two
            .join()
            .expect("second no unwind")
            .expect("second succeeds");
        assert!(guard_blocked);
        assert!(matches!(
            early_second_writer,
            Err(mpsc::TryRecvError::Empty)
        ));
    });
    second_entered_rx.recv().expect("second eventually entered");
    assert_eq!(
        fixture
            .store
            .retained_payload_usage(&[])
            .expect("usage")
            .current_usage_bytes,
        8
    );
}

#[test]
fn payload_usage_unwind_poison_refuses_accounting_and_future_mutations() {
    // Arrange
    let fixture = Fixture::new();
    let writer = fixture.store.clone();
    // Act
    let result = std::thread::spawn(move || {
        writer.with_payload_mutation::<()>(|| panic!("injected writer unwind"))
    })
    .join();
    // Assert
    assert!(result.is_err());
    assert!(fixture.store.retained_payload_usage(&[]).is_err());
    assert!(fixture.store.payload_usage_revision().is_err());
    assert!(
        fixture
            .store
            .write_raw_for_test(StorageNamespace::BlockIndex, "block:after-poison", vec![1])
            .is_err()
    );
}
