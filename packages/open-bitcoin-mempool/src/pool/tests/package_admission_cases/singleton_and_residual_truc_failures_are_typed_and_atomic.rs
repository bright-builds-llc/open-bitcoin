// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn singleton_and_residual_truc_failures_are_typed_and_atomic() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut rejected = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    rejected.version = 3;
    let reject_mempool = Mempool::new(PolicyConfig {
        truc_policy: TrucPolicy::Reject,
        ..PolicyConfig::default()
    });

    let mut parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    parent.version = 3;
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut enforce_mempool = Mempool::new(PolicyConfig {
        truc_policy: TrucPolicy::Enforce,
        ..PolicyConfig::default()
    });
    enforce_mempool
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            1_000_000,
        )))
        .expect("rolling floor");

    // Act
    let singleton = reject_mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![rejected]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("typed singleton rejection");
    let residual = enforce_mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![parent, child]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("typed residual rejection");

    // Assert
    assert!(singleton.report.members().iter().all(|member| matches!(
        member,
        PackageMemberResult::HardRejected(HardMemberFailure::TrucPolicy { .. })
    )));
    assert!(residual.report.members().iter().all(|member| matches!(
        member,
        PackageMemberResult::HardRejected(HardMemberFailure::TrucPolicy { .. })
    )));
    assert!(reject_mempool.entries().is_empty());
    assert!(enforce_mempool.entries().is_empty());
}

#[test]
fn staged_fee_guard_branches_fail_closed() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let singleton = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let singleton_package = package(vec![singleton]);

    // Act / Assert
    super::super::package_admission::force_staged_fee_branches_for_test(true, false, false, false);
    assert!(
        Mempool::default()
            .dry_run_package(
                DryRunPackageCommand {
                    package: singleton_package.clone(),
                    context: AdmissionContext::legacy_unknown(),
                },
                &snapshot,
                verify_flags(),
                consensus_params(),
            )
            .is_err()
    );
    let below_static = spend_transaction(
        coinbase_txids[0],
        0,
        500_000_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let below_static = Mempool::default()
        .dry_run_package(
            DryRunPackageCommand {
                package: package(vec![below_static]),
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("static rejection remains typed");
    assert!(matches!(
        below_static.report.members(),
        [PackageMemberResult::HardRejected(
            HardMemberFailure::Policy { .. }
        )]
    ));
    super::super::package_admission::force_staged_fee_branches_for_test(false, false, false, false);
    super::super::package_admission::force_staged_fee_errors_for_test(true, false, false);
    assert!(
        Mempool::default()
            .dry_run_package(
                DryRunPackageCommand {
                    package: singleton_package.clone(),
                    context: AdmissionContext::legacy_unknown(),
                },
                &snapshot,
                verify_flags(),
                consensus_params(),
            )
            .is_err()
    );
    super::super::package_admission::force_staged_fee_errors_for_test(false, true, false);
    assert!(
        rolling_only_mempool(PolicyConfig::default())
            .dry_run_package(
                DryRunPackageCommand {
                    package: singleton_package.clone(),
                    context: AdmissionContext::legacy_unknown(),
                },
                &snapshot,
                verify_flags(),
                consensus_params(),
            )
            .is_err()
    );
    super::super::package_admission::force_staged_fee_errors_for_test(false, false, false);
    super::super::package_admission::force_staged_fee_branches_for_test(false, false, true, false);
    let singleton_hard = rolling_only_mempool(PolicyConfig::default())
        .dry_run_package(
            DryRunPackageCommand {
                package: singleton_package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("forced rolling hard rejection");
    assert!(matches!(
        singleton_hard.report.members(),
        [PackageMemberResult::HardRejected(
            HardMemberFailure::Policy { .. }
        )]
    ));

    let parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let residual_package = package(vec![parent, child]);
    let mut mempool = rolling_only_mempool(PolicyConfig::default());
    mempool
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            1_000_000,
        )))
        .expect("high rolling floor");

    super::super::package_admission::force_staged_fee_branches_for_test(false, true, false, false);
    assert!(
        mempool
            .dry_run_package(
                DryRunPackageCommand {
                    package: residual_package.clone(),
                    context: AdmissionContext::legacy_unknown(),
                },
                &snapshot,
                verify_flags(),
                consensus_params(),
            )
            .is_err()
    );
    super::super::package_admission::force_staged_fee_branches_for_test(false, false, false, false);
    super::super::package_admission::force_staged_fee_errors_for_test(false, false, true);
    assert!(
        mempool
            .dry_run_package(
                DryRunPackageCommand {
                    package: residual_package.clone(),
                    context: AdmissionContext::legacy_unknown(),
                },
                &snapshot,
                verify_flags(),
                consensus_params(),
            )
            .is_err()
    );
    super::super::package_admission::force_staged_fee_errors_for_test(false, false, false);
    super::super::package_admission::force_staged_fee_branches_for_test(false, false, false, true);
    let residual_hard = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package: residual_package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("forced residual rolling hard rejection");
    super::super::package_admission::force_staged_fee_branches_for_test(false, false, false, false);

    assert!(residual_hard.report.members().iter().all(|member| matches!(
        member,
        PackageMemberResult::HardRejected(HardMemberFailure::Policy { .. })
    )));
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
