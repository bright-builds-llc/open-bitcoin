use std::collections::{BTreeSet, HashMap, HashSet};

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, TransactionInputContext, TransactionValidationContext,
    transaction_txid, transaction_wtxid, validate_transaction_with_context,
};
use open_bitcoin_primitives::{OutPoint, Transaction, Txid};

use crate::{
    AdmissionResult, LimitDirection, LimitKind, MEMPOOL_HEIGHT, MempoolEntry, MempoolError,
    PolicyConfig, RbfPolicy, signals_opt_in_rbf, transaction_sigops_cost,
    transaction_weight_and_virtual_size, validate_standard_transaction,
};

#[derive(Debug, Clone)]
struct MempoolState {
    entries: HashMap<Txid, MempoolEntry>,
    spent_outpoints: HashMap<OutPoint, Txid>,
    total_virtual_size: usize,
}

#[derive(Debug, Clone)]
pub struct Mempool {
    config: PolicyConfig,
    entries: HashMap<Txid, MempoolEntry>,
    spent_outpoints: HashMap<OutPoint, Txid>,
    total_virtual_size: usize,
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
            entries: HashMap::new(),
            spent_outpoints: HashMap::new(),
            total_virtual_size: 0,
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

    pub fn total_virtual_size(&self) -> usize {
        self.total_virtual_size
    }

