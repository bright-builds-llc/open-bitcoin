// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    MetricKind, MetricRetentionPolicy, MetricSample, MetricsAvailability, MetricsStatus,
    append_and_prune_metric_samples, block_relay_metric_samples, inbound_metric_samples,
    relay_metric_samples,
};
use crate::status::{
    BlockRelayEvidenceStatus, FieldAvailability, InboundHandshakeStatusCounts,
    InboundPeerServingStatus,
    relay_evidence::{RelayEvidenceCounters, RelayEvidenceStatus},
    relay_evidence::{RelayEvidenceField, RelayRecoveryCounters},
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
        (MetricKind::RelayAcceptedCount, "relay_accepted_count"),
        (MetricKind::RelayRejectedCount, "relay_rejected_count"),
        (MetricKind::RelayOrphanedCount, "relay_orphaned_count"),
        (MetricKind::RelayRequestedCount, "relay_requested_count"),
        (MetricKind::RelayServedCount, "relay_served_count"),
        (MetricKind::RelayAnnouncedCount, "relay_announced_count"),
        (MetricKind::RelaySuppressedCount, "relay_suppressed_count"),
        (MetricKind::RelayEvictedCount, "relay_evicted_count"),
        (MetricKind::RelayExpiredCount, "relay_expired_count"),
        (
            MetricKind::RelayRebroadcastDeferredCount,
            "relay_rebroadcast_deferred_count",
        ),
        (
            MetricKind::RelayRecoveryRecoveredCount,
            "relay_recovery_recovered_count",
        ),
        (
            MetricKind::RelayRecoveryDroppedConfirmedCount,
            "relay_recovery_dropped_confirmed_count",
        ),
        (
            MetricKind::RelayRecoveryDroppedDuplicateCount,
            "relay_recovery_dropped_duplicate_count",
        ),
        (
            MetricKind::RelayRecoveryDroppedMissingParentCount,
            "relay_recovery_dropped_missing_parent_count",
        ),
        (
            MetricKind::RelayRecoveryDroppedPolicyIncompatibleCount,
            "relay_recovery_dropped_policy_incompatible_count",
        ),
        (
            MetricKind::RelayRecoveryDroppedEvictedCount,
            "relay_recovery_dropped_evicted_count",
        ),
        (MetricKind::BlockServedCount, "block_served_count"),
        (
            MetricKind::BlockServingSuppressedCount,
            "block_serving_suppressed_count",
        ),
        (MetricKind::CompactAnnouncedCount, "compact_announced_count"),
        (
            MetricKind::CompactReconstructedCount,
            "compact_reconstructed_count",
        ),
        (
            MetricKind::CompactMissingTxRequestedCount,
            "compact_missing_tx_requested_count",
        ),
        (MetricKind::CompactFallbackCount, "compact_fallback_count"),
        (MetricKind::CompactMalformedCount, "compact_malformed_count"),
        (MetricKind::CompactTimeoutCount, "compact_timeout_count"),
        (MetricKind::CompactCleanupCount, "compact_cleanup_count"),
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
    assert_eq!(MetricKind::ALL.len(), 59);
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
fn relay_metric_kinds_are_low_cardinality_counters() {
    // Arrange
    let relay_kinds = [
        MetricKind::RelayAcceptedCount,
        MetricKind::RelayRejectedCount,
        MetricKind::RelayOrphanedCount,
        MetricKind::RelayRequestedCount,
        MetricKind::RelayServedCount,
        MetricKind::RelayAnnouncedCount,
        MetricKind::RelaySuppressedCount,
        MetricKind::RelayEvictedCount,
        MetricKind::RelayExpiredCount,
        MetricKind::RelayRebroadcastDeferredCount,
        MetricKind::RelayRecoveryRecoveredCount,
        MetricKind::RelayRecoveryDroppedConfirmedCount,
        MetricKind::RelayRecoveryDroppedDuplicateCount,
        MetricKind::RelayRecoveryDroppedMissingParentCount,
        MetricKind::RelayRecoveryDroppedPolicyIncompatibleCount,
        MetricKind::RelayRecoveryDroppedEvictedCount,
    ];

    // Act
    let labels = relay_kinds
        .into_iter()
        .map(MetricKind::as_str)
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        labels,
        vec![
            "relay_accepted_count",
            "relay_rejected_count",
            "relay_orphaned_count",
            "relay_requested_count",
            "relay_served_count",
            "relay_announced_count",
            "relay_suppressed_count",
            "relay_evicted_count",
            "relay_expired_count",
            "relay_rebroadcast_deferred_count",
            "relay_recovery_recovered_count",
            "relay_recovery_dropped_confirmed_count",
            "relay_recovery_dropped_duplicate_count",
            "relay_recovery_dropped_missing_parent_count",
            "relay_recovery_dropped_policy_incompatible_count",
            "relay_recovery_dropped_evicted_count",
        ]
    );
    for label in labels {
        assert!(label.ends_with("_count"));
        for forbidden in [
            "peer_id",
            "endpoint",
            "txid",
            "wtxid",
            "permission",
            "credential",
            "cookie",
            "secret",
            "reject_reason",
            "label_",
        ] {
            assert!(!label.contains(forbidden));
        }
    }
}

