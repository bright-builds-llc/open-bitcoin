use super::*;

#[test]
fn dashboard_projection_includes_required_sections_and_charts() {
    // Arrange
    let snapshot = test_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let titles = state
        .sections
        .iter()
        .map(|section| section.title.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        titles,
        vec![
            "Node",
            "Sync and Peers",
            "Mempool and Wallet",
            "Service",
            "Logs and Health"
        ]
    );
    assert_eq!(state.charts.len(), DASHBOARD_METRIC_KINDS.len());
    assert!(state.actions.iter().any(|action| action.destructive));
    let wallet_rows = &state.sections[2].rows;
    assert_eq!(
        wallet_rows
            .iter()
            .find(|row| row.label == "Freshness")
            .expect("freshness row")
            .value,
        "fresh"
    );
}

#[test]
fn dashboard_metric_labels_cover_all_metric_kinds() {
    // Arrange
    let kinds = MetricKind::ALL;

    // Act
    let labels = kinds.into_iter().map(metric_label).collect::<Vec<_>>();

    // Assert
    assert_eq!(labels.len(), MetricKind::ALL.len());
    assert!(labels.contains(&"Inbound permissioned admits"));
    assert!(labels.contains(&"Inbound protected admits"));
    assert!(labels.contains(&"Inbound inactive permission effects"));
    assert!(labels.contains(&"Inbound permission validation failures"));
    assert!(labels.contains(&"Inbound payload rejects"));
    assert!(labels.contains(&"Inbound reconnect suppressions"));
    assert!(labels.contains(&"Relay accepted"));
    assert!(labels.contains(&"Relay rebroadcast deferred"));
    assert!(labels.contains(&"Relay recovery recovered"));
}

#[test]
fn dashboard_charts_render_retained_inbound_metric_samples_without_expanding_row() {
    // Arrange
    let mut snapshot = test_snapshot();
    snapshot.metrics = MetricsStatus::available_with_samples(
        MetricRetentionPolicy::default(),
        vec![
            MetricSample::new(MetricKind::SyncHeight, 100.0, 10),
            MetricSample::new(MetricKind::InboundAdmittedPeerCount, 1.0, 10),
            MetricSample::new(MetricKind::InboundResourcePressureActiveCount, 16.0, 10),
            MetricSample::new(MetricKind::InboundReconnectSuppressedCount, 23.0, 10),
        ],
    );

    // Act
    let state = DashboardState::from_snapshot(&snapshot);
    let charts = state
        .charts
        .iter()
        .map(|chart| (chart.title.as_str(), chart.points.clone()))
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(state.charts.len(), MAX_DASHBOARD_CHARTS);
    assert!(state.charts.len() <= DASHBOARD_METRIC_KINDS.len());
    assert!(charts.contains(&("Inbound admits", vec![1])));
    assert!(charts.contains(&("Inbound resource pressure", vec![16])));
    assert!(charts.contains(&("Inbound reconnect suppressions", vec![23])));
}

#[test]
fn dashboard_sections_surface_relay_evidence_rows() {
    // Arrange
    let mut snapshot = test_snapshot();
    snapshot.mempool.relay = RelayEvidenceStatus::with_counters(RelayEvidenceCounters {
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
    snapshot.mempool.relay.recovery_counters =
        open_bitcoin_node::status::relay_evidence::RelayEvidenceField::implemented(
            RelayRecoveryCounters {
                recovered_count: 1,
                dropped_confirmed_count: 2,
                dropped_duplicate_count: 3,
                dropped_missing_parent_count: 4,
                dropped_policy_incompatible_count: 5,
                dropped_evicted_count: 6,
            },
        );

    // Act
    let state = DashboardState::from_snapshot(&snapshot);
    let rows = &state.sections[2].rows;

    // Assert
    assert_eq!(
        rows.iter()
            .find(|row| row.label == "Relay evidence")
            .expect("relay evidence row")
            .value,
        "accepted_count=1 rejected_count=2 orphaned_count=3 requested_count=4 served_count=5 announced_count=6 suppressed_count=7 evicted_count=8 expired_count=9 rebroadcast_deferred_count=10"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.label == "Relay recovery")
            .expect("relay recovery row")
            .value,
        "recovered_count=1 dropped_confirmed_count=2 dropped_duplicate_count=3 dropped_missing_parent_count=4 dropped_policy_incompatible_count=5 dropped_evicted_count=6"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.label == "Mempool evidence")
            .expect("mempool evidence row")
            .value,
        "Unavailable: mempool admission evidence unavailable"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.label == "Rebroadcast: deferred")
            .expect("rebroadcast row")
            .value,
        "Deferred: rebroadcast relay evidence not projected"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.label == "Public relay")
            .expect("public relay row")
            .value,
        "Intentionally different: public relay readiness is intentionally not claimed"
    );
}

