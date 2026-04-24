use std::path::{Path, PathBuf};

use open_bitcoin_rpc::config::{RpcAuthConfig, load_runtime_config_for_args};

use crate::{CliError, args::CliStartupArgs};

const BITCOIN_CONF_FILE_NAME: &str = "bitcoin.conf";

/// Resolved startup contract for the supported `bitcoin-cli` client path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliStartupConfig {
    pub conf_path: PathBuf,
    pub maybe_data_dir: Option<PathBuf>,
    pub rpc: CliRpcConfig,
}

/// Final RPC endpoint and auth state after config and CLI precedence resolve.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliRpcConfig {
    pub host: String,
    pub port: u16,
    pub auth: RpcAuthConfig,
}

/// Resolve config-file, datadir, endpoint, and auth precedence for `bitcoin-cli`.
pub fn resolve_startup_config(
    startup: &CliStartupArgs,
    default_data_dir: &Path,
) -> Result<CliStartupConfig, CliError> {
    let runtime = load_runtime_config_for_args(&startup.to_runtime_config_args(), default_data_dir)
        .map_err(|error| CliError::new(error.to_string()))?;
    let conf_path = resolve_conf_path(startup, default_data_dir);

    Ok(CliStartupConfig {
        conf_path,
        maybe_data_dir: runtime.maybe_data_dir,
        rpc: CliRpcConfig {
            host: runtime.rpc_client.endpoint.host,
            port: runtime.rpc_client.endpoint.port,
            auth: runtime.rpc_client.auth,
        },
    })
}

fn resolve_conf_path(startup: &CliStartupArgs, default_data_dir: &Path) -> PathBuf {
    let initial_data_dir = startup
        .maybe_data_dir
        .clone()
        .unwrap_or_else(|| default_data_dir.to_path_buf());

    startup
        .maybe_conf_path
        .clone()
        .map(|path| resolve_path(&path, &initial_data_dir))
        .unwrap_or_else(|| initial_data_dir.join(BITCOIN_CONF_FILE_NAME))
}

fn resolve_path(path: &Path, base: &Path) -> PathBuf {
    if path.is_absolute() || path.as_os_str().is_empty() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

#[cfg(test)]
mod tests;
