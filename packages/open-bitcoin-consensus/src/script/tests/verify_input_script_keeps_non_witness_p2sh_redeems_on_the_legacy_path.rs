// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

use super::*;

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
