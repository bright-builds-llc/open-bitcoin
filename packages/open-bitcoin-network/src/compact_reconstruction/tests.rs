// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{
    short_id_from_masked_u64, short_id_match_key, short_id_selector_from_header_and_nonce,
    CompactBlockPayload, PrefilledTransaction, ShortId,
};
use open_bitcoin_consensus::{compact_short_id_for_wtxid, transaction_wtxid};
use open_bitcoin_primitives::{
    Amount, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid, Wtxid,
};

use super::{
    apply_block_transactions, fill_block, init_partial_compact_block, CompactBlockTxnMisbehavior,
    CompactBlockTxnOutcome, CompactReconstructionFailureReason, CompactReconstructionInvalidReason,
    CompactReconstructionOutcome, PartialCompactBlock, MAX_COMPACT_BLOCK_TRANSACTION_COUNT,
};

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

fn coinbase_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(vec![0x01, 0x02]).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(50_000_000_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn sample_transaction(previous_txid_byte: u8, vout: u32, output_value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([previous_txid_byte; 32]),
                vout,
            },
            script_sig: ScriptBuf::from_bytes(Vec::new()).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x01, 0x02]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(output_value).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51, 0xac]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn transaction_pair(transaction: &Transaction) -> (Wtxid, Transaction) {
    let wtxid = transaction_wtxid(transaction).expect("wtxid");
    (wtxid, transaction.clone())
}

fn compact_payload_with_short_ids(
    header: BlockHeader,
    nonce: u64,
    prefilled: Vec<PrefilledTransaction>,
    transactions_for_short_ids: &[Transaction],
) -> CompactBlockPayload {
    let selector = short_id_selector_from_header_and_nonce(&header, nonce);
    let short_ids = transactions_for_short_ids
        .iter()
        .map(|transaction| {
            let wtxid = transaction_wtxid(transaction).expect("wtxid");
            compact_short_id_for_wtxid(selector, &wtxid)
        })
        .collect();

    CompactBlockPayload {
        header,
        nonce,
        short_ids,
        prefilled_transactions: prefilled,
    }
}

mod application_cases;
mod candidate_cases;
mod initialization_cases;
mod lifecycle_cases;
