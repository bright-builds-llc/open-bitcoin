// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{fmt, path::Path};

use open_bitcoin_node::FjallNodeStore;
use open_bitcoin_rpc::{
    JsonRpcId, JsonRpcVersion, RpcErrorDetail, RpcRequestEnvelope,
    method::OpenBitcoinSyncControlResponse,
};
use serde_json::{Value, json};
use ureq::Agent;

use super::{
    OperatorCommandOutcome, OperatorOutputFormat, OperatorRuntimeError, SyncArgs, SyncCommand,
};
use crate::operator::config::OperatorConfigResolution;
use crate::startup::CliRpcConfig;

pub(super) fn execute_sync_command(
    args: &SyncArgs,
    format: OperatorOutputFormat,
    config_resolution: &OperatorConfigResolution,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let Some(data_dir) = config_resolution.maybe_data_dir.as_ref() else {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: "sync commands require a datadir".to_string(),
        });
    };

    if let Some(outcome) = maybe_execute_live_sync_command(args, format, config_resolution)? {
        return Ok(outcome);
    }

    execute_offline_sync_command(data_dir, args, format)
}

fn maybe_execute_live_sync_command(
    args: &SyncArgs,
    format: OperatorOutputFormat,
    config_resolution: &OperatorConfigResolution,
) -> Result<Option<OperatorCommandOutcome>, OperatorRuntimeError> {
    let Some(startup) = super::startup_config_for_status(config_resolution) else {
        return Ok(None);
    };
    let Ok(client) = HttpSyncControlRpcClient::from_config(&startup.rpc) else {
        return Ok(None);
    };
    match client.call(&args.command) {
        Ok(metadata) => render_sync_outcome(&args.command, format, &metadata).map(Some),
        Err(SyncControlRpcError::Unavailable(_message)) => Ok(None),
        Err(SyncControlRpcError::Failed(message)) => {
            Err(OperatorRuntimeError::InvalidRequest { message })
        }
    }
}

fn execute_offline_sync_command(
    data_dir: &Path,
    args: &SyncArgs,
    format: OperatorOutputFormat,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let store =
        FjallNodeStore::open(data_dir).map_err(|error| OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        })?;
    let mut metadata = store
        .load_runtime_metadata()
        .map_err(|error| OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        })?
        .unwrap_or_default();

    match &args.command {
        SyncCommand::Status => render_sync_outcome(&args.command, format, &metadata),
        SyncCommand::Pause => {
            metadata.sync_control.paused = true;
            store
                .save_runtime_metadata(&metadata, open_bitcoin_node::PersistMode::Sync)
                .map_err(|error| OperatorRuntimeError::InvalidRequest {
                    message: error.to_string(),
                })?;
            render_sync_outcome(&args.command, format, &metadata)
        }
        SyncCommand::Resume => {
            metadata.sync_control.paused = false;
            store
                .save_runtime_metadata(&metadata, open_bitcoin_node::PersistMode::Sync)
                .map_err(|error| OperatorRuntimeError::InvalidRequest {
                    message: error.to_string(),
                })?;
            render_sync_outcome(&args.command, format, &metadata)
        }
    }
}

fn render_sync_outcome(
    command: &SyncCommand,
    format: OperatorOutputFormat,
    metadata: &open_bitcoin_node::RuntimeMetadata,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    match command {
        SyncCommand::Status => {
            let output = match format {
                OperatorOutputFormat::Json => {
                    serde_json::to_string_pretty(metadata).map_err(|error| {
                        OperatorRuntimeError::InvalidRequest {
                            message: error.to_string(),
                        }
                    })?
                }
                OperatorOutputFormat::Human => render_sync_status(metadata),
            };
            Ok(OperatorCommandOutcome::success(format!("{output}\n")))
        }
        SyncCommand::Pause => Ok(OperatorCommandOutcome::success(
            "Daemon sync paused. Use `open-bitcoin sync resume` to continue.\n",
        )),
        SyncCommand::Resume => Ok(OperatorCommandOutcome::success(
            "Daemon sync resumed. Use `open-bitcoin sync status` to inspect current state.\n",
        )),
    }
}

struct HttpSyncControlRpcClient {
    agent: Agent,
    endpoint_url: String,
    authorization_header: String,
}

impl HttpSyncControlRpcClient {
    fn from_config(config: &CliRpcConfig) -> Result<Self, OperatorRuntimeError> {
        Ok(Self {
            agent: Agent::new_with_config(
                Agent::config_builder().http_status_as_error(false).build(),
            ),
            endpoint_url: format!(
                "http://{}/",
                super::format_host_for_url(&config.host, config.port)
            ),
            authorization_header: super::authorization_header(&config.auth)?,
        })
    }

