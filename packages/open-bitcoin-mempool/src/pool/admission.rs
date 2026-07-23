// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_entry.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid,
    validate_transaction_with_context,
};
use open_bitcoin_primitives::Transaction;

use crate::{
    AdmissionContext, AdmissionResult, MempoolEntry, MempoolError, MempoolOutcome,
    TransactionVirtualSize, effective_admission_fee_rate, transaction_sigops_cost,
    transaction_weight_and_virtual_size, validate_standard_transaction,
};

use super::{
    Mempool, accept_outcome, build_validation_context, derive_input_contexts,
    enforce_min_relay_fee, recompute_state, resource_invariant_error,
    serialization_validation_error, trim_to_size, validate_limits,
};

impl Mempool {
    /// Fail-closed migration adapter that assigns legacy-unknown metadata.
    ///
    /// Plan 130-05 migrates production node admission. Plan 130-11 migrates the
    /// final local RPC caller and removes this adapter.
    #[deprecated(
        note = "Plan 130-05 migrates production node admission; Plan 130-11 migrates the final local RPC caller and removes this fail-closed adapter"
    )]
    pub fn accept_transaction(
        &mut self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, MempoolError> {
        self.accept_transaction_with_context(
            transaction,
            chainstate,
            verify_flags,
            consensus_params,
            AdmissionContext::legacy_unknown(),
        )
    }

    /// Attempts admission using metadata supplied by an effectful adapter.
    pub fn accept_transaction_with_context(
        &mut self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<AdmissionResult, MempoolError> {
        let txid = transaction_txid(&transaction)
            .map_err(|source| serialization_validation_error("transaction txid", source))?;
        if self.entries.contains_key(&txid) {
            return Err(MempoolError::DuplicateTransaction { txid });
        }

        let input_contexts = derive_input_contexts(&transaction, chainstate, &self.entries)?;
        let (weight, virtual_size_bytes) = transaction_weight_and_virtual_size(&transaction)?;
        let virtual_size = TransactionVirtualSize::new(virtual_size_bytes);
        let sigops_cost = transaction_sigops_cost(&transaction, &input_contexts)?;
        validate_standard_transaction(
            &transaction,
            &input_contexts,
            &self.config,
            weight,
            sigops_cost,
        )?;

        let validation_context =
            build_validation_context(chainstate, input_contexts, verify_flags, consensus_params);
        let fee = validate_transaction_with_context(&transaction, &validation_context).map_err(
            |source| MempoolError::Validation {
                reason: source.to_string(),
            },
        )?;
        let effective_fee_rate = effective_admission_fee_rate(
            self.config.static_relay_fee_rate,
            self.rolling_mempool_fee_rate,
        );
        enforce_min_relay_fee(effective_fee_rate, fee.to_sats(), virtual_size)?;
        let replace_set = self.replacement_set(&transaction, fee.to_sats(), virtual_size)?;

        let wtxid = transaction_wtxid(&transaction)
            .map_err(|source| serialization_validation_error("transaction wtxid", source))?;
        let mut prospective_entries = self.entries.clone();
        for conflict_txid in &replace_set {
            prospective_entries.remove(conflict_txid);
        }
        prospective_entries.insert(
            txid,
            MempoolEntry::new(
                transaction,
                txid,
                wtxid,
                fee,
                virtual_size,
                weight,
                sigops_cost,
                context.metadata,
            ),
        );

        let prospective_state =
            recompute_state(prospective_entries).map_err(resource_invariant_error)?;
        validate_limits(&prospective_state.entries, &self.config, txid)?;
        let (trimmed_state, evicted) = trim_to_size(prospective_state, &self.config)?;
        if !trimmed_state.entries.contains_key(&txid) {
            return Err(MempoolError::CandidateEvicted { txid });
        }

        self.entries = trimmed_state.entries;
        self.spent_outpoints = trimmed_state.spent_outpoints;
        self.resource_ledger = trimmed_state.resource_ledger;

        Ok(AdmissionResult {
            accepted: txid,
            replaced: replace_set.into_iter().collect(),
            evicted: evicted.into_iter().collect(),
        })
    }

    /// Fail-closed outcome migration adapter that assigns legacy-unknown metadata.
    ///
    /// Plan 130-05 migrates production node admission. Plan 130-11 migrates the
    /// final local RPC caller and removes this adapter.
    #[deprecated(
        note = "Plan 130-05 migrates production node admission; Plan 130-11 migrates the final local RPC caller and removes this fail-closed adapter"
    )]
    pub fn accept_transaction_outcome(
        &mut self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<MempoolOutcome, MempoolError> {
        self.accept_transaction_outcome_with_context(
            transaction,
            chainstate,
            verify_flags,
            consensus_params,
            AdmissionContext::legacy_unknown(),
        )
    }

    /// Returns the stable admission outcome while preserving supplied metadata.
    pub fn accept_transaction_outcome_with_context(
        &mut self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<MempoolOutcome, MempoolError> {
        accept_outcome(
            self,
            transaction,
            chainstate,
            verify_flags,
            consensus_params,
            context,
        )
    }
}
