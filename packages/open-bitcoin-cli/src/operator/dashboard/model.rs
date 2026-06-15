// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure dashboard projection model built from the shared status snapshot.

use open_bitcoin_node::{
    MetricKind, MetricSample, RecoveryEvidenceSnapshot,
    metrics::MetricsAvailability,
    status::{
        FieldAvailability, HealthSignal, HealthSignalLevel, NodeRuntimeState,
        OpenBitcoinStatusSnapshot, PeerCounts, ServiceLifecycleStatus, ServicePriorShutdownStatus,
        ServiceRestartResumeStatus, ServiceResumeProgressStatus, ServiceStaleInflightStatus,
        ServiceStatus, SyncAttemptCounters, SyncConfiguredTargets, SyncLifecycleState,
        SyncProgressSignal, SyncRecoveryCategory, SyncResourcePressure, SyncStopReasonStatus,
        WalletFreshness, WalletScanProgress,
    },
};

use crate::operator::sync_truth_render::{
    best_known_tip_text, no_progress_diagnosis_text, stay_current_text, sync_progress_text,
    sync_reconcile_text, sync_reorg_text,
};

mod resource_bounds;
#[cfg(test)]
mod tests;

use resource_bounds::resource_bounds;

/// Metric series rendered as dashboard charts.
pub const DASHBOARD_METRIC_KINDS: [MetricKind; 8] = [
    MetricKind::HeaderHeight,
    MetricKind::DownloadedBlockHeight,
    MetricKind::ConnectedBlockHeight,
    MetricKind::SyncHeight,
    MetricKind::PeerCount,
    MetricKind::MempoolTransactions,
    MetricKind::DiskUsageBytes,
    MetricKind::RpcHealth,
];

/// Dashboard projection consumed by text and interactive renderers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardState {
    pub sections: Vec<DashboardSection>,
    pub charts: Vec<MetricChart>,
    pub actions: Vec<ActionEntry>,
}

/// A compact named dashboard section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardSection {
    pub title: String,
    pub rows: Vec<DashboardRow>,
}

/// One label/value row in a section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardRow {
    pub label: String,
    pub value: String,
}

/// Bounded chart points for a dashboard metric.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricChart {
    pub kind: MetricKind,
    pub title: String,
    pub points: Vec<u64>,
    pub availability: String,
}

/// Operator action shown in the dashboard action bar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionEntry {
    pub key: String,
    pub label: String,
    pub destructive: bool,
}

impl DashboardState {
    /// Project a shared status snapshot into dashboard-ready sections and charts.
    pub fn from_snapshot(snapshot: &OpenBitcoinStatusSnapshot) -> Self {
        Self {
            sections: dashboard_sections(snapshot),
            charts: dashboard_charts(snapshot),
            actions: dashboard_actions(),
        }
    }
}

/// Return the last `width` non-negative metric values as sparkline-safe integers.
pub fn derive_metric_points(points: &[MetricSample], width: usize) -> Vec<u64> {
    if width == 0 {
        return Vec::new();
    }

    let start = points.len().saturating_sub(width);
    points.iter().skip(start).map(metric_value_to_u64).collect()
}

