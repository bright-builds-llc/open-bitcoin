// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Bounded exact-identity ledger for mempool checkpoint effects.

use std::collections::{BTreeSet, VecDeque};

use open_bitcoin_mempool::PolicyTime;

use super::{
    EffectAbort, EffectPreparationError, ExactEffectLedgerCompletion,
    MAX_COMPLETED_SNAPSHOT_EFFECTS, MAX_PENDING_SNAPSHOT_EFFECTS, SnapshotEffectId,
    SnapshotIdentity,
};
use crate::network::lifecycle_projection::{AuthorityEpoch, LifecycleGeneration};
use crate::storage::MempoolSnapshot;

/// The authority-owned reason one mempool checkpoint was captured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CheckpointTrigger {
    Periodic,
    Shutdown,
}

/// The durability boundary achieved by one completed checkpoint write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointPersistenceStrength {
    Sync,
}

/// Low-cardinality failure truth for one checkpoint attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotWriteFailure {
    Encode,
    Storage,
    AbortDispatch,
}

/// Why a requested pre-achievement abort could not be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotWriteAbortError {
    AchievedStateOnlyFailure,
}

impl std::fmt::Display for SnapshotWriteAbortError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an achieved-state-only failure cannot abort an unachieved write")
    }
}

impl std::error::Error for SnapshotWriteAbortError {}

/// One owned current-schema mempool snapshot plus its success capability.
/// The outside-lock executor may create a receipt only after persistence succeeds.
#[derive(Debug, PartialEq, Eq)]
pub struct PreparedSnapshotWrite {
    snapshot: MempoolSnapshot,
    capability: SnapshotWriteCapability,
}

impl PreparedSnapshotWrite {
    pub(in crate::network) const fn new(
        authority_epoch: AuthorityEpoch,
        persistence_generation: LifecycleGeneration,
        captured_at: PolicyTime,
        trigger: CheckpointTrigger,
        effect_id: SnapshotEffectId,
        snapshot_identity: SnapshotIdentity,
        snapshot: MempoolSnapshot,
    ) -> Self {
        Self {
            snapshot,
            capability: SnapshotWriteCapability {
                authority_epoch,
                persistence_generation,
                captured_at,
                trigger,
                effect_id,
                snapshot_identity,
            },
        }
    }

    pub fn snapshot(&self) -> &MempoolSnapshot {
        &self.snapshot
    }

    pub fn into_parts(self) -> (MempoolSnapshot, SnapshotWriteCapability) {
        (self.snapshot, self.capability)
    }

    pub const fn checkpoint_trigger(&self) -> CheckpointTrigger {
        self.capability.trigger
    }
}

/// A consuming capability that can acknowledge one exact successful snapshot write.
#[derive(Debug, PartialEq, Eq)]
pub struct SnapshotWriteCapability {
    authority_epoch: AuthorityEpoch,
    persistence_generation: LifecycleGeneration,
    captured_at: PolicyTime,
    trigger: CheckpointTrigger,
    effect_id: SnapshotEffectId,
    snapshot_identity: SnapshotIdentity,
}

impl SnapshotWriteCapability {
    pub(in crate::network) const fn persistence_generation(&self) -> LifecycleGeneration {
        self.persistence_generation
    }

    pub fn acknowledge_write(
        self,
        completed_at: PolicyTime,
        strength: CheckpointPersistenceStrength,
    ) -> SnapshotWriteReceipt {
        SnapshotWriteReceipt {
            authority_epoch: self.authority_epoch,
            persistence_generation: self.persistence_generation,
            captured_at: self.captured_at,
            trigger: self.trigger,
            maybe_completed_at: Some(completed_at),
            maybe_strength: Some(strength),
            effect_id: self.effect_id,
            snapshot_identity: self.snapshot_identity,
        }
    }
}

/// One exact pre-achievement checkpoint termination.
#[derive(Debug, PartialEq, Eq)]
pub struct SnapshotWriteAbort {
    capability: SnapshotWriteCapability,
    failed_at: PolicyTime,
    failure: SnapshotWriteFailure,
}

