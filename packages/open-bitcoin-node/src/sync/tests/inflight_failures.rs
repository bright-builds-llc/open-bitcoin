// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn block_inflight_invalid_block_releases_runtime_and_peer_inflight_for_retry() {
    // Arrange
    let path = temp_store_path("block-inflight-invalid");
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
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(valid_block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert!(summary.peer_outcomes.iter().any(|outcome| {
        outcome.maybe_failure_reason == Some(PeerFailureReason::InvalidBlock)
            && outcome.contribution.blocks_received == 0
    }));
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(
        runtime
            .store()
            .load_block(block_hash)
            .expect("load invalid block")
            .is_none()
    );
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn block_inflight_malformed_block_releases_runtime_and_peer_inflight_for_retry() {
    // Arrange
    let path = temp_store_path("block-inflight-malformed");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let first_peer_script = headers_script(0, vec![block.header.clone()]);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
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
    assert_eq!(summary.failed_peers, 1);
    assert!(summary.peer_outcomes.iter().any(|outcome| {
        outcome.maybe_failure_reason == Some(PeerFailureReason::MalformedBlock)
            && outcome.contribution.blocks_received == 0
    }));
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(1).is_err());
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);

    remove_dir_if_exists(&path);
}
