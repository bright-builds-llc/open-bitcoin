use core::fmt;

use open_bitcoin_primitives::{
    Amount, MAX_OPS_PER_SCRIPT, MAX_SCRIPT_ELEMENT_SIZE, ScriptBuf, ScriptWitness, Transaction,
};
use secp256k1::Secp256k1;

use crate::classify::{ScriptPubKeyType, classify_script_pubkey, extract_script_sig_pushes};
use crate::context::{
    PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags, TransactionInputContext,
    TransactionValidationContext,
};
use crate::crypto::{Sha256, double_sha256};
use crate::signature::{EcdsaVerificationRequest, TransactionSignatureChecker};

const MAX_STACK_SIZE: usize = 1_000;
const OP_PUSHDATA1: u8 = 0x4c;
const OP_PUSHDATA2: u8 = 0x4d;
const OP_PUSHDATA4: u8 = 0x4e;
const OP_1NEGATE: u8 = 0x4f;
const OP_1: u8 = 0x51;
const OP_16: u8 = 0x60;
const OP_NOP: u8 = 0x61;
const OP_VERIFY: u8 = 0x69;
const OP_DROP: u8 = 0x75;
const OP_DUP: u8 = 0x76;
const OP_OVER: u8 = 0x78;
const OP_SWAP: u8 = 0x7c;
const OP_SIZE: u8 = 0x82;
const OP_EQUAL: u8 = 0x87;
const OP_EQUALVERIFY: u8 = 0x88;
const OP_1ADD: u8 = 0x8b;
const OP_1SUB: u8 = 0x8c;
const OP_NEGATE: u8 = 0x8f;
const OP_NOT: u8 = 0x91;
const OP_0NOTEQUAL: u8 = 0x92;
const OP_ADD: u8 = 0x93;
const OP_SUB: u8 = 0x94;
const OP_BOOLAND: u8 = 0x9a;
const OP_BOOLOR: u8 = 0x9b;
const OP_NUMEQUAL: u8 = 0x9c;
const OP_NUMEQUALVERIFY: u8 = 0x9d;
const OP_NUMNOTEQUAL: u8 = 0x9e;
const OP_LESSTHAN: u8 = 0x9f;
const OP_GREATERTHAN: u8 = 0xa0;
const OP_MIN: u8 = 0xa3;
const OP_MAX: u8 = 0xa4;
const OP_WITHIN: u8 = 0xa5;
const OP_SHA256: u8 = 0xa8;
const OP_HASH256: u8 = 0xaa;
const OP_CHECKSIG: u8 = 0xac;
const OP_CHECKSIGVERIFY: u8 = 0xad;
const OP_CHECKMULTISIG: u8 = 0xae;
const OP_CHECKMULTISIGVERIFY: u8 = 0xaf;
const OP_RETURN: u8 = 0x6a;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    BadOpcode,
    DisabledOpcode(u8),
    EvalFalse,
    InvalidStackOperation,
    NumOverflow(usize),
    OpCount,
    OpReturn,
    PushSize(usize),
    StackOverflow(usize),
    TruncatedPushData,
    UnsupportedOpcode(u8),
    VerifyFailed,
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadOpcode => write!(f, "bad opcode"),
            Self::DisabledOpcode(opcode) => write!(f, "disabled opcode: 0x{opcode:02x}"),
            Self::EvalFalse => write!(f, "script evaluated to false"),
            Self::InvalidStackOperation => write!(f, "invalid stack operation"),
            Self::NumOverflow(size) => write!(f, "script number overflow: {size} bytes"),
            Self::OpCount => write!(f, "script exceeds opcode limit"),
            Self::OpReturn => write!(f, "OP_RETURN encountered"),
            Self::PushSize(size) => write!(f, "push exceeds stack element limit: {size} bytes"),
            Self::StackOverflow(size) => write!(f, "stack exceeds maximum size: {size}"),
            Self::TruncatedPushData => write!(f, "truncated pushdata"),
            Self::UnsupportedOpcode(opcode) => write!(f, "unsupported opcode: 0x{opcode:02x}"),
            Self::VerifyFailed => write!(f, "VERIFY failed"),
        }
    }
}

impl std::error::Error for ScriptError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    opcode: u8,
    maybe_data: Option<Vec<u8>>,
}

