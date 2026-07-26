// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Sparse overlay used to prepare one revision-bound mempool transition.

mod limits;

use std::collections::{BTreeMap, BTreeSet, HashMap};

use open_bitcoin_primitives::{OutPoint, Txid, Wtxid};

use crate::fee::rolling::RollingFeeState;
use crate::{
    AggregateStats, MempoolEntry, MempoolError, MempoolLifecycleDelta, MempoolMemberIdentity,
    MempoolRemovalCause, MempoolRemovalRole, MempoolResourceLedger,
};

use super::candidate::PreparedCandidate;
use super::lifecycle::MempoolRemovalFact;
use super::{
    Mempool, MempoolPatch, MempoolResourceDelta, TopologyUpdate, resource_invariant_error,
};

/// One checked sparse mutation that can be composed into a prospective view.
#[derive(Debug, Clone)]
pub(super) struct SubDelta {
    entry_upserts: BTreeMap<Txid, MempoolEntry>,
    removals: BTreeMap<MempoolMemberIdentity, MempoolRemovalFact>,
}

impl SubDelta {
    pub(super) fn from_entries(
        entries: impl IntoIterator<Item = MempoolEntry>,
    ) -> Result<Self, MempoolError> {
        let mut entry_upserts = BTreeMap::new();
        let mut identities_by_wtxid = BTreeMap::<Wtxid, Txid>::new();
        for entry in entries {
            if entry_upserts.contains_key(&entry.txid) {
                return Err(invariant("sub-delta contains a duplicate txid"));
            }
            if identities_by_wtxid
                .insert(entry.wtxid, entry.txid)
                .is_some()
            {
                return Err(invariant("sub-delta contains a duplicate wtxid"));
            }
            entry_upserts.insert(entry.txid, entry);
        }
        Ok(Self {
            entry_upserts,
            removals: BTreeMap::new(),
        })
    }

    pub(super) fn removals(removals: BTreeMap<MempoolMemberIdentity, MempoolRemovalFact>) -> Self {
        Self {
            entry_upserts: BTreeMap::new(),
            removals,
        }
    }
}

/// Overlay-first mempool view that owns only facts touched by staged work.
#[derive(Clone)]
pub(super) struct ProspectiveMempool<'base> {
    base: &'base Mempool,
    added_or_updated: HashMap<Txid, MempoolEntry>,
    removed: BTreeMap<MempoolMemberIdentity, MempoolRemovalFact>,
    spent_updates: HashMap<OutPoint, Option<Txid>>,
    topology_updates: BTreeMap<Txid, TopologyUpdate>,
    resource_delta: MempoolResourceDelta,
    prospective_resource_usage: MempoolResourceLedger,
    rolling_fee_state: RollingFeeState,
    #[cfg(test)]
    trim_invocations: usize,
}

impl<'base> ProspectiveMempool<'base> {
    pub(super) fn new(base: &'base Mempool) -> Self {
        Self {
            base,
            added_or_updated: HashMap::new(),
            removed: BTreeMap::new(),
            spent_updates: HashMap::new(),
            topology_updates: BTreeMap::new(),
            resource_delta: MempoolResourceDelta {
                next_ledger: base.resource_ledger,
            },
            prospective_resource_usage: base.resource_ledger,
            rolling_fee_state: base.rolling_fee_state.clone(),
            #[cfg(test)]
            trim_invocations: 0,
        }
    }

