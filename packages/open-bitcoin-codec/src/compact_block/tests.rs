// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py
// - packages/bitcoin-knots/test/functional/test_framework/messages.py

use open_bitcoin_primitives::{
    Amount, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid,
};

use crate::block::encode_block_header;
use crate::compact_block::{
    BIP152_COMPACT_BLOCKS_VERSION, BlockTransactions, BlockTransactionsRequest,
    CompactBlockPayload, PrefilledTransaction, SendCompactMessage, ShortId,
    decode_block_transactions_payload, decode_compact_block_payload,
    decode_get_block_transactions_payload, decode_send_compact_payload,
    encode_block_transactions_payload, encode_compact_block_payload,
    encode_get_block_transactions_payload, encode_send_compact_payload,
    expand_block_transaction_indexes, expand_prefilled_positions, validate_compact_block_structure,
};
use crate::compact_size::write_compact_size;
use crate::error::CodecError;
use crate::test_support::decode_hex;
use crate::transaction::{TransactionEncoding, encode_transaction};

fn sample_header() -> BlockHeader {
    BlockHeader {
        version: 2,
        previous_block_hash: BlockHash::from_byte_array([1_u8; 32]),
        merkle_root: MerkleRoot::from_byte_array([2_u8; 32]),
        time: 1_234,
        bits: 0x207f_ffff,
        nonce: 99,
    }
}

fn sample_witness_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([3_u8; 32]),
                vout: 1,
            },
            script_sig: ScriptBuf::from_bytes(Vec::new()).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x01, 0x02], vec![0x51]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51, 0xac]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn encoded_sample_witness_transaction() -> Vec<u8> {
    encode_transaction(
        &sample_witness_transaction(),
        TransactionEncoding::WithWitness,
    )
    .expect("sample transaction should encode")
}

fn encoded_null_transaction() -> Vec<u8> {
    decode_hex("01000000000000000000")
}

fn encoded_superfluous_witness_transaction() -> Vec<u8> {
    decode_hex(
        "0200000000010102020202020202020202020202020202020202020202020202020202020202020100000000ffffffff012a0000000000000001510000000000",
    )
}

fn compact_block_prefix() -> Vec<u8> {
    let mut payload = encode_block_header(&sample_header());
    payload.extend_from_slice(&1_u64.to_le_bytes());
    payload
}

fn compact_block_payload_with_counts(
    short_ids: &[ShortId],
    prefilled: &[(u64, Vec<u8>)],
) -> Vec<u8> {
    let mut payload = compact_block_prefix();
    write_compact_size(&mut payload, short_ids.len() as u64).expect("short id count");
    for short_id in short_ids {
        payload.extend_from_slice(short_id.as_wire_bytes());
    }
    write_compact_size(&mut payload, prefilled.len() as u64).expect("prefilled count");
    for (delta, transaction) in prefilled {
        write_compact_size(&mut payload, *delta).expect("prefilled delta");
        payload.extend_from_slice(transaction);
    }
    payload
}

fn getblocktxn_payload(index_deltas: &[u64]) -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(BlockHash::from_byte_array([14_u8; 32]).as_bytes());
    write_compact_size(&mut payload, index_deltas.len() as u64).expect("index count");
    for index_delta in index_deltas {
        write_compact_size(&mut payload, *index_delta).expect("index delta");
    }
    payload
}

fn blocktxn_payload(transactions: &[Vec<u8>]) -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(BlockHash::from_byte_array([15_u8; 32]).as_bytes());
    write_compact_size(&mut payload, transactions.len() as u64).expect("transaction count");
    for transaction in transactions {
        payload.extend_from_slice(transaction);
    }
    payload
}

fn assert_error_contains(error: CodecError, expected: &str) {
    assert!(
        error.to_string().contains(expected),
        "expected `{}` to contain `{expected}`",
        error,
    );
}

