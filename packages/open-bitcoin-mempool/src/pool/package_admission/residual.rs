// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Residual package preparation and coherent replacement composition.

use std::collections::BTreeMap;

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_primitives::{OutPoint, Transaction, Txid};

use crate::policy::replacement::{MempoolView, evaluate_limited_package_replacement};
use crate::{
    AdmissionContext, EffectiveFeeGroup, EffectiveFeeGroupId, HardMemberFailure, MempoolEntry,
    MempoolError, MempoolMemberIdentity, MempoolRemovalCause, NewlyPresent, PackageMemberResult,
    ReconsiderableMemberFailure,
};

use super::{
    FeeGroupDecision, ProspectiveMempool, fee_group, group_id, hard_failure,
    remove_individual_groups, run_late_script_checks,
};
use crate::pool::candidate::{CandidateMempoolView, PreparedCandidate, prepare_candidate};
use crate::pool::lifecycle::MempoolRemovalFact;
use crate::pool::prospective::SubDelta;

#[cfg(test)]
thread_local! {
    static FORCE_DUPLICATE_TRANSITION_ENTRY: std::cell::Cell<bool> = const {
        std::cell::Cell::new(false)
    };
}

#[allow(clippy::too_many_arguments)]
pub(super) fn evaluate(
    prospective: &mut ProspectiveMempool<'_>,
    request_members: &[(MempoolMemberIdentity, &Transaction)],
    indices: &[usize],
    results: &mut [PackageMemberResult],
    groups: &mut BTreeMap<EffectiveFeeGroupId, EffectiveFeeGroup>,
    individual_group_ids: &[Option<EffectiveFeeGroupId>],
    context: AdmissionContext,
    chainstate: &ChainstateSnapshot,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<(), MempoolError> {
    let mut prepared_members = Vec::with_capacity(indices.len());
    let mut preparation_view = ResidualPreparationView::new(prospective);
    for index in indices {
        let (identity, transaction) = request_members[*index];
        let prepared = match prepare_candidate(
            &preparation_view,
            transaction.clone(),
            chainstate,
            consensus_params,
            context,
        ) {
            Ok(prepared) => prepared,
            Err(MempoolError::MissingInput { .. }) => {
                results[*index] = PackageMemberResult::Reconsiderable(
                    ReconsiderableMemberFailure::MissingInputs {
                        requested: identity,
                    },
                );
                return Ok(());
            }
            Err(error) => {
                results[*index] = hard_failure(identity, error);
                return Ok(());
            }
        };
        preparation_view.insert(prepared.entry.clone());
        prepared_members.push((*index, identity, prepared));
    }
    for (_index, _identity, prepared) in &prepared_members {
        let _has_conflict = has_direct_conflict(prospective, prepared)?;
    }

    let residual_group_id = group_id(indices[0]);
    let group = match fee_group(
        prospective,
        residual_group_id,
        prepared_members
            .iter()
            .map(|(_index, identity, prepared)| (prepared, *identity)),
    )? {
        FeeGroupDecision::Accepted(group) => group,
        FeeGroupDecision::Reconsiderable(group) => {
            for (index, identity, _prepared) in &prepared_members {
                results[*index] =
                    PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::PackageFee {
                        requested: *identity,
                        effective_fee_group_id: group.id(),
                    });
            }
            remove_individual_groups(groups, indices, individual_group_ids);
            groups.insert(group.id(), group);
            return Ok(());
        }
        FeeGroupDecision::Hard(error) => {
            set_hard_failures(results, &prepared_members, error);
            remove_individual_groups(groups, indices, individual_group_ids);
            return Ok(());
        }
    };

    let prepared_candidates = prepared_members
        .iter()
        .map(|(_index, _identity, prepared)| prepared.clone())
        .collect::<Vec<_>>();
    let has_conflict = prepared_candidates
        .iter()
        .try_fold(false, |found, prepared| {
            has_direct_conflict(prospective, prepared).map(|conflict| found || conflict)
        })?;
    let replacement = if has_conflict {
        match evaluate_limited_package_replacement(
            prospective,
            &prepared_candidates,
            prospective.policy_config().incremental_relay_fee_rate,
        ) {
            Ok(replacement) => Some(replacement),
            Err(error) => {
                for (index, identity, _prepared) in &prepared_members {
                    results[*index] =
                        PackageMemberResult::HardRejected(HardMemberFailure::PackageReplacement {
                            requested: *identity,
                            reason: error.to_string(),
                        });
                }
                remove_individual_groups(groups, indices, individual_group_ids);
                return Ok(());
            }
        }
    } else {
        None
    };
    let removals = replacement
        .map(|replacement| {
            replacement
                .removals
                .into_iter()
                .map(|(member, role)| {
                    (
                        member,
                        MempoolRemovalFact {
                            cause: MempoolRemovalCause::Replacement,
                            role,
                        },
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    #[allow(unused_mut)] // Test-only failure injection appends one duplicate entry.
    let mut transition_entries = prepared_candidates
        .iter()
        .map(|prepared| prepared.entry.clone())
        .collect::<Vec<_>>();
    #[cfg(test)]
    if FORCE_DUPLICATE_TRANSITION_ENTRY.with(std::cell::Cell::get) {
        transition_entries.push(prepared_candidates[0].entry.clone());
    }
    let transition = SubDelta::transition(transition_entries, removals)?;
    let mut working = prospective.clone();
    working.compose(transition)?;
    for (_index, identity, _prepared) in &prepared_members {
        if let Err(error) = working.validate_candidate_limits(identity.txid) {
            set_hard_failures(results, &prepared_members, error);
            remove_individual_groups(groups, indices, individual_group_ids);
            return Ok(());
        }
    }

    for (_index, _identity, prepared) in &prepared_members {
        if let Err(error) = run_late_script_checks(prepared, verify_flags) {
            set_hard_failures(results, &prepared_members, error);
            remove_individual_groups(groups, indices, individual_group_ids);
            return Ok(());
        }
    }
    for (index, identity, _prepared) in &prepared_members {
        results[*index] = PackageMemberResult::FinallyPresent(NewlyPresent {
            requested: *identity,
            effective_fee_group_id: group.id(),
        });
    }
    remove_individual_groups(groups, indices, individual_group_ids);
    groups.insert(group.id(), group);
    *prospective = working;
    Ok(())
}

pub(super) fn has_direct_conflict(
    view: &ProspectiveMempool<'_>,
    candidate: &PreparedCandidate,
) -> Result<bool, MempoolError> {
    let mut found = false;
    for input in &candidate.entry.transaction.inputs {
        let Some(spender) = view.maybe_spender(&input.previous_output) else {
            continue;
        };
        if view.maybe_entry(&spender).is_none() {
            return Err(MempoolError::InternalInvariant {
                reason: "prospective spent-outpoint index references a missing entry".to_string(),
            });
        }
        found = true;
    }
    Ok(found)
}

fn set_hard_failures(
    results: &mut [PackageMemberResult],
    prepared_members: &[(usize, MempoolMemberIdentity, PreparedCandidate)],
    error: MempoolError,
) {
    for (index, identity, _prepared) in prepared_members {
        results[*index] = hard_failure(*identity, error.clone());
    }
}

struct ResidualPreparationView<'view, 'base> {
    prospective: &'view ProspectiveMempool<'base>,
    prepared_entries: BTreeMap<Txid, MempoolEntry>,
}

impl<'view, 'base> ResidualPreparationView<'view, 'base> {
    fn new(prospective: &'view ProspectiveMempool<'base>) -> Self {
        Self {
            prospective,
            prepared_entries: BTreeMap::new(),
        }
    }

    fn insert(&mut self, entry: MempoolEntry) {
        self.prepared_entries.insert(entry.txid, entry);
    }
}

impl CandidateMempoolView for ResidualPreparationView<'_, '_> {
    fn config(&self) -> &crate::PolicyConfig {
        self.prospective.policy_config()
    }

    fn maybe_entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        self.prepared_entries
            .get(txid)
            .or_else(|| self.prospective.maybe_entry(txid))
    }
}

impl MempoolView for ProspectiveMempool<'_> {
    fn maybe_entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        ProspectiveMempool::maybe_entry(self, txid)
    }

    fn maybe_spender(&self, outpoint: &OutPoint) -> Option<Txid> {
        ProspectiveMempool::maybe_spender(self, outpoint)
    }

    fn collect_descendants(&self, txid: Txid) -> std::collections::BTreeSet<Txid> {
        ProspectiveMempool::collect_descendants(self, txid)
    }
}

#[cfg(test)]
pub(in crate::pool) fn force_duplicate_transition_entry_for_test(force: bool) {
    FORCE_DUPLICATE_TRANSITION_ENTRY.with(|value| value.set(force));
}
