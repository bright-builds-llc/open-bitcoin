// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/undo.h

use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_primitives::{Block, BlockHash};

use super::Chainstate;
use super::overlay_apply::{apply_connect_on_overlay, apply_disconnect_on_overlay};
use super::work_authority::StageBinding;
use crate::coins::{CoinsOverlay, CoinsView};
use crate::{
    AnchoredBlock, BasicFilterInputError, BasicFilterInputs, BlockUndo, ChainPosition,
    ChainTransition, ChainstateError, HistoricalBlockUndo,
};
use open_bitcoin_primitives::Txid;
use std::collections::HashMap;

/// A connect stage can only be produced by its chainstate owner.
/// ```compile_fail
/// use open_bitcoin_chainstate::StagedChainstateConnect;
/// fn mutate(stage: &mut StagedChainstateConnect) { stage.position.chain_work = Default::default(); }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::{StagedChainstateConnect, coins::CoinsOverlay};
/// fn mutate(stage: &mut StagedChainstateConnect) { stage.overlay = CoinsOverlay::new(); }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::StagedChainstateConnect;
/// fn forge(existing: StagedChainstateConnect) -> StagedChainstateConnect {
///     StagedChainstateConnect { ..existing }
/// }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::StagedChainstateConnect;
/// fn mutate(stage: &mut StagedChainstateConnect) { stage.undo.transactions.clear(); }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::StagedChainstateConnect;
/// fn mutate(stage: &mut StagedChainstateConnect) { stage.next_active_chain.clear(); }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::StagedChainstateConnect;
/// fn mutate(stage: &mut StagedChainstateConnect) { stage.next_confirmed_txid_counts = None; }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::StagedChainstateConnect;
/// fn replay(stage: StagedChainstateConnect) { let _ = stage.clone(); }
/// ```
#[derive(Debug)]
pub struct StagedChainstateConnect {
    pub(super) binding: StageBinding,
    pub(super) overlay: CoinsOverlay,
    pub(super) undo: BlockUndo,
    pub(super) position: ChainPosition,
    pub(super) next_active_chain: Vec<ChainPosition>,
    pub(super) next_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
}

/// Genuine validated stage with read-only borrowed facts.
///
/// Raw construction is forbidden:
/// ```compile_fail
/// use open_bitcoin_chainstate::{StagedChainstateReorg, ChainTransition};
/// use open_bitcoin_chainstate::coins::CoinsOverlay;
/// use std::collections::HashMap;
/// let _ = StagedChainstateReorg {
///     overlay: CoinsOverlay::new(), transition: ChainTransition::default(),
///     next_active_chain: Vec::new(), next_undo_by_block: HashMap::new(),
///     next_confirmed_txid_counts: None,
///     maybe_old_tip: None, maybe_common_ancestor: None,
/// };
/// ```
/// Every staged field is immutable to external consumers:
/// ```compile_fail
/// use open_bitcoin_chainstate::{StagedChainstateReorg, coins::CoinsOverlay};
/// fn mutate(stage: &mut StagedChainstateReorg) { stage.overlay = CoinsOverlay::new(); }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::{StagedChainstateReorg, ChainTransition};
/// fn mutate(stage: &mut StagedChainstateReorg) { stage.transition = ChainTransition::default(); }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::StagedChainstateReorg;
/// fn mutate(stage: &mut StagedChainstateReorg) { stage.next_active_chain = Vec::new(); }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::StagedChainstateReorg;
/// fn mutate(stage: &mut StagedChainstateReorg) { stage.next_undo_by_block = std::collections::HashMap::new(); }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::StagedChainstateReorg;
/// fn mutate(stage: &mut StagedChainstateReorg) { stage.next_confirmed_txid_counts = None; }
/// ```
/// Borrowed access preserves exact identity without exposing mutable parts:
/// ```
/// use open_bitcoin_chainstate::{StagedChainstateReorg, ChainPosition, BlockUndo, ChainTransition};
/// use open_bitcoin_primitives::BlockHash;
/// fn inspect(stage: &StagedChainstateReorg, hash: BlockHash) {
///     let _: &ChainTransition = stage.transition();
///     let _: Option<&ChainPosition> = stage.maybe_position_at_height(0);
///     let _: Option<&BlockUndo> = stage.maybe_replacement_undo(hash);
///     let _: Option<&ChainPosition> = stage.maybe_old_tip();
///     let _: Option<&ChainPosition> = stage.maybe_common_ancestor();
/// }
/// ```
/// Mutable borrowed access is forbidden:
/// ```compile_fail
/// use open_bitcoin_chainstate::{StagedChainstateReorg, ChainTransition};
/// fn mutate(stage: &mut StagedChainstateReorg) {
///     let _: &mut ChainTransition = stage.transition();
/// }
/// ```
#[derive(Debug)]
pub struct StagedChainstateReorg {
    pub(super) binding: StageBinding,
    pub(super) overlay: CoinsOverlay,
    pub(super) transition: ChainTransition,
    pub(super) next_active_chain: Vec<ChainPosition>,
    pub(super) next_undo_by_block: HashMap<BlockHash, BlockUndo>,
    pub(super) next_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
    pub(super) maybe_old_tip: Option<ChainPosition>,
    pub(super) maybe_common_ancestor: Option<ChainPosition>,
}

