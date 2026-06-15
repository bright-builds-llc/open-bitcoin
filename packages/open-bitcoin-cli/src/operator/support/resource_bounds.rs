// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Compact support-bundle resource-bound evidence.

use std::path::Path;

use open_bitcoin_node::{
    OpenBitcoinStatusSnapshot,
    status::{FieldAvailability, ResourceBoundKind},
};
use serde::Serialize;

use super::EvidenceState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct ResourceBoundSupportEvidence {
    pub(super) state: EvidenceState,
    pub(super) maybe_overall_level: Option<String>,
    pub(super) entries: Vec<ResourceBoundSupportEntry>,
    pub(super) maybe_projected_bundle_size_bytes: Option<u64>,
    pub(super) maybe_unavailable_reason: Option<String>,
    pub(super) source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct ResourceBoundSupportEntry {
    pub(super) kind: String,
    pub(super) label: String,
    pub(super) state: String,
    pub(super) current: Option<u64>,
    pub(super) limit: Option<u64>,
    pub(super) unit: Option<String>,
    pub(super) next_action: Option<String>,
    pub(super) maybe_unavailable_reason: Option<String>,
}

pub(super) fn collect_resource_bound_support_evidence(
    status: &OpenBitcoinStatusSnapshot,
    output_dir: &Path,
) -> ResourceBoundSupportEvidence {
    match &status.resource_bounds {
        FieldAvailability::Available(bounds) => {
            let entries = bounds
                .entries
                .iter()
                .map(|entry| match &entry.usage {
                    FieldAvailability::Available(usage) => ResourceBoundSupportEntry {
                        kind: entry.kind.as_str().to_string(),
                        label: entry.label.clone(),
                        state: usage.state.as_str().to_string(),
                        current: Some(usage.current),
                        limit: usage.maybe_limit,
                        unit: Some(usage.unit.as_str().to_string()),
                        next_action: Some(usage.next_action.clone()),
                        maybe_unavailable_reason: None,
                    },
                    FieldAvailability::Unavailable { reason } => ResourceBoundSupportEntry {
                        kind: entry.kind.as_str().to_string(),
                        label: entry.label.clone(),
                        state: "unavailable".to_string(),
                        current: None,
                        limit: None,
                        unit: None,
                        next_action: None,
                        maybe_unavailable_reason: Some(reason.clone()),
                    },
                })
                .collect::<Vec<_>>();
            let maybe_projected_bundle_size_bytes = bounds
                .entries
                .iter()
                .find(|entry| entry.kind == ResourceBoundKind::SupportBundle)
                .and_then(|entry| match &entry.usage {
                    FieldAvailability::Available(usage) => Some(usage.current),
                    FieldAvailability::Unavailable { .. } => None,
                });
            ResourceBoundSupportEvidence {
                state: EvidenceState::Available,
                maybe_overall_level: Some(bounds.overall_level.as_str().to_string()),
                entries,
                maybe_projected_bundle_size_bytes,
                maybe_unavailable_reason: None,
                source: format!("status.resource_bounds for {}", output_dir.display()),
            }
        }
        FieldAvailability::Unavailable { reason } => ResourceBoundSupportEvidence {
            state: EvidenceState::Unavailable,
            maybe_overall_level: None,
            entries: Vec::new(),
            maybe_projected_bundle_size_bytes: None,
            maybe_unavailable_reason: Some(reason.clone()),
            source: format!("status.resource_bounds for {}", output_dir.display()),
        },
    }
}
