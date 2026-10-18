// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Accepted durable history observed through the real outbound socket executor.

use super::{peer_fixtures::PeerFilterFixture, *};
use crate::network::{
    ManagedInboundResponsePlanItem as Item, filter_transport::FilterTransportSnapshot,
};
use open_bitcoin_network::{GetCFCheckpt, GetCFHeaders, GetCFilters, ParsedNetworkMessage};
use std::io::{Read, Write};

impl PeerFilterFixture {
    pub fn hash(&self, height: usize) -> BlockHash {
        self.history.history.records[height].identity().block_hash()
    }

    pub fn cleanup(self) {
        let Self { history, runtime } = self;
        drop(runtime);
        history.history.cleanup();
    }
}

pub(in crate::sync::tests) fn complete_peer_handshake(runtime: &DurableSyncRuntime, peer: u64) {
    for message in [
        WireNetworkMessage::Version(open_bitcoin_network::VersionMessage {
            nonce: 999,
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
    ] {
        runtime
            .network
            .receive_sync_message(
                peer,
                message,
                50_000,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("genuine peer handshake");
    }
}

pub(in crate::sync::tests) fn with_accepted_stale_for_negative(
    action: impl FnOnce(&mut DurableSyncRuntime, BlockHash, u32),
) {
    let mut fixture = super::reorg::fixtures::ForkFixture::compact(16, 0, 16);
    let first = fixture.fork(10, 8, 0x61);
    fixture.apply(&first);
    fixture.finish();
    fixture.flush();
    let stop = first.records.last().expect("stale").identity().block_hash();
    let time = first
        .connected
        .last()
        .expect("accepted stale body")
        .block
        .header
        .time;
    let second = fixture.fork(10, 9, 0x62);
    fixture.apply(&second);
    fixture.finish();
    fixture.flush();
    assert!(
        fixture
            .runtime
            .network
            .has_earned_fixture_work()
            .expect("same live source")
    );
    action(&mut fixture.runtime, stop, time);
    fixture.cleanup();
}

fn observe(fixture: &PeerFilterFixture, request: WireNetworkMessage) -> Vec<Vec<u8>> {
    observe_runtime(&fixture.runtime, request)
}

fn observe_runtime(runtime: &DurableSyncRuntime, request: WireNetworkMessage) -> Vec<Vec<u8>> {
    let plans = runtime
        .network
        .receive_sync_message(
            1,
            request,
            50_000,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("actual managed request")
        .inbound_response_plan;
    let (mut session, mut socket, _) = super::super::peer_filter_transport::loopback();
    crate::sync::session::filter_serving::send_filter_plans(
        &runtime.network,
        &mut session,
        1,
        runtime.config.network.magic(),
        plans,
        &mut || false,
    )
    .expect("actual outbound executor");
    drop(session);
    let mut bytes = Vec::new();
    socket
        .read_to_end(&mut bytes)
        .expect("physical complete output");
    let mut frames = Vec::new();
    let mut remainder = bytes.as_slice();
    while !remainder.is_empty() {
        let size = 24
            + open_bitcoin_core::codec::parse_message_header(&remainder[..24])
                .expect("header")
                .payload_size as usize;
        ParsedNetworkMessage::decode_wire(&remainder[..size]).expect("complete frame");
        frames.push(remainder[..size].to_vec());
        remainder = &remainder[size..];
    }
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("settled"),
        FilterTransportSnapshot::default()
    );
    frames
}

#[test]
fn phase160_peer_evidence_accepted_limits_and_checkpoint_boundaries() {
    // Arrange: continuous accepted spending history, actual cold durable reopen.
    let mut fixture = PeerFilterFixture::new(3002);
    assert!(fixture.history.history.blocks[3001].transactions.len() > 1);
    // Act / Assert: each legal range completes through the concrete socket.
    for (start, stop, count) in [
        (0, 0, 1),
        (0, 999, 1000),
        (1, 1000, 1000),
        (2002, 3001, 1000),
    ] {
        assert_eq!(
            observe(
                &fixture,
                WireNetworkMessage::GetCFilters(GetCFilters {
                    filter_type: 0,
                    start_height: start,
                    stop_hash: fixture.hash(stop)
                })
            )
            .len(),
            count
        );
    }
    for (start, stop, count) in [(0, 1999, 2000), (1, 2000, 2000), (1002, 3001, 2000)] {
        let frames = observe(
            &fixture,
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: start,
                stop_hash: fixture.hash(stop),
            }),
        );
        assert_eq!(frames.len(), 1);
        let WireNetworkMessage::CFHeaders(headers) = ParsedNetworkMessage::decode_wire(&frames[0])
            .expect("headers")
            .message
        else {
            panic!("headers")
        };
        assert_eq!(headers.filter_hashes.len(), count);
        let expected = &fixture.history.history.records[start as usize..=stop];
        assert_eq!(
            headers.filter_hashes,
            expected
                .iter()
                .map(|record| record.identity().filter_hash().into())
                .collect::<Vec<_>>()
        );
    }
    for stop in [0, 999, 1000, 1001, 2000, 3001] {
        let frames = observe(
            &fixture,
            WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                filter_type: 0,
                stop_hash: fixture.hash(stop),
            }),
        );
        assert_eq!(frames.len(), 1);
        let WireNetworkMessage::CFCheckpt(checkpoint) =
            ParsedNetworkMessage::decode_wire(&frames[0])
                .expect("checkpoint")
                .message
        else {
            panic!("checkpoint")
        };
        let expected = (1000..=stop)
            .step_by(1000)
            .map(|height| {
                fixture.history.history.records[height]
                    .identity()
                    .filter_header()
                    .into()
            })
            .collect::<Vec<_>>();
        assert_eq!(checkpoint.filter_headers, expected);
    }
    // Narrow body-independence check: actual paired deletion, no full prune lifecycle claim.
    fixture
        .runtime
        .network
        .set_prune_mode(open_bitcoin_core::chainstate::PruneMode::ManualOnly)
        .expect("manual policy");
    let deleted = fixture
        .runtime
        .network
        .flush_applying_prune_plan(
            open_bitcoin_core::chainstate::FlushMode::Always,
            open_bitcoin_core::chainstate::FlushPolicyTime::new(50_000),
            u64::MAX,
            &open_bitcoin_core::chainstate::PrunePlan { heights: vec![20] },
            &[],
        )
        .expect("legal paired deletion");
    assert_eq!(deleted.deleted_block_hashes, vec![fixture.hash(20)]);
    assert!(
        fixture
            .runtime
            .store()
            .load_block(fixture.hash(20))
            .expect("body absent")
            .is_none()
    );
    assert!(
        fixture
            .runtime
            .store()
            .load_undo(fixture.hash(20))
            .expect("undo absent")
            .is_none()
    );
    assert_eq!(
        observe(
            &fixture,
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 20,
                stop_hash: fixture.hash(20)
            })
        )
        .len(),
        1
    );
    // Invalid one-over requests disconnect rather than shorten a response.
    for (index, request) in [
        WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type: 0,
            start_height: 0,
            stop_hash: fixture.hash(1000),
        }),
        WireNetworkMessage::GetCFHeaders(GetCFHeaders {
            filter_type: 0,
            start_height: 0,
            stop_hash: fixture.hash(2000),
        }),
    ]
    .into_iter()
    .enumerate()
    {
        if index > 0 {
            fixture
                .runtime
                .network
                .connect_outbound_peer(1, 0)
                .expect("fresh advertised session");
            super::peer_fixtures::complete_peer_handshake(&fixture.runtime, 1);
        }
        let result = fixture.runtime.network.receive_sync_message(
            1,
            request,
            50_000,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        );
        assert!(
            result.is_err(),
            "one-over disconnects rather than truncates"
        );
    }
    fixture
        .runtime
        .network
        .connect_outbound_peer(1, 0)
        .expect("checkpoint session");
    super::peer_fixtures::complete_peer_handshake(&fixture.runtime, 1);
    fixture
        .runtime
        .store()
        .inject_query_record_fault_for_test(
            fixture.hash(2000),
            crate::storage::fjall_store::filters::query::QueryRecordFault::MissingTarget,
        )
        .expect("actual required checkpoint gap");
    assert!(
        observe(
            &fixture,
            WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                filter_type: 0,
                stop_hash: fixture.hash(3001)
            })
        )
        .is_empty(),
        "late absent checkpoint emits no shortened prefix"
    );
    fixture.cleanup();
}

