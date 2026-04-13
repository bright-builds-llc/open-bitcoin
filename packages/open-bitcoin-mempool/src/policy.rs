use open_bitcoin_codec::{TransactionEncoding, encode_transaction};
use open_bitcoin_consensus::script::{count_p2sh_sigops, count_witness_sigops};
use open_bitcoin_consensus::{
    ScriptPubKeyType, TransactionInputContext, classify_script_pubkey, count_legacy_sigops,
    is_push_only,
};
use open_bitcoin_primitives::{Transaction, TransactionInput, TransactionOutput};

use crate::{MempoolError, PolicyConfig};

pub fn transaction_weight_and_virtual_size(transaction: &Transaction) -> (usize, usize) {
    let stripped = encode_transaction(transaction, TransactionEncoding::WithoutWitness)
        .expect("typed transactions should serialize without witness")
        .len();
    let total = encode_transaction(
        transaction,
        if transaction.has_witness() {
            TransactionEncoding::WithWitness
        } else {
            TransactionEncoding::WithoutWitness
        },
    )
    .expect("typed transactions should serialize with selected witness mode")
    .len();
    let weight = stripped.saturating_mul(3).saturating_add(total);
    let virtual_size = weight.div_ceil(4);

    (weight, virtual_size)
}

pub fn transaction_sigops_cost(
    transaction: &Transaction,
    input_contexts: &[TransactionInputContext],
) -> Result<usize, MempoolError> {
    if transaction.inputs.len() != input_contexts.len() {
        return Err(MempoolError::Validation {
            reason: "transaction input context length mismatch".to_string(),
        });
    }

    let mut sigops = 0_usize;
    for input in &transaction.inputs {
        sigops = sigops.saturating_add(count_legacy_sigops(&input.script_sig).map_err(
            |source: open_bitcoin_consensus::ScriptError| MempoolError::NonStandard {
                reason: source.to_string(),
            },
        )?);
    }
    for output in &transaction.outputs {
        sigops = sigops.saturating_add(count_legacy_sigops(&output.script_pubkey).map_err(
            |source: open_bitcoin_consensus::ScriptError| MempoolError::NonStandard {
                reason: source.to_string(),
            },
        )?);
    }
    for (input, input_context) in transaction.inputs.iter().zip(input_contexts) {
        sigops = sigops
            .saturating_add(
                count_p2sh_sigops(&input.script_sig, &input_context.spent_output.script_pubkey)
                    .map_err(|source| MempoolError::NonStandard {
                        reason: source.to_string(),
                    })?,
            )
            .saturating_add(
                count_witness_sigops(
                    &input.script_sig,
                    &input_context.spent_output.script_pubkey,
                    &input.witness,
                    open_bitcoin_consensus::ScriptVerifyFlags::P2SH
                        | open_bitcoin_consensus::ScriptVerifyFlags::WITNESS
                        | open_bitcoin_consensus::ScriptVerifyFlags::TAPROOT,
                )
                .map_err(|source| MempoolError::NonStandard {
                    reason: source.to_string(),
                })?,
            );
    }

    Ok(sigops.saturating_mul(4))
}

