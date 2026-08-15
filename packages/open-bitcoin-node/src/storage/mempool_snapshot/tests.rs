// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use std::collections::BTreeSet;

use open_bitcoin_core::{
    consensus::{crypto::hash160, transaction_txid, transaction_wtxid},
    primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput, Txid,
    },
};
use open_bitcoin_mempool::{
    MempoolAcceptanceTime, MempoolEntryMetadata, MempoolMemberIdentity, MempoolOrigin, PolicyTime,
    RelayIntent,
};

use super::{
    CapturedMempoolGeneration, MempoolSnapshot, MempoolSnapshotError, MempoolSnapshotRecord,
};

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn p2sh_script() -> ScriptBuf {
    let redeem_hash = hash160(script(&[0x51]).as_bytes());
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    script(&bytes)
}

fn spend_transaction(previous_output: OutPoint, output_value_sats: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output,
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(output_value_sats).expect("valid amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn snapshot_record(transaction: Transaction) -> MempoolSnapshotRecord {
    snapshot_record_with_metadata(transaction, MempoolEntryMetadata::legacy_unknown())
}

fn snapshot_record_with_metadata(
    transaction: Transaction,
    metadata: MempoolEntryMetadata,
) -> MempoolSnapshotRecord {
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let (_, virtual_size) = open_bitcoin_mempool::transaction_weight_and_virtual_size(&transaction)
        .expect("transaction size");
    MempoolSnapshotRecord::try_from_compatibility(
        transaction,
        txid,
        wtxid,
        1_000,
        virtual_size,
        metadata,
    )
    .expect("valid compatibility fixture")
}

fn known_local_requested(accepted_at: i64) -> MempoolEntryMetadata {
    MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(accepted_at)),
        MempoolOrigin::Local,
        RelayIntent::Requested,
    )
}

#[test]
fn captured_generation_try_new_rejects_u64_max() {
    // Arrange
    let terminal = u64::MAX;

    // Act
    let rejected = CapturedMempoolGeneration::try_new(terminal);
    let zero = CapturedMempoolGeneration::try_new(0);
    let one = CapturedMempoolGeneration::try_new(1);
    let last_valid = CapturedMempoolGeneration::try_new(u64::MAX - 1);

    // Assert
    assert_eq!(rejected, Err(MempoolSnapshotError::StructuralCorruption));
    assert_eq!(zero.map(CapturedMempoolGeneration::raw), Ok(0));
    assert_eq!(one.map(CapturedMempoolGeneration::raw), Ok(1));
    assert_eq!(
        last_valid.map(CapturedMempoolGeneration::raw),
        Ok(u64::MAX - 1)
    );
}

#[test]
fn current_snapshot_rejects_terminal_captured_generation() {
    // Arrange
    let captured_generation = CapturedMempoolGeneration::new(u64::MAX);

    // Act
    let result = MempoolSnapshot::try_new_current(
        captured_generation,
        PolicyTime::from_unix_seconds(120),
        Vec::new(),
        BTreeSet::new(),
    );

    // Assert
    assert_eq!(result, Err(MempoolSnapshotError::StructuralCorruption));
}

#[test]
fn current_snapshot_preserves_legacy_unknown_acceptance_time() {
    // Arrange
    let transaction = spend_transaction(
        OutPoint {
            txid: Txid::from_byte_array([31_u8; 32]),
            vout: 0,
        },
        499_000,
    );
    let record = snapshot_record(transaction);

    // Act
    let snapshot = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(7),
        PolicyTime::from_unix_seconds(120),
        vec![record],
        BTreeSet::new(),
    )
    .expect("legacy-unknown age remains representable");

    // Assert
    assert_eq!(
        snapshot.records[0].acceptance_time,
        MempoolAcceptanceTime::LegacyUnknown
    );
}

#[test]
fn current_snapshot_rejects_acceptance_time_after_capture() {
    // Arrange
    let transaction = spend_transaction(
        OutPoint {
            txid: Txid::from_byte_array([36_u8; 32]),
            vout: 0,
        },
        499_000,
    );
    let record = snapshot_record_with_metadata(transaction, known_local_requested(121));

    // Act
    let result = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(7),
        PolicyTime::from_unix_seconds(120),
        vec![record],
        BTreeSet::new(),
    );

    // Assert
    assert_eq!(result, Err(MempoolSnapshotError::StructuralCorruption));
}

#[test]
fn current_snapshot_rejects_foreign_unbroadcast_member() {
    // Arrange
    let transaction = spend_transaction(
        OutPoint {
            txid: Txid::from_byte_array([32_u8; 32]),
            vout: 0,
        },
        499_000,
    );
    let record = snapshot_record_with_metadata(transaction, known_local_requested(90));
    let foreign = MempoolMemberIdentity {
        txid: Txid::from_byte_array([33_u8; 32]),
        wtxid: open_bitcoin_core::primitives::Wtxid::from_byte_array([34_u8; 32]),
    };

    // Act
    let result = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(7),
        PolicyTime::from_unix_seconds(120),
        vec![record],
        BTreeSet::from([foreign]),
    );

    // Assert
    assert_eq!(result, Err(MempoolSnapshotError::IdentityMismatch));
}

#[test]
fn legacy_snapshot_keeps_unknown_acceptance_time_unknown() {
    // Arrange
    let transaction = spend_transaction(
        OutPoint {
            txid: Txid::from_byte_array([35_u8; 32]),
            vout: 0,
        },
        499_000,
    );
    let record = snapshot_record(transaction);

    // Act
    let snapshot = MempoolSnapshot::from_legacy_v1(vec![record]);

    // Assert
    assert_eq!(
        snapshot.records[0].acceptance_time,
        MempoolAcceptanceTime::LegacyUnknown
    );
    assert_eq!(snapshot.captured_generation(), None);
    assert_eq!(snapshot.captured_at(), None);
    assert!(snapshot.unbroadcast_members().is_empty());
}
