// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
};
use open_bitcoin_primitives::{Transaction, TransactionInput};

use super::{sample_chainstate_snapshot, script, spend_transaction};
use crate::{
    AdmissionContext, FinalMempoolMembership, Mempool, MempoolCapacity, MempoolError,
    PackageMemberResult, PackageStatus, PolicyConfig, SubmissionPackage, SubmitPackageCommand,
    WellFormedPackage,
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

fn admission_transaction(previous_txid: open_bitcoin_primitives::Txid) -> Transaction {
    spend_transaction(
        previous_txid,
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    )
}

fn submission(
    transactions: Vec<Transaction>,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
) -> SubmissionPackage {
    SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(transactions).expect("well-formed package"),
        snapshot,
    )
    .expect("submission refinement")
}

#[test]
fn singleton_preparation_preserves_state_and_exposes_canonical_facts() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = admission_transaction(coinbase_txids[0]);
    let expected_txid = transaction_txid(&transaction).expect("transaction txid");
    let expected_wtxid = transaction_wtxid(&transaction).expect("transaction wtxid");
    let mut mempool = Mempool::default();
    let before = mempool.complete_snapshot();

    // Act
    let prepared = mempool
        .prepare_transaction_with_context(
            transaction.clone(),
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("singleton preparation");

    // Assert
    assert_eq!(mempool.complete_snapshot(), before);
    assert_eq!(prepared.facts().delta().admitted.len(), 1);
    assert_eq!(
        prepared.facts().admitted_order(),
        &[crate::MempoolMemberIdentity {
            txid: expected_txid,
            wtxid: expected_wtxid,
        }]
    );
    assert_eq!(prepared.facts().final_present().len(), 1);
    assert_eq!(prepared.facts().final_present()[0].transaction, transaction);
    assert_eq!(
        prepared.facts().delta().final_membership[0].membership,
        FinalMempoolMembership::Present
    );
    assert!(prepared.facts().removed().is_empty());
    assert!(prepared.facts().teardown_order().is_empty());
    assert!(prepared.facts().maybe_admission_result().is_some());
    assert!(prepared.facts().maybe_package_report().is_none());
    assert!(
        prepared
            .package_report_for_facade()
            .expect_err("singleton capability is not a package")
            .to_string()
            .contains("package facade received a singleton transition")
    );
    assert!(format!("{prepared:?}").contains("PreparedMempoolTransition"));

    let validated = mempool
        .validate_prepared_mempool_transition(prepared)
        .expect("singleton validation");
    assert!(format!("{validated:?}").contains("ValidatedMempoolTransition"));
    let delta = mempool.apply_validated_mempool_transition(validated);
    assert_eq!(delta.admitted.len(), 1);
    assert!(mempool.entry(&expected_txid).is_some());
}

