// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure dashboard projection model built from the shared status snapshot.

use open_bitcoin_node::{
    MetricKind, MetricSample,
    metrics::MetricsAvailability,
    status::{
        ChainTipStatus, FieldAvailability, HealthSignal, HealthSignalLevel, NodeRuntimeState,
        OpenBitcoinStatusSnapshot, PeerCounts, ServiceStatus, SyncLagStatus, SyncLifecycleState,
        SyncProgress, SyncResourcePressure, WalletFreshness, WalletScanProgress,
    },
};

/// Metric series rendered as dashboard charts.
pub const DASHBOARD_METRIC_KINDS: [MetricKind; 5] = [
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
                row("Network", string_availability(&snapshot.sync.network)),
                row("Chain", chain_tip_availability(&snapshot.sync.chain_tip)),
                row(
                    "Progress",
                    sync_progress_availability(&snapshot.sync.sync_progress),
                ),
                row("State", sync_lifecycle(&snapshot.sync.lifecycle)),
                row("Phase", string_availability(&snapshot.sync.phase)),
                row("Lag", sync_lag(&snapshot.sync.lag)),
                row("Pressure", sync_pressure(&snapshot.sync.resource_pressure)),
                row(
                    "Peers",
                    peer_counts_availability(&snapshot.peers.peer_counts),
                ),
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

fn sync_lifecycle(value: &FieldAvailability<SyncLifecycleState>) -> String {
    match value {
        FieldAvailability::Available(value) => sync_lifecycle_name(*value).to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_lag(value: &FieldAvailability<SyncLagStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "headers={} blocks={}",
            value.headers_remaining, value.blocks_remaining
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_pressure(value: &FieldAvailability<SyncResourcePressure>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "blocks {}/{} peers {}/{}",
            value.blocks_in_flight,
            value.max_blocks_in_flight_total,
            value.outbound_peers,
            value.target_outbound_peers
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
    vec![
        row("Manager", string_availability(&service.manager)),
        row("Installed", bool_availability(&service.installed)),
        row("Enabled", bool_availability(&service.enabled)),
        row("Running", bool_availability(&service.running)),
    ]
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

#[cfg(test)]
mod tests {
    use open_bitcoin_node::{
        MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus,
        status::{
            BuildProvenance, ConfigStatus, FieldAvailability, HealthSignal, HealthSignalLevel,
            MempoolStatus, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot, PeerCounts,
            PeerStatus, ServiceStatus, SyncStatus, WalletFreshness, WalletStatus,
        },
    };

    use super::{DASHBOARD_METRIC_KINDS, DashboardState, derive_metric_points};

    #[test]
    fn dashboard_projection_includes_required_sections_and_charts() {
        // Arrange
        let snapshot = test_snapshot();

        // Act
        let state = DashboardState::from_snapshot(&snapshot);

        // Assert
        let titles = state
            .sections
            .iter()
            .map(|section| section.title.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            titles,
            vec![
                "Node",
                "Sync and Peers",
                "Mempool and Wallet",
                "Service",
                "Logs and Health"
            ]
        );
        assert_eq!(state.charts.len(), DASHBOARD_METRIC_KINDS.len());
        assert!(state.actions.iter().any(|action| action.destructive));
        let wallet_rows = &state.sections[2].rows;
        assert_eq!(wallet_rows[2].label, "Freshness");
        assert_eq!(wallet_rows[2].value, "fresh");
    }

    #[test]
    fn derive_metric_points_is_width_bounded() {
        // Arrange
        let samples = vec![
            MetricSample::new(MetricKind::SyncHeight, 1.0, 1),
            MetricSample::new(MetricKind::SyncHeight, 2.0, 2),
            MetricSample::new(MetricKind::SyncHeight, 3.0, 3),
        ];

        // Act
        let points = derive_metric_points(&samples, 2);

        // Assert
        assert_eq!(points, vec![2, 3]);
    }

    fn test_snapshot() -> OpenBitcoinStatusSnapshot {
        OpenBitcoinStatusSnapshot {
            node: NodeStatus {
                state: NodeRuntimeState::Running,
                version: "0.1.0".to_string(),
            },
            config: ConfigStatus {
                datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
                config_paths: vec!["/tmp/open-bitcoin/bitcoin.conf".to_string()],
            },
            service: ServiceStatus {
                manager: FieldAvailability::available("launchd".to_string()),
                installed: FieldAvailability::available(true),
                enabled: FieldAvailability::available(true),
                running: FieldAvailability::available(true),
            },
            sync: SyncStatus {
                network: FieldAvailability::available("regtest".to_string()),
                chain_tip: FieldAvailability::unavailable("no tip"),
                sync_progress: FieldAvailability::unavailable("no sync"),
                lifecycle: FieldAvailability::unavailable("no sync lifecycle"),
                phase: FieldAvailability::unavailable("no sync phase"),
                lag: FieldAvailability::unavailable("no sync lag"),
                last_error: FieldAvailability::unavailable("no sync error"),
                recovery_action: FieldAvailability::unavailable("no recovery action"),
                resource_pressure: FieldAvailability::unavailable("no sync pressure"),
            },
            peers: PeerStatus {
                peer_counts: FieldAvailability::available(PeerCounts {
                    inbound: 1,
                    outbound: 2,
                }),
                recent_peers: FieldAvailability::unavailable("no peer telemetry"),
            },
            mempool: MempoolStatus {
                transactions: FieldAvailability::available(4),
            },
            wallet: WalletStatus {
                trusted_balance_sats: FieldAvailability::available(50_000),
                freshness: FieldAvailability::available(WalletFreshness::Fresh),
                scan_progress: FieldAvailability::unavailable("wallet already fresh"),
            },
            logs: open_bitcoin_node::LogStatus::default(),
            metrics: MetricsStatus::available_with_samples(
                MetricRetentionPolicy::default(),
                vec![MetricSample::new(MetricKind::SyncHeight, 100.0, 10)],
            ),
            health_signals: vec![HealthSignal {
                level: HealthSignalLevel::Info,
                source: "test".to_string(),
                message: "ok".to_string(),
            }],
            build: BuildProvenance::unavailable(),
        }
    }
}
