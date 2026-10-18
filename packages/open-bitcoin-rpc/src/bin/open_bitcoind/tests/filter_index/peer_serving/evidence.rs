// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Genuine configured daemon authority and independent concrete wire observations.

use super::super::fixtures::DaemonFixture;
use super::*;
use open_bitcoin_network::{GetCFCheckpt, GetCFHeaders, GetCFilters, ParsedNetworkMessage};
use open_bitcoin_node::core::consensus::block_hash;
use std::{
    io::{Read, Write},
    time::Duration,
};

fn read_frame(stream: &mut std::net::TcpStream) -> Vec<u8> {
    let mut bytes = vec![0; 24];
    stream.read_exact(&mut bytes).expect("actual frame header");
    let size = open_bitcoin_codec::parse_message_header(&bytes)
        .expect("header")
        .payload_size as usize;
    assert!(size <= 4_000_000);
    bytes.resize(24 + size, 0);
    stream
        .read_exact(&mut bytes[24..])
        .expect("actual frame payload");
    bytes
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn independent_wire(
    recipes: serde_json::Value,
    magic: open_bitcoin_node::core::primitives::NetworkMagic,
) -> Vec<Vec<u8>> {
    use std::process::{Command, Stdio};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");
    let input = serde_json::to_vec(
        &serde_json::json!({"wire":recipes,"proof":[],"handlers":[],"malformed":[]}),
    )
    .expect("public inputs");
    assert!(input.len() < 8 * 1024 * 1024);
    // Thin glue delegates deadline/capture/input limits to the existing Bun driver.
    let mut helper=Command::new("bun").current_dir(&root).args(["--eval","import {runBounded,PIN} from './scripts/generate-peer-filter-fixtures.ts'; const root=process.cwd(); if(runBounded('git',['-C','packages/bitcoin-knots','rev-parse','HEAD'],root).trim()!==PIN) throw Error('pin mismatch'); process.stdout.write(runBounded('python3',['scripts/basic-filter-oracle.py','--peer-wire-proof'],root,await Bun.stdin.text()));"]).env("PYTHONDONTWRITEBYTECODE","1").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().expect("unchanged bounded Bun helper");
    helper
        .stdin
        .take()
        .expect("stdin")
        .write_all(&input)
        .expect("bounded input");
    let output = helper.wait_with_output().expect("helper complete");
    assert!(output.status.success());
    assert!(output.stdout.len() < 16 * 1024 * 1024);
    let output: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("independent expectations");
    output["wire"]
        .as_array()
        .expect("wire rows")
        .iter()
        .map(|row| {
            let raw = row["expected"]["frame_hex"].as_str().expect("hex");
            let mut bytes: Vec<u8> = raw
                .as_bytes()
                .chunks_exact(2)
                .map(|pair| {
                    u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII"), 16).expect("byte")
                })
                .collect();
            bytes[..4].copy_from_slice(&magic.to_bytes());
            bytes
        })
        .collect()
}

fn expectations(fixture: &DaemonFixture, start: usize, stop: usize) -> Vec<Vec<u8>> {
    use serde_json::json;
    let records: Vec<_> = (0..=stop)
        .map(|height| {
            let open_bitcoin_node::BasicFilterQuery::Found(record) = fixture
                .opened
                .network
                .basic_filter_query(block_hash(&fixture.blocks[height].header))
                .expect("genuine persisted row")
            else {
                panic!("complete row")
            };
            record
        })
        .collect();
    let stop_hash = hex(block_hash(&fixture.blocks[stop].header).as_bytes());
    let mut recipes:Vec<_>=records[start..].iter().enumerate().map(|(offset,row)|json!({"name":format!("filter-{offset}"),"command":"cfilter","input":{"filter_type":0,"block_hash_raw":hex(row.identity().block_hash().as_bytes()),"filter_data":hex(row.encoded_bytes())}})).collect();
    recipes.push(json!({"name":"headers","command":"cfheaders","input":{"filter_type":0,"stop_hash_raw":stop_hash,"prev_header_raw":if start==0 {"00".repeat(32)}else{hex(records[start-1].filter_header().as_bytes())},"hashes_raw":records[start..].iter().map(|row|hex(row.identity().filter_hash().as_bytes())).collect::<Vec<_>>()}}));
    recipes.push(json!({"name":"checkpoints","command":"cfcheckpt","input":{"filter_type":0,"stop_hash_raw":stop_hash,"headers_raw":(1000..=stop).step_by(1000).map(|height|hex(records[height].filter_header().as_bytes())).collect::<Vec<_>>()}}));
    independent_wire(json!(recipes), fixture.config.sync.runtime.network.magic())
}

async fn compare_families(
    fixture: &DaemonFixture,
    start: u32,
    stop: open_bitcoin_node::core::primitives::BlockHash,
    expected: Vec<Vec<u8>>,
) {
    let magic = fixture.config.sync.runtime.network.magic();
    with_peer(fixture, move |socket, _| {
        let mut actual = Vec::new();
        for (request, count) in [
            (
                WireNetworkMessage::GetCFilters(GetCFilters {
                    filter_type: 0,
                    start_height: start,
                    stop_hash: stop,
                }),
                expected.len() - 2,
            ),
            (
                WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                    filter_type: 0,
                    start_height: start,
                    stop_hash: stop,
                }),
                1,
            ),
            (
                WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                    filter_type: 0,
                    stop_hash: stop,
                }),
                1,
            ),
        ] {
            socket
                .write_all(&request.encode_wire(magic).expect("request"))
                .expect("physical request");
            for _ in 0..count {
                actual.push(read_frame(socket));
            }
        }
        assert_eq!(
            actual, expected,
            "genuine configured bytes versus pinned independent serializers"
        );
    })
    .await;
}

