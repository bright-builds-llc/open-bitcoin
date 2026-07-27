// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/policy.cpp
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/test/txvalidation_tests.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py
// - packages/bitcoin-knots/test/functional/mempool_truc.py

use super::*;

#[test]
fn newly_present_becomes_post_trim_absent_and_dry_run_submit_reports_are_equal() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let package =
        WellFormedPackage::try_from(vec![transaction.clone()]).expect("well-formed package");
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![transaction]).expect("well-formed submission"),
        &snapshot,
    )
    .expect("submission refinement");
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    };
    let mut mempool = Mempool::new(config);
    let verify_flags = ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY;
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    };

    // Act
    super::super::package_admission::reset_package_trim_count_for_test();
    let dry_run = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags,
            consensus_params,
        )
        .expect("dry-run");
    assert_eq!(
        super::super::package_admission::package_trim_count_for_test(),
        1
    );
    super::super::package_admission::reset_package_trim_count_for_test();
    let submitted = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags,
            consensus_params,
        )
        .expect("submit");

    // Assert
    assert_eq!(
        super::super::package_admission::package_trim_count_for_test(),
        1
    );
    assert_eq!(dry_run.report, submitted.report);
    assert!(matches!(
        submitted.report.members(),
        [PackageMemberResult::PostTrimAbsent(
            crate::PostTrimAbsence {
                prior: PriorMemberSuccess::FinallyPresent { .. },
                ..
            }
        )]
    ));
    assert!(submitted.delta.admitted.is_empty());
    assert!(submitted.delta.removed.is_empty());
    assert!(mempool.entries().is_empty());
    assert!(mempool.rolling_mempool_fee_rate() > RollingMempoolFeeRate::ZERO);
}

