// Parity breadcrumbs:
// - packages/bitcoin-knots/src/arith_uint256.cpp
// - packages/bitcoin-knots/src/pow.cpp

use super::{
    CompactTargetError, check_proof_of_work, check_proof_of_work_with_params, compact_target_bytes,
};
use crate::ConsensusParams;
use crate::chain_proof::block_proof;

#[test]
fn phase165_normalized_zero_is_rejected_by_decode_proof_and_hash_checks() {
    // Arrange: SetCompact shifts nWord before deriving its sign flag.
    for bits in [
        0,
        0x0180_0000,
        0x0100_0001,
        0x0200_0001,
        0x0180_0001,
        0x0280_0001,
        0x0080_ffff,
        0xff80_0000,
    ] {
        // Act
        let decoded = compact_target_bytes(bits);
        let proof = block_proof(bits);
        let admitted = check_proof_of_work([0; 32], bits);
        // Assert
        assert_eq!(decoded, Err(CompactTargetError::Zero), "{bits:#x}");
        assert_eq!(proof, Err(CompactTargetError::Zero), "{bits:#x}");
        assert_eq!(admitted, Err(CompactTargetError::Zero), "{bits:#x}");
    }
}

#[test]
fn phase165_equivalent_compact_encodings_have_the_same_target() {
    // Arrange: these encodings all normalize to target 1.
    let mut expected = [0; 32];
    expected[0] = 1;
    for bits in [0x0101_0000, 0x0101_ffff, 0x0200_0100, 0x0300_0001] {
        // Act
        let target = compact_target_bytes(bits);
        // Assert
        assert_eq!(target, Ok(expected));
    }
}

#[test]
fn phase165_compact_sign_and_overflow_boundaries_match_setcompact() {
    // Arrange
    let invalid = [
        (0x0181_0000, CompactTargetError::Negative),
        (0x0280_0100, CompactTargetError::Negative),
        (0x1d80_ffff, CompactTargetError::Negative),
        (0x2300_0001, CompactTargetError::Overflow),
        (0x2200_0100, CompactTargetError::Overflow),
        (0x2101_0000, CompactTargetError::Overflow),
    ];
    for (bits, expected) in invalid {
        // Act
        let decoded = compact_target_bytes(bits);
        let proof = block_proof(bits);
        let admission = check_proof_of_work_with_params([0; 32], bits, &ConsensusParams::default());
        // Assert
        assert_eq!(decoded, Err(expected));
        assert_eq!(proof, Err(expected));
        assert_eq!(admission, Err(expected));
    }
    for bits in [0x207f_ffff, 0x2100_ffff, 0x2200_00ff] {
        // Act
        let decoded = compact_target_bytes(bits);
        // Assert
        assert!(decoded.is_ok(), "{bits:#x}");
    }
}

#[test]
fn phase165_selected_pow_limit_accepts_equality_and_refuses_larger_targets() {
    // Arrange: target and powLimit both equal 1; target 2 is still arithmetically valid.
    let params = ConsensusParams {
        pow_limit_bits: 0x0101_0000,
        ..ConsensusParams::default()
    };
    let mut equal_hash = [0; 32];
    equal_hash[0] = 1;
    let mut greater_hash = equal_hash;
    greater_hash[0] = 2;
    // Act
    let equality = check_proof_of_work_with_params(equal_hash, 0x0101_0000, &params);
    let excessive_hash = check_proof_of_work_with_params(greater_hash, 0x0101_0000, &params);
    let excessive_target = check_proof_of_work_with_params([0; 32], 0x0102_0000, &params);
    // Assert
    assert_eq!(equality, Ok(true));
    assert_eq!(excessive_hash, Ok(false));
    assert_eq!(excessive_target, Ok(false));
}

#[test]
fn phase165_selected_pow_limit_accepts_a_stricter_target() {
    // Arrange
    let params = ConsensusParams {
        pow_limit_bits: 0x0102_0000,
        ..ConsensusParams::default()
    };
    // Act
    let result = check_proof_of_work_with_params([0; 32], 0x0101_0000, &params);
    // Assert
    assert_eq!(result, Ok(true));
}

#[test]
fn phase165_selected_pow_limit_rejects_invalid_targets_and_invalid_configuration() {
    // Arrange
    let params = ConsensusParams::default();
    let invalid_params = ConsensusParams {
        pow_limit_bits: 0,
        ..params
    };
    // Act
    let zero = check_proof_of_work_with_params([0; 32], 0x0180_0001, &params);
    let invalid_limit = check_proof_of_work_with_params([0; 32], 0x0101_0000, &invalid_params);
    // Assert
    assert_eq!(zero, Err(CompactTargetError::Zero));
    assert_eq!(invalid_limit, Err(CompactTargetError::Zero));
}

#[test]
fn phase165_selected_header_admission_uses_the_selected_pow_limit() {
    // Arrange
    let header = open_bitcoin_codec::parse_block_header(&decode_genesis()).expect("genesis header");
    let params = ConsensusParams {
        pow_limit_bits: 0x1d00_ffff,
        ..ConsensusParams::default()
    };
    let restricted = ConsensusParams {
        pow_limit_bits: 0x0101_0000,
        ..params
    };
    // Act
    let valid = crate::block::check_block_header_with_params(&header, &params);
    let refused = crate::block::check_block_header_with_params(&header, &restricted);
    // Assert
    assert!(valid.is_ok());
    assert_eq!(
        refused.expect_err("target above limit").reject_reason,
        "high-hash"
    );
}

#[test]
fn phase165_selected_header_admission_preserves_invalid_target_error() {
    // Arrange
    let mut header =
        open_bitcoin_codec::parse_block_header(&decode_genesis()).expect("genesis header");
    header.bits = 0x0180_0001;
    // Act
    let result = crate::block::check_block_header_with_params(&header, &ConsensusParams::default());
    // Assert
    assert_eq!(
        result.expect_err("normalized zero").reject_reason,
        "bad-diffbits"
    );
}

fn decode_genesis() -> Vec<u8> {
    include_str!("../../../../open-bitcoin-codec/testdata/block_header.hex")
        .trim()
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(core::str::from_utf8(pair).expect("ASCII hex"), 16)
                .expect("hex byte")
        })
        .collect()
}
