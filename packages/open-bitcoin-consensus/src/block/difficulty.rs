use open_bitcoin_primitives::BlockHeader;

use crate::context::BlockValidationContext;
use crate::crypto::{CompactTargetError, compact_target_bytes};
use crate::validation::{BlockValidationError, BlockValidationResult, block_error};

type TargetLimbs = [u64; 4];
type WideTargetLimbs = [u64; 8];

pub(crate) fn difficulty_adjustment_interval(
    consensus_params: &crate::context::ConsensusParams,
) -> i64 {
    if consensus_params.pow_target_spacing_seconds <= 0 {
        return 1;
    }

    let interval =
        consensus_params.pow_target_timespan_seconds / consensus_params.pow_target_spacing_seconds;
    interval.max(1)
}

pub(crate) fn next_work_required(
    header: &BlockHeader,
    context: &BlockValidationContext,
) -> Result<u32, BlockValidationError> {
    let consensus_params = &context.consensus_params;
    if context.height == 0 {
        return Ok(consensus_params.pow_limit_bits);
    }

    let interval = difficulty_adjustment_interval(consensus_params);
    if i64::from(context.height) % interval != 0 {
        let allow_min_difficulty_block = consensus_params.allow_min_difficulty_blocks
            && i64::from(header.time)
                > i64::from(context.previous_header.time)
                    + consensus_params
                        .pow_target_spacing_seconds
                        .saturating_mul(2);
        if allow_min_difficulty_block {
            return Ok(consensus_params.pow_limit_bits);
        }

        return Ok(context.previous_header.bits);
    }

    if consensus_params.no_pow_retargeting {
        return Ok(context.previous_header.bits);
    }

    if consensus_params.pow_target_timespan_seconds <= 0 {
        return Err(block_error(
            BlockValidationResult::InvalidHeader,
            "bad-diffbits",
            Some("invalid proof-of-work target timespan".to_string()),
        ));
    }

    let Some(retarget_anchor) = context.maybe_retarget_anchor else {
        return Err(block_error(
            BlockValidationResult::InvalidHeader,
            "bad-diffbits",
            Some("missing retarget anchor for difficulty adjustment".to_string()),
        ));
    };

    let minimum_timespan = consensus_params.pow_target_timespan_seconds / 4;
    let maximum_timespan = consensus_params
        .pow_target_timespan_seconds
        .saturating_mul(4);
    let actual_timespan = (i64::from(context.previous_header.time)
        - retarget_anchor.first_block_time)
        .clamp(minimum_timespan, maximum_timespan);
    let actual_timespan = actual_timespan as u64;
    let target_timespan = consensus_params.pow_target_timespan_seconds as u64;

    let previous_target = target_limbs_from_bits(context.previous_header.bits)?;
    let pow_limit = target_limbs_from_bits(consensus_params.pow_limit_bits)?;
    let mut wide_target = wide_target_from_target(previous_target);
    multiply_wide_target(&mut wide_target, actual_timespan);
    divide_wide_target(&mut wide_target, target_timespan);

    if wide_target_exceeds_limit(&wide_target, &pow_limit) {
        return Ok(consensus_params.pow_limit_bits);
    }

    Ok(compact_bits_from_wide_target(&wide_target))
}

fn target_limbs_from_bits(bits: u32) -> Result<TargetLimbs, BlockValidationError> {
    let bytes = compact_target_bytes(bits).map_err(map_compact_target_error)?;
    let mut limbs = [0_u64; 4];
    for (index, chunk) in bytes.chunks_exact(8).enumerate() {
        limbs[index] = u64::from_le_bytes(chunk.try_into().expect("32-byte chunk split"));
    }

    Ok(limbs)
}

fn map_compact_target_error(error: CompactTargetError) -> BlockValidationError {
    block_error(
        BlockValidationResult::InvalidHeader,
        "bad-diffbits",
        Some(error.to_string()),
    )
}

fn wide_target_from_target(target: TargetLimbs) -> WideTargetLimbs {
    let mut wide_target = [0_u64; 8];
    wide_target[..4].copy_from_slice(&target);
    wide_target
}

