// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Serialized same-database SyncAll publication. The runtime must also serialize
//! coins/metadata writers: read/compare is not CAS against arbitrary raw writers.

use super::super::{
    FjallNodeStore, StorageError, StorageNamespace, StorageRecoveryAction, backend_failure,
    prune::{PRUNE_LOCKS_KEY, encode_prune_locks},
};
use super::BasicFilterWorkToken;
use super::ownership::BasicFilterAppendIdentity;
use crate::storage::filter_index::{
    self as codec, StoredFilterRecord, StoredFilterState, index_corruption,
};
use fjall::PersistMode as FjallPersistMode;
use open_bitcoin_core::chainstate::filter_index::catch_up::{
    BASIC_INDEX_MAX_CANDIDATES, BASIC_INDEX_MAX_ENCODED_BYTES,
    BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES,
};
use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};
use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, CoinsView, FilterCheckpoint, IndexInputProtection, IndexPrefix,
    VerifiedChainstateFence,
};
use std::sync::MutexGuard;

#[derive(Default)]
pub(crate) struct PublicationControl {
    poisoned: bool,
    pub(super) maybe_reorg_suspension: Option<super::reorg::BasicFilterReorgSuspension>,
    pub(in crate::storage::fjall_store) revision: u64,
    pub(in crate::storage::fjall_store) maybe_append_identity: Option<BasicFilterAppendIdentity>,
    pub(in crate::storage::fjall_store) maybe_pending_coins: Option<PendingCoinsPublication>,
    pub(in crate::storage::fjall_store) maybe_completed_metadata:
        Option<CompletedMetadataPublication>,
    #[cfg(test)]
    maybe_fault: Option<FilterPublicationFault>,
    #[cfg(test)]
    maybe_writer_interleave: Option<BasicFilterWriterInterleave>,
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum BasicFilterWriterInterleave {
    BeforeCoins,
    BeforeMetadata,
    BeforeConfirm,
    RecreateBeforeConfirm,
}

#[derive(Clone, Copy)]
pub(in crate::storage::fjall_store) struct PendingCoinsPublication {
    pub previous: BasicFilterAppendIdentity,
    pub new_tip: open_bitcoin_core::primitives::BlockHash,
    pub revision: u64,
    pub completed: bool,
}

#[derive(Clone, Copy)]
pub(in crate::storage::fjall_store) struct CompletedMetadataPublication {
    pub coins: PendingCoinsPublication,
    pub identity: BasicFilterAppendIdentity,
}

impl PublicationControl {
    pub(crate) fn begin_coins_write(
        &mut self,
        new_tip: open_bitcoin_core::primitives::BlockHash,
    ) -> Result<(), StorageError> {
        #[cfg(test)]
        fault(self, FilterPublicationFault::BeforeCoins)?;
        let maybe_identity = self.maybe_append_identity;
        self.invalidate_append()?;
        self.maybe_pending_coins = maybe_identity.map(|previous| PendingCoinsPublication {
            previous,
            new_tip,
            revision: self.revision,
            completed: false,
        });
        Ok(())
    }

    pub(crate) fn complete_coins_write(&mut self) {
        if let Some(pending) = &mut self.maybe_pending_coins {
            pending.completed = true;
        }
    }

