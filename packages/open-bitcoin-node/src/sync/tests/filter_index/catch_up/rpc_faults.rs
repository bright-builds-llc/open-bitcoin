// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp

//! Private fault evidence through the exact production authority used by RPC.

use super::*;
use crate::storage::fjall_store::filters::{FilterPublicationFault, query::QueryRecordFault};
use crate::storage::validation_history::ValidationProvenance;
use crate::{BasicFilterQuery, BasicFilterQueryError, BasicFilterReadFailure};
use open_bitcoin_core::chainstate::{FlushMode, FlushPolicyTime};
use open_bitcoin_core::consensus::{ConsensusParams, ScriptVerifyFlags};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

static NEXT_FAULT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[path = "rpc_faults/captures.rs"]
mod captures;
fn read_fault_status(runtime: &DurableSyncRuntime) -> crate::status::BasicIndexOperatorStatus {
    runtime
        .network
        .operator_snapshot()
        .expect("operator")
        .basic_index()
        .clone()
}

fn assert_sticky_read_fault(
    runtime: &DurableSyncRuntime,
    before: &crate::status::BasicIndexOperatorStatus,
    backend: bool,
) {
    use crate::status::{
        BasicIndexFailureCategory as Failure, BasicIndexOperationalState as State,
        FieldAvailability as Field,
    };
    let expected = read_fault_status(runtime);
    assert_eq!(
        expected.maybe_failure,
        Field::available(Some(if backend {
            Failure::Backend
        } else {
            Failure::IntegrityOrInvalidFacts
        }))
    );
    assert_eq!(expected.operational_state, Field::available(State::Failed));
    assert_eq!(expected.initial_sync_complete, before.initial_sync_complete);
    assert_eq!(expected.configured_indexing, before.configured_indexing);
    assert_eq!(expected.maybe_accepted_height, before.maybe_accepted_height);
    assert_eq!(expected.protection, before.protection);
    assert!(matches!(
        expected.maybe_processed_height,
        Field::Unavailable { .. }
    ));
    assert!(matches!(
        expected.maybe_safe_durable_height,
        Field::Unavailable { .. }
    ));
    assert!(matches!(
        expected.maybe_lag_blocks,
        Field::Unavailable { .. }
    ));
    assert!(matches!(expected.retention, Field::Unavailable { .. }));
    assert!(matches!(
        expected.retained_availability,
        Field::Unavailable { .. }
    ));
    let rows = runtime.store().basic_operator_namespaces_for_test();
    let scans = runtime.store().basic_filter_validation_scans_for_test();
    let reads = runtime.store().basic_filter_point_reads_for_test();
    let progress = runtime
        .network
        .maybe_basic_index_progress()
        .expect("progress");
    for _ in 0..32 {
        assert_eq!(read_fault_status(runtime), expected);
        let observation = runtime
            .store()
            .basic_filter_operator_observation()
            .expect("bounded capture");
        assert_eq!(observation.work.metadata_reads, 2);
        assert!(observation.work.metadata_bytes <= 4096);
        let json = serde_json::to_string(&expected).expect("JSON");
        assert!(json.len() <= 4096);
        for private in ["/private/", "backend-read-marker", "block_hash", "script"] {
            assert!(!json.contains(private), "private evidence {private}");
        }
    }
    assert_eq!(runtime.store().basic_operator_namespaces_for_test(), rows);
    assert_eq!(
        runtime.store().basic_filter_validation_scans_for_test(),
        scans
    );
    assert_eq!(runtime.store().basic_filter_point_reads_for_test(), reads);
    assert_eq!(
        runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress"),
        progress
    );
}

