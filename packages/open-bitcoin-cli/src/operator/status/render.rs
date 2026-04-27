// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Human and JSON status rendering.

use open_bitcoin_node::{
    MetricsStatus,
    status::{
        BuildProvenance, ChainTipStatus, FieldAvailability, HealthSignal, HealthSignalLevel,
        NodeRuntimeState, OpenBitcoinStatusSnapshot, PeerCounts, ServiceStatus, SyncProgress,
    },
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
        "Sync: {}",
        sync_progress_availability(&snapshot.sync.sync_progress)
    ));
    lines.push(format!(
        "Peers: {}",
        peer_counts_availability(&snapshot.peers.peer_counts)
    ));
    lines.push(format!(
        "Mempool: {}",
        u64_availability(&snapshot.mempool.transactions, "transactions")
    ));
    lines.push(format!(
        "Wallet: {}",
        u64_availability(&snapshot.wallet.trusted_balance_sats, "trusted sats")
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

fn sync_progress_availability(value: &FieldAvailability<SyncProgress>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "{:.2}% blocks={}/{}",
            value.progress_ratio * 100.0,
            value.block_height,
            value.header_height
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
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

fn u64_availability(value: &FieldAvailability<u64>, label: &str) -> String {
    match value {
        FieldAvailability::Available(value) => format!("{value} {label}"),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn service_text(service: &ServiceStatus) -> String {
    format!(
        "manager={} installed={} enabled={} running={}",
        string_availability(&service.manager),
        bool_availability(&service.installed),
        bool_availability(&service.enabled),
        bool_availability(&service.running)
    )
}

fn bool_availability(value: &FieldAvailability<bool>) -> String {
    match value {
        FieldAvailability::Available(value) => value.to_string(),
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

fn build_text(build: &BuildProvenance) -> String {
    format!(
        "version={} commit={} target={} profile={}",
        build.version,
        string_availability(&build.commit),
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
