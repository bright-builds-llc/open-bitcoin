// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::HashMap;
use std::fmt;

use open_bitcoin_core::{
    chainstate::{
        AnchoredBlock, BlockUndo, ChainPosition, ChainTransition, Chainstate, ChainstateError,
        ChainstateSnapshot, Coin, CoinsBatch, CoinsView, FlushMode, FlushPolicyTime,
        MemoryCoinsView, PruneLockInfo, PrunePlan, StagedChainstateConnect, StagedChainstateReorg,
    },
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::{Block, BlockHash, OutPoint},
};
use open_bitcoin_network::HeaderEntry;

use crate::storage::StorageError;

/// Flush failure that still names hashes whose paired delete already committed.
#[derive(Debug)]
pub(crate) struct FlushApplyError {
    pub(crate) error: StorageError,
    pub(crate) deleted_block_hashes: Vec<BlockHash>,
}

pub trait ChainstateStore: FlushPersistSink {
    /// Borrow the configured BASIC reader on this store; memory stores disable it.
    fn maybe_basic_filter_query_store(&self) -> Option<&crate::FjallNodeStore> {
        None
    }
    /// Concrete durable authority; memory stores have no persistent history.
    fn maybe_validation_history_store(&self) -> Option<crate::FjallNodeStore> {
        None
    }
    fn load_snapshot(&self) -> Option<ChainstateSnapshot>;
    fn save_snapshot(&mut self, snapshot: ChainstateSnapshot);
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError>;
    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError>;
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError>;
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError>;
    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError>;
    fn load_undo(&self, block_hash: BlockHash) -> Result<Option<BlockUndo>, ChainstateError>;
    fn save_undo(&mut self, block_hash: BlockHash, undo: BlockUndo) -> Result<(), ChainstateError>;
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryChainstateStore {
    maybe_snapshot: Option<ChainstateSnapshot>,
    coins: MemoryCoinsView,
    undo_by_block: HashMap<BlockHash, BlockUndo>,
}

impl MemoryChainstateStore {
    pub fn from_snapshot(snapshot: ChainstateSnapshot) -> Self {
        let coins = MemoryCoinsView::from_coins(
            snapshot.utxos.clone(),
            snapshot
                .active_chain
                .last()
                .map(|position| position.block_hash),
        );
        let undo_by_block = snapshot.undo_by_block.clone();
        Self {
            maybe_snapshot: Some(snapshot),
            coins,
            undo_by_block,
        }
    }

    pub fn snapshot(&self) -> Option<&ChainstateSnapshot> {
        self.maybe_snapshot.as_ref()
    }
}

impl ChainstateStore for MemoryChainstateStore {
    fn load_snapshot(&self) -> Option<ChainstateSnapshot> {
        self.maybe_snapshot.clone()
    }

    fn save_snapshot(&mut self, snapshot: ChainstateSnapshot) {
        self.maybe_snapshot = Some(snapshot);
    }

    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        self.coins.get_coin(outpoint)
    }

    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        self.coins.have_coin(outpoint)
    }

    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        self.coins.best_block()
    }

    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        self.coins.head_blocks()
    }

    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        self.coins.batch_write(writes, maybe_best_block)
    }

    fn load_undo(&self, block_hash: BlockHash) -> Result<Option<BlockUndo>, ChainstateError> {
        Ok(self.undo_by_block.get(&block_hash).cloned())
    }

    fn save_undo(&mut self, block_hash: BlockHash, undo: BlockUndo) -> Result<(), ChainstateError> {
        self.undo_by_block.insert(block_hash, undo);
        Ok(())
    }
}

impl FlushPersistSink for MemoryChainstateStore {
    fn load_prune_protection(&self) -> Result<PruneProtectionSnapshot, StorageError> {
        Ok(PruneProtectionSnapshot::no_index(Vec::new()))
    }
    fn persist_block(&mut self, _block: &Block) -> Result<(), StorageError> {
        Ok(())
    }

    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError> {
        self.save_undo(hash, undo.clone()).map_err(map_memory_store)
    }

    fn persist_header_entries(&mut self, _entries: &[HeaderEntry]) -> Result<(), StorageError> {
        Ok(())
    }

    fn persist_chain_meta(
        &mut self,
        _active_chain: &[open_bitcoin_core::chainstate::ChainPosition],
    ) -> Result<(), StorageError> {
        Ok(())
    }
}

