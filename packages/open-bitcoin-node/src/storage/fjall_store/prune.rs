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
        ChainPosition, CoinsView, IndexPrefix, PruneLockInfo, VerifiedChainstateFence,
        filter_index::lifecycle::EffectiveIndexOwnership, height_forbidden_by_any_lock,
        height_inside_keep_window,
    },
    primitives::BlockHash,
};

use super::coins::undo_key;
use super::filters::PublicationControl;
use super::{
    FjallNodeStore, StorageError, StorageNamespace, backend_failure, block_key, corruption,
};
use crate::storage::coins_view::FjallCoinsView;

mod records;

#[cfg(test)]
pub(in crate::storage::fjall_store) use records::PRUNE_SUMMARY_KEY;
pub(in crate::storage::fjall_store) use records::{
    PRUNE_LOCKS_KEY, decode_prune_locks, encode_prune_locks,
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
    /// Load fresh validated facts for application; this is not deletion authority.
    pub fn load_prune_protection(
        &self,
    ) -> Result<crate::chainstate::PruneProtectionSnapshot, StorageError> {
        let control = self.filter_publication_guard()?;
        let maybe_owner = self.maybe_basic_filter_owner_guarded(&control)?;
        self.maybe_owned_prune_ancestry(maybe_owner)?;
        Ok(crate::chainstate::PruneProtectionSnapshot {
            maybe_owner,
            locks: self.load_prune_locks()?,
        })
    }
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
        let control = self.filter_publication_guard()?;
        self.check_prune_candidate_guarded(&control, intent)?;
        self.sync_prune_intent_guarded(intent)
    }

    fn sync_prune_intent_guarded(&self, intent: PruneIntent) -> Result<(), StorageError> {
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
        self.commit_paired_delete_observing(height, block_hash, || {})
    }

    fn commit_paired_delete_observing(
        &self,
        height: u32,
        block_hash: BlockHash,
        after_check: impl FnOnce(),
    ) -> Result<PairedDeleteOutcome, StorageError> {
        let control = self.filter_publication_guard()?;
        self.check_prune_candidate_guarded(&control, PruneIntent { height, block_hash })?;
        after_check();
        self.with_payload_mutation(|| self.commit_paired_delete_guarded(height, block_hash))
    }

    /// Channel-ordered observation of the actual guarded destructive path.
    #[cfg(test)]
    pub(in crate::storage::fjall_store) fn delete_after_protection_for_test(
        &self,
        height: u32,
        block_hash: BlockHash,
        after_check: impl FnOnce(),
    ) -> Result<PairedDeleteOutcome, StorageError> {
        self.commit_paired_delete_observing(height, block_hash, after_check)
    }

    fn check_prune_candidate_guarded(
        &self,
        control: &PublicationControl,
        intent: PruneIntent,
    ) -> Result<(), StorageError> {
        let maybe_owner = self.maybe_basic_filter_owner_guarded(control)?;
        let maybe_positions = self.maybe_owned_prune_ancestry(maybe_owner)?;
        check_owned_prune_candidate(maybe_owner, maybe_positions.as_deref(), intent)?;
        if height_forbidden_by_any_lock(intent.height, &self.load_prune_locks()?) {
            return Err(fail_closed(
                "prune height is forbidden by a current prune lock",
            ));
        }
        Ok(())
    }

    /// Current recovered coins and ancestry must still support the saved release.
    /// Normal metadata/coins effects are serialized by managed authority; the
    /// publication guard additionally excludes concurrent lifecycle changes.
    fn maybe_owned_prune_ancestry(
        &self,
        maybe_owner: Option<EffectiveIndexOwnership>,
    ) -> Result<Option<Vec<ChainPosition>>, StorageError> {
        let Some(owner) = maybe_owner.filter(|owner| owner.maybe_effective_protection().is_some())
        else {
            return Ok(None);
        };
        let view = self.coins_view();
        if !view.head_blocks().map_err(as_coins_fail_closed)?.is_empty() {
            return Err(fail_closed("unrecovered BASIC coins heads"));
        }
        let best = view.best_block().map_err(as_coins_fail_closed)?;
        let (positions, _) = self.load_chain_meta_for_open()?;
        let fence = VerifiedChainstateFence::new(best, Some(&positions)).map_err(fail_closed)?;
        let saved = owner.checkpoint();
        if fence
            .maybe_position(saved.fence_height())
            .is_none_or(|position| position.block_hash != saved.fence_hash())
        {
            return Err(fail_closed(
                "saved BASIC fence is outside current durable ancestry",
            ));
        }
        if let IndexPrefix::Committed(endpoint) = saved.checkpoint().prefix()
            && !fence
                .maybe_position(endpoint.height())
                .is_some_and(|position| {
                    position.block_hash == endpoint.block_hash()
                        && position.previous_block_hash() == endpoint.parent_hash()
                })
        {
            return Err(fail_closed(
                "saved BASIC checkpoint is outside current durable ancestry",
            ));
        }
        Ok(Some(positions))
    }

    fn commit_paired_delete_guarded(
        &self,
        height: u32,
        block_hash: BlockHash,
    ) -> Result<PairedDeleteOutcome, StorageError> {
        let block_present = self.has_block(block_hash)?;
        let undo_present = self.has_undo(block_hash)?;
        if !block_present && !undo_present {
            return Ok(PairedDeleteOutcome::AlreadyAbsent);
        }

        self.sync_prune_intent_guarded(PruneIntent { height, block_hash })?;

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
    let control = store.filter_publication_guard().map_err(as_fail_closed)?;
    let maybe_owner = store
        .maybe_basic_filter_owner_guarded(&control)
        .map_err(as_fail_closed)?;
    let Some(intent) = load_resume_intent(store)? else {
        return Ok(());
    };
    let maybe_positions = store
        .maybe_owned_prune_ancestry(maybe_owner)
        .map_err(as_fail_closed)?;
    check_owned_prune_candidate(maybe_owner, maybe_positions.as_deref(), intent)?;
    let mut current_locks = store.load_prune_locks().map_err(as_fail_closed)?;
    current_locks.extend_from_slice(locks);
    let active_chain = match maybe_positions {
        Some(positions) => positions,
        None => store.load_chain_meta_for_open()?.0,
    };
    let Some(tip) = active_chain.last() else {
        return Err(fail_closed("active chain is empty"));
    };
    ensure_intent_may_finish(store, &intent, &active_chain, tip, &current_locks)?;
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
        .with_payload_mutation(|| {
            store.commit_paired_delete_guarded(intent.height, intent.block_hash)
        })
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

fn as_coins_fail_closed(error: open_bitcoin_core::chainstate::ChainstateError) -> StorageError {
    fail_closed(error)
}

fn check_owned_prune_candidate(
    maybe_owner: Option<EffectiveIndexOwnership>,
    maybe_positions: Option<&[ChainPosition]>,
    intent: PruneIntent,
) -> Result<(), StorageError> {
    let Some(protection) =
        maybe_owner.and_then(EffectiveIndexOwnership::maybe_effective_protection)
    else {
        return Ok(());
    };
    if !maybe_positions.is_some_and(|positions| {
        positions
            .get(intent.height as usize)
            .is_some_and(|position| {
                position.height == intent.height && position.block_hash == intent.block_hash
            })
    }) {
        return Err(fail_closed(
            "prune height/hash is not current durable ancestry",
        ));
    }
    protection
        .check_prune_intent(intent.height)
        .map_err(fail_closed)
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
