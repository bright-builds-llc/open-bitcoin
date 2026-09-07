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

fn open_runtime_activation_source() -> &'static str {
    include_str!("../../sync/open_runtime.rs")
}

#[test]
fn open_runtime_does_not_hydrate_leftover_utxos() {
    // Arrange
    let open_src = open_runtime_activation_source();

    // Act / Assert
    assert!(open_src.contains("initialize"));
    assert!(open_src.contains("from_coins_cache"));
    assert!(open_src.contains("from_chainstate"));
    assert!(!open_src.contains("hydrate_chainstate_for_open"));
}

#[test]
fn open_stores_initialize_lifecycle_and_cache() {
    // Arrange
    let open_src = open_runtime_activation_source();

    // Act / Assert
    assert!(open_src.contains("let (lifecycle, _view, cache) = initialize("));
    assert!(open_src.contains("from_coins_cache(cache"));
    assert!(open_src.contains("from_chainstate("));
    assert!(open_src.contains("lifecycle"));
    assert!(!open_src.contains("from_parent("));
    assert!(!open_src.contains("from_chainstate(store, chainstate)"));
}

#[test]
fn same_datadir_reopen_tip_matches_coins_best_block() {
    // Arrange
    let path = temp_store_path("reopen-from-coins-b");
    remove_dir_if_exists(&path);
    let coins_tip = ChainPosition::new(header(BlockHash::from_byte_array([0_u8; 32]), 1), 0, 1, 1);
    let leftover_tip =
        ChainPosition::new(header(BlockHash::from_byte_array([0x11; 32]), 2), 1, 2, 2);
    let coins_outpoint = OutPoint {
        txid: open_bitcoin_core::primitives::Txid::from_byte_array([0xab; 32]),
        vout: 0,
    };
    let leftover_outpoint = OutPoint {
        txid: open_bitcoin_core::primitives::Txid::from_byte_array([0xee; 32]),
        vout: 7,
    };
    let coins_coin = open_bitcoin_core::chainstate::Coin {
        output: TransactionOutput {
            value: Amount::from_sats(7_000).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
        },
        is_coinbase: false,
        created_height: 0,
        created_median_time_past: 1,
    };
    let leftover_coin = open_bitcoin_core::chainstate::Coin {
        output: TransactionOutput {
            value: Amount::from_sats(9_000).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
        },
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 2,
    };
    let mut coins_utxos = std::collections::HashMap::new();
    coins_utxos.insert(coins_outpoint.clone(), coins_coin.clone());
    let coins_snapshot = ChainstateSnapshot::new(vec![coins_tip], coins_utxos, Default::default());
    let mut leftover_utxos = std::collections::HashMap::new();
    leftover_utxos.insert(leftover_outpoint.clone(), leftover_coin);
    let leftover_snapshot =
        ChainstateSnapshot::new(vec![leftover_tip], leftover_utxos, Default::default());
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .seed_coins_from_snapshot(&coins_snapshot)
            .expect("seed coins B");
        store
            .save_chainstate_snapshot(&leftover_snapshot, PersistMode::Sync)
            .expect("plant leftover");
    }

    // Act
    let store = FjallNodeStore::open(&path).expect("reopen");
    let coins_best = store
        .coins_view()
        .best_block()
        .expect("coins B")
        .expect("flushed tip");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("open from coins B");
    let maybe_tip = runtime
        .network_handle()
        .maybe_chain_tip()
        .expect("in-memory tip");
    let leftover_lookup = runtime
        .store()
        .coins_view()
        .get_coin(&leftover_outpoint)
        .expect("leftover lookup");
    let coins_lookup = runtime
        .store()
        .coins_view()
        .get_coin(&coins_outpoint)
        .expect("coins lookup");

    // Assert
    assert_eq!(maybe_tip.map(|tip| tip.block_hash), Some(coins_best));
    assert_eq!(leftover_lookup, None);
    assert_eq!(coins_lookup, Some(coins_coin));
    remove_dir_if_exists(&path);
}

