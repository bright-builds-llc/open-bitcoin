// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/policy/packages.md
// - packages/bitcoin-knots/src/kernel/mempool_options.h
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/policy/policy.h
// - packages/bitcoin-knots/src/policy/rbf.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/txmempool.h

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use open_bitcoin_chainstate::ChainstateSnapshot;
use open_bitcoin_consensus::TransactionInputContext;
#[cfg(test)]
use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags, TransactionValidationContext};
use open_bitcoin_primitives::{OutPoint, Transaction, Txid};

use crate::fee::rolling::RollingFeeState;
use crate::{
    EffectiveAdmissionFeeRate, FeeRate, MEMPOOL_HEIGHT, MempoolEntry, MempoolError,
    MempoolResourceLedger, PolicyConfig, PolicyTime, RbfPolicy, ResourceAccountingError,
    RollingMempoolFeeRate, TransactionVirtualSize, signals_opt_in_rbf,
};

mod admission;
mod admission_outcome;
mod candidate;
mod expiry;
mod lifecycle;
#[cfg(test)]
mod oracle;
mod package_admission;
mod patch;
#[allow(dead_code)] // Package orchestration consumes prospective pressure trim in Plan 132-04.
mod pressure;
#[allow(dead_code)] // Package orchestration consumes this staged infrastructure in Plan 132-04.
mod prospective;
mod topology;
use self::admission_outcome::accept as accept_outcome;
use self::topology::collect_conflicts_and_descendants;
pub use lifecycle::{
    FinalMempoolMembership, MempoolCapacityEnforcement, MempoolCapacityStatus,
    MempoolLifecycleDelta, MempoolLifecycleDeltaBuilder, MempoolLifecycleInvariantError,
    MempoolLifecycleRemoval, MempoolLifecycleSummary, MempoolMemberIdentity, MempoolMemberState,
    MempoolPressureSummary, MempoolRemovalCause, MempoolRemovalRole, MempoolRetryClear,
    MempoolRetryClearCause, RollingFeeParityStatus,
};
#[cfg(test)]
use oracle::{recompute_state, validate_limits};
#[cfg(test)]
use topology::{collect_ancestors, collect_descendants};

/// Separates one admission attempt result from facts that were actually committed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolTransition {
    pub outcome: crate::MempoolOutcome,
    pub delta: MempoolLifecycleDelta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MempoolRevision(pub(super) u64);

impl MempoolRevision {
    const ZERO: Self = Self(0);

