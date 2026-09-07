// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Fjall-backed `ChainstateStore` that leaves leftover snapshots unread.

use std::fmt;

use open_bitcoin_core::{
    chainstate::{BlockUndo, ChainstateError, ChainstateSnapshot, Coin, CoinsBatch, CoinsView},
    primitives::{Block, BlockHash, OutPoint},
};
use open_bitcoin_network::HeaderEntry;

use super::{ChainstateStore, FlushPersistSink};
use crate::storage::{FjallNodeStore, PersistMode, StorageError, coins_view::FjallCoinsView};

/// Production chainstate store. Leftover snapshot blobs stay unread.
#[derive(Clone)]
pub struct FjallChainstateStore {
    store: FjallNodeStore,
}

impl FjallChainstateStore {
    pub fn from_store(store: FjallNodeStore) -> Self {
        Self { store }
    }

    pub const fn inner(&self) -> &FjallNodeStore {
        &self.store
    }
}

impl fmt::Debug for FjallChainstateStore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("FjallChainstateStore").finish()
    }
}

impl ChainstateStore for FjallChainstateStore {
    fn load_snapshot(&self) -> Option<ChainstateSnapshot> {
        None
    }

    fn save_snapshot(&mut self, _snapshot: ChainstateSnapshot) {}

    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        FjallCoinsView::from_store(&self.store).get_coin(outpoint)
    }

    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        FjallCoinsView::from_store(&self.store).have_coin(outpoint)
    }

    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        FjallCoinsView::from_store(&self.store).best_block()
    }

    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        FjallCoinsView::from_store(&self.store).head_blocks()
    }

    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        FjallCoinsView::from_store(&self.store).batch_write(writes, maybe_best_block)
    }

    fn load_undo(&self, block_hash: BlockHash) -> Result<Option<BlockUndo>, ChainstateError> {
        self.store.load_undo(block_hash).map_err(map_fjall)
    }

    fn save_undo(&mut self, block_hash: BlockHash, undo: BlockUndo) -> Result<(), ChainstateError> {
        self.store
            .save_undo(block_hash, &undo, PersistMode::Flush)
            .map_err(map_fjall)
    }
}

impl FlushPersistSink for FjallChainstateStore {
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError> {
        FlushPersistSink::persist_block(&mut self.store, block)
    }

    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError> {
        FlushPersistSink::persist_undo(&mut self.store, hash, undo)
    }

    fn persist_header_entries(&mut self, entries: &[HeaderEntry]) -> Result<(), StorageError> {
        FlushPersistSink::persist_header_entries(&mut self.store, entries)
    }

    fn persist_chain_meta(
        &mut self,
        active_chain: &[open_bitcoin_core::chainstate::ChainPosition],
    ) -> Result<(), StorageError> {
        FlushPersistSink::persist_chain_meta(&mut self.store, active_chain)
    }
}

fn map_fjall(error: StorageError) -> ChainstateError {
    match error {
        StorageError::InterruptedWrite { .. } => {
            ChainstateError::InterruptedWrite { heads: Vec::new() }
        }
        other => ChainstateError::CoinsStorage {
            detail: other.to_string(),
        },
    }
}
