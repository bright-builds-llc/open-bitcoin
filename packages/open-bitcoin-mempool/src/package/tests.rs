// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/packages.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp

use std::collections::HashMap;

use open_bitcoin_chainstate::{ChainstateSnapshot, Coin};
use open_bitcoin_consensus::{transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    Txid, Wtxid,
};

use super::{
    EffectiveFeeGroup, EffectiveFeeGroupId, HardMemberFailure, MAX_PACKAGE_COUNT,
    MAX_PACKAGE_WEIGHT, NewlyPresent, PackageFingerprint, PackageMemberResult, PackageReport,
    PackageShapeError, PackageStatus, PriorMemberSuccess, ReconsiderableMemberFailure,
    SubmissionPackage, SubmissionPackageKind, WellFormedPackage, WitnessAlias,
};
use super::{PackageMember, shape::transaction_encoding_error};
use crate::{
    FeeRate, MempoolMemberIdentity, TransactionVirtualSize, transaction_weight_and_virtual_size,
};

fn transaction_with_input(seed: u8) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([seed; 32]),
                vout: u32::from(seed),
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(i64::from(seed) + 1).expect("fixture amount"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: u32::from(seed),
    }
}

fn child_of(parent: &Transaction, seed: u8) -> Transaction {
    let parent_txid = transaction_txid(parent).expect("fixture parent txid");
    Transaction {
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: parent_txid,
                vout: 0,
            },
            ..transaction_with_input(seed).inputs[0].clone()
        }],
        ..transaction_with_input(seed)
    }
}

fn transaction_with_weight(target_weight: usize) -> Transaction {
    let mut payload_len = target_weight.saturating_sub(256);
    for _ in 0..4 {
        let mut transaction = transaction_with_input(201);
        transaction.inputs[0].witness = ScriptWitness::new(vec![vec![0_u8; payload_len]]);
        let (weight, _) =
            transaction_weight_and_virtual_size(&transaction).expect("fixture weight");
        if weight == target_weight {
            return transaction;
        }
        if weight < target_weight {
            payload_len += target_weight - weight;
        } else {
            payload_len -= weight - target_weight;
        }
    }

    panic!("unable to construct exact-weight transaction fixture");
}

fn snapshot_with_utxo(outpoint: OutPoint) -> ChainstateSnapshot {
    let coin = Coin {
        output: TransactionOutput {
            value: Amount::from_sats(10_000).expect("fixture amount"),
            script_pubkey: ScriptBuf::default(),
        },
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 1,
    };
    ChainstateSnapshot::new(vec![], HashMap::from([(outpoint, coin)]), HashMap::new())
}

fn wtxid_from_display_hex(display_hex: &str) -> Wtxid {
    let mut bytes = decode_hex(display_hex);
    bytes.reverse();
    Wtxid::from_byte_array(bytes.try_into().expect("32-byte wtxid fixture"))
}

fn decode_hex(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = char::from(pair[0]).to_digit(16).expect("hex fixture");
            let low = char::from(pair[1]).to_digit(16).expect("hex fixture");
            ((high << 4) | low) as u8
        })
        .collect()
}

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

fn report_fixture() -> WellFormedPackage {
    WellFormedPackage::try_from(vec![transaction_with_input(70), transaction_with_input(71)])
        .expect("report fixture package")
}

fn report_identity(package: &WellFormedPackage, index: usize) -> MempoolMemberIdentity {
    package
        .maybe_identity_at(index)
        .expect("fixture identity at index")
}

fn fee_group(
    id: EffectiveFeeGroupId,
    ordered_wtxids: Vec<Wtxid>,
) -> Result<EffectiveFeeGroup, super::EffectiveFeeGroupError> {
    let base_fee_sats = Amount::from_sats(200).expect("valid base fee");
    let modified_fee_sats = Amount::from_sats(300).expect("valid modified fee");
    let virtual_size = TransactionVirtualSize::new(100);
    let effective_fee_rate = FeeRate::from_fee_sats_and_vbytes(300, virtual_size);
    EffectiveFeeGroup::try_new(
        id,
        ordered_wtxids,
        base_fee_sats,
        modified_fee_sats,
        virtual_size,
        effective_fee_rate,
    )
}

