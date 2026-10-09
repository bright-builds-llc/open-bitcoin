// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Required consensus inputs cannot be replaced by immutable filter records.

use crate::storage::{FjallNodeStore, StorageError, filter_index::index_corruption};
use open_bitcoin_core::chainstate::{
    AnchoredBlock, BasicFilterInputs, BlockUndo, ChainPosition, HistoricalBlockUndo, IndexPrefix,
    StagedChainstateReorg, filter_index::catch_up::TurnWork,
};
use open_bitcoin_core::primitives::BlockHash;
use std::collections::HashMap;

pub(crate) fn production_budget()
-> Result<open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexTurnBudget, StorageError>
{
    let normal = TurnWork {
        blocks: 8,
        body_bytes: 1024 * 1024,
        undo_bytes: 4 * 1024 * 1024,
        cloned_bytes: 16 * 1024 * 1024,
        script_items: 4 * 1024 * 1024,
        script_bytes: 32 * 1024 * 1024,
        encoded_bytes: 1024 * 1024,
        record_operations: 512,
        checkpoint_operations: 1_000_000,
        projection_operations: 256,
    };
    let absolute = TurnWork {
        blocks: 1,
        body_bytes: 4_000_000 * 32,
        undo_bytes: 256 * 1024 * 1024,
        cloned_bytes: 1024 * 1024 * 1024,
        script_items: 128 * 1024 * 1024,
        script_bytes: 64 * 1024 * 1024 * 1024,
        encoded_bytes: 0x0200_0000 + 170,
        record_operations: 512,
        checkpoint_operations: 1_000_000,
        projection_operations: 256,
    };
    open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexTurnBudget::new(
        normal, absolute,
    )
    .map_err(index_corruption)
}

pub(in crate::chainstate) fn required_sources(
    store: &FjallNodeStore,
    staged: &StagedChainstateReorg,
    replacements: &[AnchoredBlock],
    old_positions: &[ChainPosition],
    old_undo: &HashMap<BlockHash, BlockUndo>,
    processed: open_bitcoin_core::chainstate::FilterCheckpoint,
) -> Result<TurnWork, StorageError> {
    let mut work = TurnWork::default();
    // Reuse the existing finite scheduler envelope, including source dimensions.
    let budget = production_budget()?;
    let normal = budget.normal();
    let absolute = budget.absolute_singleton();
    let maximum = TurnWork {
        blocks: normal.blocks,
        ..absolute
    };
    for position in &staged.transition().disconnected {
        historical_source(store, position, old_undo, &mut work, maximum)?;
    }
    if let Some(ancestor) = staged.maybe_common_ancestor() {
        let first = match processed.prefix() {
            IndexPrefix::Empty => 0,
            IndexPrefix::Committed(id) => id
                .height()
                .checked_add(1)
                .ok_or_else(|| index_corruption("BASIC preflight height exhausted"))?,
        };
        for height in first..=ancestor.height {
            let position = old_positions
                .get(height as usize)
                .ok_or_else(|| index_corruption("BASIC preflight missing shared position"))?;
            if store
                .maybe_basic_filter_reusable_record(position, &mut work, maximum, |work, cost| {
                    charge(work, cost, maximum)?;
                    Ok(Some(maximum))
                })?
                .maybe_record()
                .is_none()
            {
                historical_source(store, position, old_undo, &mut work, maximum)?;
            }
        }
    }
    for (anchored, position) in replacements.iter().zip(&staged.transition().connected) {
        charge(
            &mut work,
            TurnWork {
                checkpoint_operations: 1,
                ..Default::default()
            },
            maximum,
        )?;
        let undo = staged
            .maybe_replacement_undo(position.block_hash)
            .ok_or_else(|| index_corruption("missing genuine BASIC replacement undo"))?;
        let transaction_count = anchored.block.transactions.len() as u64;
        charge(
            &mut work,
            TurnWork {
                checkpoint_operations: transaction_count,
                ..Default::default()
            },
            maximum,
        )?;
        for tx in &anchored.block.transactions {
            charge(
                &mut work,
                TurnWork {
                    checkpoint_operations: tx.inputs.len() as u64 + tx.outputs.len() as u64,
                    ..Default::default()
                },
                maximum,
            )?;
        }
        let mut body_work = TurnWork {
            body_bytes: std::mem::size_of_val(&anchored.block) as u64,
            ..Default::default()
        };
        for tx in &anchored.block.transactions {
            body_work.body_bytes = body_work
                .body_bytes
                .checked_add(std::mem::size_of_val(tx) as u64)
                .ok_or_else(|| index_corruption("BASIC borrowed body overflow"))?;
            for input in &tx.inputs {
                body_work.body_bytes = body_work
                    .body_bytes
                    .checked_add(
                        std::mem::size_of_val(input) as u64
                            + input.script_sig.as_bytes().len() as u64,
                    )
                    .ok_or_else(|| index_corruption("BASIC borrowed body overflow"))?;
            }
            for output in &tx.outputs {
                let len = output.script_pubkey.as_bytes().len() as u64;
                body_work.body_bytes = body_work
                    .body_bytes
                    .checked_add(std::mem::size_of_val(output) as u64 + len)
                    .ok_or_else(|| index_corruption("BASIC borrowed body overflow"))?;
                body_work.script_items += 1;
                body_work.script_bytes = body_work
                    .script_bytes
                    .checked_add(len)
                    .ok_or_else(|| index_corruption("BASIC borrowed script overflow"))?;
            }
        }
        // Txid encoding plus merkle layers use the existing turn's four-copy
        // reservation; witness bytes are not part of txid/body binding.
        body_work.cloned_bytes = body_work
            .body_bytes
            .checked_mul(4)
            .ok_or_else(|| index_corruption("BASIC borrowed body allocation overflow"))?;
        charge(&mut work, body_work, maximum)?;
        charge_undo(undo, &mut work, maximum)?;
        BasicFilterInputs::from_historical(
            &anchored.block,
            position,
            Some(HistoricalBlockUndo {
                block_hash: position.block_hash,
                undo,
            }),
        )
        .map_err(index_corruption)?;
    }
    Ok(work)
}

