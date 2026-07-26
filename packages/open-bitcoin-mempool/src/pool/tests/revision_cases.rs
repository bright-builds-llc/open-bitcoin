// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_primitives::TransactionInput;

use super::{sample_chainstate_snapshot, script, spend_transaction};
use crate::{
    AdmissionContext, Mempool, MempoolError, MempoolResourceLedger, RollingMempoolFeeRate,
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
