// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/blockfilter.h
// - packages/bitcoin-knots/src/undo.h
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptWitness, Transaction, TransactionInput,
    TransactionOutput,
};

use open_bitcoin_consensus::{
    BasicFilterEncodingError, CodecError, FilterHeaderError, FilterHeaderPredecessor,
    block_merkle_root,
};

use super::{
    BasicFilterGenerationError, BasicFilterInputError, BasicFilterInputs, HistoricalBlockUndo,
};
use crate::{BlockUndo, ChainPosition, TxUndo};

mod fixtures;

fn coinbase_block(height: u32) -> (Block, ChainPosition) {
    let mut block = Block {
        header: BlockHeader {
            previous_block_hash: if height == 0 {
                Default::default()
            } else {
                BlockHash::from_byte_array([1; 32])
            },
            ..BlockHeader::default()
        },
        transactions: vec![Transaction {
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: Default::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(0).expect("valid amount"),
                script_pubkey: Default::default(),
            }],
            ..Transaction::default()
        }],
    };
    block.header.merkle_root = block_merkle_root(&block.transactions)
        .expect("fixture merkle root")
        .0;
    let position = ChainPosition::new(block.header.clone(), height, 1, 0);
    (block, position)
}

fn rebind_body(block: &mut Block, position: &mut ChainPosition) {
    block.header.merkle_root = block_merkle_root(&block.transactions)
        .expect("fixture merkle root")
        .0;
    *position = ChainPosition::new(
        block.header.clone(),
        position.height,
        position.chain_work,
        position.median_time_past,
    );
}

#[test]
fn unchanged_validated_body_succeeds_but_changed_transaction_refuses_same_header() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    let before = fixture.chainstate.snapshot();
    let unchanged = staged.basic_filter_inputs(&fixture.spending);
    assert!(unchanged.is_ok());
    for replace_output in [true, false] {
        let mut altered = fixture.spending.clone();
        if replace_output {
            altered.transactions[1].outputs[0].script_pubkey = fixtures::script(&[0x56]);
        } else {
            altered.transactions[1].inputs[0].previous_output.txid =
                open_bitcoin_primitives::Txid::from_byte_array([9; 32]);
        }
        // Act
        let historical_error = historical_error(&altered, &staged.position, &staged.undo);
        let staged_error = staged
            .basic_filter_inputs(&altered)
            .expect_err("changed body must refuse");
        // Assert
        assert_eq!(altered.header, fixture.spending.header);
        assert_eq!(historical_error, BasicFilterInputError::BodyMerkleMismatch);
        assert_eq!(staged_error, historical_error);
        assert_eq!(fixture.chainstate.snapshot(), before);
    }
}

#[test]
fn genuine_merkle_mutation_refuses_even_matching_root_and_position() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    let mut block = fixture.spending.clone();
    let mut position = staged.position.clone();
    block.transactions.push(block.transactions[2].clone());
    rebind_body(&mut block, &mut position);
    assert_eq!(block.header, fixture.spending.header);
    assert_eq!(position, staged.position);
    let mut undo = staged.undo.clone();
    undo.transactions.push(undo.transactions[1].clone());
    // Act
    let error = historical_error(&block, &position, &undo);
    // Assert
    assert_eq!(error, BasicFilterInputError::MutatedBody);
}

#[test]
fn repeated_real_subtree_refuses_a_root_preserving_body_substitution() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    let mut original = fixture.spending.clone();
    let mut position = staged.position.clone();
    for lock_time in 1..=3 {
        let mut transaction = original.transactions[2].clone();
        transaction.lock_time = lock_time;
        original.transactions.push(transaction);
    }
    rebind_body(&mut original, &mut position);
    let mut altered = original.clone();
    altered
        .transactions
        .extend_from_slice(&original.transactions[4..6]);
    let (root, mutated) = block_merkle_root(&altered.transactions).expect("mutated subtree root");
    let mut undo = staged.undo.clone();
    let extra_undo = undo.transactions[1].clone();
    undo.transactions
        .resize(altered.transactions.len() - 1, extra_undo);
    // Act
    let error = historical_error(&altered, &position, &undo);
    // Assert
    assert_eq!(root, original.header.merkle_root);
    assert!(mutated);
    assert_eq!(error, BasicFilterInputError::MutatedBody);
}

#[test]
fn body_encoding_error_preserves_typed_source_and_context() {
    // Arrange
    let cause = CodecError::LengthOutOfRange {
        field: "transaction inputs",
        value: u64::MAX,
    };
    let error = BasicFilterInputError::BodyEncoding(cause.clone());
    // Act
    let source = std::error::Error::source(&error).expect("codec source");
    // Assert
    assert_eq!(source.downcast_ref::<CodecError>(), Some(&cause));
    assert_eq!(
        error.to_string(),
        format!("block body encoding failed: {cause}")
    );
}

