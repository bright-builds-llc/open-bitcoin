// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bench/addrman.cpp
// - packages/bitcoin-knots/src/bench/peer_eviction.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/protocol.h

use open_bitcoin_codec::parse_message_header;
use open_bitcoin_network::{HeaderStore, ParsedNetworkMessage, WireNetworkMessage};
use open_bitcoin_primitives::NetworkMagic;

use crate::{
    error::BenchError,
    fixtures::BenchFixtures,
    registry::{BenchCase, BenchGroupId, NETWORK_WIRE_SYNC_MAPPING},
};

const CASE_ID: &str = "network-wire-sync.encode-decode-locator";

pub const CASES: [BenchCase; 1] = [BenchCase {
    id: CASE_ID,
    group: BenchGroupId::NetworkWireSync,
    description: "Encodes and decodes a wire message and builds a header locator.",
    knots_mapping: &NETWORK_WIRE_SYNC_MAPPING,
    run_once,
}];

fn run_once() -> Result<(), BenchError> {
    let fixtures = BenchFixtures::shared()?;
    parse_message_header(&fixtures.codec.message_header_bytes)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;

    let message = WireNetworkMessage::Ping { nonce: 42 };
    let encoded = message
        .encode_wire(NetworkMagic::MAINNET)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let parsed = ParsedNetworkMessage::decode_wire(&encoded)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    if parsed.message != message {
        return Err(BenchError::case_failed(
            CASE_ID,
            "wire message did not round trip",
        ));
    }

    let mut headers = HeaderStore::default();
    headers.seed_from_chain(&fixtures.network.active_chain);
    let locator = headers.locator();
    if locator.block_hashes.is_empty() {
        return Err(BenchError::case_failed(
            CASE_ID,
            "header locator fixture was empty",
        ));
    }

    Ok(())
}
