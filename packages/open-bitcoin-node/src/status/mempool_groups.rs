// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Identifier-free mempool snapshot groups for shared status.

use serde::{Deserialize, Serialize};

use crate::network::{
    CheckpointEvidenceSnapshot, CheckpointOutcome, CheckpointPersistenceStrength,
    ManagedMempoolInfo, ManagedMempoolRecoverySummary,
};

use super::{
    FieldAvailability, MempoolStatus,
    relay_evidence::{RelayEvidenceCounters, RelayEvidenceStatus},
};

/// Occupancy-sensitive rolling-fee decay labels from UI-SPEC / research A1.
pub const DECAY_HALF_LIFE_12H: &str = "half_life_12h";
pub const DECAY_HALF_LIFE_6H: &str = "half_life_6h";
pub const DECAY_HALF_LIFE_3H: &str = "half_life_3h";
pub const DECAY_NOT_DECAYING: &str = "not_decaying";

/// Reason used when a mempool group is absent from a legacy snapshot.
pub const MEMPOOL_GROUP_UNAVAILABLE_REASON: &str = "mempool group unavailable";

/// Distinct resource roles: virtual size, accounted usage, and accounted capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolResourcesGroup {
    pub virtual_size: u64,
    pub accounted_usage: u64,
    pub accounted_capacity: u64,
    pub transaction_count: u64,
}

/// Distinct fee-floor roles in sat/kvB. Effective admission is not a Knots alias.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolFeeFloorsGroup {
    pub static_relay_floor: i64,
    pub rolling_mempool_floor: i64,
    pub effective_admission_floor: i64,
    pub incremental_relay_fee: i64,
}

/// Pressure removals plus a fixed decay label. No wall-clock ETA (D-10, D-13).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolPressureGroup {
    pub pressure_removal_count: u64,
    pub decay_half_life_label: String,
}

/// Pressure-removal count owned by this group; relay eviction is read from
/// `relay.outcome_counters` and is not copied here (D-13).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolEvictionGroup {
    pub pressure_removal_count: u64,
}

/// Identifier-free checkpoint facts. Generation integers stay off this group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolCheckpointGroup {
    pub outcome: String,
    pub overdue: bool,
    pub persistence_strength: String,
    pub age_seconds: Option<u64>,
    pub loss_bound_seconds: Option<u64>,
    pub dirty_generation_present: bool,
}

/// Count-only recovery projection. Recovery member identities are never copied.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolRecoveryGroup {
    pub recovered_count: u64,
    pub dropped_confirmed_count: u64,
    pub dropped_duplicate_count: u64,
    pub dropped_missing_parent_count: u64,
    pub dropped_policy_incompatible_count: u64,
    pub dropped_expired_count: u64,
    pub dropped_evicted_count: u64,
}

/// Local retry/fanout aggregates. Distinct from Phase 105 deferred counters (D-13).
///
/// `relay_disabled` is boolean-as-0/1. Local still-present membership while relay is
/// off lives on [`MempoolAdmissionGroup::still_present`].
///
/// `attempted` has no separate live leftover counter and is reported as zero until a
/// later phase owns it. `emitted` copies the current relay announce outcome counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolRetryGroup {
    pub eligible: u64,
    pub queued: u64,
    pub attempted: u64,
    pub emitted: u64,
    pub requested: u64,
    pub served: u64,
    pub suppressed: u64,
    pub relay_disabled: u64,
    pub cleared: u64,
}

/// Admission aggregates only (D-15). Independent of retry/fanout axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolAdmissionGroup {
    pub accepted: u64,
    pub still_present: u64,
    pub cleared: u64,
}

/// Maps managed mempool pressure into the shared resource group.
pub fn resources_from_managed_info(info: &ManagedMempoolInfo) -> MempoolResourcesGroup {
    MempoolResourcesGroup {
        virtual_size: info.total_virtual_size as u64,
        accounted_usage: info.accounted_memory as u64,
        accounted_capacity: info.mempool_capacity as u64,
        transaction_count: info.transaction_count as u64,
    }
}

/// Maps managed mempool fee roles 1:1 without swapping minrelay and incremental.
pub fn fee_floors_from_managed_info(info: &ManagedMempoolInfo) -> MempoolFeeFloorsGroup {
    MempoolFeeFloorsGroup {
        static_relay_floor: info.static_relay_fee_rate_sats_per_kvb,
        rolling_mempool_floor: info.rolling_mempool_fee_rate_sats_per_kvb,
        effective_admission_floor: info.effective_admission_fee_rate_sats_per_kvb,
        incremental_relay_fee: info.incremental_relay_fee_rate_sats_per_kvb,
    }
}