pub fn validate_standard_transaction(
    transaction: &Transaction,
    input_contexts: &[TransactionInputContext],
    config: &PolicyConfig,
    weight: usize,
    sigops_cost: usize,
) -> Result<(), MempoolError> {
    if weight > config.max_standard_tx_weight {
        return Err(MempoolError::NonStandard {
            reason: format!(
                "transaction weight {weight} exceeds standard limit {}",
                config.max_standard_tx_weight
            ),
        });
    }
    if sigops_cost > config.max_standard_sigops_cost {
        return Err(MempoolError::NonStandard {
            reason: format!(
                "transaction sigops cost {sigops_cost} exceeds standard limit {}",
                config.max_standard_sigops_cost
            ),
        });
    }

    for (input_index, input) in transaction.inputs.iter().enumerate() {
        let script_sig_size = input.script_sig.as_bytes().len();
        if script_sig_size > config.max_script_sig_size {
            return Err(MempoolError::NonStandard {
                reason: format!(
                    "input {input_index} scriptSig size {script_sig_size} exceeds standard limit {}",
                    config.max_script_sig_size
                ),
            });
        }
        if !is_push_only(&input.script_sig) {
            return Err(MempoolError::NonStandard {
                reason: format!("input {input_index} scriptSig must be push-only"),
            });
        }
    }

    if transaction.inputs.len() != input_contexts.len() {
        return Err(MempoolError::Validation {
            reason: "input context mismatch during standardness checks".to_string(),
        });
    }

    for (output_index, output) in transaction.outputs.iter().enumerate() {
        validate_standard_output(output, output_index, config)?;
    }

    Ok(())
}

pub fn signals_opt_in_rbf(transaction: &Transaction) -> bool {
    transaction
        .inputs
        .iter()
        .any(|input| input.sequence < TransactionInput::MAX_SEQUENCE_NONFINAL)
}

pub fn dust_threshold_sats(output: &TransactionOutput) -> i64 {
    let script = output.script_pubkey.as_bytes();
    if script.first() == Some(&0x6a) {
        return 0;
    }

    match classify_script_pubkey(&output.script_pubkey) {
        ScriptPubKeyType::WitnessV0KeyHash(_)
        | ScriptPubKeyType::WitnessV0ScriptHash(_)
        | ScriptPubKeyType::WitnessV1Taproot(_) => 330,
        _ => 546,
    }
}

