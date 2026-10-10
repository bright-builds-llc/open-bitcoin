// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::chainstate::BasicFilterStartupMode;

mod admission;
mod fencing;

fn budget() -> open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork {
    use open_bitcoin_core::chainstate::filter_index::catch_up::{
        BASIC_INDEX_MAX_ENCODED_BYTES, TurnWork,
    };
    TurnWork {
        blocks: 128,
        encoded_bytes: BASIC_INDEX_MAX_ENCODED_BYTES,
        cloned_bytes: BASIC_INDEX_MAX_ENCODED_BYTES * 16,
        record_operations: 10_000,
        checkpoint_operations: 1_000_000,
        projection_operations: 10_000,
        ..TurnWork::default()
    }
}

fn recovered(store: &FjallNodeStore, positions: &[ChainPosition]) {
    seed(store, positions);
    for position in positions {
        store
            .save_block(
                &fixture_block(position.previous_block_hash(), position.height),
                PersistMode::Sync,
            )
            .expect("body");
        if position.height != 0 {
            store
                .save_undo(
                    position.block_hash,
                    &BlockUndo {
                        transactions: Vec::new(),
                    },
                    PersistMode::Sync,
                )
                .expect("undo");
        }
    }
    store
        .configure_basic_filter_index_before_prune(
            Some(positions.last().expect("tip").block_hash),
            BasicFilterStartupMode::Enabled,
        )
        .expect("recovered authority");
}

fn append(
    store: &FjallNodeStore,
    records: &[StoredFilterRecord],
) -> super::super::BasicFilterAppendOutcome {
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let prepared = store
        .prepare_basic_filter_append(proof, records)
        .expect("prepare");
    store
        .complete_basic_filter_append(prepared)
        .expect("achieved append")
}