#[test]
fn phase160_peer_evidence_actual_outbound_missing_middle_final_predecessor_and_faults() {
    use crate::storage::fjall_store::filters::{
        FilterPublicationFault, query::QueryRecordFault as Fault,
    };
    // Arrange / Act / Assert: late absence never emits a shortened physical success.
    for (height, fault) in [
        (2, Fault::MissingTarget),
        (3, Fault::MissingTarget),
        (1, Fault::MissingTarget),
        (3, Fault::CorruptTarget),
    ] {
        let fixture = PeerFilterFixture::new(16);
        fixture
            .runtime
            .store()
            .inject_query_record_fault_for_test(fixture.hash(height), fault)
            .expect("real persisted fault");
        for request in [
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 2,
                stop_hash: fixture.hash(3),
            }),
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: 2,
                stop_hash: fixture.hash(3),
            }),
        ] {
            assert!(observe(&fixture, request).is_empty());
        }
        fixture.cleanup();
    }
    let fixture = PeerFilterFixture::new(16);
    fixture
        .runtime
        .store()
        .set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
    let result = fixture
        .runtime
        .network
        .receive_sync_message(
            1,
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 1,
                stop_hash: fixture.hash(3),
            }),
            50_000,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("sanitized local failure");
    assert_eq!(
        result.inbound_response_plan,
        vec![Item::FilterServeUnavailable(
            crate::network::filter_transport::BasicFilterServeUnavailable::StorageFailure
        )]
    );
    assert!(format!("{:?}", result.inbound_response_plan).len() < 128);
    fixture.cleanup();
}