#[tokio::test]
async fn phase160_peer_evidence_configured_genuine_catchup_exact_families_and_missing() {
    // Arrange: real startup leaves a complete short prefix and a later accepted gap.
    let seed = DaemonFixture::new(40);
    let (path, blocks) = seed.close();
    let fixture = DaemonFixture::open_serving(path, blocks, true, true, &["in"]);
    assert_eq!(
        fixture.blocks[2].transactions.len(),
        3,
        "actual historical and same-block spends"
    );
    assert!(
        !fixture
            .opened
            .network
            .maybe_basic_index_summary()
            .expect("owner")
            .expect("enabled")
            .synced
    );
    let before = fixture
        .opened
        .network
        .maybe_basic_index_summary()
        .expect("progress");
    compare_families(
        &fixture,
        0,
        block_hash(&fixture.blocks[2].header),
        expectations(&fixture, 0, 2),
    )
    .await;
    let stop = block_hash(&fixture.blocks[39].header);
    let magic = fixture.config.sync.runtime.network.magic();
    with_peer(&fixture, move |socket, _| {
        for request in [
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 0,
                stop_hash: stop,
            }),
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: 0,
                stop_hash: stop,
            }),
        ] {
            socket
                .write_all(&request.encode_wire(magic).expect("cross-prefix request"))
                .expect("request");
            socket
                .write_all(
                    &WireNetworkMessage::Ping { nonce: 160_117 }
                        .encode_wire(magic)
                        .expect("sentinel"),
                )
                .expect("sentinel write");
            assert_eq!(
                ParsedNetworkMessage::decode_wire(&read_frame(socket))
                    .expect("only sentinel")
                    .message,
                WireNetworkMessage::Pong { nonce: 160_117 }
            );
        }
    })
    .await;
    // Assert: peer queries perform no owner repair or request-time generation.
    assert_eq!(
        fixture
            .opened
            .network
            .maybe_basic_index_summary()
            .expect("unchanged"),
        before
    );
    fixture.cleanup();
}