pub fn eval_script(stack: &mut Vec<Vec<u8>>, script: &ScriptBuf) -> Result<(), ScriptError> {
    let bytes = script.as_bytes();
    let mut pc = 0;
    let mut op_count = 0;

    while pc < bytes.len() {
        let instruction = read_instruction(bytes, &mut pc)?;
        if instruction.opcode > OP_16 {
            op_count += 1;
            if op_count > MAX_OPS_PER_SCRIPT {
                return Err(ScriptError::OpCount);
            }
        }

        if let Some(data) = instruction.maybe_data {
            push_stack(stack, data)?;
            continue;
        }

        match instruction.opcode {
            OP_1NEGATE => push_stack(stack, encode_script_num(-1))?,
            OP_1..=OP_16 => {
                let value = i64::from(instruction.opcode) - i64::from(OP_1) + 1;
                push_stack(stack, encode_script_num(value))?;
            }
            OP_NOP => {}
            OP_VERIFY => {
                if !cast_to_bool(&pop_bytes(stack)?) {
                    return Err(ScriptError::VerifyFailed);
                }
            }
            OP_DROP => {
                pop_bytes(stack)?;
            }
            OP_DUP => {
                let value = stack
                    .last()
                    .cloned()
                    .ok_or(ScriptError::InvalidStackOperation)?;
                push_stack(stack, value)?;
            }
            OP_OVER => {
                let value = stack
                    .get(
                        stack
                            .len()
                            .checked_sub(2)
                            .ok_or(ScriptError::InvalidStackOperation)?,
                    )
                    .cloned()
                    .ok_or(ScriptError::InvalidStackOperation)?;
                push_stack(stack, value)?;
            }
            OP_SWAP => {
                if stack.len() < 2 {
                    return Err(ScriptError::InvalidStackOperation);
                }
                let top = stack.len() - 1;
                stack.swap(top, top - 1);
            }
            OP_SIZE => {
                let value = stack.last().ok_or(ScriptError::InvalidStackOperation)?;
                push_stack(stack, encode_script_num(value.len() as i64))?;
            }
            OP_EQUAL => {
                let right = pop_bytes(stack)?;
                let left = pop_bytes(stack)?;
                push_stack(stack, encode_bool(left == right))?;
            }
            OP_EQUALVERIFY => {
                let right = pop_bytes(stack)?;
                let left = pop_bytes(stack)?;
                if left != right {
                    return Err(ScriptError::VerifyFailed);
                }
            }
            OP_1ADD => unary_num_op(stack, |value| value + 1)?,
            OP_1SUB => unary_num_op(stack, |value| value - 1)?,
            OP_NEGATE => unary_num_op(stack, |value| -value)?,
            OP_NOT => unary_num_op(stack, |value| if value == 0 { 1 } else { 0 })?,
            OP_0NOTEQUAL => unary_num_op(stack, |value| if value == 0 { 0 } else { 1 })?,
            OP_ADD => binary_num_op(stack, |left, right| left + right)?,
            OP_SUB => binary_num_op(stack, |left, right| left - right)?,
            OP_BOOLAND => binary_num_op(stack, script_booland)?,
            OP_BOOLOR => binary_num_op(stack, script_boolor)?,
            OP_NUMEQUAL => binary_num_op(stack, |left, right| if left == right { 1 } else { 0 })?,
            OP_NUMEQUALVERIFY => {
                let right = pop_num(stack)?;
                let left = pop_num(stack)?;
                if left != right {
                    return Err(ScriptError::VerifyFailed);
                }
            }
            OP_NUMNOTEQUAL => {
                binary_num_op(stack, |left, right| if left != right { 1 } else { 0 })?
            }
            OP_LESSTHAN => binary_num_op(stack, |left, right| if left < right { 1 } else { 0 })?,
            OP_GREATERTHAN => binary_num_op(stack, |left, right| if left > right { 1 } else { 0 })?,
            OP_MIN => binary_num_op(stack, |left, right| left.min(right))?,
            OP_MAX => binary_num_op(stack, |left, right| left.max(right))?,
            OP_WITHIN => {
                let max = pop_num(stack)?;
                let min = pop_num(stack)?;
                let value = pop_num(stack)?;
                let within = value >= min && value < max;
                push_stack(stack, encode_bool(within))?;
            }
            OP_SHA256 => {
                let value = pop_bytes(stack)?;
                push_stack(stack, Sha256::digest(&value).to_vec())?;
            }
            OP_HASH256 => {
                let value = pop_bytes(stack)?;
                push_stack(stack, double_sha256(&value).to_vec())?;
            }
            OP_RETURN => return Err(ScriptError::OpReturn),
            OP_CHECKSIG | OP_CHECKSIGVERIFY | OP_CHECKMULTISIG | OP_CHECKMULTISIGVERIFY => {
                return Err(ScriptError::UnsupportedOpcode(instruction.opcode));
            }
            opcode if is_disabled_opcode(opcode) => {
                return Err(ScriptError::DisabledOpcode(opcode));
            }
            opcode => return Err(ScriptError::UnsupportedOpcode(opcode)),
        }
    }

    Ok(())
}

