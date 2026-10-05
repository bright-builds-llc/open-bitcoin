// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Trusted host transitions. Lock order: managed authority, publication, payload.
//! Holding publication stops token issuance throughout each durable transition.

use super::super::prune::{PRUNE_LOCKS_KEY, encode_prune_locks};
use super::publication::{LifecyclePublicationPoint, lifecycle_fault};
use super::{FjallNodeStore, StorageError, StoredFilterState, codec, index_corruption};
use fjall::PersistMode;
use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, BasicFilterInputs, FilterRecoveryPlan, HistoricalBlockUndo,
    IndexInputProtection, IndexPrefix, VerifiedChainstateFence,
    filter_index::lifecycle::IndexLifecycle,
};

impl FjallNodeStore {
    #[cfg(test)]
    pub(crate) fn maybe_basic_filter_lifecycle_for_test(
        &self,
    ) -> Result<Option<IndexLifecycle>, StorageError> {
        self.maybe_basic_filter_lifecycle()
    }

    /// Invalidate work durably before releasing only the BASIC owned lock.
    /// Retry Disabled-with-lock completes release without advancing generation.
    pub(crate) fn disable_basic_filter_index(&self) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        let Some(owner) = self.maybe_basic_filter_owner_guarded(&control)? else {
            return Ok(());
        };
        self.validate_basic_filter_records()?;
        self.maybe_basic_filter_checkpoint()?;
        let disabled = owner.lifecycle().disable().map_err(index_corruption)?;
        if disabled != owner.lifecycle() {
            lifecycle_fault(&mut control, LifecyclePublicationPoint::BeforeDisable)?;
            let mut batch = self.db.batch().durability(Some(PersistMode::SyncAll));
            batch.insert(
                &self.block_index,
                codec::ownership::OWNER_KEY,
                codec::ownership::encode_owner(disabled),
            );
            self.finish_basic_filter_batch(batch, &mut control)?;
            lifecycle_fault(&mut control, LifecyclePublicationPoint::AfterDisable)?;
        }
        let mut locks = self.load_prune_locks()?;
        if !locks.iter().any(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK) {
            return Ok(());
        }
        locks.retain(|lock| lock.name != BASIC_INDEX_PRUNE_LOCK);
        lifecycle_fault(&mut control, LifecyclePublicationPoint::BeforeRelease)?;
        let mut batch = self.db.batch().durability(Some(PersistMode::SyncAll));
        batch.insert(
            &self.block_index,
            PRUNE_LOCKS_KEY,
            encode_prune_locks(&locks)?,
        );
        self.finish_basic_filter_batch(batch, &mut control)?;
        lifecycle_fault(&mut control, LifecyclePublicationPoint::AfterRelease)
    }

    /// Preflight retained authoritative inputs, then acquire protection and Active
    /// ownership atomically. Callers serialize durable coins/metadata via authority.
    pub(crate) fn enable_basic_filter_index(
        &self,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        self.verify_basic_filter_fence(fence)?;
        let Some(owner) = self.maybe_basic_filter_owner_guarded(&control)? else {
            self.preflight_basic_filter_history(IndexInputProtection::FromHeight(0), fence)?;
            if let Some(intent) = self.maybe_prune_intent()? {
                IndexInputProtection::FromHeight(0)
                    .check_prune_intent(intent.height)
                    .map_err(index_corruption)?;
            }
            lifecycle_fault(&mut control, LifecyclePublicationPoint::BeforeEnable)?;
            self.initialize_basic_filter_state_guarded(fence, &mut control)?;
            return lifecycle_fault(&mut control, LifecyclePublicationPoint::AfterEnable);
        };
        if matches!(owner.lifecycle(), IndexLifecycle::Active { .. }) {
            return Ok(());
        }
        let active = owner.lifecycle().enable().map_err(index_corruption)?;
        self.validate_basic_filter_records()?;
        let saved = self
            .maybe_basic_filter_checkpoint()?
            .ok_or_else(|| index_corruption("missing BASIC checkpoint"))?;
        let (checkpoint, protection) =
            match self.scan_basic_filter_checkpoint(saved, owner.saved_protection(), fence)? {
                FilterRecoveryPlan::Keep {
                    checkpoint,
                    protection,
                }
                | FilterRecoveryPlan::Reconcile {
                    checkpoint,
                    protection,
                } => (checkpoint, protection),
                FilterRecoveryPlan::Refuse(error) => return Err(index_corruption(error)),
                FilterRecoveryPlan::LegacyAbsent => {
                    return Err(index_corruption("unexpected saved BASIC absence"));
                }
            };
        // Preserve extra retention left behind by an interrupted disable.
        let protection = owner
            .maybe_effective_protection()
            .filter(|retained| retained.covers(protection))
            .unwrap_or(protection);
        self.preflight_basic_filter_history(protection, fence)?;
        if let Some(intent) = self.maybe_prune_intent()? {
            protection
                .check_prune_intent(intent.height)
                .map_err(index_corruption)?;
        }
        let mut locks = self.load_prune_locks()?;
        locks.retain(|lock| lock.name != BASIC_INDEX_PRUNE_LOCK);
        if let Some(lock) = protection.maybe_prune_lock() {
            locks.push(lock);
        }
        let state = StoredFilterState {
            maybe_endpoint: match checkpoint.prefix() {
                IndexPrefix::Empty => None,
                IndexPrefix::Committed(id) => Some((id.height(), id.block_hash())),
            },
            fence_height: fence.tip().height,
            fence_hash: fence.tip().block_hash,
            protection,
        };
        let mut batch = self.db.batch().durability(Some(PersistMode::SyncAll));
        batch.insert(
            &self.block_index,
            codec::STATE_KEY,
            codec::encode_state(state),
        );
        batch.insert(
            &self.block_index,
            codec::ownership::OWNER_KEY,
            codec::ownership::encode_owner(active),
        );
        batch.insert(
            &self.block_index,
            PRUNE_LOCKS_KEY,
            encode_prune_locks(&locks)?,
        );
        lifecycle_fault(&mut control, LifecyclePublicationPoint::BeforeEnable)?;
        self.finish_basic_filter_batch(batch, &mut control)?;
        lifecycle_fault(&mut control, LifecyclePublicationPoint::AfterEnable)
    }

    fn preflight_basic_filter_history(
        &self,
        protection: IndexInputProtection,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<(), StorageError> {
        let IndexInputProtection::FromHeight(first) = protection else {
            return Ok(());
        };
        for height in first..=fence.tip().height {
            let position = fence
                .maybe_position(height)
                .ok_or_else(|| index_corruption("missing BASIC activation position"))?;
            let block = self
                .load_block(position.block_hash)?
                .ok_or_else(|| index_corruption("missing BASIC activation body"))?;
            let maybe_undo = if height == 0 {
                None
            } else {
                Some(
                    self.load_undo(position.block_hash)?
                        .ok_or_else(|| index_corruption("missing BASIC activation undo"))?,
                )
            };
            let maybe_history = maybe_undo.as_ref().map(|undo| HistoricalBlockUndo {
                block_hash: position.block_hash,
                undo,
            });
            BasicFilterInputs::from_historical(&block, position, maybe_history)
                .map_err(index_corruption)?;
        }
        Ok(())
    }
}
