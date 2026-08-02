// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::*;

fn legacy_mempool_snapshot_value() -> serde_json::Value {
    let snapshot = legacy_mempool_snapshot();
    let record = &snapshot.records[0];
    let transaction = open_bitcoin_core::codec::encode_transaction(
        &record.transaction,
        open_bitcoin_core::codec::TransactionEncoding::WithWitness,
    )
    .expect("encode legacy transaction");
    serde_json::json!({
        "schema_version": 1,
        "payload": {
            "records": [{
                "txid": record.txid.to_byte_array(),
                "wtxid": record.wtxid.to_byte_array(),
                "transaction": transaction,
                "fee_sats": record.fee_sats,
                "virtual_size": record.virtual_size
            }]
        }
    })
}

#[test]
fn mempool_snapshot_codec_rejects_truncated_v2_json() {
    // Arrange
    let mut encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    encoded.truncate(encoded.len() / 2);

    // Act
    let error = decode_mempool_snapshot(&encoded).expect_err("truncated v2 should fail");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ref detail,
            ..
        } if detail == "mempool snapshot decode failed"
    ));
}

#[test]
fn mempool_snapshot_codec_rejects_unknown_local_version() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("json");
    value["payload"]["format_version"] = serde_json::json!(999);
    let bytes = serde_json::to_vec(&value).expect("serialize");

    // Act
    let error = decode_mempool_snapshot(&bytes).expect_err("unknown local version");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ref detail,
            ..
        } if detail == "unsupported mempool snapshot version"
    ));
}

#[test]
fn mempool_snapshot_codec_rejects_encoded_byte_limit() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let limits = MempoolSnapshotDecodeLimits {
        max_encoded_bytes: encoded.len() - 1,
        ..MempoolSnapshotDecodeLimits::default()
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(&encoded, limits)
        .expect_err("encoded byte limit should fail");

    // Assert
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
fn mempool_snapshot_codec_rejects_record_count_limit() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let limits = MempoolSnapshotDecodeLimits {
        max_records: 0,
        ..MempoolSnapshotDecodeLimits::default()
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(&encoded, limits)
        .expect_err("record count limit should fail");

    // Assert
    assert!(matches!(error, StorageError::Corruption { .. }));
}

#[test]
fn mempool_snapshot_codec_rejects_transaction_byte_limit() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let limits = MempoolSnapshotDecodeLimits {
        max_transaction_bytes: 1,
        max_total_transaction_bytes: 1,
        ..MempoolSnapshotDecodeLimits::default()
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(&encoded, limits)
        .expect_err("transaction byte limit should fail");

    // Assert
    assert!(matches!(error, StorageError::Corruption { .. }));
}

#[test]
fn mempool_snapshot_codec_rejects_unbroadcast_count_limit() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let limits = MempoolSnapshotDecodeLimits {
        max_unbroadcast_members: 0,
        ..MempoolSnapshotDecodeLimits::default()
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(&encoded, limits)
        .expect_err("unbroadcast count limit should fail");

    // Assert
    assert!(matches!(error, StorageError::Corruption { .. }));
}

#[test]
fn mempool_snapshot_codec_rejects_duplicate_v2_identity() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("json");
    let record = value["payload"]["records"][0].clone();
    value["payload"]["records"]
        .as_array_mut()
        .expect("records")
        .push(record);
    let bytes = serde_json::to_vec(&value).expect("serialize");

    // Act
    let error = decode_mempool_snapshot(&bytes).expect_err("duplicate identity");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ref detail,
            ..
        } if detail == "mempool snapshot identity mismatch"
    ));
}

#[test]
fn mempool_snapshot_encoder_rejects_duplicate_v2_identity() {
    // Arrange
    let mut snapshot = mempool_snapshot();
    snapshot.records.push(snapshot.records[0].clone());

    // Act
    let error = encode_mempool_snapshot(&snapshot).expect_err("duplicate identity");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ref detail,
            ..
        } if detail == "mempool snapshot identity mismatch"
    ));
}

#[test]
fn mempool_snapshot_codec_rejects_foreign_v2_unbroadcast_member() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("json");
    value["payload"]["unbroadcast_members"][0]["txid"] = serde_json::json!(vec![9_u8; 32]);
    let bytes = serde_json::to_vec(&value).expect("serialize");

    // Act
    let error = decode_mempool_snapshot(&bytes).expect_err("foreign unbroadcast member");

    // Assert
    assert!(matches!(error, StorageError::Corruption { .. }));
}

#[test]
fn legacy_snapshot_cannot_be_encoded_as_a_local_write() {
    // Arrange
    let snapshot = legacy_mempool_snapshot();

    // Act
    let error = encode_mempool_snapshot(&snapshot).expect_err("legacy local write must fail");

    // Assert
    assert!(matches!(error, StorageError::Corruption { .. }));
}

#[test]
fn legacy_mempool_snapshot_keeps_known_time_without_origin_or_relay_authority() {
    // Arrange
    let mut value = legacy_mempool_snapshot_value();
    value["payload"]["records"][0]["accepted_at_unix_seconds"] = serde_json::json!(90);
    value["payload"]["records"][0]["origin"] = serde_json::json!("local");
    value["payload"]["records"][0]["relay_requested"] = serde_json::json!(true);
    let bytes = serde_json::to_vec(&value).expect("serialize");

    // Act
    let decoded = decode_mempool_snapshot(&bytes).expect("decode legacy");

    // Assert
    assert_eq!(
        decoded.records[0].acceptance_time,
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90))
    );
    assert_eq!(
        decoded.records[0].metadata.origin,
        MempoolOrigin::RecoveryUnknown
    );
    assert_eq!(
        decoded.records[0].metadata.relay_intent,
        RelayIntent::NotRequested
    );
    assert!(decoded.unbroadcast_members().is_empty());
}

#[test]
fn legacy_mempool_snapshot_rejects_identity_mismatch() {
    // Arrange
    let original = legacy_mempool_snapshot_value();

    // Act / Assert
    for identity_field in ["txid", "wtxid"] {
        let mut value = original.clone();
        value["payload"]["records"][0][identity_field] = serde_json::json!(vec![7_u8; 32]);
        let bytes = serde_json::to_vec(&value).expect("serialize");
        let error = decode_mempool_snapshot(&bytes).expect_err("identity mismatch");
        assert!(matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::Mempool,
                ref detail,
                ..
            } if detail == "mempool snapshot identity mismatch"
        ));
    }
}
