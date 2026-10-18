// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Public outbound TCP caller on the consuming same-datadir handoff.

use super::{
    core_types::{chainstate::CoinsView, primitives::NetworkMagic},
    decode_hex,
    fixtures::{Phase162Fixture, Phase162Handoff},
    process, recipe,
};
use open_bitcoin_network::{
    GetCFCheckpt, GetCFHeaders, GetCFilters, ParsedNetworkMessage, ServiceFlags, VersionMessage,
    WireNetworkMessage,
};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallNodeStore, SyncNetwork, SyncPeerAddress, SyncRunSummary,
    SyncRuntimeConfig, TcpPeerTransport, chainstate::BasicFilterStartupMode,
    network::filter_transport::FilterTransportSnapshot,
};
use serde_json::{Value, json};
use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    process::Command,
    thread,
    time::{Duration, Instant},
};

const DEADLINE: Duration = Duration::from_secs(20);
const MAX_FRAME_BYTES: usize = 16 * 1024;

struct Exchange {
    request: WireNetworkMessage,
    expected: Vec<Vec<u8>>,
}

#[derive(Debug, Default)]
pub(crate) struct Observed {
    pub requests: usize,
    pub frames: usize,
    pub bytes: usize,
    pub elapsed_ms: u128,
}

fn check(condition: bool, message: &'static str) -> io::Result<()> {
    if condition {
        Ok(())
    } else {
        Err(io::Error::other(message))
    }
}

fn case<'a>(oracle: &'a Value, name: &str) -> &'a Value {
    oracle["cases"]
        .as_array()
        .expect("raw input cases")
        .iter()
        .find(|row| row["name"] == name)
        .expect("named raw ancestry")
}

fn wire(oracle: &Value, name: &str) -> Vec<u8> {
    let row = oracle["wire"]
        .as_array()
        .expect("independent frames")
        .iter()
        .find(|row| row["name"] == name)
        .expect("named oracle frame");
    decode_hex(row["expected"]["frame_hex"].as_str().expect("pinned frame"))
}

fn old_headers(oracle: &Value, names: &[&str]) -> Vec<u8> {
    let corpus = json!({"wire":[{"name":"outbound-old-headers","command":"cfheaders","input":{
        "filter_type":0,"stop_hash_raw":case(oracle,names[names.len()-1])["block_hash_raw"],
        "prev_header_raw":case(oracle,names[0])["predecessor_raw"],
        "hashes_raw":names.iter().map(|name|case(oracle,name)["hash_raw"].clone()).collect::<Vec<_>>()}}],
        "proof":[],"handlers":[],"malformed":[]});
    let mut command = Command::new("bun");
    command.current_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .args(["--eval", "import {runBounded,validateResult,PIN} from './scripts/generate-peer-filter-fixtures.ts'; const root=process.cwd(); if(runBounded('git',['-C','packages/bitcoin-knots','rev-parse','HEAD'],root).trim()!==PIN) throw Error('pin mismatch'); const input=await Bun.stdin.text(); process.stdout.write(validateResult(runBounded('python3',['scripts/basic-filter-oracle.py','--peer-wire-proof'],root,input),JSON.parse(input)).wire);"]);
    let output = process::run(
        &mut command,
        serde_json::to_vec(&corpus).expect("oracle input"),
        process::Limits {
            input_bytes: 64 * 1024,
            stdout_bytes: 64 * 1024,
            stderr_bytes: 4096,
            deadline: DEADLINE,
        },
    )
    .expect("bounded pinned serializer");
    assert!(output.status.success() && output.reaped);
    let result: Value = serde_json::from_slice(&output.stdout).expect("validated independent wire");
    let mut bytes = decode_hex(
        result["cases"][0]["expected"]["frame_hex"]
            .as_str()
            .expect("frame"),
    );
    bytes[..4].copy_from_slice(&SyncNetwork::Regtest.magic().to_bytes());
    bytes
}