impl SnapshotWriteAbort {
    pub fn new(
        capability: SnapshotWriteCapability,
        failed_at: PolicyTime,
        failure: SnapshotWriteFailure,
    ) -> Result<Self, SnapshotWriteAbortError> {
        if failure == SnapshotWriteFailure::AbortDispatch {
            return Err(SnapshotWriteAbortError::AchievedStateOnlyFailure);
        }
        Ok(Self {
            capability,
            failed_at,
            failure,
        })
    }

    pub(in crate::network) const fn capability(&self) -> &SnapshotWriteCapability {
        &self.capability
    }

    pub(in crate::network) const fn failed_at(&self) -> PolicyTime {
        self.failed_at
    }

    pub(in crate::network) const fn failure(&self) -> SnapshotWriteFailure {
        self.failure
    }
}

/// Proof that one exact current-schema snapshot write succeeded.
#[derive(Debug, PartialEq, Eq)]
pub struct SnapshotWriteReceipt {
    authority_epoch: AuthorityEpoch,
    persistence_generation: LifecycleGeneration,
    captured_at: PolicyTime,
    trigger: CheckpointTrigger,
    maybe_completed_at: Option<PolicyTime>,
    maybe_strength: Option<CheckpointPersistenceStrength>,
    effect_id: SnapshotEffectId,
    snapshot_identity: SnapshotIdentity,
}

impl SnapshotWriteReceipt {
    pub const fn captured_generation(&self) -> u64 {
        self.persistence_generation.raw()
    }

    pub const fn captured_at(&self) -> PolicyTime {
        self.captured_at
    }

    pub const fn checkpoint_trigger(&self) -> CheckpointTrigger {
        self.trigger
    }

    pub const fn completed_at(&self) -> Option<PolicyTime> {
        self.maybe_completed_at
    }

    pub const fn persistence_strength(&self) -> Option<CheckpointPersistenceStrength> {
        self.maybe_strength
    }

    pub(in crate::network) const fn authority_epoch(&self) -> AuthorityEpoch {
        self.authority_epoch
    }

    pub(in crate::network) const fn persistence_generation(&self) -> LifecycleGeneration {
        self.persistence_generation
    }

    #[cfg(test)]
    pub(in crate::network) const fn effect_id(&self) -> SnapshotEffectId {
        self.effect_id
    }

    #[cfg(test)]
    pub(in crate::network) const fn snapshot_identity(&self) -> SnapshotIdentity {
        self.snapshot_identity
    }

    #[cfg(test)]
    pub(crate) const fn duplicate_for_test(&self) -> Self {
        Self {
            authority_epoch: self.authority_epoch,
            persistence_generation: self.persistence_generation,
            captured_at: self.captured_at,
            trigger: self.trigger,
            maybe_completed_at: self.maybe_completed_at,
            maybe_strength: self.maybe_strength,
            effect_id: self.effect_id,
            snapshot_identity: self.snapshot_identity,
        }
    }

