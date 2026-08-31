// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::collections::HashMap;
use std::fmt;

use open_bitcoin_consensus::context::{MinDifficultyRecoveryTarget, RetargetAnchor};
use open_bitcoin_consensus::{
    BlockValidationContext, ConsensusParams, ScriptVerifyFlags, block_hash, check_block_contextual,
    transaction_txid,
};
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader, OutPoint, Txid};

use crate::coins::{CoinsCache, CoinsOverlay, CoinsView, MemoryCoinsView};
use crate::{
    AnchoredBlock, BlockUndo, ChainPosition, ChainTransition, ChainstateError, ChainstateSnapshot,
    Coin,
};

mod apply;

const MEDIAN_TIME_PAST_WINDOW: usize = 11;

type StagedDisconnect = (
    CoinsOverlay,
    ChainPosition,
    Vec<ChainPosition>,
    HashMap<BlockHash, BlockUndo>,
    Option<HashMap<Txid, u32>>,
);
type ConnectApplyOutcome = (ChainPosition, BlockUndo, Option<HashMap<Txid, u32>>);

pub struct Chainstate {
    active_chain: Vec<ChainPosition>,
    coins: CoinsCache<MemoryCoinsView>,
    undo_by_block: HashMap<BlockHash, BlockUndo>,
    maybe_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
}

#[derive(Debug)]
pub struct StagedChainstateConnect {
    pub overlay: CoinsOverlay,
    pub undo: BlockUndo,
    pub position: ChainPosition,
    pub next_active_chain: Vec<ChainPosition>,
    pub next_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
}

#[derive(Debug)]
pub struct StagedChainstateReorg {
    pub overlay: CoinsOverlay,
    pub transition: ChainTransition,
    pub next_active_chain: Vec<ChainPosition>,
    pub next_undo_by_block: HashMap<BlockHash, BlockUndo>,
    pub next_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
}

impl Default for Chainstate {
    fn default() -> Self {
        Self {
            active_chain: Vec::new(),
            coins: CoinsCache::from_parent(MemoryCoinsView::from_coins(HashMap::new(), None)),
            undo_by_block: HashMap::new(),
            maybe_confirmed_txid_counts: Some(HashMap::new()),
        }
    }
}

impl fmt::Debug for Chainstate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Chainstate")
            .field("active_chain", &self.active_chain)
            .field("utxos", &self.utxos())
            .field("undo_by_block", &self.undo_by_block)
            .field(
                "maybe_confirmed_txid_counts",
                &self.maybe_confirmed_txid_counts,
            )
            .finish()
    }
}

impl PartialEq for Chainstate {
    fn eq(&self, other: &Self) -> bool {
        self.snapshot() == other.snapshot()
    }
}

impl Eq for Chainstate {}

impl Clone for Chainstate {
    fn clone(&self) -> Self {
        Self::from_snapshot(self.snapshot())
    }
}

