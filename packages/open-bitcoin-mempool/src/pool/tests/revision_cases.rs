// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_primitives::{BlockHash, TransactionInput};

use super::{build_block, sample_chainstate_snapshot, script, spend_transaction};
use crate::{
    AdmissionContext, BlockLifecycleContext, FeeRate, Mempool, MempoolAcceptanceTime,
    MempoolEntryMetadata, MempoolError, MempoolOrigin, MempoolResourceLedger, PolicyTime,
    RelayIntent, RollingMempoolFeeRate,
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

#[test]
fn candidate_preparation_finishes_before_script_checks_run() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    transaction.inputs[0].script_sig = script(&[0x01, 0x52]);
    let mempool = Mempool::default();

    // Act
    let prepared = super::super::candidate::prepare_candidate(
        &mempool,
        transaction,
        &snapshot,
        consensus_params(),
        AdmissionContext::legacy_unknown(),
    )
    .expect("non-script preparation should succeed");
    let script_error = super::super::candidate::check_candidate_scripts(&prepared, verify_flags())
        .expect_err("invalid redeem script must fail at the script seam");

    // Assert
    assert_eq!(prepared.fees.base, prepared.fees.modified);
    assert!(prepared.fees.base.to_sats() > 0);
    assert!(matches!(script_error, MempoolError::Validation { .. }));
}

#[test]
fn pre_script_policy_failure_wins_over_an_invalid_script() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_999,
        TransactionInput::SEQUENCE_FINAL,
    );
    transaction.inputs[0].script_sig = script(&[0x01, 0x52]);
    let mut mempool = Mempool::default();

    // Act
    let error = mempool
        .accept_transaction_with_context(
            transaction,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect_err("fee policy must reject before scripts");

    // Assert
    assert!(matches!(error, MempoolError::RelayFeeTooLow { .. }));
    assert!(mempool.entries().is_empty());
    assert_eq!(mempool.resource_ledger(), MempoolResourceLedger::ZERO);
    assert_eq!(
        mempool.rolling_mempool_fee_rate(),
        RollingMempoolFeeRate::ZERO
    );
}

#[test]
fn script_failure_after_policy_leaves_authoritative_state_unchanged() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    transaction.inputs[0].script_sig = script(&[0x01, 0x52]);
    let mut mempool = Mempool::default();

    // Act
    let error = mempool
        .accept_transaction_with_context(
            transaction,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect_err("scripts must reject after policy");

    // Assert
    assert!(matches!(error, MempoolError::Validation { .. }));
    assert!(mempool.entries().is_empty());
    assert_eq!(mempool.resource_ledger(), MempoolResourceLedger::ZERO);
    assert_eq!(
        mempool.rolling_mempool_fee_rate(),
        RollingMempoolFeeRate::ZERO
    );
}

#[test]
fn candidate_preparation_supports_block_time_locktime_cutoff() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let params = ConsensusParams {
        enforce_bip113_median_time_past: false,
        ..consensus_params()
    };

    // Act
    let prepared = super::super::candidate::prepare_candidate(
        &Mempool::default(),
        transaction,
        &snapshot,
        params,
        AdmissionContext::legacy_unknown(),
    )
    .expect("block-time cutoff candidate should prepare");

    // Assert
    assert!(prepared.fees.base.to_sats() > 0);
}

#[test]
fn candidate_preparation_rejects_non_final_locktime_before_scripts() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL,
    );
    transaction.lock_time = 3;

    // Act
    let error = super::super::candidate::prepare_candidate(
        &Mempool::default(),
        transaction,
        &snapshot,
        consensus_params(),
        AdmissionContext::legacy_unknown(),
    )
    .expect_err("future-height locktime should fail");

    // Assert
    assert!(matches!(error, MempoolError::Validation { .. }));
    assert!(error.to_string().contains("non-final"));
}

#[test]
fn candidate_preparation_rejects_unsatisfied_sequence_locks_before_scripts() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(coinbase_txids[0], 0, 499_999_000, 10);

    // Act
    let error = super::super::candidate::prepare_candidate(
        &Mempool::default(),
        transaction,
        &snapshot,
        consensus_params(),
        AdmissionContext::legacy_unknown(),
    )
    .expect_err("relative-height lock should fail");

    // Assert
    assert!(matches!(error, MempoolError::Validation { .. }));
    assert!(error.to_string().contains("non-BIP68-final"));
}

