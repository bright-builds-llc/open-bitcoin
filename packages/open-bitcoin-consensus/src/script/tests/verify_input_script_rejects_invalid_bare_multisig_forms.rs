// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

use super::*;

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
