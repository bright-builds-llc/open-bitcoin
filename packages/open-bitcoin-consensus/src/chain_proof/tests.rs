// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/arith_uint256.h

use super::{ProofTimeError, ProofWork, block_proof, proof_equivalent_time};
use crate::ConsensusParams;
use crate::crypto::CompactTargetError;

#[test]
fn phase160_proof_matches_independent_target_vectors() {
    // Arrange: independently evaluate floor(2^256 / (target + 1)) from chain.cpp.
    let vectors = [
        (0x207f_ffff, [2, 0, 0, 0]),
        (0x1d00_ffff, [0x1_0001_0001, 0, 0, 0]),
        (0x1d12_3456, [0xe10005d, 0, 0, 0]),
        (0x2100_ffff, [1, 0, 0, 0]),
        (0x0101_0000, [0, 0, 0, 0x8000_0000_0000_0000]),
        (0x0102_0000, [0x5555_5555_5555_5555; 4]),
        (0x0300_ffff, [0, 0, 0, 0x0001_0000_0000_0000]),
    ];
    for (bits, expected) in vectors {
        // Act
        let work = block_proof(bits).expect("positive compact target");
        // Assert
        assert_eq!(work, ProofWork(expected), "bits={bits:#x}");
        assert!(work > ProofWork::ZERO);
    }
}

#[test]
fn phase160_proof_rejects_invalid_compact_targets() {
    // Arrange
    let vectors = [
        (0, CompactTargetError::Zero),
        (0x0100_0001, CompactTargetError::Zero),
        (0x0200_0001, CompactTargetError::Zero),
        (0x1d80_ffff, CompactTargetError::Negative),
        (0x2301_0000, CompactTargetError::Overflow),
        (0x2200_0100, CompactTargetError::Overflow),
        (0x2101_0000, CompactTargetError::Overflow),
    ];
    for (bits, expected) in vectors {
        // Act
        let result = block_proof(bits);
        // Assert
        assert_eq!(result, Err(expected), "bits={bits:#x}");
    }
}

#[test]
fn phase160_proof_accumulation_carries_above_u128() {
    // Arrange
    let low = ProofWork([u64::MAX, u64::MAX, 4, 0]);
    let one = ProofWork([1, 0, 0, 0]);
    // Act
    let sum = low.wrapping_add(one);
    // Assert
    assert_eq!(sum, ProofWork([0, 0, 5, 0]));
}

#[test]
fn phase160_proof_accumulation_wraps_at_256_bits() {
    // Arrange
    let high = block_proof(0x0101_0000).expect("unit target");
    // Act
    let sum = high.wrapping_add(high);
    // Assert
    assert_eq!(sum, ProofWork::ZERO);
}

#[test]
fn phase160_proof_subtraction_borrows_and_wraps() {
    // Arrange
    let high = ProofWork([0, 0, 1, 0]);
    let one = ProofWork([1, 0, 0, 0]);
    // Act
    let difference = high.wrapping_sub(one);
    let wrapped = ProofWork::ZERO.wrapping_sub(one);
    // Assert
    assert_eq!(difference, ProofWork([u64::MAX, u64::MAX, 0, 0]));
    assert_eq!(wrapped, ProofWork([u64::MAX; 4]));
}

#[test]
fn phase160_proof_order_uses_most_significant_limb() {
    // Arrange
    let high = ProofWork([0, 0, 1, 0]);
    let low = ProofWork([u64::MAX, u64::MAX, 0, 0]);
    // Act
    let order = high.partial_cmp(&low);
    // Assert
    assert_eq!(order, Some(core::cmp::Ordering::Greater));
    assert_eq!(high.cmp(&high), core::cmp::Ordering::Equal);
}

#[test]
fn phase160_proof_time_signed_difference_matches_both_directions() {
    // Arrange
    let proof = block_proof(0x207f_ffff).expect("regtest target");
    let from = ProofWork([9, 1, 3, 0]);
    let to = from.wrapping_add(ProofWork([6, 0, 0, 0]));
    let spacing = ConsensusParams::default().pow_target_spacing_seconds;
    // Act
    let forward = proof_equivalent_time(to, from, proof, spacing);
    let backward = proof_equivalent_time(from, to, proof, spacing);
    // Assert
    assert_eq!(forward, Ok(1_800));
    assert_eq!(backward, Ok(-1_800));
}

#[test]
fn phase160_proof_time_equal_work_is_zero() {
    // Arrange
    let proof = block_proof(0x1d00_ffff).expect("mainnet target");
    // Act
    let age = proof_equivalent_time(proof, proof, proof, 600);
    // Assert
    assert_eq!(age, Ok(0));
}

