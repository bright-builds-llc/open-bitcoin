// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Individual-first package evaluation and the dry-run/submit capability boundary.

use std::collections::BTreeMap;

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_primitives::{Amount, Transaction};

use crate::{
    AdmissionContext, DryRunPackageCommand, DryRunPackageResult, EffectiveFeeGroup,
    EffectiveFeeGroupId, FeeRate, FinalMempoolMembership, HardMemberFailure, MempoolError,
    MempoolLifecycleDelta, MempoolMemberIdentity, MempoolMemberState, NewlyPresent,
    PackageMemberResult, PackageReport, PackageStatus, ReconsiderableMemberFailure,
    SubmitPackageCommand, SubmittedPackageResult, TransactionVirtualSize, WellFormedPackage,
    WitnessAlias,
};

use super::admission::lifecycle_invariant_error;
use super::candidate::{PreparedCandidate, check_candidate_scripts, prepare_candidate};
use super::prospective::ProspectiveMempool;
use super::{Mempool, MempoolPatch};

#[cfg(test)]
use std::cell::Cell;

#[cfg(test)]
thread_local! {
    static SCRIPT_CHECK_COUNT: Cell<usize> = const { Cell::new(0) };
    static FORCE_RESIDUAL_FEE_GROUP_ERROR: Cell<bool> = const { Cell::new(false) };
}

pub(super) struct PreparedPackageEvaluation {
    pub(super) report: PackageReport,
    pub(super) patch: Option<MempoolPatch>,
}

