// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/test/txvalidation_tests.cpp
// - packages/bitcoin-knots/test/functional/mempool_package_rbf.py
// - packages/bitcoin-knots/test/functional/mempool_truc.py
// - packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py

use super::*;

#[test]
fn max_bound_shape_fingerprint_order_and_try_from_package_refinement_are_pinned() {
    // Arrange
    let (_, confirmed_txids) = sample_chainstate_snapshot(27);
    let members = confirmed_txids
        .iter()
        .take(MAX_PACKAGE_COUNT)
        .enumerate()
        .map(|(index, txid)| {
            spend_transaction(
                *txid,
                0,
                499_999_000 - index as i64,
                TransactionInput::SEQUENCE_FINAL,
            )
        })
        .collect::<Vec<_>>();

    // Act
    let empty = WellFormedPackage::try_from(Vec::new());
    let max_bound = WellFormedPackage::try_from(members.clone()).expect("25-member package");
    let too_many = WellFormedPackage::try_from(
        confirmed_txids
            .iter()
            .take(MAX_PACKAGE_COUNT + 1)
            .enumerate()
            .map(|(index, txid)| {
                spend_transaction(
                    *txid,
                    0,
                    499_999_000 - index as i64,
                    TransactionInput::SEQUENCE_FINAL,
                )
            })
            .collect::<Vec<_>>(),
    );
    let first = WellFormedPackage::try_from(vec![members[0].clone(), members[1].clone()])
        .expect("first permutation");
    let second = WellFormedPackage::try_from(vec![members[1].clone(), members[0].clone()])
        .expect("second permutation");
    let singleton = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![members[0].clone()]).expect("singleton"),
        &empty_snapshot(),
    )
    .expect("singleton refinement");

    // Assert
    assert!(matches!(empty, Err(PackageShapeError::Empty)));
    assert_eq!(max_bound.len(), 25);
    assert_eq!(MAX_PACKAGE_WEIGHT, 404_000);
    assert!(matches!(
        too_many,
        Err(PackageShapeError::TooManyTransactions {
            count: 26,
            maximum: 25
        })
    ));
    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(first.maybe_identity_at(0), second.maybe_identity_at(1));
    assert_eq!(singleton.kind(), SubmissionPackageKind::Single);
}

#[test]
fn positive_child_with_unconfirmed_parents_try_from_package_refinement_is_checked() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        confirmed_txids[0],
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

    // Act
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![parent, child]).expect("well-formed package"),
        &snapshot,
    )
    .expect("CWUP refinement");

    // Assert
    assert_eq!(
        submission.kind(),
        SubmissionPackageKind::ChildWithUnconfirmedParents
    );
}

#[test]
fn missing_input_report_carries_one_exact_parent() {
    // Arrange
    let missing_parent = Txid::from_byte_array([0x81; 32]);
    let transaction = spend_transaction(missing_parent, 0, 1_000, TransactionInput::SEQUENCE_FINAL);
    let package = WellFormedPackage::try_from(vec![transaction]).expect("singleton package");

    // Act
    let result = Mempool::default()
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &empty_snapshot(),
            verify_flags(),
            consensus_params(),
        )
        .expect("missing-input report");

    // Assert
    assert!(matches!(
        result.report.members(),
        [PackageMemberResult::Reconsiderable(
            ReconsiderableMemberFailure::MissingInputs {
                missing_parents,
                ..
            }
        )] if missing_parents == &[missing_parent]
    ));
}

