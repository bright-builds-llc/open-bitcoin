// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;
use crate::sync::block_reconcile;

#[test]
fn bounded_block_requests_use_validated_best_chain_headers_only() {
    // Arrange
    let path = temp_store_path("bounded-best-chain-requests");
    remove_dir_if_exists(&path);
    let active_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let requestable_block = build_block(block_hash(&active_block.header), 1);
    let durable_local_block = build_block(block_hash(&requestable_block.header), 2);
    let inflight_block = build_block(block_hash(&durable_local_block.header), 3);
    let unvalidated_block = build_block(BlockHash::from_byte_array([42_u8; 32]), 99);
    let active_hash = block_hash(&active_block.header);
    let requestable_hash = block_hash(&requestable_block.header);
    let durable_local_hash = block_hash(&durable_local_block.header);
    let inflight_hash = block_hash(&inflight_block.header);
    let unvalidated_hash = block_hash(&unvalidated_block.header);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    HeaderEntry {
                        block_hash: active_hash,
                        header: active_block.header.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    HeaderEntry {
                        block_hash: requestable_hash,
                        header: requestable_block.header.clone(),
                        height: 1,
                        chain_work: 2,
                    },
                    HeaderEntry {
                        block_hash: durable_local_hash,
                        header: durable_local_block.header.clone(),
                        height: 2,
                        chain_work: 3,
                    },
                    HeaderEntry {
                        block_hash: inflight_hash,
                        header: inflight_block.header.clone(),
                        height: 3,
                        chain_work: 4,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save best-chain headers");
        store
            .save_chainstate_snapshot(
                &ChainstateSnapshot::new(
                    vec![ChainPosition::new(
                        active_block.header.clone(),
                        0,
                        1,
                        i64::from(active_block.header.time),
                    )],
                    Default::default(),
                    Default::default(),
                ),
                PersistMode::Sync,
            )
            .expect("save active chain snapshot");
        store
            .save_block(&durable_local_block, PersistMode::Sync)
            .expect("save durable local block");
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    runtime.inflight_blocks.insert(inflight_hash);
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 3,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: unvalidated_hash.into(),
        }])),
    ]]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(inflight_block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert_eq!(summary.best_header_height, 3);
    assert_eq!(requested_hashes, vec![requestable_hash]);
    assert!(!requested_hashes.contains(&active_hash));
    assert!(!requested_hashes.contains(&durable_local_hash));
    assert!(!requested_hashes.contains(&inflight_hash));
    assert!(!requested_hashes.contains(&unvalidated_hash));
    assert!(runtime.inflight_blocks.contains(&inflight_hash));

    remove_dir_if_exists(&path);
}

#[test]
fn bounded_block_requests_respect_per_peer_and_total_caps() {
    // Arrange
    let path = temp_store_path("bounded-request-caps");
    remove_dir_if_exists(&path);
    let first = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let second = build_block(block_hash(&first.header), 1);
    let third = build_block(block_hash(&second.header), 2);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    HeaderEntry {
                        block_hash: block_hash(&first.header),
                        header: first.header.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&second.header),
                        header: second.header.clone(),
                        height: 1,
                        chain_work: 2,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&third.header),
                        header: third.header.clone(),
                        height: 2,
                        chain_work: 3,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save headers");
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_blocks_in_flight_per_peer: 1,
            max_blocks_in_flight_total: 2,
            ..sync_config()
        },
    )
    .expect("runtime");
    for peer_id in [1, 2, 3] {
        runtime
            .network
            .connect_outbound_peer(peer_id, 1_777_225_210)
            .expect("connect peer");
        runtime
            .network
            .receive_sync_message(
                peer_id,
                WireNetworkMessage::Version(VersionMessage {
                    start_height: 2,
                    ..VersionMessage::default()
                }),
                1_777_225_210,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("receive version");
        runtime
            .network
            .receive_sync_message(
                peer_id,
                WireNetworkMessage::Verack,
                1_777_225_210,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("receive verack");
    }

    // Act
    let first_peer_messages =
        block_reconcile::request_missing_blocks(&mut runtime, 1).expect("peer one request");
    let second_peer_messages =
        block_reconcile::request_missing_blocks(&mut runtime, 2).expect("peer two request");
    let first_peer_retry =
        block_reconcile::request_missing_blocks(&mut runtime, 1).expect("peer one retry");
    let third_peer_messages = block_reconcile::request_missing_blocks(&mut runtime, 3)
        .expect("peer three request");

    // Assert
    assert_eq!(getdata_block_hashes(&first_peer_messages).len(), 1);
    assert_eq!(getdata_block_hashes(&second_peer_messages).len(), 1);
    assert!(getdata_block_hashes(&first_peer_retry).is_empty());
    assert!(getdata_block_hashes(&third_peer_messages).is_empty());
    assert_eq!(runtime.inflight_blocks.len(), 2);
    assert_eq!(
        runtime
            .network
            .peer_requested_blocks(1)
            .expect("peer one requested blocks")
            .len(),
        1
    );
    assert_eq!(
        runtime
            .network
            .peer_requested_blocks(2)
            .expect("peer two requested blocks")
            .len(),
        1
    );
    assert!(runtime.inflight_blocks.len() <= runtime.config.max_blocks_in_flight_total);

    remove_dir_if_exists(&path);
}

#[test]
fn phase110_block_serving_cleanup_never_exceeds_total_inflight_limit() {
    // Arrange
    let path = temp_store_path("phase110-block-serving-cleanup-total-limit");
    remove_dir_if_exists(&path);
    let first = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let second = build_block(block_hash(&first.header), 1);
    let third = build_block(block_hash(&second.header), 2);
    let fourth = build_block(block_hash(&third.header), 3);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    HeaderEntry {
                        block_hash: block_hash(&first.header),
                        header: first.header.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&second.header),
                        header: second.header.clone(),
                        height: 1,
                        chain_work: 2,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&third.header),
                        header: third.header.clone(),
                        height: 2,
                        chain_work: 3,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&fourth.header),
                        header: fourth.header.clone(),
                        height: 3,
                        chain_work: 4,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save headers");
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_blocks_in_flight_per_peer: 2,
            max_blocks_in_flight_total: 3,
            ..sync_config()
        },
    )
    .expect("runtime");
    for peer_id in [110, 111, 112] {
        connect_runtime_peer(&mut runtime, peer_id, 3);
    }

    // Act
    let first_peer_messages =
        block_reconcile::request_missing_blocks(&mut runtime, 110).expect("peer one");
    let second_peer_messages =
        block_reconcile::request_missing_blocks(&mut runtime, 111).expect("peer two");
    let third_peer_messages =
        block_reconcile::request_missing_blocks(&mut runtime, 112).expect("peer three");

    // Assert
    assert_eq!(getdata_block_hashes(&first_peer_messages).len(), 2);
    assert_eq!(getdata_block_hashes(&second_peer_messages).len(), 1);
    assert!(getdata_block_hashes(&third_peer_messages).is_empty());
    assert_eq!(runtime.inflight_blocks.len(), 3);
    assert!(runtime.inflight_blocks.len() <= runtime.config.max_blocks_in_flight_total);

    remove_dir_if_exists(&path);
}