#[test]
fn dashboard_charts_render_retained_relay_metric_samples_without_dynamic_labels() {
    // Arrange
    let mut snapshot = test_snapshot();
    snapshot.metrics = MetricsStatus::available_with_samples(
        MetricRetentionPolicy::default(),
        vec![
            MetricSample::new(MetricKind::RelayAcceptedCount, 1.0, 10),
            MetricSample::new(MetricKind::RelayRejectedCount, 2.0, 10),
            MetricSample::new(MetricKind::RelayRebroadcastDeferredCount, 10.0, 10),
        ],
    );

    // Act
    let state = DashboardState::from_snapshot(&snapshot);
    let charts = state
        .charts
        .iter()
        .map(|chart| (chart.title.as_str(), chart.points.clone()))
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(state.charts.len(), MAX_DASHBOARD_CHARTS);
    assert!(charts.contains(&("Relay accepted", vec![1])));
    assert!(charts.contains(&("Relay rejected", vec![2])));
    assert!(charts.contains(&("Relay rebroadcast deferred", vec![10])));
    let serialized = format!("{charts:?}");
    for forbidden in [
        "peer_id",
        "endpoint",
        "txid",
        "wtxid",
        "permission",
        "credential",
        "cookie",
        "secret",
        "dynamic_label",
    ] {
        assert!(!serialized.contains(forbidden), "leaked {forbidden}");
    }
}