    pub(in crate::storage::fjall_store) fn chain_meta_fault(&mut self) -> Result<(), StorageError> {
        #[cfg(test)]
        fault(self, FilterPublicationFault::BeforeChainMeta)?;
        Ok(())
    }
    /// Clear every clone's live authority before any potentially ambiguous write.
    pub(crate) fn invalidate_append(&mut self) -> Result<(), StorageError> {
        self.maybe_append_identity = None;
        self.maybe_pending_coins = None;
        self.maybe_completed_metadata = None;
        let Some(revision) = self.revision.checked_add(1) else {
            self.poisoned = true;
            return Err(publication_failure("BASIC durable revision exhausted"));
        };
        self.revision = revision;
        Ok(())
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FilterPublicationFault {
    BeforeBody,
    BeforeUndo,
    BeforeCoins,
    BeforeRecords,
    BeforeCheckpoint,
    BeforeProtection,
    AfterCommit,
    BeforeChainMeta,
    BeforeDisable,
    AfterDisable,
    BeforeRelease,
    AfterRelease,
    BeforeEnable,
    AfterEnable,
}

pub(super) enum LifecyclePublicationPoint {
    BeforeDisable,
    AfterDisable,
    BeforeRelease,
    AfterRelease,
    BeforeEnable,
    AfterEnable,
}

pub(super) fn lifecycle_fault(
    _control: &mut PublicationControl,
    _point: LifecyclePublicationPoint,
) -> Result<(), StorageError> {
    #[cfg(test)]
    fault(
        _control,
        match _point {
            LifecyclePublicationPoint::BeforeDisable => FilterPublicationFault::BeforeDisable,
            LifecyclePublicationPoint::AfterDisable => FilterPublicationFault::AfterDisable,
            LifecyclePublicationPoint::BeforeRelease => FilterPublicationFault::BeforeRelease,
            LifecyclePublicationPoint::AfterRelease => FilterPublicationFault::AfterRelease,
            LifecyclePublicationPoint::BeforeEnable => FilterPublicationFault::BeforeEnable,
            LifecyclePublicationPoint::AfterEnable => FilterPublicationFault::AfterEnable,
        },
    )?;
    Ok(())
}

impl FjallNodeStore {
    #[cfg(test)]
    pub(crate) fn set_basic_filter_writer_interleave(&self, point: BasicFilterWriterInterleave) {
        self.filter_publication
            .lock()
            .expect("control")
            .maybe_writer_interleave = Some(point);
    }

    /// Invoke an actual raw clone writer outside the guarded production boundary.
    #[cfg(test)]
    pub(crate) fn run_basic_filter_writer_interleave(
        &self,
        point: BasicFilterWriterInterleave,
    ) -> Result<(), StorageError> {
        let maybe_selected = {
            let mut control = self.filter_publication_guard()?;
            if control.maybe_writer_interleave == Some(point)
                || (point == BasicFilterWriterInterleave::BeforeConfirm
                    && control.maybe_writer_interleave
                        == Some(BasicFilterWriterInterleave::RecreateBeforeConfirm))
            {
                control.maybe_writer_interleave.take()
            } else {
                None
            }
        };
        let Some(selected) = maybe_selected else {
            return Ok(());
        };
        let mut clone = self.clone();
        let best = clone.coins_view().best_block().map_err(index_corruption)?;
        clone
            .coins_view()
            .batch_write(
                open_bitcoin_core::chainstate::CoinsBatch {
                    entries: Default::default(),
                },
                best,
            )
            .map_err(index_corruption)?;
        if selected == BasicFilterWriterInterleave::RecreateBeforeConfirm {
            let (positions, _) = clone.load_chain_meta_for_open()?;
            crate::chainstate::FlushPersistSink::persist_chain_meta(&mut clone, &positions)?;
        }
        Ok(())
    }
    pub(crate) fn filter_publication_guard(
        &self,
    ) -> Result<MutexGuard<'_, PublicationControl>, StorageError> {
        let guard = self
            .filter_publication
            .lock()
            .map_err(|_| publication_failure("BASIC publication mutex poisoned"))?;
        if guard.poisoned {
            return Err(publication_failure("BASIC publication requires reopen"));
        }
        Ok(guard)
    }

    #[cfg(test)]
    pub(crate) fn set_basic_filter_fault(&self, fault: FilterPublicationFault) {
        self.filter_publication
            .lock()
            .expect("publication control")
            .maybe_fault = Some(fault);
    }

    #[cfg(test)]
    pub(crate) fn check_basic_filter_payload_fault(
        &self,
        point: FilterPublicationFault,
    ) -> Result<(), StorageError> {
        let mut control = self
            .filter_publication
            .lock()
            .map_err(|_| publication_failure("BASIC publication mutex poisoned"))?;
        fault(&mut control, point)
    }

    #[cfg(test)]
    pub(crate) fn set_basic_filter_revision_for_test(&self, revision: u64) {
        let mut control = self.filter_publication.lock().expect("control");
        control.revision = revision;
        if let Some(identity) = &mut control.maybe_append_identity {
            identity.revision = revision;
        }
    }

    #[cfg_attr(not(test), allow(dead_code))] // Phase 157 owns explicit activation.
    pub(crate) fn initialize_basic_filter_state(
        &self,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        self.initialize_basic_filter_state_guarded(fence, &mut control)
    }

    pub(super) fn initialize_basic_filter_state_guarded(
        &self,
        fence: &VerifiedChainstateFence<'_>,
        control: &mut PublicationControl,
    ) -> Result<(), StorageError> {
        self.verify_basic_filter_fence(fence)?;
        if self.maybe_basic_filter_state()?.is_some() {
            return Err(index_corruption("BASIC state already initialized"));
        }
        let (records, projections) = self.basic_filter_artifacts()?;
        let mut locks = self.load_prune_locks()?;
        if records
            || projections
            || self.maybe_basic_filter_lifecycle()?.is_some()
            || locks.iter().any(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK)
        {
            return Err(index_corruption("partial BASIC state cannot initialize"));
        }
        let protection = IndexInputProtection::FromHeight(0);
        if let Some(lock) = protection.maybe_prune_lock() {
            locks.push(lock);
        }
        let state = StoredFilterState {
            maybe_endpoint: None,
            fence_height: fence.tip().height,
            fence_hash: fence.tip().block_hash,
            protection,
        };
        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        batch.insert(
            &self.block_index,
            codec::STATE_KEY,
            codec::encode_state(state),
        );
        batch.insert(
            &self.block_index,
            codec::ownership::OWNER_KEY,
            codec::ownership::encode_owner(IndexLifecycle::Active {
                generation: IndexGeneration::new(0),
            }),
        );
        checkpoint_fault(control)?;
        protection_fault(control)?;
        batch.insert(
            &self.block_index,
            PRUNE_LOCKS_KEY,
            encode_prune_locks(&locks)?,
        );
        self.finish_basic_filter_batch(batch, control)
    }

    /// Persist validated immutable candidates without creating or changing authority.
    #[cfg_attr(not(test), allow(dead_code))] // Phase 157 owns scheduled catch-up.
    pub(crate) fn persist_basic_filter_records(
        &self,
        work: &BasicFilterWorkToken,
        fence: &VerifiedChainstateFence<'_>,
        records: &[StoredFilterRecord],
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        self.check_basic_filter_work_guarded(work, fence, &control)?;
        let encoded = self.prepare_basic_filter_records(records)?;
        records_fault(&mut control)?;
        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        for (key, bytes) in encoded {
            batch.insert(&self.block_index, key, bytes);
        }
        self.finish_basic_filter_batch(batch, &mut control)
    }

    /// Atomically publish immutable rows, changed projection, explicit state and
    /// the complete lock map. Rewinds hide suffix without erasing any immutable row.
    #[cfg_attr(not(test), allow(dead_code))] // Scheduled workers arrive in Phase 157.
    pub(crate) fn publish_basic_filter_checkpoint(
        &self,
        work: &BasicFilterWorkToken,
        fence: &VerifiedChainstateFence<'_>,
        checkpoint: FilterCheckpoint,
        protection: IndexInputProtection,
        records: &[StoredFilterRecord],
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        self.check_basic_filter_work_guarded(work, fence, &control)?;
        self.publish_basic_filter_checkpoint_guarded(
            fence,
            checkpoint,
            protection,
            records,
            &mut control,
        )
    }

    /// Private startup/lifecycle reconciliation; caller holds publication and proves
    /// exclusive recovery authority. It never serves as a worker completion API.
    pub(super) fn publish_basic_filter_checkpoint_guarded(
        &self,
        fence: &VerifiedChainstateFence<'_>,
        checkpoint: FilterCheckpoint,
        protection: IndexInputProtection,
        records: &[StoredFilterRecord],
        control: &mut PublicationControl,
    ) -> Result<(), StorageError> {
        self.verify_basic_filter_fence(fence)?;
        let Some(owner) = self.maybe_basic_filter_owner_guarded(control)? else {
            return Err(index_corruption(
                "BASIC publication requires explicit state",
            ));
        };
        if !matches!(owner.lifecycle(), IndexLifecycle::Active { .. }) {
            return Err(index_corruption("disabled BASIC checkpoint publication"));
        }
        self.validate_basic_filter_records()?;
        let saved_checkpoint = self
            .maybe_basic_filter_checkpoint()?
            .ok_or_else(|| index_corruption("missing BASIC checkpoint"))?;
        let mut locks = self.load_prune_locks()?;
        let saved_protection = owner
            .maybe_effective_protection()
            .ok_or_else(|| index_corruption("missing BASIC reserved protection"))?;
        if let open_bitcoin_core::chainstate::FilterRecoveryPlan::Refuse(error) =
            self.scan_basic_filter_checkpoint(saved_checkpoint, saved_protection, fence)?
        {
            return Err(index_corruption(error));
        }
        if !protection.covers(checkpoint.input_protection()) {
            return Err(index_corruption("weak BASIC publication protection"));
        }
        let encoded = self.prepare_basic_filter_records(records)?;
        let projections = self.prepare_basic_filter_projection(fence, checkpoint, records)?;
        let maybe_endpoint = match checkpoint.prefix() {
            IndexPrefix::Empty => None,
            IndexPrefix::Committed(id) => Some((id.height(), id.block_hash())),
        };
        locks.retain(|lock| lock.name != BASIC_INDEX_PRUNE_LOCK);
        if let Some(lock) = protection.maybe_prune_lock() {
            locks.push(lock);
        }
        let state = StoredFilterState {
            maybe_endpoint,
            fence_height: fence.tip().height,
            fence_hash: fence.tip().block_hash,
            protection,
        };
        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        for (key, bytes) in encoded {
            batch.insert(&self.block_index, key, bytes);
        }
        for (key, bytes) in projections {
            batch.insert(&self.block_index, key, bytes);
        }
        checkpoint_fault(control)?;
        batch.insert(
            &self.block_index,
            codec::STATE_KEY,
            codec::encode_state(state),
        );
        protection_fault(control)?;
        batch.insert(
            &self.block_index,
            PRUNE_LOCKS_KEY,
            encode_prune_locks(&locks)?,
        );
        self.finish_basic_filter_batch(batch, control)
    }

    pub(super) fn materialize_basic_filter_owner_guarded(
        &self,
        owner: IndexLifecycle,
        control: &mut PublicationControl,
    ) -> Result<(), StorageError> {
        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        checkpoint_fault(control)?;
        batch.insert(
            &self.block_index,
            codec::ownership::OWNER_KEY,
            codec::ownership::encode_owner(owner),
        );
        self.finish_basic_filter_batch(batch, control)
    }

    pub(crate) fn verify_basic_filter_fence(
        &self,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<(), StorageError> {
        let view = self.coins_view();
        if !view.head_blocks().map_err(index_corruption)?.is_empty() {
            return Err(index_corruption("unrecovered BASIC coins heads"));
        }
        let best = view.best_block().map_err(index_corruption)?;
        let (positions, _) = self.load_chain_meta_for_open()?;
        let durable =
            VerifiedChainstateFence::new(best, Some(&positions)).map_err(index_corruption)?;
        if durable != *fence {
            return Err(index_corruption("stale BASIC chainstate fence"));
        }
        Ok(())
    }

    fn prepare_basic_filter_records(
        &self,
        records: &[StoredFilterRecord],
    ) -> Result<Vec<(String, Vec<u8>)>, StorageError> {
        if records.len() as u64 > BASIC_INDEX_MAX_CANDIDATES {
            return Err(index_corruption("BASIC append record count bound"));
        }
        let mut total = 0_usize;
        for record in records {
            let envelope = record
                .encoded_bytes()
                .len()
                .checked_add(codec::RECORD_OVERHEAD)
                .ok_or_else(|| index_corruption("BASIC append byte overflow"))?;
            if envelope as u64 > BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES {
                return Err(index_corruption("BASIC append singleton byte bound"));
            }
            total = total
                .checked_add(envelope)
                .ok_or_else(|| index_corruption("BASIC append byte overflow"))?;
        }
        if total as u64 > BASIC_INDEX_MAX_ENCODED_BYTES {
            return Err(index_corruption("BASIC append aggregate byte bound"));
        }
        let mut encoded = Vec::with_capacity(records.len());
        for (index, record) in records.iter().enumerate() {
            let id = record.identity();
            let key = codec::record_key(id.block_hash());
            let maybe_predecessor = if id.height() == 0 {
                None
            } else {
                match records[..index]
                    .iter()
                    .find(|p| p.identity().block_hash() == id.parent_hash())
                {
                    Some(previous) => Some(previous.identity()),
                    None => self
                        .load_basic_filter_record(id.parent_hash())?
                        .map(|p| p.identity()),
                }
            };
            let bytes = codec::encode_record(record);
            codec::decode_record(&key, &bytes, maybe_predecessor.as_ref())?;
            if let Some(existing) = self.load_basic_filter_record(id.block_hash())?
                && existing != *record
            {
                return Err(index_corruption("conflicting immutable BASIC record"));
            }
            if records[..index].iter().any(|previous| {
                previous.identity().block_hash() == id.block_hash() && previous != record
            }) {
                return Err(index_corruption("conflicting BASIC candidate identity"));
            }
            encoded.push((key, bytes));
        }
        Ok(encoded)
    }

    fn prepare_basic_filter_projection(
        &self,
        fence: &VerifiedChainstateFence<'_>,
        checkpoint: FilterCheckpoint,
        records: &[StoredFilterRecord],
    ) -> Result<Vec<(String, Vec<u8>)>, StorageError> {
        let IndexPrefix::Committed(endpoint) = checkpoint.prefix() else {
            return Ok(Vec::new());
        };
        if endpoint.height() > fence.tip().height {
            return Err(index_corruption("BASIC checkpoint beyond coins fence"));
        }
        let mut projections = Vec::new();
        let mut maybe_previous = None;
        for height in 0..=endpoint.height() {
            let Some(position) = fence.maybe_position(height) else {
                return Err(index_corruption("missing BASIC fence position"));
            };
            let key = codec::record_key(position.block_hash);
            let bytes = match records
                .iter()
                .find(|r| r.identity().block_hash() == position.block_hash)
            {
                Some(record) => codec::encode_record(record),
                None => self
                    .get_bytes(StorageNamespace::BlockIndex, &key)?
                    .ok_or_else(|| index_corruption("missing BASIC publication record"))?,
            };
            let record = codec::decode_record(&key, &bytes, maybe_previous.as_ref())?;
            maybe_previous = Some(record.identity());
            let projection_key = codec::active_key(height);
            let maybe_existing = self
                .get_bytes(StorageNamespace::BlockIndex, &projection_key)?
                .map(|bytes| codec::decode_projection(&projection_key, &bytes))
                .transpose()?;
            if maybe_existing != Some(position.block_hash) {
                projections.push((
                    projection_key,
                    codec::encode_projection(height, position.block_hash),
                ));
                if projections.len() as u64 > BASIC_INDEX_MAX_CANDIDATES {
                    return Err(index_corruption("BASIC projection append bound"));
                }
            }
        }
        if maybe_previous != Some(endpoint) {
            return Err(index_corruption(
                "BASIC checkpoint endpoint differs from verified projection",
            ));
        }
        Ok(projections)
    }

    pub(super) fn finish_basic_filter_batch(
        &self,
        batch: fjall::OwnedWriteBatch,
        control: &mut PublicationControl,
    ) -> Result<(), StorageError> {
        control.invalidate_append()?;
        if let Err(error) = batch.commit() {
            control.poisoned = true;
            return Err(backend_failure(StorageNamespace::BlockIndex, error));
        }
        after_commit_fault(control)
    }
}

fn publication_failure(message: &str) -> StorageError {
    StorageError::BackendFailure {
        namespace: StorageNamespace::BlockIndex,
        message: message.to_owned(),
        action: StorageRecoveryAction::Restart,
    }
}

#[cfg(test)]
fn fault(
    control: &mut PublicationControl,
    point: FilterPublicationFault,
) -> Result<(), StorageError> {
    if control.maybe_fault == Some(point) {
        control.maybe_fault = None;
        control.invalidate_append()?;
        control.poisoned = true;
        return Err(publication_failure(
            "injected BASIC publication failure; reopen required",
        ));
    }
    Ok(())
}

pub(super) fn records_fault(_control: &mut PublicationControl) -> Result<(), StorageError> {
    #[cfg(test)]
    fault(_control, FilterPublicationFault::BeforeRecords)?;
    Ok(())
}
pub(super) fn checkpoint_fault(_control: &mut PublicationControl) -> Result<(), StorageError> {
    #[cfg(test)]
    fault(_control, FilterPublicationFault::BeforeCheckpoint)?;
    Ok(())
}
pub(super) fn protection_fault(_control: &mut PublicationControl) -> Result<(), StorageError> {
    #[cfg(test)]
    fault(_control, FilterPublicationFault::BeforeProtection)?;
    Ok(())
}
fn after_commit_fault(_control: &mut PublicationControl) -> Result<(), StorageError> {
    #[cfg(test)]
    fault(_control, FilterPublicationFault::AfterCommit)?;
    Ok(())
}
