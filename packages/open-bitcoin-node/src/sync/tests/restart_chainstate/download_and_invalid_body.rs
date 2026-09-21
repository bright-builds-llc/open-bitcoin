// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn same_datadir_reopen_reports_downloaded_and_connected_block_hashes_after_partial_download() {
    // Arrange
    let path = temp_store_path("restart-partial-download-status");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child_one = build_block(block_hash(&genesis.header), 1);
    let child_two = build_block(block_hash(&child_one.header), 2);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    HeaderEntry {
                        block_hash: block_hash(&genesis.header),
                        header: genesis.header.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&child_one.header),
                        header: child_one.header.clone(),
                        height: 1,
                        chain_work: 2,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&child_two.header),
                        header: child_two.header.clone(),
                        height: 2,
                        chain_work: 3,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save headers");
        store
            .save_block(&genesis, PersistMode::Sync)
            .expect("save genesis");
        store
            .save_block(&child_one, PersistMode::Sync)
            .expect("save child one");
    }

    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(2)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(child_two.header.time))
        .expect("sync after restart");

    // Assert
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(summary.downloaded_block_height, 1);
    assert_eq!(summary.best_block_height, 1);
    assert_eq!(
        summary.maybe_downloaded_block_hash,
        Some(block_hash_hex(block_hash(&child_one.header)))
    );
    assert_eq!(
        summary.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&child_one.header)))
    );
    assert_eq!(
        summary.sync_status(SyncNetwork::Regtest).sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 2,
            block_height: 1,
            downloaded_block_height: 1,
            connected_block_height: 1,
            validated_active_chain_height: 1,
            maybe_downloaded_block_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_connected_block_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_validated_active_chain_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_validated_active_chain_work: Some("2".to_string()),
            progress_ratio: 0.5,
            messages_processed: 2,
            headers_received: 0,
            blocks_received: 0,
        })
    );
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetData(_)))
    );
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let durable_progress = metadata
        .maybe_sync_state
        .expect("durable sync state")
        .sync
        .sync_progress;
    assert_eq!(
        durable_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 2,
            block_height: 1,
            downloaded_block_height: 1,
            connected_block_height: 1,
            validated_active_chain_height: 1,
            maybe_downloaded_block_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_connected_block_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_validated_active_chain_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_validated_active_chain_work: Some("2".to_string()),
            progress_ratio: 0.5,
            messages_processed: 2,
            headers_received: 0,
            blocks_received: 0,
        })
    );

    remove_dir_if_exists(&path);
}

#[test]
fn invalid_block_body_is_peer_attributed_and_not_persisted() {
    // Arrange
    let path = temp_store_path("invalid-block-body");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let genesis_hash = block_hash(&genesis.header);
    let mut invalid_genesis = genesis.clone();
    invalid_genesis.transactions[0].outputs[0].value = Amount::from_sats(51).expect("valid amount");
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone()],
        }),
        WireNetworkMessage::Block(invalid_genesis),
    ];
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![script]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(genesis.header.time))
        .expect("sync records peer failure");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.connected_peers, 0);
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Failed);
    assert_eq!(
        outcome.maybe_failure_reason,
        Some(PeerFailureReason::InvalidBlock)
    );
    assert_eq!(outcome.contribution.headers_received, 1);
    assert_eq!(outcome.contribution.blocks_received, 0);
    assert!(
        outcome
            .maybe_error
            .as_ref()
            .is_some_and(|message| message.contains("invalid data"))
    );
    assert!(
        runtime
            .store()
            .load_block(genesis_hash)
            .expect("load rejected block")
            .is_none()
    );
    assert!(
        runtime
            .store()
            .load_chainstate_snapshot()
            .expect("leftover unread")
            .is_none(),
        "persist_progress must not write leftover snapshots"
    );
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let durable_state = metadata.maybe_sync_state.expect("durable sync state");
    assert_eq!(
        durable_state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Active)
    );
    assert!(matches!(
        durable_state.sync.last_error,
        FieldAvailability::Available(ref value) if value.contains("invalid data")
    ));
    assert!(matches!(
        durable_state.sync.recovery_action,
        FieldAvailability::Available(ref value) if value.contains("different peer")
    ));

    remove_dir_if_exists(&path);
}
