// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::{FjallNodeStore, SNAPSHOT_KEY};
use crate::network::{PreparedSnapshotWrite, SnapshotWriteReceipt};
use crate::storage::{
    MempoolSnapshot, PersistMode, StorageError, StorageNamespace, snapshot_codec,
};

impl FjallNodeStore {
    /// Execute one owned current-schema snapshot write outside lifecycle authority.
    ///
    /// The returned receipt proves that encoding and the requested persistence
    /// mode both succeeded. Failures consume the prepared write without
    /// creating achieved-effect credit.
    pub fn execute_prepared_mempool_snapshot_write(
        &self,
        prepared: PreparedSnapshotWrite,
        mode: PersistMode,
    ) -> Result<SnapshotWriteReceipt, StorageError> {
        execute_prepared_mempool_snapshot_write_with(prepared, mode, |snapshot, mode| {
            self.save_mempool_snapshot(snapshot, mode)
        })
    }

    #[cfg(test)]
    pub(crate) fn execute_prepared_mempool_snapshot_write_with<F>(
        prepared: PreparedSnapshotWrite,
        mode: PersistMode,
        save: F,
    ) -> Result<SnapshotWriteReceipt, StorageError>
    where
        F: FnOnce(&MempoolSnapshot, PersistMode) -> Result<(), StorageError>,
    {
        execute_prepared_mempool_snapshot_write_with(prepared, mode, save)
    }

    /// Persist the accepted-mempool snapshot owned by Open Bitcoin.
    pub fn save_mempool_snapshot(
        &self,
        snapshot: &MempoolSnapshot,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let bytes = snapshot_codec::encode_mempool_snapshot(snapshot)?;
        self.put_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY, bytes, mode)
    }

    /// Load the accepted-mempool snapshot, if present.
    pub fn load_mempool_snapshot(&self) -> Result<Option<MempoolSnapshot>, StorageError> {
        self.get_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY)?
            .map(|bytes| snapshot_codec::decode_mempool_snapshot(&bytes))
            .transpose()
    }

    /// Remove the persisted accepted-mempool snapshot.
    pub fn clear_mempool_snapshot(&self, mode: PersistMode) -> Result<(), StorageError> {
        self.remove_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY, mode)
    }
}

fn execute_prepared_mempool_snapshot_write_with<F>(
    prepared: PreparedSnapshotWrite,
    mode: PersistMode,
    save: F,
) -> Result<SnapshotWriteReceipt, StorageError>
where
    F: FnOnce(&MempoolSnapshot, PersistMode) -> Result<(), StorageError>,
{
    let (snapshot, capability) = prepared.into_parts();
    save(&snapshot, mode)?;
    Ok(capability.acknowledge_write())
}
