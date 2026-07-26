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

//! Exact feerate-chunk diagram construction and partial-order comparison.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use open_bitcoin_primitives::Txid;

use super::{MempoolView, PackageReplacementError, package_virtual_size};
use crate::MempoolEntry;
use crate::pool::candidate::PreparedCandidate;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FeeChunk {
    fee_sats: i64,
    virtual_size: usize,
}

pub(super) fn enforce_diagram_improvement(
    view: &impl MempoolView,
    package: &[PreparedCandidate],
    direct_conflicts: &BTreeSet<Txid>,
    removed_txids: &BTreeSet<Txid>,
) -> Result<(), PackageReplacementError> {
    let old_chunks = old_fee_chunks(view, removed_txids)?;
    let mut new_chunks = Vec::new();
    let mut retained_parents = BTreeSet::new();
    for conflict in direct_conflicts {
        let entry = view
            .maybe_entry(conflict)
            .ok_or(PackageReplacementError::MissingConflict { txid: *conflict })?;
        retained_parents.extend(
            entry
                .parents
                .iter()
                .filter(|parent| !removed_txids.contains(parent))
                .copied(),
        );
    }
    for parent in retained_parents {
        let entry = view
            .maybe_entry(&parent)
            .ok_or(PackageReplacementError::RemovalEntryMissing { txid: parent })?;
        new_chunks.push(entry_chunk(entry));
    }
    new_chunks.push(FeeChunk {
        fee_sats: package.iter().try_fold(0_i64, |total, candidate| {
            total
                .checked_add(candidate.fees.modified.to_sats())
                .ok_or(PackageReplacementError::FeeOverflow)
        })?,
        virtual_size: package_virtual_size(package)?.as_usize(),
    });
    sort_chunks(&mut new_chunks);

    if !diagram_strictly_improves(&old_chunks, &new_chunks)? {
        return Err(PackageReplacementError::FeeRateDiagramNotImproved);
    }
    Ok(())
}

fn old_fee_chunks(
    view: &impl MempoolView,
    removed_txids: &BTreeSet<Txid>,
) -> Result<Vec<FeeChunk>, PackageReplacementError> {
    let mut chunks = Vec::new();
    for txid in removed_txids {
        let entry = view
            .maybe_entry(txid)
            .ok_or(PackageReplacementError::RemovalEntryMissing { txid: *txid })?;
        if entry
            .children
            .iter()
            .any(|child| removed_txids.contains(child))
        {
            continue;
        }
        let ancestors = removed_ancestors(view, entry, removed_txids)?;
        if ancestors.is_empty() {
            chunks.push(entry_chunk(entry));
            continue;
        }
        let ancestor_chunk = aggregate_entries(view, &ancestors)?;
        let package_chunk = combine_chunks(ancestor_chunk, entry_chunk(entry))?;
        if rate_cmp(
            entry.fee_sats(),
            entry.virtual_size.as_usize(),
            package_chunk.fee_sats,
            package_chunk.virtual_size,
        )
        .is_gt()
        {
            chunks.push(package_chunk);
        } else {
            chunks.push(ancestor_chunk);
            chunks.push(entry_chunk(entry));
        }
    }
    sort_chunks(&mut chunks);
    Ok(chunks)
}

fn removed_ancestors(
    view: &impl MempoolView,
    entry: &MempoolEntry,
    removed_txids: &BTreeSet<Txid>,
) -> Result<BTreeSet<Txid>, PackageReplacementError> {
    let mut ancestors = BTreeSet::new();
    let mut pending = entry.parents.iter().copied().collect::<Vec<_>>();
    while let Some(txid) = pending.pop() {
        if !removed_txids.contains(&txid) || !ancestors.insert(txid) {
            continue;
        }
        let ancestor = view
            .maybe_entry(&txid)
            .ok_or(PackageReplacementError::RemovalEntryMissing { txid })?;
        pending.extend(ancestor.parents.iter().copied());
    }
    Ok(ancestors)
}

fn aggregate_entries(
    view: &impl MempoolView,
    txids: &BTreeSet<Txid>,
) -> Result<FeeChunk, PackageReplacementError> {
    txids.iter().try_fold(
        FeeChunk {
            fee_sats: 0,
            virtual_size: 0,
        },
        |chunk, txid| {
            let entry = view
                .maybe_entry(txid)
                .ok_or(PackageReplacementError::RemovalEntryMissing { txid: *txid })?;
            combine_chunks(chunk, entry_chunk(entry))
        },
    )
}

fn entry_chunk(entry: &MempoolEntry) -> FeeChunk {
    FeeChunk {
        fee_sats: entry.fee_sats(),
        virtual_size: entry.virtual_size.as_usize(),
    }
}

