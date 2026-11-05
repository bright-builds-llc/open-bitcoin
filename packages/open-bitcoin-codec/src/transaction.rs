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

pub mod wire;
use wire::{TransactionView, WireOutput, WireTransaction};

pub(crate) trait DecodeMode {
    const SERIALIZED_SCRIPT: bool;
    type Value;
    type Output;
    type Transaction;
    fn value(sats: i64) -> Result<Self::Value, CodecError>;
    fn output(value: Self::Value, script_pubkey: ScriptBuf) -> Self::Output;
    fn transaction(
        version: i32,
        inputs: Vec<TransactionInput>,
        outputs: Vec<Self::Output>,
        lock_time: u32,
    ) -> Self::Transaction;
}

pub(crate) struct DomainMode<const SERIALIZED: bool>;
impl<const SERIALIZED: bool> DecodeMode for DomainMode<SERIALIZED> {
    const SERIALIZED_SCRIPT: bool = SERIALIZED;
    type Value = Amount;
    type Output = TransactionOutput;
    type Transaction = Transaction;
    fn value(sats: i64) -> Result<Amount, CodecError> {
        Ok(Amount::from_sats(sats)?)
    }
    fn output(value: Amount, script_pubkey: ScriptBuf) -> TransactionOutput {
        TransactionOutput {
            value,
            script_pubkey,
        }
    }
    fn transaction(
        version: i32,
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>,
        lock_time: u32,
    ) -> Transaction {
        Transaction {
            version,
            inputs,
            outputs,
            lock_time,
        }
    }
}

pub(crate) struct SubmissionWire;
impl DecodeMode for SubmissionWire {
    const SERIALIZED_SCRIPT: bool = true;
    type Value = i64;
    type Output = WireOutput;
    type Transaction = WireTransaction;
    fn value(sats: i64) -> Result<i64, CodecError> {
        Ok(sats)
    }
    fn output(value_sats: i64, script_pubkey: ScriptBuf) -> WireOutput {
        WireOutput {
            value_sats,
            script_pubkey,
        }
    }
    fn transaction(
        version: i32,
        inputs: Vec<TransactionInput>,
        outputs: Vec<WireOutput>,
        lock_time: u32,
    ) -> WireTransaction {
        WireTransaction {
            version,
            inputs,
            outputs,
            lock_time,
        }
    }
}

pub(crate) fn push_decoded<T>(
    values: &mut Vec<T>,
    value: T,
    field: &'static str,
) -> Result<(), CodecError> {
    #[cfg(test)]
    let additional = wire::tests::reserve_growth(field);
    #[cfg(not(test))]
    let additional = 1;
    values
        .try_reserve(additional)
        .map_err(|_| CodecError::LengthOutOfRange {
            field,
            value: values.len() as u64 + 1,
        })?;
    values.push(value);
    #[cfg(test)]
    wire::tests::observe(field, values.capacity());
    Ok(())
}

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
    parse_transaction_mode::<DomainMode<false>>(reader, allow_witness)
}

pub(crate) fn parse_transaction_mode<M: DecodeMode>(
    reader: &mut Reader<'_>,
    allow_witness: bool,
) -> Result<M::Transaction, CodecError> {
    let version = reader.read_i32_le()?;
    let mut inputs = parse_inputs::<M>(reader)?;
    let outputs;
    let mut flags = 0_u8;

    if inputs.is_empty() && allow_witness {
        flags = reader.read_u8()?;
        if flags != 0 {
            inputs = parse_inputs::<M>(reader)?;
            outputs = parse_outputs::<M>(reader)?;
        } else {
            outputs = Vec::new();
        }
    } else {
        outputs = parse_outputs::<M>(reader)?;
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

    Ok(M::transaction(version, inputs, outputs, lock_time))
}

pub fn encode_transaction(
    transaction: &Transaction,
    encoding: TransactionEncoding,
) -> Result<Vec<u8>, CodecError> {
    encode_transaction_view(transaction, encoding)
}

/// Serialize exact borrowed transaction facts, without validating or normalizing them.
pub fn encode_transaction_view(
    transaction: &impl TransactionView,
    encoding: TransactionEncoding,
) -> Result<Vec<u8>, CodecError> {
    let mut out = Vec::new();
    write_i32_le(&mut out, transaction.version());

    let include_witness = encoding == TransactionEncoding::WithWitness
        && transaction
            .inputs()
            .iter()
            .any(|input| !input.witness.is_empty());
    if include_witness {
        out.push(0x00);
        out.push(0x01);
    }

    encode_inputs(&mut out, transaction.inputs())?;
    write_compact_size(&mut out, transaction.outputs().len() as u64)?;
    for output in transaction.outputs() {
        write_i64_le(&mut out, output.value_sats);
        encode_script(&mut out, output.script_pubkey)?;
    }

    if include_witness {
        for input in transaction.inputs() {
            encode_witness(&mut out, &input.witness)?;
        }
    }

    write_u32_le(&mut out, transaction.lock_time());
    Ok(out)
}

fn parse_inputs<M: DecodeMode>(
    reader: &mut Reader<'_>,
) -> Result<Vec<TransactionInput>, CodecError> {
    let count = compact_size_to_usize(read_compact_size(reader)?, "transaction input count");
    let mut inputs = Vec::new();
    #[cfg(test)]
    wire::tests::observe("inputs", inputs.capacity());
    for _ in 0..count {
        let previous_output = OutPoint {
            txid: Txid::from_byte_array(reader.read_array::<32>()?),
            vout: reader.read_u32_le()?,
        };
        let script_sig = parse_script::<M>(reader)?;
        let sequence = reader.read_u32_le()?;
        push_decoded(
            &mut inputs,
            TransactionInput {
                previous_output,
                script_sig,
                sequence,
                witness: ScriptWitness::default(),
            },
            "inputs",
        )?;
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

fn parse_outputs<M: DecodeMode>(reader: &mut Reader<'_>) -> Result<Vec<M::Output>, CodecError> {
    let count = compact_size_to_usize(read_compact_size(reader)?, "transaction output count");
    let mut outputs = Vec::new();
    #[cfg(test)]
    wire::tests::observe("outputs", outputs.capacity());
    for _ in 0..count {
        let value = M::value(reader.read_i64_le()?)?;
        let script_pubkey = parse_script::<M>(reader)?;
        push_decoded(&mut outputs, M::output(value, script_pubkey), "outputs")?;
    }
    Ok(outputs)
}

fn parse_script<M: DecodeMode>(reader: &mut Reader<'_>) -> Result<ScriptBuf, CodecError> {
    let len = compact_size_to_usize(read_compact_size(reader)?, "script length");
    let bytes = reader.read_vec(len)?;
    if M::SERIALIZED_SCRIPT {
        return Ok(ScriptBuf::from_serialized_bytes(bytes));
    }
    Ok(ScriptBuf::from_bytes(bytes)?)
}

fn encode_script(out: &mut Vec<u8>, script: &ScriptBuf) -> Result<(), CodecError> {
    write_compact_size(out, script.as_bytes().len() as u64)?;
    out.extend_from_slice(script.as_bytes());
    Ok(())
}

fn parse_witness(reader: &mut Reader<'_>) -> Result<ScriptWitness, CodecError> {
    let count = compact_size_to_usize(read_compact_size(reader)?, "witness item count");
    let mut stack = Vec::new();
    #[cfg(test)]
    wire::tests::observe("witness", stack.capacity());
    for _ in 0..count {
        let len = compact_size_to_usize(read_compact_size(reader)?, "witness item length");
        let item = reader.read_vec(len)?;
        push_decoded(&mut stack, item, "witness")?;
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