#[test]
fn persist_progress_does_not_write_leftover_and_reopen_ignores_leftover_utxos() {
    // Arrange
    let path = temp_store_path("persist-no-leftover-write");
    remove_dir_if_exists(&path);
    let tip = ChainPosition::new(header(BlockHash::from_byte_array([0_u8; 32]), 1), 0, 1, 1);
    let coins_outpoint = OutPoint {
        txid: open_bitcoin_core::primitives::Txid::from_byte_array([0xcd; 32]),
        vout: 1,
    };
    let leftover_outpoint = OutPoint {
        txid: open_bitcoin_core::primitives::Txid::from_byte_array([0xdd; 32]),
        vout: 2,
    };
    let coins_coin = open_bitcoin_core::chainstate::Coin {
        output: TransactionOutput {
            value: Amount::from_sats(8_000).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
        },
        is_coinbase: false,
        created_height: 0,
        created_median_time_past: 1,
    };
    let leftover_coin = open_bitcoin_core::chainstate::Coin {
        output: TransactionOutput {
            value: Amount::from_sats(3_000).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
        },
        is_coinbase: false,
        created_height: 0,
        created_median_time_past: 1,
    };
    let mut coins_utxos = std::collections::HashMap::new();
    coins_utxos.insert(coins_outpoint.clone(), coins_coin.clone());
    let coins_snapshot =
        ChainstateSnapshot::new(vec![tip.clone()], coins_utxos, Default::default());
    let mut leftover_utxos = std::collections::HashMap::new();
    leftover_utxos.insert(leftover_outpoint.clone(), leftover_coin);
    let leftover_snapshot = ChainstateSnapshot::new(vec![tip], leftover_utxos, Default::default());
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .seed_coins_from_snapshot(&coins_snapshot)
            .expect("seed coins");
        store
            .save_chainstate_snapshot(&leftover_snapshot, PersistMode::Sync)
            .expect("plant leftover");
    }

    // Act
    let store = FjallNodeStore::open(&path).expect("reopen");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("open");
    runtime
        .persist_progress()
        .expect("headers and runtime only");
    let persist_src = include_str!("../../sync/runtime_state.rs");

    // Assert
    assert!(!persist_src.contains("save_chainstate_snapshot"));
    assert!(!persist_src.contains("seed_coins_from_snapshot"));
    assert!(
        runtime
            .store()
            .load_chainstate_snapshot()
            .expect("leftover unread")
            .is_some(),
        "leftover file may remain unread"
    );
    assert_eq!(
        runtime
            .store()
            .coins_view()
            .get_coin(&leftover_outpoint)
            .expect("leftover must not become coins B"),
        None
    );
    assert_eq!(
        runtime
            .store()
            .coins_view()
            .get_coin(&coins_outpoint)
            .expect("coins B"),
        Some(coins_coin)
    );
    remove_dir_if_exists(&path);
}

#[test]
fn persist_progress_does_not_credit_when_coins_b_lags_memory_tip() {
    // Arrange
    let path = temp_store_path("credit-requires-coins-b");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone(), child.header.clone()],
        }),
        WireNetworkMessage::Block(genesis.clone()),
        WireNetworkMessage::Block(child.clone()),
    ]]);

    // Act
    let _summary = runtime
        .sync_once(&mut transport, i64::from(child.header.time))
        .expect("memory connect");
    runtime.persist_progress().expect("headers only");
    let unflushed_state = runtime
        .durable_sync_state(
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time),
        )
        .expect("status before flush");
    let maybe_coins_best = runtime.store().coins_view().best_block().expect("read B");
    let child_hash = block_hash(&child.header);
    runtime
        .network_handle()
        .flush_coins(
            open_bitcoin_core::chainstate::FlushMode::Always,
            open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(0),
            u64::MAX,
        )
        .expect("always flush advances B");
    runtime.persist_progress().expect("headers after flush");
    let flushed_state = runtime
        .durable_sync_state(
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time),
        )
        .expect("status after flush");
    let flushed_b = runtime
        .store()
        .coins_view()
        .best_block()
        .expect("read B after flush");

    // Assert
    assert_ne!(
        maybe_coins_best,
        Some(child_hash),
        "IfNeeded must not have flushed the memory tip"
    );
    match &unflushed_state.sync.progress_credit {
        FieldAvailability::Available(credit) => {
            assert_ne!(
                credit.credited_validated_active_chain_hash,
                block_hash_hex(child_hash)
            );
        }
        FieldAvailability::Unavailable { .. } => {}
    }
    assert_eq!(flushed_b, Some(child_hash));
    let FieldAvailability::Available(credit) = &flushed_state.sync.progress_credit else {
        panic!("flushed coins B should allow Available credit");
    };
    assert_eq!(
        credit.credited_validated_active_chain_hash,
        block_hash_hex(child_hash)
    );
    remove_dir_if_exists(&path);
}