#[test]
fn relay_status_maps_to_each_fixed_relay_metric_kind() {
    // Arrange
    let timestamp = 1_777_225_105;
    let mut relay = RelayEvidenceStatus::with_counters(RelayEvidenceCounters {
        accepted_count: 1,
        rejected_count: 2,
        orphaned_count: 3,
        requested_count: 4,
        served_count: 5,
        announced_count: 6,
        suppressed_count: 7,
        evicted_count: 8,
        expired_count: 9,
        rebroadcast_deferred_count: 10,
    });
    relay.recovery_counters = RelayEvidenceField::implemented(RelayRecoveryCounters {
        recovered_count: 11,
        dropped_confirmed_count: 12,
        dropped_duplicate_count: 13,
        dropped_missing_parent_count: 14,
        dropped_policy_incompatible_count: 15,
        dropped_evicted_count: 16,
    });

    // Act
    let samples = relay_metric_samples(&relay, timestamp);

    // Assert
    assert_eq!(
        samples,
        vec![
            MetricSample::new(MetricKind::RelayAcceptedCount, 1.0, timestamp),
            MetricSample::new(MetricKind::RelayRejectedCount, 2.0, timestamp),
            MetricSample::new(MetricKind::RelayOrphanedCount, 3.0, timestamp),
            MetricSample::new(MetricKind::RelayRequestedCount, 4.0, timestamp),
            MetricSample::new(MetricKind::RelayServedCount, 5.0, timestamp),
            MetricSample::new(MetricKind::RelayAnnouncedCount, 6.0, timestamp),
            MetricSample::new(MetricKind::RelaySuppressedCount, 7.0, timestamp),
            MetricSample::new(MetricKind::RelayEvictedCount, 8.0, timestamp),
            MetricSample::new(MetricKind::RelayExpiredCount, 9.0, timestamp),
            MetricSample::new(MetricKind::RelayRebroadcastDeferredCount, 10.0, timestamp),
            MetricSample::new(MetricKind::RelayRecoveryRecoveredCount, 11.0, timestamp),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedConfirmedCount,
                12.0,
                timestamp,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedDuplicateCount,
                13.0,
                timestamp,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedMissingParentCount,
                14.0,
                timestamp,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedPolicyIncompatibleCount,
                15.0,
                timestamp,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedEvictedCount,
                16.0,
                timestamp,
            ),
        ]
    );
    let serialized = serde_json::to_string(&samples).expect("relay metric samples json");
    for forbidden in [
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "wtxid",
        "127.0.0.1:18444",
        "peer_id",
        "permission",
        "credential",
        "cookie",
        "secret",
        "dynamic_label",
    ] {
        assert!(!serialized.contains(forbidden));
    }
}

#[test]
fn block_relay_metric_kinds_are_low_cardinality_counters() {
    // Arrange
    let block_relay_kinds = [
        MetricKind::BlockServedCount,
        MetricKind::BlockServingSuppressedCount,
        MetricKind::CompactAnnouncedCount,
        MetricKind::CompactReconstructedCount,
        MetricKind::CompactMissingTxRequestedCount,
        MetricKind::CompactFallbackCount,
        MetricKind::CompactMalformedCount,
        MetricKind::CompactTimeoutCount,
        MetricKind::CompactCleanupCount,
    ];

    // Act
    let labels = block_relay_kinds
        .into_iter()
        .map(MetricKind::as_str)
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        labels,
        vec![
            "block_served_count",
            "block_serving_suppressed_count",
            "compact_announced_count",
            "compact_reconstructed_count",
            "compact_missing_tx_requested_count",
            "compact_fallback_count",
            "compact_malformed_count",
            "compact_timeout_count",
            "compact_cleanup_count",
        ]
    );
    for label in labels {
        assert!(label.ends_with("_count"));
        for forbidden in [
            "peer_id",
            "endpoint",
            "block_hash",
            "txid",
            "permission",
            "credential",
            "cookie",
            "secret",
            "dynamic_label",
        ] {
            assert!(!label.contains(forbidden));
        }
    }
}

