// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/undo.h
// - packages/bitcoin-knots/src/validation.cpp

//! Synthetic allocation/structure evidence only; no canonical or historical authority.

use super::*;
use open_bitcoin_core::{
    chainstate::{
        Coin, TxUndo,
        types::{BlockOverwriteUndo, OverwrittenCoin},
    },
    consensus::{BasicFilter, FilterHeaderPredecessor, block_merkle_root, transaction_txid},
    primitives::{Amount, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Txid},
};
use std::mem::size_of;

fn coin(script: &[u8]) -> Coin {
    Coin {
        output: TransactionOutput {
            value: Amount::from_sats(42).expect("bounded synthetic amount"),
            script_pubkey: ScriptBuf::from_bytes(script.to_vec()).expect("small script"),
        },
        is_coinbase: true,
        created_height: 1,
        created_median_time_past: 2,
    }
}

fn batch(overwritten: Vec<OverwrittenCoin>) -> BlockOverwriteUndo {
    BlockOverwriteUndo {
        block_hash: BlockHash::from_byte_array([1; 32]),
        height: 3,
        creating_median_time_past: 4,
        overwritten,
        // Accounting preserves even untrusted metadata; it cannot seal this batch.
        integrity: [7; 32],
    }
}

fn entry(previous: &[u8], new: &[u8]) -> OverwrittenCoin {
    OverwrittenCoin {
        transaction_index: 0,
        outpoint: OutPoint {
            txid: Txid::from_byte_array([2; 32]),
            vout: 0,
        },
        previous_coin: coin(previous),
        new_coin: coin(new),
    }
}

fn complete_undo() -> BlockUndo {
    BlockUndo {
        transactions: vec![TxUndo {
            restored_inputs: vec![coin(&[0x51; 5])],
        }],
        overwrites: vec![batch(vec![
            entry(&[0x52; 2], &[0x53; 4]),
            entry(&[0x54; 3], &[0x54; 3]),
        ])],
    }
}

// Independent layout formula: headers/inline Coins already belong to their containers.
fn complete_undo_bytes() -> u64 {
    (size_of::<BlockUndo>()
        + size_of::<TxUndo>()
        + size_of::<Coin>()
        + 5
        + size_of::<BlockOverwriteUndo>()
        + 2 * size_of::<OverwrittenCoin>()
        + 2
        + 4
        + 3
        + 3) as u64
}

#[test]
fn phase167_undo_budget_owned_counts_complete_allocations_once() {
    // Arrange
    let undo = complete_undo();
    let block = Block::default();
    let expected_undo = complete_undo_bytes();
    // Act
    let work = fact_work(&block, &undo).expect("small complete undo");
    // Assert
    assert_eq!(work.undo_bytes, expected_undo);
    assert_eq!(work.body_bytes, size_of::<Block>() as u64);
    assert_eq!(
        work.cloned_bytes,
        size_of::<Block>() as u64 + expected_undo + size_of::<ChainPosition>() as u64
    );
    assert_eq!((work.script_items, work.script_bytes), (1, 5));
}

#[test]
fn phase167_undo_budget_borrowed_counts_complete_allocations_without_clones() {
    // Arrange
    let undo = complete_undo();
    // Act
    let work = borrowed_undo_work(Some(&undo)).expect("small borrowed undo");
    // Assert
    assert_eq!(work.undo_bytes, complete_undo_bytes());
    assert_eq!(work.cloned_bytes, 0);
    assert_eq!((work.script_items, work.script_bytes), (1, 5));
}

#[test]
fn phase167_undo_budget_empty_capture_batch_has_exact_allocation_delta() {
    // Arrange
    let unknown = BlockUndo::default();
    let captured = BlockUndo {
        overwrites: vec![batch(Vec::new())],
        ..BlockUndo::default()
    };
    // Act
    let unknown_work = borrowed_undo_work(Some(&unknown)).expect("unknown undo");
    let captured_work = borrowed_undo_work(Some(&captured)).expect("captured empty scan");
    // Assert
    assert_eq!(unknown_work.undo_bytes, size_of::<BlockUndo>() as u64);
    assert_eq!(
        captured_work.undo_bytes - unknown_work.undo_bytes,
        size_of::<BlockOverwriteUndo>() as u64
    );
    assert!(unknown.overwrites.is_empty());
    assert_eq!(captured.overwrites.len(), 1);
    assert_eq!(
        borrowed_undo_work(None).expect("absent undo"),
        TurnWork::default()
    );
}

#[test]
fn phase167_undo_budget_entries_and_both_owned_scripts_have_exact_deltas() {
    // Arrange
    let empty = BlockUndo {
        overwrites: vec![batch(Vec::new())],
        ..BlockUndo::default()
    };
    let mut entries = empty.clone();
    entries.overwrites[0].overwritten = vec![entry(&[], &[]), entry(&[], &[])];
    let mut scripts = entries.clone();
    scripts.overwrites[0].overwritten = complete_undo().overwrites.remove(0).overwritten;
    // Act
    let empty_work = borrowed_undo_work(Some(&empty)).expect("empty batch");
    let entry_work = borrowed_undo_work(Some(&entries)).expect("two entries");
    let script_work = borrowed_undo_work(Some(&scripts)).expect("both scripts");
    // Assert
    assert_eq!(
        entry_work.undo_bytes - empty_work.undo_bytes,
        (2 * size_of::<OverwrittenCoin>()) as u64
    );
    assert_eq!(
        script_work.undo_bytes - entry_work.undo_bytes,
        2 + 4 + 3 + 3
    );
    assert_eq!((script_work.script_items, script_work.script_bytes), (0, 0));
}