#[test]
fn phase160_peer_evidence_genuine_retained_competing_branch_actual_outbound() {
    // Arrange: independently staged expected rows then real durable managed reorgs.
    let fixture = super::reorg::fixtures::ForkFixture::compact(16, 0, 16);
    fixture
        .runtime
        .network
        .configure_basic_peer_serving(true)
        .expect("grant");
    fixture
        .runtime
        .network
        .connect_outbound_peer(1, 0)
        .expect("actual version");
    super::peer_fixtures::complete_peer_handshake(&fixture.runtime, 1);
    let first = fixture.fork(10, 8, 0x61);
    fixture.apply(&first);
    fixture.finish();
    fixture.flush();
    let stop = first.records.last().expect("stop").identity().block_hash();
    let second = fixture.fork(10, 9, 0x62);
    fixture.apply(&second);
    fixture.finish();
    let network = &fixture.runtime.network;
    assert!(network.has_earned_fixture_work().expect("same live source"));
    let headers = network.header_entries().expect("accepted branch headers");
    for branch in [&first, &second] {
        assert!(branch.records.iter().all(|record| {
            headers
                .iter()
                .any(|header| header.block_hash == record.identity().block_hash())
        }));
    }
    // Act
    let actual = observe_runtime(
        &fixture.runtime,
        WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type: 0,
            start_height: 11,
            stop_hash: stop,
        }),
    );
    // Assert: requested stale ancestry, distinct from active height substitutions.
    assert_eq!(actual.len(), 8);
    for (frame, record) in actual.iter().zip(&first.records) {
        let WireNetworkMessage::CFilter(filter) = ParsedNetworkMessage::decode_wire(frame)
            .expect("actual retained branch")
            .message
        else {
            panic!("filter")
        };
        assert_eq!(filter.block_hash, record.identity().block_hash());
        assert_eq!(filter.filter, record.encoded_bytes());
        assert_eq!(
            fixture
                .runtime
                .store()
                .validation_provenance(filter.block_hash)
                .expect("persisted acceptance"),
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
    }
    fixture.cleanup();
}

#[test]
fn phase160_peer_evidence_catchup_complete_prefix_all_families_no_future_prefix() {
    // Arrange
    let fixture = PeerFilterFixture::lagging(416, 16);
    let before = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("lagged owner");
    // Act / Assert
    for (stop, present) in [(2, true), (415, false)] {
        for request in [
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 0,
                stop_hash: fixture.hash(stop),
            }),
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: 0,
                stop_hash: fixture.hash(stop),
            }),
        ] {
            assert_eq!(!observe(&fixture, request).is_empty(), present);
        }
    }
    assert_eq!(
        observe(
            &fixture,
            WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                filter_type: 0,
                stop_hash: fixture.hash(415)
            })
        )
        .len(),
        1,
        "no required checkpoint rows below1000"
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("no query-time catchup"),
        before
    );
    fixture.cleanup();
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0);
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("hex ASCII"), 16).expect("hex byte")
        })
        .collect()
}

