// Parity breadcrumbs:
// - packages/bitcoin-knots/src/arith_uint256.h

use super::*;
use crate::StorageNamespace;
use open_bitcoin_core::consensus::chain_proof::ProofWork;

#[test]
fn phase165_work_codec_preserves_all_exact_bytes() {
    // Arrange
    let bytes = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 128,
    ];
    let number = ProofWork::from_le_bytes(bytes);
    let record = ClassifiedWorkRecord::ExactNumber(number);

    // Act
    let encoded = encode_work_record(StorageNamespace::Chainstate, &record).expect("encode exact");
    let decoded = decode_work_record(StorageNamespace::Chainstate, &encoded).expect("decode exact");

    // Assert
    assert_eq!(decoded, record);
    assert!(
        String::from_utf8(encoded)
            .expect("JSON")
            .contains("\"kind\":\"exact_number\"")
    );
    let ClassifiedWorkRecord::ExactNumber(decoded_number) = decoded else {
        panic!("exact bytes must remain exact numeric data");
    };
    assert_eq!(decoded_number.to_le_bytes(), bytes);
}

#[test]
fn phase165_work_codec_classifies_historic_u128_numbers() {
    // Arrange
    let historic = [
        (&b"0"[..], 0),
        (&b"340282366920938463463374607431768211455"[..], u128::MAX),
    ];

    for (bytes, score) in historic {
        // Act
        let decoded =
            decode_work_record(StorageNamespace::Chainstate, bytes).expect("legacy score");

        // Assert
        assert_eq!(decoded, ClassifiedWorkRecord::LegacySynthetic(score));
    }
}

#[test]
fn phase165_work_codec_refuses_unknown_record_version() {
    // Arrange
    let bytes = br#"{"version":2,"kind":"legacy_synthetic","score":1}"#;

    // Act
    let result = decode_work_record(StorageNamespace::Chainstate, bytes);

    // Assert
    assert!(result.is_err());
}

#[test]
fn phase165_work_codec_exact_boundaries_remain_exact() {
    // Arrange
    let mut above_u128 = [0; 32];
    above_u128[16] = 1;
    let values = [[0; 32], [255; 32], above_u128];

    for bytes in values {
        let record = ClassifiedWorkRecord::ExactNumber(ProofWork::from_le_bytes(bytes));
        // Act
        let encoded =
            encode_work_record(StorageNamespace::Headers, &record).expect("encode boundary");
        let decoded =
            decode_work_record(StorageNamespace::Headers, &encoded).expect("decode boundary");

        // Assert
        assert_eq!(decoded, record);
    }
}

#[test]
fn phase165_work_codec_tagged_legacy_keeps_u128_precision() {
    // Arrange
    let fixed = br#"{"version":1,"kind":"legacy_synthetic","score":340282366920938463463374607431768211455}"#;
    let record = ClassifiedWorkRecord::LegacySynthetic(u128::MAX);

    // Act
    let decoded = decode_work_record(StorageNamespace::Chainstate, fixed).expect("tagged max-u128");
    let encoded = encode_work_record(StorageNamespace::Chainstate, &record).expect("encode legacy");

    // Assert
    assert_eq!(decoded, record);
    assert_eq!(encoded, fixed);
}

#[test]
fn phase165_work_codec_fixed_exact_record_is_little_endian() {
    // Arrange
    let fixed = br#"{"version":1,"kind":"exact_number","bytes_le":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,128]}"#;

    // Act
    let decoded =
        decode_work_record(StorageNamespace::BlockIndex, fixed).expect("fixed exact bytes");

    // Assert
    let ClassifiedWorkRecord::ExactNumber(number) = decoded else {
        panic!("tagged bytes remain exact numbers");
    };
    assert_eq!(
        number.to_hex(),
        "801f1e1d1c1b1a191817161514131211100f0e0d0c0b0a090807060504030201"
    );
}

#[test]
fn phase165_work_codec_refuses_malformed_legacy_numbers() {
    // Arrange
    let malformed = [
        "340282366920938463463374607431768211456",
        "-1",
        "1.0",
        "1e0",
        "\"1\"",
        "null",
        "true",
        "[]",
        "1 2",
        "",
        "01",
        r#"{"version":1,"kind":"legacy_synthetic","score":340282366920938463463374607431768211456}"#,
    ];

    for input in malformed {
        // Act
        let result = decode_work_record(StorageNamespace::Chainstate, input.as_bytes());

        // Assert
        assert!(result.is_err(), "must refuse {input}");
    }
}

