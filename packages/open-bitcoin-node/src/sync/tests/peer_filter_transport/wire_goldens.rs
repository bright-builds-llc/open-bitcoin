// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/protocol.cpp
// - packages/bitcoin-knots/test/functional/test_framework/messages.py
// - packages/bitcoin-knots/src/net_processing.cpp

//! Independent protocol vectors and explicitly seeded negative eligibility controls.

use open_bitcoin_core::primitives::{BlockHash, Hash32, NetworkMagic};
use open_bitcoin_network::{
    CFCheckpt, CFHeaders, CFilter, GetCFCheckpt, GetCFHeaders, GetCFilters, ParsedNetworkMessage,
    WireNetworkMessage,
};
use serde_json::Value;

fn assert_actual_negative(runtime: &mut crate::DurableSyncRuntime, stop: BlockHash) {
    use std::io::{Read, Write};
    let (session, mut socket, peer) = super::loopback();
    let magic = runtime.config.network.magic();
    runtime.config.max_messages_per_peer = 3;
    let remote = std::thread::spawn(move || {
        for message in [
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage {
                nonce: 160_115,
                ..Default::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 0,
                stop_hash: stop,
            }),
        ] {
            socket
                .write_all(&message.encode_wire(magic).expect("actual negative request"))
                .expect("remote physical write");
        }
        let mut bytes = Vec::new();
        socket
            .read_to_end(&mut bytes)
            .expect("actual production output");
        bytes
    });
    let result = runtime.sync_connected_peer(session, &peer, 160_115, 1, 50_000, &mut || 50_001);
    assert!(result.is_err(), "eligibility must disconnect: {result:?}");
    let bytes = remote.join().expect("bounded remote");
    let mut rest = bytes.as_slice();
    while !rest.is_empty() {
        let size = 24
            + open_bitcoin_core::codec::parse_message_header(&rest[..24])
                .expect("complete prefix header")
                .payload_size as usize;
        let message = ParsedNetworkMessage::decode_wire(&rest[..size])
            .expect("complete prefix frame")
            .message;
        assert!(!matches!(
            message,
            WireNetworkMessage::CFilter(_)
                | WireNetworkMessage::CFHeaders(_)
                | WireNetworkMessage::CFCheckpt(_)
        ));
        rest = &rest[size..];
    }
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("negative releases all owners"),
        crate::network::filter_transport::FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_peer_evidence_header_only_raw_body_and_unknown_legacy_are_negative() {
    use crate::sync::tests::filter_index::peer_fixtures::PeerFilterFixture;
    // Arrange: an actual accepted durable chain plus a real mined header that was never connected.
    let mut fixture = PeerFilterFixture::new(16);
    let mut candidate = super::super::filter_index::fixture_block(fixture.hash(15), 16);
    candidate.header.time = 2600;
    candidate.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            candidate.header.nonce = *nonce;
            open_bitcoin_core::consensus::check_block_header(&candidate.header).is_ok()
        })
        .expect("actual easy header PoW");
    let stop = open_bitcoin_core::consensus::block_hash(&candidate.header);
    fixture
        .runtime
        .network
        .receive_sync_message(
            1,
            WireNetworkMessage::Headers(open_bitcoin_network::HeadersMessage {
                headers: vec![candidate.header.clone()],
            }),
            50_000,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("header-only admission");
    // Act / Assert: header presence and then body presence grant no scripts validation.
    assert_actual_negative(&mut fixture.runtime, stop);
    fixture
        .runtime
        .store()
        .save_block(&candidate, crate::PersistMode::Sync)
        .expect("raw body control");
    assert_direct_provenance_negative(&fixture.runtime, stop);
    fixture
        .runtime
        .store()
        .save_header_entries(
            &fixture.runtime.network.header_entries().expect("headers"),
            crate::PersistMode::Sync,
        )
        .expect("explicit legacy raw snapshot control");
    assert_eq!(
        fixture
            .runtime
            .store()
            .validation_provenance(stop)
            .expect("legacy provenance"),
        crate::storage::validation_history::ValidationProvenance::UnknownLegacy
    );
    assert_direct_provenance_negative(&fixture.runtime, stop);
    fixture.cleanup();
}

