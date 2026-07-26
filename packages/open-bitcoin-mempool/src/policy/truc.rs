// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/policy/truc_policy.h
// - packages/bitcoin-knots/src/policy/truc_policy.cpp
// - packages/bitcoin-knots/test/functional/mempool_truc.py

//! Pure version-3 transaction policy over a pre-replacement mempool view.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use open_bitcoin_primitives::Txid;

use super::replacement::MempoolView;
use crate::pool::candidate::PreparedCandidate;
use crate::{MempoolEntry, TrucPolicy};

pub const MAX_TRUC_VIRTUAL_SIZE: usize = 10_000;
pub const MAX_TRUC_CHILD_VIRTUAL_SIZE: usize = 1_000;
pub const MAX_TRUC_ANCESTOR_COUNT: usize = 2;
pub const MAX_TRUC_DESCENDANT_COUNT: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EligibleSiblingEviction {
    pub(crate) sibling: Txid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TrucPolicyError {
    Rejected { txid: Txid },
    TooLarge { txid: Txid, virtual_size: usize },
    ChildTooLarge { txid: Txid, virtual_size: usize },
    AncestorLimit { txid: Txid, count: usize },
    DescendantLimit { txid: Txid, count: usize },
    VersionInheritance { parent: Txid, child: Txid },
    SiblingTopology { parent: Txid },
    ParentAndChild { txid: Txid },
    IneligibleSibling { sibling: Txid },
}

impl fmt::Display for TrucPolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected { txid } => write!(formatter, "TRUC policy rejects version 3 {txid:?}"),
            Self::TooLarge { txid, virtual_size } => write!(
                formatter,
                "TRUC transaction {txid:?} virtual size {virtual_size} exceeds {MAX_TRUC_VIRTUAL_SIZE}"
            ),
            Self::ChildTooLarge { txid, virtual_size } => write!(
                formatter,
                "TRUC child {txid:?} virtual size {virtual_size} exceeds {MAX_TRUC_CHILD_VIRTUAL_SIZE}"
            ),
            Self::AncestorLimit { txid, count } => write!(
                formatter,
                "TRUC transaction {txid:?} ancestor count {count} exceeds {MAX_TRUC_ANCESTOR_COUNT}"
            ),
            Self::DescendantLimit { txid, count } => write!(
                formatter,
                "TRUC transaction {txid:?} descendant count {count} exceeds {MAX_TRUC_DESCENDANT_COUNT}"
            ),
            Self::VersionInheritance { parent, child } => write!(
                formatter,
                "TRUC version inheritance mismatch between parent {parent:?} and child {child:?}"
            ),
            Self::SiblingTopology { parent } => {
                write!(
                    formatter,
                    "TRUC parent {parent:?} would have multiple children"
                )
            }
            Self::ParentAndChild { txid } => write!(
                formatter,
                "TRUC package member {txid:?} cannot be both a parent and a child"
            ),
            Self::IneligibleSibling { sibling } => {
                write!(
                    formatter,
                    "TRUC sibling {sibling:?} is not eligible for eviction"
                )
            }
        }
    }
}

impl std::error::Error for TrucPolicyError {}

pub(crate) fn evaluate_truc_package<V: MempoolView>(
    view: &V,
    members: &[PreparedCandidate],
    policy: TrucPolicy,
    direct_conflicts: &BTreeSet<Txid>,
) -> Result<Option<EligibleSiblingEviction>, TrucPolicyError> {
    match policy {
        TrucPolicy::Reject => {
            if let Some(member) = members
                .iter()
                .find(|member| member.entry.transaction.version == 3)
            {
                return Err(TrucPolicyError::Rejected {
                    txid: member.entry.txid,
                });
            }
            Ok(None)
        }
        TrucPolicy::Accept => Ok(None),
        TrucPolicy::Enforce => {
            let maybe_sibling_eviction = find_sibling_eviction(view, members, direct_conflicts)?;
            validate_truc_package(
                view,
                members,
                policy,
                direct_conflicts,
                maybe_sibling_eviction,
            )?;
            Ok(maybe_sibling_eviction)
        }
    }
}