fn finally_present(
    requested: MempoolMemberIdentity,
    effective_fee_group_id: EffectiveFeeGroupId,
) -> PackageMemberResult {
    PackageMemberResult::FinallyPresent(NewlyPresent {
        requested,
        effective_fee_group_id,
    })
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

#[test]
fn too_many_member_results_are_rejected() {
    // Arrange
    let package = report_fixture();
    let requested = report_identity(&package, 0);
    let failure = || {
        PackageMemberResult::HardRejected(HardMemberFailure::Policy {
            requested,
            reason: "fixture".to_string(),
        })
    };

    // Act
    let result = PackageReport::try_new(
        &package,
        PackageStatus::Failed,
        vec![failure(), failure(), failure()],
        vec![],
    );

    // Assert
    assert!(result.is_err());
}

#[test]
fn swapped_identity_order_is_rejected() {
    // Arrange
    let package = report_fixture();
    let first = report_identity(&package, 0);
    let second = report_identity(&package, 1);
    let members = vec![
        PackageMemberResult::HardRejected(HardMemberFailure::Policy {
            requested: second,
            reason: "second first".to_string(),
        }),
        PackageMemberResult::HardRejected(HardMemberFailure::Policy {
            requested: first,
            reason: "first second".to_string(),
        }),
    ];

    // Act
    let result = PackageReport::try_new(&package, PackageStatus::Failed, members, vec![]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn mismatched_status_is_rejected() {
    // Arrange
    let package = report_fixture();
    let members = vec![
        PackageMemberResult::HardRejected(HardMemberFailure::Policy {
            requested: report_identity(&package, 0),
            reason: "first".to_string(),
        }),
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::MissingInputs {
            requested: report_identity(&package, 1),
        }),
    ];

    // Act
    let result = PackageReport::try_new(&package, PackageStatus::Complete, members, vec![]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn empty_fee_group_is_rejected() {
    // Arrange
    let id = EffectiveFeeGroupId::from_u64(1);
    let virtual_size = TransactionVirtualSize::new(100);

    // Act
    let result = EffectiveFeeGroup::try_new(
        id,
        vec![],
        Amount::from_sats(200).expect("base fee"),
        Amount::from_sats(300).expect("modified fee"),
        virtual_size,
        FeeRate::from_fee_sats_and_vbytes(300, virtual_size),
    );

    // Assert
    assert!(result.is_err());
}

#[test]
fn duplicate_group_membership_is_rejected() {
    // Arrange
    let id = EffectiveFeeGroupId::from_u64(2);
    let wtxid = Wtxid::from_byte_array([0x82; 32]);

    // Act
    let result = fee_group(id, vec![wtxid, wtxid]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn zero_vsize_fee_group_is_rejected() {
    // Arrange
    let id = EffectiveFeeGroupId::from_u64(3);
    let wtxid = Wtxid::from_byte_array([0x83; 32]);

    // Act
    let result = EffectiveFeeGroup::try_new(
        id,
        vec![wtxid],
        Amount::from_sats(200).expect("base fee"),
        Amount::from_sats(300).expect("modified fee"),
        TransactionVirtualSize::ZERO,
        FeeRate::ZERO,
    );

    // Assert
    assert!(result.is_err());
}

#[test]
fn inconsistent_effective_rate_is_rejected() {
    // Arrange
    let id = EffectiveFeeGroupId::from_u64(4);
    let wtxid = Wtxid::from_byte_array([0x84; 32]);

    // Act
    let result = EffectiveFeeGroup::try_new(
        id,
        vec![wtxid],
        Amount::from_sats(200).expect("base fee"),
        Amount::from_sats(300).expect("modified fee"),
        TransactionVirtualSize::new(100),
        FeeRate::from_sats_per_kvb(9_999),
    );

    // Assert
    assert!(result.is_err());
}

#[test]
fn eligible_only_group_references_are_enforced() {
    // Arrange
    let package = report_fixture();
    let first = report_identity(&package, 0);
    let second = report_identity(&package, 1);
    let id = EffectiveFeeGroupId::from_u64(5);
    let group = fee_group(id, vec![first.wtxid, second.wtxid]).expect("valid group shape");
    let members = vec![
        finally_present(first, id),
        PackageMemberResult::AlreadyPresent(super::ExistingMember { requested: second }),
    ];

    // Act
    let result = PackageReport::try_new(&package, PackageStatus::Complete, members, vec![group]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn duplicate_group_ids_and_cross_group_membership_are_rejected() {
    // Arrange
    let package = report_fixture();
    let first = report_identity(&package, 0);
    let second = report_identity(&package, 1);
    let id = EffectiveFeeGroupId::from_u64(6);
    let first_group = fee_group(id, vec![first.wtxid, second.wtxid]).expect("first group");
    let second_group =
        fee_group(id, vec![Wtxid::from_byte_array([0x86; 32])]).expect("second group");
    let members = vec![finally_present(first, id), finally_present(second, id)];

    // Act
    let result = PackageReport::try_new(
        &package,
        PackageStatus::Complete,
        members,
        vec![first_group, second_group],
    );

    // Assert
    assert!(result.is_err());
}

#[test]
fn cross_group_membership_is_rejected() {
    // Arrange
    let package = report_fixture();
    let first = report_identity(&package, 0);
    let second = report_identity(&package, 1);
    let first_id = EffectiveFeeGroupId::from_u64(60);
    let second_id = EffectiveFeeGroupId::from_u64(61);
    let first_group = fee_group(first_id, vec![first.wtxid]).expect("first group");
    let second_group = fee_group(second_id, vec![first.wtxid, second.wtxid]).expect("second group");
    let members = vec![
        finally_present(first, first_id),
        finally_present(second, second_id),
    ];

    // Act
    let result = PackageReport::try_new(
        &package,
        PackageStatus::Complete,
        members,
        vec![first_group, second_group],
    );

    // Assert
    assert!(result.is_err());
}

#[test]
fn unexpected_fee_group_without_eligible_member_is_rejected() {
    // Arrange
    let package = report_fixture();
    let first = report_identity(&package, 0);
    let second = report_identity(&package, 1);
    let id = EffectiveFeeGroupId::from_u64(62);
    let group = fee_group(id, vec![first.wtxid]).expect("valid group");
    let members = vec![
        PackageMemberResult::AlreadyPresent(super::ExistingMember { requested: first }),
        PackageMemberResult::SameTxidDifferentWitness(WitnessAlias {
            requested: second,
            existing_wtxid: Wtxid::from_byte_array([0x90; 32]),
        }),
    ];

    // Act
    let result = PackageReport::try_new(&package, PackageStatus::Complete, members, vec![group]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn missing_fee_group_for_eligible_member_is_rejected() {
    // Arrange
    let package = report_fixture();
    let first = report_identity(&package, 0);
    let second = report_identity(&package, 1);
    let id = EffectiveFeeGroupId::from_u64(63);
    let members = vec![
        finally_present(first, id),
        PackageMemberResult::AlreadyPresent(super::ExistingMember { requested: second }),
    ];

    // Act
    let result = PackageReport::try_new(&package, PackageStatus::Complete, members, vec![]);

    // Assert
    assert!(result.is_err());
}

#[test]
fn out_of_range_vsize_fee_group_is_rejected() {
    // Arrange
    let id = EffectiveFeeGroupId::from_u64(64);
    let virtual_size = TransactionVirtualSize::new(usize::MAX);

    // Act
    let result = EffectiveFeeGroup::try_new(
        id,
        vec![Wtxid::from_byte_array([0x91; 32])],
        Amount::from_sats(200).expect("base fee"),
        Amount::from_sats(300).expect("modified fee"),
        virtual_size,
        FeeRate::ZERO,
    );

    // Assert
    assert!(result.is_err());
}

#[test]
fn report_accessors_preserve_order_and_checked_fee_group_facts() {
    // Arrange
    let package = report_fixture();
    let first = report_identity(&package, 0);
    let second = report_identity(&package, 1);
    let id = EffectiveFeeGroupId::from_u64(7);
    let group = fee_group(id, vec![first.wtxid, second.wtxid]).expect("valid group");
    let members = vec![
        finally_present(first, id),
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::PackageFee {
            requested: second,
            effective_fee_group_id: id,
        }),
    ];

    // Act
    let report = PackageReport::try_new(&package, PackageStatus::Partial, members, vec![group])
        .expect("valid partial report");
    let stored_group = &report.effective_fee_groups()[0];

    // Assert
    assert_eq!(report.fingerprint(), package.fingerprint());
    assert_eq!(report.status(), &PackageStatus::Partial);
    assert_eq!(report.members().len(), 2);
    assert_eq!(report.members()[0].requested_identity(), first);
    assert_eq!(report.members()[1].requested_identity(), second);
    assert_eq!(stored_group.id(), id);
    assert_eq!(stored_group.id().as_u64(), 7);
    assert_eq!(stored_group.ordered_wtxids(), &[first.wtxid, second.wtxid]);
    assert_eq!(stored_group.base_fee_sats().to_sats(), 200);
    assert_eq!(stored_group.modified_fee_sats().to_sats(), 300);
    assert_eq!(
        stored_group.virtual_size(),
        TransactionVirtualSize::new(100)
    );
    assert_eq!(stored_group.effective_fee_rate().sats_per_kvb(), 3_000);
}

#[test]
fn witness_alias_and_post_trim_results_retain_typed_origins() {
    // Arrange
    let package = report_fixture();
    let first = report_identity(&package, 0);
    let second = report_identity(&package, 1);
    let existing_wtxid = Wtxid::from_byte_array([0x88; 32]);
    let id = EffectiveFeeGroupId::from_u64(8);
    let members = vec![
        PackageMemberResult::SameTxidDifferentWitness(WitnessAlias {
            requested: first,
            existing_wtxid,
        }),
        PackageMemberResult::PostTrimAbsent(super::PostTrimAbsence {
            requested: second,
            prior: PriorMemberSuccess::FinallyPresent {
                effective_fee_group_id: id,
            },
        }),
    ];
    let group = fee_group(id, vec![second.wtxid]).expect("post-trim group");

    // Act
    let report = PackageReport::try_new(&package, PackageStatus::Partial, members, vec![group])
        .expect("valid post-trim report");

    // Assert
    assert_eq!(report.members()[0].requested_identity(), first);
    assert_eq!(report.members()[1].requested_identity(), second);
}

#[test]
fn complete_and_failed_reports_accept_non_grouped_typed_results() {
    // Arrange
    let package = report_fixture();
    let first = report_identity(&package, 0);
    let second = report_identity(&package, 1);
    let complete_members = vec![
        PackageMemberResult::AlreadyPresent(super::ExistingMember { requested: first }),
        PackageMemberResult::SameTxidDifferentWitness(WitnessAlias {
            requested: second,
            existing_wtxid: Wtxid::from_byte_array([0x92; 32]),
        }),
    ];
    let failed_members = vec![
        PackageMemberResult::PostTrimAbsent(super::PostTrimAbsence {
            requested: first,
            prior: PriorMemberSuccess::AlreadyPresent,
        }),
        PackageMemberResult::PostTrimAbsent(super::PostTrimAbsence {
            requested: second,
            prior: PriorMemberSuccess::SameTxidDifferentWitness {
                existing_wtxid: Wtxid::from_byte_array([0x93; 32]),
            },
        }),
    ];

    // Act
    let complete =
        PackageReport::try_new(&package, PackageStatus::Complete, complete_members, vec![]);
    let failed = PackageReport::try_new(&package, PackageStatus::Failed, failed_members, vec![]);

    // Assert
    assert!(complete.is_ok());
    assert!(failed.is_ok());
}

#[test]
fn report_and_fee_group_errors_are_human_readable() {
    // Arrange
    let id = EffectiveFeeGroupId::from_u64(65);
    let wtxid = Wtxid::from_byte_array([0x94; 32]);
    let fee_errors = [
        super::EffectiveFeeGroupError::EmptyMembership,
        super::EffectiveFeeGroupError::DuplicateMembership { wtxid },
        super::EffectiveFeeGroupError::ZeroVirtualSize,
        super::EffectiveFeeGroupError::VirtualSizeOutOfRange {
            virtual_size: usize::MAX,
        },
        super::EffectiveFeeGroupError::InconsistentEffectiveRate {
            expected: FeeRate::ZERO,
            actual: FeeRate::from_sats_per_kvb(1),
        },
    ];
    let report_errors = [
        super::PackageReportError::MemberCountMismatch {
            expected: 2,
            actual: 1,
        },
        super::PackageReportError::IdentityMismatch { index: 1 },
        super::PackageReportError::StatusMismatch {
            supplied: PackageStatus::Complete,
            derived: PackageStatus::Failed,
        },
        super::PackageReportError::DuplicateFeeGroupId { id },
        super::PackageReportError::DuplicateCrossGroupMembership { wtxid },
        super::PackageReportError::UnexpectedFeeGroup { id },
        super::PackageReportError::FeeGroupMembersMismatch { id },
        super::PackageReportError::MissingFeeGroup { id },
    ];

    // Act
    let fee_messages: Vec<String> = fee_errors.iter().map(ToString::to_string).collect();
    let report_messages: Vec<String> = report_errors.iter().map(ToString::to_string).collect();

    // Assert
    assert!(fee_messages.iter().all(|message| !message.is_empty()));
    assert!(report_messages.iter().all(|message| !message.is_empty()));
}
