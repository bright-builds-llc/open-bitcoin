// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn append_and_prune_metric_samples_drops_expired_samples() {
    // Arrange
    let policy = MetricRetentionPolicy {
        sample_interval_seconds: 30,
        max_samples_per_series: 4,
        max_age_seconds: 50,
    };
    let existing_samples = MetricKind::ALL
        .into_iter()
        .map(|kind| MetricSample::new(kind, 1.0, 149))
        .collect::<Vec<_>>();
    let new_samples = MetricKind::ALL
        .into_iter()
        .map(|kind| MetricSample::new(kind, 2.0, 150))
        .collect::<Vec<_>>();

    // Act
    let retained = append_and_prune_metric_samples(&existing_samples, &new_samples, policy, 200);

    // Assert
    assert_eq!(retained, new_samples);
}

#[test]
fn append_and_prune_metric_samples_caps_each_series() {
    // Arrange
    let policy = MetricRetentionPolicy {
        sample_interval_seconds: 1,
        max_samples_per_series: 2,
        max_age_seconds: 1_000,
    };
    let existing_samples = vec![
        MetricSample::new(MetricKind::HeaderHeight, 10.0, 100),
        MetricSample::new(MetricKind::SyncHeight, 1.0, 105),
    ];
    let new_samples = vec![
        MetricSample::new(MetricKind::HeaderHeight, 11.0, 110),
        MetricSample::new(MetricKind::HeaderHeight, 12.0, 120),
        MetricSample::new(MetricKind::HeaderHeight, 13.0, 130),
    ];

    // Act
    let retained = append_and_prune_metric_samples(&existing_samples, &new_samples, policy, 200);

    // Assert
    assert_eq!(
        retained,
        vec![
            MetricSample::new(MetricKind::SyncHeight, 1.0, 105),
            MetricSample::new(MetricKind::HeaderHeight, 12.0, 120),
            MetricSample::new(MetricKind::HeaderHeight, 13.0, 130),
        ]
    );
}

#[test]
fn append_and_prune_metric_samples_orders_by_kind_then_timestamp() {
    // Arrange
    let policy = MetricRetentionPolicy {
        sample_interval_seconds: 1,
        max_samples_per_series: 4,
        max_age_seconds: 1_000,
    };
    let existing_samples = vec![
        MetricSample::new(MetricKind::PeerCount, 3.0, 10),
        MetricSample::new(MetricKind::SyncHeight, 1.0, 50),
    ];
    let new_samples = vec![
        MetricSample::new(MetricKind::HeaderHeight, 2.0, 20),
        MetricSample::new(MetricKind::SyncHeight, 1.5, 40),
    ];

    // Act
    let retained = append_and_prune_metric_samples(&existing_samples, &new_samples, policy, 200);

    // Assert
    assert_eq!(
        retained,
        vec![
            MetricSample::new(MetricKind::SyncHeight, 1.5, 40),
            MetricSample::new(MetricKind::SyncHeight, 1.0, 50),
            MetricSample::new(MetricKind::HeaderHeight, 2.0, 20),
            MetricSample::new(MetricKind::PeerCount, 3.0, 10),
        ]
    );
}

#[test]
fn append_and_prune_metric_samples_enforces_sample_interval_buckets() {
    // Arrange
    let policy = MetricRetentionPolicy {
        sample_interval_seconds: 30,
        max_samples_per_series: 2,
        max_age_seconds: 1_000,
    };
    let existing_samples = vec![
        MetricSample::new(MetricKind::HeaderHeight, 100.0, 100),
        MetricSample::new(MetricKind::HeaderHeight, 101.0, 110),
    ];
    let new_samples = vec![
        MetricSample::new(MetricKind::HeaderHeight, 102.0, 119),
        MetricSample::new(MetricKind::HeaderHeight, 103.0, 120),
        MetricSample::new(MetricKind::HeaderHeight, 104.0, 149),
    ];

    // Act
    let retained = append_and_prune_metric_samples(&existing_samples, &new_samples, policy, 200);

    // Assert
    assert_eq!(
        retained,
        vec![
            MetricSample::new(MetricKind::HeaderHeight, 102.0, 119),
            MetricSample::new(MetricKind::HeaderHeight, 104.0, 149),
        ]
    );
}

#[test]
fn available_metrics_status_preserves_retention_and_series() {
    // Arrange
    let policy = MetricRetentionPolicy {
        sample_interval_seconds: 15,
        max_samples_per_series: 3,
        max_age_seconds: 60,
    };

    // Act
    let available = MetricsStatus::available(policy);
    let unavailable = MetricsStatus::unavailable(policy, "metrics collector not started");

    // Assert
    assert_eq!(available.retention, policy);
    assert_eq!(available.enabled_series, MetricKind::ALL.to_vec());
    assert_eq!(available.availability, MetricsAvailability::Available);
    assert!(available.samples.is_empty());
    assert_eq!(unavailable.retention, policy);
    assert_eq!(unavailable.enabled_series, MetricKind::ALL.to_vec());
    assert!(unavailable.samples.is_empty());
    assert_eq!(
        unavailable.availability,
        MetricsAvailability::Unavailable {
            reason: "metrics collector not started".to_string()
        }
    );
}

#[test]
fn available_metrics_status_can_carry_bounded_samples() {
    // Arrange
    let policy = MetricRetentionPolicy {
        sample_interval_seconds: 15,
        max_samples_per_series: 3,
        max_age_seconds: 60,
    };
    let samples = vec![MetricSample::new(MetricKind::SyncHeight, 840_000.0, 10)];

    // Act
    let status = MetricsStatus::available_with_samples(policy, samples.clone());

    // Assert
    assert_eq!(status.retention, policy);
    assert_eq!(status.samples, samples);
}
