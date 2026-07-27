// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn first_non_genesis_block_connect_advances_downloaded_and_connected_height() {
    // Arrange
    let path = temp_store_path("block-response-first-connect");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Block(child.clone()),
    ]]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(child.header.time))
        .expect("sync");

    // Assert
    assert_eq!(summary.downloaded_block_height, 1);
    assert_eq!(summary.best_block_height, 1);
    assert_eq!(summary.blocks_received, 1);
    assert_eq!(summary.peer_outcomes.len(), 1);
    assert_eq!(summary.peer_outcomes[0].contribution.blocks_received, 1);
    assert!(
        runtime
            .store()
            .load_block(child_hash)
            .expect("load connected child")
            .is_some()
    );

    remove_dir_if_exists(&path);
}

#[test]
fn connected_active_chain_progress_survives_runtime_reopen() {
    // Arrange
    let path = temp_store_path("block-response-connected-reopen");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    let expected_child_hash = block_hash_hex(child_hash);
    save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Block(child.clone()),
    ]]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(child.header.time))
        .expect("sync");
    let connected_progress = summary.sync_status(SyncNetwork::Regtest).sync_progress;
    drop(runtime);

    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let reopened_runtime =
        DurableSyncRuntime::open(reopened_store, sync_config()).expect("reopen runtime");
    let reopened_summary = reopened_runtime.snapshot_summary();
    let reopened_state = reopened_runtime
        .durable_sync_state_for_summary(
            &reopened_summary,
            SyncLifecycleState::Active,
            None,
            1_777_225_182,
        )
        .expect("reopened durable status");

    // Assert
    assert_eq!(summary.downloaded_block_height, 1);
    assert_eq!(summary.best_block_height, 1);
    assert_eq!(
        connected_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 1,
            block_height: 1,
            downloaded_block_height: 1,
            connected_block_height: 1,
            validated_active_chain_height: 1,
            maybe_downloaded_block_hash: Some(expected_child_hash.clone()),
            maybe_connected_block_hash: Some(expected_child_hash.clone()),
            maybe_validated_active_chain_hash: Some(expected_child_hash.clone()),
            maybe_validated_active_chain_work: Some("2".to_string()),
            progress_ratio: 1.0,
            messages_processed: 3,
            headers_received: 0,
            blocks_received: 1,
        })
    );
    assert_eq!(reopened_summary.best_block_height, 1);
    assert_eq!(reopened_summary.downloaded_block_height, 1);
    assert_eq!(
        reopened_summary.maybe_connected_block_hash,
        Some(expected_child_hash.clone())
    );
    assert_eq!(
        reopened_summary.maybe_validated_active_chain_work,
        Some("2".to_string())
    );
    assert_eq!(
        reopened_state.sync.sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 1,
            block_height: 1,
            downloaded_block_height: 1,
            connected_block_height: 1,
            validated_active_chain_height: 1,
            maybe_downloaded_block_hash: Some(expected_child_hash.clone()),
            maybe_connected_block_hash: Some(expected_child_hash.clone()),
            maybe_validated_active_chain_hash: Some(expected_child_hash),
            maybe_validated_active_chain_work: Some("2".to_string()),
            progress_ratio: 1.0,
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        })
    );
    assert!(
        reopened_runtime
            .store()
            .load_block(child_hash)
            .expect("load reopened child")
            .is_some()
    );
    let snapshot = reopened_runtime
        .store()
        .load_chainstate_snapshot()
        .expect("load chainstate snapshot")
        .expect("chainstate snapshot");
    let active_tip = snapshot.active_chain.last().expect("active tip");
    assert_eq!(active_tip.height, 1);
    assert_eq!(active_tip.block_hash, child_hash);
    assert_eq!(active_tip.chain_work, 2);

    remove_dir_if_exists(&path);
}

#[test]
fn unrequested_extending_block_response_is_no_credit_and_does_not_mutate_chainstate() {
    // Arrange
    let path = temp_store_path("block-response-unrequested-extending");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(&path, &[(&genesis, 0)], &[(&genesis, 0)]);
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Block(child),
    ]]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(genesis.header.time))
        .expect("sync");
    let active_chain = runtime
        .network
        .chainstate_snapshot()
        .expect("authoritative chainstate snapshot")
        .active_chain;

    // Assert
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);
    assert_eq!(active_chain.len(), 1);
    assert_eq!(
        active_chain.last().map(|position| position.block_hash),
        Some(block_hash(&genesis.header))
    );
    assert_peer_reason_without_block_credit(&summary, PeerFailureReason::DisconnectedBlock);
    assert!(
        runtime
            .store()
            .load_block(child_hash)
            .expect("load unrequested child")
            .is_none()
    );

    remove_dir_if_exists(&path);
}

#[test]
fn sync_progress_reports_downloaded_and_connected_block_hashes() {
    // Arrange
    let path = temp_store_path("sync-progress-connected-hashes");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Block(child.clone()),
    ]]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(child.header.time))
        .expect("sync");
    let sync_progress = summary.sync_status(SyncNetwork::Regtest).sync_progress;

    // Assert
    assert_eq!(
        sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 1,
            block_height: 1,
            downloaded_block_height: 1,
            connected_block_height: 1,
            validated_active_chain_height: 1,
            maybe_downloaded_block_hash: Some(block_hash_hex(child_hash)),
            maybe_connected_block_hash: Some(block_hash_hex(child_hash)),
            maybe_validated_active_chain_hash: Some(block_hash_hex(child_hash)),
            maybe_validated_active_chain_work: Some("2".to_string()),
            progress_ratio: 1.0,
            messages_processed: 3,
            headers_received: 0,
            blocks_received: 1,
        })
    );

    remove_dir_if_exists(&path);
}

#[test]
fn sync_progress_reports_downloaded_only_block_hash() {
    // Arrange
    let path = temp_store_path("sync-progress-downloaded-only-hash");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let genesis_hash = block_hash(&genesis.header);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_block(&child, PersistMode::Sync)
            .expect("save downloaded child");
    }

    let store = FjallNodeStore::open(&path).expect("reopen store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime.snapshot_summary();
    let status = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_180)
        .expect("durable status");

    // Assert
    assert_eq!(
        status.sync.sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 1,
            block_height: 0,
            downloaded_block_height: 1,
            connected_block_height: 0,
            validated_active_chain_height: 0,
            maybe_downloaded_block_hash: Some(block_hash_hex(child_hash)),
            maybe_connected_block_hash: Some(block_hash_hex(genesis_hash)),
            maybe_validated_active_chain_hash: Some(block_hash_hex(genesis_hash)),
            maybe_validated_active_chain_work: Some("1".to_string()),
            progress_ratio: 0.0,
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        })
    );

    remove_dir_if_exists(&path);
}
