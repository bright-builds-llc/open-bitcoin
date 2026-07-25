// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid};
use open_bitcoin_primitives::TransactionInput;

use super::{sample_chainstate_snapshot, spend_transaction};
use crate::{
    AdmissionContext, Mempool, MempoolAcceptanceTime, MempoolEntryMetadata, MempoolOrigin,
    MempoolRemovalCause, MempoolRemovalRole, PolicyConfig, PolicyTime, RelayIntent,
    recompute_resource_ledger,
};

const TEST_EXPIRY_HOURS: u64 = 1;
const SECONDS_PER_HOUR: i64 = 3_600;

fn submit_with_acceptance(
    mempool: &mut Mempool,
    snapshot: &open_bitcoin_chainstate::ChainstateSnapshot,
    transaction: open_bitcoin_primitives::Transaction,
    accepted_at: MempoolAcceptanceTime,
) {
    let metadata =
        MempoolEntryMetadata::new(accepted_at, MempoolOrigin::Local, RelayIntent::NotRequested);
    mempool
        .accept_transaction_transition_with_context(
            transaction,
            snapshot,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            AdmissionContext::new(metadata),
        )
        .expect("transition outcome");
}

fn short_expiry_mempool() -> Mempool {
    Mempool::new(PolicyConfig {
        mempool_expiry_hours: TEST_EXPIRY_HOURS,
        ..PolicyConfig::default()
    })
}

fn assert_ledger_matches_oracle(mempool: &Mempool) {
    let oracle = recompute_resource_ledger(mempool.entries(), &mempool.spent_outpoints)
        .expect("oracle recomputation");
    assert_eq!(mempool.resource_ledger(), oracle);
}

#[test]
fn expiry_removes_aged_entry_and_descendants() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let parent = spend_transaction(
        coinbase_txids[0],
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
    let child_txid = transaction_txid(&child).expect("child txid");
    let accepted_at = PolicyTime::new(1_000);
    let mut mempool = short_expiry_mempool();
    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        parent,
        MempoolAcceptanceTime::Known(accepted_at),
    );
    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        child,
        MempoolAcceptanceTime::Known(PolicyTime::new(1_100)),
    );
    let now = PolicyTime::new(
        accepted_at.unix_seconds() + TEST_EXPIRY_HOURS as i64 * SECONDS_PER_HOUR + 1,
    );

    // Act
    let delta = mempool.expire(now).expect("expire sweep");

    // Assert
    assert!(mempool.entry(&parent_txid).is_none());
    assert!(mempool.entry(&child_txid).is_none());
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == parent_txid
            && removal.cause == MempoolRemovalCause::Expiry
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == child_txid
            && removal.cause == MempoolRemovalCause::Expiry
            && removal.role == MempoolRemovalRole::Descendant
    }));
    assert_ledger_matches_oracle(&mempool);
}

#[test]
fn expiry_retains_fresh_known_entries() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let accepted_at = PolicyTime::new(5_000);
    let mut mempool = short_expiry_mempool();
    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        transaction,
        MempoolAcceptanceTime::Known(accepted_at),
    );
    let now = PolicyTime::new(
        accepted_at.unix_seconds() + TEST_EXPIRY_HOURS as i64 * SECONDS_PER_HOUR - 1,
    );

    // Act
    let delta = mempool.expire(now).expect("expire sweep");

    // Assert
    assert!(mempool.entry(&txid).is_some());
    assert!(delta.removed.is_empty());
    assert_ledger_matches_oracle(&mempool);
}

#[test]
fn expiry_skips_legacy_unknown_without_inventing_time() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let mut mempool = short_expiry_mempool();
    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        transaction,
        MempoolAcceptanceTime::LegacyUnknown,
    );
    let now = PolicyTime::new(TEST_EXPIRY_HOURS as i64 * SECONDS_PER_HOUR * 100);

    // Act
    let delta = mempool.expire(now).expect("expire sweep");

    // Assert
    assert!(mempool.entry(&txid).is_some());
    assert!(delta.removed.is_empty());
    assert_eq!(
        mempool.entry(&txid).expect("retained").metadata.accepted_at,
        MempoolAcceptanceTime::LegacyUnknown
    );
    assert_ledger_matches_oracle(&mempool);
}

#[test]
fn expiry_emits_mempool_removal_cause_expiry() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let aged = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let aged_txid = transaction_txid(&aged).expect("aged txid");
    let fresh = spend_transaction(
        coinbase_txids[1],
        0,
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let fresh_txid = transaction_txid(&fresh).expect("fresh txid");
    let aged_at = PolicyTime::new(100);
    let mut mempool = short_expiry_mempool();
    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        aged,
        MempoolAcceptanceTime::Known(aged_at),
    );
    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        fresh,
        MempoolAcceptanceTime::Known(PolicyTime::new(
            aged_at.unix_seconds() + TEST_EXPIRY_HOURS as i64 * SECONDS_PER_HOUR,
        )),
    );
    let now =
        PolicyTime::new(aged_at.unix_seconds() + TEST_EXPIRY_HOURS as i64 * SECONDS_PER_HOUR + 1);

    // Act
    let delta = mempool.expire(now).expect("expire sweep");

    // Assert
    assert!(mempool.entry(&aged_txid).is_none());
    assert!(mempool.entry(&fresh_txid).is_some());
    assert!(!delta.removed.is_empty());
    assert!(
        delta
            .removed
            .iter()
            .all(|removal| removal.cause == MempoolRemovalCause::Expiry)
    );
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == aged_txid && removal.role == MempoolRemovalRole::Direct
    }));
    assert_ledger_matches_oracle(&mempool);
}

#[test]
fn expiry_clamps_overflowing_expiry_hours() {
    // Arrange — u64::MAX cannot convert to i64; sweep must still succeed without inventing times.
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(1);
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let mut mempool = Mempool::new(PolicyConfig {
        mempool_expiry_hours: u64::MAX,
        ..PolicyConfig::default()
    });
    submit_with_acceptance(
        &mut mempool,
        &snapshot,
        transaction,
        MempoolAcceptanceTime::Known(PolicyTime::new(1)),
    );

    // Act
    let delta = mempool
        .expire(PolicyTime::new(i64::MAX))
        .expect("overflowing expiry config must not panic");

    // Assert — with clamped max duration, a Known(1) entry is still within the window.
    assert!(mempool.entry(&txid).is_some());
    assert!(delta.removed.is_empty());
}
