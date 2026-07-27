// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn sync_summary_status_keeps_connected_height_alias_with_hashes() {
    // Arrange
    let downloaded_hash =
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
    let connected_hash =
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string();
    let summary = SyncRunSummary {
        target_outbound_peers: 4,
        maybe_target_header_height: None,
        attempted_peers: 2,
        connected_peers: 1,
        failed_peers: 0,
        messages_processed: 9,
        headers_received: 7,
        blocks_received: 3,
        best_header_height: 30,
        downloaded_block_height: 27,
        best_block_height: 25,
        maybe_downloaded_block_hash: Some(downloaded_hash.clone()),
        maybe_connected_block_hash: Some(connected_hash.clone()),
        maybe_validated_active_chain_work: Some("26".to_string()),
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    };

    // Act
    let status = summary.sync_status(SyncNetwork::Regtest);

    // Assert
    assert_eq!(
        status.sync_progress,
        crate::FieldAvailability::available(crate::status::SyncProgress {
            header_height: 30,
            block_height: 25,
            downloaded_block_height: 27,
            connected_block_height: 25,
            validated_active_chain_height: 25,
            maybe_downloaded_block_hash: Some(downloaded_hash),
            maybe_connected_block_hash: Some(connected_hash.clone()),
            maybe_validated_active_chain_hash: Some(connected_hash),
            maybe_validated_active_chain_work: Some("26".to_string()),
            progress_ratio: 25.0 / 30.0,
            messages_processed: 9,
            headers_received: 7,
            blocks_received: 3,
        })
    );
}

#[test]
fn sync_summary_status_projections_include_counters() {
    // Arrange
    let summary = SyncRunSummary {
        target_outbound_peers: 4,
        maybe_target_header_height: None,
        attempted_peers: 4,
        connected_peers: 3,
        failed_peers: 1,
        messages_processed: 12,
        headers_received: 7,
        blocks_received: 5,
        best_header_height: 100,
        downloaded_block_height: 75,
        best_block_height: 25,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    };

    // Act
    let sync_status = summary.sync_status(SyncNetwork::Regtest);
    let peer_status = summary.peer_status();

    // Assert
    assert_eq!(
        sync_status.sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 100,
            block_height: 25,
            downloaded_block_height: 75,
            connected_block_height: 25,
            validated_active_chain_height: 25,
            maybe_downloaded_block_hash: None,
            maybe_connected_block_hash: None,
            maybe_validated_active_chain_hash: None,
            maybe_validated_active_chain_work: None,
            progress_ratio: 0.25,
            messages_processed: 12,
            headers_received: 7,
            blocks_received: 5,
        })
    );
    assert_eq!(
        sync_status.progress_signal,
        FieldAvailability::available(SyncProgressSignal::BlockProgress)
    );
    assert!(matches!(
        sync_status.last_successful_progress_unix_seconds,
        FieldAvailability::Unavailable { .. }
    ));
    assert_eq!(
        peer_status.peer_counts,
        FieldAvailability::available(crate::status::PeerCounts {
            inbound: 0,
            outbound: 3,
        })
    );
    assert_eq!(
        sync_status.resource_pressure,
        FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 0,
            max_blocks_in_flight_total: 0,
            max_messages_per_peer: 0,
            max_sync_rounds: 0,
            outbound_peers: 3,
            target_outbound_peers: 4,
        })
    );
}

