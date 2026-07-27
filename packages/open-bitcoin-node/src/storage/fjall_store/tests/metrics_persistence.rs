// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn metrics_history_appends_across_reopen() {
    // Arrange
    let path = temp_store_path("metrics-history-reopen");
    remove_dir_if_exists(&path);
    let retention = MetricRetentionPolicy {
        sample_interval_seconds: 30,
        max_samples_per_series: 4,
        max_age_seconds: 1_000,
    };

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .append_metric_samples(
                &[MetricSample::new(MetricKind::SyncHeight, 10.0, 10)],
                retention,
                10,
                PersistMode::Sync,
            )
            .expect("append first metrics");
        store
            .append_metric_samples(
                &[MetricSample::new(MetricKind::SyncHeight, 20.0, 40)],
                retention,
                40,
                PersistMode::Sync,
            )
            .expect("append second metrics");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_metrics_snapshot()
            .expect("load metrics")
            .expect("metrics snapshot")
            .samples,
        vec![
            MetricSample::new(MetricKind::SyncHeight, 10.0, 10),
            MetricSample::new(MetricKind::SyncHeight, 20.0, 40),
        ]
    );

    remove_dir_if_exists(&path);
}

#[test]
fn metrics_history_prunes_per_series_cap() {
    // Arrange
    let path = temp_store_path("metrics-history-series-cap");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let retention = MetricRetentionPolicy {
        sample_interval_seconds: 30,
        max_samples_per_series: 2,
        max_age_seconds: 1_000,
    };
    store
        .append_metric_samples(
            &[MetricSample::new(MetricKind::SyncHeight, 5.0, 5)],
            retention,
            5,
            PersistMode::Sync,
        )
        .expect("append sync metrics");

    // Act
    let snapshot = store
        .append_metric_samples(
            &[
                MetricSample::new(MetricKind::HeaderHeight, 10.0, 10),
                MetricSample::new(MetricKind::HeaderHeight, 20.0, 20),
                MetricSample::new(MetricKind::HeaderHeight, 30.0, 30),
            ],
            retention,
            30,
            PersistMode::Sync,
        )
        .expect("append header metrics");

    // Assert
    assert_eq!(
        snapshot.samples,
        vec![
            MetricSample::new(MetricKind::SyncHeight, 5.0, 5),
            MetricSample::new(MetricKind::HeaderHeight, 20.0, 20),
            MetricSample::new(MetricKind::HeaderHeight, 30.0, 30),
        ]
    );

    remove_dir_if_exists(&path);
}

#[test]
fn metrics_history_prunes_expired_samples() {
    // Arrange
    let path = temp_store_path("metrics-history-expired");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let retention = MetricRetentionPolicy {
        sample_interval_seconds: 30,
        max_samples_per_series: 4,
        max_age_seconds: 10,
    };
    store
        .append_metric_samples(
            &[MetricSample::new(MetricKind::PeerCount, 1.0, 10)],
            retention,
            10,
            PersistMode::Sync,
        )
        .expect("append old metrics");

    // Act
    let snapshot = store
        .append_metric_samples(
            &[MetricSample::new(MetricKind::PeerCount, 2.0, 25)],
            retention,
            25,
            PersistMode::Sync,
        )
        .expect("append fresh metrics");

    // Assert
    assert_eq!(
        snapshot.samples,
        vec![MetricSample::new(MetricKind::PeerCount, 2.0, 25)]
    );

    remove_dir_if_exists(&path);
}

#[test]
fn missing_metrics_snapshot_reports_unavailable_status() {
    // Arrange
    let path = temp_store_path("metrics-status");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let retention = MetricRetentionPolicy {
        sample_interval_seconds: 15,
        max_samples_per_series: 2,
        max_age_seconds: 60,
    };

    // Act
    let missing_status = store
        .load_metrics_status(retention)
        .expect("load missing metrics status");
    store
        .save_metrics_snapshot(
            &MetricsStorageSnapshot {
                samples: Vec::new(),
            },
            PersistMode::Sync,
        )
        .expect("save empty metrics snapshot");
    let available_status = store
        .load_metrics_status(retention)
        .expect("load available metrics status");

    // Assert
    assert_eq!(missing_status.retention, retention);
    assert_eq!(missing_status.enabled_series, MetricKind::ALL.to_vec());
    assert_eq!(
        missing_status.availability,
        MetricsAvailability::Unavailable {
            reason: "metrics history unavailable: no metrics snapshot recorded".to_string()
        }
    );
    assert_eq!(available_status.retention, retention);
    assert_eq!(available_status.enabled_series, MetricKind::ALL.to_vec());
    assert_eq!(
        available_status.availability,
        MetricsAvailability::Available
    );

    remove_dir_if_exists(&path);
}