fn exchanges(oracle: &Value) -> Vec<Exchange> {
    let mut exchanges = Vec::new();
    for (start_height, names) in [
        (19, ["active-19", "active-20", "active-21"]),
        (713, ["active-713", "active-714", "active-715"]),
    ] {
        let stop_hash = super::oracle_hash(case(oracle, names[2]));
        exchanges.push(Exchange {
            request: WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height,
                stop_hash,
            }),
            expected: names
                .iter()
                .map(|name| wire(oracle, &format!("{name}-cfilter")))
                .collect(),
        });
        exchanges.push(Exchange {
            request: WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height,
                stop_hash,
            }),
            expected: vec![old_headers(oracle, &names)],
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
        let stop_hash = super::oracle_hash(case(oracle, leaf));
        exchanges.push(Exchange {
            request: WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height,
                stop_hash,
            }),
            expected: names
                .iter()
                .map(|name| wire(oracle, &format!("{name}-cfilter")))
                .collect(),
        });
        exchanges.push(Exchange {
            request: WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height,
                stop_hash,
            }),
            expected: vec![wire(oracle, &format!("{leaf}-cfheaders"))],
        });
        let expected = wire(oracle, &format!("{leaf}-cfcheckpt"));
        let WireNetworkMessage::CFCheckpt(reply) = ParsedNetworkMessage::decode_wire(&expected)
            .expect("independent checkpoint frame")
            .message
        else {
            panic!("cfcheckpt frame")
        };
        assert_eq!(
            reply.filter_headers.len(),
            1,
            "positive h1000 ancestry required"
        );
        assert_eq!(
            super::hex(reply.filter_headers[0].as_bytes()),
            case(oracle, checkpoint)["header_raw"]
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
        case(oracle, "active-1000")["header_raw"],
        case(oracle, "fork-1000")["header_raw"]
    );
    exchanges
}

fn config(endpoint: SocketAddr, requests: usize) -> SyncRuntimeConfig {
    assert!(endpoint.ip().is_loopback() && endpoint.port() != 0);
    let config = SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        manual_peers: vec![SyncPeerAddress::manual("127.0.0.1", endpoint.port())],
        dns_seeds: Vec::new(),
        target_outbound_peers: 1,
        connect_timeout_ms: 2_000,
        read_timeout_ms: 2_000,
        max_messages_per_peer: requests + 3,
        max_rounds: 1,
        max_peer_retries: 0,
        ..Default::default()
    };
    assert_eq!(config.candidate_peers(), config.manual_peers);
    assert_eq!(config.manual_peers.len(), 1);
    assert_eq!(config.manual_peers[0].host, "127.0.0.1");
    assert!(config.dns_seeds.is_empty());
    assert_eq!(
        config.network.magic(),
        NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda])
    );
    config
}

fn remaining(started: Instant) -> io::Result<Duration> {
    DEADLINE
        .checked_sub(started.elapsed())
        .filter(|left| !left.is_zero())
        .ok_or_else(|| io::Error::new(io::ErrorKind::TimedOut, "total peer deadline"))
}

fn read_frame(stream: &mut TcpStream, started: Instant) -> io::Result<Vec<u8>> {
    let mut bytes = vec![0; 24];
    exact(stream, started, &mut bytes)?;
    let header = open_bitcoin_codec::parse_message_header(&bytes).map_err(io::Error::other)?;
    check(
        header.magic == SyncNetwork::Regtest.magic(),
        "Regtest frame identity",
    )?;
    let size = header.payload_size as usize;
    check(size <= MAX_FRAME_BYTES, "bounded physical response")?;
    bytes.resize(24 + size, 0);
    exact(stream, started, &mut bytes[24..])?;
    ParsedNetworkMessage::decode_wire(&bytes).map_err(io::Error::other)?;
    Ok(bytes)
}

