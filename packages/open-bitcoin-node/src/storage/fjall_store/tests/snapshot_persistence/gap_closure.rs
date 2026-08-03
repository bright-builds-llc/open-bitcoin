// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::*;

const FORMER_RECORD_LIMIT_PLUS_ONE: usize = 50_001;
const FORMER_LIMIT_TEST_MAX_ENCODED_BYTES: usize = 64 * 1024 * 1024;

fn bounded_unique_snapshot_transaction(index: usize) -> Transaction {
    let mut previous_txid = [0_u8; 32];
    previous_txid[..size_of::<usize>()].copy_from_slice(&index.to_le_bytes());
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array(previous_txid),
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1_000).expect("bounded test amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn legacy_unknown_snapshot(transaction: Transaction) -> MempoolSnapshot {
    let txid = transaction_txid(&transaction).expect("legacy txid");
    let wtxid = transaction_wtxid(&transaction).expect("legacy wtxid");
    let transaction_bytes = open_bitcoin_core::codec::encode_transaction(
        &transaction,
        open_bitcoin_core::codec::TransactionEncoding::WithWitness,
    )
    .expect("encode legacy transaction");
    let virtual_size = transaction_weight_and_virtual_size(&transaction)
        .expect("legacy virtual size")
        .1;
    let legacy = serde_json::json!({
        "schema_version": SchemaVersion::CURRENT.get(),
        "payload": {
            "records": [{
                "txid": txid.to_byte_array(),
                "wtxid": wtxid.to_byte_array(),
                "transaction": transaction_bytes,
                "fee_sats": 1_000,
                "virtual_size": virtual_size
            }]
        }
    });
    let bytes = serde_json::to_vec(&legacy).expect("encode exact legacy snapshot");
    crate::storage::snapshot_codec::decode_mempool_snapshot(&bytes)
        .expect("decode exact legacy snapshot")
}

#[test]
fn legacy_unknown_recovery_survives_current_sync_checkpoint_and_reopen() {
    // Arrange
    let path = temp_store_path("legacy-unknown-current-checkpoint");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let (handle, coinbase_txid) = checkpoint_network_handle();
    let recovered_transaction = checkpoint_spend(coinbase_txid, 499_999_000);
    let recovered_txid = transaction_txid(&recovered_transaction).expect("recovered txid");
    let legacy_snapshot = legacy_unknown_snapshot(recovered_transaction);
    let prepared_recovery = handle
        .prepare_mempool_recovery_at(
            &legacy_snapshot,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            PolicyTime::new(200_000),
        )
        .expect("prepare legacy recovery");
    let recovery_summary = handle
        .install_mempool_recovery(prepared_recovery)
        .expect("install legacy recovery");
    let later_transaction = checkpoint_spend(recovered_txid, 499_998_000);
    let later_txid = transaction_txid(&later_transaction).expect("later txid");
    handle
        .submit_local_transaction_outcome_at(
            later_transaction,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            200_001,
            RelayIntent::NotRequested,
        )
        .expect("post-recovery mutation");

    // Act
    let prepared_checkpoint = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_002), CheckpointTrigger::Periodic)
        .expect("prepare current checkpoint");
    let prepared_records = prepared_checkpoint.snapshot().records.clone();
    let receipt = store
        .execute_prepared_mempool_snapshot_write(&handle, prepared_checkpoint, || {
            PolicyTime::new(200_003)
        })
        .expect("persist current checkpoint");
    let completion = handle
        .complete_snapshot_write(receipt)
        .expect("complete current checkpoint");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen store");
    let persisted = reopened
        .load_mempool_snapshot_with_limits(snapshot_decode_limits())
        .expect("load current snapshot")
        .expect("current snapshot exists");

    // Assert
    assert_eq!(recovery_summary.recovered_count, 1);
    assert_eq!(
        handle
            .mempool_entry_metadata(&recovered_txid)
            .expect("read recovered metadata")
            .expect("recovered entry")
            .accepted_at,
        MempoolAcceptanceTime::LegacyUnknown,
    );
    assert_eq!(completion, EffectCompletion::Applied);
    for records in [&prepared_records, &persisted.records] {
        assert_eq!(
            records
                .iter()
                .find(|record| record.member_identity().expect("identity").txid == recovered_txid)
                .expect("recovered record")
                .acceptance_time,
            MempoolAcceptanceTime::LegacyUnknown
        );
        assert_eq!(
            records
                .iter()
                .find(|record| record.member_identity().expect("identity").txid == later_txid)
                .expect("later record")
                .acceptance_time,
            MempoolAcceptanceTime::Known(PolicyTime::new(200_001))
        );
    }

    remove_dir_if_exists(&path);
}

#[test]
fn former_50_000_record_limit_checkpoints_and_reopens_with_policy_bounds() {
    // Arrange
    let path = temp_store_path("former-50000-record-limit");
    remove_dir_if_exists(&path);
    let captured_at = PolicyTime::new(300_000);
    let mut records = Vec::with_capacity(FORMER_RECORD_LIMIT_PLUS_ONE);
    for index in 0..FORMER_RECORD_LIMIT_PLUS_ONE {
        records.push(
            MempoolSnapshotRecord::try_from_canonical(
                bounded_unique_snapshot_transaction(index),
                MempoolAcceptanceTime::Known(captured_at),
            )
            .expect("canonical bounded record"),
        );
    }
    let first_identity = records
        .first()
        .expect("first record")
        .member_identity()
        .expect("first identity");
    let last_identity = records
        .last()
        .expect("last record")
        .member_identity()
        .expect("last identity");
    let snapshot = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(77),
        captured_at,
        records,
        BTreeSet::new(),
    )
    .expect("snapshot above former fixed limit");
    let encoded = crate::storage::snapshot_codec::encode_mempool_snapshot(&snapshot)
        .expect("encode bounded snapshot corpus");
    assert!(encoded.len() < FORMER_LIMIT_TEST_MAX_ENCODED_BYTES);
    assert!(
        snapshot
            .records
            .len()
            .checked_mul(size_of::<MempoolSnapshotRecord>())
            .expect("bounded record memory")
            < FORMER_LIMIT_TEST_MAX_ENCODED_BYTES
    );
    let store = FjallNodeStore::open(&path).expect("open store");

    // Act
    store
        .save_mempool_snapshot(&snapshot, PersistMode::Sync)
        .expect("Sync-persist snapshot above former limit");
    drop(encoded);
    drop(snapshot);
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen store");
    let limits = MempoolSnapshotDecodeLimits::from_policy(&PolicyConfig::default())
        .expect("policy-derived decode limits");
    let persisted = reopened
        .load_mempool_snapshot_with_limits(limits)
        .expect("load snapshot above former limit")
        .expect("persisted snapshot");

    // Assert
    assert_eq!(persisted.records.len(), FORMER_RECORD_LIMIT_PLUS_ONE);
    assert_eq!(
        persisted
            .records
            .first()
            .expect("first record")
            .member_identity()
            .expect("first identity"),
        first_identity
    );
    assert_eq!(
        persisted
            .records
            .last()
            .expect("last record")
            .member_identity()
            .expect("last identity"),
        last_identity
    );
    remove_dir_if_exists(&path);
}
