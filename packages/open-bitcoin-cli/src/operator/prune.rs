// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp

//! `open-bitcoin prune run` and `open-bitcoin prune lock` commands.
//!
//! These commands call `pruneblockchain`, `listprunelocks`, `setprunelock`,
//! and `clearprunelock`. They do not open a database.

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

/// `open-bitcoin prune` arguments.
#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct PruneArgs {
    /// `prune run` or `prune lock`.
    #[command(subcommand)]
    pub command: PruneCommand,
}

/// `open-bitcoin prune` subcommands.
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum PruneCommand {
    /// `open-bitcoin prune run <height-or-timestamp>`.
    Run {
        /// Height or Unix timestamp forwarded to `pruneblockchain`.
        height_or_timestamp: i64,
    },
    /// `open-bitcoin prune lock` list, set, and clear.
    Lock(PruneLockArgs),
}

/// `open-bitcoin prune lock` arguments.
#[derive(Debug, Clone, PartialEq, Eq, Args)]
pub struct PruneLockArgs {
    /// `prune lock list`, `prune lock set`, or `prune lock clear`.
    #[command(subcommand)]
    pub command: PruneLockCommand,
}

/// `open-bitcoin prune lock` subcommands.
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum PruneLockCommand {
    /// `open-bitcoin prune lock list`.
    List,
    /// `open-bitcoin prune lock set --name <name> --height-first <n> --height-last <n>`.
    Set {
        /// Lock name forwarded as one RPC string.
        #[arg(long = "name")]
        name: String,
        /// Inclusive first height.
        #[arg(long = "height-first")]
        height_first: u32,
        /// Inclusive last height.
        #[arg(long = "height-last")]
        height_last: u32,
    },
    /// `open-bitcoin prune lock clear --name <name>`.
    Clear {
        /// Lock name forwarded as one RPC string.
        #[arg(long = "name")]
        name: String,
    },
}

/// Calls the node RPC methods for a parsed prune command.
pub(crate) fn execute_prune_command(
    args: &PruneArgs,
    cli: &OperatorCli,
    config_resolution: &OperatorConfigResolution,
    default_data_dir: &Path,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let startup = prune_startup_config(config_resolution, default_data_dir)?;
    let client = HttpPruneRpcClient::from_config(&startup.rpc)?;
    execute_prune_command_with_client(args, cli.format, &client)
}

pub(crate) fn execute_prune_command_with_client(
    args: &PruneArgs,
    format: OperatorOutputFormat,
    client: &dyn PruneRpcClient,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let (method, params) = prune_rpc_request(&args.command);
    let result = client.call(method, params)?;
    let stdout = match format {
        OperatorOutputFormat::Json => serde_json::to_string_pretty(&result).map_err(|error| {
            OperatorRuntimeError::InvalidRequest {
                message: error.to_string(),
            }
        })?,
        OperatorOutputFormat::Human => render_prune_human(&args.command, &result)?,
    };
    Ok(OperatorCommandOutcome::success(format!("{stdout}\n")))
}

fn prune_rpc_request(command: &PruneCommand) -> (&'static str, Value) {
    match command {
        PruneCommand::Run {
            height_or_timestamp,
        } => ("pruneblockchain", json!([height_or_timestamp])),
        PruneCommand::Lock(lock) => match &lock.command {
            PruneLockCommand::List => ("listprunelocks", json!([])),
            PruneLockCommand::Set {
                name,
                height_first,
                height_last,
            } => ("setprunelock", json!([name, height_first, height_last])),
            PruneLockCommand::Clear { name } => ("clearprunelock", json!([name])),
        },
    }
}

fn render_prune_human(
    command: &PruneCommand,
    result: &Value,
) -> Result<String, OperatorRuntimeError> {
    match command {
        PruneCommand::Run { .. } => render_pruned_height(result),
        PruneCommand::Lock(lock) => render_lock_human(&lock.command, result),
    }
}

fn render_pruned_height(result: &Value) -> Result<String, OperatorRuntimeError> {
    let height = result.as_i64().ok_or_else(|| invalid("pruned height"))?;
    Ok(format!("Pruned height: {height}"))
}

fn render_lock_human(
    command: &PruneLockCommand,
    result: &Value,
) -> Result<String, OperatorRuntimeError> {
    match command {
        PruneLockCommand::List => render_lock_list(result),
        PruneLockCommand::Set { .. } => render_lock_set(result),
        PruneLockCommand::Clear { name } => render_lock_clear(name, result),
    }
}

