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

use std::{
    collections::VecDeque,
    env, fs,
    path::PathBuf,
    process,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingActivationConfig, CompactRelayActivationConfig,
    HeadersMessage, InboundListenerConfig, InventoryList, ParsedNetworkMessage,
    ParsedPeerPermissionClass, PeerPermissionClassRegistry, VersionMessage, WireNetworkMessage,
};
use open_bitcoin_node::{
    DurableSyncRuntime, FieldAvailability, FjallNodeStore, ResolvedSyncPeerAddress,
    SyncLifecycleState, SyncNetwork, SyncPeerAddress, SyncPeerReceiveOutcome, SyncPeerSession,
    SyncRuntimeConfig, SyncRuntimeError, SyncTransport,
    core::{
        codec::parse_message_header,
        consensus::{block_hash, block_merkle_root, check_block_header},
        primitives::{
            Amount, Block, BlockHash, BlockHeader, InventoryType, InventoryVector, NetworkMagic,
            OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
            Txid,
        },
        wallet::AddressNetwork,
    },
};
use open_bitcoin_rpc::{
    ManagedRpcContext, RpcAuthConfig, RuntimeConfig,
    dispatch::dispatch,
    http::{build_http_state, build_http_state_with_shared_context, router},
    inbound_listener::{activate_inbound_listener, start_inbound_accept_loop},
    method::{GetBlockchainInfoRequest, MethodCall},
};
use open_bitcoin_test_harness::{
    ExpectedOutcome, FunctionalCase, HarnessTarget, RpcHttpTarget, run_suite, skipped_suite,
    write_reports_from_env,
};
use serde_json::json;
use tokio::net::TcpStream;

const RPC_USERNAME: &str = "alice";
const RPC_PASSWORD: &str = "secret";
const SUITE_NAME: &str = "rpc-black-box-parity";
const PHASE127_EASY_BITS: u32 = 0x207f_ffff;
const PHASE127_RPC_USERNAME: &str = "phase127-rpc-user";
const PHASE127_RPC_PASSWORD: &str = "phase127-secret";
const PHASE127_FORBIDDEN_PERMISSION: &str = "phase127-private-permission";
const WIRE_HEADER_LENGTH: usize = 24;
static NEXT_PHASE127_DIR: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct Phase127ScriptedTransport {
    inbound: VecDeque<WireNetworkMessage>,
}

#[derive(Debug)]
struct Phase127ScriptedSession {
    inbound: VecDeque<WireNetworkMessage>,
}

struct Phase127WirePeer {
    stream: TcpStream,
    buffered: Vec<u8>,
}

impl SyncTransport for Phase127ScriptedTransport {
    type Session = Phase127ScriptedSession;

    fn connect(
        &mut self,
        _peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        Ok(Phase127ScriptedSession {
            inbound: core::mem::take(&mut self.inbound),
        })
    }
}

impl SyncPeerSession for Phase127ScriptedSession {
    fn send(
        &mut self,
        _message: &WireNetworkMessage,
        _magic: NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        Ok(self.inbound.pop_front().map_or(
            SyncPeerReceiveOutcome::Closed,
            SyncPeerReceiveOutcome::Message,
        ))
    }
}

impl Phase127WirePeer {
    async fn connect(endpoint: &str) -> Self {
        let stream = TcpStream::connect(endpoint)
            .await
            .expect("phase 127 loopback peer should connect");
        Self {
            stream,
            buffered: Vec::new(),
        }
    }

    async fn send(&self, message: WireNetworkMessage, magic: NetworkMagic) {
        let bytes = message
            .encode_wire(magic)
            .expect("phase 127 message should encode");
        let mut written = 0;
        while written < bytes.len() {
            self.stream
                .writable()
                .await
                .expect("phase 127 peer should become writable");
            match self.stream.try_write(&bytes[written..]) {
                Ok(0) => panic!("phase 127 peer write made no progress"),
                Ok(count) => written += count,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("phase 127 peer write failed: {error}"),
            }
        }
    }

    async fn receive(&mut self) -> WireNetworkMessage {
        loop {
            if self.buffered.len() >= WIRE_HEADER_LENGTH {
                let header = parse_message_header(&self.buffered[..WIRE_HEADER_LENGTH])
                    .expect("phase 127 response header should decode");
                let frame_length = WIRE_HEADER_LENGTH + header.payload_size as usize;
                if self.buffered.len() >= frame_length {
                    let frame = self.buffered.drain(..frame_length).collect::<Vec<_>>();
                    return ParsedNetworkMessage::decode_wire(&frame)
                        .expect("phase 127 response should decode")
                        .message;
                }
            }

            self.stream
                .readable()
                .await
                .expect("phase 127 peer should become readable");
            let mut bytes = [0_u8; 4_096];
            match self.stream.try_read(&mut bytes) {
                Ok(0) => panic!("phase 127 listener closed before a complete response"),
                Ok(count) => self.buffered.extend_from_slice(&bytes[..count]),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("phase 127 peer read failed: {error}"),
            }
        }
    }
}

fn functional_cases() -> Vec<FunctionalCase> {
    vec![
        FunctionalCase {
            name: "getblockchaininfo shape",
            method: "getblockchaininfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec![
                "chain",
                "blocks",
                "headers",
                "initialblockdownload",
            ]),
        },
        FunctionalCase {
            name: "getnetworkinfo shape",
            method: "getnetworkinfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec![
                "version",
                "subversion",
                "protocolversion",
                "connections",
            ]),
        },
        FunctionalCase {
            name: "getmempoolinfo shape",
            method: "getmempoolinfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec!["size", "bytes", "loaded"]),
        },
        FunctionalCase {
            name: "unknown method error shape",
            method: "openbitcoin_does_not_exist",
            params: json!([]),
            expected: ExpectedOutcome::ErrorCode(-32601),
        },
    ]
}