#[test]
fn phase161_read_fault_callers_immediate_and_final_rpc_fault_matrix() {
    for final_read in [false, true] {
        for kind in 0..4 {
            // Arrange
            let (history, runtime) = genuine();
            let hash = history.records[2].identity().block_hash();
            let maybe_completion = if final_read {
                let BasicFilterQuery::Pending(mut barrier) =
                    runtime.network.basic_filter_query(hash).expect("pending")
                else {
                    panic!("pending");
                };
                runtime
                    .network
                    .drive_basic_filter_index_turn()
                    .expect("publication");
                let Poll::Ready(Ok(completion)) = poll(&mut barrier) else {
                    panic!("completion");
                };
                Some(completion)
            } else {
                runtime
                    .network
                    .drive_basic_filter_index_turn()
                    .expect("publication");
                None
            };
            let before = read_fault_status(&runtime);
            if kind == 3 {
                runtime
                    .store()
                    .set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
            } else {
                let fault = match kind {
                    0 => QueryRecordFault::CorruptTarget,
                    1 => QueryRecordFault::CorruptParent,
                    _ => QueryRecordFault::MissingParent,
                };
                runtime
                    .store()
                    .inject_query_record_fault_for_test(hash, fault)
                    .expect("fault");
            }
            // Act
            let result = match maybe_completion {
                Some(completion) => runtime.network.complete_basic_filter_read(hash, completion),
                None => runtime.network.basic_filter_query(hash),
            };
            // Assert
            assert!(matches!(result, Err(BasicFilterQueryError::Storage(_))));
            assert_sticky_read_fault(&runtime, &before, kind == 3);
            assert!(runtime.network.basic_filter_query(hash).is_err());
            assert_sticky_read_fault(&runtime, &before, kind == 3);
            drop(runtime);
            if kind == 3 {
                let recovered = DurableSyncRuntime::open_configured(
                    FjallNodeStore::open(&history.path).expect("actual all-handle reopen"),
                    SyncRuntimeConfig {
                        network: SyncNetwork::Regtest,
                        ..sync_config()
                    },
                    crate::chainstate::BasicFilterStartupMode::Enabled,
                )
                .expect("complete verified transient recovery");
                assert_eq!(
                    read_fault_status(&recovered).maybe_failure,
                    crate::status::FieldAvailability::available(None)
                );
                assert!(
                    recovered
                        .store()
                        .basic_filter_operator_observation()
                        .expect("capture")
                        .maybe_read_fault
                        .is_none()
                );
                drop(recovered);
            }
            history.cleanup();
        }
    }
}

#[test]
fn phase161_read_fault_callers_owner_pause_does_not_mask_observed_failure() {
    use crate::chainstate::filter_index::AcceptedBasicIndexFailure as Failure;
    for backend in [false, true] {
        for pause in [
            Failure::RequiredBody,
            Failure::RequiredUndo,
            Failure::FactLimit,
        ] {
            // Arrange
            let (history, runtime) = genuine();
            runtime
                .network
                .drive_basic_filter_index_turn()
                .expect("publication");
            runtime
                .network
                .note_basic_index_failure(pause)
                .expect("controlled owner pause");
            let before = read_fault_status(&runtime);
            let hash = history.records[2].identity().block_hash();
            if backend {
                runtime
                    .store()
                    .set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
            } else {
                runtime
                    .store()
                    .inject_query_record_fault_for_test(hash, QueryRecordFault::CorruptTarget)
                    .expect("fault");
            }
            // Act
            assert!(runtime.network.basic_filter_query(hash).is_err());
            // Assert
            assert_sticky_read_fault(&runtime, &before, backend);
            drop(runtime);
            history.cleanup();
        }
    }
}

#[test]
fn phase161_read_fault_callers_absence_unknown_and_cancelled_do_not_poison() {
    use crate::status::FieldAvailability;
    // Arrange
    let (history, runtime) = genuine();
    let hash = history.records[2].identity().block_hash();
    let BasicFilterQuery::Pending(barrier) =
        runtime.network.basic_filter_query(hash).expect("pending")
    else {
        panic!("pending");
    };
    drop(barrier);
    assert_eq!(runtime.network.basic_filter_waiter_count_for_test(), 0);
    runtime
        .network
        .set_basic_filter_waiter_id_for_test(u64::MAX);
    assert!(runtime.network.basic_filter_query(hash).is_err());
    assert!(
        runtime
            .store()
            .basic_filter_operator_observation()
            .expect("readiness refusal")
            .maybe_read_fault
            .is_none()
    );
    runtime.network.set_basic_filter_waiter_id_for_test(10);
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("publication");
    let before = read_fault_status(&runtime);
    // Act
    assert!(matches!(
        runtime
            .network
            .basic_filter_query(BlockHash::from_byte_array([77; 32]))
            .expect("unknown"),
        BasicFilterQuery::UnknownBlock
    ));
    runtime
        .store()
        .inject_query_record_fault_for_test(hash, QueryRecordFault::MissingTarget)
        .expect("missing");
    assert!(matches!(
        runtime.network.basic_filter_query(hash).expect("missing"),
        BasicFilterQuery::Missing { .. }
    ));
    // Assert
    assert_eq!(read_fault_status(&runtime), before);
    assert_eq!(before.maybe_failure, FieldAvailability::available(None));
    assert!(
        runtime
            .store()
            .basic_filter_operator_observation()
            .expect("capture")
            .maybe_read_fault
            .is_none()
    );
    drop(runtime);
    history.cleanup();
}

