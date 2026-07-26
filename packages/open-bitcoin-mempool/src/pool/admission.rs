// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_entry.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_primitives::Transaction;

use crate::{
    AdmissionContext, AdmissionResult, FinalMempoolMembership, MempoolError, MempoolLifecycleDelta,
    MempoolLifecycleRemoval, MempoolMemberIdentity, MempoolMemberState, MempoolOutcome,
    MempoolRemovalCause, MempoolRemovalRole, MempoolRetryClear, MempoolRetryClearCause,
    MempoolTransition, effective_admission_fee_rate,
};

use super::candidate::{check_candidate_scripts, prepare_candidate};
use super::patch::prepare_admission_layout;
use super::{Mempool, MempoolPatch, accept_outcome, enforce_min_relay_fee};

pub(super) struct CommittedAdmission {
    pub result: AdmissionResult,
    pub delta: MempoolLifecycleDelta,
}

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
        self.commit_transaction_with_context(
            transaction,
            chainstate,
            verify_flags,
            consensus_params,
            context,
        )
        .map(|committed| committed.result)
    }

    pub(super) fn commit_transaction_with_context(
        &mut self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<CommittedAdmission, MempoolError> {
        let prepared = prepare_candidate(self, transaction, chainstate, consensus_params, context)?;
        let (patch, result) = prepare_admission_patch(self, &prepared)?;
        check_candidate_scripts(&prepared, verify_flags)?;
        let delta = self.apply_prepared(patch)?;

        Ok(CommittedAdmission { result, delta })
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
        self.accept_transaction_transition_with_context(
            transaction,
            chainstate,
            verify_flags,
            consensus_params,
            AdmissionContext::legacy_unknown(),
        )
        .map(|transition| transition.outcome)
    }

    /// Compatibility projection for callers that have not migrated to committed deltas.
    ///
    /// Plan 130-05 migrates production node admission. Plan 130-11 migrates the
    /// final local RPC caller and removes this projection.
    #[deprecated(
        note = "Plan 130-05 migrates production node admission; Plan 130-11 removes this transition-derived compatibility projection"
    )]
    pub fn accept_transaction_outcome_with_context(
        &mut self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<MempoolOutcome, MempoolError> {
        self.accept_transaction_transition_with_context(
            transaction,
            chainstate,
            verify_flags,
            consensus_params,
            context,
        )
        .map(|transition| transition.outcome)
    }

    /// Attempts admission and returns both attempt vocabulary and committed lifecycle facts.
    pub fn accept_transaction_transition_with_context(
        &mut self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<MempoolTransition, MempoolError> {
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

pub(super) fn prepare_admission_patch(
    mempool: &Mempool,
    prepared: &super::candidate::PreparedCandidate,
) -> Result<(MempoolPatch, AdmissionResult), MempoolError> {
    let txid = prepared.entry.txid;
    let wtxid = prepared.entry.wtxid;
    let fee = prepared.fees.modified;
    let virtual_size = prepared.entry.virtual_size;
    let effective_fee_rate = effective_admission_fee_rate(
        mempool.config.static_relay_fee_rate,
        mempool.rolling_mempool_fee_rate(),
    );
    enforce_min_relay_fee(effective_fee_rate, fee.to_sats(), virtual_size)?;
    let direct_conflicts = mempool.direct_conflicts(&prepared.entry.transaction);
    let replace_set =
        mempool.replacement_set(&prepared.entry.transaction, fee.to_sats(), virtual_size)?;
    let replacement_members = replace_set
        .iter()
        .filter_map(|replaced_txid| {
            mempool.entries.get(replaced_txid).map(|entry| {
                (
                    MempoolMemberIdentity {
                        txid: *replaced_txid,
                        wtxid: entry.wtxid,
                    },
                    if direct_conflicts.contains(replaced_txid) {
                        MempoolRemovalRole::Direct
                    } else {
                        MempoolRemovalRole::Descendant
                    },
                )
            })
        })
        .collect::<Vec<_>>();

    let (layout, evicted, prospective_rolling) =
        prepare_admission_layout(mempool, &prepared.entry, replace_set.clone())?;
    if !layout.contains(txid) {
        return Err(MempoolError::CandidateEvicted { txid });
    }

    let admitted = MempoolMemberIdentity { txid, wtxid };
    let mut delta_builder = MempoolLifecycleDelta::builder();
    delta_builder
        .record_admitted(admitted)
        .map_err(lifecycle_invariant_error)?;
    delta_builder
        .record_final_membership(MempoolMemberState {
            member: admitted,
            membership: FinalMempoolMembership::Present,
        })
        .map_err(lifecycle_invariant_error)?;
    for (member, role) in replacement_members {
        record_committed_removal(
            &mut delta_builder,
            member,
            MempoolRemovalCause::Replacement,
            role,
        )
        .map_err(lifecycle_invariant_error)?;
    }
    for (member, role) in &evicted {
        record_committed_removal(
            &mut delta_builder,
            *member,
            MempoolRemovalCause::Pressure,
            *role,
        )
        .map_err(lifecycle_invariant_error)?;
    }
    let delta = delta_builder.build().map_err(lifecycle_invariant_error)?;
    let result = AdmissionResult {
        accepted: txid,
        replaced: replace_set.into_iter().collect(),
        evicted: evicted.into_keys().map(|member| member.txid).collect(),
    };
    let patch = layout.into_patch(mempool, prospective_rolling, delta)?;
    Ok((patch, result))
}

fn record_committed_removal(
    builder: &mut crate::MempoolLifecycleDeltaBuilder,
    member: MempoolMemberIdentity,
    cause: MempoolRemovalCause,
    role: MempoolRemovalRole,
) -> Result<(), crate::MempoolLifecycleInvariantError> {
    builder.record_removal(MempoolLifecycleRemoval {
        member,
        cause,
        role,
    })?;
    builder.record_final_membership(MempoolMemberState {
        member,
        membership: FinalMempoolMembership::Absent,
    })?;
    builder.record_retry_clear(MempoolRetryClear {
        member,
        cause: MempoolRetryClearCause::LifecycleRemoval,
    })
}

pub(super) fn lifecycle_invariant_error(
    source: crate::MempoolLifecycleInvariantError,
) -> MempoolError {
    MempoolError::InternalInvariant {
        reason: source.to_string(),
    }
}
