// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/merkle.cpp

use super::*;
use open_bitcoin_codec::block::wire::WireBlock;
use open_bitcoin_codec::transaction::wire::{OutputRef, TransactionView};
use open_bitcoin_primitives::{
    Amount, Block, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
    TransactionOutput, Txid,
};

fn transaction(coinbase: bool, stripped_size: usize) -> Transaction {
    // One input, one output and one-byte CompactSize fields occupy 60 bytes.
    assert!((60..=250).contains(&stripped_size));
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: if coinbase {
                OutPoint::null()
            } else {
                OutPoint {
                    txid: Txid::from_byte_array([7; 32]),
                    vout: 0,
                }
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_serialized_bytes(vec![0x51; stripped_size - 60]),
        }],
        lock_time: 0,
    }
}

fn body(transactions: Vec<Transaction>) -> Block {
    let mut block = Block {
        header: BlockHeader::default(),
        transactions,
    };
    reseal(&mut block);
    block
}

fn reseal(block: &mut Block) {
    block.header.merkle_root = block_merkle_root_view(&block.transactions)
        .expect("txid root")
        .0;
}

fn committed() -> Block {
    let mut spend = transaction(false, 65);
    spend.inputs[0].witness = ScriptWitness::new(vec![vec![0x51]]);
    let mut block = body(vec![transaction(true, 64), spend]);
    block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![9; 32]]);
    let root = witness_merkle_root(&block).expect("shared witness fact");
    let mut preimage = [9; 64];
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
    reseal(&mut block);
    block
}

fn assert_preserved(block: &Block, selected: bool, expected: bool) {
    let before = block.clone();
    let bytes = open_bitcoin_codec::encode_block(block).expect("exact input bytes");
    let hash = crate::block_hash(&block.header);
    assert_eq!(
        super::super::is_block_mutated(block, selected),
        Ok(expected)
    );
    assert_eq!(block, &before);
    assert_eq!(
        open_bitcoin_codec::encode_block(block).expect("unchanged bytes"),
        bytes
    );
    assert_eq!(crate::block_hash(&block.header), hash);
}

#[test]
fn phase167_compact_mutation_empty_matching_zero_root_is_false() {
    // Arrange
    let block = body(vec![]);
    // Act / Assert
    assert_preserved(&block, true, false);
}

#[test]
fn phase167_compact_mutation_noncoinbase_exact_63_64_65_boundary() {
    for (length, expected) in [(63, false), (64, true), (65, false)] {
        // Arrange
        let block = body(vec![transaction(false, length)]);
        let encoded =
            encode_transaction_view(&block.transactions[0], TransactionEncoding::WithoutWitness)
                .expect("length");
        assert_eq!(encoded.len(), length);
        // Act / Assert
        assert_preserved(&block, true, expected);
    }
}

#[test]
fn phase167_compact_mutation_noncoinbase_later_64_is_true() {
    // Arrange
    let block = body(vec![transaction(false, 63), transaction(false, 64)]);
    // Act / Assert
    assert_preserved(&block, false, true);
}

#[test]
fn phase167_compact_mutation_noncoinbase_no_64_skips_bad_selected_witness() {
    // Arrange
    let mut transaction = transaction(false, 65);
    transaction.inputs[0].witness = ScriptWitness::new(vec![vec![1]]);
    let block = body(vec![transaction]);
    // Act / Assert
    assert_preserved(&block, true, false);
    assert_preserved(&block, false, false);
}

#[test]
fn phase167_compact_mutation_first_coinbase_64_uses_witness_branch() {
    // Arrange
    let mut block = body(vec![transaction(true, 64)]);
    // Act / Assert
    assert_preserved(&block, true, false);
    block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![1]]);
    assert_preserved(&block, true, true);
}

#[test]
fn phase167_compact_mutation_merkle_mismatch_wins_before_early_return() {
    // Arrange
    let mut block = body(vec![transaction(false, 65)]);
    block.header.merkle_root = MerkleRoot::from_byte_array([3; 32]);
    // Act / Assert
    assert_preserved(&block, true, true);
}

#[test]
fn phase167_compact_mutation_merkle_mismatch_wins_before_witness_branch() {
    // Arrange
    let mut block = committed();
    block.header.merkle_root = MerkleRoot::from_byte_array([3; 32]);
    // Act / Assert
    assert_preserved(&block, false, true);
}

#[test]
fn phase167_compact_mutation_real_duplicate_pairs_and_subtrees_are_true() {
    for subtree in [false, true] {
        // Arrange
        let first = transaction(false, 65);
        let mut second = first.clone();
        second.lock_time = 1;
        let transactions = if subtree {
            vec![first.clone(), second.clone(), first, second]
        } else {
            vec![first.clone(), first]
        };
        let block = body(transactions);
        // Act / Assert
        assert_preserved(&block, true, true);
    }
}

#[test]
fn phase167_compact_mutation_ordinary_odd_padding_is_not_duplicate() {
    // Arrange
    let transactions = (0..3)
        .map(|index| {
            let mut transaction = transaction(false, 65);
            transaction.lock_time = index;
            transaction
        })
        .collect();
    let block = body(transactions);
    // Act / Assert
    assert_preserved(&block, true, false);
}

#[test]
fn phase167_compact_mutation_complete_domain_and_raw_views_agree() {
    // Arrange
    let block = committed();
    let raw = WireBlock {
        header: block.header.clone(),
        transactions: block
            .transactions
            .clone()
            .into_iter()
            .map(Into::into)
            .collect(),
    };
    // Act
    let domain = super::super::is_block_mutated(&block, true);
    let wire = super::super::is_block_mutated(&raw, true);
    // Assert
    assert_eq!(domain, Ok(false));
    assert_eq!(wire, Ok(false));
    assert_eq!(
        super::super::witness_merkle_root(&raw),
        super::super::witness_merkle_root(&block)
    );
}

