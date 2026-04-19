use core::cmp::Ordering;

use open_bitcoin_consensus::{
    ConsensusParams, SpentOutput, TransactionInputContext, TransactionValidationContext,
};
use open_bitcoin_mempool::{FeeRate, dust_threshold_sats, transaction_weight_and_virtual_size};
use open_bitcoin_primitives::{
    Amount, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
};

use super::{BuildRequest, BuiltTransaction, Recipient, Wallet, WalletUtxo};
use crate::WalletError;
use crate::descriptor::{DescriptorRecord, DescriptorRole, SingleKeyDescriptor};

pub(super) fn build_transaction(
    wallet: &Wallet,
    request: &BuildRequest,
    coinbase_maturity: u32,
) -> Result<BuiltTransaction, WalletError> {
    if request.recipients.is_empty() {
        return Err(WalletError::NoRecipients);
    }

    let mut spendable = wallet
        .utxos
        .iter()
        .filter(|utxo| wallet.is_spendable(utxo, coinbase_maturity))
        .cloned()
        .collect::<Vec<_>>();
    if spendable.is_empty() {
        return Err(WalletError::NoSpendableCoins);
    }
    spendable.sort_by(|left, right| compare_effective_value(wallet, request.fee_rate, left, right));

    let recipients_total = request
        .recipients
        .iter()
        .fold(0_i64, |sum, recipient| sum + recipient.value.to_sats());
    let maybe_change_descriptor =
        resolve_change_descriptor(wallet, request.maybe_change_descriptor_id);
    let maybe_change_script = maybe_change_descriptor
        .map(|descriptor| descriptor.descriptor.script_pubkey())
        .transpose()?;
    let maybe_change_script = maybe_change_script.as_ref();

    let mut selected = Vec::new();
    let mut available_sats = 0_i64;
    for utxo in spendable {
        available_sats += utxo.output.value.to_sats();
        selected.push(utxo);

        let no_change_vsize =
            estimate_vsize(wallet, &selected, &request.recipients, None, request)?;
        let no_change_fee = request.fee_rate.fee_for_virtual_size(no_change_vsize);
        if available_sats < recipients_total + no_change_fee {
            continue;
        }

        let maybe_change_output =
            build_change_output(wallet, request, maybe_change_script, &selected)?;

        let mut outputs = request
            .recipients
            .iter()
            .map(|recipient| TransactionOutput {
                value: recipient.value,
                script_pubkey: recipient.script_pubkey.clone(),
            })
            .collect::<Vec<_>>();
        let change_output_index = if let Some(change_output) = maybe_change_output.clone() {
            outputs.push(change_output);
            Some(outputs.len() - 1)
        } else {
            None
        };

        let transaction = Transaction {
            version: 2,
            inputs: selected
                .iter()
                .map(|utxo| placeholder_input(wallet, utxo, request.enable_rbf))
                .collect::<Result<Vec<_>, WalletError>>()?,
            outputs,
            lock_time: request
                .maybe_lock_time
                .unwrap_or_else(|| wallet.maybe_tip_height.unwrap_or(0)),
        };
        let inputs_total = selected
            .iter()
            .fold(0_i64, |sum, utxo| sum + utxo.output.value.to_sats());
        let outputs_total = transaction
            .outputs
            .iter()
            .fold(0_i64, |sum, output| sum + output.value.to_sats());
        let fee_sats = inputs_total - outputs_total;
        if change_output_index.is_none()
            && maybe_change_script.is_none()
            && fee_sats > no_change_fee
        {
            return Err(WalletError::ChangeDescriptorRequired);
        }

        return Ok(BuiltTransaction {
            transaction,
            selected_inputs: selected,
            fee: amount_from_sats(fee_sats)?,
            change_output_index,
        });
    }

    Err(WalletError::InsufficientFunds {
        needed_sats: recipients_total,
        available_sats,
    })
}