fn assert_direct_provenance_negative(runtime: &crate::DurableSyncRuntime, stop: BlockHash) {
    // Raw-body controls inspect immediate request dispatch. Running the sync maintenance
    // loop here could legitimately validate a retained body, changing its provenance.
    runtime
        .network
        .connect_outbound_peer(160_116, 0)
        .expect("fresh real own-version capability");
    crate::sync::tests::filter_index::peer_fixtures::complete_peer_handshake(runtime, 160_116);
    let request = open_bitcoin_network::policy::FilterRequest::Filters {
        filter_type: 0,
        start_height: 0,
        stop_hash: stop,
    };
    assert_eq!(
        runtime
            .network
            .prepare_basic_filter_response(160_116, request, runtime.consensus_params)
            .expect("immediate provenance gate"),
        crate::network::filter_transport::BasicFilterPrepareOutcome::InvalidDisconnect(
            open_bitcoin_network::policy::FilterRequestInvalid::UnknownOrDisallowedStop
        )
    );
    assert!(
        runtime
            .network
            .receive_sync_message(
                160_116,
                WireNetworkMessage::GetCFilters(GetCFilters {
                    filter_type: 0,
                    start_height: 0,
                    stop_hash: stop
                }),
                50_000,
                runtime.verify_flags,
                runtime.consensus_params
            )
            .is_err()
    );
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("negative retains nothing"),
        crate::network::filter_transport::FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_peer_evidence_seeded_old_timestamp_and_old_proof_are_negative_only() {
    use crate::sync::tests::filter_index::peer_fixtures::with_accepted_stale_for_negative;
    use open_bitcoin_core::chainstate::{Chainstate, CoinsCache, FlushPolicyTime};
    for old_proof in [false, true] {
        // Arrange: the stale stop itself was genuinely accepted, displaced and indexed.
        with_accepted_stale_for_negative(|runtime, stop, time| {
            let snapshot = runtime
                .network
                .chainstate_snapshot()
                .expect("genuine durable active chain");
            let mut headers = open_bitcoin_network::HeaderStore::from_entries(
                runtime
                    .network
                    .header_entries()
                    .expect("retained actual branches"),
            )
            .expect("complete headers");
            let mut parent = snapshot.active_chain.last().expect("tip").block_hash;
            // These extra best-header facts are deliberately raw-seeded NEGATIVE controls.
            // Hard target then easy target yields old equivalent work at current easy proof;
            // the alternative keeps recent work but crosses the strict timestamp boundary.
            for offset in 0..2 {
                let mut header = snapshot.active_chain.last().expect("tip").header.clone();
                header.previous_block_hash = parent;
                header.time = if old_proof {
                    4000 + offset
                } else {
                    time + 2_592_001 + offset
                };
                header.bits = if old_proof && offset == 0 {
                    0x1d00_ffff
                } else {
                    0x207f_ffff
                };
                header.nonce = offset;
                let hash = open_bitcoin_core::consensus::block_hash(&header);
                let parent_entry = headers.entry(&parent).expect("raw negative parent number");
                let work = parent_entry.chain_work.wrapping_add(
                    open_bitcoin_core::consensus::chain_proof::block_proof(header.bits)
                        .expect("raw numerical proof only"),
                );
                let height = parent_entry.height + 1;
                headers.record_position(&open_bitcoin_core::chainstate::ChainPosition {
                    block_hash: hash,
                    header,
                    height,
                    chain_work: work,
                    median_time_past: 0,
                });
                parent = hash;
            }
            let store = runtime.store().clone();
            let state = Chainstate::from_coins_cache(
                CoinsCache::from_parent(store.coins_view()),
                snapshot.active_chain,
                snapshot.undo_by_block,
                None,
            );
            let manager = crate::ManagedChainstate::from_recovered_chainstate(
                crate::FjallChainstateStore::from_store(store),
                state,
                crate::chainstate::FlushLifecycle::ready(
                    FlushPolicyTime::new(0),
                    FlushPolicyTime::new(0),
                    0,
                    false,
                ),
            )
            .expect("actual durable coins adapter");
            let mut network = crate::ManagedPeerNetwork::from_initialized_chainstate(
                manager,
                Default::default(),
                Default::default(),
                16,
                Default::default(),
                Default::default(),
                false,
            );
            network.seed_header_store(headers);
            network.set_basic_filter_serving(true);
            runtime.network = crate::ManagedNetworkHandle::new(network);
            // Act / Assert: the production outbound loop rejects eligibility before response bytes.
            assert_actual_negative(runtime, stop);
        });
    }
}

fn hex(value: &Value) -> Vec<u8> {
    let value = value.as_str().expect("fixture hex string");
    assert!(value.len().is_multiple_of(2));
    (0..value.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&value[index..index + 2], 16).expect("fixture hex byte"))
        .collect()
}