    fn next(self) -> Result<Self, MempoolError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(MempoolError::RevisionExhausted)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TopologyUpdate {
    parents: BTreeSet<Txid>,
    children: BTreeSet<Txid>,
    ancestor_stats: crate::AggregateStats,
    descendant_stats: crate::AggregateStats,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MempoolResourceDelta {
    next_ledger: MempoolResourceLedger,
}

pub(super) struct MempoolPatch {
    base_revision: MempoolRevision,
    next_revision: MempoolRevision,
    entry_upserts: BTreeMap<Txid, MempoolEntry>,
    entry_removals: BTreeSet<Txid>,
    spent_updates: BTreeMap<OutPoint, Option<Txid>>,
    topology_updates: BTreeMap<Txid, TopologyUpdate>,
    resource_delta: MempoolResourceDelta,
    rolling_fee_state: RollingFeeState,
    delta: MempoolLifecycleDelta,
}

#[derive(Debug, Clone)]
pub struct Mempool {
    config: PolicyConfig,
    rolling_fee_state: RollingFeeState,
    entries: HashMap<Txid, MempoolEntry>,
    spent_outpoints: HashMap<OutPoint, Txid>,
    resource_ledger: MempoolResourceLedger,
    revision: MempoolRevision,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
struct CompleteMempoolSnapshot {
    entries: HashMap<Txid, MempoolEntry>,
    spent_outpoints: HashMap<OutPoint, Txid>,
    resource_ledger: MempoolResourceLedger,
    rolling_fee_state: RollingFeeState,
    revision: MempoolRevision,
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
            rolling_fee_state: RollingFeeState::new(),
            entries: HashMap::new(),
            spent_outpoints: HashMap::new(),
            resource_ledger: MempoolResourceLedger::ZERO,
            revision: MempoolRevision::ZERO,
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
        self.rolling_fee_state.rolling_fee_rate()
    }

    #[cfg(test)]
    fn complete_snapshot(&self) -> CompleteMempoolSnapshot {
        CompleteMempoolSnapshot {
            entries: self.entries.clone(),
            spent_outpoints: self.spent_outpoints.clone(),
            resource_ledger: self.resource_ledger,
            rolling_fee_state: self.rolling_fee_state.clone(),
            revision: self.revision,
        }
    }

    /// Installs a rolling floor for operator evidence and Phase-131 pressure seams.
    pub fn set_rolling_mempool_fee_rate(
        &mut self,
        rate: RollingMempoolFeeRate,
    ) -> Result<(), MempoolError> {
        let mut next_state = self.rolling_fee_state.clone();
        next_state.set_rolling_fee_rate(rate);
        self.apply_rolling_state(next_state)
    }

    /// Knots `trackPackageRemoved` for pressure bumps and hermetic fixtures.
    pub fn track_package_removed(
        &mut self,
        package_plus_incremental: FeeRate,
    ) -> Result<(), MempoolError> {
        let mut next_state = self.rolling_fee_state.clone();
        next_state.track_package_removed(package_plus_incremental);
        self.apply_rolling_state(next_state)
    }

    /// Applies block-gated rolling decay with an injected policy clock.
    pub fn materialize_rolling_fee_rate(
        &mut self,
        now: PolicyTime,
    ) -> Result<RollingMempoolFeeRate, MempoolError> {
        let mut next_state = self.rolling_fee_state.clone();
        let rate = next_state.decay_toward(
            now,
            self.accounted_memory(),
            self.config.mempool_capacity,
            self.config.incremental_relay_fee_rate,
        );
        self.apply_rolling_state(next_state)?;
        Ok(rate)
    }

    fn apply_rolling_state(&mut self, next_state: RollingFeeState) -> Result<(), MempoolError> {
        if next_state == self.rolling_fee_state {
            return Ok(());
        }
        let next_revision = self.revision.next()?;
        self.rolling_fee_state = next_state;
        self.advance_revision(next_revision);
        Ok(())
    }

    fn advance_revision(&mut self, next_revision: MempoolRevision) {
        self.revision = next_revision;
    }

    pub(super) fn apply_prepared(
        &mut self,
        patch: MempoolPatch,
    ) -> Result<MempoolLifecycleDelta, MempoolError> {
        if self.revision != patch.base_revision {
            return Err(MempoolError::StalePreparedTransition {
                expected_revision: patch.base_revision.0,
                actual_revision: self.revision.0,
            });
        }

        let MempoolPatch {
            base_revision: _,
            next_revision,
            entry_upserts,
            entry_removals,
            spent_updates,
            topology_updates,
            resource_delta,
            rolling_fee_state,
            delta,
        } = patch;
        for txid in entry_removals {
            self.entries.remove(&txid);
        }
        for (txid, entry) in entry_upserts {
            self.entries.insert(txid, entry);
        }
        for (outpoint, maybe_spender) in spent_updates {
            if let Some(spender) = maybe_spender {
                self.spent_outpoints.insert(outpoint, spender);
            } else {
                self.spent_outpoints.remove(&outpoint);
            }
        }
        for (txid, update) in topology_updates {
            if let Some(entry) = self.entries.get_mut(&txid) {
                entry.parents = update.parents;
                entry.children = update.children;
                entry.ancestor_stats = update.ancestor_stats;
                entry.descendant_stats = update.descendant_stats;
            }
        }
        self.resource_ledger = resource_delta.next_ledger;
        self.rolling_fee_state = rolling_fee_state;
        self.advance_revision(next_revision);
        Ok(delta)
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
    view: &impl candidate::CandidateMempoolView,
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

        let Some(parent_entry) = view.maybe_entry(&input.previous_output.txid) else {
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

#[cfg(test)]
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

pub(super) fn resource_invariant_error(source: ResourceAccountingError) -> MempoolError {
    MempoolError::InternalInvariant {
        reason: source.to_string(),
    }
}

#[cfg(test)]
mod tests;
