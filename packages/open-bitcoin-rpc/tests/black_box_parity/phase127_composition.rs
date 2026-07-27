// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/src/node/context.h
// - packages/bitcoin-knots/src/rpc/server_util.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::fixtures::*;
use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase127_production_composition_shares_sync_serving_and_operator_authority() {
    // Arrange
    let data_dir = phase127_data_dir();
    let _ = fs::remove_dir_all(&data_dir);
    let runtime_config = phase127_runtime_config(data_dir.clone());
    let store = FjallNodeStore::open(&data_dir).expect("phase 127 store should open");
    let mut runtime = DurableSyncRuntime::open_with_runtime_activation(
        store.clone(),
        phase127_sync_config(),
        runtime_config.relay,
        runtime_config.block_serving,
        true,
    )
    .expect("phase 127 durable runtime should open");
    let mut preexisting_sync_state = runtime
        .durable_sync_state(
            SyncLifecycleState::Recovering,
            Some("phase 127 stale startup warning".to_string()),
            1_231_006_499,
        )
        .expect("phase 127 startup sync metadata should project");
    let FieldAvailability::Available(preexisting_progress) =
        &mut preexisting_sync_state.sync.sync_progress
    else {
        panic!("phase 127 startup sync progress should be available");
    };
    preexisting_progress.header_height = 9;
    preexisting_progress.block_height = 4;
    runtime
        .persist_durable_sync_state(preexisting_sync_state)
        .expect("phase 127 startup sync metadata should persist");
    let shared_handle = runtime.network_handle();
    let pre_sync_context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &runtime_config,
        shared_handle,
        Some(store.clone()),
    )
    .expect("phase 127 pre-sync context should compose");
    let block = phase127_mined_block();
    let expected_hash = block_hash(&block.header);
    let mut transport = phase127_transport(&block);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("phase 127 scripted durable sync should succeed");
    let pre_sync_context_tip = pre_sync_context
        .maybe_chain_tip()
        .expect("phase 127 context should read the shared authority")
        .expect("phase 127 sync should establish a tip");
    assert_eq!(pre_sync_context_tip.block_hash, expected_hash);
    assert_eq!(
        summary.maybe_connected_block_hash,
        Some(encoded_hash(expected_hash))
    );
    let mut pre_sync_context = pre_sync_context;
    let live_chain_info = dispatch(
        &mut pre_sync_context,
        MethodCall::GetBlockchainInfo(GetBlockchainInfoRequest::default()),
    )
    .expect("phase 127 pre-existing RPC context should load current durable sync metadata");
    assert_eq!(live_chain_info["blocks"], json!(summary.best_block_height));
    assert_eq!(
        live_chain_info["headers"],
        json!(summary.best_header_height)
    );
    assert_eq!(live_chain_info["verificationprogress"], json!(0.0));
    assert_eq!(live_chain_info["initialblockdownload"], json!(false));
    assert!(
        live_chain_info["warnings"]
            .as_array()
            .is_some_and(|warnings| warnings
                .iter()
                .all(|warning| { warning != "phase 127 stale startup warning" }))
    );
    assert!(
        store
            .load_block(expected_hash)
            .expect("phase 127 durable body lookup should succeed")
            .is_some()
    );

    drop(pre_sync_context);
    drop(runtime);
    drop(store);

    let restarted_store =
        FjallNodeStore::open(&data_dir).expect("phase 127 store should reopen after cache loss");
    let restarted_runtime = DurableSyncRuntime::open_with_runtime_activation(
        restarted_store.clone(),
        phase127_sync_config(),
        runtime_config.relay,
        runtime_config.block_serving,
        true,
    )
    .expect("phase 127 runtime should recover without cache hydration");
    let restarted_handle = restarted_runtime.network_handle();
    let mut restarted_context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &runtime_config,
        restarted_handle.clone(),
        Some(restarted_store),
    )
    .expect("phase 127 restarted context should compose");
    let mut previous_block_hash = expected_hash;
    for height in 1..=restarted_context.coinbase_maturity() {
        let maturity_block = phase127_mined_block_after(previous_block_hash, height);
        previous_block_hash = block_hash(&maturity_block.header);
        restarted_context
            .connect_local_block(&maturity_block)
            .expect("phase 127 coinbase maturity block should connect");
    }
    let available_transaction = phase127_spend_transaction(
        transaction_txid(&block.transactions[0]).expect("phase 127 coinbase transaction id"),
    );
    restarted_context
        .submit_local_transaction(available_transaction.clone())
        .expect("phase 127 transaction should be available for relay serving");
    let activation = activate_inbound_listener(&runtime_config.inbound).await;
    let endpoint = activation
        .bound_endpoints()
        .first()
        .expect("phase 127 loopback listener should bind")
        .bound_endpoint
        .clone();
    restarted_context
        .set_inbound_listener_evidence(activation.evidence().clone())
        .expect("phase 127 listener evidence should use shared authority");
    let shared_context = Arc::new(tokio::sync::Mutex::new(restarted_context));
    let listener_worker = start_inbound_accept_loop(activation, Arc::clone(&shared_context))
        .expect("phase 127 inbound listener should start");
    let mut peer = Phase127WirePeer::connect(&endpoint).await;
    let magic = SyncNetwork::Regtest.magic();
    peer.send(
        WireNetworkMessage::Version(VersionMessage {
            nonce: 127,
            ..VersionMessage::default()
        }),
        magic,
    )
    .await;
    let handshake = [
        peer.receive().await,
        peer.receive().await,
        peer.receive().await,
        peer.receive().await,
    ];
    assert!(matches!(handshake[0], WireNetworkMessage::Version(_)));
    assert!(matches!(handshake[1], WireNetworkMessage::WtxidRelay));
    assert!(matches!(handshake[2], WireNetworkMessage::Verack));
    assert!(matches!(handshake[3], WireNetworkMessage::SendHeaders));
    peer.send(WireNetworkMessage::Verack, magic).await;
    let compact_offer = peer.receive().await;
    assert!(matches!(
        compact_offer,
        WireNetworkMessage::SendCompact(ref message)
            if !message.announce && message.version == BIP152_COMPACT_BLOCKS_VERSION
    ));
    peer.send(phase127_block_request(&block), magic).await;
    let served = peer.receive().await;
    assert!(matches!(
        served,
        WireNetworkMessage::Block(ref served_block)
            if block_hash(&served_block.header) == expected_hash
    ));
    peer.send(
        phase127_mixed_missing_transaction_block_request(&block),
        magic,
    )
    .await;
    let mixed_block_response = peer.receive().await;
    let mixed_not_found_response = peer.receive().await;
    assert!(matches!(
        mixed_block_response,
        WireNetworkMessage::Block(ref served_block)
            if block_hash(&served_block.header) == expected_hash
    ));
    assert!(matches!(
        mixed_not_found_response,
        WireNetworkMessage::NotFound(ref inventory)
            if inventory.inventory.len() == 1
                && inventory.inventory[0].object_hash
                    == Txid::from_byte_array([127_u8; 32]).into()
    ));
    peer.send(
        phase127_mixed_available_transaction_block_request(&block, &available_transaction),
        magic,
    )
    .await;
    let available_then_block = [peer.receive().await, peer.receive().await];
    assert!(matches!(
        available_then_block[0],
        WireNetworkMessage::Tx(ref transaction)
            if transaction_txid(transaction)
                == transaction_txid(&available_transaction)
    ));
    assert!(matches!(
        available_then_block[1],
        WireNetworkMessage::Block(ref served_block)
            if block_hash(&served_block.header) == expected_hash
    ));
    peer.send(
        phase127_mixed_block_available_transaction_request(&block, &available_transaction),
        magic,
    )
    .await;
    let block_then_available = [peer.receive().await, peer.receive().await];
    assert!(matches!(
        block_then_available[0],
        WireNetworkMessage::Block(ref served_block)
            if block_hash(&served_block.header) == expected_hash
    ));
    assert!(matches!(
        block_then_available[1],
        WireNetworkMessage::Tx(ref transaction)
            if transaction_txid(transaction)
                == transaction_txid(&available_transaction)
    ));
    peer.send(
        phase127_mixed_cycle_request(&block, &available_transaction),
        magic,
    )
    .await;
    let mixed_cycles = [
        peer.receive().await,
        peer.receive().await,
        peer.receive().await,
    ];
    assert!(matches!(
        mixed_cycles[0],
        WireNetworkMessage::Block(ref served_block)
            if block_hash(&served_block.header) == expected_hash
    ));
    assert!(matches!(
        mixed_cycles[1],
        WireNetworkMessage::NotFound(ref inventory)
            if inventory.inventory.len() == 1
                && inventory.inventory[0].object_hash
                    == Txid::from_byte_array([126_u8; 32]).into()
    ));
    assert!(matches!(
        mixed_cycles[2],
        WireNetworkMessage::Tx(ref transaction)
            if transaction_txid(transaction)
                == transaction_txid(&available_transaction)
    ));
    peer.send(phase127_two_block_request(&block), magic).await;
    let two_blocks = [peer.receive().await, peer.receive().await];
    assert!(two_blocks.iter().all(|message| matches!(
        message,
        WireNetworkMessage::Block(served_block)
            if block_hash(&served_block.header) == expected_hash
    )));
    peer.send(
        phase127_unknown_available_transaction_request(&available_transaction),
        magic,
    )
    .await;
    peer.send(WireNetworkMessage::Ping { nonce: 127 }, magic)
        .await;
    let unknown_then_available = [peer.receive().await, peer.receive().await];
    assert!(matches!(
        unknown_then_available[0],
        WireNetworkMessage::Tx(ref transaction)
            if transaction_txid(transaction)
                == transaction_txid(&available_transaction)
    ));
    assert!(matches!(
        unknown_then_available[1],
        WireNetworkMessage::Pong { nonce: 127 }
    ));
    peer.send(
        phase127_missing_unknown_available_transaction_request(&available_transaction),
        magic,
    )
    .await;
    peer.send(WireNetworkMessage::Ping { nonce: 128 }, magic)
        .await;
    let missing_unknown_available = [
        peer.receive().await,
        peer.receive().await,
        peer.receive().await,
    ];
    assert!(matches!(
        missing_unknown_available[0],
        WireNetworkMessage::NotFound(ref inventory)
            if inventory.inventory.len() == 1
                && inventory.inventory[0].object_hash
                    == Txid::from_byte_array([124_u8; 32]).into()
    ));
    assert!(matches!(
        missing_unknown_available[1],
        WireNetworkMessage::Tx(ref transaction)
            if transaction_txid(transaction)
                == transaction_txid(&available_transaction)
    ));
    assert!(matches!(
        missing_unknown_available[2],
        WireNetworkMessage::Pong { nonce: 128 }
    ));
    for _ in 0..100 {
        if restarted_handle
            .block_served_write_count()
            .is_ok_and(|count| count == 7)
        {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(
        restarted_handle
            .block_served_write_count()
            .expect("phase 127 served evidence should remain authoritative"),
        7
    );

    let rpc_listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("phase 127 RPC listener should bind");
    let rpc_address = rpc_listener.local_addr().expect("phase 127 RPC address");
    let rpc_state = build_http_state_with_shared_context(
        RpcAuthConfig::user_password(PHASE127_RPC_USERNAME, PHASE127_RPC_PASSWORD),
        Arc::clone(&shared_context),
    )
    .expect("phase 127 RPC state should build");
    let rpc_server = tokio::spawn(async move {
        axum::serve(rpc_listener, router(rpc_state))
            .await
            .expect("phase 127 RPC server should run");
    });
    let mut target = RpcHttpTarget::new(
        "phase127-open-bitcoin",
        rpc_address.to_string(),
        PHASE127_RPC_USERNAME,
        PHASE127_RPC_PASSWORD,
    );
    let (chain_response, network_response, status_response) =
        tokio::task::spawn_blocking(move || {
            let chain = target
                .request("getblockchaininfo", json!([]))
                .expect("phase 127 blockchain RPC should succeed");
            let network = target
                .request("getnetworkinfo", json!([]))
                .expect("phase 127 network RPC should succeed");
            let status = target
                .request("openbitcoinnetworkstatus", json!([]))
                .expect("phase 127 status RPC should succeed");
            (chain, network, status)
        })
        .await
        .expect("phase 127 RPC client task should join");
    let authoritative_block_relay = {
        let context = shared_context.lock().await;
        serde_json::to_value(
            context
                .authoritative_operator_snapshot()
                .expect("phase 127 operator snapshot should be available")
                .block_relay(),
        )
        .expect("phase 127 operator snapshot should serialize")
    };

    // Assert
    assert_eq!(
        chain_response["result"]["bestblockhash"],
        json!(encoded_hash(previous_block_hash))
    );
    assert_eq!(
        sorted_result_keys(&chain_response),
        [
            "bestblockhash",
            "blocks",
            "chain",
            "headers",
            "initialblockdownload",
            "mediantime",
            "verificationprogress",
            "warnings",
        ]
    );
    assert_eq!(
        sorted_result_keys(&network_response),
        [
            "connections",
            "connections_in",
            "connections_out",
            "incrementalfee",
            "localrelay",
            "localservices",
            "protocolversion",
            "relayfee",
            "subversion",
            "version",
            "warnings",
        ]
    );
    assert_eq!(
        sorted_result_keys(&status_response),
        ["block_relay", "inbound", "metrics", "relay"]
    );
    assert_eq!(
        status_response["result"]["block_relay"],
        authoritative_block_relay
    );

    rpc_server.abort();
    drop(peer);
    listener_worker.shutdown().await;
    drop(shared_context);
    drop(restarted_runtime);
    fs::remove_dir_all(data_dir).expect("phase 127 datadir should be removed");
}
