// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Shared checkpoint bytes are data, never append or acceptance authority.

use crate::StorageError;
use crate::storage::filter_index as codec;
use crate::storage::fjall_store::{FjallNodeStore, encode_prune_locks};
use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, FilterCheckpoint, IndexInputProtection, IndexPrefix, PruneLockInfo,
};
use open_bitcoin_core::primitives::BlockHash;

pub(super) struct CheckpointRows {
    pub(super) protection: IndexInputProtection,
    pub(super) locks: Vec<PruneLockInfo>,
    pub(super) state: codec::StoredFilterState,
    pub(super) lock_count: u64,
    pub(super) lock_size: u64,
}

pub(super) fn checkpoint_rows(
    current: FilterCheckpoint,
    proposed: FilterCheckpoint,
    saved: IndexInputProtection,
    durable_height: u32,
    durable_hash: BlockHash,
    mut locks: Vec<PruneLockInfo>,
) -> Result<CheckpointRows, StorageError> {
    let protection = if current == proposed {
        saved
    } else {
        proposed.input_protection()
    };
    locks.retain(|lock| lock.name != BASIC_INDEX_PRUNE_LOCK);
    if let Some(lock) = protection.maybe_prune_lock() {
        locks.push(lock);
    }
    let state = codec::StoredFilterState {
        maybe_endpoint: super::maybe_checkpoint_endpoint(proposed),
        fence_height: durable_height,
        fence_hash: durable_hash,
        protection,
    };
    let lock_count = locks.len() as u64;
    let lock_size = locks.iter().try_fold(4u64, |size, lock| {
        size.checked_add(lock.name.len() as u64 + 10)
            .ok_or_else(|| codec::index_corruption("BASIC lock-map encoded overflow"))
    })?;
    Ok(CheckpointRows {
        protection,
        locks,
        state,
        lock_count,
        lock_size,
    })
}

impl FjallNodeStore {
    /// Forecast only bytes from real prepared historical inputs and held progress.
    pub(crate) fn submission_basic_rows(
        &self,
        position: &open_bitcoin_core::chainstate::ChainPosition,
        maybe_inputs: Option<&open_bitcoin_core::chainstate::BasicFilterInputs<'_>>,
        maybe_progress: Option<
            open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexProgress,
        >,
    ) -> Result<Vec<(String, Vec<u8>)>, StorageError> {
        use open_bitcoin_core::chainstate::filter_index::{
            catch_up::{BasicIndexState, TurnWork},
            lifecycle::IndexLifecycle,
        };
        let maybe_lifecycle = self.maybe_basic_filter_lifecycle()?;
        let Some(IndexLifecycle::Active { generation }) = maybe_lifecycle else {
            if maybe_progress.is_some_and(|progress| progress.state() != BasicIndexState::Disabled)
            {
                return Err(codec::index_corruption(
                    "BASIC projected lifecycle mismatch",
                ));
            }
            return Ok(vec![]);
        };
        let progress = maybe_progress
            .ok_or_else(|| codec::index_corruption("missing projected BASIC progress"))?;
        let inputs =
            maybe_inputs.ok_or_else(|| codec::index_corruption("missing prepared BASIC inputs"))?;
        let target = progress.accepted_target();
        let previous = progress
            .maybe_safe_durable_endpoint()
            .ok_or_else(|| codec::index_corruption("missing BASIC predecessor"))?;
        if progress.state() != BasicIndexState::Active
            || progress.generation() != generation
            || progress.maybe_processed_endpoint() != Some(previous)
            || previous.height() != target.height()
            || previous.block_hash() != target.block_hash()
            || position.height
                != target
                    .height()
                    .checked_add(1)
                    .ok_or_else(|| codec::index_corruption("BASIC projected height overflow"))?
            || position.previous_block_hash() != target.block_hash()
        {
            return Err(codec::index_corruption(
                "nonconsecutive projected BASIC target",
            ));
        }
        let state = self
            .maybe_basic_filter_state()?
            .ok_or_else(|| codec::index_corruption("missing projected BASIC state"))?;
        if state.maybe_endpoint != Some((previous.height(), previous.block_hash()))
            || state.fence_height != target.height()
            || state.fence_hash != target.block_hash()
            || state.protection != progress.protection()
        {
            return Err(codec::index_corruption("stale projected BASIC state"));
        }
        let mut work = TurnWork::default();
        let maximum = crate::chainstate::basic_filter_turn_budget()?.absolute_singleton();
        if self.read_basic_filter_local_identity(previous.block_hash(), &mut work, Some(maximum))?
            != previous
        {
            return Err(codec::index_corruption(
                "projected BASIC predecessor differs",
            ));
        }
        let record = codec::StoredFilterRecord::generate(inputs, position, Some(&previous))?;
        let record_key = codec::record_key(position.block_hash);
        let record_bytes = codec::encode_record(&record);
        if self
            .get_bytes(crate::StorageNamespace::BlockIndex, &record_key)?
            .is_some_and(|bytes| bytes != record_bytes)
        {
            return Err(codec::index_corruption(
                "conflicting projected BASIC record",
            ));
        }
        let projection_key = codec::active_key(position.height);
        let projection_bytes = codec::encode_projection(position.height, position.block_hash);
        if self
            .get_bytes(crate::StorageNamespace::BlockIndex, &projection_key)?
            .is_some_and(|bytes| bytes != projection_bytes)
        {
            return Err(codec::index_corruption(
                "conflicting projected BASIC active row",
            ));
        }
        let checkpoint = checkpoint_rows(
            FilterCheckpoint::new(IndexPrefix::Committed(previous)),
            FilterCheckpoint::new(IndexPrefix::Committed(record.identity())),
            progress.protection(),
            position.height,
            position.block_hash,
            self.load_prune_locks()?,
        )?;
        let (state_bytes, lock_bytes) = checkpoint.encode()?;
        Ok(vec![
            (record_key, record_bytes),
            (projection_key, projection_bytes),
            (codec::STATE_KEY.to_owned(), state_bytes),
            (
                crate::storage::fjall_store::PRUNE_LOCKS_KEY.to_owned(),
                lock_bytes,
            ),
        ])
    }
}

impl CheckpointRows {
    /// Call only after the actual writer reserves the reported unchanged work.
    pub(super) fn encode(&self) -> Result<(Vec<u8>, Vec<u8>), StorageError> {
        Ok((
            codec::encode_state(self.state),
            encode_prune_locks(&self.locks)?,
        ))
    }
}

#[cfg(test)]
mod tests;