    fn call(
        &self,
        command: &SyncCommand,
    ) -> Result<open_bitcoin_node::RuntimeMetadata, SyncControlRpcError> {
        let method = sync_control_method_name(command);
        let response = self
            .agent
            .post(&self.endpoint_url)
            .header("Authorization", &self.authorization_header)
            .send_json(RpcRequestEnvelope {
                jsonrpc: Some(JsonRpcVersion::V2),
                method: method.to_string(),
                params: json!([]),
                id: Some(JsonRpcId::Number(1)),
            })
            .map_err(|error| SyncControlRpcError::Unavailable(error.to_string()))?;
        let status = response.status().as_u16();
        if status == 401 || status == 403 {
            return Err(SyncControlRpcError::Failed(
                "RPC authentication failed for operator sync command".to_string(),
            ));
        }
        if status != 200 {
            return Err(SyncControlRpcError::Failed(format!(
                "sync control RPC endpoint returned HTTP status {status}"
            )));
        }
        let value: Value = response
            .into_body()
            .read_json()
            .map_err(|error| SyncControlRpcError::Failed(error.to_string()))?;
        let result = extract_sync_control_result(value)?;
        serde_json::from_value::<OpenBitcoinSyncControlResponse>(result)
            .map(|response| response.metadata)
            .map_err(|error| SyncControlRpcError::Failed(error.to_string()))
    }
}

fn sync_control_method_name(command: &SyncCommand) -> &'static str {
    match command {
        SyncCommand::Status => "openbitcoinsyncstatus",
        SyncCommand::Pause => "openbitcoinsyncpause",
        SyncCommand::Resume => "openbitcoinsyncresume",
    }
}

fn extract_sync_control_result(response: Value) -> Result<Value, SyncControlRpcError> {
    let Value::Object(object) = response else {
        return Err(SyncControlRpcError::Failed(
            "sync control RPC response must be an object".to_string(),
        ));
    };
    if let Some(error) = object.get("error") {
        if error.is_null() {
            return object.get("result").cloned().ok_or_else(|| {
                SyncControlRpcError::Failed(
                    "sync control RPC response is missing result".to_string(),
                )
            });
        }
        let detail: RpcErrorDetail = serde_json::from_value(error.clone())
            .map_err(|error| SyncControlRpcError::Failed(error.to_string()))?;
        return Err(SyncControlRpcError::Failed(detail.message));
    }
    object.get("result").cloned().ok_or_else(|| {
        SyncControlRpcError::Failed("sync control RPC response is missing result".to_string())
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SyncControlRpcError {
    Unavailable(String),
    Failed(String),
}

impl fmt::Display for SyncControlRpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable(message) | Self::Failed(message) => f.write_str(message),
        }
    }
}

pub(super) fn render_config_paths(
    resolution: &OperatorConfigResolution,
    format: OperatorOutputFormat,
) -> Result<String, OperatorRuntimeError> {
    let sources = resolution
        .source_names()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    if format == OperatorOutputFormat::Json {
        return serde_json::to_string_pretty(&json!({
            "config_path": resolution.maybe_config_path.as_ref().map(|path| path_to_string(path.as_path())),
            "bitcoin_conf_path": resolution.maybe_bitcoin_conf_path.as_ref().map(|path| path_to_string(path.as_path())),
            "datadir": resolution.maybe_data_dir.as_ref().map(|path| path_to_string(path.as_path())),
            "log_dir": resolution.maybe_log_dir.as_ref().map(|path| path_to_string(path.as_path())),
            "metrics_store_path": resolution.maybe_metrics_store_path.as_ref().map(|path| path_to_string(path.as_path())),
            "sources_considered": sources,
        }))
        .map(|value| format!("{value}\n"))
        .map_err(|error| OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        });
    }
    Ok(format!(
        "Config: {}\nBitcoin config: {}\nDatadir: {}\nLogs: {}\nMetrics: {}\nSources: {}\n",
        display_path(resolution.maybe_config_path.as_deref()),
        display_path(resolution.maybe_bitcoin_conf_path.as_deref()),
        display_path(resolution.maybe_data_dir.as_deref()),
        display_path(resolution.maybe_log_dir.as_deref()),
        display_path(resolution.maybe_metrics_store_path.as_deref()),
        sources.join(" > ")
    ))
}

fn render_sync_status(metadata: &open_bitcoin_node::RuntimeMetadata) -> String {
    let lifecycle = metadata
        .maybe_sync_state
        .as_ref()
        .and_then(|state| match state.sync.lifecycle {
            open_bitcoin_node::FieldAvailability::Available(value) => {
                Some(format!("{value:?}").to_lowercase())
            }
            open_bitcoin_node::FieldAvailability::Unavailable { .. } => None,
        })
        .unwrap_or_else(|| "unavailable".to_string());
    let phase = metadata
        .maybe_sync_state
        .as_ref()
        .and_then(|state| match &state.sync.phase {
            open_bitcoin_node::FieldAvailability::Available(value) => Some(value.clone()),
            open_bitcoin_node::FieldAvailability::Unavailable { .. } => None,
        })
        .unwrap_or_else(|| "unavailable".to_string());
    let updated_at = metadata
        .maybe_sync_state
        .as_ref()
        .map_or(0, |state| state.updated_at_unix_seconds);
    format!(
        "Sync paused: {}\nSync lifecycle: {}\nSync phase: {}\nLast clean shutdown: {}\nLast update: {}",
        metadata.sync_control.paused, lifecycle, phase, metadata.last_clean_shutdown, updated_at
    )
}

fn display_path(maybe_path: Option<&Path>) -> String {
    maybe_path
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "Unavailable".to_string())
}

fn path_to_string(path: &Path) -> String {
    path.display().to_string()
}
