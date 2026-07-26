// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Pre-script candidate preparation and the shared contextual script seam.

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptExecutionData, ScriptInputVerificationContext, ScriptVerifyFlags,
    TransactionInputContext, TransactionValidationContext, check_transaction, check_tx_inputs,
    is_final_transaction, sequence_locks, transaction_txid, transaction_wtxid, verify_input_script,
};
use open_bitcoin_primitives::{Amount, Transaction};

use crate::{
    AdmissionContext, Mempool, MempoolEntry, MempoolError, TransactionVirtualSize,
    transaction_sigops_cost, transaction_weight_and_virtual_size, validate_standard_transaction,
};

use super::{derive_input_contexts, serialization_validation_error};

/// The base and policy-modified fee facts prepared for admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CandidateFees {
    pub(super) base: Amount,
    pub(super) modified: Amount,
}

/// Canonical candidate facts that are complete before contextual script execution.
#[derive(Debug, Clone)]
pub(super) struct PreparedCandidate {
    pub(super) entry: MempoolEntry,
    pub(super) fees: CandidateFees,
    input_contexts: Vec<TransactionInputContext>,
    spend_height: u32,
    block_time: i64,
    median_time_past: i64,
    consensus_params: ConsensusParams,
}

/// Performs every context-free and contextual non-script check for one candidate.
///
/// This function intentionally accepts no [`ScriptVerifyFlags`]. Script execution is owned by
/// [`check_candidate_scripts`], after legacy mempool policy has accepted the prepared facts.
pub(super) fn prepare_candidate(
    mempool: &Mempool,
    transaction: Transaction,
    chainstate: &ChainstateSnapshot,
    consensus_params: ConsensusParams,
    context: AdmissionContext,
) -> Result<PreparedCandidate, MempoolError> {
    let txid = transaction_txid(&transaction)
        .map_err(|source| serialization_validation_error("transaction txid", source))?;
    if mempool.entries.contains_key(&txid) {
        return Err(MempoolError::DuplicateTransaction { txid });
    }
    let wtxid = transaction_wtxid(&transaction)
        .map_err(|source| serialization_validation_error("transaction wtxid", source))?;

    check_transaction(&transaction).map_err(validation_error)?;
    let input_contexts = derive_input_contexts(&transaction, chainstate, &mempool.entries)?;
    let (weight, virtual_size_bytes) = transaction_weight_and_virtual_size(&transaction)?;
    let virtual_size = TransactionVirtualSize::new(virtual_size_bytes);
    let sigops_cost = transaction_sigops_cost(&transaction, &input_contexts)?;
    validate_standard_transaction(
        &transaction,
        &input_contexts,
        &mempool.config,
        weight,
        sigops_cost,
    )?;

    let maybe_tip = chainstate.tip();
    let spend_height = maybe_tip.map_or(0, |tip| tip.height.saturating_add(1));
    let block_time = maybe_tip.map_or(0, |tip| i64::from(tip.header.time));
    let median_time_past = maybe_tip.map_or(0, |tip| tip.median_time_past);
    let validation_context = TransactionValidationContext {
        inputs: input_contexts.clone(),
        spend_height,
        block_time,
        median_time_past,
        verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        consensus_params,
    };
    let lock_time_cutoff = if consensus_params.enforce_bip113_median_time_past {
        median_time_past
    } else {
        block_time
    };
    if !is_final_transaction(
        &transaction,
        spend_height,
        lock_time_cutoff,
        &consensus_params,
    ) {
        return Err(MempoolError::Validation {
            reason: "non-final transaction".to_string(),
        });
    }
    if !sequence_locks(&transaction, &validation_context).map_err(validation_error)? {
        return Err(MempoolError::Validation {
            reason: "non-BIP68-final".to_string(),
        });
    }
    let base_fee = check_tx_inputs(&transaction, &validation_context).map_err(validation_error)?;

    Ok(PreparedCandidate {
        entry: MempoolEntry::new(
            transaction,
            txid,
            wtxid,
            base_fee,
            virtual_size,
            weight,
            sigops_cost,
            context.metadata,
        ),
        fees: CandidateFees {
            base: base_fee,
            modified: base_fee,
        },
        input_contexts,
        spend_height,
        block_time,
        median_time_past,
        consensus_params,
    })
}

/// Executes only contextual scripts against facts owned by a prepared candidate.
pub(super) fn check_candidate_scripts(
    prepared: &PreparedCandidate,
    verify_flags: ScriptVerifyFlags,
) -> Result<(), MempoolError> {
    let validation_context = TransactionValidationContext {
        inputs: prepared.input_contexts.clone(),
        spend_height: prepared.spend_height,
        block_time: prepared.block_time,
        median_time_past: prepared.median_time_past,
        verify_flags,
        consensus_params: prepared.consensus_params,
    };
    let precomputed = validation_context
        .precompute(&prepared.entry.transaction)
        .map_err(|source| serialization_validation_error("script precomputation", source))?;

    for (input_index, (input, input_context)) in prepared
        .entry
        .transaction
        .inputs
        .iter()
        .zip(&validation_context.inputs)
        .enumerate()
    {
        let mut execution_data = ScriptExecutionData::default();
        verify_input_script(ScriptInputVerificationContext {
            script_sig: &input.script_sig,
            script_pubkey: &input_context.spent_output.script_pubkey,
            witness: &input.witness,
            transaction: &prepared.entry.transaction,
            input_index,
            spent_input: input_context,
            validation_context: &validation_context,
            spent_amount: input_context.spent_output.value,
            verify_flags,
            precomputed: &precomputed,
            execution_data: &mut execution_data,
        })
        .map_err(|source| MempoolError::Validation {
            reason: format!("mandatory-script-verify-flag-failed: {source}"),
        })?;
    }

    Ok(())
}

fn validation_error(source: open_bitcoin_consensus::TxValidationError) -> MempoolError {
    MempoolError::Validation {
        reason: source.to_string(),
    }
}
