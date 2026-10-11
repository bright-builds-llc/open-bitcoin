// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Trusted host transitions. Lock order: managed authority, publication, payload.
//! Holding publication stops token issuance throughout each durable transition.

use super::super::prune::{PRUNE_LOCKS_KEY, encode_prune_locks};
use super::PublicationControl;
use super::publication::{LifecyclePublicationPoint, lifecycle_fault};
use super::retention::{RetainedBasicFilterObservation, RetentionObservationLimits};
use super::startup::InspectedBasicFilterRecovery;
use super::{FjallNodeStore, StorageError, StoredFilterState, codec, index_corruption};
use fjall::PersistMode;
#[cfg(test)]
use open_bitcoin_core::chainstate::filter_index::lifecycle::IndexLifecycle;
use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, BasicFilterInputs, HistoricalBlockUndo, IndexInputProtection,
    IndexPrefix, VerifiedChainstateFence,
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
        control.maybe_read_fault_protection = None;
        self.disable_basic_filter_index_guarded(
            &mut control,
            RetentionObservationLimits::default(),
        )?;
        let retained = control.retention;
        control.reset_verified_forest(retained);
        Ok(())
    }

    pub(super) fn disable_basic_filter_index_guarded(
        &self,
        control: &mut PublicationControl,
        limits: RetentionObservationLimits,
    ) -> Result<(), StorageError> {
        control.invalidate_read_integrity()?;
        control.invalidate_append()?;
        let Some(owner) = self.maybe_basic_filter_owner_guarded(control)? else {
            control.read_integrity = true;
            control.install_retention(RetainedBasicFilterObservation::empty());
            return Ok(());
        };
        let retained = self.measure_basic_filter_records(limits)?;
        self.maybe_basic_filter_checkpoint()?;
        let disabled = owner.lifecycle().disable().map_err(index_corruption)?;
        if disabled != owner.lifecycle() {
            lifecycle_fault(control, LifecyclePublicationPoint::BeforeDisable)?;
            let mut batch = self.db.batch().durability(Some(PersistMode::SyncAll));
            batch.insert(
                &self.block_index,
                codec::ownership::OWNER_KEY,
                codec::ownership::encode_owner(disabled),
            );
            self.finish_basic_filter_batch(batch, control)?;
            lifecycle_fault(control, LifecyclePublicationPoint::AfterDisable)?;
        }
        let mut locks = self.load_prune_locks()?;
        if !locks.iter().any(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK) {
            control.read_integrity = true;
            control.install_retention(retained);
            return Ok(());
        }
        locks.retain(|lock| lock.name != BASIC_INDEX_PRUNE_LOCK);
        lifecycle_fault(control, LifecyclePublicationPoint::BeforeRelease)?;
        let mut batch = self.db.batch().durability(Some(PersistMode::SyncAll));
        batch.insert(
            &self.block_index,
            PRUNE_LOCKS_KEY,
            encode_prune_locks(&locks)?,
        );
        self.finish_basic_filter_batch(batch, control)?;
        lifecycle_fault(control, LifecyclePublicationPoint::AfterRelease)?;
        control.read_integrity = true;
        control.install_retention(retained);
        Ok(())
    }

    /// Preflight retained authoritative inputs, then acquire protection and Active
    /// ownership atomically. Callers serialize durable coins/metadata via authority.
    pub(crate) fn enable_basic_filter_index(
        &self,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        let maybe_previous_fault = control.maybe_read_fault;
        let result = self.enable_basic_filter_index_guarded(
            fence,
            &mut control,
            RetentionObservationLimits::default(),
        );
        if result.is_err() {
            control.maybe_read_fault = maybe_previous_fault;
            control.maybe_read_fault_protection = None;
            control.read_integrity = false;
            control.install_retention(RetainedBasicFilterObservation::Failed);
        } else {
            let retained = control.retention;
            control.reset_verified_forest(retained);
        }
        result
    }

    pub(super) fn enable_basic_filter_index_guarded(
        &self,
        fence: &VerifiedChainstateFence<'_>,
        control: &mut PublicationControl,
        limits: RetentionObservationLimits,
    ) -> Result<(), StorageError> {
        control.invalidate_read_integrity()?;
        control.invalidate_append()?;
        self.verify_basic_filter_fence(fence)?;
        let Some(owner) = self.maybe_basic_filter_owner_guarded(control)? else {
            self.preflight_basic_filter_history(IndexInputProtection::FromHeight(0), fence)?;
            if let Some(intent) = self.maybe_prune_intent()? {
                IndexInputProtection::FromHeight(0)
                    .check_prune_intent(intent.height)
                    .map_err(index_corruption)?;
            }
            lifecycle_fault(control, LifecyclePublicationPoint::BeforeEnable)?;
            self.initialize_basic_filter_state_guarded(fence, control)?;
            lifecycle_fault(control, LifecyclePublicationPoint::AfterEnable)?;
            return self.install_recovered_basic_filter_append_guarded(
                fence,
                control,
                super::retention::RetainedBasicFilterObservation::empty(),
            );
        };
        let active = owner.lifecycle().enable().map_err(index_corruption)?;
        let retained = self.measure_basic_filter_records(limits)?;
        let InspectedBasicFilterRecovery {
            checkpoint,
            protection,
            reconcile,
        } = self.inspect_basic_filter_recovery(owner, fence)?;
        // Stronger retained locks protect more than the still-required suffix.
        // Ahead immutable rows do not shorten this recovered checkpoint boundary.
        self.preflight_basic_filter_history(checkpoint.input_protection(), fence)?;
        if let Some(intent) = self.maybe_prune_intent()? {
            protection
                .check_prune_intent(intent.height)
                .map_err(index_corruption)?;
        }
        if active == owner.lifecycle()
            && !reconcile
            && self.maybe_basic_filter_lifecycle()?.is_some()
        {
            return self.install_recovered_basic_filter_append_guarded(fence, control, retained);
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
        lifecycle_fault(control, LifecyclePublicationPoint::BeforeEnable)?;
        self.finish_basic_filter_batch(batch, control)?;
        lifecycle_fault(control, LifecyclePublicationPoint::AfterEnable)?;
        self.install_recovered_basic_filter_append_guarded(fence, control, retained)
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
            let block = self.load_block(position.block_hash)?.ok_or_else(|| {
                index_corruption(format!("missing BASIC activation body at height {height}"))
            })?;
            let maybe_undo = if height == 0 {
                None
            } else {
                Some(self.load_undo(position.block_hash)?.ok_or_else(|| {
                    index_corruption(format!("missing BASIC activation undo at height {height}"))
                })?)
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