impl Mempool {
    /// Evaluates a package without exposing or committing the prepared transition.
    pub fn dry_run_package(
        &self,
        command: DryRunPackageCommand,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<DryRunPackageResult, MempoolError> {
        #[cfg(test)]
        let before = self.complete_snapshot();
        let evaluation = evaluate_package(
            self,
            &command.package,
            command.context,
            chainstate,
            verify_flags,
            consensus_params,
        )?;
        #[cfg(test)]
        assert_eq!(self.complete_snapshot(), before);
        Ok(DryRunPackageResult {
            report: evaluation.report,
        })
    }

    /// Evaluates one checked submission capability and guardedly applies its prepared facts.
    pub fn submit_package(
        &mut self,
        command: SubmitPackageCommand,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<SubmittedPackageResult, MempoolError> {
        let _submission_kind = command.package.kind();
        let evaluation = evaluate_package(
            self,
            command.package.package(),
            command.context,
            chainstate,
            verify_flags,
            consensus_params,
        )?;
        let delta = if let Some(patch) = evaluation.patch {
            self.apply_prepared(patch)?
        } else {
            MempoolLifecycleDelta::empty()
        };
        Ok(SubmittedPackageResult {
            report: evaluation.report,
            delta,
        })
    }
}

fn evaluate_package(
    base: &Mempool,
    package: &WellFormedPackage,
    context: AdmissionContext,
    chainstate: &ChainstateSnapshot,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<PreparedPackageEvaluation, MempoolError> {
    let mut prospective = ProspectiveMempool::new(base);
    let request_members = package.members_with_identities().collect::<Vec<_>>();
    let mut results = Vec::with_capacity(package.len());
    let mut reconsiderable_indices = Vec::new();
    let mut skip_residual = false;
    let mut groups = BTreeMap::new();
    let mut maybe_individual_group_ids = vec![None; package.len()];

    for (index, (identity, transaction)) in request_members.iter().copied().enumerate() {
        if let Some(existing) = prospective.maybe_entry(&identity.txid) {
            results.push(if existing.wtxid == identity.wtxid {
                PackageMemberResult::AlreadyPresent(crate::ExistingMember {
                    requested: identity,
                })
            } else {
                PackageMemberResult::SameTxidDifferentWitness(WitnessAlias {
                    requested: identity,
                    existing_wtxid: existing.wtxid,
                })
            });
            continue;
        }

        match evaluate_singleton(
            &prospective,
            transaction,
            identity,
            index,
            context,
            chainstate,
            verify_flags,
            consensus_params,
        )? {
            SingletonEvaluation::Accepted {
                next,
                result,
                group,
            } => {
                maybe_individual_group_ids[index] = Some(group.id());
                groups.insert(group.id(), group);
                results.push(result);
                prospective = *next;
            }
            SingletonEvaluation::Reconsiderable {
                result,
                maybe_group,
            } => {
                if let Some(group) = maybe_group {
                    maybe_individual_group_ids[index] = Some(group.id());
                    groups.insert(group.id(), group);
                }
                results.push(result);
                reconsiderable_indices.push(index);
            }
            SingletonEvaluation::Hard(result) => {
                results.push(result);
                skip_residual = true;
            }
        }
    }

    if !skip_residual && !reconsiderable_indices.is_empty() {
        evaluate_residual(
            &mut prospective,
            &request_members,
            &reconsiderable_indices,
            &mut results,
            &mut groups,
            &maybe_individual_group_ids,
            context,
            chainstate,
            verify_flags,
            consensus_params,
        )?;
    }

    let status = package_status(&results);
    let effective_fee_groups = groups.into_values().collect::<Vec<_>>();
    let report = PackageReport::try_new(package, status, results, effective_fee_groups)
        .map_err(report_invariant_error)?;
    let delta = lifecycle_delta(&report)?;
    let patch = if delta.admitted.is_empty() {
        None
    } else {
        Some(prospective.prepare_patch(delta)?)
    };
    Ok(PreparedPackageEvaluation { report, patch })
}

#[allow(clippy::too_many_arguments)]
fn evaluate_singleton<'base>(
    prospective: &ProspectiveMempool<'base>,
    transaction: &Transaction,
    identity: MempoolMemberIdentity,
    index: usize,
    context: AdmissionContext,
    chainstate: &ChainstateSnapshot,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<SingletonEvaluation<'base>, MempoolError> {
    let prepared = match prepare_candidate(
        prospective,
        transaction.clone(),
        chainstate,
        consensus_params,
        context,
    ) {
        Ok(prepared) => prepared,
        Err(MempoolError::MissingInput { .. }) => {
            return Ok(SingletonEvaluation::Reconsiderable {
                result: PackageMemberResult::Reconsiderable(
                    ReconsiderableMemberFailure::MissingInputs {
                        requested: identity,
                    },
                ),
                maybe_group: None,
            });
        }
        Err(error) => return Ok(SingletonEvaluation::Hard(hard_failure(identity, error))),
    };
    let group = fee_group(group_id(index), [(&prepared, identity)])?;
    if prospective
        .enforce_admission_fee(
            prepared.fees.modified.to_sats(),
            prepared.entry.virtual_size,
        )
        .is_err()
    {
        return Ok(SingletonEvaluation::Reconsiderable {
            result: PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::PackageFee {
                requested: identity,
                effective_fee_group_id: group.id(),
            }),
            maybe_group: Some(group),
        });
    }

    let mut next = prospective.clone();
    next.stage_candidate(prepared.clone())?;
    if let Err(error) = next.validate_candidate_limits(identity.txid) {
        return Ok(SingletonEvaluation::Hard(hard_failure(identity, error)));
    }
    if let Err(error) = run_script_checks(&prepared, verify_flags) {
        return Ok(SingletonEvaluation::Hard(hard_failure(identity, error)));
    }
    Ok(SingletonEvaluation::Accepted {
        next: Box::new(next),
        result: PackageMemberResult::FinallyPresent(NewlyPresent {
            requested: identity,
            effective_fee_group_id: group.id(),
        }),
        group,
    })
}

#[allow(clippy::too_many_arguments)]
fn evaluate_residual(
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
    let mut working = prospective.clone();
    let mut prepared_members = Vec::with_capacity(indices.len());
    for index in indices {
        let (identity, transaction) = request_members[*index];
        let prepared = match prepare_candidate(
            &working,
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
        working.stage_candidate(prepared.clone())?;
        if let Err(error) = working.validate_candidate_limits(identity.txid) {
            results[*index] = hard_failure(identity, error);
            return Ok(());
        }
        prepared_members.push((*index, identity, prepared));
    }

    let residual_group_id = group_id(indices[0]);
    let group = fee_group(
        residual_group_id,
        prepared_members
            .iter()
            .map(|(_index, identity, prepared)| (prepared, *identity)),
    )?;
    if working
        .enforce_admission_fee(group.modified_fee_sats().to_sats(), group.virtual_size())
        .is_err()
    {
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

    for (index, identity, prepared) in &prepared_members {
        if let Err(error) = run_script_checks(prepared, verify_flags) {
            results[*index] = hard_failure(*identity, error);
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

fn remove_individual_groups(
    groups: &mut BTreeMap<EffectiveFeeGroupId, EffectiveFeeGroup>,
    indices: &[usize],
    individual_group_ids: &[Option<EffectiveFeeGroupId>],
) {
    for index in indices {
        if let Some(group_id) = individual_group_ids[*index] {
            groups.remove(&group_id);
        }
    }
}

enum SingletonEvaluation<'base> {
    Accepted {
        next: Box<ProspectiveMempool<'base>>,
        result: PackageMemberResult,
        group: EffectiveFeeGroup,
    },
    Reconsiderable {
        result: PackageMemberResult,
        maybe_group: Option<EffectiveFeeGroup>,
    },
    Hard(PackageMemberResult),
}

fn group_id(index: usize) -> EffectiveFeeGroupId {
    EffectiveFeeGroupId::from_u64(index.saturating_add(1) as u64)
}

fn fee_group<'candidate>(
    id: EffectiveFeeGroupId,
    members: impl IntoIterator<Item = (&'candidate PreparedCandidate, MempoolMemberIdentity)>,
) -> Result<EffectiveFeeGroup, MempoolError> {
    let mut ordered_wtxids = Vec::new();
    let mut base_fee_sats = 0_i64;
    let mut modified_fee_sats = 0_i64;
    let mut virtual_size = TransactionVirtualSize::ZERO;
    for (prepared, identity) in members {
        ordered_wtxids.push(identity.wtxid);
        base_fee_sats = base_fee_sats
            .checked_add(prepared.fees.base.to_sats())
            .ok_or_else(|| group_invariant("base fee overflow"))?;
        modified_fee_sats = modified_fee_sats
            .checked_add(prepared.fees.modified.to_sats())
            .ok_or_else(|| group_invariant("modified fee overflow"))?;
        virtual_size = virtual_size
            .checked_add(
                prepared.entry.virtual_size,
                "package fee group virtual size",
            )
            .map_err(super::resource_invariant_error)?;
    }
    #[cfg(test)]
    if ordered_wtxids.len() > 1 && FORCE_RESIDUAL_FEE_GROUP_ERROR.with(Cell::get) {
        return Err(group_invariant("injected residual failure"));
    }
    let base_fee_sats =
        Amount::from_sats(base_fee_sats).map_err(|_| group_invariant("base fee out of range"))?;
    let modified_fee_sats = Amount::from_sats(modified_fee_sats)
        .map_err(|_| group_invariant("modified fee out of range"))?;
    let effective_fee_rate =
        FeeRate::from_fee_sats_and_vbytes(modified_fee_sats.to_sats(), virtual_size);
    EffectiveFeeGroup::try_new(
        id,
        ordered_wtxids,
        base_fee_sats,
        modified_fee_sats,
        virtual_size,
        effective_fee_rate,
    )
    .map_err(|error| MempoolError::InternalInvariant {
        reason: error.to_string(),
    })
}

fn group_invariant(reason: &'static str) -> MempoolError {
    MempoolError::InternalInvariant {
        reason: format!("package fee group {reason}"),
    }
}

fn hard_failure(identity: MempoolMemberIdentity, error: MempoolError) -> PackageMemberResult {
    PackageMemberResult::HardRejected(HardMemberFailure::Policy {
        requested: identity,
        reason: error.to_string(),
    })
}

fn package_status(members: &[PackageMemberResult]) -> PackageStatus {
    let present = members
        .iter()
        .filter(|member| {
            matches!(
                member,
                PackageMemberResult::FinallyPresent(_)
                    | PackageMemberResult::AlreadyPresent(_)
                    | PackageMemberResult::SameTxidDifferentWitness(_)
            )
        })
        .count();
    if present == members.len() {
        PackageStatus::Complete
    } else if present == 0 {
        PackageStatus::Failed
    } else {
        PackageStatus::Partial
    }
}

fn lifecycle_delta(report: &PackageReport) -> Result<MempoolLifecycleDelta, MempoolError> {
    let mut builder = MempoolLifecycleDelta::builder();
    for member in report.members() {
        let PackageMemberResult::FinallyPresent(result) = member else {
            continue;
        };
        builder
            .record_admitted(result.requested)
            .map_err(lifecycle_invariant_error)?;
        builder
            .record_final_membership(MempoolMemberState {
                member: result.requested,
                membership: FinalMempoolMembership::Present,
            })
            .map_err(lifecycle_invariant_error)?;
    }
    builder.build().map_err(lifecycle_invariant_error)
}

fn report_invariant_error(error: crate::PackageReportError) -> MempoolError {
    MempoolError::InternalInvariant {
        reason: error.to_string(),
    }
}

fn run_script_checks(
    prepared: &PreparedCandidate,
    verify_flags: ScriptVerifyFlags,
) -> Result<(), MempoolError> {
    #[cfg(test)]
    SCRIPT_CHECK_COUNT.with(|count| count.set(count.get() + 1));
    check_candidate_scripts(prepared, verify_flags)
}

#[cfg(test)]
pub(super) fn reset_script_check_count_for_test() {
    SCRIPT_CHECK_COUNT.with(|count| count.set(0));
}

#[cfg(test)]
pub(super) fn script_check_count_for_test() -> usize {
    SCRIPT_CHECK_COUNT.with(Cell::get)
}

#[cfg(test)]
pub(super) fn force_residual_fee_group_error_for_test(force: bool) {
    FORCE_RESIDUAL_FEE_GROUP_ERROR.with(|value| value.set(force));
}

#[cfg(test)]
pub(super) fn empty_fee_group_error_for_test() -> MempoolError {
    fee_group(group_id(0), std::iter::empty()).expect_err("empty group must fail")
}

#[cfg(test)]
pub(super) fn group_invariant_for_test() -> MempoolError {
    group_invariant("test invariant")
}

#[cfg(test)]
pub(super) fn report_invariant_error_for_test(error: crate::PackageReportError) -> MempoolError {
    report_invariant_error(error)
}

#[cfg(test)]
pub(super) fn evaluate_package_for_test(
    base: &Mempool,
    package: &WellFormedPackage,
    context: AdmissionContext,
    chainstate: &ChainstateSnapshot,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<PreparedPackageEvaluation, MempoolError> {
    evaluate_package(
        base,
        package,
        context,
        chainstate,
        verify_flags,
        consensus_params,
    )
}
