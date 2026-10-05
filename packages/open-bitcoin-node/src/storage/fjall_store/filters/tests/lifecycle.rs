// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};

fn retained_history(store: &FjallNodeStore, positions: &[ChainPosition]) {
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
                    &BlockUndo::default(),
                    PersistMode::Sync,
                )
                .expect("undo");
        }
    }
}

#[test]
fn filter_index_disable_invalidates_both_writers_and_retains_bytes_after_reopen() {
    // Arrange
    let path = temp_path("lifecycle-disable");
    let store = FjallNodeStore::open(&path).expect("store");
    let (positions, records) = fixtures(3);
    seed(&store, &positions);
    let ordinary = PruneLockInfo {
        name: "operator".to_owned(),
        height_first: 5,
        height_last: 10,
    };
    store
        .sync_prune_locks(std::slice::from_ref(&ordinary))
        .expect("ordinary lock");
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    publish_current(
        &store,
        &fence(&positions),
        checkpoint(&records[0]),
        checkpoint(&records[0]).input_protection(),
        &records[..1],
    )
    .expect("prefix");
    let state = store.maybe_basic_filter_state().expect("state");
    let state_bytes = store
        .get_bytes(StorageNamespace::BlockIndex, codec::STATE_KEY)
        .expect("state bytes");
    let record_key = codec::record_key(records[0].identity().block_hash());
    let record_bytes = store
        .get_bytes(StorageNamespace::BlockIndex, &record_key)
        .expect("record bytes");
    let projection_bytes = store
        .get_bytes(StorageNamespace::BlockIndex, &codec::active_key(0))
        .expect("projection bytes");
    let work = store
        .maybe_basic_filter_work(&fence(&positions))
        .expect("work")
        .expect("active");

    // Act
    store.disable_basic_filter_index().expect("disable");

    // Assert
    assert!(
        store
            .maybe_basic_filter_work(&fence(&positions))
            .expect("disabled")
            .is_none()
    );
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
                checkpoint(&records[2]).input_protection(),
                &records[1..]
            )
            .is_err()
    );
    assert_eq!(
        store.maybe_basic_filter_lifecycle().expect("owner"),
        Some(IndexLifecycle::Disabled {
            generation: IndexGeneration::new(1)
        })
    );
    assert_eq!(
        store.load_prune_locks().expect("locks"),
        vec![ordinary.clone()]
    );
    drop(work);
    drop(store);
    let store = FjallNodeStore::open(&path).expect("reopen");
    store
        .recover_basic_filter_index_before_prune(Some(positions[2].block_hash))
        .expect("disabled recovery");
    assert_eq!(store.maybe_basic_filter_state().expect("state"), state);
    assert_eq!(
        store
            .get_bytes(StorageNamespace::BlockIndex, codec::STATE_KEY)
            .expect("unchanged state bytes"),
        state_bytes
    );
    assert_eq!(
        store
            .get_bytes(StorageNamespace::BlockIndex, &record_key)
            .expect("unchanged row bytes"),
        record_bytes
    );
    assert_eq!(
        store
            .get_bytes(StorageNamespace::BlockIndex, &codec::active_key(0))
            .expect("unchanged projection bytes"),
        projection_bytes
    );
    assert_eq!(
        store.load_prune_locks().expect("ordinary lock survives"),
        vec![ordinary]
    );
    assert_eq!(
        store
            .load_basic_filter_record(records[0].identity().block_hash())
            .expect("row"),
        Some(records[0].clone())
    );
    assert!(
        store
            .load_basic_filter_record(records[1].identity().block_hash())
            .expect("stale row")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_disable_faults_preserve_truthful_reopen_and_retry_generation() {
    for (fault, disabled, released) in [
        (FilterPublicationFault::BeforeDisable, false, false),
        (FilterPublicationFault::AfterDisable, true, false),
        (FilterPublicationFault::BeforeRelease, true, false),
        (FilterPublicationFault::AfterRelease, true, true),
    ] {
        // Arrange
        let path = temp_path("lifecycle-disable-fault");
        let store = FjallNodeStore::open(&path).expect("store");
        let (positions, _) = fixtures(2);
        seed(&store, &positions);
        store
            .initialize_basic_filter_state(&fence(&positions))
            .expect("init");
        let state = store.maybe_basic_filter_state().expect("state");
        store.set_basic_filter_fault(fault);

        // Act
        assert!(store.disable_basic_filter_index().is_err());

        // Assert
        assert!(store.maybe_basic_filter_work(&fence(&positions)).is_err());
        drop(store);
        let store = FjallNodeStore::open(&path).expect("reopen");
        assert_eq!(store.maybe_basic_filter_state().expect("state"), state);
        let owner = store
            .maybe_basic_filter_lifecycle()
            .expect("owner")
            .expect("present");
        assert_eq!(matches!(owner, IndexLifecycle::Disabled { .. }), disabled);
        assert_eq!(
            store.load_prune_locks().expect("locks").is_empty(),
            released
        );
        store
            .recover_basic_filter_index_before_prune(Some(positions[1].block_hash))
            .expect("recovery");
        store.disable_basic_filter_index().expect("retry");
        store.disable_basic_filter_index().expect("idempotent");
        assert_eq!(
            store.maybe_basic_filter_lifecycle().expect("owner"),
            Some(IndexLifecycle::Disabled {
                generation: IndexGeneration::new(1)
            })
        );
        assert!(store.load_prune_locks().expect("released").is_empty());
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn filter_index_enable_preflights_inputs_before_acquiring_protection() {
    for missing_body in [true, false] {
        // Arrange
        let path = temp_path("lifecycle-missing-input");
        let store = FjallNodeStore::open(&path).expect("store");
        let (positions, _) = fixtures(2);
        seed(&store, &positions);
        store
            .initialize_basic_filter_state(&fence(&positions))
            .expect("init");
        store.disable_basic_filter_index().expect("disable");
        if !missing_body {
            store
                .save_block(
                    &fixture_block(positions[0].previous_block_hash(), 0),
                    PersistMode::Sync,
                )
                .expect("genesis");
            store
                .save_block(
                    &fixture_block(positions[1].previous_block_hash(), 1),
                    PersistMode::Sync,
                )
                .expect("body");
        }
        let state = store.maybe_basic_filter_state().expect("state");
        let owner = store.maybe_basic_filter_lifecycle().expect("owner");

        // Act
        assert!(store.enable_basic_filter_index(&fence(&positions)).is_err());

        // Assert
        assert_eq!(store.maybe_basic_filter_state().expect("state"), state);
        assert_eq!(store.maybe_basic_filter_lifecycle().expect("owner"), owner);
        assert!(store.load_prune_locks().expect("locks").is_empty());
        assert!(
            store
                .maybe_basic_filter_work(&fence(&positions))
                .expect("no work")
                .is_none()
        );
        retained_history(&store, &positions);
        store
            .enable_basic_filter_index(&fence(&positions))
            .expect("enable");
        store
            .enable_basic_filter_index(&fence(&positions))
            .expect("idempotent enable");
        assert_eq!(
            store.maybe_basic_filter_lifecycle().expect("owner"),
            Some(IndexLifecycle::Active {
                generation: IndexGeneration::new(2)
            })
        );
        assert_eq!(
            store.load_prune_locks().expect("protection"),
            vec![
                IndexInputProtection::FromHeight(0)
                    .maybe_prune_lock()
                    .expect("lock")
            ]
        );
        drop(store);
        let store = FjallNodeStore::open(&path).expect("reopen");
        assert!(
            store
                .maybe_basic_filter_work(&fence(&positions))
                .expect("work")
                .is_some()
        );
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn filter_index_enable_faults_reopen_disabled_or_atomically_protected_active() {
    for (fault, active) in [
        (FilterPublicationFault::BeforeEnable, false),
        (FilterPublicationFault::AfterEnable, true),
    ] {
        // Arrange
        let path = temp_path("lifecycle-enable-fault");
        let store = FjallNodeStore::open(&path).expect("store");
        let (positions, _) = fixtures(2);
        seed(&store, &positions);
        retained_history(&store, &positions);
        store
            .initialize_basic_filter_state(&fence(&positions))
            .expect("init");
        store.disable_basic_filter_index().expect("disable");
        store.set_basic_filter_fault(fault);

        // Act
        assert!(store.enable_basic_filter_index(&fence(&positions)).is_err());

        // Assert
        assert!(store.maybe_basic_filter_work(&fence(&positions)).is_err());
        drop(store);
        let store = FjallNodeStore::open(&path).expect("reopen");
        assert_eq!(
            matches!(
                store
                    .maybe_basic_filter_lifecycle()
                    .expect("owner")
                    .expect("saved"),
                IndexLifecycle::Active { .. }
            ),
            active
        );
        assert_eq!(store.load_prune_locks().expect("locks").is_empty(), !active);
        store
            .recover_basic_filter_index_before_prune(Some(positions[1].block_hash))
            .expect("recovery");
        store
            .enable_basic_filter_index(&fence(&positions))
            .expect("retry");
        assert_eq!(
            store.maybe_basic_filter_lifecycle().expect("owner"),
            Some(IndexLifecycle::Active {
                generation: IndexGeneration::new(2)
            })
        );
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn filter_index_generation_exhaustion_never_wraps_or_mutates_authority() {
    for active in [true, false] {
        // Arrange
        let path = temp_path("lifecycle-exhaustion");
        let store = FjallNodeStore::open(&path).expect("store");
        let (positions, _) = fixtures(1);
        seed(&store, &positions);
        store
            .initialize_basic_filter_state(&fence(&positions))
            .expect("init");
        let lifecycle = if active {
            IndexLifecycle::Active {
                generation: IndexGeneration::new(u64::MAX),
            }
        } else {
            IndexLifecycle::Disabled {
                generation: IndexGeneration::new(u64::MAX),
            }
        };
        store
            .write_raw_for_test(
                StorageNamespace::BlockIndex,
                codec::ownership::OWNER_KEY,
                codec::ownership::encode_owner(lifecycle).to_vec(),
            )
            .expect("exhaustion fixture");
        let state = store.maybe_basic_filter_state().expect("state");
        let locks = store.load_prune_locks().expect("locks");

        // Act
        let result = if active {
            store.disable_basic_filter_index()
        } else {
            store.enable_basic_filter_index(&fence(&positions))
        };

        // Assert
        assert!(
            result
                .expect_err("exhaustion")
                .to_string()
                .contains("generation exhausted")
        );
        assert_eq!(store.maybe_basic_filter_state().expect("state"), state);
        assert_eq!(store.load_prune_locks().expect("locks"), locks);
        assert_eq!(
            store.maybe_basic_filter_lifecycle().expect("owner"),
            Some(lifecycle)
        );
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn filter_index_absent_activation_requires_complete_genesis_through_tip_history() {
    // Arrange
    let path = temp_path("lifecycle-absent");
    let store = FjallNodeStore::open(&path).expect("store");
    let (positions, _) = fixtures(2);
    seed(&store, &positions);

    // Act
    assert!(store.enable_basic_filter_index(&fence(&positions)).is_err());

    // Assert
    assert!(store.maybe_basic_filter_state().expect("absent").is_none());
    assert!(
        store
            .maybe_basic_filter_lifecycle()
            .expect("absent")
            .is_none()
    );
    assert!(store.load_prune_locks().expect("absent").is_empty());
    retained_history(&store, &positions);
    store
        .enable_basic_filter_index(&fence(&positions))
        .expect("activation");
    assert_eq!(
        store.maybe_basic_filter_lifecycle().expect("owner"),
        Some(IndexLifecycle::Active {
            generation: IndexGeneration::new(0)
        })
    );
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("checkpoint"),
        Some(FilterCheckpoint::new(IndexPrefix::Empty))
    );
    assert_eq!(
        store.load_prune_locks().expect("locks"),
        vec![
            IndexInputProtection::FromHeight(0)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn filter_index_reenable_reconciles_to_recovered_coins_without_erasing_suffix() {
    // Arrange
    let path = temp_path("lifecycle-reconcile");
    let store = FjallNodeStore::open(&path).expect("store");
    let (positions, records) = fixtures(3);
    seed(&store, &positions);
    store
        .initialize_basic_filter_state(&fence(&positions))
        .expect("init");
    publish_current(
        &store,
        &fence(&positions),
        checkpoint(&records[2]),
        checkpoint(&records[2]).input_protection(),
        &records,
    )
    .expect("prefix");
    store.disable_basic_filter_index().expect("disable");
    seed(&store, &positions[..2]);
    drop(store);
    let store = FjallNodeStore::open(&path).expect("reopen Disabled against recovered B");
    store
        .recover_basic_filter_index_before_prune(Some(positions[1].block_hash))
        .expect("retained Disabled recovery");

    // Act
    store
        .enable_basic_filter_index(&fence(&positions[..2]))
        .expect("reconcile and protect");

    // Assert
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("reconciled"),
        Some(checkpoint(&records[1]))
    );
    assert_eq!(
        store
            .load_basic_filter_record(records[2].identity().block_hash())
            .expect("retained suffix"),
        Some(records[2].clone())
    );
    assert_eq!(
        store.basic_filter_projection(2).expect("hidden suffix"),
        records[2].identity().block_hash()
    );
    assert!(
        store
            .maybe_active_basic_filter_record(2)
            .expect("hidden from active prefix")
            .is_none()
    );
    assert_eq!(
        store.load_prune_locks().expect("covering protection"),
        vec![
            IndexInputProtection::FromHeight(2)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    drop(store);
    let store = FjallNodeStore::open(&path).expect("Active reopen");
    store
        .recover_basic_filter_index_before_prune(Some(positions[1].block_hash))
        .expect("durable reconciliation");
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("checkpoint"),
        Some(checkpoint(&records[1]))
    );
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}
