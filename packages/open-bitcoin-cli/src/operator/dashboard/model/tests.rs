// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{
    MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus,
    status::{
        BuildProvenance, ConfigStatus, FieldAvailability, HealthSignal, HealthSignalLevel,
        MempoolStatus, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot, PeerCounts,
        PeerStatus, ServiceLifecycleStatus, ServicePriorShutdownStatus, ServiceRestartResumeStatus,
        ServiceResumeProgressStatus, ServiceStaleInflightStatus, ServiceStatus,
        SyncAttemptCounters, SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState,
        SyncProgress, SyncProgressSignal, SyncRecoveryCategory, SyncResourcePressure, SyncStatus,
        SyncStopReasonStatus, WalletFreshness, WalletStatus,
    },
};

use super::{DASHBOARD_METRIC_KINDS, DashboardState, derive_metric_points};

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
    assert_eq!(wallet_rows[2].label, "Freshness");
    assert_eq!(wallet_rows[2].value, "fresh");
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
            "Last progress",
            "Latest stop reason",
            "Last error",
            "Recovery category",
            "Recovery",
            "Pressure",
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
        "99.99% headers=840100 downloaded_blocks=840006 connected_blocks=840004"
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
            latest_stop_reason: FieldAvailability::<SyncStopReasonStatus>::unavailable(
                "no latest stop reason",
            ),
            last_error: FieldAvailability::unavailable("no sync error"),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable("no recovery action"),
            resource_pressure: FieldAvailability::unavailable("no sync pressure"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 1,
                outbound: 2,
            }),
            recent_peers: FieldAvailability::unavailable("no peer telemetry"),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(4),
        },
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
            maybe_downloaded_block_hash: Some("22".repeat(32)),
            maybe_connected_block_hash: Some("11".repeat(32)),
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
    };
    snapshot.peers.peer_counts = FieldAvailability::available(PeerCounts {
        inbound: 0,
        outbound: 2,
    });
    snapshot
}
