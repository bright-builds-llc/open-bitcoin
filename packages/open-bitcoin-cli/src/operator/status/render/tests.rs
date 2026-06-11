// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{
    BuildProvenance, LogStatus, MetricsStatus,
    status::{
        BestKnownTipStatus, ConfigStatus, FieldAvailability, MempoolStatus, NodeRuntimeState,
        NodeStatus, OpenBitcoinStatusSnapshot, PeerCounts, PeerStatus, PeerTelemetry,
        ServiceLifecycleStatus, ServicePriorShutdownStatus, ServiceRestartResumeStatus,
        ServiceResumeProgressStatus, ServiceStaleInflightStatus, ServiceStatus, StayCurrentStatus,
        SyncAttemptCounters, SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState,
        SyncProgress, SyncProgressSignal, SyncRecoveryCategory, SyncResourcePressure, SyncStatus,
        SyncStopReasonStatus, WalletStatus,
    },
};

use super::{StatusRenderMode, render_status};

#[test]
fn status_render_includes_sync_progress_and_peer_evidence() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains("headers=840100 downloaded_blocks=840006 connected_blocks=840004"));
    for expected in [
        "Sync configured targets: outbound_peers=4 target_header_height=840200",
        "Sync attempts: attempted_peers=3 connected_peers=2 failed_peers=1 max_sync_rounds=8",
        "Sync latest stop reason: target_header_reached",
        "awaiting_blocks",
        "Sync recovery category: invalid_peer_data",
        "Sync recovery: Retry sync after peer backoff",
        "peer stalled before block connect",
        "failed:seed.bitcoin.sipa.be:8333 via dns_seed",
    ] {
        assert!(rendered.contains(expected));
    }

    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.sync.configured_targets =
        FieldAvailability::unavailable("operator target unavailable");
    snapshot.sync.attempt_counters = FieldAvailability::unavailable("attempt counters unavailable");
    snapshot.sync.latest_stop_reason = FieldAvailability::unavailable("stop reason unavailable");

    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    for expected in [
        "Sync configured targets: Unavailable: operator target unavailable",
        "Sync attempts: Unavailable: attempt counters unavailable",
        "Sync latest stop reason: Unavailable: stop reason unavailable",
    ] {
        assert!(rendered.contains(expected));
    }
    for unexpected in [
        "Sync configured targets: outbound_peers=0",
        "Sync attempts: attempted_peers=0",
        "Sync latest stop reason: ok",
    ] {
        assert!(!rendered.contains(unexpected));
    }
}

#[test]
fn phase63_service_lifecycle_rendering_human_status_contract() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains(
        "Service: lifecycle=running manager=launchd installed=true enabled=true running=true file=/tmp/open-bitcoin-node.service logs=/tmp/logs/open-bitcoin.log diagnostics=Unavailable: service diagnostics unavailable"
    ));

    let mut unavailable = shared_sync_truth_snapshot();
    unavailable.service = ServiceStatus {
        manager: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        lifecycle: FieldAvailability::available(ServiceLifecycleStatus::UnavailableManager),
        installed: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        enabled: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        running: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        service_file_path: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        log_path: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: launchd unavailable",
        ),
        diagnostics: FieldAvailability::available(
            "unsupported platform: launchd unavailable".to_string(),
        ),
        restart_resume: FieldAvailability::unavailable(
            "service restart/resume evidence unavailable",
        ),
    };

    let rendered = render_status(&unavailable, StatusRenderMode::Human).expect("human status");

    assert!(rendered.contains("Service: lifecycle=unavailable-manager manager=Unavailable: service manager unavailable: unsupported platform: launchd unavailable"));
    assert!(rendered.contains("file=Unavailable: service manager unavailable"));
    assert!(rendered.contains("logs=Unavailable: service manager unavailable"));
    assert!(rendered.contains("diagnostics=unsupported platform: launchd unavailable"));
}

#[test]
fn service_restart_resume_status_render_includes_phase64_evidence() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let human = render_status(&snapshot, StatusRenderMode::Human).expect("human status");
    let json = render_status(&snapshot, StatusRenderMode::Json).expect("json status");
    let decoded: serde_json::Value = serde_json::from_str(&json).expect("decode status json");

    // Assert
    assert!(human.contains("restart_resume=datadir=/tmp/open-bitcoin same_datadir=true prior_shutdown=clean downloaded=840006 connected=840004 stale_inflight=cleared recovery_category=clean_shutdown next_action=Resume service sync review from preserved durable progress."));
    assert_eq!(decoded["service"]["restart_resume"]["state"], "available");
    assert_eq!(
        decoded["service"]["restart_resume"]["value"]["prior_shutdown"]["value"],
        "clean"
    );
    assert_eq!(
        decoded["service"]["restart_resume"]["value"]["stale_inflight"]["value"],
        "cleared"
    );
    assert_eq!(
        decoded["service"]["restart_resume"]["value"]["durable_progress"]["value"]["downloaded_block_height"],
        840_006
    );
}

fn shared_sync_truth_snapshot() -> OpenBitcoinStatusSnapshot {
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec!["/tmp/open-bitcoin/open-bitcoin.jsonc".to_string()],
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
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 2,
            }),
            recent_peers: FieldAvailability::available(vec![PeerTelemetry {
                peer: "seed.bitcoin.sipa.be:8333".to_string(),
                source: "dns_seed".to_string(),
                state: "failed".to_string(),
                network: "mainnet".to_string(),
                attempts: 1,
                maybe_resolved_endpoint: FieldAvailability::available(
                    "203.0.113.10:8333".to_string(),
                ),
                capabilities: FieldAvailability::unavailable("peer capabilities unavailable"),
                headers_received: 3,
                blocks_received: 0,
                maybe_last_activity_unix_seconds: FieldAvailability::available(1_717_000_000),
                failure_reason: FieldAvailability::available("compatibility".to_string()),
                error: FieldAvailability::available(
                    "failed:seed.bitcoin.sipa.be:8333 via dns_seed".to_string(),
                ),
            }]),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable("mempool unavailable"),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::unavailable("wallet unavailable"),
            freshness: FieldAvailability::unavailable("wallet unavailable"),
            scan_progress: FieldAvailability::unavailable("wallet unavailable"),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::default(),
        health_signals: Vec::new(),
        build: BuildProvenance::unavailable(),
    }
}
