// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Final package report rewriting and lifecycle assembly after pressure trimming.

use crate::{
    FinalMempoolMembership, MempoolError, MempoolLifecycleDelta, MempoolLifecycleRemoval,
    MempoolMemberState, PackageMemberResult, PackageReport, PostTrimAbsence, PriorMemberSuccess,
};

use super::super::admission::lifecycle_invariant_error;
use super::super::prospective::ProspectiveMempool;

pub(super) fn rewrite_final_membership(
    prospective: &ProspectiveMempool<'_>,
    results: &mut [PackageMemberResult],
) {
    for result in results {
        let maybe_absent = match result {
            PackageMemberResult::FinallyPresent(present)
                if prospective
                    .maybe_entry(&present.requested.txid)
                    .is_none_or(|entry| entry.wtxid != present.requested.wtxid) =>
            {
                Some(PostTrimAbsence {
                    requested: present.requested,
                    prior: PriorMemberSuccess::FinallyPresent {
                        effective_fee_group_id: present.effective_fee_group_id,
                    },
                })
            }
            PackageMemberResult::AlreadyPresent(existing)
                if prospective
                    .maybe_entry(&existing.requested.txid)
                    .is_none_or(|entry| entry.wtxid != existing.requested.wtxid) =>
            {
                Some(PostTrimAbsence {
                    requested: existing.requested,
                    prior: PriorMemberSuccess::AlreadyPresent,
                })
            }
            PackageMemberResult::SameTxidDifferentWitness(alias) => {
                if let Some(existing) = prospective.maybe_entry(&alias.requested.txid) {
                    alias.existing_wtxid = existing.wtxid;
                    None
                } else {
                    Some(PostTrimAbsence {
                        requested: alias.requested,
                        prior: PriorMemberSuccess::SameTxidDifferentWitness {
                            existing_wtxid: alias.existing_wtxid,
                        },
                    })
                }
            }
            _ => None,
        };
        if let Some(absent) = maybe_absent {
            *result = PackageMemberResult::PostTrimAbsent(absent);
        }
    }
}

pub(super) fn lifecycle_delta(
    report: &PackageReport,
    prospective: &ProspectiveMempool<'_>,
) -> Result<MempoolLifecycleDelta, MempoolError> {
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
    for (member, fact) in prospective
        .removal_facts()
        .filter(|(member, _fact)| prospective.base_contains(*member))
    {
        builder
            .record_removal(MempoolLifecycleRemoval {
                member,
                cause: fact.cause,
                role: fact.role,
            })
            .map_err(lifecycle_invariant_error)?;
        builder
            .record_final_membership(MempoolMemberState {
                member,
                membership: FinalMempoolMembership::Absent,
            })
            .map_err(lifecycle_invariant_error)?;
    }
    builder.build().map_err(lifecycle_invariant_error)
}
