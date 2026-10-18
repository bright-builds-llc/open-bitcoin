// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Physical configured replies; client totals are distinct from private receipt counters.

use super::core_types::{
    chainstate::{FlushMode, FlushPolicyTime},
    codec::encode_block,
    consensus::{ScriptVerifyFlags, block_hash, crypto::Sha256},
};
use super::fixtures::live_source;
use super::{
    decode_hex,
    fixtures::{Phase162Fixture, Phase162Handoff, Provenance},
    oracle_case, oracle_hash, process, recipe, retention,
};
use open_bitcoin_network::{
    GetCFCheckpt, GetCFHeaders, GetCFilters, ParsedNetworkMessage, ServiceFlags, VersionMessage,
    WireNetworkMessage,
};
use open_bitcoin_node::{
    FjallNodeStore, PersistMode, SyncNetwork, SyncRuntimeConfig, core::primitives::NetworkMagic,
    network::filter_transport::FilterTransportSnapshot,
};
use serde_json::{Value, json};
use std::{io, process::Command, sync::Arc, time::Duration};
use tokio::net::TcpStream;

#[derive(Clone)]
struct Exchange {
    request: WireNetworkMessage,
    expected: Vec<Vec<u8>>,
}

fn wire(fixture: &Phase162Fixture, name: &str) -> Vec<u8> {
    let row = fixture.provenance.oracle["wire"]
        .as_array()
        .expect("independent wire rows")
        .iter()
        .find(|row| row["name"] == name)
        .expect("named independent frame");
    decode_hex(row["expected"]["frame_hex"].as_str().expect("pinned frame"))
}

fn bodyless_headers(fixture: &Phase162Fixture, names: &[&str]) -> Vec<u8> {
    let first = oracle_case(fixture, names[0]);
    let stop = oracle_case(fixture, names[names.len() - 1]);
    let corpus = json!({"wire":[{"name":"bodyless-headers","command":"cfheaders","input":{
        "filter_type":0,"stop_hash_raw":stop["block_hash_raw"],"prev_header_raw":first["predecessor_raw"],
        "hashes_raw":names.iter().map(|name| oracle_case(fixture, name)["hash_raw"].clone()).collect::<Vec<_>>()}}],
        "proof":[],"handlers":[],"malformed":[]});
    let mut command = Command::new("bun");
    // Thin invocation of the existing pin, serializer and strict result validator.
    command.current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .args(["--eval", "import {runBounded,validateResult,PIN} from './scripts/generate-peer-filter-fixtures.ts'; const root=process.cwd(); if(runBounded('git',['-C','packages/bitcoin-knots','rev-parse','HEAD'],root).trim()!==PIN) throw Error('pin mismatch'); const input=await Bun.stdin.text(); process.stdout.write(validateResult(runBounded('python3',['scripts/basic-filter-oracle.py','--peer-wire-proof'],root,input),JSON.parse(input)).wire);"]);
    let output = process::run(
        &mut command,
        serde_json::to_vec(&corpus).expect("independent inputs"),
        process::Limits {
            input_bytes: 64 * 1024,
            stdout_bytes: 64 * 1024,
            stderr_bytes: 4096,
            deadline: Duration::from_secs(20),
        },
    )
    .expect("bounded existing serializer");
    assert!(output.status.success() && output.reaped);
    let result: Value = serde_json::from_slice(&output.stdout).expect("validated wire JSON");
    let mut bytes = decode_hex(
        result["cases"][0]["expected"]["frame_hex"]
            .as_str()
            .expect("exact frame"),
    );
    bytes[..4].copy_from_slice(&fixture.config.sync.runtime.network.magic().to_bytes());
    bytes
}

