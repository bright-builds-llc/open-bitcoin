// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/common/args.cpp

//! Open Bitcoin-owned daemon runtime config layered above `bitcoin.conf`.

use std::{fs, path::Path};

use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_node::{SyncPeerAddress, SyncRuntimeConfig};

use crate::config::{
    ConfigError, DaemonSyncConfig, DaemonSyncMode, OPEN_BITCOIN_CONFIG_FILE_NAME,
    OpenBitcoinConfig, SyncNetwork, parse_open_bitcoin_jsonc_config,
};

use super::{CliSettings, chain::chain_name, resolve_path, rpc_address::parse_rpc_client_address};

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
    let effective_mode = if let Some(mode) = maybe_cli_mode {
        mode
    } else {
        resolve_config_sync_mode(maybe_config)?
    };
    let mut sync = daemon_sync_config_for_mode(chain, effective_mode)?;
    if sync.is_enabled() {
        apply_sync_overrides(&mut sync.runtime, maybe_config)?;
    }
    Ok(sync)
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

fn resolve_config_sync_mode(
    maybe_config: Option<&OpenBitcoinConfig>,
) -> Result<DaemonSyncMode, ConfigError> {
    let Some(config) = maybe_config else {
        return Ok(DaemonSyncMode::Disabled);
    };
    let mode = DaemonSyncMode::parse(&config.sync.mode)?;
    if config.sync.network_enabled {
        if mode != DaemonSyncMode::MainnetIbd {
            return Err(ConfigError::new(format!(
                "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: sync.network_enabled requires sync.mode = \"mainnet-ibd\" for daemon mainnet sync activation."
            )));
        }
        return Ok(mode);
    }
    if mode.is_enabled() {
        return Err(ConfigError::new(format!(
            "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: sync.mode = \"{}\" requires sync.network_enabled = true.",
            mode.as_str()
        )));
    }

    Ok(DaemonSyncMode::Disabled)
}

fn apply_sync_overrides(
    runtime: &mut SyncRuntimeConfig,
    maybe_config: Option<&OpenBitcoinConfig>,
) -> Result<(), ConfigError> {
    let Some(config) = maybe_config else {
        return Ok(());
    };
    if let Some(manual_peers) = config.sync.maybe_manual_peers.as_ref() {
        runtime.manual_peers = manual_peers
            .iter()
            .map(|peer| parse_sync_peer_address(peer, runtime.network, true))
            .collect::<Result<Vec<_>, _>>()?;
    }
    if let Some(dns_seeds) = config.sync.maybe_dns_seeds.as_ref() {
        runtime.dns_seeds = dns_seeds.clone();
    }
    if let Some(target_outbound_peers) = config.sync.maybe_target_outbound_peers {
        if target_outbound_peers == 0 {
            return Err(ConfigError::new(format!(
                "Error reading {OPEN_BITCOIN_CONFIG_FILE_NAME}: sync.target_outbound_peers must be greater than zero."
            )));
        }
        runtime.target_outbound_peers = target_outbound_peers;
    }
    Ok(())
}

fn parse_sync_peer_address(
    value: &str,
    network: SyncNetwork,
    manual: bool,
) -> Result<SyncPeerAddress, ConfigError> {
    let endpoint = parse_rpc_client_address(value, None, network.default_port())?;
    let peer = if manual {
        SyncPeerAddress::manual(endpoint.host, endpoint.port)
    } else {
        SyncPeerAddress::dns_seed(endpoint.host, endpoint.port)
    };
    Ok(peer)
}
