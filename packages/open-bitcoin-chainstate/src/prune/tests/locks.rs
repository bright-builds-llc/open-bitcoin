// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::prune::{PruneLockInfo, height_forbidden_by_any_lock, height_forbidden_by_lock};

fn lock(name: &str, height_first: u32, height_last: u32) -> PruneLockInfo {
    PruneLockInfo {
        name: name.to_owned(),
        height_first,
        height_last,
    }
}

#[test]
fn lock_100_forbids_boundary_heights_90_and_110() {
    // Arrange
    let prune_lock = lock("x", 100, 100);

    // Act / Assert
    assert!(height_forbidden_by_lock(90, &prune_lock));
    assert!(height_forbidden_by_lock(110, &prune_lock));
}

#[test]
fn lock_100_allows_heights_outside_expanded_range() {
    // Arrange
    let prune_lock = lock("x", 100, 100);

    // Act / Assert
    assert!(!height_forbidden_by_lock(89, &prune_lock));
    assert!(!height_forbidden_by_lock(111, &prune_lock));
}

#[test]
fn lock_100_forbids_inclusive_range_90_through_110() {
    // Arrange
    let prune_lock = lock("x", 100, 100);

    // Act / Assert
    for height in 90..=110 {
        assert!(
            height_forbidden_by_lock(height, &prune_lock),
            "height {height} should be forbidden by lock 100..=100"
        );
    }
}

#[test]
fn low_height_lock_uses_knots_lock_height_one_branch() {
    // Arrange
    let prune_lock = lock("low", 5, 5);

    // Act / Assert — Knots: lock_height=1, lock_height_last=15 → forbidden >1 && <=15
    assert!(!height_forbidden_by_lock(1, &prune_lock));
    assert!(height_forbidden_by_lock(2, &prune_lock));
    assert!(height_forbidden_by_lock(15, &prune_lock));
    assert!(!height_forbidden_by_lock(16, &prune_lock));
}

#[test]
fn height_forbidden_by_any_lock_is_logical_or() {
    // Arrange
    let locks = [lock("a", 100, 100), lock("b", 200, 200)];

    // Act / Assert
    assert!(height_forbidden_by_any_lock(90, &locks));
    assert!(height_forbidden_by_any_lock(210, &locks));
    assert!(!height_forbidden_by_any_lock(150, &locks));
}
