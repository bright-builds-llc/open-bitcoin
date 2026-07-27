// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn scripted_headers_sync_persists_progress_and_status() {
    // Arrange
    let path = temp_store_path("headers");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let child = header(block_hash(&genesis), 2);
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.clone(), child.clone()],
        }),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_022)
        .expect("sync");

    // Assert
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Connected);
    assert_eq!(summary.headers_received, 2);
    assert_eq!(summary.best_header_height, 1);
    assert_eq!(summary.best_block_height, 0);
    assert_eq!(
        summary
            .sync_status(SyncNetwork::Regtest)
            .sync_progress
            .clone(),
        crate::FieldAvailability::available(SyncProgress {
            header_height: 1,
            block_height: 0,
            downloaded_block_height: 0,
            connected_block_height: 0,
            validated_active_chain_height: 0,
            maybe_downloaded_block_hash: None,
            maybe_connected_block_hash: None,
            maybe_validated_active_chain_hash: None,
            maybe_validated_active_chain_work: None,
            progress_ratio: 0.0,
            messages_processed: 3,
            headers_received: 2,
            blocks_received: 0,
        })
    );
    assert_eq!(
        runtime
            .store()
            .load_header_entries()
            .expect("load headers")
            .expect("headers")
            .entries
            .len(),
        2
    );
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| { matches!(message, WireNetworkMessage::GetHeaders { .. }) })
    );
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetData(_)))
    );

    remove_dir_if_exists(&path);
}

#[test]
fn sync_until_idle_continues_equal_message_rounds_when_heights_advance() {
    // Arrange
    let path = temp_store_path("until-idle-progress");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 21);
    let child = header(block_hash(&genesis), 22);
    let grandchild = header(block_hash(&child), 23);
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 4,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        headers_script(0, vec![genesis]),
        headers_script(1, vec![child]),
        headers_script(2, vec![grandchild]),
        Vec::new(),
    ]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, 1_777_225_155)
        .expect("sync until idle");

    // Assert
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(runtime.snapshot_summary().best_header_height, 2);

    remove_dir_if_exists(&path);
}

#[test]
fn sync_until_idle_stops_at_configured_header_target_after_multiple_batches() {
    // Arrange
    let path = temp_store_path("until-idle-header-target");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 31);
    let child = header(block_hash(&genesis), 32);
    let grandchild = header(block_hash(&child), 33);
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            maybe_target_header_height: Some(2),
            max_rounds: 5,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        headers_script(0, vec![genesis]),
        headers_script(1, vec![child]),
        headers_script(2, vec![grandchild]),
        headers_script(3, Vec::new()),
    ]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, 1_777_225_156)
        .expect("sync until target");
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_156)
        .expect("durable status");

    // Assert
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::TargetHeaderReached {
            target_header_height: 2,
            best_header_height: 2,
        })
    );
    assert!(summary.health_signals.iter().any(|signal| {
        signal.level == HealthSignalLevel::Info
            && signal.message.contains("sync header target reached")
    }));
    assert_eq!(
        state.sync.phase,
        FieldAvailability::available("header_target_reached".to_string())
    );

    remove_dir_if_exists(&path);
}

#[test]
fn sync_until_idle_records_no_progress_diagnosis_without_public_network() {
    // Arrange
    let path = temp_store_path("until-idle-no-progress");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 4,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        version_verack_script(0),
        version_verack_script(0),
        version_verack_script(0),
    ]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, 1_777_225_157)
        .expect("sync until no progress");
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_157)
        .expect("durable status");

    // Assert
    assert_eq!(summary.best_header_height, 0);
    assert_eq!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::NoProgress {
            rounds_completed: 2,
        })
    );
    assert!(summary.health_signals.iter().any(|signal| {
        signal.level == HealthSignalLevel::Warn
            && signal.message.contains("no new header or block progress")
    }));
    assert_eq!(
        state.sync.phase,
        FieldAvailability::available("no_progress".to_string())
    );

    remove_dir_if_exists(&path);
}
