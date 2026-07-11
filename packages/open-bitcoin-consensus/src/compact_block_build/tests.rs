// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use open_bitcoin_codec::{
    CodecError, decode_compact_block_payload, encode_compact_block_payload,
    validate_compact_block_structure,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness,
    Transaction, TransactionInput, TransactionOutput, Txid,
};

use super::build_compact_block_payload;
use crate::crypto::{compact_short_id_for_wtxid, compact_short_id_selector, transaction_wtxid};

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn coinbase_transaction() -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&[0x01, 0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: script(&[0x52]),
        }],
        lock_time: 0,
    }
}

fn non_coinbase_transaction(previous_txid: Txid, marker: u8) -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[marker]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn block_with_transactions(transactions: Vec<Transaction>) -> Block {
    Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([1_u8; 32]),
            time: 1_231_006_505,
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions,
    }
}

#[test]
fn build_compact_block_payload_coinbase_only_prefills_coinbase() {
    // Arrange
    let coinbase = coinbase_transaction();
    let block = block_with_transactions(vec![coinbase.clone()]);
    let nonce = 0x1122_3344_5566_7788_u64;

    // Act
    let payload = build_compact_block_payload(&block, nonce).expect("coinbase-only block builds");

    // Assert
    assert_eq!(payload.header, block.header);
    assert_eq!(payload.nonce, nonce);
    assert!(payload.short_ids.is_empty());
    assert_eq!(payload.prefilled_transactions.len(), 1);
    assert_eq!(payload.prefilled_transactions[0].index_delta, 0);
    assert_eq!(payload.prefilled_transactions[0].transaction, coinbase);
    assert!(validate_compact_block_structure(&payload).is_ok());
}

#[test]
fn build_compact_block_payload_short_ids_match_wtxid_siphash() {
    // Arrange
    let coinbase = coinbase_transaction();
    let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
    let tx_a = non_coinbase_transaction(coinbase_txid, 0x51);
    let tx_b = non_coinbase_transaction(coinbase_txid, 0x52);
    let block = block_with_transactions(vec![coinbase.clone(), tx_a.clone(), tx_b.clone()]);
    let nonce = 7_u64;
    let selector = compact_short_id_selector(&block.header, nonce);
    let expected_a =
        compact_short_id_for_wtxid(selector, &transaction_wtxid(&tx_a).expect("tx_a wtxid"));
    let expected_b =
        compact_short_id_for_wtxid(selector, &transaction_wtxid(&tx_b).expect("tx_b wtxid"));

    // Act
    let payload = build_compact_block_payload(&block, nonce).expect("multi-tx block builds");

    // Assert
    assert_eq!(payload.prefilled_transactions.len(), 1);
    assert_eq!(payload.prefilled_transactions[0].index_delta, 0);
    assert_eq!(payload.prefilled_transactions[0].transaction, coinbase);
    assert_eq!(payload.short_ids, vec![expected_a, expected_b]);
    assert!(validate_compact_block_structure(&payload).is_ok());
}

#[test]
fn build_compact_block_payload_rejects_empty_transactions() {
    // Arrange
    let block = block_with_transactions(Vec::new());

    // Act
    let result = build_compact_block_payload(&block, 1);

    // Assert
    assert_eq!(result, Err(CodecError::CompactBlockEmpty));
}

#[test]
fn build_compact_block_payload_round_trips_encode_decode() {
    // Arrange
    let coinbase = coinbase_transaction();
    let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
    let spend = non_coinbase_transaction(coinbase_txid, 0x53);
    let block = block_with_transactions(vec![coinbase, spend]);
    let nonce = 99_u64;

    // Act
    let payload = build_compact_block_payload(&block, nonce).expect("block builds");
    let encoded = encode_compact_block_payload(&payload).expect("encode");
    let decoded = decode_compact_block_payload(&encoded).expect("decode");

    // Assert
    assert_eq!(decoded, payload);
}
