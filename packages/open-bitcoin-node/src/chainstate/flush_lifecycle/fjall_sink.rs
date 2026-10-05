// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Concrete storage effects forwarded into the existing paired-delete owner.

use super::*;

impl FlushPersistSink for FjallNodeStore {
    fn retained_payload_usage(
        &self,
        active_chain: &[ChainPosition],
    ) -> Result<RetainedPayloadUsage, StorageError> {
        FjallNodeStore::retained_payload_usage(self, active_chain)
    }
    fn payload_usage_revision(&self) -> Result<PayloadUsageRevision, StorageError> {
        FjallNodeStore::payload_usage_revision(self)
    }
    fn load_prune_locks(&self) -> Result<Vec<PruneLockInfo>, StorageError> {
        FjallNodeStore::load_prune_locks(self)
    }
    fn load_prune_protection(&self) -> Result<PruneProtectionSnapshot, StorageError> {
        FjallNodeStore::load_prune_protection(self)
    }
    fn sync_prune_locks(&self, locks: &[PruneLockInfo]) -> Result<(), StorageError> {
        FjallNodeStore::sync_prune_locks(self, locks)
    }
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError> {
        FjallNodeStore::save_block(self, block, PersistMode::Flush).map(|_| ())
    }
    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError> {
        FjallNodeStore::save_undo(self, hash, undo, PersistMode::Flush)
    }
    fn persist_header_entries(&mut self, entries: &[HeaderEntry]) -> Result<(), StorageError> {
        if entries.is_empty() {
            return Ok(());
        }
        FjallNodeStore::save_header_entries(self, entries, PersistMode::Flush)
    }
    fn persist_chain_meta(&mut self, active_chain: &[ChainPosition]) -> Result<(), StorageError> {
        FjallNodeStore::save_chain_meta(self, active_chain, PersistMode::Flush)
    }
    fn disk_free_bytes(&self) -> u64 {
        probe_disk_free_bytes(self.datadir())
    }
    fn commit_paired_unlink(
        &mut self,
        height: u32,
        block_hash: BlockHash,
    ) -> Result<PairedDeleteOutcome, StorageError> {
        self.commit_paired_delete(height, block_hash)
    }
    fn record_successful_prune_batch(
        &mut self,
        deleted_heights: &[u32],
    ) -> Result<(), StorageError> {
        FjallNodeStore::record_successful_prune_batch(self, deleted_heights)
    }
}