#[test]
fn exact_existing_and_alias_targets_use_actual_identity_after_trim() {
    for request_alias in [false, true] {
        // Arrange
        let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
        let existing = spend_transaction(
            coinbase_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let existing_txid = transaction_txid(&existing).expect("existing txid");
        let existing_wtxid = transaction_wtxid(&existing).expect("existing wtxid");
        let child = spend_transaction(
            existing_txid,
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        );
        let mut requested = existing.clone();
        if request_alias {
            requested.inputs[0].witness = ScriptWitness::new(vec![vec![1_u8]]);
        }
        let requested_wtxid = transaction_wtxid(&requested).expect("requested wtxid");
        let submission = SubmissionPackage::try_from_package(
            WellFormedPackage::try_from(vec![requested, child]).expect("well-formed package"),
            &snapshot,
        )
        .expect("submission refinement");
        let mut mempool = Mempool::default();
        submit(&mut mempool, &snapshot, existing).expect("existing admission");
        let capacity =
            MempoolCapacity::new(mempool.accounted_memory().as_usize().saturating_add(1));
        super::super::package_admission::set_mempool_capacity_for_test(&mut mempool, capacity);

        // Act
        let submitted = mempool
            .submit_package(
                SubmitPackageCommand {
                    package: submission,
                    context: AdmissionContext::legacy_unknown(),
                },
                &snapshot,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("submit and trim");

        // Assert
        let [
            PackageMemberResult::PostTrimAbsent(first),
            PackageMemberResult::PostTrimAbsent(second),
        ] = submitted.report.members()
        else {
            panic!("exact/alias target and new child should both be absent");
        };
        assert!(matches!(
            first.prior,
            PriorMemberSuccess::AlreadyPresent
                | PriorMemberSuccess::SameTxidDifferentWitness { .. }
        ));
        assert!(matches!(
            second.prior,
            PriorMemberSuccess::FinallyPresent { .. }
        ));
        assert_eq!(submitted.delta.removed.len(), 1);
        assert_eq!(submitted.delta.removed[0].member.wtxid, existing_wtxid);
        assert_eq!(submitted.delta.retry_clears.len(), 1);
        assert_eq!(
            submitted.delta.retry_clears[0].member,
            submitted.delta.removed[0].member
        );
        assert_eq!(
            submitted.delta.retry_clears[0].cause,
            MempoolRetryClearCause::LifecycleRemoval
        );
        if request_alias {
            assert_ne!(requested_wtxid, existing_wtxid);
            assert_ne!(submitted.delta.removed[0].member.wtxid, requested_wtxid);
        }
    }
}

#[test]
fn package_rbf_delta_is_atomic_and_dry_run_matches_submit() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let (original_parent, original_child, replacement_parent, replacement_child) =
        package_rbf_transactions(coinbase_txids[0]);
    let original_parent_txid = transaction_txid(&original_parent).expect("original parent txid");
    let original_child_txid = transaction_txid(&original_child).expect("original child txid");
    let replacement_parent_txid =
        transaction_txid(&replacement_parent).expect("replacement parent txid");
    let replacement_child_txid =
        transaction_txid(&replacement_child).expect("replacement child txid");
    let package =
        WellFormedPackage::try_from(vec![replacement_parent.clone(), replacement_child.clone()])
            .expect("well-formed replacement");
    let submission = SubmissionPackage::try_from_package(package.clone(), &snapshot)
        .expect("submission refinement");
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, original_parent).expect("original parent");
    submit(&mut mempool, &snapshot, original_child).expect("original child");
    let before_dry_run = mempool.complete_snapshot();
    let verify_flags = ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY;
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    };

    // Act
    let dry_run = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags,
            consensus_params,
        )
        .expect("replacement dry-run");
    assert_eq!(mempool.complete_snapshot(), before_dry_run);
    let submitted = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags,
            consensus_params,
        )
        .expect("replacement submission");

    // Assert
    assert_eq!(dry_run.report, submitted.report);
    assert_eq!(
        submitted
            .delta
            .admitted
            .iter()
            .map(|member| member.txid)
            .collect::<Vec<_>>(),
        vec![replacement_parent_txid, replacement_child_txid]
    );
    assert_eq!(submitted.delta.removed.len(), 2);
    assert!(submitted.delta.removed.iter().any(|removal| {
        removal.member.txid == original_parent_txid
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(submitted.delta.removed.iter().any(|removal| {
        removal.member.txid == original_child_txid
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert_eq!(
        submitted.delta.retry_clears.len(),
        submitted.delta.removed.len()
    );
    assert!(submitted.delta.removed.iter().all(|removal| {
        submitted.delta.retry_clears.iter().any(|clear| {
            clear.member == removal.member
                && clear.cause == MempoolRetryClearCause::LifecycleRemoval
        })
    }));
    assert!(!mempool.entries().contains_key(&original_parent_txid));
    assert!(!mempool.entries().contains_key(&original_child_txid));
    assert!(mempool.entries().contains_key(&replacement_parent_txid));
    assert!(mempool.entries().contains_key(&replacement_child_txid));
}

#[test]
fn package_rbf_replacement_rollback_preserves_complete_state() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let (original_parent, original_child, _replacement_parent, _replacement_child) =
        package_rbf_transactions(coinbase_txids[0]);
    let weak_parent = spend_transaction(
        coinbase_txids[0],
        0,
        499_998_500,
        TransactionInput::SEQUENCE_FINAL,
    );
    let weak_parent_txid = transaction_txid(&weak_parent).expect("weak parent txid");
    let weak_child = spend_transaction(
        weak_parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![weak_parent, weak_child]).expect("well-formed package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, original_parent).expect("original parent");
    submit(&mut mempool, &snapshot, original_child).expect("original child");
    let before = mempool.complete_snapshot();

    // Act
    let submitted = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("policy rejection is reported");

    // Assert
    assert!(submitted.delta.is_empty());
    assert!(submitted.report.members().iter().all(|member| matches!(
        member,
        PackageMemberResult::HardRejected(crate::HardMemberFailure::PackageReplacement { .. })
    )));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn package_rbf_replacement_rollback_after_staged_removals_on_script_failure() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let (original_parent, original_child, replacement_parent, mut replacement_child) =
        package_rbf_transactions(coinbase_txids[0]);
    replacement_child.inputs[0].script_sig = script(&[0x01, 0x52]);
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![replacement_parent, replacement_child])
            .expect("well-formed package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, original_parent).expect("original parent");
    submit(&mut mempool, &snapshot, original_child).expect("original child");
    let before = mempool.complete_snapshot();

    // Act
    let submitted = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("script failure is reported");

    // Assert
    assert!(submitted.delta.is_empty());
    assert!(submitted.report.members().iter().any(|member| matches!(
        member,
        PackageMemberResult::HardRejected(crate::HardMemberFailure::Policy { reason, .. })
        if reason.contains("mandatory-script-verify-flag-failed")
    )));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn package_rbf_replacement_trim_keeps_one_truthful_final_delta() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let (original_parent, original_child, replacement_parent, replacement_child) =
        package_rbf_transactions(coinbase_txids[0]);
    let original_txids = [
        transaction_txid(&original_parent).expect("original parent txid"),
        transaction_txid(&original_child).expect("original child txid"),
    ];
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![replacement_parent, replacement_child])
            .expect("well-formed package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, original_parent).expect("original parent");
    submit(&mut mempool, &snapshot, original_child).expect("original child");
    super::super::package_admission::set_mempool_capacity_for_test(
        &mut mempool,
        MempoolCapacity::new(0),
    );

    // Act
    let submitted = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("replacement and trim");

    // Assert
    assert!(mempool.entries().is_empty());
    assert!(submitted.delta.admitted.is_empty());
    assert!(original_txids.iter().all(|txid| {
        submitted.delta.removed.iter().any(|removal| {
            removal.member.txid == *txid && removal.cause == MempoolRemovalCause::Replacement
        })
    }));
    assert!(submitted.report.members().iter().all(|member| matches!(
        member,
        PackageMemberResult::PostTrimAbsent(crate::PostTrimAbsence {
            prior: PriorMemberSuccess::FinallyPresent { .. },
            ..
        })
    )));
}

#[test]
fn package_rbf_replacement_composition_failure_rolls_back() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let (original_parent, original_child, replacement_parent, replacement_child) =
        package_rbf_transactions(coinbase_txids[0]);
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![replacement_parent, replacement_child])
            .expect("well-formed package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mut mempool = Mempool::default();
    submit(&mut mempool, &snapshot, original_parent).expect("original parent");
    submit(&mut mempool, &snapshot, original_child).expect("original child");
    let before = mempool.complete_snapshot();

    // Act
    super::super::package_admission::force_duplicate_transition_entry_for_test(true);
    let result = mempool.submit_package(
        SubmitPackageCommand {
            package: submission,
            context: AdmissionContext::legacy_unknown(),
        },
        &snapshot,
        ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
            | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        ConsensusParams {
            coinbase_maturity: 1,
            ..ConsensusParams::default()
        },
    );
    super::super::package_admission::force_duplicate_transition_entry_for_test(false);

    // Assert
    assert!(matches!(
        result,
        Err(crate::MempoolError::InternalInvariant { ref reason })
        if reason.contains("duplicate txid")
    ));
    assert_eq!(mempool.complete_snapshot(), before);
}
