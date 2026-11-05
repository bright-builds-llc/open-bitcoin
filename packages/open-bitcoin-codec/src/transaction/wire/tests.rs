// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/transaction.h
// - packages/bitcoin-knots/src/serialize.h
// - packages/bitcoin-knots/src/core_read.cpp

use super::*;
use crate::block::wire::parse_submission_block_prefix;
use crate::compact_size::{MAX_SIZE, write_compact_size};
use crate::{TransactionEncoding, encode_transaction, parse_transaction};
use open_bitcoin_primitives::{MAX_MONEY, OutPoint, ScriptWitness, Txid};
use std::cell::{Cell, RefCell};

thread_local! {
    static CAPACITIES: RefCell<Vec<(&'static str, usize)>> = const { RefCell::new(Vec::new()) };
    static FAIL_NEXT_RESERVE: Cell<Option<&'static str>> = const { Cell::new(None) };
}
pub(crate) fn observe(field: &'static str, capacity: usize) {
    CAPACITIES.with(|values| values.borrow_mut().push((field, capacity)));
}
pub(crate) fn reserve_growth(field: &'static str) -> usize {
    FAIL_NEXT_RESERVE.with(|next| {
        if next.get() == Some(field) {
            next.set(None);
            usize::MAX
        } else {
            1
        }
    })
}

fn transaction_bytes(value: i64, script: &[u8]) -> Vec<u8> {
    let mut bytes = vec![1, 0, 0, 0, 1];
    bytes.extend_from_slice(&[2; 32]);
    bytes.extend_from_slice(&[0, 0, 0, 0, 0, 255, 255, 255, 255, 1]);
    bytes.extend_from_slice(&value.to_le_bytes());
    write_compact_size(&mut bytes, script.len() as u64).expect("script length");
    bytes.extend_from_slice(script);
    bytes.extend_from_slice(&[0; 4]);
    bytes
}
fn block_bytes(transaction: &[u8]) -> Vec<u8> {
    let mut bytes = vec![0; 80];
    bytes.push(1);
    bytes.extend_from_slice(transaction);
    bytes
}

#[test]
fn input_reserve_failure_is_real_one_shot_and_never_exposes_partial_transaction() {
    // Arrange
    let bytes = transaction_bytes(42, &[0x51]);
    CAPACITIES.with(|values| values.borrow_mut().clear());
    FAIL_NEXT_RESERVE.with(|next| next.set(Some("inputs")));

    // Act
    let error = parse_transaction(&bytes).expect_err("genuine Vec reserve overflow");
    let consumed = FAIL_NEXT_RESERVE.with(|next| next.get());
    let observed = CAPACITIES.with(|values| values.borrow().clone());
    let retry = parse_transaction(&bytes).expect("one-shot fault consumed");

    // Assert
    assert_eq!(
        error,
        CodecError::LengthOutOfRange {
            field: "inputs",
            value: 1,
        }
    );
    assert_eq!(consumed, None);
    assert!(
        observed
            .iter()
            .filter(|(field, _)| *field == "inputs")
            .all(|(_, capacity)| *capacity == 0)
    );
    assert_eq!(retry.inputs.len(), 1);
    assert_eq!(retry.outputs.len(), 1);
}

#[test]
fn phase167_submission_wire_codec_signed_complete_and_fallible() {
    // Arrange
    for value in [i64::MIN, -1, MAX_MONEY + 1, i64::MAX] {
        let bytes = block_bytes(&transaction_bytes(value, &[0x51]));
        // Act
        let (wire, consumed) = parse_submission_block_prefix(&bytes).expect("complete signed wire");
        // Assert
        assert_eq!(consumed, bytes.len());
        assert_eq!(wire.transactions[0].outputs[0].value_sats, value);
        assert!(wire.try_into_block().is_err());
    }
}

#[test]
fn phase167_submission_wire_codec_wide_serialized_record() {
    // Arrange
    let bytes = transaction_bytes(42, &vec![0x51; 10001]);
    // Act
    let domain = parse_serialized_transaction(&bytes).expect("raw storage script");
    // Assert
    assert_eq!(
        encode_transaction(&domain, TransactionEncoding::WithoutWitness).expect("encode"),
        bytes
    );
    assert_eq!(
        parse_transaction(&bytes)
            .expect_err("legacy script guard")
            .to_string(),
        "script too large: 10001"
    );
}

#[test]
fn phase167_submission_wire_codec_late_eof_before_invalid_amount() {
    // Arrange
    let mut transaction = transaction_bytes(-1, &[0x51]);
    transaction.pop();
    // Act
    let error = parse_submission_block_prefix(&block_bytes(&transaction)).expect_err("late EOF");
    // Assert
    assert_eq!(
        error,
        CodecError::UnexpectedEof {
            needed: 4,
            remaining: 3
        }
    );
    assert_eq!(
        parse_transaction(&transaction)
            .expect_err("old amount first")
            .to_string(),
        "amount out of range: -1"
    );
}

#[test]
fn phase167_submission_wire_codec_capacity_before_same_eof() {
    // Arrange
    for count in [4096_u64, MAX_SIZE] {
        let mut inputs = vec![1, 0, 0, 0];
        write_compact_size(&mut inputs, count).expect("count");
        CAPACITIES.with(|values| values.borrow_mut().clear());
        // Act
        let error = parse_transaction(&inputs).expect_err("no first input");
        let capacities = CAPACITIES.with(|values| values.borrow().clone());
        // Assert
        eprintln!(
            "capacity observation: declared_count={count}, input_bytes={}, events={capacities:?}, error={error:?}",
            inputs.len()
        );
        assert_eq!(
            error,
            CodecError::UnexpectedEof {
                needed: 32,
                remaining: 0
            }
        );
        assert!(
            capacities
                .iter()
                .all(|(_, capacity)| *capacity < count as usize)
        );
    }
}

#[test]
fn phase167_submission_wire_codec_all_container_growth_tracks_decoded_elements() {
    // Arrange
    for count in [4096_u64, MAX_SIZE] {
        let mut count_bytes = Vec::new();
        write_compact_size(&mut count_bytes, count).expect("count");
        let transaction = transaction_bytes(42, &[]);
        let mut input_bytes = vec![1, 0, 0, 0];
        input_bytes.extend_from_slice(&count_bytes);
        input_bytes.extend_from_slice(&transaction[5..46]);
        let mut output_bytes = transaction[..46].to_vec();
        output_bytes.extend_from_slice(&count_bytes);
        output_bytes.extend_from_slice(&transaction[47..56]);
        let mut witness_bytes = vec![1, 0, 0, 0, 0, 1];
        witness_bytes.extend_from_slice(&transaction[4..56]);
        witness_bytes.extend_from_slice(&count_bytes);
        witness_bytes.push(0);
        let mut block = vec![0; 80];
        block.extend_from_slice(&count_bytes);
        block.extend_from_slice(&transaction);
        for (field, bytes, is_block) in [
            ("inputs", input_bytes, false),
            ("outputs", output_bytes, false),
            ("witness", witness_bytes, false),
            ("transactions", block, true),
        ] {
            CAPACITIES.with(|values| values.borrow_mut().clear());
            // Act
            let error = if is_block {
                parse_submission_block_prefix(&bytes)
                    .map(|_| ())
                    .expect_err("second block transaction missing")
            } else {
                parse_transaction(&bytes)
                    .map(|_| ())
                    .expect_err("second container element missing")
            };
            let capacities = CAPACITIES.with(|values| {
                values
                    .borrow()
                    .iter()
                    .filter(|(name, _)| *name == field)
                    .copied()
                    .collect::<Vec<_>>()
            });
            // Assert
            eprintln!(
                "capacity observation: field={field}, declared_count={count}, input_bytes={}, events={capacities:?}, error={error:?}",
                bytes.len()
            );
            assert!(matches!(error, CodecError::UnexpectedEof { .. }));
            assert_eq!(capacities.len(), 2, "empty and first decoded element");
            assert_eq!(capacities[0].1, 0);
            assert!(capacities[1].1 > 0 && capacities[1].1 < count as usize);
        }
    }
}

#[test]
fn phase167_submission_wire_codec_exact_signed_wide_and_witness_roundtrips() {
    // Arrange
    for value in [i64::MIN, -1, 0, MAX_MONEY, MAX_MONEY + 1, i64::MAX] {
        for witness in [false, true] {
            let mut bytes = transaction_bytes(value, &vec![0x51; 10001]);
            if witness {
                bytes.splice(4..4, [0, 1]);
                let end = bytes.len() - 4;
                bytes.splice(end..end, [1, 2, 7, 8]);
            }
            let mut reader = Reader::new(&bytes);
            // Act
            let wire = super::super::parse_transaction_mode::<super::super::SubmissionWire>(
                &mut reader,
                true,
            )
            .expect("lossless raw transaction");
            // Assert
            reader.finish().expect("complete record");
            assert_eq!(
                super::super::encode_transaction_view(&wire, TransactionEncoding::WithWitness)
                    .expect("encode"),
                bytes
            );
            assert_eq!(wire.outputs[0].value_sats, value);
            assert_eq!(wire.outputs[0].script_pubkey.as_bytes().len(), 10001);
        }
    }
}

#[test]
fn phase167_submission_wire_codec_old_combined_defect_order_is_exact() {
    // Arrange
    let mut bytes = transaction_bytes(-1, &vec![0x51; 10001]);
    bytes.pop();
    // Act
    let old = parse_transaction(&bytes).expect_err("immediate checked amount");
    let raw = parse_submission_block_prefix(&block_bytes(&bytes)).expect_err("raw full decode");
    // Assert
    assert_eq!(old.to_string(), "amount out of range: -1");
    assert_eq!(
        raw,
        CodecError::UnexpectedEof {
            needed: 4,
            remaining: 3
        }
    );
    let mut wide = transaction_bytes(42, &vec![0x51; 10001]);
    wide.pop();
    assert_eq!(
        parse_transaction(&wide)
            .expect_err("old script before locktime")
            .to_string(),
        "script too large: 10001"
    );
    assert_eq!(
        parse_serialized_transaction(&wide).expect_err("strict raw script then locktime"),
        CodecError::UnexpectedEof {
            needed: 4,
            remaining: 3
        }
    );
}

#[test]
fn phase167_submission_wire_codec_flags_and_witness_fail_after_raw_amount() {
    // Arrange
    let transaction = transaction_bytes(-1, &[]);
    let mut unknown = vec![1, 0, 0, 0, 0, 2];
    unknown.extend_from_slice(&transaction[4..]);
    let mut empty = unknown.clone();
    empty[5] = 1;
    empty.insert(empty.len() - 4, 0);
    let mut late = unknown.clone();
    late[5] = 1;
    late.splice(late.len() - 4..late.len() - 4, [1, 2, 7]);
    // Act / Assert
    assert_eq!(
        parse_submission_block_prefix(&block_bytes(&unknown)).expect_err("unknown flag"),
        CodecError::InvalidWitnessFlag(2)
    );
    assert_eq!(
        parse_submission_block_prefix(&block_bytes(&empty)).expect_err("empty witness"),
        CodecError::SuperfluousWitnessRecord
    );
    assert!(matches!(
        parse_submission_block_prefix(&block_bytes(&late)),
        Err(CodecError::UnexpectedEof { .. })
    ));
    assert_eq!(
        parse_transaction(&unknown)
            .expect_err("legacy amount first")
            .to_string(),
        "amount out of range: -1"
    );
}

#[test]
fn phase167_submission_wire_codec_existing_witness_and_suffix_controls() {
    // Arrange
    let transaction = Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([3; 32]),
                vout: 1,
            },
            script_sig: ScriptBuf::default(),
            sequence: 3,
            witness: ScriptWitness::new(vec![vec![7; 32]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(42).expect("amount"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: 9,
    };
    let bytes = encode_transaction(&transaction, TransactionEncoding::WithWitness).expect("encode");
    let mut block = block_bytes(&bytes);
    let length = block.len();
    block.extend_from_slice(&[7, 8]);
    // Act
    let (wire, consumed) = parse_submission_block_prefix(&block).expect("prefix");
    // Assert
    assert_eq!(consumed, length);
    assert_eq!(
        wire.try_into_block().expect("domain").transactions[0],
        transaction
    );
    assert!(crate::parse_block(&block).is_err());
    assert!(super::parse_serialized_transaction(&[bytes, vec![1]].concat()).is_err());
    assert_eq!(MAX_SIZE, 0x0200_0000);
}
