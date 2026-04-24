use open_bitcoin_primitives::{MAX_SCRIPT_ELEMENT_SIZE, ScriptBuf, ScriptWitness, Transaction};
use secp256k1::Secp256k1;

use crate::classify::{
    ScriptPubKeyType, classify_script_pubkey, extract_redeem_script, is_push_only,
};
use crate::context::{
    PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags, TransactionInputContext,
    TransactionValidationContext,
};
use crate::crypto::Sha256;
use crate::sighash::SigVersion;
use crate::signature::TransactionSignatureChecker;

use super::ScriptError;
use super::ScriptInputVerificationContext;
use super::encoding::encode_push_data;
use super::legacy::{LegacyExecutionContext, eval_script_internal, verify_top_stack_true};
use super::opcodes::{OP_0NOTEQUAL, OP_CHECKSIG, OP_DUP, OP_EQUALVERIFY, OP_HASH160};
use super::stack::{encode_bool, push_stack};
use super::taproot::verify_taproot_program;

pub(super) fn verify_input_script(
    context: ScriptInputVerificationContext<'_>,
) -> Result<(), ScriptError> {
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
        super::stack::pop_bytes(&mut redeem_stack)?;
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

pub(super) fn is_witness_program_type(script_type: &ScriptPubKeyType) -> bool {
    matches!(
        script_type,
        ScriptPubKeyType::WitnessV0KeyHash(_)
            | ScriptPubKeyType::WitnessV0ScriptHash(_)
            | ScriptPubKeyType::WitnessV1Taproot(_)
            | ScriptPubKeyType::WitnessUnknown { .. }
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn verify_witness_program(
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
            let exec_script = ScriptBuf::from_bytes(exec_script_bytes)
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
        ScriptPubKeyType::WitnessV1Taproot(program) if !is_p2sh => verify_taproot_program(
            stack,
            witness,
            transaction,
            input_index,
            spent_input,
            validation_context,
            precomputed,
            verify_flags,
            execution_data,
            program,
            secp,
        ),
        ScriptPubKeyType::WitnessV1Taproot(_) if is_p2sh => {
            if verify_flags.contains(ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM) {
                return Err(ScriptError::UnsupportedOpcode(OP_0NOTEQUAL));
            }
            Ok(())
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
