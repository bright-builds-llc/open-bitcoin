#![cfg_attr(not(test), allow(dead_code))]
// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h

//! Durable prune locks and the support-evidence summary.
//!
//! Both records live under fixed block-index keys. A lock name is a field
//! inside the value. It is never a key and never a filesystem path.

use fjall::PersistMode as FjallPersistMode;
use open_bitcoin_core::chainstate::PruneLockInfo;

use super::super::{FjallNodeStore, StorageError, StorageNamespace, backend_failure, corruption};

/// Fixed block-index key for the whole prune-lock map.
pub(in crate::storage::fjall_store) const PRUNE_LOCKS_KEY: &str = "prune_locks";

/// Fixed block-index key for the support-evidence summary.
pub(in crate::storage::fjall_store) const PRUNE_SUMMARY_KEY: &str = "prune_summary";

const MAX_LOCK_NAME_BYTES: usize = 1024;

/// Counts of successful prune batches and the last batch's maximum height.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PruneSupportSummary {
    /// Successful batches that recorded at least one live deleted height.
    pub successful_batch_count: u64,
    /// Heights whose live payload was deleted across those batches.
    pub pruned_height_count: u64,
    /// Maximum height of the latest successful batch only.
    pub maybe_last_prune_height: Option<u32>,
}

impl FjallNodeStore {
    /// Loads the durable lock map. An absent key is an empty list.
    pub fn load_prune_locks(&self) -> Result<Vec<PruneLockInfo>, StorageError> {
        let Some(bytes) = self.block_index_bytes(PRUNE_LOCKS_KEY)? else {
            return Ok(Vec::new());
        };
        decode_prune_locks(&bytes)
    }

    /// Replaces the durable lock map in one SyncAll batch.
    ///
    /// The lock name stays inside the value. An empty slice writes an empty map.
    pub fn sync_prune_locks(&self, locks: &[PruneLockInfo]) -> Result<(), StorageError> {
        let _control = self.filter_publication_guard()?;
        let bytes = encode_prune_locks(locks)?;
        self.sync_block_index_value(PRUNE_LOCKS_KEY, bytes)
    }

    /// Loads the support summary. An absent key is zeros and no last height.
    pub(crate) fn load_prune_support_summary(&self) -> Result<PruneSupportSummary, StorageError> {
        let Some(bytes) = self.block_index_bytes(PRUNE_SUMMARY_KEY)? else {
            return Ok(PruneSupportSummary {
                successful_batch_count: 0,
                pruned_height_count: 0,
                maybe_last_prune_height: None,
            });
        };
        decode_prune_summary(&bytes)
    }

    /// Operator support counts. An absent summary is zeros and no last height.
    pub fn load_operator_support_counts(
        &self,
    ) -> Result<crate::status::PruneSupportCounts, StorageError> {
        let summary = self.load_prune_support_summary()?;
        Ok(crate::status::PruneSupportCounts {
            successful_batch_count: summary.successful_batch_count,
            pruned_height_count: summary.pruned_height_count,
            maybe_last_prune_height: summary.maybe_last_prune_height,
        })
    }

    /// Records one successful batch of live deleted heights.
    ///
    /// An empty slice returns `Ok` and does not write. A non-empty slice adds
    /// one batch, adds the slice length to the height count, and sets the last
    /// height to the maximum of this slice only.
    pub(crate) fn record_successful_prune_batch(
        &self,
        deleted_heights: &[u32],
    ) -> Result<(), StorageError> {
        if deleted_heights.is_empty() {
            return Ok(());
        }
        let mut summary = self.load_prune_support_summary()?;
        summary.successful_batch_count = checked_add(
            summary.successful_batch_count,
            1,
            "prune batch count overflowed",
        )?;
        let added_heights = u64::try_from(deleted_heights.len())
            .map_err(|_| block_index_corruption("prune height count overflowed"))?;
        summary.pruned_height_count = checked_add(
            summary.pruned_height_count,
            added_heights,
            "prune height count overflowed",
        )?;
        summary.maybe_last_prune_height = deleted_heights.iter().copied().max();
        self.sync_block_index_value(PRUNE_SUMMARY_KEY, encode_prune_summary(&summary)?)
    }

    fn block_index_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        self.get_bytes(StorageNamespace::BlockIndex, key)
    }

    fn sync_block_index_value(&self, key: &str, bytes: Vec<u8>) -> Result<(), StorageError> {
        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        batch.insert(&self.block_index, key, bytes);
        batch
            .commit()
            .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))
    }
}

pub(in crate::storage::fjall_store) fn encode_prune_locks(
    locks: &[PruneLockInfo],
) -> Result<Vec<u8>, StorageError> {
    refuse_duplicate_names(locks)?;
    let count = u32::try_from(locks.len())
        .map_err(|_| block_index_corruption("prune lock count overflowed"))?;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&count.to_le_bytes());
    for lock in locks {
        let name_bytes = lock.name.as_bytes();
        if name_bytes.len() > MAX_LOCK_NAME_BYTES {
            return Err(block_index_corruption("prune lock name exceeds 1024 bytes"));
        }
        let Ok(name_len) = u16::try_from(name_bytes.len()) else {
            return Err(block_index_corruption("prune lock name exceeds 1024 bytes"));
        };
        bytes.extend_from_slice(&name_len.to_le_bytes());
        bytes.extend_from_slice(name_bytes);
        bytes.extend_from_slice(&lock.height_first.to_le_bytes());
        bytes.extend_from_slice(&lock.height_last.to_le_bytes());
    }
    Ok(bytes)
}