pub fn verify_script(script_sig: &ScriptBuf, script_pubkey: &ScriptBuf) -> Result<(), ScriptError> {
    let mut stack = Vec::new();
    eval_script(&mut stack, script_sig)?;
    eval_script(&mut stack, script_pubkey)?;

    let Some(top) = stack.last() else {
        return Err(ScriptError::EvalFalse);
    };
    if !cast_to_bool(top) {
        return Err(ScriptError::EvalFalse);
    }

    Ok(())
}

pub struct ScriptInputVerificationContext<'a> {
    pub script_sig: &'a ScriptBuf,
    pub script_pubkey: &'a ScriptBuf,
    pub witness: &'a ScriptWitness,
    pub transaction: &'a Transaction,
    pub input_index: usize,
    pub spent_input: &'a TransactionInputContext,
    pub validation_context: &'a TransactionValidationContext,
    pub spent_amount: Amount,
    pub verify_flags: ScriptVerifyFlags,
    pub precomputed: &'a PrecomputedTransactionData,
    pub execution_data: &'a mut ScriptExecutionData,
}

pub fn verify_input_script(context: ScriptInputVerificationContext<'_>) -> Result<(), ScriptError> {
    if !context.witness.is_empty() {
        return Err(ScriptError::UnsupportedOpcode(OP_0NOTEQUAL));
    }

    let classified = classify_script_pubkey(context.script_pubkey);
    match classified {
        ScriptPubKeyType::PayToPubKey { compressed, pubkey } => {
            verify_pay_to_pubkey(context, &pubkey, compressed)
        }
        ScriptPubKeyType::Multisig {
            required_signatures,
            pubkeys,
        } => verify_bare_multisig(context, &pubkeys, required_signatures),
        _ => verify_script(context.script_sig, context.script_pubkey),
    }
}

pub fn count_legacy_sigops(script: &ScriptBuf) -> Result<usize, ScriptError> {
    let bytes = script.as_bytes();
    let mut pc = 0;
    let mut sigops = 0;
    while pc < bytes.len() {
        let instruction = read_instruction(bytes, &mut pc)?;
        match instruction.opcode {
            OP_CHECKSIG | OP_CHECKSIGVERIFY => sigops += 1,
            OP_CHECKMULTISIG | OP_CHECKMULTISIGVERIFY => sigops += 20,
            _ => {}
        }
    }
    Ok(sigops)
}