fn dashboard_sections(snapshot: &OpenBitcoinStatusSnapshot) -> Vec<DashboardSection> {
    vec![
        DashboardSection {
            title: "Node".to_string(),
            rows: vec![
                row("State", runtime_state_name(snapshot.node.state)),
                row("Version", snapshot.node.version.clone()),
                row("Build", build_summary(snapshot)),
                row("Datadir", string_availability(&snapshot.config.datadir)),
            ],
        },
        DashboardSection {
            title: "Sync and Peers".to_string(),
            rows: vec![
                row("State", sync_lifecycle(&snapshot.sync.lifecycle)),
                row("Phase", string_availability(&snapshot.sync.phase)),
                row(
                    "Configured targets",
                    sync_configured_targets(&snapshot.sync.configured_targets),
                ),
                row(
                    "Attempt counters",
                    sync_attempt_counters(&snapshot.sync.attempt_counters),
                ),
                row(
                    "Signal",
                    sync_progress_signal(&snapshot.sync.progress_signal),
                ),
                row(
                    "Best-known tip",
                    best_known_tip_text(&snapshot.sync.best_known_tip),
                ),
                row(
                    "Stay-current",
                    stay_current_text(&snapshot.sync.stay_current),
                ),
                row(
                    "Stay-current action",
                    string_availability(&snapshot.sync.stay_current_next_action),
                ),
                row(
                    "No-progress diagnosis",
                    no_progress_diagnosis_text(&snapshot.sync.no_progress_diagnosis),
                ),
                row(
                    "No-progress action",
                    string_availability(&snapshot.sync.no_progress_next_action),
                ),
                row(
                    "Last progress",
                    u64_availability(
                        &snapshot.sync.last_successful_progress_unix_seconds,
                        "unix seconds",
                    ),
                ),
                row(
                    "Latest stop reason",
                    sync_stop_reason(&snapshot.sync.latest_stop_reason),
                ),
                row("Last error", string_availability(&snapshot.sync.last_error)),
                row(
                    "Recovery category",
                    sync_recovery_category(&snapshot.sync.recovery_category),
                ),
                row(
                    "Recovery",
                    string_availability(&snapshot.sync.recovery_action),
                ),
                row(
                    "Recovery evidence",
                    recovery_evidence(&snapshot.recovery_evidence),
                ),
                row("Pressure", sync_pressure(&snapshot.sync.resource_pressure)),
                row(
                    "Resource bounds",
                    resource_bounds(&snapshot.resource_bounds),
                ),
                row("Latest reorg", sync_reorg_text(&snapshot.sync.latest_reorg)),
                row(
                    "Reconcile",
                    sync_reconcile_text(&snapshot.sync.reconcile_progress),
                ),
                row(
                    "Peers",
                    peer_counts_availability(&snapshot.peers.peer_counts),
                ),
                row("Progress", sync_progress_text(&snapshot.sync.sync_progress)),
            ],
        },
        DashboardSection {
            title: "Mempool and Wallet".to_string(),
            rows: vec![
                row(
                    "Mempool",
                    u64_availability(&snapshot.mempool.transactions, "transactions"),
                ),
                row(
                    "Wallet",
                    u64_availability(&snapshot.wallet.trusted_balance_sats, "trusted sats"),
                ),
                row("Freshness", wallet_freshness(&snapshot.wallet.freshness)),
                row("Scan", wallet_scan_progress(&snapshot.wallet.scan_progress)),
            ],
        },
        DashboardSection {
            title: "Service".to_string(),
            rows: service_rows(&snapshot.service),
        },
        DashboardSection {
            title: "Logs and Health".to_string(),
            rows: vec![
                row("Logs", log_summary(&snapshot.logs)),
                row("Metrics", metrics_summary(snapshot)),
                row("Health", health_summary(&snapshot.health_signals)),
            ],
        },
    ]
}

fn dashboard_charts(snapshot: &OpenBitcoinStatusSnapshot) -> Vec<MetricChart> {
    DASHBOARD_METRIC_KINDS
        .into_iter()
        .map(|kind| {
            let points = snapshot
                .metrics
                .samples
                .iter()
                .filter(|sample| sample.kind == kind)
                .cloned()
                .collect::<Vec<_>>();
            let chart_points = derive_metric_points(
                &points,
                snapshot.metrics.retention.max_samples_per_series.min(60),
            );
            MetricChart {
                kind,
                title: metric_label(kind).to_string(),
                availability: chart_availability(snapshot, &chart_points),
                points: chart_points,
            }
        })
        .collect()
}