fn multiply_wide_target(wide_target: &mut WideTargetLimbs, factor: u64) {
    let mut carry = 0_u128;
    for limb in wide_target.iter_mut() {
        let value = u128::from(*limb) * u128::from(factor) + carry;
        *limb = value as u64;
        carry = value >> 64;
    }
}

fn divide_wide_target(wide_target: &mut WideTargetLimbs, divisor: u64) {
    let mut remainder = 0_u128;
    for limb in wide_target.iter_mut().rev() {
        let dividend = (remainder << 64) | u128::from(*limb);
        *limb = (dividend / u128::from(divisor)) as u64;
        remainder = dividend % u128::from(divisor);
    }
}

fn wide_target_exceeds_limit(wide_target: &WideTargetLimbs, pow_limit: &TargetLimbs) -> bool {
    if wide_target[4..].iter().any(|limb| *limb != 0) {
        return true;
    }

    for index in (0..4).rev() {
        if wide_target[index] > pow_limit[index] {
            return true;
        }
        if wide_target[index] < pow_limit[index] {
            return false;
        }
    }

    false
}

fn compact_bits_from_wide_target(wide_target: &WideTargetLimbs) -> u32 {
    let mut bytes = [0_u8; 32];
    for (index, limb) in wide_target[..4].iter().enumerate() {
        bytes[index * 8..(index + 1) * 8].copy_from_slice(&limb.to_le_bytes());
    }

    let Some(mut size) = bytes
        .iter()
        .rposition(|byte| *byte != 0)
        .map(|index| index + 1)
    else {
        return 0;
    };

    let mut compact = if size <= 3 {
        let mut mantissa = 0_u32;
        for (index, byte) in bytes[..size].iter().enumerate() {
            mantissa |= u32::from(*byte) << (8 * index);
        }
        mantissa << (8 * (3 - size))
    } else {
        u32::from(bytes[size - 3])
            | (u32::from(bytes[size - 2]) << 8)
            | (u32::from(bytes[size - 1]) << 16)
    };

    if (compact & 0x0080_0000) != 0 {
        compact >>= 8;
        size += 1;
    }

    ((size as u32) << 24) | (compact & 0x007f_ffff)
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::BlockHeader;

    use super::{
        TargetLimbs, WideTargetLimbs, compact_bits_from_wide_target,
        difficulty_adjustment_interval, next_work_required, target_limbs_from_bits,
        wide_target_exceeds_limit,
    };
    use crate::context::{BlockValidationContext, ConsensusParams, RetargetAnchor};

    fn boundary_context(consensus_params: ConsensusParams) -> BlockValidationContext {
        BlockValidationContext {
            height: difficulty_adjustment_interval(&consensus_params) as u32,
            previous_header: BlockHeader {
                bits: consensus_params.pow_limit_bits,
                time: 120,
                ..BlockHeader::default()
            },
            maybe_retarget_anchor: Some(RetargetAnchor {
                first_block_time: 100,
            }),
            previous_median_time_past: 0,
            current_time: 130,
            consensus_params,
        }
    }

    #[test]
    fn boundary_disabled_and_missing_anchor_paths_are_covered() {
        // Arrange
        let boundary_params = ConsensusParams {
            allow_min_difficulty_blocks: false,
            no_pow_retargeting: false,
            pow_target_spacing_seconds: 10,
            pow_target_timespan_seconds: 20,
            ..ConsensusParams::default()
        };
        let header = BlockHeader {
            time: 130,
            bits: boundary_params.pow_limit_bits,
            ..BlockHeader::default()
        };
        let disabled_params = ConsensusParams {
            no_pow_retargeting: true,
            ..boundary_params
        };

        // Act
        let disabled_bits =
            next_work_required(&header, &boundary_context(disabled_params)).expect("boundary work");
        let missing_anchor_error = next_work_required(
            &header,
            &BlockValidationContext {
                maybe_retarget_anchor: None,
                ..boundary_context(boundary_params)
            },
        )
        .expect_err("missing anchor must fail");

        // Assert
        assert_eq!(disabled_bits, disabled_params.pow_limit_bits);
        assert_eq!(missing_anchor_error.reject_reason, "bad-diffbits");
        assert_eq!(
            missing_anchor_error.debug_message.as_deref(),
            Some("missing retarget anchor for difficulty adjustment"),
        );
    }

    #[test]
    fn boundary_invalid_target_timespan_is_rejected() {
        // Arrange
        let consensus_params = ConsensusParams {
            allow_min_difficulty_blocks: false,
            no_pow_retargeting: false,
            pow_target_spacing_seconds: 10,
            pow_target_timespan_seconds: 0,
            ..ConsensusParams::default()
        };
        let header = BlockHeader {
            time: 130,
            bits: consensus_params.pow_limit_bits,
            ..BlockHeader::default()
        };

        // Act
        let error = next_work_required(&header, &boundary_context(consensus_params))
            .expect_err("non-positive target timespan must fail");

        // Assert
        assert_eq!(error.reject_reason, "bad-diffbits");
        assert_eq!(
            error.debug_message.as_deref(),
            Some("invalid proof-of-work target timespan"),
        );
    }

    #[test]
    fn retarget_math_caps_at_pow_limit() {
        // Arrange
        let consensus_params = ConsensusParams {
            allow_min_difficulty_blocks: false,
            no_pow_retargeting: false,
            pow_target_spacing_seconds: 10,
            pow_target_timespan_seconds: 20,
            ..ConsensusParams::default()
        };
        let header = BlockHeader {
            time: 210,
            bits: consensus_params.pow_limit_bits,
            ..BlockHeader::default()
        };
        let context = BlockValidationContext {
            previous_header: BlockHeader {
                bits: consensus_params.pow_limit_bits,
                time: 200,
                ..BlockHeader::default()
            },
            maybe_retarget_anchor: Some(RetargetAnchor {
                first_block_time: 100,
            }),
            ..boundary_context(consensus_params)
        };

        // Act
        let bits = next_work_required(&header, &context).expect("capped boundary work");

        // Assert
        assert_eq!(bits, consensus_params.pow_limit_bits);
    }

    #[test]
    fn invalid_compact_bits_map_to_bad_diffbits() {
        // Arrange / Act
        let error = target_limbs_from_bits(0).expect_err("zero compact target must fail");

        // Assert
        assert_eq!(error.reject_reason, "bad-diffbits");
        assert_eq!(
            error.debug_message.as_deref(),
            Some("compact target must be non-zero"),
        );
    }

    #[test]
    fn compact_and_limit_helpers_cover_remaining_branches() {
        // Arrange
        let equal_limit: TargetLimbs = [10, 0, 0, 0];
        let lower_target: WideTargetLimbs = [9, 0, 0, 0, 0, 0, 0, 0];
        let higher_target: WideTargetLimbs = [11, 0, 0, 0, 0, 0, 0, 0];
        let high_limb_target: WideTargetLimbs = [0, 0, 0, 0, 1, 0, 0, 0];
        let small_target: WideTargetLimbs = [0x1234, 0, 0, 0, 0, 0, 0, 0];
        let signbit_target: WideTargetLimbs = [0x0080_0000, 0, 0, 0, 0, 0, 0, 0];

        // Act / Assert
        assert_eq!(compact_bits_from_wide_target(&[0; 8]), 0);
        assert_eq!(compact_bits_from_wide_target(&small_target), 0x0212_3400);
        assert_eq!(compact_bits_from_wide_target(&signbit_target), 0x0400_8000);
        assert!(!wide_target_exceeds_limit(
            &[10, 0, 0, 0, 0, 0, 0, 0],
            &equal_limit
        ));
        assert!(!wide_target_exceeds_limit(&lower_target, &equal_limit));
        assert!(wide_target_exceeds_limit(&higher_target, &equal_limit));
        assert!(wide_target_exceeds_limit(&high_limb_target, &equal_limit));
    }
}
