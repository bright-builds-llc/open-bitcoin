// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn apply_recovery_one_head_and_inconsistent_count() {
    // Arrange
    let (path, store) = open_temp_store("recovery-decision");
    let view = FjallCoinsView::from_store(&store);

    // Act
    let kept = match apply_recovery_decision(&store, view, RecoveryDecision::OneHead) {
        Ok(view) => view,
        Err(error) => panic!("one head: {error:?}"),
    };
    let inconsistent = expect_error(
        apply_recovery_decision(
            &store,
            FjallCoinsView::from_store(&store),
            RecoveryDecision::InconsistentOtherCount { count: 3 },
        ),
        "inconsistent",
    );

    // Assert
    assert_eq!(kept.head_blocks().expect("heads"), Vec::<BlockHash>::new());
    assert!(
        matches!(
            inconsistent,
            StorageError::Corruption {
                namespace: StorageNamespace::Coins,
                ref detail,
                action: StorageRecoveryAction::Repair,
            } if detail.contains("unexpected head_blocks count 3")
        ),
        "expected inconsistent count, got {inconsistent:?}"
    );
    remove_dir_if_exists(&path);
}

#[test]
fn fjall_store_implements_flush_persist_sink() {
    // Arrange
    let (path, mut store) = open_temp_store("fjall-sink");
    let block_header = header(BlockHash::from_byte_array([0_u8; 32]), 9);
    let block = Block {
        header: block_header.clone(),
        transactions: vec![coinbase(0, 50)],
    };
    let hash = block_hash(&block_header);

    // Act
    let block_ok = store.persist_block(&block);
    let undo_ok = store.persist_undo(hash, &BlockUndo::default());
    let empty_headers = store.persist_header_entries(&[]);
    let headers_ok = store.persist_header_entries(&[header_entry(block_header, 0, 1)]);

    // Assert
    assert!(block_ok.is_ok(), "{block_ok:?}");
    assert!(undo_ok.is_ok(), "{undo_ok:?}");
    assert!(empty_headers.is_ok(), "{empty_headers:?}");
    assert!(headers_ok.is_ok(), "{headers_ok:?}");
    remove_dir_if_exists(&path);
}

#[test]
fn probe_disk_free_bytes_is_defined() {
    // Arrange / Act
    let root_free = probe_disk_free_bytes(Path::new("/"));
    let missing_free = probe_disk_free_bytes(Path::new(
        "/open-bitcoin-missing-datadir-for-disk-probe-142",
    ));

    // Assert
    assert_ne!(
        root_free,
        u64::MAX,
        "production probe must not stub available space as u64::MAX"
    );
    assert_eq!(missing_free, 0);
}

#[test]
fn execute_flush_periodic_due_uses_sync_from_decide_flush() {
    // Arrange
    let mut lifecycle = FlushLifecycle::ready_for_test(
        default_coins_cache_byte_limit(),
        0,
        FlushPolicyTime::from_unix_seconds(1),
        false,
    );
    let mut cache = CoinsCache::from_parent(RecordingCoinsView {
        writes: Cell::new(0),
    });

    // Act
    let execution = lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Periodic,
            FlushPolicyTime::from_unix_seconds(2),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("periodic sync");

    // Assert
    assert!(
        matches!(execution.decision, FlushDecision::Sync(_)),
        "Periodic due must map to cache Sync, got {:?}",
        execution.decision
    );
    assert!(execution.wrote_coins);
    assert_eq!(cache.parent().writes.get(), 1);
}

#[test]
fn persist_calls_execute_flush_ifneeded() {
    // Arrange / Act / Assert
    let persist_src = include_str!("../../../chainstate.rs");
    assert!(
        persist_src.contains("self.flush_lifecycle.execute_flush"),
        "ManagedChainstate::persist must call execute_flush"
    );
    assert!(
        persist_src.contains("FlushMode::IfNeeded"),
        "ManagedChainstate::persist must use IfNeeded"
    );
    assert!(
        !persist_src.contains("save_snapshot(self.chainstate.snapshot())"),
        "ManagedChainstate::persist must not write leftover snapshots"
    );
}
