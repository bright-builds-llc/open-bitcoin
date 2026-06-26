// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

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
        (MetricKind::DownloadedBlockHeight, "downloaded_block_height"),
        (MetricKind::ConnectedBlockHeight, "connected_block_height"),
        (
            MetricKind::ValidatedActiveChainHeight,
            "validated_active_chain_height",
        ),
        (MetricKind::PeerCount, "peer_count"),
        (MetricKind::MempoolTransactions, "mempool_transactions"),
        (
            MetricKind::WalletTrustedBalanceSats,
            "wallet_trusted_balance_sats",
        ),
        (MetricKind::DiskUsageBytes, "disk_usage_bytes"),
        (MetricKind::RpcHealth, "rpc_health"),
        (MetricKind::ServiceRestarts, "service_restarts"),
        (
            MetricKind::InboundAdmittedPeerCount,
            "inbound_admitted_peer_count",
        ),
        (
            MetricKind::InboundRejectedPeerCount,
            "inbound_rejected_peer_count",
        ),
        (
            MetricKind::InboundCapRejectCount,
            "inbound_cap_reject_count",
        ),
        (
            MetricKind::InboundReservedSlotRejectCount,
            "inbound_reserved_slot_reject_count",
        ),
        (
            MetricKind::InboundDuplicateRejectCount,
            "inbound_duplicate_reject_count",
        ),
        (
            MetricKind::InboundSelfConnectionRejectCount,
            "inbound_self_connection_reject_count",
        ),
        (
            MetricKind::InboundPermissionedAdmitCount,
            "inbound_permissioned_admit_count",
        ),
        (
            MetricKind::InboundProtectedAdmitCount,
            "inbound_protected_admit_count",
        ),
        (
            MetricKind::InboundInactivePermissionEffectCount,
            "inbound_inactive_permission_effect_count",
        ),
        (
            MetricKind::InboundPermissionValidationFailureCount,
            "inbound_permission_validation_failure_count",
        ),
        (
            MetricKind::InboundEvictionCandidateCount,
            "inbound_eviction_candidate_count",
        ),
        (
            MetricKind::InboundDisconnectCount,
            "inbound_disconnect_count",
        ),
        (
            MetricKind::InboundActiveBanCount,
            "inbound_active_ban_count",
        ),
        (
            MetricKind::InboundMisbehaviorObservationCount,
            "inbound_misbehavior_observation_count",
        ),
        (
            MetricKind::InboundProtectedNoActionCount,
            "inbound_protected_no_action_count",
        ),
        (
            MetricKind::InboundResourcePressureActiveCount,
            "inbound_resource_pressure_active_count",
        ),
        (
            MetricKind::InboundReadQueuePressureCount,
            "inbound_read_queue_pressure_count",
        ),
        (
            MetricKind::InboundWriteQueuePressureCount,
            "inbound_write_queue_pressure_count",
        ),
        (
            MetricKind::InboundRequestCapReachedCount,
            "inbound_request_cap_reached_count",
        ),
        (
            MetricKind::InboundPayloadRejectedCount,
            "inbound_payload_rejected_count",
        ),
        (
            MetricKind::InboundTimeoutDisconnectCount,
            "inbound_timeout_disconnect_count",
        ),
        (
            MetricKind::InboundChurnRejectedCount,
            "inbound_churn_rejected_count",
        ),
        (
            MetricKind::InboundReconnectSuppressedCount,
            "inbound_reconnect_suppressed_count",
        ),
    ];

    // Act / Assert
    for (kind, expected_name) in kinds {
        assert_eq!(kind.as_str(), expected_name);
    }
}

#[test]
fn inbound_metric_kinds_are_low_cardinality_counters() {
    // Arrange
    let inbound_kinds = [
        MetricKind::InboundAdmittedPeerCount,
        MetricKind::InboundRejectedPeerCount,
        MetricKind::InboundCapRejectCount,
        MetricKind::InboundReservedSlotRejectCount,
        MetricKind::InboundDuplicateRejectCount,
        MetricKind::InboundSelfConnectionRejectCount,
        MetricKind::InboundPermissionedAdmitCount,
        MetricKind::InboundProtectedAdmitCount,
        MetricKind::InboundInactivePermissionEffectCount,
        MetricKind::InboundPermissionValidationFailureCount,
        MetricKind::InboundEvictionCandidateCount,
        MetricKind::InboundDisconnectCount,
        MetricKind::InboundActiveBanCount,
        MetricKind::InboundMisbehaviorObservationCount,
        MetricKind::InboundProtectedNoActionCount,
        MetricKind::InboundResourcePressureActiveCount,
        MetricKind::InboundReadQueuePressureCount,
        MetricKind::InboundWriteQueuePressureCount,
        MetricKind::InboundRequestCapReachedCount,
        MetricKind::InboundPayloadRejectedCount,
        MetricKind::InboundTimeoutDisconnectCount,
        MetricKind::InboundChurnRejectedCount,
        MetricKind::InboundReconnectSuppressedCount,
    ];

    // Act
    let labels = inbound_kinds
        .into_iter()
        .map(MetricKind::as_str)
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(MetricKind::ALL.len(), 34);
    assert_eq!(
        labels,
        vec![
            "inbound_admitted_peer_count",
            "inbound_rejected_peer_count",
            "inbound_cap_reject_count",
            "inbound_reserved_slot_reject_count",
            "inbound_duplicate_reject_count",
            "inbound_self_connection_reject_count",
            "inbound_permissioned_admit_count",
            "inbound_protected_admit_count",
            "inbound_inactive_permission_effect_count",
            "inbound_permission_validation_failure_count",
            "inbound_eviction_candidate_count",
            "inbound_disconnect_count",
            "inbound_active_ban_count",
            "inbound_misbehavior_observation_count",
            "inbound_protected_no_action_count",
            "inbound_resource_pressure_active_count",
            "inbound_read_queue_pressure_count",
            "inbound_write_queue_pressure_count",
            "inbound_request_cap_reached_count",
            "inbound_payload_rejected_count",
            "inbound_timeout_disconnect_count",
            "inbound_churn_rejected_count",
            "inbound_reconnect_suppressed_count",
        ]
    );
    for label in labels {
        assert!(label.ends_with("_count"));
        for forbidden in [
            ["end", "point"].concat(),
            ["peer", "_id"].concat(),
            ["remote", "_addr"].concat(),
            ["remote", "_end", "point"].concat(),
            ["class", "_name"].concat(),
            ["raw", "_permission"].concat(),
            ["raw", "_config"].concat(),
        ] {
            assert!(!label.contains(&forbidden));
        }
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
    assert!(status.samples.is_empty());
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
