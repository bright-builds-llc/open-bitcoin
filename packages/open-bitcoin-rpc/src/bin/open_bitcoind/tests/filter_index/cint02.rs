// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

use super::phase162::{
    hex,
    outbound::{self, Exchange},
};
use axum::{
    body::to_bytes,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
};
use open_bitcoin_network::{
    CFCheckpt, CFHeaders, CFilter, GetCFCheckpt, GetCFHeaders, GetCFilters, InventoryList,
    ParsedNetworkMessage, ServiceFlags, VersionMessage, WireNetworkMessage,
};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallNodeStore, SyncNetwork, TcpPeerTransport,
    core::{
        chainstate::PruneMode,
        primitives::{BlockHash, InventoryType, InventoryVector},
        wallet::AddressNetwork,
    },
};
use open_bitcoin_rpc::{
    ManagedRpcContext,
    config::{BasicFilterIndexSetting, RpcAuthConfig, RuntimeConfig},
    http::{build_http_state_with_shared_context, handle_http_request},
    inbound_listener::{activate_inbound_listener, start_inbound_accept_loop_with_announcements},
};
use serde_json::{Value, json};
use std::{
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    sync::Arc,
    time::{Duration, Instant},
};

#[path = "cint02/fixture.rs"]
mod fixture;
use fixture::{BOUNDARY, Evidence, Source, TIP};

const MAGIC: open_bitcoin_node::core::primitives::NetworkMagic = SyncNetwork::Regtest.magic();

fn display_hash(bytes: &[u8]) -> String {
    let mut display = bytes.to_vec();
    display.reverse();
    hex(&display)
}

fn exchange(request: WireNetworkMessage, responses: Vec<WireNetworkMessage>) -> Exchange {
    Exchange {
        request,
        expected: responses
            .into_iter()
            .map(|response| response.encode_wire(MAGIC).expect("expected Regtest frame"))
            .collect(),
    }
}

fn filter_exchanges(evidence: &Evidence) -> Vec<Exchange> {
    let pair = [&evidence.pruned, &evidence.retained];
    let stop_hash = evidence.retained.identity().block_hash();
    let mut cases = vec![
        exchange(
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: BOUNDARY,
                stop_hash,
            }),
            pair.iter()
                .map(|record| {
                    WireNetworkMessage::CFilter(CFilter {
                        filter_type: 0,
                        block_hash: record.identity().block_hash(),
                        filter: record.encoded_bytes().to_vec(),
                    })
                })
                .collect(),
        ),
        exchange(
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: BOUNDARY,
                stop_hash,
            }),
            vec![WireNetworkMessage::CFHeaders(CFHeaders {
                filter_type: 0,
                stop_hash,
                previous_filter_header: evidence.pruned.identity().previous_header().into(),
                filter_hashes: pair
                    .iter()
                    .map(|record| record.identity().filter_hash().into())
                    .collect(),
            })],
        ),
    ];
    for branch in [&evidence.active, &evidence.displaced] {
        let stop_hash = branch.identity().block_hash();
        cases.push(exchange(
            WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                filter_type: 0,
                stop_hash,
            }),
            vec![WireNetworkMessage::CFCheckpt(CFCheckpt {
                filter_type: 0,
                stop_hash,
                filter_headers: vec![branch.filter_header().into()],
            })],
        ));
    }
    cases
}

fn inventory(hash: BlockHash) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: hash.into(),
    }])
}

fn inbound_client(endpoint: SocketAddr, cases: Vec<Exchange>) {
    let mut stream = TcpStream::connect_timeout(&endpoint, Duration::from_secs(2))
        .expect("physical inbound connection");
    let started = Instant::now();
    let receive = |stream: &mut TcpStream| {
        ParsedNetworkMessage::decode_wire(
            &outbound::read_frame(stream, started).expect("complete Regtest frame"),
        )
        .expect("valid Regtest frame")
        .message
    };
    outbound::send(
        &mut stream,
        started,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 12_162,
            ..Default::default()
        }),
    )
    .expect("client version");
    let WireNetworkMessage::Version(version) = receive(&mut stream) else {
        panic!("inbound version response")
    };
    assert!(
        version
            .services
            .contains(ServiceFlags::COMPACT_FILTERS | ServiceFlags::NETWORK_LIMITED)
    );
    assert!(!version.services.contains(ServiceFlags::NETWORK));
    assert!((0..8).any(|_| receive(&mut stream) == WireNetworkMessage::Verack));
    outbound::send(&mut stream, started, WireNetworkMessage::Verack).expect("client verack");
    outbound::send(
        &mut stream,
        started,
        WireNetworkMessage::Ping { nonce: 11_999 },
    )
    .expect("handshake fence");
    assert!((0..8).any(|_| receive(&mut stream) == WireNetworkMessage::Pong { nonce: 11_999 }));
    for (index, case) in cases.into_iter().enumerate() {
        outbound::send(&mut stream, started, case.request).expect("inbound request");
        for expected in case.expected {
            assert_eq!(
                outbound::read_frame(&mut stream, started).expect("inbound response"),
                expected,
                "exact inbound frame for exchange {index}"
            );
        }
        let nonce = 11_000 + index as u64;
        outbound::send(&mut stream, started, WireNetworkMessage::Ping { nonce })
            .expect("response fence");
        assert_eq!(receive(&mut stream), WireNetworkMessage::Pong { nonce });
    }
}

