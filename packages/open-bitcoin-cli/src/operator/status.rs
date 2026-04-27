// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Operator status collection and rendering surface.

use std::{fmt, path::PathBuf, time::Duration};

use open_bitcoin_node::{
    FjallNodeStore, LogRetentionPolicy, MetricRetentionPolicy, MetricsStatus,
    logging::writer::load_log_status,
    status::{
        BuildProvenance, ChainTipStatus, ConfigStatus, FieldAvailability, HealthSignal,
        HealthSignalLevel, MempoolStatus, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot,
        PeerCounts, PeerStatus, ServiceStatus, SyncProgress, SyncStatus, WalletStatus,
    },
};
use open_bitcoin_rpc::method::{
    GetBalancesResponse, GetBlockchainInfoResponse, GetMempoolInfoResponse, GetNetworkInfoResponse,
    GetWalletInfoResponse,
};

use super::{
    NetworkSelection,
    config::{OperatorConfigPathKind, OperatorConfigResolution},
    detect::{
        DetectedInstallation, DetectionConfidence, DetectionSourcePathKind, DetectionUncertainty,
        ProductFamily, ServiceManager,
    },
};

mod render;
pub use render::render_status;

/// Operator status request supplied by CLI flags and config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusRequest {
    /// Requested render mode.
    pub render_mode: StatusRenderMode,
    /// Optional Open Bitcoin JSONC config path.
    pub maybe_config_path: Option<PathBuf>,
    /// Optional datadir path.
    pub maybe_data_dir: Option<PathBuf>,
    /// Optional Bitcoin network selection.
    pub maybe_network: Option<NetworkSelection>,
    /// Whether collectors may attempt live RPC.
    pub include_live_rpc: bool,
    /// Whether human output should disable color.
    pub no_color: bool,
}

/// Status rendering mode requested by the operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusRenderMode {
    /// Quiet human-readable status.
    Human,
    /// Stable serde-backed JSON rendering.
    Json,
}

/// Inputs needed by a status collector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusCollectorInput {
    /// Operator status request.
    pub request: StatusRequest,
    /// Config resolution evidence.
    pub config_resolution: OperatorConfigResolution,
    /// Read-only detection evidence.
    pub detection_evidence: StatusDetectionEvidence,
    /// Optional live RPC adapter input.
    pub maybe_live_rpc: Option<StatusLiveRpcAdapterInput>,
}

/// Detection evidence available to status collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusDetectionEvidence {
    /// Detected installations from the read-only detector.
    pub detected_installations: Vec<DetectedInstallation>,
}

/// Live RPC adapter input without credential values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusLiveRpcAdapterInput {
    /// RPC endpoint.
    pub endpoint: String,
    /// Credential source only; no credential contents.
    pub auth_source: StatusRpcAuthSource,
    /// Request timeout.
    pub timeout: Duration,
}

/// Live RPC auth source without sensitive values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusRpcAuthSource {
    /// Use a cookie file path.
    CookieFile {
        /// Cookie path only; never cookie contents.
        path: PathBuf,
    },
    /// User credentials were configured elsewhere, but values are not carried.
    UserCredentialsConfigured,
    /// No auth source is configured.
    None,
}

/// Live RPC adapter contract used by status collection.
pub trait StatusRpcClient {
    fn get_network_info(&self) -> Result<GetNetworkInfoResponse, StatusRpcError>;
    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, StatusRpcError>;
    fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, StatusRpcError>;
    fn get_wallet_info(&self) -> Result<GetWalletInfoResponse, StatusRpcError>;
    fn get_balances(&self) -> Result<GetBalancesResponse, StatusRpcError>;
}

/// Error returned by an injected status RPC adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusRpcError {
    message: String,
}

impl StatusRpcError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for StatusRpcError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for StatusRpcError {}

/// Build a shared status snapshot from local, detection, and optional live RPC evidence.
pub fn collect_status_snapshot(
    input: &StatusCollectorInput,
    maybe_rpc_client: Option<&dyn StatusRpcClient>,
) -> OpenBitcoinStatusSnapshot {
    if input.request.include_live_rpc
        && let Some(rpc_client) = maybe_rpc_client
    {
        return collect_live_status_snapshot(input, rpc_client);
    }

    stopped_status_snapshot(input, NodeRuntimeState::Stopped, "node stopped")
}

