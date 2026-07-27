// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

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
