// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};

fn active_store(
    name: &str,
    count: u32,
) -> (
    PathBuf,
    FjallNodeStore,
    Vec<ChainPosition>,
    Vec<StoredFilterRecord>,
) {
    let path = temp_path(name);
    let (positions, records) = fixtures(count);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    (path, store, positions, records)
}

fn maybe_owner_bytes(store: &FjallNodeStore) -> Option<Vec<u8>> {
    store
        .get_bytes(StorageNamespace::BlockIndex, codec::ownership::OWNER_KEY)
        .expect("owner bytes")
}

#[test]
fn filter_index_work_mint_materializes_legacy_owner_before_returning_authority() {
    for startup in [true, false] {
        // Arrange
        let (path, store, positions, records) = active_store("legacy-owner", 2);
        publish(&store, &positions, &records);
        store
            .block_index
            .remove(codec::ownership::OWNER_KEY)
            .expect("legacy fixture");
        store
            .persist(StorageNamespace::BlockIndex, PersistMode::Sync)
            .expect("sync legacy");
        drop(store);
        let store = FjallNodeStore::open(&path).expect("legacy actual reopen");
        // Act
        if startup {
            store
                .recover_basic_filter_index_before_prune(Some(positions[1].block_hash))
                .expect("production legacy startup");
        }
        let maybe_work = store
            .maybe_basic_filter_work(&fence(&positions))
            .expect("work");
        let maybe_bytes = maybe_owner_bytes(&store);
        let issued = maybe_work.is_some();
        drop(maybe_work);
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("actual reopen after materialization");
        // Assert
        assert!(issued);
        assert_eq!(maybe_bytes, maybe_owner_bytes(&reopened));
        assert_eq!(
            codec::ownership::decode_owner(&maybe_bytes.expect("materialized"))
                .expect("valid owner"),
            IndexLifecycle::Active {
                generation: IndexGeneration::new(0)
            }
        );
        assert_eq!(
            reopened
                .maybe_basic_filter_checkpoint()
                .expect("retained checkpoint"),
            Some(checkpoint(&records[1]))
        );
        assert_eq!(
            reopened
                .load_basic_filter_record(records[1].identity().block_hash())
                .expect("retained rows"),
            Some(records[1].clone())
        );
        drop(reopened);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn filter_index_same_generation_old_frontier_refuses_rows_and_checkpoint_after_reopen() {
    // Arrange
    let (path, store, positions, records) = active_store("stale-frontier", 3);
    let work = store
        .maybe_basic_filter_work(&fence(&positions))
        .expect("work")
        .expect("active");
    publish(&store, &positions, &records[..1]);
    let maybe_before_state = store.maybe_basic_filter_state().expect("state");
    let before_locks = store.load_prune_locks().expect("locks");
    let maybe_before_owner = maybe_owner_bytes(&store);
    // Act
    assert!(
        store
            .persist_basic_filter_records(&work, &fence(&positions), &records[1..])
            .is_err()
    );
    assert!(
        store
            .publish_basic_filter_checkpoint(
                &work,
                &fence(&positions),
                checkpoint(&records[2]),
                IndexInputProtection::FromHeight(3),
                &records[1..]
            )
            .is_err()
    );
    drop(work);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual reopen");
    // Assert
    assert_eq!(
        reopened.maybe_basic_filter_state().expect("state"),
        maybe_before_state
    );
    assert_eq!(reopened.load_prune_locks().expect("locks"), before_locks);
    assert_eq!(maybe_owner_bytes(&reopened), maybe_before_owner);
    for record in &records[1..] {
        assert_eq!(
            reopened
                .load_basic_filter_record(record.identity().block_hash())
                .expect("no delayed rows"),
            None
        );
    }
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_foreign_store_work_refuses_before_writing() {
    // Arrange
    let (first_path, first, positions, records) = active_store("foreign-first", 2);
    let (second_path, second, _, _) = active_store("foreign-second", 2);
    let work = first
        .maybe_basic_filter_work(&fence(&positions))
        .expect("work")
        .expect("active");
    let maybe_before_state = second.maybe_basic_filter_state().expect("state");
    let locks = second.load_prune_locks().expect("locks");
    // Act
    assert!(
        second
            .persist_basic_filter_records(&work, &fence(&positions), &records)
            .is_err()
    );
    assert!(
        second
            .publish_basic_filter_checkpoint(
                &work,
                &fence(&positions),
                checkpoint(&records[1]),
                IndexInputProtection::FromHeight(2),
                &records
            )
            .is_err()
    );
    drop(work);
    drop(first);
    drop(second);
    let reopened = FjallNodeStore::open(&second_path).expect("actual reopen");
    // Assert
    assert_eq!(
        reopened.maybe_basic_filter_state().expect("state"),
        maybe_before_state
    );
    assert_eq!(reopened.load_prune_locks().expect("locks"), locks);
    assert_eq!(
        reopened.basic_filter_artifacts().expect("no rows"),
        (false, false)
    );
    drop(reopened);
    std::fs::remove_dir_all(first_path).expect("cleanup first");
    std::fs::remove_dir_all(second_path).expect("cleanup second");
}

#[test]
fn filter_index_prior_store_incarnation_work_refuses_on_reopen() {
    // Arrange
    let (path, store, positions, records) = active_store("incarnation", 2);
    let work = store
        .maybe_basic_filter_work(&fence(&positions))
        .expect("work")
        .expect("active");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual reopen with retained token");
    // Act
    let result = reopened.persist_basic_filter_records(&work, &fence(&positions), &records);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        reopened.basic_filter_artifacts().expect("no mutation"),
        (false, false)
    );
    assert!(
        reopened
            .maybe_basic_filter_work(&fence(&positions))
            .expect("fresh work")
            .is_some()
    );
    drop(work);
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_disabled_retained_state_has_no_work_with_or_without_extra_lock() {
    for retain_lock in [true, false] {
        // Arrange
        let (path, store, positions, records) = active_store("disabled", 3);
        publish(&store, &positions, &records);
        let maybe_before_state = store.maybe_basic_filter_state().expect("state");
        let work = store
            .maybe_basic_filter_work(&fence(&positions))
            .expect("work")
            .expect("active");
        store
            .write_raw_for_test(
                StorageNamespace::BlockIndex,
                codec::ownership::OWNER_KEY,
                codec::ownership::encode_owner(IndexLifecycle::Disabled {
                    generation: IndexGeneration::new(1),
                })
                .to_vec(),
            )
            .expect("disabled lifecycle fixture");
        if !retain_lock {
            store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    super::super::super::prune::PRUNE_LOCKS_KEY,
                    super::super::super::prune::encode_prune_locks(&[]).expect("empty locks"),
                )
                .expect("released fixture");
        }
        // Act
        assert!(
            store
                .persist_basic_filter_records(&work, &fence(&positions), &records)
                .is_err()
        );
        assert!(
            store
                .publish_basic_filter_checkpoint(
                    &work,
                    &fence(&positions),
                    checkpoint(&records[2]),
                    IndexInputProtection::FromHeight(3),
                    &[]
                )
                .is_err()
        );
        assert!(
            store
                .maybe_basic_filter_work(&fence(&positions))
                .expect("disabled work")
                .is_none()
        );
        drop(work);
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("actual reopen");
        reopened
            .recover_basic_filter_index_before_prune(Some(positions[2].block_hash))
            .expect("disabled retained history startup");
        // Assert
        assert_eq!(
            reopened.maybe_basic_filter_state().expect("state"),
            maybe_before_state
        );
        assert_eq!(
            reopened
                .load_basic_filter_record(records[2].identity().block_hash())
                .expect("retained"),
            Some(records[2].clone())
        );
        let control = reopened.filter_publication_guard().expect("guard");
        let owner = reopened
            .maybe_basic_filter_owner_guarded(&control)
            .expect("owner")
            .expect("present");
        assert_eq!(
            owner.maybe_effective_protection(),
            retain_lock.then_some(IndexInputProtection::FromHeight(3))
        );
        drop(control);
        drop(reopened);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn filter_index_changed_generation_and_prepared_branch_refuse_live_work() {
    for changed_generation in [true, false] {
        // Arrange
        let (path, store, mut positions, records) = active_store("changed-work", 2);
        let work = store
            .maybe_basic_filter_work(&fence(&positions))
            .expect("work")
            .expect("active");
        if changed_generation {
            store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    codec::ownership::OWNER_KEY,
                    codec::ownership::encode_owner(IndexLifecycle::Active {
                        generation: IndexGeneration::new(1),
                    })
                    .to_vec(),
                )
                .expect("generation fixture");
        } else {
            let mut header = positions[1].header.clone();
            header.time += 1;
            positions[1] = ChainPosition::new(header, 1, 2, 1_001);
            seed(&store, &positions);
        }
        let maybe_before_state = store.maybe_basic_filter_state().expect("state");
        let before_locks = store.load_prune_locks().expect("locks");
        let maybe_before_owner = maybe_owner_bytes(&store);
        // Act
        assert!(
            store
                .persist_basic_filter_records(&work, &fence(&positions), &records)
                .is_err()
        );
        assert!(
            store
                .publish_basic_filter_checkpoint(
                    &work,
                    &fence(&positions),
                    checkpoint(&records[1]),
                    IndexInputProtection::FromHeight(2),
                    &records
                )
                .is_err()
        );
        drop(work);
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("actual reopen");
        // Assert
        assert_eq!(
            reopened.maybe_basic_filter_state().expect("state"),
            maybe_before_state
        );
        assert_eq!(reopened.load_prune_locks().expect("locks"), before_locks);
        assert_eq!(maybe_owner_bytes(&reopened), maybe_before_owner);
        assert_eq!(
            reopened.basic_filter_artifacts().expect("no rows"),
            (false, false)
        );
        drop(reopened);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn filter_index_work_loader_refuses_missing_weak_and_malformed_owned_protection() {
    for category in ["missing", "weak", "malformed", "owner"] {
        // Arrange
        let (path, store, positions, _) = active_store("bad-owner", 2);
        let (key, bytes) = match category {
            "missing" => (
                super::super::super::prune::PRUNE_LOCKS_KEY,
                super::super::super::prune::encode_prune_locks(&[]).expect("empty"),
            ),
            "weak" => (
                super::super::super::prune::PRUNE_LOCKS_KEY,
                super::super::super::prune::encode_prune_locks(&[
                    IndexInputProtection::FromHeight(1)
                        .maybe_prune_lock()
                        .expect("lock"),
                ])
                .expect("weak"),
            ),
            "malformed" => (super::super::super::prune::PRUNE_LOCKS_KEY, vec![255]),
            "owner" => (codec::ownership::OWNER_KEY, vec![1, 0, 3]),
            _ => panic!("unknown category"),
        };
        store
            .write_raw_for_test(StorageNamespace::BlockIndex, key, bytes.clone())
            .expect("corrupt fixture");
        // Act
        let result = store.maybe_basic_filter_work(&fence(&positions));
        // Assert
        assert!(result.is_err());
        assert_eq!(
            store
                .get_bytes(StorageNamespace::BlockIndex, key)
                .expect("untouched"),
            Some(bytes)
        );
        assert_eq!(
            store.basic_filter_artifacts().expect("no rows"),
            (false, false)
        );
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn filter_index_absent_work_requires_complete_artifact_absence() {
    // Arrange
    let path = temp_path("absent-work");
    let (positions, records) = fixtures(1);
    let store = FjallNodeStore::open(&path).expect("open");
    seed(&store, &positions);
    // Act / Assert
    assert!(
        store
            .maybe_basic_filter_work(&fence(&positions))
            .expect("absent")
            .is_none()
    );
    seed_orphan_records(&store, &records);
    assert!(store.maybe_basic_filter_work(&fence(&positions)).is_err());
    assert_eq!(maybe_owner_bytes(&store), None);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_legacy_owner_materialization_failure_keeps_conservative_state_after_reopen() {
    // Arrange
    let (path, store, positions, _) = active_store("legacy-owner-fault", 2);
    store
        .block_index
        .remove(codec::ownership::OWNER_KEY)
        .expect("legacy fixture");
    store
        .persist(StorageNamespace::BlockIndex, PersistMode::Sync)
        .expect("sync legacy");
    let maybe_before_state = store.maybe_basic_filter_state().expect("state");
    let locks = store.load_prune_locks().expect("locks");
    store.set_basic_filter_fault(FilterPublicationFault::BeforeCheckpoint);
    // Act
    assert!(store.maybe_basic_filter_work(&fence(&positions)).is_err());
    assert!(store.maybe_basic_filter_work(&fence(&positions)).is_err());
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("actual reopen");
    // Assert
    assert_eq!(maybe_owner_bytes(&reopened), None);
    assert_eq!(
        reopened.maybe_basic_filter_state().expect("state"),
        maybe_before_state
    );
    assert_eq!(reopened.load_prune_locks().expect("locks"), locks);
    assert!(
        reopened
            .maybe_basic_filter_work(&fence(&positions))
            .expect("recovered work")
            .is_some()
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_effective_owner_reload_is_bounded_independent_of_prefix_length() {
    // Arrange
    let (path, store, positions, records) = active_store("bounded-owner", 129);
    for batch in records.chunks(128) {
        publish(&store, &positions, batch);
    }
    store
        .filter_integrity_reads
        .store(0, std::sync::atomic::Ordering::Relaxed);
    let control = store.filter_publication_guard().expect("guard");
    // Act
    let owner = store
        .maybe_basic_filter_owner_guarded(&control)
        .expect("owner")
        .expect("present");
    let reads = store
        .filter_integrity_reads
        .load(std::sync::atomic::Ordering::Relaxed);
    // Assert
    assert_eq!(owner.checkpoint().checkpoint(), checkpoint(&records[128]));
    assert_eq!(
        reads, 5,
        "state, owner, endpoint, projection, immediate parent only"
    );
    drop(control);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}
