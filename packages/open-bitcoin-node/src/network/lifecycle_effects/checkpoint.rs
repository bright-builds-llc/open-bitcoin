// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Bounded exact-identity ledger for mempool checkpoint effects.

use std::collections::{BTreeSet, VecDeque};

use open_bitcoin_mempool::PolicyTime;

use super::{
    CheckpointTrigger, EffectAbort, EffectPreparationError, ExactEffectLedgerCompletion,
    MAX_COMPLETED_SNAPSHOT_EFFECTS, MAX_PENDING_SNAPSHOT_EFFECTS, PreparedSnapshotWrite,
    SnapshotEffectId, SnapshotEffectKey, SnapshotIdentity, SnapshotWriteCapability,
    SnapshotWriteReceipt,
};
use crate::network::lifecycle_projection::{AuthorityEpoch, LifecycleGeneration};
use crate::storage::MempoolSnapshot;

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
