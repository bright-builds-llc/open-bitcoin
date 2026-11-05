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
    let encoded_signature = encode_push_data(signature);
    let bytes = script.as_bytes();
    let mut remaining = Vec::with_capacity(bytes.len());
    let mut offset = 0;
    while offset < bytes.len() {
        if bytes[offset..].starts_with(&encoded_signature) {
            offset += encoded_signature.len();
            continue;
        }

        let instruction_start = offset;
        match read_instruction_syntax(bytes, &mut offset) {
            Ok(_) => remaining.extend_from_slice(&bytes[instruction_start..offset]),
            Err(_) => {
                // FindAndDelete stops at failed GetOp and retains its entire suffix.
                remaining.extend_from_slice(&bytes[instruction_start..]);
                break;
            }
        }
    }

    ScriptBuf::from_serialized_bytes(remaining)
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
    let instruction = read_instruction_syntax(bytes, pc)?;
    if let Some(data) = &instruction.maybe_data
        && data.len() > MAX_SCRIPT_ELEMENT_SIZE
    {
        return Err(ScriptError::PushSize(data.len()));
    }
    Ok(instruction)
}

/// Decode GetOp syntax without the interpreter's element-size limit.
/// Observational sigop counting stops on malformed syntax but can traverse large pushes.
pub(super) fn read_instruction_syntax(
    bytes: &[u8],
    pc: &mut usize,
) -> Result<Instruction, ScriptError> {
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
    Ok(data.to_vec())
}

#[cfg(test)]
mod serialized_tests {
    use super::*;

    #[test]
    fn phase167_serialized_script_paths_find_delete_pin_literal_matrix() {
        type LiteralCase<'a> = (&'a str, &'a [u8], &'a [u8], &'a [u8]);

        // Arrange: thirteen independent literal source-derived cases, not spend proofs.
        let cases: &[LiteralCase<'_>] = &[
            (
                "direct payload",
                &[0x30],
                &[3, 1, 0x30, 0x99, 0x51],
                &[3, 1, 0x30, 0x99, 0x51],
            ),
            (
                "nonminimal same item",
                &[0x30],
                &[0x4c, 1, 0x30, 0x51],
                &[0x4c, 1, 0x30, 0x51],
            ),
            (
                "pushdata payload",
                &[0x30],
                &[0x4c, 2, 1, 0x30, 0x51],
                &[0x4c, 2, 1, 0x30, 0x51],
            ),
            (
                "repeated boundary",
                &[0x30],
                &[1, 0x30, 1, 0x30, 0x51],
                &[0x51],
            ),
            (
                "retained prefix and suffix",
                &[0x30],
                &[0x51, 1, 0x30, 1, 0x30, 0x52],
                &[0x51, 0x52],
            ),
            ("all removed", &[0x30], &[1, 0x30, 1, 0x30], &[]),
            (
                "malformed without removal",
                &[0x30],
                &[0x4c, 3, 1, 0x30],
                &[0x4c, 3, 1, 0x30],
            ),
            (
                "malformed after removal",
                &[0x30],
                &[1, 0x30, 0x4c, 3, 1, 0x30],
                &[0x4c, 3, 1, 0x30],
            ),
            (
                "prefix and malformed tail",
                &[0x30],
                &[0x51, 1, 0x30, 0x4c, 3, 1, 0x30],
                &[0x51, 0x4c, 3, 1, 0x30],
            ),
            ("truncated length", &[0x30], &[1, 0x30, 0x4c], &[0x4c]),
            ("empty signature boundary", &[], &[0, 0x51], &[0x51]),
            (
                "empty signature payload",
                &[],
                &[1, 0, 0, 0x51],
                &[1, 0, 0x51],
            ),
            (
                "DER shaped payload",
                &[0x30, 6, 2, 1, 1, 2, 1, 1, 1],
                &[10, 9, 0x30, 6, 2, 1, 1, 2, 1, 1, 1, 0x75, 0x51],
                &[10, 9, 0x30, 6, 2, 1, 1, 2, 1, 1, 1, 0x75, 0x51],
            ),
        ];
        // Act: collect every case even when an earlier observation disagrees.
        let observed: Vec<_> = cases
            .iter()
            .map(|(label, signature, bytes, _)| {
                (
                    *label,
                    remove_signature_from_script(
                        &ScriptBuf::from_serialized_bytes(bytes.to_vec()),
                        signature,
                    )
                    .into_bytes(),
                )
            })
            .collect();
        // Assert
        let expected: Vec<_> = cases
            .iter()
            .map(|(label, _, _, bytes)| (*label, bytes.to_vec()))
            .collect();
        assert_eq!(observed, expected);
    }