#[test]
fn phase167_undo_budget_both_scripts_are_charged_even_with_equal_contents() {
    // Arrange
    let bare = BlockUndo {
        overwrites: vec![batch(vec![entry(&[], &[]), entry(&[], &[])])],
        ..BlockUndo::default()
    };
    let mut scripted = bare.clone();
    scripted.overwrites[0].overwritten = complete_undo().overwrites.remove(0).overwritten;
    // Act
    let before = borrowed_undo_work(Some(&bare)).expect("bare entries");
    let after = borrowed_undo_work(Some(&scripted)).expect("scripted entries");
    // Assert
    assert_eq!(after.undo_bytes - before.undo_bytes, 2 + 4 + 3 + 3);
    assert_eq!((after.script_items, after.script_bytes), (0, 0));
}

#[test]
fn phase167_undo_budget_exact_ceiling_accepts_small_complete_undo() {
    // Arrange
    let undo = complete_undo();
    let mut work = TurnWork {
        undo_bytes: MAX_ACCEPTED_FACT_BYTES - complete_undo_bytes(),
        ..TurnWork::default()
    };
    // Act
    let result = add_undo_work(&mut work, &undo);
    // Assert
    assert_eq!(result, Ok(()));
    assert_eq!(work.undo_bytes, MAX_ACCEPTED_FACT_BYTES);
    assert_eq!((work.script_items, work.script_bytes), (1, 5));
    assert_eq!(work.cloned_bytes, 0);
}

#[test]
fn phase167_undo_budget_one_over_ceiling_refuses_small_complete_undo() {
    // Arrange
    let undo = complete_undo();
    let mut work = TurnWork {
        undo_bytes: MAX_ACCEPTED_FACT_BYTES - complete_undo_bytes() + 1,
        ..TurnWork::default()
    };
    // Act
    let result = add_undo_work(&mut work, &undo);
    // Assert
    assert_eq!(result, Err(AcceptedBasicIndexFailure::FactLimit));
    assert_eq!(work.undo_bytes, MAX_ACCEPTED_FACT_BYTES + 1);
    assert_eq!(work.cloned_bytes, 0);
}

#[test]
fn phase167_undo_budget_new_batch_entry_and_each_script_enforce_remaining_allowance() {
    // Arrange
    let header = size_of::<BlockUndo>();
    let batch_bytes = size_of::<BlockOverwriteUndo>();
    let entry_bytes = size_of::<OverwrittenCoin>();
    let cases = [
        (batch(Vec::new()), header),
        (batch(vec![entry(&[], &[])]), header + batch_bytes),
        (
            batch(vec![entry(&[0x51; 2], &[])]),
            header + batch_bytes + entry_bytes,
        ),
        (
            batch(vec![entry(&[0x51; 2], &[0x52; 3])]),
            header + batch_bytes + entry_bytes + 2,
        ),
    ];
    for (batch, allowance) in cases {
        let undo = BlockUndo {
            overwrites: vec![batch],
            ..BlockUndo::default()
        };
        let mut work = TurnWork {
            undo_bytes: MAX_ACCEPTED_FACT_BYTES - allowance as u64,
            ..TurnWork::default()
        };
        // Act
        let result = add_undo_work(&mut work, &undo);
        // Assert
        assert_eq!(result, Err(AcceptedBasicIndexFailure::FactLimit));
        assert!(work.undo_bytes > MAX_ACCEPTED_FACT_BYTES);
        assert_eq!(
            (work.script_items, work.script_bytes, work.cloned_bytes),
            (0, 0, 0)
        );
    }
}

#[test]
fn phase167_undo_budget_checked_overflow_refuses_without_advancing_counter() {
    // Arrange
    let mut work = TurnWork {
        undo_bytes: u64::MAX,
        ..TurnWork::default()
    };
    // Act
    let result = add_undo_work(&mut work, &complete_undo());
    // Assert
    assert_eq!(result, Err(AcceptedBasicIndexFailure::FactLimit));
    assert_eq!(work.undo_bytes, u64::MAX);
    assert_eq!((work.script_items, work.script_bytes), (0, 0));
}

#[test]
fn phase167_undo_budget_logical_lengths_ignore_spare_vector_capacity() {
    // Arrange
    let undo = complete_undo();
    let mut spare = undo.clone();
    spare.transactions.reserve(8);
    spare.transactions[0].restored_inputs.reserve(8);
    spare.overwrites.reserve(8);
    spare.overwrites[0].overwritten.reserve(8);
    // Act
    let work = fact_work(&Block::default(), &spare).expect("small spare capacity");
    let borrowed = borrowed_undo_work(Some(&spare)).expect("borrowed capacity");
    // Assert
    assert_eq!(spare, undo);
    assert_eq!(work.undo_bytes, complete_undo_bytes());
    assert_eq!(borrowed.undo_bytes, work.undo_bytes);
    assert_eq!(borrowed.cloned_bytes, 0);
}

