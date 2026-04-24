use std::{
    ffi::OsString,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
    path::PathBuf,
};

use open_bitcoin_node::core::{consensus::ConsensusParams, wallet::AddressNetwork};

mod loader;

pub const DEFAULT_COOKIE_AUTH_USER: &str = "__cookie__";
pub const DEFAULT_COOKIE_FILE_NAME: &str = ".cookie";
pub(super) const DEFAULT_RPC_HOST: &str = "127.0.0.1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    message: String,
}

impl ConfigError {
    pub(super) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl core::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WalletRuntimeScope {
    #[default]
    LocalOperatorSingleWallet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpcAuthConfig {
    Cookie { maybe_cookie_file: Option<PathBuf> },
    UserPassword { username: String, password: String },
}

impl RpcAuthConfig {
    pub fn cookie(maybe_cookie_file: Option<PathBuf>) -> Self {
        Self::Cookie { maybe_cookie_file }
    }

    pub fn user_password(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self::UserPassword {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl Default for RpcAuthConfig {
    fn default() -> Self {
        Self::cookie(Some(PathBuf::from(DEFAULT_COOKIE_FILE_NAME)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcServerConfig {
    pub enabled: bool,
    pub bind_address: SocketAddr,
    pub auth: RpcAuthConfig,
}

impl Default for RpcServerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bind_address: default_rpc_address(AddressNetwork::Mainnet),
            auth: RpcAuthConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcClientConfig {
    pub endpoint: RpcClientEndpoint,
    pub auth: RpcAuthConfig,
}

impl Default for RpcClientConfig {
    fn default() -> Self {
        Self {
            endpoint: default_rpc_endpoint(AddressNetwork::Mainnet),
            auth: RpcAuthConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcClientEndpoint {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletRuntimeConfig {
    pub scope: WalletRuntimeScope,
    pub coinbase_maturity: u32,
}

impl Default for WalletRuntimeConfig {
    fn default() -> Self {
        Self {
            scope: WalletRuntimeScope::LocalOperatorSingleWallet,
            coinbase_maturity: ConsensusParams::default().coinbase_maturity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub chain: AddressNetwork,
    pub maybe_data_dir: Option<PathBuf>,
    pub rpc_server: RpcServerConfig,
    pub rpc_client: RpcClientConfig,
    pub wallet: WalletRuntimeConfig,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            chain: AddressNetwork::Mainnet,
            maybe_data_dir: None,
            rpc_server: RpcServerConfig::default(),
            rpc_client: RpcClientConfig::default(),
            wallet: WalletRuntimeConfig::default(),
        }
    }
}

pub fn load_runtime_config() -> Result<RuntimeConfig, ConfigError> {
    loader::load_runtime_config()
}

pub fn load_runtime_config_for_args(
    cli_args: &[OsString],
    default_data_dir: &Path,
) -> Result<RuntimeConfig, ConfigError> {
    loader::load_runtime_config_for_args(cli_args, default_data_dir)
}

pub(super) fn default_rpc_port(chain: AddressNetwork) -> u16 {
    match chain {
        AddressNetwork::Mainnet => 8332,
        AddressNetwork::Testnet => 18_332,
        AddressNetwork::Signet => 38_332,
        AddressNetwork::Regtest => 18_443,
    }
}

fn default_rpc_address(chain: AddressNetwork) -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), default_rpc_port(chain))
}

fn default_rpc_endpoint(chain: AddressNetwork) -> RpcClientEndpoint {
    RpcClientEndpoint {
        host: DEFAULT_RPC_HOST.to_string(),
        port: default_rpc_port(chain),
    }
}

#[cfg(test)]
mod tests;