#[test]
fn block_relay_metric_status_maps_to_each_fixed_metric_kind() {
    // Arrange
    let timestamp = 1_777_225_205;
    let block_relay = BlockRelayEvidenceStatus::with_components(
        crate::status::BlockServingEvidenceStatus::with_activation_eligibility_and_status(
            crate::status::BlockServingActivationEvidence {
                block_serving_enabled: true,
                compact_relay_enabled: true,
            },
            crate::status::BlockServingEligibilityCounters {
                eligible_peer_count: 2,
                ineligible_peer_count: 3,
                disabled_count: 1,
                activation_required_count: 0,
                inbound_serving_required_count: 1,
                permission_required_count: 1,
                protected_not_serving_count: 0,
                status_unavailable_count: 0,
                permission_effect_inactive_count: 1,
            },
            crate::status::BlockServingStatusCounters {
                validated_count: 5,
                available_count: 4,
                stale_count: 1,
                side_chain_count: 2,
                pruned_count: 1,
                unavailable_count: 3,
                unvalidated_count: 0,
                unknown_count: 1,
                suppressed_count: 2,
            },
        ),
        crate::status::CompactRelayNegotiationCounters {
            version2_high_bandwidth_count: 3,
            version2_low_bandwidth_count: 1,
            unsupported_version_count: 1,
        },
        crate::status::CompactRelayAnnouncementCounters {
            compact_announced_count: 6,
            compact_headers_fallback_count: 2,
            compact_inventory_fallback_count: 1,
            compact_suppressed_count: 2,
        },
        crate::status::CompactRelayReconstructionCounters {
            compact_reconstructed_count: 4,
            compact_reconstruction_failed_count: 1,
            compact_malformed_count: 1,
        },
        crate::status::CompactRelayMissingTransactionCounters {
            compact_missing_tx_requested_count: 2,
            compact_missing_tx_suppressed_count: 1,
        },
        crate::status::CompactRelayFallbackCounters {
            compact_fallback_count: 2,
            compact_timeout_count: 1,
        },
        crate::status::CompactRelayInFlightCounters {
            in_flight_count: 3,
            getblocktxn_in_flight_count: 2,
            peers_with_in_flight_count: 2,
        },
        crate::status::CompactRelayCleanupCounters {
            compact_cleanup_count: 3,
            compact_download_peer_disconnect_count: 1,
            compact_download_timeout_count: 1,
            compact_download_reorg_count: 0,
            compact_download_restart_count: 0,
            compact_download_block_connected_count: 1,
        },
    );

    // Act
    let samples = block_relay_metric_samples(&block_relay, timestamp);

    // Assert
    assert_eq!(
        samples,
        vec![
            MetricSample::new(MetricKind::BlockServedCount, 2.0, timestamp),
            MetricSample::new(MetricKind::BlockServingSuppressedCount, 2.0, timestamp),
            MetricSample::new(MetricKind::CompactAnnouncedCount, 6.0, timestamp),
            MetricSample::new(MetricKind::CompactReconstructedCount, 4.0, timestamp),
            MetricSample::new(MetricKind::CompactMissingTxRequestedCount, 2.0, timestamp),
            MetricSample::new(MetricKind::CompactFallbackCount, 2.0, timestamp),
            MetricSample::new(MetricKind::CompactMalformedCount, 1.0, timestamp),
            MetricSample::new(MetricKind::CompactTimeoutCount, 1.0, timestamp),
            MetricSample::new(MetricKind::CompactCleanupCount, 3.0, timestamp),
        ]
    );
    let serialized = serde_json::to_string(&samples).expect("block relay metric samples json");
    for forbidden in [
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "127.0.0.1:18444",
        "peer_id",
        "credential",
        "cookie",
        "secret",
        "dynamic_label",
    ] {
        assert!(!serialized.contains(forbidden));
    }
}

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

fn inbound_status_fixture() -> InboundPeerServingStatus {
    InboundPeerServingStatus {
        listener_state: "ready".to_string(),
        bound_endpoints: Vec::new(),
        preflight_reason: "ready".to_string(),
        admitted_inbound_peers: 1,
        rejected_inbound_peers: 2,
        handshake: InboundHandshakeStatusCounts::default(),
        duplicate_rejects: 5,
        self_connection_rejects: 6,
        cap_rejects: 3,
        reserved_slot_rejects: 4,
        latest_admission_event: FieldAvailability::unavailable("no admission event"),
        permissioned_inbound_peers: 7,
        protected_inbound_peers: 8,
        permission_class: "ordinary_inbound".to_string(),
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        inactive_permission_effect_observations: 9,
        permission_validation_failures: 10,
        latest_permission_decision: FieldAvailability::unavailable("no permission decision"),
        local_advertisement_candidates: Vec::new(),
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: 0,
        getaddr_requests_suppressed: 0,
        learned_address_entries: 0,
        learned_address_rejections: 0,
        latest_address_decision: FieldAvailability::unavailable("no address decision"),
        eviction_candidates_evaluated: 11,
        disconnects_requested: 12,
        discouraged_peers: 0,
        active_bans: 13,
        expired_bans: 0,
        manual_unbans: 0,
        misbehavior_observations: 14,
        protected_no_actions: 15,
        latest_peer_policy_decision: FieldAvailability::unavailable("no peer policy decision"),
        resource_pressure_events: 16,
        read_queue_pressure_events: 17,
        write_queue_pressure_events: 18,
        request_cap_events: 19,
        payload_rejections: 20,
        timeout_disconnects: 21,
        churn_rejections: 22,
        reconnect_suppressions: 23,
        latest_resource_governance_decision: FieldAvailability::unavailable(
            "no resource governance decision",
        ),
    }
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