    #[test]
    fn phase167_serialized_script_paths_find_delete_preserves_nonminimal_encodings() {
        // Arrange
        let cases = [
            vec![0x4c, 1, 0x30, 0x51],
            vec![0x4d, 1, 0, 0x30, 0x51],
            vec![0x4e, 1, 0, 0, 0, 0x30, 0x51],
        ];
        // Act
        let observed: Vec<_> = cases
            .iter()
            .map(|bytes| {
                remove_signature_from_script(
                    &ScriptBuf::from_serialized_bytes(bytes.clone()),
                    &[0x30],
                )
                .into_bytes()
            })
            .collect();
        // Assert
        assert_eq!(observed, cases);
    }

    #[test]
    fn phase167_serialized_script_paths_find_delete_skips_large_push_payload() {
        // Arrange: a complete521-byte push, then a real target boundary.
        let mut bytes = vec![0x4d, 0x09, 0x02];
        let mut payload = vec![0x99; 521];
        payload[100..102].copy_from_slice(&[1, 0x30]);
        bytes.extend_from_slice(&payload);
        let mut expected = bytes.clone();
        expected.push(0x51);
        bytes.extend_from_slice(&[1, 0x30, 0x51]);
        // Act
        let removed =
            remove_signature_from_script(&ScriptBuf::from_serialized_bytes(bytes), &[0x30]);
        // Assert
        assert_eq!(removed.into_bytes(), expected);
    }

    #[test]
    fn phase167_serialized_script_paths_find_delete_retains_complete_malformed_tails() {
        // Arrange: truncated length and truncated payload for all three encodings.
        let tails = [
            vec![0x4c],
            vec![0x4d, 1],
            vec![0x4e, 1, 0, 0],
            vec![0x4c, 3, 1, 0x30],
            vec![0x4d, 3, 0, 1, 0x30],
            vec![0x4e, 3, 0, 0, 0, 1, 0x30],
        ];
        // Act
        let observed: Vec<_> = tails
            .iter()
            .flat_map(|tail| {
                [false, true].map(|remove_first| {
                    let mut bytes = if remove_first { vec![1, 0x30] } else { vec![] };
                    bytes.extend_from_slice(tail);
                    remove_signature_from_script(&ScriptBuf::from_serialized_bytes(bytes), &[0x30])
                        .into_bytes()
                })
            })
            .collect();
        // Assert
        let expected: Vec<_> = tails
            .iter()
            .flat_map(|tail| [tail.clone(), tail.clone()])
            .collect();
        assert_eq!(observed, expected);
    }

    #[test]
    fn phase167_serialized_script_paths_find_delete_sequential_signature_removal() {
        // Arrange: the31 pattern inside the two-byte push must survive both passes.
        let bytes = vec![0x51, 1, 0x30, 2, 1, 0x31, 1, 0x31, 1, 0x30, 0x52];
        let script = ScriptBuf::from_serialized_bytes(bytes);
        // Act
        let first = remove_signature_from_script(&script, &[0x30]);
        let second = remove_signature_from_script(&first, &[0x31]);
        // Assert
        assert_eq!(second.as_bytes(), &[0x51, 2, 1, 0x31, 0x52]);
    }

    #[test]
    fn phase167_serialized_script_paths_find_delete_empty_signature_keeps_length_fields() {
        // Arrange: nonminimal empty pushes are distinct from target OP_0.
        let bytes = vec![0x4c, 0, 0x4d, 0, 0, 0x4e, 0, 0, 0, 0, 0, 0x51];
        // Act
        let removed = remove_signature_from_script(&ScriptBuf::from_serialized_bytes(bytes), &[]);
        // Assert
        assert_eq!(
            removed.as_bytes(),
            &[0x4c, 0, 0x4d, 0, 0, 0x4e, 0, 0, 0, 0, 0x51]
        );
    }

    #[test]
    fn phase167_serialized_script_paths_wide_rebuild_never_restores_original() {
        // Arrange
        let mut bytes = vec![0x61; 10_001];
        bytes.extend_from_slice(&[1, 0x30, 1, 0x30]);
        let script = ScriptBuf::from_serialized_bytes(bytes);
        // Act
        let removed = remove_signature_from_script(&script, &[0x30]);
        // Assert
        assert_eq!(removed.as_bytes(), vec![0x61; 10_001]);
    }
}
