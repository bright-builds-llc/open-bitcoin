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
mod tests {
    use super::{
        LockEvidence, LockEvidenceKind, RecoveryActionClass, RecoveryCause,
        RecoveryClassifierInput, RecoveryEvidenceBasis, RecoveryEvidenceSnapshot,
        classify_recovery,
    };
    use crate::status::{
        FieldAvailability, ResourceBoundEntry, ResourceBoundKind, ResourceBoundSnapshot,
        ResourceBoundUnit, SyncRecoveryCategory, usage_against_budget,
    };
    use crate::storage::{
        RecoveryMarker, SchemaVersion, StorageError, StorageNamespace, StorageRecoveryAction,
    };

    #[test]
    fn recovery_classifier_schema_mismatch_maps_backup_rebuild_evidence() {
        // Arrange
        let mut input = base_classifier_input();
        input.maybe_storage_error = Some(StorageError::SchemaMismatch {
            expected: SchemaVersion::new(2).expect("nonzero schema version"),
            actual: SchemaVersion::new(1).expect("nonzero schema version"),
        });

        // Act
        let evidence = available_recovery_evidence(classify_recovery(input));

        // Assert
        assert_eq!(evidence.category, SyncRecoveryCategory::IncompatibleSchema);
        assert_eq!(evidence.cause, RecoveryCause::SchemaMismatch);
        assert_eq!(
            evidence.action_class,
            RecoveryActionClass::BackupThenRebuild
        );
        assert_eq!(
            evidence.compatibility_action,
            FieldAvailability::unavailable("no compatibility recovery action recorded")
        );
    }

    #[test]
    fn recovery_classifier_partial_write_sources_map_backup_rebuild_evidence() {
        // Arrange
        let marker = RecoveryMarker::new(
            StorageNamespace::Runtime,
            StorageRecoveryAction::Repair,
            "runtime write interrupted",
        );
        let mut marker_input = base_classifier_input();
        marker_input.maybe_recovery_marker = Some(marker);
        let mut write_input = base_classifier_input();
        write_input.maybe_storage_error = Some(StorageError::InterruptedWrite {
            namespace: StorageNamespace::Headers,
            action: StorageRecoveryAction::RestoreFromBackup,
        });

        // Act
        let marker_evidence = available_recovery_evidence(classify_recovery(marker_input));
        let write_evidence = available_recovery_evidence(classify_recovery(write_input));

        // Assert
        assert_eq!(marker_evidence.cause, RecoveryCause::PartialWrite);
        assert_eq!(
            marker_evidence.action_class,
            RecoveryActionClass::BackupThenRebuild
        );
        assert!(
            marker_evidence
                .evidence_basis
                .contains(&RecoveryEvidenceBasis::RecoveryMarker)
        );
        assert_eq!(
            marker_evidence.maybe_affected_namespace,
            Some("runtime".to_string())
        );
        assert_eq!(
            marker_evidence.compatibility_action,
            FieldAvailability::available(
                StorageRecoveryAction::Repair.operator_message().to_string()
            )
        );
        assert_eq!(write_evidence.cause, RecoveryCause::PartialWrite);
        assert_eq!(
            write_evidence.action_class,
            RecoveryActionClass::BackupThenRebuild
        );
        assert_eq!(
            write_evidence.maybe_affected_namespace,
            Some("headers".to_string())
        );
    }

    #[test]
    fn recovery_classifier_corruption_sources_split_record_and_marker_causes() {
        // Arrange
        let mut record_input = base_classifier_input();
        record_input.maybe_storage_error = Some(StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            detail: "snapshot decode failed".to_string(),
            action: StorageRecoveryAction::Repair,
        });
        let mut marker_input = base_classifier_input();
        marker_input.maybe_storage_error = Some(StorageError::RecoveryMarkerCorruption {
            namespace: StorageNamespace::Runtime,
            detail: "malformed recovery marker".to_string(),
            action: StorageRecoveryAction::Repair,
        });

        // Act
        let record_evidence = available_recovery_evidence(classify_recovery(record_input));
        let marker_evidence = available_recovery_evidence(classify_recovery(marker_input));