async fn with_peer(
    fixture: &DaemonFixture,
    action: impl FnOnce(&mut std::net::TcpStream, ServiceFlags) + Send + 'static,
) {
    let mut listener =
        crate::inbound_startup::start_inbound_listener_for_runtime_with_context_and_announcements(
            &fixture.config,
            Arc::clone(&fixture.shared),
            fixture.opened.peer_identity_authority.clone(),
            fixture.opened.announcement_outboxes.clone(),
            fixture.opened.network.clone(),
        )
        .await;
    assert_eq!(listener.state, InboundListenerState::Listening);
    let endpoint = listener.bound_endpoints[0].clone();
    let magic = fixture.config.sync.runtime.network.magic();
    tokio::task::spawn_blocking(move || {
        let mut socket = std::net::TcpStream::connect(endpoint).expect("actual configured inbound");
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("bounded read");
        socket
            .set_write_timeout(Some(Duration::from_secs(5)))
            .expect("bounded write");
        socket
            .write_all(
                &WireNetworkMessage::Version(VersionMessage {
                    nonce: 160_111,
                    ..Default::default()
                })
                .encode_wire(magic)
                .expect("remote version"),
            )
            .expect("version write");
        let WireNetworkMessage::Version(version) =
            ParsedNetworkMessage::decode_wire(&read_frame(&mut socket))
                .expect("actual version")
                .message
        else {
            panic!("version")
        };
        loop {
            if ParsedNetworkMessage::decode_wire(&read_frame(&mut socket))
                .expect("ack")
                .message
                == WireNetworkMessage::Verack
            {
                break;
            }
        }
        socket
            .write_all(&WireNetworkMessage::Verack.encode_wire(magic).expect("ack"))
            .expect("ack write");
        socket
            .write_all(
                &WireNetworkMessage::Ping { nonce: 160_112 }
                    .encode_wire(magic)
                    .expect("sentinel"),
            )
            .expect("sentinel write");
        loop {
            if ParsedNetworkMessage::decode_wire(&read_frame(&mut socket))
                .expect("drain")
                .message
                == (WireNetworkMessage::Pong { nonce: 160_112 })
            {
                break;
            }
        }
        action(&mut socket, version.services);
    })
    .await
    .expect("concrete peer completed");
    listener.shutdown().await;
}

#[tokio::test]
async fn phase160_peer_evidence_configured_version_matrix_before_catchup() {
    for (enabled, general, tokens, expected) in [
        (true, false, vec!["in"], false),
        (true, true, vec!["in"], true),
        (true, false, vec!["in", "blockfilters"], true),
        (true, false, vec!["in", "all"], true),
        (true, false, vec!["in", "all", "blockfilters"], true),
        (true, true, vec!["in", "blockfilters"], true),
        (true, true, vec!["in", "all"], true),
        (true, true, vec!["in", "all", "blockfilters"], true),
        (false, false, vec!["in", "all"], true),
        (false, false, vec!["in"], false),
    ] {
        // Arrange: startup recovers actual durable accepted coins, before owner catchup.
        let seed = DaemonFixture::new(40);
        let (path, blocks) = seed.close();
        let fixture = DaemonFixture::open_serving(path, blocks, enabled, general, &tokens);
        let summary = fixture
            .opened
            .network
            .maybe_basic_index_summary()
            .expect("configured summary");
        if enabled {
            assert!(!summary.expect("enabled index").synced);
        }
        // Act / Assert: ordinary configured listener physically sends its own version.
        with_peer(&fixture, move |_, services| {
            assert_eq!(services.contains(ServiceFlags::COMPACT_FILTERS), expected);
            assert!(services.contains(ServiceFlags::NETWORK_LIMITED));
            assert!(!services.contains(ServiceFlags::NETWORK));
        })
        .await;
        fixture.cleanup();
    }
}

#[tokio::test]
async fn phase160_peer_evidence_configured_all_only_no_backend_all_families() {
    // Arrange
    let seed = DaemonFixture::new(16);
    let (path, blocks) = seed.close();
    let fixture = DaemonFixture::open_serving(path, blocks, false, false, &["in", "all"]);
    let stop = block_hash(&fixture.blocks[2].header);
    let magic = fixture.config.sync.runtime.network.magic();
    // Act / Assert: semantic valid requests produce only the later ping sentinel.
    with_peer(&fixture, move |socket, services| {
        assert!(services.contains(ServiceFlags::COMPACT_FILTERS));
        for request in [
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 0,
                stop_hash: stop,
            }),
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: 0,
                stop_hash: stop,
            }),
            WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                filter_type: 0,
                stop_hash: stop,
            }),
        ] {
            socket
                .write_all(&request.encode_wire(magic).expect("valid request"))
                .expect("actual request");
            socket
                .write_all(
                    &WireNetworkMessage::Ping { nonce: 160_113 }
                        .encode_wire(magic)
                        .expect("sentinel"),
                )
                .expect("sentinel write");
            assert_eq!(
                ParsedNetworkMessage::decode_wire(&read_frame(socket))
                    .expect("only sentinel")
                    .message,
                WireNetworkMessage::Pong { nonce: 160_113 }
            );
        }
        socket
            .write_all(
                &WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                    filter_type: 255,
                    stop_hash: stop,
                })
                .encode_wire(magic)
                .expect("semantic invalid request"),
            )
            .expect("physical invalid request");
        let mut byte = [0; 1];
        match socket.read(&mut byte) {
            Ok(0) => {}
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::ConnectionAborted
                ) => {}
            other => panic!("invalid all-only request must disconnect without response: {other:?}"),
        }
    })
    .await;
    assert_eq!(
        fixture
            .opened
            .network
            .basic_filter_transport_snapshot()
            .expect("no leaked request"),
        open_bitcoin_node::network::filter_transport::FilterTransportSnapshot::default()
    );
    fixture.cleanup();
}

