// Parity breadcrumbs:
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::{block_sigop_overflow, enforce_sigop_cost_limit, map_script_error};
use crate::script::{count_legacy_sigops, count_p2sh_sigops, count_witness_sigops};
use crate::validation::{BlockValidationError, BlockValidationResult, block_error};
use crate::{ScriptVerifyFlags, SpentOutput, WITNESS_SCALE_FACTOR};
use open_bitcoin_codec::{block::wire::BlockView, transaction::wire::TransactionView};
use open_bitcoin_primitives::Transaction;

/// Checked aggregate cost. This is a pure counter, never an acceptance receipt.
/// The coins owner supplies each transaction's real inputs before consuming them.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSigopBudget {
    cost: usize,
}

impl BlockSigopBudget {
    /// Start one budget per block, including its coinbase.
    pub const fn new() -> Self {
        Self { cost: 0 }
    }

    /// Aggregate legacy*4 + active P2SH*4 + active witness cost so far.
    pub const fn cost(&self) -> usize {
        self.cost
    }

    /// Account exactly once, using exact consensus flags and one coin per noncoinbase input.
    /// Missing inputs, arithmetic overflow and totals above80000 leave the budget unchanged.
    pub fn add_transaction(
        &mut self,
        transaction: &Transaction,
        spent_outputs: &[SpentOutput],
        flags: ScriptVerifyFlags,
    ) -> Result<(), BlockValidationError> {
        let cost = transaction_sigop_cost(transaction, spent_outputs, flags)?;
        self.add_cost(cost)
    }

    fn add_cost(&mut self, cost: usize) -> Result<(), BlockValidationError> {
        let total = self
            .cost
            .checked_add(cost)
            .ok_or_else(block_sigop_overflow)?;
        enforce_sigop_cost_limit(total)?;
        self.cost = total;
        Ok(())
    }
}

/// Pinned GetTransactionSigOpCost transformation, without script execution or coins authority.
pub fn transaction_sigop_cost(
    transaction: &Transaction,
    spent_outputs: &[SpentOutput],
    flags: ScriptVerifyFlags,
) -> Result<usize, BlockValidationError> {
    let legacy = legacy_sigop_cost(transaction)?;
    if transaction.is_coinbase() {
        return Ok(legacy);
    }
    legacy
        .checked_add(split_sigop_cost(transaction, spent_outputs, flags)?)
        .ok_or_else(block_sigop_overflow)
}

pub(super) fn legacy_block_sigop_cost(
    block: &impl BlockView,
) -> Result<usize, BlockValidationError> {
    block
        .transactions()
        .iter()
        .try_fold(0usize, |cost, transaction| {
            cost.checked_add(legacy_sigop_cost(transaction)?)
                .ok_or_else(block_sigop_overflow)
        })
}

pub(super) fn legacy_sigop_cost(
    transaction: &impl TransactionView,
) -> Result<usize, BlockValidationError> {
    let mut count = 0usize;
    for script in transaction
        .inputs()
        .iter()
        .map(|input| &input.script_sig)
        .chain(transaction.outputs().map(|output| output.script_pubkey))
    {
        count = count
            .checked_add(count_legacy_sigops(script).map_err(map_script_error)?)
            .ok_or_else(block_sigop_overflow)?;
    }
    count
        .checked_mul(WITNESS_SCALE_FACTOR)
        .ok_or_else(block_sigop_overflow)
}

pub(super) fn split_sigop_cost(
    transaction: &Transaction,
    spent_outputs: &[SpentOutput],
    flags: ScriptVerifyFlags,
) -> Result<usize, BlockValidationError> {
    if transaction.inputs.len() != spent_outputs.len() {
        return Err(block_error(
            BlockValidationResult::MissingPrev,
            "bad-txns-inputs-missingorspent",
            None,
        ));
    }
    let mut count = 0usize;
    for (input, spent) in transaction.inputs.iter().zip(spent_outputs) {
        let p2sh = if flags.contains(ScriptVerifyFlags::P2SH) {
            count_p2sh_sigops(&input.script_sig, &spent.script_pubkey)
                .map_err(map_script_error)?
                .checked_mul(WITNESS_SCALE_FACTOR)
                .ok_or_else(block_sigop_overflow)?
        } else {
            0
        };
        let witness = count_witness_sigops(
            &input.script_sig,
            &spent.script_pubkey,
            &input.witness,
            flags,
        )
        .map_err(map_script_error)?;
        count = count
            .checked_add(p2sh)
            .and_then(|cost| cost.checked_add(witness))
            .ok_or_else(block_sigop_overflow)?;
    }
    Ok(count)
}

#[cfg(test)]
mod tests;
