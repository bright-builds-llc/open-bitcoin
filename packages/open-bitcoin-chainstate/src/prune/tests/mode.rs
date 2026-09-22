// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::prune::{
    MIN_DISK_SPACE_FOR_BLOCK_FILES_MIB, PruneMode, PruneModeParseError, parse_prune_arg,
};

#[test]
fn parse_prune_arg_maps_zero_to_disabled() {
    // Arrange / Act
    let result = parse_prune_arg(0);

    // Assert
    assert_eq!(result, Ok(PruneMode::Disabled));
}

#[test]
fn parse_prune_arg_maps_one_to_manual_only() {
    // Arrange / Act
    let result = parse_prune_arg(1);

    // Assert
    assert_eq!(result, Ok(PruneMode::ManualOnly));
}

#[test]
fn parse_prune_arg_maps_550_to_automatic_minimum_target() {
    // Arrange / Act
    let result = parse_prune_arg(550);

    // Assert
    assert_eq!(
        result,
        Ok(PruneMode::Automatic {
            target_mib: MIN_DISK_SPACE_FOR_BLOCK_FILES_MIB
        })
    );
}

#[test]
fn parse_prune_arg_maps_1000_to_automatic_target() {
    // Arrange / Act
    let result = parse_prune_arg(1000);

    // Assert
    assert_eq!(result, Ok(PruneMode::Automatic { target_mib: 1000 }));
}

#[test]
fn parse_prune_arg_refuses_negative() {
    // Arrange / Act
    let result = parse_prune_arg(-1);

    // Assert
    assert_eq!(result, Err(PruneModeParseError::Negative));
}

#[test]
fn parse_prune_arg_refuses_two_as_below_minimum() {
    // Arrange / Act
    let result = parse_prune_arg(2);

    // Assert
    assert_eq!(
        result,
        Err(PruneModeParseError::BelowMinimum { requested_mib: 2 })
    );
}

#[test]
fn parse_prune_arg_refuses_549_as_below_minimum() {
    // Arrange / Act
    let result = parse_prune_arg(549);

    // Assert
    assert_eq!(
        result,
        Err(PruneModeParseError::BelowMinimum { requested_mib: 549 })
    );
}

#[test]
fn prune_mode_default_is_disabled() {
    // Arrange / Act
    let mode = PruneMode::default();

    // Assert
    assert_eq!(mode, PruneMode::Disabled);
}