#[test]
fn phase157_append_genesis_publishes_exact_durable_checkpoint() {
    // Arrange
    let path = temp_path("append-genesis");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(1);
    recovered(&store, &positions);
    let operator = PruneLockInfo {
        name: "operator".to_owned(),
        height_first: 7,
        height_last: 99,
    };
    let mut locks = store.load_prune_locks().expect("locks");
    locks.push(operator.clone());
    store
        .sync_prune_locks(&locks)
        .expect("ordinary operator lock");
    // Act
    let achieved = append(&store, &records);
    // Assert
    assert_eq!(achieved.processed, checkpoint(&records[0]));
    assert_eq!(achieved.safe_checkpoint, checkpoint(&records[0]));
    assert!(achieved.work.record_operations > 0);
    assert!(achieved.batch_bytes > 0);
    assert!(
        store
            .load_prune_locks()
            .expect("preserved operator map")
            .contains(&operator)
    );
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("saved"),
        Some(achieved.safe_checkpoint)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_identical_retry_is_bounded_and_has_no_batch() {
    // Arrange
    let path = temp_path("append-retry");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(3);
    recovered(&store, &positions);
    append(&store, &records);
    let before = store.maybe_basic_filter_state().expect("before");
    // Act
    let retry = append(&store, &records);
    // Assert
    assert_eq!(retry.batch_bytes, 0);
    assert_eq!(retry.safe_checkpoint, checkpoint(&records[2]));
    assert_eq!(store.maybe_basic_filter_state().expect("same"), before);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_missing_processed_parent_refuses_before_effects() {
    // Arrange
    let path = temp_path("append-parent");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(4);
    recovered(&store, &positions);
    append(&store, &records[..2]);
    store
        .block_index
        .remove(codec::record_key(records[0].identity().block_hash()))
        .expect("remove fixture");
    let before = store.maybe_basic_filter_state().expect("before");
    // Act
    let result = store.maybe_basic_filter_append_proof_with_budget(budget());
    // Assert
    assert!(result.is_err());
    assert_eq!(store.maybe_basic_filter_state().expect("after"), before);
    assert!(
        store
            .get_bytes(
                StorageNamespace::BlockIndex,
                &codec::record_key(records[2].identity().block_hash())
            )
            .expect("absent")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn alternate_bytes(record: &StoredFilterRecord) -> Vec<u8> {
    let mut bytes = codec::encode_record(record);
    let hash = open_bitcoin_core::primitives::FilterHash::from_byte_array(
        open_bitcoin_core::consensus::crypto::double_sha256(&[0]),
    );
    let header = open_bitcoin_core::consensus::compute_filter_header(
        hash,
        record.identity().previous_header(),
    );
    bytes.truncate(codec::RECORD_OVERHEAD);
    bytes[102..134].copy_from_slice(hash.as_bytes());
    bytes[134..166].copy_from_slice(header.as_bytes());
    bytes[166..170].copy_from_slice(&1_u32.to_le_bytes());
    bytes.push(0);
    bytes
}

#[test]
fn phase157_append_conflicting_immutable_row_refuses_without_rewrite() {
    // Arrange
    let path = temp_path("append-conflict");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(2);
    recovered(&store, &positions);
    append(&store, &records[..1]);
    let key = codec::record_key(records[1].identity().block_hash());
    let bytes = alternate_bytes(&records[1]);
    // Bypass raw publication deliberately to exercise local immutable conflict
    // detection while the original recovered append proof is still live.
    store
        .block_index
        .insert(&key, bytes.clone())
        .expect("valid conflicting hidden row");
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    // Act
    let result = store.prepare_basic_filter_append(proof, &records[1..]);
    // Assert
    assert!(
        result
            .err()
            .expect("immutable conflict")
            .to_string()
            .contains("conflicting immutable BASIC record")
    );
    assert_eq!(
        store
            .get_bytes(StorageNamespace::BlockIndex, &key)
            .expect("retained"),
        Some(bytes)
    );
    assert_eq!(
        store
            .maybe_basic_filter_state()
            .expect("state")
            .expect("present")
            .maybe_endpoint,
        None
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_saved_hidden_rows_are_reused_after_local_validation() {
    // Arrange
    let path = temp_path("append-hidden");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(3);
    recovered(&store, &positions);
    seed_orphan_records(&store, &records);
    assert!(
        store
            .maybe_basic_filter_append_proof_with_budget(budget())
            .expect("raw writes revoke authority")
            .is_none()
    );
    // Reestablish authority only through the actual complete recovery/preflight.
    store
        .configure_basic_filter_index_before_prune(
            Some(positions.last().expect("tip").block_hash),
            BasicFilterStartupMode::Enabled,
        )
        .expect("full recovery of retained hidden rows");
    // Act
    let achieved = append(&store, &records);
    // Assert
    assert_eq!(achieved.safe_checkpoint, checkpoint(&records[2]));
    assert_eq!(
        store
            .load_basic_filter_record(records[2].identity().block_hash())
            .expect("general reader"),
        Some(records[2].clone())
    );
    assert_eq!(achieved.work.projection_operations, 9);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_saved_projection_conflict_refuses_before_new_rows() {
    // Arrange
    let path = temp_path("append-projection-conflict");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(2);
    recovered(&store, &positions);
    // Inject backend tampering beneath the raw-write invalidation boundary so
    // this test still exercises append's bounded saved-projection validation.
    store
        .block_index
        .insert(
            codec::active_key(0),
            codec::encode_projection(0, positions[1].block_hash),
        )
        .expect("conflict fixture");
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    // Act
    let result = store.prepare_basic_filter_append(proof, &records);
    // Assert
    assert!(result.is_err());
    assert!(
        store
            .get_bytes(
                StorageNamespace::BlockIndex,
                &codec::record_key(records[0].identity().block_hash())
            )
            .expect("absent")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_candidates_ahead_of_durability_do_not_release_protection() {
    // Arrange
    let path = temp_path("append-ahead");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(3);
    recovered(&store, &positions[..1]);
    // Act
    let achieved = append(&store, &records);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual reopen");
    reopened
        .configure_basic_filter_index_before_prune(
            Some(positions[0].block_hash),
            BasicFilterStartupMode::Enabled,
        )
        .expect("reconcile conservative safe checkpoint");
    // Assert
    assert_eq!(achieved.processed, checkpoint(&records[2]));
    assert_eq!(achieved.safe_checkpoint, checkpoint(&records[0]));
    assert_eq!(
        reopened
            .maybe_basic_filter_append_proof_with_budget(budget())
            .expect("proof")
            .expect("recovered")
            .processed(),
        checkpoint(&records[0])
    );
    assert!(
        reopened
            .maybe_active_basic_filter_record(1)
            .expect("hidden")
            .is_none()
    );
    assert_eq!(
        reopened
            .load_basic_filter_record(records[2].identity().block_hash())
            .expect("immutable retained"),
        Some(records[2].clone())
    );
    assert_eq!(
        reopened
            .maybe_basic_filter_state()
            .expect("state")
            .expect("saved")
            .protection,
        IndexInputProtection::FromHeight(1)
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_stale_prepared_work_refuses_after_real_coins_writer() {
    // Arrange
    let path = temp_path("append-stale-coins");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(2);
    recovered(&store, &positions);
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let prepared = store
        .prepare_basic_filter_append(proof, &records)
        .expect("prepare");
    let before = store.maybe_basic_filter_state().expect("before");
    // Act
    store
        .clone()
        .coins_view()
        .batch_write(
            open_bitcoin_core::chainstate::CoinsBatch {
                entries: Default::default(),
            },
            Some(positions[1].block_hash),
        )
        .expect("actual interleaving writer");
    let result = store.complete_basic_filter_append(prepared);
    // Assert
    assert!(result.is_err());
    assert_eq!(store.maybe_basic_filter_state().expect("unchanged"), before);
    assert_eq!(
        store.basic_filter_artifacts().expect("no rows"),
        (false, false)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_stale_prepared_frontier_refuses_after_other_completion() {
    // Arrange
    let path = temp_path("append-stale-frontier");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(3);
    recovered(&store, &positions);
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let prepared = store
        .prepare_basic_filter_append(proof, &records)
        .expect("prepare");
    // Act
    append(&store, &records[..1]);
    let result = store.complete_basic_filter_append(prepared);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store
            .maybe_basic_filter_append_proof_with_budget(budget())
            .expect("proof")
            .expect("current")
            .processed(),
        checkpoint(&records[0])
    );
    assert!(
        store
            .get_bytes(
                StorageNamespace::BlockIndex,
                &codec::record_key(records[1].identity().block_hash())
            )
            .expect("absent")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn fault_reopen(point: super::super::FilterPublicationFault) {
    // Arrange
    let path = temp_path(&format!("append-fault-{point:?}"));
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(3);
    recovered(&store, &positions);
    append(&store, &records[..1]);
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let prepared = store
        .prepare_basic_filter_append(proof, &records[1..])
        .expect("prepare");
    let before = store.maybe_basic_filter_state().expect("before");
    store.set_basic_filter_fault(point);
    // Act
    let result = store.complete_basic_filter_append(prepared);
    assert!(result.is_err());
    assert!(
        store
            .clone()
            .maybe_basic_filter_append_proof_with_budget(budget())
            .is_err()
    );
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("genuine drop and reopen");
    // Assert
    let committed = point == super::super::FilterPublicationFault::AfterCommit;
    assert_eq!(
        reopened
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        Some(if committed {
            checkpoint(&records[2])
        } else {
            FilterCheckpoint::new(IndexPrefix::Empty)
        })
    );
    if !committed {
        assert_eq!(
            reopened.maybe_basic_filter_state().expect("old state"),
            before
        );
    }
    for record in &records[1..] {
        assert_eq!(
            reopened
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable"),
            committed.then(|| record.clone())
        );
    }
    let state = reopened
        .maybe_basic_filter_state()
        .expect("state")
        .expect("saved");
    let locks = reopened.load_prune_locks().expect("locks");
    assert!(locks.contains(&state.protection.maybe_prune_lock().expect("reserved lock")));
    reopened
        .configure_basic_filter_index_before_prune(
            Some(positions[2].block_hash),
            BasicFilterStartupMode::Enabled,
        )
        .expect("full recovered proof");
    assert_eq!(
        reopened
            .maybe_basic_filter_append_proof_with_budget(budget())
            .expect("proof")
            .expect("recovered")
            .processed(),
        if committed {
            checkpoint(&records[2])
        } else {
            FilterCheckpoint::new(IndexPrefix::Empty)
        }
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_before_records_recovers_complete_old_state() {
    fault_reopen(super::super::FilterPublicationFault::BeforeRecords);
}
#[test]
fn phase157_append_before_checkpoint_recovers_complete_old_state() {
    fault_reopen(super::super::FilterPublicationFault::BeforeCheckpoint);
}
#[test]
fn phase157_append_before_protection_recovers_complete_old_state() {
    fault_reopen(super::super::FilterPublicationFault::BeforeProtection);
}
#[test]
fn phase157_append_after_commit_recovers_complete_new_state() {
    fault_reopen(super::super::FilterPublicationFault::AfterCommit);
}

#[test]
fn phase157_append_fixed_suffix_operations_do_not_grow_with_prefix() {
    // Arrange
    let mut measured = Vec::new();
    for prefix in [16, 256] {
        let path = temp_path("append-prefix-work");
        let store = FjallNodeStore::open(&path).expect("open");
        let (positions, records) = fixtures(prefix + 3);
        recovered(&store, &positions);
        for batch in records[..prefix as usize].chunks(128) {
            publish(&store, &positions, batch);
        }
        store
            .enable_basic_filter_index(&fence(&positions))
            .expect("complete prefix recovery");
        store
            .filter_integrity_reads
            .store(0, std::sync::atomic::Ordering::Relaxed);
        // Act
        let achieved = append(&store, &records[prefix as usize..]);
        measured.push((
            achieved.work,
            achieved.batch_bytes,
            store
                .filter_integrity_reads
                .load(std::sync::atomic::Ordering::Relaxed),
        ));
        // Assert
        assert_eq!(
            achieved.safe_checkpoint,
            checkpoint(records.last().expect("tip"))
        );
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
    assert_eq!(measured[0], measured[1]);
    assert!(measured[0].0.record_operations > 0);
    assert!(measured[0].2 > 0 && measured[0].2 < 40);
    eprintln!("phase157 fixed-suffix work (16/256): {measured:?}");
}

#[test]
fn phase157_append_contiguous_turns_keep_processed_separate_from_safe() {
    // Arrange
    let path = temp_path("append-turns");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(4);
    recovered(&store, &positions);
    // Act
    let first = append(&store, &records[..2]);
    let second = append(&store, &records[2..]);
    // Assert
    assert_eq!(first.processed, checkpoint(&records[1]));
    assert_eq!(
        first.safe_checkpoint,
        FilterCheckpoint::new(IndexPrefix::Empty)
    );
    assert_eq!(second.processed, checkpoint(&records[3]));
    assert_eq!(second.safe_checkpoint, second.processed);
    assert_eq!(
        store.basic_filter_projection(3).expect("projection"),
        records[3].identity().block_hash()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_append_skipped_height_refuses_before_effects() {
    // Arrange
    let path = temp_path("append-skip");
    let store = FjallNodeStore::open(&path).expect("open");
    let (positions, records) = fixtures(3);
    recovered(&store, &positions);
    let proof = store
        .maybe_basic_filter_append_proof_with_budget(budget())
        .expect("proof")
        .expect("recovered");
    let before = store.maybe_basic_filter_state().expect("before");
    // Act
    let result = store.prepare_basic_filter_append(proof, &records[1..]);
    // Assert
    assert!(result.is_err());
    assert_eq!(store.maybe_basic_filter_state().expect("after"), before);
    assert_eq!(
        store.basic_filter_artifacts().expect("rows"),
        (false, false)
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}
