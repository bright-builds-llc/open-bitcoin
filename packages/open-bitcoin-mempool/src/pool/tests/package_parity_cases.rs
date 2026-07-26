// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/test/txvalidation_tests.cpp
// - packages/bitcoin-knots/test/functional/mempool_package_rbf.py
// - packages/bitcoin-knots/test/functional/mempool_truc.py
// - packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py

//! Integrated Phase 132 package-policy closure against pinned Knots behavior.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, TransactionInputContext, transaction_txid,
    transaction_wtxid,
};
use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    Txid, Wtxid,
};

use super::{sample_chainstate_snapshot, script, spend_transaction, submit};
use crate::policy::replacement::{
    MempoolView, PackageReplacementError, evaluate_limited_package_replacement,
};
use crate::policy::truc::{TrucPolicyError, evaluate_truc_package};
use crate::pool::candidate::{PreparedCandidate, prepare_candidate};
use crate::pool::package_admission::{
    PackagePolicyStage, evaluate_package_for_test, package_policy_probe_for_test,
    package_trim_count_for_test, reset_package_trim_count_for_test,
};
use crate::pool::pressure::trim_prospective_to_capacity;
use crate::pool::prospective::ProspectiveMempool;
use crate::{
    AdmissionContext, CandidateFees, DryRunPackageCommand, EffectiveFeeGroup,
    EffectiveFeeGroupError, EffectiveFeeGroupId, EphemeralPolicy, FeeRate, HardMemberFailure,
    IncrementalRelayFeeRate, Mempool, MempoolCapacity, MempoolEntry, MempoolEntryMetadata,
    MempoolError, MempoolLifecycleDelta, MempoolMemberIdentity, PackageMemberResult, PackageReport,
    PackageReportError, PackageShapeError, PackageStatus, PolicyConfig, StaticRelayFeeRate,
    SubmissionPackage, SubmissionPackageKind, SubmitPackageCommand, TransactionVirtualSize,
    TrucPolicy, WellFormedPackage, MAX_PACKAGE_COUNT, MAX_PACKAGE_WEIGHT,
    recompute_resource_ledger, validate_standard_transaction,
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

fn empty_snapshot() -> ChainstateSnapshot {
    ChainstateSnapshot::new(Vec::new(), BTreeMap::new().into_iter().collect(), BTreeMap::new().into_iter().collect())
}

fn identity(byte: u8) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: Txid::from_byte_array([byte; 32]),
        wtxid: Wtxid::from_byte_array([byte.wrapping_add(100); 32]),
    }
}