/// Consume-only evidence of genuine absorption, distinct from a durable checkpoint.
/// ```compile_fail
/// use open_bitcoin_chainstate::AcceptedChainstateReorg;
/// let _ = AcceptedChainstateReorg {
///     maybe_old_endpoint: None, maybe_new_endpoint: None,
///     maybe_common_ancestor_endpoint: None,
/// };
/// ```
/// Accepted receipts cannot be cloned:
/// ```compile_fail
/// use open_bitcoin_chainstate::AcceptedChainstateReorg;
/// fn duplicate(receipt: AcceptedChainstateReorg) -> AcceptedChainstateReorg { receipt.clone() }
/// ```
/// Preview has no accepted receipt:
/// ```compile_fail
/// use open_bitcoin_chainstate::{Chainstate, StagedChainstateReorg, AcceptedChainstateReorg};
/// fn preview(chain: &mut Chainstate, stage: &StagedChainstateReorg) {
///     let _: AcceptedChainstateReorg = chain.install_staged_reorg_preview(stage);
/// }
/// ```
/// Read-only receipt inspection remains available to downstream adapters:
/// ```
/// use open_bitcoin_chainstate::AcceptedChainstateReorg;
/// use open_bitcoin_primitives::BlockHash;
/// fn inspect(receipt: &AcceptedChainstateReorg) {
///     let _: Option<(u32, BlockHash)> = receipt.maybe_old_endpoint();
///     let _: Option<(u32, BlockHash)> = receipt.maybe_new_endpoint();
///     let _: Option<(u32, BlockHash)> = receipt.maybe_common_ancestor_endpoint();
/// }
/// ```
#[derive(Debug)]
pub struct AcceptedChainstateReorg {
    pub(super) maybe_old_endpoint: Option<(u32, BlockHash)>,
    pub(super) maybe_new_endpoint: Option<(u32, BlockHash)>,
    pub(super) maybe_common_ancestor_endpoint: Option<(u32, BlockHash)>,
}

impl StagedChainstateConnect {
    /// Live body validation, distinct from an explicitly mechanical fixture stage.
    pub fn has_earned_work(&self) -> bool {
        self.binding.has_earned_work()
    }
    /// Borrow the undo generated by this stage without permitting mutation.
    pub const fn undo(&self) -> &BlockUndo {
        &self.undo
    }

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
    /// Whether this stage descends from the live owner rather than a raw fixture import.
    pub fn has_earned_work(&self) -> bool {
        self.binding.has_earned_work()
    }
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
    /// Bind preference to its exact stage, including the stage's own preview continuation.
    pub fn check_reorg_work_source(
        &self,
        source: &crate::LiveWorkSourceBinding,
        staged: &StagedChainstateReorg,
    ) -> Result<(), ChainstateError> {
        if !staged.binding.matches_source(source) || !staged.has_earned_work() {
            return Err(ChainstateError::StaleStage);
        }
        self.check_staged_reorg(staged)
    }
    /// Validate a complete replacement and capture original endpoints before preview.
    pub fn stage_reorg(
        &self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<StagedChainstateReorg, ChainstateError> {
        let derive_work = self
            .work_authority
            .check_parent(&self.active_chain, consensus_params)?;
        let binding = self.work_authority.bind(self.tip(), consensus_params);
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
                derive_work,
                i64::from(anchored_block.block.header.time),
                verify_flags,
                consensus_params,
            )
            .map_err(|source| ChainstateError::ReplacementBlock {
                block_hash: open_bitcoin_consensus::block_hash(&anchored_block.block.header),
                source: Box::new(source),
            })?;
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
            binding,
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
    pub fn validate_staged_connect(
        &mut self,
        staged: StagedChainstateConnect,
    ) -> Result<ValidatedChainstateConnect<'_, V>, ChainstateError> {
        self.work_authority
            .check(&staged.binding, self.tip(), false)?;
        Ok(ValidatedChainstateConnect {
            owner: self,
            staged,
        })
    }

    /// Refuse foreign or stale input, then absorb through an exclusive owner borrow.
    pub fn absorb_staged_connect(
        &mut self,
        staged: StagedChainstateConnect,
    ) -> Result<ChainPosition, ChainstateError> {
        Ok(self.validate_staged_connect(staged)?.absorb())
    }

