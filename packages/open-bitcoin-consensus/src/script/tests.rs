use open_bitcoin_primitives::{
    Amount, Hash32, MAX_OPS_PER_SCRIPT, MAX_SCRIPT_ELEMENT_SIZE, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid,
};
use secp256k1::{Keypair, Message, PublicKey, Scalar, Secp256k1, SecretKey, XOnlyPublicKey};

use crate::classify::ScriptPubKeyType;
use crate::context::{PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags};
use crate::context::{SpentOutput, TransactionInputContext, TransactionValidationContext};
use crate::crypto::{Sha256, hash160};
use crate::sighash::{SigHashType, SigVersion, legacy_sighash};
use open_bitcoin_primitives::ScriptBuf;

use super::encoding::{
    compact_size_len, encode_push_data, remove_signature_from_script, write_compact_size,
};
use super::legacy::{
    LegacyExecutionContext, eval_script_internal, execute_checkmultisig, execute_checksig,
    map_signature_error, verify_top_stack_true,
};
use super::opcodes::{
    OP_0NOTEQUAL, OP_1, OP_CHECKMULTISIG, OP_CHECKMULTISIGVERIFY, OP_CHECKSIG, OP_CHECKSIGADD,
    OP_CHECKSIGVERIFY, OP_DUP, OP_ELSE, OP_ENDIF, OP_EQUALVERIFY, OP_HASH160, OP_IF, OP_NOTIF,
    OP_RESERVED, OP_VER, decode_small_int_opcode, is_disabled_opcode, is_op_success,
};
use super::sigops::witness_sigops_for_type;
use super::stack::{
    ConditionStack, MAX_STACK_SIZE, cast_to_bool, decode_script_num, decode_small_num, encode_bool,
    encode_script_num,
};
use super::taproot::{
    TAPROOT_CONTROL_BASE_SIZE, TAPROOT_LEAF_TAPSCRIPT, compute_tapbranch_hash,
    compute_tapleaf_hash, compute_taproot_merkle_root, execute_checksigadd, execute_tapscript,
    execute_tapscript_checksig, verify_taproot_commitment,
};
use super::witness::verify_witness_program;
use super::{
    ScriptError, ScriptInputVerificationContext, count_legacy_sigops, count_p2sh_sigops,
    count_witness_sigops, eval_script, verify_script,
};
use crate::TransactionSignatureChecker;
use crate::taproot_tagged_hash;

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn decode_hex(input: &str) -> Vec<u8> {
    let trimmed = input.trim();
    let mut bytes = Vec::with_capacity(trimmed.len() / 2);
    let chars: Vec<char> = trimmed.chars().collect();
    for pair in chars.chunks(2) {
        let high = pair[0].to_digit(16).expect("hex fixture");
        let low = pair[1].to_digit(16).expect("hex fixture");
        bytes.push(((high << 4) | low) as u8);
    }
    bytes
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

fn tap_tweak_scalar(internal_key: &[u8; 32], maybe_merkle_root: Option<[u8; 32]>) -> Scalar {
    let mut preimage = internal_key.to_vec();
    if let Some(merkle_root) = maybe_merkle_root {
        preimage.extend_from_slice(&merkle_root);
    }
    Scalar::from_be_bytes(taproot_tagged_hash("TapTweak", &preimage).to_byte_array())
        .expect("tap tweak must be in range")
}

fn taproot_script_pubkey(xonly_public_key: &XOnlyPublicKey) -> ScriptBuf {
    let mut bytes = vec![0x51, 32];
    bytes.extend_from_slice(&xonly_public_key.serialize());
    script(&bytes)
}

fn taproot_keypair(
    secret_key_byte: u8,
    maybe_merkle_root: Option<[u8; 32]>,
) -> (Keypair, XOnlyPublicKey, secp256k1::Parity, XOnlyPublicKey) {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([secret_key_byte; 32]).expect("secret key");
    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    let (internal_key, _) = XOnlyPublicKey::from_keypair(&keypair);
    let tweak = tap_tweak_scalar(&internal_key.serialize(), maybe_merkle_root);
    let tweaked_keypair = keypair
        .add_xonly_tweak(&secp, &tweak)
        .expect("taproot tweak");
    let (output_key, parity) = XOnlyPublicKey::from_keypair(&tweaked_keypair);
    (tweaked_keypair, internal_key, parity, output_key)
}

fn push_only_script(pushes: &[&[u8]]) -> ScriptBuf {
    let mut bytes = Vec::new();
    for push in pushes {
        bytes.push(push.len() as u8);
        bytes.extend_from_slice(push);
    }
    script(&bytes)
}

fn control_prefix(leaf_version: u8, parity: secp256k1::Parity) -> u8 {
    leaf_version | u8::from(parity == secp256k1::Parity::Odd)
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
        0xa8, 0x20, 0xca, 0x97, 0x81, 0x12, 0xca, 0x1b, 0xbd, 0xca, 0xfa, 0xc2, 0x31, 0xb3, 0x9a,
        0x23, 0xdc, 0x4d, 0xa7, 0x86, 0xef, 0xf8, 0x14, 0x7c, 0x4e, 0x72, 0xb9, 0x80, 0x77, 0x85,
        0xaf, 0xee, 0x48, 0xbb, 0x87,
    ]);

    assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
}

