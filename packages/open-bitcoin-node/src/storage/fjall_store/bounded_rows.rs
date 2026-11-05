// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.cpp

//! Bounded data representations in the existing storage transaction owner.

use super::tracked::OwnedWriteBatch;
use super::*;
use std::collections::BTreeMap;
mod codec;
use codec::{MAX_BYTES, MAX_ENTRIES, PreparedRows};
pub(in crate::storage) use codec::{RowFamily, decode_detached_rows};

pub(super) fn prepare(family: RowFamily, bytes: Vec<u8>) -> Result<PreparedRows, StorageError> {
    codec::prepare(family, bytes)
}

/// Apply the exact existing family replacement to detached data, never a write batch.
pub(in crate::storage) fn project_replacement(
    rows: &mut BTreeMap<Vec<u8>, Vec<u8>>,
    family: RowFamily,
    bytes: Vec<u8>,
) -> Result<(), StorageError> {
    let prepared = prepare(family, bytes)?;
    for (count, key) in rows.keys().filter(|key| family.owns(key)).enumerate() {
        if count >= MAX_ENTRIES || key.len() > 4096 {
            return Err(corruption(
                family.namespace(),
                "old row family exceeds inspection limits",
            ));
        }
    }
    rows.retain(|key, _| !family.owns(key));
    rows.extend(prepared.rows);
    Ok(())
}

pub(super) fn append_replacement(
    store: &FjallNodeStore,
    batch: &mut OwnedWriteBatch,
    prepared: PreparedRows,
) -> Result<(), StorageError> {
    let family = prepared.family;
    let keyspace = store.keyspace(family.namespace());
    let snapshot = store.db.snapshot();
    let mut keys = Vec::new();
    for guard in snapshot.prefix(keyspace, family.root()) {
        let key = guard
            .key()
            .map_err(|error| backend_failure(family.namespace(), error))?;
        if !family.owns(&key) {
            continue;
        }
        if keys.len() >= MAX_ENTRIES || key.len() > 4096 {
            return Err(corruption(
                family.namespace(),
                "old row family exceeds inspection limits",
            ));
        }
        keys.push(key);
    }
    for key in keys {
        batch.remove(keyspace, key);
    }
    for (key, value) in prepared.rows {
        batch.insert(keyspace, key, value);
    }
    Ok(())
}

pub(super) fn read(
    store: &FjallNodeStore,
    family: RowFamily,
) -> Result<Option<Vec<u8>>, StorageError> {
    let snapshot = store.db.snapshot();
    let mut records = BTreeMap::new();
    let mut total = 0_usize;
    for guard in snapshot.prefix(store.keyspace(family.namespace()), family.root()) {
        let (key, value) = guard
            .into_inner()
            .map_err(|error| backend_failure(family.namespace(), error))?;
        if !family.owns(&key) {
            continue;
        }
        if key.as_ref() != family.root().as_bytes() && value.len() > codec::ROW_BYTES {
            return Err(corruption(
                family.namespace(),
                "oversized segmented authority row",
            ));
        }
        total = total
            .checked_add(key.len())
            .and_then(|n| n.checked_add(value.len()))
            .ok_or_else(|| corruption(family.namespace(), "row family byte overflow"))?;
        if records.len() >= MAX_ENTRIES || total > MAX_BYTES || key.len() > 4096 {
            return Err(corruption(
                family.namespace(),
                "row family exceeds inspection limits",
            ));
        }
        records.insert(key.to_vec(), value.to_vec());
    }
    decode_detached_rows(family, &records, false)
}

impl FjallNodeStore {
    /// Persist header metadata and the block-index projection atomically.
    pub fn save_header_entries(
        &self,
        entries: &[HeaderEntry],
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let headers = prepare(RowFamily::Headers, encode_header_entries(entries)?)?;
        let index = prepare(RowFamily::BlockIndex, encode_block_index_entries(entries)?)?;
        let _publication = self.filter_publication_guard()?;
        self.invalidate_validation_coverage()?;
        let mut batch = self.db.batch();
        if let Some(mode) = fjall_persist_mode(mode) {
            batch = batch.durability(Some(mode));
        }
        append_replacement(self, &mut batch, headers)?;
        append_replacement(self, &mut batch, index)?;
        batch
            .commit()
            .map_err(|error| backend_failure(StorageNamespace::Headers, error))
    }

    /// Load the complete persisted header projection as untrusted data.
    pub fn load_header_entries(&self) -> Result<Option<StoredHeaderEntries>, StorageError> {
        read(self, RowFamily::Headers)?
            .map(|bytes| decode_header_entries(&bytes))
            .transpose()
    }

    /// Load the complete persisted index projection as untrusted data.
    pub fn load_block_index_entries(&self) -> Result<Option<StoredHeaderEntries>, StorageError> {
        read(self, RowFamily::BlockIndex)?
            .map(|bytes| decode_block_index_entries(&bytes))
            .transpose()
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod durable_tests;
#[cfg(test)]
pub(crate) use durable_tests::{DurableRowsMeasurement, measure_detached_rows_for_test};

#[cfg(test)]
thread_local! {
    static METADATA_FAULT: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(super) fn metadata_commit_fault(after: bool) -> Result<(), StorageError> {
    if METADATA_FAULT.get() == Some(after) {
        METADATA_FAULT.set(None);
        return Err(StorageError::BackendFailure {
            namespace: StorageNamespace::Chainstate,
            message: "injected metadata batch failure".into(),
            action: StorageRecoveryAction::Restart,
        });
    }
    Ok(())
}
