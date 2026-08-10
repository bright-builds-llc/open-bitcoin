// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::*;

#[test]
fn persisted_input_limits_fit_the_global_encoded_dimensions_exactly() {
    // Arrange
    let limits = crate::storage::snapshot_codec::persisted_mempool_input_limits()
        .expect("persisted input limits");

    // Act
    let exact = crate::storage::snapshot_codec::encoded_size_upper_bound(
        limits.max_total_transaction_bytes,
        limits.max_records,
        limits.max_unbroadcast_members,
    );
    let one_record_over = crate::storage::snapshot_codec::encoded_size_upper_bound(
        limits.max_total_transaction_bytes,
        limits.max_records + 1,
        limits.max_unbroadcast_members,
    );

    // Assert
    assert_eq!(limits.max_encoded_bytes, 268_435_456);
    assert_eq!(limits.max_records, 220_096);
    assert_eq!(limits.max_unbroadcast_members, 5_000);
    assert_eq!(limits.max_total_transaction_bytes, 67_108_864);
    assert_eq!(exact, Some(limits.max_encoded_bytes));
    assert_eq!(one_record_over, Some(limits.max_encoded_bytes + 512));
}

#[test]
fn persisted_input_limits_derive_aggregate_edges_from_transaction_bytes() {
    // Arrange
    let limits = crate::storage::snapshot_codec::persisted_mempool_input_limits()
        .expect("persisted input limits");
    let minimum_input_bytes = OutPoint::SERIALIZED_LEN + 1 + core::mem::size_of::<u32>();

    // Act
    let exact_bytes = limits
        .max_input_edges
        .checked_mul(minimum_input_bytes)
        .expect("exact aggregate edge bytes");
    let one_over_bytes = (limits.max_input_edges + 1)
        .checked_mul(minimum_input_bytes)
        .expect("one-over aggregate edge bytes");

    // Assert
    assert_eq!(minimum_input_bytes, 41);
    assert_eq!(limits.max_input_edges, 1_636_801);
    assert!(exact_bytes <= limits.max_total_transaction_bytes);
    assert!(one_over_bytes > limits.max_total_transaction_bytes);
}

#[test]
fn persisted_input_limits_derive_per_record_edges_from_transaction_bytes() {
    // Arrange
    let limits = crate::storage::snapshot_codec::persisted_mempool_input_limits()
        .expect("persisted input limits");
    let minimum_input_bytes = OutPoint::SERIALIZED_LEN + 1 + core::mem::size_of::<u32>();

    // Act
    let exact_bytes = limits
        .max_input_edges_per_record
        .checked_mul(minimum_input_bytes)
        .expect("exact per-record edge bytes");
    let one_over_bytes = (limits.max_input_edges_per_record + 1)
        .checked_mul(minimum_input_bytes)
        .expect("one-over per-record edge bytes");

    // Assert
    assert_eq!(limits.max_transaction_bytes, 4_194_304);
    assert_eq!(limits.max_input_edges_per_record, 102_300);
    assert!(exact_bytes <= limits.max_transaction_bytes);
    assert!(one_over_bytes > limits.max_transaction_bytes);
}

#[test]
fn persisted_input_limits_reject_one_record_over_its_transaction_budget() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let value: serde_json::Value = serde_json::from_slice(&encoded).expect("json");
    let transaction_bytes = value["payload"]["records"][0]["transaction"]
        .as_str()
        .expect("transaction hex")
        .len()
        / 2;
    let limits = MempoolSnapshotDecodeLimits {
        max_encoded_bytes: encoded.len(),
        max_records: 220_096,
        max_unbroadcast_members: 5_000,
        max_transaction_bytes: transaction_bytes - 1,
        max_total_transaction_bytes: transaction_bytes,
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(&encoded, limits)
        .expect_err("per-record transaction budget must fail");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption { ref detail, .. }
            if detail == "mempool snapshot exceeds a resource bound"
    ));
}

#[test]
fn persisted_input_limits_reject_aggregate_transaction_bytes_below_record_limit() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("json");
    let record = value["payload"]["records"][0].clone();
    let transaction_bytes = record["transaction"]
        .as_str()
        .expect("transaction hex")
        .len()
        / 2;
    value["payload"]["records"]
        .as_array_mut()
        .expect("records")
        .push(record);
    let bytes = serde_json::to_vec(&value).expect("serialize");
    let limits = MempoolSnapshotDecodeLimits {
        max_encoded_bytes: bytes.len(),
        max_records: 220_096,
        max_unbroadcast_members: 5_000,
        max_transaction_bytes: transaction_bytes,
        max_total_transaction_bytes: transaction_bytes
            .checked_mul(2)
            .and_then(|total| total.checked_sub(1))
            .expect("aggregate one-under limit"),
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(&bytes, limits)
        .expect_err("aggregate transaction budget must fail");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption { ref detail, .. }
            if detail == "mempool snapshot exceeds a resource bound"
    ));
}

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
        } if detail == "mempool snapshot structure is corrupt"
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
fn escaped_oversized_unknown_key_is_rejected_before_serde_unescaping() {
    // Arrange
    let encoded = encode_mempool_snapshot(&mempool_snapshot()).expect("encode mempool");
    let text = String::from_utf8(encoded).expect("snapshot JSON");
    let hostile_key = "\\u0078".repeat(4 * 1_024);
    let hostile = text.replacen("\"schema_version\"", &format!("\"{hostile_key}\""), 1);
    let limits = MempoolSnapshotDecodeLimits {
        max_encoded_bytes: hostile.len(),
        max_transaction_bytes: 1,
        max_total_transaction_bytes: 1,
        ..MempoolSnapshotDecodeLimits::default()
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(hostile.as_bytes(), limits)
        .expect_err("escaped unknown field must fail before unescaping");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption { ref detail, .. }
            if detail == "mempool snapshot exceeds a resource bound"
    ));
}

#[test]
fn raw_key_preflight_rejects_oversized_unterminated_escaped_key_under_narrow_budget() {
    // Arrange
    let hostile_key = format!("{}\\", "x".repeat(65));
    let hostile = format!("{{\"{hostile_key}");
    let limits = MempoolSnapshotDecodeLimits {
        max_encoded_bytes: hostile.len(),
        max_records: 0,
        max_unbroadcast_members: 0,
        max_transaction_bytes: 0,
        max_total_transaction_bytes: 0,
    };

    // Act
    let error = decode_mempool_snapshot_with_limits(hostile.as_bytes(), limits)
        .expect_err("oversized unterminated key must fail before Serde");

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
