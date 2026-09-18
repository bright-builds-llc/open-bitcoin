// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::primitives::BlockHash;

use super::{FjallNodeStore, StorageError, StorageNamespace};

impl FjallNodeStore {
    /// Returns whether payload bytes exist for `block_hash`.
    ///
    /// Presence is a `contains_key` probe on `block:<64-hex>` and does not decode
    /// the body. Analogous to Knots `HaveBlockData` / `CheckBlockDataAvailability`.
    pub fn has_block(&self, block_hash: BlockHash) -> Result<bool, StorageError> {
        self.block_index
            .contains_key(super::block_key(block_hash))
            .map_err(|error| super::backend_failure(StorageNamespace::BlockIndex, error))
    }
}
