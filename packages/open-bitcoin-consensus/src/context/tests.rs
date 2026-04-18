use open_bitcoin_codec::CodecError;
use open_bitcoin_primitives::{Amount, BlockHeader, ScriptBuf, Transaction};
use open_bitcoin_primitives::{MAX_MONEY, TransactionInput, TransactionOutput, Txid};

use super::{
    BlockValidationContext, ConsensusParams, PrecomputedTransactionData, ScriptVerifyFlags,
    SpentOutput, TransactionInputContext, TransactionValidationContext, calculate_sequence_locks,
    check_tx_inputs, evaluate_sequence_locks, is_final_transaction, sequence_locks,
    write_compact_size,
};

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn sample_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([7_u8; 32]),
                vout: 0,
            },
            script_sig: script(&[0x51]),
            sequence: 5,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 10,
    }
}

fn sample_context() -> TransactionValidationContext {
    TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script(&[0x52, 0x87]),
                is_coinbase: false,
            },
            created_height: 3,
            created_median_time_past: 1_000,
        }],
        spend_height: 10,
        block_time: 2_000,
        median_time_past: 2_000,
        verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        consensus_params: ConsensusParams::default(),
    }
}

#[test]
fn finality_uses_locktime_threshold_and_sequences() {
    let mut transaction = sample_transaction();
    let params = ConsensusParams::default();

    assert!(!is_final_transaction(&transaction, 5, 2_000, &params));

    transaction.inputs[0].sequence = TransactionInput::SEQUENCE_FINAL;
    assert!(is_final_transaction(&transaction, 5, 2_000, &params));

    transaction.lock_time = params.locktime_threshold + 1;
    transaction.inputs[0].sequence = 1;
    assert!(!is_final_transaction(
        &transaction,
        10,
        i64::from(params.locktime_threshold),
        &params,
    ));

    transaction.lock_time = 1;
    assert!(is_final_transaction(&transaction, 10, 0, &params));
}

#[test]
fn sequence_lock_helpers_match_height_and_time_modes() {
    let mut transaction = sample_transaction();
    let context = sample_context();

    let locks = calculate_sequence_locks(&transaction, &context).expect("locks");
    assert_eq!(locks, (7, -1));
    assert!(evaluate_sequence_locks(&context, locks));
    assert!(sequence_locks(&transaction, &context).expect("sequence locks"));

    transaction.inputs[0].sequence = TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1;
    let locks = calculate_sequence_locks(&transaction, &context).expect("locks");
    assert_eq!(
        locks,
        (
            -1,
            context.inputs[0].created_median_time_past
                + (1_i64 << TransactionInput::SEQUENCE_LOCKTIME_GRANULARITY)
                - 1,
        ),
    );
}

#[test]
fn check_tx_inputs_returns_fee_and_rejects_premature_coinbase() {
    let transaction = sample_transaction();
    let fee = check_tx_inputs(&transaction, &sample_context()).expect("fee");
    assert_eq!(fee.to_sats(), 10);

    let mut context = sample_context();
    context.inputs[0].spent_output.is_coinbase = true;
    context.inputs[0].created_height = 9;
    let error =
        check_tx_inputs(&transaction, &context).expect_err("premature coinbase spend must fail");

    assert_eq!(error.reject_reason, "bad-txns-premature-spend-of-coinbase");
}

#[test]
fn precomputed_transaction_data_hashes_change_with_inputs() {
    let transaction = sample_transaction();
    let data = PrecomputedTransactionData::new(&transaction, &sample_context().inputs)
        .expect("precomputed data");

    assert_ne!(data.hash_prevouts.to_byte_array(), [0_u8; 32]);
    assert_ne!(data.hash_sequence.to_byte_array(), [0_u8; 32]);
    assert_ne!(data.hash_outputs.to_byte_array(), [0_u8; 32]);
    assert!(data.maybe_spent_amounts_hash.is_some());
    assert!(data.maybe_spent_scripts_hash.is_some());
}

