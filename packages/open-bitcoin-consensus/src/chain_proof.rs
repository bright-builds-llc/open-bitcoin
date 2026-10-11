// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/arith_uint256.h

use crate::crypto::{CompactTargetError, compact_target_bytes};

/// Exact unsigned 256-bit work, without any assertion of chain validation provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofWork([u64; 4]);

impl ProofWork {
    /// The initial cumulative work before any headers are accumulated.
    pub const ZERO: Self = Self([0; 4]);

    /// Accumulate work modulo 2^256, matching Knots `arith_uint256`.
    pub fn wrapping_add(self, other: Self) -> Self {
        let mut limbs = [0; 4];
        let mut carry = false;
        for (index, limb) in limbs.iter_mut().enumerate() {
            let (sum, first_carry) = self.0[index].overflowing_add(other.0[index]);
            let (sum, second_carry) = sum.overflowing_add(u64::from(carry));
            *limb = sum;
            carry = first_carry || second_carry;
        }
        Self(limbs)
    }

    /// Subtract work modulo 2^256, matching Knots `arith_uint256`.
    pub fn wrapping_sub(self, other: Self) -> Self {
        let mut limbs = [0; 4];
        let mut borrow = false;
        for (index, limb) in limbs.iter_mut().enumerate() {
            let (difference, first_borrow) = self.0[index].overflowing_sub(other.0[index]);
            let (difference, second_borrow) = difference.overflowing_sub(u64::from(borrow));
            *limb = difference;
            borrow = first_borrow || second_borrow;
        }
        Self(limbs)
    }

    fn shift_left_one(self) -> Self {
        let mut limbs = [0; 4];
        let mut carry = 0;
        for (index, limb) in limbs.iter_mut().enumerate() {
            *limb = (self.0[index] << 1) | carry;
            carry = self.0[index] >> 63;
        }
        Self(limbs)
    }

    fn wrapping_multiply_u64(self, factor: u64) -> Self {
        let mut product = Self::ZERO;
        let mut multiplicand = self;
        for bit in 0..64 {
            if factor & (1_u64 << bit) != 0 {
                product = product.wrapping_add(multiplicand);
            }
            multiplicand = multiplicand.shift_left_one();
        }
        product
    }

    // Callers establish nonzero divisors. Restoring division always takes 256 steps.
    fn divide_nonzero(self, divisor: Self) -> Self {
        let mut quotient = Self::ZERO;
        let mut remainder = Self::ZERO;
        for bit in (0..256).rev() {
            // Each remainder fits the consumed numerator prefix (at most 256 bits).
            remainder = remainder.shift_left_one();
            remainder.0[0] |= (self.0[bit / 64] >> (bit % 64)) & 1;
            if remainder >= divisor {
                remainder = remainder.wrapping_sub(divisor);
                quotient.0[bit / 64] |= 1_u64 << (bit % 64);
            }
        }
        quotient
    }
}

impl Ord for ProofWork {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.iter().rev().cmp(other.0.iter().rev())
    }
}

impl PartialOrd for ProofWork {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Derive exact work from a positive, nonzero compact target; no chain validity is implied.
pub fn block_proof(bits: u32) -> Result<ProofWork, CompactTargetError> {
    let bytes = compact_target_bytes(bits)?;
    let mut limbs = [0; 4];
    for (index, chunk) in bytes.chunks_exact(8).enumerate() {
        let mut limb = [0; 8];
        limb.copy_from_slice(chunk);
        limbs[index] = u64::from_le_bytes(limb);
    }
    let target = ProofWork(limbs);
    if target == ProofWork::ZERO {
        return Err(CompactTargetError::Zero);
    }
    let one = ProofWork([1, 0, 0, 0]);
    let numerator = ProofWork(target.0.map(|limb| !limb));
    // A compact target cannot encode 2^256 - 1, so target + 1 is nonzero.
    Ok(numerator
        .divide_nonzero(target.wrapping_add(one))
        .wrapping_add(one))
}

/// Invalid inputs to proof-equivalent time; no clock or validation provenance is inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofTimeError {
    /// The best header must supply a positive target-derived block proof.
    ZeroBestProof,
    /// Consensus target spacing must be positive.
    InvalidSpacing,
}

impl core::fmt::Display for ProofTimeError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ZeroBestProof => formatter.write_str("best header proof must be nonzero"),
            Self::InvalidSpacing => formatter.write_str("target spacing must be positive"),
        }
    }
}

impl std::error::Error for ProofTimeError {}

/// Compute signed equivalent seconds using the best header's proof and consensus spacing.
///
/// Use `ConsensusParams::pow_target_spacing_seconds` for spacing. Multiplication wraps
/// modulo 2^256 before floor division, and magnitudes above 63 bits clamp to `i64::MAX`.
/// Callers separately establish header/branch validation provenance.
pub fn proof_equivalent_time(
    to: ProofWork,
    from: ProofWork,
    best_proof: ProofWork,
    target_spacing_seconds: i64,
) -> Result<i64, ProofTimeError> {
    if target_spacing_seconds <= 0 {
        return Err(ProofTimeError::InvalidSpacing);
    }
    if best_proof == ProofWork::ZERO {
        return Err(ProofTimeError::ZeroBestProof);
    }
    let (difference, sign) = if to > from {
        (to.wrapping_sub(from), 1)
    } else {
        (from.wrapping_sub(to), -1)
    };
    let quotient = difference
        .wrapping_multiply_u64(target_spacing_seconds as u64)
        .divide_nonzero(best_proof);
    if quotient.0[1..].iter().any(|limb| *limb != 0) || quotient.0[0] > i64::MAX as u64 {
        return Ok(sign * i64::MAX);
    }
    Ok(sign * quotient.0[0] as i64)
}

#[cfg(test)]
mod tests;