pub(crate) fn validate_truc_package<V: MempoolView>(
    view: &V,
    members: &[PreparedCandidate],
    policy: TrucPolicy,
    direct_conflicts: &BTreeSet<Txid>,
    maybe_sibling_eviction: Option<EligibleSiblingEviction>,
) -> Result<(), TrucPolicyError> {
    if policy != TrucPolicy::Enforce {
        return evaluate_non_enforced(members, policy);
    }

    let candidate_entries = members
        .iter()
        .map(|member| (member.entry.txid, &member.entry))
        .collect::<BTreeMap<_, _>>();
    let candidate_txids = candidate_entries.keys().copied().collect::<BTreeSet<_>>();

    for member in members {
        validate_member_size(&member.entry)?;
        validate_member_topology(
            view,
            &member.entry,
            &candidate_entries,
            &candidate_txids,
            direct_conflicts,
            maybe_sibling_eviction,
        )?;
    }
    Ok(())
}

fn evaluate_non_enforced(
    members: &[PreparedCandidate],
    policy: TrucPolicy,
) -> Result<(), TrucPolicyError> {
    if policy == TrucPolicy::Reject
        && let Some(member) = members
            .iter()
            .find(|member| member.entry.transaction.version == 3)
    {
        return Err(TrucPolicyError::Rejected {
            txid: member.entry.txid,
        });
    }
    Ok(())
}

fn validate_member_size(entry: &MempoolEntry) -> Result<(), TrucPolicyError> {
    if entry.transaction.version == 3 && entry.virtual_size.as_usize() > MAX_TRUC_VIRTUAL_SIZE {
        return Err(TrucPolicyError::TooLarge {
            txid: entry.txid,
            virtual_size: entry.virtual_size.as_usize(),
        });
    }
    Ok(())
}

fn validate_member_topology(
    view: &impl MempoolView,
    entry: &MempoolEntry,
    candidates: &BTreeMap<Txid, &MempoolEntry>,
    candidate_txids: &BTreeSet<Txid>,
    direct_conflicts: &BTreeSet<Txid>,
    maybe_sibling_eviction: Option<EligibleSiblingEviction>,
) -> Result<(), TrucPolicyError> {
    let parents = direct_parents(view, entry, candidates);
    let children = candidate_children(entry.txid, candidates);
    for parent in &parents {
        if is_truc(parent) != is_truc(entry) {
            return Err(TrucPolicyError::VersionInheritance {
                parent: parent.txid,
                child: entry.txid,
            });
        }
    }
    for child in &children {
        if is_truc(child) != is_truc(entry) {
            return Err(TrucPolicyError::VersionInheritance {
                parent: entry.txid,
                child: child.txid,
            });
        }
    }
    if !is_truc(entry) {
        return Ok(());
    }
    if !parents.is_empty() && !children.is_empty() {
        return Err(TrucPolicyError::ParentAndChild { txid: entry.txid });
    }

    let ancestor_count = prospective_ancestor_count(view, entry, candidates);
    if ancestor_count > MAX_TRUC_ANCESTOR_COUNT {
        return Err(TrucPolicyError::AncestorLimit {
            txid: entry.txid,
            count: ancestor_count,
        });
    }
    if !parents.is_empty() && entry.virtual_size.as_usize() > MAX_TRUC_CHILD_VIRTUAL_SIZE {
        return Err(TrucPolicyError::ChildTooLarge {
            txid: entry.txid,
            virtual_size: entry.virtual_size.as_usize(),
        });
    }

    let descendant_count = prospective_descendant_count(entry, candidates);
    if descendant_count > MAX_TRUC_DESCENDANT_COUNT {
        return Err(TrucPolicyError::DescendantLimit {
            txid: entry.txid,
            count: descendant_count,
        });
    }
    validate_parent_children(
        &parents,
        candidate_txids,
        direct_conflicts,
        maybe_sibling_eviction,
    )
}

