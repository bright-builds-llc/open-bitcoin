// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Fjall-backed `ChainstateStore` that leaves leftover snapshots unread.

use std::fmt;

use super::{FlushLifecycle, ManagedChainstate};
use crate::storage::fjall_store::filters::BasicFilterAppendProof;
use crate::storage::fjall_store::filters::{CompletedBasicFilterReorg, PreparedBasicFilterReorg};
use crate::storage::{StorageNamespace, StorageRecoveryAction};
use open_bitcoin_core::chainstate::AcceptedChainstateReorg;
use open_bitcoin_core::chainstate::{Chainstate, filter_index::lifecycle::IndexGeneration};

/// Opaque achieved completion; only a tracked manager's own successful flush creates it.
pub struct CompletedValidatedFlush {
    pending: PendingValidatedFlush,
}

pub(in crate::chainstate) struct PendingValidatedFlush {
    store: FjallNodeStore,
    proof: BasicFilterAppendProof,
    accepted: (u32, BlockHash),
    metadata_revision: u64,
}

pub(in crate::chainstate) struct ValidatedChainstateLineage {
    store: FjallNodeStore,
    generation: IndexGeneration,
    branch: BlockHash,
    maybe_accepted: Option<(u32, BlockHash)>,
    maybe_pending_reorg: Option<PendingBasicFilterReorg>,
    preview_frozen: bool,
}

mod reorg;
use reorg::PendingBasicFilterReorg;
pub(crate) use reorg::{ValidatedBasicFilterAppendPositions, ValidatedBasicFilterReorg};

fn receipt_error(detail: &str) -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::BlockIndex,
        detail: detail.to_owned(),
        action: StorageRecoveryAction::Restart,
    }
}

impl ValidatedChainstateLineage {
    pub(in crate::chainstate) fn observe(&mut self, position: &ChainPosition) -> bool {
        let Some(accepted) = self.maybe_accepted else {
            return false;
        };
        if self.preview_frozen
            || self.maybe_pending_reorg.is_some()
            || accepted.0.checked_add(1) != Some(position.height)
            || accepted.1 != position.previous_block_hash()
        {
            return false;
        }
        self.maybe_accepted = Some((position.height, position.block_hash));
        true
    }

    pub(in crate::chainstate) fn maybe_prepare(
        &self,
        maybe_tip: Option<&ChainPosition>,
    ) -> Result<Option<PendingValidatedFlush>, StorageError> {
        if self.preview_frozen
            || self.maybe_pending_reorg.is_some()
            || maybe_tip.map(|p| (p.height, p.block_hash)) != self.maybe_accepted
        {
            return Err(receipt_error("untracked BASIC live flush endpoint"));
        }
        let Some(accepted) = self.maybe_accepted else {
            return Ok(None);
        };
        let Some(proof) = self.store.maybe_basic_filter_append_proof()? else {
            return Ok(None);
        };
        if proof.generation() != self.generation || proof.branch_identity() != self.branch {
            return Err(receipt_error("stale BASIC validated lineage"));
        }
        let Some(metadata_revision) = proof.revision().checked_add(2) else {
            self.store.invalidate_basic_filter_append()?;
            return Err(receipt_error("BASIC own-publication revision exhausted"));
        };
        Ok(Some(PendingValidatedFlush {
            store: self.store.clone(),
            proof,
            accepted,
            metadata_revision,
        }))
    }
}

impl PendingValidatedFlush {
    pub(in crate::chainstate) fn complete(
        self,
        maybe_lineage: Option<&ValidatedChainstateLineage>,
        maybe_tip: Option<&ChainPosition>,
    ) -> Result<CompletedValidatedFlush, StorageError> {
        let Some(lineage) = maybe_lineage else {
            return Err(self.reject("lost BASIC validated lineage"));
        };
        if !self.store.shares_basic_filter_store(&lineage.store)
            || Some(self.accepted) != lineage.maybe_accepted
            || lineage.maybe_pending_reorg.is_some()
            || lineage.preview_frozen
            || maybe_tip.map(|p| (p.height, p.block_hash)) != Some(self.accepted)
            || self.proof.generation() != lineage.generation
            || self.proof.branch_identity() != lineage.branch
        {
            return Err(self.reject("changed BASIC validated flush endpoint"));
        }
        Ok(CompletedValidatedFlush { pending: self })
    }

    pub(in crate::chainstate) fn abort(self) -> Result<(), StorageError> {
        self.store.discard_basic_filter_flush(&self.proof)
    }