#[test]
fn phase160_proof_time_nonuniform_difficulty_uses_best_proof_and_floors() {
    // Arrange: 3 * mainnet proof + 5 * proof(0x1d123456), independently derived.
    let cumulative = ProofWork([14_064_746_964, 0, 0, 0]);
    let mainnet = block_proof(0x1d00_ffff).expect("mainnet target");
    let other = block_proof(0x1d12_3456).expect("nonuniform target");
    // Act
    let mainnet_age = proof_equivalent_time(cumulative, ProofWork::ZERO, mainnet, 600);
    let other_age = proof_equivalent_time(cumulative, ProofWork::ZERO, other, 600);
    // Assert: floor(14_064_746_964 * 600 / denominator).
    assert_eq!(mainnet_age, Ok(1_964));
    assert_eq!(other_age, Ok(35_768));
}

#[test]
fn phase160_proof_time_spacing_is_supplied_by_consensus_params() {
    // Arrange
    let proof = block_proof(0x207f_ffff).expect("regtest target");
    let params = ConsensusParams {
        pow_target_spacing_seconds: 17,
        ..ConsensusParams::default()
    };
    // Act
    let age = proof_equivalent_time(
        proof,
        ProofWork::ZERO,
        proof,
        params.pow_target_spacing_seconds,
    );
    // Assert
    assert_eq!(age, Ok(17));
}

#[test]
fn phase160_proof_time_preserves_exact_thirty_day_boundaries() {
    // Arrange: two work units per second, at a one-second consensus spacing.
    let thirty_days = 30 * 24 * 60 * 60;
    let proof = block_proof(0x207f_ffff).expect("regtest target");
    for seconds in [thirty_days - 1, thirty_days, thirty_days + 1] {
        let to = ProofWork([seconds * 2, 0, 0, 0]);
        // Act
        let age = proof_equivalent_time(to, ProofWork::ZERO, proof, 1);
        // Assert
        assert_eq!(age, Ok(seconds as i64));
    }
}

#[test]
fn phase160_proof_time_modular_product_wraps_before_dividing() {
    // Arrange: proof(target=1)=2^255, so multiplying by 2 wraps to zero.
    let high = block_proof(0x0101_0000).expect("unit target");
    let best = block_proof(0x2100_ffff).expect("maximum compact target");
    // Act
    let age = proof_equivalent_time(high, ProofWork::ZERO, best, 2);
    // Assert
    assert_eq!(age, Ok(0));
}

#[test]
fn phase160_proof_time_modular_product_preserves_low_remainder() {
    // Arrange: (2^255 + 2) * 3 mod 2^256 = 2^255 + 6.
    let best = block_proof(0x0101_0000).expect("unit target");
    let to = best.wrapping_add(ProofWork([2, 0, 0, 0]));
    // Act
    let age = proof_equivalent_time(to, ProofWork::ZERO, best, 3);
    // Assert
    assert_eq!(age, Ok(1));
}

#[test]
fn phase160_proof_time_saturates_both_signs_at_int64_max() {
    // Arrange
    let best = block_proof(0x2100_ffff).expect("one proof unit");
    for high in [ProofWork([1 << 63, 0, 0, 0]), ProofWork([0, 1, 0, 0])] {
        // Act
        let forward = proof_equivalent_time(high, ProofWork::ZERO, best, 1);
        let backward = proof_equivalent_time(ProofWork::ZERO, high, best, 1);
        // Assert
        assert_eq!(forward, Ok(i64::MAX));
        assert_eq!(backward, Ok(-i64::MAX));
    }
}

#[test]
fn phase160_proof_time_retains_int64_max_without_overflow() {
    // Arrange
    let to = ProofWork([i64::MAX as u64, 0, 0, 0]);
    let best = block_proof(0x2100_ffff).expect("one proof unit");
    // Act
    let age = proof_equivalent_time(to, ProofWork::ZERO, best, 1);
    // Assert
    assert_eq!(age, Ok(i64::MAX));
}

#[test]
fn phase160_proof_time_rejects_zero_denominator() {
    // Arrange
    let to = block_proof(0x207f_ffff).expect("regtest target");
    // Act
    let age = proof_equivalent_time(to, ProofWork::ZERO, ProofWork::ZERO, 600);
    // Assert
    assert_eq!(age, Err(ProofTimeError::ZeroBestProof));
}

#[test]
fn phase160_proof_time_rejects_unusable_spacing() {
    // Arrange
    let proof = block_proof(0x207f_ffff).expect("regtest target");
    for spacing in [0, -1, i64::MIN] {
        // Act
        let age = proof_equivalent_time(proof, ProofWork::ZERO, proof, spacing);
        // Assert
        assert_eq!(age, Err(ProofTimeError::InvalidSpacing));
    }
}

