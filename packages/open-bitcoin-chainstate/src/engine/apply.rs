// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use open_bitcoin_consensus::block::enforce_coinbase_reward_limit;
use open_bitcoin_consensus::{
    BlockValidationContext, BlockValidationResult, ScriptVerifyFlags, TransactionInputContext,
    TransactionValidationContext, ValidationError, transaction_txid,
    validate_transaction_with_context,
};
use open_bitcoin_primitives::{
    Amount, Block, MAX_MONEY, OutPoint, ScriptBuf, Transaction, TransactionInput,
};

use crate::coins::{CoinsOverlay, CoinsView};
use crate::{BlockUndo, ChainstateError, Coin, TxUndo};

const OP_RETURN: u8 = 0x6a;

pub(crate) fn apply_non_coinbase_transaction<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &V,
    block_undo: &mut BlockUndo,
    transaction: &Transaction,
    block_time: i64,
    verify_flags: ScriptVerifyFlags,
    block_context: &BlockValidationContext,
) -> Result<Amount, ChainstateError> {
    let transaction_context = build_transaction_context(
        overlay,
        parent,
        transaction,
        block_time,
        verify_flags,
        block_context,
    )?;
    let fee = validate_transaction_with_context(transaction, &transaction_context)
        .map_err(|source| ChainstateError::TransactionValidation { source })?;

    let mut undo = TxUndo::default();
    for input in &transaction.inputs {
        let coin = remove_spent_input(overlay, parent, input)?;
        undo.restored_inputs.push(coin);
    }
    block_undo.transactions.push(undo);

    Ok(fee)
}

pub(crate) fn remove_spent_input<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &V,
    input: &TransactionInput,
) -> Result<Coin, ChainstateError> {
    overlay.spend_coin(parent, &input.previous_output)
}

pub(crate) fn accumulated_fee_out_of_range() -> ChainstateError {
    ChainstateError::BlockValidation {
        source: ValidationError::new(
            BlockValidationResult::Consensus,
            "bad-txns-accumulated-fee-outofrange",
            Some("accumulated fee in the block out of range".to_string()),
        ),
    }
}

pub(crate) fn restore_non_coinbase_inputs<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &V,
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
        if overlay.peek_coin(parent, &outpoint)?.is_some() {
            return Err(ChainstateError::RestoredCoinOverwrite { outpoint });
        }
        overlay.add_coin(parent, outpoint, restored_coin.clone(), false)?;
    }

    Ok(())
}

pub(crate) fn build_transaction_context<V: CoinsView>(
    overlay: &CoinsOverlay,
    parent: &V,
    transaction: &Transaction,
    block_time: i64,
    verify_flags: ScriptVerifyFlags,
    block_context: &BlockValidationContext,
) -> Result<TransactionValidationContext, ChainstateError> {
    let mut inputs = Vec::with_capacity(transaction.inputs.len());
    for input in &transaction.inputs {
        let Some(coin) = overlay.peek_coin(parent, &input.previous_output)? else {
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
        spend_height: block_context.height,
        block_time,
        median_time_past: block_context.previous_median_time_past,
        verify_flags,
        consensus_params: block_context.consensus_params,
    })
}

pub(crate) fn add_transaction_outputs<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &V,
    transaction: &Transaction,
    height: u32,
    created_median_time_past: i64,
) -> Result<(), ChainstateError> {
    let txid = transaction_txid(transaction).map_err(txid_serialization_error)?;
    for (vout, output) in transaction.outputs.iter().enumerate() {
        if is_unspendable_script(&output.script_pubkey) {
            continue;
        }

        let outpoint = OutPoint {
            txid,
            vout: vout as u32,
        };
        overlay.add_coin(
            parent,
            outpoint,
            Coin {
                output: output.clone(),
                is_coinbase: transaction.is_coinbase(),
                created_height: height,
                created_median_time_past,
            },
            false,
        )?;
    }

    Ok(())
}

pub(crate) fn remove_transaction_outputs<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &V,
    transaction: &Transaction,
    expected_height: u32,
) -> Result<(), ChainstateError> {
    let txid = transaction_txid(transaction).map_err(txid_serialization_error)?;
    for (vout, output) in transaction.outputs.iter().enumerate() {
        if is_unspendable_script(&output.script_pubkey) {
            continue;
        }

        let outpoint = OutPoint {
            txid,
            vout: vout as u32,
        };
        let Some(existing_coin) = overlay.peek_coin(parent, &outpoint)? else {
            return Err(ChainstateError::DisconnectSpentOutputMismatch { outpoint });
        };
        if existing_coin.output != *output
            || existing_coin.created_height != expected_height
            || existing_coin.is_coinbase != transaction.is_coinbase()
        {
            return Err(ChainstateError::DisconnectSpentOutputMismatch { outpoint });
        }
        overlay.spend_coin(parent, &outpoint)?;
    }

    Ok(())
}

pub(crate) fn txid_serialization_error(source: impl std::fmt::Display) -> ChainstateError {
    ChainstateError::Serialization {
        context: "txid derivation",
        reason: source.to_string(),
    }
}

pub(crate) fn is_unspendable_script(script_pubkey: &ScriptBuf) -> bool {
    script_pubkey
        .as_bytes()
        .first()
        .is_some_and(|opcode| *opcode == OP_RETURN)
}

pub(crate) fn apply_connect_transactions<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &V,
    block: &Block,
    height: u32,
    previous_median_time_past: i64,
    verify_flags: ScriptVerifyFlags,
    block_context: &BlockValidationContext,
) -> Result<(BlockUndo, i64), ChainstateError> {
    let mut block_undo = BlockUndo::default();
    let block_time = i64::from(block.header.time);
    let mut total_fees_sats = 0_i64;
    for (transaction_index, transaction) in block.transactions.iter().enumerate() {
        if transaction_index > 0 {
            let fee = apply_non_coinbase_transaction(
                overlay,
                parent,
                &mut block_undo,
                transaction,
                block_time,
                verify_flags,
                block_context,
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
            overlay,
            parent,
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

    Ok((block_undo, total_fees_sats))
}

pub(crate) fn apply_disconnect_transactions<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &V,
    block: &Block,
    expected_height: u32,
    block_undo: &BlockUndo,
) -> Result<(), ChainstateError> {
    for transaction_index in (0..block.transactions.len()).rev() {
        let transaction = &block.transactions[transaction_index];
        remove_transaction_outputs(overlay, parent, transaction, expected_height)?;

        if transaction_index > 0 {
            let tx_undo = &block_undo.transactions[transaction_index - 1];
            restore_non_coinbase_inputs(overlay, parent, transaction, tx_undo)?;
        }
    }

    Ok(())
}
