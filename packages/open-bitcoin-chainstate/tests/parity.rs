// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use open_bitcoin_chainstate::{
    AnchoredBlock, ChainPosition, Chainstate, ChainstateError, prefer_candidate_tip,
};
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header, transaction_txid,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput,
};

const EASY_BITS: u32 = 0x207f_ffff;

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut magnitude = value as u64;
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    let mut script = Vec::with_capacity(encoded.len() + 2);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script.push(0x51);
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
    previous_txid: open_bitcoin_primitives::Txid,
    vout: u32,
    value: i64,
) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout,
            },
            script_sig: script(&[0x51]),
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

fn op_return_transaction(previous_txid: open_bitcoin_primitives::Txid) -> Transaction {
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
        .expect("expected nonce at easy target");
}

fn build_block(previous_block_hash: BlockHash, time: u32, transactions: Vec<Transaction>) -> Block {
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn connect_block(chainstate: &mut Chainstate, block: &Block, chain_work: u128) {
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
        .expect("block should connect");
}

#[test]
fn connect_disconnect_and_reorg_preserve_phase_four_outcomes() {
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    connect_block(&mut chainstate, &genesis_block, 1);

    let branch_a = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    );
    connect_block(&mut chainstate, &branch_a, 2);
    assert_eq!(chainstate.utxos().len(), 2);

    let branch_b = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_006_650,
        vec![
            coinbase_transaction(1, 50),
            spend_transaction(transaction_txid(&genesis_coinbase).expect("txid"), 0, 30),
        ],
    );

    let branch_a_tip = chainstate.tip().cloned().expect("branch a tip");
    let branch_b_tip = ChainPosition::new(
        branch_b.header.clone(),
        1,
        3,
        i64::from(branch_b.header.time),
    );
    assert!(prefer_candidate_tip(&branch_a_tip, &branch_b_tip));

    let transition = chainstate
        .reorg(
            std::slice::from_ref(&branch_a),
            &[AnchoredBlock {
                block: branch_b.clone(),
                chain_work: 3,
            }],
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("heavier branch should activate");

    assert_eq!(transition.disconnected.len(), 1);
    assert_eq!(transition.connected.len(), 1);
    assert_eq!(chainstate.tip(), Some(&transition.connected[0]));
    assert_eq!(chainstate.utxos().len(), 2);
}

#[test]
fn unspendable_outputs_do_not_enter_the_utxo_view() {
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    connect_block(&mut chainstate, &genesis_block, 1);

    let block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_006_700,
        vec![
            coinbase_transaction(1, 50),
            op_return_transaction(transaction_txid(&genesis_coinbase).expect("txid")),
        ],
    );
    connect_block(&mut chainstate, &block, 2);

    assert_eq!(chainstate.utxos().len(), 1);
}

#[test]
fn bip30_style_output_overwrites_are_rejected() {
    let mut chainstate = Chainstate::new();
    let duplicate_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![duplicate_coinbase.clone()],
    );
    connect_block(&mut chainstate, &genesis_block, 1);

    let duplicate_block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_006_600,
        vec![duplicate_coinbase],
    );
    let error = chainstate
        .connect_block(
            &duplicate_block,
            2,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                enforce_bip34_height_in_coinbase: false,
                ..ConsensusParams::default()
            },
        )
        .expect_err("duplicate coinbase outputs should be rejected");

    assert!(matches!(error, ChainstateError::OutputOverwrite { .. }));
}
