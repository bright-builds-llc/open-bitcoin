// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

use super::*;

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