    fn absorb_validated_connect(&mut self, staged: StagedChainstateConnect) -> ChainPosition {
        let StagedChainstateConnect {
            binding,
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
        self.work_authority.accept(&binding);
        position
    }

    /// Flush a staged reorg overlay and install metadata. See `absorb_staged_connect`.
    pub fn absorb_staged_reorg(
        &mut self,
        staged: StagedChainstateReorg,
    ) -> Result<ChainTransition, ChainstateError> {
        Ok(self.absorb_staged_reorg_with_receipt(staged)?.0)
    }

    /// Absorb genuine staged effects and mint accepted endpoint facts after installation.
    pub fn absorb_staged_reorg_with_receipt(
        &mut self,
        staged: StagedChainstateReorg,
    ) -> Result<(ChainTransition, AcceptedChainstateReorg), ChainstateError> {
        Ok(self.validate_staged_reorg(staged)?.absorb())
    }

    /// Validate origin and generation before granting exclusive, infallible absorption.
    pub fn validate_staged_reorg(
        &mut self,
        staged: StagedChainstateReorg,
    ) -> Result<ValidatedChainstateReorg<'_, V>, ChainstateError> {
        self.check_staged_reorg(&staged)?;
        Ok(ValidatedChainstateReorg {
            owner: self,
            staged,
        })
    }

    /// Validate a prepared reorg before any downstream preview or publication effect.
    pub fn check_staged_reorg(
        &self,
        staged: &StagedChainstateReorg,
    ) -> Result<(), ChainstateError> {
        self.work_authority.check(&staged.binding, self.tip(), true)
    }

    /// Check the original owner generation before freezing any preview consumers.
    pub fn check_staged_reorg_preview(
        &self,
        staged: &StagedChainstateReorg,
    ) -> Result<(), ChainstateError> {
        self.work_authority
            .check(&staged.binding, self.tip(), false)
    }

    fn absorb_validated_reorg(
        &mut self,
        staged: StagedChainstateReorg,
    ) -> (ChainTransition, AcceptedChainstateReorg) {
        let StagedChainstateReorg {
            binding,
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
        self.work_authority.accept(&binding);
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
    pub fn install_staged_reorg_preview(
        &mut self,
        staged: &StagedChainstateReorg,
    ) -> Result<(), ChainstateError> {
        self.check_staged_reorg_preview(staged)?;
        let maybe_best_block = staged
            .next_active_chain
            .last()
            .map(|position| position.block_hash);
        self.absorb_overlay(staged.overlay.clone(), maybe_best_block);
        self.active_chain = staged.next_active_chain.clone();
        self.undo_by_block = staged.next_undo_by_block.clone();
        self.maybe_confirmed_txid_counts = staged.next_confirmed_txid_counts.clone();
        self.work_authority.preview(&staged.binding);
        Ok(())
    }

    fn absorb_overlay(&mut self, overlay: CoinsOverlay, maybe_best_block: Option<BlockHash>) {
        self.coins
            .absorb_batch_write(overlay.into_dirty_batch(), maybe_best_block);
    }
}

/// A stage checked against its exclusively borrowed owner. Consuming it is infallible.
/// No mutation can intervene after validation:
/// ```compile_fail
/// use open_bitcoin_chainstate::{Chainstate, StagedChainstateConnect};
/// fn race(chain: &mut Chainstate, stage: StagedChainstateConnect) {
///     let accepted = chain.validate_staged_connect(stage).expect("valid owner");
///     chain.coins_mut();
///     accepted.absorb();
/// }
/// ```
/// ```compile_fail
/// use open_bitcoin_chainstate::{Chainstate, StagedChainstateConnect};
/// use open_bitcoin_chainstate::engine::ValidatedChainstateConnect;
/// fn forge(owner: &mut Chainstate, staged: StagedChainstateConnect) {
///     let _ = ValidatedChainstateConnect { owner, staged };
/// }
/// ```
pub struct ValidatedChainstateConnect<'a, V: CoinsView> {
    owner: &'a mut Chainstate<V>,
    staged: StagedChainstateConnect,
}

impl<V: CoinsView> ValidatedChainstateConnect<'_, V> {
    pub fn absorb(self) -> ChainPosition {
        self.owner.absorb_validated_connect(self.staged)
    }
}

/// An exclusively borrowed reorg owner, including any authorized single preview.
pub struct ValidatedChainstateReorg<'a, V: CoinsView> {
    owner: &'a mut Chainstate<V>,
    staged: StagedChainstateReorg,
}

impl<V: CoinsView> ValidatedChainstateReorg<'_, V> {
    pub fn absorb(self) -> (ChainTransition, AcceptedChainstateReorg) {
        self.owner.absorb_validated_reorg(self.staged)
    }
}
