// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::path::Path;

use open_bitcoin_node::FjallNodeStore;
use serde_json::json;

use super::{
    OperatorCommandOutcome, OperatorOutputFormat, OperatorRuntimeError, SyncArgs, SyncCommand,
};
use crate::operator::config::OperatorConfigResolution;

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

    match args.command {
        SyncCommand::Status => {
            let output = match format {
                OperatorOutputFormat::Json => {
                    serde_json::to_string_pretty(&metadata).map_err(|error| {
                        OperatorRuntimeError::InvalidRequest {
                            message: error.to_string(),
                        }
                    })?
                }
                OperatorOutputFormat::Human => render_sync_status(&metadata),
            };
            Ok(OperatorCommandOutcome::success(format!("{output}\n")))
        }
        SyncCommand::Pause => {
            metadata.sync_control.paused = true;
            store
                .save_runtime_metadata(&metadata, open_bitcoin_node::PersistMode::Sync)
                .map_err(|error| OperatorRuntimeError::InvalidRequest {
                    message: error.to_string(),
                })?;
            Ok(OperatorCommandOutcome::success(
                "Daemon sync paused. Use `open-bitcoin sync resume` to continue.\n",
            ))
        }
        SyncCommand::Resume => {
            metadata.sync_control.paused = false;
            store
                .save_runtime_metadata(&metadata, open_bitcoin_node::PersistMode::Sync)
                .map_err(|error| OperatorRuntimeError::InvalidRequest {
                    message: error.to_string(),
                })?;
            Ok(OperatorCommandOutcome::success(
                "Daemon sync resumed. Use `open-bitcoin sync status` to inspect current state.\n",
            ))
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
