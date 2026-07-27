// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

use super::*;

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