#[test]
fn replacement_preparation_exposes_every_removed_body() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let original = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let original_txid = transaction_txid(&original).expect("original txid");
    let descendant = spend_transaction(
        original_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let descendant_txid = transaction_txid(&descendant).expect("descendant txid");
    let replacement = spend_transaction(
        coinbase_txids[0],
        0,
        499_996_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();
    for transaction in [original.clone(), descendant.clone()] {
        mempool
            .accept_transaction_with_context(
                transaction,
                &snapshot,
                verify_flags(),
                consensus_params(),
                AdmissionContext::legacy_unknown(),
            )
            .expect("fixture admission");
    }
    let before = mempool.complete_snapshot();

    // Act
    let prepared = mempool
        .prepare_transaction_with_context(
            replacement,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("replacement preparation");

    // Assert
    let removed_bodies = prepared
        .facts()
        .removed()
        .iter()
        .map(|removed| (removed.removal.member.txid, removed.transaction.clone()))
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(removed_bodies.get(&original_txid), Some(&original));
    assert_eq!(removed_bodies.get(&descendant_txid), Some(&descendant));
    assert_eq!(
        prepared
            .facts()
            .teardown_order()
            .iter()
            .map(|member| member.txid)
            .collect::<Vec<_>>(),
        [descendant_txid, original_txid]
    );
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn package_patch_preparation_exposes_package_result() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = admission_transaction(coinbase_txids[0]);
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![transaction]).expect("well-formed package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mempool = Mempool::default();

    // Act
    let prepared = mempool
        .prepare_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("package preparation");

    // Assert
    assert!(prepared.facts().maybe_admission_result().is_none());
    assert!(prepared.facts().maybe_package_report().is_some());
    assert!(
        prepared
            .admission_result_for_facade()
            .expect_err("package capability is not a singleton")
            .to_string()
            .contains("singleton facade received a package transition")
    );
    assert_eq!(prepared.facts().final_present().len(), 1);
}

#[test]
fn package_preparation_propagates_revision_exhaustion() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = admission_transaction(coinbase_txids[0]);
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![transaction]).expect("well-formed package"),
        &snapshot,
    )
    .expect("submission refinement");
    let mempool = Mempool {
        revision: super::super::MempoolRevision(u64::MAX),
        ..Mempool::default()
    };

    // Act
    let error = mempool
        .prepare_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("revision exhaustion must propagate");

    // Assert
    assert_eq!(error, MempoolError::RevisionExhausted);
}