fn genuine() -> (ValidatedHistory, DurableSyncRuntime) {
    let mut history = ValidatedHistory::new("rpc-owner-fault", false);
    let id = NEXT_FAULT
        .fetch_update(
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
            |value| value.checked_add(1),
        )
        .expect("checked fixture ID");
    history.path = std::env::temp_dir().join(format!(
        "phase159-node-rpc-fault-{}-{id}",
        std::process::id()
    ));
    std::fs::create_dir(&history.path).expect("reserve exclusive fixture");
    let config = SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        ..sync_config()
    };
    let runtime = DurableSyncRuntime::open(
        FjallNodeStore::open(&history.path).expect("fresh complete store"),
        config.clone(),
    )
    .expect("empty ordinary runtime");
    for block in &history.blocks[..2] {
        runtime
            .network
            .connect_local_block(block, ScriptVerifyFlags::P2SH, params())
            .expect("genuine managed genesis and child acceptance");
        runtime
            .store()
            .save_block(block, PersistMode::Sync)
            .expect("ordinary body");
    }
    runtime
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(1), u64::MAX)
        .expect("actual coins fence");
    assert!(
        runtime
            .store()
            .recovered_validation_history()
            .expect("coverage")
            .is_complete()
            .expect("genuine complete coverage")
    );
    drop(runtime);
    let runtime = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&history.path).expect("all-handle reopen"),
        config,
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("earned configured epoch");
    runtime
        .network
        .connect_local_block(&history.blocks[2], ScriptVerifyFlags::P2SH, params())
        .expect("genuine accepted pending successor");
    runtime
        .store()
        .save_block(&history.blocks[2], PersistMode::Sync)
        .expect("retained accepted body");
    assert_eq!(
        runtime
            .store()
            .validation_provenance(history.records[2].identity().block_hash())
            .expect("authentic history"),
        ValidationProvenance::ScriptsValid
    );
    (history, runtime)
}

fn params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..SyncNetwork::Regtest.consensus_params()
    }
}

fn poll(
    barrier: &mut crate::BasicFilterReadBarrier,
) -> Poll<Result<crate::BasicFilterReadCompletion, BasicFilterQueryError>> {
    Pin::new(barrier).poll(&mut Context::from_waker(Waker::noop()))
}

fn publication(fault: FilterPublicationFault) {
    // Arrange: genuine accepted pending work, not a raw seeded validity flag.
    let (history, runtime) = genuine();
    let hash = history.records[2].identity().block_hash();
    let BasicFilterQuery::Pending(mut barrier) = runtime
        .network
        .basic_filter_query(hash)
        .expect("actual production query")
    else {
        panic!("captured pending successor")
    };
    assert!(poll(&mut barrier).is_pending());
    runtime.store().set_basic_filter_fault(fault);
    // Act
    assert!(runtime.network.drive_basic_filter_index_turn().is_err());
    // Assert
    assert!(matches!(
        poll(&mut barrier),
        Poll::Ready(Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::OwnerFailed
        )))
    ));
    assert_eq!(
        runtime
            .store()
            .validation_provenance(hash)
            .expect("acceptance survives"),
        ValidationProvenance::ScriptsValid
    );
    assert!(matches!(
        runtime.network.basic_filter_query(hash),
        Err(BasicFilterQueryError::Storage(_)) | Err(BasicFilterQueryError::Readiness(_))
    ));
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_rpc_owner_faults_before_records() {
    publication(FilterPublicationFault::BeforeRecords);
}
#[test]
fn phase159_rpc_owner_faults_before_checkpoint() {
    publication(FilterPublicationFault::BeforeCheckpoint);
}
#[test]
fn phase159_rpc_owner_faults_before_protection() {
    publication(FilterPublicationFault::BeforeProtection);
}
#[test]
fn phase159_rpc_owner_faults_after_commit() {
    publication(FilterPublicationFault::AfterCommit);
}

