// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn block_notfound_is_peer_attributed_no_credit() {
    // Arrange
    let path = temp_store_path("block-response-notfound");
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
        notfound_for_block(child_hash),
    ]]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(child.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == child_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);
    assert_peer_reason_without_block_credit(&summary, PeerFailureReason::BlockNotFound);
    assert!(
        runtime
            .store()
            .load_block(child_hash)
            .expect("load missing child")
            .is_none()
    );

    remove_dir_if_exists(&path);
}

#[test]
fn duplicate_block_response_is_peer_attributed_no_credit() {
    // Arrange
    let path = temp_store_path("block-response-duplicate");
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
    let active_chain = runtime
        .network
        .chainstate_snapshot()
        .expect("authoritative chainstate snapshot")
        .active_chain;

    // Assert
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(summary.downloaded_block_height, 1);
    assert_eq!(summary.best_block_height, 1);
    assert_eq!(active_chain.len(), 2);
    assert_eq!(
        active_chain.last().map(|position| position.block_hash),
        Some(child_hash)
    );
    assert_peer_reason_without_block_credit(&summary, PeerFailureReason::DuplicateBlock);

    remove_dir_if_exists(&path);
}

#[test]
fn disconnected_block_response_is_peer_attributed_no_credit() {
    // Arrange
    let path = temp_store_path("block-response-disconnected");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([7_u8; 32]), 1);
    let block_hash = block_hash(&block.header);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
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
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);
    assert!(
        runtime
            .network
            .chainstate_snapshot()
            .expect("authoritative chainstate snapshot")
            .active_chain
            .is_empty()
    );
    assert_peer_reason_without_block_credit(&summary, PeerFailureReason::DisconnectedBlock);
    assert!(
        runtime
            .store()
            .load_block(block_hash)
            .expect("load disconnected block")
            .is_none()
    );

    remove_dir_if_exists(&path);
}

#[test]
fn non_extending_block_response_is_peer_attributed_no_credit() {
    // Arrange
    let path = temp_store_path("block-response-non-extending");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let side_block = build_block(BlockHash::from_byte_array([42_u8; 32]), 1);
    let side_hash = block_hash(&side_block.header);
    save_best_chain_with_active_blocks(&path, &[(&genesis, 0)], &[(&genesis, 0)]);
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
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
    assert_peer_reason_without_block_credit(&summary, PeerFailureReason::NonExtendingBlock);
    assert!(
        runtime
            .store()
            .load_block(side_hash)
            .expect("load non-extending block")
            .is_none()
    );

    remove_dir_if_exists(&path);
}
