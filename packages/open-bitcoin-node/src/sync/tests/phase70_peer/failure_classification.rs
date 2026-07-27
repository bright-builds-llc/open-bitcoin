// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase78_validation_stall_classifies_validation_subsystem() {
    // Arrange
    let path = temp_store_path("phase78-validation-stall");
    remove_dir_if_exists(&path);
    let valid_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let mut invalid_block = valid_block.clone();
    invalid_block.transactions[0].outputs[0].value = Amount::from_sats(51).expect("valid amount");
    let first_peer_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![valid_block.header.clone()],
        }),
        WireNetworkMessage::Block(invalid_block),
    ];
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(valid_block.header.time))
        .expect("sync with invalid block");
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            None,
            i64::from(valid_block.header.time),
        )
        .expect("durable validation stall status");

    // Assert
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::InvalidBlock)
    );
    assert_progress_credit_unavailable(&state);
    let stall = available_stall_diagnosis(&state);
    assert_eq!(serialized_label(stall.stalled_subsystem), "validation");
    assert_eq!(stall.stalled_subsystem, StalledSubsystem::Validation);
    assert_eq!(
        stall.evidence_basis,
        vec![
            "no_progress_diagnosis=BehindAwaitingHeaders".to_string(),
            "recovery_category=invalid_peer_data".to_string(),
            "peer_failure_reason=invalid_block".to_string(),
        ]
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_malformed_block_releases_inflight_and_rotates() {
    // Arrange
    let path = temp_store_path("phase70-peer-malformed-rotation");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let first_peer_script = headers_script(0, vec![block.header.clone()]);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
    let mut transport = ErrorAfterMessagesTransport::new(
        vec![first_peer_script, version_verack_script(0)],
        SyncRuntimeError::Network {
            message: "malformed block payload".to_string(),
        },
        1,
    );

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_reason_without_block_credit(&summary, PeerFailureReason::MalformedBlock);
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(1).is_err());
    assert_first_peer_backoff(&runtime);

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_invalid_block_releases_inflight_and_rotates() {
    // Arrange
    let path = temp_store_path("phase70-peer-invalid-rotation");
    remove_dir_if_exists(&path);
    let valid_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&valid_block.header);
    let mut invalid_block = valid_block.clone();
    invalid_block.transactions[0].outputs[0].value = Amount::from_sats(51).expect("valid amount");
    let first_peer_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![valid_block.header.clone()],
        }),
        WireNetworkMessage::Block(invalid_block),
    ];
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(valid_block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_reason_without_block_credit(&summary, PeerFailureReason::InvalidBlock);
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(1).is_err());
    assert_first_peer_backoff(&runtime);

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_duplicate_block_releases_inflight_without_credit() {
    // Arrange
    let path = temp_store_path("phase70-peer-duplicate-no-credit");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    runtime.inflight_blocks.insert(child_hash);
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
    assert_reason_without_block_credit(&summary, PeerFailureReason::DuplicateBlock);
    assert!(runtime.inflight_blocks.is_empty());
    assert_first_peer_backoff(&runtime);

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_disconnected_block_releases_inflight_without_credit() {
    // Arrange
    let path = temp_store_path("phase70-peer-disconnected-no-credit");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([7_u8; 32]), 1);
    let block_hash = block_hash(&block.header);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    runtime.inflight_blocks.insert(block_hash);
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Block(block.clone()),
    ]]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("sync");

    // Assert
    assert_reason_without_block_credit(&summary, PeerFailureReason::DisconnectedBlock);
    assert!(runtime.inflight_blocks.is_empty());
    assert_first_peer_backoff(&runtime);

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_non_extending_block_releases_inflight_without_credit() {
    // Arrange
    let path = temp_store_path("phase70-peer-non-extending-no-credit");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let side_block = build_block(BlockHash::from_byte_array([42_u8; 32]), 1);
    let side_hash = block_hash(&side_block.header);
    save_best_chain_with_active_blocks(&path, &[(&genesis, 0)], &[(&genesis, 0)]);
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    runtime.inflight_blocks.insert(side_hash);
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Block(side_block.clone()),
    ]]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(side_block.header.time))
        .expect("sync");

    // Assert
    assert_reason_without_block_credit(&summary, PeerFailureReason::NonExtendingBlock);
    assert!(runtime.inflight_blocks.is_empty());
    assert_first_peer_backoff(&runtime);

    remove_dir_if_exists(&path);
}