#[test]
fn verify_script_matches_knots_hash256_vector() {
    let script_sig = script(&[0x01, 0x61]);
    let script_pubkey = script(&[
        0xaa, 0x20, 0xbf, 0x5d, 0x3a, 0xff, 0xb7, 0x3e, 0xfd, 0x2e, 0xc6, 0xc3, 0x6a, 0xd3, 0x11,
        0x2d, 0xd9, 0x33, 0xef, 0xed, 0x63, 0xc4, 0xe1, 0xcb, 0xff, 0xcf, 0xa8, 0x8e, 0x27, 0x59,
        0xc1, 0x44, 0xf2, 0xd8, 0x87,
    ]);

    assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
}

#[test]
fn verify_script_rejects_false_final_stack() {
    let error = verify_script(&script(&[]), &script(&[0x00])).expect_err("false stack must fail");

    assert_eq!(error, ScriptError::EvalFalse);
}

#[test]
fn verify_script_rejects_empty_stack_after_execution() {
    let error = verify_script(&script(&[]), &script(&[])).expect_err("empty final stack must fail");

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
    assert_eq!(decode_script_num(&[]), Ok(0));
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
    assert_eq!(
        witness_sigops_for_type(
            &ScriptPubKeyType::WitnessV0ScriptHash([0_u8; 32]),
            &ScriptWitness::default(),
        )
        .expect("empty witness script should not count sigops"),
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
        eval_script(&mut Vec::new(), &script(&[OP_ENDIF])).expect_err("ENDIF without IF must fail"),
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
    let error =
        eval_script(&mut Vec::new(), &script(&[0x00, 0x69])).expect_err("verify false must fail");

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
        let error = eval_script(&mut Vec::new(), &candidate).expect_err("empty-stack op must fail");
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
        eval_script(&mut Vec::new(), &script(&[0xac])).expect_err("checksig must be unsupported"),
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
        eval_script(&mut Vec::new(), &script(&[0x4c])).expect_err("truncated pushdata1 must fail"),
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
fn verify_input_script_enforces_taproot_discouragement_flags() {
    let transaction = legacy_transaction(36);
    let (_tweaked_keypair, internal_key, parity, output_key) =
        taproot_keypair(37, Some(compute_tapleaf_hash(0xc2, &[OP_1])));
    let script_pubkey = taproot_script_pubkey(&output_key);
    let control = {
        let mut bytes = vec![control_prefix(0xc2, parity)];
        bytes.extend_from_slice(&internal_key.serialize());
        bytes
    };
    let witness = ScriptWitness::new(vec![vec![OP_1], control.clone()]);
    let (spent_input, validation_context, precomputed) = legacy_context(
        script_pubkey.clone(),
        &transaction,
        ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::WITNESS
            | ScriptVerifyFlags::TAPROOT
            | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_TAPROOT_VERSION,
    );
    let mut execution_data = ScriptExecutionData::default();
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
            | ScriptVerifyFlags::TAPROOT
            | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_TAPROOT_VERSION,
        precomputed: &precomputed,
        execution_data: &mut execution_data,
    })
    .expect_err("upgradable taproot version should be discouraged");
    assert_eq!(error, ScriptError::UnsupportedOpcode(OP_CHECKSIGADD));

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([38_u8; 32]).expect("secret key");
    let script_keypair = Keypair::from_secret_key(&secp, &secret_key);
    let (script_key, _) = XOnlyPublicKey::from_keypair(&script_keypair);
    let op_success_script = vec![0x50];
    let tapleaf_hash = compute_tapleaf_hash(TAPROOT_LEAF_TAPSCRIPT, &op_success_script);
    let (_tweaked_keypair, internal_key, parity, output_key) =
        taproot_keypair(39, Some(tapleaf_hash));
    let op_success_script_pubkey = taproot_script_pubkey(&output_key);
    let control = {
        let mut bytes = vec![control_prefix(TAPROOT_LEAF_TAPSCRIPT, parity)];
        bytes.extend_from_slice(&internal_key.serialize());
        bytes
    };
    let (op_success_spent_input, op_success_validation_context, op_success_precomputed) =
        legacy_context(
            op_success_script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::TAPROOT
                | ScriptVerifyFlags::DISCOURAGE_OP_SUCCESS,
        );
    let witness = ScriptWitness::new(vec![op_success_script.clone(), control]);
    let error = super::verify_input_script(ScriptInputVerificationContext {
        script_sig: &ScriptBuf::default(),
        script_pubkey: &op_success_script_pubkey,
        witness: &witness,
        transaction: &transaction,
        input_index: 0,
        spent_input: &op_success_spent_input,
        validation_context: &op_success_validation_context,
        spent_amount: op_success_spent_input.spent_output.value,
        verify_flags: ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::WITNESS
            | ScriptVerifyFlags::TAPROOT
            | ScriptVerifyFlags::DISCOURAGE_OP_SUCCESS,
        precomputed: &op_success_precomputed,
        execution_data: &mut execution_data,
    })
    .expect_err("OP_SUCCESS should be discouraged when flagged");
    assert_eq!(error, ScriptError::UnsupportedOpcode(0x50));

    let tapscript_bytes = vec![0x01, 0x02, OP_CHECKSIG];
    let tapleaf_hash = compute_tapleaf_hash(TAPROOT_LEAF_TAPSCRIPT, &tapscript_bytes);
    let (_tweaked_keypair, internal_key, parity, output_key) =
        taproot_keypair(40, Some(tapleaf_hash));
    let discouragement_script_pubkey = taproot_script_pubkey(&output_key);
    let control = {
        let mut bytes = vec![control_prefix(TAPROOT_LEAF_TAPSCRIPT, parity)];
        bytes.extend_from_slice(&internal_key.serialize());
        bytes
    };
    let (discourage_spent_input, discourage_validation_context, discourage_precomputed) =
        legacy_context(
            discouragement_script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::TAPROOT
                | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_PUBKEYTYPE,
        );
    let witness = ScriptWitness::new(vec![vec![1_u8; 64], tapscript_bytes, control]);
    let error = super::verify_input_script(ScriptInputVerificationContext {
        script_sig: &ScriptBuf::default(),
        script_pubkey: &discouragement_script_pubkey,
        witness: &witness,
        transaction: &transaction,
        input_index: 0,
        spent_input: &discourage_spent_input,
        validation_context: &discourage_validation_context,
        spent_amount: discourage_spent_input.spent_output.value,
        verify_flags: ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::WITNESS
            | ScriptVerifyFlags::TAPROOT
            | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_PUBKEYTYPE,
        precomputed: &discourage_precomputed,
        execution_data: &mut execution_data,
    })
    .expect_err("upgradable tapscript pubkey types should be discouraged");
    assert_eq!(error, ScriptError::UnsupportedOpcode(OP_CHECKSIGADD));
    let _ = script_key;
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
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::CLEANSTACK,
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
    let (checksigverify_input, checksigverify_context, checksigverify_precomputed) = legacy_context(
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
    let (checkmultisigverify_input, checkmultisigverify_context, checkmultisigverify_precomputed) =
        legacy_context(
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
    let checkmultisigverify_script_sig = push_only_script(&[&[], &checkmultisigverify_signature]);

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
    let execution_context = LegacyExecutionContext {
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
        execute_checksig(&mut Vec::new(), &script(&[0xad]), None, None, true)
            .expect_err("missing checker must fail"),
        ScriptError::UnsupportedOpcode(0xad)
    );
    assert_eq!(
        execute_checkmultisig(&mut Vec::new(), &script(&[0xaf]), None, None, &mut 0, true)
            .expect_err("missing checker must fail"),
        ScriptError::UnsupportedOpcode(0xaf)
    );
    assert_eq!(
        execute_checkmultisig(&mut Vec::new(), &script(&[0xae]), None, None, &mut 0, false)
            .expect_err("missing checker must fail"),
        ScriptError::UnsupportedOpcode(0xae)
    );
    assert_eq!(
        execute_checksig(
            &mut vec![vec![1_u8]],
            &script(&[0xac]),
            Some(&execution_context),
            None,
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
            None,
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
            None,
            &mut 0,
            false,
        )
        .expect_err("too many pubkeys must fail"),
        ScriptError::PubKeyCount
    );
    let mut op_count = MAX_OPS_PER_SCRIPT;
    assert_eq!(
        execute_checkmultisig(
            &mut vec![vec![1]],
            &script(&[0xae]),
            Some(&execution_context),
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
    let nullfail_multisig_context = LegacyExecutionContext {
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
            None,
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
            &ScriptWitness::default(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV1Taproot([1_u8; 32]),
            false,
            &secp,
        ),
        Ok(())
    );
    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::default(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
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
            &ScriptWitness::default(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM,
            &mut execution_data,
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
            &ScriptWitness::default(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
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
            &ScriptWitness::default(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
            &ScriptPubKeyType::NonStandard,
            false,
            &secp,
        ),
        Err(ScriptError::WitnessProgramWrongLength)
    );
    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::new(vec![vec![1_u8]]),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV0KeyHash([1_u8; 20]),
            false,
            &secp,
        ),
        Err(ScriptError::WitnessProgramMismatch)
    );
    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::default(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV0ScriptHash([1_u8; 32]),
            false,
            &secp,
        ),
        Err(ScriptError::WitnessProgramWitnessEmpty)
    );
    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::new(vec![vec![0_u8; 521], script(&[OP_1]).as_bytes().to_vec(),]),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
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
fn eval_script_internal_dispatches_verify_and_tapscript_signature_opcodes() {
    let signing_secp = Secp256k1::new();
    let verify_secp = Secp256k1::verification_only();
    let secret_key = SecretKey::from_byte_array([46_u8; 32]).expect("secret key");
    let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
    let transaction = legacy_transaction(60);
    let script_pubkey = {
        let mut bytes = vec![33];
        bytes.extend_from_slice(&public_key.serialize());
        bytes.push(OP_CHECKSIG);
        script(&bytes)
    };
    let (spent_input, validation_context, precomputed) =
        legacy_context(script_pubkey, &transaction, ScriptVerifyFlags::NONE);
    let execution_context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(&verify_secp, &validation_context, &precomputed),
        transaction: &transaction,
        input_index: 0,
        spent_input: &spent_input,
        verify_flags: ScriptVerifyFlags::NONE,
        sig_version: SigVersion::Base,
    };

    let checksigverify_script = script(&[OP_CHECKSIGVERIFY]);
    let checksigverify_signature = sign_legacy_script(
        &checksigverify_script,
        &transaction,
        &secret_key,
        SigHashType::ALL,
    );
    let mut checksigverify_stack = vec![checksigverify_signature, public_key.serialize().to_vec()];
    eval_script_internal(
        &mut checksigverify_stack,
        &checksigverify_script,
        Some(&execution_context),
        None,
    )
    .expect("CHECKSIGVERIFY dispatch should succeed");
    assert!(checksigverify_stack.is_empty());

    let checkmultisigverify_script = script(&[OP_CHECKMULTISIGVERIFY]);
    let checkmultisigverify_signature = sign_legacy_script(
        &checkmultisigverify_script,
        &transaction,
        &secret_key,
        SigHashType::ALL,
    );
    let mut checkmultisigverify_stack = vec![
        Vec::new(),
        checkmultisigverify_signature,
        encode_script_num(1),
        public_key.serialize().to_vec(),
        encode_script_num(1),
    ];
    eval_script_internal(
        &mut checkmultisigverify_stack,
        &checkmultisigverify_script,
        Some(&execution_context),
        None,
    )
    .expect("CHECKMULTISIGVERIFY dispatch should succeed");
    assert!(checkmultisigverify_stack.is_empty());

    let tapscript = script(&[OP_CHECKSIGADD]);
    let tapscript_leaf_hash = compute_tapleaf_hash(TAPROOT_LEAF_TAPSCRIPT, tapscript.as_bytes());
    let (_taproot_keypair, _internal_key, _parity, output_key) =
        taproot_keypair(61, Some(tapscript_leaf_hash));
    let tapscript_secret_key = SecretKey::from_byte_array([62_u8; 32]).expect("secret key");
    let tapscript_keypair = Keypair::from_secret_key(&signing_secp, &tapscript_secret_key);
    let (tapscript_public_key, _) = XOnlyPublicKey::from_keypair(&tapscript_keypair);
    let taproot_script_pubkey = taproot_script_pubkey(&output_key);
    let (taproot_spent_input, taproot_validation_context, taproot_precomputed) = legacy_context(
        taproot_script_pubkey,
        &transaction,
        ScriptVerifyFlags::TAPROOT,
    );
    let tapscript_context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(
            &verify_secp,
            &taproot_validation_context,
            &taproot_precomputed,
        ),
        transaction: &transaction,
        input_index: 0,
        spent_input: &taproot_spent_input,
        verify_flags: ScriptVerifyFlags::TAPROOT,
        sig_version: SigVersion::Tapscript,
    };
    let mut execution_data = ScriptExecutionData {
        maybe_tapleaf_hash: Some(Hash32::from_byte_array(tapscript_leaf_hash)),
        maybe_codeseparator_position: Some(u32::MAX),
        maybe_validation_weight_left: Some(200),
        ..ScriptExecutionData::default()
    };
    let _digest = crate::sighash::taproot_sighash(
        &execution_data,
        &transaction,
        0,
        SigHashType::DEFAULT,
        SigVersion::Tapscript,
        &taproot_validation_context,
    )
    .expect("tapscript sighash");
    let tapscript_signature = decode_hex(
        "206c2348aa463803ab09643c637262ac905e04f2449aeabff8a26577252cdaa66af20f3731774c5860ec25d8a0394dd7e7b354ce3d3436771060c4a293896519",
    );
    let mut checksigadd_stack = vec![
        tapscript_signature,
        encode_script_num(1),
        tapscript_public_key.serialize().to_vec(),
    ];
    eval_script_internal(
        &mut checksigadd_stack,
        &tapscript,
        Some(&tapscript_context),
        Some(&mut execution_data),
    )
    .expect("CHECKSIGADD dispatch should succeed");
    assert_eq!(decode_script_num(&checksigadd_stack[0]), Ok(2));
}

#[test]
fn execute_checksig_and_tapscript_paths_cover_taproot_edge_cases() {
    let verify_secp = Secp256k1::verification_only();
    let (_taproot_keypair, _internal_key, _parity, output_key) = taproot_keypair(63, None);
    let transaction = legacy_transaction(64);
    let script_pubkey = taproot_script_pubkey(&output_key);
    let (spent_input, validation_context, precomputed) = legacy_context(
        script_pubkey,
        &transaction,
        ScriptVerifyFlags::TAPROOT | ScriptVerifyFlags::NULLFAIL,
    );
    let taproot_context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(&verify_secp, &validation_context, &precomputed),
        transaction: &transaction,
        input_index: 0,
        spent_input: &spent_input,
        verify_flags: ScriptVerifyFlags::TAPROOT | ScriptVerifyFlags::NULLFAIL,
        sig_version: SigVersion::Taproot,
    };
    let mut execution_data = ScriptExecutionData::default();
    let _digest = crate::sighash::taproot_sighash(
        &execution_data,
        &transaction,
        0,
        SigHashType::DEFAULT,
        SigVersion::Taproot,
        &validation_context,
    )
    .expect("taproot sighash");
    let signature = decode_hex(
        "9938161cbeb1e2d75e06896f95f566d5495faeaeba14fe1ea358a97b64cd76bd1dd3f46cf6a4fb80024487ad5953fcc36cce5504e869f548b314a3cf5fdd0d3c",
    );
    let mut success_stack = vec![signature, output_key.serialize().to_vec()];
    execute_checksig(
        &mut success_stack,
        &script(&[OP_CHECKSIG]),
        Some(&taproot_context),
        Some(&mut execution_data),
        false,
    )
    .expect("taproot CHECKSIG should succeed");
    assert_eq!(success_stack, vec![encode_bool(true)]);

    let nullfail_error = execute_checksig(
        &mut vec![vec![2_u8; 64], output_key.serialize().to_vec()],
        &script(&[OP_CHECKSIG]),
        Some(&taproot_context),
        Some(&mut execution_data),
        false,
    )
    .expect_err("invalid taproot signatures should trip NULLFAIL");
    assert_eq!(nullfail_error, ScriptError::SigNullFail);
}

#[test]
fn execute_checksigverify_pops_the_success_result() {
    // Arrange
    let verify_secp = Secp256k1::verification_only();
    let signing_secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([72_u8; 32]).expect("secret key");
    let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
    let transaction = legacy_transaction(73);
    let script_pubkey = {
        let mut bytes = vec![33];
        bytes.extend_from_slice(&public_key.serialize());
        bytes.push(OP_CHECKSIG);
        script(&bytes)
    };
    let (spent_input, validation_context, precomputed) =
        legacy_context(script_pubkey, &transaction, ScriptVerifyFlags::NONE);
    let execution_context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(&verify_secp, &validation_context, &precomputed),
        transaction: &transaction,
        input_index: 0,
        spent_input: &spent_input,
        verify_flags: ScriptVerifyFlags::NONE,
        sig_version: SigVersion::Base,
    };
    let checksigverify_script = script(&[OP_CHECKSIGVERIFY]);
    let checksigverify_signature = sign_legacy_script(
        &checksigverify_script,
        &transaction,
        &secret_key,
        SigHashType::ALL,
    );
    let mut stack = vec![checksigverify_signature, public_key.serialize().to_vec()];

    // Act
    execute_checksig(
        &mut stack,
        &checksigverify_script,
        Some(&execution_context),
        None,
        true,
    )
    .expect("CHECKSIGVERIFY should remove its success marker");

    // Assert
    assert!(stack.is_empty());
}

