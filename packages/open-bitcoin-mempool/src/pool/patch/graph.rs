// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Prospective topology, aggregate, limit, and eviction calculations.

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::Txid;

use crate::{
    AggregateStats, FeeRate, LimitDirection, LimitKind, MempoolEntry, MempoolError, PolicyConfig,
};

use super::super::{Mempool, TopologyUpdate, resource_invariant_error};

pub(super) struct ProspectiveGraph {
    pub(super) updates: BTreeMap<Txid, TopologyUpdate>,
    pub(super) self_rates: BTreeMap<Txid, FeeRate>,
}

impl ProspectiveGraph {
    pub(super) fn build(
        mempool: &Mempool,
        upserts: &BTreeMap<Txid, MempoolEntry>,
        removals: &BTreeSet<Txid>,
    ) -> Result<Self, MempoolError> {
        let txids = mempool
            .entries
            .keys()
            .filter(|txid| !removals.contains(txid))
            .copied()
            .chain(
                upserts
                    .keys()
                    .filter(|txid| !removals.contains(txid))
                    .copied(),
            )
            .collect::<BTreeSet<_>>();
        let mut parents = txids
            .iter()
            .map(|txid| (*txid, BTreeSet::new()))
            .collect::<BTreeMap<_, _>>();
        let mut children = parents.clone();
        for txid in &txids {
            let entry = existing_entry(mempool, upserts, *txid);
            for input in &entry.transaction.inputs {
                let parent_txid = input.previous_output.txid;
                if !txids.contains(&parent_txid) {
                    continue;
                }
                let parent_entry = existing_entry(mempool, upserts, parent_txid);
                if input.previous_output.vout as usize >= parent_entry.transaction.outputs.len() {
                    continue;
                }
                parents.entry(*txid).or_default().insert(parent_txid);
                children.entry(parent_txid).or_default().insert(*txid);
            }
        }

        let mut updates = BTreeMap::new();
        let mut self_rates = BTreeMap::new();
        for txid in txids {
            let entry = existing_entry(mempool, upserts, txid);
            self_rates.insert(txid, entry.fee_rate());
            let ancestors = closure(&parents, txid);
            let descendants = closure(&children, txid);
            let ancestor_stats = aggregate_stats(mempool, upserts, txid, &ancestors)?;
            let descendant_stats = aggregate_stats(mempool, upserts, txid, &descendants)?;
            updates.insert(
                txid,
                TopologyUpdate {
                    parents: parents.remove(&txid).unwrap_or_default(),
                    children: children.remove(&txid).unwrap_or_default(),
                    ancestor_stats,
                    descendant_stats,
                },
            );
        }
        Ok(Self {
            updates,
            self_rates,
        })
    }

    pub(super) fn validate_limits(
        &self,
        config: &PolicyConfig,
        candidate: Txid,
    ) -> Result<(), MempoolError> {
        let Some(candidate_update) = self.updates.get(&candidate) else {
            return Err(MempoolError::InternalInvariant {
                reason: format!("candidate {candidate:?} missing from prospective graph"),
            });
        };
        validate_stat_limit(
            candidate_update.ancestor_stats,
            config.max_ancestor_count,
            config.max_ancestor_virtual_size,
            LimitDirection::Ancestor,
            None,
        )?;
        let mut ancestors = closure_from_updates(&self.updates, candidate, true);
        ancestors.insert(candidate);
        for ancestor in ancestors {
            let update = &self.updates[&ancestor];
            validate_stat_limit(
                update.descendant_stats,
                config.max_descendant_count,
                config.max_descendant_virtual_size,
                LimitDirection::Descendant,
                Some(ancestor),
            )?;
        }
        Ok(())
    }

    pub(super) fn descendants(&self, txid: Txid) -> BTreeSet<Txid> {
        closure_from_updates(&self.updates, txid, false)
    }