fn read_instruction(bytes: &[u8], pc: &mut usize) -> Result<Instruction, ScriptError> {
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

fn push_stack(stack: &mut Vec<Vec<u8>>, value: Vec<u8>) -> Result<(), ScriptError> {
    stack.push(value);
    if stack.len() > MAX_STACK_SIZE {
        return Err(ScriptError::StackOverflow(stack.len()));
    }
    Ok(())
}

fn pop_bytes(stack: &mut Vec<Vec<u8>>) -> Result<Vec<u8>, ScriptError> {
    stack.pop().ok_or(ScriptError::InvalidStackOperation)
}

fn pop_num(stack: &mut Vec<Vec<u8>>) -> Result<i64, ScriptError> {
    let value = pop_bytes(stack)?;
    decode_script_num(&value)
}

fn unary_num_op(
    stack: &mut Vec<Vec<u8>>,
    operation: impl FnOnce(i64) -> i64,
) -> Result<(), ScriptError> {
    let value = pop_num(stack)?;
    push_stack(stack, encode_script_num(operation(value)))
}

fn binary_num_op(
    stack: &mut Vec<Vec<u8>>,
    operation: impl FnOnce(i64, i64) -> i64,
) -> Result<(), ScriptError> {
    let right = pop_num(stack)?;
    let left = pop_num(stack)?;
    push_stack(stack, encode_script_num(operation(left, right)))
}

fn script_booland(left: i64, right: i64) -> i64 {
    if left != 0 && right != 0 { 1 } else { 0 }
}

fn script_boolor(left: i64, right: i64) -> i64 {
    if left != 0 || right != 0 { 1 } else { 0 }
}

fn verify_pay_to_pubkey(
    context: ScriptInputVerificationContext<'_>,
    pubkey: &[u8],
    require_compressed_pubkey: bool,
) -> Result<(), ScriptError> {
    let pushes =
        extract_script_sig_pushes(context.script_sig).ok_or(ScriptError::InvalidStackOperation)?;
    let signature = pushes.last().ok_or(ScriptError::InvalidStackOperation)?;
    let secp = Secp256k1::verification_only();
    let checker =
        TransactionSignatureChecker::new(&secp, context.validation_context, context.precomputed);

    checker
        .verify_ecdsa(EcdsaVerificationRequest {
            script_code: context.script_pubkey,
            transaction: context.transaction,
            input_index: context.input_index,
            spent_input: context.spent_input,
            signature_bytes: signature,
            public_key_bytes: pubkey,
            sig_version: crate::sighash::SigVersion::Base,
            require_compressed_pubkey,
        })
        .map_err(|_| ScriptError::VerifyFailed)
}

fn verify_bare_multisig(
    context: ScriptInputVerificationContext<'_>,
    pubkeys: &[Vec<u8>],
    required_signatures: usize,
) -> Result<(), ScriptError> {
    let pushes =
        extract_script_sig_pushes(context.script_sig).ok_or(ScriptError::InvalidStackOperation)?;
    let Some((dummy, signatures)) = pushes.split_first() else {
        return Err(ScriptError::InvalidStackOperation);
    };
    if !dummy.is_empty() || signatures.len() < required_signatures {
        return Err(ScriptError::VerifyFailed);
    }

    let secp = Secp256k1::verification_only();
    let checker =
        TransactionSignatureChecker::new(&secp, context.validation_context, context.precomputed);
    let mut pubkey_index = 0;
    let mut matched_signatures = 0;
    for signature in signatures.iter().take(required_signatures) {
        let mut matched = false;
        while pubkey_index < pubkeys.len() {
            if checker
                .verify_ecdsa(EcdsaVerificationRequest {
                    script_code: context.script_pubkey,
                    transaction: context.transaction,
                    input_index: context.input_index,
                    spent_input: context.spent_input,
                    signature_bytes: signature,
                    public_key_bytes: &pubkeys[pubkey_index],
                    sig_version: crate::sighash::SigVersion::Base,
                    require_compressed_pubkey: pubkeys[pubkey_index].len() == 33,
                })
                .is_ok()
            {
                matched = true;
                pubkey_index += 1;
                matched_signatures += 1;
                break;
            }
            pubkey_index += 1;
        }
        if !matched {
            return Err(ScriptError::VerifyFailed);
        }
    }

    debug_assert_eq!(matched_signatures, required_signatures);
    Ok(())
}

fn encode_bool(value: bool) -> Vec<u8> {
    if value { vec![1_u8] } else { Vec::new() }
}

fn cast_to_bool(value: &[u8]) -> bool {
    for (index, byte) in value.iter().enumerate() {
        if *byte == 0 {
            continue;
        }
        if index == value.len() - 1 && *byte == 0x80 {
            return false;
        }
        return true;
    }
    false
}

fn decode_script_num(bytes: &[u8]) -> Result<i64, ScriptError> {
    if bytes.len() > 4 {
        return Err(ScriptError::NumOverflow(bytes.len()));
    }
    if bytes.is_empty() {
        return Ok(0);
    }

    let mut value = 0_i64;
    for (index, byte) in bytes.iter().enumerate() {
        value |= i64::from(*byte) << (8 * index);
    }

    let last = *bytes.last().expect("non-empty checked above");
    if (last & 0x80) != 0 {
        let mask = !(0x80_i64 << (8 * (bytes.len() - 1)));
        Ok(-(value & mask))
    } else {
        Ok(value)
    }
}

fn encode_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return Vec::new();
    }

    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    if encoded.last().is_some_and(|byte| (byte & 0x80) != 0) {
        encoded.push(if negative { 0x80 } else { 0x00 });
    } else if negative {
        let last = encoded.last_mut().expect("non-empty because value != 0");
        *last |= 0x80;
    }

    encoded
}

