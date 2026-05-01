// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp

//! Open Bitcoin-owned daemon runtime config layered above `bitcoin.conf`.

use std::{fs, path::Path};

use open_bitcoin_node::core::wallet::AddressNetwork;

use crate::config::{
    ConfigError, DaemonSyncConfig, DaemonSyncMode, OPEN_BITCOIN_CONFIG_FILE_NAME,
    OpenBitcoinConfig, parse_open_bitcoin_jsonc_config,
};

use super::{CliSettings, chain::chain_name, resolve_path};

pub(super) fn load_open_bitcoin_config(
    cli: &CliSettings,
    base_data_dir: &Path,
) -> Result<Option<OpenBitcoinConfig>, ConfigError> {
    let (config_path, explicit_config) =
        if let Some(path) = cli.maybe_open_bitcoin_config_path.as_ref() {
            (resolve_path(path, base_data_dir), true)
        } else {
            (base_data_dir.join(OPEN_BITCOIN_CONFIG_FILE_NAME), false)
        };

    if config_path.as_os_str().is_empty() {
        return Err(ConfigError::new(format!(
            "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: specified Open Bitcoin config path is empty."
        )));
    }
    if config_path.is_dir() {
        return Err(ConfigError::new(format!(
            "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: config file \"{}\" is a directory.",
            config_path.display()
        )));
    }
    if !config_path.exists() {
        if explicit_config {
            return Err(ConfigError::new(format!(
                "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: specified Open Bitcoin config file \"{}\" could not be opened.",
                config_path.display()
            )));
        }
        return Ok(None);
    }

    let text = fs::read_to_string(&config_path).map_err(|error| {
        ConfigError::new(format!(
            "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: {error}"
        ))
    })?;
    parse_open_bitcoin_jsonc_config(&text).map(Some)
}

pub(super) fn resolve_daemon_sync_config(
    chain: AddressNetwork,
    maybe_cli_mode: Option<DaemonSyncMode>,
    maybe_config: Option<&OpenBitcoinConfig>,
) -> Result<DaemonSyncConfig, ConfigError> {
    if let Some(mode) = maybe_cli_mode {
        return daemon_sync_config_for_mode(chain, mode);
    }

    let Some(config) = maybe_config else {
        return Ok(DaemonSyncConfig::default());
    };
    let mode = DaemonSyncMode::parse(&config.sync.mode)?;
    if config.sync.network_enabled {
        if mode != DaemonSyncMode::MainnetIbd {
            return Err(ConfigError::new(format!(
                "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: sync.network_enabled requires sync.mode = \"mainnet-ibd\" for daemon mainnet sync activation."
            )));
        }
        return daemon_sync_config_for_mode(chain, mode);
    }
    if mode.is_enabled() {
        return Err(ConfigError::new(format!(
            "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: sync.mode = \"{}\" requires sync.network_enabled = true.",
            mode.as_str()
        )));
    }

    Ok(DaemonSyncConfig::default())
}

fn daemon_sync_config_for_mode(
    chain: AddressNetwork,
    mode: DaemonSyncMode,
) -> Result<DaemonSyncConfig, ConfigError> {
    match mode {
        DaemonSyncMode::Disabled => Ok(DaemonSyncConfig::disabled()),
        DaemonSyncMode::MainnetIbd => {
            if chain != AddressNetwork::Mainnet {
                return Err(ConfigError::new(format!(
                    "open-bitcoind mainnet sync activation requires -chain=main or -main; current chain is {}.",
                    chain_name(chain)
                )));
            }
            Ok(DaemonSyncConfig::mainnet_ibd())
        }
    }
}
