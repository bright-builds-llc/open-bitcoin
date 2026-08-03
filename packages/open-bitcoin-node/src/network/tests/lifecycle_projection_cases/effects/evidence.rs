// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

const PERIODIC_INTERVAL_SECONDS: u64 = 300;

fn prepare_typed_receipt(
    handle: &ManagedNetworkHandle,
    captured_at: i64,
    completed_at: i64,
) -> SnapshotWriteReceipt {
    handle
        .prepare_mempool_snapshot_write(PolicyTime::new(captured_at), CheckpointTrigger::Periodic)
        .expect("checkpoint should prepare")
        .into_parts()
        .1
        .acknowledge_write(
            PolicyTime::new(completed_at),
            CheckpointPersistenceStrength::Sync,
        )
}

#[test]
fn current_checkpoint_success_records_exact_durable_evidence() {
    // Arrange
    let handle = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let receipt = prepare_typed_receipt(&handle, 200_000, 200_004);

    // Act
    let completion = handle
        .complete_snapshot_write(receipt)
        .expect("typed completion should dispatch");
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(200_010), PERIODIC_INTERVAL_SECONDS)
        .expect("checkpoint evidence");

    // Assert
    assert_eq!(completion, EffectCompletion::Applied);
    assert_eq!(evidence.current_generation, 0);
    assert_eq!(evidence.maybe_dirty_generation, None);
    assert_eq!(evidence.maybe_in_flight_generation, None);
    assert_eq!(evidence.maybe_last_durable_generation, Some(0));
    assert_eq!(evidence.maybe_captured_at, Some(PolicyTime::new(200_000)));
    assert_eq!(evidence.maybe_completed_at, Some(PolicyTime::new(200_004)));
    assert_eq!(evidence.maybe_trigger, Some(CheckpointTrigger::Periodic));
    assert_eq!(
        evidence.maybe_persistence_strength,
        Some(CheckpointPersistenceStrength::Sync)
    );
    assert_eq!(evidence.outcome, CheckpointOutcome::Succeeded);
    assert_eq!(evidence.maybe_failure, None);
    assert_eq!(evidence.checkpoint_age_seconds, Some(10));
    assert_eq!(
        evidence.maybe_generation_loss_range, None,
        "equal current and durable generations have an empty loss interval"
    );
    assert_eq!(evidence.maybe_loss_bound_seconds, Some(300));
    assert!(!evidence.overdue);
}

#[test]
fn stale_success_advances_durable_high_water_without_clearing_newer_dirty() {
    // Arrange
    let (network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    let receipt = prepare_typed_receipt(&handle, 200_000, 200_004);
    handle
        .submit_local_transaction_outcome_at(
            spend_transaction(coinbase_txid, 499_999_000),
            verify_flags(),
            consensus_params(),
            200_002,
            RelayIntent::Requested,
        )
        .expect("newer mutation should apply");

    // Act
    let completion = handle
        .complete_snapshot_write(receipt)
        .expect("stale success should dispatch");
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(200_010), PERIODIC_INTERVAL_SECONDS)
        .expect("checkpoint evidence");

    // Assert
    assert_eq!(completion, EffectCompletion::AchievedButStale);
    assert_eq!(evidence.current_generation, 1);
    assert_eq!(evidence.maybe_dirty_generation, Some(1));
    assert_eq!(evidence.maybe_last_durable_generation, Some(0));
    assert_eq!(
        evidence.maybe_generation_loss_range,
        Some(CheckpointGenerationLossRange {
            maybe_after_generation: Some(0),
            through_generation: 1,
        })
    );
}

#[test]
fn duplicate_success_is_idempotent_and_cannot_regress_evidence() {
    // Arrange
    let handle = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let receipt = prepare_typed_receipt(&handle, 200_000, 200_004);
    let duplicate = receipt.duplicate_for_test();
    handle
        .complete_snapshot_write(receipt)
        .expect("first completion should dispatch");
    let before = handle
        .checkpoint_evidence(PolicyTime::new(200_010), PERIODIC_INTERVAL_SECONDS)
        .expect("checkpoint evidence before duplicate");

    // Act
    let completion = handle
        .complete_snapshot_write(duplicate)
        .expect("duplicate should dispatch");
    let after = handle
        .checkpoint_evidence(PolicyTime::new(200_010), PERIODIC_INTERVAL_SECONDS)
        .expect("checkpoint evidence after duplicate");

    // Assert
    assert_eq!(completion, EffectCompletion::AlreadyApplied);
    assert_eq!(after, before);
}

