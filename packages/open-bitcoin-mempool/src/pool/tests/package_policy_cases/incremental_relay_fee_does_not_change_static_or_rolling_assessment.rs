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
fn truc_enforce_uses_pinned_10_000_and_1_000_vbyte_topology_limits() {
    // Arrange / Act
    use crate::policy::truc::{
        MAX_TRUC_ANCESTOR_COUNT, MAX_TRUC_CHILD_VIRTUAL_SIZE, MAX_TRUC_DESCENDANT_COUNT,
        MAX_TRUC_VIRTUAL_SIZE,
    };

    // Assert
    assert_eq!(MAX_TRUC_VIRTUAL_SIZE, 10_000);
    assert_eq!(MAX_TRUC_CHILD_VIRTUAL_SIZE, 1_000);
    assert_eq!(MAX_TRUC_ANCESTOR_COUNT, 2);
    assert_eq!(MAX_TRUC_DESCENDANT_COUNT, 2);
}

#[test]
fn truc_child_replacement_and_sibling_eviction_precede_replacement_staging() {
    // Arrange
    use super::super::package_admission::PackagePolicyStage;

    // Act
    let (trace, scripts, trims) =
        super::super::package_admission::package_policy_probe_for_test(None);
    let truc = trace
        .iter()
        .position(|stage| *stage == PackagePolicyStage::Truc)
        .expect("TRUC stage");
    let replacement = trace
        .iter()
        .position(|stage| *stage == PackagePolicyStage::Replacement)
        .expect("replacement stage");

    // Assert
    assert!(truc < replacement);
    assert_eq!((scripts, trims), (1, 1));
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
fn ephemeral_rejection_runs_zero_scripts_and_rolls_back_dry_run_and_submit() {
    // The pure policy matrix also covers nonzero base, nonzero modified fee, multiple dust
    // outputs, and multiple parents before this package-engine integration probe.
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = ephemeral_parent(coinbase_txids[0], 500_000_000);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = ephemeral_child(parent_txid, false, 499_995_000);
    let package =
        WellFormedPackage::try_from(vec![parent.clone(), child.clone()]).expect("package");
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![parent, child]).expect("submission package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mut mempool = Mempool::new(PolicyConfig {
        truc_policy: TrucPolicy::Enforce,
        ..PolicyConfig::default()
    });
    mempool
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            1_000,
        )))
        .expect("rolling fee injection");
    let before = mempool.complete_snapshot();
    let verify_flags = ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY;
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    };

    // Act
    super::super::package_admission::reset_script_check_count_for_test();
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
        .expect("ephemeral dry-run rejection report");
    assert_eq!(
        super::super::package_admission::script_check_count_for_test(),
        0
    );
    super::super::package_admission::reset_script_check_count_for_test();
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
        .expect("ephemeral submission rejection report");

    // Assert
    assert_eq!(dry_run.report, submitted.report);
    assert_eq!(
        super::super::package_admission::script_check_count_for_test(),
        0
    );
    assert!(submitted.delta.is_empty());
    assert_eq!(mempool.complete_snapshot(), before);
    assert!(submitted.report.members().iter().all(|member| matches!(
        member,
        PackageMemberResult::HardRejected(crate::HardMemberFailure::EphemeralPolicy {
            reason,
            ..
        }) if reason.contains("did not spend all parent ephemeral dust")
    )));
}

#[test]
fn ephemeral_success_calls_scripts_only_after_complete_dust_spending() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = ephemeral_parent(coinbase_txids[0], 500_000_000);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = ephemeral_child(parent_txid, true, 499_995_000);
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![parent, child]).expect("package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mut mempool = Mempool::new(PolicyConfig {
        truc_policy: TrucPolicy::Enforce,
        ..PolicyConfig::default()
    });
    mempool
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            1_000,
        )))
        .expect("rolling fee injection");

    // Act
    super::super::package_admission::reset_script_check_count_for_test();
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
        .expect("complete ephemeral sweep");

    // Assert
    assert_eq!(
        super::super::package_admission::script_check_count_for_test(),
        2,
        "{:?}",
        submitted.report.members()
    );
    assert_eq!(
        submitted.delta.admitted.len(),
        2,
        "{:?}",
        submitted.report.members()
    );
    assert!(
        submitted
            .report
            .members()
            .iter()
            .all(|member| matches!(member, PackageMemberResult::FinallyPresent(_)))
    );
}

#[test]
fn in_mempool_dust_parent_requires_complete_spend_before_ephemeral_script_stage() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = ephemeral_parent(coinbase_txids[0], 500_000_000);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let parent_submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![parent]).expect("parent package"),
        &snapshot,
    )
    .expect("parent submission refinement");
    let child = ephemeral_child(parent_txid, false, 499_995_000);
    let child_submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![child]).expect("child package"),
        &snapshot,
    )
    .expect("child submission refinement");
    let mut mempool = Mempool::new(PolicyConfig {
        truc_policy: TrucPolicy::Enforce,
        ..PolicyConfig::default()
    });
    let verify_flags = ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY;
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    };
    let parent_result = mempool
        .submit_package(
            SubmitPackageCommand {
                package: parent_submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags,
            consensus_params,
        )
        .expect("zero-fee dusty parent");
    assert_eq!(parent_result.delta.admitted.len(), 1);
    let before_child = mempool.complete_snapshot();

    // Act
    super::super::package_admission::reset_script_check_count_for_test();
    let child_result = mempool
        .submit_package(
            SubmitPackageCommand {
                package: child_submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags,
            consensus_params,
        )
        .expect("in-mempool ephemeral rejection report");

    // Assert
    assert_eq!(
        super::super::package_admission::script_check_count_for_test(),
        0
    );
    assert!(child_result.delta.is_empty());
    assert_eq!(mempool.complete_snapshot(), before_child);
    assert!(matches!(
        child_result.report.members(),
        [PackageMemberResult::HardRejected(
            crate::HardMemberFailure::EphemeralPolicy { .. }
        )]
    ));
}
