use std::collections::HashSet;

use open_bitcoin_codec::{CodecError, TransactionEncoding, encode_transaction};
use open_bitcoin_primitives::{Amount, MAX_MONEY, OutPoint, Transaction};

use crate::MAX_BLOCK_WEIGHT;
use crate::context::{
    ScriptExecutionData, ScriptVerifyFlags, SpentOutput, TransactionValidationContext,
    check_tx_inputs, is_final_transaction, sequence_locks,
};
use crate::script::{ScriptInputVerificationContext, verify_input_script};
use crate::validation::{TxValidationError, TxValidationResult, tx_error};

pub fn check_transaction(transaction: &Transaction) -> Result<(), TxValidationError> {
    if transaction.inputs.is_empty() {
        return Err(tx_error(
            TxValidationResult::Consensus,
            "bad-txns-vin-empty",
            None,
        ));
    }
    if transaction.outputs.is_empty() {
        return Err(tx_error(
            TxValidationResult::Consensus,
            "bad-txns-vout-empty",
            None,
        ));
    }

    let stripped_size = encode_transaction(transaction, TransactionEncoding::WithoutWitness)
        .map_err(map_codec_error)?
        .len();
    if stripped_size.saturating_mul(4) > MAX_BLOCK_WEIGHT {
        return Err(tx_error(
            TxValidationResult::Consensus,
            "bad-txns-oversize",
            None,
        ));
    }

    let mut total_output_value = 0_i64;
    for output in &transaction.outputs {
        total_output_value += output.value.to_sats();
        if !(0..=MAX_MONEY).contains(&total_output_value) {
            return Err(tx_error(
                TxValidationResult::Consensus,
                "bad-txns-txouttotal-toolarge",
                None,
            ));
        }
    }

    let mut seen_outpoints = HashSet::<OutPoint>::new();
    for input in &transaction.inputs {
        if !seen_outpoints.insert(input.previous_output.clone()) {
            return Err(tx_error(
                TxValidationResult::Consensus,
                "bad-txns-inputs-duplicate",
                None,
            ));
        }
    }

    if transaction.is_coinbase() {
        let script_len = transaction.inputs[0].script_sig.as_bytes().len();
        if !(2..=100).contains(&script_len) {
            return Err(tx_error(
                TxValidationResult::Consensus,
                "bad-cb-length",
                None,
            ));
        }
        return Ok(());
    }

    for input in &transaction.inputs {
        if input.previous_output.is_null() {
            return Err(tx_error(
                TxValidationResult::Consensus,
                "bad-txns-prevout-null",
                None,
            ));
        }
    }

    Ok(())
}