fn exact(stream: &mut TcpStream, started: Instant, mut bytes: &mut [u8]) -> io::Result<()> {
    while !bytes.is_empty() {
        stream.set_read_timeout(Some(remaining(started)?))?;
        match stream.read(bytes) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "incomplete physical frame",
                ));
            }
            Ok(count) => bytes = &mut bytes[count..],
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn send(stream: &mut TcpStream, started: Instant, message: WireNetworkMessage) -> io::Result<()> {
    let frame = message
        .encode_wire(SyncNetwork::Regtest.magic())
        .map_err(io::Error::other)?;
    let mut bytes = frame.as_slice();
    while !bytes.is_empty() {
        stream.set_write_timeout(Some(remaining(started)?))?;
        match stream.write(bytes) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "incomplete request write",
                ));
            }
            Ok(count) => bytes = &bytes[count..],
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn handshake(stream: &mut TcpStream, started: Instant) -> io::Result<()> {
    let WireNetworkMessage::Version(version) =
        ParsedNetworkMessage::decode_wire(&read_frame(stream, started)?)
            .map_err(io::Error::other)?
            .message
    else {
        return Err(io::Error::other("version first"));
    };
    check(
        version.services.contains(ServiceFlags::COMPACT_FILTERS),
        "BASIC service advertised",
    )?;
    check(
        version.services.contains(ServiceFlags::NETWORK_LIMITED),
        "pruned service advertised",
    )?;
    check(
        !version.services.contains(ServiceFlags::NETWORK),
        "no archive service",
    )?;
    send(
        stream,
        started,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 162_006,
            ..Default::default()
        }),
    )?;
    let mut acknowledged = false;
    for _ in 0..8 {
        let message = ParsedNetworkMessage::decode_wire(&read_frame(stream, started)?)
            .map_err(io::Error::other)?
            .message;
        if message == WireNetworkMessage::Verack {
            acknowledged = true;
            break;
        }
    }
    check(acknowledged, "finite version/verack handshake")?;
    send(stream, started, WireNetworkMessage::Verack)?;
    send(stream, started, WireNetworkMessage::Ping { nonce: 162_007 })?;
    for _ in 0..8 {
        let message = ParsedNetworkMessage::decode_wire(&read_frame(stream, started)?)
            .map_err(io::Error::other)?
            .message;
        if message == (WireNetworkMessage::Pong { nonce: 162_007 }) {
            return Ok(());
        }
        check(
            !matches!(
                message,
                WireNetworkMessage::CFilter(_)
                    | WireNetworkMessage::CFHeaders(_)
                    | WireNetworkMessage::CFCheckpt(_)
            ),
            "no unsolicited filter",
        )?;
    }
    Err(io::Error::other("handshake fence missing"))
}

fn remote(listener: TcpListener, exchanges: Vec<Exchange>) -> io::Result<Observed> {
    let started = Instant::now();
    let (mut stream, endpoint) = loop {
        remaining(started)?;
        match listener.accept() {
            Ok(accepted) => break accepted,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(2))
            }
            Err(error) => return Err(error),
        }
    };
    check(endpoint.ip().is_loopback(), "loopback caller only")?;
    stream.set_nonblocking(false)?;
    handshake(&mut stream, started)
        .map_err(|error| io::Error::other(format!("physical handshake: {error}")))?;
    let mut observed = Observed::default();
    for exchange in exchanges {
        send(&mut stream, started, exchange.request)?;
        observed.requests += 1;
        for expected in exchange.expected {
            let actual = read_frame(&mut stream, started).map_err(|error| {
                io::Error::other(format!("filter response {}: {error}", observed.requests))
            })?;
            check(
                actual == expected,
                "complete physical frame differs from independent oracle",
            )?;
            observed.frames += 1;
            observed.bytes += actual.len();
        }
    }
    // The exact configured message cap closes the session after its final request.
    stream.set_read_timeout(Some(remaining(started)?))?;
    let mut tail = [0; 1];
    let tail_count = stream
        .read(&mut tail)
        .map_err(|error| io::Error::other(format!("finite session close: {error}")))?;
    check(tail_count == 0, "no extra frame after final complete reply")?;
    observed.elapsed_ms = started.elapsed().as_millis();
    Ok(observed)
}

