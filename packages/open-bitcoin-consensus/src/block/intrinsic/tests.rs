// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/tx_check.cpp
// - packages/bitcoin-knots/src/consensus/merkle.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::*;
use crate::crypto::{block_hash, transaction_txid_view};
use open_bitcoin_codec::transaction::wire::{WireOutput, WireTransaction};
use open_bitcoin_primitives::{
    BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness, TransactionInput, Txid,
};

fn coinbase(value: i64) -> WireTransaction {
    WireTransaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_serialized_bytes(vec![1, 1]),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![WireOutput {
            value_sats: value,
            script_pubkey: ScriptBuf::from_serialized_bytes(vec![0x51]),
        }],
        lock_time: 0,
    }
}
fn spend(value: i64) -> WireTransaction {
    let mut transaction = coinbase(value);
    transaction.inputs[0].previous_output = OutPoint {
        txid: Txid::from_byte_array([1; 32]),
        vout: 0,
    };
    transaction
}
fn mine(block: &mut WireBlock) {
    for nonce in 0..100000 {
        block.header.nonce = nonce;
        if crate::block::check_block_header(&block.header).is_ok() {
            return;
        }
    }
    panic!("bounded easy-target test mining exhausted");
}
fn block(transactions: Vec<WireTransaction>) -> WireBlock {
    let (merkle_root, _) = block_merkle_root_view(&transactions).expect("exact root");
    let mut block = WireBlock {
        header: BlockHeader {
            version: 1,
            merkle_root,
            bits: 0x207f_ffff,
            ..Default::default()
        },
        transactions,
    };
    mine(&mut block);
    block
}
fn reason(block: &WireBlock) -> &'static str {
    check_submission_wire_block(block, &ConsensusParams::default())
        .expect_err("invalid block")
        .reject_reason
}

#[test]
fn phase167_submission_wire_intrinsic_selected_pow_precedes_merkle_and_amount() {
    // Arrange
    let mut block = block(vec![coinbase(-1)]);
    block.header.merkle_root = MerkleRoot::from_byte_array([9; 32]);
    mine(&mut block);
    let selected = ConsensusParams {
        pow_limit_bits: 0x1d00_ffff,
        ..Default::default()
    };
    // Act
    let error =
        check_submission_wire_block(&block, &selected).expect_err("selected target limit first");
    // Assert
    assert_eq!(error.result, BlockValidationResult::InvalidHeader);
    assert_eq!(error.reject_reason, "high-hash");
    assert_eq!(reason(&block), "bad-txnmrklroot");
}

#[test]
fn phase167_submission_wire_intrinsic_merkle_precedes_empty_size_and_amount() {
    // Arrange
    let mut wide = coinbase(-1);
    wide.outputs[0].script_pubkey = ScriptBuf::from_serialized_bytes(vec![0; 1_000_000]);
    for (transactions, expected) in [
        (vec![], "bad-blk-length"),
        (vec![coinbase(-1)], "bad-txns-vout-negative"),
        (vec![wide], "bad-blk-length"),
    ] {
        let mut block = block(transactions);
        block.header.merkle_root = MerkleRoot::from_byte_array([9; 32]);
        mine(&mut block);
        // Act / Assert
        assert_eq!(reason(&block), "bad-txnmrklroot");
        block.header.merkle_root = block_merkle_root_view(&block.transactions).expect("root").0;
        mine(&mut block);
        assert_eq!(reason(&block), expected);
    }
}

#[test]
fn phase167_submission_wire_intrinsic_coinbase_precedes_amount() {
    // Arrange / Act / Assert
    assert_eq!(reason(&block(vec![spend(-1)])), "bad-cb-missing");
    let mut second = coinbase(-1);
    second.lock_time = 1;
    assert_eq!(
        reason(&block(vec![coinbase(-1), second])),
        "bad-cb-multiple"
    );
    assert_eq!(reason(&block(vec![coinbase(-1)])), "bad-txns-vout-negative");
}

#[test]
fn phase167_submission_wire_intrinsic_mutation_precedes_transaction_checks() {
    // Arrange
    let duplicate = spend(-1);
    let block = block(vec![coinbase(0), spend(0), duplicate.clone(), duplicate]);
    // Act / Assert
    assert_eq!(reason(&block), "bad-txns-duplicate");
}

#[test]
fn phase167_submission_wire_intrinsic_transactions_precede_legacy_sigops() {
    // Arrange
    let mut transaction = spend(-1);
    transaction.outputs[0].script_pubkey = ScriptBuf::from_serialized_bytes(vec![0xac; 20001]);
    transaction.inputs.push(transaction.inputs[0].clone());
    // Act / Assert
    assert_eq!(
        reason(&block(vec![coinbase(0), transaction.clone()])),
        "bad-txns-vout-negative"
    );
    transaction.outputs[0].value_sats = 0;
    assert_eq!(
        reason(&block(vec![coinbase(0), transaction.clone()])),
        "bad-txns-inputs-duplicate"
    );
    transaction.inputs.pop();
    assert_eq!(
        reason(&block(vec![coinbase(0), transaction.clone()])),
        "bad-blk-sigops"
    );
    transaction.outputs[0].script_pubkey = ScriptBuf::from_serialized_bytes(vec![0xac; 20000]);
    assert!(
        check_submission_wire_block(
            &block(vec![coinbase(0), transaction]),
            &ConsensusParams::default()
        )
        .is_ok()
    );
}