fn collect_live_status_snapshot(
    input: &StatusCollectorInput,
    rpc_client: &dyn StatusRpcClient,
) -> OpenBitcoinStatusSnapshot {
    let network_info = match rpc_client.get_network_info() {
        Ok(value) => value,
        Err(error) => {
            return stopped_status_snapshot(
                input,
                NodeRuntimeState::Unreachable,
                error.to_string(),
            );
        }
    };
    let blockchain_info = match rpc_client.get_blockchain_info() {
        Ok(value) => value,
        Err(error) => {
            return stopped_status_snapshot(
                input,
                NodeRuntimeState::Unreachable,
                error.to_string(),
            );
        }
    };
    let mempool_info = match rpc_client.get_mempool_info() {
        Ok(value) => value,
        Err(error) => {
            return stopped_status_snapshot(
                input,
                NodeRuntimeState::Unreachable,
                error.to_string(),
            );
        }
    };
    let _wallet_info = match rpc_client.get_wallet_info() {
        Ok(value) => value,
        Err(error) => {
            return stopped_status_snapshot(
                input,
                NodeRuntimeState::Unreachable,
                error.to_string(),
            );
        }
    };
    let balances = match rpc_client.get_balances() {
        Ok(value) => value,
        Err(error) => {
            return stopped_status_snapshot(
                input,
                NodeRuntimeState::Unreachable,
                error.to_string(),
            );
        }
    };

    let mut health_signals = detection_health_signals(&input.detection_evidence);
    health_signals.extend(rpc_warning_signals(
        &network_info.warnings,
        &blockchain_info.warnings,
    ));
    health_signals.extend(log_health_signals(input));

    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: node_version(&network_info),
        },
        config: config_status(&input.config_resolution),
        service: service_status(&input.detection_evidence),
        sync: SyncStatus {
            network: FieldAvailability::available(blockchain_info.chain.clone()),
            chain_tip: FieldAvailability::available(ChainTipStatus {
                height: u64::from(blockchain_info.blocks),
                block_hash: blockchain_info
                    .maybe_best_block_hash
                    .unwrap_or_else(|| "unknown".to_string()),
            }),
            sync_progress: FieldAvailability::available(SyncProgress {
                header_height: u64::from(blockchain_info.headers),
                block_height: u64::from(blockchain_info.blocks),
                progress_ratio: blockchain_info.verificationprogress,
                messages_processed: 0,
                headers_received: u64::from(blockchain_info.headers),
                blocks_received: u64::from(blockchain_info.blocks),
            }),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: saturating_usize_to_u32(network_info.connections_in),
                outbound: saturating_usize_to_u32(network_info.connections_out),
            }),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(saturating_usize_to_u64(mempool_info.size)),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::available(non_negative_i64_to_u64(
                balances.mine.trusted_sats,
            )),
        },
        logs: log_status(&input.config_resolution),
        metrics: metrics_status(&input.config_resolution),
        health_signals,
        build: BuildProvenance::unavailable(),
    }
}

fn stopped_status_snapshot(
    input: &StatusCollectorInput,
    state: NodeRuntimeState,
    reason: impl Into<String>,
) -> OpenBitcoinStatusSnapshot {
    let reason = reason.into();
    let mut health_signals = detection_health_signals(&input.detection_evidence);
    health_signals.extend(log_health_signals(input));
    if state == NodeRuntimeState::Unreachable {
        health_signals.push(HealthSignal {
            level: HealthSignalLevel::Warn,
            source: "rpc".to_string(),
            message: reason.clone(),
        });
    }

    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state,
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        config: config_status(&input.config_resolution),
        service: service_status(&input.detection_evidence),
        sync: SyncStatus {
            network: FieldAvailability::unavailable(reason.clone()),
            chain_tip: FieldAvailability::unavailable(reason.clone()),
            sync_progress: FieldAvailability::unavailable(reason.clone()),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::unavailable(reason.clone()),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable(reason.clone()),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::unavailable(reason),
        },
        logs: log_status(&input.config_resolution),
        metrics: metrics_status(&input.config_resolution),
        health_signals,
        build: BuildProvenance::unavailable(),
    }
}

fn config_status(resolution: &OperatorConfigResolution) -> ConfigStatus {
    let datadir = resolution
        .maybe_data_dir
        .as_ref()
        .map(|path| FieldAvailability::available(path.display().to_string()))
        .unwrap_or_else(|| FieldAvailability::unavailable("datadir unavailable"));
    let mut config_paths = Vec::new();
    for report in &resolution.path_reports {
        if matches!(
            report.kind,
            OperatorConfigPathKind::ConfigFile | OperatorConfigPathKind::BitcoinConf
        ) {
            config_paths.push(report.path.display().to_string());
        }
    }
    if config_paths.is_empty() {
        if let Some(path) = resolution.maybe_config_path.as_ref() {
            config_paths.push(path.display().to_string());
        }
        if let Some(path) = resolution.maybe_bitcoin_conf_path.as_ref() {
            config_paths.push(path.display().to_string());
        }
    }

    ConfigStatus {
        datadir,
        config_paths,
    }
}

fn service_status(evidence: &StatusDetectionEvidence) -> ServiceStatus {
    let maybe_candidate = evidence
        .detected_installations
        .iter()
        .flat_map(|installation| installation.service_candidates.iter())
        .find(|candidate| candidate.present);

    if let Some(candidate) = maybe_candidate {
        return ServiceStatus {
            manager: FieldAvailability::available(service_manager_name(candidate.manager)),
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::unavailable("service manager not inspected"),
            running: FieldAvailability::unavailable("service manager not inspected"),
        };
    }

    ServiceStatus {
        manager: FieldAvailability::unavailable("service manager not inspected"),
        installed: FieldAvailability::unavailable("service manager not inspected"),
        enabled: FieldAvailability::unavailable("service manager not inspected"),
        running: FieldAvailability::unavailable("service manager not inspected"),
    }
}