fn exchanges(fixture: &Phase162Fixture) -> Vec<Exchange> {
    let mut exchanges = Vec::new();
    for names in [
        ["active-19", "active-20", "active-21"],
        ["active-713", "active-714", "active-715"],
    ] {
        let start_height = names[0]
            .split('-')
            .nth(1)
            .expect("raw height")
            .parse()
            .expect("height");
        let stop_hash = oracle_hash(oracle_case(fixture, names[2]));
        exchanges.push(Exchange {
            request: WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height,
                stop_hash,
            }),
            expected: names
                .iter()
                .map(|name| wire(fixture, &format!("{name}-cfilter")))
                .collect(),
        });
        exchanges.push(Exchange {
            request: WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height,
                stop_hash,
            }),
            expected: vec![bodyless_headers(fixture, &names)],
        });
    }
    for (leaf, start_height, names, checkpoint) in [
        (
            "active-1000",
            997,
            ["active-997", "active-998", "active-999", "active-1000"],
            "active-1000",
        ),
        (
            "fork-1002",
            999,
            ["fork-999", "fork-1000", "fork-1001", "fork-1002"],
            "fork-1000",
        ),
    ] {
        let stop_hash = oracle_hash(oracle_case(fixture, leaf));
        exchanges.push(Exchange {
            request: WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height,
                stop_hash,
            }),
            expected: names
                .iter()
                .map(|name| wire(fixture, &format!("{name}-cfilter")))
                .collect(),
        });
        let expected = wire(fixture, &format!("{leaf}-cfheaders"));
        let WireNetworkMessage::CFHeaders(headers) = ParsedNetworkMessage::decode_wire(&expected)
            .expect("oracle header frame")
            .message
        else {
            panic!("oracle cfheaders")
        };
        assert_eq!(headers.filter_hashes.len(), 4);
        assert_eq!(
            super::hex(headers.previous_filter_header.as_bytes()),
            oracle_case(fixture, names[0])["predecessor_raw"]
        );
        exchanges.push(Exchange {
            request: WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height,
                stop_hash,
            }),
            expected: vec![expected],
        });
        let expected = wire(fixture, &format!("{leaf}-cfcheckpt"));
        let WireNetworkMessage::CFCheckpt(checkpoints) =
            ParsedNetworkMessage::decode_wire(&expected)
                .expect("oracle checkpoint frame")
                .message
        else {
            panic!("oracle cfcheckpt")
        };
        assert_eq!(
            checkpoints.filter_headers.len(),
            1,
            "one positive h1000 checkpoint"
        );
        assert_eq!(
            super::hex(checkpoints.filter_headers[0].as_bytes()),
            oracle_case(fixture, checkpoint)["header_raw"]
        );
        exchanges.push(Exchange {
            request: WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                filter_type: 0,
                stop_hash,
            }),
            expected: vec![expected],
        });
    }
    assert_ne!(
        oracle_case(fixture, "active-1000")["header_raw"],
        oracle_case(fixture, "fork-1000")["header_raw"]
    );
    exchanges
}

async fn exact(stream: &TcpStream, buffer: &mut [u8]) {
    let mut offset = 0;
    while offset < buffer.len() {
        stream.readable().await.expect("bounded socket readable");
        match stream.try_read(&mut buffer[offset..]) {
            Ok(0) => panic!("closed incomplete frame"),
            Ok(count) => offset += count,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => panic!("physical socket read: {error}"),
        }
    }
}

async fn read_frame(stream: &TcpStream) -> Vec<u8> {
    let mut bytes = vec![0; 24];
    exact(stream, &mut bytes).await;
    let size = open_bitcoin_codec::parse_message_header(&bytes)
        .expect("frame header")
        .payload_size as usize;
    assert!(size <= 16 * 1024, "short response frame bound");
    bytes.resize(24 + size, 0);
    exact(stream, &mut bytes[24..]).await;
    bytes
}

async fn send(stream: &TcpStream, magic: NetworkMagic, message: WireNetworkMessage) {
    let bytes = message.encode_wire(magic).expect("valid request frame");
    let mut offset = 0;
    while offset < bytes.len() {
        stream.writable().await.expect("bounded socket writable");
        match stream.try_write(&bytes[offset..]) {
            Ok(0) => panic!("zero socket write"),
            Ok(count) => offset += count,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => panic!("physical socket write: {error}"),
        }
    }
}

async fn sentinel(stream: &TcpStream, magic: NetworkMagic, nonce: u64) {
    send(stream, magic, WireNetworkMessage::Ping { nonce }).await;
    assert_eq!(
        ParsedNetworkMessage::decode_wire(&read_frame(stream).await)
            .expect("fenced pong")
            .message,
        WireNetworkMessage::Pong { nonce }
    );
}

