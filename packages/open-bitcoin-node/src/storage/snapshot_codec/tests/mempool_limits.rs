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
    let identity = record.member_identity().expect("canonical member identity");
    let (_, virtual_size) =
        transaction_weight_and_virtual_size(&record.transaction).expect("transaction size");
    serde_json::json!({
        "schema_version": 1,
        "payload": {
            "records": [{
                "txid": identity.txid.to_byte_array(),
                "wtxid": identity.wtxid.to_byte_array(),
                "transaction": transaction,
                "fee_sats": 4_321,
                "virtual_size": virtual_size
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
fn mempool_snapshot_record_limit_stops_before_deserializing_the_extra_record() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("json");
    let mut malformed_extra = value["payload"]["records"][0].clone();
    malformed_extra["transaction"] = serde_json::json!({ "would_allocate": true });
    value["payload"]["records"]
        .as_array_mut()
        .expect("records")
        .push(malformed_extra);
    let bytes = serde_json::to_vec(&value).expect("serialize");
    let limits = MempoolSnapshotDecodeLimits {
        max_records: 1,
        ..MempoolSnapshotDecodeLimits::default()
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(&bytes, limits)
        .expect_err("record count must fail before decoding the extra record");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption { ref detail, .. }
            if detail == "mempool snapshot exceeds a resource bound"
    ));
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
fn escaped_transaction_token_is_rejected_before_unescaping_under_narrow_budget() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let text = String::from_utf8(encoded).expect("snapshot JSON");
    let field_end =
        text.find("\"transaction\"").expect("transaction field") + "\"transaction\"".len();
    let transaction_start = text[field_end..]
        .find('"')
        .map(|offset| field_end + offset + 1)
        .expect("transaction value");
    let transaction_end = text[transaction_start..]
        .find('"')
        .map(|offset| transaction_start + offset)
        .expect("transaction terminator");
    let escaped = "\\u0030".repeat(64);
    let hostile = format!(
        "{}{}{}",
        &text[..transaction_start],
        escaped,
        &text[transaction_end..]
    );
    let limits = MempoolSnapshotDecodeLimits {
        max_encoded_bytes: hostile.len(),
        max_transaction_bytes: 1,
        max_total_transaction_bytes: 1,
        ..MempoolSnapshotDecodeLimits::default()
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(hostile.as_bytes(), limits)
        .expect_err("escaped transaction must fail before unescaping");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption { ref detail, .. }
            if detail == "mempool snapshot exceeds a resource bound"
    ));
}

#[test]
fn oversized_unknown_key_is_rejected_without_owned_key_materialization() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let text = String::from_utf8(encoded).expect("snapshot JSON");
    let hostile_key = "x".repeat(16 * 1_024);
    let hostile = text.replacen("\"schema_version\"", &format!("\"{hostile_key}\""), 1);
    let limits = MempoolSnapshotDecodeLimits {
        max_encoded_bytes: hostile.len(),
        max_transaction_bytes: 1,
        max_total_transaction_bytes: 1,
        ..MempoolSnapshotDecodeLimits::default()
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(hostile.as_bytes(), limits)
        .expect_err("unknown field must fail without an owned key");

    // Assert
    assert!(matches!(error, StorageError::Corruption { .. }));
}

#[test]
fn legacy_transaction_sequence_limit_stops_before_parsing_the_extra_byte() {
    // Arrange
    let mut value = legacy_mempool_snapshot_value();
    value["payload"]["records"][0]["transaction"] = serde_json::json!([1, "not-a-byte"]);
    let bytes = serde_json::to_vec(&value).expect("serialize");
    let limits = MempoolSnapshotDecodeLimits {
        max_transaction_bytes: 1,
        max_total_transaction_bytes: 1,
        ..MempoolSnapshotDecodeLimits::default()
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(&bytes, limits)
        .expect_err("transaction sequence limit must stop before the extra element");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption { ref detail, .. }
            if detail == "mempool snapshot exceeds a resource bound"
    ));
}

#[test]
fn current_snapshot_encodes_transaction_bytes_as_compact_hex() {
    // Arrange
    let snapshot = mempool_snapshot();

    // Act
    let encoded = encode_mempool_snapshot(&snapshot).expect("encode mempool");
    let value: serde_json::Value = serde_json::from_slice(&encoded).expect("json");

    // Assert
    let transaction = value["payload"]["records"][0]["transaction"]
        .as_str()
        .expect("current transaction hex");
    assert_eq!(transaction.len() % 2, 0);
    assert!(transaction.bytes().all(|byte| byte.is_ascii_hexdigit()));
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