    pub(in crate::network) const fn exact_key(&self) -> SnapshotEffectKey {
        SnapshotEffectKey {
            authority_incarnation: self.authority_epoch,
            reserved_generation: self.persistence_generation,
            captured_at: self.captured_at,
            trigger: self.trigger,
            reserved_effect: self.effect_id,
            reserved_snapshot: self.snapshot_identity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::network) struct SnapshotEffectKey {
    authority_incarnation: AuthorityEpoch,
    reserved_generation: LifecycleGeneration,
    captured_at: PolicyTime,
    trigger: CheckpointTrigger,
    reserved_effect: SnapshotEffectId,
    reserved_snapshot: SnapshotIdentity,
}

impl From<&SnapshotWriteCapability> for SnapshotEffectKey {
    fn from(capability: &SnapshotWriteCapability) -> Self {
        Self {
            authority_incarnation: capability.authority_epoch,
            reserved_generation: capability.persistence_generation,
            captured_at: capability.captured_at,
            trigger: capability.trigger,
            reserved_effect: capability.effect_id,
            reserved_snapshot: capability.snapshot_identity,
        }
    }
}

impl From<&SnapshotWriteReceipt> for SnapshotEffectKey {
    fn from(receipt: &SnapshotWriteReceipt) -> Self {
        receipt.exact_key()
    }
}

#[derive(Debug, Clone, Default)]
pub(in crate::network) struct SnapshotEffectLedger {
    pending: BTreeSet<SnapshotEffectKey>,
    completed_order: VecDeque<SnapshotEffectId>,
    completed: BTreeSet<SnapshotEffectKey>,
    next_id: u64,
}

impl SnapshotEffectLedger {
    pub(in crate::network) fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    pub(in crate::network) fn reserve_next(
        &mut self,
        authority_epoch: AuthorityEpoch,
        persistence_generation: LifecycleGeneration,
        captured_at: PolicyTime,
        trigger: CheckpointTrigger,
        snapshot: MempoolSnapshot,
    ) -> Result<PreparedSnapshotWrite, EffectPreparationError> {
        let effect_id = SnapshotEffectId::new(self.next_id);
        let next_id = self
            .next_id
            .checked_add(1)
            .ok_or(EffectPreparationError::EffectIdentityExhausted)?;
        let prepared = PreparedSnapshotWrite::new(
            authority_epoch,
            persistence_generation,
            captured_at,
            trigger,
            effect_id,
            SnapshotIdentity::from_effect_id(effect_id),
            snapshot,
        );
        self.try_reserve_key(SnapshotEffectKey::from(&prepared.capability))?;
        self.next_id = next_id;
        Ok(prepared)
    }

    fn try_reserve_key(&mut self, key: SnapshotEffectKey) -> Result<(), EffectPreparationError> {
        if self.pending.len() >= MAX_PENDING_SNAPSHOT_EFFECTS {
            return Err(EffectPreparationError::SnapshotEffectPending);
        }
        if self.pending.contains(&key) || self.completed.contains(&key) {
            return Err(EffectPreparationError::EffectIdentityCollision);
        }
        self.pending.insert(key);
        Ok(())
    }

    pub(in crate::network) fn complete_exact(
        &mut self,
        receipt: &SnapshotWriteReceipt,
    ) -> ExactEffectLedgerCompletion {
        let key = SnapshotEffectKey::from(receipt);
        if self.completed.contains(&key) {
            return ExactEffectLedgerCompletion::AlreadyRecorded;
        }
        if !self.pending.remove(&key) {
            return ExactEffectLedgerCompletion::NotPending;
        }
        self.record_completed_key(key);
        ExactEffectLedgerCompletion::Recorded
    }

    pub(in crate::network) fn abort_exact(
        &mut self,
        capability: &SnapshotWriteCapability,
    ) -> EffectAbort {
        let key = SnapshotEffectKey::from(capability);
        if self.completed.contains(&key) {
            return EffectAbort::AlreadyCompleted;
        }
        if self.pending.remove(&key) {
            return EffectAbort::Aborted;
        }
        EffectAbort::NotPending
    }

    fn record_completed_key(&mut self, key: SnapshotEffectKey) {
        if !self.completed.insert(key) {
            return;
        }
        self.completed_order.push_back(key.reserved_effect);
        if self.completed_order.len() <= MAX_COMPLETED_SNAPSHOT_EFFECTS {
            return;
        }
        let Some(evicted) = self.completed_order.pop_front() else {
            return;
        };
        let maybe_evicted_key = self
            .completed
            .iter()
            .find(|key| key.reserved_effect == evicted)
            .copied();
        if let Some(evicted_key) = maybe_evicted_key {
            self.completed.remove(&evicted_key);
        }
    }

    pub(in crate::network) fn is_completed(&self, key: SnapshotEffectKey) -> bool {
        self.completed.contains(&key)
    }

    #[cfg(test)]
    pub(in crate::network) fn try_reserve_for_test(
        &mut self,
        prepared: PreparedSnapshotWrite,
    ) -> Result<(), EffectPreparationError> {
        self.try_reserve_key(SnapshotEffectKey::from(&prepared.capability))
    }

    #[cfg(test)]
    pub(in crate::network) fn record_completed_for_test(&mut self, receipt: &SnapshotWriteReceipt) {
        self.record_completed_key(SnapshotEffectKey::from(receipt));
    }

    #[cfg(test)]
    pub(in crate::network) fn is_completed_exact(&self, receipt: &SnapshotWriteReceipt) -> bool {
        self.completed.contains(&SnapshotEffectKey::from(receipt))
    }

    #[cfg(test)]
    pub(in crate::network) fn pending_len(&self) -> usize {
        self.pending.len()
    }

    #[cfg(test)]
    pub(in crate::network) fn completed_len(&self) -> usize {
        self.completed.len()
    }
}
