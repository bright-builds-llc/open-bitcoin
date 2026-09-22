// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

/// Knots `MIN_BLOCKS_TO_KEEP` — trailing heights that must never be pruned.
pub const MIN_BLOCKS_TO_KEEP: u32 = 288;

/// Inclusive last height that may be a prune candidate for `tip`.
pub fn last_prunable_height(tip: u32) -> u32 {
    tip.saturating_sub(MIN_BLOCKS_TO_KEEP)
}

/// Returns true when `height` sits inside the trailing keep window.
///
/// Knots comparison: `height > tip.saturating_sub(MIN_BLOCKS_TO_KEEP)`.
pub fn height_inside_keep_window(tip: u32, height: u32) -> bool {
    height > tip.saturating_sub(MIN_BLOCKS_TO_KEEP)
}

/// Returns true when automatic prune may begin for the injected tip.
///
/// Knots `FindFilesToPrune` starts only when `tip > prune_after_height`.
pub fn automatic_prune_allowed(tip: u32, prune_after_height: u32) -> bool {
    tip > prune_after_height
}
