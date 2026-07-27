// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn unavailable_inbound_status_emits_no_metric_samples() {
    // Arrange
    let inbound = FieldAvailability::unavailable("inbound status unavailable");

    // Act
    let samples = inbound_metric_samples(&inbound, 1_777_225_022);

    // Assert
    assert!(samples.is_empty());
}

#[test]
fn inbound_status_maps_to_each_fixed_inbound_metric_kind() {
    // Arrange
    let timestamp = 1_777_225_022;
    let inbound = FieldAvailability::available(inbound_status_fixture());

    // Act
    let samples = inbound_metric_samples(&inbound, timestamp);

    // Assert
    assert_eq!(
        samples,
        vec![
            MetricSample::new(MetricKind::InboundAdmittedPeerCount, 1.0, timestamp),
            MetricSample::new(MetricKind::InboundRejectedPeerCount, 2.0, timestamp),
            MetricSample::new(MetricKind::InboundCapRejectCount, 3.0, timestamp),
            MetricSample::new(MetricKind::InboundReservedSlotRejectCount, 4.0, timestamp),
            MetricSample::new(MetricKind::InboundDuplicateRejectCount, 5.0, timestamp),
            MetricSample::new(MetricKind::InboundSelfConnectionRejectCount, 6.0, timestamp,),
            MetricSample::new(MetricKind::InboundPermissionedAdmitCount, 7.0, timestamp),
            MetricSample::new(MetricKind::InboundProtectedAdmitCount, 8.0, timestamp),
            MetricSample::new(
                MetricKind::InboundInactivePermissionEffectCount,
                9.0,
                timestamp,
            ),
            MetricSample::new(
                MetricKind::InboundPermissionValidationFailureCount,
                10.0,
                timestamp,
            ),
            MetricSample::new(MetricKind::InboundEvictionCandidateCount, 11.0, timestamp),
            MetricSample::new(MetricKind::InboundDisconnectCount, 12.0, timestamp),
            MetricSample::new(MetricKind::InboundActiveBanCount, 13.0, timestamp),
            MetricSample::new(
                MetricKind::InboundMisbehaviorObservationCount,
                14.0,
                timestamp,
            ),
            MetricSample::new(MetricKind::InboundProtectedNoActionCount, 15.0, timestamp),
            MetricSample::new(
                MetricKind::InboundResourcePressureActiveCount,
                16.0,
                timestamp,
            ),
            MetricSample::new(MetricKind::InboundReadQueuePressureCount, 17.0, timestamp),
            MetricSample::new(MetricKind::InboundWriteQueuePressureCount, 18.0, timestamp),
            MetricSample::new(MetricKind::InboundRequestCapReachedCount, 19.0, timestamp),
            MetricSample::new(MetricKind::InboundPayloadRejectedCount, 20.0, timestamp),
            MetricSample::new(MetricKind::InboundTimeoutDisconnectCount, 21.0, timestamp),
            MetricSample::new(MetricKind::InboundChurnRejectedCount, 22.0, timestamp),
            MetricSample::new(MetricKind::InboundReconnectSuppressedCount, 23.0, timestamp,),
        ]
    );
}

#[test]
fn inactive_permission_metric_uses_observation_count_not_label_count() {
    // Arrange
    let timestamp = 1_777_225_022;
    let mut status = inbound_status_fixture();
    status.inactive_permission_effects = vec!["label_one".to_string(), "label_two".to_string()];
    status.inactive_permission_effect_observations = 9;
    let inbound = FieldAvailability::available(status);

    // Act
    let samples = inbound_metric_samples(&inbound, timestamp);
    let sample = samples
        .iter()
        .find(|sample| sample.kind == MetricKind::InboundInactivePermissionEffectCount)
        .expect("inactive permission metric sample");

    // Assert
    assert_eq!(sample.value, 9.0);
}