    pub(super) fn eviction_package(&self) -> Result<(Txid, FeeRate), MempoolError> {
        self.updates
            .iter()
            .min_by(|(left_txid, left), (right_txid, right)| {
                self.descendant_score(**left_txid, left)
                    .cmp(&self.descendant_score(**right_txid, right))
                    .then_with(|| left_txid.cmp(right_txid))
            })
            .map(|(txid, update)| {
                (
                    *txid,
                    FeeRate::from_fee_sats_and_vbytes(
                        update.descendant_stats.total_fee_sats,
                        update.descendant_stats.virtual_size,
                    ),
                )
            })
            .ok_or_else(|| MempoolError::InternalInvariant {
                reason: "over-capacity prospective graph has no eviction candidate".to_string(),
            })
    }

    fn descendant_score(&self, txid: Txid, update: &TopologyUpdate) -> FeeRate {
        let descendant_rate = update_descendant_score(update);
        self.self_rates
            .get(&txid)
            .copied()
            .map_or(descendant_rate, |self_rate| self_rate.max(descendant_rate))
    }
}

pub(super) fn entry_for<'a>(
    mempool: &'a Mempool,
    upserts: &'a BTreeMap<Txid, MempoolEntry>,
    txid: Txid,
) -> Option<&'a MempoolEntry> {
    upserts.get(&txid).or_else(|| mempool.entries.get(&txid))
}

pub(super) fn existing_entry<'a>(
    mempool: &'a Mempool,
    upserts: &'a BTreeMap<Txid, MempoolEntry>,
    txid: Txid,
) -> &'a MempoolEntry {
    if let Some(entry) = upserts.get(&txid) {
        return entry;
    }
    &mempool.entries[&txid]
}

pub(super) fn closure(edges: &BTreeMap<Txid, BTreeSet<Txid>>, start: Txid) -> BTreeSet<Txid> {
    let mut found = BTreeSet::new();
    let mut pending = edges
        .get(&start)
        .into_iter()
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    while let Some(txid) = pending.pop() {
        if !found.insert(txid) {
            continue;
        }
        pending.extend(edges.get(&txid).into_iter().flatten().copied());
    }
    found
}

fn closure_from_updates(
    updates: &BTreeMap<Txid, TopologyUpdate>,
    start: Txid,
    use_parents: bool,
) -> BTreeSet<Txid> {
    let edges = updates
        .iter()
        .map(|(txid, update)| {
            (
                *txid,
                if use_parents {
                    update.parents.clone()
                } else {
                    update.children.clone()
                },
            )
        })
        .collect();
    closure(&edges, start)
}

pub(super) fn aggregate_stats(
    mempool: &Mempool,
    upserts: &BTreeMap<Txid, MempoolEntry>,
    txid: Txid,
    related: &BTreeSet<Txid>,
) -> Result<AggregateStats, MempoolError> {
    let Some(entry) = entry_for(mempool, upserts, txid) else {
        return Err(MempoolError::InternalInvariant {
            reason: format!("prospective entry {txid:?} is missing"),
        });
    };
    let mut virtual_size = entry.virtual_size;
    let mut total_fee_sats = entry.fee_sats();
    for related_txid in related {
        if let Some(related_entry) = entry_for(mempool, upserts, *related_txid) {
            virtual_size = virtual_size
                .checked_add(
                    related_entry.virtual_size,
                    "prospective aggregate virtual size",
                )
                .map_err(resource_invariant_error)?;
            total_fee_sats = checked_fee_sum(total_fee_sats, related_entry.fee_sats())?;
        }
    }
    let count = checked_count(related.len(), "prospective aggregate count overflow")?;
    Ok(AggregateStats::new(count, virtual_size, total_fee_sats))
}

pub(super) fn checked_fee_sum(left: i64, right: i64) -> Result<i64, MempoolError> {
    left.checked_add(right)
        .ok_or_else(|| MempoolError::InternalInvariant {
            reason: "prospective aggregate fee overflow".to_string(),
        })
}

pub(super) fn checked_count(count: usize, reason: &'static str) -> Result<usize, MempoolError> {
    count
        .checked_add(1)
        .ok_or_else(|| MempoolError::InternalInvariant {
            reason: reason.to_string(),
        })
}

pub(super) fn validate_stat_limit(
    stats: AggregateStats,
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

fn update_descendant_score(update: &TopologyUpdate) -> FeeRate {
    FeeRate::from_fee_sats_and_vbytes(
        update.descendant_stats.total_fee_sats,
        update.descendant_stats.virtual_size,
    )
}
