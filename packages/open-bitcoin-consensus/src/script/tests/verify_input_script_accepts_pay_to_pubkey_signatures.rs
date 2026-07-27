// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

use super::*;

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
