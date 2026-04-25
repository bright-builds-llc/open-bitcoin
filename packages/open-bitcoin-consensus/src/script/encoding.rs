// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

use open_bitcoin_primitives::{MAX_SCRIPT_ELEMENT_SIZE, ScriptBuf, ScriptWitness};

use super::{
    ScriptError,
    opcodes::{OP_PUSHDATA1, OP_PUSHDATA2, OP_PUSHDATA4},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Instruction {
    pub(super) opcode: u8,
    pub(super) maybe_data: Option<Vec<u8>>,
}

pub(super) fn serialized_witness_size(witness: &ScriptWitness) -> usize {
    let mut size = compact_size_len(witness.stack().len() as u64);
    for item in witness.stack() {
        size += compact_size_len(item.len() as u64);
        size += item.len();
    }
    size
}

pub(super) fn compact_size_len(value: u64) -> usize {
    match value {
        0..=252 => 1,
        253..=0xffff => 3,
        0x1_0000..=0xffff_ffff => 5,
        _ => 9,
    }
}

pub(super) fn write_compact_size(out: &mut Vec<u8>, value: u64) {
    match value {
        0..=252 => out.push(value as u8),
        253..=0xffff => {
            out.push(0xfd);
            out.extend_from_slice(&(value as u16).to_le_bytes());
        }
        0x1_0000..=0xffff_ffff => {
            out.push(0xfe);
            out.extend_from_slice(&(value as u32).to_le_bytes());
        }
        _ => {
            out.push(0xff);
            out.extend_from_slice(&value.to_le_bytes());
        }
    }
}

pub(super) fn remove_signature_from_script(script: &ScriptBuf, signature: &[u8]) -> ScriptBuf {
    if signature.is_empty() {
        return script.clone();
    }

    let encoded_signature = encode_push_data(signature);
    let mut remaining = Vec::with_capacity(script.as_bytes().len());
    let mut offset = 0;
    while offset < script.as_bytes().len() {
        if script.as_bytes()[offset..].starts_with(&encoded_signature) {
            offset += encoded_signature.len();
            continue;
        }

        remaining.push(script.as_bytes()[offset]);
        offset += 1;
    }

    ScriptBuf::from_bytes(remaining).unwrap_or_else(|_| script.clone())
}

pub(super) fn encode_push_data(data: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(data.len() + 5);
    match data.len() {
        0..=0x4b => encoded.push(data.len() as u8),
        0x4c..=0xff => {
            encoded.push(OP_PUSHDATA1);
            encoded.push(data.len() as u8);
        }
        0x100..=0xffff => {
            encoded.push(OP_PUSHDATA2);
            encoded.extend_from_slice(&(data.len() as u16).to_le_bytes());
        }
        _ => {
            encoded.push(OP_PUSHDATA4);
            encoded.extend_from_slice(&(data.len() as u32).to_le_bytes());
        }
    }
    encoded.extend_from_slice(data);
    encoded
}

pub(super) fn read_instruction(bytes: &[u8], pc: &mut usize) -> Result<Instruction, ScriptError> {
    let opcode = *bytes.get(*pc).ok_or(ScriptError::BadOpcode)?;
    *pc += 1;

    if opcode <= 0x4b {
        let data = read_push_data(bytes, pc, opcode as usize)?;
        return Ok(Instruction {
            opcode,
            maybe_data: Some(data),
        });
    }

    let maybe_data = match opcode {
        OP_PUSHDATA1 => {
            let length = usize::from(*bytes.get(*pc).ok_or(ScriptError::TruncatedPushData)?);
            *pc += 1;
            Some(read_push_data(bytes, pc, length)?)
        }
        OP_PUSHDATA2 => {
            let length_bytes = bytes
                .get(*pc..(*pc + 2))
                .ok_or(ScriptError::TruncatedPushData)?;
            *pc += 2;
            let length = usize::from(u16::from_le_bytes([length_bytes[0], length_bytes[1]]));
            Some(read_push_data(bytes, pc, length)?)
        }
        OP_PUSHDATA4 => {
            let length_bytes = bytes
                .get(*pc..(*pc + 4))
                .ok_or(ScriptError::TruncatedPushData)?;
            *pc += 4;
            let length = u32::from_le_bytes([
                length_bytes[0],
                length_bytes[1],
                length_bytes[2],
                length_bytes[3],
            ]) as usize;
            Some(read_push_data(bytes, pc, length)?)
        }
        _ => None,
    };

    Ok(Instruction { opcode, maybe_data })
}

fn read_push_data(bytes: &[u8], pc: &mut usize, length: usize) -> Result<Vec<u8>, ScriptError> {
    let end = pc
        .checked_add(length)
        .ok_or(ScriptError::TruncatedPushData)?;
    let data = bytes.get(*pc..end).ok_or(ScriptError::TruncatedPushData)?;
    *pc = end;
    if data.len() > MAX_SCRIPT_ELEMENT_SIZE {
        return Err(ScriptError::PushSize(data.len()));
    }
    Ok(data.to_vec())
}
