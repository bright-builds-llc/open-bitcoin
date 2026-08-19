// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    FieldAvailability, MempoolAdmissionGroup, MempoolCheckpointGroup, MempoolFeeFloorsGroup,
    MempoolPressureGroup, MempoolRecoveryGroup, MempoolResourcesGroup, MempoolRetryGroup,
    MempoolStatus,
    relay_evidence::{RelayEvidenceCounters, RelayEvidenceField},
};

pub(crate) fn mempool_policy_entries(mempool: &MempoolStatus) -> Vec<(&'static str, String)> {
    vec![
        ("Virtual size", virtual_size_text(&mempool.resources)),
        ("Accounted usage", accounted_usage_text(&mempool.resources)),
        (
            "Accounted capacity",
            accounted_capacity_text(&mempool.resources),
        ),
        (
            "Static relay floor",
            static_relay_floor_text(&mempool.fee_floors),
        ),
        (
            "Rolling mempool floor",
            rolling_mempool_floor_text(&mempool.fee_floors),
        ),
        (
            "Effective admission floor",
            effective_admission_floor_text(&mempool.fee_floors),
        ),
        (
            "Incremental relay fee",
            incremental_relay_fee_text(&mempool.fee_floors),
        ),
        (
            "Pressure removals",
            pressure_removals_text(&mempool.pressure),
        ),
        ("Eviction (relay)", eviction_relay_text(mempool)),
        ("Checkpoint", checkpoint_text(&mempool.checkpoint)),
        ("Recovery", recovery_text(&mempool.recovery)),
        ("Retry", retry_text(&mempool.retry)),
        (
            "Admission states",
            admission_states_text(&mempool.admission),
        ),
        ("Relay states", relay_states_text(&mempool.retry)),
    ]
}

pub(crate) fn mempool_policy_lines(mempool: &MempoolStatus) -> Vec<String> {
    mempool_policy_entries(mempool)
        .into_iter()
        .map(|(label, value)| format!("{label}: {value}"))
        .collect()
}

fn virtual_size_text(value: &FieldAvailability<MempoolResourcesGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!("{} vbytes", group.virtual_size),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn accounted_usage_text(value: &FieldAvailability<MempoolResourcesGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!("{} accounted bytes", group.accounted_usage),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn accounted_capacity_text(value: &FieldAvailability<MempoolResourcesGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => {
            format!("{} accounted bytes", group.accounted_capacity)
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn static_relay_floor_text(value: &FieldAvailability<MempoolFeeFloorsGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!("{} sat/kvB", group.static_relay_floor),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn rolling_mempool_floor_text(value: &FieldAvailability<MempoolFeeFloorsGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!("{} sat/kvB", group.rolling_mempool_floor),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn effective_admission_floor_text(value: &FieldAvailability<MempoolFeeFloorsGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => {
            format!("{} sat/kvB", group.effective_admission_floor)
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn incremental_relay_fee_text(value: &FieldAvailability<MempoolFeeFloorsGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!("{} sat/kvB", group.incremental_relay_fee),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn pressure_removals_text(value: &FieldAvailability<MempoolPressureGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!(
            "pressure_removal_count={} decay={}",
            group.pressure_removal_count, group.decay_half_life_label
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn eviction_relay_text(mempool: &MempoolStatus) -> String {
    if let FieldAvailability::Unavailable { reason } = &mempool.eviction {
        return format!("Unavailable: {reason}");
    }
    match &mempool.relay.outcome_counters {
        RelayEvidenceField::Implemented(counters) => evicted_count_text(counters),
        RelayEvidenceField::Unavailable { reason } => format!("Unavailable: {reason}"),
        RelayEvidenceField::Deferred { reason } => format!("Deferred: {reason}"),
        RelayEvidenceField::IntentionallyDifferent { reason } => {
            format!("Intentionally different: {reason}")
        }
    }
}

fn evicted_count_text(counters: &RelayEvidenceCounters) -> String {
    format!("evicted_count={}", counters.evicted_count)
}

fn checkpoint_text(value: &FieldAvailability<MempoolCheckpointGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!(
            "outcome={} overdue={} persistence_strength={} age_seconds={} loss_bound_seconds={} dirty_generation_present={}",
            group.outcome,
            group.overdue,
            group.persistence_strength,
            option_u64_text(group.age_seconds),
            option_u64_text(group.loss_bound_seconds),
            group.dirty_generation_present
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn option_u64_text(maybe_value: Option<u64>) -> String {
    match maybe_value {
        Some(value) => value.to_string(),
        None => "unavailable".to_string(),
    }
}

fn recovery_text(value: &FieldAvailability<MempoolRecoveryGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!(
            "recovered_count={} dropped_confirmed_count={} dropped_duplicate_count={} dropped_missing_parent_count={} dropped_policy_incompatible_count={} dropped_expired_count={} dropped_evicted_count={}",
            group.recovered_count,
            group.dropped_confirmed_count,
            group.dropped_duplicate_count,
            group.dropped_missing_parent_count,
            group.dropped_policy_incompatible_count,
            group.dropped_expired_count,
            group.dropped_evicted_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn retry_text(value: &FieldAvailability<MempoolRetryGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!(
            "eligible={} queued={} attempted={} emitted={} requested={} served={} suppressed={} relay_disabled={} cleared={}",
            group.eligible,
            group.queued,
            group.attempted,
            group.emitted,
            group.requested,
            group.served,
            group.suppressed,
            group.relay_disabled,
            group.cleared
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn admission_states_text(value: &FieldAvailability<MempoolAdmissionGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!(
            "accepted={} still_present={} cleared={}",
            group.accepted, group.still_present, group.cleared
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn relay_states_text(value: &FieldAvailability<MempoolRetryGroup>) -> String {
    match value {
        FieldAvailability::Available(group) => format!(
            "eligible={} queued={} attempted={} emitted={} requested={} served={} suppressed={} relay_disabled={}",
            group.eligible,
            group.queued,
            group.attempted,
            group.emitted,
            group.requested,
            group.served,
            group.suppressed,
            group.relay_disabled
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}
