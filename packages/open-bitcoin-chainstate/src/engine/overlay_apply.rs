// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::collections::HashMap;

use open_bitcoin_consensus::{
    BlockValidationContext, ScriptVerifyFlags, block_hash, check_block_contextual,
};
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader, Txid};

use super::{
    ConnectApplyOutcome, apply, compute_median_time_past, maybe_min_difficulty_recovery_target,
    maybe_retarget_anchor, next_counts_after_connect, next_counts_after_disconnect,
};
use crate::coins::{CoinsCache, CoinsOverlay, CoinsView};
use crate::{BlockUndo, ChainPosition, ChainstateError};

#[allow(clippy::too_many_arguments)]
pub(super) fn apply_connect_on_overlay<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &CoinsCache<V>,
    active_chain: &[ChainPosition],
    maybe_confirmed_txid_counts: &Option<HashMap<Txid, u32>>,
    block: &Block,
    chain_work: u128,
    current_time: i64,
    verify_flags: ScriptVerifyFlags,
    consensus_params: open_bitcoin_consensus::ConsensusParams,
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

pub(super) fn apply_disconnect_on_overlay<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &CoinsCache<V>,
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
