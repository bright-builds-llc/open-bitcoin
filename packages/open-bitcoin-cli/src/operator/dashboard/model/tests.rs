// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{
    MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus, RecoveryActionClass,
    RecoveryCause, RecoveryEvidenceBasis, RecoveryEvidenceSnapshot,
    status::{
        BestKnownTipSource, BestKnownTipStatus, BlockRelayEvidenceStatus,
        BlockServingActivationEvidence, BlockServingEligibilityCounters,
        BlockServingStatusCounters, BuildProvenance, CompactRelayAnnouncementCounters,
        CompactRelayCleanupCounters, CompactRelayFallbackCounters, CompactRelayInFlightCounters,
        CompactRelayMissingTransactionCounters, CompactRelayNegotiationCounters,
        CompactRelayReconstructionCounters, ConfigStatus, FieldAvailability, HealthSignal,
        HealthSignalLevel, MempoolStatus, NoProgressDiagnosis, NoProgressThresholdEvidence,
        NoProgressThresholdState, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot,
        PeerContributionEvidence, PeerContributionKind, PeerCounts, PeerStatus, PeerTipAgreement,
        PeerTipAgreementStatus, ProgressCreditEvidence, ProgressCreditKind, ProgressWindowEvidence,
        RejectedProgressActivity, RejectedProgressActivityKind, ServiceLifecycleStatus,
        ServicePriorShutdownStatus, ServiceRestartResumeStatus, ServiceResumeProgressStatus,
        ServiceStaleInflightStatus, ServiceStatus, StallDiagnosisConfidence,
        StallDiagnosisEvidence, StalledSubsystem, StayCurrentStatus, SyncAttemptCounters,
        SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState, SyncProgress, SyncProgressSignal,
        SyncReconcileProgressStatus, SyncRecoveryCategory, SyncReorgEvidence, SyncResourcePressure,
        SyncStatus, SyncStopReasonStatus, TipFreshnessStatus, WalletFreshness, WalletStatus,
        inbound_status_unavailable,
        relay_evidence::{RelayEvidenceCounters, RelayEvidenceStatus, RelayRecoveryCounters},
    },
};

use super::{
    DASHBOARD_METRIC_KINDS, DashboardState, MAX_DASHBOARD_CHARTS, derive_metric_points,
    metric_label,
};

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

#[test]
fn dashboard_sections_surface_sync_progress_and_peer_counts() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    let sync_labels = sync_rows
        .iter()
        .map(|row| row.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        sync_labels,
        vec![
            "State",
            "Phase",
            "Configured targets",
            "Attempt counters",
            "Signal",
            "Best-known tip",
            "Stay-current",
            "Stay-current action",
            "No-progress diagnosis",
            "No-progress action",
            "Progress credit",
            "Expected progress window",
            "No-progress threshold",
            "Last useful work",
            "Last peer contribution",
            "Stalled subsystem",
            "Last progress",
            "Latest stop reason",
            "Last error",
            "Recovery category",
            "Recovery",
            "Recovery evidence",
            "Pressure",
            "Resource bounds",
            "Latest reorg",
            "Reconcile",
            "Peers",
            "Progress",
        ]
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Configured targets")
            .expect("configured targets row")
            .value,
        "outbound_peers=4 target_header_height=840200"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Attempt counters")
            .expect("attempt counters row")
            .value,
        "attempted_peers=3 connected_peers=2 failed_peers=1 max_sync_rounds=8"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Latest stop reason")
            .expect("latest stop reason row")
            .value,
        "target_header_reached"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Progress")
            .expect("progress row")
            .value,
        "99.99% headers=840100 downloaded_blocks=840006 connected_blocks=840004 validated_active_chain_height=840004 validated_active_chain_hash=1111111111111111111111111111111111111111111111111111111111111111 validated_active_chain_work=840005"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Signal")
            .expect("signal row")
            .value,
        "awaiting_blocks"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Peers")
            .expect("peers row")
            .value,
        "inbound=0 outbound=2"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Recovery category")
            .expect("recovery category row")
            .value,
        "invalid_peer_data"
    );
    assert!(
        sync_rows
            .iter()
            .find(|row| row.label == "Pressure")
            .expect("pressure row")
            .value
            .contains("peers 2/4")
    );
}

