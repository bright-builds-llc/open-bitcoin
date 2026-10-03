#![cfg_attr(not(test), allow(dead_code))]
// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h
// - packages/bitcoin-knots/src/validation.h

//! Paired deletion of one height's block payload and undo.
//!
//! `have_pruned` is inserted in the delete batch. That is stricter than Knots,
//! which sets `m_have_pruned` before `UnlinkPrunedFiles`. Startup finishes the
//! height named by `prune_intent` or refuses closed before the node can flush.

use fjall::PersistMode as FjallPersistMode;
use open_bitcoin_core::{
    chainstate::{
        ChainPosition, CoinsView, PruneLockInfo, height_forbidden_by_any_lock,
        height_inside_keep_window,
    },
    primitives::BlockHash,
};

use super::coins::undo_key;
use super::{
    FjallNodeStore, StorageError, StorageNamespace, backend_failure, block_key, corruption,
};
use crate::storage::coins_view::FjallCoinsView;

mod records;

#[cfg(test)]
pub(in crate::storage::fjall_store) use records::{PRUNE_LOCKS_KEY, PRUNE_SUMMARY_KEY};

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
    pub fn load_have_pruned(&self) -> Result<bool, StorageError> {
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
        self.with_payload_mutation(|| self.commit_paired_delete_inner(height, block_hash))
    }

    fn commit_paired_delete_inner(
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

/// Finishes the one height named by `prune_intent`, or refuses closed.
///
/// Returns `Ok(())` when the intent is absent. A present intent is removed
/// only when its hash is the active-chain position at that height, the height
/// is outside the keep window and any supplied locks, and coins best-block is
/// absent or equal to the active tip. Refusal leaves remaining payload keys
/// in place.
pub(crate) fn resume_prune_intent(
    store: &FjallNodeStore,
    locks: &[PruneLockInfo],
) -> Result<(), StorageError> {
    let Some(intent) = load_resume_intent(store)? else {
        return Ok(());
    };
    let (active_chain, _maybe_confirmed_txid_counts) = store.load_chain_meta_for_open()?;
    let Some(tip) = active_chain.last() else {
        return Err(fail_closed("active chain is empty"));
    };
    ensure_intent_may_finish(store, &intent, &active_chain, tip, locks)?;
    finish_intent(store, intent)
}

fn load_resume_intent(store: &FjallNodeStore) -> Result<Option<PruneIntent>, StorageError> {
    match store.maybe_prune_intent() {
        Ok(maybe_intent) => Ok(maybe_intent),
        Err(error) => Err(as_fail_closed(error)),
    }
}

fn ensure_intent_may_finish(
    store: &FjallNodeStore,
    intent: &PruneIntent,
    active_chain: &[ChainPosition],
    tip: &ChainPosition,
    locks: &[PruneLockInfo],
) -> Result<(), StorageError> {
    let hash_matches = active_chain.iter().any(|position| {
        position.height == intent.height && position.block_hash == intent.block_hash
    });
    if !hash_matches {
        return Err(fail_closed(
            "prune_intent hash is not the active-chain block at that height",
        ));
    }
    if height_inside_keep_window(tip.height, intent.height) {
        return Err(fail_closed("prune_intent height is inside the keep window"));
    }
    if height_forbidden_by_any_lock(intent.height, locks) {
        return Err(fail_closed(
            "prune_intent height is forbidden by a prune lock",
        ));
    }
    let maybe_best_block = FjallCoinsView::from_store(store)
        .best_block()
        .map_err(|error| fail_closed(error.to_string()))?;
    if let Some(best_block) = maybe_best_block
        && best_block != tip.block_hash
    {
        return Err(fail_closed("coins best-block is not the active-chain tip"));
    }
    Ok(())
}

fn finish_intent(store: &FjallNodeStore, intent: PruneIntent) -> Result<(), StorageError> {
    let block_present = store.has_block(intent.block_hash)?;
    let undo_present = store.has_undo(intent.block_hash)?;
    if !block_present && !undo_present {
        return sync_clear_prune_intent(store);
    }

    let outcome = store
        .commit_paired_delete(intent.height, intent.block_hash)
        .map_err(as_fail_closed)?;
    if outcome == PairedDeleteOutcome::AlreadyAbsent {
        sync_clear_prune_intent(store)?;
    }

    let block_still_present = store.has_block(intent.block_hash)?;
    let undo_still_present = store.has_undo(intent.block_hash)?;
    if block_still_present || undo_still_present {
        return Err(fail_closed("paired delete left a block or undo key"));
    }
    if outcome == PairedDeleteOutcome::DeletedLiveMate {
        store.record_successful_prune_batch(&[intent.height])?;
    }
    Ok(())
}

fn sync_clear_prune_intent(store: &FjallNodeStore) -> Result<(), StorageError> {
    let mut batch = store.db.batch().durability(Some(FjallPersistMode::SyncAll));
    batch.remove(&store.block_index, PRUNE_INTENT_KEY);
    batch
        .commit()
        .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))
}

fn fail_closed(reason: impl std::fmt::Display) -> StorageError {
    corruption(
        StorageNamespace::BlockIndex,
        format!("fail_closed prune resume stopped closed and did not reindex: {reason}"),
    )
}

fn as_fail_closed(error: StorageError) -> StorageError {
    match error {
        StorageError::Corruption { detail, .. } => fail_closed(detail),
        other => other,
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