fn combine_chunks(left: FeeChunk, right: FeeChunk) -> Result<FeeChunk, PackageReplacementError> {
    Ok(FeeChunk {
        fee_sats: left
            .fee_sats
            .checked_add(right.fee_sats)
            .ok_or(PackageReplacementError::FeeOverflow)?,
        virtual_size: left
            .virtual_size
            .checked_add(right.virtual_size)
            .ok_or(PackageReplacementError::VirtualSizeOverflow)?,
    })
}

fn sort_chunks(chunks: &mut [FeeChunk]) {
    chunks.sort_by(|left, right| {
        rate_cmp(
            right.fee_sats,
            right.virtual_size,
            left.fee_sats,
            left.virtual_size,
        )
        .then_with(|| left.virtual_size.cmp(&right.virtual_size))
    });
}

pub(super) fn rate_cmp(
    left_fee: i64,
    left_size: usize,
    right_fee: i64,
    right_size: usize,
) -> Ordering {
    let left = i128::from(left_fee) * right_size as i128;
    let right = i128::from(right_fee) * left_size as i128;
    left.cmp(&right)
}

fn diagram_strictly_improves(
    old_chunks: &[FeeChunk],
    new_chunks: &[FeeChunk],
) -> Result<bool, PackageReplacementError> {
    let old_total_size = total_chunk_size(old_chunks)?;
    let new_total_size = total_chunk_size(new_chunks)?;
    if new_total_size < old_total_size {
        return Ok(false);
    }
    let mut breakpoints = BTreeSet::new();
    add_breakpoints(&mut breakpoints, old_chunks, old_total_size)?;
    add_breakpoints(&mut breakpoints, new_chunks, old_total_size)?;

    let mut strictly_better = new_total_size > old_total_size;
    for virtual_size in breakpoints {
        let old_fee = fee_at_virtual_size(old_chunks, virtual_size)?;
        let new_fee = fee_at_virtual_size(new_chunks, virtual_size)?;
        match compare_fraction(new_fee, old_fee)? {
            Ordering::Less => return Ok(false),
            Ordering::Greater => strictly_better = true,
            Ordering::Equal => {}
        }
    }
    Ok(strictly_better)
}

fn total_chunk_size(chunks: &[FeeChunk]) -> Result<usize, PackageReplacementError> {
    chunks.iter().try_fold(0_usize, |total, chunk| {
        total
            .checked_add(chunk.virtual_size)
            .ok_or(PackageReplacementError::VirtualSizeOverflow)
    })
}

fn add_breakpoints(
    breakpoints: &mut BTreeSet<usize>,
    chunks: &[FeeChunk],
    maximum: usize,
) -> Result<(), PackageReplacementError> {
    let mut cumulative = 0_usize;
    for chunk in chunks {
        cumulative = cumulative
            .checked_add(chunk.virtual_size)
            .ok_or(PackageReplacementError::VirtualSizeOverflow)?;
        if cumulative <= maximum {
            breakpoints.insert(cumulative);
        }
    }
    breakpoints.insert(maximum);
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fraction {
    numerator: i128,
    denominator: i128,
}

fn fee_at_virtual_size(
    chunks: &[FeeChunk],
    target: usize,
) -> Result<Fraction, PackageReplacementError> {
    let mut full_fee = 0_i128;
    let mut consumed = 0_usize;
    for chunk in chunks {
        let next = consumed
            .checked_add(chunk.virtual_size)
            .ok_or(PackageReplacementError::VirtualSizeOverflow)?;
        if target <= next {
            let partial = target.saturating_sub(consumed);
            let denominator = chunk.virtual_size as i128;
            let numerator = full_fee
                .checked_mul(denominator)
                .and_then(|fee| {
                    i128::from(chunk.fee_sats)
                        .checked_mul(partial as i128)
                        .and_then(|partial_fee| fee.checked_add(partial_fee))
                })
                .ok_or(PackageReplacementError::FeeOverflow)?;
            return Ok(Fraction {
                numerator,
                denominator,
            });
        }
        full_fee += i128::from(chunk.fee_sats);
        consumed = next;
    }
    Ok(Fraction {
        numerator: full_fee,
        denominator: 1,
    })
}

fn compare_fraction(left: Fraction, right: Fraction) -> Result<Ordering, PackageReplacementError> {
    let left_scaled = left
        .numerator
        .checked_mul(right.denominator)
        .ok_or(PackageReplacementError::FeeOverflow)?;
    let right_scaled = right
        .numerator
        .checked_mul(left.denominator)
        .ok_or(PackageReplacementError::FeeOverflow)?;
    Ok(left_scaled.cmp(&right_scaled))
}
