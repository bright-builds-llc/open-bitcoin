// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Serializable metrics retention and status contracts.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Metric series names exposed to status and dashboard consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricKind {
    SyncHeight,
    HeaderHeight,
    PeerCount,
    MempoolTransactions,
    WalletTrustedBalanceSats,
    DiskUsageBytes,
    RpcHealth,
    ServiceRestarts,
}

impl MetricKind {
    pub const ALL: [Self; 8] = [
        Self::SyncHeight,
        Self::HeaderHeight,
        Self::PeerCount,
        Self::MempoolTransactions,
        Self::WalletTrustedBalanceSats,
        Self::DiskUsageBytes,
        Self::RpcHealth,
        Self::ServiceRestarts,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncHeight => "sync_height",
            Self::HeaderHeight => "header_height",
            Self::PeerCount => "peer_count",
            Self::MempoolTransactions => "mempool_transactions",
            Self::WalletTrustedBalanceSats => "wallet_trusted_balance_sats",
            Self::DiskUsageBytes => "disk_usage_bytes",
            Self::RpcHealth => "rpc_health",
            Self::ServiceRestarts => "service_restarts",
        }
    }
}

/// Bounded retention policy for historical metric series.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetricRetentionPolicy {
    pub sample_interval_seconds: u64,
    pub max_samples_per_series: usize,
    pub max_age_seconds: u64,
}

impl Default for MetricRetentionPolicy {
    fn default() -> Self {
        Self {
            sample_interval_seconds: 30,
            max_samples_per_series: 2_880,
            max_age_seconds: 86_400,
        }
    }
}

/// Numeric metric value captured at a specific Unix timestamp.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricSample {
    pub kind: MetricKind,
    pub value: f64,
    pub timestamp_unix_seconds: u64,
}

impl MetricSample {
    pub const fn new(kind: MetricKind, value: f64, timestamp_unix_seconds: u64) -> Self {
        Self {
            kind,
            value,
            timestamp_unix_seconds,
        }
    }
}

/// Combine new samples with existing history and enforce bounded per-series retention.
pub fn append_and_prune_metric_samples(
    existing_samples: &[MetricSample],
    new_samples: &[MetricSample],
    policy: MetricRetentionPolicy,
    now_unix_seconds: u64,
) -> Vec<MetricSample> {
    let minimum_timestamp = now_unix_seconds.saturating_sub(policy.max_age_seconds);
    let sample_interval_seconds = policy.sample_interval_seconds.max(1);
    let mut retained = Vec::new();

    for kind in MetricKind::ALL {
        let mut samples_by_bucket: BTreeMap<u64, MetricSample> = BTreeMap::new();
        for sample in existing_samples
            .iter()
            .chain(new_samples.iter())
            .filter(|sample| {
                sample.kind == kind && sample.timestamp_unix_seconds >= minimum_timestamp
            })
        {
            let bucket = sample.timestamp_unix_seconds / sample_interval_seconds;
            match samples_by_bucket.get(&bucket) {
                Some(retained_sample)
                    if retained_sample.timestamp_unix_seconds > sample.timestamp_unix_seconds => {}
                _ => {
                    samples_by_bucket.insert(bucket, sample.clone());
                }
            }
        }

        let series = samples_by_bucket.into_values().collect::<Vec<_>>();
        let retained_count = series.len().min(policy.max_samples_per_series);
        let start = series.len().saturating_sub(retained_count);
        retained.extend(series.into_iter().skip(start));
    }

    retained
}

/// Availability of the metrics collector or history store.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum MetricsAvailability {
    Available,
    Unavailable { reason: String },
}

impl MetricsAvailability {
    pub fn unavailable(reason: impl Into<String>) -> Self {
        Self::Unavailable {
            reason: reason.into(),
        }
    }
}

/// Metrics status projection embedded in the shared node status snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetricsStatus {
    pub availability: MetricsAvailability,
    pub retention: MetricRetentionPolicy,
    pub enabled_series: Vec<MetricKind>,
}

impl MetricsStatus {
    pub fn available(retention: MetricRetentionPolicy) -> Self {
        Self {
            availability: MetricsAvailability::Available,
            retention,
            enabled_series: MetricKind::ALL.to_vec(),
        }
    }

    pub fn unavailable(retention: MetricRetentionPolicy, reason: impl Into<String>) -> Self {
        Self {
            availability: MetricsAvailability::unavailable(reason),
            retention,
            enabled_series: MetricKind::ALL.to_vec(),
        }
    }
}

impl Default for MetricsStatus {
    fn default() -> Self {
        Self::unavailable(
            MetricRetentionPolicy::default(),
            "metrics history unavailable until runtime collector starts",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MetricKind, MetricRetentionPolicy, MetricSample, MetricsAvailability, MetricsStatus,
        append_and_prune_metric_samples,
    };

    #[test]
    fn default_metric_retention_matches_operator_contract() {
        // Arrange / Act
        let policy = MetricRetentionPolicy::default();

        // Assert
        assert_eq!(policy.sample_interval_seconds, 30);
        assert_eq!(policy.max_samples_per_series, 2_880);
        assert_eq!(policy.max_age_seconds, 86_400);
        assert_eq!(
            policy.sample_interval_seconds * policy.max_samples_per_series as u64,
            policy.max_age_seconds
        );
    }

    #[test]
    fn metric_kind_names_are_stable() {
        // Arrange
        let kinds = [
            (MetricKind::SyncHeight, "sync_height"),
            (MetricKind::HeaderHeight, "header_height"),
            (MetricKind::PeerCount, "peer_count"),
            (MetricKind::MempoolTransactions, "mempool_transactions"),
            (
                MetricKind::WalletTrustedBalanceSats,
                "wallet_trusted_balance_sats",
            ),
            (MetricKind::DiskUsageBytes, "disk_usage_bytes"),
            (MetricKind::RpcHealth, "rpc_health"),
            (MetricKind::ServiceRestarts, "service_restarts"),
        ];

        // Act / Assert
        for (kind, expected_name) in kinds {
            assert_eq!(kind.as_str(), expected_name);
        }
    }

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
        assert_eq!(
            serde_json::to_value(&status.availability).expect("availability json")["state"],
            "unavailable"
        );
    }

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
        let retained =
            append_and_prune_metric_samples(&existing_samples, &new_samples, policy, 200);

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
        let retained =
            append_and_prune_metric_samples(&existing_samples, &new_samples, policy, 200);

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
        let retained =
            append_and_prune_metric_samples(&existing_samples, &new_samples, policy, 200);

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
        let retained =
            append_and_prune_metric_samples(&existing_samples, &new_samples, policy, 200);

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
        assert_eq!(unavailable.retention, policy);
        assert_eq!(unavailable.enabled_series, MetricKind::ALL.to_vec());
        assert_eq!(
            unavailable.availability,
            MetricsAvailability::Unavailable {
                reason: "metrics collector not started".to_string()
            }
        );
    }
}