fn policy_transaction(id: u8, version: i32, inputs: Vec<OutPoint>) -> Transaction {
    Transaction {
        version,
        inputs: inputs
            .into_iter()
            .map(|previous_output| TransactionInput {
                previous_output,
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
                witness: ScriptWitness::default(),
            })
            .collect(),
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(i64::from(id) + 1).expect("valid output"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: u32::from(id),
    }
}

fn policy_entry(
    id: u8,
    version: i32,
    inputs: Vec<OutPoint>,
    fee_sats: i64,
    virtual_size: usize,
    descendant_count: usize,
) -> MempoolEntry {
    let fee = Amount::from_sats(fee_sats).expect("valid fee");
    let mut entry = MempoolEntry::new(
        policy_transaction(id, version, inputs),
        Txid::from_byte_array([id; 32]),
        Wtxid::from_byte_array([id.wrapping_add(100); 32]),
        fee,
        TransactionVirtualSize::new(virtual_size),
        virtual_size.saturating_mul(4),
        0,
        MempoolEntryMetadata::legacy_unknown(),
    );
    entry.descendant_stats.count = descendant_count;
    entry
}

fn policy_candidate(
    id: u8,
    version: i32,
    inputs: Vec<OutPoint>,
    fee_sats: i64,
    virtual_size: usize,
) -> PreparedCandidate {
    let fee = Amount::from_sats(fee_sats).expect("valid candidate fee");
    PreparedCandidate::for_policy_test(
        policy_entry(id, version, inputs, fee_sats, virtual_size, 1),
        CandidateFees {
            base: fee,
            modified: fee,
        },
    )
}

#[derive(Default)]
struct PolicyView {
    entries: BTreeMap<Txid, MempoolEntry>,
    spenders: BTreeMap<OutPoint, Txid>,
    descendant_calls: Cell<usize>,
}

impl MempoolView for PolicyView {
    fn maybe_entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        self.entries.get(txid)
    }

    fn maybe_spender(&self, outpoint: &OutPoint) -> Option<Txid> {
        self.spenders.get(outpoint).copied()
    }

    fn collect_descendants(&self, txid: Txid) -> BTreeSet<Txid> {
        self.descendant_calls
            .set(self.descendant_calls.get().saturating_add(1));
        let mut descendants = BTreeSet::new();
        let mut pending = self
            .entries
            .get(&txid)
            .map(|entry| entry.children.iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        while let Some(descendant) = pending.pop() {
            if !descendants.insert(descendant) {
                continue;
            }
            if let Some(entry) = self.entries.get(&descendant) {
                pending.extend(entry.children.iter().copied());
            }
        }
        descendants
    }
}

fn checked_fee_group(
    id: EffectiveFeeGroupId,
    ordered_wtxids: Vec<Wtxid>,
) -> Result<EffectiveFeeGroup, EffectiveFeeGroupError> {
    let virtual_size = TransactionVirtualSize::new(100);
    EffectiveFeeGroup::try_new(
        id,
        ordered_wtxids,
        Amount::from_sats(200).expect("base fee"),
        Amount::from_sats(300).expect("modified fee"),
        virtual_size,
        FeeRate::from_fee_sats_and_vbytes(300, virtual_size),
    )
}

fn output_policy_result(
    script_pubkey: ScriptBuf,
    value_sats: i64,
    permissions: EphemeralPolicy,
) -> Result<(), MempoolError> {
    let transaction = Transaction {
        version: 3,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([0x31; 32]),
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value_sats).expect("valid output value"),
            script_pubkey,
        }],
        lock_time: 0,
    };
    let input_context = TransactionInputContext {
        spent_output: open_bitcoin_consensus::SpentOutput {
            value: Amount::from_sats(10_000).expect("valid spent value"),
            script_pubkey: script(&[
                0xa9, 0x14, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                0x87,
            ]),
            is_coinbase: false,
        },
        created_height: 1,
        created_median_time_past: 1,
    };
    validate_standard_transaction(
        &transaction,
        &[input_context],
        &PolicyConfig {
            ephemeral_policy: permissions,
            ..PolicyConfig::default()
        },
        100,
        0,
    )
}

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
            .collect(),
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
    assert_eq!(empty_fee_group, Err(EffectiveFeeGroupError::EmptyMembership));
    assert!(matches!(
        duplicate_fee_group,
        Err(EffectiveFeeGroupError::DuplicateMembership { .. })
    ));
    assert!(matches!(
        inconsistent_rate,
        Err(EffectiveFeeGroupError::InconsistentEffectiveRate { .. })
    ));
}

#[test]
fn dry_run_submit_valid_parent_invalid_child_partial_acceptance_and_lifecycle_match() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        confirmed_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let mut invalid_child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    invalid_child.inputs[0].script_sig = script(&[0x01, 0x52]);
    let package =
        WellFormedPackage::try_from(vec![parent.clone(), invalid_child.clone()]).expect("package");
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![parent, invalid_child]).expect("submission package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mut mempool = Mempool::default();
    let before = mempool.complete_snapshot();

    // Act
    let dry_run = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("dry run");
    assert_eq!(mempool.complete_snapshot(), before);
    let submitted = mempool
        .submit_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("submit report");

    // Assert
    assert_eq!(dry_run.report, submitted.report);
    assert_eq!(submitted.report.status(), &PackageStatus::Partial);
    assert_eq!(submitted.delta.admitted.len(), 1);
    assert_eq!(submitted.delta.admitted[0].txid, parent_txid);
    assert!(mempool.entry(&parent_txid).is_some());
    assert!(matches!(
        submitted.report.members(),
        [
            PackageMemberResult::FinallyPresent(_),
            PackageMemberResult::HardRejected(_)
        ]
    ));
}

#[test]
fn stale_revision_sparse_patch_rejects_before_apply_without_mutation() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(3);
    let package_transaction = spend_transaction(
        confirmed_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let package =
        WellFormedPackage::try_from(vec![package_transaction.clone()]).expect("package");
    let mut mempool = Mempool::default();
    let evaluation = evaluate_package_for_test(
        &mempool,
        &package,
        AdmissionContext::legacy_unknown(),
        &snapshot,
        verify_flags(),
        consensus_params(),
    )
    .expect("prepared evaluation");
    let patch = evaluation.patch.expect("sparse patch");
    submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            confirmed_txids[1],
            0,
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("revision-changing admission");
    let before_apply = mempool.complete_snapshot();

    // Act
    let stale = mempool.apply_prepared(patch);

    // Assert
    assert!(matches!(stale, Err(MempoolError::StalePreparedTransition { .. })));
    assert_eq!(mempool.complete_snapshot(), before_apply);
    assert!(mempool.entry(&transaction_txid(&package_transaction).expect("txid")).is_none());
}

