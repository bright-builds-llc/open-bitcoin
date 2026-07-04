// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared sanitized block-serving evidence status contracts.

use super::FieldAvailability;
use serde::{Deserialize, Serialize};

/// Default reason when block-serving evidence has not been projected.
pub const BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON: &str = "block serving evidence unavailable";

/// Explicit block-serving activation evidence safe for operator status surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockServingActivationEvidence {
    pub block_serving_enabled: bool,
    pub compact_relay_enabled: bool,
}

/// Aggregate peer eligibility counters safe for public status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockServingEligibilityCounters {
    pub eligible_peer_count: u64,
    pub ineligible_peer_count: u64,
    pub disabled_count: u64,
    pub activation_required_count: u64,
    pub inbound_serving_required_count: u64,
    pub permission_required_count: u64,
    pub protected_not_serving_count: u64,
    pub status_unavailable_count: u64,
    pub permission_effect_inactive_count: u64,
}

/// Aggregate block-serving status counters safe for public status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockServingStatusCounters {
    pub validated_count: u64,
    pub available_count: u64,
    pub stale_count: u64,
    pub side_chain_count: u64,
    pub pruned_count: u64,
    pub unavailable_count: u64,
    pub unvalidated_count: u64,
    pub unknown_count: u64,
    pub suppressed_count: u64,
}

/// Shared status contract for sanitized block-serving evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockServingEvidenceStatus {
    #[serde(default = "activation_default")]
    pub activation: FieldAvailability<BlockServingActivationEvidence>,
    #[serde(default = "eligibility_counters_default")]
    pub eligibility: FieldAvailability<BlockServingEligibilityCounters>,
    #[serde(default = "status_counters_default")]
    pub status: FieldAvailability<BlockServingStatusCounters>,
}

impl BlockServingEvidenceStatus {
    pub fn default_unavailable() -> Self {
        Self {
            activation: activation_default(),
            eligibility: eligibility_counters_default(),
            status: status_counters_default(),
        }
    }

    pub fn with_activation_eligibility_and_status(
        activation: BlockServingActivationEvidence,
        eligibility: BlockServingEligibilityCounters,
        status: BlockServingStatusCounters,
    ) -> Self {
        Self {
            activation: FieldAvailability::available(activation),
            eligibility: FieldAvailability::available(eligibility),
            status: FieldAvailability::available(status),
        }
    }
}

impl Default for BlockServingEvidenceStatus {
    fn default() -> Self {
        Self::default_unavailable()
    }
}

fn activation_default() -> FieldAvailability<BlockServingActivationEvidence> {
    FieldAvailability::unavailable(BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON)
}

fn eligibility_counters_default() -> FieldAvailability<BlockServingEligibilityCounters> {
    FieldAvailability::available(BlockServingEligibilityCounters::default())
}

fn status_counters_default() -> FieldAvailability<BlockServingStatusCounters> {
    FieldAvailability::available(BlockServingStatusCounters::default())
}

#[cfg(test)]
mod tests;