#[test]
fn membership_change_makes_prepared_patch_stale_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let first = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let intervening = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = Mempool::default();
    let prepared = super::super::candidate::prepare_candidate(
        &mempool,
        first,
        &snapshot,
        consensus_params(),
        AdmissionContext::legacy_unknown(),
    )
    .expect("candidate preparation");
    let (patch, _result) = super::super::admission::prepare_admission_patch(&mempool, &prepared)
        .expect("patch preparation");
    let mut mempool = mempool;
    mempool
        .accept_transaction_with_context(
            intervening,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("intervening admission");
    let before = mempool.complete_snapshot();

    // Act
    let error = mempool
        .apply_prepared(patch)
        .expect_err("membership change must stale the patch");

    // Assert
    assert_eq!(
        error,
        MempoolError::StalePreparedTransition {
            expected_revision: 0,
            actual_revision: 1,
        }
    );
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn rolling_change_makes_prepared_patch_stale_without_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mempool = Mempool::default();
    let prepared = super::super::candidate::prepare_candidate(
        &mempool,
        transaction,
        &snapshot,
        consensus_params(),
        AdmissionContext::legacy_unknown(),
    )
    .expect("candidate preparation");
    let (patch, _result) = super::super::admission::prepare_admission_patch(&mempool, &prepared)
        .expect("patch preparation");
    let mut mempool = mempool;
    mempool
        .track_package_removed(FeeRate::from_sats_per_kvb(5_000))
        .expect("rolling bump");
    let before = mempool.complete_snapshot();

    // Act
    let error = mempool
        .apply_prepared(patch)
        .expect_err("rolling-only change must stale the patch");

    // Assert
    assert!(matches!(
        error,
        MempoolError::StalePreparedTransition {
            expected_revision: 0,
            actual_revision: 1,
        }
    ));
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn revision_overflow_fails_before_admission_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool {
        revision: super::super::MempoolRevision(u64::MAX),
        ..Mempool::default()
    };
    let before = mempool.complete_snapshot();

    // Act
    let error = mempool
        .accept_transaction_with_context(
            transaction,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect_err("revision overflow must fail");

    // Assert
    assert_eq!(error, MempoolError::RevisionExhausted);
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn no_op_expiry_and_rolling_updates_do_not_advance_revision() {
    // Arrange
    let mut mempool = Mempool::default();
    let initial_revision = mempool.revision;

    // Act
    let delta = mempool
        .expire(PolicyTime::new(10))
        .expect("empty expiry succeeds");
    mempool
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::ZERO)
        .expect("equal rolling floor succeeds");
    mempool
        .track_package_removed(FeeRate::ZERO)
        .expect("non-increasing package rate succeeds");
    let materialized = mempool
        .materialize_rolling_fee_rate(PolicyTime::new(20))
        .expect("zero rolling state materializes");

    // Assert
    assert!(delta.is_empty());
    assert_eq!(materialized, RollingMempoolFeeRate::ZERO);
    assert_eq!(mempool.revision, initial_revision);
}

#[test]
fn successful_admission_and_duplicate_no_op_have_exact_revision_behavior() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();

    // Act
    mempool
        .accept_transaction_with_context(
            transaction.clone(),
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("admission succeeds");
    let after_admission = mempool.revision;
    let duplicate = mempool.accept_transaction_with_context(
        transaction,
        &snapshot,
        verify_flags(),
        consensus_params(),
        AdmissionContext::legacy_unknown(),
    );

    // Assert
    assert_eq!(after_admission.0, 1);
    assert!(matches!(
        duplicate,
        Err(MempoolError::DuplicateTransaction { .. })
    ));
    assert_eq!(mempool.revision, after_admission);
}

#[test]
fn expiry_removal_advances_revision_exactly_once() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let metadata = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::new(1)),
        MempoolOrigin::Local,
        RelayIntent::NotRequested,
    );
    let mut mempool = Mempool::default();
    mempool
        .accept_transaction_with_context(
            transaction,
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::new(metadata),
        )
        .expect("admission succeeds");
    let before_expiry = mempool.revision;

    // Act
    let delta = mempool
        .expire(PolicyTime::new(2_000_000))
        .expect("expiry succeeds");

    // Assert
    assert!(!delta.removed.is_empty());
    assert_eq!(mempool.revision.0, before_expiry.0 + 1);
}

