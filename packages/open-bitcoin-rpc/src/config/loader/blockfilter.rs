// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/settings.cpp

use super::{ConfigEntry, ConfigError, config_section_name};
use crate::config::blockfilter::{
    BasicFilterIndexSetting, FilterOptionValue, resolve_filter_index,
};
use open_bitcoin_node::core::wallet::AddressNetwork;

pub(super) fn parse_cli(
    values: &mut Vec<FilterOptionValue>,
    key: &str,
    maybe_value: Option<&str>,
    negated: bool,
) -> bool {
    if key != "blockfilterindex" {
        return false;
    }
    values.push(interpret_value(maybe_value.unwrap_or_default(), negated));
    true
}

pub(super) fn resolve(
    cli: &[FilterOptionValue],
    entries: &[ConfigEntry],
    chain: AddressNetwork,
) -> Result<BasicFilterIndexSetting, ConfigError> {
    let mut network = Vec::new();
    let mut defaults = Vec::new();
    let active_section = config_section_name(chain);
    for entry in entries {
        let negated = entry.key == "noblockfilterindex";
        if !negated && entry.key != "blockfilterindex" {
            continue;
        }
        let values = match entry.maybe_section.as_deref() {
            None => &mut defaults,
            Some(section) if section == active_section => &mut network,
            Some(_) => continue,
        };
        values.push(interpret_value(&entry.value, negated));
    }
    resolve_filter_index(cli, &network, &defaults)
}

fn interpret_value(value: &str, negated: bool) -> FilterOptionValue {
    if !negated {
        return FilterOptionValue::String(value.to_owned());
    }
    // InterpretBool uses atoi-style integer prefixes, not Rust's generic config boolean parser.
    let enabled = !value.is_empty() && !integer_prefix_nonzero(value);
    FilterOptionValue::Negation(enabled)
}

fn integer_prefix_nonzero(value: &str) -> bool {
    let trimmed = value.trim_matches([' ', '\t', '\n', '\r', '\u{000b}', '\u{000c}']);
    let digits = trimmed
        .strip_prefix('+')
        .or_else(|| trimmed.strip_prefix('-'))
        .unwrap_or(trimmed);
    digits
        .bytes()
        .take_while(u8::is_ascii_digit)
        .any(|digit| digit != b'0')
}

pub(super) fn supported_config_key(key: &str) -> bool {
    matches!(
        key,
        "server"
            | "rpcbind"
            | "rpcport"
            | "rpcconnect"
            | "rpcuser"
            | "rpcpassword"
            | "rpccookiefile"
            | "includeconf"
            | "datadir"
            | "blockfilterindex"
            | "noblockfilterindex"
    )
}
