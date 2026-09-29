// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Operator prune facts on the shared status snapshot.
//!
//! Configured mode stays beside the live lock list, the manual outcome, and
//! support counts. Those live facts are not chainstate durability.

use open_bitcoin_core::chainstate::{
    PruneMode, PruneStatusProjection, parse_prune_arg, project_prune_status,
};
use serde::{Deserialize, Serialize};

use super::FieldAvailability;

/// One loaded prune lock row for operator display.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PruneLockRow {
    pub name: String,
    pub height_first: u32,
    pub height_last: u32,
}

/// Typed refusal from a manual prune request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManualPruneRefusalCode {
    Disabled,
    ChainTooShort,
    TargetAboveTip,
    KeepWindow,
    NegativeHeight,
    TimestampNotFound,
}

/// Last manual prune outcome. Plan 05 is the writer of a height or refusal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManualPruneSurface {
    None,
    Height { height: i64 },
    Refused { reason: ManualPruneRefusalCode },
}

/// Support counts. `last_prune_height` is absent until a successful batch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PruneSupportCounts {
    pub successful_batch_count: u64,
    pub pruned_height_count: u64,
    #[serde(
        rename = "last_prune_height",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maybe_last_prune_height: Option<u32>,
}

/// Configured prune facts plus live locks, the manual outcome, and counts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PruneOperatorStatus {
    pub pruned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maybe_automatic_pruning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maybe_prune_target_size: Option<u64>,
    pub pruneheight: FieldAvailability<Option<u32>>,
    pub locks: FieldAvailability<Vec<PruneLockRow>>,
    pub manual_prune: FieldAvailability<ManualPruneSurface>,
    pub support_counts: FieldAvailability<PruneSupportCounts>,
}

impl PruneOperatorStatus {
    /// Disabled configured facts with live fields unread.
    pub fn unread_disabled(reason: &str) -> Self {
        Self::configured_live_unavailable(project_prune_status(PruneMode::Disabled, None), reason)
    }

    /// Copies configured quartet fields and the caller-supplied live facts.
    pub fn from_projection(
        projection: PruneStatusProjection,
        pruneheight: FieldAvailability<Option<u32>>,
        locks: FieldAvailability<Vec<PruneLockRow>>,
        manual_prune: FieldAvailability<ManualPruneSurface>,
        support_counts: FieldAvailability<PruneSupportCounts>,
    ) -> Self {
        Self {
            pruned: projection.pruned,
            maybe_automatic_pruning: projection.maybe_automatic_pruning,
            maybe_prune_target_size: projection.maybe_prune_target_size,
            pruneheight,
            locks,
            manual_prune,
            support_counts,
        }
    }

    /// Stopped collector: JSONC mode, and no invented live height or counts.
    pub fn from_stopped_config(maybe_prune: Option<i64>, reason: &str) -> Self {
        let projection = match maybe_prune {
            None => project_prune_status(PruneMode::Disabled, None),
            Some(prune) => match parse_prune_arg(prune) {
                Ok(mode) => project_prune_status(mode, None),
                Err(_) => project_prune_status(PruneMode::Disabled, None),
            },
        };
        Self::configured_live_unavailable(projection, reason)
    }

    fn configured_live_unavailable(projection: PruneStatusProjection, reason: &str) -> Self {
        Self {
            pruned: projection.pruned,
            maybe_automatic_pruning: projection.maybe_automatic_pruning,
            maybe_prune_target_size: projection.maybe_prune_target_size,
            pruneheight: FieldAvailability::unavailable(reason),
            locks: FieldAvailability::unavailable(reason),
            manual_prune: FieldAvailability::unavailable(reason),
            support_counts: FieldAvailability::unavailable(reason),
        }
    }
}

impl Default for PruneOperatorStatus {
    fn default() -> Self {
        Self::unread_disabled("not collected")
    }
}

#[cfg(test)]
mod tests {
    use super::PruneSupportCounts;

    #[test]
    fn loaded_never_deleted_support_counts_omit_last_prune_height() {
        let counts = PruneSupportCounts {
            successful_batch_count: 0,
            pruned_height_count: 0,
            maybe_last_prune_height: None,
        };
        let json = serde_json::to_value(&counts).expect("serialize counts");
        assert_eq!(json["successful_batch_count"], 0);
        assert_eq!(json["pruned_height_count"], 0);
        assert!(json.get("last_prune_height").is_none());
        assert!(json.get("maybe_last_prune_height").is_none());

        let with_height = PruneSupportCounts {
            maybe_last_prune_height: Some(40),
            ..counts
        };
        let json = serde_json::to_value(&with_height).expect("serialize height");
        assert_eq!(json["last_prune_height"], 40);
        assert!(json.get("maybe_last_prune_height").is_none());
    }
}
