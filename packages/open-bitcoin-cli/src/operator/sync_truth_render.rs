// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared text projections for durable full-sync truth fields.

use open_bitcoin_node::status::{
    BestKnownTipStatus, FieldAvailability, NoProgressDiagnosis, StayCurrentStatus, SyncProgress,
    SyncReconcileProgressStatus, SyncReorgEvidence,
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
