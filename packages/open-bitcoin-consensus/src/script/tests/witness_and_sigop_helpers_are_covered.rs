// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

use super::*;

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
            ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
            &mut execution_data,
            &ScriptPubKeyType::PayToAnchor,
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
            &ScriptPubKeyType::PayToAnchor,
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
            &ScriptPubKeyType::PayToAnchor,
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
