// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::collections::BTreeMap;

use crate::prune::{
    AutomaticPruneInput, PruneLockInfo, PruneMode, last_prunable_height, plan_automatic_prune,
};

fn sizes_for_range(first: u32, last: u32, size: u64) -> BTreeMap<u32, u64> {
    let mut height_sizes = BTreeMap::new();
    for height in first..=last {
        height_sizes.insert(height, size);
    }
    height_sizes
}

#[test]
fn plan_automatic_empty_when_tip_equals_prune_after() {
    // Arrange
    let input = AutomaticPruneInput {
        tip: 1000,
        prune_after_height: 1000,
        mode: PruneMode::Automatic { target_mib: 550 },
        height_sizes: sizes_for_range(0, 1000, 1_000),
        current_usage_bytes: 10_000_000,
        locks: vec![],
    };

    // Act
    let plan = plan_automatic_prune(&input);

    // Assert
    assert!(plan.heights.is_empty());
}

#[test]
fn plan_automatic_empty_when_usage_already_at_or_under_target() {
    // Arrange
    let target_mib = 550_u64;
    let target_bytes = target_mib * 1024 * 1024;
    let input = AutomaticPruneInput {
        tip: 1001,
        prune_after_height: 1000,
        mode: PruneMode::Automatic { target_mib },
        height_sizes: sizes_for_range(0, 700, 1_000),
        current_usage_bytes: target_bytes,
        locks: vec![],
    };

    // Act
    let plan = plan_automatic_prune(&input);

    // Assert
    assert!(plan.heights.is_empty());
}

#[test]
fn plan_automatic_never_includes_keep_window_heights() {
    // Arrange
    let tip = 1000_u32;
    let last = last_prunable_height(tip);
    assert_eq!(last, 712);
    let input = AutomaticPruneInput {
        tip,
        prune_after_height: 0,
        mode: PruneMode::Automatic { target_mib: 550 },
        height_sizes: sizes_for_range(0, tip, 1),
        // Force selecting every eligible height: usage far above a tiny target.
        // target_mib 550 still yields a huge target_bytes; use enough usage that
        // we select through the keep-window boundary candidates.
        current_usage_bytes: u64::from(last + 1) + (550 * 1024 * 1024),
        locks: vec![],
    };

    // Act
    let plan = plan_automatic_prune(&input);

    // Assert
    assert!(
        !plan.heights.iter().any(|&h| h >= 713),
        "tip=1000 must never plan height 713 or above; got {:?}",
        plan.heights
    );
    assert!(
        plan.heights.contains(&712),
        "height 712 may appear for tip=1000; got {:?}",
        plan.heights
    );
    for height in 713..=tip {
        assert!(
            !plan.heights.contains(&height),
            "keep-window height {height} must never be selected"
        );
    }
}

#[test]
fn plan_automatic_byte_budget_selects_oldest_until_under_target() {
    // Arrange
    let input = AutomaticPruneInput {
        tip: 1000,
        prune_after_height: 0,
        mode: PruneMode::Automatic { target_mib: 1 },
        height_sizes: BTreeMap::from([(700, 1_200_000), (701, 1_200_000), (702, 1_200_000)]),
        current_usage_bytes: 3_000_000,
        locks: vec![],
    };

    // Act
    let plan = plan_automatic_prune(&input);

    // Assert
    // remaining: 3_000_000 → 1_800_000 → 600_000 (<= 1_048_576) after 701
    assert_eq!(plan.heights, vec![700, 701]);
    assert!(!plan.heights.contains(&702));
}

#[test]
fn plan_automatic_omits_lock_protected_heights_without_failing() {
    // Arrange
    let mut height_sizes = BTreeMap::new();
    for height in 90..=110 {
        height_sizes.insert(height, 100);
    }
    // Neighbors outside the ±10 buffer around lock 100.
    height_sizes.insert(89, 100);
    height_sizes.insert(111, 100);

    let input = AutomaticPruneInput {
        tip: 1000,
        prune_after_height: 0,
        mode: PruneMode::Automatic { target_mib: 1 },
        height_sizes,
        current_usage_bytes: 10_000_000,
        locks: vec![PruneLockInfo {
            name: "test-lock".to_owned(),
            height_first: 100,
            height_last: 100,
        }],
    };

    // Act
    let plan = plan_automatic_prune(&input);

    // Assert — lock first=last=100 forbids [90, 110]; neighbors still selectable.
    for height in 90..=110 {
        assert!(
            !plan.heights.contains(&height),
            "locked-buffer height {height} must be omitted"
        );
    }
    assert!(plan.heights.contains(&89));
    assert!(plan.heights.contains(&111));
}

#[test]
fn plan_automatic_empty_for_disabled_and_manual_only() {
    // Arrange
    let height_sizes = sizes_for_range(0, 700, 1_000);
    let disabled = AutomaticPruneInput {
        tip: 1000,
        prune_after_height: 0,
        mode: PruneMode::Disabled,
        height_sizes: height_sizes.clone(),
        current_usage_bytes: 10_000_000,
        locks: vec![],
    };
    let manual_only = AutomaticPruneInput {
        mode: PruneMode::ManualOnly,
        ..disabled.clone()
    };

    // Act
    let disabled_plan = plan_automatic_prune(&disabled);
    let manual_plan = plan_automatic_prune(&manual_only);

    // Assert
    assert!(disabled_plan.heights.is_empty());
    assert!(manual_plan.heights.is_empty());
}

#[test]
fn plan_automatic_empty_when_target_mib_overflows_byte_conversion() {
    // Arrange — checked_mul(1024*1024) overflows → empty plan, no panic.
    let overflowing_mib = (u64::MAX / (1024 * 1024)) + 1;
    let input = AutomaticPruneInput {
        tip: 1000,
        prune_after_height: 0,
        mode: PruneMode::Automatic {
            target_mib: overflowing_mib,
        },
        height_sizes: BTreeMap::from([(700, 1_000)]),
        current_usage_bytes: u64::MAX,
        locks: vec![],
    };

    // Act
    let plan = plan_automatic_prune(&input);

    // Assert
    assert!(plan.heights.is_empty());
}

#[test]
fn plan_automatic_stops_before_heights_past_last_prunable() {
    // Arrange — only heights inside the keep window appear in sizes, so the
    // ascending walk hits `height > last` and breaks without selecting any.
    let tip = 1000_u32;
    let last = last_prunable_height(tip);
    assert_eq!(last, 712);
    let input = AutomaticPruneInput {
        tip,
        prune_after_height: 0,
        mode: PruneMode::Automatic { target_mib: 1 },
        height_sizes: BTreeMap::from([(713, 1_000_000), (800, 1_000_000), (900, 1_000_000)]),
        current_usage_bytes: 10_000_000,
        locks: vec![],
    };

    // Act
    let plan = plan_automatic_prune(&input);

    // Assert
    assert!(plan.heights.is_empty());
    assert!(!plan.heights.contains(&713));
}
