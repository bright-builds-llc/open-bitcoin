// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Test-only full-state oracle for sparse patch verification.

use std::collections::{BTreeSet, HashMap};

use open_bitcoin_primitives::{OutPoint, Txid};

use crate::{
    LimitDirection, LimitKind, MempoolEntry, MempoolError, MempoolResourceLedger, PolicyConfig,
    ResourceAccountingError, build_resource_ledger,
};

use super::topology::{collect_ancestors, collect_descendants};

#[derive(Debug, Clone)]
pub(super) struct MempoolState {
    pub(super) entries: HashMap<Txid, MempoolEntry>,
    #[allow(dead_code)] // Retained for index-coherence assertions.
    pub(super) spent_outpoints: HashMap<OutPoint, Txid>,
    pub(super) resource_ledger: MempoolResourceLedger,
}

pub(super) fn validate_limits(
    entries: &HashMap<Txid, MempoolEntry>,
    config: &PolicyConfig,
    candidate_txid: Txid,
) -> Result<(), MempoolError> {
    let Some(candidate_entry) = entries.get(&candidate_txid) else {
        return Err(MempoolError::InternalInvariant {
            reason: format!("candidate {candidate_txid:?} missing from prospective state"),
        });
    };
    validate_limit(
        candidate_entry.ancestor_stats,
        config.max_ancestor_count,
        config.max_ancestor_virtual_size,
        LimitDirection::Ancestor,
        None,
    )?;

    let mut candidate_ancestors = collect_ancestors(entries, candidate_txid);
    candidate_ancestors.insert(candidate_txid);
    for ancestor_txid in candidate_ancestors {
        let Some(entry) = entries.get(&ancestor_txid) else {
            return Err(MempoolError::InternalInvariant {
                reason: format!(
                    "ancestor {ancestor_txid:?} missing during descendant limit validation"
                ),
            });
        };
        validate_limit(
            entry.descendant_stats,
            config.max_descendant_count,
            config.max_descendant_virtual_size,
            LimitDirection::Descendant,
            Some(ancestor_txid),
        )?;
    }
    Ok(())
}

pub(super) fn recompute_state(
    mut entries: HashMap<Txid, MempoolEntry>,
) -> Result<MempoolState, ResourceAccountingError> {
    reset_entries(&mut entries);
    install_relations(&mut entries);
    let spent_outpoints = build_spent_index(&entries);
    install_aggregate_stats(&mut entries)?;
    let resource_ledger = build_resource_ledger(&entries, &spent_outpoints)?;
    Ok(MempoolState {
        entries,
        spent_outpoints,
        resource_ledger,
    })
}

fn validate_limit(
    stats: crate::AggregateStats,
    max_count: usize,
    max_virtual_size: usize,
    direction: LimitDirection,
    txid: Option<Txid>,
) -> Result<(), MempoolError> {
    if stats.count > max_count {
        return Err(MempoolError::LimitExceeded {
            direction,
            kind: LimitKind::Count,
            txid,
            attempted: stats.count,
            max: max_count,
        });
    }
    if stats.virtual_size.as_usize() > max_virtual_size {
        return Err(MempoolError::LimitExceeded {
            direction,
            kind: LimitKind::VirtualSize,
            txid,
            attempted: stats.virtual_size.as_usize(),
            max: max_virtual_size,
        });
    }
    Ok(())
}

fn reset_entries(entries: &mut HashMap<Txid, MempoolEntry>) {
    for entry in entries.values_mut() {
        entry.parents.clear();
        entry.children.clear();
        let stats = crate::AggregateStats::new(1, entry.virtual_size, entry.fee_sats());
        entry.ancestor_stats = stats;
        entry.descendant_stats = stats;
    }
}

fn install_relations(entries: &mut HashMap<Txid, MempoolEntry>) {
    let relations = entries
        .iter()
        .flat_map(|(txid, entry)| {
            entry.transaction.inputs.iter().filter_map(|input| {
                let parent = entries.get(&input.previous_output.txid)?;
                ((input.previous_output.vout as usize) < parent.transaction.outputs.len())
                    .then_some((input.previous_output.txid, *txid))
            })
        })
        .collect::<Vec<_>>();
    for (parent_txid, child_txid) in relations {
        if let Some(parent) = entries.get_mut(&parent_txid) {
            parent.children.insert(child_txid);
        }
        if let Some(child) = entries.get_mut(&child_txid) {
            child.parents.insert(parent_txid);
        }
    }
}

fn build_spent_index(entries: &HashMap<Txid, MempoolEntry>) -> HashMap<OutPoint, Txid> {
    entries
        .iter()
        .flat_map(|(txid, entry)| {
            entry
                .transaction
                .inputs
                .iter()
                .map(|input| (input.previous_output.clone(), *txid))
        })
        .collect()
}

fn install_aggregate_stats(
    entries: &mut HashMap<Txid, MempoolEntry>,
) -> Result<(), ResourceAccountingError> {
    let txids = entries.keys().copied().collect::<BTreeSet<_>>();
    let updates = txids
        .into_iter()
        .map(|txid| {
            let entry = &entries[&txid];
            let ancestors = collect_ancestors(entries, txid);
            let descendants = collect_descendants(entries, txid);
            Ok((
                txid,
                aggregate(
                    entries,
                    entry,
                    &ancestors,
                    "ancestor aggregate virtual size",
                )?,
                aggregate(
                    entries,
                    entry,
                    &descendants,
                    "descendant aggregate virtual size",
                )?,
            ))
        })
        .collect::<Result<Vec<_>, ResourceAccountingError>>()?;
    for (txid, ancestor_stats, descendant_stats) in updates {
        if let Some(entry) = entries.get_mut(&txid) {
            entry.ancestor_stats = ancestor_stats;
            entry.descendant_stats = descendant_stats;
        }
    }
    Ok(())
}

fn aggregate(
    entries: &HashMap<Txid, MempoolEntry>,
    entry: &MempoolEntry,
    related: &BTreeSet<Txid>,
    component: &'static str,
) -> Result<crate::AggregateStats, ResourceAccountingError> {
    let virtual_size = related
        .iter()
        .filter_map(|txid| entries.get(txid))
        .try_fold(entry.virtual_size, |total, related_entry| {
            total.checked_add(related_entry.virtual_size, component)
        })?;
    let total_fee_sats = entry.fee_sats()
        + related
            .iter()
            .filter_map(|txid| entries.get(txid))
            .map(MempoolEntry::fee_sats)
            .sum::<i64>();
    Ok(crate::AggregateStats::new(
        related.len().saturating_add(1),
        virtual_size,
        total_fee_sats,
    ))
}

#[cfg(test)]
#[path = "tests/oracle_internal_cases.rs"]
mod tests;