#[test]
fn phase112_send_compact_round_trips_version_2_true() {
    // Arrange
    let message = SendCompactMessage {
        announce: true,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    };

    // Act
    let encoded = encode_send_compact_payload(&message);
    let decoded = decode_send_compact_payload(&encoded).expect("version 2 payload should decode");

    // Assert
    assert_eq!(encoded, vec![1, 2, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(encoded.len(), 9);
    assert_eq!(decoded, message);
}

#[test]
fn phase112_send_compact_decodes_unsupported_versions_as_data() {
    for version in [1_u64, 3] {
        // Arrange
        let message = SendCompactMessage {
            announce: true,
            version,
        };
        let encoded = encode_send_compact_payload(&message);

        // Act
        let decoded = decode_send_compact_payload(&encoded).expect("unsupported version is data");

        // Assert
        assert_eq!(decoded, message);
    }
}

#[test]
fn phase112_send_compact_rejects_short_and_trailing_payloads() {
    // Arrange
    let short_payload = [1_u8, 2, 0, 0, 0, 0, 0, 0];
    let trailing_payload = [1_u8, 2, 0, 0, 0, 0, 0, 0, 0, 0xff];

    // Act
    let short_error =
        decode_send_compact_payload(&short_payload).expect_err("short payload should be rejected");
    let trailing_error = decode_send_compact_payload(&trailing_payload)
        .expect_err("trailing payload should be rejected");

    // Assert
    assert_eq!(
        short_error.to_string(),
        "unexpected EOF: needed 8 bytes, remaining 7",
    );
    assert_eq!(trailing_error.to_string(), "trailing data: 1 bytes");
}

#[test]
fn phase112_cmpctblock_round_trips_short_ids_and_prefilled_witness_transactions() {
    // Arrange
    let payload = CompactBlockPayload {
        header: sample_header(),
        nonce: 0x0807_0605_0403_0201,
        short_ids: vec![
            ShortId::from_wire_bytes([1, 2, 3, 4, 5, 6]),
            ShortId::from_wire_bytes([7, 8, 9, 10, 11, 12]),
        ],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: sample_witness_transaction(),
        }],
    };

    // Act
    let encoded = encode_compact_block_payload(&payload).expect("cmpctblock should encode");
    let decoded = decode_compact_block_payload(&encoded).expect("cmpctblock should decode");
    let reencoded =
        encode_compact_block_payload(&decoded).expect("decoded cmpctblock should re-encode");

    // Assert
    assert_eq!(decoded, payload);
    assert_eq!(reencoded, encoded);
    assert_eq!(&encoded[89..101], &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    assert!(encoded.windows(2).any(|window| window == [0x00, 0x01]));
}

#[test]
fn phase112_short_id_is_exactly_six_wire_bytes() {
    // Arrange
    let short_id = ShortId::from_wire_bytes([1, 2, 3, 4, 5, 6]);
    let payload = CompactBlockPayload {
        header: sample_header(),
        nonce: 7,
        short_ids: vec![short_id],
        prefilled_transactions: Vec::new(),
    };

    // Act
    let encoded = encode_compact_block_payload(&payload).expect("cmpctblock should encode");
    let decoded = decode_compact_block_payload(&encoded).expect("cmpctblock should decode");

    // Assert
    assert_eq!(short_id.as_wire_bytes(), &[1, 2, 3, 4, 5, 6]);
    assert_eq!(encoded[88], 1);
    assert_eq!(&encoded[89..95], short_id.as_wire_bytes());
    assert_eq!(encoded[95], 0);
    assert_eq!(decoded.short_ids, vec![short_id]);
}

#[test]
fn phase112_cmpctblock_rejects_empty_compact_block() {
    // Arrange
    let payload = CompactBlockPayload {
        header: sample_header(),
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: Vec::new(),
    };

    // Act
    let error =
        encode_compact_block_payload(&payload).expect_err("empty compact block should be rejected");

    // Assert
    assert_eq!(error, CodecError::CompactBlockEmpty);
}

#[test]
fn phase112_cmpctblock_rejects_overflowing_prefilled_positions() {
    // Arrange
    let payload = CompactBlockPayload {
        header: sample_header(),
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: u64::MAX,
            transaction: sample_witness_transaction(),
        }],
    };

    // Act
    let error =
        expand_prefilled_positions(&payload).expect_err("overflowing position should be rejected");

    // Assert
    assert_eq!(error, CodecError::DifferentialIndexOverflow);
}

#[test]
fn phase112_cmpctblock_structure_rejects_implied_count_overflow() {
    // Arrange
    let payload = CompactBlockPayload {
        header: sample_header(),
        nonce: 1,
        short_ids: vec![ShortId::from_wire_bytes([1, 2, 3, 4, 5, 6]); usize::from(u16::MAX) + 1],
        prefilled_transactions: Vec::new(),
    };

    // Act
    let error = validate_compact_block_structure(&payload).expect_err("count should overflow");

    // Assert
    assert_eq!(
        error.to_string(),
        "compact block transaction count out of range: 65536"
    );
}

