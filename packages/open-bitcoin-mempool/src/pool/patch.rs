// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Sparse prospective topology and revision-bound mempool patches.

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::Txid;

use crate::fee::rolling::RollingFeeState;
use crate::{
    FeeRate, MempoolEntry, MempoolError, MempoolLifecycleDelta, MempoolMemberIdentity,
    MempoolRemovalRole, MempoolResourceLedger,
};

use super::{
    Mempool, MempoolPatch, MempoolResourceDelta, TopologyUpdate, resource_invariant_error,
};

mod graph;

use self::graph::{ProspectiveGraph, existing_entry};
#[cfg(test)]
use self::graph::{aggregate_stats, checked_count, checked_fee_sum, closure, validate_stat_limit};

pub(super) fn prepare_admission_layout(
    mempool: &Mempool,
    candidate: &MempoolEntry,
    mut removals: BTreeSet<Txid>,
) -> Result<
    (
        SparseLayout,
        BTreeMap<MempoolMemberIdentity, MempoolRemovalRole>,
        RollingFeeState,
    ),
    MempoolError,
> {
    let mut upserts = BTreeMap::from([(candidate.txid, candidate.clone())]);
    let initial = SparseLayout::build(mempool, upserts.clone(), removals.clone())?;
    initial
        .graph
        .validate_limits(&mempool.config, candidate.txid)?;

    let mut rolling_fee_state = mempool.rolling_fee_state.clone();
    let mut evicted = BTreeMap::new();
    loop {
        let layout = SparseLayout::build(mempool, upserts, removals.clone())?;
        if layout
            .resource_delta
            .next_ledger
            .accounted_memory()
            .as_usize()
            <= mempool.config.mempool_capacity.as_usize()
        {
            return Ok((layout, evicted, rolling_fee_state));
        }

        let (victim, package_rate) = layout.graph.eviction_package()?;
        let package_plus_incremental = FeeRate::from_sats_per_kvb(
            package_rate.sats_per_kvb().saturating_add(
                mempool
                    .config
                    .incremental_relay_fee_rate
                    .fee_rate()
                    .sats_per_kvb(),
            ),
        );
        rolling_fee_state.track_package_removed(package_plus_incremental);

        let mut package = layout.graph.descendants(victim);
        package.insert(victim);
        for txid in &package {
            let entry = existing_entry(mempool, &layout.entry_upserts, *txid);
            evicted.insert(
                MempoolMemberIdentity {
                    txid: *txid,
                    wtxid: entry.wtxid,
                },
                if *txid == victim {
                    MempoolRemovalRole::Direct
                } else {
                    MempoolRemovalRole::Descendant
                },
            );
        }
        removals.extend(package);
        upserts = layout.entry_upserts;
    }
}

pub(super) fn prepare_removal_patch(
    mempool: &Mempool,
    removals: BTreeSet<Txid>,
    rolling_fee_state: RollingFeeState,
    delta: MempoolLifecycleDelta,
) -> Result<MempoolPatch, MempoolError> {
    SparseLayout::build(mempool, BTreeMap::new(), removals)?.into_patch(
        mempool,
        rolling_fee_state,
        delta,
    )
}

pub(super) struct SparseLayout {
    entry_upserts: BTreeMap<Txid, MempoolEntry>,
    entry_removals: BTreeSet<Txid>,
    spent_updates: BTreeMap<open_bitcoin_primitives::OutPoint, Option<Txid>>,
    topology_updates: BTreeMap<Txid, TopologyUpdate>,
    resource_delta: MempoolResourceDelta,
    graph: ProspectiveGraph,
}

impl SparseLayout {
    fn build(
        mempool: &Mempool,
        mut entry_upserts: BTreeMap<Txid, MempoolEntry>,
        entry_removals: BTreeSet<Txid>,
    ) -> Result<Self, MempoolError> {
        let graph = ProspectiveGraph::build(mempool, &entry_upserts, &entry_removals)?;
        let mut topology_updates = BTreeMap::new();
        for (txid, update) in &graph.updates {
            if let Some(upsert) = entry_upserts.get_mut(txid) {
                apply_topology(upsert, update);
                continue;
            }
            if let Some(existing) = mempool.entries.get(txid)
                && topology_differs(existing, update)
            {
                topology_updates.insert(*txid, update.clone());
            }
        }

        let mut spent_updates = BTreeMap::new();
        for (outpoint, spender) in &mempool.spent_outpoints {
            if entry_removals.contains(spender) {
                spent_updates.insert(outpoint.clone(), None);
            }
        }
        for (txid, entry) in &entry_upserts {
            for input in &entry.transaction.inputs {
                spent_updates.insert(input.previous_output.clone(), Some(*txid));
            }
        }

        let next_ledger =
            build_prospective_ledger(mempool, &entry_upserts, &entry_removals, &topology_updates)?;
        Ok(Self {
            entry_upserts,
            entry_removals,
            spent_updates,
            topology_updates,
            resource_delta: MempoolResourceDelta { next_ledger },
            graph,
        })
    }

    pub(super) fn contains(&self, txid: Txid) -> bool {
        self.graph.updates.contains_key(&txid)
    }

    pub(super) fn into_patch(
        self,
        mempool: &Mempool,
        rolling_fee_state: RollingFeeState,
        delta: MempoolLifecycleDelta,
    ) -> Result<MempoolPatch, MempoolError> {
        let next_revision = mempool.revision.next()?;
        Ok(MempoolPatch {
            base_revision: mempool.revision,
            next_revision,
            entry_upserts: self.entry_upserts,
            entry_removals: self.entry_removals,
            spent_updates: self.spent_updates,
            topology_updates: self.topology_updates,
            resource_delta: self.resource_delta,
            rolling_fee_state,
            delta,
        })
    }
}

fn build_prospective_ledger(
    mempool: &Mempool,
    upserts: &BTreeMap<Txid, MempoolEntry>,
    removals: &BTreeSet<Txid>,
    topology_updates: &BTreeMap<Txid, TopologyUpdate>,
) -> Result<MempoolResourceLedger, MempoolError> {
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
    let spent_count = txids
        .iter()
        .flat_map(|txid| {
            existing_entry(mempool, upserts, *txid)
                .transaction
                .inputs
                .iter()
        })
        .count();
    let mut ledger = MempoolResourceLedger::ZERO;
    for txid in txids {
        let entry = existing_entry(mempool, upserts, txid);
        if let Some(update) = topology_updates.get(&txid) {
            let mut updated = entry.clone();
            apply_topology(&mut updated, update);
            ledger
                .checked_add_entry(&updated)
                .map_err(resource_invariant_error)?;
        } else {
            ledger
                .checked_add_entry(entry)
                .map_err(resource_invariant_error)?;
        }
    }
    ledger
        .checked_add_spent_outpoints(spent_count)
        .map_err(resource_invariant_error)?;
    Ok(ledger)
}

fn apply_topology(entry: &mut MempoolEntry, update: &TopologyUpdate) {
    entry.parents.clone_from(&update.parents);
    entry.children.clone_from(&update.children);
    entry.ancestor_stats = update.ancestor_stats;
    entry.descendant_stats = update.descendant_stats;
}

fn topology_differs(entry: &MempoolEntry, update: &TopologyUpdate) -> bool {
    entry.parents != update.parents
        || entry.children != update.children
        || entry.ancestor_stats != update.ancestor_stats
        || entry.descendant_stats != update.descendant_stats
}

#[cfg(test)]
#[path = "tests/patch_internal_cases.rs"]
mod tests;
