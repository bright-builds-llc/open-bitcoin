// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

/// Knots `PRUNE_LOCK_BUFFER` — blocks expanded around a lock range.
pub const PRUNE_LOCK_BUFFER: u32 = 10;

/// Injected inclusive prune lock. Persistence and list/set APIs are out of scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PruneLockInfo {
    /// Caller-supplied identifier; not persisted in this phase.
    pub name: String,
    /// Inclusive first locked height.
    pub height_first: u32,
    /// Inclusive last locked height.
    pub height_last: u32,
}

/// Returns true when `height` is forbidden by a single Knots-style prune lock.
///
/// Formula from `DoPruneLocksForbidPruning`:
/// `lock_height = if height_first <= 11 { 1 } else { height_first - 11 }`,
/// `lock_height_last = height_last + 10`, forbidden when
/// `height > lock_height && height <= lock_height_last`.
pub fn height_forbidden_by_lock(height: u32, lock: &PruneLockInfo) -> bool {
    let lock_height = if lock.height_first <= 11 {
        1
    } else {
        lock.height_first - PRUNE_LOCK_BUFFER - 1
    };
    let lock_height_last = lock.height_last + PRUNE_LOCK_BUFFER;
    height > lock_height && height <= lock_height_last
}

/// Returns true when any lock in `locks` forbids `height`.
pub fn height_forbidden_by_any_lock(height: u32, locks: &[PruneLockInfo]) -> bool {
    locks
        .iter()
        .any(|lock| height_forbidden_by_lock(height, lock))
}