fn map_memory_store(error: ChainstateError) -> StorageError {
    StorageError::Corruption {
        namespace: crate::storage::StorageNamespace::Chainstate,
        detail: error.to_string(),
        action: crate::storage::StorageRecoveryAction::Repair,
    }
}

pub struct ManagedChainstate<S, V: CoinsView = MemoryCoinsView> {
    store: S,
    chainstate: Chainstate<V>,
    pub(crate) flush_lifecycle: FlushLifecycle,
    maybe_validated_lineage: Option<flush_lifecycle::ValidatedChainstateLineage>,
    maybe_basic_index_owner: Option<filter_index::AcceptedBasicIndexOwner>,
    maybe_pending_validation: Option<crate::storage::validation_history::AcceptedValidationBatch>,
}

impl<S: Clone> Clone for ManagedChainstate<S, MemoryCoinsView> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            chainstate: Chainstate::from_snapshot(self.chainstate.snapshot()),
            flush_lifecycle: self.flush_lifecycle.clone(),
            maybe_validated_lineage: None,
            maybe_basic_index_owner: None,
            maybe_pending_validation: None,
        }
    }
}

impl<S: fmt::Debug> fmt::Debug for ManagedChainstate<S, MemoryCoinsView> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ManagedChainstate")
            .field("store", &self.store)
            .field("chainstate", &self.chainstate)
            .field("flush_lifecycle", &self.flush_lifecycle)
            .finish()
    }
}

impl<S: PartialEq> PartialEq for ManagedChainstate<S, MemoryCoinsView> {
    fn eq(&self, other: &Self) -> bool {
        self.store == other.store
            && self.chainstate == other.chainstate
            && self.flush_lifecycle == other.flush_lifecycle
    }
}

impl<S: Eq> Eq for ManagedChainstate<S, MemoryCoinsView> {}

/// Opaque, fully validated replacement for one connected block.
pub(crate) struct PreparedChainstateConnect {
    staged: StagedChainstateConnect,
    maybe_history: Option<validation_history::PreparedValidationAcceptance>,
    position: ChainPosition,
    maybe_filter_facts: Option<
        Result<filter_index::AcceptedBasicFilterFacts, filter_index::AcceptedBasicIndexFailure>,
    >,
}

impl PreparedChainstateConnect {
    pub(crate) const fn position(&self) -> &ChainPosition {
        &self.position
    }
}

/// Opaque, fully validated replacement for one complete reorg.
pub(crate) struct PreparedChainstateReorg {
    staged: StagedChainstateReorg,
    maybe_history: Option<validation_history::PreparedValidationAcceptance>,
    transition: ChainTransition,
    maybe_index: Option<filter_reorg::PreparedIndexReorg>,
}

impl PreparedChainstateReorg {
    pub(crate) const fn transition(&self) -> &ChainTransition {
        &self.transition
    }
    #[cfg(test)]
    pub(crate) fn maybe_basic_filter_preparation_work(
        &self,
    ) -> Option<open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork> {
        self.maybe_index
            .as_ref()
            .map(filter_reorg::PreparedIndexReorg::work)
    }
}