fn oracle(input: serde_json::Value, wire: bool) -> serde_json::Value {
    use std::process::{Command, Stdio};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");
    let bytes = serde_json::to_vec(&input).expect("public fixture inputs");
    assert!(bytes.len() <= 8 * 1024 * 1024);
    let mut command = Command::new("bun");
    // Thin glue reuses Bun's existing10s/16MiB subprocess contract unchanged.
    command.current_dir(&root).args(["--eval", "import {runBounded,PIN} from './scripts/generate-peer-filter-fixtures.ts'; const root=process.cwd(); if(runBounded('git',['-C','packages/bitcoin-knots','rev-parse','HEAD'],root).trim()!==PIN) throw Error('pin mismatch'); process.stdout.write(runBounded('python3',['scripts/basic-filter-oracle.py',...(process.env.PEER_ORACLE_WIRE==='1'?['--peer-wire-proof']:[])],root,await Bun.stdin.text()));"]);
    command.env("PEER_ORACLE_WIRE", if wire { "1" } else { "0" });
    let mut child = command
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("unchanged pinned helper");
    child
        .stdin
        .take()
        .expect("helper input")
        .write_all(&bytes)
        .expect("bounded helper input");
    let output = child.wait_with_output().expect("helper termination");
    assert!(
        output.status.success(),
        "pinned public fixture helper failed"
    );
    assert!(output.stdout.len() <= 16 * 1024 * 1024);
    serde_json::from_slice(&output.stdout).expect("helper public expectations")
}

#[test]
fn phase160_peer_evidence_genuine_raw_spends_match_independent_pinned_socket_bytes() {
    use serde_json::json;
    // Arrange: actual durable accepted history with historical and same-block spends.
    let fixture = PeerFilterFixture::new(16);
    let snapshot = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("accepted history");
    let cases: Vec<_> = fixture.history.history.blocks[..=2].iter().enumerate().map(|(height, block)| {
        let hash = fixture.hash(height);
        let spent: Vec<_> = snapshot.undo_by_block.get(&hash).map(|undo| {
            BasicFilterInputs::from_historical(block, &snapshot.active_chain[height], Some(HistoricalBlockUndo { block_hash:hash, undo })).expect("genuine undo").spent_scripts().map(hex).collect()
        }).unwrap_or_default();
        if height == 2 { assert_eq!(spent.len(), 2); }
        json!({"name":height.to_string(), "raw_block":hex(&open_bitcoin_core::codec::encode_block(block).expect("raw consensus body")), "block_hash_raw":hex(hash.as_bytes()), "outputs":[], "spent":spent, "predecessor_raw":"00".repeat(32), "previous_name":(height>0).then(|| (height-1).to_string())})
    }).collect();
    // Act: independent BASIC computation uses raw block/undo scripts, not Rust filter bytes.
    let independent = oracle(json!({"cases":cases}), false);
    let rows = independent["cases"]
        .as_array()
        .expect("three independent rows");
    for (height, row) in rows.iter().enumerate() {
        let record = &fixture.history.history.records[height];
        assert_eq!(row["encoded"], hex(record.encoded_bytes()));
        assert_eq!(
            row["header_raw"],
            hex(record.identity().filter_header().as_bytes())
        );
        assert_eq!(row["block_hash_raw"], hex(fixture.hash(height).as_bytes()));
    }
    let mut recipes: Vec<_> = rows.iter().map(|row| json!({"name":row["name"], "command":"cfilter", "input":{"filter_type":0, "block_hash_raw":row["block_hash_raw"], "filter_data":row["encoded"]}})).collect();
    recipes.push(json!({"name":"headers", "command":"cfheaders", "input":{"filter_type":0, "stop_hash_raw":rows[2]["block_hash_raw"], "prev_header_raw":"00".repeat(32), "hashes_raw":rows.iter().map(|row| row["hash_raw"].clone()).collect::<Vec<_>>()}}));
    recipes.push(json!({"name":"checkpoints", "command":"cfcheckpt", "input":{"filter_type":0, "stop_hash_raw":rows[2]["block_hash_raw"], "headers_raw":[]}}));
    let expected = oracle(
        json!({"wire":recipes, "proof":[], "handlers":[], "malformed":[]}),
        true,
    );
    let mut frames = observe(
        &fixture,
        WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type: 0,
            start_height: 0,
            stop_hash: fixture.hash(2),
        }),
    );
    frames.extend(observe(
        &fixture,
        WireNetworkMessage::GetCFHeaders(GetCFHeaders {
            filter_type: 0,
            start_height: 0,
            stop_hash: fixture.hash(2),
        }),
    ));
    frames.extend(observe(
        &fixture,
        WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
            filter_type: 0,
            stop_hash: fixture.hash(2),
        }),
    ));
    // Assert: helpers frame mainnet; only the independently known regtest magic differs.
    for (frame, row) in frames
        .iter()
        .zip(expected["wire"].as_array().expect("expected wire"))
    {
        let mut golden = unhex(row["expected"]["frame_hex"].as_str().expect("frame hex"));
        golden[..4].copy_from_slice(&fixture.runtime.config.network.magic().to_bytes());
        assert_eq!(frame, &golden);
    }
    assert_eq!(frames.len(), 5);
    fixture.cleanup();
}
