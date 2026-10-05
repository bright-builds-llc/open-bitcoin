// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Fjall-backed `ChainstateStore` that leaves leftover snapshots unread.

use std::fmt;

use super::{FlushLifecycle, ManagedChainstate};
use crate::storage::fjall_store::filters::BasicFilterAppendProof;
use crate::storage::{StorageNamespace, StorageRecoveryAction};
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
    accepted: (u32, BlockHash),
}

fn receipt_error(detail: &str) -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::BlockIndex,
        detail: detail.to_owned(),
        action: StorageRecoveryAction::Restart,
    }
}

impl ValidatedChainstateLineage {
    pub(in crate::chainstate) fn observe(&mut self, position: &ChainPosition) -> bool {
        if self.accepted.0.checked_add(1) != Some(position.height)
            || self.accepted.1 != position.previous_block_hash()
        {
            return false;
        }
        self.accepted = (position.height, position.block_hash);
        true
    }

    pub(in crate::chainstate) fn maybe_prepare(
        &self,
        maybe_tip: Option<&ChainPosition>,
    ) -> Result<Option<PendingValidatedFlush>, StorageError> {
        if maybe_tip.map(|p| (p.height, p.block_hash)) != Some(self.accepted) {
            return Err(receipt_error("untracked BASIC live flush endpoint"));
        }
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
            accepted: self.accepted,
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
            || self.accepted != lineage.accepted
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
                    accepted: (tip.height, tip.block_hash),
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
mod proof_tests {
    use super::super::flush_lifecycle::managed_fixture;
    use super::*;
    use open_bitcoin_core::chainstate::{CoinsCache, FlushMode, FlushPolicyTime};

    fn flush(manager: &mut ManagedChainstate<FjallChainstateStore, FjallCoinsView>) {
        manager
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(10),
                u64::MAX,
            )
            .expect("real managed flush");
    }

    #[test]
    fn phase157_proof_replayed_old_completion_cannot_advance_live_authority() {
        // Arrange
        let (path, store, mut manager, _) = managed_fixture("proof-replayed");
        let pending = manager
            .maybe_validated_lineage
            .as_ref()
            .expect("tracked")
            .maybe_prepare(manager.chainstate.tip())
            .expect("receipt")
            .expect("live proof");
        flush(&mut manager);
        let current = store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("live");
        let completed = pending
            .complete(
                manager.maybe_validated_lineage.as_ref(),
                manager.chainstate.tip(),
            )
            .expect("deliberately stale sealed receipt");
        // Act
        assert!(FlushPersistSink::confirm_validated_flush(&mut store.clone(), completed).is_err());
        // Assert
        store
            .check_basic_filter_append_proof(&current)
            .expect("current authority unchanged");
        drop(current);
        drop(manager);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_foreign_completion_refuses_identical_public_facts() {
        // Arrange
        let (first_path, first, first_manager, _) = managed_fixture("proof-completion-first");
        let (second_path, second, second_manager, _) = managed_fixture("proof-completion-second");
        let pending = first_manager
            .maybe_validated_lineage
            .as_ref()
            .expect("tracked")
            .maybe_prepare(first_manager.chainstate.tip())
            .expect("receipt")
            .expect("live");
        let completed = pending
            .complete(
                first_manager.maybe_validated_lineage.as_ref(),
                first_manager.chainstate.tip(),
            )
            .expect("foreign test receipt");
        let current = second
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("live");
        // Act
        assert!(FlushPersistSink::confirm_validated_flush(&mut second.clone(), completed).is_err());
        // Assert
        second
            .check_basic_filter_append_proof(&current)
            .expect("second authority unchanged");
        drop(current);
        drop(first_manager);
        drop(second_manager);
        drop(first);
        drop(second);
        std::fs::remove_dir_all(first_path).expect("cleanup");
        std::fs::remove_dir_all(second_path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_reconstruction_never_inherits_private_lineage() {
        // Arrange
        let (path, store, manager, _) = managed_fixture("proof-reconstruction");
        let lifecycle = manager.flush_lifecycle.clone();
        let (sink, state) = manager.into_parts();
        // Act
        let mut reconstructed = ManagedChainstate::from_chainstate(sink, state, lifecycle);
        assert!(reconstructed.maybe_validated_lineage.is_none());
        flush(&mut reconstructed);
        // Assert
        assert!(
            store
                .maybe_basic_filter_append_proof()
                .expect("no inherited authority")
                .is_none()
        );
        drop(reconstructed);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_identical_test_replacement_clears_provenance() {
        // Arrange
        let (path, store, mut manager, _) = managed_fixture("proof-replacement");
        let (positions, counts) = store.load_chain_meta_for_open().expect("metadata");
        let replacement = Chainstate::from_coins_cache(
            CoinsCache::from_parent(store.coins_view()),
            positions,
            Default::default(),
            counts,
        );
        // Act
        manager.install_chainstate_for_test(replacement);
        assert!(manager.maybe_validated_lineage.is_none());
        flush(&mut manager);
        // Assert
        assert!(
            store
                .maybe_basic_filter_append_proof()
                .expect("no reseeding")
                .is_none()
        );
        drop(manager);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_memory_clone_drops_even_test_injected_lineage() {
        // Arrange
        let (path, store, mut manager, _) = managed_fixture("proof-clone");
        let mut memory =
            ManagedChainstate::from_store(super::super::MemoryChainstateStore::default());
        memory.maybe_validated_lineage = manager.maybe_validated_lineage.take();
        // Act
        let cloned = memory.clone();
        // Assert
        assert!(memory.maybe_validated_lineage.is_some());
        assert!(cloned.maybe_validated_lineage.is_none());
        drop(cloned);
        drop(memory);
        drop(manager);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_old_manager_cannot_follow_reenabled_generation() {
        // Arrange
        let (path, store, mut manager, _) = managed_fixture("proof-generation");
        store.disable_basic_filter_index().expect("disable");
        let positions = store.load_chain_meta_for_open().expect("metadata").0;
        let fence = open_bitcoin_core::chainstate::VerifiedChainstateFence::new(
            store.coins_view().best_block().expect("B"),
            Some(&positions),
        )
        .expect("full recovered fence");
        store
            .enable_basic_filter_index(&fence)
            .expect("complete enable/preflight");
        let current = store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("new generation");
        // Act
        assert!(
            manager
                .flush_with_mode(
                    FlushMode::Always,
                    FlushPolicyTime::from_unix_seconds(10),
                    u64::MAX
                )
                .is_err()
        );
        // Assert
        store
            .check_basic_filter_append_proof(&current)
            .expect("new authority unchanged");
        drop(current);
        drop(manager);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_raw_snapshot_seed_cannot_refresh_any_clone() {
        // Arrange
        let (path, store, manager, _) = managed_fixture("proof-raw-seed");
        let old = store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("live");
        let snapshot = store
            .wallet_scan_chainstate_snapshot()
            .expect("snapshot")
            .expect("present");
        let before = store.maybe_basic_filter_state().expect("safe state");
        // Act
        store
            .clone()
            .seed_coins_from_snapshot(&snapshot)
            .expect("raw seed publication");
        // Assert
        assert!(store.check_basic_filter_append_proof(&old).is_err());
        assert!(
            store
                .clone()
                .maybe_basic_filter_append_proof()
                .expect("no authority")
                .is_none()
        );
        assert_eq!(
            store
                .maybe_basic_filter_state()
                .expect("safe state unchanged"),
            before
        );
        drop(old);
        drop(manager);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}
