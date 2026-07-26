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

use open_bitcoin_consensus::transaction_txid;
use open_bitcoin_primitives::{TransactionInput, Txid};

use super::trim_prospective_to_capacity;
use crate::pool::prospective::ProspectiveMempool;
use crate::pool::tests::{sample_chainstate_snapshot, spend_transaction, submit};
use crate::{
    AccountedMempoolMemory, Mempool, MempoolCapacity, MempoolError, MempoolRemovalRole,
    MempoolResourceLedger, PolicyConfig, TransactionVirtualSize,
};

#[test]
fn pressure_trim_removes_a_victim_and_its_descendant() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();
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
    submit(&mut mempool, &snapshot, parent).expect("parent admission");
    submit(&mut mempool, &snapshot, child).expect("child admission");
    let mut prospective = ProspectiveMempool::new(&mempool);
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    };

    // Act
    let evicted =
        trim_prospective_to_capacity(&mut prospective, &config).expect("pressure trim succeeds");

    // Assert
    assert!(prospective.visible_txids().is_empty());
    assert_eq!(evicted.len(), 2);
    assert!(
        evicted
            .iter()
            .any(|fact| fact.role == MempoolRemovalRole::Direct)
    );
    assert!(
        evicted
            .iter()
            .any(|fact| fact.role == MempoolRemovalRole::Descendant)
    );
    assert!(
        prospective
            .rolling_fee_state()
            .rolling_fee_rate()
            .fee_rate()
            .sats_per_kvb()
            > 0
    );
}

#[test]
fn empty_view_needs_no_pressure_victim() {
    // Arrange
    let mempool = Mempool::default();
    let mut prospective = ProspectiveMempool::new(&mempool);
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    };

    // Act
    let evicted =
        trim_prospective_to_capacity(&mut prospective, &config).expect("empty trim succeeds");

    // Assert
    assert!(evicted.is_empty());
    assert_eq!(prospective.trim_invocations_for_test(), 1);
}

#[test]
fn over_capacity_view_without_members_fails_closed() {
    // Arrange
    let mempool = Mempool {
        resource_ledger: MempoolResourceLedger::new(
            TransactionVirtualSize::new(1),
            AccountedMempoolMemory::new(1),
        ),
        ..Mempool::default()
    };
    let mut prospective = ProspectiveMempool::new(&mempool);
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    };

    // Act
    let error = trim_prospective_to_capacity(&mut prospective, &config)
        .expect_err("missing victim must fail closed");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert_eq!(prospective.trim_invocations_for_test(), 0);
}

#[test]
fn pressure_trim_failure_discards_working_rolling_and_membership() {
    // Arrange
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();
    let transaction = spend_transaction(
        coinbase_txids[0],
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    submit(&mut mempool, &snapshot, transaction).expect("admission");
    mempool
        .entries
        .get_mut(&txid)
        .expect("entry")
        .children
        .insert(Txid::from_byte_array([99; 32]));
    let before = mempool.complete_snapshot();
    let mut prospective = ProspectiveMempool::new(&mempool);
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    };

    // Act
    let error = trim_prospective_to_capacity(&mut prospective, &config)
        .expect_err("missing descendant must fail");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
    assert_eq!(mempool.complete_snapshot(), before);
    assert!(prospective.maybe_entry(&txid).is_some());
    assert_eq!(
        prospective
            .rolling_fee_state()
            .rolling_fee_rate()
            .fee_rate()
            .sats_per_kvb(),
        0
    );
    assert_eq!(prospective.trim_invocations_for_test(), 0);
}
