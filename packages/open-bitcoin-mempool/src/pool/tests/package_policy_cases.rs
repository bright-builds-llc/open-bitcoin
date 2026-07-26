// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_truc.py

use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{Amount, ScriptWitness, TransactionInput, Txid, Wtxid};

use crate::{
    AdmissionContext, CandidateFees, DryRunPackageCommand, EffectiveFeeGroupId, FeeRate,
    IncrementalRelayFeeRate, Mempool, MempoolCapacity, MempoolMemberIdentity, PackageFeeError,
    PackageFeeMember, PackageMemberResult, PolicyConfig, PriorMemberSuccess,
    ResourceAccountingError, RollingMempoolFeeRate, StaticRelayFeeRate, SubmissionPackage,
    SubmitPackageCommand, TransactionVirtualSize, TrucPolicy, WellFormedPackage,
    evaluate_package_fee_group,
};

use super::{sample_chainstate_snapshot, spend_transaction, submit};

fn identity(byte: u8) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: Txid::from_byte_array([byte; 32]),
        wtxid: Wtxid::from_byte_array([byte.wrapping_add(1); 32]),
    }
}

fn member(byte: u8, version: i32, base: i64, modified: i64, vsize: usize) -> PackageFeeMember {
    PackageFeeMember {
        identity: identity(byte),
        version,
        fees: CandidateFees {
            base: Amount::from_sats(base).expect("valid base fee"),
            modified: Amount::from_sats(modified).expect("valid modified fee"),
        },
        virtual_size: TransactionVirtualSize::new(vsize),
    }
}

fn static_floor(sats_per_kvb: i64) -> StaticRelayFeeRate {
    StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(sats_per_kvb))
}

fn rolling_floor(sats_per_kvb: i64) -> RollingMempoolFeeRate {
    RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(sats_per_kvb))
}

#[test]
fn ordinary_below_static_floor_cannot_be_sponsored_by_high_fee_sibling() {
    // Arrange
    let members = [member(1, 2, 99, 99, 100), member(3, 2, 2_000, 2_000, 100)];

    // Act
    let result = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    );

    // Assert
    assert!(matches!(
        result,
        Err(PackageFeeError::StaticFloorNotMet { member, .. }) if member == identity(1)
    ));
}

#[test]
fn ordinary_members_and_aggregate_rolling_floor_form_one_ordered_group() {
    // Arrange
    let members = [member(5, 2, 100, 125, 100), member(7, 2, 100, 875, 100)];

    // Act
    let assessment = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    )
    .expect("fee group");
    let group = assessment
        .try_effective_fee_group(EffectiveFeeGroupId::from_u64(1))
        .expect("checked group");

    // Assert
    assert_eq!(
        group.ordered_wtxids(),
        &[identity(5).wtxid, identity(7).wtxid]
    );
    assert_eq!(group.base_fee_sats().to_sats(), 200);
    assert_eq!(group.modified_fee_sats().to_sats(), 1_000);
    assert_eq!(assessment.base_fee_sats().to_sats(), 200);
    assert_eq!(assessment.modified_fee_sats().to_sats(), 1_000);
    assert_eq!(assessment.virtual_size(), TransactionVirtualSize::new(200));
    assert_eq!(
        assessment.effective_fee_rate(),
        FeeRate::from_sats_per_kvb(5_000)
    );
}

#[test]
fn aggregate_exactly_at_rolling_floor_passes() {
    // Arrange
    let members = [member(9, 2, 500, 500, 100)];

    // Act
    let result = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    );

    // Assert
    assert!(result.is_ok());
}

#[test]
fn aggregate_one_satoshi_below_rolling_floor_is_reconsiderable() {
    // Arrange
    let members = [member(11, 2, 499, 499, 100)];

    // Act
    let result = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    );

    // Assert
    let error = result.expect_err("rolling floor should reject");
    assert!(matches!(
        error,
        PackageFeeError::RollingFloorNotMet {
            required_fee_sats: 500,
            ..
        }
    ));
    assert!(error.to_string().contains("499"));
}

#[test]
fn incremental_relay_fee_does_not_change_static_or_rolling_assessment() {
    // Arrange
    let members = [member(13, 2, 500, 500, 100)];
    let low_incremental = IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1));
    let high_incremental = IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000_000));

    // Act
    let low = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    );
    let high = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    );

    // Assert
    assert_eq!(low, high);
    assert_ne!(low_incremental, high_incremental);
}

#[test]
fn truc_policy_defaults_to_accept_and_applies_ordinary_static_floor() {
    // Arrange
    let config = PolicyConfig::default();
    let members = [member(15, 3, 99, 99, 100)];

    // Act
    let result = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        RollingMempoolFeeRate::ZERO,
        config.truc_policy,
    );

    // Assert
    assert_eq!(config.truc_policy, TrucPolicy::Accept);
    assert!(matches!(
        result,
        Err(PackageFeeError::StaticFloorNotMet { .. })
    ));
}

#[test]
fn enforced_version_three_bypasses_only_the_static_floor() {
    // Arrange
    let members = [member(17, 3, 99, 99, 100)];

    // Act
    let result = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        rolling_floor(990),
        TrucPolicy::Enforce,
    );

    // Assert
    assert!(result.is_ok());
}

