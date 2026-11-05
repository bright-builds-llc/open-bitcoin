// Parity breadcrumbs:
// - packages/bitcoin-knots/src/consensus/tx_check.cpp
// - packages/bitcoin-knots/src/primitives/transaction.h

use crate::validation::{TxValidationError, TxValidationResult, tx_error};
use crate::{MAX_BLOCK_WEIGHT, WITNESS_SCALE_FACTOR};
use open_bitcoin_codec::transaction::{
    TransactionEncoding, encode_transaction_view, wire::TransactionView,
};
use open_bitcoin_primitives::{MAX_MONEY, OutPoint};
use std::collections::HashSet;

/// Pure ordered intrinsic checks over exact serialization facts, without context authority.
pub fn check_transaction_view(transaction: &impl TransactionView) -> Result<(), TxValidationError> {
    if transaction.inputs().is_empty() {
        return Err(error("bad-txns-vin-empty"));
    }
    if transaction.outputs().len() == 0 {
        return Err(error("bad-txns-vout-empty"));
    }
    let stripped_size = encode_transaction_view(transaction, TransactionEncoding::WithoutWitness)
        .map_err(super::map_codec_error)?
        .len();
    if stripped_size.saturating_mul(WITNESS_SCALE_FACTOR) > MAX_BLOCK_WEIGHT {
        return Err(error("bad-txns-oversize"));
    }
    let mut total = 0_i64;
    for output in transaction.outputs() {
        if output.value_sats < 0 {
            return Err(error("bad-txns-vout-negative"));
        }
        if output.value_sats > MAX_MONEY {
            return Err(error("bad-txns-vout-toolarge"));
        }
        // Both terms are individually in MoneyRange before adding.
        total = total
            .checked_add(output.value_sats)
            .ok_or_else(|| error("bad-txns-txouttotal-toolarge"))?;
        if !(0..=MAX_MONEY).contains(&total) {
            return Err(error("bad-txns-txouttotal-toolarge"));
        }
    }
    let mut seen = HashSet::<OutPoint>::new();
    for input in transaction.inputs() {
        if !seen.insert(input.previous_output.clone()) {
            return Err(error("bad-txns-inputs-duplicate"));
        }
    }
    if is_coinbase(transaction) {
        let length = transaction.inputs()[0].script_sig.as_bytes().len();
        if !(2..=100).contains(&length) {
            return Err(error("bad-cb-length"));
        }
        return Ok(());
    }
    if transaction
        .inputs()
        .iter()
        .any(|input| input.previous_output.is_null())
    {
        return Err(error("bad-txns-prevout-null"));
    }
    Ok(())
}

pub(crate) fn is_coinbase(transaction: &impl TransactionView) -> bool {
    transaction.inputs().len() == 1 && transaction.inputs()[0].previous_output.is_null()
}
fn error(reason: &'static str) -> TxValidationError {
    tx_error(TxValidationResult::Consensus, reason, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_bitcoin_codec::transaction::wire::{WireOutput, WireTransaction};
    use open_bitcoin_primitives::{ScriptBuf, ScriptWitness, TransactionInput, Txid};

    fn raw(values: &[i64]) -> WireTransaction {
        WireTransaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([1; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: 0,
                witness: ScriptWitness::default(),
            }],
            outputs: values
                .iter()
                .map(|value| WireOutput {
                    value_sats: *value,
                    script_pubkey: ScriptBuf::default(),
                })
                .collect(),
            lock_time: 0,
        }
    }
    fn reason(transaction: &WireTransaction) -> &'static str {
        check_transaction_view(transaction)
            .expect_err("invalid intrinsic")
            .reject_reason
    }

    #[test]
    fn phase167_submission_wire_transaction_individual_output_order() {
        // Arrange
        for (values, expected) in [
            (vec![i64::MIN], "bad-txns-vout-negative"),
            (vec![-1], "bad-txns-vout-negative"),
            (vec![MAX_MONEY + 1], "bad-txns-vout-toolarge"),
            (vec![i64::MAX], "bad-txns-vout-toolarge"),
            (vec![MAX_MONEY, i64::MAX], "bad-txns-vout-toolarge"),
            (vec![MAX_MONEY, 1, -1], "bad-txns-txouttotal-toolarge"),
        ] {
            let transaction = raw(&values);
            // Act / Assert
            assert_eq!(reason(&transaction), expected, "{values:?}");
        }
    }
    #[test]
    fn phase167_submission_wire_transaction_earlier_structure_and_size() {
        // Arrange
        let mut transaction = raw(&[-1]);
        transaction.inputs.clear();
        // Act / Assert
        assert_eq!(reason(&transaction), "bad-txns-vin-empty");
        transaction = raw(&[-1]);
        transaction.outputs.clear();
        assert_eq!(reason(&transaction), "bad-txns-vout-empty");
        transaction = raw(&[-1]);
        transaction.outputs[0].script_pubkey = ScriptBuf::from_serialized_bytes(vec![0; 1_000_000]);
        assert_eq!(reason(&transaction), "bad-txns-oversize");
    }
    #[test]
    fn phase167_submission_wire_transaction_duplicate_coinbase_and_null_order() {
        // Arrange
        let mut transaction = raw(&[-1]);
        transaction.inputs.push(transaction.inputs[0].clone());
        // Act / Assert
        assert_eq!(reason(&transaction), "bad-txns-vout-negative");
        transaction.outputs[0].value_sats = 0;
        assert_eq!(reason(&transaction), "bad-txns-inputs-duplicate");
        transaction = raw(&[-1]);
        transaction.inputs[0].previous_output = OutPoint::null();
        assert_eq!(reason(&transaction), "bad-txns-vout-negative");
        transaction.outputs[0].value_sats = 0;
        assert_eq!(reason(&transaction), "bad-cb-length");
        transaction.inputs.push(raw(&[0]).inputs.remove(0));
        assert_eq!(reason(&transaction), "bad-txns-prevout-null");
    }
    #[test]
    fn phase167_submission_wire_transaction_typed_and_raw_share_results() {
        // Arrange
        for values in [vec![0], vec![MAX_MONEY], vec![MAX_MONEY, 1]] {
            let wire = raw(&values);
            let typed = wire
                .clone()
                .try_into_transaction()
                .expect("individual valid amounts");
            // Act / Assert
            assert_eq!(
                check_transaction_view(&wire),
                crate::transaction::check_transaction(&typed)
            );
        }
    }
}
