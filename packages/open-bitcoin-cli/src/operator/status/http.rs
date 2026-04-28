// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! HTTP-backed status RPC adapter.

use std::{fs, path::PathBuf};

use open_bitcoin_rpc::{
    RpcAuthConfig, RpcErrorDetail,
    method::{
        GetBalancesResponse, GetBlockchainInfoResponse, GetMempoolInfoResponse,
        GetNetworkInfoResponse, GetWalletInfoResponse,
    },
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use ureq::Agent;

use crate::startup::CliRpcConfig;

use super::{StatusRpcClient, StatusRpcError};

/// HTTP JSON-RPC client used by the operator status runtime path.
pub struct HttpStatusRpcClient {
    agent: Agent,
    endpoint_url: String,
    authorization_header: String,
}

impl HttpStatusRpcClient {
    /// Build a status RPC adapter from resolved CLI RPC configuration.
    pub fn from_rpc_config(
        config: &CliRpcConfig,
        maybe_wallet_name: Option<&str>,
    ) -> Result<Self, StatusRpcError> {
        Ok(Self {
            agent: Agent::new_with_config(
                Agent::config_builder().http_status_as_error(false).build(),
            ),
            endpoint_url: wallet_endpoint_url(config, maybe_wallet_name),
            authorization_header: authorization_header(&config.auth)?,
        })
    }

    fn call<T: DeserializeOwned>(&self, method: &str) -> Result<T, StatusRpcError> {
        let response = self
            .agent
            .post(&self.endpoint_url)
            .header("Authorization", &self.authorization_header)
            .send_json(json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": [],
                "id": 1,
            }))
            .map_err(|error| {
                StatusRpcError::new(format!("Could not connect to status RPC: {error}"))
            })?;
        let response_status = response.status().as_u16();
        let value: Value = response.into_body().read_json().map_err(|error| {
            StatusRpcError::new(format!("Invalid status RPC response: {error}"))
        })?;
        if let Some(error) = value.get("error")
            && !error.is_null()
        {
            let detail: RpcErrorDetail =
                serde_json::from_value(error.clone()).map_err(|parse_error| {
                    StatusRpcError::new(format!("Invalid status RPC error payload: {parse_error}"))
                })?;
            return Err(StatusRpcError::from_rpc_detail(detail));
        }
        if response_status != 200 {
            return Err(StatusRpcError::new(format!(
                "status RPC returned HTTP {response_status}"
            )));
        }
        let Some(result) = value.get("result").cloned() else {
            return Err(StatusRpcError::new("status RPC response missing result"));
        };
        serde_json::from_value(result)
            .map_err(|error| StatusRpcError::new(format!("Invalid status RPC result: {error}")))
    }
}

impl StatusRpcClient for HttpStatusRpcClient {
    fn get_network_info(&self) -> Result<GetNetworkInfoResponse, StatusRpcError> {
        self.call("getnetworkinfo")
    }

    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, StatusRpcError> {
        self.call("getblockchaininfo")
    }

    fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, StatusRpcError> {
        self.call("getmempoolinfo")
    }

    fn get_wallet_info(&self) -> Result<GetWalletInfoResponse, StatusRpcError> {
        self.call("getwalletinfo")
    }

    fn get_balances(&self) -> Result<GetBalancesResponse, StatusRpcError> {
        self.call("getbalances")
    }
}

fn authorization_header(auth: &RpcAuthConfig) -> Result<String, StatusRpcError> {
    let credentials = match auth {
        RpcAuthConfig::UserPassword { username, password } => {
            format!("{username}:{password}")
        }
        RpcAuthConfig::Cookie { maybe_cookie_file } => {
            let cookie_file = maybe_cookie_file
                .clone()
                .unwrap_or_else(|| PathBuf::from(".cookie"));
            fs::read_to_string(&cookie_file).map_err(|_| {
                StatusRpcError::new(format!(
                    "Could not locate RPC credentials at {}",
                    cookie_file.display()
                ))
            })?
        }
    };
    Ok(format!(
        "Basic {}",
        base64_encode(credentials.trim().as_bytes())
    ))
}

fn format_host_for_url(host: &str, port: u16) -> String {
    if host.contains(':') && !host.starts_with('[') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    }
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        let triple = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
        output.push(TABLE[((triple >> 18) & 0x3f) as usize] as char);
        output.push(TABLE[((triple >> 12) & 0x3f) as usize] as char);
        output.push(if chunk.len() > 1 {
            TABLE[((triple >> 6) & 0x3f) as usize] as char
        } else {
            '='
        });
        output.push(if chunk.len() > 2 {
            TABLE[(triple & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    output
}

fn wallet_endpoint_url(config: &CliRpcConfig, maybe_wallet_name: Option<&str>) -> String {
    maybe_wallet_name.map_or_else(
        || format!("http://{}/", format_host_for_url(&config.host, config.port)),
        |wallet_name| {
            format!(
                "http://{}/wallet/{}",
                format_host_for_url(&config.host, config.port),
                percent_encode_path_segment(wallet_name)
            )
        },
    )
}

fn percent_encode_path_segment(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(nibble_to_hex(byte >> 4));
            encoded.push(nibble_to_hex(byte & 0x0f));
        }
    }
    encoded
}

fn nibble_to_hex(nibble: u8) -> char {
    match nibble & 0x0f {
        0..=9 => char::from(b'0' + (nibble & 0x0f)),
        value => char::from(b'A' + (value - 10)),
    }
}