#[test]
fn dashboard_sections_surface_sync_progress_and_peer_counts_unavailable_fields() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.sync.configured_targets =
        FieldAvailability::unavailable("operator target unavailable");
    snapshot.sync.attempt_counters = FieldAvailability::unavailable("attempt counters unavailable");
    snapshot.sync.latest_stop_reason = FieldAvailability::unavailable("stop reason unavailable");

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Configured targets")
            .expect("configured targets row")
            .value,
        "Unavailable: operator target unavailable"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Attempt counters")
            .expect("attempt counters row")
            .value,
        "Unavailable: attempt counters unavailable"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Latest stop reason")
            .expect("latest stop reason row")
            .value,
        "Unavailable: stop reason unavailable"
    );
}

#[test]
fn dashboard_sections_surface_phase78_progress_guarantee_fields() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    apply_phase78_available_sync_fields(&mut snapshot.sync);

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    for (label, expected) in [
        (
            "Progress credit",
            "kind=validated_durable_active_chain height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005 source_unix_seconds=1717000020 rejected_activity_count=1",
        ),
        (
            "Expected progress window",
            "expected_progress_window_seconds=300 retry_backoff_seconds=30 max_sync_rounds=8 tip_freshness_threshold_seconds=600",
        ),
        (
            "No-progress threshold",
            "state=within_window threshold_seconds=300 elapsed_since_last_useful_work_seconds=12 evaluated_at_unix_seconds=1717000032",
        ),
        (
            "Last useful work",
            "kind=current_at_best_known_tip height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005 source_unix_seconds=1717000025 rejected_activity_count=0",
        ),
        (
            "Last peer contribution",
            "peer=peer-1 endpoint=203.0.113.10:8333 kind=headers_and_blocks messages=7 headers=3 blocks=1 last_activity_unix_seconds=1717000028 failure=Unavailable: no peer failure recorded",
        ),
        (
            "Stalled subsystem",
            "stalled_subsystem=at_tip_waiting confidence=high basis=stay_current,current_tip next_action=No operator action required. no_progress_diagnosis=current_at_best_known_tip recovery_category=Unavailable: no recovery category latest_stop_reason=best_known_tip_reached",
        ),
    ] {
        assert_eq!(
            sync_rows
                .iter()
                .find(|row| row.label == label)
                .expect("phase78 row")
                .value,
            expected
        );
    }
}

#[test]
fn dashboard_sections_surface_phase78_unavailable_reasons() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    for (label, expected) in [
        (
            "Progress credit",
            "Unavailable: progress credit evidence unavailable",
        ),
        (
            "Expected progress window",
            "Unavailable: expected progress window unavailable",
        ),
        (
            "No-progress threshold",
            "Unavailable: no-progress threshold evidence unavailable",
        ),
        (
            "Last useful work",
            "Unavailable: last useful work unavailable",
        ),
        (
            "Last peer contribution",
            "Unavailable: last peer contribution unavailable",
        ),
        (
            "Stalled subsystem",
            "Unavailable: stall diagnosis unavailable",
        ),
    ] {
        assert_eq!(
            sync_rows
                .iter()
                .find(|row| row.label == label)
                .expect("phase78 row")
                .value,
            expected
        );
    }
}

