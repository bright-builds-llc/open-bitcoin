// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Fixed low-cardinality mempool-policy metric samples.

use std::collections::BTreeMap;

use crate::network::ManagedNetworkOperatorSnapshot;
use crate::status::{
    FieldAvailability, MempoolStatus, fee_floors_from_managed_info, resources_from_managed_info,
};

use super::MetricKind;

/// Project available mempool snapshot groups into fixed Open Bitcoin metric kinds.
///
/// Unavailable groups are omitted rather than recorded as zero measurements.
pub fn mempool_policy_metric_samples(mempool: &MempoolStatus) -> BTreeMap<MetricKind, u64> {
    let mut samples = BTreeMap::new();
    if let FieldAvailability::Available(resources) = &mempool.resources {
        samples.insert(MetricKind::MempoolVirtualSize, resources.virtual_size);
        samples.insert(MetricKind::MempoolAccountedUsage, resources.accounted_usage);
        samples.insert(
            MetricKind::MempoolAccountedCapacity,
            resources.accounted_capacity,
        );
    }
    if let FieldAvailability::Available(floors) = &mempool.fee_floors {
        samples.insert(
            MetricKind::MempoolStaticRelayFloor,
            signed_floor_as_metric(floors.static_relay_floor),
        );
        samples.insert(
            MetricKind::MempoolRollingMempoolFloor,
            signed_floor_as_metric(floors.rolling_mempool_floor),
        );
        samples.insert(
            MetricKind::MempoolEffectiveAdmissionFloor,
            signed_floor_as_metric(floors.effective_admission_floor),
        );
        samples.insert(
            MetricKind::MempoolIncrementalRelayFee,
            signed_floor_as_metric(floors.incremental_relay_fee),
        );
    }
    if let FieldAvailability::Available(pressure) = &mempool.pressure {
        samples.insert(
            MetricKind::MempoolPressureRemovalCount,
            pressure.pressure_removal_count,
        );
    }
    if let FieldAvailability::Available(checkpoint) = &mempool.checkpoint {
        samples.insert(
            MetricKind::MempoolCheckpointOverdue,
            u64::from(checkpoint.overdue),
        );
    }
    if let FieldAvailability::Available(recovery) = &mempool.recovery {
        samples.insert(
            MetricKind::MempoolRecoveryRecoveredCount,
            recovery.recovered_count,
        );
    }
    if let FieldAvailability::Available(retry) = &mempool.retry {
        samples.insert(MetricKind::MempoolRetryEligible, retry.eligible);
        samples.insert(MetricKind::MempoolRetryCleared, retry.cleared);
    }
    if let FieldAvailability::Available(admission) = &mempool.admission {
        samples.insert(MetricKind::MempoolAdmissionAccepted, admission.accepted);
        samples.insert(
            MetricKind::MempoolAdmissionStillPresent,
            admission.still_present,
        );
    }
    samples
}

/// Assemble identifier-free `MempoolStatus` groups from a live operator snapshot.
pub fn mempool_status_from_operator_snapshot(
    snapshot: &ManagedNetworkOperatorSnapshot,
) -> MempoolStatus {
    let mempool_info = snapshot.mempool();
    MempoolStatus {
        transactions: FieldAvailability::available(mempool_info.transaction_count as u64),
        relay: snapshot.relay().clone(),
        resources: FieldAvailability::available(resources_from_managed_info(mempool_info)),
        fee_floors: FieldAvailability::available(fee_floors_from_managed_info(mempool_info)),
        pressure: FieldAvailability::available(snapshot.pressure().clone()),
        eviction: FieldAvailability::available(*snapshot.eviction()),
        checkpoint: FieldAvailability::available(snapshot.checkpoint().clone()),
        recovery: FieldAvailability::available(*snapshot.recovery()),
        retry: FieldAvailability::available(*snapshot.retry()),
        admission: FieldAvailability::available(*snapshot.admission()),
    }
}

fn signed_floor_as_metric(value: i64) -> u64 {
    u64::try_from(value).unwrap_or(0)
}
