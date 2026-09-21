// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;
use open_bitcoin_core::chainstate::CoinsView;

#[test]
fn competing_header_branch_wins_after_restart_when_it_extends_farther() {
    // Arrange
    let path = temp_store_path("header-fork");
    remove_dir_if_exists(&path);
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let branch_a_one = header(block_hash(&genesis), 2);
    let branch_a_two = header(block_hash(&branch_a_one), 3);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 2,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![genesis.clone(), branch_a_one, branch_a_two],
            }),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, 1_777_225_188)
            .expect("initial branch imports");
    }

    // Act
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let branch_b_one = header(block_hash(&genesis), 4);
    let branch_b_two = header(block_hash(&branch_b_one), 5);
    let branch_b_three = header(block_hash(&branch_b_two), 6);
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 3,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![branch_b_one, branch_b_two, branch_b_three],
        }),
    ]]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_199)
        .expect("fork extends");

    // Assert
    assert_eq!(summary.best_header_height, 3);
    assert_eq!(runtime.snapshot_summary().best_header_height, 3);
    assert_eq!(
        runtime
            .store()
            .load_header_entries()
            .expect("load headers")
            .expect("headers")
            .entries
            .len(),
        6
    );

    remove_dir_if_exists(&path);
}

#[test]
fn same_datadir_reopen_does_not_duplicate_connected_block_getdata() {
    // Arrange
    let path = temp_store_path("restart-block-reconnect");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let genesis_hash = block_hash(&genesis.header);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[open_bitcoin_network::HeaderEntry {
                    block_hash: genesis_hash,
                    header: genesis.header.clone(),
                    height: 0,
                    chain_work: 1,
                }],
                PersistMode::Sync,
            )
            .expect("save headers");
        store
            .save_block(&genesis, PersistMode::Sync)
            .expect("save block");
    }

    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(genesis.header.time))
        .expect("sync");

    // Assert
    assert_eq!(summary.best_header_height, 0);
    assert_eq!(summary.best_block_height, 0);
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(runtime.snapshot_summary().best_block_height, 0);
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());
    assert!(!requested_hashes.contains(&genesis_hash));
    assert!(requested_hashes.is_empty());
    let durable_summary = runtime.snapshot_summary();
    assert_eq!(durable_summary.best_block_height, 0);
    assert_eq!(
        durable_summary.maybe_connected_block_hash,
        Some(block_hash_hex(genesis_hash))
    );

    remove_dir_if_exists(&path);
}

mod download_and_invalid_body;
mod persist_and_hydrate;

#[test]
fn same_datadir_reopen_connects_best_available_branch_when_blocks_are_already_local() {
    // Arrange
    let path = temp_store_path("restart-branch-reorg");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let branch_a_one = build_block(block_hash(&genesis.header), 1);
    let branch_a_two = build_block(block_hash(&branch_a_one.header), 2);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 2,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![
                    genesis.header.clone(),
                    branch_a_one.header.clone(),
                    branch_a_two.header.clone(),
                ],
            }),
            WireNetworkMessage::Block(genesis.clone()),
            WireNetworkMessage::Block(branch_a_one.clone()),
            WireNetworkMessage::Block(branch_a_two.clone()),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, i64::from(branch_a_two.header.time))
            .expect("initial branch sync");
    }

    let branch_b_one = build_branch_block(block_hash(&genesis.header), 1, 100);
    let branch_b_two = build_branch_block(block_hash(&branch_b_one.header), 2, 100);
    let branch_b_three = build_branch_block(block_hash(&branch_b_two.header), 3, 100);
    {
        let store = FjallNodeStore::open(&path).expect("reopen store for durable branch");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 3,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![
                    branch_b_one.header.clone(),
                    branch_b_two.header.clone(),
                    branch_b_three.header.clone(),
                ],
            }),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, i64::from(branch_b_three.header.time))
            .expect("persist better branch headers");
        runtime
            .store()
            .save_block(&branch_b_one, PersistMode::Sync)
            .expect("save branch b one");
        runtime
            .store()
            .save_block(&branch_b_two, PersistMode::Sync)
            .expect("save branch b two");
        runtime
            .store()
            .save_block(&branch_b_three, PersistMode::Sync)
            .expect("save branch b three");
    }

    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(3)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(branch_b_three.header.time))
        .expect("sync after restart");

    // Assert
    assert_eq!(summary.best_header_height, 3);
    assert_eq!(summary.best_block_height, 3);
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(runtime.snapshot_summary().best_block_height, 3);
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());
    assert!(requested_hashes.is_empty());

    remove_dir_if_exists(&path);
}
