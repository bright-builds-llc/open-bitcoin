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
use crate::storage::mempool_snapshot::CapturedMempoolGeneration;
use crate::storage::snapshot_codec::encode_mempool_snapshot;
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};

const EXACT_PER_TRANSACTION_BYTES: usize = 4_194_304;
const ONE_OVER_PER_TRANSACTION_BYTES: usize = 4_194_305;

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

fn current_snapshot_with_canonical_len(seed: u8, target_len: usize) -> MempoolSnapshot {
    let record = MempoolSnapshotRecord::try_from_canonical(
        transaction_with_canonical_len(seed, target_len),
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90)),
    )
    .expect("canonical record");
    MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(42),
        PolicyTime::from_unix_seconds(120),
        vec![record],
        BTreeSet::new(),
    )
    .expect("current snapshot")
}

#[test]
fn exact_per_transaction_boundary_prepares_encodes_syncs_and_reopens() {
    // Arrange
    let path = temp_store_path("exact-per-tx-boundary");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let exact = current_snapshot_with_canonical_len(1, EXACT_PER_TRANSACTION_BYTES);
    let handle = empty_network_handle();

    // Act
    store
        .save_mempool_snapshot(&exact, PersistMode::Sync)
        .expect("sync exact representable snapshot");
    let prepared = handle
        .prepare_mempool_snapshot_write(
            PolicyTime::from_unix_seconds(130),
            CheckpointTrigger::Periodic,
        )
        .expect("representable capture should reserve");
    let receipt = FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
        &handle,
        prepared,
        |_| encode_mempool_snapshot(&exact),
        |_, mode| {
            assert_eq!(mode, PersistMode::Sync);
            store.save_mempool_snapshot(&exact, mode)
        },
        || PolicyTime::from_unix_seconds(131),
    )
    .expect("prepared exact encode should sync");
    handle
        .complete_snapshot_write(receipt)
        .expect("complete exact write");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen store");
    let loaded = reopened
        .load_mempool_snapshot_with_limits(
            MempoolSnapshotDecodeLimits::for_persisted_input().expect("persisted input limits"),
        )
        .expect("load exact snapshot")
        .expect("exact snapshot exists");

    // Assert
    assert_eq!(loaded.records.len(), exact.records.len());
    assert_eq!(
        loaded.records[0].acceptance_time,
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90))
    );
    assert_eq!(loaded.unbroadcast_members(), exact.unbroadcast_members());

    remove_dir_if_exists(&path);
}

#[test]
fn one_over_per_transaction_encode_aborts_and_preserves_prior_snapshot() {
    // Arrange
    let path = temp_store_path("one-over-per-tx-abort");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let prior = mempool_snapshot();
    store
        .save_mempool_snapshot(&prior, PersistMode::Sync)
        .expect("sync prior snapshot");
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write(
            PolicyTime::from_unix_seconds(140),
            CheckpointTrigger::Periodic,
        )
        .expect("prior capture should reserve");
    let one_over = current_snapshot_with_canonical_len(2, ONE_OVER_PER_TRANSACTION_BYTES);

    // Act
    let result = FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
        &handle,
        prepared,
        |_| encode_mempool_snapshot(&one_over),
        |_, _| panic!("put_bytes must not run after representability failure"),
        || PolicyTime::from_unix_seconds(141),
    );
    let preserved = store
        .load_mempool_snapshot_with_limits(
            MempoolSnapshotDecodeLimits::for_persisted_input().expect("persisted input limits"),
        )
        .expect("load prior after abort")
        .expect("prior snapshot remains");
    let retry = handle.prepare_mempool_snapshot_write(
        PolicyTime::from_unix_seconds(142),
        CheckpointTrigger::Periodic,
    );

    // Assert
    assert!(matches!(
        result,
        Err(SnapshotWriteExecutionError::Encode(StorageError::Corruption { ref detail, .. }))
            if detail == "mempool snapshot exceeds a resource bound"
    ));
    assert_eq!(preserved, prior);
    assert_eq!(
        preserved.records[0].acceptance_time,
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90))
    );
    assert_eq!(preserved.unbroadcast_members(), prior.unbroadcast_members());
    assert!(retry.is_ok());

    remove_dir_if_exists(&path);
}