/// Occupancy rule from `fee/rolling.rs`: usage < capacity/4 → 3h; usage <
/// capacity/2 → 6h; else 12h; closed decay gate → not decaying.
pub fn decay_half_life_label(usage: usize, capacity: usize, decay_gate_open: bool) -> &'static str {
    if !decay_gate_open {
        return DECAY_NOT_DECAYING;
    }
    if usage < capacity / 4 {
        return DECAY_HALF_LIFE_3H;
    }
    if usage < capacity / 2 {
        return DECAY_HALF_LIFE_6H;
    }
    DECAY_HALF_LIFE_12H
}

/// Copies the seven summary counts and ignores recovery member identities.
pub fn recovery_group_from_summary(
    summary: &ManagedMempoolRecoverySummary,
) -> MempoolRecoveryGroup {
    MempoolRecoveryGroup {
        recovered_count: summary.recovered_count,
        dropped_confirmed_count: summary.dropped_confirmed_count,
        dropped_duplicate_count: summary.dropped_duplicate_count,
        dropped_missing_parent_count: summary.dropped_missing_parent_count,
        dropped_policy_incompatible_count: summary.dropped_policy_incompatible_count,
        dropped_expired_count: summary.dropped_expired_count,
        dropped_evicted_count: summary.dropped_evicted_count,
    }
}

/// Maps authority checkpoint evidence without publishing generation integers.
pub fn checkpoint_group_from_evidence(
    evidence: &CheckpointEvidenceSnapshot,
) -> MempoolCheckpointGroup {
    MempoolCheckpointGroup {
        outcome: checkpoint_outcome_label(evidence.outcome).to_string(),
        overdue: evidence.overdue,
        persistence_strength: checkpoint_persistence_label(evidence.maybe_persistence_strength)
            .to_string(),
        age_seconds: evidence.checkpoint_age_seconds,
        loss_bound_seconds: evidence.maybe_loss_bound_seconds,
        dirty_generation_present: evidence.maybe_dirty_generation.is_some(),
    }
}

fn checkpoint_outcome_label(outcome: CheckpointOutcome) -> &'static str {
    match outcome {
        CheckpointOutcome::NeverAttempted => "never_attempted",
        CheckpointOutcome::Pending => "pending",
        CheckpointOutcome::Succeeded => "succeeded",
        CheckpointOutcome::Failed => "failed",
    }
}

fn checkpoint_persistence_label(
    maybe_strength: Option<CheckpointPersistenceStrength>,
) -> &'static str {
    match maybe_strength {
        Some(CheckpointPersistenceStrength::Sync) => "sync",
        None => "unavailable",
    }
}

/// Maps live retry facts without reading Phase 105 deferred counters.
pub fn retry_group_from_relay(
    eligible: u64,
    queued: u64,
    attempted: u64,
    emitted: u64,
    counters: &RelayEvidenceCounters,
    relay_enabled: bool,
    cleared: u64,
) -> MempoolRetryGroup {
    MempoolRetryGroup {
        eligible,
        queued,
        attempted,
        emitted,
        requested: counters.requested_count,
        served: counters.served_count,
        suppressed: counters.suppressed_count,
        relay_disabled: u64::from(!relay_enabled),
        cleared,
    }
}

/// Maps D-15 admission counts only.
pub fn admission_group_from_counts(
    accepted: u64,
    still_present: u64,
    cleared: u64,
) -> MempoolAdmissionGroup {
    MempoolAdmissionGroup {
        accepted,
        still_present,
        cleared,
    }
}

pub(super) fn mempool_resources_unavailable() -> FieldAvailability<MempoolResourcesGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

pub(super) fn mempool_fee_floors_unavailable() -> FieldAvailability<MempoolFeeFloorsGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

pub(super) fn mempool_pressure_unavailable() -> FieldAvailability<MempoolPressureGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

pub(super) fn mempool_eviction_unavailable() -> FieldAvailability<MempoolEvictionGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

pub(super) fn mempool_checkpoint_unavailable() -> FieldAvailability<MempoolCheckpointGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

pub(super) fn mempool_recovery_unavailable() -> FieldAvailability<MempoolRecoveryGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

pub(super) fn mempool_retry_unavailable() -> FieldAvailability<MempoolRetryGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

pub(super) fn mempool_admission_unavailable() -> FieldAvailability<MempoolAdmissionGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

impl MempoolStatus {
    pub fn from_transactions_and_relay(
        transactions: FieldAvailability<u64>,
        relay: RelayEvidenceStatus,
    ) -> Self {
        Self {
            transactions,
            relay,
            ..Self::default()
        }
    }
}

impl Default for MempoolStatus {
    fn default() -> Self {
        Self {
            transactions: FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON),
            relay: RelayEvidenceStatus::default(),
            resources: mempool_resources_unavailable(),
            fee_floors: mempool_fee_floors_unavailable(),
            pressure: mempool_pressure_unavailable(),
            eviction: mempool_eviction_unavailable(),
            checkpoint: mempool_checkpoint_unavailable(),
            recovery: mempool_recovery_unavailable(),
            retry: mempool_retry_unavailable(),
            admission: mempool_admission_unavailable(),
        }
    }
}
