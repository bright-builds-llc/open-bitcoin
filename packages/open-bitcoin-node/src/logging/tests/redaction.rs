// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn inbound_resource_governance_log_record_uses_allowlisted_fields() {
    // Arrange
    let event = InboundResourceGovernanceEvent {
        outcome: "rejected".to_string(),
        reason: "invalid_checksum".to_string(),
        label: "payload_rejected".to_string(),
        source: "source_envelope_gate".to_string(),
        message: "inbound_message_resource_governance".to_string(),
        next_action: "payload_rejected".to_string(),
    };

    // Act
    let record = inbound_resource_governance_log_record(&event, 1_777_225_022);

    // Assert
    assert_eq!(record.level, StructuredLogLevel::Warn);
    assert_eq!(record.source, INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE);
    assert_eq!(record.timestamp_unix_seconds, 1_777_225_022);
    assert_eq!(
        record.message,
        "outcome=rejected reason=invalid_checksum label=payload_rejected source=source_envelope_gate message=inbound_message_resource_governance next_action=payload_rejected"
    );
}

#[test]
fn inbound_resource_governance_log_record_redacts_suspicious_raw_fields() {
    // Arrange
    let event = InboundResourceGovernanceEvent {
        outcome: "rejected".to_string(),
        reason: "peer_id=42".to_string(),
        label: "raw_endpoint=192.0.2.1:8333".to_string(),
        source: "payload_bytes=00112233445566778899aabbccddeeff".to_string(),
        message: "permission_string=admin credential=fixture".to_string(),
        next_action: "secret=fixture".to_string(),
    };

    // Act
    let record = inbound_resource_governance_log_record(&event, 1_777_225_022);

    // Assert
    assert_eq!(record.source, INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE);
    assert_eq!(
        record.message,
        "outcome=rejected reason=redacted_resource_field label=redacted_resource_field source=redacted_resource_field message=redacted_resource_field next_action=redacted_resource_field"
    );
    assert!(!record.message.contains("42"));
    assert!(!record.message.contains("192.0.2.1:8333"));
    assert!(!record.message.contains("00112233445566778899aabbccddeeff"));
    assert!(!record.message.contains("admin"));
}

#[test]
fn inbound_peer_policy_log_record_uses_allowlisted_fields() {
    // Arrange
    let event = InboundPeerPolicyEvent {
        outcome: "ban_active".to_string(),
        reason: "manual_ban".to_string(),
        label: "ban_active".to_string(),
        source: "source_peer_policy_runtime_bridge".to_string(),
        message: "peer_policy_runtime_bridge".to_string(),
    };

    // Act
    let record = inbound_peer_policy_log_record(&event, 1_777_225_023);

    // Assert
    assert_eq!(record.level, StructuredLogLevel::Warn);
    assert_eq!(record.source, INBOUND_PEER_POLICY_LOG_SOURCE);
    assert_eq!(record.timestamp_unix_seconds, 1_777_225_023);
    assert_eq!(
        record.message,
        "outcome=ban_active reason=manual_ban label=ban_active source=source_peer_policy_runtime_bridge message=peer_policy_runtime_bridge"
    );
}

#[test]
fn inbound_peer_policy_log_record_redacts_suspicious_raw_fields() {
    // Arrange
    let event = InboundPeerPolicyEvent {
        outcome: "ban_active".to_string(),
        reason: "127.0.0.1:18444".to_string(),
        label: "peer_id=42".to_string(),
        source: "raw_endpoint=192.0.2.1:8333".to_string(),
        message: "permission_string=admin credential=fixture secret=fixture cookie=value payload_bytes=00112233445566778899aabbccddeeff".to_string(),
    };

    // Act
    let record = inbound_peer_policy_log_record(&event, 1_777_225_023);

    // Assert
    assert_eq!(record.source, INBOUND_PEER_POLICY_LOG_SOURCE);
    assert_eq!(
        record.message,
        "outcome=ban_active reason=redacted_peer_policy_field label=redacted_peer_policy_field source=redacted_peer_policy_field message=redacted_peer_policy_field"
    );
    for raw in [
        "127.0.0.1:18444",
        "peer_id=42",
        "192.0.2.1:8333",
        "admin",
        "credential",
        "secret",
        "cookie=value",
        "00112233445566778899aabbccddeeff",
    ] {
        assert!(!record.message.contains(raw));
    }
}