#[test]
fn rolling_mutations_and_block_gate_each_advance_revision_once() {
    // Arrange
    let mut mempool = Mempool::default();
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 1, 500_000_000);

    // Act
    mempool
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            4_000,
        )))
        .expect("setter succeeds");
    let after_setter = mempool.revision;
    mempool
        .track_package_removed(FeeRate::from_sats_per_kvb(5_000))
        .expect("package bump succeeds");
    let after_bump = mempool.revision;
    mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(100), 1),
        )
        .expect("block gate succeeds");
    let after_gate = mempool.revision;
    mempool
        .materialize_rolling_fee_rate(PolicyTime::new(100_000))
        .expect("decay succeeds");

    // Assert
    assert_eq!(after_setter.0, 1);
    assert_eq!(after_bump.0, 2);
    assert_eq!(after_gate.0, 3);
    assert_eq!(mempool.revision.0, 4);
}

#[test]
fn connected_block_removal_and_gate_share_one_revision_completion() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();
    mempool
        .accept_transaction_with_context(
            transaction.clone(),
            &snapshot,
            verify_flags(),
            consensus_params(),
            AdmissionContext::legacy_unknown(),
        )
        .expect("admission succeeds");
    let mut block = build_block(BlockHash::from_byte_array([0_u8; 32]), 3, 500_000_000);
    block.transactions.push(transaction);
    let before_block = mempool.revision;

    // Act
    let delta = mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(70), 3),
        )
        .expect("block cleanup succeeds");

    // Assert
    assert!(!delta.removed.is_empty());
    assert_eq!(mempool.revision.0, before_block.0 + 1);
}

#[test]
fn unchanged_empty_block_gate_is_a_revision_no_op() {
    // Arrange
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 1, 500_000_000);
    let mut mempool = Mempool::default();
    let before = mempool.complete_snapshot();

    // Act
    let delta = mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(0), 1),
        )
        .expect("unchanged block gate succeeds");

    // Assert
    assert!(delta.is_empty());
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn revision_overflow_fails_before_empty_block_gate_mutation() {
    // Arrange
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 1, 500_000_000);
    let mut mempool = Mempool {
        revision: super::super::MempoolRevision(u64::MAX),
        ..Mempool::default()
    };
    let before = mempool.complete_snapshot();

    // Act
    let error = mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(1), 1),
        )
        .expect_err("revision exhaustion must fail before opening the block gate");

    // Assert
    assert_eq!(error, MempoolError::RevisionExhausted);
    assert_eq!(mempool.complete_snapshot(), before);
}

#[test]
fn lifecycle_identity_conflict_fails_before_block_mutation() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let first = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let second = spend_transaction(
        coinbase_txids[1],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::default();
    for transaction in [&first, &second] {
        mempool
            .accept_transaction_with_context(
                transaction.clone(),
                &snapshot,
                verify_flags(),
                consensus_params(),
                AdmissionContext::legacy_unknown(),
            )
            .expect("fixture admission succeeds");
    }
    let first_txid =
        open_bitcoin_consensus::transaction_txid(&first).expect("fixture transaction serializes");
    let second_txid =
        open_bitcoin_consensus::transaction_txid(&second).expect("fixture transaction serializes");
    let shared_wtxid = mempool
        .entry(&first_txid)
        .expect("first fixture entry")
        .wtxid;
    mempool
        .entries
        .get_mut(&second_txid)
        .expect("second fixture entry")
        .wtxid = shared_wtxid;
    let mut block = build_block(BlockHash::from_byte_array([0_u8; 32]), 3, 500_000_000);
    block.transactions.extend([first, second]);
    let before = mempool.complete_snapshot();

    // Act
    let error = mempool
        .remove_for_connected_block_transition(
            &block,
            BlockLifecycleContext::new(PolicyTime::new(10), 3),
        )
        .expect_err("conflicting identities must fail delta preparation");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert_eq!(mempool.complete_snapshot(), before);
}