fn dashboard_actions() -> Vec<ActionEntry> {
    vec![
        action("r", "refresh", false),
        action("s", "service status", false),
        action("t", "start service", true),
        action("o", "stop service", true),
        action("x", "restart service", true),
        action("i", "install service", true),
        action("u", "uninstall service", true),
        action("e", "enable service", true),
        action("d", "disable service", true),
        action("q", "quit", false),
    ]
}

fn row(label: impl Into<String>, value: impl Into<String>) -> DashboardRow {
    DashboardRow {
        label: label.into(),
        value: value.into(),
    }
}

fn action(key: impl Into<String>, label: impl Into<String>, destructive: bool) -> ActionEntry {
    ActionEntry {
        key: key.into(),
        label: label.into(),
        destructive,
    }
}

fn string_availability(value: &FieldAvailability<String>) -> String {
    match value {
        FieldAvailability::Available(value) => value.clone(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_lifecycle(value: &FieldAvailability<SyncLifecycleState>) -> String {
    match value {
        FieldAvailability::Available(value) => sync_lifecycle_name(*value).to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_configured_targets(value: &FieldAvailability<SyncConfiguredTargets>) -> String {
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

fn sync_attempt_counters(value: &FieldAvailability<SyncAttemptCounters>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "attempted_peers={} connected_peers={} failed_peers={} max_sync_rounds={}",
            value.attempted_peers, value.connected_peers, value.failed_peers, value.max_sync_rounds
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_progress_signal(value: &FieldAvailability<SyncProgressSignal>) -> String {
    match value {
        FieldAvailability::Available(value) => sync_progress_signal_name(*value).to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_stop_reason(value: &FieldAvailability<SyncStopReasonStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => value.label.clone(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_pressure(value: &FieldAvailability<SyncResourcePressure>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "headers {}/peer:{} blocks {}/{}/{} messages {} rounds {} peers {}/{}",
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

fn sync_recovery_category(value: &FieldAvailability<SyncRecoveryCategory>) -> String {
    match value {
        FieldAvailability::Available(value) => value.as_str().to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn recovery_evidence(value: &FieldAvailability<RecoveryEvidenceSnapshot>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "category={} cause={} action_class={} next_action={}",
            value.category.as_str(),
            serialized_label(&value.cause),
            serialized_label(&value.action_class),
            value.next_action
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn peer_counts_availability(value: &FieldAvailability<PeerCounts>) -> String {
    match value {
        FieldAvailability::Available(value) => {
            format!("inbound={} outbound={}", value.inbound, value.outbound)
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

fn bool_availability(value: &FieldAvailability<bool>) -> String {
    match value {
        FieldAvailability::Available(value) => value.to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn wallet_freshness(value: &FieldAvailability<WalletFreshness>) -> String {
    match value {
        FieldAvailability::Available(value) => wallet_freshness_name(*value).to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn wallet_scan_progress(value: &FieldAvailability<WalletScanProgress>) -> String {
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

fn service_rows(service: &ServiceStatus) -> Vec<DashboardRow> {
    let mut rows = vec![
        row("Lifecycle", service_lifecycle(&service.lifecycle)),
        row("Manager", string_availability(&service.manager)),
        row("Installed", bool_availability(&service.installed)),
        row("Enabled", bool_availability(&service.enabled)),
        row("Running", bool_availability(&service.running)),
        row(
            "Service file",
            string_availability(&service.service_file_path),
        ),
        row("Logs", string_availability(&service.log_path)),
        row("Diagnostics", string_availability(&service.diagnostics)),
    ];
    rows.extend(service_restart_resume_rows(&service.restart_resume));
    rows
}

fn service_lifecycle(value: &FieldAvailability<ServiceLifecycleStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => value.as_str().to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn service_restart_resume_rows(
    value: &FieldAvailability<ServiceRestartResumeStatus>,
) -> Vec<DashboardRow> {
    match value {
        FieldAvailability::Available(value) => vec![
            row(
                "Restart/resume",
                format!(
                    "datadir={} same_datadir={} recovery_category={}",
                    string_availability(&value.datadir),
                    bool_availability(&value.same_datadir),
                    sync_recovery_category(&value.recovery_category)
                ),
            ),
            row("Prior shutdown", prior_shutdown(&value.prior_shutdown)),
            row("Resume progress", resume_progress(&value.durable_progress)),
            row("Stale in-flight", stale_inflight(&value.stale_inflight)),
            row("Resume action", string_availability(&value.next_action)),
        ],
        FieldAvailability::Unavailable { reason } => {
            vec![row("Restart/resume", format!("Unavailable: {reason}"))]
        }
    }
}

fn prior_shutdown(value: &FieldAvailability<ServicePriorShutdownStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => value.as_str().to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn stale_inflight(value: &FieldAvailability<ServiceStaleInflightStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => value.as_str().to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn resume_progress(value: &FieldAvailability<ServiceResumeProgressStatus>) -> String {
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

fn log_summary(logs: &open_bitcoin_node::LogStatus) -> String {
    let path = match &logs.path {
        open_bitcoin_node::logging::LogPathStatus::Available { path } => path.clone(),
        open_bitcoin_node::logging::LogPathStatus::Unavailable { reason } => {
            format!("Unavailable: {reason}")
        }
    };
    format!(
        "{} retention={} files recent={}",
        path,
        logs.retention.max_files,
        logs.recent_signals.len()
    )
}

fn metrics_summary(snapshot: &OpenBitcoinStatusSnapshot) -> String {
    let availability = match &snapshot.metrics.availability {
        MetricsAvailability::Available => "available".to_string(),
        MetricsAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    };
    format!(
        "{} retention={}s/{} samples history={}",
        availability,
        snapshot.metrics.retention.sample_interval_seconds,
        snapshot.metrics.retention.max_samples_per_series,
        snapshot.metrics.samples.len()
    )
}

fn health_summary(signals: &[HealthSignal]) -> String {
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

fn build_summary(snapshot: &OpenBitcoinStatusSnapshot) -> String {
    format!(
        "version={} commit={} build_time={} target={} profile={}",
        snapshot.build.version,
        availability_text(&snapshot.build.commit),
        availability_text(&snapshot.build.build_time),
        availability_text(&snapshot.build.target),
        availability_text(&snapshot.build.profile),
    )
}

fn availability_text(value: &FieldAvailability<String>) -> String {
    match value {
        FieldAvailability::Available(value) => value.clone(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn chart_availability(snapshot: &OpenBitcoinStatusSnapshot, points: &[u64]) -> String {
    match &snapshot.metrics.availability {
        MetricsAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
        MetricsAvailability::Available if points.is_empty() => {
            "Unavailable: no metric samples recorded".to_string()
        }
        MetricsAvailability::Available => "available".to_string(),
    }
}

fn metric_value_to_u64(sample: &MetricSample) -> u64 {
    if !sample.value.is_finite() || sample.value <= 0.0 {
        return 0;
    }
    if sample.value >= u64::MAX as f64 {
        return u64::MAX;
    }
    sample.value.round() as u64
}

fn metric_label(kind: MetricKind) -> &'static str {
    match kind {
        MetricKind::SyncHeight => "Sync height",
        MetricKind::HeaderHeight => "Header height",
        MetricKind::DownloadedBlockHeight => "Downloaded block height",
        MetricKind::ConnectedBlockHeight => "Connected block height",
        MetricKind::ValidatedActiveChainHeight => "Validated active-chain height",
        MetricKind::PeerCount => "Peers",
        MetricKind::MempoolTransactions => "Mempool tx",
        MetricKind::WalletTrustedBalanceSats => "Wallet sats",
        MetricKind::DiskUsageBytes => "Disk bytes",
        MetricKind::RpcHealth => "RPC health",
        MetricKind::ServiceRestarts => "Service restarts",
    }
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

fn serialized_label<T>(value: &T) -> String
where
    T: serde::Serialize,
{
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}
