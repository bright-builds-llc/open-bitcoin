// Parity breadcrumbs:
// - packages/bitcoin-knots/src/key.h
// - packages/bitcoin-knots/src/pubkey.h
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/sign.cpp

use open_bitcoin_consensus::{
    ScriptExecutionData, ScriptInputVerificationContext, ScriptVerifyFlags, SigHashType,
    SigVersion, SpentOutput, TransactionInputContext, TransactionSignatureChecker,
    TransactionValidationContext, taproot_sighash, taproot_tagged_hash, verify_input_script,
};
use open_bitcoin_primitives::{
    Amount, Hash32, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    Txid,
};
use secp256k1::{Keypair, Scalar, Secp256k1, SecretKey, XOnlyPublicKey};

const ANNEX_TAG: u8 = 0x50;
const OP_1: u8 = 0x51;
const OP_CHECKSIGADD: u8 = 0xba;
const OP_CODESEPARATOR: u8 = 0xab;
const TAPROOT_LEAF_TAPSCRIPT: u8 = 0xc0;
const VALIDATION_WEIGHT_OFFSET: i64 = 50;

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
            script_pubkey: script(&[OP_1]),
        }],
        lock_time: 0,
    }
}

fn tap_tweak_scalar(internal_key: &[u8; 32], maybe_merkle_root: Option<[u8; 32]>) -> Scalar {
    let mut preimage = internal_key.to_vec();
    if let Some(merkle_root) = maybe_merkle_root {
        preimage.extend_from_slice(&merkle_root);
    }
    Scalar::from_be_bytes(taproot_tagged_hash("TapTweak", &preimage).to_byte_array())
        .expect("tap tweak")
}

fn taproot_script_pubkey(xonly_public_key: &XOnlyPublicKey) -> ScriptBuf {
    let mut bytes = vec![OP_1, 32];
    bytes.extend_from_slice(&xonly_public_key.serialize());
    script(&bytes)
}

