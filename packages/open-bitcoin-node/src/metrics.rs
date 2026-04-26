// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Serializable metrics retention and status contracts.

use serde::{Deserialize, Serialize};

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

impl Default for MetricsStatus {
    fn default() -> Self {
        Self {
            availability: MetricsAvailability::unavailable(
                "metrics history unavailable until runtime collector starts",
            ),
            retention: MetricRetentionPolicy::default(),
            enabled_series: MetricKind::ALL.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus};

    #[test]
    fn default_metric_retention_matches_operator_contract() {
        // Arrange / Act
        let policy = MetricRetentionPolicy::default();

        // Assert
        assert_eq!(policy.sample_interval_seconds, 30);
        assert_eq!(policy.max_samples_per_series, 2_880);
        assert_eq!(policy.max_age_seconds, 86_400);
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
}
