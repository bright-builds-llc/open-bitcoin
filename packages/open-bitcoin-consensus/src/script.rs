use core::fmt;

use open_bitcoin_primitives::{
    Amount, MAX_OPS_PER_SCRIPT, MAX_SCRIPT_ELEMENT_SIZE, ScriptBuf, ScriptWitness, Transaction,
};
use secp256k1::Secp256k1;

use crate::classify::{
    ScriptPubKeyType, classify_script_pubkey, extract_redeem_script, is_push_only,
};
use crate::context::{
    PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags, TransactionInputContext,
    TransactionValidationContext,
};
use crate::crypto::{Sha256, double_sha256, hash160};
use crate::sighash::SigVersion;
use crate::signature::{EcdsaVerificationRequest, SignatureError, TransactionSignatureChecker};

const MAX_STACK_SIZE: usize = 1_000;
const OP_PUSHDATA1: u8 = 0x4c;
const OP_PUSHDATA2: u8 = 0x4d;
const OP_PUSHDATA4: u8 = 0x4e;
const OP_1NEGATE: u8 = 0x4f;
const OP_1: u8 = 0x51;
const OP_16: u8 = 0x60;
const OP_NOP: u8 = 0x61;
const OP_IF: u8 = 0x63;
const OP_NOTIF: u8 = 0x64;
const OP_ELSE: u8 = 0x67;
const OP_ENDIF: u8 = 0x68;
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
const OP_RIPEMD160: u8 = 0xa6;
const OP_SHA256: u8 = 0xa8;
const OP_HASH160: u8 = 0xa9;
const OP_HASH256: u8 = 0xaa;
const OP_CHECKSIG: u8 = 0xac;
const OP_CHECKSIGVERIFY: u8 = 0xad;
const OP_CHECKMULTISIG: u8 = 0xae;
const OP_CHECKMULTISIGVERIFY: u8 = 0xaf;
const OP_RETURN: u8 = 0x6a;
const MAX_PUBKEYS_PER_MULTISIG: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptError {
    BadOpcode,
    DisabledOpcode(u8),
    EvalFalse,
    InvalidStackOperation,
    NumOverflow(usize),
    OpCount,
    OpReturn,
    PubKeyCount,
    PubKeyType,
    PushSize(usize),
    SigCount,
    SigDer,
    SigHashType,
    SigHighS,
    SigNullDummy,
    SigNullFail,
    SigPushOnly,
    StackOverflow(usize),
    TruncatedPushData,
    UnbalancedConditional,
    UnsupportedOpcode(u8),
    VerifyFailed,
    WitnessCleanStack,
    WitnessMalleated,
    WitnessMalleatedP2sh,
    WitnessProgramMismatch,
    WitnessProgramWitnessEmpty,
    WitnessProgramWrongLength,
    WitnessPubKeyType,
    WitnessUnexpected,
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
            Self::PubKeyCount => write!(f, "invalid public key count"),
            Self::PubKeyType => write!(f, "invalid public key encoding"),
            Self::PushSize(size) => write!(f, "push exceeds stack element limit: {size} bytes"),
            Self::SigCount => write!(f, "invalid signature count"),
            Self::SigDer => write!(f, "invalid DER signature"),
            Self::SigHashType => write!(f, "invalid signature hash type"),
            Self::SigHighS => write!(f, "non-low-S signature"),
            Self::SigNullDummy => write!(f, "non-null CHECKMULTISIG dummy argument"),
            Self::SigNullFail => write!(f, "non-null failing signature"),
            Self::SigPushOnly => write!(f, "scriptSig is not push-only"),
            Self::StackOverflow(size) => write!(f, "stack exceeds maximum size: {size}"),
            Self::TruncatedPushData => write!(f, "truncated pushdata"),
            Self::UnbalancedConditional => write!(f, "unbalanced conditional"),
            Self::UnsupportedOpcode(opcode) => write!(f, "unsupported opcode: 0x{opcode:02x}"),
            Self::VerifyFailed => write!(f, "VERIFY failed"),
            Self::WitnessCleanStack => write!(f, "witness script did not leave a clean stack"),
            Self::WitnessMalleated => write!(f, "witness program has unexpected scriptSig"),
            Self::WitnessMalleatedP2sh => {
                write!(f, "nested witness program scriptSig is malleated")
            }
            Self::WitnessProgramMismatch => write!(f, "witness program mismatch"),
            Self::WitnessProgramWitnessEmpty => write!(f, "witness program witness stack is empty"),
            Self::WitnessProgramWrongLength => write!(f, "witness program wrong length"),
            Self::WitnessPubKeyType => write!(f, "witness public key must be compressed"),
            Self::WitnessUnexpected => write!(f, "unexpected witness data"),
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
    eval_script_internal(stack, script, None)
}

struct LegacyExecutionContext<'a> {
    checker: TransactionSignatureChecker<'a, secp256k1::VerifyOnly>,
    transaction: &'a Transaction,
    input_index: usize,
    spent_input: &'a TransactionInputContext,
    verify_flags: ScriptVerifyFlags,
    sig_version: SigVersion,
}

#[derive(Default)]
struct ConditionStack(Vec<bool>);

impl ConditionStack {
    fn all_true(&self) -> bool {
        self.0.iter().all(|value| *value)
    }

    fn push(&mut self, value: bool) {
        self.0.push(value);
    }

    fn pop(&mut self) -> Option<bool> {
        self.0.pop()
    }

