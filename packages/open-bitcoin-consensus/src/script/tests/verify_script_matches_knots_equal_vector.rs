use super::*;

#[test]
fn verify_script_matches_knots_equal_vector() {
    let script_sig = script(&[0x51, 0x52]);
    let script_pubkey = script(&[0x52, 0x88, 0x51, 0x87]);

    assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
}

#[test]
fn verify_script_matches_knots_add_vector() {
    let script_sig = script(&[0x51, 0x51]);
    let script_pubkey = script(&[0x93, 0x52, 0x87]);

    assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
}

#[test]
fn verify_script_matches_knots_sha256_vector() {
    let script_sig = script(&[0x01, 0x61]);
    let script_pubkey = script(&[
        0xa8, 0x20, 0xca, 0x97, 0x81, 0x12, 0xca, 0x1b, 0xbd, 0xca, 0xfa, 0xc2, 0x31, 0xb3, 0x9a,
        0x23, 0xdc, 0x4d, 0xa7, 0x86, 0xef, 0xf8, 0x14, 0x7c, 0x4e, 0x72, 0xb9, 0x80, 0x77, 0x85,
        0xaf, 0xee, 0x48, 0xbb, 0x87,
    ]);

    assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
}

#[test]
fn verify_script_matches_knots_hash256_vector() {
    let script_sig = script(&[0x01, 0x61]);
    let script_pubkey = script(&[
        0xaa, 0x20, 0xbf, 0x5d, 0x3a, 0xff, 0xb7, 0x3e, 0xfd, 0x2e, 0xc6, 0xc3, 0x6a, 0xd3, 0x11,
        0x2d, 0xd9, 0x33, 0xef, 0xed, 0x63, 0xc4, 0xe1, 0xcb, 0xff, 0xcf, 0xa8, 0x8e, 0x27, 0x59,
        0xc1, 0x44, 0xf2, 0xd8, 0x87,
    ]);

    assert_eq!(verify_script(&script_sig, &script_pubkey), Ok(()));
}

#[test]
fn verify_script_rejects_false_final_stack() {
    let error = verify_script(&script(&[]), &script(&[0x00])).expect_err("false stack must fail");

    assert_eq!(error, ScriptError::EvalFalse);
}

#[test]
fn verify_script_rejects_empty_stack_after_execution() {
    let error = verify_script(&script(&[]), &script(&[])).expect_err("empty final stack must fail");

    assert_eq!(error, ScriptError::EvalFalse);
}

#[test]
fn verify_script_rejects_op_return() {
    let error = verify_script(&script(&[]), &script(&[0x6a])).expect_err("OP_RETURN must fail");

    assert_eq!(error, ScriptError::OpReturn);
}

#[test]
fn count_legacy_sigops_skips_push_data() {
    let sigops =
        count_legacy_sigops(&script(&[0x01, 0xac, 0xac, 0xae])).expect("sigops should parse");

    assert_eq!(sigops, 21);
}

#[test]
fn eval_script_reports_stack_overflow() {
    let pushes = vec![0x51; 1001];
    let error =
        eval_script(&mut Vec::new(), &script(&pushes)).expect_err("too many pushes must fail");

    assert_eq!(error, ScriptError::StackOverflow(1001));
}