#[test]
fn missing_non_genesis_undo_refuses_even_coinbase_only() {
    // Arrange
    let (block, position) = coinbase_block(1);
    // Act
    let result = BasicFilterInputs::from_historical(&block, &position, None);
    // Assert
    assert_eq!(result.err(), Some(BasicFilterInputError::MissingUndo));
}

#[test]
fn explicit_empty_non_genesis_history_succeeds() {
    // Arrange
    let (block, position) = coinbase_block(1);
    let undo = BlockUndo::default();
    // Act
    let inputs = BasicFilterInputs::from_historical(
        &block,
        &position,
        Some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: &undo,
        }),
    )
    .expect("complete history");
    // Assert
    assert_eq!(inputs.spent_scripts().count(), 0);
}

#[test]
fn genesis_alone_accepts_missing_undo() {
    // Arrange
    let (block, position) = coinbase_block(0);
    // Act
    let inputs =
        BasicFilterInputs::from_historical(&block, &position, None).expect("validated genesis");
    // Assert
    assert_eq!(inputs.spent_scripts().count(), 0);
}

fn historical_error(
    block: &Block,
    position: &ChainPosition,
    undo: &BlockUndo,
) -> BasicFilterInputError {
    BasicFilterInputs::from_historical(
        block,
        position,
        Some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo,
        }),
    )
    .expect_err("must refuse historical mismatch")
}

#[test]
fn different_header_refuses_validated_position() {
    // Arrange
    let (mut block, position) = coinbase_block(1);
    block.header.nonce += 1;
    // Act / Assert
    assert_eq!(
        historical_error(&block, &position, &BlockUndo::default()),
        BasicFilterInputError::BlockIdentity
    );
}

#[test]
fn different_position_hash_refuses_even_identical_header() {
    // Arrange
    let (block, mut position) = coinbase_block(1);
    position.block_hash = BlockHash::default();
    // Act / Assert
    assert_eq!(
        historical_error(&block, &position, &BlockUndo::default()),
        BasicFilterInputError::BlockIdentity
    );
}

#[test]
fn height_and_null_parent_must_agree() {
    // Arrange
    for (height, wrong_height) in [(0, 1), (1, 0)] {
        let (block, mut position) = coinbase_block(height);
        position.height = wrong_height;
        // Act / Assert
        assert_eq!(
            historical_error(&block, &position, &BlockUndo::default()),
            BasicFilterInputError::GenesisPosition
        );
    }
}

#[test]
fn empty_block_refuses_without_subtracting_transaction_count() {
    // Arrange
    let (mut block, mut position) = coinbase_block(1);
    block.transactions.clear();
    rebind_body(&mut block, &mut position);
    // Act / Assert
    assert_eq!(
        historical_error(&block, &position, &BlockUndo::default()),
        BasicFilterInputError::EmptyBlock
    );
}

#[test]
fn first_transaction_requires_coinbase_and_outputs() {
    // Arrange
    for remove_inputs in [false, true] {
        let (mut block, mut position) = coinbase_block(1);
        if remove_inputs {
            block.transactions[0].inputs.clear();
        } else {
            block.transactions[0].outputs.clear();
        }
        rebind_body(&mut block, &mut position);
        // Act / Assert
        assert_eq!(
            historical_error(&block, &position, &BlockUndo::default()),
            BasicFilterInputError::CoinbaseStructure
        );
    }
}

#[test]
fn non_coinbase_transactions_require_inputs_outputs_and_no_null_outpoints() {
    // Arrange
    for shape in 0..3 {
        let (mut block, mut position) = coinbase_block(1);
        let mut transaction = block.transactions[0].clone();
        match shape {
            0 => transaction.inputs.clear(),
            1 => transaction.outputs.clear(),
            _ => {}
        }
        block.transactions.push(transaction);
        rebind_body(&mut block, &mut position);
        // Act / Assert
        assert_eq!(
            historical_error(&block, &position, &BlockUndo::default()),
            BasicFilterInputError::TransactionStructure { index: 1 }
        );
    }
}

#[test]
fn genesis_refuses_spending_transactions() {
    // Arrange
    let (mut block, mut position) = coinbase_block(0);
    let mut transaction = block.transactions[0].clone();
    transaction.inputs[0].previous_output.vout = 0;
    block.transactions.push(transaction);
    rebind_body(&mut block, &mut position);
    // Act / Assert
    assert_eq!(
        historical_error(&block, &position, &BlockUndo::default()),
        BasicFilterInputError::GenesisSpends
    );
}

#[test]
fn undo_identity_refuses_other_block_history() {
    // Arrange
    let (block, position) = coinbase_block(1);
    let undo = BlockUndo::default();
    // Act
    let error = BasicFilterInputs::from_historical(
        &block,
        &position,
        Some(HistoricalBlockUndo {
            block_hash: BlockHash::default(),
            undo: &undo,
        }),
    )
    .expect_err("wrong undo tag");
    // Assert
    assert_eq!(error, BasicFilterInputError::UndoIdentity);
}