#[test]
fn phase72_dashboard_projects_full_sync_truth_contract() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.sync.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height: 840_004,
        block_hash: "11".repeat(32),
        work: "840005".to_string(),
        block_time_unix_seconds: 1_717_000_010,
        observed_at_unix_seconds: 1_717_000_020,
        freshness: TipFreshnessStatus::Fresh,
        peer_agreement: vec![PeerTipAgreement {
            peer: "peer-1".to_string(),
            maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
            status: PeerTipAgreementStatus::Agrees,
            maybe_height: Some(840_004),
            maybe_hash: Some("11".repeat(32)),
            maybe_work: Some("840005".to_string()),
            maybe_last_activity_unix_seconds: Some(1_717_000_020),
        }],
    });
    snapshot.sync.stay_current =
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
    snapshot.sync.stay_current_next_action =
        FieldAvailability::available("Continue monitoring best-known tip freshness.".to_string());
    snapshot.sync.no_progress_diagnosis =
        FieldAvailability::available(NoProgressDiagnosis::CurrentAtBestKnownTip);
    snapshot.sync.no_progress_next_action =
        FieldAvailability::available("No operator action required.".to_string());
    snapshot.sync.latest_reorg = FieldAvailability::available(SyncReorgEvidence {
        common_ancestor_height: 840_000,
        common_ancestor_hash: "00".repeat(32),
        disconnected_count: 2,
        connected_count: 4,
        final_active_height: 840_004,
        final_active_hash: "11".repeat(32),
        fully_persisted: true,
    });
    snapshot.sync.reconcile_progress =
        FieldAvailability::available(SyncReconcileProgressStatus::ExtendedActiveChain {
            connected_count: 4,
            final_active_height: 840_004,
            final_active_hash: "11".repeat(32),
        });

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    let labels = sync_rows
        .iter()
        .map(|row| row.label.as_str())
        .collect::<Vec<_>>();
    for label in [
        "Best-known tip",
        "Stay-current",
        "Stay-current action",
        "No-progress diagnosis",
        "No-progress action",
        "Latest reorg",
        "Reconcile",
        "Pressure",
        "Progress",
    ] {
        assert!(labels.contains(&label), "missing row {label}");
    }
    let progress = sync_rows
        .iter()
        .find(|row| row.label == "Progress")
        .expect("progress row");
    for expected in [
        "validated_active_chain_height=840004",
        "validated_active_chain_hash=1111111111111111111111111111111111111111111111111111111111111111",
        "validated_active_chain_work=840005",
    ] {
        assert!(progress.value.contains(expected), "missing {expected}");
    }

    // Arrange
    snapshot.sync.best_known_tip =
        FieldAvailability::unavailable("best-known tip evidence unavailable");
    snapshot.sync.stay_current = FieldAvailability::unavailable("stay-current state unavailable");
    snapshot.sync.latest_reorg = FieldAvailability::unavailable("no reorg evidence recorded");
    snapshot.sync.reconcile_progress =
        FieldAvailability::unavailable("reconcile progress unavailable");

    // Act
    let unavailable_state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let unavailable_rows = &unavailable_state.sections[1].rows;
    for (label, expected) in [
        (
            "Best-known tip",
            "Unavailable: best-known tip evidence unavailable",
        ),
        (
            "Stay-current",
            "Unavailable: stay-current state unavailable",
        ),
        ("Latest reorg", "Unavailable: no reorg evidence recorded"),
        ("Reconcile", "Unavailable: reconcile progress unavailable"),
    ] {
        assert_eq!(
            unavailable_rows
                .iter()
                .find(|row| row.label == label)
                .expect("phase72 row")
                .value,
            expected
        );
    }
}

#[test]
fn dashboard_service_restart_resume_rows_surface_phase64_evidence() {
    // Arrange
    let snapshot = test_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let service_rows = &state.sections[3].rows;
    let labels = service_rows
        .iter()
        .map(|row| row.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        labels,
        vec![
            "Lifecycle",
            "Manager",
            "Installed",
            "Enabled",
            "Running",
            "Service file",
            "Logs",
            "Diagnostics",
            "Restart/resume",
            "Prior shutdown",
            "Resume progress",
            "Stale in-flight",
            "Resume action",
        ]
    );
    assert_eq!(service_rows[0].value, "running");
    assert_eq!(service_rows[1].value, "launchd");
    assert_eq!(service_rows[5].value, "/tmp/open-bitcoin-node.service");
    assert_eq!(service_rows[6].value, "/tmp/logs/open-bitcoin.log");
    assert_eq!(
        service_rows[7].value,
        "Unavailable: service diagnostics unavailable"
    );
    assert_eq!(
        service_rows[8].value,
        "datadir=/tmp/open-bitcoin same_datadir=true recovery_category=clean_shutdown"
    );
    assert_eq!(service_rows[9].value, "clean");
    assert_eq!(service_rows[10].value, "downloaded=840006 connected=840004");
    assert_eq!(service_rows[11].value, "cleared");
    assert_eq!(
        service_rows[12].value,
        "Resume service sync review from preserved durable progress."
    );

    let mut unavailable = test_snapshot();
    unavailable.service = ServiceStatus {
        manager: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        lifecycle: FieldAvailability::available(ServiceLifecycleStatus::UnavailableManager),
        installed: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        enabled: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        running: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        service_file_path: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        log_path: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        diagnostics: FieldAvailability::available(
            "unsupported platform: manager unavailable".to_string(),
        ),
        restart_resume: FieldAvailability::unavailable(
            "service restart/resume evidence unavailable",
        ),
    };

    let unavailable_state = DashboardState::from_snapshot(&unavailable);
    let unavailable_rows = &unavailable_state.sections[3].rows;
    assert_eq!(unavailable_rows[0].value, "unavailable-manager");
    assert_eq!(
        unavailable_rows[1].value,
        "Unavailable: service manager unavailable: unsupported platform: manager unavailable"
    );
    assert_eq!(
        unavailable_rows[7].value,
        "unsupported platform: manager unavailable"
    );
    assert_eq!(
        unavailable_rows[8].value,
        "Unavailable: service restart/resume evidence unavailable"
    );
}

