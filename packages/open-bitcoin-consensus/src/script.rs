use core::fmt;

use open_bitcoin_primitives::{
    Amount, Hash32, MAX_OPS_PER_SCRIPT, MAX_SCRIPT_ELEMENT_SIZE, ScriptBuf, ScriptWitness,
    Transaction,
};
use secp256k1::{Parity, Scalar, Secp256k1, XOnlyPublicKey};

use crate::classify::{
    ScriptPubKeyType, classify_script_pubkey, extract_redeem_script, is_push_only,
};
use crate::context::{
    PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags, TransactionInputContext,
    TransactionValidationContext,
};
use crate::crypto::{Sha256, double_sha256, hash160};
use crate::sighash::{SigVersion, taproot_tagged_hash};
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
const OP_CODESEPARATOR: u8 = 0xab;
const OP_CHECKSIG: u8 = 0xac;
const OP_CHECKSIGVERIFY: u8 = 0xad;
const OP_CHECKMULTISIG: u8 = 0xae;
const OP_CHECKMULTISIGVERIFY: u8 = 0xaf;
const OP_CHECKSIGADD: u8 = 0xba;
const OP_RETURN: u8 = 0x6a;
const MAX_PUBKEYS_PER_MULTISIG: usize = 20;
const TAPROOT_LEAF_MASK: u8 = 0xfe;
const TAPROOT_LEAF_TAPSCRIPT: u8 = 0xc0;
const TAPROOT_CONTROL_BASE_SIZE: usize = 33;
const TAPROOT_CONTROL_NODE_SIZE: usize = 32;
const TAPROOT_CONTROL_MAX_NODE_COUNT: usize = 128;
const TAPROOT_CONTROL_MAX_SIZE: usize =
    TAPROOT_CONTROL_BASE_SIZE + TAPROOT_CONTROL_NODE_SIZE * TAPROOT_CONTROL_MAX_NODE_COUNT;