fn decode_prune_locks(bytes: &[u8]) -> Result<Vec<PruneLockInfo>, StorageError> {
    let mut cursor = ByteCursor::new(bytes);
    let count = cursor.u32()?;
    let mut locks = Vec::new();
    for _ in 0..count {
        let name_len = usize::from(cursor.u16()?);
        if name_len > MAX_LOCK_NAME_BYTES {
            return Err(block_index_corruption("prune lock name exceeds 1024 bytes"));
        }
        let name_bytes = cursor.take(name_len)?;
        let Ok(name) = std::str::from_utf8(name_bytes) else {
            return Err(block_index_corruption("prune lock name is not utf-8"));
        };
        locks.push(PruneLockInfo {
            name: name.to_owned(),
            height_first: cursor.u32()?,
            height_last: cursor.u32()?,
        });
    }
    cursor.finish()?;
    refuse_duplicate_names(&locks)?;
    Ok(locks)
}

fn encode_prune_summary(summary: &PruneSupportSummary) -> Result<Vec<u8>, StorageError> {
    if summary.successful_batch_count == 0 && summary.maybe_last_prune_height.is_some() {
        return Err(block_index_corruption(
            "prune summary last height requires a batch",
        ));
    }
    let mut bytes = Vec::with_capacity(21);
    bytes.extend_from_slice(&summary.successful_batch_count.to_le_bytes());
    bytes.extend_from_slice(&summary.pruned_height_count.to_le_bytes());
    match summary.maybe_last_prune_height {
        None => bytes.push(0),
        Some(height) => {
            bytes.push(1);
            bytes.extend_from_slice(&height.to_le_bytes());
        }
    }
    Ok(bytes)
}

fn decode_prune_summary(bytes: &[u8]) -> Result<PruneSupportSummary, StorageError> {
    let mut cursor = ByteCursor::new(bytes);
    let successful_batch_count = cursor.u64()?;
    let pruned_height_count = cursor.u64()?;
    let flag = cursor.u8()?;
    let maybe_last_prune_height = match flag {
        0 => None,
        1 => Some(cursor.u32()?),
        _ => {
            return Err(block_index_corruption(
                "prune summary height flag is invalid",
            ));
        }
    };
    cursor.finish()?;
    if successful_batch_count == 0 && maybe_last_prune_height.is_some() {
        return Err(block_index_corruption(
            "prune summary last height requires a batch",
        ));
    }
    Ok(PruneSupportSummary {
        successful_batch_count,
        pruned_height_count,
        maybe_last_prune_height,
    })
}

fn refuse_duplicate_names(locks: &[PruneLockInfo]) -> Result<(), StorageError> {
    for (index, lock) in locks.iter().enumerate() {
        if locks[..index].iter().any(|prior| prior.name == lock.name) {
            return Err(block_index_corruption("prune lock name is duplicated"));
        }
    }
    Ok(())
}

fn checked_add(left: u64, right: u64, detail: &str) -> Result<u64, StorageError> {
    left.checked_add(right)
        .ok_or_else(|| block_index_corruption(detail))
}

fn block_index_corruption(detail: impl std::fmt::Display) -> StorageError {
    corruption(StorageNamespace::BlockIndex, detail)
}

struct ByteCursor<'a> {
    rest: &'a [u8],
}

impl<'a> ByteCursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { rest: bytes }
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], StorageError> {
        if self.rest.len() < len {
            return Err(block_index_corruption("truncated prune record"));
        }
        let (head, tail) = self.rest.split_at(len);
        self.rest = tail;
        Ok(head)
    }

    fn u8(&mut self) -> Result<u8, StorageError> {
        let bytes = self.take(1)?;
        Ok(bytes[0])
    }

    fn u16(&mut self) -> Result<u16, StorageError> {
        let bytes = self.take(2)?;
        let Ok(encoded) = <[u8; 2]>::try_from(bytes) else {
            return Err(block_index_corruption("truncated prune record"));
        };
        Ok(u16::from_le_bytes(encoded))
    }

    fn u32(&mut self) -> Result<u32, StorageError> {
        let bytes = self.take(4)?;
        let Ok(encoded) = <[u8; 4]>::try_from(bytes) else {
            return Err(block_index_corruption("truncated prune record"));
        };
        Ok(u32::from_le_bytes(encoded))
    }

    fn u64(&mut self) -> Result<u64, StorageError> {
        let bytes = self.take(8)?;
        let Ok(encoded) = <[u8; 8]>::try_from(bytes) else {
            return Err(block_index_corruption("truncated prune record"));
        };
        Ok(u64::from_le_bytes(encoded))
    }

    fn finish(self) -> Result<(), StorageError> {
        if self.rest.is_empty() {
            return Ok(());
        }
        Err(block_index_corruption("prune record has trailing bytes"))
    }
}
