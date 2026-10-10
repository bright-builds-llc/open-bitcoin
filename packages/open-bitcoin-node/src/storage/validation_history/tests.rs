// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

fn identity() -> BlockValidationIdentity {
    BlockValidationIdentity::new(
        BlockHash::from_byte_array([1; 32]),
        BlockHash::from_byte_array([0; 32]),
        0,
    )
    .expect("valid genesis identity")
}

#[test]
fn phase159_validation_history_codec_roundtrip_binds_raw_identity() {
    // Arrange
    let record = ValidationHistoryRecord::new(identity(), StoredValidationStatus::KnownOnly);
    let key = record_key(identity().hash());
    // Act
    let decoded = decode_record(&key, &encode_record(record)).expect("valid envelope");
    // Assert
    assert_eq!(decoded, record);
    assert_eq!(decoded.identity().hash().as_bytes(), &[1; 32]);
}

#[test]
fn phase159_validation_history_codec_rejects_malformed_envelopes() {
    // Arrange
    let record = ValidationHistoryRecord::new(identity(), StoredValidationStatus::ScriptsValid);
    let key = record_key(identity().hash());
    let bytes = encode_record(record);
    // Act / Assert
    for (offset, value) in [(0, 2), (1, 3), (6, 9)] {
        let mut malformed = bytes.clone();
        malformed[offset] = value;
        assert!(decode_record(&key, &malformed).is_err());
    }
    assert!(decode_record(&key, &bytes[..bytes.len() - 1]).is_err());
    assert!(decode_record("validated_block:v2:00", &bytes).is_err());
    assert!(decode_record(&key.to_uppercase(), &bytes).is_err());
}

#[test]
fn phase159_validation_history_codec_allows_only_monotonic_same_identity() {
    // Arrange
    let known = ValidationHistoryRecord::new(identity(), StoredValidationStatus::KnownOnly);
    let valid = ValidationHistoryRecord::new(identity(), StoredValidationStatus::ScriptsValid);
    let conflicting = ValidationHistoryRecord::new(
        BlockValidationIdentity::new(identity().hash(), BlockHash::from_byte_array([2; 32]), 1)
            .expect("valid non-genesis identity"),
        StoredValidationStatus::ScriptsValid,
    );
    // Act / Assert
    assert!(validate_upgrade(Some(known), valid).is_ok());
    assert!(validate_upgrade(Some(valid), valid).is_ok());
    assert!(validate_upgrade(Some(valid), known).is_err());
    assert!(validate_upgrade(Some(valid), conflicting).is_err());
}

#[test]
fn phase159_validation_history_codec_coverage_keeps_legacy_absence_unknown() {
    // Arrange / Act / Assert
    assert_eq!(
        ValidationCoverage::UnknownLegacy.absent_provenance(),
        ValidationProvenance::UnknownLegacy
    );
    assert_eq!(
        ValidationCoverage::Complete.absent_provenance(),
        ValidationProvenance::NeverConnected
    );
    for coverage in [
        ValidationCoverage::Complete,
        ValidationCoverage::UnknownLegacy,
    ] {
        assert_eq!(
            decode_coverage(&encode_coverage(coverage)).expect("coverage"),
            coverage
        );
    }
    assert!(decode_coverage(&[2, 1]).is_err());
    assert!(decode_coverage(&[1, 3]).is_err());
    assert!(decode_coverage(&[1, 1, 0]).is_err());
}

#[test]
fn phase159_validation_history_codec_checks_identity_and_batch_bounds() {
    // Arrange / Act / Assert
    assert!(
        BlockValidationIdentity::new(
            BlockHash::from_byte_array([0; 32]),
            BlockHash::from_byte_array([0; 32]),
            0
        )
        .is_err()
    );
    assert!(BlockValidationIdentity::new(identity().hash(), identity().hash(), 1).is_err());
    assert!(
        BlockValidationIdentity::new(identity().hash(), BlockHash::from_byte_array([0; 32]), 1)
            .is_err()
    );
    assert!(
        BlockValidationIdentity::new(identity().hash(), BlockHash::from_byte_array([2; 32]), 0)
            .is_err()
    );
    assert!(validate_batch_size(1).is_ok());
    assert!(validate_batch_size(MAX_VALIDATION_IDENTITIES + 1).is_err());
    assert!(validate_batch_size(usize::MAX).is_err());
}
