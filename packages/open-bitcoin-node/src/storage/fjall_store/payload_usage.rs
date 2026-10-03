// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Exact logical retained bytes, serialized with complete payload mutations.

use std::{
    collections::BTreeMap,
    sync::{
        MutexGuard,
        atomic::{AtomicU64, Ordering},
    },
};

use fjall::Readable;
use open_bitcoin_core::chainstate::ChainPosition;

use super::{
    FjallNodeStore, PersistMode, StorageError, StorageNamespace, backend_failure, block_key,
    coins::undo_key, corruption,
};

static NEXT_STORE_ID: AtomicU64 = AtomicU64::new(1);

/// Identity of facts measured in this open store, or a conservative refusal of reuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadUsageRevision {
    /// A fresh open or mutation requires measurement before reuse.
    Unmeasured,
    /// Only equal measured identities can authorize skipping measurement.
    Current { store_id: u64, generation: u64 },
    /// Identity space exhausted; facts must always be measured again.
    Invalid,
}

impl PayloadUsageRevision {
    /// Whether equality with previously measured facts permits reuse.
    pub const fn is_reusable(self) -> bool {
        matches!(self, Self::Current { .. })
    }
}

/// All retained payload bytes and measured sizes of present active-chain pairs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetainedPayloadUsage {
    pub current_usage_bytes: u64,
    pub height_sizes: BTreeMap<u32, u64>,
    pub revision: PayloadUsageRevision,
}

pub(super) struct PayloadUsageState {
    maybe_store_id: Option<u64>,
    maybe_generation: Option<u64>,
    measured: bool,
}

impl PayloadUsageState {
    pub(super) fn new() -> Self {
        Self {
            maybe_store_id: NEXT_STORE_ID
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
                .ok(),
            maybe_generation: Some(0),
            measured: false,
        }
    }

    fn revision(&self) -> PayloadUsageRevision {
        let (Some(store_id), Some(generation)) = (self.maybe_store_id, self.maybe_generation)
        else {
            return PayloadUsageRevision::Invalid;
        };
        if !self.measured {
            return PayloadUsageRevision::Unmeasured;
        }
        PayloadUsageRevision::Current {
            store_id,
            generation,
        }
    }

    fn invalidate(&mut self) {
        self.maybe_generation = self.maybe_generation.and_then(|value| value.checked_add(1));
        self.measured = false;
    }
}

impl FjallNodeStore {
    pub(super) fn put_bytes(
        &self,
        namespace: StorageNamespace,
        key: &str,
        bytes: Vec<u8>,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let mutate = || {
            self.keyspace(namespace)
                .insert(key, bytes)
                .map_err(|error| backend_failure(namespace, error))?;
            self.persist(namespace, mode)
        };
        if is_payload_key(namespace, key) {
            return self.with_payload_mutation(mutate);
        }
        mutate()
    }

    pub(super) fn remove_bytes(
        &self,
        namespace: StorageNamespace,
        key: &str,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let mutate = || {
            self.keyspace(namespace)
                .remove(key)
                .map_err(|error| backend_failure(namespace, error))?;
            self.persist(namespace, mode)
        };
        if is_payload_key(namespace, key) {
            return self.with_payload_mutation(mutate);
        }
        mutate()
    }

    /// Measures all block/undo values and candidate pairs from one guarded snapshot.
    pub fn retained_payload_usage(
        &self,
        active_chain: &[ChainPosition],
    ) -> Result<RetainedPayloadUsage, StorageError> {
        let mut state = self.payload_guard()?;
        state.measured = false;
        let snapshot = self.db.snapshot();
        let block_sizes = snapshot.prefix(&self.block_index, "block:").map(|guard| {
            guard
                .size()
                .map(u64::from)
                .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))
        });
        let undo_sizes = snapshot.prefix(&self.chainstate, "undo:").map(|guard| {
            guard
                .size()
                .map(u64::from)
                .map_err(|error| backend_failure(StorageNamespace::Chainstate, error))
        });
        let current_usage_bytes = sum_sizes(block_sizes.chain(undo_sizes))?;
        let mut height_sizes = BTreeMap::new();
        for position in active_chain {
            let maybe_block_size = snapshot
                .size_of(&self.block_index, block_key(position.block_hash))
                .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
            let maybe_undo_size = snapshot
                .size_of(&self.chainstate, undo_key(position.block_hash))
                .map_err(|error| backend_failure(StorageNamespace::Chainstate, error))?;
            if maybe_block_size.is_none() && maybe_undo_size.is_none() {
                continue;
            }
            let size = sum_sizes([
                Ok(u64::from(maybe_block_size.unwrap_or(0))),
                Ok(u64::from(maybe_undo_size.unwrap_or(0))),
            ])?;
            height_sizes.insert(position.height, size);
        }
        state.measured = true;
        Ok(RetainedPayloadUsage {
            current_usage_bytes,
            height_sizes,
            revision: state.revision(),
        })
    }

    /// Returns the revision under the same guard used by writers and measurement.
    pub fn payload_usage_revision(&self) -> Result<PayloadUsageRevision, StorageError> {
        Ok(self.payload_guard()?.revision())
    }

    fn payload_guard(&self) -> Result<MutexGuard<'_, PayloadUsageState>, StorageError> {
        self.payload_usage
            .lock()
            .map_err(|_| usage_error("payload usage mutex poisoned"))
    }

    /// Keeps invalidation, partial live effects and persistence inside one RAII guard.
    pub(super) fn with_payload_mutation<T>(
        &self,
        mutate: impl FnOnce() -> Result<T, StorageError>,
    ) -> Result<T, StorageError> {
        let mut state = self.payload_guard()?;
        state.invalidate();
        mutate()
    }

    /// Pauses a real clone writer after invalidation without recursively locking raw writes.
    #[cfg(test)]
    pub(crate) fn mutate_payload_for_test(
        &self,
        hash: open_bitcoin_core::primitives::BlockHash,
        bytes: Vec<u8>,
        on_begin: impl FnOnce(),
        maybe_failure: Option<StorageError>,
    ) -> Result<(), StorageError> {
        self.with_payload_mutation(|| {
            on_begin();
            self.block_index
                .insert(block_key(hash), bytes)
                .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;
            if let Some(failure) = maybe_failure {
                return Err(failure);
            }
            self.persist(StorageNamespace::BlockIndex, PersistMode::Flush)
        })
    }
}

fn sum_sizes(
    sizes: impl IntoIterator<Item = Result<u64, StorageError>>,
) -> Result<u64, StorageError> {
    sizes.into_iter().try_fold(0_u64, |total, maybe_size| {
        total
            .checked_add(maybe_size?)
            .ok_or_else(|| usage_error("retained payload byte count overflowed"))
    })
}

fn is_payload_key(namespace: StorageNamespace, key: &str) -> bool {
    match namespace {
        StorageNamespace::BlockIndex => key.starts_with("block:"),
        StorageNamespace::Chainstate => key.starts_with("undo:"),
        _ => false,
    }
}

fn usage_error(detail: impl std::fmt::Display) -> StorageError {
    corruption(StorageNamespace::BlockIndex, detail)
}

#[cfg(test)]
mod tests;
