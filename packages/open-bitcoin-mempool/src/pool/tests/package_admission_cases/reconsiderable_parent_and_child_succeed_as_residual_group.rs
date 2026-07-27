// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn reconsiderable_parent_and_child_succeed_as_residual_group() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_994_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let child_txid = transaction_txid(&child).expect("child txid");
    let submission = SubmissionPackage::try_from_package(package(vec![parent, child]), &snapshot)
        .expect("submission refinement");
    let mut mempool = rolling_only_mempool(PolicyConfig::default());

    // Act
    let result = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry-run");

    // Assert
    assert_eq!(result.report.status(), &PackageStatus::Complete);
    assert!(
        result
            .report
            .members()
            .iter()
            .all(|member| matches!(member, PackageMemberResult::FinallyPresent(_)))
    );
    assert_eq!(result.report.effective_fee_groups().len(), 1);
    assert!(mempool.entries().contains_key(&parent_txid));
    assert!(mempool.entries().contains_key(&child_txid));
    assert_eq!(result.delta.admitted.len(), 2);
}

#[test]
fn residual_failure_preserves_live_state() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_950,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_999_900,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = rolling_only_mempool(PolicyConfig::default());
    let before = mempool.complete_snapshot();

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![parent, child]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry-run");

    // Assert
    assert_eq!(result.report.status(), &PackageStatus::Failed);
    assert!(result.report.members().iter().all(|member| matches!(
        member,
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::PackageFee { .. })
    )));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn dry_run_leaves_complete_state_unchanged() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = Mempool::default();
    let before = mempool.complete_snapshot();

    // Act
    let result = mempool.dry_run_package(
        DryRunPackageCommand {
            package: package(vec![transaction]),
            context: AdmissionContext::legacy_unknown(),
        },
        &snapshot,
        verify_flags(),
        consensus_params(),
    );

    // Assert
    assert!(result.is_ok());
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn dry_run_and_submit_return_identical_reports() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let dry_package = package(vec![transaction.clone()]);
    let submit_package = SubmissionPackage::try_from_package(package(vec![transaction]), &snapshot)
        .expect("submission refinement");
    let mut mempool = Mempool::default();

    // Act
    let dry_result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: dry_package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry-run");
    let submit_result = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submit_package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("submit");

    // Assert
    assert_eq!(dry_result.report, submit_result.report);
    assert_eq!(submit_result.delta.admitted.len(), 1);
}

#[test]
fn stale_submit_patch_fails_without_partial_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = Mempool::default();
    let prepared = super::super::package_admission::evaluate_package_for_test(
        &mempool,
        &package(vec![transaction]),
        AdmissionContext::legacy_unknown(),
        &snapshot,
        verify_flags(),
        consensus_params(),
    )
    .expect("package preparation");
    let mut mempool = mempool;
    mempool
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(
            crate::FeeRate::from_sats_per_kvb(5_000),
        ))
        .expect("intervening rolling mutation");
    let before_apply = mempool.complete_snapshot();

    // Act
    let error = mempool
        .apply_prepared(prepared.patch.expect("membership patch"))
        .expect_err("stale package transition");

    // Assert
    assert!(matches!(
        error,
        MempoolError::StalePreparedTransition { .. }
    ));
    assert_eq!(mempool.complete_snapshot(), before_apply);
}

#[test]
fn submit_existing_package_returns_empty_delta_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, transaction.clone()).expect("fixture admission");
    let submission = SubmissionPackage::try_from_package(package(vec![transaction]), &snapshot)
        .expect("submission refinement");
    let before = mempool.complete_snapshot();

    // Act
    let result = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("existing submit");

    // Assert
    assert!(result.delta.is_empty());
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn singleton_overlay_composition_failure_is_returned_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let conflicting_spender = Txid::from_byte_array([91; 32]);
    let mut mempool = Mempool::default();
    mempool.spent_outpoints.insert(
        transaction.inputs[0].previous_output.clone(),
        conflicting_spender,
    );
    let before = mempool.complete_snapshot();

    // Act
    let error = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![transaction]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("conflicting overlay fact");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn submit_overlay_composition_failure_is_returned_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let submission =
        SubmissionPackage::try_from_package(package(vec![transaction.clone()]), &snapshot)
            .expect("submission refinement");
    let mut mempool = Mempool::default();
    mempool.spent_outpoints.insert(
        transaction.inputs[0].previous_output.clone(),
        Txid::from_byte_array([92; 32]),
    );
    let before = mempool.complete_snapshot();

    // Act
    let error = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("conflicting overlay fact");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn later_submit_failure_does_not_commit_an_earlier_accepted_parent() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let submission =
        SubmissionPackage::try_from_package(package(vec![parent, child.clone()]), &snapshot)
            .expect("submission refinement");
    let mut mempool = Mempool::default();
    mempool.spent_outpoints.insert(
        child.inputs[0].previous_output.clone(),
        Txid::from_byte_array([98; 32]),
    );
    let before = mempool.complete_snapshot();

    // Act
    let error = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("later overlay conflict");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert_eq!(mempool.complete_snapshot(), before);
    assert!(!mempool.entries().contains_key(&parent_txid));
}

#[test]
fn residual_overlay_composition_failure_is_returned_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = rolling_only_mempool(PolicyConfig::default());
    mempool.spent_outpoints.insert(
        transaction.inputs[0].previous_output.clone(),
        Txid::from_byte_array([93; 32]),
    );
    let before = mempool.complete_snapshot();

    // Act
    let error = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![transaction]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("residual overlay conflict");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn residual_fee_group_failure_is_returned_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_994_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = rolling_only_mempool(PolicyConfig::default());
    let before = mempool.complete_snapshot();
    super::super::package_admission::force_residual_fee_group_error_for_test(true);

    // Act
    let result = mempool.dry_run_package(
        DryRunPackageCommand {
            package: package(vec![parent, child]),
            context: AdmissionContext::legacy_unknown(),
        },
        &snapshot,
        verify_flags(),
        consensus_params(),
    );
    super::super::package_admission::force_residual_fee_group_error_for_test(false);

    // Assert
    assert!(matches!(
        result,
        Err(MempoolError::InternalInvariant { .. })
    ));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn residual_hard_fee_failure_is_reported_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_994_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = rolling_only_mempool(PolicyConfig::default());
    let before = mempool.complete_snapshot();
    super::super::package_admission::force_residual_fee_group_hard_for_test(true);

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![parent, child]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("typed hard failure");
    super::super::package_admission::force_residual_fee_group_hard_for_test(false);

    // Assert
    assert!(result.report.members().iter().all(|member| matches!(
        member,
        PackageMemberResult::HardRejected(HardMemberFailure::Policy { .. })
    )));
    assert_eq!(mempool.complete_snapshot(), before);
}
