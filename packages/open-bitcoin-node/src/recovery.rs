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