fn validate_parent_children(
    parents: &[&MempoolEntry],
    candidate_txids: &BTreeSet<Txid>,
    direct_conflicts: &BTreeSet<Txid>,
    maybe_sibling_eviction: Option<EligibleSiblingEviction>,
) -> Result<(), TrucPolicyError> {
    for parent in parents {
        let retained_children = parent
            .children
            .iter()
            .filter(|txid| {
                !candidate_txids.contains(txid)
                    && !direct_conflicts.contains(txid)
                    && maybe_sibling_eviction.is_none_or(|intent| intent.sibling != **txid)
            })
            .count();
        if retained_children > 0 {
            return Err(TrucPolicyError::SiblingTopology {
                parent: parent.txid,
            });
        }
    }
    Ok(())
}

fn find_sibling_eviction(
    view: &impl MempoolView,
    members: &[PreparedCandidate],
    direct_conflicts: &BTreeSet<Txid>,
) -> Result<Option<EligibleSiblingEviction>, TrucPolicyError> {
    let candidates = members
        .iter()
        .map(|member| (member.entry.txid, &member.entry))
        .collect::<BTreeMap<_, _>>();
    let mut maybe_sibling = None;
    for member in members.iter().filter(|member| is_truc(&member.entry)) {
        for parent in direct_parents(view, &member.entry, &candidates) {
            for sibling_txid in &parent.children {
                if candidates.contains_key(sibling_txid) || direct_conflicts.contains(sibling_txid)
                {
                    continue;
                }
                let Some(sibling) = view.maybe_entry(sibling_txid) else {
                    return Err(TrucPolicyError::IneligibleSibling {
                        sibling: *sibling_txid,
                    });
                };
                if parent.children.len() != 1
                    || !is_truc(sibling)
                    || sibling.ancestor_stats.count != MAX_TRUC_ANCESTOR_COUNT
                    || sibling.descendant_stats.count != 1
                    || maybe_sibling.is_some()
                {
                    return Err(TrucPolicyError::IneligibleSibling {
                        sibling: *sibling_txid,
                    });
                }
                maybe_sibling = Some(EligibleSiblingEviction {
                    sibling: *sibling_txid,
                });
            }
        }
    }
    Ok(maybe_sibling)
}

fn direct_parents<'a>(
    view: &'a impl MempoolView,
    entry: &MempoolEntry,
    candidates: &'a BTreeMap<Txid, &MempoolEntry>,
) -> Vec<&'a MempoolEntry> {
    entry
        .transaction
        .inputs
        .iter()
        .filter_map(|input| {
            candidates
                .get(&input.previous_output.txid)
                .copied()
                .or_else(|| view.maybe_entry(&input.previous_output.txid))
        })
        .collect()
}

fn candidate_children<'a>(
    txid: Txid,
    candidates: &'a BTreeMap<Txid, &MempoolEntry>,
) -> Vec<&'a MempoolEntry> {
    candidates
        .values()
        .copied()
        .filter(|candidate| {
            candidate
                .transaction
                .inputs
                .iter()
                .any(|input| input.previous_output.txid == txid)
        })
        .collect()
}

fn prospective_ancestor_count(
    view: &impl MempoolView,
    entry: &MempoolEntry,
    candidates: &BTreeMap<Txid, &MempoolEntry>,
) -> usize {
    let mut ancestors = BTreeSet::new();
    let mut pending = direct_parents(view, entry, candidates);
    while let Some(parent) = pending.pop() {
        if !ancestors.insert(parent.txid) {
            continue;
        }
        pending.extend(direct_parents(view, parent, candidates));
    }
    ancestors.len().saturating_add(1)
}

fn prospective_descendant_count(
    entry: &MempoolEntry,
    candidates: &BTreeMap<Txid, &MempoolEntry>,
) -> usize {
    let mut descendants = BTreeSet::new();
    let mut pending = candidate_children(entry.txid, candidates);
    while let Some(child) = pending.pop() {
        if !descendants.insert(child.txid) {
            continue;
        }
        pending.extend(candidate_children(child.txid, candidates));
    }
    descendants.len().saturating_add(1)
}

fn is_truc(entry: &MempoolEntry) -> bool {
    entry.transaction.version == 3
}

#[cfg(test)]
mod tests;
