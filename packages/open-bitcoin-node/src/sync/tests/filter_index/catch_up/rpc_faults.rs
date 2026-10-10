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
    drop(runtime);
    history.cleanup();
}