fn render_lock_list(result: &Value) -> Result<String, OperatorRuntimeError> {
    let rows = result
        .as_array()
        .ok_or_else(|| invalid("prune lock list"))?;
    if rows.is_empty() {
        return Ok("Prune locks: none".to_string());
    }

    let mut lines = Vec::with_capacity(rows.len());
    for row in rows {
        lines.push(format_lock_row(row)?);
    }
    Ok(lines.join("\n"))
}

fn render_lock_set(result: &Value) -> Result<String, OperatorRuntimeError> {
    let name = json_name(result)?;
    let height_first = json_u32(result, "height_first")?;
    let height_last = json_u32(result, "height_last")?;
    Ok(format!(
        "Prune lock set: name={name} height_first={height_first} height_last={height_last}"
    ))
}

fn render_lock_clear(name: &str, result: &Value) -> Result<String, OperatorRuntimeError> {
    let success = result
        .get("success")
        .and_then(Value::as_bool)
        .ok_or_else(|| invalid("prune lock clear"))?;
    if success {
        return Ok(format!("Prune lock cleared: name={name}"));
    }
    Ok("Prune lock clear: success=false".to_string())
}

fn format_lock_row(row: &Value) -> Result<String, OperatorRuntimeError> {
    let name = json_name(row)?;
    let height_first = json_u32(row, "height_first")?;
    let height_last = json_u32(row, "height_last")?;
    Ok(format!(
        "Prune lock: name={name} height_first={height_first} height_last={height_last}"
    ))
}

fn json_name(value: &Value) -> Result<&str, OperatorRuntimeError> {
    value
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid("prune lock name"))
}

fn json_u32(value: &Value, field: &str) -> Result<u32, OperatorRuntimeError> {
    let number = value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| invalid(field))?;
    u32::try_from(number).map_err(|_| invalid(field))
}

fn invalid(field: &str) -> OperatorRuntimeError {
    OperatorRuntimeError::InvalidRequest {
        message: format!("prune RPC result is missing {field}"),
    }
}

fn prune_startup_config(
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

pub(crate) trait PruneRpcClient {
    fn call(&self, method: &str, params: Value) -> Result<Value, OperatorRuntimeError>;
}

struct HttpPruneRpcClient {
    agent: Agent,
    endpoint_url: String,
    authorization_header: String,
}

impl HttpPruneRpcClient {
    fn from_config(config: &crate::startup::CliRpcConfig) -> Result<Self, OperatorRuntimeError> {
        Ok(Self {
            agent: Agent::new_with_config(
                Agent::config_builder().http_status_as_error(false).build(),
            ),
            endpoint_url: format!(
                "http://{}/",
                super::runtime::format_host_for_url(&config.host, config.port)
            ),
            authorization_header: super::runtime::authorization_header(&config.auth)?,
        })
    }
}

impl PruneRpcClient for HttpPruneRpcClient {
    fn call(&self, method: &str, params: Value) -> Result<Value, OperatorRuntimeError> {
        let response = self
            .agent
            .post(&self.endpoint_url)
            .header("Authorization", &self.authorization_header)
            .send_json(RpcRequestEnvelope {
                jsonrpc: Some(JsonRpcVersion::V2),
                method: method.to_string(),
                params,
                id: Some(JsonRpcId::Number(1)),
            })
            .map_err(|error| OperatorRuntimeError::InvalidRequest {
                message: error.to_string(),
            })?;
        let status = response.status().as_u16();
        if status == 401 {
            return Err(OperatorRuntimeError::InvalidRequest {
                message: "RPC authentication failed for operator prune command".to_string(),
            });
        }
        if status != 200 {
            return Err(OperatorRuntimeError::InvalidRequest {
                message: format!("RPC endpoint returned HTTP status {status}"),
            });
        }
        let value: Value = response.into_body().read_json().map_err(|error| {
            OperatorRuntimeError::InvalidRequest {
                message: error.to_string(),
            }
        })?;
        extract_prune_result(value)
    }
}

fn extract_prune_result(response: Value) -> Result<Value, OperatorRuntimeError> {
    if let Some(error) = response.get("error")
        && !error.is_null()
    {
        let detail: RpcErrorDetail =
            serde_json::from_value(error.clone()).map_err(|parse_error| {
                OperatorRuntimeError::InvalidRequest {
                    message: format!("Invalid prune RPC error payload: {parse_error}"),
                }
            })?;
        return Err(OperatorRuntimeError::InvalidRequest {
            message: detail.message,
        });
    }
    response
        .get("result")
        .cloned()
        .ok_or_else(|| OperatorRuntimeError::InvalidRequest {
            message: "prune RPC response missing result".to_string(),
        })
}
