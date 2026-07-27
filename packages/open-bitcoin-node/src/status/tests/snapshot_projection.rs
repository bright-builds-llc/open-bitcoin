// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use crate::status::EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON;

#[test]
fn populated_snapshot_serializes_obs_01_fields() {
    // Arrange
    let snapshot = OpenBitcoinStatusSnapshot {
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
            restart_resume: FieldAvailability::unavailable(
                "service restart/resume evidence unavailable",
            ),
        },
        sync: SyncStatus {
            network: FieldAvailability::available("mainnet".to_string()),
            chain_tip: FieldAvailability::available(ChainTipStatus {
                height: 840_000,
                block_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                    .to_string(),
            }),
            sync_progress: FieldAvailability::available(SyncProgress {
                header_height: 840_001,
                block_height: 840_000,
                downloaded_block_height: 840_000,
                connected_block_height: 840_000,
                validated_active_chain_height: 840_000,
                maybe_downloaded_block_hash: Some(
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                ),
                maybe_connected_block_hash: Some(
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                ),
                maybe_validated_active_chain_hash: Some(
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                ),
                maybe_validated_active_chain_work: Some("840001".to_string()),
                progress_ratio: 0.99,
                messages_processed: 12,
                headers_received: 1,
                blocks_received: 1,
            }),
            lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
            phase: FieldAvailability::available("block_download".to_string()),
            configured_targets: FieldAvailability::available(SyncConfiguredTargets {
                target_outbound_peers: 4,
                maybe_target_header_height: Some(840_001),
            }),
            attempt_counters: FieldAvailability::available(SyncAttemptCounters {
                attempted_peers: 2,
                connected_peers: 2,
                failed_peers: 0,
                max_sync_rounds: 8,
            }),
            progress_signal: FieldAvailability::available(SyncProgressSignal::BlockProgress),
            lag: FieldAvailability::available(SyncLagStatus {
                headers_remaining: 0,
                blocks_remaining: 1,
            }),
            last_successful_progress_unix_seconds: FieldAvailability::available(1_715_000_000),
            progress_credit: FieldAvailability::unavailable(PROGRESS_CREDIT_UNAVAILABLE_REASON),
            expected_progress_window: FieldAvailability::unavailable(
                EXPECTED_PROGRESS_WINDOW_UNAVAILABLE_REASON,
            ),
            no_progress_threshold: FieldAvailability::unavailable(
                NO_PROGRESS_THRESHOLD_UNAVAILABLE_REASON,
            ),
            last_useful_work: FieldAvailability::unavailable(LAST_USEFUL_WORK_UNAVAILABLE_REASON),
            last_peer_contribution: FieldAvailability::unavailable(
                LAST_PEER_CONTRIBUTION_UNAVAILABLE_REASON,
            ),
            stall_diagnosis: FieldAvailability::unavailable(STALL_DIAGNOSIS_UNAVAILABLE_REASON),
            latest_stop_reason: FieldAvailability::unavailable("no stop reason recorded"),
            last_error: FieldAvailability::unavailable("no sync error recorded"),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable("no recovery action required"),
            resource_pressure: FieldAvailability::available(SyncResourcePressure {
                blocks_in_flight: 1,
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
            stay_current: FieldAvailability::unavailable("stay-current state unavailable"),
            stay_current_next_action: FieldAvailability::unavailable(
                "stay-current next action unavailable",
            ),
            no_progress_diagnosis: FieldAvailability::unavailable(
                NO_PROGRESS_DIAGNOSIS_UNAVAILABLE_REASON,
            ),
            no_progress_next_action: FieldAvailability::unavailable(
                NO_PROGRESS_NEXT_ACTION_UNAVAILABLE_REASON,
            ),
            latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
            reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 8,
            }),
            recent_peers: FieldAvailability::available(vec![PeerTelemetry {
                peer: "seed.bitcoin.sipa.be:8333".to_string(),
                source: "dns_seed".to_string(),
                state: "connected".to_string(),
                network: "mainnet".to_string(),
                attempts: 1,
                maybe_resolved_endpoint: FieldAvailability::available(
                    "203.0.113.10:8333".to_string(),
                ),
                capabilities: FieldAvailability::available("services=9 prefs=headers".to_string()),
                headers_received: 1,
                blocks_received: 1,
                maybe_last_activity_unix_seconds: FieldAvailability::available(1_715_000_000),
                failure_reason: FieldAvailability::unavailable("peer healthy"),
                error: FieldAvailability::unavailable("peer healthy"),
            }]),
            inbound: FieldAvailability::unavailable(INBOUND_STATUS_UNAVAILABLE_REASON),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(12),
            relay: RelayEvidenceStatus::default(),
        },
        block_relay: BlockRelayEvidenceStatus::default_unavailable(),
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::available(25_000),
            freshness: FieldAvailability::available(WalletFreshness::Fresh),
            scan_progress: FieldAvailability::unavailable("wallet already fresh"),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::default(),
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
        health_signals: vec![HealthSignal {
            level: HealthSignalLevel::Info,
            source: "status".to_string(),
            message: "node healthy".to_string(),
        }],
        build: BuildProvenance::unavailable(),
    };

    // Act
    let encoded = serde_json::to_value(&snapshot).expect("snapshot json");

    // Assert
    assert_eq!(encoded["config"]["datadir"]["state"], "available");
    assert_eq!(
        encoded["config"]["config_paths"][0],
        "/tmp/open-bitcoin/bitcoin.conf"
    );
    assert_eq!(encoded["sync"]["network"]["value"], "mainnet");
    assert_eq!(encoded["sync"]["chain_tip"]["value"]["height"], 840_000);
    assert_eq!(
        encoded["sync"]["sync_progress"]["value"]["header_height"],
        840_001
    );
    assert_eq!(encoded["sync"]["lifecycle"]["value"], "active");
    assert_eq!(encoded["sync"]["phase"]["value"], "block_download");
    assert_eq!(
        encoded["sync"]["configured_targets"]["value"]["target_outbound_peers"],
        4
    );
    assert_eq!(
        encoded["sync"]["attempt_counters"]["value"]["attempted_peers"],
        2
    );
    assert_eq!(
        encoded["sync"]["progress_signal"]["value"],
        "block_progress"
    );
    assert_eq!(encoded["sync"]["lag"]["value"]["blocks_remaining"], 1);
    assert_eq!(
        encoded["sync"]["last_successful_progress_unix_seconds"]["value"],
        1_715_000_000
    );
    assert_eq!(encoded["sync"]["progress_credit"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["expected_progress_window"]["state"],
        "unavailable"
    );
    assert_eq!(
        encoded["sync"]["no_progress_threshold"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["last_useful_work"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["last_peer_contribution"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["stall_diagnosis"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["latest_stop_reason"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["recovery_category"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["recovery_action"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["peer_counts"]["value"]["outbound"], 8);
    assert_eq!(
        encoded["peers"]["recent_peers"]["value"][0]["source"],
        "dns_seed"
    );
    assert_eq!(encoded["wallet"]["freshness"]["value"], "fresh");
    assert_eq!(encoded["wallet"]["scan_progress"]["state"], "unavailable");
    assert_eq!(encoded["health_signals"][0]["message"], "node healthy");
}

#[test]
fn wallet_freshness_states_serialize_distinctly_in_snapshot() {
    // Arrange
    let states = [
        (
            WalletFreshness::Fresh,
            FieldAvailability::unavailable("wallet already fresh"),
            "fresh",
        ),
        (
            WalletFreshness::Stale,
            FieldAvailability::unavailable("wallet scan not running"),
            "stale",
        ),
        (
            WalletFreshness::Partial,
            FieldAvailability::available(WalletScanProgress {
                scanned_through_height: 40,
                target_tip_height: 100,
            }),
            "partial",
        ),
        (
            WalletFreshness::Scanning,
            FieldAvailability::available(WalletScanProgress {
                scanned_through_height: 60,
                target_tip_height: 100,
            }),
            "scanning",
        ),
    ];

    // Act
    let encoded = states
        .into_iter()
        .map(|(freshness, scan_progress, expected)| {
            let mut snapshot = stopped_snapshot();
            snapshot.wallet = WalletStatus {
                trusted_balance_sats: FieldAvailability::available(25_000),
                freshness: FieldAvailability::available(freshness),
                scan_progress,
            };
            let encoded = serde_json::to_value(snapshot).expect("snapshot json");
            (encoded, expected)
        })
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(encoded[0].0["wallet"]["freshness"]["value"], encoded[0].1);
    assert_eq!(encoded[1].0["wallet"]["freshness"]["value"], encoded[1].1);
    assert_eq!(encoded[2].0["wallet"]["freshness"]["value"], encoded[2].1);
    assert_eq!(encoded[3].0["wallet"]["freshness"]["value"], encoded[3].1);
    assert_eq!(
        encoded[2].0["wallet"]["scan_progress"]["value"]["scanned_through_height"],
        40
    );
    assert_eq!(
        encoded[3].0["wallet"]["scan_progress"]["value"]["target_tip_height"],
        100
    );
}