#[test]
fn phase167_submission_wire_intrinsic_raw_complete_decode_before_consensus() {
    // Arrange
    let block = block(vec![coinbase(-1)]);
    let bytes = open_bitcoin_codec::block::encode_block_view(&block).expect("raw encode");
    let mut truncated = bytes.clone();
    truncated.pop();
    // Act / Assert
    assert!(matches!(
        open_bitcoin_codec::block::wire::parse_submission_block_prefix(&truncated),
        Err(CodecError::UnexpectedEof { .. })
    ));
    let (decoded, consumed) =
        open_bitcoin_codec::block::wire::parse_submission_block_prefix(&bytes)
            .expect("complete signed raw");
    assert_eq!(consumed, bytes.len());
    assert_eq!(reason(&decoded), "bad-txns-vout-negative");
}

#[test]
fn phase167_submission_wire_intrinsic_witness_reserve_changes_no_intrinsic_input() {
    // Arrange
    for value in [0, -1] {
        let spend = spend(0);
        let witness_leaf = crate::crypto::transaction_wtxid_view(&spend)
            .expect("witness leaf")
            .to_byte_array();
        let witness_root = crate::crypto::double_sha256(&[[0; 32], witness_leaf].concat());
        let commitment = crate::crypto::double_sha256(&[witness_root, [0; 32]].concat());
        let mut coinbase = coinbase(value);
        coinbase.outputs.push(WireOutput {
            value_sats: 0,
            script_pubkey: ScriptBuf::from_serialized_bytes(
                [
                    vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed],
                    commitment.to_vec(),
                ]
                .concat(),
            ),
        });
        let original = block(vec![coinbase, spend]);
        let mut reserved = original.clone();
        reserved.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![0; 32]]);
        // Act / Assert
        assert_eq!(
            encode_block_header(&original.header),
            encode_block_header(&reserved.header)
        );
        assert_eq!(block_hash(&original.header), block_hash(&reserved.header));
        assert_eq!(
            block_merkle_root_view(&original.transactions),
            block_merkle_root_view(&reserved.transactions)
        );
        assert_eq!(
            serialized_block_size(&original, false),
            serialized_block_size(&reserved, false)
        );
        assert_eq!(
            sigops::legacy_block_sigop_cost(&original),
            sigops::legacy_block_sigop_cost(&reserved)
        );
        for (left, right) in original.transactions.iter().zip(&reserved.transactions) {
            assert_eq!(transaction_txid_view(left), transaction_txid_view(right));
            let stripped_left = encode_transaction_view(left, TransactionEncoding::WithoutWitness)
                .expect("stripped");
            let stripped_right =
                encode_transaction_view(right, TransactionEncoding::WithoutWitness)
                    .expect("stripped");
            assert_eq!(stripped_left, stripped_right);
            assert_eq!(stripped_left.len(), stripped_right.len());
            assert_eq!(is_coinbase(left), is_coinbase(right));
            assert_eq!(left.outputs, right.outputs);
            assert_eq!(
                left.inputs
                    .iter()
                    .map(|input| &input.previous_output)
                    .collect::<Vec<_>>(),
                right
                    .inputs
                    .iter()
                    .map(|input| &input.previous_output)
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                sigops::legacy_sigop_cost(left),
                sigops::legacy_sigop_cost(right)
            );
        }
        assert_eq!(
            check_submission_wire_block(&original, &ConsensusParams::default()),
            check_submission_wire_block(&reserved, &ConsensusParams::default())
        );
        assert_ne!(
            serialized_block_size(&original, true),
            serialized_block_size(&reserved, true)
        );
        if value == 0 {
            assert!(check_submission_wire_block(&reserved, &ConsensusParams::default()).is_ok());
        }
    }
}

#[test]
fn phase167_submission_wire_intrinsic_selected_limit_preserves_typed_pow_default() {
    // Arrange
    let wire = block(vec![coinbase(0)]);
    let typed = wire.clone().try_into_block().expect("valid amount");
    let strict = ConsensusParams {
        pow_limit_bits: 0x1d00_ffff,
        ..Default::default()
    };
    // Act / Assert
    assert!(crate::block::check_block(&typed).is_ok());
    assert_eq!(
        check_submission_wire_block(&wire, &strict)
            .expect_err("selected limit")
            .reject_reason,
        "high-hash"
    );
}

#[test]
fn phase167_submission_wire_intrinsic_typed_and_wire_share_checks() {
    // Arrange
    let mut transactions = vec![coinbase(0), spend(0)];
    for script in [vec![0x51; 10001], vec![0xac; 20001]] {
        transactions[1].outputs[0].script_pubkey = ScriptBuf::from_serialized_bytes(script);
        let wire = block(transactions.clone());
        let typed = wire.clone().try_into_block().expect("money valid");
        // Act / Assert
        assert_eq!(
            crate::block::check_block(&typed),
            check_submission_wire_block(&wire, &ConsensusParams::default())
        );
    }
}
