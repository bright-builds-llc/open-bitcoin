// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    BuildProvenance, ChainTipStatus, ConfigStatus, FieldAvailability, HealthSignal,
    HealthSignalLevel, MempoolStatus, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot,
    PeerCounts, PeerStatus, PeerTelemetry, ServiceStatus, SyncAttemptCounters,
    SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState, SyncProgress, SyncProgressSignal,
    SyncResourcePressure, SyncStatus, SyncStopReasonStatus, WalletFreshness, WalletScanProgress,
    WalletStatus,
};
use crate::{LogStatus, MetricsStatus};

#[test]
fn unavailable_field_serializes_with_reason() {
    // Arrange
    let value = FieldAvailability::<String>::unavailable("node stopped");

    // Act
    let encoded = serde_json::to_value(&value).expect("availability json");

    // Assert
    assert_eq!(encoded["state"], "unavailable");
    assert_eq!(encoded["value"]["reason"], "node stopped");
}

#[test]
fn unavailable_build_provenance_keeps_missing_fields_visible() {
    // Arrange / Act
    let provenance = BuildProvenance::unavailable();
    let encoded = serde_json::to_value(provenance).expect("provenance json");

    // Assert
    assert_eq!(encoded["commit"]["state"], "unavailable");
    assert_eq!(encoded["build_time"]["state"], "unavailable");
    assert_eq!(encoded["target"]["state"], "unavailable");
}

#[test]
fn phase62_sync_truth_contract() {
    // Arrange
    let sync = SyncStatus {
        network: FieldAvailability::available("mainnet".to_string()),
        chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
        sync_progress: FieldAvailability::unavailable("sync progress unavailable"),
        lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
        phase: FieldAvailability::available("headers".to_string()),
        configured_targets: FieldAvailability::available(SyncConfiguredTargets {
            target_outbound_peers: 4,
            maybe_target_header_height: Some(840_123),
        }),
        attempt_counters: FieldAvailability::available(SyncAttemptCounters {
            attempted_peers: 3,
            connected_peers: 2,
            failed_peers: 1,
            max_sync_rounds: 8,
        }),
        progress_signal: FieldAvailability::available(SyncProgressSignal::HeaderProgress),
        lag: FieldAvailability::available(SyncLagStatus {
            headers_remaining: 0,
            blocks_remaining: 12,
        }),
        last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_000),
        latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
            label: "target_header_reached".to_string(),
            message: "sync header target reached".to_string(),
        }),
        last_error: FieldAvailability::unavailable("no sync error recorded"),
        recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
        recovery_action: FieldAvailability::unavailable("no recovery action required"),
        resource_pressure: FieldAvailability::unavailable("resource pressure unavailable"),
    };

    // Act
    let encoded = serde_json::to_value(&sync).expect("sync status json");
    let legacy_sync: SyncStatus = serde_json::from_value(serde_json::json!({
        "network": { "state": "available", "value": "mainnet" },
        "chain_tip": { "state": "unavailable", "value": { "reason": "chain tip unavailable" } },
        "sync_progress": { "state": "unavailable", "value": { "reason": "sync progress unavailable" } },
        "lifecycle": { "state": "available", "value": "active" },
        "phase": { "state": "available", "value": "headers" },
        "progress_signal": { "state": "available", "value": "header_progress" },
        "lag": {
            "state": "available",
            "value": { "headers_remaining": 0, "blocks_remaining": 12 }
        },
        "last_successful_progress_unix_seconds": { "state": "available", "value": 1717000000 },
        "last_error": { "state": "unavailable", "value": { "reason": "no sync error recorded" } },
        "recovery_category": { "state": "unavailable", "value": { "reason": "no recovery category recorded" } },
        "recovery_action": { "state": "unavailable", "value": { "reason": "no recovery action required" } },
        "resource_pressure": { "state": "unavailable", "value": { "reason": "resource pressure unavailable" } }
    }))
    .expect("legacy sync status json");

    // Assert
    assert_eq!(encoded["configured_targets"]["state"], "available");
    assert_eq!(
        encoded["configured_targets"]["value"]["target_outbound_peers"],
        4
    );
    assert_eq!(
        encoded["configured_targets"]["value"]["maybe_target_header_height"],
        840_123
    );
    assert_eq!(encoded["attempt_counters"]["value"]["attempted_peers"], 3);
    assert_eq!(encoded["attempt_counters"]["value"]["max_sync_rounds"], 8);
    assert_eq!(
        encoded["latest_stop_reason"]["value"]["label"],
        "target_header_reached"
    );
    assert_eq!(
        legacy_sync.configured_targets,
        FieldAvailability::unavailable("configured targets unavailable")
    );
    assert_eq!(
        legacy_sync.attempt_counters,
        FieldAvailability::unavailable("attempt counters unavailable")
    );
    assert_eq!(
        legacy_sync.latest_stop_reason,
        FieldAvailability::unavailable("latest stop reason unavailable")
    );
}

