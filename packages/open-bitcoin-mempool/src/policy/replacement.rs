// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/rbf.h
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/test/feefrac_tests.cpp
// - packages/bitcoin-knots/src/test/rbf_tests.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/util/feefrac.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_package_rbf.py

//! Pure policy for the bounded two-member package-replacement exception.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use open_bitcoin_primitives::{OutPoint, Txid};

use crate::pool::candidate::PreparedCandidate;
use crate::{
    IncrementalRelayFeeRate, MempoolEntry, MempoolMemberIdentity, MempoolRemovalRole,
    TransactionVirtualSize,
};

mod diagram;
#[cfg(test)]
mod tests;

pub(crate) const MAX_REPLACEMENT_CANDIDATES: usize = 100;

pub(crate) trait MempoolView {
    fn maybe_entry(&self, txid: &Txid) -> Option<&MempoolEntry>;
    fn maybe_spender(&self, outpoint: &OutPoint) -> Option<Txid>;
    fn collect_descendants(&self, txid: Txid) -> BTreeSet<Txid>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LimitedPackageReplacement {
    pub(crate) removals: BTreeMap<MempoolMemberIdentity, MempoolRemovalRole>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PackageReplacementError {
    WrongPackageSize {
        actual: usize,
    },
    WrongTopology,
    InMempoolAncestor {
        candidate: Txid,
        ancestor: Txid,
    },
    NoDirectConflicts,
    MissingConflict {
        txid: Txid,
    },
    InvalidDescendantCount {
        txid: Txid,
        count: usize,
    },
    PotentialCountOverflow,
    TooManyPotentialReplacements {
        count: usize,
        limit: usize,
    },
    RemovalEntryMissing {
        txid: Txid,
    },
    FeeOverflow,
    VirtualSizeOverflow,
    InvalidIncrementalRelayFee,
    InsufficientReplacementFee {
        replacement_fee_sats: i64,
        required_fee_sats: i64,
    },
    PackageFeeRateNotAboveParent,
    FeeRateDiagramNotImproved,
}

impl fmt::Display for PackageReplacementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongPackageSize { actual } => {
                write!(
                    formatter,
                    "package replacement requires exactly 2 members, got {actual}"
                )
            }
            Self::WrongTopology => {
                write!(
                    formatter,
                    "package replacement requires an exact parent-child package"
                )
            }
            Self::InMempoolAncestor {
                candidate,
                ancestor,
            } => write!(
                formatter,
                "package replacement candidate {candidate:?} has in-mempool ancestor {ancestor:?}"
            ),
            Self::NoDirectConflicts => {
                write!(
                    formatter,
                    "package replacement requires at least one direct conflict"
                )
            }
            Self::MissingConflict { txid } => {
                write!(
                    formatter,
                    "direct conflict {txid:?} is missing from the mempool view"
                )
            }
            Self::InvalidDescendantCount { txid, count } => write!(
                formatter,
                "direct conflict {txid:?} has invalid descendant count {count}"
            ),
            Self::PotentialCountOverflow => {
                write!(formatter, "potential replacement count overflow")
            }
            Self::TooManyPotentialReplacements { count, limit } => write!(
                formatter,
                "potential replacement count {count} exceeds limit {limit}"
            ),
            Self::RemovalEntryMissing { txid } => {
                write!(formatter, "replacement removal entry {txid:?} is missing")
            }
            Self::FeeOverflow => write!(formatter, "package replacement fee overflow"),
            Self::VirtualSizeOverflow => {
                write!(formatter, "package replacement virtual-size overflow")
            }
            Self::InvalidIncrementalRelayFee => {
                write!(formatter, "incremental relay fee must be non-negative")
            }
            Self::InsufficientReplacementFee {
                replacement_fee_sats,
                required_fee_sats,
            } => write!(
                formatter,
                "replacement modified fee {replacement_fee_sats} does not cover required fee {required_fee_sats}"
            ),
            Self::PackageFeeRateNotAboveParent => {
                write!(
                    formatter,
                    "replacement package feerate must be above its parent feerate"
                )
            }
            Self::FeeRateDiagramNotImproved => {
                write!(
                    formatter,
                    "replacement feerate diagram does not strictly improve"
                )
            }
        }
    }
}

