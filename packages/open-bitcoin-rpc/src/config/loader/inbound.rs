// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

//! Open Bitcoin-owned inbound listener CLI flag parsing.

use super::{CliSettings, ConfigError, parse_bool, parse_usize};

pub(super) fn parse_inbound_cli_arg(
    settings: &mut CliSettings,
    key: &str,
    maybe_value: Option<&str>,
    negated: bool,
) -> Result<bool, ConfigError> {
    match key {
        "openbitcoininbound" => {
            settings.maybe_inbound_enabled = Some(parse_bool(maybe_value, negated)?);
        }
        "openbitcoinlisten" => {
            let value = required_value("openbitcoinlisten", maybe_value)?;
            settings
                .maybe_inbound_listen_addresses
                .get_or_insert_with(Vec::new)
                .push(value.to_string());
        }
        "openbitcoinmaxinbound" => {
            let value = required_value("openbitcoinmaxinbound", maybe_value)?;
            settings.maybe_max_inbound_peers = Some(parse_usize("openbitcoinmaxinbound", value)?);
        }
        "openbitcoinreservedslots" => {
            let value = required_value("openbitcoinreservedslots", maybe_value)?;
            settings.maybe_inbound_reserved_slots =
                Some(parse_usize("openbitcoinreservedslots", value)?);
        }
        "openbitcoinallowpublic" => {
            settings.maybe_inbound_allow_public = Some(parse_bool(maybe_value, negated)?);
        }
        "openbitcoininboundpermissionclass" => {
            let value = required_value("openbitcoininboundpermissionclass", maybe_value)?;
            settings
                .maybe_inbound_permission_class_specs
                .get_or_insert_with(Vec::new)
                .push(value.to_string());
        }
        _ => return Ok(false),
    }

    Ok(true)
}

fn required_value<'a>(flag: &str, maybe_value: Option<&'a str>) -> Result<&'a str, ConfigError> {
    maybe_value.ok_or_else(|| {
        ConfigError::new(format!(
            "Error parsing command line arguments: Can not set -{flag} with no value. Please specify value with -{flag}=value.",
        ))
    })
}
