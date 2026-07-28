// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_removal_reason.h
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use open_bitcoin_codec::CodecError;
use open_bitcoin_consensus::transaction_txid;
use open_bitcoin_primitives::{Block, Transaction, Txid, Wtxid};

use super::patch::prepare_removal_patch;
use super::{Mempool, MempoolError, PreparedMempoolTransition, collect_conflicts_and_descendants};
use crate::{
    AccountedMempoolMemory, BlockLifecycleContext, EffectiveAdmissionFeeRate,
    IncrementalRelayFeeRate, MempoolCapacity, RollingMempoolFeeRate, StaticRelayFeeRate,
    TransactionVirtualSize, effective_admission_fee_rate,
};

/// Stable semantic reason for removing a transaction from the mempool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolRemovalCause {
    Replacement,
    Expiry,
    Pressure,
    BlockConfirmation,
    BlockConflict,
    Reorg,
}

impl MempoolRemovalCause {
    /// Returns the fixed low-cardinality evidence label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Replacement => "replacement",
            Self::Expiry => "expiry",
            Self::Pressure => "pressure",
            Self::BlockConfirmation => "block_confirmation",
            Self::BlockConflict => "block_conflict",
            Self::Reorg => "reorg",
        }
    }

    /// Resolves contradictory duplicate causes without depending on insertion order.
    ///
    /// Block facts outrank admission and maintenance facts, followed by replacement,
    /// expiry, pressure, and reorg. Real transitions should normally provide only one
    /// cause; this precedence keeps aggregation deterministic when affected sets overlap.
    const fn priority(self) -> u8 {
        match self {
            Self::BlockConfirmation => 0,
            Self::BlockConflict => 1,
            Self::Replacement => 2,
            Self::Expiry => 3,
            Self::Pressure => 4,
            Self::Reorg => 5,
        }
    }
}

/// Whether a removal was selected directly or followed from an affected ancestor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolRemovalRole {
    Direct,
    Descendant,
}

impl MempoolRemovalRole {
    /// Returns the fixed low-cardinality evidence label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Descendant => "descendant",
        }
    }
}

/// Canonical txid/wtxid pair for one lifecycle member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MempoolMemberIdentity {
    pub txid: Txid,
    pub wtxid: Wtxid,
}

/// Final authoritative mempool membership after a committed transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalMempoolMembership {
    Present,
    Absent,
}

impl FinalMempoolMembership {
    /// Returns the fixed low-cardinality evidence label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Absent => "absent",
        }
    }
}

/// Final state for one affected member.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MempoolMemberState {
    pub member: MempoolMemberIdentity,
    pub membership: FinalMempoolMembership,
}

/// Semantic reason that initial-broadcast retry state may be cleared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolRetryClearCause {
    LifecycleRemoval,
    EligibleServe,
    TransportWritten,
}

impl MempoolRetryClearCause {
    /// Returns the fixed low-cardinality evidence label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleRemoval => "lifecycle_removal",
            Self::EligibleServe => "eligible_serve",
            Self::TransportWritten => "transport_written",
        }
    }

    const fn priority(self) -> u8 {
        match self {
            Self::LifecycleRemoval => 0,
            Self::TransportWritten => 1,
            Self::EligibleServe => 2,
        }
    }
}

/// One committed retry-clear fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MempoolRetryClear {
    pub member: MempoolMemberIdentity,
    pub cause: MempoolRetryClearCause,
}

/// A typed contradiction encountered while assembling a lifecycle delta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolLifecycleInvariantError {
    IdentityConflict { txid: Txid, wtxid: Wtxid },
    MissingFinalMembership { member: MempoolMemberIdentity },
}

impl fmt::Display for MempoolLifecycleInvariantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentityConflict { txid, wtxid } => write!(
                formatter,
                "mempool lifecycle identity pair txid={txid:?}, wtxid={wtxid:?} conflicts with a prior pair"
            ),
            Self::MissingFinalMembership { member } => {
                write!(
                    formatter,
                    "mempool lifecycle member {member:?} has no final membership"
                )
            }
        }
    }
}

impl std::error::Error for MempoolLifecycleInvariantError {}

/// Cache-agnostic facts produced by one committed mempool transition.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MempoolLifecycleDelta {
    /// Admitted members in supplied transition or topological order.
    pub admitted: Vec<MempoolMemberIdentity>,
    /// Deterministically ordered, identity-deduplicated removals.
    pub removed: Vec<MempoolLifecycleRemoval>,
    /// Deterministically ordered final state for every affected member.
    pub final_membership: Vec<MempoolMemberState>,
    /// Deterministically ordered, identity-deduplicated retry-clear facts.
    pub retry_clears: Vec<MempoolRetryClear>,
}

