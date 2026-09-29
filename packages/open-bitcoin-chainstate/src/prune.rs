// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! I/O-free prune mode, keep-window, lock-buffer, and status-projection helpers.

mod locks;
mod mode;
mod plan;
mod range;
mod status;

#[cfg(test)]
mod tests;

pub use locks::{
    PRUNE_LOCK_BUFFER, PruneLockInfo, height_forbidden_by_any_lock, height_forbidden_by_lock,
};
pub use mode::{
    MIN_DISK_SPACE_FOR_BLOCK_FILES_MIB, PruneMode, PruneModeParseError, parse_prune_arg,
};
pub use plan::{
    AutomaticPruneInput, ManualPruneInput, ManualPruneRefusal, PrunePlan, plan_automatic_prune,
    plan_manual_prune,
};
pub use range::{
    MIN_BLOCKS_TO_KEEP, automatic_prune_allowed, height_inside_keep_window, last_prunable_height,
};
pub use status::{
    ManualPruneArgument, ManualPruneArgumentError, PRUNE_BLOCKCHAIN_TIMESTAMP_THRESHOLD,
    PRUNE_TIMESTAMP_WINDOW_SECONDS, PruneStatusProjection, get_prune_height, project_prune_status,
    resolve_manual_prune_argument,
};
