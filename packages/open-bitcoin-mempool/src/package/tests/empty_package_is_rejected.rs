// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/packages.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp

use super::*;

#[test]
fn empty_package_is_rejected() {
    // Arrange
    let transactions = vec![];

    // Act
    let result = WellFormedPackage::try_from(transactions);

    // Assert
    assert!(matches!(result, Err(PackageShapeError::Empty)));
}

#[test]
fn exactly_maximum_package_count_is_accepted() {
    // Arrange
    let transactions: Vec<Transaction> = (1..=MAX_PACKAGE_COUNT)
        .map(|seed| transaction_with_input(seed as u8))
        .collect();

    // Act
    let package = WellFormedPackage::try_from(transactions).expect("maximum count is valid");

    // Assert
    assert_eq!(package.len(), MAX_PACKAGE_COUNT);
}

#[test]
fn package_above_maximum_count_is_rejected() {
    // Arrange
    let transactions: Vec<Transaction> = (0..=MAX_PACKAGE_COUNT)
        .map(|seed| transaction_with_input((seed + 1) as u8))
        .collect();

    // Act
    let result = WellFormedPackage::try_from(transactions);

    // Assert
    assert!(matches!(
        result,
        Err(PackageShapeError::TooManyTransactions { count, maximum })
            if count == MAX_PACKAGE_COUNT + 1 && maximum == MAX_PACKAGE_COUNT
    ));
}

#[test]
fn exactly_maximum_package_weight_is_accepted() {
    // Arrange
    let transaction = transaction_with_weight(MAX_PACKAGE_WEIGHT);

    // Act
    let package = WellFormedPackage::try_from(vec![transaction]).expect("maximum weight is valid");

    // Assert
    assert_eq!(package.total_weight(), MAX_PACKAGE_WEIGHT);
}

#[test]
fn package_above_maximum_weight_is_rejected() {
    // Arrange
    let transaction = transaction_with_weight(MAX_PACKAGE_WEIGHT + 1);

    // Act
    let result = WellFormedPackage::try_from(vec![transaction]);

    // Assert
    assert!(matches!(
        result,
        Err(PackageShapeError::TooHeavy { weight, maximum })
            if weight == MAX_PACKAGE_WEIGHT + 1 && maximum == MAX_PACKAGE_WEIGHT
    ));
}

#[test]
fn same_txid_with_different_witness_is_rejected_as_duplicate_txid() {
    // Arrange
    let mut first = transaction_with_input(10);
    first.inputs[0].witness = ScriptWitness::new(vec![vec![1]]);
    let mut second = first.clone();
    second.inputs[0].witness = ScriptWitness::new(vec![vec![2]]);
    assert_eq!(
        transaction_txid(&first).expect("first txid"),
        transaction_txid(&second).expect("second txid"),
    );
    assert_ne!(
        transaction_wtxid(&first).expect("first wtxid"),
        transaction_wtxid(&second).expect("second wtxid"),
    );

    // Act
    let result = WellFormedPackage::try_from(vec![first, second]);

    // Assert
    assert!(matches!(
        result,
        Err(PackageShapeError::DuplicateTxid { .. })
    ));
}

#[test]
fn repeated_transaction_is_rejected_as_duplicate_wtxid() {
    // Arrange
    let transaction = transaction_with_input(11);

    // Act
    let result = WellFormedPackage::try_from(vec![transaction.clone(), transaction]);

    // Assert
    assert!(matches!(
        result,
        Err(PackageShapeError::DuplicateWtxid { .. })
    ));
}

#[test]
fn child_before_parent_is_rejected() {
    // Arrange
    let parent = transaction_with_input(12);
    let child = child_of(&parent, 13);

    // Act
    let result = WellFormedPackage::try_from(vec![child, parent]);

    // Assert
    assert!(matches!(
        result,
        Err(PackageShapeError::ChildBeforeParent { child_index: 0, .. })
    ));
}

#[test]
fn zero_input_member_is_rejected() {
    // Arrange
    let transaction = Transaction {
        inputs: vec![],
        ..transaction_with_input(14)
    };

    // Act
    let result = WellFormedPackage::try_from(vec![transaction]);

    // Assert
    assert!(matches!(
        result,
        Err(PackageShapeError::ZeroInputMember { index: 0 })
    ));
}

#[test]
fn cross_member_double_spend_is_rejected() {
    // Arrange
    let first = transaction_with_input(15);
    let mut second = transaction_with_input(16);
    second.inputs[0].previous_output = first.inputs[0].previous_output.clone();

    // Act
    let result = WellFormedPackage::try_from(vec![first, second]);

    // Assert
    assert!(matches!(
        result,
        Err(PackageShapeError::CrossMemberInputConflict {
            first_index: 0,
            second_index: 1,
            ..
        })
    ));
}

