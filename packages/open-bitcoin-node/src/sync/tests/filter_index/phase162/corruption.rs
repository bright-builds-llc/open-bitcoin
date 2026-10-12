// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Persisted corruption after real readiness completion, without repairing bytes.

use super::{missing_history::*, publication_faults::*, *};
use crate::status::{
    BasicIndexFailureCategory, BasicIndexOperationalState, BasicIndexOperatorStatus,
    FieldAvailability,
};
use crate::storage::fjall_store::filters::query::QueryRecordFault;
use crate::{BasicFilterQuery, BasicFilterQueryError};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

fn poll(
    barrier: &mut crate::BasicFilterReadBarrier,
) -> Poll<Result<crate::BasicFilterReadCompletion, BasicFilterQueryError>> {
    Pin::new(barrier).poll(&mut Context::from_waker(Waker::noop()))
}

fn status(fixture: &AcceptedRecipe) -> BasicIndexOperatorStatus {
    fixture
        .runtime
        .network_handle()
        .operator_snapshot()
        .expect("bounded operator snapshot")
        .basic_index()
        .clone()
}

fn assert_failure(fixture: &AcceptedRecipe, before: &BasicIndexOperatorStatus) {
    let failed = status(fixture);
    assert_eq!(
        failed.maybe_failure,
        FieldAvailability::available(Some(BasicIndexFailureCategory::IntegrityOrInvalidFacts))
    );
    assert_eq!(
        failed.operational_state,
        FieldAvailability::available(BasicIndexOperationalState::Failed)
    );
    assert_eq!(failed.initial_sync_complete, before.initial_sync_complete);
    assert_eq!(failed.maybe_accepted_height, before.maybe_accepted_height);
    assert_eq!(failed.configured_indexing, before.configured_indexing);
    assert_eq!(failed.protection, before.protection);
    assert!(matches!(
        failed.maybe_processed_height,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        failed.maybe_safe_durable_height,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        failed.maybe_lag_blocks,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        failed.retention,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        failed.retained_availability,
        FieldAvailability::Unavailable { .. }
    ));
    let persisted = raw(fixture.runtime.store());
    for _ in 0..3 {
        assert_eq!(status(fixture), failed);
        let json = serde_json::to_string(&failed).expect("redacted operator projection");
        for private in [
            "block_hash",
            "script",
            "backend-read-marker",
            "filter:record:",
            "/private/",
        ] {
            assert!(!json.contains(private));
        }
        assert!(json.len() <= 4096);
    }
    assert_eq!(
        raw(fixture.runtime.store()),
        persisted,
        "status observation has no repair effects"
    );
}

fn ready(fixture: &AcceptedRecipe, hash: BlockHash) -> crate::BasicFilterReadBarrier {
    let BasicFilterQuery::Pending(mut barrier) = fixture
        .runtime
        .network_handle()
        .basic_filter_query(hash)
        .expect("real pending query")
    else {
        panic!("lagged synchronized owner must capture real pending frontier");
    };
    assert!(poll(&mut barrier).is_pending());
    barrier
}

fn completion(barrier: &mut crate::BasicFilterReadBarrier) -> crate::BasicFilterReadCompletion {
    let Poll::Ready(Ok(completion)) = poll(barrier) else {
        panic!("actual ordinary publication must earn readiness completion");
    };
    completion
}

fn assert_integrity(result: Result<BasicFilterQuery, BasicFilterQueryError>) {
    assert!(matches!(
        result,
        Err(BasicFilterQueryError::Storage(StorageError::Corruption {
            namespace: StorageNamespace::BlockIndex,
            action: StorageRecoveryAction::Repair,
            ..
        }))
    ));
}