fn historical_source(
    store: &FjallNodeStore,
    position: &ChainPosition,
    old_undo: &HashMap<BlockHash, BlockUndo>,
    work: &mut TurnWork,
    maximum: TurnWork,
) -> Result<(), StorageError> {
    charge(
        work,
        TurnWork {
            record_operations: 1,
            ..Default::default()
        },
        maximum,
    )?;
    let (block, _) = store
        .maybe_basic_filter_turn_body(
            position.block_hash,
            maximum.body_bytes.saturating_sub(work.body_bytes),
            true,
            |cost| {
                charge(work, cost, maximum)?;
                Ok(true)
            },
        )?
        .ok_or_else(|| index_corruption("missing BASIC required body"))?;
    let maybe_undo = if position.height == 0 {
        None
    } else {
        let expected = old_undo
            .get(&position.block_hash)
            .ok_or_else(|| index_corruption("missing BASIC genuine historical undo"))?;
        Some(store.basic_filter_required_undo(position.block_hash, expected, work, maximum)?)
    };
    BasicFilterInputs::from_historical(
        &block,
        position,
        maybe_undo.as_ref().map(|undo| HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo,
        }),
    )
    .map_err(index_corruption)?;
    Ok(())
}

fn charge_undo(
    undo: &BlockUndo,
    work: &mut TurnWork,
    maximum: TurnWork,
) -> Result<(), StorageError> {
    charge(
        work,
        TurnWork {
            checkpoint_operations: undo.transactions.len() as u64,
            ..Default::default()
        },
        maximum,
    )?;
    for tx in &undo.transactions {
        charge(
            work,
            TurnWork {
                checkpoint_operations: tx.restored_inputs.len() as u64,
                ..Default::default()
            },
            maximum,
        )?;
    }
    charge(
        work,
        crate::chainstate::filter_index::borrowed_undo_work(Some(undo))?,
        maximum,
    )
}

fn charge(work: &mut TurnWork, cost: TurnWork, maximum: TurnWork) -> Result<(), StorageError> {
    let total = work.checked_add(cost).map_err(index_corruption)?;
    if !total.fits(maximum) {
        return Err(index_corruption("BASIC required-source preflight budget"));
    }
    *work = total;
    Ok(())
}