impl std::error::Error for PackageReplacementError {}

pub(crate) fn evaluate_limited_package_replacement<V: MempoolView>(
    view: &V,
    package: &[PreparedCandidate],
    incremental_relay_fee_rate: IncrementalRelayFeeRate,
) -> Result<LimitedPackageReplacement, PackageReplacementError> {
    let [parent, child] = package else {
        return Err(PackageReplacementError::WrongPackageSize {
            actual: package.len(),
        });
    };
    if !child
        .entry
        .transaction
        .inputs
        .iter()
        .any(|input| input.previous_output.txid == parent.entry.txid)
    {
        return Err(PackageReplacementError::WrongTopology);
    }
    reject_in_mempool_ancestors(view, package)?;

    let direct_conflicts = direct_conflicts(view, package);
    if direct_conflicts.is_empty() {
        return Err(PackageReplacementError::NoDirectConflicts);
    }
    enforce_conservative_candidate_bound(view, &direct_conflicts)?;

    let removed_txids = collect_removal_union(view, &direct_conflicts);
    let removals = removal_facts(view, &direct_conflicts, &removed_txids)?;
    enforce_replacement_fees(view, package, &removed_txids, incremental_relay_fee_rate)?;
    enforce_parent_package_feerate(package)?;
    diagram::enforce_diagram_improvement(view, package, &direct_conflicts, &removed_txids)?;

    Ok(LimitedPackageReplacement { removals })
}

fn reject_in_mempool_ancestors(
    view: &impl MempoolView,
    package: &[PreparedCandidate],
) -> Result<(), PackageReplacementError> {
    for candidate in package {
        for input in &candidate.entry.transaction.inputs {
            if view.maybe_entry(&input.previous_output.txid).is_some() {
                return Err(PackageReplacementError::InMempoolAncestor {
                    candidate: candidate.entry.txid,
                    ancestor: input.previous_output.txid,
                });
            }
        }
    }
    Ok(())
}

fn direct_conflicts(view: &impl MempoolView, package: &[PreparedCandidate]) -> BTreeSet<Txid> {
    package
        .iter()
        .flat_map(|candidate| &candidate.entry.transaction.inputs)
        .filter_map(|input| view.maybe_spender(&input.previous_output))
        .collect()
}

fn enforce_conservative_candidate_bound(
    view: &impl MempoolView,
    direct_conflicts: &BTreeSet<Txid>,
) -> Result<(), PackageReplacementError> {
    let mut potential_count = 0_usize;
    for txid in direct_conflicts {
        let Some(entry) = view.maybe_entry(txid) else {
            return Err(PackageReplacementError::MissingConflict { txid: *txid });
        };
        let descendant_count = entry.descendant_stats.count;
        if descendant_count == 0 {
            return Err(PackageReplacementError::InvalidDescendantCount {
                txid: *txid,
                count: descendant_count,
            });
        }
        potential_count = potential_count
            .checked_add(descendant_count)
            .ok_or(PackageReplacementError::PotentialCountOverflow)?;
        if potential_count > MAX_REPLACEMENT_CANDIDATES {
            return Err(PackageReplacementError::TooManyPotentialReplacements {
                count: potential_count,
                limit: MAX_REPLACEMENT_CANDIDATES,
            });
        }
    }
    Ok(())
}

fn collect_removal_union(
    view: &impl MempoolView,
    direct_conflicts: &BTreeSet<Txid>,
) -> BTreeSet<Txid> {
    let mut removed = BTreeSet::new();
    for txid in direct_conflicts {
        removed.insert(*txid);
        removed.extend(view.collect_descendants(*txid));
    }
    removed
}