#[test]
fn phase112_cmpctblock_rejects_prefilled_position_beyond_implied_count() {
    // Arrange
    let payload = CompactBlockPayload {
        header: sample_header(),
        nonce: 1,
        short_ids: vec![ShortId::from_wire_bytes([1, 2, 3, 4, 5, 6])],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 2,
            transaction: sample_witness_transaction(),
        }],
    };

    // Act
    let error = encode_compact_block_payload(&payload)
        .expect_err("out-of-bounds position should be rejected");

    // Assert
    assert_eq!(
        error,
        CodecError::PrefilledTransactionOutOfBounds {
            position: 2,
            transaction_count: 2,
        },
    );
}

#[test]
fn phase112_getblocktxn_round_trips_multi_index_deltas() {
    // Arrange
    let request = BlockTransactionsRequest {
        block_hash: BlockHash::from_byte_array([9_u8; 32]),
        index_deltas: vec![0, 2, 4],
    };

    // Act
    let encoded =
        encode_get_block_transactions_payload(&request).expect("getblocktxn should encode");
    let decoded =
        decode_get_block_transactions_payload(&encoded).expect("getblocktxn should decode");
    let reencoded = encode_get_block_transactions_payload(&decoded)
        .expect("decoded getblocktxn should re-encode");
    let indexes =
        expand_block_transaction_indexes(&decoded).expect("differential indexes should expand");

    // Assert
    assert_eq!(decoded, request);
    assert_eq!(reencoded, encoded);
    assert_eq!(indexes, vec![0, 3, 8]);
}

#[test]
fn phase112_getblocktxn_allows_empty_index_vector() {
    // Arrange
    let request = BlockTransactionsRequest {
        block_hash: BlockHash::from_byte_array([10_u8; 32]),
        index_deltas: Vec::new(),
    };

    // Act
    let encoded =
        encode_get_block_transactions_payload(&request).expect("getblocktxn should encode");
    let decoded =
        decode_get_block_transactions_payload(&encoded).expect("empty getblocktxn should decode");

    // Assert
    assert_eq!(decoded, request);
    assert_eq!(encoded.len(), 33);
}

#[test]
fn phase112_getblocktxn_rejects_index_above_u16() {
    // Arrange
    let request = BlockTransactionsRequest {
        block_hash: BlockHash::from_byte_array([11_u8; 32]),
        index_deltas: vec![u64::from(u16::MAX) + 1],
    };

    // Act
    let error = expand_block_transaction_indexes(&request)
        .expect_err("indexes above u16 should be rejected");

    // Assert
    assert_eq!(error, CodecError::DifferentialIndexOverflow);
}

#[test]
fn phase112_blocktxn_round_trips_witness_transactions() {
    // Arrange
    let response = BlockTransactions {
        block_hash: BlockHash::from_byte_array([12_u8; 32]),
        transactions: vec![sample_witness_transaction()],
    };

    // Act
    let encoded = encode_block_transactions_payload(&response).expect("blocktxn should encode");
    let decoded = decode_block_transactions_payload(&encoded).expect("blocktxn should decode");
    let reencoded =
        encode_block_transactions_payload(&decoded).expect("decoded blocktxn should encode");

    // Assert
    assert_eq!(decoded, response);
    assert_eq!(reencoded, encoded);
    assert!(decoded.transactions[0].has_witness());
}

#[test]
fn phase112_blocktxn_allows_empty_transaction_vector() {
    // Arrange
    let response = BlockTransactions {
        block_hash: BlockHash::from_byte_array([13_u8; 32]),
        transactions: Vec::new(),
    };

    // Act
    let encoded = encode_block_transactions_payload(&response).expect("blocktxn should encode");
    let decoded =
        decode_block_transactions_payload(&encoded).expect("empty blocktxn should decode");

    // Assert
    assert_eq!(decoded, response);
    assert_eq!(encoded.len(), 33);
}