impl Chainstate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_snapshot(snapshot: ChainstateSnapshot) -> Self {
        let maybe_best_block = snapshot.tip().map(|tip| tip.block_hash);
        Self {
            active_chain: snapshot.active_chain,
            coins: CoinsCache::from_parent(MemoryCoinsView::from_coins(
                snapshot.utxos,
                maybe_best_block,
            )),
            undo_by_block: snapshot.undo_by_block,
            maybe_confirmed_txid_counts: snapshot.maybe_confirmed_txid_counts,
        }
    }

    pub fn snapshot(&self) -> ChainstateSnapshot {
        let mut snapshot = ChainstateSnapshot::new(
            self.active_chain.clone(),
            self.coins.collect_unspent(),
            self.undo_by_block.clone(),
        );
        snapshot.maybe_confirmed_txid_counts = self.maybe_confirmed_txid_counts.clone();
        snapshot
    }

    pub fn tip(&self) -> Option<&ChainPosition> {
        self.active_chain.last()
    }

    pub fn utxos(&self) -> HashMap<OutPoint, Coin> {
        self.coins.collect_unspent()
    }

    pub fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        CoinsView::have_coin(&self.coins, outpoint)
    }

    pub fn have_coin_in_cache(&self, outpoint: &OutPoint) -> bool {
        self.coins.have_coin_in_cache(outpoint)
    }

    pub fn coins_best_block(&self) -> Option<BlockHash> {
        self.coins.best_block()
    }

    pub fn connect_block(
        &mut self,
        block: &Block,
        chain_work: u128,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, ChainstateError> {
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
    ) -> Result<ChainPosition, ChainstateError> {
        let staged = self.stage_connect_block_with_current_time(
            block,
            chain_work,
            current_time,
            verify_flags,
            consensus_params,
        )?;
        self.commit_staged_connect(staged)
    }

    pub fn stage_connect_block_with_current_time(
        &self,
        block: &Block,
        chain_work: u128,
        current_time: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<StagedChainstateConnect, ChainstateError> {
        let mut overlay = CoinsOverlay::new();
        let (position, undo, next_confirmed_txid_counts) = apply_connect_on_overlay(
            &mut overlay,
            &self.coins,
            &self.active_chain,
            &self.maybe_confirmed_txid_counts,
            block,
            chain_work,
            current_time,
            verify_flags,
            consensus_params,
        )?;
        overlay.set_best_block(position.block_hash);
        let mut next_active_chain = self.active_chain.clone();
        next_active_chain.push(position.clone());
        Ok(StagedChainstateConnect {
            overlay,
            undo,
            position,
            next_active_chain,
            next_confirmed_txid_counts,
        })
    }

    pub fn commit_staged_connect(
        &mut self,
        staged: StagedChainstateConnect,
    ) -> Result<ChainPosition, ChainstateError> {
        let position = staged.position;
        let undo = staged.undo;
        let next_active_chain = staged.next_active_chain;
        let next_confirmed_txid_counts = staged.next_confirmed_txid_counts;
        self.apply_flushed(staged.overlay, Some(position.block_hash), |this| {
            this.undo_by_block.insert(position.block_hash, undo);
            this.active_chain = next_active_chain;
            this.maybe_confirmed_txid_counts = next_confirmed_txid_counts;
            position
        })
    }

    pub fn disconnect_tip(&mut self, block: &Block) -> Result<ChainPosition, ChainstateError> {
        let (overlay, tip, next_active_chain, next_undo_by_block, next_confirmed_txid_counts) =
            self.stage_disconnect_tip(block)?;
        let maybe_best_block = next_active_chain.last().map(|position| position.block_hash);
        self.apply_flushed(overlay, maybe_best_block, |this| {
            this.active_chain = next_active_chain;
            this.undo_by_block = next_undo_by_block;
            this.maybe_confirmed_txid_counts = next_confirmed_txid_counts;
            tip
        })
    }

    pub fn reorg(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainTransition, ChainstateError> {
        let staged = self.stage_reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        self.commit_staged_reorg(staged)
    }

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
        })
    }

    pub fn commit_staged_reorg(
        &mut self,
        staged: StagedChainstateReorg,
    ) -> Result<ChainTransition, ChainstateError> {
        let maybe_best_block = staged
            .next_active_chain
            .last()
            .map(|position| position.block_hash);
        self.apply_flushed(staged.overlay, maybe_best_block, |this| {
            this.active_chain = staged.next_active_chain;
            this.undo_by_block = staged.next_undo_by_block;
            this.maybe_confirmed_txid_counts = staged.next_confirmed_txid_counts;
            staged.transition
        })
    }

    fn apply_flushed<T>(
        &mut self,
        overlay: CoinsOverlay,
        maybe_best_block: Option<BlockHash>,
        then: impl FnOnce(&mut Self) -> T,
    ) -> Result<T, ChainstateError> {
        self.coins
            .batch_write(overlay.into_dirty_batch(), maybe_best_block)?;
        Ok(then(self))
    }

    fn stage_disconnect_tip(&self, block: &Block) -> Result<StagedDisconnect, ChainstateError> {
        let mut overlay = CoinsOverlay::new();
        let mut next_active_chain = self.active_chain.clone();
        let mut next_undo_by_block = self.undo_by_block.clone();
        let mut next_confirmed_txid_counts = self.maybe_confirmed_txid_counts.clone();
        let tip = apply_disconnect_on_overlay(
            &mut overlay,
            &self.coins,
            &mut next_active_chain,
            &mut next_undo_by_block,
            &mut next_confirmed_txid_counts,
            block,
        )?;
        if let Some(next_tip) = next_active_chain.last() {
            overlay.set_best_block(next_tip.block_hash);
        }
        Ok((
            overlay,
            tip,
            next_active_chain,
            next_undo_by_block,
            next_confirmed_txid_counts,
        ))
    }
}

