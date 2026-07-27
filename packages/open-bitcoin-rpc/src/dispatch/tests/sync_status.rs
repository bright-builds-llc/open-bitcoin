// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::chain_fixtures::empty_context;
use super::storage_fixtures::{remove_dir_if_exists, temp_store_path};
use super::*;

fn phase62_runtime_metadata() -> RuntimeMetadata {
    RuntimeMetadata {
        maybe_sync_state: Some(DurableSyncState {
            sync: SyncStatus {
                network: FieldAvailability::available("main".to_string()),
                chain_tip: FieldAvailability::available(ChainTipStatus {
                    height: 840_004,
                    block_hash: "00".repeat(32),
                }),
                sync_progress: FieldAvailability::available(SyncProgress {
                    header_height: 840_200,
                    block_height: 840_004,
                    downloaded_block_height: 840_006,
                    connected_block_height: 840_004,
                    validated_active_chain_height: 840_004,
                    maybe_downloaded_block_hash: Some("11".repeat(32)),
                    maybe_connected_block_hash: Some("00".repeat(32)),
                    maybe_validated_active_chain_hash: Some("00".repeat(32)),
                    maybe_validated_active_chain_work: Some("840005".to_string()),
                    progress_ratio: 840_004.0 / 840_200.0,
                    messages_processed: 42,
                    headers_received: 100,
                    blocks_received: 3,
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
                progress_signal: FieldAvailability::available(SyncProgressSignal::HeaderProgress),
                lag: FieldAvailability::available(SyncLagStatus {
                    headers_remaining: 0,
                    blocks_remaining: 100,
                }),
                last_successful_progress_unix_seconds: FieldAvailability::available(1_715_000_000),
                progress_credit: FieldAvailability::unavailable(
                    "progress credit evidence unavailable",
                ),
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
                    message: "sync header target reached".to_string(),
                }),
                last_error: FieldAvailability::available(
                    "peer stalled before block connect".to_string(),
                ),
                recovery_category: FieldAvailability::available(
                    SyncRecoveryCategory::InvalidPeerData,
                ),
                recovery_action: FieldAvailability::available(
                    "Restart the node and retry the storage operation.".to_string(),
                ),
                resource_pressure: FieldAvailability::available(SyncResourcePressure {
                    blocks_in_flight: 8,
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
                no_progress_diagnosis: FieldAvailability::unavailable(
                    "no-progress diagnosis unavailable",
                ),
                no_progress_next_action: FieldAvailability::unavailable(
                    "no-progress next action unavailable",
                ),
                latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
                reconcile_progress: FieldAvailability::unavailable(
                    "reconcile progress unavailable",
                ),
            },
            peers: PeerStatus {
                peer_counts: FieldAvailability::available(PeerCounts {
                    inbound: 0,
                    outbound: 2,
                }),
                recent_peers: FieldAvailability::available(Vec::new()),
                inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                    INBOUND_STATUS_UNAVAILABLE_REASON,
                ),
            },
            health_signals: Vec::new(),
            updated_at_unix_seconds: 1_715_000_000,
        }),
        ..RuntimeMetadata::default()
    }
}

fn phase72_runtime_metadata() -> RuntimeMetadata {
    let mut metadata = phase62_runtime_metadata();
    let sync_state = metadata
        .maybe_sync_state
        .as_mut()
        .expect("phase62 metadata includes sync state");
    let FieldAvailability::Available(sync_progress) = &mut sync_state.sync.sync_progress else {
        panic!("phase62 metadata includes sync progress");
    };
    sync_progress.maybe_connected_block_hash = Some("11".repeat(32));
    sync_progress.maybe_validated_active_chain_hash = Some("11".repeat(32));
    sync_state.sync.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height: 840_004,
        block_hash: "11".repeat(32),
        work: "840005".to_string(),
        block_time_unix_seconds: 1_717_000_010,
        observed_at_unix_seconds: 1_717_000_020,
        freshness: TipFreshnessStatus::Fresh,
        peer_agreement: Vec::new(),
    });
    sync_state.sync.stay_current =
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
    sync_state.sync.no_progress_diagnosis =
        FieldAvailability::available(NoProgressDiagnosis::PeerBackoff);
    sync_state.sync.latest_reorg = FieldAvailability::available(SyncReorgEvidence {
        common_ancestor_height: 840_000,
        common_ancestor_hash: "11".repeat(32),
        disconnected_count: 2,
        connected_count: 4,
        final_active_height: 840_004,
        final_active_hash: "11".repeat(32),
        fully_persisted: true,
    });
    sync_state.sync.reconcile_progress =
        FieldAvailability::available(SyncReconcileProgressStatus::ExtendedActiveChain {
            connected_count: 4,
            final_active_height: 840_004,
            final_active_hash: "11".repeat(32),
        });
    metadata
}

