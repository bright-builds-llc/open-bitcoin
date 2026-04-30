// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Operator status collection and rendering surface.

use std::{
    fmt,
    path::{Path, PathBuf},
    time::Duration,
};

use open_bitcoin_node::{
    FjallNodeStore, LogRetentionPolicy, MetricRetentionPolicy, MetricsStatus, WalletRegistry,
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
    detect::{DetectedInstallation, ServiceCandidate},
    service::ServiceLifecycleState,
};

mod detection;
mod http;
mod render;
#[cfg(test)]
mod tests;
mod wallet;

pub use http::HttpStatusRpcClient;
pub use render::render_status;

/// Operator status request supplied by CLI flags and config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusRequest {
    pub render_mode: StatusRenderMode,
    pub maybe_config_path: Option<PathBuf>,
    pub maybe_data_dir: Option<PathBuf>,
    pub maybe_network: Option<NetworkSelection>,
    pub include_live_rpc: bool,
    pub no_color: bool,
}

/// Status rendering mode requested by the operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusRenderMode {
    Human,
    Json,
}

/// Inputs needed by a status collector.
pub struct StatusCollectorInput {
    pub request: StatusRequest,
    pub config_resolution: OperatorConfigResolution,
    pub detection_evidence: StatusDetectionEvidence,
    pub maybe_live_rpc: Option<StatusLiveRpcAdapterInput>,
    pub maybe_service_manager: Option<Box<dyn super::service::ServiceManager>>,
    pub wallet_rpc_access: StatusWalletRpcAccess,
}

/// Detection evidence available to status collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusDetectionEvidence {
    pub detected_installations: Vec<DetectedInstallation>,
    pub service_candidates: Vec<ServiceCandidate>,
}

/// Live RPC adapter input without credential values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusLiveRpcAdapterInput {
    pub endpoint: String,
    pub auth_source: StatusRpcAuthSource,
    pub timeout: Duration,
}

/// Live RPC auth source without sensitive values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusRpcAuthSource {
    CookieFile { path: PathBuf },
    UserCredentialsConfigured,
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
    maybe_code: Option<open_bitcoin_rpc::RpcErrorCode>,
    message: String,
}

impl StatusRpcError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            maybe_code: None,
            message: message.into(),
        }
    }

    pub fn from_rpc_detail(detail: open_bitcoin_rpc::RpcErrorDetail) -> Self {
        Self {
            maybe_code: Some(detail.code),
            message: detail.message,
        }
    }

    pub fn maybe_code(&self) -> Option<open_bitcoin_rpc::RpcErrorCode> {
        self.maybe_code
    }
}

impl fmt::Display for StatusRpcError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for StatusRpcError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusWalletRpcAccess {
    Root,
    Wallet(String),
    Unavailable { reason: String },
}

impl StatusWalletRpcAccess {
    pub(crate) fn maybe_wallet_name(&self) -> Option<&str> {
        match self {
            Self::Wallet(wallet_name) => Some(wallet_name.as_str()),
            Self::Root | Self::Unavailable { .. } => None,
        }
    }

    fn maybe_unavailable_reason(&self) -> Option<&str> {
        match self {
            Self::Unavailable { reason } => Some(reason.as_str()),
            Self::Root | Self::Wallet(_) => None,
        }
    }
}

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
    let mut health_signals = detection::detection_health_signals(&input.detection_evidence);
    health_signals.extend(rpc_warning_signals(
        &network_info.warnings,
        &blockchain_info.warnings,
    ));
    let wallet =
        collect_live_wallet_status(input, rpc_client, &blockchain_info, &mut health_signals);
    health_signals.extend(log_health_signals(input));

    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: node_version(&network_info),
        },
        config: config_status(&input.config_resolution),
        service: collect_service_status(input),
        sync: SyncStatus {
            network: FieldAvailability::available(blockchain_info.chain.clone()),
            chain_tip: FieldAvailability::available(ChainTipStatus {
                height: u64::from(blockchain_info.blocks),
                block_hash: blockchain_info
                    .maybe_best_block_hash
                    .clone()
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
        wallet,
        logs: log_status(&input.config_resolution),
        metrics: metrics_status(&input.config_resolution),
        health_signals,
        build: current_build_provenance(),
    }
}

fn stopped_status_snapshot(
    input: &StatusCollectorInput,
    state: NodeRuntimeState,
    reason: impl Into<String>,
) -> OpenBitcoinStatusSnapshot {
    let reason = reason.into();
    let mut health_signals = detection::detection_health_signals(&input.detection_evidence);
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
        service: collect_service_status(input),
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
            trusted_balance_sats: FieldAvailability::unavailable(reason.clone()),
            freshness: FieldAvailability::unavailable(reason.clone()),
            scan_progress: FieldAvailability::unavailable(reason),
        },
        logs: log_status(&input.config_resolution),
        metrics: metrics_status(&input.config_resolution),
        health_signals,
        build: current_build_provenance(),
    }
}

