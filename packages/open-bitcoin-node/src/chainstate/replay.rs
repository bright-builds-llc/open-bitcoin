// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Apply-only interrupted-flush replay (`ReplayBlocks` shape) in the node crate.

use std::collections::HashSet;

use open_bitcoin_core::{
    chainstate::{
        BlockUndo, ChainstateError, Coin, CoinsCache, CoinsView, RecoveryDecision, TxUndo,
        decide_recovery,
    },
    consensus::transaction_txid,
    primitives::{Block, BlockHash, OutPoint, Transaction},
};
use open_bitcoin_network::{HeaderEntry, HeaderStore};

use crate::storage::{
    FjallNodeStore, PersistMode, StorageError, StorageNamespace, StorageRecoveryAction,
    coins_view::FjallCoinsView,
};

#[cfg(test)]
mod tests;

pub fn replay_interrupted_flush(
    store: &FjallNodeStore,
    view: FjallCoinsView,
) -> Result<FjallCoinsView, StorageError> {
    let heads = view.head_blocks().map_err(map_chainstate)?;
    match decide_recovery(heads.len()) {
        RecoveryDecision::ConsistentEmptyHeads | RecoveryDecision::OneHead => return Ok(view),
        RecoveryDecision::InconsistentOtherCount { count } => {
            return Err(coins_corruption(format!(
                "unexpected head_blocks count {count}"
            )));
        }
        RecoveryDecision::InterruptedTwoHeads => {}
    }

    let Some(new) = heads.first().copied() else {
        return Err(coins_corruption("interrupted heads missing new tip"));
    };
    let Some(old) = heads.get(1).copied() else {
        return Err(coins_corruption("interrupted heads missing old tip"));
    };

    let zero = BlockHash::from_byte_array([0_u8; 32]);
    let headers = require_header_store(store)?;
    let mut cache = CoinsCache::from_parent(view);
    let fork = if old == zero {
        zero
    } else {
        find_fork(&headers, old, new)?
    };
    if old != zero {
        rollback_old_branch(&mut cache, store, &headers, old, fork)?;
    }
    rollforward_new_branch(&mut cache, store, &headers, fork, new)?;
    cache.set_best_block(new);
    let (mut view, writes) = cache.into_dirty_parent_write();
    view.batch_write_with_persist_mode(writes, Some(new), PersistMode::Sync)
        .map_err(map_chainstate)?;
    Ok(FjallCoinsView::from_store(store))
}

fn require_header_store(store: &FjallNodeStore) -> Result<HeaderStore, StorageError> {
    let Some(headers) = store.load_header_store()? else {
        return Err(interrupted_write());
    };
    Ok(headers)
}

fn find_fork(
    headers: &HeaderStore,
    old: BlockHash,
    new: BlockHash,
) -> Result<BlockHash, StorageError> {
    let mut new_ancestors = HashSet::new();
    new_ancestors.insert(BlockHash::from_byte_array([0_u8; 32]));
    let mut cursor = new;
    while cursor != BlockHash::from_byte_array([0_u8; 32]) {
        new_ancestors.insert(cursor);
        cursor = require_entry(headers, &cursor)?.header.previous_block_hash;
    }

    cursor = old;
    loop {
        if new_ancestors.contains(&cursor) {
            return Ok(cursor);
        }
        if cursor == BlockHash::from_byte_array([0_u8; 32]) {
            return Err(interrupted_write());
        }
        cursor = require_entry(headers, &cursor)?.header.previous_block_hash;
    }
}

fn rollback_old_branch(
    cache: &mut CoinsCache<FjallCoinsView>,
    store: &FjallNodeStore,
    headers: &HeaderStore,
    old: BlockHash,
    fork: BlockHash,
) -> Result<(), StorageError> {
    for hash in walk_exclusive(headers, old, fork)? {
        rollback_block(cache, store, hash)?;
    }
    Ok(())
}

fn rollforward_new_branch(
    cache: &mut CoinsCache<FjallCoinsView>,
    store: &FjallNodeStore,
    headers: &HeaderStore,
    fork: BlockHash,
    new: BlockHash,
) -> Result<(), StorageError> {
    let mut hashes = walk_exclusive(headers, new, fork)?;
    hashes.reverse();
    for hash in hashes {
        rollforward_block(cache, store, headers, hash)?;
    }
    Ok(())
}

fn walk_exclusive(
    headers: &HeaderStore,
    tip: BlockHash,
    fork: BlockHash,
) -> Result<Vec<BlockHash>, StorageError> {
    let mut hashes = Vec::new();
    let mut cursor = tip;
    while cursor != fork {
        if cursor == BlockHash::from_byte_array([0_u8; 32]) {
            return Err(interrupted_write());
        }
        hashes.push(cursor);
        cursor = require_entry(headers, &cursor)?.header.previous_block_hash;
    }
    Ok(hashes)
}

fn rollback_block(
    cache: &mut CoinsCache<FjallCoinsView>,
    store: &FjallNodeStore,
    hash: BlockHash,
) -> Result<(), StorageError> {
    let block = require_block(store, hash)?;
    let undo = require_undo(store, hash)?;
    let non_coinbase = non_coinbase_transactions(&block);
    if undo.transactions.len() != non_coinbase.len() {
        return Err(interrupted_write());
    }

    for transaction_index in (0..block.transactions.len()).rev() {
        spend_created_outputs(cache, &block.transactions[transaction_index])?;
        if transaction_index == 0 {
            continue;
        }
        restore_undo_inputs(
            cache,
            &block.transactions[transaction_index],
            &undo.transactions[transaction_index - 1],
        )?;
    }
    Ok(())
}