pub(super) fn input_contexts_for(
    _wallet: &Wallet,
    built: &BuiltTransaction,
) -> Result<Vec<TransactionInputContext>, WalletError> {
    built
        .selected_inputs
        .iter()
        .map(|utxo| {
            Ok(TransactionInputContext {
                spent_output: SpentOutput {
                    value: utxo.output.value,
                    script_pubkey: utxo.output.script_pubkey.clone(),
                    is_coinbase: utxo.is_coinbase,
                },
                created_height: utxo.created_height,
                created_median_time_past: utxo.created_median_time_past,
            })
        })
        .collect()
}

pub(super) fn validation_context(
    wallet: &Wallet,
    input_contexts: &[TransactionInputContext],
) -> TransactionValidationContext {
    TransactionValidationContext {
        inputs: input_contexts.to_vec(),
        spend_height: wallet.spend_height(),
        block_time: wallet.maybe_tip_median_time_past.unwrap_or(0),
        median_time_past: wallet.maybe_tip_median_time_past.unwrap_or(0),
        verify_flags: super::standard_wallet_verify_flags(),
        consensus_params: ConsensusParams::default(),
    }
}

pub(super) fn resolve_change_descriptor(
    wallet: &Wallet,
    maybe_descriptor_id: Option<u32>,
) -> Option<&DescriptorRecord> {
    maybe_descriptor_id
        .and_then(|descriptor_id| wallet.descriptor(descriptor_id))
        .or_else(|| {
            wallet
                .descriptors
                .iter()
                .find(|descriptor| descriptor.role == DescriptorRole::Internal)
        })
}

pub(super) fn estimate_vsize(
    wallet: &Wallet,
    selected_inputs: &[WalletUtxo],
    recipients: &[Recipient],
    maybe_change_output: Option<&TransactionOutput>,
    request: &BuildRequest,
) -> Result<usize, WalletError> {
    let outputs = recipients
        .iter()
        .map(|recipient| TransactionOutput {
            value: recipient.value,
            script_pubkey: recipient.script_pubkey.clone(),
        })
        .chain(maybe_change_output.into_iter().cloned())
        .collect::<Vec<_>>();
    let transaction = Transaction {
        version: 2,
        inputs: selected_inputs
            .iter()
            .map(|utxo| placeholder_input(wallet, utxo, request.enable_rbf))
            .collect::<Result<Vec<_>, WalletError>>()?,
        outputs,
        lock_time: request
            .maybe_lock_time
            .unwrap_or_else(|| wallet.maybe_tip_height.unwrap_or(0)),
    };

    Ok(transaction_weight_and_virtual_size(&transaction).1)
}

pub(super) fn estimate_change_vsize(
    wallet: &Wallet,
    selected_inputs: &[WalletUtxo],
    recipients: &[Recipient],
    change_output: &TransactionOutput,
    request: &BuildRequest,
) -> Result<usize, WalletError> {
    estimate_vsize(
        wallet,
        selected_inputs,
        recipients,
        Some(change_output),
        request,
    )
}

fn build_change_output(
    wallet: &Wallet,
    request: &BuildRequest,
    maybe_change_script: Option<&ScriptBuf>,
    selected_inputs: &[WalletUtxo],
) -> Result<Option<TransactionOutput>, WalletError> {
    let Some(change_script) = maybe_change_script else {
        return Ok(None);
    };

    let placeholder = TransactionOutput {
        value: amount_from_sats(1)?,
        script_pubkey: change_script.clone(),
    };
    let recipients = request.recipients.as_slice();
    let with_change_vsize =
        estimate_change_vsize(wallet, selected_inputs, recipients, &placeholder, request)?;
    let change_fee = request.fee_rate.fee_for_virtual_size(with_change_vsize);
    let available_sats = selected_inputs
        .iter()
        .fold(0_i64, |sum, utxo| sum + utxo.output.value.to_sats());
    let recipients_total = recipients
        .iter()
        .fold(0_i64, |sum, recipient| sum + recipient.value.to_sats());
    let change_sats = available_sats - recipients_total - change_fee;
    if change_sats <= 0 {
        return Ok(None);
    }

    let candidate_output = TransactionOutput {
        value: amount_from_sats(change_sats)?,
        script_pubkey: change_script.clone(),
    };
    if change_sats > dust_threshold_sats(&candidate_output) {
        return Ok(Some(candidate_output));
    }

    Ok(None)
}