fn collect_live_wallet_status(
    input: &StatusCollectorInput,
    rpc_client: &dyn StatusRpcClient,
    blockchain_info: &GetBlockchainInfoResponse,
    health_signals: &mut Vec<HealthSignal>,
) -> WalletStatus {
    if let Some(reason) = input.wallet_rpc_access.maybe_unavailable_reason() {
        health_signals.push(wallet_health_signal(reason));
        return wallet::unavailable_wallet_status(reason);
    }

    let wallet_info = match rpc_client.get_wallet_info() {
        Ok(value) => value,
        Err(error) => {
            let reason = wallet_rpc_error_reason(&error);
            health_signals.push(wallet_health_signal(&reason));
            return wallet::unavailable_wallet_status(reason);
        }
    };
    let balances = match rpc_client.get_balances() {
        Ok(value) => value,
        Err(error) => {
            let reason = wallet_rpc_error_reason(&error);
            health_signals.push(wallet_health_signal(&reason));
            return wallet::unavailable_wallet_status(reason);
        }
    };

    wallet::live_wallet_status(
        &wallet_info,
        blockchain_info,
        non_negative_i64_to_u64(balances.mine.trusted_sats),
    )
}

fn wallet_rpc_error_reason(error: &StatusRpcError) -> String {
    match error.maybe_code() {
        Some(open_bitcoin_rpc::RpcErrorCode::WalletNotFound) => {
            "No wallet is loaded. Wallet fields are unavailable until a wallet is created or loaded."
                .to_string()
        }
        Some(open_bitcoin_rpc::RpcErrorCode::WalletNotSpecified) => {
            "Multiple wallets are loaded and no selected wallet metadata is available. Wallet fields are unavailable until one wallet is selected."
                .to_string()
        }
        _ => error.to_string(),
    }
}

fn wallet_health_signal(reason: impl Into<String>) -> HealthSignal {
    HealthSignal {
        level: HealthSignalLevel::Warn,
        source: "wallet".to_string(),
        message: reason.into(),
    }
}

pub(crate) fn resolve_status_wallet_rpc_access(
    maybe_data_dir: Option<&Path>,
) -> StatusWalletRpcAccess {
    let Some(data_dir) = maybe_data_dir else {
        return StatusWalletRpcAccess::Root;
    };
    let Ok(store) = FjallNodeStore::open(data_dir) else {
        return StatusWalletRpcAccess::Root;
    };
    let Ok(registry) = WalletRegistry::load(&store) else {
        return StatusWalletRpcAccess::Root;
    };
    wallet_rpc_access_from_registry(&registry)
}

fn wallet_rpc_access_from_registry(registry: &WalletRegistry) -> StatusWalletRpcAccess {
    if let Some(selected_wallet_name) = registry.selected_wallet_name() {
        return StatusWalletRpcAccess::Wallet(selected_wallet_name.to_string());
    }

    match registry.wallet_names() {
        [] => StatusWalletRpcAccess::Root,
        [wallet_name] => StatusWalletRpcAccess::Wallet(wallet_name.clone()),
        _ => StatusWalletRpcAccess::Unavailable {
            reason:
                "Multiple wallets are loaded and no selected wallet metadata is available. Wallet fields are unavailable until one wallet is selected."
                    .to_string(),
        },
    }
}