#[test]
fn dashboard_recovery_evidence_available_row_uses_top_level_status_evidence() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.recovery_evidence = FieldAvailability::available(phase77_recovery_evidence());
    snapshot.sync.recovery_action =
        FieldAvailability::available("legacy action cause=legacy action_class=legacy".to_string());

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    let recovery_evidence = sync_rows
        .iter()
        .find(|row| row.label == "Recovery evidence")
        .expect("recovery evidence row");
    assert_eq!(
        recovery_evidence.value,
        "category=storage_lock_contention cause=stale_lock_evidence action_class=read_only_inspection next_action=Inspect the datadir read-only and avoid deleting lock artifacts automatically."
    );
}

#[test]
fn dashboard_recovery_evidence_unavailable_row_preserves_reason() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.recovery_evidence = FieldAvailability::unavailable("recovery evidence unavailable");

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    let recovery_evidence = sync_rows
        .iter()
        .find(|row| row.label == "Recovery evidence")
        .expect("recovery evidence row");
    assert_eq!(
        recovery_evidence.value,
        "Unavailable: recovery evidence unavailable"
    );
}

fn apply_phase78_available_sync_fields(sync: &mut SyncStatus) {
    sync.progress_credit = FieldAvailability::available(ProgressCreditEvidence {
        kind: ProgressCreditKind::ValidatedDurableActiveChain,
        credited_validated_active_chain_height: 840_004,
        credited_validated_active_chain_hash: "11".repeat(32),
        credited_validated_active_chain_work: "840005".to_string(),
        source_unix_seconds: 1_717_000_020,
        rejected_activity: vec![RejectedProgressActivity {
            kind: RejectedProgressActivityKind::HeaderDownload,
            observed_count: 3,
            reason: "headers do not prove durable active-chain progress".to_string(),
        }],
    });
    sync.expected_progress_window = FieldAvailability::available(ProgressWindowEvidence {
        retry_backoff_seconds: 30,
        max_sync_rounds: 8,
        expected_progress_window_seconds: 300,
        tip_freshness_threshold_seconds: 600,
    });
    sync.no_progress_threshold = FieldAvailability::available(NoProgressThresholdEvidence {
        threshold_seconds: 300,
        elapsed_since_last_useful_work_seconds: 12,
        state: NoProgressThresholdState::WithinWindow,
        evaluated_at_unix_seconds: 1_717_000_032,
    });
    sync.last_useful_work = FieldAvailability::available(ProgressCreditEvidence {
        kind: ProgressCreditKind::CurrentAtBestKnownTip,
        credited_validated_active_chain_height: 840_004,
        credited_validated_active_chain_hash: "11".repeat(32),
        credited_validated_active_chain_work: "840005".to_string(),
        source_unix_seconds: 1_717_000_025,
        rejected_activity: Vec::new(),
    });
    sync.last_peer_contribution = FieldAvailability::available(PeerContributionEvidence {
        peer: "peer-1".to_string(),
        maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
        kind: PeerContributionKind::HeadersAndBlocks,
        messages_processed: 7,
        headers_received: 3,
        blocks_received: 1,
        maybe_last_activity_unix_seconds: Some(1_717_000_028),
        maybe_failure_reason_label: None,
    });
    sync.stall_diagnosis = FieldAvailability::available(StallDiagnosisEvidence {
        stalled_subsystem: StalledSubsystem::AtTipWaiting,
        confidence: StallDiagnosisConfidence::High,
        evidence_basis: vec!["stay_current".to_string(), "current_tip".to_string()],
        next_action: "No operator action required.".to_string(),
        maybe_no_progress_diagnosis: Some(NoProgressDiagnosis::CurrentAtBestKnownTip),
        maybe_recovery_category: None,
        maybe_latest_stop_reason_label: Some("best_known_tip_reached".to_string()),
        source_unix_seconds: 1_717_000_032,
    });
}