impl MempoolLifecycleDelta {
    /// Starts a checked deterministic delta builder.
    pub fn builder() -> MempoolLifecycleDeltaBuilder {
        MempoolLifecycleDeltaBuilder::default()
    }

    /// Returns an empty delta for an attempt that committed no membership change.
    pub const fn empty() -> Self {
        Self {
            admitted: Vec::new(),
            removed: Vec::new(),
            final_membership: Vec::new(),
            retry_clears: Vec::new(),
        }
    }

    /// Returns whether the attempt committed no lifecycle facts.
    pub fn is_empty(&self) -> bool {
        self.admitted.is_empty()
            && self.removed.is_empty()
            && self.final_membership.is_empty()
            && self.retry_clears.is_empty()
    }
}

/// Checked assembler for deterministic lifecycle facts.
#[derive(Debug, Default)]
pub struct MempoolLifecycleDeltaBuilder {
    admitted: Vec<MempoolMemberIdentity>,
    admitted_seen: BTreeSet<MempoolMemberIdentity>,
    removed: BTreeMap<MempoolMemberIdentity, MempoolLifecycleRemoval>,
    final_membership: BTreeMap<MempoolMemberIdentity, MempoolMemberState>,
    retry_clears: BTreeMap<MempoolMemberIdentity, MempoolRetryClear>,
    txid_identities: BTreeMap<Txid, MempoolMemberIdentity>,
    wtxid_identities: BTreeMap<Wtxid, MempoolMemberIdentity>,
}

impl MempoolLifecycleDeltaBuilder {
    /// Records an admitted identity while preserving first-supplied order.
    pub fn record_admitted(
        &mut self,
        member: MempoolMemberIdentity,
    ) -> Result<(), MempoolLifecycleInvariantError> {
        self.record_identity(member)?;
        if self.admitted_seen.insert(member) {
            self.admitted.push(member);
        }
        Ok(())
    }

    /// Records one removal using stable cause precedence and direct-role precedence.
    pub fn record_removal(
        &mut self,
        removal: MempoolLifecycleRemoval,
    ) -> Result<(), MempoolLifecycleInvariantError> {
        let member = removal.member;
        self.record_identity(member)?;
        self.removed
            .entry(member)
            .and_modify(|existing| {
                if removal.cause.priority() < existing.cause.priority() {
                    existing.cause = removal.cause;
                }
                if removal.role == MempoolRemovalRole::Direct {
                    existing.role = MempoolRemovalRole::Direct;
                }
            })
            .or_insert(removal);
        Ok(())
    }

    /// Records one final state; absent wins contradictory duplicates deterministically.
    pub fn record_final_membership(
        &mut self,
        state: MempoolMemberState,
    ) -> Result<(), MempoolLifecycleInvariantError> {
        self.record_identity(state.member)?;
        self.final_membership
            .entry(state.member)
            .and_modify(|existing| {
                if state.membership == FinalMempoolMembership::Absent {
                    existing.membership = FinalMempoolMembership::Absent;
                }
            })
            .or_insert(state);
        Ok(())
    }

    /// Records one retry-clear fact with LifecycleRemoval > TransportWritten > EligibleServe.
    pub fn record_retry_clear(
        &mut self,
        clear: MempoolRetryClear,
    ) -> Result<(), MempoolLifecycleInvariantError> {
        self.record_identity(clear.member)?;
        self.retry_clears
            .entry(clear.member)
            .and_modify(|existing| {
                if clear.cause.priority() < existing.cause.priority() {
                    existing.cause = clear.cause;
                }
            })
            .or_insert(clear);
        Ok(())
    }

    /// Validates completeness and emits deterministic collections.
    pub fn build(self) -> Result<MempoolLifecycleDelta, MempoolLifecycleInvariantError> {
        for member in self
            .admitted
            .iter()
            .copied()
            .chain(self.removed.keys().copied())
            .chain(self.retry_clears.keys().copied())
        {
            if !self.final_membership.contains_key(&member) {
                return Err(MempoolLifecycleInvariantError::MissingFinalMembership { member });
            }
        }

        Ok(MempoolLifecycleDelta {
            admitted: self.admitted,
            removed: self.removed.into_values().collect(),
            final_membership: self.final_membership.into_values().collect(),
            retry_clears: self.retry_clears.into_values().collect(),
        })
    }

