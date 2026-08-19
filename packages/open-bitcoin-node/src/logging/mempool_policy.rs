// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Allowlisted mempool-policy structured log records.

use crate::status::{FieldAvailability, MempoolStatus};

use super::{MEMPOOL_POLICY_LOG_SOURCE, StructuredLogLevel, StructuredLogRecord};

/// Build an Info record whose message uses only snapshot-matching keys.
///
/// The builder accepts group counts only. Recovery member identities never enter
/// the message, even when a caller holds a `MempoolRecoveryRecord` elsewhere.
pub fn mempool_policy_log_record(
    mempool: &MempoolStatus,
    timestamp_unix_seconds: u64,
) -> StructuredLogRecord {
    let (virtual_size, accounted_usage, accounted_capacity) = match &mempool.resources {
        FieldAvailability::Available(resources) => (
            resources.virtual_size,
            resources.accounted_usage,
            resources.accounted_capacity,
        ),
        FieldAvailability::Unavailable { .. } => (0, 0, 0),
    };
    let (
        static_relay_floor,
        rolling_mempool_floor,
        effective_admission_floor,
        incremental_relay_fee,
    ) = match &mempool.fee_floors {
        FieldAvailability::Available(floors) => (
            floors.static_relay_floor,
            floors.rolling_mempool_floor,
            floors.effective_admission_floor,
            floors.incremental_relay_fee,
        ),
        FieldAvailability::Unavailable { .. } => (0, 0, 0, 0),
    };
    let (pressure_removal_count, decay) = match &mempool.pressure {
        FieldAvailability::Available(pressure) => (
            pressure.pressure_removal_count,
            allowlisted_decay(&pressure.decay_half_life_label),
        ),
        FieldAvailability::Unavailable { .. } => (0, "not_decaying"),
    };
    let recovered_count = match &mempool.recovery {
        FieldAvailability::Available(recovery) => recovery.recovered_count,
        FieldAvailability::Unavailable { .. } => 0,
    };
    let (retry_eligible, retry_cleared, relay_disabled) = match &mempool.retry {
        FieldAvailability::Available(retry) => {
            (retry.eligible, retry.cleared, retry.relay_disabled)
        }
        FieldAvailability::Unavailable { .. } => (0, 0, 0),
    };
    let (admission_accepted, admission_still_present, admission_cleared) = match &mempool.admission
    {
        FieldAvailability::Available(admission) => (
            admission.accepted,
            admission.still_present,
            admission.cleared,
        ),
        FieldAvailability::Unavailable { .. } => (0, 0, 0),
    };
    let message = format!(
        "virtual_size={virtual_size} accounted_usage={accounted_usage} accounted_capacity={accounted_capacity} static_relay_floor={static_relay_floor} rolling_mempool_floor={rolling_mempool_floor} effective_admission_floor={effective_admission_floor} incremental_relay_fee={incremental_relay_fee} pressure_removal_count={pressure_removal_count} decay={decay} recovered_count={recovered_count} retry_eligible={retry_eligible} retry_cleared={retry_cleared} admission_accepted={admission_accepted} admission_still_present={admission_still_present} admission_cleared={admission_cleared} relay_disabled={relay_disabled}"
    );

    StructuredLogRecord::new(
        StructuredLogLevel::Info,
        MEMPOOL_POLICY_LOG_SOURCE,
        message,
        timestamp_unix_seconds,
    )
}

fn allowlisted_decay(label: &str) -> &'static str {
    match label {
        "half_life_12h" => "half_life_12h",
        "half_life_6h" => "half_life_6h",
        "half_life_3h" => "half_life_3h",
        _ => "not_decaying",
    }
}