fn context_with_runtime_metadata(test_name: &str, metadata: RuntimeMetadata) -> ManagedRpcContext {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .save_runtime_metadata(&metadata, PersistMode::Sync)
        .expect("save runtime metadata");
    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Mainnet,
        maybe_data_dir: Some(path),
        ..RuntimeConfig::default()
    });
    context.set_daemon_sync_control(DaemonSyncControl::store_backed(store, PersistMode::Sync));
    context
}

#[test]
fn get_blockchain_info_uses_durable_connected_block_height_not_downloaded_height() {
    // Arrange
    let path = temp_store_path("durable-sync-truth");
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .save_runtime_metadata(
            &RuntimeMetadata {
                maybe_sync_state: Some(DurableSyncState {
                    sync: SyncStatus {
                        network: FieldAvailability::available("main".to_string()),
                        chain_tip: FieldAvailability::available(ChainTipStatus {
                            height: 840_004,
                            block_hash: "00".repeat(32),
                        }),
                        sync_progress: FieldAvailability::available(SyncProgress {
                            header_height: 840_200,
                            block_height: 840_004,
                            downloaded_block_height: 840_006,
                            connected_block_height: 840_004,
                            validated_active_chain_height: 840_004,
                            maybe_downloaded_block_hash: Some("11".repeat(32)),
                            maybe_connected_block_hash: Some("00".repeat(32)),
                            maybe_validated_active_chain_hash: Some("00".repeat(32)),
                            maybe_validated_active_chain_work: Some("840005".to_string()),
                            progress_ratio: 840_004.0 / 840_200.0,
                            messages_processed: 42,
                            headers_received: 100,
                            blocks_received: 3,
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
                        progress_signal: FieldAvailability::available(
                            SyncProgressSignal::HeaderProgress,
                        ),
                        lag: FieldAvailability::available(SyncLagStatus {
                            headers_remaining: 0,
                            blocks_remaining: 100,
                        }),
                        last_successful_progress_unix_seconds: FieldAvailability::available(
                            1_715_000_000,
                        ),
                        progress_credit: FieldAvailability::unavailable(
                            "progress credit evidence unavailable",
                        ),
                        expected_progress_window: FieldAvailability::unavailable(
                            "expected progress window unavailable",
                        ),
                        no_progress_threshold: FieldAvailability::unavailable(
                            "no-progress threshold evidence unavailable",
                        ),
                        last_useful_work: FieldAvailability::unavailable(
                            "last useful work unavailable",
                        ),
                        last_peer_contribution: FieldAvailability::unavailable(
                            "last peer contribution unavailable",
                        ),
                        stall_diagnosis: FieldAvailability::unavailable(
                            "stall diagnosis unavailable",
                        ),
                        latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
                            label: "target_header_reached".to_string(),
                            message: "sync header target reached".to_string(),
                        }),
                        last_error: FieldAvailability::available(
                            "peer stalled before block connect".to_string(),
                        ),
                        recovery_category: FieldAvailability::available(
                            SyncRecoveryCategory::InvalidPeerData,
                        ),
                        recovery_action: FieldAvailability::available(
                            "Restart the node and retry the storage operation.".to_string(),
                        ),
                        resource_pressure: FieldAvailability::available(SyncResourcePressure {
                            blocks_in_flight: 8,
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
                        no_progress_diagnosis: FieldAvailability::unavailable(
                            "no-progress diagnosis unavailable",
                        ),
                        no_progress_next_action: FieldAvailability::unavailable(
                            "no-progress next action unavailable",
                        ),
                        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
                        reconcile_progress: FieldAvailability::unavailable(
                            "reconcile progress unavailable",
                        ),
                    },
                    peers: PeerStatus {
                        peer_counts: FieldAvailability::available(PeerCounts {
                            inbound: 0,
                            outbound: 2,
                        }),
                        recent_peers: FieldAvailability::available(Vec::new()),
                        inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                            INBOUND_STATUS_UNAVAILABLE_REASON,
                        ),
                    },
                    health_signals: Vec::new(),
                    updated_at_unix_seconds: 1_715_000_000,
                }),
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save runtime metadata");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen store");
    let reopened_metadata = reopened
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    assert!(reopened_metadata.maybe_sync_state.is_some());
    drop(reopened);
    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Mainnet,
        maybe_data_dir: Some(path),
        ..RuntimeConfig::default()
    });
    assert!(
        context
            .current_durable_sync_state()
            .expect("current durable sync state")
            .is_some()
    );

    // Act
    let blockchain = dispatch(
        &mut context,
        MethodCall::GetBlockchainInfo(GetBlockchainInfoRequest::default()),
    )
    .expect("blockchain");

    // Assert
    assert_eq!(blockchain["headers"], json!(840200));
    assert_eq!(blockchain["blocks"], json!(840004));
    assert_eq!(blockchain["initialblockdownload"], json!(true));
    assert_eq!(
        blockchain["warnings"][0],
        json!("peer stalled before block connect")
    );
    assert_eq!(
        blockchain["warnings"][1],
        json!("progress_signal=header_progress")
    );
    assert_eq!(
        blockchain["warnings"][2],
        json!("latest_stop_reason=target_header_reached")
    );
    assert_eq!(
        blockchain["warnings"][3],
        json!("recovery_category=invalid_peer_data")
    );
    assert_eq!(
        blockchain["warnings"][4],
        json!("Restart the node and retry the storage operation.")
    );
}