#[test]
fn genesis_refuses_nonempty_explicit_undo() {
    // Arrange
    let (block, position) = coinbase_block(0);
    let undo = BlockUndo {
        transactions: vec![TxUndo::default()],
    };
    // Act / Assert
    assert_eq!(
        historical_error(&block, &position, &undo),
        BasicFilterInputError::TransactionCount {
            expected: 0,
            actual: 1
        }
    );
}

#[test]
fn historical_undo_transaction_count_requires_exact_correspondence() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    for count in [1, 3] {
        let mut undo = staged.undo.clone();
        undo.transactions.resize(count, TxUndo::default());
        // Act / Assert
        assert_eq!(
            historical_error(&fixture.spending, &staged.position, &undo),
            BasicFilterInputError::TransactionCount {
                expected: 2,
                actual: count
            }
        );
    }
}

#[test]
fn historical_undo_input_counts_require_exact_correspondence() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    for (index, count) in [(0, 0), (1, 2)] {
        let mut undo = staged.undo.clone();
        let coin = undo.transactions[index].restored_inputs[0].clone();
        undo.transactions[index].restored_inputs.resize(count, coin);
        // Act / Assert
        assert_eq!(
            historical_error(&fixture.spending, &staged.position, &undo),
            BasicFilterInputError::InputCount {
                index: index + 1,
                expected: 1,
                actual: count
            }
        );
    }
}

#[test]
fn contextual_generation_refuses_missing_wrong_branch_and_wrong_height() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    let inputs = staged
        .basic_filter_inputs(&fixture.spending)
        .expect("complete facts");
    let parent = fixture.chainstate.tip().expect("funding tip");
    let cases = [
        (
            FilterHeaderPredecessor::Genesis,
            FilterHeaderError::GenesisHeight,
        ),
        (
            FilterHeaderPredecessor::Previous {
                block_hash: BlockHash::default(),
                height: 1,
                header: Default::default(),
            },
            FilterHeaderError::PreviousHash,
        ),
        (
            FilterHeaderPredecessor::Previous {
                block_hash: parent.block_hash,
                height: 0,
                header: Default::default(),
            },
            FilterHeaderError::PreviousHeight,
        ),
    ];
    for (predecessor, expected) in cases {
        // Act / Assert
        assert_eq!(
            inputs.generate(predecessor),
            Err(BasicFilterGenerationError::Header(expected))
        );
    }
}

#[test]
fn input_refusals_have_specific_display_and_error_contracts() {
    // Arrange
    let cases = [
        (
            BasicFilterInputError::BlockIdentity,
            "block header/hash does not match validated position",
        ),
        (
            BasicFilterInputError::MutatedBody,
            "block body contains a mutated transaction merkle tree",
        ),
        (
            BasicFilterInputError::BodyMerkleMismatch,
            "block body merkle root does not match validated header",
        ),
        (
            BasicFilterInputError::GenesisPosition,
            "height zero and null genesis parent must correspond",
        ),
        (
            BasicFilterInputError::EmptyBlock,
            "historical block has no coinbase transaction",
        ),
        (
            BasicFilterInputError::CoinbaseStructure,
            "first transaction must be a coinbase with outputs",
        ),
        (
            BasicFilterInputError::TransactionStructure { index: 2 },
            "transaction 2 has invalid non-coinbase structure",
        ),
        (
            BasicFilterInputError::GenesisSpends,
            "genesis cannot contain spent inputs",
        ),
        (
            BasicFilterInputError::MissingUndo,
            "non-genesis block requires explicit complete undo",
        ),
        (
            BasicFilterInputError::UndoIdentity,
            "historical undo identity does not match block",
        ),
        (
            BasicFilterInputError::TransactionCount {
                expected: 2,
                actual: 1,
            },
            "undo transaction count 1 does not match 2",
        ),
        (
            BasicFilterInputError::InputCount {
                index: 2,
                expected: 1,
                actual: 0,
            },
            "transaction 2 undo input count 0 does not match 1",
        ),
    ];
    for (error, expected) in cases {
        // Act / Assert
        assert_eq!(error.to_string(), expected);
        assert!(std::error::Error::source(&error).is_none());
    }
}

#[test]
fn generation_errors_preserve_typed_sources_and_display() {
    // Arrange
    let errors = [
        BasicFilterGenerationError::Encoding(BasicFilterEncodingError::OutputTooLarge),
        BasicFilterGenerationError::Header(FilterHeaderError::PreviousHash),
    ];
    for error in errors {
        // Act
        let source = std::error::Error::source(&error).expect("typed underlying error");
        // Assert
        assert_eq!(error.to_string(), source.to_string());
    }
}

mod validated;
