// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{Amount, Transaction, TransactionInput, Txid};

use super::{non_standard_spend, sample_chainstate_snapshot, script, spend_transaction, submit};
use crate::{
    AdmissionContext, DryRunPackageCommand, FeeRate, HardMemberFailure, Mempool, MempoolEntry,
    MempoolError, PackageMemberResult, PackageReportError, PackageStatus, PolicyConfig,
    ReconsiderableMemberFailure, RollingMempoolFeeRate, StaticRelayFeeRate, SubmissionPackage,
    SubmitPackageCommand, TransactionVirtualSize, WellFormedPackage,
};

fn verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
}

fn consensus_params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    }
}

fn package(transactions: Vec<Transaction>) -> WellFormedPackage {
    WellFormedPackage::try_from(transactions).expect("well-formed package")
}

fn invalid_script_transaction(mut transaction: Transaction) -> Transaction {
    transaction.inputs[0].script_sig = script(&[0x01, 0x52]);
    transaction
}

fn rolling_only_mempool(mut config: PolicyConfig) -> Mempool {
    config.static_relay_fee_rate = StaticRelayFeeRate::new(FeeRate::ZERO);
    let mut mempool = Mempool::new(config);
    mempool
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            1_000,
        )))
        .expect("rolling fixture");
    mempool
}

#[test]
fn exact_existing_member_is_already_present() {
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

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![transaction]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry-run");

    // Assert
    assert!(matches!(
        result.report.members(),
        [PackageMemberResult::AlreadyPresent(_)]
    ));
}

#[test]
fn same_txid_different_witness_reports_existing_wtxid() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let existing = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let existing_wtxid = transaction_wtxid(&existing).expect("existing wtxid");
    let mut requested = existing.clone();
    requested.inputs[0].witness = open_bitcoin_primitives::ScriptWitness::new(vec![vec![1_u8]]);
    assert_eq!(
        transaction_txid(&existing).expect("existing txid"),
        transaction_txid(&requested).expect("requested txid")
    );
    assert_ne!(
        existing_wtxid,
        transaction_wtxid(&requested).expect("requested wtxid")
    );
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, existing).expect("fixture admission");

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![requested]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry-run");

    // Assert
    let [PackageMemberResult::SameTxidDifferentWitness(alias)] = result.report.members() else {
        panic!("expected witness alias");
    };
    assert_eq!(alias.existing_wtxid, existing_wtxid);
}

#[test]
fn hard_failure_does_not_stop_later_valid_singleton() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let invalid = non_standard_spend(coinbase_txids[0]);
    let valid = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let valid_txid = transaction_txid(&valid).expect("valid txid");
    let mempool = Mempool::default();

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![invalid, valid]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry-run");

    // Assert
    assert_eq!(result.report.status(), &PackageStatus::Partial);
    assert!(matches!(
        &result.report.members()[0],
        PackageMemberResult::HardRejected(HardMemberFailure::Policy { .. })
    ));
    assert_eq!(
        result.report.members()[1].requested_identity().txid,
        valid_txid
    );
    assert!(matches!(
        &result.report.members()[1],
        PackageMemberResult::FinallyPresent(_)
    ));
}

#[test]
fn singleton_pre_script_policy_failure_executes_zero_scripts() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let low_fee_invalid_script = invalid_script_transaction(spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    ));
    let mempool = Mempool::default();
    super::super::package_admission::reset_script_check_count_for_test();

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![low_fee_invalid_script]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry-run");

    // Assert
    assert!(matches!(
        result.report.members(),
        [PackageMemberResult::HardRejected(_)]
    ));
    assert_eq!(
        super::super::package_admission::script_check_count_for_test(),
        0
    );
}

#[test]
fn singleton_script_failure_is_not_retained() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let invalid_script = invalid_script_transaction(spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    ));
    let mempool = Mempool::default();

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![invalid_script]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry-run");

    // Assert
    assert!(matches!(
        result.report.members(),
        [PackageMemberResult::HardRejected(_)]
    ));
    assert!(mempool.entries().is_empty());
}

#[test]
fn valid_parent_with_invalid_child_commits_only_parent() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let invalid_child = invalid_script_transaction(spend_transaction(
        parent_txid,
        0,
        499_997_000,
        TransactionInput::SEQUENCE_FINAL,
    ));
    let invalid_child_txid = transaction_txid(&invalid_child).expect("child txid");
    let submission =
        SubmissionPackage::try_from_package(package(vec![parent, invalid_child]), &snapshot)
            .expect("submission refinement");
    let mut mempool = Mempool::default();

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
    assert_eq!(result.report.status(), &PackageStatus::Partial);
    assert!(matches!(
        &result.report.members()[0],
        PackageMemberResult::FinallyPresent(_)
    ));
    assert!(matches!(
        &result.report.members()[1],
        PackageMemberResult::HardRejected(_)
    ));
    assert!(mempool.entries().contains_key(&parent_txid));
    assert!(!mempool.entries().contains_key(&invalid_child_txid));
    assert_eq!(result.delta.admitted.len(), 1);
}

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

