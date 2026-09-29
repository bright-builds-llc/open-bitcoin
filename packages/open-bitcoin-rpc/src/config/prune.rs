// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockmanager_args.cpp

use open_bitcoin_node::core::chainstate::{PruneMode, PruneModeParseError};

use super::ConfigError;

/// Maps the JSONC `prune` integer through the existing Knots integer contract.
///
/// `0` is disabled, `1` is manual-only, and `N >= 550` is an automatic MiB
/// target. Negatives and `2..=549` fail closed.
pub(super) fn resolve_prune_mode(prune: i64) -> Result<PruneMode, ConfigError> {
    open_bitcoin_node::core::chainstate::parse_prune_arg(prune).map_err(|error| match error {
        PruneModeParseError::Negative => ConfigError::new(format!(
            "prune {prune} cannot be configured with a negative value"
        )),
        PruneModeParseError::BelowMinimum { requested_mib } => ConfigError::new(format!(
            "prune {requested_mib} is below the minimum of 550 MiB"
        )),
    })
}