#[test]
fn relay_mempool_log_record_uses_fixed_outcome_counts() {
    // Arrange
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
    let record = relay_mempool_log_record(&relay, 1_777_225_105);

    // Assert
    assert_eq!(record.level, StructuredLogLevel::Info);
    assert_eq!(record.source, RELAY_MEMPOOL_LOG_SOURCE);
    assert_eq!(record.timestamp_unix_seconds, 1_777_225_105);
    assert_eq!(
        record.message,
        "accepted=1 rejected=2 orphaned=3 requested=4 served=5 announced=6 suppressed=7 evicted=8 expired=9 rebroadcast_deferred=10 recovered=11 dropped_confirmed=12 dropped_duplicate=13 dropped_missing_parent=14 dropped_policy_incompatible=15 dropped_evicted=16"
    );
}

#[test]
fn relay_mempool_log_record_omits_sensitive_and_dynamic_material() {
    // Arrange
    let mut relay = RelayEvidenceStatus::with_counters(RelayEvidenceCounters {
        accepted_count: 1,
        rejected_count: 0,
        orphaned_count: 0,
        requested_count: 0,
        served_count: 0,
        announced_count: 0,
        suppressed_count: 0,
        evicted_count: 0,
        expired_count: 0,
        rebroadcast_deferred_count: 0,
    });
    relay.mempool_admission = RelayEvidenceField::unavailable(
        "txid=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    );
    relay.recovery_counters =
        RelayEvidenceField::unavailable("wtxid=abcdef0123456789 endpoint=127.0.0.1:18444");
    relay.local_submission =
        RelayEvidenceField::deferred("wtxid=abcdef0123456789 endpoint=127.0.0.1:18444");
    relay.fanout =
        RelayEvidenceField::deferred("peer_id=42 permission_string=forcerelay dynamic_label=raw");
    relay.public_relay = RelayEvidenceField::intentionally_different(
        "credential=fixture cookie=value secret=fixture reject_reason=freeform",
    );

    // Act
    let record = relay_mempool_log_record(&relay, 1_777_225_106);

    // Assert
    assert_eq!(record.source, RELAY_MEMPOOL_LOG_SOURCE);
    assert_eq!(
        record.message,
        "accepted=1 rejected=0 orphaned=0 requested=0 served=0 announced=0 suppressed=0 evicted=0 expired=0 rebroadcast_deferred=0 recovered=0 dropped_confirmed=0 dropped_duplicate=0 dropped_missing_parent=0 dropped_policy_incompatible=0 dropped_evicted=0"
    );
    for raw in [
        "0123456789abcdef",
        "wtxid",
        "127.0.0.1:18444",
        "peer_id",
        "permission_string",
        "credential",
        "cookie",
        "secret",
        "dynamic_label",
        "reject_reason",
    ] {
        assert!(!record.message.contains(raw), "leaked {raw}");
    }
}

#[test]
fn phase123_block_served_log_uses_runtime_count() {
    // Arrange
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
    let served_count = 9_u64;

    // Act
    let record = block_relay_log_record(&block_relay, served_count, 1_777_225_305);

    // Assert
    assert_eq!(record.level, StructuredLogLevel::Info);
    assert_eq!(record.source, BLOCK_RELAY_LOG_SOURCE);
    assert_eq!(record.timestamp_unix_seconds, 1_777_225_305);
    for expected in [
        "outcome=projected",
        "cause=status_projection",
        "label=block_relay",
        "serve_label=block_serving_eligible",
        "suppress_label=block_serving_suppressed",
        "announcement_label=compact_announced",
        "reconstruction_label=compact_reconstruction_failed",
        "timeout_label=compact_download_timeout",
        "cleanup_label=compact_download_peer_disconnect",
        "block_served_count=9",
        "compact_cleanup_count=3",
    ] {
        assert!(record.message.contains(expected), "missing {expected}");
    }
}

#[test]
fn block_relay_log_record_omits_sensitive_and_dynamic_material() {
    // Arrange
    let block_relay = BlockRelayEvidenceStatus::default_unavailable();

    // Act
    let record = block_relay_log_record(&block_relay, 0, 1_777_225_306);

    // Assert
    assert_eq!(record.source, BLOCK_RELAY_LOG_SOURCE);
    for raw in [
        "0123456789abcdef",
        "127.0.0.1:18444",
        "peer_id",
        "permission_string",
        "credential",
        "cookie",
        "secret",
        "dynamic_label",
    ] {
        assert!(!record.message.contains(raw), "leaked {raw}");
    }
}
