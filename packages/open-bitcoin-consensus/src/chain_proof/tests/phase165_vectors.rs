// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/arith_uint256.h

use super::super::{ProofWork, ProofWorkParseError, block_proof};
use crate::crypto::compact_target_bytes;

const FIXTURE: &str =
    include_str!("../../../../../docs/parity/fixtures/phase165-work-vectors.json");

fn cases(section: &str, next_section: &str) -> Vec<&'static str> {
    assert!(FIXTURE.contains("a9aee730466ac67d35a3c03ee24676be5e045878"));
    assert!(FIXTURE.contains("source-derived-arithmetic"));
    FIXTURE
        .split_once(section)
        .expect("fixture section")
        .1
        .split_once(next_section)
        .expect("section end")
        .0
        .split("      \"name\": ")
        .skip(1)
        .collect()
}

fn field<'a>(case: &'a str, key: &str) -> &'a str {
    super::fixture_field(case, key)
}

fn bits(case: &str) -> u32 {
    u32::from_str_radix(field(case, "bits_hex"), 16).expect("compact hex")
}

#[test]
fn phase165_fixed_proof_corpus_matches_independent_wide_expectations() {
    // Arrange: source-derived fixed corpus, not values produced by block_proof.
    let vectors = cases("\"proof_vectors\": [", "\"addition_vectors\": [");
    assert_eq!(vectors.len(), 11);
    for case in vectors {
        // Act
        let proof = block_proof(bits(case)).expect("positive target");
        let target = compact_target_bytes(bits(case)).expect("positive target");
        // Assert
        assert_eq!(proof.to_hex(), field(case, "proof_hex"));
        assert_eq!(
            ProofWork::from_le_bytes(target).to_hex(),
            field(case, "target_hex")
        );
    }
}

#[test]
fn phase165_fixed_accumulation_corpus_carries_and_wraps() {
    // Arrange
    let vectors = cases("\"addition_vectors\": [", "\"invalid_vectors\": [");
    assert_eq!(vectors.len(), 6);
    for case in vectors.iter().take(5) {
        let left = ProofWork::from_hex(field(case, "left_hex")).expect("left work");
        let right = ProofWork::from_hex(field(case, "right_hex")).expect("right work");
        // Act
        let sum = left.wrapping_add(right);
        // Assert
        assert_eq!(sum.to_hex(), field(case, "sum_hex"));
    }
}

#[test]
fn phase165_fixed_mixed_target_accumulation_keeps_all_limbs() {
    // Arrange: last addition case contains the independently derived mixed total.
    let vectors = cases("\"addition_vectors\": [", "\"invalid_vectors\": [");
    let case = vectors.last().expect("mixed target sum");
    let inputs = [0x0101_0000, 0x0102_0000, 0x207f_ffff, 0x1d00_ffff];
    // Act
    let sum = inputs.into_iter().fold(ProofWork::ZERO, |sum, bits| {
        sum.wrapping_add(block_proof(bits).expect("positive target"))
    });
    // Assert
    assert_eq!(sum.to_hex(), field(case, "sum_hex"));
}

#[test]
fn phase165_fixed_invalid_corpus_maps_knots_zero_work_to_errors() {
    // Arrange
    let vectors = cases("\"invalid_vectors\": [", "\n  ]");
    assert_eq!(vectors.len(), 14);
    for case in vectors {
        // Act
        let proof = block_proof(bits(case));
        // Assert
        assert_eq!(
            format!("{:?}", proof.expect_err("invalid target")),
            field(case, "error")
        );
    }
}

#[test]
fn phase165_work_bytes_and_hex_preserve_every_bit_and_endianness() {
    // Arrange: each byte is distinct, including the unsigned high bit.
    let bytes = core::array::from_fn(|index| (index as u8) * 8 + 7);
    let expected = "fff7efe7dfd7cfc7bfb7afa79f978f877f776f675f574f473f372f271f170f07";
    // Act
    let work = ProofWork::from_le_bytes(bytes);
    let hex = work.to_hex();
    let decoded = ProofWork::from_hex(expected).expect("fixed hex");
    // Assert
    assert_eq!(hex, expected);
    assert_eq!(work.to_le_bytes(), bytes);
    assert_eq!(decoded.to_le_bytes(), bytes);
    assert_eq!(work.to_string(), expected);
    assert_eq!(expected.parse::<ProofWork>(), Ok(work));
}

#[test]
fn phase165_work_hex_accepts_either_case_and_zero() {
    // Arrange
    let uppercase = "ABCDEF0123456789".repeat(4);
    // Act
    let work = ProofWork::from_hex(&uppercase).expect("uppercase hex");
    let zero = ProofWork::from_hex(&"0".repeat(64));
    // Assert
    assert_eq!(work.to_hex(), uppercase.to_ascii_lowercase());
    assert_eq!(zero, Ok(ProofWork::ZERO));
}

#[test]
fn phase165_work_hex_refuses_non_fixed_width_and_unsigned_forms() {
    // Arrange
    let inputs = [
        String::new(),
        "0".repeat(63),
        "0".repeat(65),
        format!("0x{}", "0".repeat(64)),
    ];
    for input in inputs {
        // Act
        let result = ProofWork::from_hex(&input);
        // Assert
        assert_eq!(result, Err(ProofWorkParseError::InvalidLength));
    }
}

#[test]
fn phase165_work_hex_refuses_non_ascii_signs_whitespace_and_non_hex_digits() {
    // Arrange
    for input in [
        format!("g{}", "0".repeat(63)),
        format!("+{}", "0".repeat(63)),
        format!("-{}", "0".repeat(63)),
        format!(" {}", "0".repeat(63)),
        "é".repeat(32),
    ] {
        // Act
        let result = ProofWork::from_hex(&input);
        // Assert
        assert_eq!(result, Err(ProofWorkParseError::InvalidDigit));
    }
}

#[test]
fn phase165_work_order_compares_each_unsigned_high_limb() {
    // Arrange
    for limb in 0..4 {
        let mut high = [0; 32];
        high[limb * 8 + 7] = 0x80;
        let mut low = [0xff; 32];
        low[limb * 8 + 7..].fill(0);
        low[limb * 8 + 7] = 0x7f;
        let high = ProofWork::from_le_bytes(high);
        let low = ProofWork::from_le_bytes(low);
        // Act
        let order = high.cmp(&low);
        // Assert
        assert_eq!(order, core::cmp::Ordering::Greater);
        assert!(low < high);
    }
}

#[test]
fn phase165_work_parse_errors_describe_the_refusal() {
    // Arrange
    let errors = [
        ProofWorkParseError::InvalidLength,
        ProofWorkParseError::InvalidDigit,
    ];
    // Act
    let descriptions = errors.map(|error| error.to_string());
    // Assert
    assert_eq!(
        descriptions,
        [
            "work hex must contain exactly 64 digits",
            "work hex contains a non-hexadecimal digit"
        ]
    );
}
