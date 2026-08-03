// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::*;

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
