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

use std::collections::HashMap;

use open_bitcoin_consensus::transaction_wtxid;
use open_bitcoin_primitives::{Amount, TransactionInput, Txid};

use super::trim_to_size;
use crate::fee::rolling::RollingFeeState;
use crate::{
    MempoolCapacity, MempoolEntry, MempoolError, MempoolRemovalRole, PolicyConfig,
    TransactionVirtualSize,
};

use super::super::oracle::recompute_state;
use super::super::tests::spend_transaction;

fn entry(txid: Txid, previous_txid: Txid, virtual_size: usize, fee_sats: i64) -> MempoolEntry {
    let transaction = spend_transaction(
        previous_txid,
        0,
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let wtxid = transaction_wtxid(&transaction).expect("fixture transaction serializes");
    MempoolEntry::new(
        transaction,
        txid,
        wtxid,
        Amount::from_sats(fee_sats).expect("fixture fee is in range"),
        TransactionVirtualSize::new(virtual_size),
        100,
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    )
}

#[test]
fn pressure_trim_removes_a_victim_and_its_descendant() {
    // Arrange
    let parent_txid = Txid::from_byte_array([1; 32]);
    let child_txid = Txid::from_byte_array([2; 32]);
    let parent = entry(parent_txid, Txid::from_byte_array([9; 32]), 1, 1_000);
    let child = entry(child_txid, parent_txid, 1, 1_000);
    let state = recompute_state(HashMap::from([(parent_txid, parent), (child_txid, child)]))
        .expect("fixture state recomputes");
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    };
    let mut rolling = RollingFeeState::new();

    // Act
    let (trimmed, evicted) =
        trim_to_size(state, &config, &mut rolling).expect("pressure trim succeeds");

    // Assert
    assert!(trimmed.entries.is_empty());
    assert_eq!(evicted.len(), 2);
    assert!(
        evicted
            .values()
            .any(|role| *role == MempoolRemovalRole::Direct)
    );
    assert!(
        evicted
            .values()
            .any(|role| *role == MempoolRemovalRole::Descendant)
    );
    assert!(rolling.rolling_fee_rate().fee_rate().sats_per_kvb() > 0);
}

#[test]
fn inconsistent_empty_state_stops_pressure_trim_without_a_victim() {
    // Arrange
    let txid = Txid::from_byte_array([3; 32]);
    let seed = entry(txid, Txid::from_byte_array([9; 32]), 1, 1_000);
    let mut state =
        recompute_state(HashMap::from([(txid, seed)])).expect("fixture state recomputes");
    state.entries.clear();
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    };
    let mut rolling = RollingFeeState::new();

    // Act
    let (trimmed, evicted) =
        trim_to_size(state, &config, &mut rolling).expect("empty graph stops trimming");

    // Assert
    assert!(trimmed.entries.is_empty());
    assert!(evicted.is_empty());
}

#[test]
fn pressure_trim_propagates_recompute_overflow() {
    // Arrange
    let victim_txid = Txid::from_byte_array([1; 32]);
    let parent_txid = Txid::from_byte_array([2; 32]);
    let child_txid = Txid::from_byte_array([3; 32]);
    let victim = entry(victim_txid, Txid::from_byte_array([8; 32]), 1, 1);
    let parent = entry(parent_txid, Txid::from_byte_array([9; 32]), 1, 10_000);
    let child = entry(child_txid, parent_txid, 1, 10_000);
    let mut state = recompute_state(HashMap::from([
        (victim_txid, victim),
        (parent_txid, parent),
        (child_txid, child),
    ]))
    .expect("fixture state recomputes");
    let parent_entry = state
        .entries
        .get_mut(&parent_txid)
        .expect("parent fixture entry");
    parent_entry.virtual_size = TransactionVirtualSize::new(usize::MAX);
    parent_entry.fee = Amount::ZERO;
    state
        .entries
        .get_mut(&child_txid)
        .expect("child fixture entry")
        .fee = Amount::ZERO;
    let config = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(0),
        ..PolicyConfig::default()
    };
    let mut rolling = RollingFeeState::new();

    // Act
    let error =
        trim_to_size(state, &config, &mut rolling).expect_err("recompute overflow must propagate");

    // Assert
    assert!(matches!(error, MempoolError::InternalInvariant { .. }));
}