async fn denied_disconnect(stream: &TcpStream) {
    tokio::time::timeout(Duration::from_secs(5), async {
        let mut byte = [0; 1];
        loop {
            stream.readable().await.expect("denial readable");
            match stream.try_read(&mut byte) {
                Ok(0) => break,
                Err(error)
                    if matches!(
                        error.kind(),
                        io::ErrorKind::ConnectionReset | io::ErrorKind::ConnectionAborted
                    ) =>
                {
                    break;
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => continue,
                other => panic!("absent capability requires zero bytes then disconnect: {other:?}"),
            }
        }
    })
    .await
    .expect("denial must physically disconnect before deadline");
}

async fn handshake(stream: &TcpStream, magic: NetworkMagic) -> ServiceFlags {
    send(
        stream,
        magic,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 162_004,
            ..Default::default()
        }),
    )
    .await;
    let WireNetworkMessage::Version(version) =
        ParsedNetworkMessage::decode_wire(&read_frame(stream).await)
            .expect("physical version")
            .message
    else {
        panic!("version first")
    };
    let mut acknowledged = false;
    for _ in 0..8 {
        if ParsedNetworkMessage::decode_wire(&read_frame(stream).await)
            .expect("handshake frame")
            .message
            == WireNetworkMessage::Verack
        {
            acknowledged = true;
            break;
        }
    }
    assert!(acknowledged, "finite version/verack handshake");
    send(stream, magic, WireNetworkMessage::Verack).await;
    send(stream, magic, WireNetworkMessage::Ping { nonce: 162_005 }).await;
    let mut fenced = false;
    for _ in 0..8 {
        let message = ParsedNetworkMessage::decode_wire(&read_frame(stream).await)
            .expect("handshake drain")
            .message;
        if message == (WireNetworkMessage::Pong { nonce: 162_005 }) {
            fenced = true;
            break;
        }
        assert!(!matches!(
            message,
            WireNetworkMessage::CFilter(_)
                | WireNetworkMessage::CFHeaders(_)
                | WireNetworkMessage::CFCheckpt(_)
        ));
    }
    assert!(fenced);
    version.services
}