fn removal_facts(
    view: &impl MempoolView,
    direct_conflicts: &BTreeSet<Txid>,
    removed_txids: &BTreeSet<Txid>,
) -> Result<BTreeMap<MempoolMemberIdentity, MempoolRemovalRole>, PackageReplacementError> {
    removed_txids
        .iter()
        .map(|txid| {
            let entry = view
                .maybe_entry(txid)
                .ok_or(PackageReplacementError::RemovalEntryMissing { txid: *txid })?;
            let role = if direct_conflicts.contains(txid) {
                MempoolRemovalRole::Direct
            } else {
                MempoolRemovalRole::Descendant
            };
            Ok((
                MempoolMemberIdentity {
                    txid: *txid,
                    wtxid: entry.wtxid,
                },
                role,
            ))
        })
        .collect()
}

fn enforce_replacement_fees(
    view: &impl MempoolView,
    package: &[PreparedCandidate],
    removed_txids: &BTreeSet<Txid>,
    incremental_relay_fee_rate: IncrementalRelayFeeRate,
) -> Result<(), PackageReplacementError> {
    let original_fee = removed_txids.iter().try_fold(0_i64, |total, txid| {
        let entry = view
            .maybe_entry(txid)
            .ok_or(PackageReplacementError::RemovalEntryMissing { txid: *txid })?;
        total
            .checked_add(entry.fee_sats())
            .ok_or(PackageReplacementError::FeeOverflow)
    })?;
    let replacement_fee = package.iter().try_fold(0_i64, |total, candidate| {
        total
            .checked_add(candidate.fees.modified.to_sats())
            .ok_or(PackageReplacementError::FeeOverflow)
    })?;
    let replacement_vsize = package_virtual_size(package)?;
    let incremental_rate = incremental_relay_fee_rate.fee_rate().sats_per_kvb();
    if incremental_rate < 0 {
        return Err(PackageReplacementError::InvalidIncrementalRelayFee);
    }
    let incremental_fee = checked_fee_for_virtual_size(incremental_rate, replacement_vsize)?;
    let required_fee = original_fee
        .checked_add(incremental_fee)
        .ok_or(PackageReplacementError::FeeOverflow)?;
    if replacement_fee < required_fee {
        return Err(PackageReplacementError::InsufficientReplacementFee {
            replacement_fee_sats: replacement_fee,
            required_fee_sats: required_fee,
        });
    }
    Ok(())
}

fn checked_fee_for_virtual_size(
    sats_per_kvb: i64,
    virtual_size: TransactionVirtualSize,
) -> Result<i64, PackageReplacementError> {
    let product = i128::from(sats_per_kvb)
        .checked_mul(
            i128::try_from(virtual_size.as_usize())
                .map_err(|_| PackageReplacementError::FeeOverflow)?,
        )
        .ok_or(PackageReplacementError::FeeOverflow)?;
    let rounded = product
        .checked_add(999)
        .ok_or(PackageReplacementError::FeeOverflow)?
        / 1_000;
    i64::try_from(rounded).map_err(|_| PackageReplacementError::FeeOverflow)
}

fn package_virtual_size(
    package: &[PreparedCandidate],
) -> Result<TransactionVirtualSize, PackageReplacementError> {
    package
        .iter()
        .try_fold(TransactionVirtualSize::ZERO, |total, candidate| {
            total
                .checked_add(
                    candidate.entry.virtual_size,
                    "package replacement virtual size",
                )
                .map_err(|_| PackageReplacementError::VirtualSizeOverflow)
        })
}

fn enforce_parent_package_feerate(
    package: &[PreparedCandidate],
) -> Result<(), PackageReplacementError> {
    let parent = &package[0];
    let package_fee = package.iter().try_fold(0_i64, |total, candidate| {
        total
            .checked_add(candidate.fees.modified.to_sats())
            .ok_or(PackageReplacementError::FeeOverflow)
    })?;
    let package_vsize = package_virtual_size(package)?.as_usize();
    if !rate_is_greater(
        package_fee,
        package_vsize,
        parent.fees.modified.to_sats(),
        parent.entry.virtual_size.as_usize(),
    ) {
        return Err(PackageReplacementError::PackageFeeRateNotAboveParent);
    }
    Ok(())
}

fn rate_is_greater(left_fee: i64, left_size: usize, right_fee: i64, right_size: usize) -> bool {
    diagram::rate_cmp(left_fee, left_size, right_fee, right_size).is_gt()
}
