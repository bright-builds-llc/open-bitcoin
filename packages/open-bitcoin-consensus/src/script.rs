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

mod encoding;
mod opcodes;
mod stack;

use self::encoding::{
    encode_push_data, read_instruction, remove_signature_from_script, serialized_witness_size,
    write_compact_size,
};
use self::opcodes::{
    MAX_PUBKEYS_PER_MULTISIG, OP_0NOTEQUAL, OP_1, OP_1ADD, OP_1NEGATE, OP_1SUB, OP_16, OP_ADD,
    OP_BOOLAND, OP_BOOLOR, OP_CHECKMULTISIG, OP_CHECKMULTISIGVERIFY, OP_CHECKSIG, OP_CHECKSIGADD,
    OP_CHECKSIGVERIFY, OP_CODESEPARATOR, OP_DROP, OP_DUP, OP_ELSE, OP_ENDIF, OP_EQUAL,
    OP_EQUALVERIFY, OP_GREATERTHAN, OP_HASH160, OP_HASH256, OP_IF, OP_LESSTHAN, OP_MAX, OP_MIN,
    OP_NEGATE, OP_NOP, OP_NOT, OP_NOTIF, OP_NUMEQUAL, OP_NUMEQUALVERIFY, OP_NUMNOTEQUAL, OP_OVER,
    OP_RETURN, OP_RIPEMD160, OP_SHA256, OP_SIZE, OP_SUB, OP_SWAP, OP_VERIFY, OP_WITHIN,
    decode_small_int_opcode, is_disabled_opcode, is_op_success,
};
use self::stack::{
    ConditionStack, MAX_STACK_SIZE, binary_num_op, cast_to_bool, decode_small_num, encode_bool,
    encode_script_num, pop_bytes, pop_num, push_stack, script_booland, script_boolor, unary_num_op,
};

#[cfg(test)]
use self::encoding::compact_size_len;
#[cfg(test)]
use self::stack::decode_script_num;

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

#[cfg(test)]
mod tests;
