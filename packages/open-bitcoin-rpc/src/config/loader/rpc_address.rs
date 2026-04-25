// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use crate::config::RpcClientEndpoint;

use super::{ConfigError, parse_port};

pub(super) fn parse_rpc_client_address(
    rpc_connect: &str,
    maybe_explicit_port: Option<u16>,
    default_port: u16,
) -> Result<RpcClientEndpoint, ConfigError> {
    let (host, maybe_embedded_port) = split_rpc_connect(rpc_connect)?;
    if host.is_empty() {
        return Err(ConfigError::new(format!(
            "invalid rpc address: {rpc_connect}"
        )));
    }

    let port = maybe_explicit_port
        .or(maybe_embedded_port)
        .unwrap_or(default_port);
    Ok(RpcClientEndpoint { host, port })
}

fn split_rpc_connect(value: &str) -> Result<(String, Option<u16>), ConfigError> {
    if let Some(stripped) = value.strip_prefix('[') {
        let Some(end_index) = stripped.find(']') else {
            return Err(ConfigError::new(format!("invalid rpc address: {value}")));
        };
        let host = stripped[..end_index].to_string();
        let remainder = &stripped[end_index + 1..];
        if remainder.is_empty() {
            return Ok((host, None));
        }
        let Some(port) = remainder.strip_prefix(':') else {
            return Err(ConfigError::new(format!("invalid rpc address: {value}")));
        };
        let port = parse_port(port)?;
        return Ok((host, Some(port)));
    }

    if value.matches(':').count() == 1 {
        let Some((host, port)) = value.rsplit_once(':') else {
            return Ok((value.to_string(), None));
        };
        let port = parse_port(port)?;
        return Ok((host.to_string(), Some(port)));
    }

    Ok((value.to_string(), None))
}