#[test]
fn phase112_malformed_cmpctblock_matrix_rejects_before_reconstruction() {
    let cases = [
        ("missing header", Vec::new(), "unexpected EOF: needed"),
        (
            "missing nonce",
            encode_block_header(&sample_header()),
            "unexpected EOF: needed 8 bytes",
        ),
        (
            "truncated short id",
            {
                let mut payload = compact_block_prefix();
                write_compact_size(&mut payload, 1).expect("short id count");
                payload.extend_from_slice(&[1, 2, 3, 4, 5]);
                payload
            },
            "unexpected EOF: needed 6 bytes",
        ),
        (
            "trailing data",
            {
                let mut payload = compact_block_payload_with_counts(
                    &[ShortId::from_wire_bytes([1, 2, 3, 4, 5, 6])],
                    &[],
                );
                payload.push(0xff);
                payload
            },
            "trailing data: 1 bytes",
        ),
        (
            "non-canonical short id count",
            {
                let mut payload = compact_block_prefix();
                payload.extend_from_slice(&[0xfd, 1, 0]);
                payload
            },
            "non-canonical compact size for value 1",
        ),
        (
            "empty compact block",
            compact_block_payload_with_counts(&[], &[]),
            "compact block has no short ids or prefilled transactions",
        ),
        (
            "excessive transaction count",
            {
                let mut payload = compact_block_prefix();
                payload.extend_from_slice(&[0xfe, 0, 0, 1, 0]);
                payload
            },
            "compact block transaction count out of range",
        ),
        (
            "prefilled delta overflow",
            compact_block_payload_with_counts(
                &[],
                &[(
                    u64::from(u16::MAX) + 1,
                    encoded_sample_witness_transaction(),
                )],
            ),
            "differential index overflow",
        ),
        (
            "prefilled position beyond implied count",
            compact_block_payload_with_counts(
                &[ShortId::from_wire_bytes([1, 2, 3, 4, 5, 6])],
                &[(2, encoded_sample_witness_transaction())],
            ),
            "prefilled transaction position",
        ),
        (
            "null prefilled transaction",
            compact_block_payload_with_counts(&[], &[(0, encoded_null_transaction())]),
            "compact block prefilled transaction is structurally null",
        ),
        (
            "superfluous witness records",
            compact_block_payload_with_counts(
                &[],
                &[(0, encoded_superfluous_witness_transaction())],
            ),
            "superfluous witness record",
        ),
    ];

    for (name, payload, expected) in cases {
        // Act
        let error = decode_compact_block_payload(&payload).expect_err(name);

        // Assert
        assert_error_contains(error, expected);
    }
}

#[test]
fn phase112_malformed_getblocktxn_matrix_rejects_bad_indexes() {
    let cases = [
        (
            "trailing bytes",
            {
                let mut payload = getblocktxn_payload(&[]);
                payload.push(0xff);
                payload
            },
            "trailing data: 1 bytes",
        ),
        (
            "non-canonical index count",
            {
                let mut payload = Vec::new();
                payload.extend_from_slice(BlockHash::from_byte_array([16_u8; 32]).as_bytes());
                payload.extend_from_slice(&[0xfd, 1, 0]);
                payload
            },
            "non-canonical compact size for value 1",
        ),
        (
            "differential expansion above u16",
            getblocktxn_payload(&[u64::from(u16::MAX) + 1]),
            "differential index overflow",
        ),
    ];

    for (name, payload, expected) in cases {
        // Act
        let error = decode_get_block_transactions_payload(&payload).expect_err(name);

        // Assert
        assert_error_contains(error, expected);
    }
}

#[test]
fn phase112_malformed_blocktxn_matrix_rejects_bad_transactions() {
    let cases = [
        (
            "trailing bytes",
            {
                let mut payload = blocktxn_payload(&[]);
                payload.push(0xff);
                payload
            },
            "trailing data: 1 bytes",
        ),
        (
            "non-canonical transaction count",
            {
                let mut payload = Vec::new();
                payload.extend_from_slice(BlockHash::from_byte_array([17_u8; 32]).as_bytes());
                payload.extend_from_slice(&[0xfd, 1, 0]);
                payload
            },
            "non-canonical compact size for value 1",
        ),
        (
            "truncated transaction",
            blocktxn_payload(&[vec![0x01]]),
            "unexpected EOF: needed",
        ),
        (
            "superfluous witness records",
            blocktxn_payload(&[encoded_superfluous_witness_transaction()]),
            "superfluous witness record",
        ),
    ];

    for (name, payload, expected) in cases {
        // Act
        let error = decode_block_transactions_payload(&payload).expect_err(name);

        // Assert
        assert_error_contains(error, expected);
    }
}