    fn record_identity(
        &mut self,
        identity: MempoolMemberIdentity,
    ) -> Result<(), MempoolLifecycleInvariantError> {
        if let Some(conflicting) = self.txid_identities.get(&identity.txid).copied()
            && conflicting != identity
        {
            return Err(MempoolLifecycleInvariantError::IdentityConflict {
                txid: identity.txid,
                wtxid: identity.wtxid,
            });
        }
        if let Some(conflicting) = self.wtxid_identities.get(&identity.wtxid).copied()
            && conflicting != identity
        {
            return Err(MempoolLifecycleInvariantError::IdentityConflict {
                txid: identity.txid,
                wtxid: identity.wtxid,
            });
        }

        self.txid_identities.insert(identity.txid, identity);
        self.wtxid_identities.insert(identity.wtxid, identity);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolCapacityStatus {
    Empty,
    UnderCapacity,
    AtCapacity,
    OverCapacity,
}

impl MempoolCapacityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::UnderCapacity => "under_capacity",
            Self::AtCapacity => "at_capacity",
            Self::OverCapacity => "over_capacity",
        }
    }
}

/// Truthful capacity-enforcement evidence for operators.
///
/// Capacity enforcement uses accounted memory against `MempoolCapacity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolCapacityEnforcement {
    AccountedMemory,
}

impl MempoolCapacityEnforcement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountedMemory => "accounted_memory",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollingFeeParityStatus {
    Active,
}

