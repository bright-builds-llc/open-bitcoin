// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/undo.h

use std::collections::HashMap;
use std::fmt;

use open_bitcoin_consensus::context::{MinDifficultyRecoveryTarget, RetargetAnchor};
use open_bitcoin_consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid};
use open_bitcoin_primitives::{Block, BlockHash, OutPoint, Txid};

use crate::coins::{CoinsCache, CoinsOverlay, CoinsView, MemoryCoinsView};
use crate::{
    AnchoredBlock, BlockUndo, ChainPosition, ChainTransition, ChainstateError, ChainstateSnapshot,
    Coin,
};

mod apply;
mod overlay_apply;
mod stage;

use overlay_apply::{apply_connect_on_overlay, apply_disconnect_on_overlay};

const MEDIAN_TIME_PAST_WINDOW: usize = 11;

type StagedDisconnect = (
    CoinsOverlay,
    ChainPosition,
    Vec<ChainPosition>,
    HashMap<BlockHash, BlockUndo>,
    Option<HashMap<Txid, u32>>,
);
pub(super) type ConnectApplyOutcome = (ChainPosition, BlockUndo, Option<HashMap<Txid, u32>>);

pub struct Chainstate<V: CoinsView = MemoryCoinsView> {
    active_chain: Vec<ChainPosition>,
    coins: CoinsCache<V>,
    undo_by_block: HashMap<BlockHash, BlockUndo>,
    maybe_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
}

pub type MemoryBackedChainstate = Chainstate<MemoryCoinsView>;

#[derive(Debug)]
pub struct StagedChainstateConnect {
    pub overlay: CoinsOverlay,
    pub undo: BlockUndo,
    pub position: ChainPosition,
    pub next_active_chain: Vec<ChainPosition>,
    pub next_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
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
    overlay: CoinsOverlay,
    transition: ChainTransition,
    next_active_chain: Vec<ChainPosition>,
    next_undo_by_block: HashMap<BlockHash, BlockUndo>,
    next_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
    maybe_old_tip: Option<ChainPosition>,
    maybe_common_ancestor: Option<ChainPosition>,
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
    maybe_old_endpoint: Option<(u32, BlockHash)>,
    maybe_new_endpoint: Option<(u32, BlockHash)>,
    maybe_common_ancestor_endpoint: Option<(u32, BlockHash)>,
}

impl Default for Chainstate<MemoryCoinsView> {
    fn default() -> Self {
        Self {
            active_chain: Vec::new(),
            coins: CoinsCache::from_parent(MemoryCoinsView::from_coins(HashMap::new(), None)),
            undo_by_block: HashMap::new(),
            maybe_confirmed_txid_counts: Some(HashMap::new()),
        }
    }
}

impl fmt::Debug for Chainstate<MemoryCoinsView> {
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

impl PartialEq for Chainstate<MemoryCoinsView> {
    fn eq(&self, other: &Self) -> bool {
        self.snapshot() == other.snapshot()
    }
}

impl Eq for Chainstate<MemoryCoinsView> {}

impl Chainstate<MemoryCoinsView> {
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

    pub fn utxos(&self) -> HashMap<OutPoint, Coin> {
        self.coins.collect_unspent()
    }
}

impl<V: CoinsView> Chainstate<V> {
    pub fn from_parent(
        parent: V,
        active_chain: Vec<ChainPosition>,
        undo_by_block: HashMap<BlockHash, BlockUndo>,
        maybe_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
    ) -> Self {
        Self {
            active_chain,
            coins: CoinsCache::from_parent(parent),
            undo_by_block,
            maybe_confirmed_txid_counts,
        }
    }

    pub fn from_coins_cache(
        coins: CoinsCache<V>,
        active_chain: Vec<ChainPosition>,
        undo_by_block: HashMap<BlockHash, BlockUndo>,
        maybe_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
    ) -> Self {
        Self {
            active_chain,
            coins,
            undo_by_block,
            maybe_confirmed_txid_counts,
        }
    }

    pub fn active_chain(&self) -> &[ChainPosition] {
        &self.active_chain
    }

    pub fn undo_by_block(&self) -> &HashMap<BlockHash, BlockUndo> {
        &self.undo_by_block
    }

    /// Drops one undo so a later flush cannot write that key back.
    pub fn forget_undo(&mut self, block_hash: BlockHash) {
        self.undo_by_block.remove(&block_hash);
    }

    pub fn admission_snapshot(&self) -> Result<ChainstateSnapshot, ChainstateError> {
        let mut snapshot = ChainstateSnapshot::new(
            self.active_chain.clone(),
            self.coins.collect_admission_unspent()?,
            self.undo_by_block.clone(),
        );
        snapshot.maybe_confirmed_txid_counts = self.maybe_confirmed_txid_counts.clone();
        Ok(snapshot)
    }

    pub fn overlay_snapshot(&self) -> ChainstateSnapshot {
        let mut snapshot = ChainstateSnapshot::new(
            self.active_chain.clone(),
            self.coins.collect_overlay_unspent(),
            self.undo_by_block.clone(),
        );
        snapshot.maybe_confirmed_txid_counts = self.maybe_confirmed_txid_counts.clone();
        snapshot
    }

    pub fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        self.coins.get_coin(outpoint)
    }

    pub fn coins(&self) -> &CoinsCache<V> {
        &self.coins
    }

    pub fn coins_mut(&mut self) -> &mut CoinsCache<V> {
        &mut self.coins
    }

    pub fn tip(&self) -> Option<&ChainPosition> {
        self.active_chain.last()
    }

    pub fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        CoinsView::have_coin(&self.coins, outpoint)
    }

    pub fn have_coin_in_cache(&self, outpoint: &OutPoint) -> bool {
        self.coins.have_coin_in_cache(outpoint)
    }

    pub fn coins_best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
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

pub(super) fn next_counts_after_connect(
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

pub(super) fn next_counts_after_disconnect(
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

pub(super) fn maybe_retarget_anchor(
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

pub(super) fn maybe_min_difficulty_recovery_target(
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