#[test]
fn execute_checkmultisigverify_pops_the_success_result() {
    // Arrange
    let verify_secp = Secp256k1::verification_only();
    let signing_secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([74_u8; 32]).expect("secret key");
    let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
    let transaction = legacy_transaction(75);
    let script_pubkey = {
        let mut bytes = vec![0x51, 33];
        bytes.extend_from_slice(&public_key.serialize());
        bytes.push(0x51);
        bytes.push(OP_CHECKMULTISIG);
        script(&bytes)
    };
    let (spent_input, validation_context, precomputed) =
        legacy_context(script_pubkey, &transaction, ScriptVerifyFlags::NONE);
    let execution_context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(&verify_secp, &validation_context, &precomputed),
        transaction: &transaction,
        input_index: 0,
        spent_input: &spent_input,
        verify_flags: ScriptVerifyFlags::NONE,
        sig_version: SigVersion::Base,
    };
    let checkmultisigverify_script = script(&[OP_CHECKMULTISIGVERIFY]);
    let checkmultisigverify_signature = sign_legacy_script(
        &checkmultisigverify_script,
        &transaction,
        &secret_key,
        SigHashType::ALL,
    );
    let mut stack = vec![
        Vec::new(),
        checkmultisigverify_signature,
        encode_script_num(1),
        public_key.serialize().to_vec(),
        encode_script_num(1),
    ];

    // Act
    execute_checkmultisig(
        &mut stack,
        &checkmultisigverify_script,
        Some(&execution_context),
        None,
        &mut 0,
        true,
    )
    .expect("CHECKMULTISIGVERIFY should remove its success marker");

    // Assert
    assert!(stack.is_empty());
}