async fn physical_session(fixture: &Phase162Fixture, exchanges: Vec<Exchange>, granted: bool) {
    let expected_exchanges = if granted { exchanges.as_slice() } else { &[] };
    let expected_frames = expected_exchanges
        .iter()
        .map(|exchange| exchange.expected.len())
        .sum();
    let expected_bytes = expected_exchanges
        .iter()
        .flat_map(|exchange| &exchange.expected)
        .map(Vec::len)
        .sum();
    let mut listener =
        crate::inbound_startup::start_inbound_listener_for_runtime_with_context_and_announcements(
            &fixture.config,
            Arc::clone(&fixture.shared),
            fixture.opened.peer_identity_authority.clone(),
            fixture.opened.announcement_outboxes.clone(),
            fixture.opened.network.clone(),
        )
        .await;
    assert_eq!(
        listener.state,
        open_bitcoin_rpc::inbound_listener::InboundListenerState::Listening
    );
    assert_eq!(listener.bound_endpoints.len(), 1);
    let endpoint: std::net::SocketAddr = listener.bound_endpoints[0]
        .parse()
        .expect("actual endpoint");
    assert!(endpoint.ip().is_loopback() && endpoint.port() != 0);
    let magic = fixture.config.sync.runtime.network.magic();
    let network = fixture.opened.network.clone();
    let general = fixture.config.peer_block_filters;
    let decision = fixture
        .config
        .inbound
        .permission_classes
        .resolve_inbound(endpoint.ip())
        .with_basic_filter_serving(general);
    let capability = decision.basic_filter_capability();
    let mut client = tokio::spawn(async move {
        eprintln!("phase162 inbound stage: connecting general={general} granted={granted}");
        let stream = TcpStream::connect(endpoint)
            .await
            .expect("actual configured socket");
        let services = handshake(&stream, magic).await;
        eprintln!("phase162 inbound stage: handshake_complete=true");
        assert_eq!(services.contains(ServiceFlags::COMPACT_FILTERS), granted);
        assert!(services.contains(ServiceFlags::NETWORK_LIMITED));
        assert!(!services.contains(ServiceFlags::NETWORK));
        let peers = network
            .peer_manager_snapshot()
            .expect("actual admitted peer");
        assert_eq!(peers.inbound_admission_counters().current_inbound_peers, 1);
        let scope = peers
            .maybe_basic_peer_scope()
            .expect("effective live scope");
        assert_eq!(scope.general_sessions, u64::from(general));
        assert_eq!(
            scope.explicit_sessions,
            u64::from(capability.explicit_permission_requested)
        );
        assert_eq!(
            scope.all_sessions,
            u64::from(capability.all_permission_requested)
        );
        assert_eq!(scope.offered_sessions, u64::from(granted));
        let mut physical_frames = 0;
        let mut physical_bytes = 0;
        for (index, exchange) in exchanges.into_iter().enumerate() {
            eprintln!(
                "phase162 inbound stage: exchange={index} command={} expected_frames={} granted={granted}",
                exchange.request.command_name(),
                exchange.expected.len()
            );
            send(&stream, magic, exchange.request).await;
            if !granted {
                denied_disconnect(&stream).await;
                break;
            }
            if granted {
                for expected in exchange.expected {
                    let actual = read_frame(&stream).await;
                    assert_eq!(actual, expected, "independent exact physical frame {index}");
                    physical_frames += 1;
                    physical_bytes += actual.len();
                }
            }
            sentinel(&stream, magic, 162_100 + index as u64).await;
            assert_eq!(
                network
                    .basic_filter_transport_snapshot()
                    .expect("post-pong completed caller"),
                FilterTransportSnapshot::default()
            );
        }
        (physical_frames, physical_bytes)
    });
    let outcome = tokio::time::timeout(Duration::from_secs(30), &mut client).await;
    if !matches!(outcome, Ok(Ok(_)))
        && let Some(worker) = listener.maybe_worker.as_ref()
    {
        let evidence = worker.evidence();
        eprintln!(
            "phase162 inbound failure: admitted={} rejected={} resource_rejections={} timeout_disconnects={} latest_event={:?} admission_reason={:?} resource_reason={:?}",
            evidence.admitted_inbound_peers,
            evidence.rejected_inbound_peers,
            evidence.resource_rejections,
            evidence.timeout_disconnects,
            evidence.maybe_latest_admission_event,
            evidence.maybe_admission_reject_reason,
            evidence
                .maybe_latest_resource_event
                .as_ref()
                .map(|event| &event.reason)
        );
    }
    if outcome.is_err() {
        client.abort();
        let _cancelled = client.await;
    }
    let network = &fixture.opened.network;
    let clean = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if network
                .peer_manager_snapshot()
                .expect("disconnect observation")
                .inbound_admission_counters()
                .current_inbound_peers
                == 0
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await;
    // Shutdown/join runs even when the bounded client panics or reaches its deadline.
    tokio::time::timeout(Duration::from_secs(5), listener.shutdown())
        .await
        .expect("bounded listener join");
    clean.expect("actual disconnected session cleanup");
    let (frames, bytes) = outcome
        .expect("finite physical session")
        .expect("physical peer assertions");
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("final released charges"),
        FilterTransportSnapshot::default()
    );
    assert_eq!((frames, bytes), (expected_frames, expected_bytes));
    assert_eq!(frames, if granted { 20 } else { 0 });
    if !granted {
        eprintln!(
            "phase162 retained inbound denial: client_achieved_frames=0 client_achieved_bytes=0 pre_request_fenced=true terminal_disconnect=true charges=0 peer_removed=true joined=true"
        );
        return;
    }
    eprintln!(
        "phase162 retained inbound: general={general} explicit={} all={} granted={granted} client_achieved_frames={frames} client_achieved_bytes={bytes} sentinel_fenced=true charges=0 joined=true",
        capability.explicit_permission_requested, capability.all_permission_requested
    );
}

pub(crate) async fn assert_phase162_inbound(
    mut fixture: Phase162Fixture,
    settled: &retention::SettledIdentity,
) -> Phase162Fixture {
    let exchanges = exchanges(&fixture);
    for (general, listener, permissions, granted) in [
        (true, false, vec!["in"], true),
        (true, true, vec!["in"], true),
        (false, true, vec!["in", "blockfilters"], true),
        (false, true, vec!["in", "all"], true),
        (false, true, vec!["in"], false),
    ] {
        fixture = Phase162Fixture::open_serving(fixture.close(), general, listener, &permissions);
        settled.assert_unchanged(&fixture);
        for name in [
            "active-19",
            "active-20",
            "active-21",
            "active-713",
            "active-714",
        ] {
            retention::assert_pair(&fixture, oracle_hash(oracle_case(&fixture, name)), false);
        }
        if listener {
            if granted {
                physical_session(&fixture, exchanges.clone(), granted).await;
            } else {
                for exchange in exchanges.clone() {
                    physical_session(&fixture, vec![exchange], granted).await;
                    settled.assert_unchanged(&fixture);
                }
            }
        } else {
            let mut disabled = crate::inbound_startup::start_inbound_listener_for_runtime_with_context_and_announcements(
                &fixture.config, Arc::clone(&fixture.shared), fixture.opened.peer_identity_authority.clone(),
                fixture.opened.announcement_outboxes.clone(), fixture.opened.network.clone()).await;
            assert_eq!(
                disabled.state,
                open_bitcoin_rpc::inbound_listener::InboundListenerState::Disabled
            );
            assert!(
                disabled.bound_endpoints.is_empty(),
                "serving alone never starts listeners"
            );
            disabled.shutdown().await;
        }
        settled.assert_unchanged(&fixture);
    }
    let fixture = Phase162Fixture::open_with_prune(fixture.close(), 550);
    settled.assert_unchanged(&fixture);
    fixture
}

