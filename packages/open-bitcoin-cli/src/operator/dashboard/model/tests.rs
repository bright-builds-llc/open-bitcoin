// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{
    MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus,
    status::{
        BuildProvenance, ConfigStatus, FieldAvailability, HealthSignal, HealthSignalLevel,
        MempoolStatus, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot, PeerCounts,
        PeerStatus, ServiceStatus, SyncLagStatus, SyncLifecycleState, SyncProgress,
        SyncProgressSignal, SyncRecoveryCategory, SyncResourcePressure, SyncStatus,
        WalletFreshness, WalletStatus,
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
fn dashboard_projection_preserves_shared_sync_truth_fields() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
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
    assert!(
        sync_rows
            .iter()
            .find(|row| row.label == "Pressure")
            .expect("pressure row")
            .value
            .contains("peers 2/4")
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
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::available(true),
            running: FieldAvailability::available(true),
        },
        sync: SyncStatus {
            network: FieldAvailability::available("regtest".to_string()),
            chain_tip: FieldAvailability::unavailable("no tip"),
            sync_progress: FieldAvailability::unavailable("no sync"),
            lifecycle: FieldAvailability::unavailable("no sync lifecycle"),
            phase: FieldAvailability::unavailable("no sync phase"),
            progress_signal: FieldAvailability::available(SyncProgressSignal::Steady),
            lag: FieldAvailability::unavailable("no sync lag"),
            last_successful_progress_unix_seconds: FieldAvailability::unavailable(
                "no successful sync progress",
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
        progress_signal: FieldAvailability::available(SyncProgressSignal::AwaitingBlocks),
        lag: FieldAvailability::available(SyncLagStatus {
            headers_remaining: 0,
            blocks_remaining: 96,
        }),
        last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_000),
        last_error: FieldAvailability::available("peer stalled before block connect".to_string()),
        recovery_category: FieldAvailability::available(
            SyncRecoveryCategory::PublicNetworkUnreachable,
        ),
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