#[test]
fn script_error_display_covers_all_variants() {
    let cases = [
        (ScriptError::BadOpcode, "bad opcode"),
        (ScriptError::DisabledOpcode(0x7e), "disabled opcode: 0x7e"),
        (ScriptError::EvalFalse, "script evaluated to false"),
        (
            ScriptError::InvalidStackOperation,
            "invalid stack operation",
        ),
        (
            ScriptError::NumOverflow(5),
            "script number overflow: 5 bytes",
        ),
        (ScriptError::OpCount, "script exceeds opcode limit"),
        (ScriptError::OpReturn, "OP_RETURN encountered"),
        (ScriptError::PubKeyCount, "invalid public key count"),
        (ScriptError::PubKeyType, "invalid public key encoding"),
        (
            ScriptError::PushSize(521),
            "push exceeds stack element limit: 521 bytes",
        ),
        (ScriptError::SigCount, "invalid signature count"),
        (ScriptError::SigDer, "invalid DER signature"),
        (ScriptError::SigHashType, "invalid signature hash type"),
        (ScriptError::SigHighS, "non-low-S signature"),
        (
            ScriptError::SigNullDummy,
            "non-null CHECKMULTISIG dummy argument",
        ),
        (ScriptError::SigNullFail, "non-null failing signature"),
        (ScriptError::SigPushOnly, "scriptSig is not push-only"),
        (
            ScriptError::StackOverflow(1001),
            "stack exceeds maximum size: 1001",
        ),
        (ScriptError::TruncatedPushData, "truncated pushdata"),
        (ScriptError::UnbalancedConditional, "unbalanced conditional"),
        (
            ScriptError::UnsupportedOpcode(0xac),
            "unsupported opcode: 0xac",
        ),
        (ScriptError::VerifyFailed, "VERIFY failed"),
        (
            ScriptError::WitnessCleanStack,
            "witness script did not leave a clean stack",
        ),
        (
            ScriptError::WitnessMalleated,
            "witness program has unexpected scriptSig",
        ),
        (
            ScriptError::WitnessMalleatedP2sh,
            "nested witness program scriptSig is malleated",
        ),
        (
            ScriptError::WitnessProgramMismatch,
            "witness program mismatch",
        ),
        (
            ScriptError::WitnessProgramWitnessEmpty,
            "witness program witness stack is empty",
        ),
        (
            ScriptError::WitnessProgramWrongLength,
            "witness program wrong length",
        ),
        (
            ScriptError::WitnessPubKeyType,
            "witness public key must be compressed",
        ),
        (ScriptError::WitnessUnexpected, "unexpected witness data"),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn helpers_cover_bool_number_and_disabled_opcode_edges() {
    assert!(!cast_to_bool(&[0x80]));
    assert!(!cast_to_bool(&[0x00]));
    assert!(cast_to_bool(&[0x01]));
    assert_eq!(decode_script_num(&[]), Ok(0));
    assert_eq!(decode_script_num(&[0x81]), Ok(-1));
    assert_eq!(decode_script_num(&[0x01, 0x80]), Ok(-1));
    assert_eq!(decode_script_num(&[0; 5]), Err(ScriptError::NumOverflow(5)));
    assert_eq!(encode_script_num(0), Vec::<u8>::new());
    assert_eq!(encode_script_num(-1), vec![0x81]);
    assert_eq!(encode_script_num(128), vec![0x80, 0x00]);
    assert!(is_disabled_opcode(0x7e));
    assert!(!is_disabled_opcode(0x51));
    assert_eq!(decode_small_int_opcode(0x51), Some(1));
    assert_eq!(decode_small_int_opcode(0x61), None);
}

#[test]
fn low_level_script_helpers_cover_remaining_direct_paths() {
    assert_eq!(
        verify_top_stack_true(&[Vec::new()]).expect_err("false stack top must fail"),
        ScriptError::EvalFalse
    );

    let untouched = script(&[0x51, 0x51]);
    assert_eq!(
        remove_signature_from_script(&untouched, &[0xaa, 0xbb]),
        untouched
    );

    assert_eq!(
        witness_sigops_for_type(&ScriptPubKeyType::NonStandard, &ScriptWitness::default())
            .expect("helper should succeed"),
        None
    );
    assert_eq!(
        witness_sigops_for_type(
            &ScriptPubKeyType::WitnessV0ScriptHash([0_u8; 32]),
            &ScriptWitness::default(),
        )
        .expect("empty witness script should not count sigops"),
        None
    );
}

#[test]
fn condition_stack_and_control_flow_helpers_are_covered() {
    let mut condition_stack = ConditionStack::default();
    assert!(condition_stack.is_empty());
    assert!(condition_stack.all_true());
    assert!(condition_stack.outer_all_true());
    condition_stack.push(true);
    condition_stack.push(false);
    assert!(!condition_stack.all_true());
    assert!(condition_stack.outer_all_true());
    condition_stack.toggle_top().expect("toggle should succeed");
    assert!(condition_stack.all_true());
    assert_eq!(condition_stack.pop(), Some(true));
    assert_eq!(condition_stack.pop(), Some(true));
    assert_eq!(
        condition_stack
            .toggle_top()
            .expect_err("empty toggle should fail"),
        ScriptError::UnbalancedConditional
    );

    let mut stack = Vec::new();
    eval_script(
        &mut stack,
        &script(&[0x00, OP_IF, 0x01, 0x01, OP_ENDIF, OP_1]),
    )
    .expect("inactive branch pushes should be skipped");
    assert_eq!(stack, vec![vec![1_u8]]);

    let mut stack = Vec::new();
    eval_script(
        &mut stack,
        &script(&[OP_1, OP_NOTIF, OP_1, OP_ELSE, OP_1, OP_ENDIF]),
    )
    .expect("NOTIF/ELSE should execute");
    assert_eq!(stack, vec![vec![1_u8]]);

    let mut stack = Vec::new();
    eval_script(
        &mut stack,
        &script(&[0x00, OP_IF, OP_1, OP_IF, OP_ELSE, OP_ENDIF, OP_ENDIF, OP_1]),
    )
    .expect("nested inactive branches should parse and skip execution");
    assert_eq!(stack, vec![vec![1_u8]]);

    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[OP_ENDIF])).expect_err("ENDIF without IF must fail"),
        ScriptError::UnbalancedConditional
    );
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[OP_1, OP_IF]))
            .expect_err("unterminated IF must fail"),
        ScriptError::UnbalancedConditional
    );
    assert_eq!(
        verify_top_stack_true(&[]).expect_err("empty stack must fail"),
        ScriptError::EvalFalse
    );
}