pub fn prefer_candidate_tip(current: &ChainPosition, candidate: &ChainPosition) -> bool {
    if candidate.chain_work != current.chain_work {
        return candidate.chain_work > current.chain_work;
    }
    if candidate.height != current.height {
        return candidate.height > current.height;
    }

    candidate.block_hash > current.block_hash
}

#[allow(clippy::too_many_arguments)]
fn apply_connect_on_overlay(
    overlay: &mut CoinsOverlay,
    parent: &CoinsCache<MemoryCoinsView>,
    active_chain: &[ChainPosition],
    maybe_confirmed_txid_counts: &Option<HashMap<Txid, u32>>,
    block: &Block,
    chain_work: u128,
    current_time: i64,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<ConnectApplyOutcome, ChainstateError> {
    let expected_previous = active_chain
        .last()
        .map_or(BlockHash::from_byte_array([0_u8; 32]), |tip| tip.block_hash);
    let actual_previous = block.header.previous_block_hash;
    if actual_previous != expected_previous {
        return Err(ChainstateError::InvalidTipExtension {
            expected_previous,
            actual_previous,
        });
    }

    let height = active_chain
        .last()
        .map_or(0, |tip| tip.height.saturating_add(1));
    let previous_header = active_chain
        .last()
        .map_or_else(BlockHeader::default, |tip| tip.header.clone());
    let maybe_retarget_anchor = maybe_retarget_anchor(active_chain, height, &consensus_params);
    let maybe_min_difficulty_recovery_target =
        maybe_min_difficulty_recovery_target(active_chain, height, &consensus_params);
    let previous_median_time_past = active_chain.last().map_or(0, |tip| tip.median_time_past);
    let block_context = BlockValidationContext {
        height,
        previous_header,
        maybe_retarget_anchor,
        maybe_min_difficulty_recovery_target,
        previous_median_time_past,
        current_time,
        consensus_params,
    };
    check_block_contextual(block, &block_context)
        .map_err(|source| ChainstateError::BlockValidation { source })?;

    let next_confirmed_txid_counts = next_counts_after_connect(maybe_confirmed_txid_counts, block)?;
    let (block_undo, _total_fees_sats) = apply::apply_connect_transactions(
        overlay,
        parent,
        block,
        height,
        previous_median_time_past,
        verify_flags,
        &block_context,
    )?;
    let median_time_past = compute_median_time_past(active_chain, Some(block.header.time));
    let position = ChainPosition::new(block.header.clone(), height, chain_work, median_time_past);
    Ok((position, block_undo, next_confirmed_txid_counts))
}

fn apply_disconnect_on_overlay(
    overlay: &mut CoinsOverlay,
    parent: &CoinsCache<MemoryCoinsView>,
    next_active_chain: &mut Vec<ChainPosition>,
    next_undo_by_block: &mut HashMap<BlockHash, BlockUndo>,
    next_confirmed_txid_counts: &mut Option<HashMap<Txid, u32>>,
    block: &Block,
) -> Result<ChainPosition, ChainstateError> {
    let Some(tip) = next_active_chain.last().cloned() else {
        return Err(ChainstateError::MissingTip);
    };
    let actual_block = block_hash(&block.header);
    if actual_block != tip.block_hash {
        return Err(ChainstateError::DisconnectBlockMismatch {
            expected_tip: tip.block_hash,
            actual_block,
        });
    }

    let Some(block_undo) = next_undo_by_block.get(&tip.block_hash).cloned() else {
        return Err(ChainstateError::MissingUndo {
            block_hash: tip.block_hash,
        });
    };
    if block.transactions.len().saturating_sub(1) != block_undo.transactions.len() {
        return Err(ChainstateError::UndoMismatch {
            expected_transactions: block.transactions.len().saturating_sub(1),
            actual_transactions: block_undo.transactions.len(),
        });
    }

    *next_confirmed_txid_counts = next_counts_after_disconnect(next_confirmed_txid_counts, block)?;
    apply::apply_disconnect_transactions(overlay, parent, block, tip.height, &block_undo)?;
    next_undo_by_block.remove(&tip.block_hash);
    next_active_chain.pop();
    Ok(tip)
}

fn next_counts_after_connect(
    maybe_confirmed_txid_counts: &Option<HashMap<Txid, u32>>,
    block: &Block,
) -> Result<Option<HashMap<Txid, u32>>, ChainstateError> {
    maybe_confirmed_txid_counts
        .as_ref()
        .map(|confirmed_txid_counts| {
            let mut next_counts = confirmed_txid_counts.clone();
            for transaction in &block.transactions {
                let txid = transaction_txid(transaction).map_err(txid_serialization_error)?;
                let count = next_counts.entry(txid).or_default();
                *count = count
                    .checked_add(1)
                    .ok_or_else(|| ChainstateError::Serialization {
                        context: "confirmed transaction count",
                        reason: format!("active-chain occurrence count overflow for {txid:?}"),
                    })?;
            }
            Ok(next_counts)
        })
        .transpose()
}

fn next_counts_after_disconnect(
    maybe_confirmed_txid_counts: &Option<HashMap<Txid, u32>>,
    block: &Block,
) -> Result<Option<HashMap<Txid, u32>>, ChainstateError> {
    maybe_confirmed_txid_counts
        .as_ref()
        .map(|confirmed_txid_counts| {
            let mut next_counts = confirmed_txid_counts.clone();
            for transaction in &block.transactions {
                let txid = transaction_txid(transaction).map_err(txid_serialization_error)?;
                let Some(count) = next_counts.get_mut(&txid) else {
                    return Err(ChainstateError::Serialization {
                        context: "confirmed transaction count",
                        reason: format!("missing active-chain occurrence for {txid:?}"),
                    });
                };
                if *count > 1 {
                    *count -= 1;
                } else {
                    next_counts.remove(&txid);
                }
            }
            Ok(next_counts)
        })
        .transpose()
}

fn difficulty_adjustment_interval(consensus_params: &ConsensusParams) -> u32 {
    if consensus_params.pow_target_spacing_seconds <= 0 {
        return 1;
    }

    let interval =
        consensus_params.pow_target_timespan_seconds / consensus_params.pow_target_spacing_seconds;
    interval.max(1) as u32
}

fn maybe_retarget_anchor(
    active_chain: &[ChainPosition],
    height: u32,
    consensus_params: &ConsensusParams,
) -> Option<RetargetAnchor> {
    if height == 0 || consensus_params.no_pow_retargeting {
        return None;
    }

    let interval = difficulty_adjustment_interval(consensus_params);
    if !height.is_multiple_of(interval) {
        return None;
    }

    let anchor_height = height.checked_sub(interval)?;
    let anchor_position = active_chain.get(anchor_height as usize)?;

    Some(RetargetAnchor {
        first_block_time: i64::from(anchor_position.header.time),
    })
}

fn maybe_min_difficulty_recovery_target(
    active_chain: &[ChainPosition],
    height: u32,
    consensus_params: &ConsensusParams,
) -> Option<MinDifficultyRecoveryTarget> {
    if height == 0 || !consensus_params.allow_min_difficulty_blocks {
        return None;
    }

    let interval = difficulty_adjustment_interval(consensus_params);
    if height.is_multiple_of(interval) {
        return None;
    }

    let mut index = active_chain.len().checked_sub(1)?;
    loop {
        let position = active_chain.get(index)?;
        let should_keep_walking = index > 0
            && !position.height.is_multiple_of(interval)
            && position.header.bits == consensus_params.pow_limit_bits;
        if !should_keep_walking {
            return Some(MinDifficultyRecoveryTarget {
                bits: position.header.bits,
            });
        }

        index = index.saturating_sub(1);
    }
}

#[cfg(test)]
pub(crate) use apply::accumulated_fee_out_of_range;
pub(crate) use apply::txid_serialization_error;

pub(super) fn compute_median_time_past(
    active_chain: &[ChainPosition],
    maybe_new_time: Option<u32>,
) -> i64 {
    let mut times: Vec<u32> = active_chain
        .iter()
        .rev()
        .take(MEDIAN_TIME_PAST_WINDOW)
        .map(|position| position.header.time)
        .collect();
    if let Some(new_time) = maybe_new_time {
        times.push(new_time);
    }
    if times.is_empty() {
        return 0;
    }

    times.sort_unstable();
    i64::from(times[times.len() / 2])
}

#[cfg(test)]
mod tests;
