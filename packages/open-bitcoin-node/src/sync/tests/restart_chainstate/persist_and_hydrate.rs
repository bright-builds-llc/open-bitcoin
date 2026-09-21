// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

fn open_runtime_activation_source() -> &'static str {
    include_str!("../../open_runtime.rs")
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
    let persist_src = include_str!("../../runtime_state.rs");

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