#[tokio::test]
async fn phase160_peer_evidence_configured_retained_stale_and_paired_body_absence() {
    use super::super::fixtures::{daemon_block, params};
    use open_bitcoin_node::core::{
        chainstate::{AnchoredBlock, FlushMode, FlushPolicyTime, PrunePlan},
        consensus::{ScriptVerifyFlags, check_block_header},
    };
    // Arrange:401 genuinely accepted blocks, complete persisted index and real configured listener.
    let seed = DaemonFixture::new(401);
    let (path, blocks) = seed.close();
    let fixture = DaemonFixture::open_serving(path, blocks, true, true, &["in"]).requalify();
    fixture.finish();
    let bodyless = block_hash(&fixture.blocks[20].header);
    let expected_bodyless = expectations(&fixture, 20, 20);
    let stale = block_hash(&fixture.blocks[400].header);
    let expected_stale = expectations(&fixture, 399, 400);
    // Act: actual existing paired delete; the filter remains complete without either mate.
    let deleted = fixture
        .opened
        .network
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::new(50_000),
            u64::MAX,
            &PrunePlan { heights: vec![20] },
            &[],
        )
        .expect("existing paired prune owner");
    assert_eq!(deleted.deleted_block_hashes, vec![bodyless]);
    assert!(
        fixture
            .store
            .load_block(bodyless)
            .expect("body absent")
            .is_none()
    );
    assert!(
        fixture
            .store
            .load_undo(bodyless)
            .expect("undo absent")
            .is_none()
    );
    compare_families(&fixture, 20, bodyless, expected_bodyless).await;
    let snapshot = fixture
        .opened
        .network
        .chainstate_snapshot()
        .expect("accepted undo");
    fixture
        .store
        .save_undo(
            stale,
            &snapshot.undo_by_block[&stale],
            open_bitcoin_node::PersistMode::Sync,
        )
        .expect("retained genuine disconnect undo");
    let mut connected = Vec::new();
    let mut parent = fixture.blocks[399].clone();
    for height in 400..=401 {
        let mut block = daemon_block(Some(&parent), height);
        block.header.time += 1;
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("actual fork PoW");
        fixture
            .store
            .save_block(&block, open_bitcoin_node::PersistMode::Sync)
            .expect("retained competing body");
        connected.push(AnchoredBlock {
            block: block.clone(),
            chain_work: open_bitcoin_node::core::consensus::chain_proof::ProofWork::ZERO,
        });
        parent = block;
    }
    fixture
        .opened
        .network
        .reorg_to_branch(
            &[fixture.blocks[400].clone()],
            &connected,
            open_bitcoin_mempool::ReorgLifecycleContext::new(
                open_bitcoin_mempool::PolicyTime::from_unix_seconds(50_000),
            ),
            ScriptVerifyFlags::P2SH,
            params(),
        )
        .expect("genuine accepted competing branch");
    for _ in 0..4 {
        fixture
            .opened
            .network
            .drive_basic_filter_index_turn()
            .expect("ordinary catchup");
    }
    // Assert: stale requested ancestry matches pre-reorg pinned bytes across all families.
    compare_families(&fixture, 399, stale, expected_stale).await;
    fixture.cleanup();
}