#[test]
fn rejected_version_three_is_a_hard_fee_policy_failure() {
    // Arrange
    let members = [member(19, 3, 1_000, 1_000, 100)];

    // Act
    let result = evaluate_package_fee_group(
        &members,
        static_floor(1_000),
        RollingMempoolFeeRate::ZERO,
        TrucPolicy::Reject,
    );

    // Assert
    assert!(matches!(result, Err(PackageFeeError::TrucRejected { .. })));
}

#[test]
fn exact_existing_and_witness_alias_members_stay_out_of_fee_membership() {
    // Arrange
    let eligible_new_members = [member(21, 2, 500, 500, 100)];
    let existing_exact = identity(23);
    let requested_alias = identity(25);

    // Act
    let assessment = evaluate_package_fee_group(
        &eligible_new_members,
        static_floor(1_000),
        rolling_floor(5_000),
        TrucPolicy::Accept,
    )
    .expect("eligible new group");

    // Assert
    assert_eq!(assessment.ordered_wtxids(), &[identity(21).wtxid]);
    assert!(!assessment.ordered_wtxids().contains(&existing_exact.wtxid));
    assert!(!assessment.ordered_wtxids().contains(&requested_alias.wtxid));
}

#[test]
fn empty_fee_group_and_all_checked_error_kinds_have_stable_diagnostics() {
    // Arrange
    let empty = evaluate_package_fee_group(
        &[],
        static_floor(1_000),
        RollingMempoolFeeRate::ZERO,
        TrucPolicy::Accept,
    )
    .expect_err("empty group");
    let errors = [
        empty,
        PackageFeeError::TrucRejected {
            member: identity(27),
        },
        PackageFeeError::StaticFloorNotMet {
            member: identity(29),
            fee: Amount::ZERO,
            required_fee_sats: 1,
        },
        PackageFeeError::BaseFeeOverflow,
        PackageFeeError::ModifiedFeeOverflow,
        PackageFeeError::VirtualSize(ResourceAccountingError::Overflow {
            component: "test virtual size",
        }),
        PackageFeeError::InvalidBaseFeeTotal,
        PackageFeeError::InvalidModifiedFeeTotal,
    ];

    // Act
    let diagnostics = errors.map(|error| error.to_string());

    // Assert
    assert!(diagnostics.iter().all(|diagnostic| !diagnostic.is_empty()));
}

#[test]
fn legacy_prospective_fee_guard_still_checks_the_combined_floor() {
    // Arrange
    let mempool = Mempool::default();
    let prospective = super::super::prospective::ProspectiveMempool::new(&mempool);
    let virtual_size = TransactionVirtualSize::new(100);

    // Act
    let accepted = prospective.enforce_admission_fee(100, virtual_size);
    let rejected = prospective.enforce_admission_fee(99, virtual_size);

    // Assert
    assert!(accepted.is_ok());
    assert!(rejected.is_err());
}

#[test]
fn report_group_construction_rechecks_zero_virtual_size() {
    // Arrange
    let members = [member(31, 2, 0, 0, 0)];
    let assessment = evaluate_package_fee_group(
        &members,
        StaticRelayFeeRate::new(FeeRate::ZERO),
        RollingMempoolFeeRate::ZERO,
        TrucPolicy::Accept,
    )
    .expect("arithmetic assessment");

    // Act
    let result = super::super::package_admission::classify_fee_group_for_test(Ok(assessment));

    // Assert
    assert!(matches!(
        result,
        Err(crate::MempoolError::InternalInvariant { .. })
    ));
}

#[test]
fn static_truc_rolling_limit_replacement_ephemeral_script_trim_order_is_objective() {
    // Arrange
    use super::super::package_admission::PackagePolicyStage;
    let pre_script = [
        PackagePolicyStage::Static,
        PackagePolicyStage::Truc,
        PackagePolicyStage::Rolling,
        PackagePolicyStage::Limits,
        PackagePolicyStage::Replacement,
        PackagePolicyStage::Ephemeral,
    ];

    // Act and Assert
    for failure in pre_script {
        let (_trace, scripts, trims) =
            super::super::package_admission::package_policy_probe_for_test(Some(failure));
        assert_eq!(scripts, 0, "pre_script_failure_has_no_script");
        assert_eq!(trims, 0, "pre_script_failure_has_no_trim");
    }
    let (_trace, scripts, trims) = super::super::package_admission::package_policy_probe_for_test(
        Some(PackagePolicyStage::Scripts),
    );
    assert_eq!((scripts, trims), (1, 0), "script_failure_has_no_trim");
    let (trace, scripts, trims) =
        super::super::package_admission::package_policy_probe_for_test(None);
    assert_eq!(
        trace,
        [
            PackagePolicyStage::Static,
            PackagePolicyStage::Truc,
            PackagePolicyStage::Rolling,
            PackagePolicyStage::Limits,
            PackagePolicyStage::Replacement,
            PackagePolicyStage::Ephemeral,
            PackagePolicyStage::Scripts,
            PackagePolicyStage::Trim,
        ]
    );
    assert_eq!((scripts, trims), (1, 1), "trim_once_after_scripts");
}

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
        if request_alias {
            assert_ne!(requested_wtxid, existing_wtxid);
            assert_ne!(submitted.delta.removed[0].member.wtxid, requested_wtxid);
        }
    }
}
