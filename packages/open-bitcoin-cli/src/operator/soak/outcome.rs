// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Soak-owned outcome labels.

use open_bitcoin_node::status::{NoProgressDiagnosis, SyncRecoveryCategory, SyncStopReasonStatus};
use serde::{Deserialize, Serialize};

use crate::operator::support::{FullSyncEvidence, SupportEvidenceVerdict};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SoakOutcomeLabel {
    CleanCompletion,
    DiagnosedBlocker,
    OperatorStop,
    ResourceStop,
    RecoveryStop,
    UnexpectedTermination,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct SoakOutcomeEvidence {
    pub(crate) maybe_sync_stop_reason: Option<SyncStopReasonStatus>,
    pub(crate) maybe_recovery_category: Option<SyncRecoveryCategory>,
    pub(crate) maybe_no_progress_diagnosis: Option<NoProgressDiagnosis>,
    pub(crate) maybe_full_sync_evidence: Option<FullSyncEvidence>,
    pub(crate) maybe_process_exit: Option<SoakProcessExitEvidence>,
}

impl SoakOutcomeEvidence {
    #[cfg(test)]
    pub(crate) const fn empty() -> Self {
        Self {
            maybe_sync_stop_reason: None,
            maybe_recovery_category: None,
            maybe_no_progress_diagnosis: None,
            maybe_full_sync_evidence: None,
            maybe_process_exit: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SoakProcessExitEvidence {
    pub(crate) operator_requested_stop: bool,
    pub(crate) interrupted: bool,
    pub(crate) maybe_exit_code: Option<i32>,
    pub(crate) maybe_signal: Option<String>,
}

impl SoakProcessExitEvidence {
    #[cfg(test)]
    pub(crate) const fn operator_stop() -> Self {
        Self {
            operator_requested_stop: true,
            interrupted: false,
            maybe_exit_code: None,
            maybe_signal: None,
        }
    }

    #[cfg(test)]
    pub(crate) const fn interrupted_process() -> Self {
        Self {
            operator_requested_stop: false,
            interrupted: true,
            maybe_exit_code: None,
            maybe_signal: None,
        }
    }
}

pub(crate) fn classify_soak_outcome(evidence: &SoakOutcomeEvidence) -> SoakOutcomeLabel {
    if has_operator_stop(evidence) {
        return SoakOutcomeLabel::OperatorStop;
    }
    if has_resource_stop(evidence) {
        return SoakOutcomeLabel::ResourceStop;
    }
    if has_recovery_stop(evidence) {
        return SoakOutcomeLabel::RecoveryStop;
    }
    if has_support_verdict(evidence, SupportEvidenceVerdict::DiagnosedBlocker) {
        return SoakOutcomeLabel::DiagnosedBlocker;
    }
    if evidence
        .maybe_process_exit
        .as_ref()
        .is_some_and(|exit| exit.interrupted)
    {
        return SoakOutcomeLabel::UnexpectedTermination;
    }
    if has_support_verdict(evidence, SupportEvidenceVerdict::StayCurrentProven)
        || has_support_verdict(evidence, SupportEvidenceVerdict::SyncToTipProven)
    {
        return SoakOutcomeLabel::CleanCompletion;
    }

    SoakOutcomeLabel::UnexpectedTermination
}

fn has_operator_stop(evidence: &SoakOutcomeEvidence) -> bool {
    evidence
        .maybe_process_exit
        .as_ref()
        .is_some_and(|exit| exit.operator_requested_stop)
        || evidence
            .maybe_sync_stop_reason
            .as_ref()
            .is_some_and(|reason| operator_stop_label(reason.label.as_str()))
}

fn operator_stop_label(label: &str) -> bool {
    matches!(label, "operator_stop" | "operator_cancellation")
}

fn has_resource_stop(evidence: &SoakOutcomeEvidence) -> bool {
    matches!(
        evidence.maybe_recovery_category,
        Some(SyncRecoveryCategory::ResourceExhaustion)
    ) || matches!(
        evidence.maybe_no_progress_diagnosis,
        Some(NoProgressDiagnosis::StorageOrResourceBlocked)
    )
}

fn has_recovery_stop(evidence: &SoakOutcomeEvidence) -> bool {
    matches!(
        evidence.maybe_recovery_category,
        Some(
            SyncRecoveryCategory::IncompatibleSchema
                | SyncRecoveryCategory::StoreCorruption
                | SyncRecoveryCategory::StorageLockContention
                | SyncRecoveryCategory::StorageBackendFailure
        )
    )
}

fn has_support_verdict(evidence: &SoakOutcomeEvidence, verdict: SupportEvidenceVerdict) -> bool {
    evidence
        .maybe_full_sync_evidence
        .as_ref()
        .is_some_and(|full_sync| full_sync.verdict.label == verdict)
}