fn test_snapshot() -> OpenBitcoinStatusSnapshot {
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec!["/tmp/open-bitcoin/bitcoin.conf".to_string()],
        },
        service: ServiceStatus {
            manager: FieldAvailability::available("launchd".to_string()),
            lifecycle: FieldAvailability::available(ServiceLifecycleStatus::Running),
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::available(true),
            running: FieldAvailability::available(true),
            service_file_path: FieldAvailability::available(
                "/tmp/open-bitcoin-node.service".to_string(),
            ),
            log_path: FieldAvailability::available("/tmp/logs/open-bitcoin.log".to_string()),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::available(ServiceRestartResumeStatus {
                datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
                same_datadir: FieldAvailability::available(true),
                prior_shutdown: FieldAvailability::available(ServicePriorShutdownStatus::Clean),
                durable_progress: FieldAvailability::available(ServiceResumeProgressStatus {
                    downloaded_block_height: 840_006,
                    connected_block_height: 840_004,
                    maybe_downloaded_block_hash: Some("22".repeat(32)),
                    maybe_connected_block_hash: Some("11".repeat(32)),
                }),
                stale_inflight: FieldAvailability::available(ServiceStaleInflightStatus::Cleared),
                recovery_category: FieldAvailability::available(
                    SyncRecoveryCategory::CleanShutdown,
                ),
                next_action: FieldAvailability::available(
                    "Resume service sync review from preserved durable progress.".to_string(),
                ),
            }),
        },
        sync: SyncStatus {
            network: FieldAvailability::available("regtest".to_string()),
            chain_tip: FieldAvailability::unavailable("no tip"),
            sync_progress: FieldAvailability::unavailable("no sync"),
            lifecycle: FieldAvailability::unavailable("no sync lifecycle"),
            phase: FieldAvailability::unavailable("no sync phase"),
            configured_targets: FieldAvailability::<SyncConfiguredTargets>::unavailable(
                "no configured sync targets",
            ),
            attempt_counters: FieldAvailability::<SyncAttemptCounters>::unavailable(
                "no sync attempt counters",
            ),
            progress_signal: FieldAvailability::available(SyncProgressSignal::Steady),
            lag: FieldAvailability::unavailable("no sync lag"),
            last_successful_progress_unix_seconds: FieldAvailability::unavailable(
                "no successful sync progress",
            ),
            progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
            expected_progress_window: FieldAvailability::unavailable(
                "expected progress window unavailable",
            ),
            no_progress_threshold: FieldAvailability::unavailable(
                "no-progress threshold evidence unavailable",
            ),
            last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
            last_peer_contribution: FieldAvailability::unavailable(
                "last peer contribution unavailable",
            ),
            stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
            latest_stop_reason: FieldAvailability::<SyncStopReasonStatus>::unavailable(
                "no latest stop reason",
            ),
            last_error: FieldAvailability::unavailable("no sync error"),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable("no recovery action"),
            resource_pressure: FieldAvailability::unavailable("no sync pressure"),
            best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
                "best-known tip evidence unavailable",
            ),
            stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(
                "stay-current state unavailable",
            ),
            stay_current_next_action: FieldAvailability::unavailable(
                "stay-current next action unavailable",
            ),
            no_progress_diagnosis: FieldAvailability::unavailable(
                "no-progress diagnosis unavailable",
            ),
            no_progress_next_action: FieldAvailability::unavailable(
                "no-progress next action unavailable",
            ),
            latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
            reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 1,
                outbound: 2,
            }),
            recent_peers: FieldAvailability::unavailable("no peer telemetry"),
            inbound: inbound_status_unavailable(),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(4),
            relay: RelayEvidenceStatus::default(),
        },
        block_relay: BlockRelayEvidenceStatus::default_unavailable(),
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::available(50_000),
            freshness: FieldAvailability::available(WalletFreshness::Fresh),
            scan_progress: FieldAvailability::unavailable("wallet already fresh"),
        },
        logs: open_bitcoin_node::LogStatus::default(),
        metrics: MetricsStatus::available_with_samples(
            MetricRetentionPolicy::default(),
            vec![MetricSample::new(MetricKind::SyncHeight, 100.0, 10)],
        ),
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
        health_signals: vec![HealthSignal {
            level: HealthSignalLevel::Info,
            source: "test".to_string(),
            message: "ok".to_string(),
        }],
        build: BuildProvenance::unavailable(),
    }
}