fn open(handoff: &Phase162Handoff, config: SyncRuntimeConfig) -> DurableSyncRuntime {
    let args = [
        format!("-datadir={}", handoff.path.display()).into(),
        "-regtest".into(),
        "-blockfilterindex=basic".into(),
        "-peerblockfilters=1".into(),
        "-openbitcoininbound=0".into(),
    ];
    let policy = open_bitcoin_rpc::config::load_runtime_config_for_args(&args, &handoff.path)
        .expect("existing parsed same-datadir activation policies");
    assert_eq!(policy.sync.runtime.network, config.network);
    assert!(policy.peer_block_filters && !policy.inbound.enabled && !policy.inbound.allow_public);
    let runtime = DurableSyncRuntime::open_with_configured_runtime_activation(
        FjallNodeStore::open(&handoff.path).expect("daemon handles fully closed"),
        config,
        policy.relay,
        policy.block_serving,
        policy.inbound.enabled,
        BasicFilterStartupMode::Enabled,
    )
    .expect("public configured outbound runtime");
    let mut network = runtime.network_handle();
    network
        .set_prune_mode(policy.prune_mode)
        .expect("preserved configured prune mode");
    network
        .configure_basic_peer_serving(policy.peer_block_filters)
        .expect("explicit BASIC serving policy");
    assert_eq!(
        network.prune_mode().expect("actual prune mode"),
        policy.prune_mode
    );
    assert_eq!(runtime.config().network, SyncNetwork::Regtest);
    assert_eq!(
        runtime.config().network.consensus_params(),
        handoff.recipe.params
    );
    super::fixtures::admit_retained_header_proof(
        &network,
        runtime.store(),
        &handoff.provenance.receipts,
        handoff.recipe.params,
    );
    assert!(
        FjallNodeStore::open(&handoff.path).is_err(),
        "runtime physically owns datadir"
    );
    runtime
}

fn run(handoff: Phase162Handoff, exchanges: Vec<Exchange>) -> (Phase162Handoff, Observed) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("ephemeral loopback remote");
    listener.set_nonblocking(true).expect("bounded accept");
    let endpoint = listener.local_addr().expect("actual endpoint");
    let mut runtime = open(&handoff, config(endpoint, exchanges.len()));
    assert_retained_history(&runtime, &handoff);
    let before = runtime
        .store()
        .hydrate_chainstate_for_open()
        .expect("before metadata")
        .expect("accepted durable state");
    let protection = runtime
        .store()
        .load_prune_protection()
        .expect("before checkpoint");
    let peer = thread::spawn(move || remote(listener, exchanges));
    let result = catch_unwind(AssertUnwindSafe(|| {
        runtime.sync_once(&mut TcpPeerTransport, 500_006)
    }));
    // Join on successful/error/panic paths before inspecting any captured task outcome.
    let joined = peer.join();
    let maybe_after = runtime.store().hydrate_chainstate_for_open();
    let after_protection = runtime.store().load_prune_protection();
    let transport = runtime.network_handle().basic_filter_transport_snapshot();
    let peers = runtime.network_handle().peer_manager_snapshot();
    let outboxes = runtime.announcement_outboxes().snapshots();
    let retention = catch_unwind(AssertUnwindSafe(|| {
        assert_retained_history(&runtime, &handoff)
    }));
    drop(runtime);
    drop(FjallNodeStore::open(&handoff.path).expect("outbound runtime handles closed"));
    let summary = match result {
        Ok(summary) => summary.expect("public sync_once result"),
        Err(panic) => resume_unwind(panic),
    };
    let observed = joined
        .expect("remote joined before outcome assertion")
        .expect("physical remote completed");
    assert_summary(&summary, observed.requests);
    assert_eq!(
        maybe_after
            .expect("after metadata")
            .expect("accepted durable state"),
        before
    );
    assert_eq!(after_protection.expect("after checkpoint"), protection);
    assert_eq!(
        transport.expect("all charges settled"),
        FilterTransportSnapshot::default()
    );
    assert!(peers.expect("settled peers").peer_ids().is_empty());
    assert!(outboxes.expect("settled outboxes").is_empty());
    if let Err(panic) = retention {
        resume_unwind(panic);
    }
    (handoff, observed)
}