    pub(super) fn maybe_entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        if self.is_removed_txid(*txid) {
            return None;
        }
        self.added_or_updated
            .get(txid)
            .or_else(|| self.base.entries.get(txid))
    }

    pub(super) fn maybe_spender(&self, outpoint: &OutPoint) -> Option<Txid> {
        if let Some(maybe_spender) = self.spent_updates.get(outpoint) {
            return *maybe_spender;
        }
        self.base
            .spent_outpoints
            .get(outpoint)
            .copied()
            .filter(|txid| !self.is_removed_txid(*txid))
    }

    #[allow(dead_code)] // Consumed by the package orchestration plan after this infrastructure lands.
    pub(super) fn stage_candidate(
        &mut self,
        candidate: PreparedCandidate,
    ) -> Result<SubDelta, MempoolError> {
        let sub_delta = SubDelta::from_entries([candidate.entry])?;
        self.compose(sub_delta.clone())?;
        Ok(sub_delta)
    }

    pub(super) fn compose(&mut self, sub_delta: SubDelta) -> Result<(), MempoolError> {
        let mut working = self.clone();
        working.apply_sub_delta(sub_delta)?;
        *self = working;
        Ok(())
    }

    pub(super) fn stage_descendant_package_removal(
        &mut self,
        victim: Txid,
        cause: MempoolRemovalCause,
    ) -> Result<Vec<MempoolMemberIdentity>, MempoolError> {
        let Some(victim_entry) = self.maybe_entry(&victim) else {
            return Err(invariant("prospective removal victim is missing"));
        };
        let mut txids = self.collect_descendants(victim);
        txids.insert(victim);
        let mut removals = BTreeMap::new();
        let mut members = Vec::with_capacity(txids.len());
        for txid in txids {
            let Some(entry) = self.maybe_entry(&txid) else {
                return Err(invariant("prospective descendant is missing"));
            };
            let member = MempoolMemberIdentity {
                txid,
                wtxid: entry.wtxid,
            };
            removals.insert(
                member,
                MempoolRemovalFact {
                    cause,
                    role: if txid == victim_entry.txid {
                        MempoolRemovalRole::Direct
                    } else {
                        MempoolRemovalRole::Descendant
                    },
                },
            );
            members.push(member);
        }
        self.compose(SubDelta::removals(removals))?;
        Ok(members)
    }

    pub(super) fn prepare_patch(
        self,
        delta: MempoolLifecycleDelta,
    ) -> Result<MempoolPatch, MempoolError> {
        let next_revision = self.base.revision.next()?;
        let removed_txids = self
            .removed
            .keys()
            .map(|member| member.txid)
            .collect::<BTreeSet<_>>();
        let entry_upserts = self
            .added_or_updated
            .into_iter()
            .filter(|(txid, _entry)| {
                !removed_txids.contains(txid) && !self.base.entries.contains_key(txid)
            })
            .collect();
        let entry_removals = removed_txids
            .into_iter()
            .filter(|txid| self.base.entries.contains_key(txid))
            .collect();
        Ok(MempoolPatch {
            base_revision: self.base.revision,
            next_revision,
            entry_upserts,
            entry_removals,
            spent_updates: self.spent_updates.into_iter().collect(),
            topology_updates: self.topology_updates,
            resource_delta: self.resource_delta,
            rolling_fee_state: self.rolling_fee_state,
            delta,
        })
    }

    pub(super) fn accounted_memory(&self) -> crate::AccountedMempoolMemory {
        self.prospective_resource_usage.accounted_memory()
    }

    pub(super) fn visible_txids(&self) -> BTreeSet<Txid> {
        self.base
            .entries
            .keys()
            .copied()
            .chain(self.added_or_updated.keys().copied())
            .filter(|txid| !self.is_removed_txid(*txid))
            .collect()
    }

    pub(super) fn rolling_fee_state_mut(&mut self) -> &mut RollingFeeState {
        &mut self.rolling_fee_state
    }

    #[cfg(test)]
    pub(super) const fn rolling_fee_state(&self) -> &RollingFeeState {
        &self.rolling_fee_state
    }

    pub(super) fn record_trim_invocation(&mut self) {
        #[cfg(test)]
        {
            self.trim_invocations += 1;
        }
    }

    fn apply_sub_delta(&mut self, sub_delta: SubDelta) -> Result<(), MempoolError> {
        self.validate_sub_delta(&sub_delta)?;
        let mut affected = BTreeSet::new();
        self.apply_removals(sub_delta.removals, &mut affected)?;
        self.apply_additions(sub_delta.entry_upserts, &mut affected)?;
        self.expand_affected(&mut affected);
        self.refresh_aggregates(&affected)?;
        self.refresh_topology_updates();
        self.refresh_resource_ledger()?;
        Ok(())
    }

    fn validate_sub_delta(&self, sub_delta: &SubDelta) -> Result<(), MempoolError> {
        let removal_txids = sub_delta
            .removals
            .keys()
            .map(|member| member.txid)
            .collect::<BTreeSet<_>>();
        for member in sub_delta.removals.keys() {
            if self.removed.contains_key(member) {
                return Err(invariant("sub-delta repeats a removal"));
            }
            let Some(entry) = self.maybe_entry(&member.txid) else {
                return Err(invariant("sub-delta removes a missing entry"));
            };
            if entry.wtxid != member.wtxid {
                return Err(invariant("sub-delta removal identity conflicts"));
            }
            if entry
                .children
                .iter()
                .any(|child| !removal_txids.contains(child))
            {
                return Err(invariant(
                    "sub-delta removal leaves a descendant without its parent",
                ));
            }
        }

        for entry in sub_delta.entry_upserts.values() {
            if self.maybe_entry(&entry.txid).is_some() || removal_txids.contains(&entry.txid) {
                return Err(invariant("sub-delta contains a duplicate addition"));
            }
            if self.visible_txids().into_iter().any(|txid| {
                self.maybe_entry(&txid)
                    .is_some_and(|existing| existing.wtxid == entry.wtxid)
            }) {
                return Err(invariant("sub-delta addition conflicts by wtxid"));
            }
            for input in &entry.transaction.inputs {
                if let Some(spender) = self.maybe_spender(&input.previous_output)
                    && !removal_txids.contains(&spender)
                {
                    return Err(invariant("sub-delta contains a double-spend edit"));
                }
            }
        }
        Ok(())
    }

    fn apply_removals(
        &mut self,
        removals: BTreeMap<MempoolMemberIdentity, MempoolRemovalFact>,
        affected: &mut BTreeSet<Txid>,
    ) -> Result<(), MempoolError> {
        for (member, fact) in removals {
            let entry = self
                .maybe_entry(&member.txid)
                .cloned()
                .ok_or_else(|| invariant("prospective removal entry disappeared"))?;
            affected.extend(entry.parents.iter().copied());
            affected.extend(entry.children.iter().copied());
            affected.extend(self.collect_ancestors(member.txid));
            for parent in &entry.parents {
                if let Some(parent_entry) = self.promote_entry(*parent) {
                    parent_entry.children.remove(&member.txid);
                }
            }
            for child in &entry.children {
                if let Some(child_entry) = self.promote_entry(*child) {
                    child_entry.parents.remove(&member.txid);
                }
            }
            for input in &entry.transaction.inputs {
                if self.maybe_spender(&input.previous_output) == Some(member.txid) {
                    self.spent_updates
                        .insert(input.previous_output.clone(), None);
                }
            }
            self.added_or_updated.remove(&member.txid);
            self.removed.insert(member, fact);
        }
        Ok(())
    }

    fn apply_additions(
        &mut self,
        additions: BTreeMap<Txid, MempoolEntry>,
        affected: &mut BTreeSet<Txid>,
    ) -> Result<(), MempoolError> {
        for (txid, mut entry) in additions {
            let parents = entry
                .transaction
                .inputs
                .iter()
                .filter_map(|input| {
                    self.maybe_entry(&input.previous_output.txid)
                        .filter(|parent| {
                            (input.previous_output.vout as usize) < parent.transaction.outputs.len()
                        })
                        .map(|parent| parent.txid)
                })
                .collect::<BTreeSet<_>>();
            let children = self
                .added_or_updated
                .values()
                .filter(|child| {
                    child
                        .transaction
                        .inputs
                        .iter()
                        .any(|input| input.previous_output.txid == txid)
                })
                .map(|child| child.txid)
                .collect::<BTreeSet<_>>();
            entry.parents.clone_from(&parents);
            entry.children.clone_from(&children);
            let own_stats = AggregateStats::new(1, entry.virtual_size, entry.fee_sats());
            entry.ancestor_stats = own_stats;
            entry.descendant_stats = own_stats;
            self.added_or_updated.insert(txid, entry);
            affected.insert(txid);
            for parent in parents {
                affected.insert(parent);
                let parent_entry = self
                    .promote_entry(parent)
                    .ok_or_else(|| invariant("prospective parent disappeared"))?;
                parent_entry.children.insert(txid);
            }
            for child in children {
                affected.insert(child);
                let child_entry = self
                    .promote_entry(child)
                    .ok_or_else(|| invariant("prospective child disappeared"))?;
                child_entry.parents.insert(txid);
            }
            let inputs = self
                .maybe_entry(&txid)
                .ok_or_else(|| invariant("prospective addition disappeared"))?
                .transaction
                .inputs
                .iter()
                .map(|input| input.previous_output.clone())
                .collect::<Vec<_>>();
            for outpoint in inputs {
                self.spent_updates.insert(outpoint, Some(txid));
            }
        }
        Ok(())
    }

    fn promote_entry(&mut self, txid: Txid) -> Option<&mut MempoolEntry> {
        if !self.added_or_updated.contains_key(&txid) {
            let entry = self.maybe_entry(&txid)?.clone();
            self.added_or_updated.insert(txid, entry);
        }
        self.added_or_updated.get_mut(&txid)
    }

    fn expand_affected(&self, affected: &mut BTreeSet<Txid>) {
        let seeds = affected.iter().copied().collect::<Vec<_>>();
        for txid in seeds {
            affected.extend(self.collect_ancestors(txid));
            affected.extend(self.collect_descendants(txid));
        }
        affected.retain(|txid| self.maybe_entry(txid).is_some());
    }

    fn refresh_aggregates(&mut self, affected: &BTreeSet<Txid>) -> Result<(), MempoolError> {
        let updates = affected
            .iter()
            .map(|txid| {
                Ok((
                    *txid,
                    self.aggregate(*txid, &self.collect_ancestors(*txid))?,
                    self.aggregate(*txid, &self.collect_descendants(*txid))?,
                ))
            })
            .collect::<Result<Vec<_>, MempoolError>>()?;
        for (txid, ancestor_stats, descendant_stats) in updates {
            let entry = self
                .promote_entry(txid)
                .ok_or_else(|| invariant("affected prospective entry disappeared"))?;
            entry.ancestor_stats = ancestor_stats;
            entry.descendant_stats = descendant_stats;
        }
        Ok(())
    }

    fn aggregate(
        &self,
        txid: Txid,
        related: &BTreeSet<Txid>,
    ) -> Result<AggregateStats, MempoolError> {
        let entry = self
            .maybe_entry(&txid)
            .ok_or_else(|| invariant("aggregate root is missing"))?;
        let mut virtual_size = entry.virtual_size;
        let mut total_fee_sats = entry.fee_sats();
        for related_txid in related {
            let related_entry = self
                .maybe_entry(related_txid)
                .ok_or_else(|| invariant("aggregate member is missing"))?;
            virtual_size = virtual_size
                .checked_add(
                    related_entry.virtual_size,
                    "prospective aggregate virtual size",
                )
                .map_err(resource_invariant_error)?;
            total_fee_sats = total_fee_sats
                .checked_add(related_entry.fee_sats())
                .ok_or_else(|| invariant("prospective aggregate fee overflow"))?;
        }
        let count = related
            .len()
            .checked_add(1)
            .ok_or_else(|| invariant("prospective aggregate count overflow"))?;
        Ok(AggregateStats::new(count, virtual_size, total_fee_sats))
    }

    fn collect_ancestors(&self, txid: Txid) -> BTreeSet<Txid> {
        self.collect_relation_closure(txid, true)
    }

    pub(super) fn collect_descendants(&self, txid: Txid) -> BTreeSet<Txid> {
        self.collect_relation_closure(txid, false)
    }

    fn collect_relation_closure(&self, txid: Txid, use_parents: bool) -> BTreeSet<Txid> {
        let mut found = BTreeSet::new();
        let mut pending = self
            .maybe_entry(&txid)
            .map(|entry| {
                if use_parents {
                    entry.parents.iter().copied().collect::<Vec<_>>()
                } else {
                    entry.children.iter().copied().collect::<Vec<_>>()
                }
            })
            .unwrap_or_default();
        while let Some(related_txid) = pending.pop() {
            if !found.insert(related_txid) {
                continue;
            }
            if let Some(entry) = self.maybe_entry(&related_txid) {
                pending.extend(if use_parents {
                    entry.parents.iter().copied()
                } else {
                    entry.children.iter().copied()
                });
            }
        }
        found
    }

    fn refresh_topology_updates(&mut self) {
        self.topology_updates = self
            .added_or_updated
            .iter()
            .filter(|(txid, _entry)| self.base.entries.contains_key(txid))
            .map(|(txid, entry)| {
                (
                    *txid,
                    TopologyUpdate {
                        parents: entry.parents.clone(),
                        children: entry.children.clone(),
                        ancestor_stats: entry.ancestor_stats,
                        descendant_stats: entry.descendant_stats,
                    },
                )
            })
            .collect();
    }

    fn refresh_resource_ledger(&mut self) -> Result<(), MempoolError> {
        let mut ledger = self.base.resource_ledger;
        let removed_txids = self
            .removed
            .keys()
            .map(|member| member.txid)
            .collect::<BTreeSet<_>>();
        for txid in &removed_txids {
            if let Some(base_entry) = self.base.entries.get(txid) {
                ledger
                    .checked_remove_entry(base_entry)
                    .map_err(resource_invariant_error)?;
            }
        }
        for (txid, updated) in &self.added_or_updated {
            if let Some(base_entry) = self.base.entries.get(txid) {
                ledger
                    .checked_replace_entry(base_entry, updated)
                    .map_err(resource_invariant_error)?;
            } else {
                ledger
                    .checked_add_entry(updated)
                    .map_err(resource_invariant_error)?;
            }
        }
        for (outpoint, maybe_spender) in &self.spent_updates {
            let had_base = self.base.spent_outpoints.contains_key(outpoint);
            match (had_base, maybe_spender.is_some()) {
                (true, false) => ledger
                    .checked_remove_spent_outpoints(1)
                    .map_err(resource_invariant_error)?,
                (false, true) => ledger
                    .checked_add_spent_outpoints(1)
                    .map_err(resource_invariant_error)?,
                _ => {}
            }
        }
        self.prospective_resource_usage = ledger;
        self.resource_delta = MempoolResourceDelta {
            next_ledger: ledger,
        };
        Ok(())
    }

    fn is_removed_txid(&self, txid: Txid) -> bool {
        self.removed.keys().any(|member| member.txid == txid)
    }

    #[cfg(test)]
    pub(super) fn materialize_for_test(&self) -> Result<super::oracle::MempoolState, MempoolError> {
        let mut entries = self.base.entries.clone();
        for member in self.removed.keys() {
            entries.remove(&member.txid);
        }
        for (txid, entry) in &self.added_or_updated {
            if !self.is_removed_txid(*txid) {
                entries.insert(*txid, entry.clone());
            }
        }
        super::recompute_state(entries).map_err(resource_invariant_error)
    }

    #[cfg(test)]
    pub(super) const fn full_clone_count_for_test(&self) -> usize {
        0
    }

    #[cfg(test)]
    pub(super) const fn full_recompute_count_for_test(&self) -> usize {
        0
    }

    #[cfg(test)]
    pub(super) const fn trim_invocations_for_test(&self) -> usize {
        self.trim_invocations
    }
}

fn invariant(reason: &'static str) -> MempoolError {
    MempoolError::InternalInvariant {
        reason: reason.to_string(),
    }
}
