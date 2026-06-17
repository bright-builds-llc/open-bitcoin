// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{fmt, path::Path};

use open_bitcoin_node::{
    DurableSyncState, FieldAvailability, FjallNodeStore, RuntimeMetadata, SyncLifecycleState,
    status::{
        PeerCounts, SyncAttemptCounters, SyncConfiguredTargets, SyncProgress, SyncProgressSignal,
        SyncResourcePressure,
    },
};
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
            reject_offline_mutating_sync_conflict(&args.command, &metadata)?;
            metadata.sync_control.paused = true;
            store
                .save_runtime_metadata(&metadata, open_bitcoin_node::PersistMode::Sync)
                .map_err(|error| OperatorRuntimeError::InvalidRequest {
                    message: error.to_string(),
                })?;
            render_sync_outcome(&args.command, format, &metadata)
        }
        SyncCommand::Resume => {
            reject_offline_mutating_sync_conflict(&args.command, &metadata)?;
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

fn reject_offline_mutating_sync_conflict(
    command: &SyncCommand,
    metadata: &RuntimeMetadata,
) -> Result<(), OperatorRuntimeError> {
    if metadata.last_clean_shutdown {
        return Ok(());
    }
    let Some(lifecycle) = durable_sync_lifecycle(metadata) else {
        return Ok(());
    };
    if !matches!(
        lifecycle,
        SyncLifecycleState::Active
            | SyncLifecycleState::Paused
            | SyncLifecycleState::Recovering
            | SyncLifecycleState::Failed
    ) {
        return Ok(());
    }
    Err(OperatorRuntimeError::InvalidRequest {
        message: format!(
            "live daemon sync appears to own the durable store; live RPC was unavailable, so refusing offline sync {} to avoid a second-writer store conflict. Use live RPC, stop open-bitcoind cleanly, or inspect offline status before mutating durable sync control.",
            sync_command_display_name(command)
        ),
    })
}

fn durable_sync_lifecycle(metadata: &RuntimeMetadata) -> Option<SyncLifecycleState> {
    metadata
        .maybe_sync_state
        .as_ref()
        .and_then(|state| match state.sync.lifecycle {
            FieldAvailability::Available(value) => Some(value),
            FieldAvailability::Unavailable { .. } => None,
        })
}

fn sync_command_display_name(command: &SyncCommand) -> &'static str {
    match command {
        SyncCommand::Status => "status",
        SyncCommand::Pause => "pause",
        SyncCommand::Resume => "resume",
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

#[rustfmt::skip]
fn render_sync_status(metadata: &open_bitcoin_node::RuntimeMetadata) -> String {
    let maybe_sync_state = metadata.maybe_sync_state.as_ref();
    format!(
        "Sync paused: {}\nSync lifecycle: {}\nSync phase: {}\nConfigured targets: {}\nAttempt counters: {}\nProgress signal: {}\nLast progress: {}\nLatest stop reason: {}\nLatest error: {}\nRecovery category: {}\nRecovery action: {}\nResource pressure: {}\nPeer health: {}\nHeader height: {}\nDownloaded block: {}\nConnected block: {}\nBounded counters: {}\nLast clean shutdown: {}\nLast update: {}",
        metadata.sync_control.paused,
        sync_field(maybe_sync_state, |state| &state.sync.lifecycle, |value| format!("{value:?}").to_lowercase()),
        sync_field(maybe_sync_state, |state| &state.sync.phase, Clone::clone),
        sync_field(maybe_sync_state, |state| &state.sync.configured_targets, sync_targets_text),
        sync_field(maybe_sync_state, |state| &state.sync.attempt_counters, sync_attempts_text),
        sync_field(maybe_sync_state, |state| &state.sync.progress_signal, |value| sync_progress_signal_name(*value).to_string()),
        sync_field(maybe_sync_state, |state| &state.sync.last_successful_progress_unix_seconds, |value| format!("{value} unix seconds")),
        sync_field(maybe_sync_state, |state| &state.sync.latest_stop_reason, |value| value.label.clone()),
        sync_field(maybe_sync_state, |state| &state.sync.last_error, Clone::clone),
        sync_field(maybe_sync_state, |state| &state.sync.recovery_category, |value| value.as_str().to_string()),
        sync_field(maybe_sync_state, |state| &state.sync.recovery_action, Clone::clone),
        sync_field(maybe_sync_state, |state| &state.sync.resource_pressure, sync_pressure_text),
        sync_field(maybe_sync_state, |state| &state.peers.peer_counts, peer_counts_text),
        sync_field(maybe_sync_state, |state| &state.sync.sync_progress, |value| value.header_height.to_string()),
        sync_field(maybe_sync_state, |state| &state.sync.sync_progress, downloaded_block_text),
        sync_field(maybe_sync_state, |state| &state.sync.sync_progress, connected_block_text),
        sync_field(maybe_sync_state, |state| &state.sync.sync_progress, bounded_counters_text),
        metadata.last_clean_shutdown,
        maybe_sync_state.map_or_else(|| "Unavailable: sync state unavailable".to_string(), |state| state.updated_at_unix_seconds.to_string()),
    )
}

fn sync_field<T>(
    maybe_sync_state: Option<&DurableSyncState>,
    field: impl FnOnce(&DurableSyncState) -> &FieldAvailability<T>,
    render: impl FnOnce(&T) -> String,
) -> String {
    maybe_sync_state.map_or_else(
        || "Unavailable: sync state unavailable".to_string(),
        |state| field_text(field(state), render),
    )
}

fn field_text<T>(value: &FieldAvailability<T>, render: impl FnOnce(&T) -> String) -> String {
    match value {
        FieldAvailability::Available(value) => render(value),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_targets_text(value: &SyncConfiguredTargets) -> String {
    let target_header_height = value.maybe_target_header_height.map_or_else(
        || "Unavailable: no target header configured".to_string(),
        |height| height.to_string(),
    );
    format!(
        "outbound_peers={} target_header_height={target_header_height}",
        value.target_outbound_peers
    )
}

fn sync_attempts_text(value: &SyncAttemptCounters) -> String {
    format!(
        "attempted_peers={} connected_peers={} failed_peers={} max_sync_rounds={}",
        value.attempted_peers, value.connected_peers, value.failed_peers, value.max_sync_rounds
    )
}

fn sync_pressure_text(value: &SyncResourcePressure) -> String {
    format!(
        "header_requests_in_flight_per_peer={} headers_per_message={} blocks_in_flight={}/{}/{} messages_per_peer={} sync_rounds={} outbound_peers={}/{}",
        value.max_header_requests_in_flight_per_peer,
        value.max_headers_per_message,
        value.blocks_in_flight,
        value.max_blocks_in_flight_per_peer,
        value.max_blocks_in_flight_total,
        value.max_messages_per_peer,
        value.max_sync_rounds,
        value.outbound_peers,
        value.target_outbound_peers
    )
}

fn peer_counts_text(value: &PeerCounts) -> String {
    format!("inbound={} outbound={}", value.inbound, value.outbound)
}

fn downloaded_block_text(value: &SyncProgress) -> String {
    let hash = value
        .maybe_downloaded_block_hash
        .as_deref()
        .unwrap_or("Unavailable: no downloaded block hash recorded");
    format!("height={} hash={hash}", value.downloaded_block_height)
}

fn connected_block_text(value: &SyncProgress) -> String {
    let hash = value
        .maybe_connected_block_hash
        .as_deref()
        .unwrap_or("Unavailable: no connected block hash recorded");
    format!("height={} hash={hash}", value.connected_block_height)
}

fn bounded_counters_text(value: &SyncProgress) -> String {
    format!(
        "messages_processed={} headers_received={} blocks_received={}",
        value.messages_processed, value.headers_received, value.blocks_received
    )
}

fn sync_progress_signal_name(signal: SyncProgressSignal) -> &'static str {
    match signal {
        SyncProgressSignal::HeaderProgress => "header_progress",
        SyncProgressSignal::BlockProgress => "block_progress",
        SyncProgressSignal::WaitingForPeers => "waiting_for_peers",
        SyncProgressSignal::PeerFailures => "peer_failures",
        SyncProgressSignal::AwaitingBlocks => "awaiting_blocks",
        SyncProgressSignal::Steady => "steady",
    }
}

fn display_path(maybe_path: Option<&Path>) -> String {
    maybe_path
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "Unavailable".to_string())
}

fn path_to_string(path: &Path) -> String {
    path.display().to_string()
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    use open_bitcoin_node::{
        DurableSyncState, FieldAvailability, FjallNodeStore, PeerStatus, RuntimeMetadata,
        SyncLifecycleState, SyncStatus,
        status::{
            BestKnownTipStatus, PeerCounts, RECONCILE_PROGRESS_UNAVAILABLE_REASON,
            StayCurrentStatus, SyncAttemptCounters, SyncConfiguredTargets, SyncProgress,
            SyncProgressSignal, SyncRecoveryCategory, SyncResourcePressure, SyncStopReasonStatus,
        },
    };

    use super::{
        OperatorOutputFormat, SyncArgs, SyncCommand, execute_offline_sync_command,
        render_sync_status, sync_pressure_text,
    };

    static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

    fn temp_store_path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "open-bitcoin-cli-sync-control-{label}-{}",
            NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
        ))
    }

    #[rustfmt::skip]
    fn runtime_metadata_with_lifecycle(
        lifecycle: SyncLifecycleState,
        last_clean_shutdown: bool,
    ) -> RuntimeMetadata {
        let unavailable = "not needed for sync-control test";
        RuntimeMetadata {
            last_clean_shutdown,
            maybe_sync_state: Some(DurableSyncState {
                sync: SyncStatus {
                    network: FieldAvailability::available("mainnet".to_string()),
                    chain_tip: FieldAvailability::unavailable(unavailable),
                    sync_progress: FieldAvailability::unavailable(unavailable),
                    lifecycle: FieldAvailability::available(lifecycle),
                    phase: FieldAvailability::available("block_download".to_string()),
                    configured_targets: FieldAvailability::unavailable(unavailable),
                    attempt_counters: FieldAvailability::unavailable(unavailable),
                    progress_signal: FieldAvailability::available(SyncProgressSignal::AwaitingBlocks),
                    lag: FieldAvailability::unavailable(unavailable),
                    last_successful_progress_unix_seconds: FieldAvailability::unavailable(unavailable),
                    progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
                    expected_progress_window: FieldAvailability::unavailable("expected progress window unavailable"),
                    no_progress_threshold: FieldAvailability::unavailable("no-progress threshold evidence unavailable"),
                    last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
                    last_peer_contribution: FieldAvailability::unavailable("last peer contribution unavailable"),
                    stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
                    latest_stop_reason: FieldAvailability::unavailable(unavailable),
                    last_error: FieldAvailability::unavailable("no sync error recorded"),
                    recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
                    recovery_action: FieldAvailability::unavailable("no recovery action required"),
                    resource_pressure: FieldAvailability::unavailable(unavailable),
                    best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(unavailable),
                    stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(unavailable),
                    stay_current_next_action: FieldAvailability::unavailable(unavailable),
                    no_progress_diagnosis: FieldAvailability::unavailable("no-progress diagnosis unavailable"),
                    no_progress_next_action: FieldAvailability::unavailable("no-progress next action unavailable"),
                    latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
                    reconcile_progress: FieldAvailability::unavailable(RECONCILE_PROGRESS_UNAVAILABLE_REASON),
                },
                peers: PeerStatus {
                    peer_counts: FieldAvailability::unavailable(unavailable),
                    recent_peers: FieldAvailability::unavailable(unavailable),
                },
                health_signals: Vec::new(),
                updated_at_unix_seconds: 1,
            }),
            ..RuntimeMetadata::default()
        }
    }

    #[test]
    fn offline_pause_refuses_unclean_active_daemon_sync_state() {
        // Arrange
        let path = temp_store_path("active-conflict");
        let _ = fs::remove_dir_all(&path);
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_runtime_metadata(
                &runtime_metadata_with_lifecycle(SyncLifecycleState::Active, false),
                open_bitcoin_node::PersistMode::Sync,
            )
            .expect("save metadata");
        drop(store);
        let args = SyncArgs {
            command: SyncCommand::Pause,
        };

        // Act
        let error = execute_offline_sync_command(&path, &args, OperatorOutputFormat::Human)
            .expect_err("offline pause should fail");

        // Assert
        assert!(error.to_string().contains("second-writer store conflict"));
        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    fn offline_pause_allows_missing_sync_state() {
        // Arrange
        let path = temp_store_path("missing-state");
        let _ = fs::remove_dir_all(&path);
        let args = SyncArgs {
            command: SyncCommand::Pause,
        };

        // Act
        let outcome = execute_offline_sync_command(&path, &args, OperatorOutputFormat::Human)
            .expect("offline pause");
        let store = FjallNodeStore::open(&path).expect("store");
        let metadata = store
            .load_runtime_metadata()
            .expect("load metadata")
            .expect("metadata");

        // Assert
        assert!(outcome.stdout.text.contains("Daemon sync paused"));
        assert!(metadata.sync_control.paused);
        let _ = fs::remove_dir_all(&path);
    }

    #[test]
    #[rustfmt::skip]
    fn render_sync_status_surfaces_phase62_truth_fields() {
        // Arrange
        let mut metadata = runtime_metadata_with_lifecycle(SyncLifecycleState::Active, true);
        let state = metadata.maybe_sync_state.as_mut().expect("sync state");
        state.updated_at_unix_seconds = 1_717_000_001;
        state.sync.configured_targets = FieldAvailability::available(SyncConfiguredTargets { target_outbound_peers: 4, maybe_target_header_height: Some(840_200) });
        state.sync.attempt_counters = FieldAvailability::available(SyncAttemptCounters { attempted_peers: 3, connected_peers: 2, failed_peers: 1, max_sync_rounds: 8 });
        state.sync.progress_signal = FieldAvailability::available(SyncProgressSignal::HeaderProgress);
        state.sync.sync_progress = FieldAvailability::available(SyncProgress { header_height: 840_200, block_height: 840_004, downloaded_block_height: 840_006, connected_block_height: 840_004, validated_active_chain_height: 840_004, maybe_downloaded_block_hash: Some("22".repeat(32)), maybe_connected_block_hash: Some("11".repeat(32)), maybe_validated_active_chain_hash: Some("11".repeat(32)), maybe_validated_active_chain_work: Some("840005".to_string()), progress_ratio: 840_004.0 / 840_200.0, messages_processed: 7, headers_received: 3, blocks_received: 1 });
        state.sync.last_successful_progress_unix_seconds = FieldAvailability::available(1_717_000_000);
        state.sync.latest_stop_reason = FieldAvailability::available(SyncStopReasonStatus { label: "target_header_reached".to_string(), message: "sync header target reached".to_string() });
        state.sync.last_error = FieldAvailability::available("peer stalled before block connect".to_string());
        state.sync.recovery_category = FieldAvailability::available(SyncRecoveryCategory::InvalidPeerData);
        state.sync.recovery_action = FieldAvailability::available("Retry sync after peer backoff.".to_string());
        state.sync.resource_pressure = FieldAvailability::available(SyncResourcePressure { blocks_in_flight: 8, max_header_requests_in_flight_per_peer: 1, max_headers_per_message: 2_000, max_blocks_in_flight_per_peer: 16, max_blocks_in_flight_total: 64, max_messages_per_peer: 64, max_sync_rounds: 8, outbound_peers: 2, target_outbound_peers: 4 });
        state.peers.peer_counts = FieldAvailability::available(PeerCounts { inbound: 0, outbound: 2 });

        // Act
        let rendered = render_sync_status(&metadata);

        // Assert
        for expected in ["Sync paused: false", "Sync lifecycle: active", "Sync phase: block_download", "Configured targets: outbound_peers=4 target_header_height=840200", "Attempt counters: attempted_peers=3 connected_peers=2 failed_peers=1 max_sync_rounds=8", "Progress signal: header_progress", "Last progress: 1717000000 unix seconds", "Latest stop reason: target_header_reached", "Latest error: peer stalled before block connect", "Recovery category: invalid_peer_data", "Recovery action: Retry sync after peer backoff.", "Resource pressure: header_requests_in_flight_per_peer=1 headers_per_message=2000 blocks_in_flight=8/16/64 messages_per_peer=64 sync_rounds=8 outbound_peers=2/4", "Peer health: inbound=0 outbound=2", "Header height: 840200", &format!("Downloaded block: height=840006 hash={}", "22".repeat(32)), &format!("Connected block: height=840004 hash={}", "11".repeat(32)), "Bounded counters: messages_processed=7 headers_received=3 blocks_received=1", "Last clean shutdown: true", "Last update: 1717000001"] {
            assert!(rendered.contains(expected), "missing {expected}");
        }

        let state = metadata.maybe_sync_state.as_mut().expect("sync state");
        state.sync.configured_targets = FieldAvailability::unavailable("targets unavailable");
        state.sync.latest_stop_reason = FieldAvailability::unavailable("stop reason unavailable");
        if let FieldAvailability::Available(progress) = &mut state.sync.sync_progress {
            progress.maybe_downloaded_block_hash = None;
            progress.maybe_connected_block_hash = None;
        }
        let unavailable = render_sync_status(&metadata);
        assert!(unavailable.contains("Configured targets: Unavailable: targets unavailable"));
        assert!(unavailable.contains("Latest stop reason: Unavailable: stop reason unavailable"));
        assert!(unavailable.contains("Downloaded block: height=840006 hash=Unavailable: no downloaded block hash recorded"));
        assert!(unavailable.contains("Connected block: height=840004 hash=Unavailable: no connected block hash recorded"));
    }

    #[test]
    #[rustfmt::skip]
    fn phase71_runtime_support_resource_pressure_lists_all_configured_bounds() {
        let pressure = SyncResourcePressure { blocks_in_flight: 8, max_header_requests_in_flight_per_peer: 1, max_headers_per_message: 2_000, max_blocks_in_flight_per_peer: 16, max_blocks_in_flight_total: 64, max_messages_per_peer: 64, max_sync_rounds: 8, outbound_peers: 2, target_outbound_peers: 4 };
        assert_eq!(sync_pressure_text(&pressure), "header_requests_in_flight_per_peer=1 headers_per_message=2000 blocks_in_flight=8/16/64 messages_per_peer=64 sync_rounds=8 outbound_peers=2/4");
    }
}
