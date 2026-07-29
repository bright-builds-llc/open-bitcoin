// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Family-specific identities and bounded ledgers for lock-free lifecycle effects.

use std::collections::{BTreeSet, VecDeque};

use open_bitcoin_network::{PHASE94_MAX_PEER_QUEUED_MESSAGES, PeerId};

use super::lifecycle_projection::{AuthorityEpoch, LifecycleGeneration};
use crate::storage::MempoolSnapshot;

pub const MAX_PENDING_PEER_EFFECTS: usize = PHASE94_MAX_PEER_QUEUED_MESSAGES;
pub const MAX_COMPLETED_PEER_EFFECTS: usize = PHASE94_MAX_PEER_QUEUED_MESSAGES;
pub const MAX_PENDING_SNAPSHOT_EFFECTS: usize = 1;
pub const MAX_COMPLETED_SNAPSHOT_EFFECTS: usize = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectCompletion {
    Applied,
    AchievedButStale,
    AlreadyApplied,
}

/// Classification for a known pre-achievement effect termination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectAbort {
    /// The exact pending reservation was released.
    Aborted,
    /// The exact effect was already recorded as achieved.
    AlreadyCompleted,
    /// No pending reservation matched the complete immutable binding.
    NotPending,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::network) enum EffectPreparationError {
    PeerEffectsAtCapacity,
    SnapshotEffectPending,
    EffectIdentityCollision,
    EffectIdentityExhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::network) enum ExactEffectLedgerCompletion {
    Recorded,
    AlreadyRecorded,
    NotPending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerEffectId(u64);

impl PeerEffectId {
    pub(in crate::network) const fn new(raw: u64) -> Self {
        Self(raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SnapshotEffectId(u64);

impl SnapshotEffectId {
    pub(in crate::network) const fn new(raw: u64) -> Self {
        Self(raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeerSessionGeneration(u64);

impl PeerSessionGeneration {
    pub const INITIAL: Self = Self(0);

    #[cfg(test)]
    pub(in crate::network) const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub(in crate::network) fn checked_next(self) -> Result<Self, EffectPreparationError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(EffectPreparationError::EffectIdentityExhausted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SnapshotIdentity(u64);

impl SnapshotIdentity {
    #[cfg(test)]
    pub(in crate::network) const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub(in crate::network) const fn from_effect_id(effect_id: SnapshotEffectId) -> Self {
        Self(effect_id.0)
    }
}

/// A consuming capability that can acknowledge one exact successful peer write.
///
/// ```compile_fail
/// fn replay(capability: open_bitcoin_node::network::PeerEffectCapability) {
///     let _duplicate = capability.clone();
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct PeerEffectCapability {
    authority_epoch: AuthorityEpoch,
    lifecycle_generation: LifecycleGeneration,
    effect_id: PeerEffectId,
    peer_id: PeerId,
    peer_session_generation: PeerSessionGeneration,
}

impl PeerEffectCapability {
    pub(in crate::network) const fn new(
        authority_epoch: AuthorityEpoch,
        lifecycle_generation: LifecycleGeneration,
        effect_id: PeerEffectId,
        peer_id: PeerId,
        peer_session_generation: PeerSessionGeneration,
    ) -> Self {
        Self {
            authority_epoch,
            lifecycle_generation,
            effect_id,
            peer_id,
            peer_session_generation,
        }
    }

    pub fn acknowledge_write(self) -> PeerEffectReceipt {
        PeerEffectReceipt {
            authority_epoch: self.authority_epoch,
            lifecycle_generation: self.lifecycle_generation,
            effect_id: self.effect_id,
            peer_id: self.peer_id,
            peer_session_generation: self.peer_session_generation,
        }
    }

    pub const fn peer_id(&self) -> PeerId {
        self.peer_id
    }
    pub(in crate::network) const fn authority_epoch(&self) -> AuthorityEpoch {
        self.authority_epoch
    }
}

/// Proof that one exact peer write succeeded outside the authority lock.
///
/// ```compile_fail
/// fn replay(receipt: open_bitcoin_node::network::PeerEffectReceipt) {
///     let _duplicate = receipt.clone();
/// }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct PeerEffectReceipt {
    authority_epoch: AuthorityEpoch,
    lifecycle_generation: LifecycleGeneration,
    effect_id: PeerEffectId,
    peer_id: PeerId,
    peer_session_generation: PeerSessionGeneration,
}

impl PeerEffectReceipt {
    pub(in crate::network) const fn authority_epoch(&self) -> AuthorityEpoch {
        self.authority_epoch
    }

    pub(in crate::network) const fn lifecycle_generation(&self) -> LifecycleGeneration {
        self.lifecycle_generation
    }

    #[cfg(test)]
    pub(in crate::network) const fn effect_id(&self) -> PeerEffectId {
        self.effect_id
    }

    pub const fn peer_id(&self) -> PeerId {
        self.peer_id
    }
    pub(in crate::network) const fn peer_session_generation(&self) -> PeerSessionGeneration {
        self.peer_session_generation
    }

    #[cfg(test)]
    pub(in crate::network) const fn duplicate_for_test(&self) -> Self {
        Self {
            authority_epoch: self.authority_epoch,
            lifecycle_generation: self.lifecycle_generation,
            effect_id: self.effect_id,
            peer_id: self.peer_id,
            peer_session_generation: self.peer_session_generation,
        }
    }

    pub(in crate::network) const fn exact_key(&self) -> PeerEffectKey {
        PeerEffectKey {
            authority_incarnation: self.authority_epoch,
            reserved_generation: self.lifecycle_generation,
            reserved_effect: self.effect_id,
            target_peer: self.peer_id,
            target_session: self.peer_session_generation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::network) struct PeerEffectKey {
    authority_incarnation: AuthorityEpoch,
    reserved_generation: LifecycleGeneration,
    reserved_effect: PeerEffectId,
    target_peer: PeerId,
    target_session: PeerSessionGeneration,
}

impl From<&PeerEffectCapability> for PeerEffectKey {
    fn from(capability: &PeerEffectCapability) -> Self {
        Self {
            authority_incarnation: capability.authority_epoch,
            reserved_generation: capability.lifecycle_generation,
            reserved_effect: capability.effect_id,
            target_peer: capability.peer_id,
            target_session: capability.peer_session_generation,
        }
    }
}

impl From<&PeerEffectReceipt> for PeerEffectKey {
    fn from(receipt: &PeerEffectReceipt) -> Self {
        receipt.exact_key()
    }
}

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
        effect_id: SnapshotEffectId,
        snapshot_identity: SnapshotIdentity,
        snapshot: MempoolSnapshot,
    ) -> Self {
        Self {
            snapshot,
            capability: SnapshotWriteCapability {
                authority_epoch,
                persistence_generation,
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
}

/// A consuming capability that can acknowledge one exact successful snapshot write.
#[derive(Debug, PartialEq, Eq)]
pub struct SnapshotWriteCapability {
    authority_epoch: AuthorityEpoch,
    persistence_generation: LifecycleGeneration,
    effect_id: SnapshotEffectId,
    snapshot_identity: SnapshotIdentity,
}

impl SnapshotWriteCapability {
    pub fn acknowledge_write(self) -> SnapshotWriteReceipt {
        SnapshotWriteReceipt {
            authority_epoch: self.authority_epoch,
            persistence_generation: self.persistence_generation,
            effect_id: self.effect_id,
            snapshot_identity: self.snapshot_identity,
        }
    }
}

/// Proof that one exact current-schema snapshot write succeeded.
#[derive(Debug, PartialEq, Eq)]
pub struct SnapshotWriteReceipt {
    authority_epoch: AuthorityEpoch,
    persistence_generation: LifecycleGeneration,
    effect_id: SnapshotEffectId,
    snapshot_identity: SnapshotIdentity,
}

impl SnapshotWriteReceipt {
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
    pub(in crate::network) const fn duplicate_for_test(&self) -> Self {
        Self {
            authority_epoch: self.authority_epoch,
            persistence_generation: self.persistence_generation,
            effect_id: self.effect_id,
            snapshot_identity: self.snapshot_identity,
        }
    }

    pub(in crate::network) const fn exact_key(&self) -> SnapshotEffectKey {
        SnapshotEffectKey {
            authority_incarnation: self.authority_epoch,
            reserved_generation: self.persistence_generation,
            reserved_effect: self.effect_id,
            reserved_snapshot: self.snapshot_identity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::network) struct SnapshotEffectKey {
    authority_incarnation: AuthorityEpoch,
    reserved_generation: LifecycleGeneration,
    reserved_effect: SnapshotEffectId,
    reserved_snapshot: SnapshotIdentity,
}

impl From<&SnapshotWriteCapability> for SnapshotEffectKey {
    fn from(capability: &SnapshotWriteCapability) -> Self {
        Self {
            authority_incarnation: capability.authority_epoch,
            reserved_generation: capability.persistence_generation,
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
pub(in crate::network) struct PeerEffectLedger {
    pending: BTreeSet<PeerEffectKey>,
    completed_order: VecDeque<PeerEffectId>,
    completed: BTreeSet<PeerEffectKey>,
    next_id: u64,
}

impl PeerEffectLedger {
    pub(in crate::network) fn reserve_next(
        &mut self,
        authority_epoch: AuthorityEpoch,
        lifecycle_generation: LifecycleGeneration,
        peer_id: PeerId,
        peer_session_generation: PeerSessionGeneration,
    ) -> Result<PeerEffectCapability, EffectPreparationError> {
        let effect_id = PeerEffectId::new(self.next_id);
        let next_id = self
            .next_id
            .checked_add(1)
            .ok_or(EffectPreparationError::EffectIdentityExhausted)?;
        let capability = PeerEffectCapability::new(
            authority_epoch,
            lifecycle_generation,
            effect_id,
            peer_id,
            peer_session_generation,
        );
        self.try_reserve_key(PeerEffectKey::from(&capability))?;
        self.next_id = next_id;
        Ok(capability)
    }

    fn try_reserve_key(&mut self, key: PeerEffectKey) -> Result<(), EffectPreparationError> {
        if self.pending.len() >= MAX_PENDING_PEER_EFFECTS {
            return Err(EffectPreparationError::PeerEffectsAtCapacity);
        }
        if self.pending.contains(&key) || self.completed.contains(&key) {
            return Err(EffectPreparationError::EffectIdentityCollision);
        }
        self.pending.insert(key);
        Ok(())
    }

    pub(in crate::network) fn complete_exact(
        &mut self,
        receipt: &PeerEffectReceipt,
    ) -> ExactEffectLedgerCompletion {
        let key = PeerEffectKey::from(receipt);
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
        capability: &PeerEffectCapability,
    ) -> EffectAbort {
        let key = PeerEffectKey::from(capability);
        if self.completed.contains(&key) {
            return EffectAbort::AlreadyCompleted;
        }
        if self.pending.remove(&key) {
            return EffectAbort::Aborted;
        }
        EffectAbort::NotPending
    }
    pub(in crate::network) fn is_pending(&self, receipt: &PeerEffectReceipt) -> bool {
        self.pending.contains(&PeerEffectKey::from(receipt))
    }

    fn record_completed_key(&mut self, key: PeerEffectKey) {
        if !self.completed.insert(key) {
            return;
        }
        self.completed_order.push_back(key.reserved_effect);
        if self.completed_order.len() <= MAX_COMPLETED_PEER_EFFECTS {
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

    pub(in crate::network) fn is_completed(&self, key: PeerEffectKey) -> bool {
        self.completed.contains(&key)
    }

    pub(in crate::network) fn has_pending_for_peer(&self, peer_id: PeerId) -> bool {
        self.pending.iter().any(|key| key.target_peer == peer_id)
    }

    #[cfg(test)]
    pub(in crate::network) fn try_reserve_for_test(
        &mut self,
        capability: PeerEffectCapability,
    ) -> Result<(), EffectPreparationError> {
        self.try_reserve_key(PeerEffectKey::from(&capability))
    }

    #[cfg(test)]
    pub(in crate::network) fn record_completed_for_test(&mut self, receipt: &PeerEffectReceipt) {
        self.record_completed_key(PeerEffectKey::from(receipt));
    }

    #[cfg(test)]
    pub(in crate::network) fn is_completed_exact(&self, receipt: &PeerEffectReceipt) -> bool {
        self.completed.contains(&PeerEffectKey::from(receipt))
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

#[derive(Debug, Clone, Default)]
pub(in crate::network) struct SnapshotEffectLedger {
    pending: BTreeSet<SnapshotEffectKey>,
    completed_order: VecDeque<SnapshotEffectId>,
    completed: BTreeSet<SnapshotEffectKey>,
    next_id: u64,
}

impl SnapshotEffectLedger {
    pub(in crate::network) fn reserve_next(
        &mut self,
        authority_epoch: AuthorityEpoch,
        persistence_generation: LifecycleGeneration,
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
