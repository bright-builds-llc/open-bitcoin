// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

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
        dropped_expired_count: 16,
        dropped_evicted_count: 17,
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
                MetricKind::RelayRecoveryDroppedExpiredCount,
                16.0,
                timestamp,
            ),
            MetricSample::new(
                MetricKind::RelayRecoveryDroppedEvictedCount,
                17.0,
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
fn phase123_block_served_metric_uses_runtime_count() {
    // Arrange
    let timestamp = 1_777_225_205;
    let served_count = 9_u64;
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
    let samples = block_relay_metric_samples(&block_relay, served_count, timestamp);

    // Assert
    assert_eq!(
        samples,
        vec![
            MetricSample::new(MetricKind::BlockServedCount, 9.0, timestamp),
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

fn available_durability_fixture() -> ChainstateDurabilityEvidence {
    ChainstateDurabilityEvidence {
        cache_size: CacheSizeLabel::Critical,
        last_flush_reason: LastFlushReasonLabel::Periodic,
        write_kind: WriteKindLabel::Sync,
        readiness: ReadinessLabel::ReadyToFlush,
        cache_bytes: 9_000,
        cache_byte_limit: 8_192,
        recovery_outcome: CoinsRecoveryOutcome::Replayed,
        maybe_coins_best_block_height: Some(840_004),
        maybe_coins_best_block_hash: Some(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
        ),
        last_serving_status: ServingStatusLabel::Unavailable,
        last_payload_present: false,
        last_index_known: true,
        last_validated_on_active_chain: true,
        available_count: 3,
        unavailable_count: 1,
        index_known_without_payload_count: 1,
    }
}

#[test]
fn chainstate_durability_metric_kinds_are_low_cardinality_gauges_and_counters() {
    // Arrange
    let durability = FieldAvailability::available(available_durability_fixture());
    let timestamp = 1_779_000_001;

    // Act
    let samples = chainstate_durability_metric_samples(&durability, timestamp);
    let kinds = samples.iter().map(|sample| sample.kind).collect::<Vec<_>>();
    let labels = kinds
        .iter()
        .copied()
        .map(MetricKind::as_str)
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        kinds,
        vec![
            MetricKind::ChainstateDurabilityCacheSizeClass,
            MetricKind::ChainstateDurabilityLastFlushReasonClass,
            MetricKind::ChainstateDurabilityWriteKindClass,
            MetricKind::ChainstateDurabilityRecoveryClass,
            MetricKind::ChainstateDurabilityAvailableCount,
            MetricKind::ChainstateDurabilityUnavailableCount,
            MetricKind::ChainstateDurabilityIndexKnownWithoutPayloadCount,
        ]
    );
    for label in labels {
        assert!(label.ends_with("_class") || label.ends_with("_count"));
        assert!(!label.contains("dynamic_label"));
        assert!(!label.contains("peer_id"));
    }
}

#[test]
fn chainstate_durability_metric_status_maps_to_each_fixed_metric_kind() {
    // Arrange
    let durability = FieldAvailability::available(available_durability_fixture());
    let timestamp = 1_779_000_002;

    // Act
    let samples = chainstate_durability_metric_samples(&durability, timestamp);

    // Assert
    assert_eq!(
        samples,
        vec![
            MetricSample::new(
                MetricKind::ChainstateDurabilityCacheSizeClass,
                2.0,
                timestamp
            ),
            MetricSample::new(
                MetricKind::ChainstateDurabilityLastFlushReasonClass,
                2.0,
                timestamp,
            ),
            MetricSample::new(
                MetricKind::ChainstateDurabilityWriteKindClass,
                2.0,
                timestamp
            ),
            MetricSample::new(
                MetricKind::ChainstateDurabilityRecoveryClass,
                1.0,
                timestamp
            ),
            MetricSample::new(
                MetricKind::ChainstateDurabilityAvailableCount,
                3.0,
                timestamp
            ),
            MetricSample::new(
                MetricKind::ChainstateDurabilityUnavailableCount,
                1.0,
                timestamp,
            ),
            MetricSample::new(
                MetricKind::ChainstateDurabilityIndexKnownWithoutPayloadCount,
                1.0,
                timestamp,
            ),
        ]
    );
}

#[test]
fn chainstate_durability_metric_unavailable_emits_no_samples() {
    // Arrange
    let durability = FieldAvailability::<ChainstateDurabilityEvidence>::unavailable(
        "chainstate durability evidence unavailable",
    );

    // Act
    let samples = chainstate_durability_metric_samples(&durability, 1_779_000_003);

    // Assert
    assert!(samples.is_empty());
}

#[test]
fn chainstate_durability_metric_samples_omit_hash_peer_and_pruned() {
    // Arrange
    let durability = FieldAvailability::available(available_durability_fixture());

    // Act
    let samples = chainstate_durability_metric_samples(&durability, 1_779_000_004);
    let debug_text = format!("{samples:?}");
    let serialized = serde_json::to_string(&samples).expect("durability metric samples json");

    // Assert
    for text in [debug_text.as_str(), serialized.as_str()] {
        assert!(!text.contains("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
        for forbidden in ["peer_id", "pruned", "block_hash", "dynamic_label"] {
            assert!(!text.contains(forbidden), "leaked {forbidden} in {text}");
        }
    }
}

#[test]
fn metric_kind_all_len_is_81() {
    // Arrange / Act / Assert
    assert_eq!(MetricKind::ALL.len(), 81);
}