#[test]
fn flag_helpers_and_precompute_edge_cases_are_exercised() {
    let mut flags = ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS;
    flags |= ScriptVerifyFlags::CHECKSEQUENCEVERIFY;

    assert!(flags.contains(ScriptVerifyFlags::P2SH));
    assert!(flags.contains(ScriptVerifyFlags::WITNESS));
    assert!(flags.contains(ScriptVerifyFlags::CHECKSEQUENCEVERIFY));
    assert_eq!(flags.bits(), 0xc01);
    assert_eq!(flags.to_string(), "0xc01");

    let transaction = sample_transaction();
    let empty_data =
        PrecomputedTransactionData::new(&transaction, &[]).expect("empty inputs are allowed");
    assert!(empty_data.maybe_spent_amounts_hash.is_none());
    assert!(empty_data.maybe_spent_scripts_hash.is_none());

    let error = PrecomputedTransactionData::new(
        &transaction,
        &[
            sample_context().inputs[0].clone(),
            sample_context().inputs[0].clone(),
        ],
    )
    .expect_err("mismatched inputs must fail");
    assert!(matches!(
        error,
        CodecError::LengthOutOfRange {
            field: "spent_outputs",
            value: 2
        }
    ));
}

#[test]
fn finality_short_circuit_and_sequence_lock_edge_cases_are_covered() {
    let mut transaction = sample_transaction();
    let params = ConsensusParams::default();

    transaction.lock_time = 0;
    assert!(is_final_transaction(&transaction, 1, 0, &params));

    let mut context = sample_context();
    let error = calculate_sequence_locks(
        &transaction,
        &TransactionValidationContext {
            inputs: vec![],
            ..context.clone()
        },
    )
    .expect_err("missing inputs must fail");
    assert_eq!(error.reject_reason, "bad-txns-inputs-missingorspent");

    transaction.inputs[0].sequence = TransactionInput::SEQUENCE_LOCKTIME_DISABLE_FLAG | 1;
    let locks = calculate_sequence_locks(&transaction, &context).expect("locks");
    assert_eq!(locks, (-1, -1));

    context.spend_height = 7;
    assert!(!evaluate_sequence_locks(&context, (7, -1)));
    assert!(!evaluate_sequence_locks(
        &context,
        (-1, context.median_time_past)
    ));
}

#[test]
fn check_tx_inputs_covers_mismatch_and_value_failures() {
    let transaction = sample_transaction();
    let error = check_tx_inputs(
        &transaction,
        &TransactionValidationContext {
            inputs: vec![],
            ..sample_context()
        },
    )
    .expect_err("missing inputs must fail");
    assert_eq!(error.reject_reason, "bad-txns-inputs-missingorspent");

    let coinbase_fee = check_tx_inputs(
        &Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint::null(),
                script_sig: script(&[0x01, 0x01]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: Default::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        },
        &sample_context(),
    )
    .expect("coinbase fee should be zero");
    assert_eq!(coinbase_fee.to_sats(), 0);

    let overflow_context = TransactionValidationContext {
        inputs: vec![
            TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(MAX_MONEY).expect("max money"),
                    script_pubkey: script(&[0x51]),
                    is_coinbase: false,
                },
                created_height: 1,
                created_median_time_past: 0,
            },
            TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(MAX_MONEY).expect("max money"),
                    script_pubkey: script(&[0x51]),
                    is_coinbase: false,
                },
                created_height: 1,
                created_median_time_past: 0,
            },
        ],
        ..sample_context()
    };
    let overflow_transaction = Transaction {
        version: 2,
        inputs: vec![
            sample_transaction().inputs[0].clone(),
            TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([8_u8; 32]),
                    vout: 1,
                },
                script_sig: script(&[0x51]),
                sequence: 0,
                witness: Default::default(),
            },
        ],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    assert_eq!(
        check_tx_inputs(&overflow_transaction, &overflow_context)
            .expect_err("overflowing inputs must fail")
            .reject_reason,
        "bad-txns-inputvalues-outofrange",
    );

    let overspend_context = sample_context();
    let overspend_transaction = Transaction {
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(60).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        ..sample_transaction()
    };
    assert_eq!(
        check_tx_inputs(&overspend_transaction, &overspend_context)
            .expect_err("overspend must fail")
            .reject_reason,
        "bad-txns-in-belowout",
    );

    let mut compact = Vec::new();
    write_compact_size(&mut compact, 253);
    write_compact_size(&mut compact, 65_536);
    write_compact_size(&mut compact, u64::MAX);
    assert_eq!(compact[0], 0xfd);
    assert_eq!(compact[3], 0xfe);
    assert_eq!(compact[8], 0xff);
}

#[test]
fn block_context_carries_expected_defaults() {
    let context = BlockValidationContext {
        height: 11,
        previous_header: BlockHeader::default(),
        previous_median_time_past: 1_000,
        consensus_params: ConsensusParams::default(),
    };

    assert_eq!(context.height, 11);
    assert!(context.consensus_params.enforce_segwit);
}