fn is_disabled_opcode(opcode: u8) -> bool {
    matches!(
        opcode,
        0x7e | 0x7f
            | 0x80
            | 0x81
            | 0x83
            | 0x84
            | 0x85
            | 0x86
            | 0x8d
            | 0x8e
            | 0x95
            | 0x96
            | 0x97
            | 0x98
            | 0x99
    )
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{
        Amount, ScriptWitness, Transaction, TransactionInput, TransactionOutput, Txid,
    };
    use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

    use crate::context::{TransactionInputContext, TransactionValidationContext};
    use open_bitcoin_primitives::ScriptBuf;

    use crate::context::{PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags};
    use crate::sighash::{SigHashType, legacy_sighash};

    use super::{
        ScriptError, ScriptInputVerificationContext, cast_to_bool, count_legacy_sigops,
        decode_script_num, encode_script_num, eval_script, is_disabled_opcode, verify_script,
    };

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    #[test]
    fn verify_script_matches_knots_equal_vector() {
        let script_sig = script(&[0x51, 0x52]);
        let script_pubkey = script(&[0x52, 0x88, 0x51, 0x87]);

        assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
    }

    #[test]
    fn verify_script_matches_knots_add_vector() {
        let script_sig = script(&[0x51, 0x51]);
        let script_pubkey = script(&[0x93, 0x52, 0x87]);

        assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
    }

    #[test]
    fn verify_script_matches_knots_sha256_vector() {
        let script_sig = script(&[0x01, 0x61]);
        let script_pubkey = script(&[
            0xa8, 0x20, 0xca, 0x97, 0x81, 0x12, 0xca, 0x1b, 0xbd, 0xca, 0xfa, 0xc2, 0x31, 0xb3,
            0x9a, 0x23, 0xdc, 0x4d, 0xa7, 0x86, 0xef, 0xf8, 0x14, 0x7c, 0x4e, 0x72, 0xb9, 0x80,
            0x77, 0x85, 0xaf, 0xee, 0x48, 0xbb, 0x87,
        ]);

        assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
    }

    #[test]
    fn verify_script_matches_knots_hash256_vector() {
        let script_sig = script(&[0x01, 0x61]);
        let script_pubkey = script(&[
            0xaa, 0x20, 0xbf, 0x5d, 0x3a, 0xff, 0xb7, 0x3e, 0xfd, 0x2e, 0xc6, 0xc3, 0x6a, 0xd3,
            0x11, 0x2d, 0xd9, 0x33, 0xef, 0xed, 0x63, 0xc4, 0xe1, 0xcb, 0xff, 0xcf, 0xa8, 0x8e,
            0x27, 0x59, 0xc1, 0x44, 0xf2, 0xd8, 0x87,
        ]);

        assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
    }

    #[test]
    fn verify_script_rejects_false_final_stack() {
        let error =
            verify_script(&script(&[]), &script(&[0x00])).expect_err("false stack must fail");

        assert_eq!(error, ScriptError::EvalFalse);
    }

    #[test]
    fn verify_script_rejects_empty_stack_after_execution() {
        let error =
            verify_script(&script(&[]), &script(&[])).expect_err("empty final stack must fail");

        assert_eq!(error, ScriptError::EvalFalse);
    }

    #[test]
    fn verify_script_rejects_op_return() {
        let error = verify_script(&script(&[]), &script(&[0x6a])).expect_err("OP_RETURN must fail");

        assert_eq!(error, ScriptError::OpReturn);
    }

    #[test]
    fn count_legacy_sigops_skips_push_data() {
        let sigops =
            count_legacy_sigops(&script(&[0x01, 0xac, 0xac, 0xae])).expect("sigops should parse");

        assert_eq!(sigops, 21);
    }

    #[test]
    fn eval_script_reports_stack_overflow() {
        let pushes = vec![0x51; 1001];
        let error =
            eval_script(&mut Vec::new(), &script(&pushes)).expect_err("too many pushes must fail");

        assert_eq!(error, ScriptError::StackOverflow(1001));
    }

    #[test]
    fn script_error_display_covers_all_variants() {
        let cases = [
            (ScriptError::BadOpcode, "bad opcode"),
            (ScriptError::DisabledOpcode(0x7e), "disabled opcode: 0x7e"),
            (ScriptError::EvalFalse, "script evaluated to false"),
            (
                ScriptError::InvalidStackOperation,
                "invalid stack operation",
            ),
            (
                ScriptError::NumOverflow(5),
                "script number overflow: 5 bytes",
            ),
            (ScriptError::OpCount, "script exceeds opcode limit"),
            (ScriptError::OpReturn, "OP_RETURN encountered"),
            (
                ScriptError::PushSize(521),
                "push exceeds stack element limit: 521 bytes",
            ),
            (
                ScriptError::StackOverflow(1001),
                "stack exceeds maximum size: 1001",
            ),
            (ScriptError::TruncatedPushData, "truncated pushdata"),
            (
                ScriptError::UnsupportedOpcode(0xac),
                "unsupported opcode: 0xac",
            ),
            (ScriptError::VerifyFailed, "VERIFY failed"),
        ];

        for (error, expected) in cases {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn helpers_cover_bool_number_and_disabled_opcode_edges() {
        assert!(!cast_to_bool(&[0x80]));
        assert!(!cast_to_bool(&[0x00]));
        assert!(cast_to_bool(&[0x01]));
        assert_eq!(decode_script_num(&[0x81]), Ok(-1));
        assert_eq!(decode_script_num(&[0x01, 0x80]), Ok(-1));
        assert_eq!(decode_script_num(&[0; 5]), Err(ScriptError::NumOverflow(5)));
        assert_eq!(encode_script_num(0), Vec::<u8>::new());
        assert_eq!(encode_script_num(-1), vec![0x81]);
        assert_eq!(encode_script_num(128), vec![0x80, 0x00]);
        assert!(is_disabled_opcode(0x7e));
        assert!(!is_disabled_opcode(0x51));
    }

    #[test]
    fn eval_script_supports_stack_and_numeric_helpers() {
        let mut stack = Vec::new();
        eval_script(
            &mut stack,
            &script(&[
                0x4f, 0x75, 0x51, 0x52, 0x78, 0x7c, 0x75, 0x82, 0x75, 0x8b, 0x8c, 0x8f, 0x8f, 0x91,
                0x92, 0x51, 0x51, 0x94,
            ]),
        )
        .expect("script should execute");

        assert_eq!(stack, vec![vec![1_u8], Vec::<u8>::new(), Vec::<u8>::new()]);
    }

    #[test]
    fn eval_script_covers_dup_and_boolean_binary_ops() {
        let mut stack = Vec::new();
        eval_script(&mut stack, &script(&[0x51, 0x76, 0x51, 0x9a, 0x00, 0x9b]))
            .expect("dup/bool ops should execute");

        assert_eq!(stack, vec![vec![1_u8], vec![1_u8]]);
    }

    #[test]
    fn eval_script_covers_false_boolean_binary_ops() {
        let mut stack = Vec::new();
        eval_script(&mut stack, &script(&[0x00, 0x51, 0x9a, 0x00, 0x00, 0x9b]))
            .expect("false bool ops should execute");

        assert_eq!(stack, vec![Vec::<u8>::new(), Vec::<u8>::new()]);
    }

    #[test]
    fn eval_script_supports_boolean_and_comparison_ops() {
        let mut stack = Vec::new();
        eval_script(
            &mut stack,
            &script(&[
                0x51, 0x51, 0x9a, 0x51, 0x00, 0x9b, 0x51, 0x51, 0x9c, 0x51, 0x52, 0x9e, 0x51, 0x52,
                0x9f, 0x52, 0x51, 0xa0, 0x51, 0x52, 0xa3, 0x51, 0x52, 0xa4, 0x51, 0x51, 0x52, 0xa5,
            ]),
        )
        .expect("script should execute");

        assert_eq!(stack.len(), 9);
        assert!(stack.iter().all(|item| cast_to_bool(item)));
    }

    #[test]
    fn eval_script_supports_verify_variants() {
        let mut stack = Vec::new();
        eval_script(&mut stack, &script(&[0x51, 0x69, 0x51, 0x51, 0x9d]))
            .expect("verify variants should succeed");

        assert!(stack.is_empty());
    }

    #[test]
    fn eval_script_verify_false_branch_is_reported() {
        let error = eval_script(&mut Vec::new(), &script(&[0x00, 0x69]))
            .expect_err("verify false must fail");

        assert_eq!(error, ScriptError::VerifyFailed);
    }

    #[test]
    fn eval_script_rejects_invalid_stack_operations() {
        let cases = [
            script(&[0x75]),
            script(&[0x76]),
            script(&[0x78]),
            script(&[0x7c]),
            script(&[0x82]),
        ];

        for candidate in cases {
            let error =
                eval_script(&mut Vec::new(), &candidate).expect_err("empty-stack op must fail");
            assert_eq!(error, ScriptError::InvalidStackOperation);
        }
    }

    #[test]
    fn eval_script_rejects_verify_failures_and_unsupported_opcodes() {
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[0x51, 0x52, 0x88]))
                .expect_err("equalverify mismatch must fail"),
            ScriptError::VerifyFailed,
        );
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[0x51, 0x52, 0x9d]))
                .expect_err("numequalverify mismatch must fail"),
            ScriptError::VerifyFailed,
        );
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[0xac]))
                .expect_err("checksig must be unsupported"),
            ScriptError::UnsupportedOpcode(0xac),
        );
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[0x62]))
                .expect_err("unknown opcode must be unsupported"),
            ScriptError::UnsupportedOpcode(0x62),
        );
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[0x7e])).expect_err("disabled opcode must fail"),
            ScriptError::DisabledOpcode(0x7e),
        );
    }

    #[test]
    fn eval_script_rejects_opcount_and_pushdata_errors() {
        let opcount_script = vec![0x61; 202];
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&opcount_script))
                .expect_err("too many opcodes must fail"),
            ScriptError::OpCount,
        );
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[0x4c]))
                .expect_err("truncated pushdata1 must fail"),
            ScriptError::TruncatedPushData,
        );
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[0x4d, 0x01]))
                .expect_err("truncated pushdata2 must fail"),
            ScriptError::TruncatedPushData,
        );
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[0x4e, 0x01, 0x00, 0x00]))
                .expect_err("truncated pushdata4 must fail"),
            ScriptError::TruncatedPushData,
        );
        assert_eq!(
            count_legacy_sigops(&script(&[0x01])).expect_err("bad push must fail"),
            ScriptError::TruncatedPushData,
        );
    }

    #[test]
    fn eval_script_accepts_all_pushdata_forms() {
        let mut stack = Vec::new();
        eval_script(
            &mut stack,
            &script(&[
                0x4c, 0x01, 0x05, 0x4d, 0x01, 0x00, 0x06, 0x4e, 0x01, 0x00, 0x00, 0x00, 0x07,
            ]),
        )
        .expect("pushdata variants should execute");

        assert_eq!(stack, vec![vec![0x05], vec![0x06], vec![0x07]]);
    }

    #[test]
    fn eval_script_rejects_oversized_pushes() {
        let mut bytes = vec![0x4d, 0x09, 0x02];
        bytes.extend(vec![0x00; 521]);

        assert_eq!(
            eval_script(&mut Vec::new(), &script(&bytes)).expect_err("oversized push must fail"),
            ScriptError::PushSize(521),
        );
    }

    #[test]
    fn verify_input_script_rejects_witness_for_current_legacy_path() {
        let mut execution_data = ScriptExecutionData::default();
        let transaction = Transaction::default();
        let validation_context = TransactionValidationContext {
            inputs: vec![],
            spend_height: 0,
            block_time: 0,
            median_time_past: 0,
            verify_flags: ScriptVerifyFlags::WITNESS,
            consensus_params: crate::context::ConsensusParams::default(),
        };
        let spent_input = TransactionInputContext {
            spent_output: crate::context::SpentOutput {
                value: Amount::from_sats(0).expect("valid amount"),
                script_pubkey: script(&[0x51]),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        };
        let precomputed = PrecomputedTransactionData::new(&transaction, &[]).expect("precompute");

        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &script(&[0x51]),
            script_pubkey: &script(&[0x51]),
            witness: &ScriptWitness::new(vec![vec![0x01]]),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: Amount::from_sats(0).expect("valid amount"),
            verify_flags: ScriptVerifyFlags::WITNESS,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("witness should be unsupported in current verifier");

        assert_eq!(error, ScriptError::UnsupportedOpcode(0x92));
    }

    #[test]
    fn verify_input_script_accepts_pay_to_pubkey_signatures() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([17_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let script_pubkey = {
            let mut bytes = vec![33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.push(0xac);
            script(&bytes)
        };
        let transaction = Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([1_u8; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: Default::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(40).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        };
        let spent_input = TransactionInputContext {
            spent_output: crate::context::SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script_pubkey.clone(),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        };
        let validation_context = TransactionValidationContext {
            inputs: vec![spent_input.clone()],
            spend_height: 1,
            block_time: 0,
            median_time_past: 0,
            verify_flags: ScriptVerifyFlags::NONE,
            consensus_params: crate::context::ConsensusParams::default(),
        };
        let precomputed = validation_context
            .precompute(&transaction)
            .expect("precompute");

        let digest = legacy_sighash(&script_pubkey, &transaction, 0, SigHashType::ALL);
        let message = Message::from_digest(digest.to_byte_array());
        let mut signature = signing_secp.sign_ecdsa(message, &secret_key);
        signature.normalize_s();
        let serialized = signature.serialize_der();
        let mut signature_bytes = serialized.as_ref().to_vec();
        signature_bytes.push(SigHashType::ALL.raw() as u8);
        let script_sig = {
            let mut bytes = vec![signature_bytes.len() as u8];
            bytes.extend_from_slice(&signature_bytes);
            script(&bytes)
        };
        let mut execution_data = ScriptExecutionData::default();

        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &script_sig,
                script_pubkey: &script_pubkey,
                witness: &ScriptWitness::default(),
                transaction: &transaction,
                input_index: 0,
                spent_input: &spent_input,
                validation_context: &validation_context,
                spent_amount: spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::NONE,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );
    }

    #[test]
    fn verify_input_script_accepts_bare_multisig_signatures() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([19_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let script_pubkey = {
            let mut bytes = vec![0x51, 33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.push(0x51);
            bytes.push(0xae);
            script(&bytes)
        };
        let transaction = Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([2_u8; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: Default::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(40).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        };
        let spent_input = TransactionInputContext {
            spent_output: crate::context::SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script_pubkey.clone(),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        };
        let validation_context = TransactionValidationContext {
            inputs: vec![spent_input.clone()],
            spend_height: 1,
            block_time: 0,
            median_time_past: 0,
            verify_flags: ScriptVerifyFlags::NONE,
            consensus_params: crate::context::ConsensusParams::default(),
        };
        let precomputed = validation_context
            .precompute(&transaction)
            .expect("precompute");

        let digest = legacy_sighash(&script_pubkey, &transaction, 0, SigHashType::ALL);
        let message = Message::from_digest(digest.to_byte_array());
        let mut signature = signing_secp.sign_ecdsa(message, &secret_key);
        signature.normalize_s();
        let serialized = signature.serialize_der();
        let mut signature_bytes = serialized.as_ref().to_vec();
        signature_bytes.push(SigHashType::ALL.raw() as u8);
        let script_sig = {
            let mut bytes = vec![0x00, signature_bytes.len() as u8];
            bytes.extend_from_slice(&signature_bytes);
            script(&bytes)
        };
        let mut execution_data = ScriptExecutionData::default();

        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &script_sig,
                script_pubkey: &script_pubkey,
                witness: &ScriptWitness::default(),
                transaction: &transaction,
                input_index: 0,
                spent_input: &spent_input,
                validation_context: &validation_context,
                spent_amount: spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::NONE,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );
    }

    #[test]
    fn verify_input_script_rejects_invalid_bare_multisig_forms() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([21_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let script_pubkey = {
            let mut bytes = vec![0x51, 33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.push(0x51);
            bytes.push(0xae);
            script(&bytes)
        };
        let transaction = Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([3_u8; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: Default::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(40).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        };
        let spent_input = TransactionInputContext {
            spent_output: crate::context::SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script_pubkey.clone(),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        };
        let validation_context = TransactionValidationContext {
            inputs: vec![spent_input.clone()],
            spend_height: 1,
            block_time: 0,
            median_time_past: 0,
            verify_flags: ScriptVerifyFlags::NONE,
            consensus_params: crate::context::ConsensusParams::default(),
        };
        let precomputed = validation_context
            .precompute(&transaction)
            .expect("precompute");

        let mut execution_data = ScriptExecutionData::default();
        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &ScriptBuf::default(),
                script_pubkey: &script_pubkey,
                witness: &ScriptWitness::default(),
                transaction: &transaction,
                input_index: 0,
                spent_input: &spent_input,
                validation_context: &validation_context,
                spent_amount: spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::NONE,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            }),
            Err(ScriptError::InvalidStackOperation)
        );

        let bad_dummy_script_sig = script(&[0x01, 0x01]);
        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &bad_dummy_script_sig,
                script_pubkey: &script_pubkey,
                witness: &ScriptWitness::default(),
                transaction: &transaction,
                input_index: 0,
                spent_input: &spent_input,
                validation_context: &validation_context,
                spent_amount: spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::NONE,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            }),
            Err(ScriptError::VerifyFailed)
        );

        let bad_signature_script_sig = script(&[0x00, 0x01, 0x02]);
        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &bad_signature_script_sig,
                script_pubkey: &script_pubkey,
                witness: &ScriptWitness::default(),
                transaction: &transaction,
                input_index: 0,
                spent_input: &spent_input,
                validation_context: &validation_context,
                spent_amount: spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::NONE,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            }),
            Err(ScriptError::VerifyFailed)
        );
    }
}
