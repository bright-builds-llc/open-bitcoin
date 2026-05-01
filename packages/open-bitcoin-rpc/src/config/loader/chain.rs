// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

//! Chain-selection helpers for baseline-compatible daemon config loading.

use open_bitcoin_node::core::wallet::AddressNetwork;

use super::{ConfigEntry, ConfigError, parse_bool};

pub(super) fn parse_chain_key(key: &str) -> Result<AddressNetwork, ConfigError> {
    match key {
        "main" => Ok(AddressNetwork::Mainnet),
        "test" | "testnet" => Ok(AddressNetwork::Testnet),
        "signet" => Ok(AddressNetwork::Signet),
        "regtest" => Ok(AddressNetwork::Regtest),
        _ => Err(ConfigError::new(format!("invalid chain key: {key}"))),
    }
}

pub(super) fn parse_chain_name(value: &str) -> Result<AddressNetwork, ConfigError> {
    match value {
        "main" | "mainnet" => Ok(AddressNetwork::Mainnet),
        "test" | "testnet" | "testnet3" => Ok(AddressNetwork::Testnet),
        "signet" => Ok(AddressNetwork::Signet),
        "regtest" => Ok(AddressNetwork::Regtest),
        _ => Err(ConfigError::new(format!("invalid chain value: {value}"))),
    }
}

pub(super) fn supported_chain_key(key: &str) -> bool {
    matches!(key, "main" | "test" | "testnet" | "signet" | "regtest")
}

pub(super) fn determine_chain(
    maybe_cli_chain: Option<AddressNetwork>,
    entries: &[ConfigEntry],
) -> Result<AddressNetwork, ConfigError> {
    if let Some(chain) = maybe_cli_chain {
        return Ok(chain);
    }

    let mut maybe_chain = None;
    for entry in entries {
        if entry.maybe_section.is_some() || !supported_chain_key(&entry.key) {
            continue;
        }
        if parse_bool(Some(&entry.value), false)? {
            maybe_chain = Some(parse_chain_key(&entry.key)?);
        }
    }
    Ok(maybe_chain.unwrap_or(AddressNetwork::Mainnet))
}

pub(super) fn config_section_name(chain: AddressNetwork) -> &'static str {
    chain_name(chain)
}

pub(super) fn chain_name(chain: AddressNetwork) -> &'static str {
    match chain {
        AddressNetwork::Mainnet => "main",
        AddressNetwork::Testnet => "test",
        AddressNetwork::Signet => "signet",
        AddressNetwork::Regtest => "regtest",
    }
}