#[test]
fn phase165_work_codec_refuses_ambiguous_and_unknown_tags() {
    // Arrange
    let malformed = [
        r#"{"version":1,"kind":"accepted_work","score":1}"#,
        r#"{"version":1,"score":1}"#,
        r#"{"kind":"legacy_synthetic","score":1}"#,
        r#"{"version":0,"kind":"legacy_synthetic","score":1}"#,
        r#"{"version":1,"kind":"legacy_synthetic","score":1,"bytes_le":null}"#,
        r#"{"version":1,"kind":"exact_number","score":1}"#,
        r#"{"version":1,"kind":"legacy_synthetic","score":1,"extra":0}"#,
        r#"{"version":1,"kind":"legacy_synthetic","kind":"exact_number","score":1}"#,
        r#"{"version":1,"version":1,"kind":"legacy_synthetic","score":1}"#,
        r#"{"version":1,"kind":"legacy_synthetic","score":1,"score":1}"#,
    ];

    for input in malformed {
        // Act
        let result = decode_work_record(StorageNamespace::Headers, input.as_bytes());

        // Assert
        assert!(result.is_err(), "must refuse {input}");
    }
}

#[test]
fn phase165_work_codec_refuses_wrong_exact_width_and_byte_values() {
    // Arrange
    let mut malformed_bytes = vec![serde_json::json!([]), serde_json::json!(null)];
    for length in [31, 33] {
        malformed_bytes.push(serde_json::json!(vec![0; length]));
    }
    for invalid_byte in [-1, 256] {
        let mut bytes = vec![0; 32];
        bytes[31] = invalid_byte;
        malformed_bytes.push(serde_json::json!(bytes));
    }

    for bytes in malformed_bytes {
        let record = serde_json::json!({"version":1,"kind":"exact_number","bytes_le":bytes});
        let input = serde_json::to_vec(&record).expect("malformed fixture JSON");
        // Act
        let result = decode_work_record(StorageNamespace::BlockIndex, &input);

        // Assert
        assert!(result.is_err());
    }
}

#[test]
fn phase165_work_codec_refuses_conflicting_exact_fields() {
    // Arrange
    let record =
        serde_json::json!({"version":1,"kind":"exact_number","bytes_le":vec![0; 32],"score":0});
    let input = serde_json::to_vec(&record).expect("mixed JSON");

    // Act
    let result = decode_work_record(StorageNamespace::BlockIndex, &input);

    // Assert
    assert!(result.is_err());
}

#[test]
fn phase165_work_codec_embedded_serde_preserves_classification() {
    // Arrange
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Fixture {
        chain_work: ClassifiedWorkRecord,
    }
    let fixed = br#"{"chain_work":340282366920938463463374607431768211455}"#;

    // Act
    let fixture: Fixture = serde_json::from_slice(fixed).expect("historic embedded number");
    let encoded = serde_json::to_vec(&fixture).expect("tagged embedded output");
    let restored: Fixture = serde_json::from_slice(&encoded).expect("tagged embedded input");

    // Assert
    assert_eq!(
        fixture.chain_work,
        ClassifiedWorkRecord::LegacySynthetic(u128::MAX)
    );
    assert_eq!(restored.chain_work, fixture.chain_work);
}

#[test]
fn phase165_work_codec_embedded_exact_bytes_remain_exact() {
    // Arrange
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Fixture {
        chain_work: ClassifiedWorkRecord,
    }
    let fixture = Fixture {
        chain_work: ClassifiedWorkRecord::ExactNumber(ProofWork::from_le_bytes([255; 32])),
    };

    // Act
    let encoded = serde_json::to_vec(&fixture).expect("embedded exact output");
    let restored: Fixture = serde_json::from_slice(&encoded).expect("embedded exact input");

    // Assert
    assert_eq!(restored.chain_work, fixture.chain_work);
}

#[test]
fn phase165_work_codec_embedded_oversized_shape_refuses() {
    // Arrange
    let record = format!(r#"{{"version":1,"kind":"{}","score":0}}"#, "x".repeat(512));

    // Act
    let result = serde_json::from_str::<ClassifiedWorkRecord>(&record);

    // Assert
    assert!(result.is_err());
}

#[test]
fn phase165_work_codec_accepts_byte_limit_boundary() {
    // Arrange
    let mut record = vec![b' '; 511];
    record.push(b'0');

    // Act
    let decoded = decode_work_record(StorageNamespace::Headers, &record).expect("bounded legacy");

    // Assert
    assert_eq!(decoded, ClassifiedWorkRecord::LegacySynthetic(0));
}

#[test]
fn phase165_work_codec_refuses_oversized_records_with_bounded_error() {
    // Arrange
    let mut oversized = vec![b' '; 1024];
    oversized.push(b'0');

    // Act
    let result = decode_work_record(StorageNamespace::Headers, &oversized);

    // Assert
    let crate::StorageError::Corruption {
        namespace,
        detail,
        action,
    } = result.expect_err("oversized record")
    else {
        panic!("bounded corruption category required");
    };
    assert_eq!(namespace, StorageNamespace::Headers);
    assert_eq!(action, crate::StorageRecoveryAction::Repair);
    assert!(detail.len() <= 64);
}