const ANNEX_TAG: u8 = 0x50;
const VALIDATION_WEIGHT_PER_SIGOP_PASSED: i64 = 50;
const VALIDATION_WEIGHT_OFFSET: i64 = 50;

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
    eval_script_internal(stack, script, None, None)
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
    mut maybe_execution_data: Option<&mut ScriptExecutionData>,
) -> Result<(), ScriptError> {
    let bytes = script.as_bytes();
    let mut pc = 0;
    let mut op_count = 0;
    let mut condition_stack = ConditionStack::default();
    let mut opcode_position = 0_u32;

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
            opcode_position = opcode_position.saturating_add(1);
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
            opcode_position = opcode_position.saturating_add(1);
            continue;
        }
        if instruction.opcode == OP_ELSE {
            if !condition_stack.outer_all_true() {
                opcode_position = opcode_position.saturating_add(1);
                continue;
            }
            condition_stack.toggle_top()?;
            opcode_position = opcode_position.saturating_add(1);
            continue;
        }
        if instruction.opcode == OP_ENDIF {
            if condition_stack.pop().is_none() {
                return Err(ScriptError::UnbalancedConditional);
            }
            opcode_position = opcode_position.saturating_add(1);
            continue;
        }
        if !should_execute {
            opcode_position = opcode_position.saturating_add(1);
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
            OP_CODESEPARATOR => {
                if let Some(execution_data) = maybe_execution_data.as_deref_mut() {
                    execution_data.maybe_codeseparator_position = Some(opcode_position);
                }
            }
            OP_RETURN => return Err(ScriptError::OpReturn),
            OP_CHECKSIG => execute_checksig(
                stack,
                script,
                maybe_context,
                maybe_execution_data.as_deref_mut(),
                false,
            )?,
            OP_CHECKSIGVERIFY => {
                let checksigverify = execute_checksig(
                    stack,
                    script,
                    maybe_context,
                    maybe_execution_data.as_deref_mut(),
                    true,
                );
                checksigverify?;
            }
            OP_CHECKMULTISIG => execute_checkmultisig(
                stack,
                script,
                maybe_context,
                maybe_execution_data.as_deref_mut(),
                &mut op_count,
                false,
            )?,
            OP_CHECKMULTISIGVERIFY => {
                let checkmultisigverify = execute_checkmultisig(
                    stack,
                    script,
                    maybe_context,
                    maybe_execution_data.as_deref_mut(),
                    &mut op_count,
                    true,
                );
                checkmultisigverify?;
            }
            OP_CHECKSIGADD => {
                let checksigadd = execute_checksigadd(
                    stack,
                    script,
                    maybe_context,
                    maybe_execution_data.as_deref_mut(),
                );
                checksigadd?;
            }
            opcode if is_disabled_opcode(opcode) => {
                return Err(ScriptError::DisabledOpcode(opcode));
            }
            opcode => return Err(ScriptError::UnsupportedOpcode(opcode)),
        }
        opcode_position = opcode_position.saturating_add(1);
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
    let script_sig_eval = eval_script_internal(
        &mut stack,
        context.script_sig,
        Some(&execution_context),
        Some(&mut *context.execution_data),
    );
    script_sig_eval?;
    if context.verify_flags.contains(ScriptVerifyFlags::P2SH) {
        maybe_stack_copy = Some(stack.clone());
    }
    eval_script_internal(
        &mut stack,
        context.script_pubkey,
        Some(&execution_context),
        Some(&mut *context.execution_data),
    )?;
    verify_top_stack_true(&stack)?;

    let mut had_witness = false;
    let script_pubkey_type = classify_script_pubkey(context.script_pubkey);
    if context.verify_flags.contains(ScriptVerifyFlags::WITNESS)
        && is_witness_program_type(&script_pubkey_type)
    {
        had_witness = true;
        if !context.script_sig.as_bytes().is_empty() {
            return Err(ScriptError::WitnessMalleated);
        }
        verify_witness_program(
            &mut stack,
            context.witness,
            context.transaction,
            context.input_index,
            context.spent_input,
            context.validation_context,
            context.precomputed,
            context.verify_flags,
            &mut *context.execution_data,
            &script_pubkey_type,
            false,
            &secp,
        )?;
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
        let redeem_eval = eval_script_internal(
            &mut redeem_stack,
            &redeem_script,
            Some(&execution_context),
            Some(&mut *context.execution_data),
        );
        redeem_eval?;
        verify_top_stack_true(&redeem_stack)?;

        if context.verify_flags.contains(ScriptVerifyFlags::WITNESS) {
            let redeem_type = classify_script_pubkey(&redeem_script);
            let mut verify_nested_redeem_witness = |redeem_stack: &mut Vec<Vec<u8>>,
                                                    redeem_type: &ScriptPubKeyType|
             -> Result<bool, ScriptError> {
                if !is_witness_program_type(redeem_type) {
                    return Ok(false);
                }
                if context.script_sig.as_bytes() != single_push_script(&redeem_script).as_slice() {
                    return Err(ScriptError::WitnessMalleatedP2sh);
                }
                let witness_result = verify_witness_program(
                    redeem_stack,
                    context.witness,
                    context.transaction,
                    context.input_index,
                    context.spent_input,
                    context.validation_context,
                    context.precomputed,
                    context.verify_flags,
                    &mut *context.execution_data,
                    redeem_type,
                    true,
                    &secp,
                );
                witness_result?;
                Ok(true)
            };
            if verify_nested_redeem_witness(&mut redeem_stack, &redeem_type)? {
                had_witness = true;
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

// Keep this helper close to the upstream witness-program boundary shape.
#[allow(clippy::too_many_arguments)]
fn verify_witness_program(
    stack: &mut Vec<Vec<u8>>,
    witness: &ScriptWitness,
    transaction: &Transaction,
    input_index: usize,
    spent_input: &TransactionInputContext,
    validation_context: &TransactionValidationContext,
    precomputed: &PrecomputedTransactionData,
    verify_flags: ScriptVerifyFlags,
    execution_data: &mut ScriptExecutionData,
    script_type: &ScriptPubKeyType,
    is_p2sh: bool,
    secp: &Secp256k1<secp256k1::VerifyOnly>,
) -> Result<(), ScriptError> {
    match script_type {
        ScriptPubKeyType::WitnessV0KeyHash(program) => {
            if witness.stack().len() != 2 {
                return Err(ScriptError::WitnessProgramMismatch);
            }
            let mut exec_script_bytes = vec![OP_DUP, OP_HASH160, 20];
            exec_script_bytes.extend_from_slice(program);
            exec_script_bytes.extend_from_slice(&[OP_EQUALVERIFY, OP_CHECKSIG]);
            let exec_script =
                ScriptBuf::from_bytes(exec_script_bytes).expect("generated P2WPKH script is valid");
            execute_witness_script(
                stack,
                transaction,
                input_index,
                spent_input,
                validation_context,
                precomputed,
                verify_flags,
                execution_data,
                &exec_script,
                witness.stack().to_vec(),
                secp,
            )
        }
        ScriptPubKeyType::WitnessV0ScriptHash(program) => {
            let Some((script_bytes, witness_items)) = witness.stack().split_last() else {
                return Err(ScriptError::WitnessProgramWitnessEmpty);
            };
            if Sha256::digest(script_bytes) != *program {
                return Err(ScriptError::WitnessProgramMismatch);
            }
            let exec_script = ScriptBuf::from_bytes(script_bytes.clone())
                .map_err(|_| ScriptError::WitnessProgramMismatch)?;
            execute_witness_script(
                stack,
                transaction,
                input_index,
                spent_input,
                validation_context,
                precomputed,
                verify_flags,
                execution_data,
                &exec_script,
                witness_items.to_vec(),
                secp,
            )
        }
        ScriptPubKeyType::WitnessV1Taproot(program) if !is_p2sh => {
            if !verify_flags.contains(ScriptVerifyFlags::TAPROOT) {
                stack.clear();
                push_stack(stack, encode_bool(true))?;
                return Ok(());
            }
            if witness.stack().is_empty() {
                return Err(ScriptError::WitnessProgramWitnessEmpty);
            }

            let mut taproot_stack = witness.stack().to_vec();
            if taproot_stack.len() >= 2
                && taproot_stack
                    .last()
                    .is_some_and(|annex| !annex.is_empty() && annex[0] == ANNEX_TAG)
            {
                let annex = taproot_stack.pop().expect("checked above");
                execution_data.maybe_annex = Some(annex);
            } else {
                execution_data.maybe_annex = None;
            }

            if taproot_stack.len() == 1 {
                let signature = taproot_stack.pop().expect("checked above");
                let is_valid_signature =
                    TransactionSignatureChecker::new(secp, validation_context, precomputed)
                        .verify_schnorr(
                            &signature,
                            program,
                            transaction,
                            input_index,
                            SigVersion::Taproot,
                            execution_data,
                        )
                        .map_err(map_signature_error)?;
                if !is_valid_signature {
                    return Err(ScriptError::VerifyFailed);
                }
                stack.clear();
                push_stack(stack, encode_bool(true))?;
                return Ok(());
            }

            debug_assert!(taproot_stack.len() >= 2);
            let control = taproot_stack
                .pop()
                .ok_or(ScriptError::WitnessProgramWitnessEmpty)?;
            let script_bytes = taproot_stack
                .pop()
                .ok_or(ScriptError::WitnessProgramWitnessEmpty)?;
            if !is_valid_taproot_control_size(&control) {
                return Err(ScriptError::WitnessProgramWrongLength);
            }
            let tapleaf_hash = compute_tapleaf_hash(control[0] & TAPROOT_LEAF_MASK, &script_bytes);
            if !verify_taproot_commitment(secp, &control, program, tapleaf_hash) {
                return Err(ScriptError::WitnessProgramMismatch);
            }
            execution_data.maybe_tapleaf_hash = Some(Hash32::from_byte_array(tapleaf_hash));
            execution_data.maybe_codeseparator_position = Some(u32::MAX);

            let leaf_version = control[0] & TAPROOT_LEAF_MASK;
            if leaf_version == TAPROOT_LEAF_TAPSCRIPT {
                execution_data.maybe_validation_weight_left =
                    Some(serialized_witness_size(witness) as i64 + VALIDATION_WEIGHT_OFFSET);
                let exec_script = ScriptBuf::from_bytes(script_bytes)
                    .map_err(|_| ScriptError::WitnessProgramMismatch)?;
                return execute_tapscript(
                    stack,
                    transaction,
                    input_index,
                    spent_input,
                    validation_context,
                    precomputed,
                    verify_flags,
                    execution_data,
                    &exec_script,
                    taproot_stack,
                    secp,
                );
            }
            if verify_flags.contains(ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_TAPROOT_VERSION) {
                return Err(ScriptError::UnsupportedOpcode(OP_CHECKSIGADD));
            }
            stack.clear();
            push_stack(stack, encode_bool(true))?;
            Ok(())
        }
        ScriptPubKeyType::WitnessV1Taproot(_) if is_p2sh => {
            if verify_flags.contains(ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM) {
                return Err(ScriptError::UnsupportedOpcode(OP_0NOTEQUAL));
            }
            Ok(())
        }
        ScriptPubKeyType::WitnessUnknown { version: 0, .. } if !is_p2sh => {
            Err(ScriptError::WitnessProgramWrongLength)
        }
        ScriptPubKeyType::WitnessUnknown { version: 0, .. } => {
            Err(ScriptError::WitnessProgramWrongLength)
        }
        ScriptPubKeyType::WitnessUnknown { .. } if !is_p2sh => {
            if verify_flags.contains(ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM) {
                return Err(ScriptError::UnsupportedOpcode(OP_0NOTEQUAL));
            }
            stack.clear();
            push_stack(stack, encode_bool(true))?;
            Ok(())
        }
        ScriptPubKeyType::WitnessUnknown { .. } => Ok(()),
        _ => Err(ScriptError::WitnessProgramWrongLength),
    }
}

// Keep this helper close to the upstream witness-program boundary shape.
#[allow(clippy::too_many_arguments)]
fn execute_witness_script(
    stack: &mut Vec<Vec<u8>>,
    transaction: &Transaction,
    input_index: usize,
    spent_input: &TransactionInputContext,
    validation_context: &TransactionValidationContext,
    precomputed: &PrecomputedTransactionData,
    verify_flags: ScriptVerifyFlags,
    execution_data: &mut ScriptExecutionData,
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
        checker: TransactionSignatureChecker::new(secp, validation_context, precomputed),
        transaction,
        input_index,
        spent_input,
        verify_flags,
        sig_version: SigVersion::WitnessV0,
    };
    eval_script_internal(
        &mut witness_eval_stack,
        exec_script,
        Some(&witness_context),
        Some(execution_data),
    )?;
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

fn is_witness_program_type(script_type: &ScriptPubKeyType) -> bool {
    matches!(
        script_type,
        ScriptPubKeyType::WitnessV0KeyHash(_)
            | ScriptPubKeyType::WitnessV0ScriptHash(_)
            | ScriptPubKeyType::WitnessV1Taproot(_)
            | ScriptPubKeyType::WitnessUnknown { .. }
    )
}

fn is_valid_taproot_control_size(control: &[u8]) -> bool {
    control.len() >= TAPROOT_CONTROL_BASE_SIZE
        && control.len() <= TAPROOT_CONTROL_MAX_SIZE
        && (control.len() - TAPROOT_CONTROL_BASE_SIZE).is_multiple_of(TAPROOT_CONTROL_NODE_SIZE)
}

fn compute_tapleaf_hash(leaf_version: u8, script: &[u8]) -> [u8; 32] {
    let mut data = Vec::with_capacity(script.len() + 16);
    data.push(leaf_version);
    write_compact_size(&mut data, script.len() as u64);
    data.extend_from_slice(script);
    taproot_tagged_hash("TapLeaf", &data).to_byte_array()
}

fn compute_tapbranch_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::with_capacity(64);
    if left <= right {
        data.extend_from_slice(left);
        data.extend_from_slice(right);
    } else {
        data.extend_from_slice(right);
        data.extend_from_slice(left);
    }
    taproot_tagged_hash("TapBranch", &data).to_byte_array()
}

fn compute_taproot_merkle_root(control: &[u8], tapleaf_hash: [u8; 32]) -> [u8; 32] {
    let mut value = tapleaf_hash;
    for node in control[TAPROOT_CONTROL_BASE_SIZE..].chunks_exact(TAPROOT_CONTROL_NODE_SIZE) {
        value = compute_tapbranch_hash(&value, node.try_into().expect("32-byte node"));
    }
    value
}

fn verify_taproot_commitment(
    secp: &Secp256k1<secp256k1::VerifyOnly>,
    control: &[u8],
    program: &[u8; 32],
    tapleaf_hash: [u8; 32],
) -> bool {
    let Ok(internal_key) = XOnlyPublicKey::from_byte_array(
        control[1..TAPROOT_CONTROL_BASE_SIZE]
            .try_into()
            .expect("32-byte internal key"),
    ) else {
        return false;
    };
    let Ok(output_key) = XOnlyPublicKey::from_byte_array(*program) else {
        return false;
    };
    let merkle_root = compute_taproot_merkle_root(control, tapleaf_hash);
    let mut tweak_preimage = Vec::with_capacity(64);
    tweak_preimage.extend_from_slice(&control[1..TAPROOT_CONTROL_BASE_SIZE]);
    tweak_preimage.extend_from_slice(&merkle_root);
    let parity = if (control[0] & 1) == 1 {
        Parity::Odd
    } else {
        Parity::Even
    };
    Scalar::from_be_bytes(taproot_tagged_hash("TapTweak", &tweak_preimage).to_byte_array())
        .is_ok_and(|tweak_scalar| {
            internal_key.tweak_add_check(secp, &output_key, parity, tweak_scalar)
        })
}

fn serialized_witness_size(witness: &ScriptWitness) -> usize {
    let mut size = compact_size_len(witness.stack().len() as u64);
    for item in witness.stack() {
        size += compact_size_len(item.len() as u64);
        size += item.len();
    }
    size
}

fn compact_size_len(value: u64) -> usize {
    match value {
        0..=252 => 1,
        253..=0xffff => 3,
        0x1_0000..=0xffff_ffff => 5,
        _ => 9,
    }
}

fn write_compact_size(out: &mut Vec<u8>, value: u64) {
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

fn is_op_success(opcode: u8) -> bool {
    opcode == 80
        || opcode == 98
        || (126..=129).contains(&opcode)
        || (131..=134).contains(&opcode)
        || (137..=138).contains(&opcode)
        || (141..=142).contains(&opcode)
        || (149..=153).contains(&opcode)
        || (187..=254).contains(&opcode)
}

// Keep this helper close to the upstream tapscript boundary shape.
#[allow(clippy::too_many_arguments)]
fn execute_tapscript(
    stack: &mut Vec<Vec<u8>>,
    transaction: &Transaction,
    input_index: usize,
    spent_input: &TransactionInputContext,
    validation_context: &TransactionValidationContext,
    precomputed: &PrecomputedTransactionData,
    verify_flags: ScriptVerifyFlags,
    execution_data: &mut ScriptExecutionData,
    exec_script: &ScriptBuf,
    witness_stack: Vec<Vec<u8>>,
    secp: &Secp256k1<secp256k1::VerifyOnly>,
) -> Result<(), ScriptError> {
    let bytes = exec_script.as_bytes();
    let mut pc = 0;
    while pc < bytes.len() {
        let instruction = read_instruction(bytes, &mut pc)?;
        if is_op_success(instruction.opcode) {
            if verify_flags.contains(ScriptVerifyFlags::DISCOURAGE_OP_SUCCESS) {
                return Err(ScriptError::UnsupportedOpcode(instruction.opcode));
            }
            stack.clear();
            push_stack(stack, encode_bool(true))?;
            return Ok(());
        }
    }

    let mut tapscript_stack = Vec::with_capacity(witness_stack.len());
    if witness_stack.len() > MAX_STACK_SIZE {
        return Err(ScriptError::StackOverflow(witness_stack.len()));
    }
    for element in witness_stack {
        if element.len() > MAX_SCRIPT_ELEMENT_SIZE {
            return Err(ScriptError::PushSize(element.len()));
        }
        push_stack(&mut tapscript_stack, element)?;
    }

    let tapscript_context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(secp, validation_context, precomputed),
        transaction,
        input_index,
        spent_input,
        verify_flags,
        sig_version: SigVersion::Tapscript,
    };
    eval_script_internal(
        &mut tapscript_stack,
        exec_script,
        Some(&tapscript_context),
        Some(execution_data),
    )?;
    if tapscript_stack.len() != 1 {
        return Err(ScriptError::WitnessCleanStack);
    }
    verify_top_stack_true(&tapscript_stack)?;
    *stack = tapscript_stack;
    Ok(())
}

fn execute_checksig(
    stack: &mut Vec<Vec<u8>>,
    script: &ScriptBuf,
    maybe_context: Option<&LegacyExecutionContext<'_>>,
    maybe_execution_data: Option<&mut ScriptExecutionData>,
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
    let is_valid_signature = match context.sig_version {
        SigVersion::Base | SigVersion::WitnessV0 => {
            let script_code = if context.sig_version == SigVersion::Base {
                remove_signature_from_script(script, &signature)
            } else {
                script.clone()
            };
            context
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
                .map_err(map_signature_error)?
        }
        SigVersion::Taproot => {
            let execution_data = maybe_execution_data.ok_or(ScriptError::VerifyFailed)?;
            context
                .checker
                .verify_schnorr(
                    &signature,
                    &public_key,
                    context.transaction,
                    context.input_index,
                    SigVersion::Taproot,
                    execution_data,
                )
                .map_err(map_signature_error)?
        }
        SigVersion::Tapscript => {
            let execution_data = maybe_execution_data.ok_or(ScriptError::VerifyFailed)?;
            execute_tapscript_checksig(context, execution_data, &signature, &public_key)?
        }
    };

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
    _maybe_execution_data: Option<&mut ScriptExecutionData>,
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
    if context.sig_version == SigVersion::Tapscript {
        return Err(ScriptError::UnsupportedOpcode(OP_CHECKMULTISIG));
    }
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
    let mut signatures = stack[dummy_index + 1..dummy_index + 1 + sig_count].to_vec();
    signatures.reverse();
    let mut pubkeys = stack[sig_count_index + 1..stack.len() - 1].to_vec();
    pubkeys.reverse();

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

fn execute_checksigadd(
    stack: &mut Vec<Vec<u8>>,
    _script: &ScriptBuf,
    maybe_context: Option<&LegacyExecutionContext<'_>>,
    maybe_execution_data: Option<&mut ScriptExecutionData>,
) -> Result<(), ScriptError> {
    let Some(context) = maybe_context else {
        return Err(ScriptError::UnsupportedOpcode(OP_CHECKSIGADD));
    };
    if context.sig_version != SigVersion::Tapscript {
        return Err(ScriptError::UnsupportedOpcode(OP_CHECKSIGADD));
    }
    if stack.len() < 3 {
        return Err(ScriptError::InvalidStackOperation);
    }

    let public_key = pop_bytes(stack)?;
    let value = pop_num(stack)?;
    let signature = pop_bytes(stack)?;
    let execution_data = maybe_execution_data.ok_or(ScriptError::VerifyFailed)?;
    let is_valid_signature =
        execute_tapscript_checksig(context, execution_data, &signature, &public_key)?;
    let updated_value = encode_script_num(value + i64::from(is_valid_signature));
    push_stack(stack, updated_value)?;
    Ok(())
}

fn execute_tapscript_checksig(
    context: &LegacyExecutionContext<'_>,
    execution_data: &mut ScriptExecutionData,
    signature: &[u8],
    public_key: &[u8],
) -> Result<bool, ScriptError> {
    let is_valid_signature = !signature.is_empty();
    if is_valid_signature {
        let maybe_weight_left = execution_data
            .maybe_validation_weight_left
            .as_mut()
            .ok_or(ScriptError::VerifyFailed)?;
        *maybe_weight_left -= VALIDATION_WEIGHT_PER_SIGOP_PASSED;
        if *maybe_weight_left < 0 {
            return Err(ScriptError::VerifyFailed);
        }
    }

    if public_key.is_empty() {
        return Err(ScriptError::PubKeyType);
    }
    if public_key.len() == 32 {
        if is_valid_signature {
            return context
                .checker
                .verify_schnorr(
                    signature,
                    public_key,
                    context.transaction,
                    context.input_index,
                    SigVersion::Tapscript,
                    execution_data,
                )
                .map_err(map_signature_error);
        }
        return Ok(false);
    }
    if context
        .verify_flags
        .contains(ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_PUBKEYTYPE)
    {
        return Err(ScriptError::UnsupportedOpcode(OP_CHECKSIGADD));
    }
    Ok(is_valid_signature)
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
mod tests;
