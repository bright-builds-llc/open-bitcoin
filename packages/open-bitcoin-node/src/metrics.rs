// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Serializable metrics retention and status contracts.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::status::{
    FieldAvailability, InboundPeerServingStatus,
    relay_evidence::{RelayEvidenceCounters, RelayEvidenceField, RelayEvidenceStatus},
};

/// Metric series names exposed to status and dashboard consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricKind {
    SyncHeight,
    HeaderHeight,
    DownloadedBlockHeight,
    ConnectedBlockHeight,
    ValidatedActiveChainHeight,
    PeerCount,
    MempoolTransactions,
    WalletTrustedBalanceSats,
    DiskUsageBytes,
    RpcHealth,
    ServiceRestarts,
    InboundAdmittedPeerCount,
    InboundRejectedPeerCount,
    InboundCapRejectCount,
    InboundReservedSlotRejectCount,
    InboundDuplicateRejectCount,
    InboundSelfConnectionRejectCount,
    InboundPermissionedAdmitCount,
    InboundProtectedAdmitCount,
    InboundInactivePermissionEffectCount,
    InboundPermissionValidationFailureCount,
    InboundEvictionCandidateCount,
    InboundDisconnectCount,
    InboundActiveBanCount,
    InboundMisbehaviorObservationCount,
    InboundProtectedNoActionCount,
    InboundResourcePressureActiveCount,
    InboundReadQueuePressureCount,
    InboundWriteQueuePressureCount,
    InboundRequestCapReachedCount,
    InboundPayloadRejectedCount,
    InboundTimeoutDisconnectCount,
    InboundChurnRejectedCount,
    InboundReconnectSuppressedCount,
    RelayAcceptedCount,
    RelayRejectedCount,
    RelayOrphanedCount,
    RelayRequestedCount,
    RelayServedCount,
    RelayAnnouncedCount,
    RelaySuppressedCount,
    RelayEvictedCount,
    RelayExpiredCount,
    RelayRebroadcastDeferredCount,
    RelayRecoveryRecoveredCount,
    RelayRecoveryDroppedConfirmedCount,
    RelayRecoveryDroppedDuplicateCount,
    RelayRecoveryDroppedMissingParentCount,
    RelayRecoveryDroppedPolicyIncompatibleCount,
    RelayRecoveryDroppedEvictedCount,
}