#[test]
fn static_truc_rolling_limits_replacement_ephemeral_late_script_order_is_exact() {
    // Arrange
    let expected = vec![
        PackagePolicyStage::Static,
        PackagePolicyStage::Truc,
        PackagePolicyStage::Rolling,
        PackagePolicyStage::Limits,
        PackagePolicyStage::Replacement,
        PackagePolicyStage::Ephemeral,
        PackagePolicyStage::Scripts,
        PackagePolicyStage::Trim,
    ];

    // Act
    let complete = package_policy_probe_for_test(None);

    // Assert
    assert_eq!(complete, (expected.clone(), 1, 1));
    for pre_script_failure in expected.into_iter().take(6) {
        let (trace, script_calls, trim_calls) =
            package_policy_probe_for_test(Some(pre_script_failure));
        assert_eq!(trace.last(), Some(&pre_script_failure));
        assert_eq!(script_calls, 0, "pre-script failure invoked checker");
        assert_eq!(trim_calls, 0);
    }
}

#[test]
fn overlap_over_100_conservative_limited_rbf_count_precedes_union() {
    // Arrange
    let first_spent = OutPoint {
        txid: Txid::from_byte_array([1; 32]),
        vout: 0,
    };
    let second_spent = OutPoint {
        txid: Txid::from_byte_array([2; 32]),
        vout: 0,
    };
    let mut view = PolicyView::default();
    for (id, spent) in [(20, first_spent.clone()), (21, second_spent.clone())] {
        let conflict = policy_entry(id, 2, vec![spent.clone()], 100, 100, 51);
        view.spenders.insert(spent, conflict.txid);
        view.entries.insert(conflict.txid, conflict);
    }
    let parent = policy_candidate(10, 2, vec![first_spent, second_spent], 300, 100);
    let child = policy_candidate(
        11,
        2,
        vec![OutPoint {
            txid: parent.entry.txid,
            vout: 0,
        }],
        400,
        100,
    );

    // Act
    let result = evaluate_limited_package_replacement(
        &view,
        &[parent, child],
        IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000)),
    );

    // Assert
    assert_eq!(
        result,
        Err(PackageReplacementError::TooManyPotentialReplacements {
            count: 102,
            limit: 100,
        })
    );
    assert_eq!(view.descendant_calls.get(), 0);
}

#[test]
fn truc_child_replacement_and_sibling_eviction_use_pre_replacement_facts() {
    // Arrange
    let parent_txid = Txid::from_byte_array([1; 32]);
    let old_child_txid = Txid::from_byte_array([2; 32]);
    let mut view = PolicyView::default();
    let mut parent = policy_entry(1, 3, vec![], 1_000, 100, 2);
    let mut old_child = policy_entry(
        2,
        3,
        vec![OutPoint {
            txid: parent_txid,
            vout: 0,
        }],
        1_000,
        100,
        1,
    );
    parent.children.insert(old_child_txid);
    old_child.parents.insert(parent_txid);
    old_child.ancestor_stats.count = 2;
    view.entries.insert(parent_txid, parent);
    view.entries.insert(old_child_txid, old_child);
    let replacement = [policy_candidate(
        3,
        3,
        vec![OutPoint {
            txid: parent_txid,
            vout: 0,
        }],
        1_000,
        100,
    )];

    // Act
    let child_replacement = evaluate_truc_package(
        &view,
        &replacement,
        TrucPolicy::Enforce,
        &BTreeSet::from([old_child_txid]),
    );
    let sibling_eviction =
        evaluate_truc_package(&view, &replacement, TrucPolicy::Enforce, &BTreeSet::new());
    view.entries
        .get_mut(&old_child_txid)
        .expect("old child")
        .descendant_stats
        .count = 2;
    let ineligible_sibling =
        evaluate_truc_package(&view, &replacement, TrucPolicy::Enforce, &BTreeSet::new());

    // Assert
    assert_eq!(child_replacement, Ok(None));
    assert_eq!(
        sibling_eviction
            .expect("eligible sibling eviction")
            .map(|intent| intent.sibling),
        Some(old_child_txid)
    );
    assert!(matches!(
        ineligible_sibling,
        Err(TrucPolicyError::IneligibleSibling { .. })
    ));
}

