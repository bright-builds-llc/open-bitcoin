// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/chain.h

use crate::prune::status::adjusted_prune_timestamp;
use crate::prune::{
    ManualPruneArgument, ManualPruneArgumentError, PRUNE_BLOCKCHAIN_TIMESTAMP_THRESHOLD,
    PRUNE_TIMESTAMP_WINDOW_SECONDS, PruneMode, get_prune_height, project_prune_status,
    resolve_manual_prune_argument,
};

#[test]
fn project_prune_status_disabled_omits_optional_fields() {
    // Arrange
    let mode = PruneMode::Disabled;

    // Act
    let projection = project_prune_status(mode, Some(11));

    // Assert
    assert!(!projection.pruned);
    assert_eq!(projection.maybe_pruneheight, None);
    assert_eq!(projection.maybe_automatic_pruning, None);
    assert_eq!(projection.maybe_prune_target_size, None);
}

#[test]
fn project_prune_status_manual_only_with_nothing_pruned_omits_target() {
    // Arrange
    let mode = PruneMode::ManualOnly;

    // Act
    let projection = project_prune_status(mode, None);

    // Assert
    assert!(projection.pruned);
    assert_eq!(projection.maybe_pruneheight, Some(0));
    assert_eq!(projection.maybe_automatic_pruning, Some(false));
    assert_eq!(projection.maybe_prune_target_size, None);
}

#[test]
fn project_prune_status_automatic_550_reports_byte_target() {
    // Arrange
    let mode = PruneMode::Automatic { target_mib: 550 };

    // Act
    let projection = project_prune_status(mode, Some(11));

    // Assert
    assert!(projection.pruned);
    assert_eq!(projection.maybe_pruneheight, Some(12));
    assert_eq!(projection.maybe_automatic_pruning, Some(true));
    assert_eq!(projection.maybe_prune_target_size, Some(576716800));
}

#[test]
fn project_prune_status_height_overflow_omits_pruneheight() {
    // Arrange
    let mode = PruneMode::ManualOnly;

    // Act
    let projection = project_prune_status(mode, Some(u32::MAX));

    // Assert
    assert!(projection.pruned);
    assert_eq!(projection.maybe_pruneheight, None);
    assert_eq!(projection.maybe_automatic_pruning, Some(false));
}

#[test]
fn get_prune_height_hole_under_complete_tip_is_last_incomplete_suffix_height() {
    // Arrange
    let mut complete_from_height_one = vec![true; 20];
    complete_from_height_one[10] = false;

    // Act
    let maybe_height = get_prune_height(20, &complete_from_height_one);

    // Assert
    assert_eq!(maybe_height, Some(11));
}

#[test]
fn get_prune_height_incomplete_tip_returns_tip_height() {
    // Arrange
    let complete_from_height_one = [true, true, false];

    // Act
    let maybe_height = get_prune_height(3, &complete_from_height_one);

    // Assert
    assert_eq!(maybe_height, Some(3));
}

#[test]
fn get_prune_height_tip_zero_returns_none() {
    // Arrange
    let complete_from_height_one: [bool; 0] = [];

    // Act
    let maybe_height = get_prune_height(0, &complete_from_height_one);

    // Assert
    assert_eq!(maybe_height, None);
}

#[test]
fn get_prune_height_complete_chain_from_height_one_returns_none() {
    // Arrange
    let complete_from_height_one = [true, true, true, true];

    // Act
    let maybe_height = get_prune_height(4, &complete_from_height_one);

    // Assert
    assert_eq!(maybe_height, None);
}

#[test]
fn get_prune_height_short_slice_returns_none() {
    // Arrange
    let complete_from_height_one = [true, false];

    // Act
    let maybe_height = get_prune_height(5, &complete_from_height_one);

    // Assert
    assert_eq!(maybe_height, None);
}

#[test]
fn resolve_manual_prune_argument_refuses_negative() {
    // Arrange
    let headers = [(1, 1_700_000_000_i64)];

    // Act
    let result = resolve_manual_prune_argument(-1, &headers);

    // Assert
    assert_eq!(result, Err(ManualPruneArgumentError::Negative));
}

#[test]
fn resolve_manual_prune_argument_zero_stays_sentinel() {
    // Arrange
    let headers = [(1, 1_700_000_000_i64)];

    // Act
    let result = resolve_manual_prune_argument(0, &headers);

    // Assert
    assert_eq!(result, Ok(ManualPruneArgument::Zero));
}

#[test]
fn resolve_manual_prune_argument_accepts_height_through_timestamp_threshold() {
    // Arrange
    let headers = [(1, i64::MAX)];

    // Act
    let result = resolve_manual_prune_argument(PRUNE_BLOCKCHAIN_TIMESTAMP_THRESHOLD, &headers);

    // Assert
    assert_eq!(result, Ok(ManualPruneArgument::Height(1_000_000_000)));
}

#[test]
fn resolve_manual_prune_argument_timestamp_selects_least_matching_height() {
    // Arrange
    let raw = PRUNE_BLOCKCHAIN_TIMESTAMP_THRESHOLD + PRUNE_TIMESTAMP_WINDOW_SECONDS + 100;
    let adjusted_time = raw - PRUNE_TIMESTAMP_WINDOW_SECONDS;
    let headers = [
        (30, adjusted_time + 400),
        (10, adjusted_time - 1),
        (20, adjusted_time),
    ];

    // Act
    let result = resolve_manual_prune_argument(raw, &headers);

    // Assert
    assert_eq!(result, Ok(ManualPruneArgument::Height(20)));
}

#[test]
fn adjusted_prune_timestamp_checked_underflow_fails_closed() {
    // Arrange / Act
    let adjusted_time = adjusted_prune_timestamp(i64::MIN);

    // Assert
    assert_eq!(adjusted_time, i64::MAX);
}

#[test]
fn resolve_manual_prune_argument_missing_timestamp_is_not_found() {
    // Arrange
    let raw = PRUNE_BLOCKCHAIN_TIMESTAMP_THRESHOLD + 1;
    let headers = [(4, 0_i64)];

    // Act
    let result = resolve_manual_prune_argument(raw, &headers);

    // Assert
    assert_eq!(result, Err(ManualPruneArgumentError::TimestampNotFound));
}
