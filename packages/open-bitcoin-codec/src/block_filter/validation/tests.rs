// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/util/golombrice.h
// - packages/bitcoin-knots/src/streams.h
// - packages/bitcoin-knots/src/serialize.h

use super::*;
use crate::block_filter::encode_basic_filter_values;

#[test]
fn validates_encoder_round_trips_and_collisions() {
    // Arrange
    let vectors = [vec![], vec![0], vec![0, 0], vec![7, 19], vec![784_930]];
    for values in vectors {
        let bytes = encode_basic_filter_values(&values).expect("valid values");
        // Act / Assert
        assert_eq!(validate_basic_filter_encoding(&bytes), Ok(()));
    }
}

#[test]
fn rejects_malformed_counts() {
    // Arrange
    let cases: &[&[u8]] = &[
        &[],
        &[0xfd],
        &[0xfd, 1, 0],
        &[0xfe, 1, 0, 0, 0],
        &[0xff, 1, 0, 0, 0, 0, 0, 0, 0],
        &[0xfe, 1, 0, 0, 2],
    ];
    for bytes in cases {
        // Act / Assert
        assert!(matches!(
            validate_basic_filter_encoding(bytes),
            Err(BasicFilterValidationError::Codec(_))
        ));
    }
}

#[test]
fn rejects_huge_count_before_streaming() {
    // Arrange
    let bytes = [0xfe, 0, 0, 0, 2];
    // Act / Assert
    assert_eq!(
        validate_basic_filter_encoding(&bytes),
        Err(BasicFilterValidationError::InsufficientBits)
    );
}

#[test]
fn rejects_oversized_byte_slice() {
    // Arrange
    let bytes = vec![0; crate::MAX_SIZE as usize + 1];
    // Act / Assert
    assert_eq!(
        validate_basic_filter_encoding(&bytes),
        Err(BasicFilterValidationError::BytesTooLarge)
    );
}

#[test]
fn rejects_missing_minimum_bits() {
    assert_eq!(
        validate_basic_filter_encoding(&[1, 0]),
        Err(BasicFilterValidationError::InsufficientBits)
    );
}

#[test]
fn rejects_truncated_unary() {
    assert_eq!(
        validate_basic_filter_encoding(&[1, 255, 255, 255]),
        Err(BasicFilterValidationError::TruncatedBits)
    );
}

#[test]
fn rejects_truncated_remainder() {
    assert_eq!(
        validate_basic_filter_encoding(&[1, 0xf8, 0, 0]),
        Err(BasicFilterValidationError::TruncatedBits)
    );
}

#[test]
fn rejects_out_of_range_value() {
    assert_eq!(
        validate_basic_filter_encoding(&[1, 0xff, 0, 0, 0, 0]),
        Err(BasicFilterValidationError::ValueOutOfRange)
    );
}

#[test]
fn rejects_nonzero_padding() {
    assert_eq!(
        validate_basic_filter_encoding(&[1, 0, 0, 1]),
        Err(BasicFilterValidationError::NonzeroPadding)
    );
}

#[test]
fn rejects_trailing_bytes_including_empty_filter() {
    // Arrange
    for bytes in [&[0, 0][..], &[1, 0, 0, 0, 0][..]] {
        // Act / Assert
        assert_eq!(
            validate_basic_filter_encoding(bytes),
            Err(BasicFilterValidationError::TrailingBytes)
        );
    }
}

#[test]
fn rejects_arithmetic_overflow_before_value_use() {
    // Arrange / Act / Assert
    assert_eq!(
        accumulate_delta(0, u64::MAX, 0),
        Err(BasicFilterValidationError::ArithmeticOverflow)
    );
    assert_eq!(
        accumulate_delta(u64::MAX, 0, 1),
        Err(BasicFilterValidationError::ArithmeticOverflow)
    );
    assert_eq!(
        accumulate_delta(0, u64::MAX >> 19, 1 << 19),
        Err(BasicFilterValidationError::ArithmeticOverflow)
    );
}

#[test]
fn validation_errors_describe_each_refusal() {
    // Arrange
    let errors = [
        BasicFilterValidationError::Codec(crate::CodecError::CompactSizeTooLarge(
            crate::MAX_SIZE + 1,
        )),
        BasicFilterValidationError::BytesTooLarge,
        BasicFilterValidationError::InsufficientBits,
        BasicFilterValidationError::TruncatedBits,
        BasicFilterValidationError::ArithmeticOverflow,
        BasicFilterValidationError::ValueOutOfRange,
        BasicFilterValidationError::NonzeroPadding,
        BasicFilterValidationError::TrailingBytes,
    ];
    // Act / Assert
    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}