fn log_status(resolution: &OperatorConfigResolution) -> open_bitcoin_node::LogStatus {
    let Some(log_dir) = resolution.maybe_log_dir.as_ref() else {
        return open_bitcoin_node::LogStatus::default();
    };
    load_log_status(log_dir, LogRetentionPolicy::default(), 10)
}

fn metrics_status(resolution: &OperatorConfigResolution) -> MetricsStatus {
    let retention = MetricRetentionPolicy::default();
    let Some(metrics_path) = resolution.maybe_metrics_store_path.as_ref() else {
        return MetricsStatus::default();
    };
    if !metrics_path.is_dir() {
        return MetricsStatus::unavailable(
            retention,
            format!(
                "metrics history unavailable: {} does not exist",
                metrics_path.display()
            ),
        );
    }
    match FjallNodeStore::open(metrics_path)
        .and_then(|store| store.load_metrics_status(MetricRetentionPolicy::default()))
    {
        Ok(status) => status,
        Err(error) => {
            MetricsStatus::unavailable(retention, format!("metrics history unavailable: {error}"))
        }
    }
}

fn log_health_signals(input: &StatusCollectorInput) -> Vec<HealthSignal> {
    open_bitcoin_node::logging::health_signals_from_recent_logs(
        &log_status(&input.config_resolution).recent_signals,
    )
}

fn detection_health_signals(evidence: &StatusDetectionEvidence) -> Vec<HealthSignal> {
    evidence
        .detected_installations
        .iter()
        .map(detection_health_signal)
        .collect()
}

fn detection_health_signal(installation: &DetectedInstallation) -> HealthSignal {
    let present_paths = installation
        .source_paths
        .iter()
        .filter(|source_path| source_path.present)
        .map(|source_path| {
            format!(
                "{}:{}",
                source_path_kind_name(source_path.kind),
                source_path.path.display()
            )
        })
        .collect::<Vec<_>>();
    let uncertainty = installation
        .uncertainty
        .iter()
        .map(|value| uncertainty_name(*value))
        .collect::<Vec<_>>();
    let confidence = confidence_name(installation.confidence);
    let product = product_family_name(installation.product_family);
    let message = format!(
        "uncertain {product} candidate; confidence={confidence}; paths=[{}]; uncertainty=[{}]",
        present_paths.join(", "),
        uncertainty.join(", ")
    );

    HealthSignal {
        level: HealthSignalLevel::Info,
        source: "detection".to_string(),
        message,
    }
}

fn rpc_warning_signals(
    network_warnings: &[String],
    chain_warnings: &[String],
) -> Vec<HealthSignal> {
    network_warnings
        .iter()
        .chain(chain_warnings.iter())
        .map(|warning| HealthSignal {
            level: HealthSignalLevel::Warn,
            source: "rpc".to_string(),
            message: warning.clone(),
        })
        .collect()
}

fn node_version(network_info: &GetNetworkInfoResponse) -> String {
    if network_info.subversion.is_empty() {
        return network_info.version.to_string();
    }
    network_info.subversion.clone()
}

fn service_manager_name(manager: ServiceManager) -> String {
    match manager {
        ServiceManager::Launchd => "launchd".to_string(),
        ServiceManager::Systemd => "systemd".to_string(),
        ServiceManager::Unknown => "unknown".to_string(),
    }
}

fn product_family_name(product_family: ProductFamily) -> &'static str {
    match product_family {
        ProductFamily::BitcoinCore => "bitcoin_core",
        ProductFamily::BitcoinKnots => "bitcoin_knots",
        ProductFamily::OpenBitcoin => "open_bitcoin",
        ProductFamily::Unknown => "unknown",
    }
}

fn confidence_name(confidence: DetectionConfidence) -> &'static str {
    match confidence {
        DetectionConfidence::High => "high",
        DetectionConfidence::Medium => "medium",
        DetectionConfidence::Low => "low",
    }
}

fn uncertainty_name(uncertainty: DetectionUncertainty) -> &'static str {
    match uncertainty {
        DetectionUncertainty::ProductAmbiguous => "product_ambiguous",
        DetectionUncertainty::MissingConfig => "missing_config",
        DetectionUncertainty::MissingCookie => "missing_cookie",
        DetectionUncertainty::ServiceManagerUnknown => "service_manager_unknown",
        DetectionUncertainty::WalletFormatUnknown => "wallet_format_unknown",
    }
}

fn source_path_kind_name(kind: DetectionSourcePathKind) -> &'static str {
    match kind {
        DetectionSourcePathKind::DataDir => "datadir",
        DetectionSourcePathKind::ConfigFile => "config",
        DetectionSourcePathKind::CookieFile => "cookie",
        DetectionSourcePathKind::ServiceDefinition => "service",
        DetectionSourcePathKind::WalletDirectory => "wallet_dir",
        DetectionSourcePathKind::WalletFile => "wallet_file",
    }
}

fn saturating_usize_to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

fn saturating_usize_to_u64(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn non_negative_i64_to_u64(value: i64) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

#[cfg(test)]
mod tests;
