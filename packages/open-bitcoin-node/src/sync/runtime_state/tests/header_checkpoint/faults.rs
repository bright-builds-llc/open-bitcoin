// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/headerssync.cpp

use super::*;
use crate::storage::StorageRecoveryAction;

pub(super) fn settle(
    runtime: &DurableSyncRuntime,
) -> Result<crate::chainstate::FlushExecution, crate::ManagedNetworkAuthorityError> {
    runtime
        .network_handle()
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(30_000), u64::MAX)
}

pub(super) fn snapshots(
    store: &crate::FjallNodeStore,
) -> (
    Option<crate::storage::snapshot_codec::StoredHeaderEntries>,
    Option<crate::storage::snapshot_codec::StoredHeaderEntries>,
) {
    (
        store.load_header_entries().expect("headers"),
        store.load_block_index_entries().expect("index"),
    )
}

#[test]
fn phase162_header_checkpoint_snapshot_before_commit_preserves_old_snapshot_and_retry() {
    // Arrange
    let (dir, runtime, stale) = stale_runtime_configured(true);
    let store = runtime.store();
    let before = snapshots(store);
    let coins = store.coins_view().best_block().expect("B");
    let metadata = store.load_chain_meta_for_open().expect("metadata");
    let safe = store
        .maybe_basic_filter_checkpoint()
        .expect("safe")
        .expect("actual checkpoint");
    let immutable = store
        .load_basic_filter_record(stale)
        .expect("immutable")
        .expect("actual stale filter");
    let protection = crate::FjallChainstateStore::from_store(store.clone())
        .load_prune_protection()
        .expect("protection");
    let attempts = store.snapshot_attempts();
    store.set_validation_history_fault(HistoryPublicationFault::SnapshotBeforeCommitFailure);
    // Act
    assert!(settle(&runtime).is_err());
    // Assert
    assert_eq!(snapshots(store), before);
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("unchanged safe"),
        Some(safe)
    );
    assert_eq!(
        store
            .load_basic_filter_record(stale)
            .expect("unchanged immutable"),
        Some(immutable)
    );
    assert_eq!(store.snapshot_attempts(), attempts.map(|value| value + 1));
    assert_eq!(store.coins_view().best_block().expect("B unchanged"), coins);
    assert_eq!(
        store
            .load_chain_meta_for_open()
            .expect("metadata unchanged"),
        metadata
    );
    assert_eq!(
        crate::FjallChainstateStore::from_store(store.clone())
            .load_prune_protection()
            .expect("protection unchanged"),
        protection
    );
    assert!(store.has_block(stale).expect("body"));
    assert!(store.has_undo(stale).expect("undo"));
    assert!(!store.load_have_pruned().expect("no prune acknowledgement"));
    assert!(
        settle(&runtime)
            .expect("consumed fault allows retry")
            .wrote_coins
    );
    let (headers, index) = snapshots(store);
    assert_eq!(headers, index);
    assert_eq!(headers.expect("new snapshot").entries.len(), 4);
    drop(runtime);
    let reopened = crate::FjallNodeStore::open(&dir).expect("closed retry reopen");
    assert_eq!(
        snapshots(&reopened).0.expect("durable retry").entries.len(),
        4
    );
    assert_eq!(
        reopened.validation_provenance(stale).expect("accepted"),
        ValidationProvenance::ScriptsValid
    );
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase162_header_checkpoint_snapshot_after_commit_is_durable_but_failed_until_reopen() {
    // Arrange
    let (dir, runtime, stale) = stale_runtime_configured(true);
    let store = runtime.store();
    let coins = store.coins_view().best_block().expect("B");
    let metadata = store.load_chain_meta_for_open().expect("metadata");
    let safe = store
        .maybe_basic_filter_checkpoint()
        .expect("safe")
        .expect("actual checkpoint");
    let immutable = store
        .load_basic_filter_record(stale)
        .expect("immutable")
        .expect("actual stale filter");
    let protection = crate::FjallChainstateStore::from_store(store.clone())
        .load_prune_protection()
        .expect("protection");
    store.set_validation_history_fault(HistoryPublicationFault::SnapshotAfterCommitFailure);
    // Act
    assert!(settle(&runtime).is_err());
    // Assert
    let (headers, index) = snapshots(store);
    assert_eq!(headers, index);
    assert_eq!(headers.expect("committed despite error").entries.len(), 4);
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("unchanged safe"),
        Some(safe)
    );
    assert_eq!(
        store
            .load_basic_filter_record(stale)
            .expect("unchanged immutable"),
        Some(immutable)
    );
    assert_eq!(
        crate::FjallChainstateStore::from_store(store.clone())
            .load_prune_protection()
            .expect("unchanged protection"),
        protection
    );
    assert_eq!(store.coins_view().best_block().expect("old B"), coins);
    assert_eq!(
        store.load_chain_meta_for_open().expect("old metadata"),
        metadata
    );
    assert!(store.has_block(stale).expect("body"));
    assert!(store.has_undo(stale).expect("undo"));
    assert!(store.recovered_validation_history().is_err());
    assert!(settle(&runtime).is_err());
    drop(runtime);
    let reopened = DurableSyncRuntime::open(
        crate::FjallNodeStore::open(&dir).expect("exclusive"),
        crate::SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
    )
    .expect("derive durable state");
    assert_eq!(
        reopened
            .store()
            .validation_provenance(stale)
            .expect("recovered status"),
        ValidationProvenance::ScriptsValid
    );
    assert!(
        settle(&reopened)
            .expect("normal retry after reopen")
            .wrote_coins
    );
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase162_header_checkpoint_skipped_turns_leave_all_attempts_zero_and_fault_unconsumed() {
    // Arrange
    let (dir, runtime, _) = stale_runtime();
    let handle = runtime.network_handle();
    handle
        .set_coins_next_write(FlushPolicyTime::new(u64::MAX))
        .expect("not due");
    let before = snapshots(runtime.store());
    let attempts = runtime.store().snapshot_attempts();
    runtime
        .store()
        .set_validation_history_fault(HistoryPublicationFault::SnapshotBeforeCommitFailure);
    // Act
    for mode in [FlushMode::None, FlushMode::IfNeeded, FlushMode::Periodic] {
        assert!(
            !handle
                .flush_coins(mode, FlushPolicyTime::new(1), u64::MAX)
                .expect("skip")
                .wrote_coins
        );
    }
    assert!(
        handle
            .flush_coins(FlushMode::Always, FlushPolicyTime::new(1), 0)
            .is_err()
    );
    // Assert
    assert_eq!(runtime.store().snapshot_attempts(), attempts);
    assert_eq!(snapshots(runtime.store()), before);
    assert!(
        settle(&runtime).is_err(),
        "next selected turn consumes snapshot fault"
    );
    assert_eq!(
        runtime.store().snapshot_attempts(),
        attempts.map(|value| value + 1)
    );
    assert!(settle(&runtime).expect("retry").wrote_coins);
    drop(handle);
    drop(runtime);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase162_header_checkpoint_snapshot_faults_keep_typed_headers_backend_failure() {
    // Arrange
    for fault in [
        HistoryPublicationFault::SnapshotBeforeCommitFailure,
        HistoryPublicationFault::SnapshotAfterCommitFailure,
    ] {
        let (dir, runtime, _) = stale_runtime();
        let live = runtime
            .network_handle()
            .peer_manager_snapshot()
            .expect("live");
        let snapshot = crate::network::validation_history::TrustedHeaderSnapshot::maybe_capture(
            runtime.store().clone(),
            live.header_store(),
        )
        .expect("nonempty");
        runtime.store().set_validation_history_fault(fault);
        // Act
        let error = snapshot.persist().expect_err("actual snapshot batch fault");
        // Assert
        assert!(matches!(
            error,
            StorageError::BackendFailure {
                namespace: StorageNamespace::Headers,
                action: StorageRecoveryAction::Restart,
                ..
            }
        ));
        drop(snapshot);
        drop(runtime);
        std::fs::remove_dir_all(dir).expect("cleanup");
    }
}