#[test]
fn exact_preachievement_abort_records_failure_and_keeps_generation_loss_unbounded() {
    // Arrange
    let (network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    handle
        .submit_local_transaction_outcome_at(
            spend_transaction(coinbase_txid, 499_999_000),
            verify_flags(),
            consensus_params(),
            200_000,
            RelayIntent::Requested,
        )
        .expect("dirty mutation should apply");
    let capability = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_010), CheckpointTrigger::Periodic)
        .expect("checkpoint should prepare")
        .into_parts()
        .1;
    let abort = SnapshotWriteAbort::new(
        capability,
        PolicyTime::new(200_012),
        SnapshotWriteFailure::Encode,
    )
    .expect("encode failure is pre-achievement");

    // Act
    let result = handle
        .abort_snapshot_write(abort)
        .expect("typed abort should dispatch");
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(200_400), PERIODIC_INTERVAL_SECONDS)
        .expect("checkpoint evidence");

    // Assert
    assert_eq!(result, EffectAbort::Aborted);
    assert_eq!(evidence.maybe_dirty_generation, Some(1));
    assert_eq!(evidence.maybe_in_flight_generation, None);
    assert_eq!(evidence.outcome, CheckpointOutcome::Failed);
    assert_eq!(evidence.maybe_failure, Some(SnapshotWriteFailure::Encode));
    assert_eq!(evidence.maybe_failed_at, Some(PolicyTime::new(200_012)));
    assert_eq!(evidence.maybe_loss_bound_seconds, None);
    assert_eq!(evidence.checkpoint_age_seconds, Some(390));
    assert!(evidence.overdue);
}

#[test]
fn pending_and_absent_durable_evidence_report_exact_unbounded_loss_truth() {
    // Arrange
    let handle = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let _prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_100), CheckpointTrigger::Periodic)
        .expect("checkpoint should prepare");

    // Act
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(200_050), PERIODIC_INTERVAL_SECONDS)
        .expect("checkpoint evidence");

    // Assert
    assert_eq!(evidence.outcome, CheckpointOutcome::Pending);
    assert_eq!(evidence.maybe_last_durable_generation, None);
    assert_eq!(evidence.maybe_in_flight_generation, Some(0));
    assert_eq!(
        evidence.maybe_generation_loss_range,
        Some(CheckpointGenerationLossRange {
            maybe_after_generation: None,
            through_generation: 0,
        })
    );
    assert_eq!(evidence.checkpoint_age_seconds, Some(0));
    assert_eq!(evidence.maybe_loss_bound_seconds, None);
    assert!(!evidence.overdue);
}

#[test]
fn foreign_receipt_is_rejected_without_changing_local_checkpoint_evidence() {
    // Arrange
    let first = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let second = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let foreign = prepare_typed_receipt(&first, 200_000, 200_004);
    let before = second
        .checkpoint_evidence(PolicyTime::new(200_010), PERIODIC_INTERVAL_SECONDS)
        .expect("checkpoint evidence before foreign receipt");

    // Act
    let error = second
        .complete_snapshot_write(foreign)
        .expect_err("foreign receipt must fail");
    let after = second
        .checkpoint_evidence(PolicyTime::new(200_010), PERIODIC_INTERVAL_SECONDS)
        .expect("checkpoint evidence after foreign receipt");

    // Assert
    assert_eq!(error.failure(), SnapshotWriteFailure::AbortDispatch);
    assert_eq!(after, before);
}

#[test]
fn completion_dispatch_failure_returns_the_original_achieved_receipt() {
    // Arrange
    let handle = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let receipt = prepare_typed_receipt(&handle, 200_000, 200_004);
    handle.fail_next_checkpoint_completion_dispatch_for_test();

    // Act
    let error = handle
        .complete_snapshot_write(receipt)
        .expect_err("injected dispatch failure should retain the receipt");
    let receipt = error.into_receipt();

    // Assert
    assert_eq!(receipt.captured_generation(), 0);
    assert_eq!(receipt.captured_at(), PolicyTime::new(200_000));
    assert_eq!(receipt.completed_at(), Some(PolicyTime::new(200_004)));
    assert_eq!(receipt.checkpoint_trigger(), CheckpointTrigger::Periodic);
    assert_eq!(
        receipt.persistence_strength(),
        Some(CheckpointPersistenceStrength::Sync)
    );
    assert_eq!(
        handle
            .complete_snapshot_write(receipt)
            .expect("retained receipt should retry idempotently"),
        EffectCompletion::Applied
    );
}

#[test]
fn abort_dispatch_is_not_a_valid_preachievement_abort_reason() {
    // Arrange
    let handle = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let capability = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_000), CheckpointTrigger::Periodic)
        .expect("checkpoint should prepare")
        .into_parts()
        .1;

    // Act
    let result = SnapshotWriteAbort::new(
        capability,
        PolicyTime::new(200_001),
        SnapshotWriteFailure::AbortDispatch,
    );

    // Assert
    assert_eq!(
        result,
        Err(SnapshotWriteAbortError::AchievedStateOnlyFailure)
    );
}