#[test]
fn open_bitcoin_sync_status_returns_phase72_durable_truth_contract() {
    // Arrange
    let mut context =
        context_with_runtime_metadata("sync-status-phase72", phase72_runtime_metadata());

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect("sync status");

    // Assert
    let sync = &status["metadata"]["maybe_sync_state"]["sync"];
    assert_eq!(
        sync["sync_progress"]["value"]["validated_active_chain_height"],
        json!(840_004)
    );
    assert_eq!(
        sync["sync_progress"]["value"]["maybe_validated_active_chain_hash"],
        json!("11".repeat(32))
    );
    assert_eq!(
        sync["sync_progress"]["value"]["maybe_validated_active_chain_work"],
        json!("840005")
    );
    assert_eq!(sync["best_known_tip"]["state"], json!("available"));
    assert_eq!(sync["best_known_tip"]["value"]["freshness"], json!("fresh"));
    assert_eq!(
        sync["stay_current"]["value"],
        json!("current_at_best_known_tip")
    );
    assert_eq!(
        sync["no_progress_diagnosis"]["value"],
        json!("peer_backoff")
    );
    assert_eq!(
        sync["latest_reorg"]["value"]["final_active_height"],
        json!(840_004)
    );
    assert_eq!(
        sync["reconcile_progress"]["value"]["state"],
        json!("extended_active_chain")
    );
    assert_eq!(sync["resource_pressure"]["state"], json!("available"));
}

#[test]
fn get_blockchain_info_does_not_expose_phase72_support_fields() {
    // Arrange
    let mut context =
        context_with_runtime_metadata("blockchain-info-phase72", phase72_runtime_metadata());

    // Act
    let blockchain = dispatch(
        &mut context,
        MethodCall::GetBlockchainInfo(GetBlockchainInfoRequest::default()),
    )
    .expect("blockchain");
    let serialized = serde_json::to_string(&blockchain).expect("serialize blockchain info");

    // Assert
    for forbidden in [
        "best_known_tip",
        "stay_current",
        "latest_reorg",
        "reconcile_progress",
        "resource_pressure",
        "support_evidence",
        "evidence_verdict",
        "validated_active_chain_work",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "baseline getblockchaininfo exposed {forbidden}"
        );
    }
}

#[test]
fn open_bitcoin_sync_status_returns_phase62_metadata_fields() {
    // Arrange
    let path = temp_store_path("sync-status-phase62");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .save_runtime_metadata(&phase62_runtime_metadata(), PersistMode::Sync)
        .expect("save metadata");
    let mut context = empty_context();
    context.set_daemon_sync_control(DaemonSyncControl::store_backed(store, PersistMode::Sync));

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect("sync status");

    // Assert
    assert_eq!(
        status["metadata"]["maybe_sync_state"]["sync"]["configured_targets"]["value"]["target_outbound_peers"],
        json!(4)
    );
    assert_eq!(
        status["metadata"]["maybe_sync_state"]["sync"]["attempt_counters"]["value"]["attempted_peers"],
        json!(3)
    );
    assert_eq!(
        status["metadata"]["maybe_sync_state"]["sync"]["latest_stop_reason"]["value"]["label"],
        json!("target_header_reached")
    );
}
