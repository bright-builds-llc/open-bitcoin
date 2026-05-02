// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_core::{
    consensus::context::{MinDifficultyRecoveryTarget, RetargetAnchor},
    consensus::{
        BlockValidationContext, BlockValidationError, ConsensusParams, check_block_header,
        check_block_header_contextual,
    },
    primitives::BlockHeader,
};
use open_bitcoin_network::{HeaderEntry, HeaderStore, NetworkError};

pub(super) fn validate_header_for_sync(
    header_store: &HeaderStore,
    header: &BlockHeader,
    current_time: i64,
    consensus_params: ConsensusParams,
) -> Result<(), NetworkError> {
    check_block_header(header).map_err(map_invalid_header)?;
    let context =
        build_header_validation_context(header_store, header, current_time, consensus_params)?;
    check_block_header_contextual(header, &context).map_err(map_invalid_header)?;
    Ok(())
}

fn map_invalid_header(error: BlockValidationError) -> NetworkError {
    NetworkError::InvalidHeader {
        reject_reason: error.reject_reason.to_string(),
        maybe_debug_message: error.debug_message.clone(),
    }
}

fn build_header_validation_context(
    header_store: &HeaderStore,
    header: &BlockHeader,
    current_time: i64,
    consensus_params: ConsensusParams,
) -> Result<BlockValidationContext, NetworkError> {
    let has_known_headers = header_store.best_tip().is_some();
    let previous_block_hash = header.previous_block_hash;
    let has_parent = previous_block_hash.to_byte_array() != [0_u8; 32];

    if !has_parent {
        if has_known_headers {
            return Err(NetworkError::InvalidHeader {
                reject_reason: "bad-prevblk".to_string(),
                maybe_debug_message: Some(
                    "unexpected additional genesis header in non-empty header store".to_string(),
                ),
            });
        }
        return Ok(BlockValidationContext {
            height: 0,
            previous_header: BlockHeader::default(),
            maybe_retarget_anchor: None,
            maybe_min_difficulty_recovery_target: None,
            previous_median_time_past: 0,
            current_time,
            consensus_params,
        });
    }

    let parent = header_store
        .entry(&previous_block_hash)
        .ok_or(NetworkError::MissingHeaderAncestor(previous_block_hash))?;
    let height = parent.height.saturating_add(1);
    let previous_median_time_past = header_store
        .median_time_past(parent.block_hash)
        .ok_or(NetworkError::MissingHeaderAncestor(parent.block_hash))?;
    let interval = difficulty_adjustment_interval(&consensus_params);
    let maybe_retarget_anchor =
        retarget_anchor_for_height(header_store, parent, height, interval, consensus_params)?;
    let maybe_min_difficulty_recovery_target = min_difficulty_recovery_target(
        header_store,
        parent,
        header,
        height,
        interval,
        consensus_params,
    )?;

    Ok(BlockValidationContext {
        height,
        previous_header: parent.header.clone(),
        maybe_retarget_anchor,
        maybe_min_difficulty_recovery_target,
        previous_median_time_past,
        current_time,
        consensus_params,
    })
}

fn difficulty_adjustment_interval(consensus_params: &ConsensusParams) -> u32 {
    if consensus_params.pow_target_spacing_seconds <= 0 {
        return 1;
    }

    let interval =
        consensus_params.pow_target_timespan_seconds / consensus_params.pow_target_spacing_seconds;
    interval.max(1) as u32
}

fn retarget_anchor_for_height(
    header_store: &HeaderStore,
    parent: &HeaderEntry,
    height: u32,
    interval: u32,
    consensus_params: ConsensusParams,
) -> Result<Option<RetargetAnchor>, NetworkError> {
    if consensus_params.no_pow_retargeting || height == 0 || !height.is_multiple_of(interval) {
        return Ok(None);
    }

    let anchor_height = height.saturating_sub(interval);
    let anchor = header_store
        .ancestor_at_height(parent.block_hash, anchor_height)
        .ok_or(NetworkError::MissingHeaderAncestor(parent.block_hash))?;
    Ok(Some(RetargetAnchor {
        first_block_time: i64::from(anchor.header.time),
    }))
}

fn min_difficulty_recovery_target(
    header_store: &HeaderStore,
    parent: &HeaderEntry,
    header: &BlockHeader,
    height: u32,
    interval: u32,
    consensus_params: ConsensusParams,
) -> Result<Option<MinDifficultyRecoveryTarget>, NetworkError> {
    if !consensus_params.allow_min_difficulty_blocks
        || height == 0
        || height.is_multiple_of(interval)
    {
        return Ok(None);
    }

    let delayed_enough = i64::from(header.time)
        > i64::from(parent.header.time)
            + consensus_params
                .pow_target_spacing_seconds
                .saturating_mul(2);
    if delayed_enough {
        return Ok(None);
    }

    let mut current_hash = parent.block_hash;
    loop {
        let current = header_store
            .entry(&current_hash)
            .ok_or(NetworkError::MissingHeaderAncestor(current_hash))?;
        if current.height % interval == 0 || current.header.bits != consensus_params.pow_limit_bits
        {
            return Ok(Some(MinDifficultyRecoveryTarget {
                bits: current.header.bits,
            }));
        }
        current_hash = current.header.previous_block_hash;
    }
}
