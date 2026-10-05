// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Cheap metadata admission precedes every restored-script length pass.

use super::*;
use open_bitcoin_core::chainstate::BlockUndo;

pub(super) fn acquisition_limit(budget: BasicIndexTurnBudget) -> TurnWork {
    let normal = budget.normal();
    let absolute = budget.absolute_singleton();
    macro_rules! limit {
        ($field:ident) => {
            normal.$field.max(absolute.$field)
        };
    }
    TurnWork {
        blocks: limit!(blocks),
        body_bytes: limit!(body_bytes),
        undo_bytes: limit!(undo_bytes),
        cloned_bytes: limit!(cloned_bytes),
        script_items: limit!(script_items),
        script_bytes: limit!(script_bytes),
        encoded_bytes: limit!(encoded_bytes),
        record_operations: limit!(record_operations),
        checkpoint_operations: limit!(checkpoint_operations),
        projection_operations: limit!(projection_operations),
    }
}

pub(super) fn undo_work(
    maybe_undo: Option<&BlockUndo>,
    work: &mut TurnWork,
    maximum: TurnWork,
) -> Result<TurnWork, StorageError> {
    let Some(undo) = maybe_undo else {
        return Ok(TurnWork::default());
    };
    let transactions = undo.transactions.len() as u64;
    charge_metadata(work, transactions, maximum)?;
    let inputs = undo.transactions.iter().try_fold(0_u64, |sum, tx| {
        sum.checked_add(tx.restored_inputs.len() as u64)
            .ok_or_else(|| index_corruption("BASIC undo count overflow"))
    })?;
    charge_metadata(
        work,
        transactions
            .checked_add(inputs)
            .ok_or_else(|| index_corruption("BASIC undo work overflow"))?,
        maximum,
    )?;
    crate::chainstate::filter_index::borrowed_undo_work(maybe_undo)
}

fn charge_metadata(work: &mut TurnWork, count: u64, maximum: TurnWork) -> Result<(), StorageError> {
    let total = work
        .checked_add(TurnWork {
            checkpoint_operations: count,
            ..TurnWork::default()
        })
        .map_err(index_corruption)?;
    if !total.fits(maximum) {
        return Err(index_corruption("BASIC undo metadata budget"));
    }
    *work = total;
    Ok(())
}
