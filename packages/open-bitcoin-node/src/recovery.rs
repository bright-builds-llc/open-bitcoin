// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure recovery evidence contracts and classification entrypoint.

use serde::{Deserialize, Serialize};

use crate::status::{
    FieldAvailability, ResourceBoundSnapshot, ResourcePressureState, SyncRecoveryCategory,
};
use crate::storage::{RecoveryMarker, StorageError, StorageNamespace, StorageRecoveryAction};

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
    CorruptRecord,
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
    if let FieldAvailability::Available(resource_bounds) = &input.resource_bounds
        && resource_bounds.overall_level == ResourcePressureState::StopRequired
    {
        return recovery_evidence(RecoveryEvidenceParts {
            category: SyncRecoveryCategory::ResourceExhaustion,
            action_class: RecoveryActionClass::SafeRetry,
            cause: RecoveryCause::ResourcePressure,
            evidence_basis: vec![RecoveryEvidenceBasis::ResourceBounds],
            maybe_affected_namespace: None,
            maybe_affected_path: None,
            next_action: "Free disk space or reduce the reported resource pressure before retrying the operation.".to_string(),
            compatibility_action: compatibility_action(StorageRecoveryAction::FreeDisk),
        });
    }

    if is_concurrent_datadir_use(&input) {
        return recovery_evidence(RecoveryEvidenceParts {
            category: SyncRecoveryCategory::StorageLockContention,
            action_class: RecoveryActionClass::ReadOnlyInspection,
            cause: RecoveryCause::ConcurrentDatadirUse,
            evidence_basis: vec![
                RecoveryEvidenceBasis::LockProbe,
                RecoveryEvidenceBasis::ServiceStatus,
                RecoveryEvidenceBasis::LiveRpc,
            ],
            maybe_affected_namespace: None,
            maybe_affected_path: maybe_lock_path(&input.lock_evidence),
            next_action: "Use the running daemon or stop it cleanly before any operation that needs exclusive datadir access.".to_string(),
            compatibility_action: no_compatibility_action(),
        });
    }

    if let FieldAvailability::Available(lock_evidence) = &input.lock_evidence {
        match lock_evidence.kind {
            LockEvidenceKind::ActiveContention => {
                return recovery_evidence(RecoveryEvidenceParts {
                    category: SyncRecoveryCategory::StorageLockContention,
                    action_class: RecoveryActionClass::SafeRetry,
                    cause: RecoveryCause::ActiveLock,
                    evidence_basis: vec![RecoveryEvidenceBasis::LockProbe],
                    maybe_affected_namespace: None,
                    maybe_affected_path: Some(lock_evidence.lock_path.clone()),
                    next_action: "Retry after the process holding the datadir lock exits or after confirming the running daemon owns the datadir.".to_string(),
                    compatibility_action: no_compatibility_action(),
                });
            }
            LockEvidenceKind::StaleLockEvidence => {
                return recovery_evidence(RecoveryEvidenceParts {
                    category: SyncRecoveryCategory::StorageLockContention,
                    action_class: RecoveryActionClass::ReadOnlyInspection,
                    cause: RecoveryCause::StaleLockEvidence,
                    evidence_basis: vec![RecoveryEvidenceBasis::LockProbe],
                    maybe_affected_namespace: None,
                    maybe_affected_path: Some(lock_evidence.lock_path.clone()),
                    next_action:
                        "Inspect the datadir read-only and avoid deleting lock artifacts automatically."
                            .to_string(),
                    compatibility_action: no_compatibility_action(),
                });
            }
            LockEvidenceKind::NoLockArtifact | LockEvidenceKind::ProbeUnavailable => {}
        }
    }

    if let Some(storage_error) = &input.maybe_storage_error {
        match storage_error {
            StorageError::InvalidSchemaVersion { .. } | StorageError::SchemaMismatch { .. } => {
                return recovery_evidence(RecoveryEvidenceParts {
                    category: SyncRecoveryCategory::IncompatibleSchema,
                    action_class: RecoveryActionClass::BackupThenRebuild,
                    cause: RecoveryCause::SchemaMismatch,
                    evidence_basis: vec![RecoveryEvidenceBasis::StorageError],
                    maybe_affected_namespace: None,
                    maybe_affected_path: None,
                    next_action:
                        "Back up the affected datadir before rebuilding storage with a compatible schema."
                            .to_string(),
                    compatibility_action: no_compatibility_action(),
                });
            }
            StorageError::Corruption {
                namespace, action, ..
            } => {
                return recovery_evidence(RecoveryEvidenceParts {
                    category: SyncRecoveryCategory::StoreCorruption,
                    action_class: RecoveryActionClass::BackupThenRebuild,
                    cause: RecoveryCause::CorruptRecord,
                    evidence_basis: vec![RecoveryEvidenceBasis::StorageError],
                    maybe_affected_namespace: Some(namespace_name(*namespace)),
                    maybe_affected_path: None,
                    next_action:
                        "Back up the affected datadir before restoring or rebuilding the corrupted store."
                            .to_string(),
                    compatibility_action: compatibility_action(*action),
                });
            }
            StorageError::RecoveryMarkerCorruption {
                namespace, action, ..
            } => {
                return recovery_evidence(RecoveryEvidenceParts {
                    category: SyncRecoveryCategory::StoreCorruption,
                    action_class: RecoveryActionClass::BackupThenRebuild,
                    cause: RecoveryCause::CorruptionMarker,
                    evidence_basis: vec![RecoveryEvidenceBasis::StorageError],
                    maybe_affected_namespace: Some(namespace_name(*namespace)),
                    maybe_affected_path: None,
                    next_action:
                        "Back up the affected datadir before restoring or rebuilding the corrupted store."
                            .to_string(),
                    compatibility_action: compatibility_action(*action),
                });
            }
            StorageError::InterruptedWrite { .. }
            | StorageError::UnavailableNamespace { .. }
            | StorageError::BackendFailure { .. } => {}
        }
    }

    if let Some(marker) = &input.maybe_recovery_marker {
        return recovery_evidence(RecoveryEvidenceParts {
            category: marker.action.recovery_category(),
            action_class: RecoveryActionClass::BackupThenRebuild,
            cause: RecoveryCause::PartialWrite,
            evidence_basis: vec![RecoveryEvidenceBasis::RecoveryMarker],
            maybe_affected_namespace: Some(namespace_name(marker.namespace)),
            maybe_affected_path: None,
            next_action:
                "Back up the affected datadir before rebuilding storage after the recorded partial write."
                    .to_string(),
            compatibility_action: compatibility_action(marker.action),
        });
    }

    if let Some(storage_error) = &input.maybe_storage_error {
        match storage_error {
            StorageError::InterruptedWrite { namespace, action } => {
                return recovery_evidence(RecoveryEvidenceParts {
                    category: action.recovery_category(),
                    action_class: RecoveryActionClass::BackupThenRebuild,
                    cause: RecoveryCause::PartialWrite,
                    evidence_basis: vec![RecoveryEvidenceBasis::StorageError],
                    maybe_affected_namespace: Some(namespace_name(*namespace)),
                    maybe_affected_path: None,
                    next_action:
                        "Back up the affected datadir before rebuilding storage after the interrupted write."
                            .to_string(),
                    compatibility_action: compatibility_action(*action),
                });
            }
            StorageError::UnavailableNamespace { namespace } => {
                return recovery_evidence(RecoveryEvidenceParts {
                    category: SyncRecoveryCategory::StorageBackendFailure,
                    action_class: RecoveryActionClass::StopAndEscalate,
                    cause: RecoveryCause::UnreadableNamespace,
                    evidence_basis: vec![RecoveryEvidenceBasis::StorageError],
                    maybe_affected_namespace: Some(namespace_name(*namespace)),
                    maybe_affected_path: None,
                    next_action:
                        "Stop and escalate the unreadable namespace before retrying normal operation."
                            .to_string(),
                    compatibility_action: no_compatibility_action(),
                });
            }
            StorageError::BackendFailure {
                namespace, action, ..
            } if storage_error.recovery_category()
                == SyncRecoveryCategory::StorageLockContention =>
            {
                return recovery_evidence(RecoveryEvidenceParts {
                    category: SyncRecoveryCategory::StorageLockContention,
                    action_class: RecoveryActionClass::SafeRetry,
                    cause: RecoveryCause::ActiveLock,
                    evidence_basis: vec![RecoveryEvidenceBasis::StorageError],
                    maybe_affected_namespace: Some(namespace_name(*namespace)),
                    maybe_affected_path: None,
                    next_action: "Retry after the process holding the datadir lock exits or after confirming the running daemon owns the datadir.".to_string(),
                    compatibility_action: compatibility_action(*action),
                });
            }
            StorageError::BackendFailure {
                namespace, action, ..
            } => {
                return recovery_evidence(RecoveryEvidenceParts {
                    category: storage_error.recovery_category(),
                    action_class: RecoveryActionClass::StopAndEscalate,
                    cause: RecoveryCause::BackendOpenFailure,
                    evidence_basis: vec![RecoveryEvidenceBasis::StorageError],
                    maybe_affected_namespace: Some(namespace_name(*namespace)),
                    maybe_affected_path: None,
                    next_action:
                        "Stop and escalate the backend open failure before retrying normal operation."
                            .to_string(),
                    compatibility_action: compatibility_action(*action),
                });
            }
            StorageError::InvalidSchemaVersion { .. }
            | StorageError::SchemaMismatch { .. }
            | StorageError::Corruption { .. }
            | StorageError::RecoveryMarkerCorruption { .. } => {}
        }
    }

    FieldAvailability::unavailable(input.unavailable_reason)
}