impl<S: ChainstateStore> ManagedChainstate<S, MemoryCoinsView> {
    pub fn from_store(store: S) -> Self {
        let chainstate = store
            .load_snapshot()
            .map(Chainstate::from_snapshot)
            .unwrap_or_default();

        Self {
            store,
            chainstate,
            maybe_validated_lineage: None,
            maybe_basic_index_owner: None,
            maybe_pending_validation: None,
            flush_lifecycle: FlushLifecycle::ready(
                FlushPolicyTime::from_unix_seconds(0),
                FlushPolicyTime::from_unix_seconds(0),
                0,
                false,
            ),
        }
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedChainstate<S, V> {
    pub fn from_chainstate(store: S, chainstate: Chainstate<V>, lifecycle: FlushLifecycle) -> Self {
        Self {
            store,
            chainstate,
            flush_lifecycle: lifecycle,
            maybe_validated_lineage: None,
            maybe_basic_index_owner: None,
            maybe_pending_validation: None,
        }
    }

    pub fn chainstate(&self) -> &Chainstate<V> {
        &self.chainstate
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    pub fn connect_block(
        &mut self,
        block: &Block,
        chain_work: u128,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, open_bitcoin_core::chainstate::ChainstateError> {
        self.connect_block_with_current_time(
            block,
            chain_work,
            i64::from(block.header.time),
            verify_flags,
            consensus_params,
        )
    }

    pub fn connect_block_with_current_time(
        &mut self,
        block: &Block,
        chain_work: u128,
        current_time: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, open_bitcoin_core::chainstate::ChainstateError> {
        let prepared = self.prepare_connect_block_with_current_time(
            block,
            chain_work,
            current_time,
            verify_flags,
            consensus_params,
        )?;
        self.commit_prepared_connect(prepared)
    }

    pub(crate) fn prepare_connect_block(
        &self,
        block: &Block,
        chain_work: u128,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<PreparedChainstateConnect, open_bitcoin_core::chainstate::ChainstateError> {
        self.prepare_connect_block_with_current_time(
            block,
            chain_work,
            i64::from(block.header.time),
            verify_flags,
            consensus_params,
        )
    }

    pub(crate) fn prepare_connect_block_with_current_time(
        &self,
        block: &Block,
        chain_work: u128,
        current_time: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<PreparedChainstateConnect, open_bitcoin_core::chainstate::ChainstateError> {
        let staged = self.chainstate.stage_connect_block_with_current_time(
            block,
            chain_work,
            current_time,
            verify_flags,
            consensus_params,
        )?;
        let position = staged.position().clone();
        let maybe_history =
            self.admit_validation_acceptance(std::slice::from_ref(staged.position()))?;
        let maybe_filter_facts = self.prepare_basic_filter_facts(block, &staged);
        Ok(PreparedChainstateConnect {
            staged,
            maybe_history,
            position,
            maybe_filter_facts,
        })
    }

    pub fn disconnect_tip(
        &mut self,
        block: &Block,
    ) -> Result<ChainPosition, open_bitcoin_core::chainstate::ChainstateError> {
        let position = self.chainstate.disconnect_tip(block)?;
        self.maybe_validated_lineage = None;
        self.invalidate_basic_index_owner();
        self.persist().map_err(map_persist)?;

        Ok(position)
    }

    pub fn reorg(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainTransition, open_bitcoin_core::chainstate::ChainstateError> {
        let prepared = self.prepare_reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        self.commit_prepared_reorg(prepared)
    }

    pub(crate) fn prepare_reorg(
        &self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<PreparedChainstateReorg, open_bitcoin_core::chainstate::ChainstateError> {
        let staged = self.chainstate.stage_reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        let transition = staged.transition().clone();
        let maybe_history = self.prepare_validation_reorg(&staged)?;
        let maybe_index = self.prepare_basic_index_reorg(&staged, replacement_branch)?;
        Ok(PreparedChainstateReorg {
            staged,
            maybe_history,
            transition,
            maybe_index,
        })
    }

    pub(crate) fn install_prepared_reorg_preview(
        &mut self,
        prepared: &PreparedChainstateReorg,
    ) -> Result<(), ChainstateError> {
        self.freeze_basic_index_reorg(prepared)
            .map_err(map_persist)?;
        self.chainstate
            .install_staged_reorg_preview(&prepared.staged);
        Ok(())
    }

    pub fn into_parts(self) -> (S, Chainstate<V>) {
        (self.store, self.chainstate)
    }

    fn observe_validated_lineage(&mut self, position: &ChainPosition) {
        self.maybe_validated_lineage = self
            .maybe_validated_lineage
            .take()
            .and_then(|mut lineage| lineage.observe(position).then_some(lineage));
    }

    pub(crate) fn flush_with_mode(
        &mut self,
        mode: FlushMode,
        now: FlushPolicyTime,
        disk_free_bytes: u64,
    ) -> Result<FlushExecution, StorageError>
    where
        S: FlushPersistSink,
    {
        self.flush_applying_plan(mode, now, disk_free_bytes, &PrunePlan::default(), &[])
            .map_err(|failure| failure.error)
    }

    pub(crate) fn flush_applying_plan(
        &mut self,
        mode: FlushMode,
        now: FlushPolicyTime,
        disk_free_bytes: u64,
        plan: &PrunePlan,
        locks: &[PruneLockInfo],
    ) -> Result<FlushExecution, FlushApplyError>
    where
        S: FlushPersistSink,
    {
        let maybe_pending = self
            .maybe_validated_lineage
            .as_ref()
            .map(|lineage| lineage.maybe_prepare(self.chainstate.tip()))
            .transpose()
            .map_err(|error| {
                self.note_basic_index_failure(filter_index::AcceptedBasicIndexFailure::Persistence);
                FlushApplyError {
                    error,
                    deleted_block_hashes: Vec::new(),
                }
            })?
            .flatten();
        let (undo_window, block_payloads, active_chain) = self.flush_window();
        let mut deleted = Vec::new();
        let mut result = self.flush_lifecycle.execute_flush_applying_plan(
            &mut self.store,
            self.chainstate.coins_mut(),
            mode,
            now,
            disk_free_bytes,
            &undo_window,
            &[],
            &block_payloads,
            &active_chain,
            plan,
            locks,
            &mut |hash| deleted.push(hash),
        );
        if let Some(pending) = maybe_pending {
            if result.as_ref().is_ok_and(|execution| execution.wrote_coins) {
                result = result.and_then(|execution| {
                    let completed = pending
                        .complete(self.maybe_validated_lineage.as_ref(), self.chainstate.tip())?;
                    self.store.confirm_validated_flush(completed)?;
                    self.note_basic_index_reorg_durable();
                    Ok(execution)
                });
            } else if let Err(cleanup) = pending.abort() {
                result = Err(match result {
                    Err(original) => StorageError::BackendFailure {
                        namespace: crate::storage::StorageNamespace::BlockIndex,
                        message: format!("{original}; BASIC receipt cleanup: {cleanup}"),
                        action: crate::storage::StorageRecoveryAction::Restart,
                    },
                    Ok(_) => cleanup,
                });
            }
        }
        for hash in &deleted {
            self.chainstate.forget_undo(*hash);
        }
        if result.is_err() {
            self.note_basic_index_failure(filter_index::AcceptedBasicIndexFailure::Persistence);
        }
        result.map_err(|error| FlushApplyError {
            error,
            deleted_block_hashes: deleted,
        })
    }

    #[cfg(test)]
    pub(crate) fn install_chainstate_for_test(&mut self, chainstate: Chainstate<V>) {
        self.maybe_validated_lineage = None;
        self.invalidate_basic_index_owner();
        self.chainstate = chainstate;
    }

    /// Drops undo for one hash so a prune-height walk sees that height as incomplete.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn forget_undo_for_test(&mut self, block_hash: BlockHash) {
        self.chainstate.forget_undo(block_hash);
    }

    #[cfg(test)]
    pub(crate) fn insert_overlay_coin_for_test(
        &mut self,
        outpoint: OutPoint,
        coin: Coin,
    ) -> Result<(), ChainstateError> {
        self.chainstate.coins_mut().add_coin(outpoint, coin, true)
    }

    fn persist(&mut self) -> Result<(), StorageError>
    where
        S: FlushPersistSink,
    {
        self.flush_with_mode(
            FlushMode::IfNeeded,
            FlushPolicyTime::from_unix_seconds(0),
            self.store.disk_free_bytes(),
        )
        .map(|_| ())
    }
}

fn map_persist(error: StorageError) -> ChainstateError {
    match error {
        StorageError::InterruptedWrite { .. } => {
            ChainstateError::InterruptedWrite { heads: Vec::new() }
        }
        other => ChainstateError::CoinsStorage {
            detail: other.to_string(),
        },
    }
}

impl<S, V: CoinsView> ManagedChainstate<S, V> {
    pub fn export_chainstate_snapshot(&self) -> Result<ChainstateSnapshot, ChainstateError> {
        self.chainstate.admission_snapshot()
    }
}

pub(crate) mod filter_index;
mod filter_reorg;
pub(crate) mod validation_history;
pub(crate) use filter_reorg::preflight::production_budget as basic_filter_turn_budget;
mod fjall_store;
pub use fjall_store::FjallChainstateStore;
#[cfg(test)]
pub(crate) use fjall_store::proof_tests::ReorgFixture;
pub(crate) use fjall_store::{ValidatedBasicFilterAppendPositions, ValidatedBasicFilterReorg};
mod flush_lifecycle;
pub use flush_lifecycle::{
    BasicFilterStartupMode, COINS_DB_CACHE_CAP_BYTES, CompletedValidatedFlush,
    DEFAULT_KERNEL_CACHE_BYTES, FlushExecution, FlushLifecycle, FlushPersistSink,
    MIN_DBCACHE_BYTES, ManagerReadiness, PruneProtectionSnapshot, default_coins_cache_byte_limit,
    initialize, initialize_configured, probe_disk_free_bytes,
};
mod replay;
pub use replay::replay_interrupted_flush;

#[cfg(test)]
mod tests;
