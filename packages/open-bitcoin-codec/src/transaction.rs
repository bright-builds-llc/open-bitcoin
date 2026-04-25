// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/transaction.h
// - packages/bitcoin-knots/src/primitives/transaction.cpp
// - packages/bitcoin-knots/src/serialize.h
// - packages/bitcoin-knots/src/streams.h

use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    Txid,
};

use crate::compact_size::{compact_size_to_usize, read_compact_size, write_compact_size};
use crate::error::CodecError;
use crate::primitives::{Reader, write_i32_le, write_i64_le, write_u32_le};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionEncoding {
    WithWitness,
    WithoutWitness,
}

pub fn parse_transaction(bytes: &[u8]) -> Result<Transaction, CodecError> {
    let mut reader = Reader::new(bytes);
    let transaction = parse_transaction_from_reader(&mut reader, true)?;
    reader.finish()?;
    Ok(transaction)
}

pub fn parse_transaction_without_witness(bytes: &[u8]) -> Result<Transaction, CodecError> {
    let mut reader = Reader::new(bytes);
    let transaction = parse_transaction_from_reader(&mut reader, false)?;
    reader.finish()?;
    Ok(transaction)
}

pub(crate) fn parse_transaction_from_reader(
    reader: &mut Reader<'_>,
    allow_witness: bool,
) -> Result<Transaction, CodecError> {
    let version = reader.read_i32_le()?;
    let mut inputs = parse_inputs(reader)?;
    let outputs;
    let mut flags = 0_u8;

    if inputs.is_empty() && allow_witness {
        flags = reader.read_u8()?;
        if flags != 0 {
            inputs = parse_inputs(reader)?;
            outputs = parse_outputs(reader)?;
        } else {
            outputs = Vec::new();
        }
    } else {
        outputs = parse_outputs(reader)?;
    }

    if (flags & 1) != 0 && allow_witness {
        flags ^= 1;
        for input in &mut inputs {
            input.witness = parse_witness(reader)?;
        }
        if inputs.iter().all(|input| input.witness.is_empty()) {
            return Err(CodecError::SuperfluousWitnessRecord);
        }
    }

    if flags != 0 {
        return Err(CodecError::InvalidWitnessFlag(flags));
    }

    let lock_time = reader.read_u32_le()?;

    Ok(Transaction {
        version,
        inputs,
        outputs,
        lock_time,
    })
}

pub fn encode_transaction(
    transaction: &Transaction,
    encoding: TransactionEncoding,
) -> Result<Vec<u8>, CodecError> {
    let mut out = Vec::new();
    write_i32_le(&mut out, transaction.version);

    let include_witness = encoding == TransactionEncoding::WithWitness && transaction.has_witness();
    if include_witness {
        out.push(0x00);
        out.push(0x01);
    }

    encode_inputs(&mut out, &transaction.inputs)?;
    encode_outputs(&mut out, &transaction.outputs)?;

    if include_witness {
        for input in &transaction.inputs {
            encode_witness(&mut out, &input.witness)?;
        }
    }

    write_u32_le(&mut out, transaction.lock_time);
    Ok(out)
}

fn parse_inputs(reader: &mut Reader<'_>) -> Result<Vec<TransactionInput>, CodecError> {
    let count = compact_size_to_usize(read_compact_size(reader)?, "transaction input count");
    let mut inputs = Vec::with_capacity(count);
    for _ in 0..count {
        let previous_output = OutPoint {
            txid: Txid::from_byte_array(reader.read_array::<32>()?),
            vout: reader.read_u32_le()?,
        };
        let script_sig = parse_script(reader)?;
        let sequence = reader.read_u32_le()?;
        inputs.push(TransactionInput {
            previous_output,
            script_sig,
            sequence,
            witness: ScriptWitness::default(),
        });
    }
    Ok(inputs)
}

fn encode_inputs(out: &mut Vec<u8>, inputs: &[TransactionInput]) -> Result<(), CodecError> {
    write_compact_size(out, inputs.len() as u64)?;
    for input in inputs {
        out.extend_from_slice(input.previous_output.txid.as_bytes());
        write_u32_le(out, input.previous_output.vout);
        encode_script(out, &input.script_sig)?;
        write_u32_le(out, input.sequence);
    }
    Ok(())
}

fn parse_outputs(reader: &mut Reader<'_>) -> Result<Vec<TransactionOutput>, CodecError> {
    let count = compact_size_to_usize(read_compact_size(reader)?, "transaction output count");
    let mut outputs = Vec::with_capacity(count);
    for _ in 0..count {
        let value = Amount::from_sats(reader.read_i64_le()?)?;
        let script_pubkey = parse_script(reader)?;
        outputs.push(TransactionOutput {
            value,
            script_pubkey,
        });
    }
    Ok(outputs)
}