#[test]
fn taproot_witness_program_and_execution_helpers_cover_remaining_paths() {
    let transaction = legacy_transaction(65);
    let secp = Secp256k1::verification_only();
    let (_keypair, internal_key, parity, output_key) = taproot_keypair(66, None);
    let script_pubkey = taproot_script_pubkey(&output_key);
    let (spent_input, validation_context, precomputed) = legacy_context(
        script_pubkey.clone(),
        &transaction,
        ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::TAPROOT,
    );
    let mut witness_stack = Vec::new();
    let mut execution_data = ScriptExecutionData::default();

    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::default(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::TAPROOT,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV1Taproot(output_key.serialize()),
            false,
            &secp,
        ),
        Err(ScriptError::WitnessProgramWitnessEmpty)
    );

    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::new(vec![vec![1_u8; 64]]),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::TAPROOT,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV1Taproot(output_key.serialize()),
            false,
            &secp,
        ),
        Err(ScriptError::VerifyFailed)
    );

    let mismatched_leaf_hash = compute_tapleaf_hash(TAPROOT_LEAF_TAPSCRIPT, &[OP_1]);
    let mismatched_control = {
        let mut bytes = vec![control_prefix(TAPROOT_LEAF_TAPSCRIPT, parity)];
        bytes.extend_from_slice(&internal_key.serialize());
        bytes
    };
    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::new(vec![Vec::new(), vec![OP_1], mismatched_control.clone()]),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::TAPROOT,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV1Taproot(output_key.serialize()),
            false,
            &secp,
        ),
        Err(ScriptError::WitnessProgramMismatch)
    );

    assert_eq!(compute_tapbranch_hash(&[1_u8; 32], &[2_u8; 32]).len(), 32);
    let mut control_with_node = mismatched_control.clone();
    control_with_node.extend_from_slice(&[3_u8; 32]);
    assert_ne!(
        compute_taproot_merkle_root(&control_with_node, mismatched_leaf_hash),
        mismatched_leaf_hash
    );
    assert!(!verify_taproot_commitment(
        &secp,
        &mismatched_control,
        &[0xff_u8; 32],
        mismatched_leaf_hash,
    ));

    let nested_taproot_error = verify_witness_program(
        &mut witness_stack,
        &ScriptWitness::default(),
        &transaction,
        0,
        &spent_input,
        &validation_context,
        &precomputed,
        ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::WITNESS
            | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM,
        &mut execution_data,
        &ScriptPubKeyType::WitnessV1Taproot(output_key.serialize()),
        true,
        &secp,
    )
    .expect_err("nested taproot under P2SH should be discouraged when flagged");
    assert_eq!(
        nested_taproot_error,
        ScriptError::UnsupportedOpcode(OP_0NOTEQUAL)
    );

    let op_success_result = execute_tapscript(
        &mut witness_stack,
        &transaction,
        0,
        &spent_input,
        &validation_context,
        &precomputed,
        ScriptVerifyFlags::TAPROOT,
        &mut execution_data,
        &script(&[0x50]),
        Vec::new(),
        &secp,
    );
    assert_eq!(op_success_result, Ok(()));
    assert_eq!(witness_stack, vec![encode_bool(true)]);

    assert_eq!(
        execute_tapscript(
            &mut Vec::new(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::TAPROOT,
            &mut execution_data,
            &ScriptBuf::default(),
            vec![Vec::new(); MAX_STACK_SIZE + 1],
            &secp,
        ),
        Err(ScriptError::StackOverflow(MAX_STACK_SIZE + 1))
    );
    assert_eq!(
        execute_tapscript(
            &mut Vec::new(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::TAPROOT,
            &mut execution_data,
            &ScriptBuf::default(),
            vec![vec![0_u8; MAX_SCRIPT_ELEMENT_SIZE + 1]],
            &secp,
        ),
        Err(ScriptError::PushSize(MAX_SCRIPT_ELEMENT_SIZE + 1))
    );
    assert_eq!(
        execute_tapscript(
            &mut Vec::new(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::TAPROOT,
            &mut execution_data,
            &ScriptBuf::default(),
            vec![encode_bool(true), encode_bool(true)],
            &secp,
        ),
        Err(ScriptError::WitnessCleanStack)
    );
}

#[test]
fn verify_input_script_hands_nested_taproot_programs_to_the_witness_verifier() {
    let transaction = legacy_transaction(67);
    let (_keypair, _internal_key, _parity, output_key) = taproot_keypair(68, None);
    let redeem_script = taproot_script_pubkey(&output_key);
    let redeem_hash = hash160(redeem_script.as_bytes());
    let script_pubkey = {
        let mut bytes = vec![0xa9, 20];
        bytes.extend_from_slice(&redeem_hash);
        bytes.push(0x87);
        script(&bytes)
    };
    let (spent_input, validation_context, precomputed) = legacy_context(
        script_pubkey.clone(),
        &transaction,
        ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
    );
    let mut execution_data = ScriptExecutionData::default();

    assert_eq!(
        super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &push_only_script(&[redeem_script.as_bytes()]),
            script_pubkey: &script_pubkey,
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
}

#[test]
fn verify_input_script_keeps_non_witness_p2sh_redeems_on_the_legacy_path() {
    let transaction = legacy_transaction(69);
    let redeem_script = script(&[OP_1]);
    let redeem_hash = hash160(redeem_script.as_bytes());
    let script_pubkey = {
        let mut bytes = vec![0xa9, 20];
        bytes.extend_from_slice(&redeem_hash);
        bytes.push(0x87);
        script(&bytes)
    };
    let (spent_input, validation_context, precomputed) = legacy_context(
        script_pubkey.clone(),
        &transaction,
        ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
    );
    let mut execution_data = ScriptExecutionData::default();

    assert_eq!(
        super::verify_input_script(ScriptInputVerificationContext {
            script_sig: &push_only_script(&[redeem_script.as_bytes()]),
            script_pubkey: &script_pubkey,
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
}

#[test]
fn taproot_helper_branches_are_covered() {
    let transaction = legacy_transaction(46);
    let (_keypair, internal_key, parity, output_key) = taproot_keypair(47, None);
    let script_pubkey = taproot_script_pubkey(&output_key);
    let (spent_input, validation_context, precomputed) = legacy_context(
        script_pubkey.clone(),
        &transaction,
        ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
    );
    let secp = Secp256k1::verification_only();
    let mut execution_data = ScriptExecutionData::default();
    let mut witness_stack = Vec::new();

    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::new(vec![vec![1_u8; 64]]),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV1Taproot(output_key.serialize()),
            false,
            &secp,
        ),
        Ok(())
    );

    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::new(vec![vec![1_u8; 64]]),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::TAPROOT,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV1Taproot(output_key.serialize()),
            true,
            &secp,
        ),
        Ok(())
    );

    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::new(vec![vec![1_u8; 64], vec![1_u8; 10]]),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::TAPROOT,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV1Taproot(output_key.serialize()),
            false,
            &secp,
        ),
        Err(ScriptError::WitnessProgramWrongLength)
    );
    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::default(),
            &transaction,
            0,
            &spent_input,
            &validation_context,
            &precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
            &ScriptPubKeyType::WitnessUnknown {
                version: 0,
                program: vec![1_u8; 31],
            },
            true,
            &secp,
        ),
        Err(ScriptError::WitnessProgramWrongLength)
    );

    let non_tapscript_tapleaf_hash = compute_tapleaf_hash(0xc2, &[0x51]);
    let (
        _kp_non_tapscript,
        internal_key_non_tapscript,
        parity_non_tapscript,
        output_key_non_tapscript,
    ) = taproot_keypair(50, Some(non_tapscript_tapleaf_hash));
    let non_tapscript_control = {
        let mut bytes = vec![control_prefix(0xc2, parity_non_tapscript)];
        bytes.extend_from_slice(&internal_key_non_tapscript.serialize());
        bytes
    };
    let non_tapscript_script_pubkey = taproot_script_pubkey(&output_key_non_tapscript);
    let (non_tapscript_spent_input, non_tapscript_validation_context, non_tapscript_precomputed) =
        legacy_context(
            non_tapscript_script_pubkey.clone(),
            &transaction,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::TAPROOT,
        );
    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::new(vec![
                vec![1_u8; 64],
                vec![0x51],
                non_tapscript_control.clone()
            ]),
            &transaction,
            0,
            &non_tapscript_spent_input,
            &non_tapscript_validation_context,
            &non_tapscript_precomputed,
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS | ScriptVerifyFlags::TAPROOT,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV1Taproot(output_key_non_tapscript.serialize()),
            false,
            &secp,
        ),
        Ok(())
    );
    assert_eq!(
        verify_witness_program(
            &mut witness_stack,
            &ScriptWitness::new(vec![vec![1_u8; 64], vec![0x51], non_tapscript_control]),
            &transaction,
            0,
            &non_tapscript_spent_input,
            &non_tapscript_validation_context,
            &non_tapscript_precomputed,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::TAPROOT
                | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_TAPROOT_VERSION,
            &mut execution_data,
            &ScriptPubKeyType::WitnessV1Taproot(output_key_non_tapscript.serialize()),
            false,
            &secp,
        ),
        Err(ScriptError::UnsupportedOpcode(OP_CHECKSIGADD))
    );

    let left = [2_u8; 32];
    let right = [1_u8; 32];
    assert_ne!(compute_tapbranch_hash(&left, &right), left);
    let mut bad_control = vec![0_u8; TAPROOT_CONTROL_BASE_SIZE];
    bad_control[0] = 1;
    assert!(!verify_taproot_commitment(
        &secp,
        &bad_control,
        &output_key.serialize(),
        [0_u8; 32],
    ));
    let another_output_key = {
        let (_kp, _, _, out) = taproot_keypair(48, None);
        out
    };
    let good_control = {
        let mut bytes = vec![control_prefix(0, parity)];
        bytes.extend_from_slice(&internal_key.serialize());
        bytes
    };
    assert!(!verify_taproot_commitment(
        &secp,
        &good_control,
        &another_output_key.serialize(),
        [0_u8; 32],
    ));
    assert_eq!(compact_size_len(1), 1);
    assert_eq!(compact_size_len(253), 3);
    assert_eq!(compact_size_len(65_536), 5);
    assert_eq!(compact_size_len(u64::MAX), 9);
    let mut compact = Vec::new();
    write_compact_size(&mut compact, 253);
    write_compact_size(&mut compact, 65_536);
    write_compact_size(&mut compact, u64::MAX);
    assert_eq!(compact[0], 0xfd);
    assert_eq!(compact[3], 0xfe);
    assert_eq!(compact[8], 0xff);
    assert!(!is_op_success(OP_CHECKSIG));
}