#[test]
fn phase110_block_serving_cleanup_releases_block_and_notfound_inflight() {
    // Arrange
    let path = temp_store_path("phase110-block-serving-cleanup-release-message");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let notfound_hash = BlockHash::from_byte_array([110_u8; 32]);

    // Act
    runtime.inflight_blocks.insert(block_hash);
    block_reconcile::release_inflight_for_message(
        &mut runtime,
        &WireNetworkMessage::Block(block),
    );
    runtime.inflight_blocks.insert(notfound_hash);
    block_reconcile::release_inflight_for_message(
        &mut runtime,
        &notfound_for_block(notfound_hash),
    );

    // Assert
    assert!(runtime.inflight_blocks.is_empty());

    remove_dir_if_exists(&path);
}

#[test]
fn notfound_releases_block_inflight_for_retry() {
    // Arrange
    let path = temp_store_path("block-inflight-notfound");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let first_peer_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![block.header.clone()],
        }),
        notfound_for_block(block_hash),
    ];
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);
    assert!(
        runtime
            .store()
            .load_block(block_hash)
            .expect("load notfound block")
            .is_none()
    );

    remove_dir_if_exists(&path);
}

#[test]
fn disconnect_clears_runtime_and_peer_block_inflight() {
    // Arrange
    let path = temp_store_path("block-inflight-disconnect");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let first_peer_script = headers_script(0, vec![block.header.clone()]);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
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

#[test]
fn duplicate_outbox_registration_does_not_disconnect_an_existing_peer() {
    // Arrange
    let path = temp_store_path("duplicate-outbox-registration-cleanup");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let peer_id = 128_402;
    connect_runtime_peer(&mut runtime, peer_id, 0);
    runtime
        .announcement_outboxes
        .register_peer(peer_id)
        .expect("register existing peer outbox");
    let session = ScriptedSession {
        inbound: VecDeque::new(),
        sent: Rc::new(RefCell::new(Vec::new())),
    };
    let peer = resolved_manual_peer("127.0.0.1", 18_444);
    let mut clock = || 1_777_225_211;

    // Act
    let failure = runtime
        .sync_connected_peer(session, &peer, peer_id, 1, 1_777_225_210, &mut clock)
        .expect_err("duplicate outbox ownership should reject the session");
    let snapshots = runtime
        .announcement_outboxes
        .snapshots()
        .expect("outbox snapshots");

    // Assert
    assert!(matches!(
        failure.error,
        SyncRuntimeError::Network { ref message }
            if message.contains("already registered")
    ));
    assert!(runtime.network.peer_requested_blocks(peer_id).is_ok());
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].peer_id(), peer_id);

    remove_dir_if_exists(&path);
}

#[test]
fn phase110_block_serving_cleanup_disconnect_releases_peer_and_runtime_inflight() {
    // Arrange
    let path = temp_store_path("phase110-block-serving-cleanup-disconnect");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let first_peer_script = headers_script(0, vec![block.header.clone()]);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(1).is_err());
    assert_eq!(summary.best_block_height, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn phase110_block_serving_cleanup_reopen_starts_without_stale_inflight() {
    // Arrange
    let path = temp_store_path("phase110-block-serving-cleanup-stale-reopen");
    remove_dir_if_exists(&path);
    let stale_hash = BlockHash::from_byte_array([111_u8; 32]);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        connect_runtime_peer(&mut runtime, 110, 0);
        runtime.inflight_blocks.insert(stale_hash);
        assert!(runtime.network.peer_requested_blocks(110).is_ok());
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");

    // Act
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("reopen runtime");

    // Assert
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(110).is_err());

    remove_dir_if_exists(&path);
}
