use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

pub const DEFAULT_RPC_PORT: u16 = 8332;
pub const DEFAULT_COOKIE_AUTH_USER: &str = "__cookie__";
pub const DEFAULT_COOKIE_FILE_NAME: &str = ".cookie";

fn default_rpc_address() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), DEFAULT_RPC_PORT)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WalletRuntimeScope {
    #[default]
    LocalOperatorSingleWallet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpcAuthConfig {
    CookieFile {
        username: String,
        cookie_file: PathBuf,
    },
    UserPassword {
        username: String,
        password: String,
    },
}

impl RpcAuthConfig {
    pub fn cookie_file(cookie_file: impl Into<PathBuf>) -> Self {
        Self::CookieFile {
            username: DEFAULT_COOKIE_AUTH_USER.to_string(),
            cookie_file: cookie_file.into(),
        }
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
        Self::cookie_file(DEFAULT_COOKIE_FILE_NAME)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcServerConfig {
    pub bind_address: SocketAddr,
    pub auth: RpcAuthConfig,
}

impl Default for RpcServerConfig {
    fn default() -> Self {
        Self {
            bind_address: default_rpc_address(),
            auth: RpcAuthConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcClientConfig {
    pub connect_address: SocketAddr,
    pub auth: RpcAuthConfig,
}

impl Default for RpcClientConfig {
    fn default() -> Self {
        Self {
            connect_address: default_rpc_address(),
            auth: RpcAuthConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeConfig {
    pub server: RpcServerConfig,
    pub client: RpcClientConfig,
    pub wallet_scope: WalletRuntimeScope,
}

#[cfg(test)]
mod tests;