fn compute_tapleaf_hash(leaf_version: u8, script_bytes: &[u8]) -> [u8; 32] {
    let mut data = Vec::with_capacity(script_bytes.len() + 16);
    data.push(leaf_version);
    write_compact_size(&mut data, script_bytes.len() as u64);
    data.extend_from_slice(script_bytes);
    taproot_tagged_hash("TapLeaf", &data).to_byte_array()
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

#[test]
fn taproot_key_path_round_trips_through_canonical_verifier() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([41_u8; 32]).expect("secret key");
    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    let (internal_key, _) = XOnlyPublicKey::from_keypair(&keypair);
    let tweak = tap_tweak_scalar(&internal_key.serialize(), None);
    let tweaked_keypair = keypair.add_xonly_tweak(&secp, &tweak).expect("tap tweak");
    let (output_key, _) = XOnlyPublicKey::from_keypair(&tweaked_keypair);

    let transaction = legacy_transaction(42);
    let script_pubkey = taproot_script_pubkey(&output_key);
    let spent_input = TransactionInputContext {
        spent_output: SpentOutput {
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
        verify_flags: ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::WITNESS
            | ScriptVerifyFlags::TAPROOT,
        consensus_params: Default::default(),
    };
    let precomputed = validation_context
        .precompute(&transaction)
        .expect("precompute");

    let signing_execution_data = ScriptExecutionData {
        maybe_annex: Some(vec![ANNEX_TAG, 0x01]),
        ..ScriptExecutionData::default()
    };
    let digest = taproot_sighash(
        &signing_execution_data,
        &transaction,
        0,
        SigHashType::DEFAULT,
        SigVersion::Taproot,
        &validation_context,
    )
    .expect("taproot sighash");
    let signature = secp
        .sign_schnorr_no_aux_rand(digest.as_bytes(), &tweaked_keypair)
        .as_ref()
        .to_vec();
    let witness = ScriptWitness::new(vec![signature, vec![ANNEX_TAG, 0x01]]);
    let mut execution_data = ScriptExecutionData::default();

    assert_eq!(
        verify_input_script(ScriptInputVerificationContext {
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
                | ScriptVerifyFlags::TAPROOT,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        }),
        Ok(())
    );

    let verification_secp = Secp256k1::verification_only();
    let checker =
        TransactionSignatureChecker::new(&verification_secp, &validation_context, &precomputed);
    assert_eq!(
        checker.verify_schnorr(
            witness.stack()[0].as_slice(),
            &output_key.serialize(),
            &transaction,
            0,
            SigVersion::Taproot,
            &signing_execution_data,
        ),
        Ok(true)
    );
}

#[test]
fn tapscript_script_path_round_trips_through_canonical_verifier() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([43_u8; 32]).expect("secret key");
    let script_keypair = Keypair::from_secret_key(&secp, &secret_key);
    let (script_key, _) = XOnlyPublicKey::from_keypair(&script_keypair);
    let script_bytes = {
        let mut bytes = vec![OP_CODESEPARATOR, 32];
        bytes.extend_from_slice(&script_key.serialize());
        bytes.push(OP_CHECKSIGADD);
        bytes.push(OP_1);
        bytes.push(0x87);
        bytes
    };
    let tapleaf_hash = compute_tapleaf_hash(TAPROOT_LEAF_TAPSCRIPT, &script_bytes);
    let internal_secret_key = SecretKey::from_byte_array([44_u8; 32]).expect("secret key");
    let internal_keypair = Keypair::from_secret_key(&secp, &internal_secret_key);
    let (internal_key, _) = XOnlyPublicKey::from_keypair(&internal_keypair);
    let tweak = tap_tweak_scalar(&internal_key.serialize(), Some(tapleaf_hash));
    let tweaked_keypair = internal_keypair
        .add_xonly_tweak(&secp, &tweak)
        .expect("tap tweak");
    let (output_key, parity) = XOnlyPublicKey::from_keypair(&tweaked_keypair);
    let script_pubkey = taproot_script_pubkey(&output_key);
    let transaction = legacy_transaction(45);
    let spent_input = TransactionInputContext {
        spent_output: SpentOutput {
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
        verify_flags: ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::WITNESS
            | ScriptVerifyFlags::TAPROOT
            | ScriptVerifyFlags::MINIMALIF,
        consensus_params: Default::default(),
    };
    let precomputed = validation_context
        .precompute(&transaction)
        .expect("precompute");

    let control = {
        let mut bytes = vec![
            TAPROOT_LEAF_TAPSCRIPT
                | if parity == secp256k1::Parity::Odd {
                    1
                } else {
                    0
                },
        ];
        bytes.extend_from_slice(&internal_key.serialize());
        bytes
    };
    let annex = vec![ANNEX_TAG, 0x33];
    let witness_for_size = ScriptWitness::new(vec![
        vec![1_u8; 64],
        Vec::new(),
        script_bytes.clone(),
        control.clone(),
        annex.clone(),
    ]);
    let signing_execution_data = ScriptExecutionData {
        maybe_tapleaf_hash: Some(Hash32::from_byte_array(tapleaf_hash)),
        maybe_codeseparator_position: Some(0),
        maybe_annex: Some(annex.clone()),
        maybe_validation_weight_left: Some(
            serialized_witness_size(&witness_for_size) as i64 + VALIDATION_WEIGHT_OFFSET,
        ),
    };
    let digest = taproot_sighash(
        &signing_execution_data,
        &transaction,
        0,
        SigHashType::DEFAULT,
        SigVersion::Tapscript,
        &validation_context,
    )
    .expect("tapscript sighash");
    let signature = secp
        .sign_schnorr_no_aux_rand(digest.as_bytes(), &script_keypair)
        .as_ref()
        .to_vec();
    let witness = ScriptWitness::new(vec![signature, Vec::new(), script_bytes, control, annex]);
    let mut execution_data = ScriptExecutionData::default();

    assert_eq!(
        verify_input_script(ScriptInputVerificationContext {
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
                | ScriptVerifyFlags::MINIMALIF,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        }),
        Ok(())
    );
    assert_eq!(execution_data.maybe_codeseparator_position, Some(0));
    assert_eq!(
        execution_data.maybe_tapleaf_hash,
        Some(Hash32::from_byte_array(tapleaf_hash))
    );
}
