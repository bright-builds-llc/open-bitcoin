// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Dashboard resource-bound summary rendering.

use open_bitcoin_node::status::{FieldAvailability, ResourceBoundSnapshot, ResourcePressureState};

pub(super) fn resource_bounds(value: &FieldAvailability<ResourceBoundSnapshot>) -> String {
    match value {
        FieldAvailability::Available(value) => {
            let stop_or_warning = value
                .entries
                .iter()
                .filter_map(|entry| match &entry.usage {
                    FieldAvailability::Available(usage)
                        if usage.state == ResourcePressureState::StopRequired
                            || usage.state == ResourcePressureState::Warning =>
                    {
                        Some(format!("{}={}", entry.kind.as_str(), usage.state.as_str()))
                    }
                    FieldAvailability::Available(_) | FieldAvailability::Unavailable { .. } => None,
                })
                .collect::<Vec<_>>();
            if stop_or_warning.is_empty() {
                return format!("overall={}", value.overall_level.as_str());
            }
            format!(
                "overall={} {}",
                value.overall_level.as_str(),
                stop_or_warning.join(" ")
            )
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}