impl Default for FieldAvailability<RecoveryEvidenceSnapshot> {
    fn default() -> Self {
        Self::unavailable(RECOVERY_EVIDENCE_UNAVAILABLE_REASON)
    }
}

fn is_concurrent_datadir_use(input: &RecoveryClassifierInput) -> bool {
    if input.service_same_datadir != FieldAvailability::available(true) {
        return false;
    }
    if input.live_rpc_available != FieldAvailability::available(true) {
        return false;
    }
    matches!(
        input.lock_evidence,
        FieldAvailability::Available(LockEvidence {
            kind: LockEvidenceKind::ActiveContention | LockEvidenceKind::StaleLockEvidence,
            ..
        })
    )
}

fn maybe_lock_path(lock_evidence: &FieldAvailability<LockEvidence>) -> Option<String> {
    let FieldAvailability::Available(lock_evidence) = lock_evidence else {
        return None;
    };
    Some(lock_evidence.lock_path.clone())
}

struct RecoveryEvidenceParts {
    category: SyncRecoveryCategory,
    action_class: RecoveryActionClass,
    cause: RecoveryCause,
    evidence_basis: Vec<RecoveryEvidenceBasis>,
    maybe_affected_namespace: Option<String>,
    maybe_affected_path: Option<String>,
    next_action: String,
    compatibility_action: FieldAvailability<String>,
}

fn recovery_evidence(parts: RecoveryEvidenceParts) -> FieldAvailability<RecoveryEvidenceSnapshot> {
    FieldAvailability::available(RecoveryEvidenceSnapshot {
        category: parts.category,
        action_class: parts.action_class,
        cause: parts.cause,
        evidence_basis: parts.evidence_basis,
        maybe_affected_namespace: parts.maybe_affected_namespace,
        maybe_affected_path: parts.maybe_affected_path,
        next_action: parts.next_action,
        compatibility_action: parts.compatibility_action,
    })
}

fn namespace_name(namespace: StorageNamespace) -> String {
    namespace.as_str().to_string()
}

fn compatibility_action(action: StorageRecoveryAction) -> FieldAvailability<String> {
    FieldAvailability::available(action.operator_message().to_string())
}

fn no_compatibility_action() -> FieldAvailability<String> {
    FieldAvailability::unavailable("no compatibility recovery action recorded")
}

#[cfg(test)]
mod tests;