#[test]
fn duplicate_inputs_within_one_member_remain_shape_valid() {
    // Arrange
    let mut transaction = transaction_with_input(17);
    transaction.inputs.push(transaction.inputs[0].clone());

    // Act
    let package = WellFormedPackage::try_from(vec![transaction]);

    // Assert
    assert!(package.is_ok());
}

#[test]
fn singleton_refines_only_through_checked_boundary() {
    // Arrange
    let package =
        WellFormedPackage::try_from(vec![transaction_with_input(18)]).expect("valid package");
    let snapshot = ChainstateSnapshot::new(vec![], HashMap::new(), HashMap::new());

    // Act
    let submission =
        SubmissionPackage::try_from_package(package, &snapshot).expect("valid singleton");

    // Assert
    assert_eq!(submission.kind(), SubmissionPackageKind::Single);
    assert_eq!(submission.package().len(), 1);
}

#[test]
fn direct_parents_and_child_refine_through_checked_boundary() {
    // Arrange
    let parent = transaction_with_input(19);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let confirmed_input = OutPoint {
        txid: Txid::from_byte_array([20; 32]),
        vout: 0,
    };
    let mut child = child_of(&parent, 21);
    child.inputs.push(TransactionInput {
        previous_output: confirmed_input.clone(),
        ..transaction_with_input(22).inputs[0].clone()
    });
    let package = WellFormedPackage::try_from(vec![parent, child]).expect("valid package");
    let snapshot = snapshot_with_utxo(confirmed_input);

    // Act
    let submission =
        SubmissionPackage::try_from_package(package, &snapshot).expect("valid refinement");

    // Assert
    assert_eq!(
        submission.kind(),
        SubmissionPackageKind::ChildWithUnconfirmedParents
    );
    assert_eq!(
        submission
            .package()
            .maybe_identity_at(0)
            .expect("parent")
            .txid,
        parent_txid
    );
}

#[test]
fn unrelated_member_is_rejected_by_submission_refinement() {
    // Arrange
    let package =
        WellFormedPackage::try_from(vec![transaction_with_input(23), transaction_with_input(24)])
            .expect("shape-valid package");
    let snapshot = ChainstateSnapshot::new(vec![], HashMap::new(), HashMap::new());

    // Act
    let result = SubmissionPackage::try_from_package(package, &snapshot);

    // Assert
    assert!(matches!(
        result,
        Err(PackageShapeError::NotChildWithUnconfirmedParents)
    ));
}

#[test]
fn three_generation_package_is_rejected_by_submission_refinement() {
    // Arrange
    let grandparent = transaction_with_input(25);
    let parent = child_of(&grandparent, 26);
    let child = child_of(&parent, 27);
    let package =
        WellFormedPackage::try_from(vec![grandparent, parent, child]).expect("shape-valid package");
    let snapshot = ChainstateSnapshot::new(vec![], HashMap::new(), HashMap::new());

    // Act
    let result = SubmissionPackage::try_from_package(package, &snapshot);

    // Assert
    assert!(matches!(
        result,
        Err(PackageShapeError::NotChildWithUnconfirmedParents)
    ));
}

#[test]
fn absent_unconfirmed_parent_is_rejected_by_submission_refinement() {
    // Arrange
    let parent = transaction_with_input(28);
    let missing_input = OutPoint {
        txid: Txid::from_byte_array([29; 32]),
        vout: 0,
    };
    let mut child = child_of(&parent, 30);
    child.inputs.push(TransactionInput {
        previous_output: missing_input.clone(),
        ..transaction_with_input(31).inputs[0].clone()
    });
    let package = WellFormedPackage::try_from(vec![parent, child]).expect("shape-valid package");
    let snapshot = ChainstateSnapshot::new(vec![], HashMap::new(), HashMap::new());

    // Act
    let result = SubmissionPackage::try_from_package(package, &snapshot);

    // Assert
    assert_eq!(
        result,
        Err(PackageShapeError::MissingUnconfirmedParent {
            outpoint: missing_input
        })
    );
}

#[test]
fn package_fingerprint_matches_knots_fixed_vector() {
    // Arrange
    let wtxids = [
        wtxid_from_display_hex("85cd1a31eb38f74ed5742ec9cb546712ab5aaf747de28a9168b53e846cbda17f"),
        wtxid_from_display_hex("b4749f017444b051c44dfd2720e88f314ff94f3dd6d56d40ef65854fcd7fff6b"),
        wtxid_from_display_hex("e065bac15f62bb4e761d761db928ddee65a47296b2b776785abb912cdec474e3"),
    ];
    let expected = decode_hex("0f076ece34c184294c64badd99a4444bb0140d0812f3c76baf466180db2e4fb0");

    // Act
    let fingerprint = PackageFingerprint::from_wtxids(&wtxids);

    // Assert
    assert_eq!(fingerprint.as_bytes().as_slice(), expected.as_slice());
}

