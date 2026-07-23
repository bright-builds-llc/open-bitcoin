// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/policy/packages.cpp

use std::collections::{BTreeSet, HashMap, HashSet};

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, TransactionInputContext, TransactionValidationContext,
};
use open_bitcoin_primitives::{OutPoint, Transaction, Txid};

use crate::{
    EffectiveAdmissionFeeRate, LimitDirection, LimitKind, MEMPOOL_HEIGHT, MempoolEntry,
    MempoolError, MempoolResourceLedger, PolicyConfig, RbfPolicy, ResourceAccountingError,
    RollingMempoolFeeRate, TransactionVirtualSize, build_resource_ledger, signals_opt_in_rbf,
};

mod admission;
mod admission_outcome;
mod lifecycle;
mod topology;
use self::admission_outcome::accept as accept_outcome;
use self::topology::{collect_ancestors, collect_conflicts_and_descendants, collect_descendants};
pub use lifecycle::{
    FinalMempoolMembership, MempoolCapacityStatus, MempoolLifecycleDelta,
    MempoolLifecycleDeltaBuilder, MempoolLifecycleInvariantError, MempoolLifecycleRemoval,
    MempoolLifecycleSummary, MempoolMemberIdentity, MempoolMemberState, MempoolPressureSummary,
    MempoolRemovalCause, MempoolRemovalRole, MempoolRetryClear, MempoolRetryClearCause,
    RollingFeeParityStatus,
};

/// Separates one admission attempt result from facts that were actually committed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolTransition {
    pub outcome: crate::MempoolOutcome,
    pub delta: MempoolLifecycleDelta,
}

#[derive(Debug, Clone)]
struct MempoolState {
    entries: HashMap<Txid, MempoolEntry>,
    spent_outpoints: HashMap<OutPoint, Txid>,
    resource_ledger: MempoolResourceLedger,
}

#[derive(Debug, Clone)]
pub struct Mempool {
    config: PolicyConfig,
    rolling_mempool_fee_rate: RollingMempoolFeeRate,
    entries: HashMap<Txid, MempoolEntry>,
    spent_outpoints: HashMap<OutPoint, Txid>,
    resource_ledger: MempoolResourceLedger,
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new(PolicyConfig::default())
    }
}

impl Mempool {
    pub fn new(config: PolicyConfig) -> Self {
        Self {
            config,
            rolling_mempool_fee_rate: RollingMempoolFeeRate::ZERO,
            entries: HashMap::new(),
            spent_outpoints: HashMap::new(),
            resource_ledger: MempoolResourceLedger::ZERO,
        }
    }

    pub fn config(&self) -> &PolicyConfig {
        &self.config
    }

    pub fn entries(&self) -> &HashMap<Txid, MempoolEntry> {
        &self.entries
    }

