// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/util/golombrice.h
// - packages/bitcoin-knots/src/streams.h
// - packages/bitcoin-knots/src/serialize.h

use super::*;

#[test]
fn empty_values_have_only_zero_count() {
    assert_eq!(encode_basic_filter_values(&[]), Ok(vec![0]));
}

#[test]
fn zero_deltas_preserve_mapped_collisions() {
    // Arrange
    let values = [0, 0];
    // Act
    let encoded = encode_basic_filter_values(&values).expect("bounded values");
    // Assert: 40 zero bits following N=2.
    assert_eq!(encoded, [2, 0, 0, 0, 0, 0]);
}

#[test]
fn rice_bits_are_msb_first_with_zero_padding() {
    // Arrange: q=1 then stop bit, remainder=1 in 19 bits.
    let values = [(1 << 19) + 1];
    // Act
    let encoded = encode_basic_filter_values(&values).expect("bounded values");
    // Assert: 10, 18 zeros, 1, three zero padding bits.
    assert_eq!(encoded, [1, 0x80, 0, 0x08]);
}

#[test]
fn compact_size_count_changes_at_253() {
    // Arrange
    let cases = [(252, vec![252]), (253, vec![253, 253, 0])];
    for (count, prefix) in cases {
        // Act
        let encoded = encode_basic_filter_values(&vec![0; count]).expect("bounded values");
        // Assert
        assert_eq!(&encoded[..prefix.len()], prefix);
        assert_eq!(encoded.len(), prefix.len() + (count * 20).div_ceil(8));
        assert!(encoded[prefix.len()..].iter().all(|byte| *byte == 0));
    }
}

#[test]
fn unsorted_values_refuse_before_encoding() {
    assert_eq!(
        encode_basic_filter_values(&[1, 0]),
        Err(BasicFilterEncodingError::Unsorted)
    );
}

#[test]
fn value_at_range_limit_refuses_before_unary_work() {
    assert_eq!(
        encode_basic_filter_values(&[BASIC_FILTER_M]),
        Err(BasicFilterEncodingError::OutOfRange {
            value: BASIC_FILTER_M,
            range: BASIC_FILTER_M
        })
    );
    assert!(encode_basic_filter_values(&[u64::MAX]).is_err());
}

#[test]
fn count_bound_refuses_overflowing_ranges() {
    assert_eq!(basic_filter_range(MAX_SIZE), Ok(MAX_SIZE * BASIC_FILTER_M));
    assert_eq!(
        basic_filter_range(MAX_SIZE + 1),
        Err(BasicFilterEncodingError::CountTooLarge(MAX_SIZE + 1))
    );
    assert_eq!(
        basic_filter_range(u64::MAX),
        Err(BasicFilterEncodingError::CountTooLarge(u64::MAX))
    );
}

#[test]
fn output_accounting_refuses_excess_and_overflow() {
    assert_eq!(checked_output_size(65_536, 0), Ok(5));
    assert_eq!(
        checked_output_size(0, MAX_SIZE * 8),
        Err(BasicFilterEncodingError::OutputTooLarge)
    );
    assert_eq!(
        checked_output_size(0, u64::MAX),
        Err(BasicFilterEncodingError::OutputTooLarge)
    );
}

#[test]
fn output_reservation_refuses_unrepresentable_allocation() {
    assert_eq!(
        reserve_output(&mut Vec::new(), usize::MAX),
        Err(BasicFilterEncodingError::AllocationFailed)
    );
}

#[test]
fn encoding_errors_describe_each_failure() {
    // Arrange
    let cases = [
        (
            BasicFilterEncodingError::CountTooLarge(4),
            "BASIC element count exceeds MAX_SIZE: 4",
        ),
        (
            BasicFilterEncodingError::Unsorted,
            "BASIC mapped values must be sorted",
        ),
        (
            BasicFilterEncodingError::OutOfRange { value: 7, range: 6 },
            "BASIC mapped value 7 is outside range 6",
        ),
        (
            BasicFilterEncodingError::OutputTooLarge,
            "BASIC encoded output exceeds MAX_SIZE",
        ),
        (
            BasicFilterEncodingError::AllocationFailed,
            "BASIC output allocation failed",
        ),
    ];
    for (error, expected) in cases {
        // Act / Assert
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn codec_error_conversion_preserves_failure() {
    // Arrange
    let source = CodecError::CompactSizeTooLarge(MAX_SIZE + 1);
    // Act
    let error = BasicFilterEncodingError::from(source.clone());
    // Assert
    assert_eq!(error.to_string(), source.to_string());
    assert_eq!(error, BasicFilterEncodingError::Codec(source));
}