fn row_fault(kind: u8, final_first: bool) {
    // Arrange: both opaque completions are earned before actual persisted tampering.
    let cleanup = Cleanup::new();
    let (fixture, stale) = saved_with_stale(&cleanup);
    flush_only(&fixture.runtime);
    let hash = fixture.receipts[104].block_hash;
    let mut barrier = ready(&fixture, hash);
    let mut second_barrier = ready(&fixture, hash);
    fixture.finish();
    let completion = completion(&mut barrier);
    let stale_completion = self::completion(&mut second_barrier);
    let store = fixture.runtime.store();
    assert_authority(store, &fixture.receipts, 104);
    assert_pairs(store, &fixture.receipts, 104);
    let prefix = rows(store, &fixture.receipts[..102]);
    let maybe_old_checkpoint = store.maybe_basic_filter_checkpoint().expect("real safe104");
    assert_eq!(
        maybe_old_checkpoint
            .expect("earned saved endpoint")
            .prefix(),
        IndexPrefix::Committed(rows(store, &fixture.receipts)[104].identity())
    );
    let maybe_old_state = store
        .maybe_basic_filter_state()
        .expect("persisted checkpoint scalar identity");
    let old_locks = store.load_prune_locks().expect("real protected105");
    let maybe_old_owner = store
        .maybe_basic_filter_lifecycle_for_test()
        .expect("owner");
    let before = raw(store);
    let before_status = status(&fixture);
    let fault = match kind {
        0 => QueryRecordFault::CorruptTarget,
        1 => QueryRecordFault::CorruptParent,
        2 => QueryRecordFault::MissingParent,
        _ => panic!("bounded fault kind"),
    };
    store
        .inject_query_record_fault_for_test(hash, fault)
        .expect("precise existing immutable target/predecessor fault");
    let corrupted = raw(store);
    let changed_key =
        codec::record_key(fixture.receipts[if kind == 0 { 104 } else { 103 }].block_hash);
    let unaffected = |rows: &[(Vec<u8>, Vec<u8>)]| {
        rows.iter()
            .filter(|(key, _)| key.as_slice() != changed_key.as_bytes())
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_ne!(before[0], corrupted[0]);
    assert_eq!(
        unaffected(&before[0]),
        unaffected(&corrupted[0]),
        "exactly one immutable row is damaged"
    );
    assert_eq!(
        before[1..],
        corrupted[1..],
        "all metadata/undo/coins bytes survive tampering"
    );
    // Act: exercise both ordinary immediate and readiness-final reads in each order.
    let network = fixture.runtime.network_handle();
    if final_first {
        assert_integrity(network.complete_basic_filter_read(hash, completion));
    } else {
        assert_integrity(network.basic_filter_query(hash));
        assert!(
            network
                .complete_basic_filter_read(hash, completion)
                .is_err()
        );
    }
    // Assert: discovered integrity revokes the other already earned authority.
    assert!(
        network
            .complete_basic_filter_read(hash, stale_completion)
            .is_err()
    );
    assert!(network.basic_filter_query(hash).is_err());
    assert_failure(&fixture, &before_status);
    assert_eq!(
        raw(store),
        corrupted,
        "reads never clear/repair the corrupt bytes"
    );
    assert_eq!(rows(store, &fixture.receipts[..102]), prefix);
    assert_eq!(
        store
            .load_basic_filter_record(stale.identity().block_hash())
            .expect("unrelated stale"),
        Some(stale)
    );
    assert!(
        matches!(
            store.maybe_basic_filter_checkpoint(),
            Err(StorageError::Corruption { .. })
        ),
        "corrupt endpoint ancestry cannot decode as healthy checkpoint"
    );
    assert_eq!(
        store
            .maybe_basic_filter_state()
            .expect("saved scalar truth"),
        maybe_old_state
    );
    assert_eq!(
        store.load_prune_locks().expect("retained protection"),
        old_locks
    );
    assert_eq!(
        store
            .maybe_basic_filter_lifecycle_for_test()
            .expect("retained owner"),
        maybe_old_owner
    );
    assert_authority(store, &fixture.receipts, 104);
    assert_pairs(store, &fixture.receipts, 104);
    let positions = fixture.receipts.clone();
    drop(network);
    drop(barrier);
    drop(second_barrier);
    drop(fixture);
    for _ in 0..2 {
        assert!(matches!(
            open(&cleanup.path),
            Err(SyncRuntimeError::Storage(StorageError::Corruption {
                namespace: StorageNamespace::BlockIndex,
                action: StorageRecoveryAction::Repair,
                ..
            }))
        ));
        assert_eq!(
            snapshot(&cleanup.path),
            corrupted,
            "configured closed reopen refuses without repair"
        );
    }
    let store = FjallNodeStore::open(&cleanup.path).expect("cold post-refusal inspection");
    assert_authority(&store, &positions, 104);
    assert_pairs(&store, &positions, 104);
    assert_eq!(rows(&store, &positions[..102]), prefix);
    assert!(matches!(
        store.maybe_basic_filter_checkpoint(),
        Err(StorageError::Corruption { .. })
    ));
    assert_eq!(
        store
            .maybe_basic_filter_state()
            .expect("cold saved scalar truth"),
        maybe_old_state
    );
    assert_eq!(
        store.load_prune_locks().expect("cold protection"),
        old_locks
    );
    drop(store);
    eprintln!(
        "phase162 corruption: kind={kind} final_first={final_first} integrity_refused=true stale_completion_refused=true cold_refusals=2 preserved=true"
    );
}

#[test]
fn phase162_corruption_target_immediate() {
    row_fault(0, false);
}
#[test]
fn phase162_corruption_target_final() {
    row_fault(0, true);
}
#[test]
fn phase162_corruption_predecessor_immediate() {
    row_fault(1, false);
}
#[test]
fn phase162_corruption_predecessor_final() {
    row_fault(1, true);
}
#[test]
fn phase162_corruption_missing_predecessor_immediate() {
    row_fault(2, false);
}
#[test]
fn phase162_corruption_missing_predecessor_final() {
    row_fault(2, true);
}

#[test]
fn phase162_corruption_ordinary_absence_is_missing_without_poison() {
    // Arrange
    let cleanup = Cleanup::new();
    let (fixture, _) = saved_with_stale(&cleanup);
    flush_only(&fixture.runtime);
    let hash = fixture.receipts[104].block_hash;
    let mut barrier = ready(&fixture, hash);
    fixture.finish();
    let completion = completion(&mut barrier);
    let before = status(&fixture);
    // Act: unknown block and absent known target are distinct legitimate controls.
    assert!(matches!(
        fixture
            .runtime
            .network_handle()
            .basic_filter_query(BlockHash::from_byte_array([77; 32]))
            .expect("unknown identity"),
        BasicFilterQuery::UnknownBlock
    ));
    fixture
        .runtime
        .store()
        .inject_query_record_fault_for_test(hash, QueryRecordFault::MissingTarget)
        .expect("ordinary target absence");
    assert!(matches!(
        fixture
            .runtime
            .network_handle()
            .basic_filter_query(hash)
            .expect("missing is not corrupt ancestry"),
        BasicFilterQuery::Missing { .. }
    ));
    assert!(matches!(
        fixture
            .runtime
            .network_handle()
            .complete_basic_filter_read(hash, completion)
            .expect("final missing retains provenance"),
        BasicFilterQuery::Missing {
            provenance: crate::BasicBlockValidationProvenance::ScriptsValid,
            initially_synchronized: true
        }
    ));
    // Assert
    assert_eq!(status(&fixture), before);
    assert!(
        fixture
            .runtime
            .store()
            .basic_filter_operator_observation()
            .expect("absence does not poison")
            .maybe_read_fault
            .is_none()
    );
    drop(barrier);
    drop(fixture);
    let corrupted = snapshot(&cleanup.path);
    assert!(
        open(&cleanup.path).is_err(),
        "dangling active projection cannot earn a fresh recovered epoch"
    );
    assert_eq!(snapshot(&cleanup.path), corrupted);
}
