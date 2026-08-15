// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use std::cmp::Ordering;
use std::collections::BTreeSet;

use open_bitcoin_core::codec::{TransactionEncoding, encode_transaction};
use open_bitcoin_core::primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    Txid,
};
use open_bitcoin_mempool::{MempoolAcceptanceTime, PolicyTime};

use super::*;
use crate::storage::mempool_snapshot::{CapturedMempoolGeneration, MempoolSnapshotError};
use crate::storage::snapshot_codec::{
    assert_mempool_snapshot_representable, persisted_mempool_input_limits,
    persisted_record_count_is_representable,
};
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};

const EXACT_PER_TRANSACTION_BYTES: usize = 4_194_304;
const ONE_OVER_PER_TRANSACTION_BYTES: usize = 4_194_305;
const ONE_OVER_AGGREGATE_TRANSACTION_BYTES: usize = 67_108_865;
const EXACT_RECORD_COUNT: usize = 220_096;
const ONE_OVER_RECORD_COUNT: usize = 220_097;
const PERSISTED_ENCODED_BYTE_CEILING: usize = 268_435_456;

fn transaction_with_canonical_len(seed: u8, target_len: usize) -> Transaction {
    let mut pad_len = target_len.saturating_sub(64).max(1);
    loop {
        let transaction = Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([seed; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::from_bytes(Vec::new()).expect("empty script"),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::new(vec![vec![0x01; pad_len]]),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(10_000).expect("valid amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            }],
            lock_time: 0,
        };
        let encoded = encode_transaction(&transaction, TransactionEncoding::WithWitness)
            .expect("canonical encode");
        match encoded.len().cmp(&target_len) {
            Ordering::Equal => return transaction,
            Ordering::Less => {
                pad_len = pad_len
                    .checked_add(target_len - encoded.len())
                    .expect("pad growth");
            }
            Ordering::Greater => {
                pad_len = pad_len
                    .checked_sub(encoded.len() - target_len)
                    .expect("pad shrink");
            }
        }
    }
}

fn current_snapshot_from_transactions(transactions: Vec<Transaction>) -> MempoolSnapshot {
    let records = transactions
        .into_iter()
        .map(|transaction| {
            MempoolSnapshotRecord::try_from_canonical(
                transaction,
                MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90)),
            )
            .expect("canonical record")
        })
        .collect();
    MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(42),
        PolicyTime::from_unix_seconds(120),
        records,
        BTreeSet::new(),
    )
    .expect("current snapshot")
}

fn assert_resource_bound(error: StorageError) {
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ref detail,
            ..
        } if detail == "mempool snapshot exceeds a resource bound"
    ));
}

#[test]
fn representability_accepts_exact_per_transaction_bytes() {
    // Arrange
    let snapshot = current_snapshot_from_transactions(vec![transaction_with_canonical_len(
        1,
        EXACT_PER_TRANSACTION_BYTES,
    )]);
    let expected = persisted_mempool_input_limits().expect("persisted input limits");

    // Act
    let limits = assert_mempool_snapshot_representable(&snapshot).expect("exact per-tx");

    // Assert
    assert_eq!(limits, expected);
    assert_eq!(snapshot.records.len(), 1);
}

#[test]
fn representability_rejects_one_over_per_transaction_bytes() {
    // Arrange
    let snapshot = current_snapshot_from_transactions(vec![transaction_with_canonical_len(
        2,
        ONE_OVER_PER_TRANSACTION_BYTES,
    )]);
    let record_count = snapshot.records.len();

    // Act
    let predicate = assert_mempool_snapshot_representable(&snapshot);
    let encoded = encode_mempool_snapshot(&snapshot);

    // Assert
    assert_eq!(predicate, Err(MempoolSnapshotError::ResourceBoundExceeded));
    assert_resource_bound(encoded.expect_err("one-over per-tx encode"));
    assert_eq!(snapshot.records.len(), record_count);
}

#[test]
fn representability_rejects_one_over_aggregate_transaction_bytes() {
    // Arrange
    let first = 33_554_432;
    let second = ONE_OVER_AGGREGATE_TRANSACTION_BYTES
        .checked_sub(first)
        .expect("aggregate split");
    let snapshot = current_snapshot_from_transactions(vec![
        transaction_with_canonical_len(3, first),
        transaction_with_canonical_len(4, second),
    ]);
    let record_count = snapshot.records.len();

    // Act
    let predicate = assert_mempool_snapshot_representable(&snapshot);
    let encoded = encode_mempool_snapshot(&snapshot);

    // Assert
    assert_eq!(predicate, Err(MempoolSnapshotError::ResourceBoundExceeded));
    assert_resource_bound(encoded.expect_err("one-over aggregate encode"));
    assert_eq!(snapshot.records.len(), record_count);
}

#[test]
fn representability_rejects_one_over_record_count() {
    // Arrange
    let limits = persisted_mempool_input_limits().expect("persisted input limits");

    // Act
    let exact = persisted_record_count_is_representable(EXACT_RECORD_COUNT, limits.max_records);
    let one_over =
        persisted_record_count_is_representable(ONE_OVER_RECORD_COUNT, limits.max_records);

    // Assert
    assert_eq!(limits.max_records, EXACT_RECORD_COUNT);
    assert!(exact);
    assert!(!one_over);
}

#[test]
fn encode_mempool_snapshot_uses_persisted_encoded_byte_ceiling() {
    // Arrange
    let snapshot = mempool_snapshot();
    let limits = assert_mempool_snapshot_representable(&snapshot).expect("small snapshot");

    // Act
    let bytes = encode_mempool_snapshot(&snapshot).expect("encode under persisted ceiling");

    // Assert
    assert_eq!(limits.max_encoded_bytes, PERSISTED_ENCODED_BYTE_CEILING);
    assert!(bytes.len() <= limits.max_encoded_bytes);
}
