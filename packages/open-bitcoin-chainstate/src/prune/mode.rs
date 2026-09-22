// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/node/blockstorage.cpp

/// Knots `MIN_DISK_SPACE_FOR_BLOCK_FILES` expressed in MiB (550).
pub const MIN_DISK_SPACE_FOR_BLOCK_FILES_MIB: u64 = 550;

/// Typed prune operating mode. Working state never stores Knots' raw
/// `PRUNE_TARGET_MANUAL` integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PruneMode {
    #[default]
    Disabled,
    ManualOnly,
    Automatic {
        target_mib: u64,
    },
}

/// Typed refusal for an invalid `-prune` integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PruneModeParseError {
    Negative,
    BelowMinimum { requested_mib: u64 },
}

/// Maps a Knots-style prune integer into [`PruneMode`].
///
/// Contract: `0` disabled, `1` manual-only, `N >= 550` automatic target MiB.
/// Negatives and `2..=549` refuse.
pub fn parse_prune_arg(n_prune_arg: i64) -> Result<PruneMode, PruneModeParseError> {
    if n_prune_arg < 0 {
        return Err(PruneModeParseError::Negative);
    }

    if n_prune_arg == 0 {
        return Ok(PruneMode::Disabled);
    }

    if n_prune_arg == 1 {
        return Ok(PruneMode::ManualOnly);
    }

    let requested_mib = n_prune_arg as u64;
    if requested_mib < MIN_DISK_SPACE_FOR_BLOCK_FILES_MIB {
        return Err(PruneModeParseError::BelowMinimum { requested_mib });
    }

    Ok(PruneMode::Automatic {
        target_mib: requested_mib,
    })
}