#[test]
fn package_fingerprint_is_permutation_independent_without_reordering_members() {
    // Arrange
    let first = transaction_with_input(32);
    let second = transaction_with_input(33);
    let third = transaction_with_input(34);
    let first_txid = transaction_txid(&first).expect("first txid");
    let second_txid = transaction_txid(&second).expect("second txid");
    let third_txid = transaction_txid(&third).expect("third txid");

    // Act
    let package = WellFormedPackage::try_from(vec![first.clone(), second.clone(), third.clone()])
        .expect("valid package");
    let permutation =
        WellFormedPackage::try_from(vec![third, first, second]).expect("valid permutation");
    let ordered_txids: Vec<Txid> = package
        .members()
        .map(|transaction| transaction_txid(transaction).expect("member txid"))
        .collect();

    // Assert
    assert_eq!(package.fingerprint(), permutation.fingerprint());
    assert_eq!(ordered_txids, vec![first_txid, second_txid, third_txid]);
}

#[test]
fn checked_package_observations_use_cached_request_order() {
    // Arrange
    let first = transaction_with_input(35);
    let second = transaction_with_input(36);
    let first_identity = MempoolMemberIdentity {
        txid: transaction_txid(&first).expect("first txid"),
        wtxid: transaction_wtxid(&first).expect("first wtxid"),
    };
    let second_identity = MempoolMemberIdentity {
        txid: transaction_txid(&second).expect("second txid"),
        wtxid: transaction_wtxid(&second).expect("second wtxid"),
    };

    // Act
    let package = WellFormedPackage::try_from(vec![first, second]).expect("valid package");

    // Assert
    assert!(!package.is_empty());
    assert_eq!(package.maybe_identity_at(0), Some(first_identity));
    assert_eq!(package.maybe_identity_at(2), None);
    assert_eq!(
        package.maybe_identity_for_txid(second_identity.txid),
        Some(second_identity)
    );
    assert_eq!(
        package.maybe_identity_for_txid(Txid::from_byte_array([0x44; 32])),
        None
    );
    assert_eq!(
        package.maybe_identity_for_wtxid(first_identity.wtxid),
        Some(first_identity)
    );
    assert_eq!(
        package.maybe_identity_for_wtxid(Wtxid::from_byte_array([0x55; 32])),
        None
    );
    assert_eq!(package.maybe_input_index_at(1), Some(1));
    assert_eq!(package.maybe_input_index_at(2), None);
}

#[test]
fn package_shape_errors_preserve_typed_human_readable_context() {
    // Arrange
    let txid = Txid::from_byte_array([0x66; 32]);
    let wtxid = Wtxid::from_byte_array([0x77; 32]);
    let outpoint = OutPoint { txid, vout: 3 };
    let errors = [
        PackageShapeError::Empty,
        PackageShapeError::TooManyTransactions {
            count: 26,
            maximum: 25,
        },
        transaction_encoding_error(2, "fixture encoding error"),
        PackageShapeError::TotalWeightOverflow,
        PackageShapeError::TooHeavy {
            weight: 404_001,
            maximum: 404_000,
        },
        PackageShapeError::DuplicateTxid { txid },
        PackageShapeError::DuplicateWtxid { wtxid },
        PackageShapeError::ZeroInputMember { index: 4 },
        PackageShapeError::ChildBeforeParent {
            child_index: 0,
            parent_txid: txid,
        },
        PackageShapeError::CrossMemberInputConflict {
            outpoint: outpoint.clone(),
            first_index: 0,
            second_index: 1,
        },
        PackageShapeError::NotChildWithUnconfirmedParents,
        PackageShapeError::MissingUnconfirmedParent { outpoint },
    ];

    // Act
    let messages: Vec<String> = errors.iter().map(ToString::to_string).collect();

    // Assert
    assert_eq!(messages.len(), errors.len());
    assert!(messages.iter().all(|message| !message.is_empty()));
    assert!(messages[1].contains("26"));
    assert!(messages[2].contains("fixture encoding error"));
    assert!(messages[8].contains("parent"));
    assert!(messages[11].contains("neither supplied nor present"));
}

#[test]
fn submission_refinement_defends_against_an_internal_empty_package() {
    // Arrange
    let impossible_package = WellFormedPackage {
        members: Vec::<PackageMember>::new(),
        fingerprint: PackageFingerprint([0; 32]),
    };
    let snapshot = ChainstateSnapshot::new(vec![], HashMap::new(), HashMap::new());

    // Act
    let result = SubmissionPackage::try_from_package(impossible_package, &snapshot);

    // Assert
    assert_eq!(result, Err(PackageShapeError::Empty));
}

#[test]
fn too_few_member_results_are_rejected() {
    // Arrange
    let package = report_fixture();

    // Act
    let result = PackageReport::try_new(&package, PackageStatus::Failed, vec![], vec![]);

    // Assert
    assert!(result.is_err());
}