impl RollingFeeParityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolPressureSummary {
    pub transaction_count: usize,
    pub total_virtual_size: TransactionVirtualSize,
    pub accounted_memory: AccountedMempoolMemory,
    pub mempool_capacity: MempoolCapacity,
    pub static_relay_fee_rate: StaticRelayFeeRate,
    pub incremental_relay_fee_rate: IncrementalRelayFeeRate,
    pub rolling_mempool_fee_rate: RollingMempoolFeeRate,
    pub effective_admission_fee_rate: EffectiveAdmissionFeeRate,
    pub capacity_status: MempoolCapacityStatus,
    pub capacity_enforcement: MempoolCapacityEnforcement,
    pub rolling_fee_parity: RollingFeeParityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolLifecycleRemoval {
    pub member: MempoolMemberIdentity,
    pub cause: MempoolRemovalCause,
    pub role: MempoolRemovalRole,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolLifecycleSummary {
    pub removed: Vec<MempoolLifecycleRemoval>,
    pub pressure: MempoolPressureSummary,
}

impl Mempool {
    pub fn pressure_summary(&self) -> MempoolPressureSummary {
        let capacity_status =
            capacity_status(self.accounted_memory(), self.config.mempool_capacity);
        let effective_admission_fee_rate = effective_admission_fee_rate(
            self.config.static_relay_fee_rate,
            self.rolling_mempool_fee_rate(),
        );
        MempoolPressureSummary {
            transaction_count: self.entries.len(),
            total_virtual_size: self.total_virtual_size(),
            accounted_memory: self.accounted_memory(),
            mempool_capacity: self.config.mempool_capacity,
            static_relay_fee_rate: self.config.static_relay_fee_rate,
            incremental_relay_fee_rate: self.config.incremental_relay_fee_rate,
            rolling_mempool_fee_rate: self.rolling_mempool_fee_rate(),
            effective_admission_fee_rate,
            capacity_status,
            capacity_enforcement: MempoolCapacityEnforcement::AccountedMemory,
            rolling_fee_parity: RollingFeeParityStatus::Active,
        }
    }

    /// Prepares block-confirmed and conflicting member removal without mutation.
    pub fn prepare_connected_block_transition(
        &self,
        block: &Block,
        context: BlockLifecycleContext,
    ) -> Result<PreparedMempoolTransition, MempoolError> {
        let removals = self.connected_transaction_removals(block.transactions.iter())?;
        let mut delta_builder = MempoolLifecycleDelta::builder();
        for (txid, entry, fact) in removals
            .iter()
            .filter_map(|(txid, fact)| self.entries.get(txid).map(|entry| (*txid, entry, *fact)))
        {
            record_delta_removal(
                &mut delta_builder,
                MempoolMemberIdentity {
                    txid,
                    wtxid: entry.wtxid,
                },
                fact,
            )?;
        }
        let delta = delta_builder.build().map_err(lifecycle_invariant_error)?;

        // Knots `removeForBlock`: open decay gate even when the block removes nothing.
        let mut rolling_fee_state = self.rolling_fee_state.clone();
        rolling_fee_state.open_decay_gate_after_block(context.connected_at);
        if removals.is_empty() && rolling_fee_state == self.rolling_fee_state {
            return Ok(PreparedMempoolTransition::maintenance_noop(self));
        }
        let patch = prepare_removal_patch(
            self,
            removals.into_keys().collect(),
            rolling_fee_state,
            delta,
        )?;
        PreparedMempoolTransition::maintenance_from_patch(self, patch)
    }

    /// Removes block-confirmed and conflicting members through the prepared facade.
    pub fn remove_for_connected_block_transition(
        &mut self,
        block: &Block,
        context: BlockLifecycleContext,
    ) -> Result<MempoolLifecycleDelta, MempoolError> {
        let prepared = self.prepare_connected_block_transition(block, context)?;
        let validated = self.validate_prepared_mempool_transition(prepared)?;
        Ok(self.apply_validated_mempool_transition(validated))
    }

    fn connected_transaction_removals<'a>(
        &self,
        transactions: impl IntoIterator<Item = &'a Transaction>,
    ) -> Result<BTreeMap<Txid, MempoolRemovalFact>, MempoolError> {
        let mut removals = BTreeMap::new();

        for transaction in transactions {
            let txid = transaction_txid(transaction).map_err(txid_serialization_error)?;
            if self.entries.contains_key(&txid) {
                record_removal_fact(
                    &mut removals,
                    txid,
                    MempoolRemovalFact {
                        cause: MempoolRemovalCause::BlockConfirmation,
                        role: MempoolRemovalRole::Direct,
                    },
                );
            }

            let mut direct_conflicts = self.direct_conflicts(transaction);
            direct_conflicts.remove(&txid);
            let conflict_package =
                collect_conflicts_and_descendants(&self.entries, &direct_conflicts);
            for conflict_txid in direct_conflicts {
                record_removal_fact(
                    &mut removals,
                    conflict_txid,
                    MempoolRemovalFact {
                        cause: MempoolRemovalCause::BlockConflict,
                        role: MempoolRemovalRole::Direct,
                    },
                );
            }
            for package_txid in conflict_package {
                record_removal_fact(
                    &mut removals,
                    package_txid,
                    MempoolRemovalFact {
                        cause: MempoolRemovalCause::BlockConflict,
                        role: MempoolRemovalRole::Descendant,
                    },
                );
            }
        }

        Ok(removals)
    }
}

pub(super) fn capacity_status(
    accounted_memory: AccountedMempoolMemory,
    mempool_capacity: MempoolCapacity,
) -> MempoolCapacityStatus {
    if accounted_memory == AccountedMempoolMemory::ZERO {
        return MempoolCapacityStatus::Empty;
    }
    if accounted_memory.as_usize() < mempool_capacity.as_usize() {
        return MempoolCapacityStatus::UnderCapacity;
    }
    if accounted_memory.as_usize() == mempool_capacity.as_usize() {
        return MempoolCapacityStatus::AtCapacity;
    }

    MempoolCapacityStatus::OverCapacity
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MempoolRemovalFact {
    pub cause: MempoolRemovalCause,
    pub role: MempoolRemovalRole,
}

pub(super) fn record_removal_fact(
    removals: &mut BTreeMap<Txid, MempoolRemovalFact>,
    txid: Txid,
    fact: MempoolRemovalFact,
) {
    removals
        .entry(txid)
        .and_modify(|existing| {
            if fact.cause.priority() < existing.cause.priority() {
                existing.cause = fact.cause;
            }
            if fact.role == MempoolRemovalRole::Direct {
                existing.role = MempoolRemovalRole::Direct;
            }
        })
        .or_insert(fact);
}

pub(super) fn txid_serialization_error(source: CodecError) -> MempoolError {
    super::serialization_validation_error("transaction txid", source)
}

fn record_delta_removal(
    builder: &mut MempoolLifecycleDeltaBuilder,
    member: MempoolMemberIdentity,
    fact: MempoolRemovalFact,
) -> Result<(), MempoolError> {
    builder
        .record_removal(MempoolLifecycleRemoval {
            member,
            cause: fact.cause,
            role: fact.role,
        })
        .map_err(lifecycle_invariant_error)?;
    builder
        .record_final_membership(MempoolMemberState {
            member,
            membership: FinalMempoolMembership::Absent,
        })
        .map_err(lifecycle_invariant_error)?;
    builder
        .record_retry_clear(MempoolRetryClear {
            member,
            cause: MempoolRetryClearCause::LifecycleRemoval,
        })
        .map_err(lifecycle_invariant_error)
}

pub(super) fn lifecycle_invariant_error(source: MempoolLifecycleInvariantError) -> MempoolError {
    MempoolError::InternalInvariant {
        reason: source.to_string(),
    }
}
