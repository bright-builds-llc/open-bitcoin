// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/policy.cpp
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/test/txvalidation_tests.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/test/functional/mempool_ephemeral_dust.py

use open_bitcoin_consensus::{ScriptPubKeyType, classify_script_pubkey, is_push_only};
use open_bitcoin_primitives::{ScriptBuf, TransactionOutput};

use crate::{DustRelayFeeRate, EphemeralPolicy, MempoolError, PolicyConfig};

const SERIALIZED_AMOUNT_SIZE: usize = 8;
const WITNESS_SPEND_VIRTUAL_SIZE: usize = 67;
const LEGACY_SPEND_VIRTUAL_SIZE: usize = 148;
const OP_RETURN: u8 = 0x6a;
const MAX_NULL_DATA_OUTPUTS_PER_TRANSACTION: usize = 1;
const MAX_DUST_OUTPUTS_PER_TRANSACTION: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StandardOutputFacts {
    is_null_data: bool,
    is_dust: bool,
    is_monetary: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct StandardTransactionOutputFacts {
    null_data_outputs: usize,
    dust_outputs: usize,
    monetary_outputs: usize,
}

impl StandardTransactionOutputFacts {
    pub(super) fn record(&mut self, output: StandardOutputFacts) {
        self.null_data_outputs += usize::from(output.is_null_data);
        self.dust_outputs += usize::from(output.is_dust);
        self.monetary_outputs += usize::from(output.is_monetary);
    }

    pub(super) fn enforce(self, config: &PolicyConfig) -> Result<(), MempoolError> {
        if self.dust_outputs > MAX_DUST_OUTPUTS_PER_TRANSACTION {
            return Err(MempoolError::NonStandard {
                reason: format!(
                    "transaction dust output count {} exceeds standard limit {}",
                    self.dust_outputs, MAX_DUST_OUTPUTS_PER_TRANSACTION
                ),
            });
        }
        if self.null_data_outputs > MAX_NULL_DATA_OUTPUTS_PER_TRANSACTION {
            return Err(MempoolError::NonStandard {
                reason: format!(
                    "transaction null-data output count {} exceeds standard limit {}",
                    self.null_data_outputs, MAX_NULL_DATA_OUTPUTS_PER_TRANSACTION
                ),
            });
        }
        if self.monetary_outputs == 0
            && self.null_data_outputs > 0
            && !config.permit_bare_datacarrier
        {
            return Err(MempoolError::NonStandard {
                reason: "bare data-carrier transactions are disabled".to_string(),
            });
        }
        if self.monetary_outputs == 0 && self.null_data_outputs == 0 && !config.permit_bare_anchor {
            return Err(MempoolError::NonStandard {
                reason: "bare-anchor transactions are disabled".to_string(),
            });
        }
        Ok(())
    }
}

pub fn dust_threshold_sats(output: &TransactionOutput) -> i64 {
    dust_threshold_sats_at_rate(output, DustRelayFeeRate::default())
}

pub fn dust_threshold_sats_at_rate(
    output: &TransactionOutput,
    dust_relay_fee_rate: DustRelayFeeRate,
) -> i64 {
    let script = output.script_pubkey.as_bytes();
    if script.first() == Some(&0x6a) {
        return 0;
    }

    let spend_virtual_size = match classify_script_pubkey(&output.script_pubkey) {
        ScriptPubKeyType::WitnessV0KeyHash(_)
        | ScriptPubKeyType::WitnessV0ScriptHash(_)
        | ScriptPubKeyType::WitnessV1Taproot(_)
        | ScriptPubKeyType::PayToAnchor => WITNESS_SPEND_VIRTUAL_SIZE,
        _ => LEGACY_SPEND_VIRTUAL_SIZE,
    };
    let output_size = serialized_output_size(output);
    dust_relay_fee_rate
        .fee_rate()
        .fee_for_virtual_size(crate::TransactionVirtualSize::new(
            output_size + spend_virtual_size,
        ))
}

fn serialized_output_size(output: &TransactionOutput) -> usize {
    let script_size = output.script_pubkey.as_bytes().len();
    SERIALIZED_AMOUNT_SIZE + compact_size_len(script_size) + script_size
}

fn compact_size_len(value: usize) -> usize {
    if value <= 252 { 1 } else { 3 }
}

fn is_null_data_script(script: &ScriptBuf) -> bool {
    let Some((opcode, suffix)) = script.as_bytes().split_first() else {
        return false;
    };
    if *opcode != OP_RETURN {
        return false;
    }
    ScriptBuf::from_bytes(suffix.to_vec()).is_ok_and(|suffix| is_push_only(&suffix))
}

pub(crate) fn is_dust_output(
    output: &TransactionOutput,
    dust_relay_fee_rate: DustRelayFeeRate,
) -> bool {
    output.value.to_sats() < dust_threshold_sats_at_rate(output, dust_relay_fee_rate)
}

pub(crate) fn is_permitted_ephemeral_dust(
    output: &TransactionOutput,
    permissions: EphemeralPolicy,
    dust_relay_fee_rate: DustRelayFeeRate,
) -> bool {
    if !is_dust_output(output, dust_relay_fee_rate) {
        return false;
    }
    let is_anchor = matches!(
        classify_script_pubkey(&output.script_pubkey),
        ScriptPubKeyType::PayToAnchor
    );
    let form_is_permitted = if is_anchor {
        permissions.anchor
    } else {
        permissions.send
    };
    form_is_permitted && (output.value.to_sats() == 0 || permissions.dust)
}

pub(super) fn validate_standard_output(
    output: &TransactionOutput,
    output_index: usize,
    config: &PolicyConfig,
) -> Result<StandardOutputFacts, MempoolError> {
    let script = output.script_pubkey.as_bytes();
    if is_null_data_script(&output.script_pubkey) {
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
        return Ok(StandardOutputFacts {
            is_null_data: true,
            is_dust: false,
            is_monetary: false,
        });
    }

    let script_type = classify_script_pubkey(&output.script_pubkey);
    match script_type {
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

    if matches!(script_type, ScriptPubKeyType::PayToAnchor) && !config.ephemeral_policy.anchor {
        return Err(MempoolError::NonStandard {
            reason: format!("output {output_index} pay-to-anchor outputs are disabled"),
        });
    }

    let threshold = dust_threshold_sats_at_rate(output, config.dust_relay_fee_rate);
    let is_dust = output.value.to_sats() < threshold;
    if is_dust {
        if !matches!(script_type, ScriptPubKeyType::PayToAnchor) && !config.ephemeral_policy.send {
            return Err(MempoolError::NonStandard {
                reason: format!(
                    "output {output_index} dusty non-anchor outputs require send permission"
                ),
            });
        }
        if output.value.to_sats() != 0 && !config.ephemeral_policy.dust {
            return Err(MempoolError::NonStandard {
                reason: format!("output {output_index} nonzero dust requires dust permission"),
            });
        }
    }

    Ok(StandardOutputFacts {
        is_null_data: false,
        is_dust,
        is_monetary: !is_dust,
    })
}