#[test]
fn phase160_proof_time_errors_explain_unusable_inputs() {
    // Arrange
    let errors = [
        ProofTimeError::ZeroBestProof,
        ProofTimeError::InvalidSpacing,
    ];
    // Act
    let descriptions = errors.map(|error| error.to_string());
    // Assert
    assert_eq!(
        descriptions,
        [
            "best header proof must be nonzero",
            "target spacing must be positive"
        ]
    );
}

#[test]
fn phase160_proof_time_floors_magnitude_before_applying_negative_sign() {
    // Arrange: 5 / 2 yields 2 and -2 rather than rounding to the nearest second.
    let work = ProofWork([5, 0, 0, 0]);
    let best = block_proof(0x207f_ffff).expect("regtest target");
    // Act
    let forward = proof_equivalent_time(work, ProofWork::ZERO, best, 1);
    let backward = proof_equivalent_time(ProofWork::ZERO, work, best, 1);
    // Assert
    assert_eq!(forward, Ok(2));
    assert_eq!(backward, Ok(-2));
}

#[test]
fn phase160_proof_time_accepts_maximum_positive_spacing() {
    // Arrange
    let best = block_proof(0x2100_ffff).expect("one proof unit");
    // Act
    let age = proof_equivalent_time(best, ProofWork::ZERO, best, i64::MAX);
    // Assert
    assert_eq!(age, Ok(i64::MAX));
}

// The bounded generator validates this canonical JSON schema independently.
// Keep fixture reading test-only and dependency-free in the pure-core crate.
fn pinned_proof_cases() -> Vec<&'static str> {
    let fixture = include_str!("../../../../docs/parity/fixtures/peer-filter-proof.json");
    assert!(fixture.contains("a9aee730466ac67d35a3c03ee24676be5e045878"));
    let cases: Vec<_> = fixture.split("      \"name\": ").skip(1).collect();
    assert_eq!(cases.len(), 27, "all independently generated proof cases");
    cases
}

fn fixture_field<'a>(section: &'a str, key: &str) -> &'a str {
    let marker = format!("\"{key}\": ");
    let value = section
        .split_once(&marker)
        .expect("canonical fixture field")
        .1
        .lines()
        .next()
        .expect("canonical fixture scalar");
    value.trim_end_matches(',').trim_matches('"')
}

fn decimal_proof(value: &str) -> ProofWork {
    let mut limbs = [0_u64; 4];
    assert!(!value.is_empty());
    for digit in value.bytes() {
        assert!(digit.is_ascii_digit());
        let mut carry = u128::from(digit - b'0');
        for limb in &mut limbs {
            let expanded = u128::from(*limb) * 10 + carry;
            *limb = expanded as u64;
            carry = expanded >> 64;
        }
        assert_eq!(carry, 0, "fixture work fits exactly in 256 bits");
    }
    ProofWork(limbs)
}

#[test]
fn phase160_peer_evidence_pinned_proof_goldens() {
    for case in pinned_proof_cases() {
        // Arrange: expected decimal work comes from pinned independent formulas.
        let (input, expected) = case
            .split_once("\"expected\": {")
            .expect("canonical fixture sections");
        let bits = fixture_field(input, "bits")
            .parse()
            .expect("fixture compact target");
        let expected_proof = decimal_proof(fixture_field(expected, "proof"));

        // Act
        let actual = block_proof(bits);

        // Assert: this API explicitly rejects the targets Knots maps to zero work.
        if expected_proof == ProofWork::ZERO {
            assert!(actual.is_err(), "{case}");
        } else {
            assert_eq!(actual, Ok(expected_proof), "{case}");
        }
    }
}

#[test]
fn phase160_peer_evidence_pinned_equivalent_time_goldens() {
    let mut checked = 0;
    for case in pinned_proof_cases() {
        // Arrange: lossless signed expectations include wrap and saturation cases.
        let (input, expected) = case
            .split_once("\"expected\": {")
            .expect("canonical fixture sections");
        let expected_time = fixture_field(expected, "equivalent_time");
        if expected_time == "null" {
            continue;
        }
        let to = decimal_proof(fixture_field(input, "to_work"));
        let from = decimal_proof(fixture_field(input, "from_work"));
        let proof = decimal_proof(fixture_field(expected, "proof"));
        let spacing = fixture_field(input, "spacing")
            .parse()
            .expect("fixture positive spacing");
        let expected_time = expected_time.parse().expect("fixture signed time");

        // Act
        let actual = proof_equivalent_time(to, from, proof, spacing);

        // Assert
        assert_eq!(actual, Ok(expected_time), "{case}");
        checked += 1;
    }
    assert_eq!(checked, 23, "all defined independent time expectations");
}
