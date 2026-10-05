// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

use super::super::FilterPublicationFault;
use super::*;

pub(super) fn publish_current(
    store: &FjallNodeStore,
    fence: &VerifiedChainstateFence<'_>,
    checkpoint: FilterCheckpoint,
    protection: IndexInputProtection,
    records: &[StoredFilterRecord],
) -> Result<(), StorageError> {
    let work = store
        .maybe_basic_filter_work(fence)?
        .ok_or_else(|| index_corruption("test requires Active BASIC work"))?;
    store.publish_basic_filter_checkpoint(&work, fence, checkpoint, protection, records)
}

pub(super) fn append_current(
    store: &FjallNodeStore,
    records: &[StoredFilterRecord],
) -> Result<(), StorageError> {
    let positions = store.load_chain_meta_for_open()?.0;
    let fence = VerifiedChainstateFence::new(
        store.coins_view().best_block().map_err(index_corruption)?,
        Some(&positions),
    )
    .map_err(index_corruption)?;
    let work = store
        .maybe_basic_filter_work(&fence)?
        .ok_or_else(|| index_corruption("test requires Active BASIC work"))?;
    store.persist_basic_filter_records(&work, &fence, records)
}

/// Deliberately orphaned corruption fixture, never a live publication path.
pub(super) fn seed_orphan_records(store: &FjallNodeStore, records: &[StoredFilterRecord]) {
    for record in records {
        store
            .write_raw_for_test(
                StorageNamespace::BlockIndex,
                &codec::record_key(record.identity().block_hash()),
                codec::encode_record(record),
            )
            .expect("orphan row fixture");
    }
}