fn assert_retained_history(runtime: &DurableSyncRuntime, handoff: &Phase162Handoff) {
    let snapshot = runtime
        .store()
        .hydrate_chainstate_for_open()
        .expect("actual durable metadata")
        .expect("accepted chainstate");
    assert_eq!(
        runtime.store().coins_view().best_block().expect("coins B"),
        snapshot
            .active_chain
            .last()
            .map(|position| position.block_hash)
    );
    assert!(
        runtime
            .store()
            .coins_view()
            .head_blocks()
            .expect("settled heads")
            .is_empty()
    );
    if snapshot.active_chain.len() != 1003 {
        assert_eq!(snapshot.active_chain.len(), 102, "short preflight only");
        return;
    }
    assert_eq!(
        snapshot.active_chain.last().expect("tip").block_hash,
        super::oracle_hash(case(&handoff.provenance.oracle, "fork-1002"))
    );
    assert!(
        runtime
            .store()
            .load_have_pruned()
            .expect("earned prune bit")
    );
    for position in &snapshot.active_chain {
        let present = position.height >= 715;
        assert_eq!(
            runtime
                .store()
                .load_block(position.block_hash)
                .expect("actual body")
                .is_some(),
            present
        );
        assert_eq!(
            runtime
                .store()
                .load_undo(position.block_hash)
                .expect("actual undo")
                .is_some(),
            present
        );
    }
}

fn assert_summary(summary: &SyncRunSummary, requests: usize) {
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.failed_peers, 0);
    assert_eq!(summary.messages_processed, requests + 3);
    assert_eq!(summary.peer_outcomes.len(), 1);
    assert_eq!(summary.peer_outcomes[0].network, SyncNetwork::Regtest);
    assert_eq!(summary.peer_outcomes[0].attempts, 1);
    assert!(summary.peer_outcomes[0].maybe_failure_reason.is_none());
}

pub(crate) fn assert_outbound(handoff: Phase162Handoff) -> (Phase162Handoff, Observed) {
    assert_eq!(handoff.provenance.recipe_id, recipe::RECIPE_ID);
    assert_eq!(
        handoff.provenance.oracle["recipe_id"],
        handoff.provenance.recipe_id
    );
    assert_eq!(
        handoff.provenance.oracle["ordered_input_digest"],
        handoff.provenance.ordered_input_digest
    );
    let exchanges = exchanges(&handoff.provenance.oracle);
    let (handoff, observed) = run(handoff, exchanges);
    assert_eq!(observed.requests, 10);
    assert_eq!(observed.frames, 20);
    assert_eq!(observed.bytes, 1890);
    (handoff, observed)
}

#[test]
fn phase162_outbound_public_handoff_short_preflight() {
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            crate::tests::remove_dir_if_exists(&self.0);
        }
    }
    // Arrange: accepted normal-maturity prefix; no storage-pressure corpus.
    let cleanup;
    let fixture = Phase162Fixture::seed_count(102);
    let path = fixture.path.clone();
    cleanup = Cleanup(path.clone());
    assert!(FjallNodeStore::open(&path).is_err());
    // Act: actual public dial, real version/verack and fence, then finite disconnect.
    let result = catch_unwind(AssertUnwindSafe(|| run(fixture.close(), Vec::new())));
    // Assert only after handles and peer have settled, including the RED path.
    drop(FjallNodeStore::open(&path).expect("cold-open after settlement"));
    std::fs::remove_dir_all(&path).expect("preflight cleanup");
    drop(cleanup);
    assert!(!path.exists());
    let (_handoff, observed) = match result {
        Ok(value) => value,
        Err(panic) => resume_unwind(panic),
    };
    assert_eq!(observed.frames, 0);
    assert_eq!(observed.bytes, 0);
}
