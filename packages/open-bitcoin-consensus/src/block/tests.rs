// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/block.h
// - packages/bitcoin-knots/src/consensus/merkle.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_codec::parse_block_header;
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, COIN, MAX_MONEY, MerkleRoot, OutPoint, ScriptBuf,
    ScriptWitness, Transaction, TransactionInput, TransactionOutput, Txid,
};

use super::difficulty::{difficulty_adjustment_interval, next_work_required};
use super::{
    block_sigop_overflow, block_subsidy, block_witness_merkle_root, check_block,
    check_block_contextual, check_block_header, check_block_header_contextual,
    coinbase_has_height_prefix, compact_size_len, enforce_coinbase_reward_limit,
    enforce_sigop_cost_limit, legacy_sigop_cost, map_codec_error, map_script_error,
    map_transaction_validation_error, serialized_block_size, serialized_script_num,
    split_sigop_cost, validate_block, validate_block_with_context, witness_commitment_index,
};
use crate::MAX_BLOCK_SIGOPS_COST;
use crate::context::{
    BlockValidationContext, ConsensusParams, MinDifficultyRecoveryTarget, RetargetAnchor,
    ScriptVerifyFlags, SpentOutput, TransactionInputContext, TransactionValidationContext,
};
use crate::crypto::{block_hash, block_merkle_root, transaction_txid};
use crate::validation::{BlockValidationResult, TxValidationResult, tx_error};

const EASY_BITS: u32 = 0x207f_ffff;
const GENESIS_BLOCK_HEADER_HEX: &str =
    include_str!("../../../open-bitcoin-codec/testdata/block_header.hex");

fn decode_hex(input: &str) -> Vec<u8> {
    let trimmed = input.trim();
    let mut bytes = Vec::with_capacity(trimmed.len() / 2);
    let chars: Vec<char> = trimmed.chars().collect();
    for pair in chars.chunks(2) {
        let high = pair[0].to_digit(16).expect("hex fixture");
        let low = pair[1].to_digit(16).expect("hex fixture");
        bytes.push(((high << 4) | low) as u8);
    }
    bytes
}

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

fn spend_transaction(previous_txid: Txid) -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x52]),
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

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected to find a nonce for easy regtest target");
}

fn mined_block(transactions: Vec<Transaction>) -> Block {
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root,
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn valid_block() -> (Block, Vec<Vec<SpentOutput>>) {
    let coinbase = coinbase_transaction();
    let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
    let spend = spend_transaction(coinbase_txid);
    let block = mined_block(vec![coinbase.clone(), spend.clone()]);

    let spent_outputs = vec![vec![SpentOutput {
        value: coinbase.outputs[0].value,
        script_pubkey: coinbase.outputs[0].script_pubkey.clone(),
        is_coinbase: true,
    }]];

    (block, spent_outputs)
}

fn p2sh_sigop_heavy_redeem_script(sigops: usize) -> ScriptBuf {
    let mut bytes = Vec::with_capacity(sigops + 4);
    bytes.push(0x00);
    bytes.push(0x63);
    bytes.extend(std::iter::repeat_n(0xac, sigops));
    bytes.push(0x68);
    bytes.push(0x51);
    script(&bytes)
}

fn p2sh_sigop_heavy_transaction(
    txid_byte: u8,
    sigops: usize,
) -> (Transaction, TransactionValidationContext) {
    let redeem_script = p2sh_sigop_heavy_redeem_script(sigops);
    let redeem_hash = crate::crypto::hash160(redeem_script.as_bytes());
    let script_pubkey = {
        let mut bytes = vec![0xa9, 20];
        bytes.extend_from_slice(&redeem_hash);
        bytes.push(0x87);
        script(&bytes)
    };
    let script_sig = {
        let mut bytes = vec![redeem_script.as_bytes().len() as u8];
        bytes.extend_from_slice(redeem_script.as_bytes());
        script(&bytes)
    };
    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([txid_byte; 32]),
                vout: 0,
            },
            script_sig,
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey,
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 1,
        median_time_past: 1,
        verify_flags: ScriptVerifyFlags::P2SH,
        consensus_params: ConsensusParams::default(),
    };
    (transaction, context)
}

fn unique_txid(seed: u64) -> Txid {
    let mut bytes = [0_u8; 32];
    bytes[..8].copy_from_slice(&seed.to_le_bytes());
    Txid::from_byte_array(bytes)
}

