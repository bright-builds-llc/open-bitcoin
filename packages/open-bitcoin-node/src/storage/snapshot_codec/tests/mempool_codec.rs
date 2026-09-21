// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn mempool_snapshot_round_trips_transactions() {
    // Arrange
    let snapshot = mempool_snapshot();

    // Act
    let encoded = encode_mempool_snapshot(&snapshot).expect("encode mempool");
    let decoded = decode_mempool_snapshot(&encoded).expect("decode mempool");

    // Assert
    assert_eq!(
        decoded.records[0].transaction,
        snapshot.records[0].transaction
    );
    assert_eq!(
        decoded.records[0].acceptance_time,
        snapshot.records[0].acceptance_time
    );
    assert_eq!(
        decoded.captured_generation(),
        snapshot.captured_generation()
    );
    assert_eq!(decoded.captured_at(), snapshot.captured_at());
    assert_eq!(
        decoded.unbroadcast_members(),
        snapshot.unbroadcast_members()
    );
}

#[test]
fn mempool_snapshot_codec_rejects_schema_mismatch() {
    // Arrange
    let mismatched = br#"{"schema_version":999,"payload":{"records":[]}}"#;

    // Act
    let error = decode_mempool_snapshot(mismatched).expect_err("schema mismatch should fail");

    // Assert
    assert!(matches!(error, StorageError::SchemaMismatch { .. }));
}

#[test]
fn mempool_snapshot_codec_rejects_corrupt_bytes() {
    // Arrange
    let corrupt = b"{not-json";

    // Act
    let error = decode_mempool_snapshot(corrupt).expect_err("corrupt bytes should fail");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ..
        }
    ));
}

#[test]
fn malformed_json_maps_to_corruption() {
    // Arrange
    let malformed = b"{not-json";

    // Act
    let error = decode_chainstate_snapshot(malformed).expect_err("malformed json");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            ..
        }
    ));
}

#[test]
fn mempool_snapshot_v2_encodes_only_source_authority() {
    // Arrange
    let snapshot = mempool_snapshot();

    // Act
    let encoded = encode_mempool_snapshot(&snapshot).expect("encode mempool");
    let decoded = decode_mempool_snapshot(&encoded).expect("decode mempool");
    let encoded_text = String::from_utf8(encoded.clone()).expect("utf8");

    // Assert
    assert_eq!(
        decoded.records[0].transaction,
        snapshot.records[0].transaction
    );
    assert!(encoded_text.contains("\"accepted_at_unix_seconds\": 90"));
    assert!(encoded_text.contains("\"format_version\": 2"));
    assert!(encoded_text.contains("\"captured_generation\": 42"));
    assert!(encoded_text.contains("\"captured_at_unix_seconds\": 120"));
    assert!(!encoded_text.contains("fee_sats"));
    assert!(!encoded_text.contains("virtual_size"));
    assert!(!encoded_text.contains("origin"));
    assert!(!encoded_text.contains("relay_requested"));
}

#[test]
fn legacy_unknown_current_snapshot_round_trips_as_explicit_null() {
    // Arrange
    let record = legacy_mempool_snapshot()
        .records
        .into_iter()
        .next()
        .expect("legacy record");
    let snapshot = MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(43),
        PolicyTime::from_unix_seconds(121),
        vec![record],
        BTreeSet::new(),
    )
    .expect("legacy-unknown age remains representable");

    // Act
    let encoded = encode_mempool_snapshot(&snapshot).expect("encode legacy-unknown current v2");
    let decoded = decode_mempool_snapshot(&encoded).expect("decode legacy-unknown current v2");
    let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("snapshot JSON");

    // Assert
    assert!(value["payload"]["records"][0]["accepted_at_unix_seconds"].is_null());
    assert_eq!(
        decoded.records[0].acceptance_time,
        MempoolAcceptanceTime::LegacyUnknown
    );
    value["payload"]["records"][0]
        .as_object_mut()
        .expect("record object")
        .remove("accepted_at_unix_seconds");
    let missing_key = serde_json::to_vec(&value).expect("encode missing key");
    assert!(decode_mempool_snapshot(&missing_key).is_err());
}

