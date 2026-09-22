// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::collections::BTreeSet;

use crate::prune::{
    ManualPruneInput, ManualPruneRefusal, PruneLockInfo, PruneMode, last_prunable_height,
    plan_manual_prune,
};

fn manual_input(
    tip: u32,
    prune_after_height: u32,
    mode: PruneMode,
    target_height: u32,
) -> ManualPruneInput {
    ManualPruneInput {
        tip,
        prune_after_height,
        mode,
        target_height,
        maybe_present_heights: None,
        locks: vec![],
    }
}

#[test]
fn plan_manual_refuses_target_inside_keep_window_unlike_knots_rpc_clamp() {
    // Arrange — Knots `rpc/blockchain.cpp` clamps a near-tip pruneheight to
    // tip-288; Open Bitcoin D-07 / PRUN-03 refuses instead of clamping.
    let tip = 1000_u32;
    let target = 713_u32;
    let last_prunable = last_prunable_height(tip);
    assert_eq!(last_prunable, 712);
    let input = manual_input(tip, 0, PruneMode::ManualOnly, target);

    // Act
    let result = plan_manual_prune(&input);

    // Assert — refuse; do not silently plan as if target were clamped to 712.
    assert_eq!(
        result,
        Err(ManualPruneRefusal::TargetInsideKeepWindow {
            tip,
            target,
            last_prunable,
        })
    );
}

#[test]
fn plan_manual_allows_last_prunable_target_and_never_includes_keep_window() {
    // Arrange
    let tip = 1000_u32;
    let target = 712_u32;
    let input = manual_input(tip, 0, PruneMode::ManualOnly, target);

    // Act
    let plan = plan_manual_prune(&input).expect("target=712 is legal for tip=1000");

    // Assert
    assert!(
        plan.heights.iter().all(|&h| h <= 712),
        "heights must be ⊆ 0..=712; got {:?}",
        plan.heights
    );
    assert!(!plan.heights.iter().any(|&h| h >= 713));
    assert_eq!(plan.heights.last().copied(), Some(712));
    assert_eq!(plan.heights.len(), 713);
}

#[test]
fn plan_manual_refuses_disabled_mode() {
    // Arrange
    let input = manual_input(1000, 0, PruneMode::Disabled, 500);

    // Act
    let result = plan_manual_prune(&input);

    // Assert
    assert_eq!(result, Err(ManualPruneRefusal::Disabled));
}

#[test]
fn plan_manual_refuses_when_tip_below_prune_after() {
    // Arrange
    let tip = 99_u32;
    let prune_after_height = 100_u32;
    let input = manual_input(tip, prune_after_height, PruneMode::ManualOnly, 50);

    // Act
    let result = plan_manual_prune(&input);

    // Assert
    assert_eq!(
        result,
        Err(ManualPruneRefusal::ChainTooShort {
            tip,
            prune_after_height,
        })
    );
}

#[test]
fn plan_manual_allows_tip_equal_prune_after_then_applies_keep_window() {
    // Arrange — tip == prune_after is not ChainTooShort (asymmetric vs automatic).
    let tip = 100_u32;
    let prune_after_height = 100_u32;
    let target = 50_u32;
    let input = manual_input(tip, prune_after_height, PruneMode::ManualOnly, target);
    let last_prunable = last_prunable_height(tip);

    // Act
    let result = plan_manual_prune(&input);

    // Assert — not ChainTooShort; keep-window still applies (target 50 > last 0).
    assert_ne!(
        result,
        Err(ManualPruneRefusal::ChainTooShort {
            tip,
            prune_after_height,
        })
    );
    assert_eq!(
        result,
        Err(ManualPruneRefusal::TargetInsideKeepWindow {
            tip,
            target,
            last_prunable,
        })
    );
}

#[test]
fn plan_manual_refuses_target_above_tip() {
    // Arrange
    let tip = 1000_u32;
    let target = 2000_u32;
    let input = manual_input(tip, 0, PruneMode::ManualOnly, target);

    // Act
    let result = plan_manual_prune(&input);

    // Assert
    assert_eq!(
        result,
        Err(ManualPruneRefusal::TargetAboveTip { tip, target })
    );
}

#[test]
fn plan_manual_omits_lock_protected_heights_without_failing() {
    // Arrange
    let input = ManualPruneInput {
        tip: 1000,
        prune_after_height: 0,
        mode: PruneMode::ManualOnly,
        target_height: 200,
        maybe_present_heights: None,
        locks: vec![PruneLockInfo {
            name: "test-lock".to_owned(),
            height_first: 100,
            height_last: 100,
        }],
    };

    // Act
    let plan = plan_manual_prune(&input).expect("legal target must not fail for lock overlap");

    // Assert — lock first=last=100 forbids [90, 110]; neighbors still selectable.
    for height in 90..=110 {
        assert!(
            !plan.heights.contains(&height),
            "locked-buffer height {height} must be omitted"
        );
    }
    assert!(plan.heights.contains(&89));
    assert!(plan.heights.contains(&111));
    assert!(plan.heights.contains(&200));
}

#[test]
fn plan_manual_has_no_byte_budget_fields_and_plans_full_legal_range() {
    // Arrange — ManualPruneInput has no current_usage_bytes / height_sizes.
    // A legal target plans every height through effective_end (no MiB walk).
    let tip = 1000_u32;
    let target = 100_u32;
    let input = manual_input(tip, 0, PruneMode::ManualOnly, target);

    // Act
    let plan = plan_manual_prune(&input).expect("target=100 is legal for tip=1000");

    // Assert — full contiguous range, not a usage-limited subset.
    assert_eq!(plan.heights, (0..=100).collect::<Vec<_>>());
}

#[test]
fn plan_manual_allows_automatic_mode() {
    // Arrange — Automatic is prune-enabled; only Disabled refuses manual prune.
    let input = manual_input(1000, 0, PruneMode::Automatic { target_mib: 550 }, 500);

    // Act
    let plan = plan_manual_prune(&input).expect("Automatic mode allows manual planning");

    // Assert
    assert_eq!(plan.heights.last().copied(), Some(500));
    assert!(!plan.heights.is_empty());
}

#[test]
fn plan_manual_filters_present_heights_and_omits_locks() {
    // Arrange — Some(present) path: only listed heights are candidates; past
    // effective_end are skipped via ascending break; locks still omit.
    let present = BTreeSet::from([50, 89, 100, 111, 200, 250]);
    let input = ManualPruneInput {
        tip: 1000,
        prune_after_height: 0,
        mode: PruneMode::ManualOnly,
        target_height: 200,
        maybe_present_heights: Some(present),
        locks: vec![PruneLockInfo {
            name: "test-lock".to_owned(),
            height_first: 100,
            height_last: 100,
        }],
    };

    // Act
    let plan = plan_manual_prune(&input).expect("legal present-filter plan");

    // Assert — 100 is lock-forbidden; 250 > effective_end 200 is excluded.
    assert_eq!(plan.heights, vec![50, 89, 111, 200]);
    assert!(!plan.heights.contains(&100));
    assert!(!plan.heights.contains(&250));
}
