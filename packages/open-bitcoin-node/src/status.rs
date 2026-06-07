// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared operator status snapshot contracts.

mod recovery;

use serde::{Deserialize, Serialize};

use crate::{LogStatus, MetricsStatus};

pub use recovery::SyncRecoveryCategory;

/// Explicit availability wrapper for status fields that may not be collectible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state", content = "value")]
pub enum FieldAvailability<T> {
    Available(T),
    Unavailable { reason: String },
}

impl<T> FieldAvailability<T> {
    pub const fn available(value: T) -> Self {
        Self::Available(value)
    }

    pub fn unavailable(reason: impl Into<String>) -> Self {
        Self::Unavailable {
            reason: reason.into(),
        }
    }
}

/// Daemon runtime state used by status, service, and dashboard consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeRuntimeState {
    Running,
    Stopped,
    Starting,
    Stopping,
    Unreachable,
    Unknown,
}

/// Node process status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeStatus {
    pub state: NodeRuntimeState,
    pub version: String,
}

/// Config and datadir status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigStatus {
    pub datadir: FieldAvailability<String>,
    pub config_paths: Vec<String>,
}

/// Service manager status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceLifecycleStatus {
    Unmanaged,
    InstalledStopped,
    Running,
    Failed,
    Disabled,
    UnavailableManager,
}

impl ServiceLifecycleStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unmanaged => "unmanaged",
            Self::InstalledStopped => "installed-stopped",
            Self::Running => "running",
            Self::Failed => "failed",
            Self::Disabled => "disabled",
            Self::UnavailableManager => "unavailable-manager",
        }
    }
}

/// Service manager status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub manager: FieldAvailability<String>,
    #[serde(default = "service_lifecycle_unavailable")]
    pub lifecycle: FieldAvailability<ServiceLifecycleStatus>,
    pub installed: FieldAvailability<bool>,
    pub enabled: FieldAvailability<bool>,
    pub running: FieldAvailability<bool>,
    #[serde(default = "service_file_path_unavailable")]
    pub service_file_path: FieldAvailability<String>,
    #[serde(default = "service_log_path_unavailable")]
    pub log_path: FieldAvailability<String>,
    #[serde(default = "service_diagnostics_unavailable")]
    pub diagnostics: FieldAvailability<String>,
}

fn service_lifecycle_unavailable() -> FieldAvailability<ServiceLifecycleStatus> {
    FieldAvailability::unavailable("service lifecycle unavailable")
}

fn service_file_path_unavailable() -> FieldAvailability<String> {
    FieldAvailability::unavailable("service file path unavailable")
}

fn service_log_path_unavailable() -> FieldAvailability<String> {
    FieldAvailability::unavailable("service log path unavailable")
}

fn service_diagnostics_unavailable() -> FieldAvailability<String> {
    FieldAvailability::unavailable("service diagnostics unavailable")
}

/// Chain tip projection for status output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainTipStatus {
    pub height: u64,
    pub block_hash: String,
}

/// Sync progress projection for status output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncProgress {
    pub header_height: u64,
    pub block_height: u64,
    pub downloaded_block_height: u64,
    pub connected_block_height: u64,
    pub maybe_downloaded_block_hash: Option<String>,
    pub maybe_connected_block_hash: Option<String>,
    pub progress_ratio: f64,
    pub messages_processed: u64,
    pub headers_received: u64,
    pub blocks_received: u64,
}

/// Configured sync targets projected into shared operator status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConfiguredTargets {
    pub target_outbound_peers: u32,
    pub maybe_target_header_height: Option<u64>,
}

/// Bounded sync attempt counters projected into shared operator status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncAttemptCounters {
    pub attempted_peers: u64,
    pub connected_peers: u64,
    pub failed_peers: u64,
    pub max_sync_rounds: u64,
}

/// Latest durable sync stop reason projected into shared operator status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncStopReasonStatus {
    pub label: String,
    pub message: String,
}

/// Coarse durable sync lifecycle surfaced to operator consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncLifecycleState {
    Active,
    Paused,
    Recovering,
    Failed,
    Stopped,
}

/// Progress signal derived from the latest durable sync run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncProgressSignal {
    HeaderProgress,
    BlockProgress,
    WaitingForPeers,
    PeerFailures,
    AwaitingBlocks,
    Steady,
}

/// Remaining sync lag relative to the best known validated work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncLagStatus {
    pub headers_remaining: u64,
    pub blocks_remaining: u64,
}

/// Current bounded sync resource usage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncResourcePressure {
    pub blocks_in_flight: u64,
    pub max_header_requests_in_flight_per_peer: u64,
    pub max_headers_per_message: u64,
    pub max_blocks_in_flight_per_peer: u64,
    pub max_blocks_in_flight_total: u64,
    pub max_messages_per_peer: u64,
    pub max_sync_rounds: u64,
    pub outbound_peers: u32,
    pub target_outbound_peers: u32,
}