    fn reject(self, detail: &str) -> StorageError {
        match self.abort() {
            Ok(()) => receipt_error(detail),
            Err(cleanup) => receipt_error(&format!("{detail}; receipt cleanup: {cleanup}")),
        }
    }
}

impl CompletedValidatedFlush {
    pub(crate) fn belongs_to(&self, store: &FjallNodeStore) -> bool {
        self.pending.store.shares_basic_filter_store(store)
    }
    pub(crate) fn initial_proof(&self) -> &BasicFilterAppendProof {
        &self.pending.proof
    }
    pub(crate) fn accepted_endpoint(&self) -> (u32, BlockHash) {
        self.pending.accepted
    }
    pub(crate) fn metadata_revision(&self) -> u64 {
        self.pending.metadata_revision
    }
}

impl ManagedChainstate<FjallChainstateStore, FjallCoinsView> {
    /// Actual recovered runtime only; generic/public construction never inherits provenance.
    pub(crate) fn from_recovered_chainstate(
        store: FjallChainstateStore,
        chainstate: Chainstate<FjallCoinsView>,
        lifecycle: FlushLifecycle,
    ) -> Result<Self, StorageError> {
        if !chainstate.coins().parent().belongs_to(store.inner()) {
            return Err(receipt_error("foreign BASIC recovered coins parent"));
        }
        let maybe_proof = store.inner().maybe_basic_filter_append_proof()?;
        let maybe_validated_lineage = maybe_proof
            .map(|proof| {
                let tip = chainstate
                    .tip()
                    .ok_or_else(|| receipt_error("absent BASIC recovered live endpoint"))?;
                if proof.durable_tip() != (tip.height, tip.block_hash) {
                    return Err(receipt_error("different BASIC recovered live endpoint"));
                }
                Ok(ValidatedChainstateLineage {
                    store: store.inner().clone(),
                    generation: proof.generation(),
                    branch: proof.branch_identity(),
                    maybe_accepted: Some((tip.height, tip.block_hash)),
                    maybe_pending_reorg: None,
                    preview_frozen: false,
                })
            })
            .transpose()?;
        Ok(Self {
            store,
            chainstate,
            flush_lifecycle: lifecycle,
            maybe_validated_lineage,
            maybe_basic_index_owner: None,
        })
    }
}

use open_bitcoin_core::{
    chainstate::{
        BlockUndo, ChainPosition, ChainstateError, ChainstateSnapshot, Coin, CoinsBatch, CoinsView,
        PruneLockInfo,
    },
    primitives::{Block, BlockHash, OutPoint},
};
use open_bitcoin_network::HeaderEntry;

use super::{ChainstateStore, FlushPersistSink, PruneProtectionSnapshot};
use crate::storage::fjall_store::{PayloadUsageRevision, RetainedPayloadUsage};
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
    fn confirm_validated_flush(
        &mut self,
        completed: super::CompletedValidatedFlush,
    ) -> Result<(), StorageError> {
        FlushPersistSink::confirm_validated_flush(&mut self.store, completed)
    }
    fn retained_payload_usage(
        &self,
        active_chain: &[ChainPosition],
    ) -> Result<RetainedPayloadUsage, StorageError> {
        self.store.retained_payload_usage(active_chain)
    }

    fn payload_usage_revision(&self) -> Result<PayloadUsageRevision, StorageError> {
        self.store.payload_usage_revision()
    }

    fn load_prune_locks(&self) -> Result<Vec<PruneLockInfo>, StorageError> {
        self.store.load_prune_locks()
    }

    fn load_prune_protection(&self) -> Result<PruneProtectionSnapshot, StorageError> {
        self.store.load_prune_protection()
    }

    fn sync_prune_locks(&self, locks: &[PruneLockInfo]) -> Result<(), StorageError> {
        self.store.sync_prune_locks(locks)
    }

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

    fn disk_free_bytes(&self) -> u64 {
        FlushPersistSink::disk_free_bytes(&self.store)
    }

    fn commit_paired_unlink(
        &mut self,
        height: u32,
        block_hash: BlockHash,
    ) -> Result<crate::storage::fjall_store::PairedDeleteOutcome, StorageError> {
        FlushPersistSink::commit_paired_unlink(&mut self.store, height, block_hash)
    }

    fn record_successful_prune_batch(
        &mut self,
        deleted_heights: &[u32],
    ) -> Result<(), StorageError> {
        FlushPersistSink::record_successful_prune_batch(&mut self.store, deleted_heights)
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

#[cfg(test)]
#[path = "fjall_store/tests.rs"]
pub(crate) mod proof_tests;