#[test]
fn missing_input_stays_reconsiderable_after_residual_attempt() {
    // Arrange
    let (snapshot, _coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        Txid::from_byte_array([94; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = Mempool::default();

    // Act
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![transaction]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("missing input report");

    // Assert
    assert!(matches!(
        result.report.members(),
        [PackageMemberResult::Reconsiderable(
            ReconsiderableMemberFailure::MissingInputs { .. }
        )]
    ));
}

#[test]
fn residual_pre_script_policy_failure_discards_working_overlay() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = non_standard_spend(parent_txid);
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
        .expect("policy failure report");

    // Assert
    assert!(matches!(
        &result.report.members()[1],
        PackageMemberResult::HardRejected(_)
    ));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn residual_limit_failure_discards_working_overlay() {
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
    let mempool = rolling_only_mempool(PolicyConfig {
        max_ancestor_count: 1,
        ..PolicyConfig::default()
    });
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
        .expect("limit failure report");

    // Assert
    assert!(matches!(
        &result.report.members()[1],
        PackageMemberResult::HardRejected(_)
    ));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn residual_script_failure_discards_working_overlay() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = invalid_script_transaction(spend_transaction(
        parent_txid,
        0,
        499_994_000,
        TransactionInput::SEQUENCE_FINAL,
    ));
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
        .expect("script failure report");

    // Assert
    assert!(matches!(
        &result.report.members()[1],
        PackageMemberResult::HardRejected(_)
    ));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn prospective_limit_checks_cover_all_directions_and_kinds() {
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
    let configs = [
        PolicyConfig {
            max_ancestor_count: 0,
            ..PolicyConfig::default()
        },
        PolicyConfig {
            max_ancestor_virtual_size: 0,
            ..PolicyConfig::default()
        },
        PolicyConfig {
            max_descendant_count: 1,
            ..PolicyConfig::default()
        },
        PolicyConfig {
            max_descendant_virtual_size: 1,
            ..PolicyConfig::default()
        },
    ];

    // Act
    let reports = configs.map(|config| {
        Mempool::new(config)
            .dry_run_package(
                DryRunPackageCommand {
                    package: package(vec![parent.clone(), child.clone()]),
                    context: AdmissionContext::legacy_unknown(),
                },
                &snapshot,
                verify_flags(),
                consensus_params(),
            )
            .expect("typed limit report")
            .report
    });

    // Assert
    assert!(reports.iter().all(|report| {
        report
            .members()
            .iter()
            .any(|member| matches!(member, PackageMemberResult::HardRejected(_)))
    }));
}

#[test]
fn prospective_limit_checks_fail_closed_for_missing_members() {
    // Arrange
    let mempool = Mempool::default();
    let prospective = super::super::prospective::ProspectiveMempool::new(&mempool);
    let missing_txid = Txid::from_byte_array([95; 32]);

    // Act
    let missing_candidate = prospective
        .validate_candidate_limits(missing_txid)
        .expect_err("missing candidate");

    // Assert
    assert!(matches!(
        missing_candidate,
        MempoolError::InternalInvariant { .. }
    ));
}

#[test]
fn prospective_limit_checks_fail_closed_for_missing_ancestor() {
    // Arrange
    let transaction = spend_transaction(
        Txid::from_byte_array([96; 32]),
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let mut entry = MempoolEntry::new(
        transaction,
        txid,
        wtxid,
        Amount::from_sats(1_000).expect("fee"),
        TransactionVirtualSize::new(100),
        400,
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    );
    entry.parents.insert(Txid::from_byte_array([97; 32]));
    let mut mempool = Mempool::default();
    mempool.entries.insert(txid, entry);
    let prospective = super::super::prospective::ProspectiveMempool::new(&mempool);

    // Act
    let error = prospective
        .validate_candidate_limits(txid)
        .expect_err("missing ancestor");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}

#[test]
fn package_invariant_mappers_remain_typed() {
    // Arrange
    let report_error = PackageReportError::MemberCountMismatch {
        expected: 1,
        actual: 0,
    };

    // Act
    let empty_group = super::super::package_admission::empty_fee_group_error_for_test();
    let group_invariant = super::super::package_admission::group_invariant_for_test();
    let report_invariant =
        super::super::package_admission::report_invariant_error_for_test(report_error);

    // Assert
    assert!(matches!(
        empty_group,
        MempoolError::InternalInvariant { .. }
    ));
    assert!(matches!(
        group_invariant,
        MempoolError::InternalInvariant { .. }
    ));
    assert!(matches!(
        report_invariant,
        MempoolError::InternalInvariant { .. }
    ));
}
