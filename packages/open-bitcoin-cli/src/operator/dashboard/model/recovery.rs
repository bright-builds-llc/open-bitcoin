// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Dashboard recovery evidence row rendering.

use open_bitcoin_node::{
    RecoveryEvidenceSnapshot,
    status::{FieldAvailability, SyncRecoveryCategory},
};

pub(super) fn recovery_category(value: &FieldAvailability<SyncRecoveryCategory>) -> String {
    match value {
        FieldAvailability::Available(value) => value.as_str().to_string(),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

pub(super) fn recovery_evidence(value: &FieldAvailability<RecoveryEvidenceSnapshot>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "category={} cause={} action_class={} next_action={}",
            value.category.as_str(),
            serialized_label(&value.cause),
            serialized_label(&value.action_class),
            value.next_action
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
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