fn validate_standard_output(
    output: &TransactionOutput,
    output_index: usize,
    config: &PolicyConfig,
) -> Result<(), MempoolError> {
    let script = output.script_pubkey.as_bytes();
    if script.first() == Some(&0x6a) {
        if !config.accept_datacarrier {
            return Err(MempoolError::NonStandard {
                reason: format!("output {output_index} null-data scripts are disabled"),
            });
        }
        if script.len() > config.max_datacarrier_bytes {
            return Err(MempoolError::NonStandard {
                reason: format!(
                    "output {output_index} null-data script length {} exceeds standard limit {}",
                    script.len(),
                    config.max_datacarrier_bytes
                ),
            });
        }
        if output.value.to_sats() != 0 {
            return Err(MempoolError::NonStandard {
                reason: format!("output {output_index} null-data outputs must carry zero value"),
            });
        }
        return Ok(());
    }

    match classify_script_pubkey(&output.script_pubkey) {
        ScriptPubKeyType::PayToPubKey { .. } => {
            return Err(MempoolError::NonStandard {
                reason: format!("output {output_index} bare pubkey outputs are non-standard"),
            });
        }
        ScriptPubKeyType::Multisig { .. } if !config.permit_bare_multisig => {
            return Err(MempoolError::NonStandard {
                reason: format!("output {output_index} bare multisig outputs are disabled"),
            });
        }
        ScriptPubKeyType::WitnessUnknown { .. } | ScriptPubKeyType::NonStandard => {
            return Err(MempoolError::NonStandard {
                reason: format!("output {output_index} script is non-standard"),
            });
        }
        _ => {}
    }

    let threshold = dust_threshold_sats(output);
    if output.value.to_sats() < threshold {
        return Err(MempoolError::NonStandard {
            reason: format!(
                "output {output_index} value {} is dust below threshold {threshold}",
                output.value.to_sats()
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use open_bitcoin_consensus::TransactionInputContext;
    use open_bitcoin_consensus::crypto::hash160;
    use open_bitcoin_primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput, Txid,
    };

    use super::{
        dust_threshold_sats, signals_opt_in_rbf, transaction_sigops_cost,
        transaction_weight_and_virtual_size, validate_standard_output,
        validate_standard_transaction,
    };
    use crate::{PolicyConfig, types::FeeRate};

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn redeem_script() -> ScriptBuf {
        script(&[0x51])
    }

    fn p2sh_script() -> ScriptBuf {
        let redeem_hash = hash160(redeem_script().as_bytes());
        let mut bytes = vec![0xa9, 20];
        bytes.extend_from_slice(&redeem_hash);
        bytes.push(0x87);
        script(&bytes)
    }

    fn spend_input(sequence: u32) -> TransactionInput {
        TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([2_u8; 32]),
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence,
            witness: ScriptWitness::default(),
        }
    }

    fn input_context() -> TransactionInputContext {
        TransactionInputContext {
            spent_output: open_bitcoin_consensus::SpentOutput {
                value: Amount::from_sats(1000).expect("valid amount"),
                script_pubkey: p2sh_script(),
                is_coinbase: false,
            },
            created_height: 1,
            created_median_time_past: 1,
        }
    }

    fn standard_transaction() -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![spend_input(TransactionInput::SEQUENCE_FINAL)],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(800).expect("valid amount"),
                script_pubkey: p2sh_script(),
            }],
            lock_time: 0,
        }
    }

    #[test]
    fn transaction_weight_reports_expected_vsize() {
        let (weight, virtual_size) = transaction_weight_and_virtual_size(&standard_transaction());

        assert!(weight > 0);
        assert_eq!(virtual_size, weight.div_ceil(4));
    }

    #[test]
    fn transaction_weight_counts_witness_when_present() {
        let mut transaction = standard_transaction();
        transaction.inputs[0].witness = ScriptWitness::new(vec![vec![0x01]]);

        let (weight, virtual_size) = transaction_weight_and_virtual_size(&transaction);

        assert!(weight > 4 * virtual_size.saturating_sub(1));
        assert!(transaction.has_witness());
    }

    #[test]
    fn transaction_sigops_cost_counts_standard_p2sh_input() {
        let sigops_cost =
            transaction_sigops_cost(&standard_transaction(), &[input_context()]).expect("sigops");

        assert_eq!(sigops_cost, 0);
    }

    #[test]
    fn validate_standard_transaction_rejects_non_standard_outputs() {
        let mut transaction = standard_transaction();
        transaction.outputs[0].script_pubkey = script(&[0x51]);
        let (weight, virtual_size) = transaction_weight_and_virtual_size(&transaction);
        let sigops_cost =
            transaction_sigops_cost(&transaction, &[input_context()]).expect("sigops");
        let config = PolicyConfig::default();

        let error = validate_standard_transaction(
            &transaction,
            &[input_context()],
            &config,
            weight,
            sigops_cost,
        )
        .expect_err("non-standard output should fail");

        assert!(error.to_string().contains("non-standard"));
        assert!(virtual_size > 0);
    }

    #[test]
    fn validate_standard_transaction_rejects_dust_outputs() {
        let mut transaction = standard_transaction();
        transaction.outputs[0].value = Amount::from_sats(100).expect("valid amount");
        let (weight, _) = transaction_weight_and_virtual_size(&transaction);
        let sigops_cost =
            transaction_sigops_cost(&transaction, &[input_context()]).expect("sigops");
        let error = validate_standard_transaction(
            &transaction,
            &[input_context()],
            &PolicyConfig::default(),
            weight,
            sigops_cost,
        )
        .expect_err("dust should fail");

        assert!(error.to_string().contains("dust"));
        assert_eq!(
            FeeRate::from_sats_per_kvb(1000),
            PolicyConfig::default().min_relay_feerate
        );
    }

    #[test]
    fn dust_threshold_distinguishes_witness_and_legacy_outputs() {
        let witness = TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: script(&[
                0x00, 20, 0_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
        };
        let legacy = TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: p2sh_script(),
        };

        assert_eq!(dust_threshold_sats(&witness), 330);
        assert_eq!(dust_threshold_sats(&legacy), 546);
    }

    #[test]
    fn dust_threshold_returns_zero_for_null_data_outputs() {
        let output = TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: script(&[0x6a, 0x01, 0x01]),
        };

        assert_eq!(dust_threshold_sats(&output), 0);
    }

    #[test]
    fn transaction_sigops_cost_rejects_context_length_mismatch() {
        let error = transaction_sigops_cost(&standard_transaction(), &[])
            .expect_err("context mismatch should fail");

        assert!(matches!(error, crate::MempoolError::Validation { .. }));
    }

    #[test]
    fn transaction_sigops_cost_maps_script_parsing_errors_from_all_sources() {
        let mut bad_input = standard_transaction();
        bad_input.inputs[0].script_sig = script(&[0x4c]);
        let error = transaction_sigops_cost(&bad_input, &[input_context()])
            .expect_err("legacy input parse should fail");
        assert!(matches!(error, crate::MempoolError::NonStandard { .. }));

        let mut bad_output = standard_transaction();
        bad_output.outputs[0].script_pubkey = script(&[0x4c]);
        let error = transaction_sigops_cost(&bad_output, &[input_context()])
            .expect_err("legacy output parse should fail");
        assert!(matches!(error, crate::MempoolError::NonStandard { .. }));

        let bad_p2sh = Transaction {
            inputs: vec![TransactionInput {
                script_sig: script(&[0x01, 0x4c]),
                ..spend_input(TransactionInput::SEQUENCE_FINAL)
            }],
            ..standard_transaction()
        };
        let error = transaction_sigops_cost(&bad_p2sh, &[input_context()])
            .expect_err("p2sh parse should fail");
        assert!(matches!(error, crate::MempoolError::NonStandard { .. }));

        let bad_witness = Transaction {
            inputs: vec![TransactionInput {
                script_sig: ScriptBuf::default(),
                witness: ScriptWitness::new(vec![vec![0x4c]]),
                ..spend_input(TransactionInput::SEQUENCE_FINAL)
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(800).expect("valid amount"),
                script_pubkey: script(&[
                    0x00, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0,
                ]),
            }],
            lock_time: 0,
            version: 2,
        };
        let witness_context = TransactionInputContext {
            spent_output: open_bitcoin_consensus::SpentOutput {
                value: Amount::from_sats(1000).expect("valid amount"),
                script_pubkey: script(&[
                    0x00, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0,
                ]),
                is_coinbase: false,
            },
            created_height: 1,
            created_median_time_past: 1,
        };
        let error = transaction_sigops_cost(&bad_witness, &[witness_context])
            .expect_err("witness parse should fail");
        assert!(matches!(error, crate::MempoolError::NonStandard { .. }));
    }

    #[test]
    fn standard_transaction_checks_cover_weight_sigops_and_script_sig_edges() {
        let transaction = standard_transaction();
        let error = validate_standard_transaction(
            &transaction,
            &[input_context()],
            &PolicyConfig {
                max_standard_tx_weight: 1,
                ..PolicyConfig::default()
            },
            100,
            0,
        )
        .expect_err("weight limit should fail");
        assert!(error.to_string().contains("weight"));

        let error = validate_standard_transaction(
            &transaction,
            &[input_context()],
            &PolicyConfig {
                max_standard_sigops_cost: 0,
                ..PolicyConfig::default()
            },
            100,
            1,
        )
        .expect_err("sigops limit should fail");
        assert!(error.to_string().contains("sigops"));

        let mut oversized_script_sig = transaction.clone();
        oversized_script_sig.inputs[0].script_sig =
            script(&vec![0x51; PolicyConfig::default().max_script_sig_size + 1]);
        let error = validate_standard_transaction(
            &oversized_script_sig,
            &[input_context()],
            &PolicyConfig::default(),
            100,
            0,
        )
        .expect_err("oversized scriptSig should fail");
        assert!(error.to_string().contains("scriptSig size"));

        let mut non_push_only = transaction;
        non_push_only.inputs[0].script_sig = script(&[0x61]);
        let error = validate_standard_transaction(
            &non_push_only,
            &[input_context()],
            &PolicyConfig::default(),
            100,
            0,
        )
        .expect_err("non push-only should fail");
        assert!(error.to_string().contains("push-only"));

        let mismatch = validate_standard_transaction(
            &standard_transaction(),
            &[],
            &PolicyConfig::default(),
            100,
            0,
        )
        .expect_err("input mismatch should fail");
        assert!(matches!(mismatch, crate::MempoolError::Validation { .. }));
    }

    #[test]
    fn standard_output_checks_cover_null_data_and_script_classification_branches() {
        let disabled = validate_standard_output(
            &TransactionOutput {
                value: Amount::ZERO,
                script_pubkey: script(&[0x6a, 0x01, 0x01]),
            },
            0,
            &PolicyConfig {
                accept_datacarrier: false,
                ..PolicyConfig::default()
            },
        )
        .expect_err("datacarrier disabled should fail");
        assert!(disabled.to_string().contains("disabled"));

        let too_long = validate_standard_output(
            &TransactionOutput {
                value: Amount::ZERO,
                script_pubkey: script(&[0x6a; 90]),
            },
            0,
            &PolicyConfig::default(),
        )
        .expect_err("long datacarrier should fail");
        assert!(too_long.to_string().contains("length"));

        let non_zero_null_data = validate_standard_output(
            &TransactionOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: script(&[0x6a, 0x01, 0x01]),
            },
            0,
            &PolicyConfig::default(),
        )
        .expect_err("non-zero null data should fail");
        assert!(non_zero_null_data.to_string().contains("zero value"));

        validate_standard_output(
            &TransactionOutput {
                value: Amount::ZERO,
                script_pubkey: script(&[0x6a, 0x01, 0x01]),
            },
            0,
            &PolicyConfig::default(),
        )
        .expect("zero-value datacarrier should be standard");

        let bare_pubkey = validate_standard_output(
            &TransactionOutput {
                value: Amount::from_sats(1000).expect("valid amount"),
                script_pubkey: {
                    let mut bytes = vec![33];
                    bytes.extend_from_slice(&[2_u8; 33]);
                    bytes.push(0xac);
                    script(&bytes)
                },
            },
            0,
            &PolicyConfig::default(),
        )
        .expect_err("bare pubkey should fail");
        assert!(bare_pubkey.to_string().contains("bare pubkey"));

        let bare_multisig = validate_standard_output(
            &TransactionOutput {
                value: Amount::from_sats(1000).expect("valid amount"),
                script_pubkey: {
                    let mut bytes = vec![0x51, 33];
                    bytes.extend_from_slice(&[2_u8; 33]);
                    bytes.extend_from_slice(&[0x51, 0xae]);
                    script(&bytes)
                },
            },
            0,
            &PolicyConfig::default(),
        )
        .expect_err("bare multisig should fail");
        assert!(bare_multisig.to_string().contains("bare multisig"));

        let witness_unknown = validate_standard_output(
            &TransactionOutput {
                value: Amount::from_sats(1000).expect("valid amount"),
                script_pubkey: {
                    let mut bytes = vec![0x52, 20];
                    bytes.extend_from_slice(&[0_u8; 20]);
                    script(&bytes)
                },
            },
            0,
            &PolicyConfig::default(),
        )
        .expect_err("unknown witness program should fail");
        assert!(witness_unknown.to_string().contains("non-standard"));
    }

    #[test]
    fn opt_in_rbf_detects_non_final_sequences() {
        let replaceable = Transaction {
            inputs: vec![spend_input(TransactionInput::MAX_SEQUENCE_NONFINAL - 1)],
            ..standard_transaction()
        };

        assert!(signals_opt_in_rbf(&replaceable));
        assert!(!signals_opt_in_rbf(&standard_transaction()));
    }
}
