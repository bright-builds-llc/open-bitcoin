// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bench/checkblock.cpp
// - packages/bitcoin-knots/src/bench/readwriteblock.cpp
// - packages/bitcoin-knots/src/primitives/block.h
// - packages/bitcoin-knots/src/primitives/transaction.h

use open_bitcoin_codec::{
    TransactionEncoding, encode_block_header, encode_transaction, parse_block_header,
    parse_transaction_without_witness,
};

use crate::{
    error::BenchError,
    fixtures::BenchFixtures,
    registry::{
        BLOCK_TRANSACTION_CODEC_MAPPING, BenchCase, BenchDurability, BenchGroupId, BenchMeasurement,
    },
};

const CASE_ID: &str = "block-transaction-codec.checked-in-fixtures";

pub const CASES: [BenchCase; 1] = [BenchCase {
    id: CASE_ID,
    group: BenchGroupId::BlockTransactionCodec,
    description: "Parses and re-encodes checked-in transaction and block-header fixtures.",
    measurement: BenchMeasurement {
        focus: "codec_round_trip",
        fixture: "checked_in_hex_fixtures",
        durability: BenchDurability::Pure,
    },
    knots_mapping: &BLOCK_TRANSACTION_CODEC_MAPPING,
    run_once,
}];

fn run_once() -> Result<(), BenchError> {
    let fixtures = BenchFixtures::shared()?;
    let transaction = parse_transaction_without_witness(&fixtures.codec.transaction_bytes)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let encoded_transaction = encode_transaction(&transaction, TransactionEncoding::WithoutWitness)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    if encoded_transaction != fixtures.codec.transaction_bytes {
        return Err(BenchError::case_failed(
            CASE_ID,
            "transaction fixture did not round trip",
        ));
    }

    let header = parse_block_header(&fixtures.codec.block_header_bytes)
        .map_err(|error| BenchError::case_failed(CASE_ID, error.to_string()))?;
    let encoded_header = encode_block_header(&header);
    if encoded_header != fixtures.codec.block_header_bytes {
        return Err(BenchError::case_failed(
            CASE_ID,
            "block header fixture did not round trip",
        ));
    }

    Ok(())
}
