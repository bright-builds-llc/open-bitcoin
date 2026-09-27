#![cfg_attr(not(test), allow(dead_code))]
// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/validation.h

//! Paired deletion of one height's block payload and undo.
//!
//! `have_pruned` is inserted in the delete batch. That is stricter than Knots,
//! which sets `m_have_pruned` before `UnlinkPrunedFiles`. Plan 148-02 calls
//! these methods from the flush owner.

use fjall::PersistMode as FjallPersistMode;
use open_bitcoin_core::primitives::BlockHash;

use super::coins::undo_key;
use super::{
    FjallNodeStore, StorageError, StorageNamespace, backend_failure, block_key, corruption,
};

/// Block-index key recorded only inside a committed non-empty paired delete.
pub(crate) const HAVE_PRUNED_KEY: &str = "have_pruned";

/// Block-index key naming the height whose delete has been requested.
pub(crate) const PRUNE_INTENT_KEY: &str = "prune_intent";

const HAVE_PRUNED_VALUE: u8 = 1;
const PRUNE_INTENT_LEN: usize = 36;

/// Outcome of one paired payload delete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairedDeleteOutcome {
    /// At least one mate was present, and both keys are absent after the batch.
    DeletedLiveMate,
    /// Both mates were already absent, so nothing was written.
    AlreadyAbsent,
}

/// One height and hash that a later reopen can finish or refuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PruneIntent {
    pub height: u32,
    pub block_hash: BlockHash,
}

impl FjallNodeStore {
    /// Returns whether a prior paired delete committed `have_pruned`.
    pub(crate) fn load_have_pruned(&self) -> Result<bool, StorageError> {
        self.block_index
            .contains_key(HAVE_PRUNED_KEY)
            .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))
    }

    /// Loads the interrupted-prune intent.
    ///
    /// `Ok(None)` when the key is absent. A 36-byte value decodes. Any other
    /// length is `Corruption` with `StorageRecoveryAction::Repair`.
    pub(crate) fn maybe_prune_intent(&self) -> Result<Option<PruneIntent>, StorageError> {
        let maybe_bytes = self.get_bytes(StorageNamespace::BlockIndex, PRUNE_INTENT_KEY)?;
        let Some(bytes) = maybe_bytes else {
            return Ok(None);
        };
        Ok(Some(decode_prune_intent(&bytes)?))
    }

    /// Sync-inserts `prune_intent` only. Does not insert `have_pruned`.
    pub(crate) fn sync_prune_intent(&self, intent: PruneIntent) -> Result<(), StorageError> {
        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        batch.insert(
            &self.block_index,
            PRUNE_INTENT_KEY,
            encode_prune_intent(intent),
        );
        batch
            .commit()
            .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))
    }

    /// Removes one hash's block payload and undo in one Sync batch.
    ///
    /// A live mate records `prune_intent`, then a second Sync batch tombstones
    /// both keys, inserts `have_pruned`, and clears the intent. Both mates
    /// already absent returns [`PairedDeleteOutcome::AlreadyAbsent`] without a
    /// write. A commit error is returned as `BackendFailure` and is not retried.
    pub(crate) fn commit_paired_delete(
        &self,
        height: u32,
        block_hash: BlockHash,
    ) -> Result<PairedDeleteOutcome, StorageError> {
        let block_present = self.has_block(block_hash)?;
        let undo_present = self.has_undo(block_hash)?;
        if !block_present && !undo_present {
            return Ok(PairedDeleteOutcome::AlreadyAbsent);
        }

        self.sync_prune_intent(PruneIntent { height, block_hash })?;

        let mut batch = self.db.batch().durability(Some(FjallPersistMode::SyncAll));
        batch.remove(&self.block_index, block_key(block_hash));
        batch.remove(&self.chainstate, undo_key(block_hash));
        batch.insert(&self.block_index, HAVE_PRUNED_KEY, vec![HAVE_PRUNED_VALUE]);
        batch.remove(&self.block_index, PRUNE_INTENT_KEY);
        batch
            .commit()
            .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?;

        let block_still_present = self.has_block(block_hash)?;
        let undo_still_present = self.has_undo(block_hash)?;
        if block_still_present || undo_still_present {
            return Err(corruption(
                StorageNamespace::BlockIndex,
                "paired delete left a block or undo key",
            ));
        }
        Ok(PairedDeleteOutcome::DeletedLiveMate)
    }
}

fn encode_prune_intent(intent: PruneIntent) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PRUNE_INTENT_LEN);
    bytes.extend_from_slice(&intent.height.to_le_bytes());
    bytes.extend_from_slice(intent.block_hash.as_bytes());
    bytes
}

fn decode_prune_intent(bytes: &[u8]) -> Result<PruneIntent, StorageError> {
    let Ok(encoded) = <[u8; PRUNE_INTENT_LEN]>::try_from(bytes) else {
        return Err(corruption(
            StorageNamespace::BlockIndex,
            "prune_intent must be 36 bytes",
        ));
    };
    let height = u32::from_le_bytes([encoded[0], encoded[1], encoded[2], encoded[3]]);
    let mut hash_bytes = [0_u8; 32];
    hash_bytes.copy_from_slice(&encoded[4..]);
    Ok(PruneIntent {
        height,
        block_hash: BlockHash::from_byte_array(hash_bytes),
    })
}
