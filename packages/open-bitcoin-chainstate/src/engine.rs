use std::collections::HashMap;

use open_bitcoin_consensus::block::enforce_coinbase_reward_limit;
use open_bitcoin_consensus::context::{MinDifficultyRecoveryTarget, RetargetAnchor};
use open_bitcoin_consensus::{
    BlockValidationContext, BlockValidationResult, ConsensusParams, ScriptVerifyFlags,
    TransactionInputContext, TransactionValidationContext, ValidationError, block_hash,
    check_block_contextual, transaction_txid, validate_transaction_with_context,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, Transaction,
};

use crate::{
    AnchoredBlock, BlockUndo, ChainPosition, ChainTransition, ChainstateError, ChainstateSnapshot,
    Coin, TxUndo,
};

const MEDIAN_TIME_PAST_WINDOW: usize = 11;
const OP_RETURN: u8 = 0x6a;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Chainstate {
    active_chain: Vec<ChainPosition>,
    utxos: HashMap<OutPoint, Coin>,
    undo_by_block: HashMap<BlockHash, BlockUndo>,
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
        }
    }

    pub fn snapshot(&self) -> ChainstateSnapshot {
        ChainstateSnapshot::new(
            self.active_chain.clone(),
            self.utxos.clone(),
            self.undo_by_block.clone(),
        )
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
                total_fees_sats = total_fees_sats
                    .checked_add(fee.to_sats())
                    .ok_or_else(accumulated_fee_out_of_range)?;
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

        let Some(block_undo) = self.undo_by_block.remove(&tip.block_hash) else {
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

        for transaction_index in (0..block.transactions.len()).rev() {
            let transaction = &block.transactions[transaction_index];
            remove_transaction_outputs(&mut self.utxos, transaction, tip.height)?;

            if transaction_index > 0 {
                let tx_undo = &block_undo.transactions[transaction_index - 1];
                restore_non_coinbase_inputs(&mut self.utxos, transaction, tx_undo)?;
            }
        }

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

fn apply_non_coinbase_transaction(
    next_utxos: &mut HashMap<OutPoint, Coin>,
    block_undo: &mut BlockUndo,
    transaction: &Transaction,
    block_time: i64,
    verify_flags: ScriptVerifyFlags,
    block_context: &BlockValidationContext,
) -> Result<Amount, ChainstateError> {
    let transaction_context = build_transaction_context(
        transaction,
        next_utxos,
        block_context.height,
        block_time,
        block_context.previous_median_time_past,
        verify_flags,
        block_context.consensus_params,
    )?;
    let fee = validate_transaction_with_context(transaction, &transaction_context)
        .map_err(|source| ChainstateError::TransactionValidation { source })?;

    let mut undo = TxUndo::default();
    for input in &transaction.inputs {
        let coin = next_utxos
            .remove(&input.previous_output)
            .expect("validated transaction inputs must still exist during apply phase");
        undo.restored_inputs.push(coin);
    }
    block_undo.transactions.push(undo);

    Ok(fee)
}

fn accumulated_fee_out_of_range() -> ChainstateError {
    ChainstateError::BlockValidation {
        source: ValidationError::new(
            BlockValidationResult::Consensus,
            "bad-txns-accumulated-fee-outofrange",
            Some("accumulated fee in the block out of range".to_string()),
        ),
    }
}

fn restore_non_coinbase_inputs(
    utxos: &mut HashMap<OutPoint, Coin>,
    transaction: &Transaction,
    tx_undo: &TxUndo,
) -> Result<(), ChainstateError> {
    if tx_undo.restored_inputs.len() != transaction.inputs.len() {
        return Err(ChainstateError::UndoMismatch {
            expected_transactions: transaction.inputs.len(),
            actual_transactions: tx_undo.restored_inputs.len(),
        });
    }

    for (input, restored_coin) in transaction
        .inputs
        .iter()
        .zip(&tx_undo.restored_inputs)
        .rev()
    {
        let outpoint = input.previous_output.clone();
        if utxos.contains_key(&outpoint) {
            return Err(ChainstateError::RestoredCoinOverwrite { outpoint });
        }
        utxos.insert(outpoint, restored_coin.clone());
    }

    Ok(())
}

fn build_transaction_context(
    transaction: &Transaction,
    utxos: &HashMap<OutPoint, Coin>,
    spend_height: u32,
    block_time: i64,
    median_time_past: i64,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<TransactionValidationContext, ChainstateError> {
    let mut inputs = Vec::with_capacity(transaction.inputs.len());
    for input in &transaction.inputs {
        let Some(coin) = utxos.get(&input.previous_output) else {
            return Err(ChainstateError::MissingCoin {
                outpoint: input.previous_output.clone(),
            });
        };
        inputs.push(TransactionInputContext {
            spent_output: coin.as_spent_output(),
            created_height: coin.created_height,
            created_median_time_past: coin.created_median_time_past,
        });
    }

    Ok(TransactionValidationContext {
        inputs,
        spend_height,
        block_time,
        median_time_past,
        verify_flags,
        consensus_params,
    })
}

fn add_transaction_outputs(
    utxos: &mut HashMap<OutPoint, Coin>,
    transaction: &Transaction,
    height: u32,
    created_median_time_past: i64,
) -> Result<(), ChainstateError> {
    let txid = transaction_txid(transaction)
        .expect("typed transactions should serialize for txid derivation");
    for (vout, output) in transaction.outputs.iter().enumerate() {
        if is_unspendable_script(&output.script_pubkey) {
            continue;
        }

        let outpoint = OutPoint {
            txid,
            vout: vout as u32,
        };
        if utxos.contains_key(&outpoint) {
            return Err(ChainstateError::OutputOverwrite { outpoint });
        }

        utxos.insert(
            outpoint,
            Coin {
                output: output.clone(),
                is_coinbase: transaction.is_coinbase(),
                created_height: height,
                created_median_time_past,
            },
        );
    }

    Ok(())
}

fn remove_transaction_outputs(
    utxos: &mut HashMap<OutPoint, Coin>,
    transaction: &Transaction,
    expected_height: u32,
) -> Result<(), ChainstateError> {
    let txid = transaction_txid(transaction)
        .expect("typed transactions should serialize for txid derivation");
    for (vout, output) in transaction.outputs.iter().enumerate() {
        if is_unspendable_script(&output.script_pubkey) {
            continue;
        }

        let outpoint = OutPoint {
            txid,
            vout: vout as u32,
        };
        let Some(existing_coin) = utxos.remove(&outpoint) else {
            return Err(ChainstateError::DisconnectSpentOutputMismatch { outpoint });
        };
        if existing_coin.output != *output
            || existing_coin.created_height != expected_height
            || existing_coin.is_coinbase != transaction.is_coinbase()
        {
            return Err(ChainstateError::DisconnectSpentOutputMismatch { outpoint });
        }
    }

    Ok(())
}

fn compute_median_time_past(active_chain: &[ChainPosition], maybe_new_time: Option<u32>) -> i64 {
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

fn is_unspendable_script(script_pubkey: &ScriptBuf) -> bool {
    script_pubkey
        .as_bytes()
        .first()
        .is_some_and(|opcode| *opcode == OP_RETURN)
}

#[cfg(test)]
mod tests;