#[tokio::test]
async fn phase162_inbound_no_grant_all_families() {
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            crate::tests::remove_dir_if_exists(&self.0);
        }
    }
    // Arrange: genuine normal-maturity history; no filter oracle or pressure stand-in.
    let started = std::time::Instant::now();
    let path = crate::tests::temp_store_path("phase162-no-grant");
    std::fs::create_dir(&path).expect("unique denial datadir");
    let cleanup = Cleanup(path.clone());
    let config = SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        ..Default::default()
    };
    let params = config.network.consensus_params();
    let recipe = recipe::Recipe::new(params, ScriptVerifyFlags::P2SH, 102);
    let (runtime, genesis) = live_source::live_genesis_runtime(
        &path,
        &recipe.blocks[0],
        config,
        params,
        recipe.flags,
        open_bitcoin_node::chainstate::BasicFilterStartupMode::Disabled,
    );
    live_source::admit_descendant_headers(&runtime, &recipe.blocks[1..], recipe.flags, params);
    assert_eq!(recipe.params.coinbase_maturity, 100);
    let mut receipts = Vec::new();
    let mut inputs = Vec::new();
    let store = runtime.store();
    let handle = runtime.network_handle();
    for (height, block) in recipe.blocks.iter().enumerate() {
        let receipt = if height == 0 {
            genesis.clone()
        } else {
            store.save_block(block, PersistMode::Sync).expect("body");
            handle
                .connect_local_block(block, recipe.flags, params)
                .expect("accept")
        };
        assert_eq!(receipt.height as usize, height);
        assert_eq!(receipt.block_hash, block_hash(&block.header));
        receipts.push(receipt);
        inputs.push(json!({"name":format!("active-{height}"), "previous_name":if height == 0 {String::new()} else {format!("active-{}", height - 1)}, "raw_block":super::hex(&encode_block(block).expect("raw input"))}));
    }
    assert!(
        handle
            .flush_coins(FlushMode::Always, FlushPolicyTime::new(500_000), u64::MAX)
            .expect("ordinary accepted settlement")
            .wrote_coins
    );
    let digest = super::hex(&Sha256::digest(&serde_json::to_vec(&inputs).expect("raw")));
    drop((handle, runtime));
    let provenance = Provenance {
        recipe_id: recipe::RECIPE_ID.to_owned(),
        ordered_input_digest: digest,
        receipts,
        oracle: Value::Null,
        seed_ms: started.elapsed().as_millis(),
    };
    let handoff = Phase162Handoff {
        path,
        recipe,
        provenance,
    };
    let fixture = Phase162Fixture::open_serving(handoff, false, true, &["in"]);
    let stop_hash = fixture.provenance.receipts[101].block_hash;
    // Act / Assert: each denied family owns a fresh physical connection and joined listener.
    for request in [
        WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type: 0,
            start_height: 100,
            stop_hash,
        }),
        WireNetworkMessage::GetCFHeaders(GetCFHeaders {
            filter_type: 0,
            start_height: 100,
            stop_hash,
        }),
        WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
            filter_type: 0,
            stop_hash,
        }),
    ] {
        let exchange = Exchange {
            request,
            expected: Vec::new(),
        };
        physical_session(&fixture, vec![exchange], false).await;
    }
    let handoff = fixture.close();
    drop(FjallNodeStore::open(&handoff.path).expect("all handles closed"));
    drop(cleanup);
    assert!(!handoff.path.exists());
}