pub fn validate_transaction_with_context(
    transaction: &Transaction,
    context: &TransactionValidationContext,
) -> Result<Amount, TxValidationError> {
    check_transaction(transaction)?;

    let lock_time_cutoff = if context.consensus_params.enforce_bip113_median_time_past {
        context.median_time_past
    } else {
        context.block_time
    };
    if !is_final_transaction(
        transaction,
        context.spend_height,
        lock_time_cutoff,
        &context.consensus_params,
    ) {
        return Err(tx_error(
            TxValidationResult::Consensus,
            "bad-txns-nonfinal",
            Some("non-final transaction".to_string()),
        ));
    }
    if !sequence_locks(transaction, context)? {
        return Err(tx_error(
            TxValidationResult::PrematureSpend,
            "non-BIP68-final",
            None,
        ));
    }

    let fee = check_tx_inputs(transaction, context)?;
    let precomputed = context.precompute(transaction).map_err(map_codec_error)?;

    for (input_index, (input, input_context)) in
        transaction.inputs.iter().zip(&context.inputs).enumerate()
    {
        let mut execution_data = ScriptExecutionData::default();
        verify_input_script(ScriptInputVerificationContext {
            script_sig: &input.script_sig,
            script_pubkey: &input_context.spent_output.script_pubkey,
            witness: &input.witness,
            transaction,
            input_index,
            spent_input: input_context,
            validation_context: context,
            spent_amount: input_context.spent_output.value,
            verify_flags: context.verify_flags,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .map_err(|error| {
            tx_error(
                TxValidationResult::Consensus,
                "mandatory-script-verify-flag-failed",
                Some(error.to_string()),
            )
        })?;
    }

    Ok(fee)
}

pub fn validate_transaction(
    transaction: &Transaction,
    spent_outputs: &[SpentOutput],
) -> Result<(), TxValidationError> {
    check_transaction(transaction)?;

    if transaction.is_coinbase() {
        return Ok(());
    }

    if spent_outputs.len() != transaction.inputs.len() {
        return Err(tx_error(
            TxValidationResult::MissingInputs,
            "bad-txns-inputs-missingorspent",
            None,
        ));
    }

    let mut total_input_value = 0_i64;
    for spent_output in spent_outputs {
        total_input_value += spent_output.value.to_sats();
        if !(0..=MAX_MONEY).contains(&total_input_value) {
            return Err(tx_error(
                TxValidationResult::Consensus,
                "bad-txns-inputvalues-outofrange",
                None,
            ));
        }
    }

    let total_output_value: i64 = transaction
        .outputs
        .iter()
        .map(|output| output.value.to_sats())
        .sum();
    if total_input_value < total_output_value {
        return Err(tx_error(
            TxValidationResult::Consensus,
            "bad-txns-in-belowout",
            Some(format!(
                "value in ({total_input_value}) < value out ({total_output_value})"
            )),
        ));
    }

    let context = TransactionValidationContext {
        inputs: spent_outputs
            .iter()
            .cloned()
            .map(|spent_output| crate::context::TransactionInputContext {
                spent_output,
                created_height: 0,
                created_median_time_past: 0,
            })
            .collect(),
        spend_height: u32::MAX,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::NONE,
        consensus_params: crate::context::ConsensusParams {
            enforce_bip113_median_time_past: false,
            ..Default::default()
        },
    };
    let _ = validate_transaction_with_context(transaction, &context)?;

    Ok(())
}

fn map_codec_error(error: CodecError) -> TxValidationError {
    tx_error(
        TxValidationResult::Consensus,
        "bad-txns-serialization",
        Some(error.to_string()),
    )
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{
        Amount, MAX_MONEY, ScriptBuf, ScriptWitness, TransactionInput, TransactionOutput, Txid,
    };

    use super::{
        SpentOutput, check_transaction, map_codec_error, validate_transaction,
        validate_transaction_with_context,
    };
    use crate::context::{
        ConsensusParams, ScriptVerifyFlags, TransactionInputContext, TransactionValidationContext,
    };
    use crate::validation::TxValidationResult;

    fn simple_script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn sample_spend_transaction() -> open_bitcoin_primitives::Transaction {
        open_bitcoin_primitives::Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([3_u8; 32]),
                    vout: 0,
                },
                script_sig: simple_script(&[0x52]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(40).expect("valid amount"),
                script_pubkey: simple_script(&[0x51]),
            }],
            lock_time: 0,
        }
    }

    #[test]
    fn check_transaction_rejects_duplicate_inputs() {
        let mut transaction = sample_spend_transaction();
        transaction.inputs.push(transaction.inputs[0].clone());

        let error = check_transaction(&transaction).expect_err("duplicates must fail");

        assert_eq!(error.result, TxValidationResult::Consensus);
        assert_eq!(error.reject_reason, "bad-txns-inputs-duplicate");
    }

    #[test]
    fn check_transaction_rejects_bad_coinbase_script_length() {
        let transaction = open_bitcoin_primitives::Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint::null(),
                script_sig: simple_script(&[0x01]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: simple_script(&[0x51]),
            }],
            lock_time: 0,
        };

        let error = check_transaction(&transaction).expect_err("short coinbase must fail");

        assert_eq!(error.reject_reason, "bad-cb-length");
    }

    #[test]
    fn validate_transaction_reports_missing_inputs() {
        let error = validate_transaction(&sample_spend_transaction(), &[])
            .expect_err("missing spent outputs must fail");

        assert_eq!(error.result, TxValidationResult::MissingInputs);
        assert_eq!(error.reject_reason, "bad-txns-inputs-missingorspent");
    }

    #[test]
    fn validate_transaction_rejects_script_failure() {
        let transaction = sample_spend_transaction();
        let spent_outputs = [SpentOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: simple_script(&[0x53, 0x87]),
            is_coinbase: false,
        }];

        let error = validate_transaction(&transaction, &spent_outputs)
            .expect_err("script mismatch must fail");

        assert_eq!(error.reject_reason, "mandatory-script-verify-flag-failed");
    }

    #[test]
    fn validate_transaction_rejects_output_over_input() {
        let transaction = sample_spend_transaction();
        let spent_outputs = [SpentOutput {
            value: Amount::from_sats(30).expect("valid amount"),
            script_pubkey: simple_script(&[0x52, 0x87]),
            is_coinbase: false,
        }];

        let error =
            validate_transaction(&transaction, &spent_outputs).expect_err("overspend must fail");

        assert_eq!(error.reject_reason, "bad-txns-in-belowout");
    }

    #[test]
    fn validate_transaction_accepts_matching_prevout_script() {
        let transaction = sample_spend_transaction();
        let spent_outputs = [SpentOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: simple_script(&[0x52, 0x87]),
            is_coinbase: false,
        }];

        assert_eq!(validate_transaction(&transaction, &spent_outputs), Ok(()));
    }

    #[test]
    fn check_transaction_rejects_empty_inputs_and_outputs() {
        let empty_inputs = open_bitcoin_primitives::Transaction {
            version: 1,
            inputs: vec![],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: simple_script(&[0x51]),
            }],
            lock_time: 0,
        };
        let empty_outputs = open_bitcoin_primitives::Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint::null(),
                script_sig: simple_script(&[0x01, 0x01]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![],
            lock_time: 0,
        };

        assert_eq!(
            check_transaction(&empty_inputs)
                .expect_err("empty inputs must fail")
                .reject_reason,
            "bad-txns-vin-empty",
        );
        assert_eq!(
            check_transaction(&empty_outputs)
                .expect_err("empty outputs must fail")
                .reject_reason,
            "bad-txns-vout-empty",
        );
    }

    #[test]
    fn check_transaction_rejects_oversized_and_overflowing_outputs() {
        let big_script = simple_script(&vec![0x51; 10_000]);
        let oversized = open_bitcoin_primitives::Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint::null(),
                script_sig: simple_script(&[0x01, 0x01]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: (0..101)
                .map(|_| TransactionOutput {
                    value: Amount::from_sats(1).expect("valid amount"),
                    script_pubkey: big_script.clone(),
                })
                .collect(),
            lock_time: 0,
        };
        let overflow_outputs = open_bitcoin_primitives::Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint::null(),
                script_sig: simple_script(&[0x01, 0x01]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![
                TransactionOutput {
                    value: Amount::from_sats(MAX_MONEY).expect("max money"),
                    script_pubkey: simple_script(&[0x51]),
                },
                TransactionOutput {
                    value: Amount::from_sats(MAX_MONEY).expect("max money"),
                    script_pubkey: simple_script(&[0x51]),
                },
            ],
            lock_time: 0,
        };

        assert_eq!(
            check_transaction(&oversized)
                .expect_err("oversized transaction must fail")
                .reject_reason,
            "bad-txns-oversize",
        );
        assert_eq!(
            check_transaction(&overflow_outputs)
                .expect_err("overflowing outputs must fail")
                .reject_reason,
            "bad-txns-txouttotal-toolarge",
        );
    }

    #[test]
    fn check_transaction_rejects_non_coinbase_null_prevout() {
        let mut transaction = sample_spend_transaction();
        transaction.inputs[0].previous_output = open_bitcoin_primitives::OutPoint::null();
        transaction.inputs.push(TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([8_u8; 32]),
                vout: 0,
            },
            script_sig: simple_script(&[0x52]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        });

        assert_eq!(
            check_transaction(&transaction)
                .expect_err("null prevout must fail")
                .reject_reason,
            "bad-txns-prevout-null",
        );
    }

    #[test]
    fn validate_transaction_accepts_coinbase_without_spent_outputs() {
        let transaction = open_bitcoin_primitives::Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint::null(),
                script_sig: simple_script(&[0x01, 0x01]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: simple_script(&[0x51]),
            }],
            lock_time: 0,
        };

        assert_eq!(validate_transaction(&transaction, &[]), Ok(()));
    }

    #[test]
    fn validate_transaction_rejects_input_value_overflow() {
        let mut transaction = sample_spend_transaction();
        transaction.inputs.push(TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([4_u8; 32]),
                vout: 0,
            },
            script_sig: simple_script(&[0x52]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        });
        transaction.outputs[0].value = Amount::from_sats(1).expect("valid amount");

        let spent_outputs = [
            SpentOutput {
                value: Amount::from_sats(MAX_MONEY).expect("max money"),
                script_pubkey: simple_script(&[0x52, 0x87]),
                is_coinbase: false,
            },
            SpentOutput {
                value: Amount::from_sats(MAX_MONEY).expect("max money"),
                script_pubkey: simple_script(&[0x52, 0x87]),
                is_coinbase: false,
            },
        ];

        assert_eq!(
            validate_transaction(&transaction, &spent_outputs)
                .expect_err("input total overflow must fail")
                .reject_reason,
            "bad-txns-inputvalues-outofrange",
        );
    }

    #[test]
    fn map_codec_error_produces_consensus_reject_reason() {
        let error = map_codec_error(open_bitcoin_codec::CodecError::UnexpectedEof {
            needed: 1,
            remaining: 0,
        });

        assert_eq!(error.reject_reason, "bad-txns-serialization");
    }

    #[test]
    fn validate_transaction_with_context_covers_locktime_sequence_and_script_paths() {
        let transaction = sample_spend_transaction();
        let good_context = TransactionValidationContext {
            inputs: vec![TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(50).expect("valid amount"),
                    script_pubkey: simple_script(&[0x52, 0x87]),
                    is_coinbase: false,
                },
                created_height: 1,
                created_median_time_past: 0,
            }],
            spend_height: 2,
            block_time: 10,
            median_time_past: 10,
            verify_flags: ScriptVerifyFlags::NONE,
            consensus_params: ConsensusParams {
                enforce_bip113_median_time_past: false,
                ..Default::default()
            },
        };

        let fee = validate_transaction_with_context(&transaction, &good_context).expect("valid tx");
        assert_eq!(fee.to_sats(), 10);

        let nonfinal_context = TransactionValidationContext {
            spend_height: 0,
            block_time: 0,
            ..good_context.clone()
        };
        let nonfinal_transaction = open_bitcoin_primitives::Transaction {
            lock_time: 1,
            inputs: vec![TransactionInput {
                sequence: 0,
                ..transaction.inputs[0].clone()
            }],
            ..transaction.clone()
        };
        assert_eq!(
            validate_transaction_with_context(&nonfinal_transaction, &nonfinal_context)
                .expect_err("non-final tx must fail")
                .reject_reason,
            "bad-txns-nonfinal",
        );

        let sequence_context = TransactionValidationContext {
            verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            spend_height: 1,
            median_time_past: 1,
            ..good_context.clone()
        };
        let sequence_transaction = open_bitcoin_primitives::Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                sequence: 2,
                ..transaction.inputs[0].clone()
            }],
            ..transaction.clone()
        };
        assert_eq!(
            validate_transaction_with_context(&sequence_transaction, &sequence_context)
                .expect_err("sequence locked tx must fail")
                .reject_reason,
            "non-BIP68-final",
        );

        let witness_context = TransactionValidationContext {
            verify_flags: ScriptVerifyFlags::WITNESS,
            ..good_context
        };
        let witness_transaction = open_bitcoin_primitives::Transaction {
            inputs: vec![TransactionInput {
                witness: ScriptWitness::new(vec![vec![0x01]]),
                ..transaction.inputs[0].clone()
            }],
            ..transaction
        };
        assert_eq!(
            validate_transaction_with_context(&witness_transaction, &witness_context)
                .expect_err("legacy verifier should reject witness")
                .reject_reason,
            "mandatory-script-verify-flag-failed",
        );
    }
}
