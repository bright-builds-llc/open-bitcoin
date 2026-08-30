// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::collections::HashMap;

use open_bitcoin_consensus::block::enforce_coinbase_reward_limit;
use open_bitcoin_consensus::context::{MinDifficultyRecoveryTarget, RetargetAnchor};
use open_bitcoin_consensus::{
    BlockValidationContext, ConsensusParams, ScriptVerifyFlags, block_hash, check_block_contextual,
    transaction_txid,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, MAX_MONEY, OutPoint, Transaction,
};

use crate::coins::{CoinsCache, CoinsOverlay, MemoryCoinsView};
use crate::{
    AnchoredBlock, BlockUndo, ChainPosition, ChainTransition, ChainstateError, ChainstateSnapshot,
    Coin, TxUndo,
};

mod apply;

const MEDIAN_TIME_PAST_WINDOW: usize = 11;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chainstate {
    active_chain: Vec<ChainPosition>,
    utxos: HashMap<OutPoint, Coin>,
    undo_by_block: HashMap<BlockHash, BlockUndo>,
    maybe_confirmed_txid_counts: Option<HashMap<open_bitcoin_primitives::Txid, u32>>,
}

impl Default for Chainstate {
    fn default() -> Self {
        Self {
            active_chain: Vec::new(),
            utxos: HashMap::new(),
            undo_by_block: HashMap::new(),
            maybe_confirmed_txid_counts: Some(HashMap::new()),
        }
    }
}