fn reward_limit_block(coinbase_value_sats: i64) -> Block {
    let mut coinbase = coinbase_transaction();
    coinbase.outputs[0].value = Amount::from_sats(coinbase_value_sats).expect("valid amount");
    let spend = spend_transaction(unique_txid(1));

    mined_block(vec![coinbase, spend])
}

fn reward_limit_block_context(block: &Block) -> BlockValidationContext {
    BlockValidationContext {
        height: 1,
        previous_header: BlockHeader {
            bits: block.header.bits,
            time: block.header.time - 1,
            ..BlockHeader::default()
        },
        maybe_retarget_anchor: None,
        maybe_min_difficulty_recovery_target: Some(MinDifficultyRecoveryTarget {
            bits: block.header.bits,
        }),
        previous_median_time_past: i64::from(block.header.time) - 1,
        current_time: i64::from(block.header.time),
        consensus_params: ConsensusParams {
            enforce_segwit: false,
            ..Default::default()
        },
    }
}

fn reward_limit_transaction_context(
    block: &Block,
    input_value_sats: i64,
) -> TransactionValidationContext {
    TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(input_value_sats).expect("valid amount"),
                script_pubkey: script(&[0x52]),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: i64::from(block.header.time),
        median_time_past: i64::from(block.header.time) - 1,
        verify_flags: ScriptVerifyFlags::NONE,
        consensus_params: ConsensusParams {
            enforce_segwit: false,
            ..Default::default()
        },
    }
}

mod contextual_block_checks_cover_context_mapping_and_nonfinal_rejection;
mod contextual_helpers_cover_merkle_height_and_weight_edges;
mod genesis_header_fixture_passes_pow_check;

#[test]
fn owned_oversized_witness_maps_codec_refusal_before_commitment_comparison() {
    // Arrange
    let (mut block, _) = valid_block();
    let mut commitment_script = vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed];
    commitment_script.extend_from_slice(&[0; 32]);
    block.transactions[0].outputs.push(TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: ScriptBuf::from_serialized_bytes(commitment_script),
    });
    block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![0; 32]]);
    block.transactions[1].inputs[0].witness =
        ScriptWitness::new(vec![vec![
            0;
            usize::try_from(open_bitcoin_codec::MAX_SIZE + 1)
                .expect("bounded codec oversize")
        ]]);
    block.header.merkle_root = block_merkle_root(&block.transactions)
        .expect("real txid root")
        .0;
    mine_header(&mut block);
    let mut context = reward_limit_block_context(&block);
    context.consensus_params.enforce_segwit = true;
    let header_before = block.header.clone();

    // Act
    let error = super::witness::check_witness_commitment(&block, &context)
        .expect_err("owned oversized witness must fail encoding");

    // Assert
    assert_eq!(error.result, BlockValidationResult::Consensus);
    assert_eq!(error.reject_reason, "bad-blk-serialization");
    assert_eq!(block.header, header_before);
    assert_eq!(
        block.transactions[1].inputs[0].witness.stack()[0].len(),
        usize::try_from(open_bitcoin_codec::MAX_SIZE + 1).expect("bounded size")
    );
}

#[test]
fn contextual_p2sh_sigop_budget_rejects_only_after_intrinsic_and_header_checks() {
    // Arrange
    let redeem = p2sh_sigop_heavy_redeem_script(199);
    let redeem_len = u8::try_from(redeem.as_bytes().len()).expect("bounded PUSHDATA1 length");
    let mut pushed_redeem = vec![0x4c, redeem_len];
    pushed_redeem.extend_from_slice(redeem.as_bytes());
    let mut transactions = vec![coinbase_transaction()];
    let mut contexts = Vec::new();
    for index in 1..=101_u8 {
        let (mut transaction, context) = p2sh_sigop_heavy_transaction(index, 199);
        transaction.inputs[0].script_sig = script(&pushed_redeem);
        transactions.push(transaction);
        contexts.push(context);
    }
    let block = mined_block(transactions);
    let block_context = reward_limit_block_context(&block);
    let before = block.clone();

    // Act
    let intrinsic = check_block(&block);
    let contextual = check_block_contextual(&block, &block_context);
    let result = validate_block_with_context(&block, &contexts, &block_context);

    // Assert
    assert_eq!(intrinsic, Ok(()));
    assert_eq!(contextual, Ok(()));
    assert_eq!(
        result
            .expect_err("P2SH cost exceeds the block limit")
            .reject_reason,
        "bad-blk-sigops"
    );
    assert_eq!(block, before);
}
