// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/chain.h

//! I/O-free Knots prune-height helper, `getblockchaininfo` quartet, and
//! `pruneblockchain` height-or-timestamp pre-step.

use super::mode::PruneMode;

/// Knots treats `pruneblockchain` arguments above this value as Unix timestamps.
pub const PRUNE_BLOCKCHAIN_TIMESTAMP_THRESHOLD: i64 = 1_000_000_000;

/// Knots `TIMESTAMP_WINDOW` (`chain.h` `MAX_FUTURE_BLOCK_TIME`), in seconds.
pub const PRUNE_TIMESTAMP_WINDOW_SECONDS: i64 = 7_200;

const BYTES_PER_MIB: u64 = 1024 * 1024;

/// Configured-mode prune facts for `getblockchaininfo` and operator status.
///
/// `maybe_pruneheight`, `maybe_automatic_pruning`, and `maybe_prune_target_size`
/// are absent when prune mode is disabled. `maybe_pruneheight` is `GetPruneHeight`
/// plus one, or `0` when nothing has been pruned. `maybe_prune_target_size` is
/// the automatic target in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PruneStatusProjection {
    pub pruned: bool,
    pub maybe_pruneheight: Option<u32>,
    pub maybe_automatic_pruning: Option<bool>,
    pub maybe_prune_target_size: Option<u64>,
}

/// Resolved `pruneblockchain` argument after the Knots zero and timestamp rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManualPruneArgument {
    Zero,
    Height(u32),
}

/// Refusal from [`resolve_manual_prune_argument`] before any prune plan runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManualPruneArgumentError {
    Negative,
    TimestampNotFound,
}

/// Height of the highest pruned block in the active-chain tip suffix.
///
/// `complete_from_height_one[0]` is height 1. A complete entry means that height
/// still has both payload and undo. The walk matches Knots `GetPruneHeight`
/// (`blockchain.cpp`): genesis is skipped, an incomplete tip is itself the
/// answer, and a complete suffix that reaches height 1 means nothing is pruned.
pub fn get_prune_height(tip_height: u32, complete_from_height_one: &[bool]) -> Option<u32> {
    if tip_height == 0 || complete_from_height_one.len() < tip_height as usize {
        return None;
    }

    let tip_index = (tip_height - 1) as usize;
    if !complete_from_height_one[tip_index] {
        return Some(tip_height);
    }

    let suffix_start = lowest_complete_suffix_height(tip_height, complete_from_height_one);
    if suffix_start == 1 {
        return None;
    }
    Some(suffix_start - 1)
}

/// Projects the Knots `getblockchaininfo` prune quartet from configured mode
/// and the [`get_prune_height`] result.
///
/// `maybe_last_pruned_height` is that helper (`None` when nothing is pruned).
/// Disabled mode clears every optional field. Manual-only omits the byte target.
/// Automatic reports `target_mib * 1024 * 1024` bytes.
pub fn project_prune_status(
    mode: PruneMode,
    maybe_last_pruned_height: Option<u32>,
) -> PruneStatusProjection {
    match mode {
        PruneMode::Disabled => PruneStatusProjection {
            pruned: false,
            maybe_pruneheight: None,
            maybe_automatic_pruning: None,
            maybe_prune_target_size: None,
        },
        PruneMode::ManualOnly => PruneStatusProjection {
            pruned: true,
            maybe_pruneheight: info_pruneheight(maybe_last_pruned_height),
            maybe_automatic_pruning: Some(false),
            maybe_prune_target_size: None,
        },
        PruneMode::Automatic { target_mib } => PruneStatusProjection {
            pruned: true,
            maybe_pruneheight: info_pruneheight(maybe_last_pruned_height),
            maybe_automatic_pruning: Some(true),
            maybe_prune_target_size: target_mib.checked_mul(BYTES_PER_MIB),
        },
    }
}

/// Classifies a raw `pruneblockchain` number before `plan_manual_prune`.
///
/// Zero stays the no-op sentinel. Values through
/// [`PRUNE_BLOCKCHAIN_TIMESTAMP_THRESHOLD`] are heights. Larger values subtract
/// [`PRUNE_TIMESTAMP_WINDOW_SECONDS`] and select the least height whose header
/// time is at least that adjusted time.
pub fn resolve_manual_prune_argument(
    raw: i64,
    header_times_by_height: &[(u32, i64)],
) -> Result<ManualPruneArgument, ManualPruneArgumentError> {
    if raw < 0 {
        return Err(ManualPruneArgumentError::Negative);
    }
    if raw == 0 {
        return Ok(ManualPruneArgument::Zero);
    }
    if raw <= PRUNE_BLOCKCHAIN_TIMESTAMP_THRESHOLD {
        return Ok(ManualPruneArgument::Height(raw as u32));
    }

    let adjusted_time = adjusted_prune_timestamp(raw);
    let Some(height) = earliest_header_at_least(header_times_by_height, adjusted_time) else {
        return Err(ManualPruneArgumentError::TimestampNotFound);
    };
    Ok(ManualPruneArgument::Height(height))
}

/// Subtracts [`PRUNE_TIMESTAMP_WINDOW_SECONDS`].
///
/// Arguments above the timestamp threshold fit in `i64`. A checked failure
/// returns `i64::MAX`, so the header search finds no block.
pub(super) fn adjusted_prune_timestamp(raw: i64) -> i64 {
    raw.checked_sub(PRUNE_TIMESTAMP_WINDOW_SECONDS)
        .unwrap_or(i64::MAX)
}

fn lowest_complete_suffix_height(tip_height: u32, complete_from_height_one: &[bool]) -> u32 {
    let mut suffix_start = tip_height;
    for height in (1..tip_height).rev() {
        let index = (height - 1) as usize;
        if !complete_from_height_one[index] {
            break;
        }
        suffix_start = height;
    }
    suffix_start
}

fn info_pruneheight(maybe_last_pruned_height: Option<u32>) -> Option<u32> {
    let Some(last_pruned_height) = maybe_last_pruned_height else {
        return Some(0);
    };
    let pruneheight = last_pruned_height.checked_add(1)?;
    Some(pruneheight)
}

fn earliest_header_at_least(
    header_times_by_height: &[(u32, i64)],
    adjusted_time: i64,
) -> Option<u32> {
    header_times_by_height
        .iter()
        .filter_map(|(height, header_time)| (*header_time >= adjusted_time).then_some(*height))
        .min()
}