    pub fn entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        self.entries.get(txid)
    }

    pub const fn resource_ledger(&self) -> MempoolResourceLedger {
        self.resource_ledger
    }

    pub const fn total_virtual_size(&self) -> TransactionVirtualSize {
        self.resource_ledger.total_virtual_size()
    }

    pub const fn accounted_memory(&self) -> crate::AccountedMempoolMemory {
        self.resource_ledger.accounted_memory()
    }

    pub const fn rolling_mempool_fee_rate(&self) -> RollingMempoolFeeRate {
        self.rolling_mempool_fee_rate
    }

    fn replacement_set(
        &self,
        transaction: &Transaction,
        candidate_fee_sats: i64,
        virtual_size: TransactionVirtualSize,
    ) -> Result<BTreeSet<Txid>, MempoolError> {
        let direct_conflicts = self.direct_conflicts(transaction);
        if direct_conflicts.is_empty() {
            return Ok(BTreeSet::new());
        }

        self.validate_replacement(
            transaction,
            &direct_conflicts,
            candidate_fee_sats,
            virtual_size,
        )
    }

    fn direct_conflicts(&self, transaction: &Transaction) -> BTreeSet<Txid> {
        let mut conflicts = BTreeSet::new();
        for input in &transaction.inputs {
            let Some(conflicting_txid) = self.spent_outpoints.get(&input.previous_output) else {
                continue;
            };
            conflicts.insert(*conflicting_txid);
        }

        conflicts
    }

    fn validate_replacement(
        &self,
        transaction: &Transaction,
        direct_conflicts: &BTreeSet<Txid>,
        candidate_fee_sats: i64,
        virtual_size: TransactionVirtualSize,
    ) -> Result<BTreeSet<Txid>, MempoolError> {
        if self.config.rbf_policy == RbfPolicy::Never {
            return Err(MempoolError::ConflictNotAllowed {
                conflicting: direct_conflicts.iter().copied().collect(),
                policy: RbfPolicy::Never,
            });
        }

        let has_opt_in_signal = direct_conflicts
            .iter()
            .filter_map(|txid| self.entries.get(txid))
            .any(|entry| signals_opt_in_rbf(&entry.transaction));
        if self.config.rbf_policy == RbfPolicy::OptIn && !has_opt_in_signal {
            return Err(MempoolError::ConflictNotAllowed {
                conflicting: direct_conflicts.iter().copied().collect(),
                policy: RbfPolicy::OptIn,
            });
        }

        let replace_set = collect_conflicts_and_descendants(&self.entries, direct_conflicts);
        let conflicting_fee_sats = replace_set
            .iter()
            .filter_map(|txid| self.entries.get(txid))
            .map(MempoolEntry::fee_sats)
            .sum::<i64>();
        if candidate_fee_sats <= conflicting_fee_sats {
            return Err(MempoolError::ReplacementRejected {
                reason: format!(
                    "replacement fee {candidate_fee_sats} must exceed conflicting fee {conflicting_fee_sats}"
                ),
            });
        }

        let candidate_feerate =
            crate::FeeRate::from_fee_sats_and_vbytes(candidate_fee_sats, virtual_size);
        for conflicting_txid in direct_conflicts {
            let Some(entry) = self.entries.get(conflicting_txid) else {
                continue;
            };
            if candidate_feerate <= entry.fee_rate() {
                return Err(MempoolError::ReplacementRejected {
                    reason: format!(
                        "replacement feerate {} must exceed conflicting feerate {} for {:?}",
                        candidate_feerate,
                        entry.fee_rate(),
                        conflicting_txid
                    ),
                });
            }
        }

        let required_bump = self
            .config
            .incremental_relay_fee_rate
            .fee_rate()
            .fee_for_virtual_size(virtual_size);
        let additional_fee = candidate_fee_sats - conflicting_fee_sats;
        if additional_fee < required_bump {
            return Err(MempoolError::ReplacementRejected {
                reason: format!(
                    "replacement fee bump {additional_fee} must be at least {required_bump}"
                ),
            });
        }

        let conflicting_inputs = self.collect_conflicting_inputs(direct_conflicts);
        let adds_new_unconfirmed_input = transaction.inputs.iter().any(|input| {
            self.entries.contains_key(&input.previous_output.txid)
                && !conflicting_inputs.contains(&input.previous_output)
        });
        if adds_new_unconfirmed_input {
            return Err(MempoolError::ReplacementRejected {
                reason: "replacement adds new unconfirmed inputs".to_string(),
            });
        }

        Ok(replace_set)
    }

    fn collect_conflicting_inputs(&self, direct_conflicts: &BTreeSet<Txid>) -> HashSet<OutPoint> {
        direct_conflicts
            .iter()
            .filter_map(|txid| self.entries.get(txid))
            .flat_map(|entry| {
                entry
                    .transaction
                    .inputs
                    .iter()
                    .map(|input| input.previous_output.clone())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn derive_input_contexts(
    transaction: &Transaction,
    chainstate: &ChainstateSnapshot,
    entries: &HashMap<Txid, MempoolEntry>,
) -> Result<Vec<TransactionInputContext>, MempoolError> {
    let maybe_tip = chainstate.tip();
    let mempool_median_time_past = maybe_tip.map_or(0, |tip| tip.median_time_past);
    let mut input_contexts = Vec::with_capacity(transaction.inputs.len());

    for input in &transaction.inputs {
        if let Some(coin) = chainstate.utxos.get(&input.previous_output) {
            input_contexts.push(TransactionInputContext {
                spent_output: coin.as_spent_output(),
                created_height: coin.created_height,
                created_median_time_past: coin.created_median_time_past,
            });
            continue;
        }

        let Some(parent_entry) = entries.get(&input.previous_output.txid) else {
            return Err(MempoolError::MissingInput {
                outpoint: input.previous_output.clone(),
            });
        };
        let output_index = input.previous_output.vout as usize;
        let Some(output) = parent_entry.transaction.outputs.get(output_index) else {
            return Err(MempoolError::MissingInput {
                outpoint: input.previous_output.clone(),
            });
        };

        input_contexts.push(TransactionInputContext {
            spent_output: open_bitcoin_consensus::SpentOutput {
                value: output.value,
                script_pubkey: output.script_pubkey.clone(),
                is_coinbase: false,
            },
            created_height: MEMPOOL_HEIGHT,
            created_median_time_past: mempool_median_time_past,
        });
    }

    Ok(input_contexts)
}

fn build_validation_context(
    chainstate: &ChainstateSnapshot,
    input_contexts: Vec<TransactionInputContext>,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> TransactionValidationContext {
    let maybe_tip = chainstate.tip();
    TransactionValidationContext {
        inputs: input_contexts,
        spend_height: maybe_tip.map_or(0, |tip| tip.height.saturating_add(1)),
        block_time: maybe_tip.map_or(0, |tip| i64::from(tip.header.time)),
        median_time_past: maybe_tip.map_or(0, |tip| tip.median_time_past),
        verify_flags,
        consensus_params,
    }
}

fn enforce_min_relay_fee(
    effective_fee_rate: EffectiveAdmissionFeeRate,
    fee_sats: i64,
    virtual_size: TransactionVirtualSize,
) -> Result<(), MempoolError> {
    let required_fee_sats = effective_fee_rate
        .fee_rate()
        .fee_for_virtual_size(virtual_size);
    if fee_sats < required_fee_sats {
        let fee = amount_from_fee_sats(fee_sats)?;
        return Err(MempoolError::RelayFeeTooLow {
            fee,
            required_fee_sats,
            virtual_size: virtual_size.as_usize(),
        });
    }

    Ok(())
}

fn amount_from_fee_sats(fee_sats: i64) -> Result<open_bitcoin_primitives::Amount, MempoolError> {
    open_bitcoin_primitives::Amount::from_sats(fee_sats).map_err(|source| {
        MempoolError::Validation {
            reason: format!("transaction fee is outside money range: {source}"),
        }
    })
}

fn serialization_validation_error(
    context: &'static str,
    source: impl std::fmt::Display,
) -> MempoolError {
    MempoolError::Validation {
        reason: format!("{context} serialization failed: {source}"),
    }
}

fn validate_limits(
    entries: &HashMap<Txid, MempoolEntry>,
    config: &PolicyConfig,
    candidate_txid: Txid,
) -> Result<(), MempoolError> {
    let Some(candidate_entry) = entries.get(&candidate_txid) else {
        return Err(MempoolError::InternalInvariant {
            reason: format!(
                "candidate {:?} missing from prospective state",
                candidate_txid
            ),
        });
    };
    if candidate_entry.ancestor_stats.count > config.max_ancestor_count {
        return Err(MempoolError::LimitExceeded {
            direction: LimitDirection::Ancestor,
            kind: LimitKind::Count,
            txid: None,
            attempted: candidate_entry.ancestor_stats.count,
            max: config.max_ancestor_count,
        });
    }
    if candidate_entry.ancestor_stats.virtual_size.as_usize() > config.max_ancestor_virtual_size {
        return Err(MempoolError::LimitExceeded {
            direction: LimitDirection::Ancestor,
            kind: LimitKind::VirtualSize,
            txid: None,
            attempted: candidate_entry.ancestor_stats.virtual_size.as_usize(),
            max: config.max_ancestor_virtual_size,
        });
    }

    let mut candidate_ancestors = collect_ancestors(entries, candidate_txid);
    candidate_ancestors.insert(candidate_txid);
    for ancestor_txid in candidate_ancestors {
        let Some(entry) = entries.get(&ancestor_txid) else {
            return Err(MempoolError::InternalInvariant {
                reason: format!(
                    "ancestor {:?} missing during descendant limit validation",
                    ancestor_txid
                ),
            });
        };
        if entry.descendant_stats.count > config.max_descendant_count {
            return Err(MempoolError::LimitExceeded {
                direction: LimitDirection::Descendant,
                kind: LimitKind::Count,
                txid: Some(ancestor_txid),
                attempted: entry.descendant_stats.count,
                max: config.max_descendant_count,
            });
        }
        if entry.descendant_stats.virtual_size.as_usize() > config.max_descendant_virtual_size {
            return Err(MempoolError::LimitExceeded {
                direction: LimitDirection::Descendant,
                kind: LimitKind::VirtualSize,
                txid: Some(ancestor_txid),
                attempted: entry.descendant_stats.virtual_size.as_usize(),
                max: config.max_descendant_virtual_size,
            });
        }
    }

    Ok(())
}

fn trim_to_size(
    mut state: MempoolState,
    config: &PolicyConfig,
) -> Result<
    (
        MempoolState,
        std::collections::BTreeMap<MempoolMemberIdentity, MempoolRemovalRole>,
    ),
    MempoolError,
> {
    let mut evicted = std::collections::BTreeMap::new();

    while state.resource_ledger.total_virtual_size() > config.legacy_vsize_trim_limit {
        let Some(victim_txid) = select_eviction_candidate(&state.entries) else {
            break;
        };
        let mut remove_set = collect_descendants(&state.entries, victim_txid);
        remove_set.insert(victim_txid);
        let removed_members = state
            .entries
            .iter()
            .filter(|(txid, _entry)| remove_set.contains(txid))
            .map(|(txid, entry)| MempoolMemberIdentity {
                txid: *txid,
                wtxid: entry.wtxid,
            })
            .collect::<Vec<_>>();
        for member in removed_members {
            state.entries.remove(&member.txid);
            let role = if member.txid == victim_txid {
                MempoolRemovalRole::Direct
            } else {
                MempoolRemovalRole::Descendant
            };
            evicted.insert(member, role);
        }
        state = recompute_state(state.entries).map_err(resource_invariant_error)?;
    }

    Ok((state, evicted))
}

fn select_eviction_candidate(entries: &HashMap<Txid, MempoolEntry>) -> Option<Txid> {
    entries
        .iter()
        .min_by(|(left_txid, left_entry), (right_txid, right_entry)| {
            left_entry
                .descendant_score()
                .cmp(&right_entry.descendant_score())
                .then_with(|| left_txid.cmp(right_txid))
        })
        .map(|(txid, _entry)| *txid)
}

fn recompute_state(
    mut entries: HashMap<Txid, MempoolEntry>,
) -> Result<MempoolState, ResourceAccountingError> {
    for entry in entries.values_mut() {
        entry.parents.clear();
        entry.children.clear();
        let stats = crate::AggregateStats::new(1, entry.virtual_size, entry.fee_sats());
        entry.ancestor_stats = stats;
        entry.descendant_stats = stats;
    }

    let mut relations = Vec::new();
    for (txid, entry) in &entries {
        for input in &entry.transaction.inputs {
            let Some(parent_entry) = entries.get(&input.previous_output.txid) else {
                continue;
            };
            let output_index = input.previous_output.vout as usize;
            if output_index < parent_entry.transaction.outputs.len() {
                relations.push((input.previous_output.txid, *txid));
            }
        }
    }
    for (parent_txid, child_txid) in relations {
        if let Some(parent_entry) = entries.get_mut(&parent_txid) {
            parent_entry.children.insert(child_txid);
        }
        if let Some(child_entry) = entries.get_mut(&child_txid) {
            child_entry.parents.insert(parent_txid);
        }
    }

    let mut spent_outpoints = HashMap::new();
    for (txid, entry) in &entries {
        for input in &entry.transaction.inputs {
            spent_outpoints.insert(input.previous_output.clone(), *txid);
        }
    }

    let updates = entries
        .iter()
        .map(|(txid, existing_entry)| {
            let ancestors = collect_ancestors(&entries, *txid);
            let descendants = collect_descendants(&entries, *txid);
            let ancestor_virtual_size = ancestors
                .iter()
                .filter_map(|ancestor_txid| entries.get(ancestor_txid))
                .try_fold(existing_entry.virtual_size, |total, ancestor| {
                    total.checked_add(ancestor.virtual_size, "ancestor aggregate virtual size")
                })?;
            let ancestor_fee_sats = existing_entry.fee_sats()
                + ancestors
                    .iter()
                    .filter_map(|ancestor_txid| entries.get(ancestor_txid))
                    .map(MempoolEntry::fee_sats)
                    .sum::<i64>();
            let descendant_virtual_size = descendants
                .iter()
                .filter_map(|descendant_txid| entries.get(descendant_txid))
                .try_fold(existing_entry.virtual_size, |total, descendant| {
                    total.checked_add(descendant.virtual_size, "descendant aggregate virtual size")
                })?;
            let descendant_fee_sats = existing_entry.fee_sats()
                + descendants
                    .iter()
                    .filter_map(|descendant_txid| entries.get(descendant_txid))
                    .map(MempoolEntry::fee_sats)
                    .sum::<i64>();
            Ok((
                *txid,
                crate::AggregateStats::new(
                    ancestors.len().saturating_add(1),
                    ancestor_virtual_size,
                    ancestor_fee_sats,
                ),
                crate::AggregateStats::new(
                    descendants.len().saturating_add(1),
                    descendant_virtual_size,
                    descendant_fee_sats,
                ),
            ))
        })
        .collect::<Result<Vec<_>, ResourceAccountingError>>()?;
    for (txid, ancestor_stats, descendant_stats) in updates {
        entries.entry(txid).and_modify(|entry| {
            entry.ancestor_stats = ancestor_stats;
            entry.descendant_stats = descendant_stats;
        });
    }

    let resource_ledger = build_resource_ledger(&entries, &spent_outpoints)?;
    Ok(MempoolState {
        entries,
        spent_outpoints,
        resource_ledger,
    })
}

pub(super) fn resource_invariant_error(source: ResourceAccountingError) -> MempoolError {
    MempoolError::InternalInvariant {
        reason: source.to_string(),
    }
}

#[cfg(test)]
mod tests;