impl MetricKind {
    pub const ALL: [Self; 50] = [
        Self::SyncHeight,
        Self::HeaderHeight,
        Self::DownloadedBlockHeight,
        Self::ConnectedBlockHeight,
        Self::ValidatedActiveChainHeight,
        Self::PeerCount,
        Self::MempoolTransactions,
        Self::WalletTrustedBalanceSats,
        Self::DiskUsageBytes,
        Self::RpcHealth,
        Self::ServiceRestarts,
        Self::InboundAdmittedPeerCount,
        Self::InboundRejectedPeerCount,
        Self::InboundCapRejectCount,
        Self::InboundReservedSlotRejectCount,
        Self::InboundDuplicateRejectCount,
        Self::InboundSelfConnectionRejectCount,
        Self::InboundPermissionedAdmitCount,
        Self::InboundProtectedAdmitCount,
        Self::InboundInactivePermissionEffectCount,
        Self::InboundPermissionValidationFailureCount,
        Self::InboundEvictionCandidateCount,
        Self::InboundDisconnectCount,
        Self::InboundActiveBanCount,
        Self::InboundMisbehaviorObservationCount,
        Self::InboundProtectedNoActionCount,
        Self::InboundResourcePressureActiveCount,
        Self::InboundReadQueuePressureCount,
        Self::InboundWriteQueuePressureCount,
        Self::InboundRequestCapReachedCount,
        Self::InboundPayloadRejectedCount,
        Self::InboundTimeoutDisconnectCount,
        Self::InboundChurnRejectedCount,
        Self::InboundReconnectSuppressedCount,
        Self::RelayAcceptedCount,
        Self::RelayRejectedCount,
        Self::RelayOrphanedCount,
        Self::RelayRequestedCount,
        Self::RelayServedCount,
        Self::RelayAnnouncedCount,
        Self::RelaySuppressedCount,
        Self::RelayEvictedCount,
        Self::RelayExpiredCount,
        Self::RelayRebroadcastDeferredCount,
        Self::RelayRecoveryRecoveredCount,
        Self::RelayRecoveryDroppedConfirmedCount,
        Self::RelayRecoveryDroppedDuplicateCount,
        Self::RelayRecoveryDroppedMissingParentCount,
        Self::RelayRecoveryDroppedPolicyIncompatibleCount,
        Self::RelayRecoveryDroppedEvictedCount,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncHeight => "sync_height",
            Self::HeaderHeight => "header_height",
            Self::DownloadedBlockHeight => "downloaded_block_height",
            Self::ConnectedBlockHeight => "connected_block_height",
            Self::ValidatedActiveChainHeight => "validated_active_chain_height",
            Self::PeerCount => "peer_count",
            Self::MempoolTransactions => "mempool_transactions",
            Self::WalletTrustedBalanceSats => "wallet_trusted_balance_sats",
            Self::DiskUsageBytes => "disk_usage_bytes",
            Self::RpcHealth => "rpc_health",
            Self::ServiceRestarts => "service_restarts",
            Self::InboundAdmittedPeerCount => "inbound_admitted_peer_count",
            Self::InboundRejectedPeerCount => "inbound_rejected_peer_count",
            Self::InboundCapRejectCount => "inbound_cap_reject_count",
            Self::InboundReservedSlotRejectCount => "inbound_reserved_slot_reject_count",
            Self::InboundDuplicateRejectCount => "inbound_duplicate_reject_count",
            Self::InboundSelfConnectionRejectCount => "inbound_self_connection_reject_count",
            Self::InboundPermissionedAdmitCount => "inbound_permissioned_admit_count",
            Self::InboundProtectedAdmitCount => "inbound_protected_admit_count",
            Self::InboundInactivePermissionEffectCount => {
                "inbound_inactive_permission_effect_count"
            }
            Self::InboundPermissionValidationFailureCount => {
                "inbound_permission_validation_failure_count"
            }
            Self::InboundEvictionCandidateCount => "inbound_eviction_candidate_count",
            Self::InboundDisconnectCount => "inbound_disconnect_count",
            Self::InboundActiveBanCount => "inbound_active_ban_count",
            Self::InboundMisbehaviorObservationCount => "inbound_misbehavior_observation_count",
            Self::InboundProtectedNoActionCount => "inbound_protected_no_action_count",
            Self::InboundResourcePressureActiveCount => "inbound_resource_pressure_active_count",
            Self::InboundReadQueuePressureCount => "inbound_read_queue_pressure_count",
            Self::InboundWriteQueuePressureCount => "inbound_write_queue_pressure_count",
            Self::InboundRequestCapReachedCount => "inbound_request_cap_reached_count",
            Self::InboundPayloadRejectedCount => "inbound_payload_rejected_count",
            Self::InboundTimeoutDisconnectCount => "inbound_timeout_disconnect_count",
            Self::InboundChurnRejectedCount => "inbound_churn_rejected_count",
            Self::InboundReconnectSuppressedCount => "inbound_reconnect_suppressed_count",
            Self::RelayAcceptedCount => "relay_accepted_count",
            Self::RelayRejectedCount => "relay_rejected_count",
            Self::RelayOrphanedCount => "relay_orphaned_count",
            Self::RelayRequestedCount => "relay_requested_count",
            Self::RelayServedCount => "relay_served_count",
            Self::RelayAnnouncedCount => "relay_announced_count",
            Self::RelaySuppressedCount => "relay_suppressed_count",
            Self::RelayEvictedCount => "relay_evicted_count",
            Self::RelayExpiredCount => "relay_expired_count",
            Self::RelayRebroadcastDeferredCount => "relay_rebroadcast_deferred_count",
            Self::RelayRecoveryRecoveredCount => "relay_recovery_recovered_count",
            Self::RelayRecoveryDroppedConfirmedCount => "relay_recovery_dropped_confirmed_count",
            Self::RelayRecoveryDroppedDuplicateCount => "relay_recovery_dropped_duplicate_count",
            Self::RelayRecoveryDroppedMissingParentCount => {
                "relay_recovery_dropped_missing_parent_count"
            }
            Self::RelayRecoveryDroppedPolicyIncompatibleCount => {
                "relay_recovery_dropped_policy_incompatible_count"
            }
            Self::RelayRecoveryDroppedEvictedCount => "relay_recovery_dropped_evicted_count",
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

/// Project sanitized relay evidence counters into fixed low-cardinality metric samples.
pub fn relay_metric_samples(
    relay: &RelayEvidenceStatus,
    timestamp_unix_seconds: u64,
) -> Vec<MetricSample> {
    let mut samples = match &relay.outcome_counters {
        RelayEvidenceField::Implemented(counters) => {
            relay_counter_metric_samples(*counters, timestamp_unix_seconds)
        }
        RelayEvidenceField::Unavailable { .. }
        | RelayEvidenceField::Deferred { .. }
        | RelayEvidenceField::IntentionallyDifferent { .. } => Vec::new(),
    };
    if let RelayEvidenceField::Implemented(counters) = &relay.recovery_counters {
        samples.extend([
            MetricSample::new(
                MetricKind::RelayRecoveryRecoveredCount,
                counters.recovered_count as f64,
                timestamp_unix_seconds,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedConfirmedCount,
                counters.dropped_confirmed_count as f64,
                timestamp_unix_seconds,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedDuplicateCount,
                counters.dropped_duplicate_count as f64,
                timestamp_unix_seconds,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedMissingParentCount,
                counters.dropped_missing_parent_count as f64,
                timestamp_unix_seconds,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedPolicyIncompatibleCount,
                counters.dropped_policy_incompatible_count as f64,
                timestamp_unix_seconds,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedEvictedCount,
                counters.dropped_evicted_count as f64,
                timestamp_unix_seconds,
            ),
        ]);
    }
    samples
}

fn relay_counter_metric_samples(
    counters: RelayEvidenceCounters,
    timestamp_unix_seconds: u64,
) -> Vec<MetricSample> {
    vec![
        MetricSample::new(
            MetricKind::RelayAcceptedCount,
            counters.accepted_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::RelayRejectedCount,
            counters.rejected_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::RelayOrphanedCount,
            counters.orphaned_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::RelayRequestedCount,
            counters.requested_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::RelayServedCount,
            counters.served_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::RelayAnnouncedCount,
            counters.announced_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::RelaySuppressedCount,
            counters.suppressed_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::RelayEvictedCount,
            counters.evicted_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::RelayExpiredCount,
            counters.expired_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::RelayRebroadcastDeferredCount,
            counters.rebroadcast_deferred_count as f64,
            timestamp_unix_seconds,
        ),
    ]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricsStatus {
    pub availability: MetricsAvailability,
    pub retention: MetricRetentionPolicy,
    pub enabled_series: Vec<MetricKind>,
    pub samples: Vec<MetricSample>,
}

impl MetricsStatus {
    pub fn available(retention: MetricRetentionPolicy) -> Self {
        Self::available_with_samples(retention, Vec::new())
    }

    pub fn available_with_samples(
        retention: MetricRetentionPolicy,
        samples: Vec<MetricSample>,
    ) -> Self {
        Self {
            availability: MetricsAvailability::Available,
            retention,
            enabled_series: MetricKind::ALL.to_vec(),
            samples,
        }
    }

    pub fn unavailable(retention: MetricRetentionPolicy, reason: impl Into<String>) -> Self {
        Self {
            availability: MetricsAvailability::unavailable(reason),
            retention,
            enabled_series: MetricKind::ALL.to_vec(),
            samples: Vec::new(),
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
mod tests;