impl Chainstate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_snapshot(snapshot: ChainstateSnapshot) -> Self {
        Self {
            active_chain: snapshot.active_chain,
            utxos: snapshot.utxos,
            undo_by_block: snapshot.undo_by_block,
            maybe_confirmed_txid_counts: snapshot.maybe_confirmed_txid_counts,
        }
    }

    pub fn snapshot(&self) -> ChainstateSnapshot {
        let mut snapshot = ChainstateSnapshot::new(
            self.active_chain.clone(),
            self.utxos.clone(),
            self.undo_by_block.clone(),
        );
        snapshot.maybe_confirmed_txid_counts = self.maybe_confirmed_txid_counts.clone();
        snapshot
    }

    pub fn tip(&self) -> Option<&ChainPosition> {
        self.active_chain.last()
    }

    pub fn utxos(&self) -> &HashMap<OutPoint, Coin> {
        &self.utxos
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
        let expected_previous = self
            .tip()
            .map_or(BlockHash::from_byte_array([0_u8; 32]), |tip| tip.block_hash);
        let actual_previous = block.header.previous_block_hash;
        if actual_previous != expected_previous {
            return Err(ChainstateError::InvalidTipExtension {
                expected_previous,
                actual_previous,
            });
        }

        let height = self.tip().map_or(0, |tip| tip.height.saturating_add(1));
        let previous_header = self
            .tip()
            .map_or_else(BlockHeader::default, |tip| tip.header.clone());
        let maybe_retarget_anchor =
            maybe_retarget_anchor(&self.active_chain, height, &consensus_params);
        let maybe_min_difficulty_recovery_target =
            maybe_min_difficulty_recovery_target(&self.active_chain, height, &consensus_params);
        let previous_median_time_past = self.tip().map_or(0, |tip| tip.median_time_past);
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

        let maybe_next_confirmed_txid_counts = self
            .maybe_confirmed_txid_counts
            .as_ref()
            .map(|confirmed_txid_counts| {
                let mut next_counts = confirmed_txid_counts.clone();
                for transaction in &block.transactions {
                    let txid = transaction_txid(transaction).map_err(txid_serialization_error)?;
                    let count = next_counts.entry(txid).or_default();
                    *count =
                        count
                            .checked_add(1)
                            .ok_or_else(|| ChainstateError::Serialization {
                                context: "confirmed transaction count",
                                reason: format!(
                                    "active-chain occurrence count overflow for {txid:?}"
                                ),
                            })?;
                }
                Ok(next_counts)
            })
            .transpose()?;

        let mut next_utxos = self.utxos.clone();
        let mut block_undo = BlockUndo::default();
        let block_time = i64::from(block.header.time);
        let mut total_fees_sats = 0_i64;
        for (transaction_index, transaction) in block.transactions.iter().enumerate() {
            if transaction_index > 0 {
                let fee = apply_non_coinbase_transaction(
                    &mut next_utxos,
                    &mut block_undo,
                    transaction,
                    block_time,
                    verify_flags,
                    &block_context,
                )?;
                let next_total_fees_sats = total_fees_sats
                    .checked_add(fee.to_sats())
                    .ok_or_else(accumulated_fee_out_of_range)?;
                if !(0..=MAX_MONEY).contains(&next_total_fees_sats) {
                    return Err(accumulated_fee_out_of_range());
                }
                total_fees_sats = next_total_fees_sats;
            }

            add_transaction_outputs(
                &mut next_utxos,
                transaction,
                height,
                previous_median_time_past,
            )?;
        }
        enforce_coinbase_reward_limit(
            block,
            height,
            total_fees_sats,
            &block_context.consensus_params,
        )
        .map_err(|source| ChainstateError::BlockValidation { source })?;

        let median_time_past =
            compute_median_time_past(&self.active_chain, Some(block.header.time));
        let position =
            ChainPosition::new(block.header.clone(), height, chain_work, median_time_past);
        self.utxos = next_utxos;
        self.undo_by_block.insert(position.block_hash, block_undo);
        self.active_chain.push(position.clone());
        self.maybe_confirmed_txid_counts = maybe_next_confirmed_txid_counts;

        Ok(position)
    }

    pub fn disconnect_tip(&mut self, block: &Block) -> Result<ChainPosition, ChainstateError> {
        let Some(tip) = self.active_chain.last().cloned() else {
            return Err(ChainstateError::MissingTip);
        };
        let block_hash = block_hash(&block.header);
        if block_hash != tip.block_hash {
            return Err(ChainstateError::DisconnectBlockMismatch {
                expected_tip: tip.block_hash,
                actual_block: block_hash,
            });
        }

        let Some(block_undo) = self.undo_by_block.get(&tip.block_hash).cloned() else {
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

        let maybe_next_confirmed_txid_counts = self
            .maybe_confirmed_txid_counts
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
            .transpose()?;

        for transaction_index in (0..block.transactions.len()).rev() {
            let transaction = &block.transactions[transaction_index];
            remove_transaction_outputs(&mut self.utxos, transaction, tip.height)?;

            if transaction_index > 0 {
                let tx_undo = &block_undo.transactions[transaction_index - 1];
                restore_non_coinbase_inputs(&mut self.utxos, transaction, tx_undo)?;
            }
        }

        self.maybe_confirmed_txid_counts = maybe_next_confirmed_txid_counts;
        self.undo_by_block.remove(&tip.block_hash);
        self.active_chain.pop();
        Ok(tip)
    }

    pub fn reorg(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainTransition, ChainstateError> {
        if disconnect_blocks.len() > self.active_chain.len() {
            return Err(ChainstateError::DisconnectPastGenesis {
                requested: disconnect_blocks.len(),
                available: self.active_chain.len(),
            });
        }

        let mut transition = ChainTransition::default();
        for block in disconnect_blocks {
            transition.disconnected.push(self.disconnect_tip(block)?);
        }
        for anchored_block in replacement_branch {
            let next_block = &anchored_block.block;
            let next_chain_work = anchored_block.chain_work;
            let position =
                self.connect_block(next_block, next_chain_work, verify_flags, consensus_params)?;
            transition.connected.push(position);
        }

        Ok(transition)
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

fn hashmap_parent(utxos: &HashMap<OutPoint, Coin>) -> CoinsCache<MemoryCoinsView> {
    CoinsCache::from_parent(MemoryCoinsView::from_coins(utxos.clone(), None))
}

fn write_overlay_into_hashmap(utxos: &mut HashMap<OutPoint, Coin>, overlay: CoinsOverlay) {
    for (outpoint, entry) in overlay.into_dirty_batch().entries {
        match entry.maybe_coin() {
            Some(coin) => {
                utxos.insert(outpoint, coin.clone());
            }
            None => {
                utxos.remove(&outpoint);
            }
        }
    }
}

fn apply_non_coinbase_transaction(
    next_utxos: &mut HashMap<OutPoint, Coin>,
    block_undo: &mut BlockUndo,
    transaction: &Transaction,
    block_time: i64,
    verify_flags: ScriptVerifyFlags,
    block_context: &BlockValidationContext,
) -> Result<Amount, ChainstateError> {
    let parent = hashmap_parent(next_utxos);
    let mut overlay = CoinsOverlay::new();
    let fee = apply::apply_non_coinbase_transaction(
        &mut overlay,
        &parent,
        block_undo,
        transaction,
        block_time,
        verify_flags,
        block_context,
    )?;
    write_overlay_into_hashmap(next_utxos, overlay);
    Ok(fee)
}

#[cfg(test)]
fn remove_spent_input(
    next_utxos: &mut HashMap<OutPoint, Coin>,
    input: &open_bitcoin_primitives::TransactionInput,
) -> Result<Coin, ChainstateError> {
    let parent = hashmap_parent(next_utxos);
    let mut overlay = CoinsOverlay::new();
    let coin = apply::remove_spent_input(&mut overlay, &parent, input)?;
    write_overlay_into_hashmap(next_utxos, overlay);
    Ok(coin)
}

fn restore_non_coinbase_inputs(
    utxos: &mut HashMap<OutPoint, Coin>,
    transaction: &Transaction,
    tx_undo: &TxUndo,
) -> Result<(), ChainstateError> {
    let parent = hashmap_parent(utxos);
    let mut overlay = CoinsOverlay::new();
    apply::restore_non_coinbase_inputs(&mut overlay, &parent, transaction, tx_undo)?;
    write_overlay_into_hashmap(utxos, overlay);
    Ok(())
}

#[cfg(test)]
fn build_transaction_context(
    transaction: &Transaction,
    utxos: &HashMap<OutPoint, Coin>,
    spend_height: u32,
    block_time: i64,
    median_time_past: i64,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<open_bitcoin_consensus::TransactionValidationContext, ChainstateError> {
    let parent = hashmap_parent(utxos);
    let overlay = CoinsOverlay::new();
    apply::build_transaction_context(
        &overlay,
        &parent,
        transaction,
        block_time,
        verify_flags,
        &BlockValidationContext {
            height: spend_height,
            previous_header: BlockHeader::default(),
            maybe_retarget_anchor: None,
            maybe_min_difficulty_recovery_target: None,
            previous_median_time_past: median_time_past,
            current_time: block_time,
            consensus_params,
        },
    )
}

fn add_transaction_outputs(
    utxos: &mut HashMap<OutPoint, Coin>,
    transaction: &Transaction,
    height: u32,
    created_median_time_past: i64,
) -> Result<(), ChainstateError> {
    let parent = hashmap_parent(utxos);
    let mut overlay = CoinsOverlay::new();
    apply::add_transaction_outputs(
        &mut overlay,
        &parent,
        transaction,
        height,
        created_median_time_past,
    )?;
    write_overlay_into_hashmap(utxos, overlay);
    Ok(())
}

fn remove_transaction_outputs(
    utxos: &mut HashMap<OutPoint, Coin>,
    transaction: &Transaction,
    expected_height: u32,
) -> Result<(), ChainstateError> {
    let parent = hashmap_parent(utxos);
    let mut overlay = CoinsOverlay::new();
    apply::remove_transaction_outputs(&mut overlay, &parent, transaction, expected_height)?;
    write_overlay_into_hashmap(utxos, overlay);
    Ok(())
}

pub(crate) use apply::{accumulated_fee_out_of_range, txid_serialization_error};

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
