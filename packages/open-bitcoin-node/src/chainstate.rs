// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::HashMap;

use open_bitcoin_core::{
    chainstate::{
        AnchoredBlock, BlockUndo, ChainPosition, ChainTransition, Chainstate, ChainstateError,
        ChainstateSnapshot, Coin, CoinsBatch, CoinsView, MemoryCoinsView, StagedChainstateConnect,
        StagedChainstateReorg,
    },
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::{Block, BlockHash, OutPoint},
};

pub trait ChainstateStore {
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

#[derive(Debug, PartialEq, Eq)]
pub struct ManagedChainstate<S> {
    store: S,
    chainstate: Chainstate,
}

impl<S: Clone> Clone for ManagedChainstate<S> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            chainstate: Chainstate::from_snapshot(self.chainstate.snapshot()),
        }
    }
}

/// Opaque, fully validated replacement for one connected block.
pub(crate) struct PreparedChainstateConnect {
    staged: StagedChainstateConnect,
    position: ChainPosition,
}

impl PreparedChainstateConnect {
    pub(crate) const fn position(&self) -> &ChainPosition {
        &self.position
    }
}

/// Opaque, fully validated replacement for one complete reorg.
pub(crate) struct PreparedChainstateReorg {
    staged: StagedChainstateReorg,
    transition: ChainTransition,
}

impl PreparedChainstateReorg {
    pub(crate) const fn transition(&self) -> &ChainTransition {
        &self.transition
    }
}

impl<S: ChainstateStore> ManagedChainstate<S> {
    pub fn from_store(store: S) -> Self {
        let chainstate = store
            .load_snapshot()
            .map(Chainstate::from_snapshot)
            .unwrap_or_default();

        Self { store, chainstate }
    }

    pub fn chainstate(&self) -> &Chainstate {
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
        let position = self.chainstate.connect_block_with_current_time(
            block,
            chain_work,
            current_time,
            verify_flags,
            consensus_params,
        )?;
        self.persist();
        Ok(position)
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
        Ok(PreparedChainstateConnect { staged, position })
    }

    pub(crate) fn commit_prepared_connect(
        &mut self,
        prepared: PreparedChainstateConnect,
    ) -> ChainPosition {
        // D-18: commit_prepared_mempool_transition_with applies the mempool
        // patch after this closure returns, including when R is Err. This
        // return stays infallible. FreshFlagMisapplied after an unmodified
        // live cache is a programming bug; absorb clears FRESH and overwrites dirty.
        let position = self.chainstate.absorb_staged_connect(prepared.staged);
        self.persist();
        position
    }

    pub fn disconnect_tip(
        &mut self,
        block: &Block,
    ) -> Result<ChainPosition, open_bitcoin_core::chainstate::ChainstateError> {
        let position = self.chainstate.disconnect_tip(block)?;
        self.persist();

        Ok(position)
    }

    pub fn reorg(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainTransition, open_bitcoin_core::chainstate::ChainstateError> {
        let transition = self.chainstate.reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        self.persist();
        Ok(transition)
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
        Ok(PreparedChainstateReorg { staged, transition })
    }

    pub(crate) fn install_prepared_reorg_preview(&mut self, prepared: &PreparedChainstateReorg) {
        self.chainstate
            .install_staged_reorg_preview(&prepared.staged);
    }

    pub(crate) fn commit_prepared_reorg(
        &mut self,
        prepared: PreparedChainstateReorg,
    ) -> ChainTransition {
        let transition = self.chainstate.absorb_staged_reorg(prepared.staged);
        self.persist();
        transition
    }

    pub fn into_parts(self) -> (S, Chainstate) {
        (self.store, self.chainstate)
    }

    fn persist(&mut self) {
        self.store.save_snapshot(self.chainstate.snapshot());
    }
}

mod replay;
pub use replay::replay_interrupted_flush;

#[cfg(test)]
mod tests;
