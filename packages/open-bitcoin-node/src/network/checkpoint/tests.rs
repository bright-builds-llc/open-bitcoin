// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use std::sync::{
    Arc, Barrier,
    atomic::{AtomicUsize, Ordering},
};

use open_bitcoin_mempool::{PolicyConfig, PolicyTime};
use open_bitcoin_network::LocalPeerConfig;

use super::*;
use crate::storage::{
    StorageError, StorageNamespace, StorageRecoveryAction, fjall_store::SnapshotWriteExecutionError,
};
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn empty_handle() -> ManagedNetworkHandle {
    ManagedNetworkHandle::from_network_fixture(ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    ))
}

fn fake_success<Now>(
    prepared: PreparedSnapshotWrite,
    now: &mut Now,
) -> Result<SnapshotWriteReceipt, SnapshotWriteExecutionError>
where
    Now: FnMut() -> PolicyTime,
{
    let (_, capability) = prepared.into_parts();
    Ok(capability.acknowledge_write(now(), super::super::CheckpointPersistenceStrength::Sync))
}

fn injected_storage_error() -> StorageError {
    StorageError::BackendFailure {
        namespace: StorageNamespace::Mempool,
        message: "injected checkpoint write failure".to_string(),
        action: StorageRecoveryAction::Restart,
    }
}

#[test]
fn periodic_clean_checkpoint_is_skipped() {
    // Arrange
    let coordinator = MempoolCheckpointCoordinator::new();
    let handle = empty_handle();
    let mut now = || PolicyTime::new(10);

    // Act
    let outcome = coordinator
        .run_with(
            &handle,
            CheckpointTrigger::Periodic,
            &mut now,
            &mut |_, _| panic!("clean checkpoint must not execute"),
        )
        .expect("clean tick");

    // Assert
    assert_eq!(outcome, MempoolCheckpointOutcome::SkippedClean);
}

#[test]
fn concurrent_tick_coalesces_and_stale_completion_preserves_exact_loss_range() {
    // Arrange
    let coordinator = Arc::new(MempoolCheckpointCoordinator::new());
    let handle = empty_handle();
    assert_eq!(handle.mark_checkpoint_dirty_for_test().expect("dirty"), 1);
    let entered = Arc::new(Barrier::new(2));
    let release = Arc::new(Barrier::new(2));
    let writes = Arc::new(AtomicUsize::new(0));

    // Act
    let worker_coordinator = Arc::clone(&coordinator);
    let worker_handle = handle.clone();
    let worker_entered = Arc::clone(&entered);
    let worker_release = Arc::clone(&release);
    let worker_writes = Arc::clone(&writes);
    let worker = std::thread::spawn(move || {
        let mut now = || PolicyTime::new(20);
        worker_coordinator.run_with(
            &worker_handle,
            CheckpointTrigger::Periodic,
            &mut now,
            &mut |prepared, now| {
                let write = worker_writes.fetch_add(1, Ordering::SeqCst);
                if write == 0 {
                    worker_entered.wait();
                    worker_release.wait();
                    return fake_success(prepared, now);
                }
                FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
                    &worker_handle,
                    prepared,
                    |_| Ok(Vec::new()),
                    |_, _| Err(injected_storage_error()),
                    now,
                )
            },
        )
    });
    entered.wait();
    assert_eq!(
        handle
            .mark_checkpoint_dirty_for_test()
            .expect("newer dirty"),
        2
    );
    let mut concurrent_now = || PolicyTime::new(20);
    let coalesced = coordinator
        .run_with(
            &handle,
            CheckpointTrigger::Periodic,
            &mut concurrent_now,
            &mut fake_success,
        )
        .expect("coalesced tick");
    release.wait();
    let worker_error = worker
        .join()
        .expect("checkpoint thread")
        .expect_err("follow-up storage failure");
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(20), INTERNAL_EVIDENCE_INTERVAL_SECONDS)
        .expect("checkpoint evidence");

    // Assert
    assert_eq!(coalesced, MempoolCheckpointOutcome::Coalesced);
    assert!(matches!(worker_error, MempoolCheckpointError::Execution(_)));
    assert_eq!(writes.load(Ordering::SeqCst), 2);
    assert_eq!(evidence.current_generation, 2);
    assert_eq!(evidence.maybe_last_durable_generation, Some(1));
    assert_eq!(evidence.maybe_dirty_generation, Some(2));
    assert_eq!(
        evidence.maybe_generation_loss_range,
        Some(super::super::CheckpointGenerationLossRange {
            maybe_after_generation: Some(1),
            through_generation: 2,
        })
    );
}