fn phase127_data_dir() -> PathBuf {
    std::env::temp_dir().join(format!(
        "open-bitcoin-phase127-black-box-{}-{}",
        process::id(),
        NEXT_PHASE127_DIR.fetch_add(1, Ordering::SeqCst),
    ))
}

fn phase127_runtime_config(data_dir: PathBuf) -> RuntimeConfig {
    let permission_classes = PeerPermissionClassRegistry::new([ParsedPeerPermissionClass::parse(
        PHASE127_FORBIDDEN_PERMISSION,
        ["127.0.0.1"],
        ["in", "download"],
    )
    .expect("phase 127 loopback permission should parse")]);
    RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(data_dir),
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:0".to_string()],
            max_peers: 2,
            reserved_slots: 1,
            allow_public: false,
            permission_classes,
        },
        block_serving: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
        ..RuntimeConfig::default()
    }
}

fn phase127_sync_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        manual_peers: vec![SyncPeerAddress::manual("127.0.0.1", 18_444)],
        dns_seeds: Vec::new(),
        target_outbound_peers: 1,
        max_peer_retries: 0,
        max_rounds: 1,
        ..SyncRuntimeConfig::default()
    }
}

fn phase127_mined_block() -> Block {
    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(vec![0x00, 0x51]).expect("phase 127 coinbase script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("phase 127 coinbase amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("phase 127 output script"),
        }],
        lock_time: 0,
    };
    let (merkle_root, maybe_mutated) =
        block_merkle_root(core::slice::from_ref(&transaction)).expect("phase 127 merkle root");
    assert!(!maybe_mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::default(),
            merkle_root,
            time: 1_231_006_500,
            bits: PHASE127_EASY_BITS,
            nonce: 0,
        },
        transactions: vec![transaction],
    };
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("phase 127 easy target should be mineable");
    block
}

fn phase127_transport(block: &Block) -> Phase127ScriptedTransport {
    Phase127ScriptedTransport {
        inbound: VecDeque::from([
            WireNetworkMessage::Version(VersionMessage {
                start_height: 0,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![block.header.clone()],
            }),
            WireNetworkMessage::Block(block.clone()),
        ]),
    }
}

fn phase127_block_request(block: &Block) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash(&block.header).into(),
    }]))
}

fn phase127_mixed_missing_transaction_block_request(block: &Block) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Txid::from_byte_array([127_u8; 32]).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: block_hash(&block.header).into(),
        },
    ]))
}

fn sorted_result_keys(response: &serde_json::Value) -> Vec<String> {
    let mut keys = response["result"]
        .as_object()
        .expect("phase 127 RPC result should be an object")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    keys.sort();
    keys
}

fn encoded_hash(block_hash: BlockHash) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(64);
    for byte in block_hash.as_bytes() {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn same_black_box_suite_targets_open_bitcoin_rpc() {
    // Arrange
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener.local_addr().expect("local address");
    let context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    let state = build_http_state(
        RpcAuthConfig::user_password(RPC_USERNAME, RPC_PASSWORD),
        context,
    )
    .expect("state should build");
    let server = tokio::spawn(async move {
        axum::serve(listener, router(state))
            .await
            .expect("server should run");
    });
    let mut target = RpcHttpTarget::new(
        "open-bitcoin",
        address.to_string(),
        RPC_USERNAME,
        RPC_PASSWORD,
    );
    let cases = functional_cases();

    // Act
    let report = run_suite(SUITE_NAME, &mut target, &cases);
    write_reports_from_env(&report).expect("report write should succeed");
    server.abort();

    // Assert
    assert!(report.passed(), "{report:#?}");
    assert_eq!(report.outcomes.len(), cases.len());
}

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
        WireNetworkMessage::NotFound(_)
    ));
    for _ in 0..100 {
        if restarted_handle
            .block_served_write_count()
            .is_ok_and(|count| count == 2)
        {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(
        restarted_handle
            .block_served_write_count()
            .expect("phase 127 served evidence should remain authoritative"),
        2
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
        json!(encoded_hash(expected_hash))
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

#[test]
fn same_black_box_suite_can_target_configured_knots_rpc() {
    // Arrange
    let maybe_addr = env::var("OPEN_BITCOIN_KNOTS_RPC_ADDR").ok();
    let maybe_user = env::var("OPEN_BITCOIN_KNOTS_RPC_USER").ok();
    let maybe_password = env::var("OPEN_BITCOIN_KNOTS_RPC_PASSWORD").ok();
    let cases = functional_cases();

    let (Some(addr), Some(user), Some(password)) = (maybe_addr, maybe_user, maybe_password) else {
        let report = skipped_suite(
            SUITE_NAME,
            "bitcoin-knots",
            "set OPEN_BITCOIN_KNOTS_RPC_ADDR, OPEN_BITCOIN_KNOTS_RPC_USER, and OPEN_BITCOIN_KNOTS_RPC_PASSWORD to run the same suite against Knots",
        );
        write_reports_from_env(&report).expect("skipped report should write");
        return;
    };
    let mut target = RpcHttpTarget::new("bitcoin-knots", addr, user, password);

    // Act
    let report = run_suite(SUITE_NAME, &mut target, &cases);
    write_reports_from_env(&report).expect("report write should succeed");

    // Assert
    assert!(report.passed(), "{report:#?}");
    assert_eq!(report.outcomes.len(), cases.len());
}
