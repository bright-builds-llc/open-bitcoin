use open_bitcoin_primitives::{MAX_OPS_PER_SCRIPT, ScriptBuf, Transaction};

use crate::context::{ScriptExecutionData, ScriptVerifyFlags, TransactionInputContext};
use crate::crypto::{Sha256, double_sha256, hash160};
use crate::sighash::SigVersion;
use crate::signature::{EcdsaVerificationRequest, SignatureError, TransactionSignatureChecker};

use super::ScriptError;
use super::encoding::{read_instruction, remove_signature_from_script};
use super::opcodes::{
    MAX_PUBKEYS_PER_MULTISIG, OP_0NOTEQUAL, OP_1, OP_1ADD, OP_1NEGATE, OP_1SUB, OP_16, OP_ADD,
    OP_BOOLAND, OP_BOOLOR, OP_CHECKMULTISIG, OP_CHECKMULTISIGVERIFY, OP_CHECKSIG, OP_CHECKSIGADD,
    OP_CHECKSIGVERIFY, OP_CODESEPARATOR, OP_DROP, OP_DUP, OP_ELSE, OP_ENDIF, OP_EQUAL,
    OP_EQUALVERIFY, OP_GREATERTHAN, OP_HASH160, OP_HASH256, OP_IF, OP_LESSTHAN, OP_MAX, OP_MIN,
    OP_NEGATE, OP_NOP, OP_NOT, OP_NOTIF, OP_NUMEQUAL, OP_NUMEQUALVERIFY, OP_NUMNOTEQUAL, OP_OVER,
    OP_RETURN, OP_RIPEMD160, OP_SHA256, OP_SIZE, OP_SUB, OP_SWAP, OP_VERIFY, OP_WITHIN,
    is_disabled_opcode,
};
use super::stack::{
    ConditionStack, binary_num_op, cast_to_bool, decode_small_num, encode_bool, encode_script_num,
    pop_bytes, pop_num, push_stack, script_booland, script_boolor, unary_num_op,
};
use super::taproot::{execute_checksigadd, execute_tapscript_checksig};

pub(super) fn eval_script(stack: &mut Vec<Vec<u8>>, script: &ScriptBuf) -> Result<(), ScriptError> {
    eval_script_internal(stack, script, None, None)
}

pub(super) struct LegacyExecutionContext<'a> {
    pub(super) checker: TransactionSignatureChecker<'a, secp256k1::VerifyOnly>,
    pub(super) transaction: &'a Transaction,
    pub(super) input_index: usize,
    pub(super) spent_input: &'a TransactionInputContext,
    pub(super) verify_flags: ScriptVerifyFlags,
    pub(super) sig_version: SigVersion,
}

pub(super) fn eval_script_internal(
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

pub(super) fn verify_script(
    script_sig: &ScriptBuf,
    script_pubkey: &ScriptBuf,
) -> Result<(), ScriptError> {
    let mut stack = Vec::new();
    eval_script(&mut stack, script_sig)?;
    eval_script(&mut stack, script_pubkey)?;
    verify_top_stack_true(&stack)
}

pub(super) fn verify_top_stack_true(stack: &[Vec<u8>]) -> Result<(), ScriptError> {
    let Some(top) = stack.last() else {
        return Err(ScriptError::EvalFalse);
    };
    if !cast_to_bool(top) {
        return Err(ScriptError::EvalFalse);
    }
    Ok(())
}

fn push_check_result(
    stack: &mut Vec<Vec<u8>>,
    is_valid_signature: bool,
    verify: bool,
) -> Result<(), ScriptError> {
    push_stack(stack, encode_bool(is_valid_signature))?;
    if !verify {
        return Ok(());
    }
    if !is_valid_signature {
        return Err(ScriptError::VerifyFailed);
    }
    pop_bytes(stack)?;
    Ok(())
}

pub(super) fn execute_checksig(
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
    push_check_result(stack, is_valid_signature, verify)
}

pub(super) fn execute_checkmultisig(
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
    let Some(key_count_bytes) = stack.last() else {
        return Err(ScriptError::InvalidStackOperation);
    };

    let key_count = decode_small_num(key_count_bytes)?;
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
    push_check_result(stack, matched_all_signatures, verify)
}

pub(super) fn map_signature_error(error: SignatureError) -> ScriptError {
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