#[test]
fn completion_dispatch_failure_retains_and_retries_the_same_receipt() {
    // Arrange
    let coordinator = MempoolCheckpointCoordinator::new();
    let handle = empty_handle();
    handle.mark_checkpoint_dirty_for_test().expect("dirty");
    handle.fail_next_checkpoint_completion_dispatch_for_test();
    let mut now = || PolicyTime::new(30);

    // Act
    let first = coordinator.run_with(
        &handle,
        CheckpointTrigger::Periodic,
        &mut now,
        &mut fake_success,
    );
    let retained = matches!(
        &*coordinator.state.lock().expect("coordinator state"),
        CheckpointCoordinatorState::AchievedAwaitingCompletion(_)
    );
    let retry = coordinator
        .run_with(
            &handle,
            CheckpointTrigger::Periodic,
            &mut now,
            &mut |_, _| panic!("retry must complete before capturing"),
        )
        .expect("receipt retry");

    // Assert
    assert!(matches!(
        first,
        Err(MempoolCheckpointError::CompletionDispatch(_))
    ));
    assert!(retained);
    assert_eq!(
        retry,
        MempoolCheckpointOutcome::Completed {
            writes_started: 0,
            maybe_last_completion: Some(EffectCompletion::Applied),
        }
    );
}

#[test]
fn already_completed_retained_receipt_is_idempotent() {
    // Arrange
    let coordinator = MempoolCheckpointCoordinator::new();
    let handle = empty_handle();
    handle.mark_checkpoint_dirty_for_test().expect("dirty");
    let prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(40), CheckpointTrigger::Periodic)
        .expect("prepare");
    let (_, capability) = prepared.into_parts();
    let receipt = capability.acknowledge_write(
        PolicyTime::new(40),
        super::super::CheckpointPersistenceStrength::Sync,
    );
    let duplicate = receipt.duplicate_for_test();
    assert_eq!(
        handle
            .complete_snapshot_write(duplicate)
            .expect("first completion"),
        EffectCompletion::Applied
    );
    *coordinator.state.lock().expect("coordinator state") =
        CheckpointCoordinatorState::AchievedAwaitingCompletion(receipt);
    let mut now = || PolicyTime::new(40);

    // Act
    let outcome = coordinator
        .run_with(
            &handle,
            CheckpointTrigger::Periodic,
            &mut now,
            &mut |_, _| panic!("duplicate completion must not recapture"),
        )
        .expect("duplicate completion");

    // Assert
    assert_eq!(
        outcome,
        MempoolCheckpointOutcome::Completed {
            writes_started: 0,
            maybe_last_completion: Some(EffectCompletion::AlreadyApplied),
        }
    );
}

#[test]
fn encode_failure_releases_the_flight_and_remains_retryable() {
    // Arrange
    let coordinator = MempoolCheckpointCoordinator::new();
    let handle = empty_handle();
    handle.mark_checkpoint_dirty_for_test().expect("dirty");
    let mut now = || PolicyTime::new(50);

    // Act
    let first = coordinator.run_with(
        &handle,
        CheckpointTrigger::Periodic,
        &mut now,
        &mut |prepared, now| {
            FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
                &handle,
                prepared,
                |_| Err(injected_storage_error()),
                |_, _| panic!("encode failure must not save"),
                now,
            )
        },
    );
    let retry = coordinator
        .run_with(
            &handle,
            CheckpointTrigger::Periodic,
            &mut now,
            &mut fake_success,
        )
        .expect("retry succeeds");

    // Assert
    assert!(matches!(first, Err(MempoolCheckpointError::Execution(_))));
    assert_eq!(
        retry,
        MempoolCheckpointOutcome::Completed {
            writes_started: 1,
            maybe_last_completion: Some(EffectCompletion::Applied),
        }
    );
}

#[test]
fn shutdown_settle_forces_exact_current_sync_durability() {
    // Arrange
    let coordinator = MempoolCheckpointCoordinator::new();
    let handle = empty_handle();
    handle.mark_checkpoint_dirty_for_test().expect("dirty");
    let path = std::env::temp_dir().join(format!(
        "open-bitcoin-checkpoint-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");

    // Act
    let outcome = coordinator
        .settle_shutdown(&handle, &store, || PolicyTime::new(60))
        .expect("shutdown settle");
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(60), INTERNAL_EVIDENCE_INTERVAL_SECONDS)
        .expect("checkpoint evidence");
    drop(store);
    std::fs::remove_dir_all(&path).expect("remove temp store");

    // Assert
    assert_eq!(
        outcome,
        MempoolCheckpointOutcome::Completed {
            writes_started: 1,
            maybe_last_completion: Some(EffectCompletion::Applied),
        }
    );
    assert_eq!(
        evidence.maybe_last_durable_generation,
        Some(evidence.current_generation)
    );
}

#[test]
fn shutdown_settle_refuses_success_while_another_flight_is_active() {
    // Arrange
    let coordinator = MempoolCheckpointCoordinator::new();
    let handle = empty_handle();
    handle.mark_checkpoint_dirty_for_test().expect("dirty");
    *coordinator.state.lock().expect("coordinator state") = CheckpointCoordinatorState::Persisting;
    let path = std::env::temp_dir().join(format!(
        "open-bitcoin-checkpoint-busy-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&path);
    let store = FjallNodeStore::open(&path).expect("open temp store");

    // Act
    let result = coordinator.settle_shutdown(&handle, &store, || PolicyTime::new(70));
    drop(store);
    std::fs::remove_dir_all(&path).expect("remove temp store");

    // Assert
    assert!(matches!(
        result,
        Err(MempoolCheckpointError::ShutdownNotCurrent {
            current_generation: 1,
            maybe_last_durable_generation: None,
        })
    ));
}