#[test]
fn op_success_boundary_ranges_match_taproot_allowlist() {
    assert_eq!(OP_RESERVED, 0x50);
    assert_eq!(OP_VER, 0x62);

    assert!(is_op_success(OP_RESERVED));
    assert!(is_op_success(OP_VER));
    assert!(is_op_success(0x7e));
    assert!(is_op_success(0x81));
    assert!(is_op_success(0xbb));
    assert!(is_op_success(0xfe));

    assert!(!is_op_success(0x7d));
    assert!(!is_op_success(0x82));
    assert!(!is_op_success(0xba));
    assert!(!is_op_success(0xff));
    assert!(!is_op_success(OP_CHECKSIG));
}

#[test]
fn tapscript_opcode_edge_cases_are_covered() {
    let transaction = legacy_transaction(49);
    let validation_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script(&[0x51]),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::TAPROOT,
        consensus_params: Default::default(),
    };
    let precomputed = validation_context
        .precompute(&transaction)
        .expect("precompute");
    let secp = Secp256k1::verification_only();
    let checker = TransactionSignatureChecker::new(&secp, &validation_context, &precomputed);
    let tapscript_context = LegacyExecutionContext {
        checker,
        transaction: &transaction,
        input_index: 0,
        spent_input: &validation_context.inputs[0],
        verify_flags: ScriptVerifyFlags::TAPROOT
            | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_PUBKEYTYPE,
        sig_version: SigVersion::Tapscript,
    };
    let mut execution_data = ScriptExecutionData {
        maybe_tapleaf_hash: Some(Hash32::from_byte_array([9_u8; 32])),
        maybe_codeseparator_position: Some(0),
        maybe_validation_weight_left: Some(10),
        ..ScriptExecutionData::default()
    };

    assert_eq!(
        execute_checksigadd(&mut Vec::new(), &script(&[OP_CHECKSIGADD]), None, None)
            .expect_err("missing context must fail"),
        ScriptError::UnsupportedOpcode(OP_CHECKSIGADD)
    );
    let base_context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(&secp, &validation_context, &precomputed),
        transaction: &transaction,
        input_index: 0,
        spent_input: &validation_context.inputs[0],
        verify_flags: ScriptVerifyFlags::TAPROOT
            | ScriptVerifyFlags::DISCOURAGE_UPGRADABLE_PUBKEYTYPE,
        sig_version: SigVersion::Base,
    };
    assert_eq!(
        execute_checksigadd(
            &mut vec![vec![1_u8], vec![1_u8], vec![1_u8]],
            &script(&[OP_CHECKSIGADD]),
            Some(&base_context),
            Some(&mut execution_data),
        )
        .expect_err("non-tapscript CHECKSIGADD must fail"),
        ScriptError::UnsupportedOpcode(OP_CHECKSIGADD)
    );
    assert_eq!(
        execute_checksigadd(
            &mut vec![vec![1_u8], vec![1_u8]],
            &script(&[OP_CHECKSIGADD]),
            Some(&tapscript_context),
            Some(&mut execution_data),
        )
        .expect_err("stack underflow must fail"),
        ScriptError::InvalidStackOperation
    );
    assert_eq!(
        execute_tapscript_checksig(&tapscript_context, &mut execution_data, &[], &[])
            .expect_err("empty pubkeys must fail"),
        ScriptError::PubKeyType
    );
    execution_data.maybe_validation_weight_left = Some(0);
    assert_eq!(
        execute_tapscript_checksig(
            &tapscript_context,
            &mut execution_data,
            &[1_u8; 64],
            &[1_u8; 32],
        )
        .expect_err("weight underflow must fail"),
        ScriptError::VerifyFailed
    );
    execution_data.maybe_validation_weight_left = Some(100);
    assert_eq!(
        execute_tapscript_checksig(&tapscript_context, &mut execution_data, &[], &[1_u8; 32]),
        Ok(false)
    );
    assert_eq!(
        execute_tapscript_checksig(
            &tapscript_context,
            &mut execution_data,
            &[1_u8; 64],
            &[1_u8; 33],
        )
        .expect_err("unknown pubkey type must be discouraged"),
        ScriptError::UnsupportedOpcode(OP_CHECKSIGADD)
    );
    let mut stack = vec![vec![1_u8; 64], vec![1_u8], vec![1_u8; 33]];
    let relaxed_tapscript_context = LegacyExecutionContext {
        checker: TransactionSignatureChecker::new(&secp, &validation_context, &precomputed),
        transaction: &transaction,
        input_index: 0,
        spent_input: &validation_context.inputs[0],
        verify_flags: ScriptVerifyFlags::TAPROOT,
        sig_version: SigVersion::Tapscript,
    };
    assert_eq!(
        execute_checksigadd(
            &mut stack,
            &script(&[OP_CHECKSIGADD]),
            Some(&relaxed_tapscript_context),
            Some(&mut execution_data),
        ),
        Ok(())
    );
    assert_eq!(decode_script_num(&stack[0]), Ok(2));
    let tapscript_multisig_error = execute_checkmultisig(
        &mut vec![vec![1_u8]],
        &script(&[OP_CHECKMULTISIG]),
        Some(&tapscript_context),
        Some(&mut execution_data),
        &mut 0,
        false,
    )
    .expect_err("CHECKMULTISIG is disabled in tapscript");
    assert_eq!(
        tapscript_multisig_error,
        ScriptError::UnsupportedOpcode(OP_CHECKMULTISIG)
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