pub(super) fn placeholder_input(
    wallet: &Wallet,
    utxo: &WalletUtxo,
    enable_rbf: bool,
) -> Result<TransactionInput, WalletError> {
    let descriptor = wallet
        .descriptor(utxo.descriptor_id)
        .ok_or(WalletError::UnknownDescriptor(utxo.descriptor_id))?;
    let sequence = if enable_rbf {
        TransactionInput::MAX_SEQUENCE_NONFINAL
    } else {
        TransactionInput::SEQUENCE_FINAL
    };

    let (script_sig, witness) = match &descriptor.descriptor {
        SingleKeyDescriptor::Pkh(key) => {
            let pubkey_len = if key.is_compressed() { 33 } else { 65 };
            let signature = vec![0_u8; 73];
            let public_key = vec![0_u8; pubkey_len];
            (
                super::push_script(&[signature.as_slice(), public_key.as_slice()])?,
                ScriptWitness::default(),
            )
        }
        SingleKeyDescriptor::ShWpkh(_) => {
            let redeem_script = descriptor
                .descriptor
                .redeem_script()?
                .expect("nested segwit always has redeem script");
            (
                super::push_script(&[redeem_script.as_bytes()])?,
                ScriptWitness::new(vec![vec![0_u8; 73], vec![0_u8; 33]]),
            )
        }
        SingleKeyDescriptor::Wpkh(_) => (
            ScriptBuf::default(),
            ScriptWitness::new(vec![vec![0_u8; 73], vec![0_u8; 33]]),
        ),
        SingleKeyDescriptor::Tr(_) => (
            ScriptBuf::default(),
            ScriptWitness::new(vec![vec![0_u8; 64]]),
        ),
    };

    Ok(TransactionInput {
        previous_output: utxo.outpoint.clone(),
        script_sig,
        sequence,
        witness,
    })
}

pub(super) fn amount_from_sats(sats: i64) -> Result<Amount, WalletError> {
    Ok(Amount::from_sats(sats)?)
}

pub(super) fn compare_wallet_utxos(left: &WalletUtxo, right: &WalletUtxo) -> Ordering {
    left.created_height
        .cmp(&right.created_height)
        .then_with(|| {
            left.outpoint
                .txid
                .as_bytes()
                .cmp(right.outpoint.txid.as_bytes())
        })
        .then_with(|| left.outpoint.vout.cmp(&right.outpoint.vout))
}

pub(super) fn compare_effective_value(
    wallet: &Wallet,
    fee_rate: FeeRate,
    left: &WalletUtxo,
    right: &WalletUtxo,
) -> Ordering {
    let left_effective = left.output.value.to_sats()
        - fee_rate.fee_for_virtual_size(
            wallet
                .descriptor(left.descriptor_id)
                .map(|record| record.descriptor.estimated_input_vbytes())
                .unwrap_or(0),
        );
    let right_effective = right.output.value.to_sats()
        - fee_rate.fee_for_virtual_size(
            wallet
                .descriptor(right.descriptor_id)
                .map(|record| record.descriptor.estimated_input_vbytes())
                .unwrap_or(0),
        );

    right_effective
        .cmp(&left_effective)
        .then_with(|| {
            right
                .output
                .value
                .to_sats()
                .cmp(&left.output.value.to_sats())
        })
        .then_with(|| {
            left.outpoint
                .txid
                .as_bytes()
                .cmp(right.outpoint.txid.as_bytes())
        })
        .then_with(|| left.outpoint.vout.cmp(&right.outpoint.vout))
}