/// Sync status fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyncStatus {
    pub network: FieldAvailability<String>,
    pub chain_tip: FieldAvailability<ChainTipStatus>,
    pub sync_progress: FieldAvailability<SyncProgress>,
    pub lifecycle: FieldAvailability<SyncLifecycleState>,
    pub phase: FieldAvailability<String>,
    #[serde(default = "configured_targets_unavailable")]
    pub configured_targets: FieldAvailability<SyncConfiguredTargets>,
    #[serde(default = "attempt_counters_unavailable")]
    pub attempt_counters: FieldAvailability<SyncAttemptCounters>,
    pub progress_signal: FieldAvailability<SyncProgressSignal>,
    pub lag: FieldAvailability<SyncLagStatus>,
    pub last_successful_progress_unix_seconds: FieldAvailability<u64>,
    #[serde(default = "latest_stop_reason_unavailable")]
    pub latest_stop_reason: FieldAvailability<SyncStopReasonStatus>,
    pub last_error: FieldAvailability<String>,
    #[serde(default = "no_recovery_category_recorded")]
    pub recovery_category: FieldAvailability<SyncRecoveryCategory>,
    pub recovery_action: FieldAvailability<String>,
    pub resource_pressure: FieldAvailability<SyncResourcePressure>,
}

fn configured_targets_unavailable() -> FieldAvailability<SyncConfiguredTargets> {
    FieldAvailability::unavailable("configured targets unavailable")
}

fn attempt_counters_unavailable() -> FieldAvailability<SyncAttemptCounters> {
    FieldAvailability::unavailable("attempt counters unavailable")
}

fn latest_stop_reason_unavailable() -> FieldAvailability<SyncStopReasonStatus> {
    FieldAvailability::unavailable("latest stop reason unavailable")
}

fn no_recovery_category_recorded() -> FieldAvailability<SyncRecoveryCategory> {
    FieldAvailability::unavailable("no recovery category recorded")
}

/// Peer count status details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerCounts {
    pub inbound: u32,
    pub outbound: u32,
}

/// Peer status fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerTelemetry {
    pub peer: String,
    pub source: String,
    pub state: String,
    pub network: String,
    pub attempts: u8,
    pub maybe_resolved_endpoint: FieldAvailability<String>,
    pub capabilities: FieldAvailability<String>,
    pub headers_received: u64,
    pub blocks_received: u64,
    pub maybe_last_activity_unix_seconds: FieldAvailability<u64>,
    pub failure_reason: FieldAvailability<String>,
    pub error: FieldAvailability<String>,
}

/// Peer status fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerStatus {
    pub peer_counts: FieldAvailability<PeerCounts>,
    pub recent_peers: FieldAvailability<Vec<PeerTelemetry>>,
}

/// Mempool status fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolStatus {
    pub transactions: FieldAvailability<u64>,
}

/// Wallet status fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletStatus {
    pub trusted_balance_sats: FieldAvailability<u64>,
    pub freshness: FieldAvailability<WalletFreshness>,
    pub scan_progress: FieldAvailability<WalletScanProgress>,
}

/// Wallet completeness state relative to the durable node tip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletFreshness {
    Fresh,
    Stale,
    Partial,
    Scanning,
}

/// Wallet rescan progress surfaced to operator status consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletScanProgress {
    pub scanned_through_height: u32,
    pub target_tip_height: u32,
}

/// Recent operator health signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthSignal {
    pub level: HealthSignalLevel,
    pub source: String,
    pub message: String,
}

/// Severity of a health signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthSignalLevel {
    Info,
    Warn,
    Error,
}

/// Build metadata displayed in status and support output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildProvenance {
    pub version: String,
    pub commit: FieldAvailability<String>,
    pub build_time: FieldAvailability<String>,
    pub target: FieldAvailability<String>,
    pub profile: FieldAvailability<String>,
}

impl BuildProvenance {
    pub fn unavailable() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            commit: FieldAvailability::unavailable("commit unavailable"),
            build_time: FieldAvailability::unavailable("build time unavailable"),
            target: FieldAvailability::unavailable("target unavailable"),
            profile: FieldAvailability::unavailable("profile unavailable"),
        }
    }
}

/// Durable daemon-sync truth shared between status, dashboard, CLI controls, and RPC.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DurableSyncState {
    pub sync: SyncStatus,
    pub peers: PeerStatus,
    pub health_signals: Vec<HealthSignal>,
    pub updated_at_unix_seconds: u64,
}

/// Durable operator control for the daemon sync loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SyncControlState {
    pub paused: bool,
}

/// Shared status snapshot consumed by CLI, service, dashboard, and support paths.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBitcoinStatusSnapshot {
    pub node: NodeStatus,
    pub config: ConfigStatus,
    pub service: ServiceStatus,
    pub sync: SyncStatus,
    pub peers: PeerStatus,
    pub mempool: MempoolStatus,
    pub wallet: WalletStatus,
    pub logs: LogStatus,
    pub metrics: MetricsStatus,
    pub health_signals: Vec<HealthSignal>,
    pub build: BuildProvenance,
}

#[cfg(test)]
mod tests;
