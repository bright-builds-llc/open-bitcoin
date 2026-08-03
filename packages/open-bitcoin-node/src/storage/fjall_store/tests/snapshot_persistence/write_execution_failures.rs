// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn prepared_mempool_snapshot_executor_reports_abort_dispatch_without_mutating_bytes() {
    // Arrange
    let path = temp_store_path("prepared-mempool-abort-dispatch");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let persisted_before_failure = mempool_snapshot();
    store
        .save_mempool_snapshot(&persisted_before_failure, PersistMode::Sync)
        .expect("save pre-existing snapshot");
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(500_000), CheckpointTrigger::Periodic)
        .expect("snapshot should prepare");
    let expected = StorageError::Corruption {
        namespace: StorageNamespace::Mempool,
        detail: "injected encode failure".to_string(),
        action: StorageRecoveryAction::Repair,
    };
    handle.fail_next_checkpoint_abort_dispatch_for_test();

    // Act
    let result = FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
        &handle,
        prepared,
        |_| Err(expected),
        |_, _| panic!("save must not run after encoding fails"),
        || PolicyTime::new(500_001),
    );
    let persisted_after_failure = store
        .load_mempool_snapshot_with_limits(snapshot_decode_limits())
        .expect("load pre-existing snapshot");
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(500_002), 60)
        .expect("checkpoint evidence");

    // Assert
    assert!(matches!(
        result,
        Err(ref error) if error.failure() == SnapshotWriteFailure::AbortDispatch
    ));
    assert_eq!(persisted_after_failure, Some(persisted_before_failure));
    assert_eq!(evidence.maybe_failure, None);
    assert_eq!(evidence.maybe_in_flight_generation, Some(0));

    remove_dir_if_exists(&path);
}