#[test]
fn filter_index_same_height_saved_fence_conflict_refuses_after_real_reopen() {
    // Arrange
    let path = temp_path("same-height-fence");
    let (positions, records) = fixtures(2);
    let store = FjallNodeStore::open(&path).expect("store");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("state");
    publish(&store, &positions, &records);
    let mut state = store
        .maybe_basic_filter_state()
        .expect("state")
        .expect("saved");
    state.fence_hash = positions[0].block_hash;
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            codec::STATE_KEY,
            codec::encode_state(state),
        )
        .expect("conflicting saved fence");
    drop(store);
    // Act
    let reopened = FjallNodeStore::open(&path).expect("real reopen");
    let result = reopened.maybe_basic_filter_checkpoint();
    // Assert
    let error = result.expect_err("checkpoint conflict");
    assert!(
        error
            .to_string()
            .contains("BASIC checkpoint differs from same-height saved fence")
    );
    assert_eq!(
        reopened.maybe_basic_filter_state().expect("unchanged"),
        Some(state)
    );
    for record in records {
        assert_eq!(
            reopened
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable"),
            Some(record)
        );
    }
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_fence_accepts_existing_consistent_empty_heads_marker() {
    // Arrange
    let path = temp_path("empty-heads");
    let (positions, _) = fixtures(1);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    use crate::storage::coins_codec::encode_head_blocks_key;
    store
        .coins_view()
        .write_raw_bytes(
            &encode_head_blocks_key(),
            Vec::new(), // Existing decoder accepts an empty value, not CompactSize zero.
        )
        .expect("consistent marker");
    // Act
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("typed consistent coins fence");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual second open");
    // Assert
    assert_eq!(
        reopened
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        Some(FilterCheckpoint::new(IndexPrefix::Empty))
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_fence_refuses_unrecovered_two_head_coins_before_initialization() {
    // Arrange
    let path = temp_path("two-heads");
    let (positions, _) = fixtures(2);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    use crate::storage::coins_codec::{encode_head_blocks_key, encode_head_blocks_value};
    store
        .coins_view()
        .delete_raw_bytes(&encode_best_block_key())
        .expect("interrupted B absence");
    store
        .coins_view()
        .write_raw_bytes(
            &encode_head_blocks_key(),
            encode_head_blocks_value(&[positions[1].block_hash, positions[0].block_hash])
                .expect("heads"),
        )
        .expect("interrupted H");
    // Act
    let result = store.initialize_basic_filter_state(&fence(&positions));
    // Assert
    assert!(
        result
            .expect_err("unrecovered coins refuse")
            .to_string()
            .contains("unrecovered")
    );
    assert_eq!(store.maybe_basic_filter_state().expect("state"), None);
    assert!(store.load_prune_locks().expect("locks").is_empty());
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_complete_record_and_projection_scan_has_linear_read_count() {
    // Arrange
    let path = temp_path("linear-scan");
    let (positions, records) = fixtures(256);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    for batch in records.chunks(128) {
        publish(&store, &positions, batch);
    }
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual second open");
    // Act
    reopened
        .validate_basic_filter_records()
        .expect("complete integrity scan");
    // Assert
    assert_eq!(
        reopened
            .filter_integrity_reads
            .load(std::sync::atomic::Ordering::Relaxed),
        4 * records.len() - 1
    );
    assert_eq!(
        reopened
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        Some(checkpoint(&records[255]))
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_after_record_commit_error_reopens_ahead_rows_without_authority_advancement() {
    // Arrange
    let path = temp_path("after-records");
    let (positions, records) = fixtures(2);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    publish(&store, &positions, &records[..1]);
    let state = store.maybe_basic_filter_state().expect("state");
    let locks = store.load_prune_locks().expect("locks");
    // Act
    store.set_basic_filter_fault(FilterPublicationFault::AfterCommit);
    assert!(append_current(&store, &records[1..]).is_err());
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual second open");
    // Assert
    assert_eq!(reopened.maybe_basic_filter_state().expect("state"), state);
    assert_eq!(reopened.load_prune_locks().expect("locks"), locks);
    assert_eq!(
        reopened
            .maybe_active_basic_filter_record(1)
            .expect("ahead hidden"),
        None
    );
    assert_eq!(
        reopened
            .load_basic_filter_record(records[1].identity().block_hash())
            .expect("committed ahead"),
        Some(records[1].clone())
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_empty_initialization_before_protection_failure_reopens_complete_old_absence() {
    // Arrange
    let path = temp_path("init-before-protection");
    let (positions, _) = fixtures(1);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    let operator = PruneLockInfo {
        name: "operator".to_owned(),
        height_first: 2,
        height_last: 3,
    };
    store
        .sync_prune_locks(std::slice::from_ref(&operator))
        .expect("operator");
    // Act
    store.set_basic_filter_fault(FilterPublicationFault::BeforeProtection);
    assert!(
        store
            .initialize_basic_filter_state(&fence(&positions))
            .is_err()
    );
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual second open");
    // Assert
    assert_eq!(
        reopened.maybe_basic_filter_state().expect("state absent"),
        None
    );
    assert_eq!(
        reopened.basic_filter_artifacts().expect("artifacts absent"),
        (false, false)
    );
    assert_eq!(
        reopened.load_prune_locks().expect("operator intact"),
        vec![operator]
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn checkpoint_fault_reopen(point: FilterPublicationFault) {
    // Arrange
    let path = temp_path(&format!("fault-{point:?}"));
    let (positions, records) = fixtures(2);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    publish(&store, &positions, &records[..1]);
    let operator = PruneLockInfo {
        name: "operator".to_owned(),
        height_first: 7,
        height_last: 99,
    };
    let mut locks = store.load_prune_locks().expect("locks");
    locks.push(operator.clone());
    store.sync_prune_locks(&locks).expect("operator");
    append_current(&store, &records[1..]).expect("ahead row");
    let before = store.maybe_basic_filter_state().expect("state");
    // Act
    store.set_basic_filter_fault(point);
    let result = publish_current(
        &store,
        &fence(&positions),
        checkpoint(&records[1]),
        IndexInputProtection::FromHeight(2),
        &records[1..],
    );
    assert!(result.is_err());
    assert!(
        append_current(&store, &records[..1]).is_err(),
        "poisoned instance refuses retry"
    );
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual second open after failure");
    // Assert
    let committed = point == FilterPublicationFault::AfterCommit;
    assert_eq!(
        reopened
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        Some(checkpoint(&records[usize::from(committed)]))
    );
    assert_eq!(
        reopened
            .maybe_active_basic_filter_record(1)
            .expect("visibility"),
        committed.then(|| records[1].clone())
    );
    let state = reopened
        .maybe_basic_filter_state()
        .expect("state")
        .expect("present");
    if !committed {
        assert_eq!(Some(state), before);
    }
    assert_eq!(
        state.protection,
        IndexInputProtection::FromHeight(if committed { 2 } else { 1 })
    );
    assert_eq!(state.fence_hash, positions[1].block_hash);
    let locks = reopened.load_prune_locks().expect("full locks");
    let reserved = locks
        .iter()
        .find(|lock| lock.name == open_bitcoin_core::chainstate::BASIC_INDEX_PRUNE_LOCK)
        .expect("reserved protection");
    assert_eq!(
        IndexInputProtection::from_saved_lock(reserved).expect("typed reserved lock"),
        state.protection
    );
    assert!(
        reopened
            .load_prune_locks()
            .expect("locks")
            .contains(&operator)
    );
    for record in &records {
        assert_eq!(
            reopened
                .load_basic_filter_record(record.identity().block_hash())
                .expect("retained"),
            Some(record.clone())
        );
    }
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_before_checkpoint_commit_keeps_checkpoint_and_protection() {
    checkpoint_fault_reopen(FilterPublicationFault::BeforeCheckpoint);
}
#[test]
fn filter_index_before_protection_insertion_keeps_checkpoint_and_protection() {
    checkpoint_fault_reopen(FilterPublicationFault::BeforeProtection);
}
#[test]
fn filter_index_after_successful_commit_error_reopens_complete_new_state() {
    checkpoint_fault_reopen(FilterPublicationFault::AfterCommit);
}

#[test]
fn filter_index_before_records_failure_reopens_old_state_without_ahead_rows() {
    // Arrange
    let path = temp_path("before-records");
    let (positions, records) = fixtures(2);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    publish(&store, &positions, &records[..1]);
    let state = store.maybe_basic_filter_state().expect("state");
    let locks = store.load_prune_locks().expect("locks");
    // Act
    store.set_basic_filter_fault(FilterPublicationFault::BeforeRecords);
    assert!(append_current(&store, &records[1..]).is_err());
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("second open");
    // Assert
    assert_eq!(reopened.maybe_basic_filter_state().expect("state"), state);
    assert_eq!(reopened.load_prune_locks().expect("locks"), locks);
    assert_eq!(
        reopened
            .load_basic_filter_record(records[1].identity().block_hash())
            .expect("ahead absent"),
        None
    );
    assert_eq!(
        reopened
            .load_basic_filter_record(records[0].identity().block_hash())
            .expect("old intact"),
        Some(records[0].clone())
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}