async fn inbound_frames(
    config: &RuntimeConfig,
    runtime: &DurableSyncRuntime,
    context: Arc<tokio::sync::Mutex<Context>>,
    evidence: &Evidence,
) {
    let activation = activate_inbound_listener(&config.inbound).await;
    let endpoint: SocketAddr = activation.bound_endpoints()[0]
        .bound_endpoint
        .parse()
        .expect("bound numeric loopback");
    let worker = start_inbound_accept_loop_with_announcements(
        activation,
        context,
        runtime.peer_identity_authority(),
        runtime.announcement_outboxes(),
        runtime.network_handle(),
    )
    .expect("production inbound listener");
    let mut cases = filter_exchanges(evidence);
    cases.push(exchange(
        WireNetworkMessage::GetData(inventory(evidence.retained.identity().block_hash())),
        vec![WireNetworkMessage::Block(evidence.retained_block.clone())],
    ));
    let pruned = inventory(evidence.pruned.identity().block_hash());
    cases.push(exchange(
        WireNetworkMessage::GetData(pruned.clone()),
        vec![WireNetworkMessage::NotFound(pruned)],
    ));
    let outcome = tokio::task::spawn_blocking(move || inbound_client(endpoint, cases)).await;
    tokio::time::timeout(Duration::from_secs(5), worker.shutdown())
        .await
        .expect("joined inbound listener");
    outcome.expect("bounded inbound client assertions");
    assert_eq!(
        runtime
            .network_handle()
            .basic_filter_transport_snapshot()
            .expect("released inbound charges"),
        Default::default()
    );
}

type Context =
    ManagedRpcContext<open_bitcoin_node::FjallChainstateStore, open_bitcoin_node::FjallCoinsView>;

async fn rpc_requests(shared: Arc<tokio::sync::Mutex<Context>>, evidence: &Evidence) {
    let http = build_http_state_with_shared_context(
        RpcAuthConfig::user_password("cint02", "fixture"),
        shared,
    )
    .expect("authenticated RPC context");
    let request = |method: &str, params: Value| {
        json!({"id":1,"jsonrpc":"2.0","method":method,"params":params}).to_string()
    };
    let pruned = display_hash(evidence.pruned.identity().block_hash().as_bytes());
    let invalid = request("getblockfilter", json!([pruned]));
    let unauthenticated = handle_http_request(
        &http,
        "/",
        Method::POST,
        &HeaderMap::new(),
        invalid.as_bytes(),
    )
    .await;
    assert_eq!(unauthenticated.status(), StatusCode::UNAUTHORIZED);
    let mut auth = HeaderMap::new();
    auth.insert(
        "authorization",
        HeaderValue::from_static("Basic Y2ludDAyOmZpeHR1cmU="),
    );
    let invoke = |body: String| {
        let http = &http;
        let auth = &auth;
        async move {
            let response =
                handle_http_request(http, "/", Method::POST, auth, body.as_bytes()).await;
            assert_eq!(
                response.status(),
                StatusCode::OK,
                "JSON-RPC response for {body}"
            );
            serde_json::from_slice::<Value>(
                &to_bytes(response.into_body(), 16 * 1024)
                    .await
                    .expect("bounded JSON RPC body"),
            )
            .expect("RPC JSON")
        }
    };
    for record in [
        &evidence.pruned,
        &evidence.retained,
        &evidence.active,
        &evidence.displaced,
    ] {
        let hash = display_hash(record.identity().block_hash().as_bytes());
        let reply = invoke(request("getblockfilter", json!([hash, "basic"]))).await;
        assert_eq!(
            reply["result"],
            json!({"filter":hex(record.encoded_bytes()),"header":display_hash(record.filter_header().as_bytes())})
        );
    }
    let summary = invoke(request("getindexinfo", json!([]))).await;
    assert_eq!(
        summary["result"]["basic block filter index"],
        json!({"synced":true,"best_block_height":TIP})
    );
    for (method, params) in [
        ("getwalletinfo", json!([])),
        ("rescanblockchain", json!([BOUNDARY + 1, TIP])),
    ] {
        let refused = invoke(request(method, params)).await;
        assert_eq!(refused["error"]["code"], -4, "wallet refusal for {method}");
        assert!(
            refused["error"]["message"]
                .as_str()
                .expect("typed wallet reason")
                .contains("wallet unavailable")
        );
    }
}

