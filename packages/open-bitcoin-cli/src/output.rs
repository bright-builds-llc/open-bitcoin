// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/client.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use open_bitcoin_cli::CliError;
use open_bitcoin_rpc::{RpcErrorDetail, RpcFailure};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliCommandFailure {
    pub exit_code: i32,
    pub stderr: String,
}

impl CliCommandFailure {
    pub fn new(stderr: impl Into<String>) -> Self {
        Self {
            exit_code: 1,
            stderr: stderr.into(),
        }
    }

    pub fn from_cli_error(error: CliError) -> Self {
        Self::new(error.to_string())
    }

    pub fn from_rpc_failure(failure: RpcFailure) -> Self {
        if let Some(detail) = failure.maybe_detail {
            return Self::from_rpc_error_detail(detail);
        }

        Self::new("Authentication failed")
    }

    pub fn from_rpc_error_detail(detail: RpcErrorDetail) -> Self {
        Self::new(format!(
            "error code {}: {}",
            detail.code.as_i32(),
            detail.message,
        ))
    }

    pub fn connection_failure(endpoint: &str, error: &ureq::Error) -> Self {
        match error {
            ureq::Error::ConnectionFailed
            | ureq::Error::HostNotFound
            | ureq::Error::Io(_)
            | ureq::Error::Timeout(_) => {
                Self::new(format!("Could not connect to the server {endpoint}"))
            }
            _ => Self::new(error.to_string()),
        }
    }

    pub fn authentication_failed() -> Self {
        Self::new("Incorrect rpcuser or rpcpassword")
    }

    pub fn http_status_failure(endpoint: &str, status: u16) -> Self {
        Self::new(format!(
            "RPC server at {endpoint} returned unexpected HTTP status {status}",
        ))
    }

    pub fn invalid_response(message: impl Into<String>) -> Self {
        Self::new(format!("Invalid RPC response: {}", message.into()))
    }
}

pub fn render_rpc_result(result: &Value) -> Result<String, CliCommandFailure> {
    match result {
        Value::String(value) => Ok(value.clone()),
        Value::Array(_) | Value::Object(_) => serde_json::to_string_pretty(result)
            .map_err(|error| CliCommandFailure::invalid_response(error.to_string())),
        _ => serde_json::to_string(result)
            .map_err(|error| CliCommandFailure::invalid_response(error.to_string())),
    }
}
