// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

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
use super::encoding::{read_instruction_syntax, serialized_witness_size, write_compact_size};
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
        let mut node_hash = [0_u8; TAPROOT_CONTROL_NODE_SIZE];
        node_hash.copy_from_slice(node);
        value = compute_tapbranch_hash(&value, &node_hash);
    }
    value
}

pub(super) fn verify_taproot_commitment(
    secp: &Secp256k1<secp256k1::VerifyOnly>,
    control: &[u8],
    program: &[u8; 32],
    tapleaf_hash: [u8; 32],
) -> bool {
    let mut internal_key_bytes = [0_u8; TAPROOT_CONTROL_NODE_SIZE];
    internal_key_bytes.copy_from_slice(&control[1..TAPROOT_CONTROL_BASE_SIZE]);
    let Ok(internal_key) = XOnlyPublicKey::from_byte_array(internal_key_bytes) else {
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
        let annex_index = taproot_stack.len() - 1;
        let annex = taproot_stack.remove(annex_index);
        execution_data.maybe_annex = Some(annex);
    } else {
        execution_data.maybe_annex = None;
    }

    if taproot_stack.len() == 1 {
        let signature = taproot_stack.remove(0);
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
        let exec_script = ScriptBuf::from_serialized_bytes(script_bytes);
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
        let instruction = read_instruction_syntax(bytes, &mut pc)?;
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

#[cfg(test)]
mod serialized_tests {
    use super::*;
    use crate::script::serialized_tests::{context, transaction};

    fn execute(
        bytes: Vec<u8>,
        items: Vec<Vec<u8>>,
        flags: ScriptVerifyFlags,
        weight: i64,
    ) -> Result<Vec<Vec<u8>>, ScriptError> {
        let transaction = transaction();
        let (spent, validation, precomputed) = context(&transaction);
        let mut stack = Vec::new();
        let mut data = ScriptExecutionData {
            maybe_validation_weight_left: Some(weight),
            ..Default::default()
        };
        execute_tapscript(
            &mut stack,
            &transaction,
            0,
            &spent,
            &validation,
            &precomputed,
            flags,
            &mut data,
            &ScriptBuf::from_serialized_bytes(bytes),
            items,
            &Secp256k1::verification_only(),
        )?;
        Ok(stack)
    }

    fn program(bytes: Vec<u8>, corrupt: bool) -> Result<(), ScriptError> {
        let secp = Secp256k1::new();
        let secret = secp256k1::SecretKey::from_byte_array([31; 32]).expect("fixed key");
        let keypair = secp256k1::Keypair::from_secret_key(&secp, &secret);
        let (internal, _) = XOnlyPublicKey::from_keypair(&keypair);
        let leaf = compute_tapleaf_hash(0xc0, &bytes);
        let mut preimage = internal.serialize().to_vec();
        preimage.extend_from_slice(&leaf);
        let tweak =
            Scalar::from_be_bytes(taproot_tagged_hash("TapTweak", &preimage).to_byte_array())
                .expect("fixed tweak");
        let tweaked = keypair
            .add_xonly_tweak(&secp, &tweak)
            .expect("fixed output");
        let (output, parity) = XOnlyPublicKey::from_keypair(&tweaked);
        let mut control = vec![0xc0 | u8::from(parity == Parity::Odd)];
        control.extend_from_slice(&internal.serialize());
        let mut bytes = bytes;
        if corrupt {
            bytes[0] ^= 1;
        }
        let witness = ScriptWitness::new(vec![bytes, control]);
        let transaction = transaction();
        let (spent, validation, precomputed) = context(&transaction);
        verify_taproot_program(
            &mut Vec::new(),
            &witness,
            &transaction,
            0,
            &spent,
            &validation,
            &precomputed,
            ScriptVerifyFlags::TAPROOT,
            &mut ScriptExecutionData::default(),
            &output.serialize(),
            &Secp256k1::verification_only(),
        )
    }

    #[test]
    fn phase167_serialized_script_paths_taproot_committed_wide_script_executes() {
        // Arrange
        let mut bytes = vec![0x61; 10_001];
        bytes.push(0x51);
        // Act
        let result = program(bytes, false);
        // Assert
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn phase167_serialized_script_paths_taproot_keeps_commitment_check() {
        // Arrange
        let bytes = vec![0x61; 10_001];
        // Act
        let result = program(bytes, true);
        // Assert
        assert_eq!(result, Err(ScriptError::WitnessProgramMismatch));
    }

    #[test]
    fn phase167_serialized_script_paths_success_after_large_push_precedes_limits() {
        // Arrange
        let mut bytes = vec![0x4d, 0x09, 0x02];
        bytes.extend(vec![0xff; 521]);
        bytes.extend_from_slice(&[0x50, 0x4e]);
        // Act
        let result = execute(bytes, vec![vec![0; 521]; 1001], ScriptVerifyFlags::NONE, 0);
        // Assert
        assert_eq!(result, Ok(vec![vec![1]]));
    }

    #[test]
    fn phase167_serialized_script_paths_malformed_before_success_fails() {
        // Arrange
        let bytes = vec![0x4d, 0x02, 0x00, 0x50];
        // Act
        let result = execute(bytes, Vec::new(), ScriptVerifyFlags::NONE, 0);
        // Assert
        assert_eq!(result, Err(ScriptError::TruncatedPushData));
    }

    #[test]
    fn phase167_serialized_script_paths_pushed_success_is_data() {
        // Arrange
        let bytes = vec![1, 0x50, 0x75, 0];
        // Act
        let result = execute(bytes, Vec::new(), ScriptVerifyFlags::NONE, 0);
        // Assert
        assert_eq!(result, Err(ScriptError::EvalFalse));
    }

    #[test]
    fn phase167_serialized_script_paths_success_policy_still_refuses() {
        // Arrange
        let bytes = vec![0x50, 0x4e];
        // Act
        let result = execute(
            bytes,
            Vec::new(),
            ScriptVerifyFlags::DISCOURAGE_OP_SUCCESS,
            0,
        );
        // Assert
        assert_eq!(result, Err(ScriptError::UnsupportedOpcode(0x50)));
    }

    #[test]
    fn phase167_serialized_script_paths_no_success_keeps_initial_stack_limit() {
        // Arrange
        let bytes = vec![0x51];
        // Act
        let result = execute(bytes, vec![vec![]; 1001], ScriptVerifyFlags::NONE, 0);
        // Assert
        assert_eq!(result, Err(ScriptError::StackOverflow(1001)));
    }

    #[test]
    fn phase167_serialized_script_paths_no_success_keeps_initial_element_limit() {
        // Arrange
        let bytes = vec![0x51];
        // Act
        let result = execute(bytes, vec![vec![0; 521]], ScriptVerifyFlags::NONE, 0);
        // Assert
        assert_eq!(result, Err(ScriptError::PushSize(521)));
    }

    #[test]
    fn phase167_serialized_script_paths_no_success_keeps_validation_weight() {
        // Arrange
        let items = vec![vec![1], vec![1]];
        // Act
        let rejected = execute(vec![0xac], items.clone(), ScriptVerifyFlags::NONE, 49);
        let accepted = execute(vec![0xac], items, ScriptVerifyFlags::NONE, 50);
        // Assert
        assert_eq!(rejected, Err(ScriptError::VerifyFailed));
        assert_eq!(accepted, Ok(vec![vec![1]]));
    }

    #[test]
    fn phase167_serialized_script_paths_no_success_keeps_signature_validation() {
        // Arrange
        let items = vec![vec![1], vec![2; 32]];
        // Act
        let result = execute(vec![0xac], items, ScriptVerifyFlags::NONE, 50);
        // Assert
        assert_eq!(result, Err(ScriptError::SigDer));
    }
}