#[tokio::test]
async fn cint02_authenticated_rpc_and_both_peer_directions_after_canonical_pruning() {
    // Arrange: independent deterministic replay input and one actually pruned original.
    let archive = Source::new();
    let retained = Source::new();
    fixture::build(&archive, false);
    let evidence = fixture::build(&retained, true);
    let remote = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("physical outbound remote");
    remote.set_nonblocking(true).expect("bounded remote accept");
    let exchanges = filter_exchanges(&evidence);
    let count = exchanges.len();
    let mut config = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(retained.path().to_path_buf()),
        prune_mode: PruneMode::ManualOnly,
        block_filter_index: BasicFilterIndexSetting::Basic,
        peer_block_filters: true,
        ..Default::default()
    };
    config.sync.runtime = outbound::config(remote.local_addr().expect("outbound endpoint"), count);
    config.inbound.enabled = true;
    config.inbound.listen_addresses = vec!["127.0.0.1:0".to_owned()];
    config.inbound.permission_classes = open_bitcoin_network::PeerPermissionClassRegistry::new([
        open_bitcoin_network::ParsedPeerPermissionClass::parse(
            "cint02",
            ["127.0.0.1"],
            ["in", "download", "blockfilters"],
        )
        .expect("explicit loopback download and BASIC permission"),
    ]);
    config.block_serving.block_serving.enabled = true;
    let mut invalid_config = config.clone();
    invalid_config.inbound.allow_public = true;
    assert!(matches!(
        ManagedRpcContext::from_pruned_regtest_runtime_config(&invalid_config, archive.path()),
        Err(open_bitcoin_rpc::context::CanonicalContextError::InvalidGenesisConfiguration)
    ));
    let (context, mut runtime) =
        ManagedRpcContext::from_pruned_regtest_runtime_config(&config, archive.path())
            .expect("proved client context");
    drop(archive);
    let shared = Arc::new(tokio::sync::Mutex::new(context));

    // Act and assert: all consumers observe the same retained branch and deletion.
    rpc_requests(Arc::clone(&shared), &evidence).await;
    inbound_frames(&config, &runtime, Arc::clone(&shared), &evidence).await;
    let listener = remote
        .try_clone()
        .expect("same configured loopback endpoint");
    let peer = std::thread::spawn(move || outbound::remote(listener, exchanges));
    let summary = runtime.sync_once(&mut TcpPeerTransport, 2_000_000_000);
    let observed = peer
        .join()
        .expect("joined outbound remote")
        .expect("exact outbound frames");
    outbound::assert_summary(&summary.expect("public outbound sync adapter"), count);
    assert_eq!((observed.requests, observed.frames), (count, 5));
    let unexpected = exchange(
        WireNetworkMessage::Headers(open_bitcoin_network::HeadersMessage {
            headers: vec![evidence.retained_block.header.clone()],
        }),
        Vec::new(),
    );
    let peer = std::thread::spawn(move || outbound::remote(remote, vec![unexpected]));
    let refused = runtime.sync_once(&mut TcpPeerTransport, 2_000_000_001);
    let denial = peer
        .join()
        .expect("joined rejected outbound peer")
        .expect("received no acquisition response");
    assert_eq!((denial.requests, denial.frames), (1, 0));
    let refused = refused.expect("guarded outbound summary");
    assert_eq!(
        (
            refused.failed_peers,
            refused.headers_received,
            refused.blocks_received
        ),
        (1, 0, 0)
    );
    assert_eq!(refused.best_block_height, u64::from(TIP));
    assert_eq!(
        runtime
            .network_handle()
            .basic_filter_transport_snapshot()
            .expect("released outbound charges"),
        Default::default()
    );
    drop(shared);
    drop(runtime);
    let store = FjallNodeStore::open(retained.path()).expect("all client handles closed");
    let missing = evidence.pruned.identity().block_hash();
    assert!(!store.has_block(missing).expect("body absent after clients"));
    assert!(!store.has_undo(missing).expect("undo absent after clients"));
    eprintln!(
        "CINT-02 client proof: height={TIP} pruned_through={BOUNDARY} rpc_filters=4 inbound_frames=7 outbound_frames={} wallet_refusals=2",
        observed.frames
    );
}
