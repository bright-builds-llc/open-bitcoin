// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! I/O-free prune mode, keep-window, and lock-buffer decision helpers.

mod locks;
mod mode;
mod plan;
mod range;

#[cfg(test)]
mod tests;

pub use locks::{
    PRUNE_LOCK_BUFFER, PruneLockInfo, height_forbidden_by_any_lock, height_forbidden_by_lock,
};
pub use mode::{
    MIN_DISK_SPACE_FOR_BLOCK_FILES_MIB, PruneMode, PruneModeParseError, parse_prune_arg,
};
pub use plan::{AutomaticPruneInput, PrunePlan, plan_automatic_prune};
pub use range::{
    MIN_BLOCKS_TO_KEEP, automatic_prune_allowed, height_inside_keep_window, last_prunable_height,
};
