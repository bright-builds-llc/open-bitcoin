// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Compact Phase 78 support summaries.

use open_bitcoin_node::status::{
    FieldAvailability, NoProgressThresholdEvidence, ProgressCreditEvidence, ProgressWindowEvidence,
    SyncStatus,
};
use serde::Serialize;

use super::evidence::SummaryEvidence;

pub(super) fn progress_guarantee_summary(sync: &SyncStatus) -> SummaryEvidence {
    SummaryEvidence::available(format!(
        "credit={} last_useful_work={} expected_window={} threshold={}",
        progress_credit_summary_text(&sync.progress_credit),
        progress_credit_summary_text(&sync.last_useful_work),
        progress_window_summary_text(&sync.expected_progress_window),
        no_progress_threshold_summary_text(&sync.no_progress_threshold)
    ))
}

pub(super) fn stall_diagnosis_summary(sync: &SyncStatus) -> SummaryEvidence {
    match &sync.stall_diagnosis {
        FieldAvailability::Available(value) => {
            let basis = if value.evidence_basis.is_empty() {
                "none".to_string()
            } else {
                value.evidence_basis.join(",")
            };
            SummaryEvidence::available(format!(
                "stalled_subsystem={} confidence={} basis={} next_action={}",
                serialized_label(&value.stalled_subsystem),
                serialized_label(&value.confidence),
                basis,
                value.next_action
            ))
        }
        FieldAvailability::Unavailable { reason } => SummaryEvidence::unavailable(reason.clone()),
    }
}

fn progress_credit_summary_text(value: &FieldAvailability<ProgressCreditEvidence>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "kind={} height={} source_unix_seconds={} rejected_activity_count={}",
            serialized_label(&value.kind),
            value.credited_validated_active_chain_height,
            value.source_unix_seconds,
            value.rejected_activity.len()
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn progress_window_summary_text(value: &FieldAvailability<ProgressWindowEvidence>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "seconds={} retry_backoff_seconds={} max_sync_rounds={}",
            value.expected_progress_window_seconds,
            value.retry_backoff_seconds,
            value.max_sync_rounds
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn no_progress_threshold_summary_text(
    value: &FieldAvailability<NoProgressThresholdEvidence>,
) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "state={} seconds={} elapsed_seconds={}",
            serialized_label(&value.state),
            value.threshold_seconds,
            value.elapsed_since_last_useful_work_seconds
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn serialized_label<T>(value: &T) -> String
where
    T: Serialize,
{
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}
