// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp

//! Open Bitcoin package dry-run and submit operator workflow.

use std::path::Path;

use clap::{Args, Subcommand};
use open_bitcoin_rpc::{JsonRpcId, JsonRpcVersion, RpcErrorDetail, RpcRequestEnvelope};
use serde_json::{Value, json};
use ureq::Agent;

use crate::{args::CliStartupArgs, startup::resolve_startup_config};

use super::{
    OperatorCli, OperatorOutputFormat,
    config::OperatorConfigResolution,
    runtime::{OperatorCommandOutcome, OperatorRuntimeError},
};

const MAX_PACKAGE_HEX_COUNT: usize = 25;
const DRY_RUN_HEADING: &str = "Package dry-run";
const DRY_RUN_DISCLAIMER: &str =
    "Dry-run does not change mempool, relay, persistence, or evidence state.";
const SUBMIT_HEADING: &str = "Package submit";
const SUBMIT_DISCLAIMER: &str =
    "Local admission only. This is not public or default relay and not network-wide propagation.";

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct PackageArgs {
    #[command(subcommand)]
    pub command: PackageCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum PackageCommand {
    DryRun(PackageHexArgs),
    Submit(PackageHexArgs),
}

#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct PackageHexArgs {
    #[arg(long = "hex", required = true, action = clap::ArgAction::Append)]
    pub hex: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageRenderMode {
    DryRun,
    Submit,
}

pub(crate) fn execute_package_command(
    args: &PackageArgs,
    cli: &OperatorCli,
    config_resolution: &OperatorConfigResolution,
    default_data_dir: &Path,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let (mode, hex_args, mode_token) = match &args.command {
        PackageCommand::DryRun(hex_args) => (PackageRenderMode::DryRun, hex_args, "dry-run"),
        PackageCommand::Submit(hex_args) => (PackageRenderMode::Submit, hex_args, "submit"),
    };
    reject_package_hex_count(hex_args.hex.len())?;
    let startup = package_startup_config(config_resolution, default_data_dir)?;
    let result = call_open_bitcoin_package(&startup.rpc, mode_token, &hex_args.hex)?;
    let stdout = match cli.format {
        OperatorOutputFormat::Json => serde_json::to_string_pretty(&result).map_err(|error| {
            OperatorRuntimeError::InvalidRequest {
                message: error.to_string(),
            }
        })?,
        OperatorOutputFormat::Human => render_package_human(mode, &result),
    };
    Ok(OperatorCommandOutcome::success(format!("{stdout}\n")))
}

/// Renders the originating package report for human output.
pub fn render_package_human(mode: PackageRenderMode, value: &Value) -> String {
    let (heading, disclaimer) = match mode {
        PackageRenderMode::DryRun => (DRY_RUN_HEADING, DRY_RUN_DISCLAIMER),
        PackageRenderMode::Submit => (SUBMIT_HEADING, SUBMIT_DISCLAIMER),
    };
    let mut lines = vec![heading.to_string(), disclaimer.to_string()];
    if let Some(fingerprint) = value.get("fingerprint").and_then(Value::as_str) {
        lines.push(format!("Fingerprint: {fingerprint}"));
    }
    if let Some(status) = value.get("status").and_then(Value::as_str) {
        lines.push(format!("Status: {status}"));
    }
    if let Some(members) = value.get("members").and_then(Value::as_array) {
        for member in members {
            let txid = member.get("txid").and_then(Value::as_str).unwrap_or("");
            let wtxid = member.get("wtxid").and_then(Value::as_str).unwrap_or("");
            let result = member.get("result").and_then(Value::as_str).unwrap_or("");
            let admission = member
                .get("admission")
                .and_then(Value::as_str)
                .unwrap_or("");
            let relay = member.get("relay").and_then(Value::as_str).unwrap_or("");
            lines.push(format!(
                "txid={txid} wtxid={wtxid} result={result} admission={admission} relay={relay}"
            ));
        }
    }
    lines.join("\n")
}

fn reject_package_hex_count(count: usize) -> Result<(), OperatorRuntimeError> {
    if count == 0 || count > MAX_PACKAGE_HEX_COUNT {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!(
                "Package decode failed. Provide 1 to {MAX_PACKAGE_HEX_COUNT} raw transaction hex strings and retry dry-run."
            ),
        });
    }
    Ok(())
}

fn package_startup_config(
    config_resolution: &OperatorConfigResolution,
    default_data_dir: &Path,
) -> Result<crate::startup::CliStartupConfig, OperatorRuntimeError> {
    let startup = CliStartupArgs {
        maybe_conf_path: config_resolution.maybe_bitcoin_conf_path.clone(),
        maybe_data_dir: config_resolution.maybe_data_dir.clone(),
        ..CliStartupArgs::default()
    };
    resolve_startup_config(&startup, default_data_dir).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        }
    })
}

fn call_open_bitcoin_package(
    config: &crate::startup::CliRpcConfig,
    mode: &str,
    raw_txs: &[String],
) -> Result<Value, OperatorRuntimeError> {
    let endpoint = format!(
        "http://{}/",
        super::runtime::format_host_for_url(&config.host, config.port)
    );
    let authorization = super::runtime::authorization_header(&config.auth)?;
    let agent = Agent::new_with_config(Agent::config_builder().http_status_as_error(false).build());
    let response = agent
        .post(&endpoint)
        .header("Authorization", &authorization)
        .send_json(RpcRequestEnvelope {
            jsonrpc: Some(JsonRpcVersion::V2),
            method: "openbitcoinpackage".to_string(),
            params: json!({
                "mode": mode,
                "rawtxs": raw_txs,
            }),
            id: Some(JsonRpcId::Number(1)),
        })
        .map_err(|error| OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        })?;
    let status = response.status().as_u16();
    if status == 401 {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: "RPC authentication failed for operator package command".to_string(),
        });
    }
    if status != 200 {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: format!("RPC endpoint {endpoint} returned HTTP status {status}"),
        });
    }
    let value: Value =
        response
            .into_body()
            .read_json()
            .map_err(|error| OperatorRuntimeError::InvalidRequest {
                message: error.to_string(),
            })?;
    if let Some(error) = value.get("error")
        && !error.is_null()
    {
        let detail: RpcErrorDetail =
            serde_json::from_value(error.clone()).map_err(|parse_error| {
                OperatorRuntimeError::InvalidRequest {
                    message: format!("Invalid package RPC error payload: {parse_error}"),
                }
            })?;
        return Err(OperatorRuntimeError::InvalidRequest {
            message: detail.message,
        });
    }
    value
        .get("result")
        .cloned()
        .ok_or_else(|| OperatorRuntimeError::InvalidRequest {
            message: "package RPC response missing result".to_string(),
        })
}
