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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::network) enum EffectPreparationError {
    PeerEffectsAtCapacity,
    SnapshotEffectPending,
    EffectIdentityCollision,
    EffectIdentityExhausted,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerSessionGeneration(u64);

impl PeerSessionGeneration {
    pub const INITIAL: Self = Self(0);

    #[cfg(test)]
    pub(in crate::network) const fn new(raw: u64) -> Self {
        Self(raw)
    }

    #[cfg(test)]
    pub(in crate::network) fn checked_next(self) -> Result<Self, EffectPreparationError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(EffectPreparationError::EffectIdentityExhausted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

/// One owned current-schema mempool snapshot plus its success capability.
///
/// The write command is intentionally non-`Clone`; the outside-lock executor
/// must consume it and may create a receipt only after persistence succeeds.
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
///
/// ```compile_fail
/// fn replay(capability: open_bitcoin_node::network::SnapshotWriteCapability) {
///     let _duplicate = capability.clone();
/// }
/// ```
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
///
/// ```compile_fail
/// fn replay(receipt: open_bitcoin_node::network::SnapshotWriteReceipt) {
///     let _duplicate = receipt.clone();
/// }
/// ```
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

    pub(in crate::network) const fn effect_id(&self) -> SnapshotEffectId {
        self.effect_id
    }

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
}

#[derive(Debug, Clone, Default)]
pub(in crate::network) struct PeerEffectLedger {
    pending: BTreeSet<PeerEffectId>,
    completed_order: VecDeque<PeerEffectId>,
    completed: BTreeSet<PeerEffectId>,
    next_id: u64,
}

impl PeerEffectLedger {
    pub(in crate::network) fn reserve_next(
        &mut self,
    ) -> Result<PeerEffectId, EffectPreparationError> {
        let effect_id = PeerEffectId::new(self.next_id);
        let next_id = self
            .next_id
            .checked_add(1)
            .ok_or(EffectPreparationError::EffectIdentityExhausted)?;
        self.try_reserve(effect_id)?;
        self.next_id = next_id;
        Ok(effect_id)
    }

    pub(in crate::network) fn try_reserve(
        &mut self,
        effect_id: PeerEffectId,
    ) -> Result<(), EffectPreparationError> {
        if self.pending.len() >= MAX_PENDING_PEER_EFFECTS {
            return Err(EffectPreparationError::PeerEffectsAtCapacity);
        }
        if self.pending.contains(&effect_id) || self.completed.contains(&effect_id) {
            return Err(EffectPreparationError::EffectIdentityCollision);
        }
        self.pending.insert(effect_id);
        Ok(())
    }

    pub(in crate::network) fn is_pending(&self, effect_id: PeerEffectId) -> bool {
        self.pending.contains(&effect_id)
    }

    pub(in crate::network) fn record_completed(&mut self, effect_id: PeerEffectId) {
        self.pending.remove(&effect_id);
        if !self.completed.insert(effect_id) {
            return;
        }
        self.completed_order.push_back(effect_id);
        if self.completed_order.len() <= MAX_COMPLETED_PEER_EFFECTS {
            return;
        }
        let Some(evicted) = self.completed_order.pop_front() else {
            return;
        };
        self.completed.remove(&evicted);
    }

    pub(in crate::network) fn is_completed(&self, effect_id: PeerEffectId) -> bool {
        self.completed.contains(&effect_id)
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
    pending: BTreeSet<SnapshotEffectId>,
    completed_order: VecDeque<SnapshotEffectId>,
    completed: BTreeSet<SnapshotEffectId>,
    next_id: u64,
}

impl SnapshotEffectLedger {
    pub(in crate::network) fn reserve_next(
        &mut self,
    ) -> Result<SnapshotEffectId, EffectPreparationError> {
        let effect_id = SnapshotEffectId::new(self.next_id);
        let next_id = self
            .next_id
            .checked_add(1)
            .ok_or(EffectPreparationError::EffectIdentityExhausted)?;
        self.try_reserve(effect_id)?;
        self.next_id = next_id;
        Ok(effect_id)
    }

    pub(in crate::network) fn try_reserve(
        &mut self,
        effect_id: SnapshotEffectId,
    ) -> Result<(), EffectPreparationError> {
        if self.pending.len() >= MAX_PENDING_SNAPSHOT_EFFECTS {
            return Err(EffectPreparationError::SnapshotEffectPending);
        }
        if self.pending.contains(&effect_id) || self.completed.contains(&effect_id) {
            return Err(EffectPreparationError::EffectIdentityCollision);
        }
        self.pending.insert(effect_id);
        Ok(())
    }

    pub(in crate::network) fn is_pending(&self, effect_id: SnapshotEffectId) -> bool {
        self.pending.contains(&effect_id)
    }

    pub(in crate::network) fn record_completed(&mut self, effect_id: SnapshotEffectId) {
        self.pending.remove(&effect_id);
        if !self.completed.insert(effect_id) {
            return;
        }
        self.completed_order.push_back(effect_id);
        if self.completed_order.len() <= MAX_COMPLETED_SNAPSHOT_EFFECTS {
            return;
        }
        let Some(evicted) = self.completed_order.pop_front() else {
            return;
        };
        self.completed.remove(&evicted);
    }

    pub(in crate::network) fn is_completed(&self, effect_id: SnapshotEffectId) -> bool {
        self.completed.contains(&effect_id)
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