#[test]
fn eval_script_supports_stack_and_numeric_helpers() {
    let mut stack = Vec::new();
    eval_script(
        &mut stack,
        &script(&[
            0x4f, 0x75, 0x51, 0x52, 0x78, 0x7c, 0x75, 0x82, 0x75, 0x8b, 0x8c, 0x8f, 0x8f, 0x91,
            0x92, 0x51, 0x51, 0x94,
        ]),
    )
    .expect("script should execute");

    assert_eq!(stack, vec![vec![1_u8], Vec::<u8>::new(), Vec::<u8>::new()]);
}

#[test]
fn eval_script_covers_dup_and_boolean_binary_ops() {
    let mut stack = Vec::new();
    eval_script(&mut stack, &script(&[0x51, 0x76, 0x51, 0x9a, 0x00, 0x9b]))
        .expect("dup/bool ops should execute");

    assert_eq!(stack, vec![vec![1_u8], vec![1_u8]]);
}

#[test]
fn eval_script_covers_false_boolean_binary_ops() {
    let mut stack = Vec::new();
    eval_script(&mut stack, &script(&[0x00, 0x51, 0x9a, 0x00, 0x00, 0x9b]))
        .expect("false bool ops should execute");

    assert_eq!(stack, vec![Vec::<u8>::new(), Vec::<u8>::new()]);
}

#[test]
fn eval_script_supports_boolean_and_comparison_ops() {
    let mut stack = Vec::new();
    eval_script(
        &mut stack,
        &script(&[
            0x51, 0x51, 0x9a, 0x51, 0x00, 0x9b, 0x51, 0x51, 0x9c, 0x51, 0x52, 0x9e, 0x51, 0x52,
            0x9f, 0x52, 0x51, 0xa0, 0x51, 0x52, 0xa3, 0x51, 0x52, 0xa4, 0x51, 0x51, 0x52, 0xa5,
        ]),
    )
    .expect("script should execute");

    assert_eq!(stack.len(), 9);
    assert!(stack.iter().all(|item| cast_to_bool(item)));
}

#[test]
fn eval_script_supports_verify_variants() {
    let mut stack = Vec::new();
    eval_script(&mut stack, &script(&[0x51, 0x69, 0x51, 0x51, 0x9d]))
        .expect("verify variants should succeed");

    assert!(stack.is_empty());
}

#[test]
fn eval_script_verify_false_branch_is_reported() {
    let error =
        eval_script(&mut Vec::new(), &script(&[0x00, 0x69])).expect_err("verify false must fail");

    assert_eq!(error, ScriptError::VerifyFailed);
}

