// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Pure prune planners: injected facts and locks in, candidate heights or refusals out.

use std::collections::{BTreeMap, BTreeSet};

use super::locks::{PruneLockInfo, height_forbidden_by_any_lock};
use super::mode::PruneMode;
use super::range::{automatic_prune_allowed, height_inside_keep_window, last_prunable_height};

/// Ascending unique candidate heights for a later unlink phase.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PrunePlan {
    pub heights: Vec<u32>,
}

/// Injected facts for automatic prune planning. No storage I/O.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomaticPruneInput {
    pub tip: u32,
    /// Injected chain-params prune-after height — never a hardcoded mainnet constant.
    pub prune_after_height: u32,
    pub mode: PruneMode,
    /// Ascending height → payload byte size. Missing heights are treated as absent.
    pub height_sizes: BTreeMap<u32, u64>,
    /// Total current usage in bytes attributed to pruneable storage.
    pub current_usage_bytes: u64,
    pub locks: Vec<PruneLockInfo>,
}

/// Builds an automatic prune candidate plan from injected facts.
///
/// Always returns a plan (possibly empty). Never errs for prune-after,
/// under-target usage, or non-automatic modes.
pub fn plan_automatic_prune(input: &AutomaticPruneInput) -> PrunePlan {
    let PruneMode::Automatic { target_mib } = input.mode else {
        return PrunePlan::default();
    };

    if !automatic_prune_allowed(input.tip, input.prune_after_height) {
        return PrunePlan::default();
    }

    let Some(target_bytes) = target_mib.checked_mul(1024 * 1024) else {
        // Overflow means the MiB target already exceeds any realistic usage budget.
        return PrunePlan::default();
    };

    if input.current_usage_bytes <= target_bytes {
        return PrunePlan::default();
    }

    let last = last_prunable_height(input.tip);
    let mut remaining_usage = input.current_usage_bytes;
    let mut heights = Vec::new();

    for (&height, &size) in &input.height_sizes {
        if height > last {
            break;
        }

        if height_forbidden_by_any_lock(height, &input.locks) {
            continue;
        }

        heights.push(height);
        remaining_usage = remaining_usage.saturating_sub(size);

        if remaining_usage <= target_bytes {
            break;
        }
    }

    PrunePlan { heights }
}

/// Injected facts for manual prune planning. No storage I/O and no byte budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManualPruneInput {
    pub tip: u32,
    /// Injected chain-params prune-after height — never a hardcoded mainnet constant.
    pub prune_after_height: u32,
    pub mode: PruneMode,
    pub target_height: u32,
    /// Optional presence filter: if Some, only heights present in the set are candidates.
    /// If None, every height in `0..=effective_end` that is not lock-forbidden is a candidate.
    pub maybe_present_heights: Option<BTreeSet<u32>>,
    pub locks: Vec<PruneLockInfo>,
}

/// Typed refusal for an illegal manual prune request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManualPruneRefusal {
    Disabled,
    ChainTooShort {
        tip: u32,
        prune_after_height: u32,
    },
    TargetInsideKeepWindow {
        tip: u32,
        target: u32,
        last_prunable: u32,
    },
    TargetAboveTip {
        tip: u32,
        target: u32,
    },
}

/// Builds a manual prune candidate plan, or a typed refusal for illegal targets.
///
/// Unlike Knots RPC (which clamps near-tip targets to `tip-288`), Open Bitcoin
/// refuses keep-window targets with [`ManualPruneRefusal::TargetInsideKeepWindow`].
pub fn plan_manual_prune(input: &ManualPruneInput) -> Result<PrunePlan, ManualPruneRefusal> {
    if matches!(input.mode, PruneMode::Disabled) {
        return Err(ManualPruneRefusal::Disabled);
    }

    if input.tip < input.prune_after_height {
        return Err(ManualPruneRefusal::ChainTooShort {
            tip: input.tip,
            prune_after_height: input.prune_after_height,
        });
    }

    if input.target_height > input.tip {
        return Err(ManualPruneRefusal::TargetAboveTip {
            tip: input.tip,
            target: input.target_height,
        });
    }

    let last_prunable = last_prunable_height(input.tip);
    if height_inside_keep_window(input.tip, input.target_height) {
        return Err(ManualPruneRefusal::TargetInsideKeepWindow {
            tip: input.tip,
            target: input.target_height,
            last_prunable,
        });
    }

    let effective_end = input.target_height.min(last_prunable);
    let mut heights = Vec::new();

    match &input.maybe_present_heights {
        Some(present) => {
            for &height in present {
                if height > effective_end {
                    break;
                }
                if height_forbidden_by_any_lock(height, &input.locks) {
                    continue;
                }
                heights.push(height);
            }
        }
        None => {
            for height in 0..=effective_end {
                if height_forbidden_by_any_lock(height, &input.locks) {
                    continue;
                }
                heights.push(height);
            }
        }
    }

    Ok(PrunePlan { heights })
}