#[test]
fn dashboard_model_block_relay_rows_surface_shared_status_contract() {
    // Arrange
    let mut snapshot = test_snapshot();
    snapshot.block_relay = BlockRelayEvidenceStatus::with_components(
        open_bitcoin_node::status::BlockServingEvidenceStatus::with_activation_eligibility_and_status(
            BlockServingActivationEvidence {
                block_serving_enabled: true,
                compact_relay_enabled: true,
            },
            BlockServingEligibilityCounters {
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
            BlockServingStatusCounters {
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
        CompactRelayNegotiationCounters {
            version2_high_bandwidth_count: 3,
            version2_low_bandwidth_count: 1,
            unsupported_version_count: 1,
        },
        CompactRelayAnnouncementCounters {
            compact_announced_count: 6,
            compact_headers_fallback_count: 2,
            compact_inventory_fallback_count: 1,
            compact_suppressed_count: 2,
        },
        CompactRelayReconstructionCounters {
            compact_reconstructed_count: 4,
            compact_reconstruction_failed_count: 1,
            compact_malformed_count: 1,
        },
        CompactRelayMissingTransactionCounters {
            compact_missing_tx_requested_count: 2,
            compact_missing_tx_suppressed_count: 1,
        },
        CompactRelayFallbackCounters {
            compact_fallback_count: 2,
            compact_timeout_count: 1,
        },
        CompactRelayInFlightCounters {
            in_flight_count: 3,
            getblocktxn_in_flight_count: 2,
            peers_with_in_flight_count: 2,
        },
        CompactRelayCleanupCounters {
            compact_cleanup_count: 3,
            compact_download_peer_disconnect_count: 1,
            compact_download_timeout_count: 1,
            compact_download_reorg_count: 0,
            compact_download_restart_count: 0,
            compact_download_block_connected_count: 1,
        },
    );

    // Act
    let first_state = DashboardState::from_snapshot(&snapshot);
    let second_state = DashboardState::from_snapshot(&snapshot);
    let first_rows = &first_state.sections[2].rows;
    let second_rows = &second_state.sections[2].rows;
    let block_relay_start = first_rows
        .iter()
        .position(|row| row.label == "Block relay activation")
        .expect("block relay rows");
    let rendered_rows = first_rows[block_relay_start..]
        .iter()
        .map(|row| (row.label.as_str(), row.value.as_str()))
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        rendered_rows,
        vec![
            (
                "Block relay activation",
                "block_serving_enabled=true compact_relay_enabled=true",
            ),
            (
                "Block relay eligibility",
                "eligible_peer_count=2 ineligible_peer_count=3 disabled_count=1 activation_required_count=0 inbound_serving_required_count=1 permission_required_count=1 protected_not_serving_count=0 status_unavailable_count=0 permission_effect_inactive_count=1",
            ),
            (
                "Block relay status",
                "validated_count=5 available_count=4 stale_count=1 side_chain_count=2 pruned_count=1 unavailable_count=3 unvalidated_count=0 unknown_count=1 suppressed_count=2",
            ),
            (
                "Compact negotiation",
                "version2_high_bandwidth_count=3 version2_low_bandwidth_count=1 unsupported_version_count=1",
            ),
            (
                "Compact announcement",
                "compact_announced_count=6 compact_headers_fallback_count=2 compact_inventory_fallback_count=1 compact_suppressed_count=2",
            ),
            (
                "Compact reconstruction",
                "compact_reconstructed_count=4 compact_reconstruction_failed_count=1 compact_malformed_count=1",
            ),
            (
                "Compact missing tx",
                "compact_missing_tx_requested_count=2 compact_missing_tx_suppressed_count=1",
            ),
            (
                "Compact fallback",
                "compact_fallback_count=2 compact_timeout_count=1",
            ),
            (
                "Compact in-flight",
                "in_flight_count=3 getblocktxn_in_flight_count=2 peers_with_in_flight_count=2",
            ),
            (
                "Compact cleanup",
                "compact_cleanup_count=3 compact_download_peer_disconnect_count=1 compact_download_timeout_count=1 compact_download_reorg_count=0 compact_download_restart_count=0 compact_download_block_connected_count=1",
            ),
        ]
    );
    assert_eq!(first_rows, second_rows);
}

#[test]
fn dashboard_model_block_relay_rows_preserve_unavailable_reason_without_sensitive_text() {
    // Arrange
    let mut snapshot = test_snapshot();
    let reason = "block relay authority unavailable";
    snapshot.block_relay.block_serving.activation = FieldAvailability::unavailable(reason);
    snapshot.block_relay.block_serving.eligibility = FieldAvailability::unavailable(reason);
    snapshot.block_relay.block_serving.status = FieldAvailability::unavailable(reason);
    snapshot.block_relay.negotiation = FieldAvailability::unavailable(reason);
    snapshot.block_relay.announcement = FieldAvailability::unavailable(reason);
    snapshot.block_relay.reconstruction = FieldAvailability::unavailable(reason);
    snapshot.block_relay.missing_transaction = FieldAvailability::unavailable(reason);
    snapshot.block_relay.fallback = FieldAvailability::unavailable(reason);
    snapshot.block_relay.in_flight = FieldAvailability::unavailable(reason);
    snapshot.block_relay.cleanup = FieldAvailability::unavailable(reason);

    // Act
    let state = DashboardState::from_snapshot(&snapshot);
    let rows = &state.sections[2].rows;
    let block_relay_start = rows
        .iter()
        .position(|row| row.label == "Block relay activation")
        .expect("block relay rows");
    let rendered_rows = rows[block_relay_start..]
        .iter()
        .map(|row| (row.label.as_str(), row.value.as_str()))
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        rendered_rows,
        [
            "Block relay activation",
            "Block relay eligibility",
            "Block relay status",
            "Compact negotiation",
            "Compact announcement",
            "Compact reconstruction",
            "Compact missing tx",
            "Compact fallback",
            "Compact in-flight",
            "Compact cleanup",
        ]
        .map(|label| (label, "Unavailable: block relay authority unavailable"))
    );
}

#[test]
fn dashboard_action_bar_includes_start_stop_restart_service_actions() {
    // Arrange
    let snapshot = test_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let actions = state
        .actions
        .iter()
        .map(|action| {
            (
                action.key.as_str(),
                action.label.as_str(),
                action.destructive,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        actions,
        vec![
            ("r", "refresh", false),
            ("s", "service status", false),
            ("t", "start service", true),
            ("o", "stop service", true),
            ("x", "restart service", true),
            ("i", "install service", true),
            ("u", "uninstall service", true),
            ("e", "enable service", true),
            ("d", "disable service", true),
            ("q", "quit", false),
        ]
    );
}

#[test]
fn derive_metric_points_is_width_bounded() {
    // Arrange
    let samples = vec![
        MetricSample::new(MetricKind::SyncHeight, 1.0, 1),
        MetricSample::new(MetricKind::SyncHeight, 2.0, 2),
        MetricSample::new(MetricKind::SyncHeight, 3.0, 3),
    ];

    // Act
    let points = derive_metric_points(&samples, 2);

    // Assert
    assert_eq!(points, vec![2, 3]);
}