        // Assert
        assert_eq!(
            record_evidence.category,
            SyncRecoveryCategory::StoreCorruption
        );
        assert_eq!(record_evidence.cause, RecoveryCause::CorruptRecord);
        assert_eq!(
            record_evidence.maybe_affected_namespace,
            Some("chainstate".to_string())
        );
        assert_eq!(
            marker_evidence.category,
            SyncRecoveryCategory::StoreCorruption
        );
        assert_eq!(marker_evidence.cause, RecoveryCause::CorruptionMarker);
        assert_eq!(
            marker_evidence.maybe_affected_namespace,
            Some("runtime".to_string())
        );
    }

    #[test]
    fn recovery_classifier_lock_evidence_maps_retry_inspection_and_concurrent_precedence() {
        // Arrange
        let mut active_input = base_classifier_input();
        active_input.lock_evidence =
            FieldAvailability::available(lock_evidence(LockEvidenceKind::ActiveContention));
        let mut stale_input = base_classifier_input();
        stale_input.lock_evidence =
            FieldAvailability::available(lock_evidence(LockEvidenceKind::StaleLockEvidence));
        let mut concurrent_input = base_classifier_input();
        concurrent_input.lock_evidence =
            FieldAvailability::available(lock_evidence(LockEvidenceKind::StaleLockEvidence));
        concurrent_input.service_same_datadir = FieldAvailability::available(true);
        concurrent_input.live_rpc_available = FieldAvailability::available(true);

        // Act
        let active_evidence = available_recovery_evidence(classify_recovery(active_input));
        let stale_evidence = available_recovery_evidence(classify_recovery(stale_input));
        let concurrent_evidence = available_recovery_evidence(classify_recovery(concurrent_input));

        // Assert
        assert_eq!(
            active_evidence.category,
            SyncRecoveryCategory::StorageLockContention
        );
        assert_eq!(active_evidence.cause, RecoveryCause::ActiveLock);
        assert_eq!(active_evidence.action_class, RecoveryActionClass::SafeRetry);
        assert_eq!(
            stale_evidence.category,
            SyncRecoveryCategory::StorageLockContention
        );
        assert_eq!(stale_evidence.cause, RecoveryCause::StaleLockEvidence);
        assert_eq!(
            stale_evidence.action_class,
            RecoveryActionClass::ReadOnlyInspection
        );
        assert_eq!(
            concurrent_evidence.category,
            SyncRecoveryCategory::StorageLockContention
        );
        assert_eq!(
            concurrent_evidence.cause,
            RecoveryCause::ConcurrentDatadirUse
        );
        assert_eq!(
            concurrent_evidence.evidence_basis,
            vec![
                RecoveryEvidenceBasis::LockProbe,
                RecoveryEvidenceBasis::ServiceStatus,
                RecoveryEvidenceBasis::LiveRpc
            ]
        );
    }

    #[test]
    fn recovery_classifier_resource_stop_and_backend_failures_map_action_classes() {
        // Arrange
        let mut resource_input = base_classifier_input();
        resource_input.resource_bounds = FieldAvailability::available(stop_resource_bounds());
        let mut backend_input = base_classifier_input();
        backend_input.maybe_storage_error = Some(StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            message: "open failed".to_string(),
            action: StorageRecoveryAction::Restart,
        });

        // Act
        let resource_evidence = available_recovery_evidence(classify_recovery(resource_input));
        let backend_evidence = available_recovery_evidence(classify_recovery(backend_input));

        // Assert
        assert_eq!(
            resource_evidence.category,
            SyncRecoveryCategory::ResourceExhaustion
        );
        assert_eq!(resource_evidence.cause, RecoveryCause::ResourcePressure);
        assert_eq!(
            resource_evidence.action_class,
            RecoveryActionClass::SafeRetry
        );
        assert_eq!(
            backend_evidence.category,
            SyncRecoveryCategory::StorageBackendFailure
        );
        assert_eq!(backend_evidence.cause, RecoveryCause::BackendOpenFailure);
        assert_eq!(
            backend_evidence.action_class,
            RecoveryActionClass::StopAndEscalate
        );
    }

    fn base_classifier_input() -> RecoveryClassifierInput {
        RecoveryClassifierInput {
            maybe_storage_error: None,
            maybe_recovery_marker: None,
            lock_evidence: FieldAvailability::unavailable("lock evidence unavailable"),
            service_same_datadir: FieldAvailability::unavailable("service evidence unavailable"),
            live_rpc_available: FieldAvailability::unavailable("live RPC unavailable"),
            resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
            unavailable_reason: "no recovery signal recorded".to_string(),
        }
    }

    fn available_recovery_evidence(
        evidence: FieldAvailability<RecoveryEvidenceSnapshot>,
    ) -> RecoveryEvidenceSnapshot {
        let FieldAvailability::Available(evidence) = evidence else {
            panic!("recovery evidence should be available");
        };
        evidence
    }

    fn lock_evidence(kind: LockEvidenceKind) -> LockEvidence {
        LockEvidence {
            kind,
            lock_path: "/tmp/open-bitcoin/lock".to_string(),
            detail: "lock probe evidence".to_string(),
        }
    }

    fn stop_resource_bounds() -> ResourceBoundSnapshot {
        ResourceBoundSnapshot::new(vec![ResourceBoundEntry::available(
            ResourceBoundKind::Disk,
            "datadir disk budget",
            usage_against_budget(
                95,
                100,
                ResourceBoundUnit::Bytes,
                "Free disk space before continuing.",
            ),
        )])
    }
}
