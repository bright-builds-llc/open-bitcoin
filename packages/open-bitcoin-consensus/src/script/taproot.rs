use open_bitcoin_primitives::{
    Hash32, MAX_SCRIPT_ELEMENT_SIZE, ScriptBuf, ScriptWitness, Transaction,
};
use secp256k1::{Parity, Scalar, Secp256k1, XOnlyPublicKey};

use crate::context::{
    PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags, TransactionInputContext,
    TransactionValidationContext,
};
use crate::sighash::{SigVersion, taproot_tagged_hash};
use crate::signature::TransactionSignatureChecker;

use super::ScriptError;
use super::encoding::{read_instruction, serialized_witness_size, write_compact_size};
use super::legacy::{
    LegacyExecutionContext, eval_script_internal, map_signature_error, verify_top_stack_true,
};
use super::opcodes::{OP_CHECKSIGADD, is_op_success};
use super::stack::{
    MAX_STACK_SIZE, encode_bool, encode_script_num, pop_bytes, pop_num, push_stack,
};

pub(super) const TAPROOT_LEAF_MASK: u8 = 0xfe;
pub(super) const TAPROOT_LEAF_TAPSCRIPT: u8 = 0xc0;
pub(super) const TAPROOT_CONTROL_BASE_SIZE: usize = 33;
const TAPROOT_CONTROL_NODE_SIZE: usize = 32;
const TAPROOT_CONTROL_MAX_NODE_COUNT: usize = 128;
const TAPROOT_CONTROL_MAX_SIZE: usize =
    TAPROOT_CONTROL_BASE_SIZE + TAPROOT_CONTROL_NODE_SIZE * TAPROOT_CONTROL_MAX_NODE_COUNT;
const ANNEX_TAG: u8 = 0x50;
const VALIDATION_WEIGHT_PER_SIGOP_PASSED: i64 = 50;
const VALIDATION_WEIGHT_OFFSET: i64 = 50;

pub(super) fn is_valid_taproot_control_size(control: &[u8]) -> bool {
    control.len() >= TAPROOT_CONTROL_BASE_SIZE
        && control.len() <= TAPROOT_CONTROL_MAX_SIZE
        && (control.len() - TAPROOT_CONTROL_BASE_SIZE).is_multiple_of(TAPROOT_CONTROL_NODE_SIZE)
}

pub(super) fn compute_tapleaf_hash(leaf_version: u8, script: &[u8]) -> [u8; 32] {
    let mut data = Vec::with_capacity(script.len() + 16);
    data.push(leaf_version);
    write_compact_size(&mut data, script.len() as u64);
    data.extend_from_slice(script);
    taproot_tagged_hash("TapLeaf", &data).to_byte_array()
}

pub(super) fn compute_tapbranch_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
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

pub(super) fn compute_taproot_merkle_root(control: &[u8], tapleaf_hash: [u8; 32]) -> [u8; 32] {
    let mut value = tapleaf_hash;
    for node in control[TAPROOT_CONTROL_BASE_SIZE..].chunks_exact(TAPROOT_CONTROL_NODE_SIZE) {
        value = compute_tapbranch_hash(&value, node.try_into().expect("32-byte node"));
    }
    value
}

pub(super) fn verify_taproot_commitment(
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

#[allow(clippy::too_many_arguments)]
pub(super) fn verify_taproot_program(
    stack: &mut Vec<Vec<u8>>,
    witness: &ScriptWitness,
    transaction: &Transaction,
    input_index: usize,
    spent_input: &TransactionInputContext,
    validation_context: &TransactionValidationContext,
    precomputed: &PrecomputedTransactionData,
    verify_flags: ScriptVerifyFlags,
    execution_data: &mut ScriptExecutionData,
    program: &[u8; 32],
    secp: &Secp256k1<secp256k1::VerifyOnly>,
) -> Result<(), ScriptError> {
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
        let exec_script =
            ScriptBuf::from_bytes(script_bytes).map_err(|_| ScriptError::WitnessProgramMismatch)?;
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

#[allow(clippy::too_many_arguments)]
pub(super) fn execute_tapscript(
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

pub(super) fn execute_checksigadd(
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

pub(super) fn execute_tapscript_checksig(
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
