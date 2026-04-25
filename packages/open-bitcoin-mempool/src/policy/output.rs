// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp

use open_bitcoin_consensus::{ScriptPubKeyType, classify_script_pubkey};
use open_bitcoin_primitives::TransactionOutput;

use crate::{MempoolError, PolicyConfig};

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

pub(super) fn validate_standard_output(
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