#[test]
fn pay_to_anchor_anchor_send_dust_and_ephemeral_predicates_are_exact() {
    // Arrange
    let pay_to_anchor = script(&[0x51, 0x02, 0x4e, 0x73]);
    let ordinary = script(&[
        0xa9, 0x14, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0x87,
    ]);

    // Act / Assert
    let defaults = PolicyConfig::default();
    assert_eq!(
        defaults.ephemeral_policy,
        EphemeralPolicy {
            anchor: true,
            send: false,
            dust: false,
        }
    );
    for anchor in [false, true] {
        for send in [false, true] {
            for dust in [false, true] {
                let permissions = EphemeralPolicy { anchor, send, dust };
                assert_eq!(
                    output_policy_result(pay_to_anchor.clone(), 0, permissions).is_ok(),
                    anchor
                );
                assert_eq!(
                    output_policy_result(pay_to_anchor.clone(), 1, permissions).is_ok(),
                    anchor && dust
                );
                assert_eq!(
                    output_policy_result(ordinary.clone(), 0, permissions).is_ok(),
                    send
                );
                assert_eq!(
                    output_policy_result(ordinary.clone(), 1, permissions).is_ok(),
                    send && dust
                );
            }
        }
    }
}

#[test]
fn max_bound_generated_sparse_overlay_recompute_has_zero_clone_and_one_trim() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(MAX_PACKAGE_COUNT as u32 + 1);
    let mempool = Mempool::default();
    let mut prospective = ProspectiveMempool::new(&mempool);

    // Act
    for (index, confirmed_txid) in confirmed_txids
        .into_iter()
        .take(MAX_PACKAGE_COUNT)
        .enumerate()
    {
        let transaction = spend_transaction(
            confirmed_txid,
            0,
            499_999_000 - index as i64,
            TransactionInput::SEQUENCE_FINAL,
        );
        let prepared = prepare_candidate(
            &mempool,
            transaction,
            &snapshot,
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("generated candidate");
        prospective
            .stage_candidate(prepared)
            .expect("sparse generated addition");
    }
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(
            prospective.accounted_memory().as_usize().saturating_sub(1),
        ),
        ..PolicyConfig::default()
    };
    let removed =
        trim_prospective_to_capacity(&mut prospective, &config).expect("one final trim");
    let recomputed = prospective
        .materialize_for_test()
        .expect("test-only full recompute");
    let ledger = recompute_resource_ledger(&recomputed.entries, &recomputed.spent_outpoints)
        .expect("resource recompute");
    let patch = prospective
        .prepare_patch(MempoolLifecycleDelta::empty())
        .expect("revision-bound patch");

    // Assert
    assert_eq!(removed.len(), 1);
    assert_eq!(recomputed.entries.len(), MAX_PACKAGE_COUNT - 1);
    assert_eq!(recomputed.resource_ledger, ledger);
    assert_eq!(prospective.full_clone_count_for_test(), 0, "zero clone");
    assert_eq!(
        prospective.full_recompute_count_for_test(),
        0,
        "zero recompute"
    );
    assert_eq!(prospective.trim_invocations_for_test(), 1, "one trim");
    let mut applied = mempool.clone();
    applied.apply_prepared(patch).expect("sparse patch apply");
    assert_eq!(applied.entries(), &recomputed.entries);
    assert_eq!(applied.resource_ledger(), recomputed.resource_ledger);
}

#[test]
fn max_bound_package_path_runs_one_final_trim_and_keeps_ordered_report() {
    // Arrange
    let (snapshot, confirmed_txids) = sample_chainstate_snapshot(MAX_PACKAGE_COUNT as u32 + 1);
    let transactions = confirmed_txids
        .into_iter()
        .take(MAX_PACKAGE_COUNT)
        .enumerate()
        .map(|(index, txid)| {
            spend_transaction(
                txid,
                0,
                499_999_000 - index as i64,
                TransactionInput::SEQUENCE_FINAL,
            )
        })
        .collect::<Vec<_>>();
    let package = WellFormedPackage::try_from(transactions).expect("max-bound package");
    let mempool = Mempool::new(PolicyConfig {
        static_relay_fee_rate: StaticRelayFeeRate::new(FeeRate::ZERO),
        mempool_capacity: MempoolCapacity::new(usize::MAX),
        ..PolicyConfig::default()
    });

    // Act
    reset_package_trim_count_for_test();
    let result = mempool
        .dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("max-bound dry run");

    // Assert
    assert_eq!(result.report.members().len(), MAX_PACKAGE_COUNT);
    assert_eq!(package_trim_count_for_test(), 1);
    assert!(result
        .report
        .members()
        .iter()
        .all(|member| matches!(member, PackageMemberResult::FinallyPresent(_))));
    assert!(mempool.entries().is_empty());
}
