// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/merkle.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_consensus::block::{is_block_mutated, witness_merkle_root};
use open_bitcoin_consensus::crypto::double_sha256;
use open_bitcoin_consensus::{block_merkle_root, check_block_header};
use open_bitcoin_primitives::{
    Amount, Block, BlockHeader, ScriptBuf, ScriptWitness, Transaction, TransactionOutput,
};

pub(crate) fn coherent_body(
    header: BlockHeader,
    transactions: Vec<Transaction>,
    mine: bool,
) -> Block {
    let mut block = Block {
        header,
        transactions,
    };
    if block.transactions.iter().any(Transaction::has_witness) {
        block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![0; 32]]);
        let root = witness_merkle_root(&block).expect("fixture witness root");
        let mut preimage = [0_u8; 64];
        preimage[..32].copy_from_slice(root.as_bytes());
        let script = [
            vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed],
            double_sha256(&preimage).to_vec(),
        ]
        .concat();
        block.transactions[0].outputs.push(TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_serialized_bytes(script),
        });
    }
    let (root, mutated) = block_merkle_root(&block.transactions).expect("fixture txid root");
    assert!(!mutated);
    block.header.merkle_root = root;
    if mine {
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("easy fixture proof of work");
    }
    assert_coherent(&block);
    block
}

pub(crate) fn assert_coherent(block: &Block) {
    assert_eq!(is_block_mutated(block, true), Ok(false));
}