#[test]
fn stopped_node_snapshot_keeps_unavailable_live_fields_explicit() {
    // Arrange / Act
    let snapshot = stopped_snapshot();
    let encoded = serde_json::to_value(&snapshot).expect("snapshot json");

    // Assert
    assert_eq!(snapshot.node.state, NodeRuntimeState::Stopped);
    assert_eq!(encoded["sync"]["network"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["chain_tip"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["sync_progress"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["lifecycle"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["phase"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["configured_targets"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["attempt_counters"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["progress_signal"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["lag"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["last_successful_progress_unix_seconds"]["state"],
        "unavailable"
    );
    assert_eq!(
        encoded["sync"]["latest_stop_reason"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["last_error"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["recovery_category"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["recovery_action"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["resource_pressure"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["peer_counts"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["recent_peers"]["state"], "unavailable");
    assert_eq!(encoded["mempool"]["transactions"]["state"], "unavailable");
    assert_eq!(
        encoded["wallet"]["trusted_balance_sats"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["wallet"]["freshness"]["state"], "unavailable");
    assert_eq!(encoded["wallet"]["scan_progress"]["state"], "unavailable");
    assert_eq!(encoded["config"]["datadir"]["state"], "available");
    assert_eq!(encoded["logs"]["retention"]["max_files"], 14);
    assert_eq!(
        encoded["metrics"]["retention"]["sample_interval_seconds"],
        30
    );
}

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
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::available(true),
            running: FieldAvailability::available(true),
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
                maybe_downloaded_block_hash: Some(
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                ),
                maybe_connected_block_hash: Some(
                    "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
                ),
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
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(12),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::available(25_000),
            freshness: FieldAvailability::available(WalletFreshness::Fresh),
            scan_progress: FieldAvailability::unavailable("wallet already fresh"),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::default(),
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

fn stopped_snapshot() -> OpenBitcoinStatusSnapshot {
    let unavailable = "node stopped";
    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Stopped,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec!["/tmp/open-bitcoin/bitcoin.conf".to_string()],
        },
        service: ServiceStatus {
            manager: FieldAvailability::unavailable("service manager not inspected"),
            installed: FieldAvailability::unavailable("service manager not inspected"),
            enabled: FieldAvailability::unavailable("service manager not inspected"),
            running: FieldAvailability::unavailable("service manager not inspected"),
        },
        sync: SyncStatus {
            network: FieldAvailability::unavailable(unavailable),
            chain_tip: FieldAvailability::unavailable(unavailable),
            sync_progress: FieldAvailability::unavailable(unavailable),
            lifecycle: FieldAvailability::unavailable(unavailable),
            phase: FieldAvailability::unavailable(unavailable),
            configured_targets: FieldAvailability::unavailable(unavailable),
            attempt_counters: FieldAvailability::unavailable(unavailable),
            progress_signal: FieldAvailability::unavailable(unavailable),
            lag: FieldAvailability::unavailable(unavailable),
            last_successful_progress_unix_seconds: FieldAvailability::unavailable(unavailable),
            latest_stop_reason: FieldAvailability::unavailable(unavailable),
            last_error: FieldAvailability::unavailable(unavailable),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable(unavailable),
            resource_pressure: FieldAvailability::unavailable(unavailable),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::unavailable(unavailable),
            recent_peers: FieldAvailability::unavailable(unavailable),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::unavailable(unavailable),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::unavailable(unavailable),
            freshness: FieldAvailability::unavailable(unavailable),
            scan_progress: FieldAvailability::unavailable(unavailable),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::default(),
        health_signals: Vec::new(),
        build: BuildProvenance::unavailable(),
    }
}
