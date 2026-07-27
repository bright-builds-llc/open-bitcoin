// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

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
use super::witness::{verify_input_script, verify_witness_program};
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

mod eval_script_internal_dispatches_verify_and_tapscript_signature_opcodes;
mod legacy_helper_error_paths_are_covered;
mod verify_input_script_accepts_pay_to_pubkey_signatures;
mod verify_input_script_enforces_witness_malleation_and_pubkey_rules;
mod verify_input_script_keeps_non_witness_p2sh_redeems_on_the_legacy_path;
mod verify_input_script_rejects_invalid_bare_multisig_forms;
mod verify_script_matches_knots_equal_vector;
mod witness_and_sigop_helpers_are_covered;