#[test]
fn legacy_mempool_snapshot_decodes_to_fail_closed_metadata() {
    // Arrange
    let transaction = mempool_transaction(24);
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let encoded_tx = open_bitcoin_core::codec::encode_transaction(
        &transaction,
        open_bitcoin_core::codec::TransactionEncoding::WithWitness,
    )
    .expect("encode tx");
    let legacy = serde_json::json!({
        "schema_version": SchemaVersion::CURRENT.get(),
        "payload": {
            "records": [{
                "txid": txid.to_byte_array(),
                "wtxid": wtxid.to_byte_array(),
                "transaction": encoded_tx,
                "fee_sats": 1000,
                "virtual_size": transaction_weight_and_virtual_size(&transaction)
                    .expect("transaction size")
                    .1
            }]
        }
    });
    let bytes = serde_json::to_vec(&legacy).expect("serialize legacy");

    // Act
    let decoded = decode_mempool_snapshot(&bytes).expect("decode legacy");

    // Assert
    assert_eq!(decoded.records.len(), 1);
    assert_eq!(
        decoded.records[0].acceptance_time,
        MempoolAcceptanceTime::LegacyUnknown
    );
    assert_eq!(
        decoded.records[0]
            .member_identity()
            .expect("canonical member identity"),
        MempoolMemberIdentity { txid, wtxid }
    );
    assert!(decoded.unbroadcast_members().is_empty());
    assert_eq!(SchemaVersion::CURRENT.get(), 2);
}

#[test]
fn mempool_snapshot_partial_metadata_is_corruption() {
    // Arrange
    let transaction = mempool_transaction(25);
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let encoded_tx = open_bitcoin_core::codec::encode_transaction(
        &transaction,
        open_bitcoin_core::codec::TransactionEncoding::WithWitness,
    )
    .expect("encode tx");
    let base = |accepted, origin, relay| {
        let mut record = serde_json::json!({
            "txid": txid.to_byte_array(),
            "wtxid": wtxid.to_byte_array(),
            "transaction": encoded_tx,
            "fee_sats": 1000,
            "virtual_size": 100
        });
        if let Some(value) = accepted {
            record["accepted_at_unix_seconds"] = serde_json::json!(value);
        }
        if let Some(value) = origin {
            record["origin"] = serde_json::json!(value);
        }
        if let Some(value) = relay {
            record["relay_requested"] = serde_json::json!(value);
        }
        serde_json::to_vec(&serde_json::json!({
            "schema_version": SchemaVersion::CURRENT.get(),
            "payload": {"records": [record]}
        }))
        .expect("serialize")
    };

    // Act / Assert
    for bytes in [
        base(Some(90), None, None),
        base(None, Some("local"), None),
        base(None, None, Some(true)),
        base(Some(90), Some("local"), None),
        base(Some(90), None, Some(true)),
        base(None, Some("local"), Some(true)),
    ] {
        let error = decode_mempool_snapshot(&bytes).expect_err("partial metadata");
        assert!(matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::Mempool,
                ..
            }
        ));
    }
}

#[test]
fn mempool_snapshot_invalid_origin_is_corruption() {
    // Arrange
    let transaction = mempool_transaction(26);
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let encoded_tx = open_bitcoin_core::codec::encode_transaction(
        &transaction,
        open_bitcoin_core::codec::TransactionEncoding::WithWitness,
    )
    .expect("encode tx");
    let bytes = serde_json::to_vec(&serde_json::json!({
        "schema_version": SchemaVersion::CURRENT.get(),
        "payload": {
            "records": [{
                "txid": txid.to_byte_array(),
                "wtxid": wtxid.to_byte_array(),
                "transaction": encoded_tx,
                "fee_sats": 1000,
                "virtual_size": 100,
                "accepted_at_unix_seconds": 90,
                "origin": "miner",
                "relay_requested": true
            }]
        }
    }))
    .expect("serialize");

    // Act
    let error = decode_mempool_snapshot(&bytes).expect_err("invalid origin");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ..
        }
    ));
}

#[test]
fn mempool_snapshot_encoded_schema_version_remains_current() {
    // Arrange
    let snapshot = mempool_snapshot();

    // Act
    let encoded = encode_mempool_snapshot(&snapshot).expect("encode");
    let value: serde_json::Value = serde_json::from_slice(&encoded).expect("json");

    // Assert
    assert_eq!(
        value["schema_version"].as_u64().expect("schema"),
        u64::from(SchemaVersion::CURRENT.get())
    );
    assert_eq!(SchemaVersion::CURRENT.get(), 2);
}
