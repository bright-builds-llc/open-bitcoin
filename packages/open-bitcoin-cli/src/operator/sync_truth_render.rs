// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared text projections for durable full-sync truth fields.

use open_bitcoin_node::status::{
    BestKnownTipStatus, FieldAvailability, NoProgressDiagnosis, NoProgressThresholdEvidence,
    PeerContributionEvidence, ProgressCreditEvidence, ProgressWindowEvidence,
    StallDiagnosisEvidence, StayCurrentStatus, SyncProgress, SyncReconcileProgressStatus,
    SyncReorgEvidence,
};

pub(crate) fn sync_progress_text(value: &FieldAvailability<SyncProgress>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "{:.2}% headers={} downloaded_blocks={} connected_blocks={} validated_active_chain_height={} validated_active_chain_hash={} validated_active_chain_work={}",
            value.progress_ratio * 100.0,
            value.header_height,
            value.downloaded_block_height,
            value.connected_block_height,
            value.validated_active_chain_height,
            optional_text(
                &value.maybe_validated_active_chain_hash,
                "validated active-chain hash unavailable"
            ),
            optional_text(
                &value.maybe_validated_active_chain_work,
                "validated active-chain work unavailable"
            )
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn best_known_tip_text(value: &FieldAvailability<BestKnownTipStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "source={} height={} hash={} work={} block_time_unix_seconds={} observed_at_unix_seconds={} freshness={} peer_agreement_count={}",
            serialized_label(&value.source),
            value.height,
            value.block_hash,
            value.work,
            value.block_time_unix_seconds,
            value.observed_at_unix_seconds,
            serialized_label(&value.freshness),
            value.peer_agreement.len()
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn stay_current_text(value: &FieldAvailability<StayCurrentStatus>) -> String {
    match value {
        FieldAvailability::Available(value) => serialized_label(value),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn no_progress_diagnosis_text(value: &FieldAvailability<NoProgressDiagnosis>) -> String {
    match value {
        FieldAvailability::Available(value) => serialized_label(value),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn progress_credit_text(value: &FieldAvailability<ProgressCreditEvidence>) -> String {
    match value {
        FieldAvailability::Available(value) => progress_credit_evidence_text(value),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn progress_window_text(value: &FieldAvailability<ProgressWindowEvidence>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "expected_progress_window_seconds={} retry_backoff_seconds={} max_sync_rounds={} tip_freshness_threshold_seconds={}",
            value.expected_progress_window_seconds,
            value.retry_backoff_seconds,
            value.max_sync_rounds,
            value.tip_freshness_threshold_seconds
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn no_progress_threshold_text(
    value: &FieldAvailability<NoProgressThresholdEvidence>,
) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "state={} threshold_seconds={} elapsed_since_last_useful_work_seconds={} evaluated_at_unix_seconds={}",
            serialized_label(&value.state),
            value.threshold_seconds,
            value.elapsed_since_last_useful_work_seconds,
            value.evaluated_at_unix_seconds
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn last_useful_work_text(value: &FieldAvailability<ProgressCreditEvidence>) -> String {
    progress_credit_text(value)
}

pub(crate) fn last_peer_contribution_text(
    value: &FieldAvailability<PeerContributionEvidence>,
) -> String {
    match value {
        FieldAvailability::Available(value) => {
            let endpoint = value
                .maybe_resolved_endpoint
                .as_deref()
                .unwrap_or("Unavailable: endpoint unavailable");
            let activity = value
                .maybe_last_activity_unix_seconds
                .map(|seconds| seconds.to_string())
                .unwrap_or_else(|| "Unavailable: peer activity unavailable".to_string());
            let failure = value
                .maybe_failure_reason_label
                .as_deref()
                .unwrap_or("Unavailable: no peer failure recorded");
            format!(
                "peer={} endpoint={} kind={} messages={} headers={} blocks={} last_activity_unix_seconds={} failure={}",
                value.peer,
                endpoint,
                serialized_label(&value.kind),
                value.messages_processed,
                value.headers_received,
                value.blocks_received,
                activity,
                failure
            )
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn stall_diagnosis_text(value: &FieldAvailability<StallDiagnosisEvidence>) -> String {
    match value {
        FieldAvailability::Available(value) => {
            let basis = if value.evidence_basis.is_empty() {
                "none".to_string()
            } else {
                value.evidence_basis.join(",")
            };
            let no_progress = value
                .maybe_no_progress_diagnosis
                .as_ref()
                .map(serialized_label)
                .unwrap_or_else(|| "Unavailable: no no-progress diagnosis".to_string());
            let recovery = value
                .maybe_recovery_category
                .as_ref()
                .map(|category| category.as_str().to_string())
                .unwrap_or_else(|| "Unavailable: no recovery category".to_string());
            let stop_reason = value
                .maybe_latest_stop_reason_label
                .as_deref()
                .unwrap_or("Unavailable: no stop reason");
            format!(
                "stalled_subsystem={} confidence={} basis={} next_action={} no_progress_diagnosis={} recovery_category={} latest_stop_reason={}",
                serialized_label(&value.stalled_subsystem),
                serialized_label(&value.confidence),
                basis,
                value.next_action,
                no_progress,
                recovery,
                stop_reason
            )
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn sync_reorg_text(value: &FieldAvailability<SyncReorgEvidence>) -> String {
    match value {
        FieldAvailability::Available(value) => sync_reorg_evidence_text(value),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(crate) fn sync_reconcile_text(
    value: &FieldAvailability<SyncReconcileProgressStatus>,
) -> String {
    match value {
        FieldAvailability::Available(value) => sync_reconcile_status_text(value),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn sync_reorg_evidence_text(value: &SyncReorgEvidence) -> String {
    format!(
        "common_ancestor_height={} common_ancestor_hash={} disconnected_count={} connected_count={} final_active_height={} final_active_hash={} fully_persisted={}",
        value.common_ancestor_height,
        value.common_ancestor_hash,
        value.disconnected_count,
        value.connected_count,
        value.final_active_height,
        value.final_active_hash,
        value.fully_persisted
    )
}

fn progress_credit_evidence_text(value: &ProgressCreditEvidence) -> String {
    format!(
        "kind={} height={} hash={} work={} source_unix_seconds={} rejected_activity_count={}",
        serialized_label(&value.kind),
        value.credited_validated_active_chain_height,
        value.credited_validated_active_chain_hash,
        value.credited_validated_active_chain_work,
        value.source_unix_seconds,
        value.rejected_activity.len()
    )
}

fn sync_reconcile_status_text(value: &SyncReconcileProgressStatus) -> String {
    match value {
        SyncReconcileProgressStatus::NoChange => "no_change".to_string(),
        SyncReconcileProgressStatus::ExtendedActiveChain {
            connected_count,
            final_active_height,
            final_active_hash,
        } => format!(
            "extended_active_chain connected_count={connected_count} final_active_height={final_active_height} final_active_hash={final_active_hash}"
        ),
        SyncReconcileProgressStatus::BranchCompetitionAwaitingBodies {
            common_ancestor_height,
            common_ancestor_hash,
            branch_tip_height,
            branch_tip_hash,
            missing_block_count,
        } => format!(
            "branch_competition_awaiting_bodies common_ancestor_height={common_ancestor_height} common_ancestor_hash={common_ancestor_hash} branch_tip_height={branch_tip_height} branch_tip_hash={branch_tip_hash} missing_block_count={missing_block_count}"
        ),
        SyncReconcileProgressStatus::SideBranchPreserved {
            branch_tip_height,
            branch_tip_hash,
            active_tip_height,
            active_tip_hash,
        } => format!(
            "side_branch_preserved branch_tip_height={branch_tip_height} branch_tip_hash={branch_tip_hash} active_tip_height={active_tip_height} active_tip_hash={active_tip_hash}"
        ),
        SyncReconcileProgressStatus::ReorgPersisted { evidence } => {
            format!("reorg_persisted {}", sync_reorg_evidence_text(evidence))
        }
    }
}

fn optional_text(maybe_value: &Option<String>, unavailable_reason: &str) -> String {
    maybe_value
        .clone()
        .unwrap_or_else(|| format!("Unavailable: {unavailable_reason}"))
}

fn serialized_label<T>(value: &T) -> String
where
    T: serde::Serialize,
{
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}
