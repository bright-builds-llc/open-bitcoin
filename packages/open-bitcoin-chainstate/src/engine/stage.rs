// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/undo.h

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_primitives::{Block, BlockHash};

use super::overlay_apply::{apply_connect_on_overlay, apply_disconnect_on_overlay};
use super::{AcceptedChainstateReorg, Chainstate, StagedChainstateConnect, StagedChainstateReorg};
use crate::coins::{CoinsOverlay, CoinsView};
use crate::{
    AnchoredBlock, BasicFilterInputError, BasicFilterInputs, BlockUndo, ChainPosition,
    ChainTransition, ChainstateError, HistoricalBlockUndo,
};

impl StagedChainstateConnect {
    /// Borrow complete filter facts from the undo produced by this validation stage.
    pub fn basic_filter_inputs<'a>(
        &'a self,
        block: &'a Block,
    ) -> Result<BasicFilterInputs<'a>, BasicFilterInputError> {
        BasicFilterInputs::from_historical(
            block,
            &self.position,
            Some(HistoricalBlockUndo {
                block_hash: self.position.block_hash,
                undo: &self.undo,
            }),
        )
    }

    pub const fn position(&self) -> &ChainPosition {
        &self.position
    }
}

impl StagedChainstateReorg {
    /// Borrow the staged validated chain position, including the retained common prefix.
    pub fn maybe_position_at_height(&self, height: u32) -> Option<&ChainPosition> {
        let position = self.next_active_chain.get(usize::try_from(height).ok()?)?;
        (position.height == height).then_some(position)
    }

    /// Borrow only undo generated for a connected replacement identity in this stage.
    pub fn maybe_replacement_undo(&self, block_hash: BlockHash) -> Option<&BlockUndo> {
        if !self
            .transition
            .connected
            .iter()
            .any(|position| position.block_hash == block_hash)
        {
            return None;
        }
        self.next_undo_by_block.get(&block_hash)
    }

    /// Return the original endpoint captured before any preview changes live state.
    pub fn maybe_old_tip(&self) -> Option<&ChainPosition> {
        self.maybe_old_tip.as_ref()
    }

    /// Return the validated retained endpoint captured before replacement connects.
    pub fn maybe_common_ancestor(&self) -> Option<&ChainPosition> {
        self.maybe_common_ancestor.as_ref()
    }

    /// Borrow the genuine validated disconnect/connect identities.
    pub const fn transition(&self) -> &ChainTransition {
        &self.transition
    }
}

impl AcceptedChainstateReorg {
    /// Return the captured displaced endpoint.
    pub const fn maybe_old_endpoint(&self) -> Option<(u32, BlockHash)> {
        self.maybe_old_endpoint
    }
    /// Return the endpoint genuinely absorbed into live chainstate, without durability claims.
    pub const fn maybe_new_endpoint(&self) -> Option<(u32, BlockHash)> {
        self.maybe_new_endpoint
    }
    /// Return the validated retained ancestor, or absence when the entire chain was disconnected.
    pub const fn maybe_common_ancestor_endpoint(&self) -> Option<(u32, BlockHash)> {
        self.maybe_common_ancestor_endpoint
    }
}

impl<V: CoinsView> Chainstate<V> {
    /// Validate a complete replacement and capture original endpoints before preview.
    pub fn stage_reorg(
        &self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<StagedChainstateReorg, ChainstateError> {
        if disconnect_blocks.len() > self.active_chain.len() {
            return Err(ChainstateError::DisconnectPastGenesis {
                requested: disconnect_blocks.len(),
                available: self.active_chain.len(),
            });
        }

        let mut overlay = CoinsOverlay::new();
        let mut next_active_chain = self.active_chain.clone();
        let mut next_undo_by_block = self.undo_by_block.clone();
        let mut next_confirmed_txid_counts = self.maybe_confirmed_txid_counts.clone();
        let mut transition = ChainTransition::default();
        let maybe_old_tip = self.tip().cloned();
        for block in disconnect_blocks {
            let tip = apply_disconnect_on_overlay(
                &mut overlay,
                &self.coins,
                &mut next_active_chain,
                &mut next_undo_by_block,
                &mut next_confirmed_txid_counts,
                block,
            )?;
            transition.disconnected.push(tip);
        }
        let maybe_common_ancestor = next_active_chain.last().cloned();
        for anchored_block in replacement_branch {
            let (position, undo, maybe_counts) = apply_connect_on_overlay(
                &mut overlay,
                &self.coins,
                &next_active_chain,
                &next_confirmed_txid_counts,
                &anchored_block.block,
                anchored_block.chain_work,
                i64::from(anchored_block.block.header.time),
                verify_flags,
                consensus_params,
            )?;
            overlay.set_best_block(position.block_hash);
            next_undo_by_block.insert(position.block_hash, undo);
            next_active_chain.push(position.clone());
            next_confirmed_txid_counts = maybe_counts;
            transition.connected.push(position);
        }
        if let Some(tip) = next_active_chain.last() {
            overlay.set_best_block(tip.block_hash);
        }
        Ok(StagedChainstateReorg {
            overlay,
            transition,
            next_active_chain,
            next_undo_by_block,
            next_confirmed_txid_counts,
            maybe_old_tip,
            maybe_common_ancestor,
        })
    }

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
        self.absorb_staged_reorg_with_receipt(staged).0
    }

    /// Absorb genuine staged effects and mint accepted endpoint facts after installation.
    pub fn absorb_staged_reorg_with_receipt(
        &mut self,
        staged: StagedChainstateReorg,
    ) -> (ChainTransition, AcceptedChainstateReorg) {
        let StagedChainstateReorg {
            overlay,
            transition,
            next_active_chain,
            next_undo_by_block,
            next_confirmed_txid_counts,
            maybe_old_tip,
            maybe_common_ancestor,
        } = staged;
        let maybe_new_endpoint = next_active_chain
            .last()
            .map(|position| (position.height, position.block_hash));
        let maybe_best_block = next_active_chain.last().map(|position| position.block_hash);
        self.absorb_overlay(overlay, maybe_best_block);
        self.active_chain = next_active_chain;
        self.undo_by_block = next_undo_by_block;
        self.maybe_confirmed_txid_counts = next_confirmed_txid_counts;
        let receipt = AcceptedChainstateReorg {
            maybe_old_endpoint: maybe_old_tip
                .map(|position| (position.height, position.block_hash)),
            maybe_new_endpoint,
            maybe_common_ancestor_endpoint: maybe_common_ancestor
                .map(|position| (position.height, position.block_hash)),
        };
        (transition, receipt)
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