    fn toggle_top(&mut self) -> Result<(), ScriptError> {
        let Some(top) = self.0.last_mut() else {
            return Err(ScriptError::UnbalancedConditional);
        };
        *top = !*top;
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn outer_all_true(&self) -> bool {
        self.0
            .get(..self.0.len().saturating_sub(1))
            .is_none_or(|values| values.iter().all(|value| *value))
    }
}

fn eval_script_internal(
    stack: &mut Vec<Vec<u8>>,
    script: &ScriptBuf,
    maybe_context: Option<&LegacyExecutionContext<'_>>,
) -> Result<(), ScriptError> {
    let bytes = script.as_bytes();
    let mut pc = 0;
    let mut op_count = 0;
    let mut condition_stack = ConditionStack::default();

    while pc < bytes.len() {
        let instruction = read_instruction(bytes, &mut pc)?;
        if instruction.opcode > OP_16 {
            op_count += 1;
            if op_count > MAX_OPS_PER_SCRIPT {
                return Err(ScriptError::OpCount);
            }
        }

        let should_execute = condition_stack.all_true();

        if let Some(data) = instruction.maybe_data {
            if should_execute {
                push_stack(stack, data)?;
            }
            continue;
        }

        if matches!(instruction.opcode, OP_IF | OP_NOTIF) {
            if should_execute {
                let value = pop_bytes(stack)?;
                if maybe_context.is_some_and(|context| {
                    context.sig_version == SigVersion::WitnessV0
                        && context.verify_flags.contains(ScriptVerifyFlags::MINIMALIF)
                }) && !matches!(value.as_slice(), [] | [1])
                {
                    return Err(ScriptError::VerifyFailed);
                }
                let condition = cast_to_bool(&value);
                condition_stack.push(if instruction.opcode == OP_NOTIF {
                    !condition
                } else {
                    condition
                });
            } else {
                condition_stack.push(false);
            }
            continue;
        }
        if instruction.opcode == OP_ELSE {
            if !condition_stack.outer_all_true() {
                continue;
            }
            condition_stack.toggle_top()?;
            continue;
        }
        if instruction.opcode == OP_ENDIF {
            if condition_stack.pop().is_none() {
                return Err(ScriptError::UnbalancedConditional);
            }
            continue;
        }
        if !should_execute {
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
            OP_RIPEMD160 => return Err(ScriptError::UnsupportedOpcode(OP_RIPEMD160)),
            OP_SHA256 => {
                let value = pop_bytes(stack)?;
                push_stack(stack, Sha256::digest(&value).to_vec())?;
            }
            OP_HASH160 => {
                let value = pop_bytes(stack)?;
                push_stack(stack, hash160(&value).to_vec())?;
            }
            OP_HASH256 => {
                let value = pop_bytes(stack)?;
                push_stack(stack, double_sha256(&value).to_vec())?;
            }
            OP_RETURN => return Err(ScriptError::OpReturn),
            OP_CHECKSIG => execute_checksig(stack, script, maybe_context, false)?,
            OP_CHECKSIGVERIFY => execute_checksig(stack, script, maybe_context, true)?,
            OP_CHECKMULTISIG => {
                execute_checkmultisig(stack, script, maybe_context, &mut op_count, false)?
            }
            OP_CHECKMULTISIGVERIFY => {
                execute_checkmultisig(stack, script, maybe_context, &mut op_count, true)?
            }
            opcode if is_disabled_opcode(opcode) => {
                return Err(ScriptError::DisabledOpcode(opcode));
            }
            opcode => return Err(ScriptError::UnsupportedOpcode(opcode)),
        }
    }

    if !condition_stack.is_empty() {
        return Err(ScriptError::UnbalancedConditional);
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
    if context
        .verify_flags
        .contains(ScriptVerifyFlags::SIGPUSHONLY)
        && !is_push_only(context.script_sig)
    {
        return Err(ScriptError::SigPushOnly);
    }
    let secp = Secp256k1::verification_only();
    let checker =
        TransactionSignatureChecker::new(&secp, context.validation_context, context.precomputed);
    let execution_context = LegacyExecutionContext {
        checker,
        transaction: context.transaction,
        input_index: context.input_index,
        spent_input: context.spent_input,
        verify_flags: context.verify_flags,
        sig_version: SigVersion::Base,
    };

    let mut stack = Vec::new();
    let mut maybe_stack_copy = None;
    eval_script_internal(&mut stack, context.script_sig, Some(&execution_context))?;
    if context.verify_flags.contains(ScriptVerifyFlags::P2SH) {
        maybe_stack_copy = Some(stack.clone());
    }
    eval_script_internal(&mut stack, context.script_pubkey, Some(&execution_context))?;
    verify_top_stack_true(&stack)?;

    let mut had_witness = false;
    let script_pubkey_type = classify_script_pubkey(context.script_pubkey);
    if context.verify_flags.contains(ScriptVerifyFlags::WITNESS)
        && matches!(
            script_pubkey_type,
            ScriptPubKeyType::WitnessV0KeyHash(_) | ScriptPubKeyType::WitnessV0ScriptHash(_)
        )
    {
        had_witness = true;
        if !context.script_sig.as_bytes().is_empty() {
            return Err(ScriptError::WitnessMalleated);
        }
        verify_witness_program(&mut stack, &context, &script_pubkey_type, false, &secp)?;
    }

    if context.verify_flags.contains(ScriptVerifyFlags::P2SH)
        && matches!(script_pubkey_type, ScriptPubKeyType::PayToScriptHash(_))
    {
        if !is_push_only(context.script_sig) {
            return Err(ScriptError::SigPushOnly);
        }

        let mut redeem_stack = maybe_stack_copy.ok_or(ScriptError::InvalidStackOperation)?;
        let redeem_script =
            extract_redeem_script(context.script_sig).ok_or(ScriptError::InvalidStackOperation)?;
        pop_bytes(&mut redeem_stack)?;
        eval_script_internal(&mut redeem_stack, &redeem_script, Some(&execution_context))?;
        verify_top_stack_true(&redeem_stack)?;

        if context.verify_flags.contains(ScriptVerifyFlags::WITNESS) {
            let redeem_type = classify_script_pubkey(&redeem_script);
            if matches!(
                redeem_type,
                ScriptPubKeyType::WitnessV0KeyHash(_) | ScriptPubKeyType::WitnessV0ScriptHash(_)
            ) {
                had_witness = true;
                if context.script_sig.as_bytes() != single_push_script(&redeem_script).as_slice() {
                    return Err(ScriptError::WitnessMalleatedP2sh);
                }
                verify_witness_program(&mut redeem_stack, &context, &redeem_type, true, &secp)?;
            }
        }
        stack = redeem_stack;
    }

    if context.verify_flags.contains(ScriptVerifyFlags::CLEANSTACK) && stack.len() != 1 {
        return Err(ScriptError::WitnessCleanStack);
    }
    if context.verify_flags.contains(ScriptVerifyFlags::WITNESS)
        && !had_witness
        && !context.witness.is_empty()
    {
        return Err(ScriptError::WitnessUnexpected);
    }

    Ok(())
}

fn verify_top_stack_true(stack: &[Vec<u8>]) -> Result<(), ScriptError> {
    let Some(top) = stack.last() else {
        return Err(ScriptError::EvalFalse);
    };
    if !cast_to_bool(top) {
        return Err(ScriptError::EvalFalse);
    }
    Ok(())
}

fn verify_witness_program(
    stack: &mut Vec<Vec<u8>>,
    context: &ScriptInputVerificationContext<'_>,
    script_type: &ScriptPubKeyType,
    is_p2sh: bool,
    secp: &Secp256k1<secp256k1::VerifyOnly>,
) -> Result<(), ScriptError> {
    match script_type {
        ScriptPubKeyType::WitnessV0KeyHash(program) => {
            if context.witness.stack().len() != 2 {
                return Err(ScriptError::WitnessProgramMismatch);
            }
            let mut exec_script_bytes = vec![OP_DUP, OP_HASH160, 20];
            exec_script_bytes.extend_from_slice(program);
            exec_script_bytes.extend_from_slice(&[OP_EQUALVERIFY, OP_CHECKSIG]);
            let exec_script =
                ScriptBuf::from_bytes(exec_script_bytes).expect("generated P2WPKH script is valid");
            execute_witness_script(
                stack,
                context,
                &exec_script,
                context.witness.stack().to_vec(),
                secp,
            )
        }
        ScriptPubKeyType::WitnessV0ScriptHash(program) => {
            let Some((script_bytes, witness_items)) = context.witness.stack().split_last() else {
                return Err(ScriptError::WitnessProgramWitnessEmpty);
            };
            if Sha256::digest(script_bytes) != *program {
                return Err(ScriptError::WitnessProgramMismatch);
            }
            let exec_script = ScriptBuf::from_bytes(script_bytes.clone())
                .map_err(|_| ScriptError::WitnessProgramMismatch)?;
            execute_witness_script(stack, context, &exec_script, witness_items.to_vec(), secp)
        }
        ScriptPubKeyType::WitnessUnknown { .. } if !is_p2sh => {
            if context
                .verify_flags
                .contains(ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM)
            {
                return Err(ScriptError::UnsupportedOpcode(OP_0NOTEQUAL));
            }
            stack.clear();
            push_stack(stack, encode_bool(true))?;
            Ok(())
        }
        ScriptPubKeyType::WitnessUnknown { .. } => Ok(()),
        _ => {
            if matches!(script_type, ScriptPubKeyType::WitnessV1Taproot(_)) {
                return Err(ScriptError::WitnessProgramWrongLength);
            }
            Err(ScriptError::WitnessProgramWrongLength)
        }
    }
}

fn execute_witness_script(
    stack: &mut Vec<Vec<u8>>,
    context: &ScriptInputVerificationContext<'_>,
    exec_script: &ScriptBuf,
    witness_stack: Vec<Vec<u8>>,
    secp: &Secp256k1<secp256k1::VerifyOnly>,
) -> Result<(), ScriptError> {
    let mut witness_eval_stack = Vec::with_capacity(witness_stack.len());
    for element in witness_stack {
        if element.len() > MAX_SCRIPT_ELEMENT_SIZE {
            return Err(ScriptError::PushSize(element.len()));
        }
        push_stack(&mut witness_eval_stack, element)?;
    }

    let witness_context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(
            secp,
            context.validation_context,
            context.precomputed,
        ),
        transaction: context.transaction,
        input_index: context.input_index,
        spent_input: context.spent_input,
        verify_flags: context.verify_flags,
        sig_version: SigVersion::WitnessV0,
    };
    eval_script_internal(&mut witness_eval_stack, exec_script, Some(&witness_context))?;
    if witness_eval_stack.len() != 1 {
        return Err(ScriptError::WitnessCleanStack);
    }
    verify_top_stack_true(&witness_eval_stack)?;

    *stack = witness_eval_stack;
    Ok(())
}

fn single_push_script(script: &ScriptBuf) -> Vec<u8> {
    encode_push_data(script.as_bytes())
}

fn execute_checksig(
    stack: &mut Vec<Vec<u8>>,
    script: &ScriptBuf,
    maybe_context: Option<&LegacyExecutionContext<'_>>,
    verify: bool,
) -> Result<(), ScriptError> {
    let Some(context) = maybe_context else {
        return Err(ScriptError::UnsupportedOpcode(if verify {
            OP_CHECKSIGVERIFY
        } else {
            OP_CHECKSIG
        }));
    };
    if stack.len() < 2 {
        return Err(ScriptError::InvalidStackOperation);
    }

    let public_key = pop_bytes(stack)?;
    let signature = pop_bytes(stack)?;
    let script_code = if context.sig_version == SigVersion::Base {
        remove_signature_from_script(script, &signature)
    } else {
        script.clone()
    };
    let is_valid_signature = context
        .checker
        .verify_ecdsa(
            EcdsaVerificationRequest {
                script_code: &script_code,
                transaction: context.transaction,
                input_index: context.input_index,
                spent_input: context.spent_input,
                signature_bytes: &signature,
                public_key_bytes: &public_key,
                sig_version: context.sig_version,
                require_compressed_pubkey: context.sig_version == SigVersion::WitnessV0
                    && context
                        .verify_flags
                        .contains(ScriptVerifyFlags::WITNESS_PUBKEYTYPE),
            },
            context.verify_flags,
        )
        .map_err(map_signature_error)?;

    if !is_valid_signature
        && context.verify_flags.contains(ScriptVerifyFlags::NULLFAIL)
        && !signature.is_empty()
    {
        return Err(ScriptError::SigNullFail);
    }

    push_stack(stack, encode_bool(is_valid_signature))?;
    if verify {
        if is_valid_signature {
            pop_bytes(stack)?;
        } else {
            return Err(ScriptError::VerifyFailed);
        }
    }

    Ok(())
}

fn execute_checkmultisig(
    stack: &mut Vec<Vec<u8>>,
    script: &ScriptBuf,
    maybe_context: Option<&LegacyExecutionContext<'_>>,
    op_count: &mut usize,
    verify: bool,
) -> Result<(), ScriptError> {
    let Some(context) = maybe_context else {
        return Err(ScriptError::UnsupportedOpcode(if verify {
            OP_CHECKMULTISIGVERIFY
        } else {
            OP_CHECKMULTISIG
        }));
    };
    if stack.is_empty() {
        return Err(ScriptError::InvalidStackOperation);
    }

    let key_count = decode_small_num(stack.last().ok_or(ScriptError::InvalidStackOperation)?)?;
    if key_count > MAX_PUBKEYS_PER_MULTISIG {
        return Err(ScriptError::PubKeyCount);
    }
    *op_count += key_count;
    if *op_count > MAX_OPS_PER_SCRIPT {
        return Err(ScriptError::OpCount);
    }

    let required_stack_items = key_count + 2;
    if stack.len() < required_stack_items {
        return Err(ScriptError::InvalidStackOperation);
    }

    let sig_count_index = stack.len() - key_count - 2;
    let sig_count = decode_small_num(&stack[sig_count_index])?;
    if sig_count > key_count {
        return Err(ScriptError::SigCount);
    }
    if sig_count_index < sig_count + 1 {
        return Err(ScriptError::InvalidStackOperation);
    }

    let dummy_index = sig_count_index - sig_count - 1;
    let dummy = stack[dummy_index].clone();
    let signatures = stack[dummy_index + 1..dummy_index + 1 + sig_count].to_vec();
    let pubkeys = stack[sig_count_index + 1..stack.len() - 1].to_vec();

    let mut script_code = script.clone();
    if context.sig_version == SigVersion::Base {
        for signature in &signatures {
            script_code = remove_signature_from_script(&script_code, signature);
        }
    }

    let mut remaining_pubkeys = pubkeys.iter();
    let mut signatures_iter = signatures.iter();
    let mut maybe_signature = signatures_iter.next();
    let mut matched_all_signatures = true;
    let mut used_signatures = 0_usize;

    while let Some(signature) = maybe_signature {
        let mut matched = false;
        for public_key in remaining_pubkeys.by_ref() {
            let is_valid_signature = context
                .checker
                .verify_ecdsa(
                    EcdsaVerificationRequest {
                        script_code: &script_code,
                        transaction: context.transaction,
                        input_index: context.input_index,
                        spent_input: context.spent_input,
                        signature_bytes: signature,
                        public_key_bytes: public_key,
                        sig_version: context.sig_version,
                        require_compressed_pubkey: context.sig_version == SigVersion::WitnessV0
                            && context
                                .verify_flags
                                .contains(ScriptVerifyFlags::WITNESS_PUBKEYTYPE),
                    },
                    context.verify_flags,
                )
                .map_err(map_signature_error)?;
            if is_valid_signature {
                matched = true;
                used_signatures += 1;
                maybe_signature = signatures_iter.next();
                break;
            }
        }

        if !matched {
            matched_all_signatures = false;
            break;
        }
    }

    let drop_count = key_count + sig_count + 3;
    for _ in 0..drop_count {
        pop_bytes(stack)?;
    }

    if context.verify_flags.contains(ScriptVerifyFlags::NULLDUMMY) && !dummy.is_empty() {
        return Err(ScriptError::SigNullDummy);
    }
    if !matched_all_signatures
        && context.verify_flags.contains(ScriptVerifyFlags::NULLFAIL)
        && signatures.iter().any(|signature| !signature.is_empty())
    {
        return Err(ScriptError::SigNullFail);
    }

    debug_assert!(used_signatures <= sig_count);
    push_stack(stack, encode_bool(matched_all_signatures))?;
    if verify {
        if matched_all_signatures {
            pop_bytes(stack)?;
        } else {
            return Err(ScriptError::VerifyFailed);
        }
    }

    Ok(())
}

pub fn count_legacy_sigops(script: &ScriptBuf) -> Result<usize, ScriptError> {
    count_sigops(script, false)
}

pub fn count_p2sh_sigops(
    script_sig: &ScriptBuf,
    script_pubkey: &ScriptBuf,
) -> Result<usize, ScriptError> {
    if !matches!(
        classify_script_pubkey(script_pubkey),
        ScriptPubKeyType::PayToScriptHash(_)
    ) {
        return Ok(0);
    }
    if !is_push_only(script_sig) {
        return Ok(0);
    }
    let Some(redeem_script) = extract_redeem_script(script_sig) else {
        return Ok(0);
    };
    count_sigops(&redeem_script, true)
}

pub fn count_witness_sigops(
    script_sig: &ScriptBuf,
    script_pubkey: &ScriptBuf,
    witness: &ScriptWitness,
    verify_flags: ScriptVerifyFlags,
) -> Result<usize, ScriptError> {
    if !verify_flags.contains(ScriptVerifyFlags::WITNESS) {
        return Ok(0);
    }

    let script_type = classify_script_pubkey(script_pubkey);
    if let Some(sigops) = witness_sigops_for_type(&script_type, witness)? {
        return Ok(sigops);
    }

    if matches!(script_type, ScriptPubKeyType::PayToScriptHash(_)) && is_push_only(script_sig) {
        let Some(redeem_script) = extract_redeem_script(script_sig) else {
            return Ok(0);
        };
        let redeem_type = classify_script_pubkey(&redeem_script);
        if let Some(sigops) = witness_sigops_for_type(&redeem_type, witness)? {
            return Ok(sigops);
        }
    }

    Ok(0)
}

fn count_sigops(script: &ScriptBuf, accurate: bool) -> Result<usize, ScriptError> {
    let bytes = script.as_bytes();
    let mut pc = 0;
    let mut sigops = 0;
    let mut last_opcode = None;
    while pc < bytes.len() {
        let instruction = read_instruction(bytes, &mut pc)?;
        match instruction.opcode {
            OP_CHECKSIG | OP_CHECKSIGVERIFY => sigops += 1,
            OP_CHECKMULTISIG | OP_CHECKMULTISIGVERIFY => {
                sigops += if accurate {
                    last_opcode
                        .and_then(decode_small_int_opcode)
                        .unwrap_or(MAX_PUBKEYS_PER_MULTISIG)
                } else {
                    MAX_PUBKEYS_PER_MULTISIG
                };
            }
            _ => {}
        }
        last_opcode = Some(instruction.opcode);
    }
    Ok(sigops)
}

fn witness_sigops_for_type(
    script_type: &ScriptPubKeyType,
    witness: &ScriptWitness,
) -> Result<Option<usize>, ScriptError> {
    match script_type {
        ScriptPubKeyType::WitnessV0KeyHash(_) => Ok(Some(1)),
        ScriptPubKeyType::WitnessV0ScriptHash(_) if !witness.stack().is_empty() => {
            let script_bytes = witness
                .stack()
                .last()
                .expect("witness stack is non-empty under the guard above");
            let witness_script = ScriptBuf::from_bytes(script_bytes.clone())
                .map_err(|_| ScriptError::WitnessProgramMismatch)?;
            Ok(Some(count_sigops(&witness_script, true)?))
        }
        _ => Ok(None),
    }
}

fn map_signature_error(error: SignatureError) -> ScriptError {
    match error {
        SignatureError::EmptySignature | SignatureError::IncorrectSignature => {
            ScriptError::VerifyFailed
        }
        SignatureError::InvalidDer => ScriptError::SigDer,
        SignatureError::InvalidHashType(_) => ScriptError::SigHashType,
        SignatureError::InvalidPublicKey => ScriptError::PubKeyType,
        SignatureError::NonCompressedPublicKey => ScriptError::WitnessPubKeyType,
        SignatureError::NonLowS => ScriptError::SigHighS,
        SignatureError::UnsupportedSigVersion => ScriptError::UnsupportedOpcode(OP_CHECKSIG),
    }
}

fn remove_signature_from_script(script: &ScriptBuf, signature: &[u8]) -> ScriptBuf {
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

    ScriptBuf::from_bytes(remaining).expect("filtered script must remain structurally valid")
}

fn encode_push_data(data: &[u8]) -> Vec<u8> {
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

fn decode_small_num(bytes: &[u8]) -> Result<usize, ScriptError> {
    let value = decode_script_num(bytes)?;
    if value < 0 {
        return Err(ScriptError::InvalidStackOperation);
    }
    Ok(value as usize)
}

fn decode_small_int_opcode(opcode: u8) -> Option<usize> {
    match opcode {
        0x51..=0x60 => Some(usize::from(opcode - 0x50)),
        _ => None,
    }
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

    use crate::classify::ScriptPubKeyType;
    use crate::context::{PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags};
    use crate::crypto::{Sha256, hash160};
    use crate::sighash::{SigHashType, SigVersion, legacy_sighash};

    use super::{
        ConditionStack, OP_1, OP_CHECKMULTISIG, OP_CHECKSIG, OP_DUP, OP_ELSE, OP_ENDIF,
        OP_EQUALVERIFY, OP_HASH160, OP_IF, OP_NOTIF, ScriptError, ScriptInputVerificationContext,
        cast_to_bool, count_legacy_sigops, count_p2sh_sigops, count_witness_sigops,
        decode_script_num, decode_small_int_opcode, decode_small_num, encode_push_data,
        encode_script_num, eval_script, execute_checkmultisig, execute_checksig,
        is_disabled_opcode, map_signature_error, remove_signature_from_script, verify_script,
        verify_top_stack_true, verify_witness_program, witness_sigops_for_type,
    };

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn legacy_transaction(txid_byte: u8) -> Transaction {
        Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([txid_byte; 32]),
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
        }
    }

    fn legacy_context(
        script_pubkey: ScriptBuf,
        transaction: &Transaction,
        verify_flags: ScriptVerifyFlags,
    ) -> (
        TransactionInputContext,
        TransactionValidationContext,
        PrecomputedTransactionData,
    ) {
        let spent_input = TransactionInputContext {
            spent_output: crate::context::SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey,
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
            verify_flags,
            consensus_params: crate::context::ConsensusParams::default(),
        };
        let precomputed = validation_context
            .precompute(transaction)
            .expect("precompute");
        (spent_input, validation_context, precomputed)
    }

    fn sign_legacy_script(
        script_code: &ScriptBuf,
        transaction: &Transaction,
        secret_key: &SecretKey,
        sighash_type: SigHashType,
    ) -> Vec<u8> {
        let signing_secp = Secp256k1::new();
        let digest = legacy_sighash(script_code, transaction, 0, sighash_type);
        let message = Message::from_digest(digest.to_byte_array());
        let mut signature = signing_secp.sign_ecdsa(message, secret_key);
        signature.normalize_s();
        let serialized = signature.serialize_der();
        let mut signature_bytes = serialized.as_ref().to_vec();
        signature_bytes.push(sighash_type.raw() as u8);
        signature_bytes
    }

    fn sign_witness_v0_script(
        script_code: &ScriptBuf,
        transaction: &Transaction,
        spent_input: &TransactionInputContext,
        precomputed: &PrecomputedTransactionData,
        secret_key: &SecretKey,
        sighash_type: SigHashType,
    ) -> Vec<u8> {
        let signing_secp = Secp256k1::new();
        let digest = crate::sighash::segwit_v0_sighash(
            script_code,
            transaction,
            0,
            spent_input,
            sighash_type,
            precomputed,
        );
        let message = Message::from_digest(digest.to_byte_array());
        let mut signature = signing_secp.sign_ecdsa(message, secret_key);
        signature.normalize_s();
        let serialized = signature.serialize_der();
        let mut signature_bytes = serialized.as_ref().to_vec();
        signature_bytes.push(sighash_type.raw() as u8);
        signature_bytes
    }

    fn push_only_script(pushes: &[&[u8]]) -> ScriptBuf {
        let mut bytes = Vec::new();
        for push in pushes {
            bytes.push(push.len() as u8);
            bytes.extend_from_slice(push);
        }
        script(&bytes)
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
            (ScriptError::PubKeyCount, "invalid public key count"),
            (ScriptError::PubKeyType, "invalid public key encoding"),
            (
                ScriptError::PushSize(521),
                "push exceeds stack element limit: 521 bytes",
            ),
            (ScriptError::SigCount, "invalid signature count"),
            (ScriptError::SigDer, "invalid DER signature"),
            (ScriptError::SigHashType, "invalid signature hash type"),
            (ScriptError::SigHighS, "non-low-S signature"),
            (
                ScriptError::SigNullDummy,
                "non-null CHECKMULTISIG dummy argument",
            ),
            (ScriptError::SigNullFail, "non-null failing signature"),
            (ScriptError::SigPushOnly, "scriptSig is not push-only"),
            (
                ScriptError::StackOverflow(1001),
                "stack exceeds maximum size: 1001",
            ),
            (ScriptError::TruncatedPushData, "truncated pushdata"),
            (ScriptError::UnbalancedConditional, "unbalanced conditional"),
            (
                ScriptError::UnsupportedOpcode(0xac),
                "unsupported opcode: 0xac",
            ),
            (ScriptError::VerifyFailed, "VERIFY failed"),
            (
                ScriptError::WitnessCleanStack,
                "witness script did not leave a clean stack",
            ),
            (
                ScriptError::WitnessMalleated,
                "witness program has unexpected scriptSig",
            ),
            (
                ScriptError::WitnessMalleatedP2sh,
                "nested witness program scriptSig is malleated",
            ),
            (
                ScriptError::WitnessProgramMismatch,
                "witness program mismatch",
            ),
            (
                ScriptError::WitnessProgramWitnessEmpty,
                "witness program witness stack is empty",
            ),
            (
                ScriptError::WitnessProgramWrongLength,
                "witness program wrong length",
            ),
            (
                ScriptError::WitnessPubKeyType,
                "witness public key must be compressed",
            ),
            (ScriptError::WitnessUnexpected, "unexpected witness data"),
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
        assert_eq!(decode_small_int_opcode(0x51), Some(1));
        assert_eq!(decode_small_int_opcode(0x61), None);
    }

    #[test]
    fn low_level_script_helpers_cover_remaining_direct_paths() {
        assert_eq!(
            verify_top_stack_true(&[Vec::new()]).expect_err("false stack top must fail"),
            ScriptError::EvalFalse
        );

        let untouched = script(&[0x51, 0x51]);
        assert_eq!(
            remove_signature_from_script(&untouched, &[0xaa, 0xbb]),
            untouched
        );

        assert_eq!(
            witness_sigops_for_type(&ScriptPubKeyType::NonStandard, &ScriptWitness::default())
                .expect("helper should succeed"),
            None
        );
    }

    #[test]
    fn condition_stack_and_control_flow_helpers_are_covered() {
        let mut condition_stack = ConditionStack::default();
        assert!(condition_stack.is_empty());
        assert!(condition_stack.all_true());
        assert!(condition_stack.outer_all_true());
        condition_stack.push(true);
        condition_stack.push(false);
        assert!(!condition_stack.all_true());
        assert!(condition_stack.outer_all_true());
        condition_stack.toggle_top().expect("toggle should succeed");
        assert!(condition_stack.all_true());
        assert_eq!(condition_stack.pop(), Some(true));
        assert_eq!(condition_stack.pop(), Some(true));
        assert_eq!(
            condition_stack
                .toggle_top()
                .expect_err("empty toggle should fail"),
            ScriptError::UnbalancedConditional
        );

        let mut stack = Vec::new();
        eval_script(
            &mut stack,
            &script(&[0x00, OP_IF, 0x01, 0x01, OP_ENDIF, OP_1]),
        )
        .expect("inactive branch pushes should be skipped");
        assert_eq!(stack, vec![vec![1_u8]]);

        let mut stack = Vec::new();
        eval_script(
            &mut stack,
            &script(&[OP_1, OP_NOTIF, OP_1, OP_ELSE, OP_1, OP_ENDIF]),
        )
        .expect("NOTIF/ELSE should execute");
        assert_eq!(stack, vec![vec![1_u8]]);

        let mut stack = Vec::new();
        eval_script(
            &mut stack,
            &script(&[0x00, OP_IF, OP_1, OP_IF, OP_ELSE, OP_ENDIF, OP_ENDIF, OP_1]),
        )
        .expect("nested inactive branches should parse and skip execution");
        assert_eq!(stack, vec![vec![1_u8]]);

        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[OP_ENDIF]))
                .expect_err("ENDIF without IF must fail"),
            ScriptError::UnbalancedConditional
        );
        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[OP_1, OP_IF]))
                .expect_err("unterminated IF must fail"),
            ScriptError::UnbalancedConditional
        );
        assert_eq!(
            verify_top_stack_true(&[]).expect_err("empty stack must fail"),
            ScriptError::EvalFalse
        );
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
    fn verify_input_script_rejects_unexpected_witness_data() {
        let mut execution_data = ScriptExecutionData::default();
        let transaction = Transaction::default();
        let validation_context = TransactionValidationContext {
            inputs: vec![],
            spend_height: 0,
            block_time: 0,
            median_time_past: 0,
            verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
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
            verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("unexpected witness data must fail");

        assert_eq!(error, ScriptError::WitnessUnexpected);
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
    fn verify_input_script_accepts_pay_to_pubkey_hash_signatures() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([18_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let public_key_bytes = public_key.serialize();
        let public_key_hash = hash160(&public_key_bytes);
        let mut script_pubkey_bytes = vec![0x76, 0xa9, 20];
        script_pubkey_bytes.extend_from_slice(&public_key_hash);
        script_pubkey_bytes.extend_from_slice(&[0x88, 0xac]);
        let script_pubkey = script(&script_pubkey_bytes);
        let transaction = legacy_transaction(4);
        let (spent_input, validation_context, precomputed) =
            legacy_context(script_pubkey.clone(), &transaction, ScriptVerifyFlags::NONE);
        let signature_bytes =
            sign_legacy_script(&script_pubkey, &transaction, &secret_key, SigHashType::ALL);
        let script_sig = push_only_script(&[&signature_bytes, &public_key_bytes]);
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
    fn verify_input_script_accepts_p2sh_redeem_scripts() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([27_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let public_key_bytes = public_key.serialize();
        let public_key_hash = hash160(&public_key_bytes);
        let mut redeem_script_bytes = vec![0x76, 0xa9, 20];
        redeem_script_bytes.extend_from_slice(&public_key_hash);
        redeem_script_bytes.extend_from_slice(&[0x88, 0xac]);
        let redeem_script = script(&redeem_script_bytes);
        let redeem_hash = hash160(redeem_script.as_bytes());
        let mut script_pubkey_bytes = vec![0xa9, 20];
        script_pubkey_bytes.extend_from_slice(&redeem_hash);
        script_pubkey_bytes.push(0x87);
        let script_pubkey = script(&script_pubkey_bytes);
        let transaction = legacy_transaction(12);
        let (spent_input, validation_context, precomputed) =
            legacy_context(script_pubkey.clone(), &transaction, ScriptVerifyFlags::P2SH);
        let signature_bytes =
            sign_legacy_script(&redeem_script, &transaction, &secret_key, SigHashType::ALL);
        let script_sig = push_only_script(&[
            &signature_bytes,
            &public_key_bytes,
            redeem_script.as_bytes(),
        ]);
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
                verify_flags: ScriptVerifyFlags::P2SH,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );
    }

    #[test]
    fn verify_input_script_enforces_p2sh_push_only() {
        let redeem_script = script(&[0x51]);
        let redeem_hash = hash160(redeem_script.as_bytes());
        let mut script_pubkey_bytes = vec![0xa9, 20];
        script_pubkey_bytes.extend_from_slice(&redeem_hash);
        script_pubkey_bytes.push(0x87);
        let script_pubkey = script(&script_pubkey_bytes);
        let transaction = legacy_transaction(13);
        let (spent_input, validation_context, precomputed) =
            legacy_context(script_pubkey.clone(), &transaction, ScriptVerifyFlags::P2SH);
        let script_sig = script(&[0x51, 0x76, 0x01, 0x51]);
        let mut execution_data = ScriptExecutionData::default();

        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &script_sig,
            script_pubkey: &script_pubkey,
            witness: &ScriptWitness::default(),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("P2SH scriptSig must be push-only");

        assert_eq!(error, ScriptError::SigPushOnly);
    }

    #[test]
    fn verify_input_script_accepts_native_and_nested_witness_v0_programs() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([28_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let public_key_bytes = public_key.serialize();
        let public_key_hash = hash160(&public_key_bytes);
        let p2wpkh_script_pubkey = {
            let mut bytes = vec![0x00, 20];
            bytes.extend_from_slice(&public_key_hash);
            script(&bytes)
        };
        let transaction = legacy_transaction(14);
        let (spent_input, validation_context, precomputed) = legacy_context(
            p2wpkh_script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
        );
        let mut script_code_bytes = vec![OP_DUP, OP_HASH160, 20];
        script_code_bytes.extend_from_slice(&public_key_hash);
        script_code_bytes.extend_from_slice(&[OP_EQUALVERIFY, OP_CHECKSIG]);
        let script_code = script(&script_code_bytes);
        let signature_bytes = sign_witness_v0_script(
            &script_code,
            &transaction,
            &spent_input,
            &precomputed,
            &secret_key,
            SigHashType::ALL,
        );
        let native_witness =
            ScriptWitness::new(vec![signature_bytes.clone(), public_key_bytes.to_vec()]);
        let mut execution_data = ScriptExecutionData::default();

        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &ScriptBuf::default(),
                script_pubkey: &p2wpkh_script_pubkey,
                witness: &native_witness,
                transaction: &transaction,
                input_index: 0,
                spent_input: &spent_input,
                validation_context: &validation_context,
                spent_amount: spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::WITNESS
                    | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );

        let redeem_script = p2wpkh_script_pubkey.clone();
        let redeem_hash = hash160(redeem_script.as_bytes());
        let nested_script_pubkey = {
            let mut bytes = vec![0xa9, 20];
            bytes.extend_from_slice(&redeem_hash);
            bytes.push(0x87);
            script(&bytes)
        };
        let (nested_spent_input, nested_validation_context, nested_precomputed) = legacy_context(
            nested_script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
        );
        let nested_script_sig = push_only_script(&[redeem_script.as_bytes()]);

        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &nested_script_sig,
                script_pubkey: &nested_script_pubkey,
                witness: &native_witness,
                transaction: &transaction,
                input_index: 0,
                spent_input: &nested_spent_input,
                validation_context: &nested_validation_context,
                spent_amount: nested_spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::WITNESS
                    | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
                precomputed: &nested_precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );
    }

    #[test]
    fn verify_input_script_accepts_witness_v0_multisig() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([30_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let witness_script = {
            let mut bytes = vec![0x51, 33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.push(0x51);
            bytes.push(OP_CHECKMULTISIG);
            script(&bytes)
        };
        let witness_hash = Sha256::digest(witness_script.as_bytes());
        let script_pubkey = {
            let mut bytes = vec![0x00, 32];
            bytes.extend_from_slice(&witness_hash);
            script(&bytes)
        };
        let transaction = legacy_transaction(31);
        let (spent_input, validation_context, precomputed) = legacy_context(
            script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
        );
        let signature_bytes = sign_witness_v0_script(
            &witness_script,
            &transaction,
            &spent_input,
            &precomputed,
            &secret_key,
            SigHashType::ALL,
        );
        let witness = ScriptWitness::new(vec![
            Vec::new(),
            signature_bytes,
            witness_script.as_bytes().to_vec(),
        ]);
        let mut execution_data = ScriptExecutionData::default();

        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &ScriptBuf::default(),
                script_pubkey: &script_pubkey,
                witness: &witness,
                transaction: &transaction,
                input_index: 0,
                spent_input: &spent_input,
                validation_context: &validation_context,
                spent_amount: spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::WITNESS
                    | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );
    }

    #[test]
    fn verify_input_script_enforces_witness_malleation_and_pubkey_rules() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([29_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let public_key_hash = hash160(&public_key.serialize());
        let script_pubkey = {
            let mut bytes = vec![0x00, 20];
            bytes.extend_from_slice(&public_key_hash);
            script(&bytes)
        };
        let transaction = legacy_transaction(15);
        let (spent_input, validation_context, precomputed) = legacy_context(
            script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
        );
        let mut script_code_bytes = vec![OP_DUP, OP_HASH160, 20];
        script_code_bytes.extend_from_slice(&public_key_hash);
        script_code_bytes.extend_from_slice(&[OP_EQUALVERIFY, OP_CHECKSIG]);
        let script_code = script(&script_code_bytes);
        let signature_bytes = sign_witness_v0_script(
            &script_code,
            &transaction,
            &spent_input,
            &precomputed,
            &secret_key,
            SigHashType::ALL,
        );
        let witness = ScriptWitness::new(vec![
            signature_bytes,
            public_key.serialize_uncompressed().to_vec(),
        ]);
        let mut execution_data = ScriptExecutionData::default();

        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &script(&[0x51]),
            script_pubkey: &script_pubkey,
            witness: &witness,
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("bare witness scriptSig must be empty");
        assert_eq!(error, ScriptError::WitnessMalleated);

        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &ScriptBuf::default(),
            script_pubkey: &script_pubkey,
            witness: &witness,
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("uncompressed witness pubkeys must fail");
        assert_eq!(error, ScriptError::VerifyFailed);

        let witness_script = {
            let mut bytes = vec![65];
            bytes.extend_from_slice(&public_key.serialize_uncompressed());
            bytes.push(OP_CHECKSIG);
            script(&bytes)
        };
        let witness_hash = Sha256::digest(witness_script.as_bytes());
        let p2wsh_script_pubkey = {
            let mut bytes = vec![0x00, 32];
            bytes.extend_from_slice(&witness_hash);
            script(&bytes)
        };
        let (wsh_spent_input, wsh_validation_context, wsh_precomputed) = legacy_context(
            p2wsh_script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
        );
        let witness_signature = sign_witness_v0_script(
            &witness_script,
            &transaction,
            &wsh_spent_input,
            &wsh_precomputed,
            &secret_key,
            SigHashType::ALL,
        );
        let p2wsh_witness =
            ScriptWitness::new(vec![witness_signature, witness_script.as_bytes().to_vec()]);
        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &ScriptBuf::default(),
            script_pubkey: &p2wsh_script_pubkey,
            witness: &p2wsh_witness,
            transaction: &transaction,
            input_index: 0,
            spent_input: &wsh_spent_input,
            validation_context: &wsh_validation_context,
            spent_amount: wsh_spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::WITNESS_PUBKEYTYPE,
            precomputed: &wsh_precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("uncompressed pubkeys in witness scripts must fail");
        assert_eq!(error, ScriptError::WitnessPubKeyType);
    }

    #[test]
    fn verify_input_script_handles_witness_program_mismatch_minimalif_and_cleanstack() {
        let witness_script = script(&[OP_IF, OP_1, OP_ELSE, 0x00, OP_ENDIF]);
        let witness_hash = Sha256::digest(witness_script.as_bytes());
        let script_pubkey = {
            let mut bytes = vec![0x00, 32];
            bytes.extend_from_slice(&witness_hash);
            script(&bytes)
        };
        let transaction = legacy_transaction(16);
        let (spent_input, validation_context, precomputed) = legacy_context(
            script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::MINIMALIF,
        );
        let mut execution_data = ScriptExecutionData::default();

        let mismatch_witness = ScriptWitness::new(vec![vec![1_u8], vec![OP_1]]);
        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &ScriptBuf::default(),
            script_pubkey: &script_pubkey,
            witness: &mismatch_witness,
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::MINIMALIF,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("witness script hash mismatch must fail");
        assert_eq!(error, ScriptError::WitnessProgramMismatch);

        let minimalif_witness =
            ScriptWitness::new(vec![vec![2_u8], witness_script.as_bytes().to_vec()]);
        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &ScriptBuf::default(),
            script_pubkey: &script_pubkey,
            witness: &minimalif_witness,
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::MINIMALIF,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("MINIMALIF witness input must fail");
        assert_eq!(error, ScriptError::VerifyFailed);

        let cleanstack_script = script(&[OP_1, OP_1]);
        let cleanstack_hash = Sha256::digest(cleanstack_script.as_bytes());
        let cleanstack_script_pubkey = {
            let mut bytes = vec![0x00, 32];
            bytes.extend_from_slice(&cleanstack_hash);
            script(&bytes)
        };
        let (cleanstack_spent_input, cleanstack_validation_context, cleanstack_precomputed) =
            legacy_context(
                cleanstack_script_pubkey.clone(),
                &transaction,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::WITNESS
                    | ScriptVerifyFlags::CLEANSTACK,
            );
        let cleanstack_witness = ScriptWitness::new(vec![cleanstack_script.as_bytes().to_vec()]);
        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &ScriptBuf::default(),
            script_pubkey: &cleanstack_script_pubkey,
            witness: &cleanstack_witness,
            transaction: &transaction,
            input_index: 0,
            spent_input: &cleanstack_spent_input,
            validation_context: &cleanstack_validation_context,
            spent_amount: cleanstack_spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::CLEANSTACK,
            precomputed: &cleanstack_precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("witness scripts must leave a clean stack");
        assert_eq!(error, ScriptError::WitnessCleanStack);
    }

    #[test]
    fn verify_input_script_enforces_sigpushonly() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([22_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let script_pubkey = {
            let mut bytes = vec![33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.push(0xac);
            script(&bytes)
        };
        let transaction = legacy_transaction(5);
        let (spent_input, validation_context, precomputed) = legacy_context(
            script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::SIGPUSHONLY,
        );
        let signature_bytes =
            sign_legacy_script(&script_pubkey, &transaction, &secret_key, SigHashType::ALL);
        let mut script_sig_bytes = vec![signature_bytes.len() as u8];
        script_sig_bytes.extend_from_slice(&signature_bytes);
        script_sig_bytes.push(0x76);
        let script_sig = script(&script_sig_bytes);
        let mut execution_data = ScriptExecutionData::default();

        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &script_sig,
            script_pubkey: &script_pubkey,
            witness: &ScriptWitness::default(),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::SIGPUSHONLY,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("non-push scriptSig must fail");

        assert_eq!(error, ScriptError::SigPushOnly);
    }

    #[test]
    fn verify_input_script_enforces_nullfail_for_failed_checksig() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([23_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let script_pubkey = {
            let mut bytes = vec![33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.extend_from_slice(&[0xac, 0x91]);
            script(&bytes)
        };
        let transaction = legacy_transaction(6);
        let (spent_input, validation_context, precomputed) = legacy_context(
            script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::NULLFAIL,
        );
        let script_sig = push_only_script(&[&[0x01, 0x02]]);
        let mut execution_data = ScriptExecutionData::default();

        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &script_sig,
            script_pubkey: &script_pubkey,
            witness: &ScriptWitness::default(),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::NULLFAIL,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("NULLFAIL should reject non-empty failing signatures");

        assert_eq!(error, ScriptError::SigNullFail);
    }

    #[test]
    fn verify_input_script_enforces_nulldummy_for_multisig() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([24_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let script_pubkey = {
            let mut bytes = vec![0x51, 33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.push(0x51);
            bytes.push(0xae);
            script(&bytes)
        };
        let transaction = legacy_transaction(7);
        let (spent_input, validation_context, precomputed) = legacy_context(
            script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::NULLDUMMY,
        );
        let signature_bytes =
            sign_legacy_script(&script_pubkey, &transaction, &secret_key, SigHashType::ALL);
        let script_sig = push_only_script(&[&[0x01], &signature_bytes]);
        let mut execution_data = ScriptExecutionData::default();

        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &script_sig,
            script_pubkey: &script_pubkey,
            witness: &ScriptWitness::default(),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::NULLDUMMY,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("NULLDUMMY should reject non-zero dummy arguments");

        assert_eq!(error, ScriptError::SigNullDummy);
    }

    #[test]
    fn verify_input_script_supports_checksigverify_and_checkmultisigverify() {
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([25_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);

        let checksigverify_script = {
            let mut bytes = vec![33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.extend_from_slice(&[0xad, 0x51]);
            script(&bytes)
        };
        let checksigverify_transaction = legacy_transaction(8);
        let (checksigverify_input, checksigverify_context, checksigverify_precomputed) =
            legacy_context(
                checksigverify_script.clone(),
                &checksigverify_transaction,
                ScriptVerifyFlags::NONE,
            );
        let checksigverify_signature = sign_legacy_script(
            &checksigverify_script,
            &checksigverify_transaction,
            &secret_key,
            SigHashType::ALL,
        );
        let checksigverify_script_sig = push_only_script(&[&checksigverify_signature]);
        let mut execution_data = ScriptExecutionData::default();

        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &checksigverify_script_sig,
                script_pubkey: &checksigverify_script,
                witness: &ScriptWitness::default(),
                transaction: &checksigverify_transaction,
                input_index: 0,
                spent_input: &checksigverify_input,
                validation_context: &checksigverify_context,
                spent_amount: checksigverify_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::NONE,
                precomputed: &checksigverify_precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );

        let checkmultisigverify_script = {
            let mut bytes = vec![0x51, 33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.extend_from_slice(&[0x51, 0xaf, 0x51]);
            script(&bytes)
        };
        let checkmultisigverify_transaction = legacy_transaction(9);
        let (
            checkmultisigverify_input,
            checkmultisigverify_context,
            checkmultisigverify_precomputed,
        ) = legacy_context(
            checkmultisigverify_script.clone(),
            &checkmultisigverify_transaction,
            ScriptVerifyFlags::NONE,
        );
        let checkmultisigverify_signature = sign_legacy_script(
            &checkmultisigverify_script,
            &checkmultisigverify_transaction,
            &secret_key,
            SigHashType::ALL,
        );
        let checkmultisigverify_script_sig =
            push_only_script(&[&[], &checkmultisigverify_signature]);

        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &checkmultisigverify_script_sig,
                script_pubkey: &checkmultisigverify_script,
                witness: &ScriptWitness::default(),
                transaction: &checkmultisigverify_transaction,
                input_index: 0,
                spent_input: &checkmultisigverify_input,
                validation_context: &checkmultisigverify_context,
                spent_amount: checkmultisigverify_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::NONE,
                precomputed: &checkmultisigverify_precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );
    }

    #[test]
    fn legacy_helper_error_paths_are_covered() {
        let transaction = legacy_transaction(10);
        let (spent_input, validation_context, precomputed) =
            legacy_context(script(&[0x51]), &transaction, ScriptVerifyFlags::NONE);
        let secp = Secp256k1::verification_only();
        let execution_context = super::LegacyExecutionContext {
            checker: crate::signature::TransactionSignatureChecker::new(
                &secp,
                &validation_context,
                &precomputed,
            ),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            verify_flags: ScriptVerifyFlags::NONE,
            sig_version: SigVersion::Base,
        };
        let mut execution_data = ScriptExecutionData::default();

        assert_eq!(
            eval_script(&mut Vec::new(), &script(&[0xa6])).expect_err("RIPEMD160 is deferred"),
            ScriptError::UnsupportedOpcode(0xa6)
        );
        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &ScriptBuf::default(),
                script_pubkey: &ScriptBuf::default(),
                witness: &ScriptWitness::default(),
                transaction: &transaction,
                input_index: 0,
                spent_input: &spent_input,
                validation_context: &validation_context,
                spent_amount: Amount::from_sats(50).expect("valid amount"),
                verify_flags: ScriptVerifyFlags::NONE,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            })
            .expect_err("empty scripts should fail"),
            ScriptError::EvalFalse
        );

        assert_eq!(
            execute_checksig(&mut Vec::new(), &script(&[0xad]), None, true)
                .expect_err("missing checker must fail"),
            ScriptError::UnsupportedOpcode(0xad)
        );
        assert_eq!(
            execute_checkmultisig(&mut Vec::new(), &script(&[0xaf]), None, &mut 0, true)
                .expect_err("missing checker must fail"),
            ScriptError::UnsupportedOpcode(0xaf)
        );
        assert_eq!(
            execute_checkmultisig(&mut Vec::new(), &script(&[0xae]), None, &mut 0, false)
                .expect_err("missing checker must fail"),
            ScriptError::UnsupportedOpcode(0xae)
        );
        assert_eq!(
            execute_checksig(
                &mut vec![vec![1_u8]],
                &script(&[0xac]),
                Some(&execution_context),
                false,
            )
            .expect_err("stack underflow must fail"),
            ScriptError::InvalidStackOperation
        );
        assert_eq!(
            execute_checkmultisig(
                &mut Vec::new(),
                &script(&[0xae]),
                Some(&execution_context),
                &mut 0,
                false,
            )
            .expect_err("empty multisig stack must fail"),
            ScriptError::InvalidStackOperation
        );
        assert_eq!(
            execute_checkmultisig(
                &mut vec![vec![21]],
                &script(&[0xae]),
                Some(&execution_context),
                &mut 0,
                false,
            )
            .expect_err("too many pubkeys must fail"),
            ScriptError::PubKeyCount
        );
        let mut op_count = super::MAX_OPS_PER_SCRIPT;
        assert_eq!(
            execute_checkmultisig(
                &mut vec![vec![1]],
                &script(&[0xae]),
                Some(&execution_context),
                &mut op_count,
                false,
            )
            .expect_err("sigop overflow must fail"),
            ScriptError::OpCount
        );
        assert_eq!(
            execute_checkmultisig(
                &mut vec![vec![1]],
                &script(&[0xae]),
                Some(&execution_context),
                &mut 0,
                false,
            )
            .expect_err("insufficient stack must fail"),
            ScriptError::InvalidStackOperation
        );
        assert_eq!(
            execute_checkmultisig(
                &mut vec![vec![2], vec![0x21, 0x01], vec![1]],
                &script(&[0xae]),
                Some(&execution_context),
                &mut 0,
                false,
            )
            .expect_err("too many signatures must fail"),
            ScriptError::SigCount
        );
        let signing_secp = Secp256k1::new();
        let secret_key = SecretKey::from_byte_array([26_u8; 32]).expect("secret key");
        let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
        let checksigverify_script = {
            let mut bytes = vec![33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.push(0xad);
            script(&bytes)
        };
        assert_eq!(
            execute_checksig(
                &mut vec![vec![0x01, 0x02], public_key.serialize().to_vec()],
                &checksigverify_script,
                Some(&execution_context),
                true,
            )
            .expect_err("failed checksigverify should fail"),
            ScriptError::VerifyFailed
        );
        let checkmultisigverify_script = {
            let mut bytes = vec![0x51, 33];
            bytes.extend_from_slice(&public_key.serialize());
            bytes.push(0x51);
            bytes.push(0xaf);
            script(&bytes)
        };
        assert_eq!(
            execute_checkmultisig(
                &mut vec![
                    Vec::new(),
                    vec![0x01, 0x02],
                    vec![0x01],
                    public_key.serialize().to_vec(),
                    vec![0x01]
                ],
                &checkmultisigverify_script,
                Some(&execution_context),
                &mut 0,
                true,
            )
            .expect_err("failed checkmultisigverify should fail"),
            ScriptError::VerifyFailed
        );
        let nullfail_checker = crate::signature::TransactionSignatureChecker::new(
            &secp,
            &validation_context,
            &precomputed,
        );
        let nullfail_multisig_context = super::LegacyExecutionContext {
            checker: nullfail_checker,
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            verify_flags: ScriptVerifyFlags::NULLFAIL,
            sig_version: SigVersion::Base,
        };
        assert_eq!(
            execute_checkmultisig(
                &mut vec![
                    Vec::new(),
                    vec![0x01, 0x02],
                    vec![0x01],
                    public_key.serialize().to_vec(),
                    vec![0x01]
                ],
                &checkmultisigverify_script,
                Some(&nullfail_multisig_context),
                &mut 0,
                false,
            )
            .expect_err("NULLFAIL should reject failing multisig signatures"),
            ScriptError::SigNullFail
        );

        assert_eq!(
            decode_small_num(&[0x81]).expect_err("negative values are invalid counts"),
            ScriptError::InvalidStackOperation
        );
        assert_eq!(
            map_signature_error(crate::signature::SignatureError::EmptySignature),
            ScriptError::VerifyFailed
        );
        assert_eq!(
            map_signature_error(crate::signature::SignatureError::IncorrectSignature),
            ScriptError::VerifyFailed
        );
        assert_eq!(
            map_signature_error(crate::signature::SignatureError::InvalidDer),
            ScriptError::SigDer
        );
        assert_eq!(
            map_signature_error(crate::signature::SignatureError::InvalidHashType(4)),
            ScriptError::SigHashType
        );
        assert_eq!(
            map_signature_error(crate::signature::SignatureError::InvalidPublicKey),
            ScriptError::PubKeyType
        );
        assert_eq!(
            map_signature_error(crate::signature::SignatureError::NonCompressedPublicKey),
            ScriptError::WitnessPubKeyType
        );
        assert_eq!(
            map_signature_error(crate::signature::SignatureError::NonLowS),
            ScriptError::SigHighS
        );
        assert_eq!(
            map_signature_error(crate::signature::SignatureError::UnsupportedSigVersion),
            ScriptError::UnsupportedOpcode(0xac)
        );

        assert_eq!(
            remove_signature_from_script(&script(&[0x51]), &[]),
            script(&[0x51])
        );
        let signature = vec![0xaa; 76];
        let encoded_signature = encode_push_data(&signature);
        let mut script_bytes = encoded_signature.clone();
        script_bytes.extend_from_slice(&[0x51]);
        assert_eq!(
            remove_signature_from_script(&script(&script_bytes), &signature),
            script(&[0x51])
        );

        let pushdata1 = vec![0_u8; 0x4c];
        let pushdata2 = vec![0_u8; 0x100];
        let pushdata4 = vec![0_u8; 0x1_0000];
        assert_eq!(encode_push_data(&pushdata1)[0], 0x4c);
        assert_eq!(encode_push_data(&pushdata2)[0], 0x4d);
        assert_eq!(encode_push_data(&pushdata4)[0], 0x4e);
    }

    #[test]
    fn witness_and_sigop_helpers_are_covered() {
        let transaction = legacy_transaction(11);
        let unknown_witness_script_pubkey = script(&[OP_1, 0x02, 0xaa, 0xbb]);
        let (spent_input, validation_context, precomputed) = legacy_context(
            unknown_witness_script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
        );
        let mut execution_data = ScriptExecutionData::default();
        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &ScriptBuf::default(),
                script_pubkey: &unknown_witness_script_pubkey,
                witness: &ScriptWitness::default(),
                transaction: &transaction,
                input_index: 0,
                spent_input: &spent_input,
                validation_context: &validation_context,
                spent_amount: spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
                precomputed: &precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );

        let nested_unknown_redeem = unknown_witness_script_pubkey.clone();
        let nested_unknown_hash = hash160(nested_unknown_redeem.as_bytes());
        let nested_unknown_script_pubkey = {
            let mut bytes = vec![0xa9, 20];
            bytes.extend_from_slice(&nested_unknown_hash);
            bytes.push(0x87);
            script(&bytes)
        };
        let (nested_spent_input, nested_validation_context, nested_precomputed) = legacy_context(
            nested_unknown_script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
        );
        let nested_script_sig = push_only_script(&[nested_unknown_redeem.as_bytes()]);
        assert_eq!(
            super::verify_input_script(ScriptInputVerificationContext {
                script_sig: &nested_script_sig,
                script_pubkey: &nested_unknown_script_pubkey,
                witness: &ScriptWitness::default(),
                transaction: &transaction,
                input_index: 0,
                spent_input: &nested_spent_input,
                validation_context: &nested_validation_context,
                spent_amount: nested_spent_input.spent_output.value,
                verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
                precomputed: &nested_precomputed,
                execution_data: &mut execution_data,
            }),
            Ok(())
        );

        let secp = Secp256k1::verification_only();
        let mut witness_stack = Vec::new();
        assert_eq!(
            verify_witness_program(
                &mut witness_stack,
                &ScriptInputVerificationContext {
                    script_sig: &ScriptBuf::default(),
                    script_pubkey: &ScriptBuf::default(),
                    witness: &ScriptWitness::default(),
                    transaction: &transaction,
                    input_index: 0,
                    spent_input: &spent_input,
                    validation_context: &validation_context,
                    spent_amount: spent_input.spent_output.value,
                    verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
                    precomputed: &precomputed,
                    execution_data: &mut execution_data,
                },
                &ScriptPubKeyType::WitnessV1Taproot([1_u8; 32]),
                false,
                &secp,
            ),
            Err(ScriptError::WitnessProgramWrongLength)
        );
        assert_eq!(
            verify_witness_program(
                &mut witness_stack,
                &ScriptInputVerificationContext {
                    script_sig: &ScriptBuf::default(),
                    script_pubkey: &ScriptBuf::default(),
                    witness: &ScriptWitness::default(),
                    transaction: &transaction,
                    input_index: 0,
                    spent_input: &spent_input,
                    validation_context: &validation_context,
                    spent_amount: spent_input.spent_output.value,
                    verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
                    precomputed: &precomputed,
                    execution_data: &mut execution_data,
                },
                &ScriptPubKeyType::WitnessUnknown {
                    version: 2,
                    program: vec![0xaa, 0xbb],
                },
                false,
                &secp,
            ),
            Ok(())
        );
        assert_eq!(witness_stack, vec![vec![1_u8]]);
        assert_eq!(
            verify_witness_program(
                &mut witness_stack,
                &ScriptInputVerificationContext {
                    script_sig: &ScriptBuf::default(),
                    script_pubkey: &ScriptBuf::default(),
                    witness: &ScriptWitness::default(),
                    transaction: &transaction,
                    input_index: 0,
                    spent_input: &spent_input,
                    validation_context: &validation_context,
                    spent_amount: spent_input.spent_output.value,
                    verify_flags: ScriptVerifyFlags::P2SH
                        | ScriptVerifyFlags::WITNESS
                        | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM,
                    precomputed: &precomputed,
                    execution_data: &mut execution_data,
                },
                &ScriptPubKeyType::WitnessUnknown {
                    version: 2,
                    program: vec![0xaa, 0xbb],
                },
                false,
                &secp,
            ),
            Err(ScriptError::UnsupportedOpcode(0x92))
        );
        assert_eq!(
            verify_witness_program(
                &mut witness_stack,
                &ScriptInputVerificationContext {
                    script_sig: &ScriptBuf::default(),
                    script_pubkey: &ScriptBuf::default(),
                    witness: &ScriptWitness::default(),
                    transaction: &transaction,
                    input_index: 0,
                    spent_input: &spent_input,
                    validation_context: &validation_context,
                    spent_amount: spent_input.spent_output.value,
                    verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
                    precomputed: &precomputed,
                    execution_data: &mut execution_data,
                },
                &ScriptPubKeyType::WitnessUnknown {
                    version: 2,
                    program: vec![0xaa, 0xbb],
                },
                true,
                &secp,
            ),
            Ok(())
        );
        assert_eq!(
            verify_witness_program(
                &mut witness_stack,
                &ScriptInputVerificationContext {
                    script_sig: &ScriptBuf::default(),
                    script_pubkey: &ScriptBuf::default(),
                    witness: &ScriptWitness::default(),
                    transaction: &transaction,
                    input_index: 0,
                    spent_input: &spent_input,
                    validation_context: &validation_context,
                    spent_amount: spent_input.spent_output.value,
                    verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
                    precomputed: &precomputed,
                    execution_data: &mut execution_data,
                },
                &ScriptPubKeyType::NonStandard,
                false,
                &secp,
            ),
            Err(ScriptError::WitnessProgramWrongLength)
        );
        let short_p2wpkh_context = ScriptInputVerificationContext {
            script_sig: &ScriptBuf::default(),
            script_pubkey: &ScriptBuf::default(),
            witness: &ScriptWitness::new(vec![vec![1_u8]]),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        };
        assert_eq!(
            verify_witness_program(
                &mut witness_stack,
                &short_p2wpkh_context,
                &ScriptPubKeyType::WitnessV0KeyHash([1_u8; 20]),
                false,
                &secp,
            ),
            Err(ScriptError::WitnessProgramMismatch)
        );
        let empty_p2wsh_context = ScriptInputVerificationContext {
            script_sig: &ScriptBuf::default(),
            script_pubkey: &ScriptBuf::default(),
            witness: &ScriptWitness::default(),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        };
        assert_eq!(
            verify_witness_program(
                &mut witness_stack,
                &empty_p2wsh_context,
                &ScriptPubKeyType::WitnessV0ScriptHash([1_u8; 32]),
                false,
                &secp,
            ),
            Err(ScriptError::WitnessProgramWitnessEmpty)
        );
        let oversized_witness_context = ScriptInputVerificationContext {
            script_sig: &ScriptBuf::default(),
            script_pubkey: &ScriptBuf::default(),
            witness: &ScriptWitness::new(vec![
                vec![0_u8; 521],
                script(&[OP_1]).as_bytes().to_vec(),
            ]),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &validation_context,
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        };
        assert_eq!(
            verify_witness_program(
                &mut witness_stack,
                &oversized_witness_context,
                &ScriptPubKeyType::WitnessV0ScriptHash(Sha256::digest(script(&[OP_1]).as_bytes())),
                false,
                &secp,
            ),
            Err(ScriptError::PushSize(521))
        );
        let cleanstack_error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &script(&[0x51]),
            script_pubkey: &script(&[0x51]),
            witness: &ScriptWitness::default(),
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &TransactionValidationContext {
                verify_flags: ScriptVerifyFlags::CLEANSTACK,
                ..validation_context.clone()
            },
            spent_amount: spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::CLEANSTACK,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("CLEANSTACK without a clean stack must fail");
        assert_eq!(cleanstack_error, ScriptError::WitnessCleanStack);

        assert_eq!(
            count_p2sh_sigops(&ScriptBuf::default(), &script(&[0x51])).unwrap(),
            0
        );
        assert_eq!(
            count_p2sh_sigops(&script(&[0x51, 0x76]), &nested_unknown_script_pubkey).unwrap(),
            0
        );
        let accurate_redeem = script(&[0x52, OP_CHECKMULTISIG]);
        let accurate_script_pubkey = {
            let redeem_hash = hash160(accurate_redeem.as_bytes());
            let mut bytes = vec![0xa9, 20];
            bytes.extend_from_slice(&redeem_hash);
            bytes.push(0x87);
            script(&bytes)
        };
        assert_eq!(
            count_p2sh_sigops(&ScriptBuf::default(), &accurate_script_pubkey).unwrap(),
            0
        );
        let accurate_script_sig = push_only_script(&[accurate_redeem.as_bytes()]);
        assert_eq!(
            count_p2sh_sigops(&accurate_script_sig, &accurate_script_pubkey).unwrap(),
            2
        );

        assert_eq!(
            count_witness_sigops(
                &ScriptBuf::default(),
                &script(&[0x51]),
                &ScriptWitness::default(),
                ScriptVerifyFlags::NONE,
            )
            .unwrap(),
            0
        );
        let p2wpkh = {
            let mut bytes = vec![0x00, 20];
            bytes.extend_from_slice(&[2_u8; 20]);
            script(&bytes)
        };
        assert_eq!(
            count_witness_sigops(
                &ScriptBuf::default(),
                &p2wpkh,
                &ScriptWitness::new(vec![vec![1_u8], vec![2_u8]]),
                ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            )
            .unwrap(),
            1
        );
        let witness_script = script(&[0x52, OP_CHECKMULTISIG]);
        let witness_hash = Sha256::digest(witness_script.as_bytes());
        let p2wsh = {
            let mut bytes = vec![0x00, 32];
            bytes.extend_from_slice(&witness_hash);
            script(&bytes)
        };
        assert_eq!(
            count_witness_sigops(
                &ScriptBuf::default(),
                &p2wsh,
                &ScriptWitness::new(vec![witness_script.as_bytes().to_vec()]),
                ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            )
            .unwrap(),
            2
        );
        assert_eq!(
            count_witness_sigops(
                &ScriptBuf::default(),
                &script(&[0x51]),
                &ScriptWitness::default(),
                ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            )
            .unwrap(),
            0
        );
        let nested_witness_hash = hash160(p2wsh.as_bytes());
        let nested_witness_script_pubkey = {
            let mut bytes = vec![0xa9, 20];
            bytes.extend_from_slice(&nested_witness_hash);
            bytes.push(0x87);
            script(&bytes)
        };
        let nested_witness_script_sig = push_only_script(&[p2wsh.as_bytes()]);
        assert_eq!(
            count_witness_sigops(
                &nested_witness_script_sig,
                &nested_witness_script_pubkey,
                &ScriptWitness::new(vec![witness_script.as_bytes().to_vec()]),
                ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            )
            .unwrap(),
            2
        );
        assert_eq!(
            count_witness_sigops(
                &nested_script_sig,
                &nested_unknown_script_pubkey,
                &ScriptWitness::default(),
                ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            )
            .unwrap(),
            0
        );
        let nested_malleated_script_pubkey = {
            let redeem_hash = hash160(p2wsh.as_bytes());
            let mut bytes = vec![0xa9, 20];
            bytes.extend_from_slice(&redeem_hash);
            bytes.push(0x87);
            script(&bytes)
        };
        let (malleated_spent_input, malleated_validation_context, malleated_precomputed) =
            legacy_context(
                nested_malleated_script_pubkey.clone(),
                &transaction,
                ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            );
        let malleated_script_sig = push_only_script(&[&[], p2wsh.as_bytes()]);
        let nested_witness = ScriptWitness::new(vec![witness_script.as_bytes().to_vec()]);
        let error = super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &malleated_script_sig,
            script_pubkey: &nested_malleated_script_pubkey,
            witness: &nested_witness,
            transaction: &transaction,
            input_index: 0,
            spent_input: &malleated_spent_input,
            validation_context: &malleated_validation_context,
            spent_amount: malleated_spent_input.spent_output.value,
            verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            precomputed: &malleated_precomputed,
            execution_data: &mut execution_data,
        })
        .expect_err("nested witness scriptSig must be an exact single push");
        assert_eq!(error, ScriptError::WitnessMalleatedP2sh);
        assert_eq!(
            count_witness_sigops(
                &ScriptBuf::default(),
                &nested_witness_script_pubkey,
                &ScriptWitness::default(),
                ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            )
            .unwrap(),
            0
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
            Err(ScriptError::InvalidStackOperation)
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
            Err(ScriptError::EvalFalse)
        );
    }
}
