// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::collections::HashMap;

use open_bitcoin_consensus::{
    BlockValidationContext, ConsensusParams, ScriptVerifyFlags, block_merkle_root,
    check_block_header,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, MAX_MONEY, OutPoint, ScriptBuf, ScriptWitness,
    Transaction, TransactionInput, TransactionOutput, Txid,
};

use super::{
    Chainstate, accumulated_fee_out_of_range, apply_non_coinbase_transaction,
    build_transaction_context, compute_median_time_past, difficulty_adjustment_interval,
    prefer_candidate_tip, remove_spent_input, restore_non_coinbase_inputs,
    txid_serialization_error,
};
use crate::{AnchoredBlock, BlockUndo, ChainPosition, Coin, TxUndo};

const EASY_BITS: u32 = 0x207f_ffff;

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    if encoded.last().is_some_and(|byte| (byte & 0x80) != 0) {
        encoded.push(if negative { 0x80 } else { 0x00 });
    } else if negative {
        let last = encoded.last_mut().expect("value is non-zero");
        *last |= 0x80;
    }

    let mut script = Vec::with_capacity(encoded.len() + 1);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script
}

fn coinbase_transaction(height: u32, value: i64) -> Transaction {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn spend_transaction(
    previous_txid: Txid,
    previous_vout: u32,
    value: i64,
    sequence: u32,
) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: previous_vout,
            },
            script_sig: script(&[0x51]),
            sequence,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn op_return_transaction(previous_txid: Txid) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(10).expect("valid amount"),
            script_pubkey: script(&[0x6a, 0x01, 0x01]),
        }],
        lock_time: 0,
    }
}

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected to find a nonce for easy regtest target");
}

fn build_block(previous_block_hash: BlockHash, time: u32, transactions: Vec<Transaction>) -> Block {
    build_block_with_bits(previous_block_hash, time, EASY_BITS, transactions)
}

fn build_block_with_bits(
    previous_block_hash: BlockHash,
    time: u32,
    bits: u32,
    transactions: Vec<Transaction>,
) -> Block {
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time,
            bits,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn connect_block(chainstate: &mut Chainstate, block: &Block, chain_work: u128) -> ChainPosition {
    chainstate
        .connect_block(
            block,
            chain_work,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("block should connect")
}

fn subsidy_plus_fees_value(height: u32, fees_sats: i64, consensus_params: &ConsensusParams) -> i64 {
    open_bitcoin_consensus::block::block_subsidy(height, consensus_params).to_sats() + fees_sats
}

fn assert_active_tip(chainstate: &Chainstate, expected: &ChainPosition) {
    assert_eq!(chainstate.tip(), Some(expected));
}

mod apply_non_coinbase_transaction_returns_fee_and_records_undo;
mod derives_contexts_from_chainstate_metadata;
mod disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out;
mod script_num_helper_covers_negative_and_high_bit_cases;
