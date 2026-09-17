// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use serde::Serialize;

use crate::operator::support::EvidenceAvailability;
use crate::operator::support::EvidenceState;

pub(super) fn csv_or_unavailable(values: &[String]) -> String {
    if values.is_empty() {
        return "unavailable".to_string();
    }
    values.join(", ")
}

pub(super) fn json_string<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

pub(super) fn json_compact<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .map(|value| value.to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

pub(super) fn availability_name(availability: &EvidenceAvailability) -> &'static str {
    evidence_state_name(availability.state)
}

pub(super) const fn evidence_state_name(state: EvidenceState) -> &'static str {
    match state {
        EvidenceState::Available => "available",
        EvidenceState::Unavailable => "unavailable",
    }
}
