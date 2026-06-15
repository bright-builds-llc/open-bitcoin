// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Human and JSON status rendering.

use open_bitcoin_node::{
    MetricsStatus, RecoveryEvidenceSnapshot,
    status::{
        BuildProvenance, ChainTipStatus, FieldAvailability, HealthSignal, HealthSignalLevel,
        NodeRuntimeState, OpenBitcoinStatusSnapshot, PeerCounts, PeerTelemetry,
        ResourceBoundSnapshot, ResourcePressureState, ServiceLifecycleStatus,
        ServicePriorShutdownStatus, ServiceRestartResumeStatus, ServiceResumeProgressStatus,
        ServiceStaleInflightStatus, ServiceStatus, SyncAttemptCounters, SyncConfiguredTargets,
        SyncLifecycleState, SyncProgressSignal, SyncRecoveryCategory, SyncResourcePressure,
        SyncStopReasonStatus, WalletFreshness, WalletScanProgress,
    },
};
use serde::Serialize;

use crate::operator::sync_truth_render::{
    best_known_tip_text, no_progress_diagnosis_text, stay_current_text, sync_progress_text,
    sync_reconcile_text, sync_reorg_text,
};

use super::StatusRenderMode;

/// Render a shared status snapshot as stable JSON or quiet human output.
pub fn render_status(
    snapshot: &OpenBitcoinStatusSnapshot,
    mode: StatusRenderMode,
) -> Result<String, serde_json::Error> {
    match mode {
        StatusRenderMode::Json => serde_json::to_string_pretty(snapshot),
        StatusRenderMode::Human => Ok(render_human_status(snapshot)),
    }
}

fn render_human_status(snapshot: &OpenBitcoinStatusSnapshot) -> String {
    let mut lines = Vec::new();
    let prominent_warnings = prominent_health_text(&snapshot.health_signals);
    if !prominent_warnings.is_empty() {
        lines.push(format!("Warnings: {prominent_warnings}"));
    }
    lines.push(format!(
        "Daemon: {}",
        runtime_state_name(snapshot.node.state)
    ));
    lines.push(format!("Version: {}", snapshot.node.version));
    lines.push(format!("Build: {}", build_text(&snapshot.build)));
    lines.push(format!(
        "Datadir: {}",
        string_availability(&snapshot.config.datadir)
    ));
    lines.push(format!(
        "Config: {}",
        if snapshot.config.config_paths.is_empty() {
            "Unavailable: config paths unavailable".to_string()
        } else {
            snapshot.config.config_paths.join(", ")
        }
    ));
    lines.push(format!(
        "Network: {}",
        string_availability(&snapshot.sync.network)
    ));
    lines.push(format!(
        "Chain: {}",
        chain_tip_availability(&snapshot.sync.chain_tip)
    ));
    lines.push(format!(
        "Sync state: {}",
        sync_lifecycle_availability(&snapshot.sync.lifecycle)
    ));
    lines.push(format!(
        "Sync phase: {}",
        string_availability(&snapshot.sync.phase)
    ));
    lines.push(format!(
        "Sync configured targets: {}",
        sync_configured_targets_availability(&snapshot.sync.configured_targets)
    ));
    lines.push(format!(
        "Sync attempts: {}",
        sync_attempt_counters_availability(&snapshot.sync.attempt_counters)
    ));
    lines.push(format!(
        "Sync signal: {}",
        sync_progress_signal_availability(&snapshot.sync.progress_signal)
    ));
    lines.push(format!(
        "Sync best-known tip: {}",
        best_known_tip_text(&snapshot.sync.best_known_tip)
    ));
    lines.push(format!(
        "Sync stay-current: {}",
        stay_current_text(&snapshot.sync.stay_current)
    ));
    lines.push(format!(
        "Sync stay-current action: {}",
        string_availability(&snapshot.sync.stay_current_next_action)
    ));
    lines.push(format!(
        "Sync no-progress diagnosis: {}",
        no_progress_diagnosis_text(&snapshot.sync.no_progress_diagnosis)
    ));
    lines.push(format!(
        "Sync no-progress action: {}",
        string_availability(&snapshot.sync.no_progress_next_action)
    ));
    lines.push(format!(
        "Sync last progress: {}",
        u64_availability(
            &snapshot.sync.last_successful_progress_unix_seconds,
            "unix seconds"
        )
    ));
    lines.push(format!(
        "Sync latest stop reason: {}",
        sync_stop_reason_availability(&snapshot.sync.latest_stop_reason)
    ));
    lines.push(format!(
        "Sync error: {}",
        string_availability(&snapshot.sync.last_error)
    ));
    lines.push(format!(
        "Sync recovery category: {}",
        sync_recovery_category_availability(&snapshot.sync.recovery_category)
    ));
    lines.push(format!(
        "Sync recovery: {}",
        string_availability(&snapshot.sync.recovery_action)
    ));
    lines.push(format!(
        "Recovery evidence: {}",
        recovery_evidence_availability(&snapshot.recovery_evidence)
    ));
    lines.push(format!(
        "Sync pressure: {}",
        sync_pressure_availability(&snapshot.sync.resource_pressure)
    ));
    lines.push(format!(
        "Resource bounds: {}",
        resource_bounds_availability(&snapshot.resource_bounds)
    ));
    lines.push(format!(
        "Sync latest reorg: {}",
        sync_reorg_text(&snapshot.sync.latest_reorg)
    ));
    lines.push(format!(
        "Sync reconcile: {}",
        sync_reconcile_text(&snapshot.sync.reconcile_progress)
    ));
    lines.push(format!(
        "Peers: {}",
        peer_counts_availability(&snapshot.peers.peer_counts)
    ));
    lines.push(format!(
        "Peer detail: {}",
        peer_telemetry_availability(&snapshot.peers.recent_peers)
    ));
    lines.push(format!(
        "Sync: {}",
        sync_progress_text(&snapshot.sync.sync_progress)
    ));
    lines.push(format!(
        "Mempool: {}",
        u64_availability(&snapshot.mempool.transactions, "transactions")
    ));
    lines.push(format!(
        "Wallet: {}",
        u64_availability(&snapshot.wallet.trusted_balance_sats, "trusted sats")
    ));
    lines.push(format!(
        "Wallet freshness: {}",
        wallet_freshness_availability(&snapshot.wallet.freshness)
    ));
    lines.push(format!(
        "Wallet scan: {}",
        wallet_scan_progress_availability(&snapshot.wallet.scan_progress)
    ));
    lines.push(format!("Service: {}", service_text(&snapshot.service)));
    lines.push(format!("Logs: {}", log_text(&snapshot.logs)));
    lines.push(format!("Metrics: {}", metrics_text(&snapshot.metrics)));
    lines.push(format!("Health: {}", health_text(&snapshot.health_signals)));
    lines.join("\n")
}

