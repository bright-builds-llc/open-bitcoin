// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Serialized same-database SyncAll publication. The runtime must also serialize
//! coins/metadata writers: read/compare is not CAS against arbitrary raw writers.

use super::super::{
    FjallNodeStore, StorageError, StorageNamespace, StorageRecoveryAction, backend_failure,
    prune::{PRUNE_LOCKS_KEY, encode_prune_locks},
};
use crate::storage::filter_index::{
    self as codec, StoredFilterRecord, StoredFilterState, index_corruption,
};
use fjall::PersistMode as FjallPersistMode;
use open_bitcoin_core::{
    chainstate::{
        BASIC_INDEX_PRUNE_LOCK, CoinsView, FilterCheckpoint, IndexInputProtection, IndexPrefix,
        VerifiedChainstateFence,
    },
    codec::MAX_SIZE,
};
use std::sync::MutexGuard;

#[derive(Default)]
pub(in crate::storage::fjall_store) struct PublicationControl {
    poisoned: bool,
    #[cfg(test)]
    maybe_fault: Option<FilterPublicationFault>,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FilterPublicationFault {
    BeforeRecords,
    BeforeCheckpoint,
    BeforeProtection,
    AfterCommit,
    BeforeChainMeta,
}

impl FjallNodeStore {
    pub(in crate::storage::fjall_store) fn filter_publication_guard(
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

    /// Test-only seam on the concrete metadata writer after a real coins flush.
    #[cfg(test)]
    pub(in crate::storage::fjall_store) fn check_basic_filter_chain_meta_fault(
        &self,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        fault(&mut control, FilterPublicationFault::BeforeChainMeta)
    }

    #[cfg_attr(not(test), allow(dead_code))] // Phase 157 owns explicit activation.
    pub(crate) fn initialize_basic_filter_state(
        &self,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        self.verify_basic_filter_fence(fence)?;
        if self.maybe_basic_filter_state()?.is_some() {
            return Err(index_corruption("BASIC state already initialized"));
        }
        let (records, projections) = self.basic_filter_artifacts()?;
        let mut locks = self.load_prune_locks()?;
        if records || projections || locks.iter().any(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK) {
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
        checkpoint_fault(&mut control)?;
        protection_fault(&mut control)?;
        batch.insert(
            &self.block_index,
            PRUNE_LOCKS_KEY,
            encode_prune_locks(&locks)?,
        );
        self.finish_basic_filter_batch(batch, &mut control)
    }

    /// Persist validated immutable candidates without creating or changing authority.
    #[cfg_attr(not(test), allow(dead_code))] // Phase 157 owns scheduled catch-up.
    pub(crate) fn persist_basic_filter_records(
        &self,
        records: &[StoredFilterRecord],
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
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
    pub(crate) fn publish_basic_filter_checkpoint(
        &self,
        fence: &VerifiedChainstateFence<'_>,
        checkpoint: FilterCheckpoint,
        protection: IndexInputProtection,
        records: &[StoredFilterRecord],
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        self.verify_basic_filter_fence(fence)?;
        let Some(saved) = self.maybe_basic_filter_state()? else {
            return Err(index_corruption(
                "BASIC publication requires explicit state",
            ));
        };
        let saved_checkpoint = self.checkpoint_from_state(saved)?;
        let mut locks = self.load_prune_locks()?;
        let maybe_lock = locks
            .iter()
            .find(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK);
        let saved_protection = match maybe_lock {
            Some(lock) => IndexInputProtection::from_saved_lock(lock).map_err(index_corruption)?,
            None if saved.protection == IndexInputProtection::HeightSpaceExhausted => {
                IndexInputProtection::HeightSpaceExhausted
            }
            None => return Err(index_corruption("missing BASIC reserved protection")),
        };
        if !saved_protection.covers(saved.protection)
            || !saved_protection.covers(saved_checkpoint.input_protection())
        {
            return Err(index_corruption("weak BASIC reserved protection"));
        }
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
        checkpoint_fault(&mut control)?;
        batch.insert(
            &self.block_index,
            codec::STATE_KEY,
            codec::encode_state(state),
        );
        protection_fault(&mut control)?;
        batch.insert(
            &self.block_index,
            PRUNE_LOCKS_KEY,
            encode_prune_locks(&locks)?,
        );
        self.finish_basic_filter_batch(batch, &mut control)
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
        if records.len() > 128 {
            return Err(index_corruption("BASIC append record count bound"));
        }
        let mut total = 0_usize;
        for record in records {
            total = total
                .checked_add(record.encoded_bytes().len())
                .and_then(|v| v.checked_add(codec::RECORD_OVERHEAD))
                .ok_or_else(|| index_corruption("BASIC append byte overflow"))?;
        }
        if total > MAX_SIZE as usize + 128 * codec::RECORD_OVERHEAD {
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
                if projections.len() > 128 {
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

    fn finish_basic_filter_batch(
        &self,
        batch: fjall::OwnedWriteBatch,
        control: &mut PublicationControl,
    ) -> Result<(), StorageError> {
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
        control.poisoned = true;
        return Err(publication_failure(
            "injected BASIC publication failure; reopen required",
        ));
    }
    Ok(())
}

fn records_fault(_control: &mut PublicationControl) -> Result<(), StorageError> {
    #[cfg(test)]
    fault(_control, FilterPublicationFault::BeforeRecords)?;
    Ok(())
}
fn checkpoint_fault(_control: &mut PublicationControl) -> Result<(), StorageError> {
    #[cfg(test)]
    fault(_control, FilterPublicationFault::BeforeCheckpoint)?;
    Ok(())
}
fn protection_fault(_control: &mut PublicationControl) -> Result<(), StorageError> {
    #[cfg(test)]
    fault(_control, FilterPublicationFault::BeforeProtection)?;
    Ok(())
}
fn after_commit_fault(_control: &mut PublicationControl) -> Result<(), StorageError> {
    #[cfg(test)]
    fault(_control, FilterPublicationFault::AfterCommit)?;
    Ok(())
}