#[test]
fn atomic_commit_rejects_stale_preparation_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let first = admission_transaction(coinbase_txids[0]);
    let intervening = admission_transaction(coinbase_txids[1]);
    let mut mempool = Mempool::default();
    let prepared = mempool
        .prepare_transaction_with_context(
            first,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("singleton preparation");
    let newer = mempool
        .prepare_transaction_with_context(
            intervening,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("intervening preparation");
    let newer_delta = mempool
        .commit_prepared_mempool_transition(newer)
        .expect("intervening commit");
    let before_stale_commit = mempool.complete_snapshot();

    // Act
    let error = mempool
        .commit_prepared_mempool_transition(prepared)
        .expect_err("intervening mutation must stale the capability");

    // Assert
    assert_eq!(
        error,
        MempoolError::StalePreparedTransition {
            expected_revision: 0,
            actual_revision: 1,
        }
    );
    assert_eq!(newer_delta.admitted.len(), 1);
    assert_eq!(mempool.complete_snapshot(), before_stale_commit);
}

#[test]
fn legacy_validation_rejects_stale_preparation_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let first = admission_transaction(coinbase_txids[0]);
    let intervening = admission_transaction(coinbase_txids[1]);
    let mut mempool = Mempool::default();
    let prepared = mempool
        .prepare_transaction_with_context(
            first,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("singleton preparation");
    let newer = mempool
        .prepare_transaction_with_context(
            intervening,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("intervening preparation");
    mempool
        .commit_prepared_mempool_transition(newer)
        .expect("intervening commit");
    let before_stale_validation = mempool.complete_snapshot();

    // Act
    let error = mempool
        .validate_prepared_mempool_transition(prepared)
        .expect_err("intervening mutation must stale the capability");

    // Assert
    assert_eq!(
        error,
        MempoolError::StalePreparedTransition {
            expected_revision: 0,
            actual_revision: 1,
        }
    );
    assert_eq!(mempool.complete_snapshot(), before_stale_validation);
}

#[test]
fn atomic_commit_returns_exact_current_delta() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = admission_transaction(coinbase_txids[0]);
    let expected_txid = transaction_txid(&transaction).expect("transaction txid");
    let mut mempool = Mempool::default();
    let prepared = mempool
        .prepare_transaction_with_context(
            transaction,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("singleton preparation");
    let expected_delta = prepared.facts().delta().clone();

    // Act
    let delta = mempool
        .commit_prepared_mempool_transition(prepared)
        .expect("current commit");

    // Assert
    assert_eq!(delta, expected_delta);
    assert!(mempool.entry(&expected_txid).is_some());
}

#[test]
fn package_noop_atomic_commit_advances_no_revision() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = admission_transaction(coinbase_txids[0]);
    let mut mempool = Mempool::default();
    mempool
        .accept_transaction_with_context(
            transaction.clone(),
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("fixture admission");
    let submission = SubmissionPackage::try_from_package(
        WellFormedPackage::try_from(vec![transaction]).expect("well-formed package"),
        &snapshot,
    )
    .expect("submission refinement");
    let before = mempool.complete_snapshot();

    // Act
    let prepared = mempool
        .prepare_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("package preparation");
    assert!(prepared.facts().maybe_admission_result().is_none());
    assert!(prepared.facts().maybe_package_report().is_some());
    let expected_delta = prepared.facts().delta().clone();
    let delta = mempool
        .commit_prepared_mempool_transition(prepared)
        .expect("no-op commit");

    // Assert
    assert_eq!(delta, expected_delta);
    assert!(delta.is_empty());
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn compatibility_facades_prepare_and_consume_exactly_once() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let singleton = admission_transaction(coinbase_txids[0]);
    let package_member = admission_transaction(coinbase_txids[1]);
    let package = submission(vec![package_member], &snapshot);
    let mut mempool = Mempool::default();

    // Act
    super::super::prepared_lifecycle::reset_transition_counts_for_test();
    mempool
        .accept_transaction_with_context(
            singleton,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("singleton facade");
    let singleton_counts = super::super::prepared_lifecycle::transition_counts_for_test();
    super::super::prepared_lifecycle::reset_transition_counts_for_test();
    mempool
        .submit_package(
            SubmitPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("package facade");
    let package_counts = super::super::prepared_lifecycle::transition_counts_for_test();

    // Assert
    assert_eq!(singleton_counts, (1, 0, 1));
    assert_eq!(package_counts, (0, 1, 1));
}

#[test]
fn partial_package_preparation_exposes_only_topological_survivors() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = admission_transaction(coinbase_txids[0]);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let mut invalid_child = spend_transaction(
        parent_txid,
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    invalid_child.inputs[0].script_sig = script(&[0x01, 0x52]);
    let package = submission(vec![parent.clone(), invalid_child], &snapshot);
    let mempool = Mempool::default();

    // Act
    let prepared = mempool
        .prepare_package(
            SubmitPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("partial package preparation");

    // Assert
    assert_eq!(
        prepared
            .facts()
            .maybe_package_report()
            .map(|report| report.status()),
        Some(&PackageStatus::Partial)
    );
    assert!(matches!(
        prepared
            .facts()
            .maybe_package_report()
            .expect("package report")
            .members(),
        [
            PackageMemberResult::FinallyPresent(_),
            PackageMemberResult::HardRejected(_)
        ]
    ));
    assert_eq!(prepared.facts().admitted_order().len(), 1);
    assert_eq!(prepared.facts().admitted_order()[0].txid, parent_txid);
    assert_eq!(prepared.facts().final_present()[0].transaction, parent);
}

#[test]
fn post_trim_absence_never_enters_final_present_projection() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = admission_transaction(coinbase_txids[0]);
    let package = submission(vec![transaction], &snapshot);
    let mempool = Mempool::new(PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    });
    let before = mempool.complete_snapshot();

    // Act
    let prepared = mempool
        .prepare_package(
            SubmitPackageCommand {
                package,
                context: AdmissionContext::legacy_unknown(),
            },
            &snapshot,
            verify_flags(),
            consensus_params(),
        )
        .expect("post-trim preparation");

    // Assert
    assert!(matches!(
        prepared
            .facts()
            .maybe_package_report()
            .expect("package report")
            .members(),
        [PackageMemberResult::PostTrimAbsent(_)]
    ));
    assert!(prepared.facts().delta().admitted.is_empty());
    assert!(prepared.facts().admitted_order().is_empty());
    assert!(prepared.facts().final_present().is_empty());
    assert_eq!(mempool.complete_snapshot(), before);
}
