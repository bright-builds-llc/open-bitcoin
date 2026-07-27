use super::*;

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