#[test]
fn phase69_sync_status_defaults_tip_and_stay_current_fields() {
    // Arrange
    let payload = serde_json::json!({
        "network": { "state": "available", "value": "regtest" },
        "chain_tip": {
            "state": "unavailable",
            "value": { "reason": "chain tip unavailable" }
        },
        "sync_progress": {
            "state": "unavailable",
            "value": { "reason": "sync progress unavailable" }
        },
        "lifecycle": { "state": "available", "value": "active" },
        "phase": { "state": "available", "value": "headers" },
        "configured_targets": {
            "state": "available",
            "value": {
                "target_outbound_peers": 1,
                "maybe_target_header_height": null
            }
        },
        "attempt_counters": {
            "state": "available",
            "value": {
                "attempted_peers": 1,
                "connected_peers": 1,
                "failed_peers": 0,
                "max_sync_rounds": 8
            }
        },
        "progress_signal": { "state": "available", "value": "steady" },
        "lag": {
            "state": "available",
            "value": { "headers_remaining": 0, "blocks_remaining": 0 }
        },
        "last_successful_progress_unix_seconds": {
            "state": "unavailable",
            "value": { "reason": "no successful sync progress recorded in this run" }
        },
        "latest_stop_reason": {
            "state": "unavailable",
            "value": { "reason": "no stop reason recorded" }
        },
        "last_error": {
            "state": "unavailable",
            "value": { "reason": "no sync error recorded" }
        },
        "recovery_category": {
            "state": "unavailable",
            "value": { "reason": "no recovery category recorded" }
        },
        "recovery_action": {
            "state": "unavailable",
            "value": { "reason": "no recovery action required" }
        },
        "resource_pressure": {
            "state": "unavailable",
            "value": { "reason": "resource pressure unavailable" }
        }
    });

    // Act
    let sync_status: SyncStatus = serde_json::from_value(payload).expect("sync status decode");

    // Assert
    assert_eq!(
        sync_status.best_known_tip,
        FieldAvailability::unavailable("best-known tip evidence unavailable")
    );
    assert_eq!(
        sync_status.stay_current,
        FieldAvailability::<StayCurrentStatus>::unavailable("stay-current state unavailable")
    );
    assert_eq!(
        sync_status.stay_current_next_action,
        FieldAvailability::<String>::unavailable("stay-current next action unavailable")
    );
}

#[test]
fn phase69_sync_status_serializes_tip_and_stay_current_fields() {
    // Arrange
    let mut sync_status = SyncRunSummary::empty(2, 2, 1).sync_status(SyncNetwork::Regtest);
    sync_status.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height: 2,
        block_hash: "aa".to_string(),
        work: "3".to_string(),
        block_time_unix_seconds: 1_777_225_000,
        observed_at_unix_seconds: 1_777_225_010,
        freshness: TipFreshnessStatus::Fresh,
        peer_agreement: vec![PeerTipAgreement {
            peer: "127.0.0.1:18444".to_string(),
            maybe_resolved_endpoint: Some("127.0.0.1:18444".to_string()),
            status: PeerTipAgreementStatus::Agrees,
            maybe_height: Some(2),
            maybe_hash: Some("aa".to_string()),
            maybe_work: Some("3".to_string()),
            maybe_last_activity_unix_seconds: Some(1_777_225_010),
        }],
    });
    sync_status.stay_current =
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
    sync_status.stay_current_next_action = FieldAvailability::available(
        "No action required; node is current at the best-known validated tip.".to_string(),
    );

    // Act
    let encoded = serde_json::to_string(&sync_status).expect("sync status encode");

    // Assert
    assert!(encoded.contains("best_known_tip"));
    assert!(encoded.contains("header_store"));
    assert!(encoded.contains("fresh"));
    assert!(encoded.contains("agrees"));
    assert!(encoded.contains("current_at_best_known_tip"));
    assert!(
        encoded.contains("No action required; node is current at the best-known validated tip.")
    );
}

#[test]
fn sync_runtime_errors_project_storage_and_network_health_signals() {
    // Arrange
    let network_error = SyncRuntimeError::Network {
        message: "connection reset".to_string(),
    };
    let storage_error = SyncRuntimeError::Storage(StorageError::UnavailableNamespace {
        namespace: StorageNamespace::Metrics,
    });

    // Act
    let network_signal = network_error.health_signal();
    let storage_signal = storage_error.health_signal();

    // Assert
    assert_eq!(network_signal.level, HealthSignalLevel::Error);
    assert_eq!(network_signal.source, "network");
    assert!(network_signal.message.contains("sync network failure"));
    assert_eq!(storage_signal.level, HealthSignalLevel::Error);
    assert_eq!(storage_signal.source, "storage");
    assert!(
        storage_signal
            .message
            .contains("storage namespace unavailable")
    );
    assert!(network_signal.message.len() <= 160);
    assert!(storage_signal.message.len() <= 160);
}
