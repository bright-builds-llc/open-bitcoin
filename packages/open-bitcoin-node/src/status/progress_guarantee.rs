// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Progress guarantee and stall diagnosis status contracts.

use serde::{Deserialize, Serialize};

use super::FieldAvailability;

/// Default unavailable reason for progress credit before runtime projection.
pub const PROGRESS_CREDIT_UNAVAILABLE_REASON: &str = "progress credit evidence unavailable";

/// Default unavailable reason for expected progress window before runtime projection.
pub const EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON: &str =
    "expected progress window unavailable";

/// Default unavailable reason for no-progress threshold evidence before runtime projection.
pub const NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON: &str =
    "no-progress threshold evidence unavailable";

/// Default unavailable reason for last useful work before runtime projection.
pub const LAST_USEFUL_WORK_UNAVAILABLE_REASON: &str = "last useful work unavailable";

/// Default unavailable reason for last peer contribution before runtime projection.
pub const LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON: &str = "last peer contribution unavailable";

/// Default unavailable reason for stall diagnosis before runtime projection.
pub const STALL_DIAGNOSIS_UNAVAILABLE_REASON: &str = "stall diagnosis unavailable";

/// Credited progress kinds that satisfy the Phase 78 forward-progress contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressCreditKind {
    #[serde(rename = "validated_durable_active_chain")]
    ValidatedDurableActiveChain,
    #[serde(rename = "current_at_best_known_tip")]
    CurrentAtBestKnownTip,
}

/// Activity that may be observable but must not be counted as credited progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RejectedProgressActivityKind {
    #[serde(rename = "header_download")]
    HeaderDownload,
    #[serde(rename = "block_download")]
    BlockDownload,
    #[serde(rename = "in_flight_request")]
    InFlightRequest,
    #[serde(rename = "peer_message")]
    PeerMessage,
    #[serde(rename = "report_projection")]
    ReportProjection,
    #[serde(rename = "retry")]
    Retry,
}

/// Bounded evidence for observable activity rejected as progress credit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RejectedProgressActivity {
    pub kind: RejectedProgressActivityKind,
    pub observed_count: u64,
    pub reason: String,
}

/// Evidence for credited active-chain progress or a stay-current at-tip state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressCreditEvidence {
    pub kind: ProgressCreditKind,
    pub credited_validated_active_chain_height: u64,
    pub credited_validated_active_chain_hash: String,
    pub credited_validated_active_chain_work: String,
    pub source_unix_seconds: u64,
    pub rejected_activity: Vec<RejectedProgressActivity>,
}

/// Evidence describing when progress is expected before a no-progress diagnosis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressWindowEvidence {
    pub retry_backoff_seconds: u64,
    pub max_sync_rounds: u64,
    pub expected_progress_window_seconds: u64,
    pub tip_freshness_threshold_seconds: u64,
}

/// Typed cause for why a sync run made no forward progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoProgressDiagnosis {
    CurrentAtBestKnownTip,
    BehindAwaitingHeaders,
    AwaitingBlockBodies,
    StaleInflightCleanup,
    PeerBackoff,
    PeerStalled,
    PeerFailuresExhausted,
    BranchCompetitionAwaitingBodies,
    RecoveringFromReorgOrStorage,
    StorageOrResourceBlocked,
}

/// Current verdict for a no-progress threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoProgressThresholdState {
    #[serde(rename = "within_window")]
    WithinWindow,
    #[serde(rename = "exceeded")]
    Exceeded,
}

/// Evidence used to decide whether the no-progress threshold has been crossed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoProgressThresholdEvidence {
    pub threshold_seconds: u64,
    pub elapsed_since_last_useful_work_seconds: u64,
    pub state: NoProgressThresholdState,
    pub evaluated_at_unix_seconds: u64,
}

/// Peer contribution category for the latest bounded sync interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerContributionKind {
    #[serde(rename = "headers_only")]
    HeadersOnly,
    #[serde(rename = "blocks_only")]
    BlocksOnly,
    #[serde(rename = "headers_and_blocks")]
    HeadersAndBlocks,
    #[serde(rename = "messages_only")]
    MessagesOnly,
    #[serde(rename = "no_useful_contribution")]
    NoUsefulContribution,
    #[serde(rename = "failure")]
    Failure,
}

/// Bounded evidence for the latest peer contribution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerContributionEvidence {
    pub peer: String,
    pub maybe_resolved_endpoint: Option<String>,
    pub kind: PeerContributionKind,
    pub messages_processed: u64,
    pub headers_received: u64,
    pub blocks_received: u64,
    pub maybe_last_activity_unix_seconds: Option<u64>,
    pub maybe_failure_reason_label: Option<String>,
}

/// Subsystem most likely responsible for an observed stall.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StalledSubsystem {
    #[serde(rename = "public_network_reachability")]
    PublicNetworkReachability,
    #[serde(rename = "incompatible_peers")]
    IncompatiblePeers,
    #[serde(rename = "slow_or_stalled_peers")]
    SlowOrStalledPeers,
    #[serde(rename = "peer_failures_exhausted")]
    PeerFailuresExhausted,
    #[serde(rename = "stale_inflight_cleanup")]
    StaleInflightCleanup,
    #[serde(rename = "branch_competition_awaiting_bodies")]
    BranchCompetitionAwaitingBodies,
    #[serde(rename = "validation")]
    Validation,
    #[serde(rename = "storage_or_resource_pressure")]
    StorageOrResourcePressure,
    #[serde(rename = "at_tip_waiting")]
    AtTipWaiting,
    #[serde(rename = "operator_stop")]
    OperatorStop,
    #[serde(rename = "local_shutdown")]
    LocalShutdown,
    #[serde(rename = "unknown")]
    Unknown,
}

/// Confidence attached to a stall diagnosis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StallDiagnosisConfidence {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}

/// Bounded evidence explaining why sync appears stalled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StallDiagnosisEvidence {
    pub stalled_subsystem: StalledSubsystem,
    pub confidence: StallDiagnosisConfidence,
    pub evidence_basis: Vec<String>,
    pub next_action: String,
    pub maybe_no_progress_diagnosis: Option<NoProgressDiagnosis>,
    pub maybe_recovery_category: Option<super::SyncRecoveryCategory>,
    pub maybe_latest_stop_reason_label: Option<String>,
    pub source_unix_seconds: u64,
}

pub(super) fn progress_credit_unavailable() -> FieldAvailability<ProgressCreditEvidence> {
    FieldAvailability::unavailable(PROGRESS_CREDIT_UNAVAILABLE_REASON)
}

pub(super) fn expected_progress_window_unavailable() -> FieldAvailability<ProgressWindowEvidence> {
    FieldAvailability::unavailable(EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON)
}

pub(super) fn no_progress_threshold_unavailable() -> FieldAvailability<NoProgressThresholdEvidence>
{
    FieldAvailability::unavailable(NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON)
}

pub(super) fn last_useful_work_unavailable() -> FieldAvailability<ProgressCreditEvidence> {
    FieldAvailability::unavailable(LAST_USEFUL_WORK_UNAVAILABLE_REASON)
}

pub(super) fn last_peer_contribution_unavailable() -> FieldAvailability<PeerContributionEvidence> {
    FieldAvailability::unavailable(LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON)
}

pub(super) fn stall_diagnosis_unavailable() -> FieldAvailability<StallDiagnosisEvidence> {
    FieldAvailability::unavailable(STALL_DIAGNOSIS_UNAVAILABLE_REASON)
}
