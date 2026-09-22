// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Pure automatic prune planner: injected sizes and locks in, candidate heights out.

use std::collections::BTreeMap;

use super::locks::{PruneLockInfo, height_forbidden_by_any_lock};
use super::mode::PruneMode;
use super::range::{automatic_prune_allowed, last_prunable_height};

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
