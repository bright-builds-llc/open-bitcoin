// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use std::collections::{BTreeSet, HashMap};

use open_bitcoin_core::{
    chainstate::{ChainstateSnapshot, Coin},
    consensus::{
        ConsensusParams, ScriptVerifyFlags, crypto::hash160, transaction_txid, transaction_wtxid,
    },
    primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput, Txid,
    },
};
use open_bitcoin_mempool::{
    MempoolAcceptanceTime, MempoolEntryMetadata, MempoolMemberIdentity, MempoolOrigin,
    PolicyConfig, PolicyTime, RelayIntent,
};

use super::{
    CapturedMempoolGeneration, MempoolRecoveryStatus, MempoolSnapshot, MempoolSnapshotError,
    MempoolSnapshotRecord, recovery_status_from_outcome,
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

fn chainstate_with_utxo(outpoint: OutPoint, value_sats: i64) -> ChainstateSnapshot {
    let mut utxos = HashMap::new();
    utxos.insert(
        outpoint,
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(value_sats).expect("valid amount"),
                script_pubkey: p2sh_script(),
            },
            is_coinbase: false,
            created_height: 0,
            created_median_time_past: 0,
        },
    );

    ChainstateSnapshot::new(Vec::new(), utxos, HashMap::new())
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
fn current_snapshot_rejects_unknown_acceptance_time() {
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

#[test]
fn mempool_snapshot_replay_recovers_accepted_records() {
    // Arrange
    let previous_output = OutPoint {
        txid: Txid::from_byte_array([1_u8; 32]),
        vout: 0,
    };
    let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
    let record = snapshot_record(spend_transaction(previous_output, 499_000));
    let snapshot = MempoolSnapshot::from_legacy_v1(vec![record.clone()]);
    let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

    // Act
    let recovery = snapshot.replay_into_mempool(
        &mut mempool,
        &chainstate,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );

    // Assert
    assert_eq!(recovery[0].status, MempoolRecoveryStatus::Recovered);
    assert_eq!(recovery[0].status.as_str(), "recovered");
    assert!(mempool.entry(&record.txid).is_some());
}

#[test]
fn mempool_snapshot_replay_drops_confirmed_records_with_evidence() {
    // Arrange
    let previous_output = OutPoint {
        txid: Txid::from_byte_array([2_u8; 32]),
        vout: 0,
    };
    let record = snapshot_record(spend_transaction(previous_output, 499_000));
    let confirmed_output = OutPoint {
        txid: record.txid,
        vout: 0,
    };
    let chainstate = chainstate_with_utxo(confirmed_output, 499_000);
    let snapshot = MempoolSnapshot::from_legacy_v1(vec![record.clone()]);
    let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

    // Act
    let recovery = snapshot.replay_into_mempool(
        &mut mempool,
        &chainstate,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );

    // Assert
    assert_eq!(recovery[0].status, MempoolRecoveryStatus::DroppedConfirmed);
    assert_eq!(recovery[0].status.as_str(), "dropped_confirmed");
    assert!(mempool.entry(&record.txid).is_none());
}

#[test]
fn mempool_snapshot_replay_drops_policy_incompatible_records_with_evidence() {
    // Arrange
    let previous_output = OutPoint {
        txid: Txid::from_byte_array([3_u8; 32]),
        vout: 0,
    };
    let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
    let record = snapshot_record(spend_transaction(previous_output, 499_999));
    let snapshot = MempoolSnapshot::from_legacy_v1(vec![record.clone()]);
    let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

    // Act
    let recovery = snapshot.replay_into_mempool(
        &mut mempool,
        &chainstate,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );

    // Assert
    assert_eq!(
        recovery[0].status,
        MempoolRecoveryStatus::DroppedPolicyIncompatible
    );
    assert_eq!(recovery[0].status.as_str(), "dropped_policy_incompatible");
    assert!(mempool.entry(&record.txid).is_none());
}

#[test]
fn recovery_status_from_outcome_classifies_unexpected_errors_as_policy_incompatible() {
    // Arrange
    let error = open_bitcoin_mempool::MempoolError::Validation {
        reason: "bad-tx".to_string(),
    };

    // Act
    let status = recovery_status_from_outcome(Err(error));

    // Assert
    assert_eq!(status, MempoolRecoveryStatus::DroppedPolicyIncompatible);
}

#[test]
fn recovery_metadata_from_mempool_copies_exact_entry_metadata() {
    // Arrange
    let previous_output = OutPoint {
        txid: Txid::from_byte_array([11_u8; 32]),
        vout: 0,
    };
    let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
    let transaction = spend_transaction(previous_output, 499_000);
    let expected = known_local_requested(90);
    let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());
    mempool
        .accept_transaction_transition_with_context(
            transaction,
            &chainstate,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
            open_bitcoin_mempool::AdmissionContext::recovery(expected),
        )
        .expect("admit known local");

    // Act
    let snapshot = MempoolSnapshot::from_mempool(&mempool);

    // Assert
    assert_eq!(snapshot.records.len(), 1);
    assert_eq!(snapshot.records[0].metadata, expected);
}

