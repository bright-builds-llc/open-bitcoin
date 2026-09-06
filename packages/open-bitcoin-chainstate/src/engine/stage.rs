// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use open_bitcoin_primitives::BlockHash;

use super::{Chainstate, StagedChainstateConnect, StagedChainstateReorg};
use crate::coins::{CoinsOverlay, CoinsView};
use crate::{ChainPosition, ChainTransition};

impl StagedChainstateConnect {
    pub const fn position(&self) -> &ChainPosition {
        &self.position
    }
}

impl StagedChainstateReorg {
    pub const fn transition(&self) -> &ChainTransition {
        &self.transition
    }
}

impl<V: CoinsView> Chainstate<V> {
    /// Flush a staged connect overlay and install metadata.
    ///
    /// Infallible for the D-18 prepare/mempool/commit window: `FreshFlagMisapplied`
    /// after an unmodified live cache is a programming bug and is absorbed as a
    /// dirty overwrite without FRESH.
    pub fn absorb_staged_connect(&mut self, staged: StagedChainstateConnect) -> ChainPosition {
        let StagedChainstateConnect {
            overlay,
            undo,
            position,
            next_active_chain,
            next_confirmed_txid_counts,
        } = staged;
        self.absorb_overlay(overlay, Some(position.block_hash));
        self.undo_by_block.insert(position.block_hash, undo);
        self.active_chain = next_active_chain;
        self.maybe_confirmed_txid_counts = next_confirmed_txid_counts;
        position
    }

    /// Flush a staged reorg overlay and install metadata. See `absorb_staged_connect`.
    pub fn absorb_staged_reorg(&mut self, staged: StagedChainstateReorg) -> ChainTransition {
        let StagedChainstateReorg {
            overlay,
            transition,
            next_active_chain,
            next_undo_by_block,
            next_confirmed_txid_counts,
        } = staged;
        let maybe_best_block = next_active_chain.last().map(|position| position.block_hash);
        self.absorb_overlay(overlay, maybe_best_block);
        self.active_chain = next_active_chain;
        self.undo_by_block = next_undo_by_block;
        self.maybe_confirmed_txid_counts = next_confirmed_txid_counts;
        transition
    }

    /// Install a clone of the staged dirty overlay plus metadata without consuming
    /// the prepared overlay. Used by network reorg staging, not by `prepare_*`.
    pub fn install_staged_reorg_preview(&mut self, staged: &StagedChainstateReorg) {
        let maybe_best_block = staged
            .next_active_chain
            .last()
            .map(|position| position.block_hash);
        self.absorb_overlay(staged.overlay.clone(), maybe_best_block);
        self.active_chain = staged.next_active_chain.clone();
        self.undo_by_block = staged.next_undo_by_block.clone();
        self.maybe_confirmed_txid_counts = staged.next_confirmed_txid_counts.clone();
    }

    fn absorb_overlay(&mut self, overlay: CoinsOverlay, maybe_best_block: Option<BlockHash>) {
        self.coins
            .absorb_batch_write(overlay.into_dirty_batch(), maybe_best_block);
    }
}