    pub fn accept_transaction(
        &mut self,
        transaction: Transaction,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, MempoolError> {
        let txid = transaction_txid(&transaction)
            .map_err(|source| serialization_validation_error("transaction txid", source))?;
        if self.entries.contains_key(&txid) {
            return Err(MempoolError::DuplicateTransaction { txid });
        }

        let input_contexts = derive_input_contexts(&transaction, chainstate, &self.entries)?;
        let (weight, virtual_size) = transaction_weight_and_virtual_size(&transaction)?;
        let sigops_cost = transaction_sigops_cost(&transaction, &input_contexts)?;
        validate_standard_transaction(
            &transaction,
            &input_contexts,
            &self.config,
            weight,
            sigops_cost,
        )?;

        let validation_context =
            build_validation_context(chainstate, input_contexts, verify_flags, consensus_params);
        let fee = validate_transaction_with_context(&transaction, &validation_context).map_err(
            |source| MempoolError::Validation {
                reason: source.to_string(),
            },
        )?;
        enforce_min_relay_fee(&self.config, fee.to_sats(), virtual_size)?;
        let replace_set = self.replacement_set(&transaction, fee.to_sats(), virtual_size)?;

        let wtxid = transaction_wtxid(&transaction)
            .map_err(|source| serialization_validation_error("transaction wtxid", source))?;
        let mut prospective_entries = self.entries.clone();
        for conflict_txid in &replace_set {
            prospective_entries.remove(conflict_txid);
        }
        prospective_entries.insert(
            txid,
            MempoolEntry::new(
                transaction,
                txid,
                wtxid,
                fee,
                virtual_size,
                weight,
                sigops_cost,
            ),
        );

        let prospective_state = recompute_state(prospective_entries);
        validate_limits(&prospective_state.entries, &self.config, txid)?;
        let (trimmed_state, evicted) = trim_to_size(prospective_state, &self.config);
        if !trimmed_state.entries.contains_key(&txid) {
            return Err(MempoolError::CandidateEvicted { txid });
        }

        self.entries = trimmed_state.entries;
        self.spent_outpoints = trimmed_state.spent_outpoints;
        self.total_virtual_size = trimmed_state.total_virtual_size;

        Ok(AdmissionResult {
            accepted: txid,
            replaced: replace_set.into_iter().collect(),
            evicted: evicted.into_iter().collect(),
        })
    }

    fn replacement_set(
        &self,
        transaction: &Transaction,
        candidate_fee_sats: i64,
        virtual_size: usize,
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
        virtual_size: usize,
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
            crate::types::FeeRate::from_fee_sats_and_vbytes(candidate_fee_sats, virtual_size);
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
            .incremental_relay_feerate
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
    config: &PolicyConfig,
    fee_sats: i64,
    virtual_size: usize,
) -> Result<(), MempoolError> {
    let required_fee_sats = config.min_relay_feerate.fee_for_virtual_size(virtual_size);
    if fee_sats < required_fee_sats {
        let fee = amount_from_fee_sats(fee_sats)?;
        return Err(MempoolError::RelayFeeTooLow {
            fee,
            required_fee_sats,
            virtual_size,
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
    if candidate_entry.ancestor_stats.virtual_size > config.max_ancestor_virtual_size {
        return Err(MempoolError::LimitExceeded {
            direction: LimitDirection::Ancestor,
            kind: LimitKind::VirtualSize,
            txid: None,
            attempted: candidate_entry.ancestor_stats.virtual_size,
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
        if entry.descendant_stats.virtual_size > config.max_descendant_virtual_size {
            return Err(MempoolError::LimitExceeded {
                direction: LimitDirection::Descendant,
                kind: LimitKind::VirtualSize,
                txid: Some(ancestor_txid),
                attempted: entry.descendant_stats.virtual_size,
                max: config.max_descendant_virtual_size,
            });
        }
    }

    Ok(())
}

fn trim_to_size(mut state: MempoolState, config: &PolicyConfig) -> (MempoolState, BTreeSet<Txid>) {
    let mut evicted = BTreeSet::new();

    while state.total_virtual_size > config.max_mempool_virtual_size {
        let Some(victim_txid) = select_eviction_candidate(&state.entries) else {
            break;
        };
        let mut remove_set = collect_descendants(&state.entries, victim_txid);
        remove_set.insert(victim_txid);
        for txid in &remove_set {
            state.entries.remove(txid);
            evicted.insert(*txid);
        }
        state = recompute_state(state.entries);
    }

    (state, evicted)
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

fn recompute_state(mut entries: HashMap<Txid, MempoolEntry>) -> MempoolState {
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

    let total_virtual_size = entries
        .values()
        .map(|entry| entry.virtual_size)
        .sum::<usize>();
    let updates = entries
        .iter()
        .map(|(txid, existing_entry)| {
            let ancestors = collect_ancestors(&entries, *txid);
            let descendants = collect_descendants(&entries, *txid);
            let ancestor_virtual_size = existing_entry.virtual_size
                + ancestors
                    .iter()
                    .filter_map(|ancestor_txid| entries.get(ancestor_txid))
                    .map(|ancestor| ancestor.virtual_size)
                    .sum::<usize>();
            let ancestor_fee_sats = existing_entry.fee_sats()
                + ancestors
                    .iter()
                    .filter_map(|ancestor_txid| entries.get(ancestor_txid))
                    .map(MempoolEntry::fee_sats)
                    .sum::<i64>();
            let descendant_virtual_size = existing_entry.virtual_size
                + descendants
                    .iter()
                    .filter_map(|descendant_txid| entries.get(descendant_txid))
                    .map(|descendant| descendant.virtual_size)
                    .sum::<usize>();
            let descendant_fee_sats = existing_entry.fee_sats()
                + descendants
                    .iter()
                    .filter_map(|descendant_txid| entries.get(descendant_txid))
                    .map(MempoolEntry::fee_sats)
                    .sum::<i64>();
            (
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
            )
        })
        .collect::<Vec<_>>();
    for (txid, ancestor_stats, descendant_stats) in updates {
        entries.entry(txid).and_modify(|entry| {
            entry.ancestor_stats = ancestor_stats;
            entry.descendant_stats = descendant_stats;
        });
    }

    MempoolState {
        entries,
        spent_outpoints,
        total_virtual_size,
    }
}

fn collect_conflicts_and_descendants(
    entries: &HashMap<Txid, MempoolEntry>,
    direct_conflicts: &BTreeSet<Txid>,
) -> BTreeSet<Txid> {
    let mut txids = BTreeSet::new();
    for txid in direct_conflicts {
        txids.insert(*txid);
        txids.extend(collect_descendants(entries, *txid));
    }

    txids
}

fn collect_ancestors(entries: &HashMap<Txid, MempoolEntry>, txid: Txid) -> BTreeSet<Txid> {
    let mut visited = BTreeSet::new();
    let Some(entry) = entries.get(&txid) else {
        return visited;
    };
    let mut stack = entry.parents.iter().copied().collect::<Vec<_>>();
    while let Some(next_txid) = stack.pop() {
        if !visited.insert(next_txid) {
            continue;
        }
        if let Some(next_entry) = entries.get(&next_txid) {
            stack.extend(next_entry.parents.iter().copied());
        }
    }

    visited
}

fn collect_descendants(entries: &HashMap<Txid, MempoolEntry>, txid: Txid) -> BTreeSet<Txid> {
    let mut visited = BTreeSet::new();
    let Some(entry) = entries.get(&txid) else {
        return visited;
    };
    let mut stack = entry.children.iter().copied().collect::<Vec<_>>();
    while let Some(next_txid) = stack.pop() {
        if !visited.insert(next_txid) {
            continue;
        }
        if let Some(next_entry) = entries.get(&next_txid) {
            stack.extend(next_entry.children.iter().copied());
        }
    }

    visited
}

#[cfg(test)]
mod tests;