fn rollforward_block(
    cache: &mut CoinsCache<FjallCoinsView>,
    store: &FjallNodeStore,
    headers: &HeaderStore,
    hash: BlockHash,
) -> Result<(), StorageError> {
    let block = require_block(store, hash)?;
    let entry = require_entry(headers, &hash)?;
    let created_median_time_past = headers
        .median_time_past(hash)
        .unwrap_or_else(|| i64::from(entry.header.time));
    for transaction in &block.transactions {
        if !transaction.is_coinbase() {
            spend_inputs_idempotent(cache, transaction)?;
        }
        add_created_outputs(cache, transaction, entry, created_median_time_past)?;
    }
    Ok(())
}

fn spend_created_outputs(
    cache: &mut CoinsCache<FjallCoinsView>,
    transaction: &Transaction,
) -> Result<(), StorageError> {
    let txid = txid_of(transaction)?;
    for vout in 0..transaction.outputs.len() {
        spend_idempotent(
            cache,
            &OutPoint {
                txid,
                vout: u32::try_from(vout).unwrap_or(u32::MAX),
            },
        )?;
    }
    Ok(())
}

fn restore_undo_inputs(
    cache: &mut CoinsCache<FjallCoinsView>,
    transaction: &Transaction,
    tx_undo: &TxUndo,
) -> Result<(), StorageError> {
    if tx_undo.restored_inputs.len() != transaction.inputs.len() {
        return Err(interrupted_write());
    }
    for (input, coin) in transaction.inputs.iter().zip(&tx_undo.restored_inputs) {
        cache
            .add_coin(input.previous_output.clone(), coin.clone(), true)
            .map_err(map_chainstate)?;
    }
    Ok(())
}

fn spend_inputs_idempotent(
    cache: &mut CoinsCache<FjallCoinsView>,
    transaction: &Transaction,
) -> Result<(), StorageError> {
    for input in &transaction.inputs {
        spend_idempotent(cache, &input.previous_output)?;
    }
    Ok(())
}

fn add_created_outputs(
    cache: &mut CoinsCache<FjallCoinsView>,
    transaction: &Transaction,
    entry: &HeaderEntry,
    created_median_time_past: i64,
) -> Result<(), StorageError> {
    let txid = txid_of(transaction)?;
    for (vout, output) in transaction.outputs.iter().enumerate() {
        let coin = Coin {
            output: output.clone(),
            is_coinbase: transaction.is_coinbase(),
            created_height: entry.height,
            created_median_time_past,
        };
        cache
            .add_coin(
                OutPoint {
                    txid,
                    vout: u32::try_from(vout).unwrap_or(u32::MAX),
                },
                coin,
                true,
            )
            .map_err(map_chainstate)?;
    }
    Ok(())
}

fn spend_idempotent(
    cache: &mut CoinsCache<FjallCoinsView>,
    outpoint: &OutPoint,
) -> Result<(), StorageError> {
    match cache.spend_coin(outpoint) {
        Ok(_) | Err(ChainstateError::MissingCoin { .. }) => Ok(()),
        Err(error) => Err(map_chainstate(error)),
    }
}

fn require_entry<'a>(
    headers: &'a HeaderStore,
    hash: &BlockHash,
) -> Result<&'a HeaderEntry, StorageError> {
    let Some(entry) = headers.entry(hash) else {
        return Err(interrupted_write());
    };
    Ok(entry)
}

fn require_block(store: &FjallNodeStore, hash: BlockHash) -> Result<Block, StorageError> {
    let Some(block) = store.load_block(hash)? else {
        return Err(interrupted_write());
    };
    Ok(block)
}

fn require_undo(store: &FjallNodeStore, hash: BlockHash) -> Result<BlockUndo, StorageError> {
    let Some(undo) = store.load_undo(hash)? else {
        return Err(interrupted_write());
    };
    Ok(undo)
}

fn non_coinbase_transactions(block: &Block) -> Vec<&Transaction> {
    block
        .transactions
        .iter()
        .filter(|transaction| !transaction.is_coinbase())
        .collect()
}

fn txid_of(transaction: &Transaction) -> Result<open_bitcoin_core::primitives::Txid, StorageError> {
    transaction_txid(transaction).map_err(coins_corruption)
}

fn map_chainstate(error: ChainstateError) -> StorageError {
    match error {
        ChainstateError::InterruptedWrite { .. } => interrupted_write(),
        ChainstateError::CoinsStorage { detail } => coins_corruption(detail),
        other => coins_corruption(other),
    }
}

fn interrupted_write() -> StorageError {
    StorageError::InterruptedWrite {
        namespace: StorageNamespace::Coins,
        action: StorageRecoveryAction::Reindex,
    }
}

fn coins_corruption(detail: impl core::fmt::Display) -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::Coins,
        detail: detail.to_string(),
        action: StorageRecoveryAction::Repair,
    }
}
