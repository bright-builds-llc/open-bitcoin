// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/blockfilter.h
// - packages/bitcoin-knots/src/undo.h
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root, check_block_header,
    transaction_txid,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput,
};

use crate::{Chainstate, StagedChainstateConnect};

pub const FUNDING_SCRIPT: &[u8] = &[0x51];
pub const SAME_BLOCK_SCRIPT: &[u8] = &[0x52];
pub const FINAL_SCRIPT: &[u8] = &[0x53];
pub const COINBASE_SCRIPT: &[u8] = &[0x54];
pub const REWARD: i64 = 5_000_000_000;

pub struct SpendingChain {
    pub chainstate: Chainstate,
    pub genesis: Block,
    pub funding: Block,
    pub spending: Block,
}

pub fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("fixture script")
}

pub fn coinbase(height: u8, output_script: &[u8]) -> Transaction {
    let height_prefix = if height == 0 {
        vec![0x00, 0x51]
    } else {
        vec![1, height, 0x51]
    };
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&height_prefix),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(REWARD).expect("fixture reward"),
            script_pubkey: script(output_script),
        }],
        lock_time: 0,
    }
}

pub fn outpoint(transaction: &Transaction) -> OutPoint {
    OutPoint {
        txid: transaction_txid(transaction).expect("fixture txid"),
        vout: 0,
    }
}

pub fn spend(previous: &Transaction, value: i64, output_script: &[u8]) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: outpoint(previous),
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("fixture spend value"),
            script_pubkey: script(output_script),
        }],
        lock_time: 0,
    }
}

pub fn block(parent: BlockHash, time: u32, transactions: Vec<Transaction>) -> Block {
    let (merkle_root, mutated) = block_merkle_root(&transactions).expect("fixture merkle");
    assert!(!mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: parent,
            merkle_root,
            time,
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions,
    };
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("easy target fixture nonce");
    block
}

pub fn stage(chainstate: &Chainstate, block: &Block, work: u128) -> StagedChainstateConnect {
    chainstate
        .stage_connect_block_with_current_time(
            block,
            work,
            i64::from(block.header.time) + 1,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("continuous fixture validation")
}

pub fn spending_chain() -> SpendingChain {
    let mut chainstate = Chainstate::default();
    let genesis = block(BlockHash::default(), 1_000, vec![coinbase(0, &[0x55])]);
    let genesis_stage = stage(&chainstate, &genesis, 1);
    chainstate
        .commit_staged_connect(genesis_stage)
        .expect("genesis commit");
    let funding = block(
        block_hash(&genesis.header),
        1_100,
        vec![coinbase(1, FUNDING_SCRIPT)],
    );
    let funding_stage = stage(&chainstate, &funding, 2);
    chainstate
        .commit_staged_connect(funding_stage)
        .expect("funding commit");
    let first = spend(&funding.transactions[0], REWARD - 1_000, SAME_BLOCK_SCRIPT);
    let second = spend(&first, REWARD - 2_000, FINAL_SCRIPT);
    let spending = block(
        block_hash(&funding.header),
        1_200,
        vec![coinbase(2, COINBASE_SCRIPT), first, second],
    );
    SpendingChain {
        chainstate,
        genesis,
        funding,
        spending,
    }
}
