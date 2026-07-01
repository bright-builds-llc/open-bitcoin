// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::{FjallNodeStore, SNAPSHOT_KEY};
use crate::storage::{
    MempoolSnapshot, PersistMode, StorageError, StorageNamespace, snapshot_codec,
};

impl FjallNodeStore {
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