#[test]
fn recovery_metadata_known_local_requested_replays_exactly_and_stays_retry_eligible() {
    // Arrange
    let previous_output = OutPoint {
        txid: Txid::from_byte_array([12_u8; 32]),
        vout: 0,
    };
    let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
    let transaction = spend_transaction(previous_output, 499_000);
    let expected = known_local_requested(90);
    let record = snapshot_record_with_metadata(transaction, expected);
    let snapshot = MempoolSnapshot::from_legacy_v1(vec![record.clone()]);
    let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

    // Act
    let recovery = snapshot.replay_into_mempool(
        &mut mempool,
        &chainstate,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );

    // Assert
    assert_eq!(recovery[0].status, MempoolRecoveryStatus::Recovered);
    let entry = mempool.entry(&record.txid).expect("recovered entry");
    assert_eq!(entry.metadata, expected);
    assert!(entry.metadata.is_retry_eligible(true));
    assert!(!entry.metadata.is_retry_eligible(false));
}

#[test]
fn recovery_metadata_known_peer_and_reorg_remain_non_local_after_replay() {
    // Arrange
    let peer_previous = OutPoint {
        txid: Txid::from_byte_array([13_u8; 32]),
        vout: 0,
    };
    let reorg_previous = OutPoint {
        txid: Txid::from_byte_array([14_u8; 32]),
        vout: 0,
    };
    let mut utxos = HashMap::new();
    utxos.insert(
        peer_previous.clone(),
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(500_000).expect("valid amount"),
                script_pubkey: p2sh_script(),
            },
            is_coinbase: false,
            created_height: 0,
            created_median_time_past: 0,
        },
    );
    utxos.insert(
        reorg_previous.clone(),
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(500_000).expect("valid amount"),
                script_pubkey: p2sh_script(),
            },
            is_coinbase: false,
            created_height: 0,
            created_median_time_past: 0,
        },
    );
    let chainstate = ChainstateSnapshot::new(Vec::new(), utxos, HashMap::new());
    let peer_metadata = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(40)),
        MempoolOrigin::Peer,
        RelayIntent::NotRequested,
    );
    let reorg_metadata = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(80)),
        MempoolOrigin::Reorg,
        RelayIntent::NotRequested,
    );
    let peer_record =
        snapshot_record_with_metadata(spend_transaction(peer_previous, 499_000), peer_metadata);
    let reorg_record =
        snapshot_record_with_metadata(spend_transaction(reorg_previous, 499_000), reorg_metadata);
    let snapshot = MempoolSnapshot::from_legacy_v1(vec![peer_record.clone(), reorg_record.clone()]);
    let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

    // Act
    let _ = snapshot.replay_into_mempool(
        &mut mempool,
        &chainstate,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );

    // Assert
    assert_eq!(
        mempool.entry(&peer_record.txid).expect("peer").metadata,
        peer_metadata
    );
    assert_eq!(
        mempool.entry(&reorg_record.txid).expect("reorg").metadata,
        reorg_metadata
    );
    assert!(!peer_metadata.is_retry_eligible(true));
    assert!(!reorg_metadata.is_retry_eligible(true));
}

#[test]
fn recovery_metadata_legacy_replays_fail_closed_not_restart_time() {
    // Arrange
    let previous_output = OutPoint {
        txid: Txid::from_byte_array([15_u8; 32]),
        vout: 0,
    };
    let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
    let record = snapshot_record(spend_transaction(previous_output, 499_000));
    assert_eq!(record.metadata, MempoolEntryMetadata::legacy_unknown());
    let snapshot = MempoolSnapshot::from_legacy_v1(vec![record.clone()]);
    let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

    // Act
    let _ = snapshot.replay_into_mempool(
        &mut mempool,
        &chainstate,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );

    // Assert
    let entry = mempool.entry(&record.txid).expect("legacy recovered");
    assert_eq!(entry.metadata, MempoolEntryMetadata::legacy_unknown());
    assert!(!entry.metadata.is_retry_eligible(true));
}

#[test]
fn recovery_metadata_duplicate_does_not_rewrite_existing_canonical_metadata() {
    // Arrange
    let previous_output = OutPoint {
        txid: Txid::from_byte_array([16_u8; 32]),
        vout: 0,
    };
    let chainstate = chainstate_with_utxo(previous_output.clone(), 500_000);
    let transaction = spend_transaction(previous_output, 499_000);
    let original = known_local_requested(90);
    let conflicting = MempoolEntryMetadata::new(
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(999)),
        MempoolOrigin::Peer,
        RelayIntent::NotRequested,
    );
    let original_record = snapshot_record_with_metadata(transaction.clone(), original);
    let duplicate_record = snapshot_record_with_metadata(transaction, conflicting);
    let snapshot = MempoolSnapshot::from_legacy_v1(vec![original_record.clone(), duplicate_record]);
    let mut mempool = open_bitcoin_mempool::Mempool::new(PolicyConfig::default());

    // Act
    let recovery = snapshot.replay_into_mempool(
        &mut mempool,
        &chainstate,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );

    // Assert
    assert_eq!(recovery[0].status, MempoolRecoveryStatus::Recovered);
    assert_eq!(recovery[1].status, MempoolRecoveryStatus::DroppedDuplicate);
    assert_eq!(
        mempool
            .entry(&original_record.txid)
            .expect("original")
            .metadata,
        original
    );
}
