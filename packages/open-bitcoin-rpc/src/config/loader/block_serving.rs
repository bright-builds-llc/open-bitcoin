// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

//! Open Bitcoin-owned block-serving CLI flag parsing.

use super::{CliSettings, ConfigError, parse_bool};

pub(super) fn parse_block_serving_cli_arg(
    settings: &mut CliSettings,
    key: &str,
    maybe_value: Option<&str>,
    negated: bool,
) -> Result<bool, ConfigError> {
    match key {
        "openbitcoinblockserving" => {
            settings.maybe_block_serving_enabled = Some(parse_bool(maybe_value, negated)?);
        }
        "openbitcoincompactrelay" => {
            settings.maybe_compact_relay_enabled = Some(parse_bool(maybe_value, negated)?);
        }
        _ => return Ok(false),
    }

    Ok(true)
}