fn hash(value: &Value) -> Hash32 {
    Hash32::from_byte_array(hex(value).try_into().expect("raw 32-byte fixture hash"))
}

fn hashes(input: &Value, key: &str) -> Vec<Hash32> {
    input[key]
        .as_array()
        .expect("fixture hash array")
        .iter()
        .map(hash)
        .collect()
}

fn request_stop(input: &Value) -> BlockHash {
    BlockHash::from_byte_array(hash(&input["stop_hash_raw"]).to_byte_array())
}

fn message(case: &Value) -> WireNetworkMessage {
    let input = &case["input"];
    let filter_type = input["filter_type"]
        .as_u64()
        .expect("fixture type")
        .try_into()
        .expect("one-byte type");
    match case["command"].as_str().expect("fixture command") {
        "getcfilters" => WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type,
            start_height: input["start_height"]
                .as_u64()
                .expect("height")
                .try_into()
                .expect("u32 height"),
            stop_hash: request_stop(input),
        }),
        "getcfheaders" => WireNetworkMessage::GetCFHeaders(GetCFHeaders {
            filter_type,
            start_height: input["start_height"]
                .as_u64()
                .expect("height")
                .try_into()
                .expect("u32 height"),
            stop_hash: request_stop(input),
        }),
        "getcfcheckpt" => WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
            filter_type,
            stop_hash: request_stop(input),
        }),
        "cfilter" => WireNetworkMessage::CFilter(CFilter {
            filter_type,
            block_hash: BlockHash::from_byte_array(hash(&input["block_hash_raw"]).to_byte_array()),
            filter: hex(&input["filter_data"]),
        }),
        "cfheaders" => WireNetworkMessage::CFHeaders(CFHeaders {
            filter_type,
            stop_hash: request_stop(input),
            previous_filter_header: hash(&input["prev_header_raw"]),
            filter_hashes: hashes(input, "hashes_raw"),
        }),
        "cfcheckpt" => WireNetworkMessage::CFCheckpt(CFCheckpt {
            filter_type,
            stop_hash: request_stop(input),
            filter_headers: hashes(input, "headers_raw"),
        }),
        _ => unreachable!("bounded six-family fixture schema"),
    }
}

#[test]
fn phase160_peer_evidence_pinned_wire_goldens() {
    // Arrange: golden bytes are generated independently of first-party Rust.
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../../../docs/parity/fixtures/peer-filter-wire.json"
    ))
    .expect("validated bounded fixture");
    assert_eq!(
        fixture["authority"]["pin"],
        "a9aee730466ac67d35a3c03ee24676be5e045878"
    );
    let cases = fixture["cases"].as_array().expect("wire cases");
    assert_eq!(cases.len(), 42);
    for case in cases {
        let message = message(case);
        let payload = hex(&case["expected"]["payload_hex"]);
        let frame = hex(&case["expected"]["frame_hex"]);

        // Act
        let actual_payload = message.encode_payload().expect("legal wire payload");
        let actual_frame = message
            .encode_wire(NetworkMagic::MAINNET)
            .expect("legal wire frame");
        let received = ParsedNetworkMessage::decode_wire(&frame).expect("independent wire input");

        // Assert: input identity, byte order, CompactSize and framing agree.
        assert_eq!(actual_payload, payload, "{}", case["name"]);
        assert_eq!(actual_frame, frame, "{}", case["name"]);
        assert_eq!(received.message, message, "{}", case["name"]);
    }
}