fn shared_sync_truth_snapshot() -> OpenBitcoinStatusSnapshot {
    let mut snapshot = test_snapshot();
    snapshot.sync = SyncStatus {
        network: FieldAvailability::available("mainnet".to_string()),
        chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
        sync_progress: FieldAvailability::available(SyncProgress {
            header_height: 840_100,
            block_height: 840_004,
            downloaded_block_height: 840_006,
            connected_block_height: 840_004,
            validated_active_chain_height: 840_004,
            maybe_downloaded_block_hash: Some("22".repeat(32)),
            maybe_connected_block_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_work: Some("840005".to_string()),
            progress_ratio: 840_004.0 / 840_100.0,
            messages_processed: 7,
            headers_received: 3,
            blocks_received: 1,
        }),
        lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
        phase: FieldAvailability::available("block_download".to_string()),
        configured_targets: FieldAvailability::available(SyncConfiguredTargets {
            target_outbound_peers: 4,
            maybe_target_header_height: Some(840_200),
        }),
        attempt_counters: FieldAvailability::available(SyncAttemptCounters {
            attempted_peers: 3,
            connected_peers: 2,
            failed_peers: 1,
            max_sync_rounds: 8,
        }),
        progress_signal: FieldAvailability::available(SyncProgressSignal::AwaitingBlocks),
        lag: FieldAvailability::available(SyncLagStatus {
            headers_remaining: 0,
            blocks_remaining: 96,
        }),
        last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_000),
        progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
        expected_progress_window: FieldAvailability::unavailable(
            "expected progress window unavailable",
        ),
        no_progress_threshold: FieldAvailability::unavailable(
            "no-progress threshold evidence unavailable",
        ),
        last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
        last_peer_contribution: FieldAvailability::unavailable(
            "last peer contribution unavailable",
        ),
        stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
        latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
            label: "target_header_reached".to_string(),
            message:
                "sync header target reached: target_header_height=840200 best_header_height=840200"
                    .to_string(),
        }),
        last_error: FieldAvailability::available("peer stalled before block connect".to_string()),
        recovery_category: FieldAvailability::available(SyncRecoveryCategory::InvalidPeerData),
        recovery_action: FieldAvailability::available(
            "Retry sync after peer backoff or choose a different peer.".to_string(),
        ),
        resource_pressure: FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 16,
            max_blocks_in_flight_total: 64,
            max_messages_per_peer: 64,
            max_sync_rounds: 8,
            outbound_peers: 2,
            target_outbound_peers: 4,
        }),
        best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
            "best-known tip evidence unavailable",
        ),
        stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(
            "stay-current state unavailable",
        ),
        stay_current_next_action: FieldAvailability::unavailable(
            "stay-current next action unavailable",
        ),
        no_progress_diagnosis: FieldAvailability::unavailable("no-progress diagnosis unavailable"),
        no_progress_next_action: FieldAvailability::unavailable(
            "no-progress next action unavailable",
        ),
        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
        reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
    };
    snapshot.peers.peer_counts = FieldAvailability::available(PeerCounts {
        inbound: 0,
        outbound: 2,
    });
    snapshot
}

fn phase77_recovery_evidence() -> RecoveryEvidenceSnapshot {
    RecoveryEvidenceSnapshot {
        category: SyncRecoveryCategory::StorageLockContention,
        action_class: RecoveryActionClass::ReadOnlyInspection,
        cause: RecoveryCause::StaleLockEvidence,
        evidence_basis: vec![RecoveryEvidenceBasis::LockProbe],
        maybe_affected_namespace: None,
        maybe_affected_path: Some("/tmp/open-bitcoin/LOCK".to_string()),
        next_action:
            "Inspect the datadir read-only and avoid deleting lock artifacts automatically."
                .to_string(),
        compatibility_action: FieldAvailability::unavailable(
            "no compatibility recovery action recorded",
        ),
    }
}
