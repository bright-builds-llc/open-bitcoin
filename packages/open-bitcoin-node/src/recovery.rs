// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure recovery evidence contracts and classification entrypoint.

use serde::{Deserialize, Serialize};

use crate::status::{FieldAvailability, ResourceBoundSnapshot, SyncRecoveryCategory};
use crate::storage::{RecoveryMarker, StorageError};

/// Default unavailable reason for recovery evidence on legacy or stopped snapshots.
pub const RECOVERY_EVIDENCE_UNAVAILABLE_REASON: &str = "recovery evidence unavailable";

/// Operator safety class for a recovery recommendation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    SafeRetry,
    ReadOnlyInspection,
    BackupThenRebuild,
    StopAndEscalate,
}

/// Typed cause behind a storage or lock recovery status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryCause {
    SchemaMismatch,
    CorruptionMarker,
    PartialWrite,
    UnreadableNamespace,
    BackendOpenFailure,
    ActiveLock,
    StaleLockEvidence,
    ConcurrentDatadirUse,
    ResourcePressure,
}

/// Evidence source used by the pure recovery classifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryEvidenceBasis {
    StorageError,
    RecoveryMarker,
    LockProbe,
    ServiceStatus,
    LiveRpc,
    ResourceBounds,
    UnavailableReason,
}

/// Probe result for a datastore lock artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockEvidenceKind {
    NoLockArtifact,
    ActiveContention,
    StaleLockEvidence,
    ProbeUnavailable,
}

/// Shared lock evidence DTO consumed by status, support, and soak surfaces.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LockEvidence {
    pub kind: LockEvidenceKind,
    pub lock_path: String,
    pub detail: String,
}

/// Shared recovery evidence emitted beside stable compatibility categories.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecoveryEvidenceSnapshot {
    pub category: SyncRecoveryCategory,
    pub action_class: RecoveryActionClass,
    pub cause: RecoveryCause,
    pub evidence_basis: Vec<RecoveryEvidenceBasis>,
    pub maybe_affected_namespace: Option<String>,
    pub maybe_affected_path: Option<String>,
    pub next_action: String,
    pub compatibility_action: FieldAvailability<String>,
}

/// Owned inputs for pure recovery classification.
#[derive(Clone, Debug, PartialEq)]
pub struct RecoveryClassifierInput {
    pub maybe_storage_error: Option<StorageError>,
    pub maybe_recovery_marker: Option<RecoveryMarker>,
    pub lock_evidence: FieldAvailability<LockEvidence>,
    pub service_same_datadir: FieldAvailability<bool>,
    pub live_rpc_available: FieldAvailability<bool>,
    pub resource_bounds: FieldAvailability<ResourceBoundSnapshot>,
    pub unavailable_reason: String,
}

/// Classify recovery signals into a shared typed evidence snapshot.
pub fn classify_recovery(
    input: RecoveryClassifierInput,
) -> FieldAvailability<RecoveryEvidenceSnapshot> {
    FieldAvailability::unavailable(input.unavailable_reason)
}

impl Default for FieldAvailability<RecoveryEvidenceSnapshot> {
    fn default() -> Self {
        Self::unavailable(RECOVERY_EVIDENCE_UNAVAILABLE_REASON)
    }
}
