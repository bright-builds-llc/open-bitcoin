use super::*;

#[test]
fn imported_sighash_vectors_match_upstream() {
    for vector in load_sighash_vectors() {
        let transaction = parse_transaction(&decode_hex(&vector.raw_tx)).expect("vector tx");
        let script_code = script(&decode_hex(&vector.script));
        let mut expected_hash = decode_hex(&vector.expected_hash);
        expected_hash.reverse();
        let digest = legacy_sighash(
            &script_code,
            &transaction,
            vector.input_index,
            SigHashType::from_u32(vector.hash_type),
        );
        assert_eq!(
            digest.to_byte_array().as_slice(),
            expected_hash.as_slice(),
            "legacy sighash mismatch for hash type {}",
            vector.hash_type
        );
    }
}

#[test]
fn imported_script_vectors_match_supported_consensus_surface() {
    for vector in SCRIPT_VECTORS {
        let witness = ScriptWitness::new(
            vector
                .witness_stack
                .iter()
                .map(|item| decode_hex(item))
                .collect(),
        );
        let script_sig = parse_script_expr(vector.script_sig);
        let script_pubkey = parse_script_expr(vector.script_pubkey);
        let verify_flags = parse_flags(vector.flags);
        let credit_tx = build_crediting_transaction(&script_pubkey, vector.amount_sats);
        let transaction = build_spending_transaction(&script_sig, &witness, &credit_tx);
        let (spent_input, context) = build_context(
            &transaction,
            &script_pubkey,
            vector.amount_sats,
            verify_flags,
        );
        let precomputed = context.precompute(&transaction).expect("precompute");
        let mut execution_data = ScriptExecutionData::default();

        let result = verify_input_script(ScriptInputVerificationContext {
            script_sig: &script_sig,
            script_pubkey: &script_pubkey,
            witness: &witness,
            transaction: &transaction,
            input_index: 0,
            spent_input: &spent_input,
            validation_context: &context,
            spent_amount: spent_input.spent_output.value,
            verify_flags,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        });

        match vector.expected {
            "OK" => assert!(result.is_ok(), "{} should pass", vector.comment),
            expected => {
                let error = result.expect_err(vector.comment);
                assert_eq!(
                    core_error_name(&error),
                    expected,
                    "unexpected script error for {}",
                    vector.comment
                );
            }
        }
    }
}