fn row_fault(fault: QueryRecordFault) {
    // Arrange: earn the epoch and target by the ordinary accepted owner.
    let (history, runtime) = genuine();
    let hash = history.records[2].identity().block_hash();
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("actual publication");
    assert!(matches!(
        runtime
            .network
            .basic_filter_query(hash)
            .expect("genuine earned row"),
        BasicFilterQuery::Found(_)
    ));
    runtime
        .store()
        .inject_query_record_fault_for_test(hash, fault)
        .expect("external backend corruption beneath earned epoch");
    // Act / Assert: route through ManagedNetworkHandle, not only the store reader.
    assert!(matches!(
        runtime.network.basic_filter_query(hash),
        Err(BasicFilterQueryError::Storage(
            StorageError::Corruption { .. }
        ))
    ));
    assert_eq!(
        runtime
            .store()
            .validation_provenance(hash)
            .expect("unaltered history"),
        ValidationProvenance::ScriptsValid
    );
    drop(runtime);
    // Real recovery must refuse inconsistent immutable ancestry; never readmit it.
    let reopened = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&history.path).expect("closed store"),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..sync_config()
        },
        crate::chainstate::BasicFilterStartupMode::Enabled,
    );
    assert!(reopened.is_err());
    history.cleanup();
}

#[test]
fn phase159_rpc_owner_faults_corrupt_target() {
    row_fault(QueryRecordFault::CorruptTarget);
}
#[test]
fn phase159_rpc_owner_faults_corrupt_parent() {
    row_fault(QueryRecordFault::CorruptParent);
}
#[test]
fn phase159_rpc_owner_faults_missing_parent() {
    row_fault(QueryRecordFault::MissingParent);
}

#[test]
fn phase159_rpc_owner_faults_missing_accepted_target_preserves_captured_provenance() {
    // Arrange
    let (history, runtime) = genuine();
    let hash = history.records[2].identity().block_hash();
    let BasicFilterQuery::Pending(mut barrier) =
        runtime.network.basic_filter_query(hash).expect("pending")
    else {
        panic!("pending")
    };
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("real completion");
    let Poll::Ready(Ok(completion)) = poll(&mut barrier) else {
        panic!("earned completion")
    };
    runtime
        .store()
        .inject_query_record_fault_for_test(hash, QueryRecordFault::MissingTarget)
        .expect("external missing target");
    // Act / Assert: the real final RPC completion retains request-time acceptance.
    assert!(matches!(
        runtime
            .network
            .complete_basic_filter_read(hash, completion)
            .expect("typed missing"),
        BasicFilterQuery::Missing {
            provenance: crate::BasicBlockValidationProvenance::ScriptsValid,
            initially_synchronized: true
        }
    ));
    assert_eq!(
        runtime
            .store()
            .validation_provenance(hash)
            .expect("retained acceptance"),
        ValidationProvenance::ScriptsValid
    );
    drop(runtime);
    let reopened = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&history.path).expect("closed reopen"),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..sync_config()
        },
        crate::chainstate::BasicFilterStartupMode::Enabled,
    );
    assert!(
        reopened.is_err(),
        "dangling projection must not be granted a fresh clean epoch"
    );
    history.cleanup();
}

#[test]
fn phase159_rpc_owner_faults_backend_read_is_distinct_from_absence() {
    // Arrange
    let (history, runtime) = genuine();
    let hash = history.records[2].identity().block_hash();
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("earned target");
    runtime
        .store()
        .set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
    // Act
    let result = runtime.network.basic_filter_query(hash);
    // Assert
    assert!(matches!(
        result,
        Err(BasicFilterQueryError::Storage(
            StorageError::BackendFailure { .. }
        ))
    ));
    assert_eq!(
        runtime
            .store()
            .validation_provenance(hash)
            .expect("history not erased"),
        ValidationProvenance::ScriptsValid
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_rpc_owner_faults_raw_clone_invalidates_final_read() {
    // Arrange
    let (history, runtime) = genuine();
    let hash = history.records[2].identity().block_hash();
    let BasicFilterQuery::Pending(mut barrier) =
        runtime.network.basic_filter_query(hash).expect("query")
    else {
        panic!("pending")
    };
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("actual owner");
    let Poll::Ready(Ok(completion)) = poll(&mut barrier) else {
        panic!("actual completion")
    };
    runtime
        .store()
        .clone()
        .remove_validation_filter_for_test(hash)
        .expect("real raw invalidating API");
    // Act / Assert
    assert!(matches!(
        runtime.network.complete_basic_filter_read(hash, completion),
        Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::Invalidated
        ))
    ));
    assert!(matches!(
        runtime.network.basic_filter_query(hash),
        Err(BasicFilterQueryError::Storage(
            StorageError::Corruption { .. }
        ))
    ));
    assert!(
        runtime
            .store()
            .basic_filter_operator_observation()
            .expect("raw invalidation is not read fault")
            .maybe_read_fault
            .is_none()
    );
    drop(runtime);
    history.cleanup();
}