#[derive(Debug, Clone, Copy)]
struct BuildProvenanceInputs<'a> {
    version: &'a str,
    maybe_commit: Option<&'a str>,
    maybe_build_time: Option<&'a str>,
    maybe_target: Option<&'a str>,
    maybe_profile: Option<&'a str>,
}

fn current_build_provenance() -> BuildProvenance {
    build_provenance_from_inputs(BuildProvenanceInputs {
        version: env!("CARGO_PKG_VERSION"),
        maybe_commit: option_env!("OPEN_BITCOIN_BUILD_COMMIT"),
        maybe_build_time: option_env!("OPEN_BITCOIN_BUILD_TIME"),
        maybe_target: option_env!("OPEN_BITCOIN_BUILD_TARGET"),
        maybe_profile: option_env!("OPEN_BITCOIN_BUILD_PROFILE"),
    })
}

fn build_provenance_from_inputs(inputs: BuildProvenanceInputs<'_>) -> BuildProvenance {
    BuildProvenance {
        version: inputs.version.to_string(),
        commit: build_provenance_field(inputs.maybe_commit, "commit unavailable"),
        build_time: build_provenance_field(inputs.maybe_build_time, "build time unavailable"),
        target: build_provenance_field(inputs.maybe_target, "target unavailable"),
        profile: build_provenance_field(inputs.maybe_profile, "profile unavailable"),
    }
}

fn build_provenance_field(
    maybe_value: Option<&str>,
    unavailable_reason: &str,
) -> FieldAvailability<String> {
    let Some(value) = maybe_value.map(str::trim).filter(|value| !value.is_empty()) else {
        return FieldAvailability::unavailable(unavailable_reason);
    };

    FieldAvailability::available(value.to_string())
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

fn collect_service_status(input: &StatusCollectorInput) -> ServiceStatus {
    if let Some(manager) = input.maybe_service_manager.as_ref() {
        match manager.status() {
            Ok(snapshot) => {
                #[cfg(target_os = "macos")]
                let manager_name = "launchd";
                #[cfg(target_os = "linux")]
                let manager_name = "systemd";
                #[cfg(not(any(target_os = "macos", target_os = "linux")))]
                let manager_name = "unknown";

                let installed = !matches!(snapshot.state, ServiceLifecycleState::Unmanaged);
                let enabled = snapshot.maybe_enabled.unwrap_or(matches!(
                    snapshot.state,
                    ServiceLifecycleState::Enabled
                        | ServiceLifecycleState::Running
                        | ServiceLifecycleState::Stopped
                ));
                let running = matches!(snapshot.state, ServiceLifecycleState::Running);

                return ServiceStatus {
                    manager: FieldAvailability::available(manager_name.to_string()),
                    installed: FieldAvailability::available(installed),
                    enabled: FieldAvailability::available(enabled),
                    running: FieldAvailability::available(running),
                };
            }
            Err(_) => {
                return ServiceStatus {
                    manager: FieldAvailability::unavailable("service manager not inspected"),
                    installed: FieldAvailability::unavailable("service manager not inspected"),
                    enabled: FieldAvailability::unavailable("service manager not inspected"),
                    running: FieldAvailability::unavailable("service manager not inspected"),
                };
            }
        }
    }

    detection_service_status(&input.detection_evidence)
}

fn detection_service_status(evidence: &StatusDetectionEvidence) -> ServiceStatus {
    let maybe_candidate = evidence
        .service_candidates
        .iter()
        .find(|candidate| candidate.present);

    if let Some(candidate) = maybe_candidate {
        return ServiceStatus {
            manager: FieldAvailability::available(detection::service_manager_name(
                candidate.manager,
            )),
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

fn saturating_usize_to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

fn saturating_usize_to_u64(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

pub(super) fn saturating_u64_to_u32(value: u64) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

fn non_negative_i64_to_u64(value: i64) -> u64 {
    u64::try_from(value).unwrap_or(0)
}