fn string_availability(value: &FieldAvailability<String>) -> String {
    match value {
        FieldAvailability::Available(value) => value.clone(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn chain_tip_availability(value: &FieldAvailability<ChainTipStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => {
            format!("height {} {}", value.height, value.block_hash)
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_lifecycle_availability(value: &FieldAvailability<SyncLifecycleState>) -> String {
    match value {
        FieldAvailability::Available(value) => sync_lifecycle_name(*value).to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_configured_targets_availability(
    value: &FieldAvailability<SyncConfiguredTargets>,
) -> String {
    match value {
        FieldAvailability::Available(value) => {
            let target_header_height = value.maybe_target_header_height.map_or_else(
                || "Unavailable: no target header configured".to_string(),
                |height| height.to_string(),
            );
            format!(
                "outbound_peers={} target_header_height={target_header_height}",
                value.target_outbound_peers
            )
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_attempt_counters_availability(value: &FieldAvailability<SyncAttemptCounters>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "attempted_peers={} connected_peers={} failed_peers={} max_sync_rounds={}",
            value.attempted_peers, value.connected_peers, value.failed_peers, value.max_sync_rounds
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_progress_signal_availability(value: &FieldAvailability<SyncProgressSignal>) -> String {
    match value {
        FieldAvailability::Available(value) => sync_progress_signal_name(*value).to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_stop_reason_availability(value: &FieldAvailability<SyncStopReasonStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => value.label.clone(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_pressure_availability(value: &FieldAvailability<SyncResourcePressure>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
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
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn resource_bounds_availability(value: &FieldAvailability<ResourceBoundSnapshot>) -> String {
    match value {
        FieldAvailability::Available(value) => {
            let unavailable = value
                .entries
                .iter()
                .filter_map(|entry| match &entry.usage {
                    FieldAvailability::Unavailable { .. } => Some(entry.kind.as_str()),
                    FieldAvailability::Available(_) => None,
                })
                .collect::<Vec<_>>();
            let next_action = value
                .entries
                .iter()
                .filter_map(|entry| match &entry.usage {
                    FieldAvailability::Available(usage)
                        if usage.state == ResourcePressureState::StopRequired =>
                    {
                        Some(format!("{}: {}", entry.kind.as_str(), usage.next_action))
                    }
                    FieldAvailability::Available(usage)
                        if usage.state == ResourcePressureState::Warning =>
                    {
                        Some(format!("{}: {}", entry.kind.as_str(), usage.next_action))
                    }
                    FieldAvailability::Available(_) | FieldAvailability::Unavailable { .. } => None,
                })
                .next()
                .unwrap_or_else(|| "none".to_string());
            let unavailable_text = if unavailable.is_empty() {
                "none".to_string()
            } else {
                unavailable.join(",")
            };
            format!(
                "overall={} unavailable={} next_action={}",
                value.overall_level.as_str(),
                unavailable_text,
                next_action
            )
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_recovery_category_availability(value: &FieldAvailability<SyncRecoveryCategory>) -> String {
    match value {
        FieldAvailability::Available(value) => value.as_str().to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn recovery_evidence_availability(value: &FieldAvailability<RecoveryEvidenceSnapshot>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "category={} cause={} action_class={} next_action={}",
            stable_json_label(&value.category),
            stable_json_label(&value.cause),
            stable_json_label(&value.action_class),
            value.next_action
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn stable_json_label(value: &impl Serialize) -> String {
    match serde_json::to_value(value) {
        Ok(serde_json::Value::String(label)) => label,
        Ok(value) => value.to_string(),
        Err(_) => "unavailable_label".to_string(),
    }
}

fn peer_counts_availability(value: &FieldAvailability<PeerCounts>) -> String {
    match value {
        FieldAvailability::Available(value) => {
            format!("in={} out={}", value.inbound, value.outbound)
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn peer_telemetry_availability(value: &FieldAvailability<Vec<PeerTelemetry>>) -> String {
    match value {
        FieldAvailability::Available(value) if value.is_empty() => {
            "no recent peer telemetry".into()
        }
        FieldAvailability::Available(value) => value
            .iter()
            .map(|peer| format!("{}:{} via {}", peer.state, peer.peer, peer.source))
            .collect::<Vec<_>>()
            .join(" | "),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn u64_availability(value: &FieldAvailability<u64>, label: &str) -> String {
    match value {
        FieldAvailability::Available(value) => format!("{value} {label}"),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn wallet_freshness_availability(value: &FieldAvailability<WalletFreshness>) -> String {
    match value {
        FieldAvailability::Available(value) => wallet_freshness_name(*value).to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn wallet_scan_progress_availability(value: &FieldAvailability<WalletScanProgress>) -> String {
    match value {
        FieldAvailability::Available(value) => {
            let progress_ratio = wallet_scan_progress_ratio(value);
            format!(
                "height {}/{} ({:.2}%)",
                value.scanned_through_height,
                value.target_tip_height,
                progress_ratio * 100.0
            )
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn service_text(service: &ServiceStatus) -> String {
    format!(
        "lifecycle={} manager={} installed={} enabled={} running={} file={} logs={} diagnostics={} restart_resume={}",
        service_lifecycle_availability(&service.lifecycle),
        string_availability(&service.manager),
        bool_availability(&service.installed),
        bool_availability(&service.enabled),
        bool_availability(&service.running),
        string_availability(&service.service_file_path),
        string_availability(&service.log_path),
        string_availability(&service.diagnostics),
        service_restart_resume_availability(&service.restart_resume)
    )
}

fn service_lifecycle_availability(value: &FieldAvailability<ServiceLifecycleStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => value.as_str().to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn bool_availability(value: &FieldAvailability<bool>) -> String {
    match value {
        FieldAvailability::Available(value) => value.to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn service_restart_resume_availability(
    value: &FieldAvailability<ServiceRestartResumeStatus>,
) -> String {
    match value {
        FieldAvailability::Available(value) => service_restart_resume_text(value),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn service_restart_resume_text(value: &ServiceRestartResumeStatus) -> String {
    format!(
        "datadir={} same_datadir={} prior_shutdown={} {} stale_inflight={} recovery_category={} next_action={}",
        string_availability(&value.datadir),
        bool_availability(&value.same_datadir),
        prior_shutdown_availability(&value.prior_shutdown),
        resume_progress_availability(&value.durable_progress),
        stale_inflight_availability(&value.stale_inflight),
        sync_recovery_category_availability(&value.recovery_category),
        string_availability(&value.next_action)
    )
}

fn prior_shutdown_availability(value: &FieldAvailability<ServicePriorShutdownStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => value.as_str().to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn stale_inflight_availability(value: &FieldAvailability<ServiceStaleInflightStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => value.as_str().to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn resume_progress_availability(value: &FieldAvailability<ServiceResumeProgressStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => {
            format!(
                "downloaded={} connected={}",
                value.downloaded_block_height, value.connected_block_height
            )
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn log_text(logs: &open_bitcoin_node::LogStatus) -> String {
    let path = match &logs.path {
        open_bitcoin_node::logging::LogPathStatus::Available { path } => path.clone(),
        open_bitcoin_node::logging::LogPathStatus::Unavailable { reason } => {
            format!("Unavailable: {reason}")
        }
    };
    format!("{} retention={} files", path, logs.retention.max_files)
}

fn metrics_text(metrics: &MetricsStatus) -> String {
    let availability = match &metrics.availability {
        open_bitcoin_node::metrics::MetricsAvailability::Available => "available".to_string(),
        open_bitcoin_node::metrics::MetricsAvailability::Unavailable { reason } => {
            format!("Unavailable: {reason}")
        }
    };
    format!(
        "{} retention={}s/{} samples history={}",
        availability,
        metrics.retention.sample_interval_seconds,
        metrics.retention.max_samples_per_series,
        metrics.samples.len()
    )
}

fn health_text(signals: &[HealthSignal]) -> String {
    if signals.is_empty() {
        return "ok".to_string();
    }
    signals
        .iter()
        .map(|signal| {
            format!(
                "{}:{}:{}",
                health_level_name(signal.level),
                signal.source,
                signal.message
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

fn prominent_health_text(signals: &[HealthSignal]) -> String {
    signals
        .iter()
        .filter(|signal| {
            matches!(
                signal.level,
                HealthSignalLevel::Warn | HealthSignalLevel::Error
            )
        })
        .map(|signal| {
            format!(
                "{}:{}:{}",
                health_level_name(signal.level),
                signal.source,
                signal.message
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

fn build_text(build: &BuildProvenance) -> String {
    format!(
        "version={} commit={} build_time={} target={} profile={}",
        build.version,
        string_availability(&build.commit),
        string_availability(&build.build_time),
        string_availability(&build.target),
        string_availability(&build.profile)
    )
}

fn runtime_state_name(state: NodeRuntimeState) -> &'static str {
    match state {
        NodeRuntimeState::Running => "running",
        NodeRuntimeState::Stopped => "stopped",
        NodeRuntimeState::Starting => "starting",
        NodeRuntimeState::Stopping => "stopping",
        NodeRuntimeState::Unreachable => "unreachable",
        NodeRuntimeState::Unknown => "unknown",
    }
}

fn health_level_name(level: HealthSignalLevel) -> &'static str {
    match level {
        HealthSignalLevel::Info => "info",
        HealthSignalLevel::Warn => "warn",
        HealthSignalLevel::Error => "error",
    }
}

fn wallet_freshness_name(freshness: WalletFreshness) -> &'static str {
    match freshness {
        WalletFreshness::Fresh => "fresh",
        WalletFreshness::Stale => "stale",
        WalletFreshness::Partial => "partial",
        WalletFreshness::Scanning => "scanning",
    }
}

fn wallet_scan_progress_ratio(progress: &WalletScanProgress) -> f64 {
    if progress.target_tip_height == 0 {
        return 0.0;
    }
    f64::from(progress.scanned_through_height) / f64::from(progress.target_tip_height)
}

fn sync_lifecycle_name(state: SyncLifecycleState) -> &'static str {
    match state {
        SyncLifecycleState::Active => "active",
        SyncLifecycleState::Paused => "paused",
        SyncLifecycleState::Recovering => "recovering",
        SyncLifecycleState::Failed => "failed",
        SyncLifecycleState::Stopped => "stopped",
    }
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

#[cfg(test)]
mod tests;