fn encode_outputs(out: &mut Vec<u8>, outputs: &[TransactionOutput]) -> Result<(), CodecError> {
    write_compact_size(out, outputs.len() as u64)?;
    for output in outputs {
        write_i64_le(out, output.value.to_sats());
        encode_script(out, &output.script_pubkey)?;
    }
    Ok(())
}

fn parse_script(reader: &mut Reader<'_>) -> Result<ScriptBuf, CodecError> {
    let len = compact_size_to_usize(read_compact_size(reader)?, "script length");
    Ok(ScriptBuf::from_bytes(reader.read_vec(len)?)?)
}

fn encode_script(out: &mut Vec<u8>, script: &ScriptBuf) -> Result<(), CodecError> {
    write_compact_size(out, script.as_bytes().len() as u64)?;
    out.extend_from_slice(script.as_bytes());
    Ok(())
}

fn parse_witness(reader: &mut Reader<'_>) -> Result<ScriptWitness, CodecError> {
    let count = compact_size_to_usize(read_compact_size(reader)?, "witness item count");
    let mut stack = Vec::with_capacity(count);
    for _ in 0..count {
        let len = compact_size_to_usize(read_compact_size(reader)?, "witness item length");
        stack.push(reader.read_vec(len)?);
    }
    Ok(ScriptWitness::new(stack))
}

fn encode_witness(out: &mut Vec<u8>, witness: &ScriptWitness) -> Result<(), CodecError> {
    write_compact_size(out, witness.stack().len() as u64)?;
    for item in witness.stack() {
        write_compact_size(out, item.len() as u64)?;
        out.extend_from_slice(item);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{Amount, ScriptBuf, ScriptWitness};

    use crate::test_support::decode_hex;

    use super::{
        OutPoint, Transaction, TransactionEncoding, TransactionInput, TransactionOutput, Txid,
        encode_transaction, parse_transaction, parse_transaction_without_witness,
    };

    const GENESIS_TRANSACTION_HEX: &str = include_str!("../testdata/transaction_valid.hex");

    #[test]
    fn transaction_round_trips_without_witness() {
        let transaction = Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([2_u8; 32]),
                    vout: 1,
                },
                script_sig: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(42).expect("valid amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51, 0xac]).expect("valid script"),
            }],
            lock_time: 0,
        };

        let encoded = encode_transaction(&transaction, TransactionEncoding::WithoutWitness)
            .expect("transaction should encode");
        let decoded = parse_transaction(&encoded).expect("transaction should decode");

        assert_eq!(decoded, transaction);
    }

    #[test]
    fn transaction_fixture_round_trips_byte_for_byte() {
        let bytes = decode_hex(GENESIS_TRANSACTION_HEX);
        let decoded = parse_transaction_without_witness(&bytes)
            .expect("fixture should decode without witness");
        let reencoded = encode_transaction(&decoded, TransactionEncoding::WithoutWitness)
            .expect("fixture should re-encode");

        assert_eq!(reencoded, bytes);
    }

    #[test]
    fn transaction_round_trips_with_witness() {
        let transaction = Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([2_u8; 32]),
                    vout: 1,
                },
                script_sig: ScriptBuf::from_bytes(vec![]).expect("valid script"),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::new(vec![vec![0x01, 0x02], vec![0x51]]),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(42).expect("valid amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51, 0xac]).expect("valid script"),
            }],
            lock_time: 0,
        };

        let encoded = encode_transaction(&transaction, TransactionEncoding::WithWitness)
            .expect("transaction should encode");
        let decoded = parse_transaction(&encoded).expect("transaction should decode");

        assert_eq!(decoded, transaction);
    }

    #[test]
    fn transaction_rejects_superfluous_witness_records() {
        let bytes = decode_hex(
            "0200000000010102020202020202020202020202020202020202020202020202020202020202020100000000ffffffff012a0000000000000001510000000000",
        );
        let error = parse_transaction(&bytes).expect_err("empty witness stacks are invalid");

        assert_eq!(error.to_string(), "superfluous witness record");
    }

    #[test]
    fn transaction_accepts_empty_vin_marker_without_witness_flag() {
        let bytes = decode_hex("01000000000000000000");
        let transaction = parse_transaction(&bytes).expect("empty transaction marker should parse");

        assert!(transaction.inputs.is_empty());
        assert!(transaction.outputs.is_empty());
        assert_eq!(transaction.lock_time, 0);
    }

    #[test]
    fn transaction_rejects_unknown_witness_flags() {
        let bytes = decode_hex("0100000000020000000000");
        let error = parse_transaction(&bytes).expect_err("unknown witness flags must fail");

        assert_eq!(error.to_string(), "invalid witness flag: 2");
    }
}
