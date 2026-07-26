// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::HashMap;

use open_bitcoin_consensus::transaction_wtxid;
use open_bitcoin_primitives::{Amount, TransactionInput, Txid};

use super::{install_aggregate_stats, validate_limits};
use crate::{MempoolEntry, PolicyConfig, ResourceAccountingError, TransactionVirtualSize};

use super::super::tests::spend_transaction;

fn entry(txid: Txid, previous_txid: Txid, virtual_size: usize) -> MempoolEntry {
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
        Amount::from_sats(1_000).expect("fixture fee is in range"),
        TransactionVirtualSize::new(virtual_size),
        100,
        0,
        crate::MempoolEntryMetadata::legacy_unknown(),
    )
}

#[test]
fn valid_candidate_passes_oracle_limits() {
    // Arrange
    let candidate_txid = Txid::from_byte_array([1; 32]);
    let candidate = entry(candidate_txid, Txid::from_byte_array([9; 32]), 1);
    let entries = HashMap::from([(candidate_txid, candidate)]);

    // Act
    let result = validate_limits(&entries, &PolicyConfig::default(), candidate_txid);

    // Assert
    assert_eq!(result, Ok(()));
}

#[test]
fn aggregate_install_propagates_ancestor_overflow() {
    // Arrange
    let child_txid = Txid::from_byte_array([1; 32]);
    let parent_txid = Txid::from_byte_array([2; 32]);
    let mut child = entry(child_txid, parent_txid, usize::MAX);
    child.parents.insert(parent_txid);
    let mut parent = entry(parent_txid, Txid::from_byte_array([9; 32]), 1);
    parent.children.insert(child_txid);
    let mut entries = HashMap::from([(child_txid, child), (parent_txid, parent)]);

    // Act
    let error =
        install_aggregate_stats(&mut entries).expect_err("ancestor aggregate must overflow");

    // Assert
    assert!(matches!(error, ResourceAccountingError::Overflow { .. }));
}

#[test]
fn aggregate_install_propagates_descendant_overflow() {
    // Arrange
    let parent_txid = Txid::from_byte_array([1; 32]);
    let child_txid = Txid::from_byte_array([2; 32]);
    let mut parent = entry(parent_txid, Txid::from_byte_array([9; 32]), usize::MAX);
    parent.children.insert(child_txid);
    let mut child = entry(child_txid, parent_txid, 1);
    child.parents.insert(parent_txid);
    let mut entries = HashMap::from([(parent_txid, parent), (child_txid, child)]);

    // Act
    let error =
        install_aggregate_stats(&mut entries).expect_err("descendant aggregate must overflow");

    // Assert
    assert!(matches!(error, ResourceAccountingError::Overflow { .. }));
}