#[test]
fn phase167_compact_mutation_witness_false_rejects_present_witness() {
    // Arrange
    let block = committed();
    // Act / Assert
    assert_preserved(&block, false, true);
}

#[test]
fn phase167_compact_mutation_missing_commitment_rejects_witness() {
    // Arrange
    let mut block = committed();
    block.transactions[0].outputs.pop();
    reseal(&mut block);
    // Act / Assert
    assert_preserved(&block, true, true);
}

#[test]
fn phase167_compact_mutation_no_witness_no_commitment_is_false() {
    // Arrange
    let block = body(vec![transaction(true, 65)]);
    // Act / Assert
    assert_preserved(&block, false, false);
    assert_preserved(&block, true, false);
}

#[test]
fn phase167_compact_mutation_reserve_cardinality_and_31_32_33_sizes() {
    for (stack, expected) in [
        (vec![], true),
        (vec![vec![9; 31]], true),
        (vec![vec![9; 32]], false),
        (vec![vec![9; 33]], true),
        (vec![vec![9; 32], vec![]], true),
    ] {
        // Arrange
        let mut block = committed();
        block.transactions[0].inputs[0].witness = ScriptWitness::new(stack);
        // Act / Assert
        assert_preserved(&block, true, expected);
    }
}

#[test]
fn phase167_compact_mutation_wrong_witness_root_is_true() {
    // Arrange
    let mut block = committed();
    block.transactions[1].inputs[0].witness = ScriptWitness::new(vec![vec![0x52]]);
    // Act / Assert
    assert_preserved(&block, true, true);
}

#[test]
fn phase167_compact_mutation_highest_matching_output_and_trailing_bytes() {
    // Arrange
    let mut block = committed();
    let valid = block.transactions[0]
        .outputs
        .last()
        .expect("commitment")
        .clone();
    let mut invalid = valid.clone();
    let mut bytes = invalid.script_pubkey.as_bytes().to_vec();
    bytes[6] ^= 1;
    invalid.script_pubkey = ScriptBuf::from_serialized_bytes(bytes);
    block.transactions[0].outputs.push(invalid);
    reseal(&mut block);
    // Act / Assert
    assert_preserved(&block, true, true);
    let mut trailing = valid.script_pubkey.as_bytes().to_vec();
    trailing.push(0xff);
    block.transactions[0].outputs.push(TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: ScriptBuf::from_serialized_bytes(trailing),
    });
    reseal(&mut block);
    assert_preserved(&block, true, false);
}

#[test]
fn phase167_compact_mutation_nonmutation_consensus_invalidity_stays_separate() {
    // Arrange
    let mut block = body(vec![transaction(true, 65)]);
    block.header.bits = 0;
    block.transactions[0].outputs[0].value =
        Amount::from_sats(open_bitcoin_primitives::MAX_MONEY).expect("amount");
    let output = block.transactions[0].outputs[0].clone();
    block.transactions[0].outputs.push(output);
    reseal(&mut block);
    // Act / Assert
    assert!(crate::block::check_block_header(&block.header).is_err());
    assert_preserved(&block, true, false);
}

struct OversizedOutputTransaction {
    script: ScriptBuf,
}

impl TransactionView for OversizedOutputTransaction {
    fn version(&self) -> i32 {
        1
    }
    fn inputs(&self) -> &[TransactionInput] {
        &[]
    }
    fn outputs(&self) -> impl ExactSizeIterator<Item = OutputRef<'_>> {
        std::iter::repeat_n((), 33_554_433).map(|()| OutputRef {
            value_sats: 0,
            script_pubkey: &self.script,
        })
    }
    fn lock_time(&self) -> u32 {
        0
    }
}

struct OversizedOutputBlock {
    header: BlockHeader,
    transactions: [OversizedOutputTransaction; 1],
}

impl BlockView for OversizedOutputBlock {
    type Transaction = OversizedOutputTransaction;
    fn header(&self) -> &BlockHeader {
        &self.header
    }
    fn transactions(&self) -> &[Self::Transaction] {
        &self.transactions
    }
}

#[test]
fn phase167_compact_mutation_encoding_error_propagates_without_large_allocation() {
    // Arrange
    let block = OversizedOutputBlock {
        header: BlockHeader::default(),
        transactions: [OversizedOutputTransaction {
            script: ScriptBuf::default(),
        }],
    };
    // Act
    let result = super::super::is_block_mutated(&block, true);
    // Assert
    assert!(matches!(
        result,
        Err(CodecError::CompactSizeTooLarge(33_554_433))
    ));
}

#[test]
fn owned_oversized_witness_encoding_refuses_after_valid_txid_root() {
    // Arrange
    let mut block = committed();
    block.transactions[1].inputs[0].witness =
        ScriptWitness::new(vec![vec![
            0;
            usize::try_from(open_bitcoin_codec::MAX_SIZE + 1)
                .expect("bounded codec oversize")
        ]]);
    let txid_root = block_merkle_root_view(&block.transactions)
        .expect("unchanged txid root")
        .0;
    assert_eq!(txid_root, block.header.merkle_root);

    // Act
    let result = super::super::is_block_mutated(&block, true);

    // Assert
    assert_eq!(
        result,
        Err(CodecError::CompactSizeTooLarge(
            open_bitcoin_codec::MAX_SIZE + 1,
        ))
    );
}
