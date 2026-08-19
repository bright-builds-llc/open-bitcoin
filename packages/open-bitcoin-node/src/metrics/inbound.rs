// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use crate::status::{FieldAvailability, InboundPeerServingStatus};

use super::{MetricKind, MetricSample};

/// Project the canonical inbound status aggregate into fixed low-cardinality metric samples.
pub fn inbound_metric_samples(
    inbound: &FieldAvailability<InboundPeerServingStatus>,
    timestamp_unix_seconds: u64,
) -> Vec<MetricSample> {
    let FieldAvailability::Available(status) = inbound else {
        return Vec::new();
    };

    vec![
        MetricSample::new(
            MetricKind::InboundAdmittedPeerCount,
            f64::from(status.admitted_inbound_peers),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundRejectedPeerCount,
            f64::from(status.rejected_inbound_peers),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundCapRejectCount,
            f64::from(status.cap_rejects),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundReservedSlotRejectCount,
            f64::from(status.reserved_slot_rejects),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundDuplicateRejectCount,
            f64::from(status.duplicate_rejects),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundSelfConnectionRejectCount,
            f64::from(status.self_connection_rejects),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundPermissionedAdmitCount,
            f64::from(status.permissioned_inbound_peers),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundProtectedAdmitCount,
            f64::from(status.protected_inbound_peers),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundInactivePermissionEffectCount,
            f64::from(status.inactive_permission_effect_observations),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundPermissionValidationFailureCount,
            f64::from(status.permission_validation_failures),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundEvictionCandidateCount,
            f64::from(status.eviction_candidates_evaluated),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundDisconnectCount,
            f64::from(status.disconnects_requested),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundActiveBanCount,
            f64::from(status.active_bans),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundMisbehaviorObservationCount,
            f64::from(status.misbehavior_observations),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundProtectedNoActionCount,
            f64::from(status.protected_no_actions),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundResourcePressureActiveCount,
            f64::from(status.resource_pressure_events),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundReadQueuePressureCount,
            f64::from(status.read_queue_pressure_events),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundWriteQueuePressureCount,
            f64::from(status.write_queue_pressure_events),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundRequestCapReachedCount,
            f64::from(status.request_cap_events),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundPayloadRejectedCount,
            f64::from(status.payload_rejections),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundTimeoutDisconnectCount,
            f64::from(status.timeout_disconnects),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundChurnRejectedCount,
            f64::from(status.churn_rejections),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::InboundReconnectSuppressedCount,
            f64::from(status.reconnect_suppressions),
            timestamp_unix_seconds,
        ),
    ]
}