fn structural_capture_fixture() -> (Block, ChainPosition, BlockUndo) {
    let mut undo = complete_undo();
    let coinbase = Transaction {
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![coin(&[0x53; 4]).output, coin(&[0x54; 3]).output],
        ..Transaction::default()
    };
    let txid = transaction_txid(&coinbase).expect("structural coinbase txid");
    for (index, entry) in undo.overwrites[0].overwritten.iter_mut().enumerate() {
        entry.outpoint = OutPoint {
            txid,
            vout: index as u32,
        };
        entry.new_coin.created_height = 3;
        entry.new_coin.created_median_time_past = 4;
    }
    let spend = Transaction {
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([9; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::from_bytes(vec![0x51; 2]).expect("small input script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x42; 10]]),
        }],
        outputs: vec![coin(&[0x55; 6]).output],
        ..Transaction::default()
    };
    let mut block = Block {
        header: BlockHeader {
            previous_block_hash: BlockHash::from_byte_array([1; 32]),
            ..BlockHeader::default()
        },
        transactions: vec![coinbase, spend],
    };
    block.header.merkle_root = block_merkle_root(&block.transactions)
        .expect("bound synthetic body")
        .0;
    let position = ChainPosition::new(
        block.header.clone(),
        3,
        ChainPosition::untrusted_fixture_work(1),
        4,
    );
    undo.overwrites[0].block_hash = position.block_hash;
    (block, position, undo)
}

#[test]
fn phase167_undo_budget_structural_capture_preserves_complete_undo_and_basic_contents() {
    // Arrange: same-shape resource fixtures do not prove prior Coins or history.
    let (block, position, complete) = structural_capture_fixture();
    let mut captured_empty = complete.clone();
    captured_empty.overwrites[0].overwritten.clear();
    let mut unknown = complete.clone();
    unknown.overwrites.clear();
    let expected_filter = BasicFilter::from_script_facts(
        position.block_hash,
        &[&[0x53; 4], &[0x54; 3], &[0x55; 6]],
        &[&[0x51; 5]],
    )
    .expect("independent BASIC script selection");
    let mut expected_body = block.clone();
    expected_body.transactions[1].inputs[0].witness = ScriptWitness::default();
    let body_bytes = (size_of::<Block>()
        + 2 * size_of::<Transaction>()
        + 2 * size_of::<TransactionInput>()
        + 3 * size_of::<TransactionOutput>()
        + 2
        + 4
        + 3
        + 6) as u64;
    let ordinary_bytes =
        (size_of::<BlockUndo>() + size_of::<TxUndo>() + size_of::<Coin>() + 5) as u64;
    for (undo, expected_undo) in [
        (&complete, complete_undo_bytes()),
        (
            &captured_empty,
            ordinary_bytes + size_of::<BlockOverwriteUndo>() as u64,
        ),
        (&unknown, ordinary_bytes),
    ] {
        // Act
        let facts = AcceptedBasicFilterFacts::capture_reorg(&block, &position, undo)
            .expect("small consistent structural capture");
        let borrowed = borrowed_undo_work(Some(undo)).expect("same borrowed undo");
        let inputs = facts
            .inputs()
            .expect("same historical script correspondence");
        let (filter, _) = inputs
            .generate(FilterHeaderPredecessor::Previous {
                block_hash: position.previous_block_hash(),
                height: 2,
                header: Default::default(),
            })
            .expect("synthetic predecessor structure");
        // Assert
        assert_eq!(&facts.undo, undo);
        assert_eq!(facts.position, position);
        assert_eq!(facts.block, expected_body);
        assert_eq!(facts.work.undo_bytes, expected_undo);
        assert_eq!(facts.work.body_bytes, body_bytes);
        assert_eq!(
            facts.work.cloned_bytes,
            body_bytes + expected_undo + size_of::<ChainPosition>() as u64
        );
        assert_eq!((facts.work.script_items, facts.work.script_bytes), (4, 18));
        assert_eq!(borrowed.undo_bytes, expected_undo);
        assert_eq!(
            (
                borrowed.script_items,
                borrowed.script_bytes,
                borrowed.cloned_bytes
            ),
            (1, 5, 0)
        );
        assert_eq!(
            inputs.spent_scripts().collect::<Vec<_>>(),
            vec![&[0x51; 5][..]]
        );
        assert_eq!(filter.encoded_bytes(), expected_filter.encoded_bytes());
    }
    assert!(unknown.overwrites.is_empty());
    assert_eq!(captured_empty.overwrites.len(), 1);
    assert!(captured_empty.overwrites[0].overwritten.is_empty());
    assert_eq!(complete.overwrites[0].overwritten.len(), 2);
    assert!(!block.transactions[1].inputs[0].witness.is_empty());
}