#[test]
fn eval_script_rejects_invalid_stack_operations() {
    let cases = [
        script(&[0x75]),
        script(&[0x76]),
        script(&[0x78]),
        script(&[0x7c]),
        script(&[0x82]),
    ];

    for candidate in cases {
        let error = eval_script(&mut Vec::new(), &candidate).expect_err("empty-stack op must fail");
        assert_eq!(error, ScriptError::InvalidStackOperation);
    }
}

#[test]
fn eval_script_rejects_verify_failures_and_unsupported_opcodes() {
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[0x51, 0x52, 0x88]))
            .expect_err("equalverify mismatch must fail"),
        ScriptError::VerifyFailed,
    );
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[0x51, 0x52, 0x9d]))
            .expect_err("numequalverify mismatch must fail"),
        ScriptError::VerifyFailed,
    );
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[0xac])).expect_err("checksig must be unsupported"),
        ScriptError::UnsupportedOpcode(0xac),
    );
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[0x62]))
            .expect_err("unknown opcode must be unsupported"),
        ScriptError::UnsupportedOpcode(0x62),
    );
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[0x7e])).expect_err("disabled opcode must fail"),
        ScriptError::DisabledOpcode(0x7e),
    );
}

#[test]
fn eval_script_rejects_opcount_and_pushdata_errors() {
    let opcount_script = vec![0x61; 202];
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&opcount_script))
            .expect_err("too many opcodes must fail"),
        ScriptError::OpCount,
    );
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[0x4c])).expect_err("truncated pushdata1 must fail"),
        ScriptError::TruncatedPushData,
    );
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[0x4d, 0x01]))
            .expect_err("truncated pushdata2 must fail"),
        ScriptError::TruncatedPushData,
    );
    assert_eq!(
        eval_script(&mut Vec::new(), &script(&[0x4e, 0x01, 0x00, 0x00]))
            .expect_err("truncated pushdata4 must fail"),
        ScriptError::TruncatedPushData,
    );
    assert_eq!(
        count_legacy_sigops(&script(&[0x01])).expect_err("bad push must fail"),
        ScriptError::TruncatedPushData,
    );
}

#[test]
fn eval_script_accepts_all_pushdata_forms() {
    let mut stack = Vec::new();
    eval_script(
        &mut stack,
        &script(&[
            0x4c, 0x01, 0x05, 0x4d, 0x01, 0x00, 0x06, 0x4e, 0x01, 0x00, 0x00, 0x00, 0x07,
        ]),
    )
    .expect("pushdata variants should execute");

    assert_eq!(stack, vec![vec![0x05], vec![0x06], vec![0x07]]);
}

#[test]
fn eval_script_rejects_oversized_pushes() {
    let mut bytes = vec![0x4d, 0x09, 0x02];
    bytes.extend(vec![0x00; 521]);

    assert_eq!(
        eval_script(&mut Vec::new(), &script(&bytes)).expect_err("oversized push must fail"),
        ScriptError::PushSize(521),
    );
}

#[test]
fn verify_input_script_rejects_unexpected_witness_data() {
    let mut execution_data = ScriptExecutionData::default();
    let transaction = Transaction::default();
    let validation_context = TransactionValidationContext {
        inputs: vec![],
        spend_height: 0,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
        consensus_params: crate::context::ConsensusParams::default(),
    };
    let spent_input = TransactionInputContext {
        spent_output: crate::context::SpentOutput {
            value: Amount::from_sats(0).expect("valid amount"),
            script_pubkey: script(&[0x51]),
            is_coinbase: false,
        },
        created_height: 0,
        created_median_time_past: 0,
    };
    let precomputed = PrecomputedTransactionData::new(&transaction, &[]).expect("precompute");

    let error = super::verify_input_script(ScriptInputVerificationContext {
        script_sig: &script(&[0x51]),
        script_pubkey: &script(&[0x51]),
        witness: &ScriptWitness::new(vec![vec![0x01]]),
        transaction: &transaction,
        input_index: 0,
        spent_input: &spent_input,
        validation_context: &validation_context,
        spent_amount: Amount::from_sats(0).expect("valid amount"),
        verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
        precomputed: &precomputed,
        execution_data: &mut execution_data,
    })
    .expect_err("unexpected witness data must fail");

    assert_eq!(error, ScriptError::WitnessUnexpected);
}
