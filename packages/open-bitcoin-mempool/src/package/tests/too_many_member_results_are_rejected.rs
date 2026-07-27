use super::*;

#[test]
fn too_many_member_results_are_rejected() {
    // Arrange
    let package = report_fixture();
    let requested = report_identity(&package, 0);
    let failure = || {
        PackageMemberResult::HardRejected(HardMemberFailure::Policy {
            requested,
            category: MempoolRejectionCategory::InternalInvariant,
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
            category: MempoolRejectionCategory::InternalInvariant,
            reason: "second first".to_string(),
        }),
        PackageMemberResult::HardRejected(HardMemberFailure::Policy {
            requested: first,
            category: MempoolRejectionCategory::InternalInvariant,
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
            category: MempoolRejectionCategory::InternalInvariant,
            reason: "first".to_string(),
        }),
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::MissingInputs {
            requested: report_identity(&package, 1),
            missing_parents: vec![Txid::from_byte_array([9; 32])],
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

#[test]
fn package_replacement_failure_preserves_requested_identity() {
    // Arrange
    let requested = MempoolMemberIdentity {
        txid: Txid::from_byte_array([0x95; 32]),
        wtxid: Wtxid::from_byte_array([0x96; 32]),
    };
    let results = [
        PackageMemberResult::HardRejected(HardMemberFailure::PackageReplacement {
            requested,
            reason: "replacement policy".to_string(),
        }),
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::PackageReplacement {
            requested,
        }),
    ];

    // Act
    let actual = results.map(|result| result.requested_identity());

    // Assert
    assert_eq!(actual, [requested, requested]);
}

#[test]
fn truc_policy_failure_preserves_requested_identity() {
    // Arrange
    let requested = MempoolMemberIdentity {
        txid: Txid::from_byte_array([0x97; 32]),
        wtxid: Wtxid::from_byte_array([0x98; 32]),
    };
    let result = PackageMemberResult::HardRejected(HardMemberFailure::TrucPolicy {
        requested,
        reason: "TRUC topology".to_string(),
    });

    // Act / Assert
    assert_eq!(result.requested_identity(), requested);
}
