// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::prune::{
    MIN_BLOCKS_TO_KEEP, automatic_prune_allowed, height_inside_keep_window, last_prunable_height,
};

#[test]
fn last_prunable_height_for_tip_1000_is_712() {
    // Arrange / Act
    let last = last_prunable_height(1000);

    // Assert
    assert_eq!(last, 712);
    assert_eq!(1000 - MIN_BLOCKS_TO_KEEP, 712);
}

#[test]
fn last_prunable_height_saturates_at_zero_for_tip_288() {
    // Arrange / Act
    let last = last_prunable_height(288);

    // Assert
    assert_eq!(last, 0);
}

#[test]
fn last_prunable_height_saturates_at_zero_for_tip_287() {
    // Arrange / Act
    let last = last_prunable_height(287);

    // Assert
    assert_eq!(last, 0);
}

#[test]
fn height_713_is_inside_keep_window_for_tip_1000() {
    // Arrange / Act
    let inside = height_inside_keep_window(1000, 713);

    // Assert
    assert!(inside);
}

#[test]
fn height_712_is_outside_keep_window_for_tip_1000() {
    // Arrange / Act
    let inside = height_inside_keep_window(1000, 712);

    // Assert
    assert!(!inside);
}

#[test]
fn tip_minus_287_through_tip_are_inside_keep_window() {
    // Arrange
    let tip = 1000_u32;

    // Act / Assert
    for height in (tip - 287)..=tip {
        assert!(
            height_inside_keep_window(tip, height),
            "height {height} should be inside keep window for tip {tip}"
        );
    }
}

#[test]
fn automatic_prune_not_allowed_when_tip_equals_prune_after() {
    // Arrange / Act
    let allowed = automatic_prune_allowed(1000, 1000);

    // Assert
    assert!(!allowed);
}

#[test]
fn automatic_prune_allowed_when_tip_exceeds_prune_after() {
    // Arrange / Act
    let allowed = automatic_prune_allowed(1001, 1000);

    // Assert
    assert!(allowed);
}
