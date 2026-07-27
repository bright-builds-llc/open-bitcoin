// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn metric_sample_round_trips_through_json() {
    // Arrange
    let sample = MetricSample::new(MetricKind::HeaderHeight, 840_000.0, 1_777_225_022);

    // Act
    let encoded = serde_json::to_string(&sample).expect("metric sample json");
    let decoded: MetricSample = serde_json::from_str(&encoded).expect("metric sample decode");

    // Assert
    assert_eq!(decoded, sample);
}

#[test]
fn default_metrics_status_exposes_retention_and_series() {
    // Arrange / Act
    let status = MetricsStatus::default();

    // Assert
    assert_eq!(status.retention, MetricRetentionPolicy::default());
    assert_eq!(status.enabled_series, MetricKind::ALL.to_vec());
    assert!(status.samples.is_empty());
    assert_eq!(
        serde_json::to_value(&status.availability).expect("availability json")["state"],
        "unavailable"
    );
}