#[test]
fn multiple_missing_parents_are_sorted_and_deduplicated() {
    // Arrange
    let first_missing_parent = Txid::from_byte_array([0x21; 32]);
    let second_missing_parent = Txid::from_byte_array([0x42; 32]);
    let mut transaction = spend_transaction(
        second_missing_parent,
        0,
        1_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    transaction.inputs.extend([
        TransactionInput {
            previous_output: OutPoint {
                txid: first_missing_parent,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        },
        TransactionInput {
            previous_output: OutPoint {
                txid: second_missing_parent,
                vout: 1,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        },
    ]);
    let package = WellFormedPackage::try_from(vec![transaction]).expect("singleton package");

    // Act
    let result = Mempool::default()
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &empty_snapshot(),
            verify_flags(),
            consensus_params(),
        )
        .expect("missing-input report");

    // Assert
    assert!(matches!(
        result.report.members(),
        [PackageMemberResult::Reconsiderable(
            ReconsiderableMemberFailure::MissingInputs {
                missing_parents,
                ..
            }
        )] if missing_parents == &[first_missing_parent, second_missing_parent]
    ));
}

#[test]
fn earlier_staged_parent_is_not_reported_missing() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        confirmed_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let external_missing_parent = Txid::from_byte_array([0x63; 32]);
    let mut child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    child.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: external_missing_parent,
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    let package = WellFormedPackage::try_from(vec![parent, child]).expect("ordered package");

    // Act
    let result = Mempool::default()
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("partial package report");

    // Assert
    assert!(matches!(
        result.report.members(),
        [
            PackageMemberResult::FinallyPresent(_),
            PackageMemberResult::Reconsiderable(
                ReconsiderableMemberFailure::MissingInputs {
                    missing_parents,
                    ..
                }
            )
        ] if missing_parents == &[external_missing_parent]
    ));
}

#[test]
fn unrelated_confirmed_input_is_not_reported_missing() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(2);
    let external_missing_parent = Txid::from_byte_array([0x84; 32]);
    let mut transaction = spend_transaction(
        external_missing_parent,
        0,
        1_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    transaction.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: confirmed_txids[0],
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    let package = WellFormedPackage::try_from(vec![transaction]).expect("singleton package");

    // Act
    let result = Mempool::default()
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("missing-input report");

    // Assert
    assert!(matches!(
        result.report.members(),
        [PackageMemberResult::Reconsiderable(
            ReconsiderableMemberFailure::MissingInputs {
                missing_parents,
                ..
            }
        )] if missing_parents == &[external_missing_parent]
    ));
}

#[test]
fn singleton_missing_parent_invariant_error_propagates() {
    // Arrange
    let missing_parent = Txid::from_byte_array([0xa5; 32]);
    let transaction = spend_transaction(missing_parent, 0, 1_000, TransactionInput::SEQUENCE_FINAL);
    let package = WellFormedPackage::try_from(vec![transaction]).expect("singleton package");
    fail_missing_parent_report_on_call_for_test(1);

    // Act
    let result = Mempool::default().dry_run_package(
        DryRunPackageCommand {
            package,
            context: AdmissionContext::legacy_unknown(),
        },
        &empty_snapshot(),
        verify_flags(),
        consensus_params(),
    );
    fail_missing_parent_report_on_call_for_test(0);

    // Assert
    assert!(matches!(
        result,
        Err(MempoolError::InternalInvariant { reason })
            if reason == "missing-input evaluation produced no absent parent"
    ));
}

#[test]
fn residual_missing_parent_invariant_error_propagates() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        confirmed_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let external_missing_parent = Txid::from_byte_array([0xc6; 32]);
    let mut child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    child.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: external_missing_parent,
            vout: 0,
        },
        script_sig: script(&[0x01, 0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    let package = WellFormedPackage::try_from(vec![parent, child]).expect("ordered package");
    fail_missing_parent_report_on_call_for_test(2);

    // Act
    let result = Mempool::default().dry_run_package(
        DryRunPackageCommand {
            package,
            context: AdmissionContext::legacy_unknown(),
        },
        &snapshot,
        verify_flags(),
        consensus_params(),
    );
    fail_missing_parent_report_on_call_for_test(0);

    // Assert
    assert!(matches!(
        result,
        Err(MempoolError::InternalInvariant { reason })
            if reason == "missing-input evaluation produced no absent parent"
    ));
}

#[test]
fn too_few_too_many_swapped_identity_and_mismatched_status_reports_reject() {
    // Arrange
    let (_, confirmed_txids) = sample_chainstate_snapshot(2);
    let package = WellFormedPackage::try_from(vec![
        spend_transaction(
            confirmed_txids[0],
            0,
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
        spend_transaction(
            confirmed_txids[1],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    ])
    .expect("report package");
    let first = package.maybe_identity_at(0).expect("first identity");
    let second = package.maybe_identity_at(1).expect("second identity");
    let failure = |requested| {
        PackageMemberResult::HardRejected(HardMemberFailure::Policy {
            requested,
            category: MempoolRejectionCategory::InternalInvariant,
            reason: "fixture".to_string(),
        })
    };

    // Act
    let too_few = PackageReport::try_new(&package, PackageStatus::Failed, vec![], vec![]);
    let too_many = PackageReport::try_new(
        &package,
        PackageStatus::Failed,
        vec![failure(first), failure(second), failure(first)],
        vec![],
    );
    let swapped_identity = PackageReport::try_new(
        &package,
        PackageStatus::Failed,
        vec![failure(second), failure(first)],
        vec![],
    );
    let mismatched_status = PackageReport::try_new(
        &package,
        PackageStatus::Complete,
        vec![failure(first), failure(second)],
        vec![],
    );

    // Assert
    assert!(matches!(
        too_few,
        Err(PackageReportError::MemberCountMismatch { actual: 0, .. })
    ));
    assert!(matches!(
        too_many,
        Err(PackageReportError::MemberCountMismatch { actual: 3, .. })
    ));
    assert!(matches!(
        swapped_identity,
        Err(PackageReportError::IdentityMismatch { index: 0 })
    ));
    assert!(matches!(
        mismatched_status,
        Err(PackageReportError::StatusMismatch { .. })
    ));
}

#[test]
fn empty_duplicate_and_inconsistent_rate_fee_group_validation_is_checked() {
    // Arrange
    let id = EffectiveFeeGroupId::from_u64(7);
    let wtxid = identity(1).wtxid;
    let virtual_size = TransactionVirtualSize::new(100);

    // Act
    let empty_fee_group = checked_fee_group(id, vec![]);
    let duplicate_fee_group = checked_fee_group(id, vec![wtxid, wtxid]);
    let inconsistent_rate = EffectiveFeeGroup::try_new(
        id,
        vec![wtxid],
        Amount::from_sats(200).expect("base fee"),
        Amount::from_sats(300).expect("modified fee"),
        virtual_size,
        FeeRate::from_sats_per_kvb(2_999),
    );

    // Assert
    assert_eq!(
        empty_fee_group,
        Err(EffectiveFeeGroupError::EmptyMembership)
    );
    assert!(matches!(
        duplicate_fee_group,
        Err(EffectiveFeeGroupError::DuplicateMembership { .. })
    ));
    assert!(matches!(
        inconsistent_rate,
        Err(EffectiveFeeGroupError::InconsistentEffectiveRate { .. })
    ));
}
