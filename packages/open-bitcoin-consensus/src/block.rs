// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/block.h
// - packages/bitcoin-knots/src/consensus/merkle.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/validation.cpp

mod difficulty;
pub mod intrinsic;
mod mutation;
pub mod sigops;
mod witness;

use open_bitcoin_codec::CodecError;
use open_bitcoin_primitives::{Amount, Block, BlockHeader, COIN, MAX_MONEY};

use crate::context::{
    BlockValidationContext, ConsensusParams, SpentOutput, TransactionValidationContext,
    is_final_transaction,
};
#[cfg(test)]
use crate::crypto::block_merkle_root;
use crate::crypto::{
    CompactTargetError, block_hash, check_proof_of_work, check_proof_of_work_with_params,
    transaction_txid,
};
use crate::transaction::{validate_transaction, validate_transaction_with_context};
use crate::validation::{
    BlockValidationError, BlockValidationResult, TxValidationError, block_error,
};
use crate::{MAX_BLOCK_SIGOPS_COST, MAX_BLOCK_WEIGHT, WITNESS_SCALE_FACTOR};
use difficulty::next_work_required;
use sigops::BlockSigopBudget;
#[cfg(test)]
use sigops::legacy_sigop_cost;
use witness::check_witness_commitment;
#[cfg(test)]
use witness::{block_witness_merkle_root, witness_commitment_index};

const MAX_FUTURE_BLOCK_TIME_SECONDS: i64 = 7_200;

/// Classify exact serialized mutation facts without validating consensus or granting authority.
pub fn is_block_mutated(
    block: &impl open_bitcoin_codec::block::wire::BlockView,
    check_witness_root: bool,
) -> Result<bool, CodecError> {
    mutation::is_block_mutated(block, check_witness_root)
}

/// Compute the shared witness tree with a zero coinbase leaf; this is a data fact only.
pub fn witness_merkle_root(
    block: &impl open_bitcoin_codec::block::wire::BlockView,
) -> Result<open_bitcoin_primitives::MerkleRoot, CodecError> {
    mutation::witness_merkle_root(block)
}

/// Fill only an absent coinbase witness reserve under the earned candidate rules.
/// Intrinsic validation and contextual header admission remain the caller's gates.
pub fn update_uncommitted_block_structures(block: &mut Block, rules: &crate::CandidateRules) {
    witness::update_uncommitted_block_structures(block, rules);
}

/// Returns the block subsidy for `height` using the configured halving interval.
pub fn block_subsidy(height: u32, consensus_params: &ConsensusParams) -> Amount {
    let halvings = height / consensus_params.subsidy_halving_interval;
    if halvings >= 64 {
        return Amount::ZERO;
    }
    Amount::from_sats((50 * COIN) >> halvings).unwrap_or(Amount::ZERO)
}

/// Rejects coinbases that pay more than the current subsidy plus accumulated fees.
pub fn enforce_coinbase_reward_limit(
    block: &Block,
    height: u32,
    total_fees_sats: i64,
    consensus_params: &ConsensusParams,
) -> Result<(), BlockValidationError> {
    let Some(coinbase_transaction) = block.transactions.first() else {
        return Err(consensus_error(
            "bad-cb-missing",
            Some("first tx is not coinbase".to_string()),
        ));
    };
    let reward_limit_sats = block_subsidy(height, consensus_params)
        .to_sats()
        .checked_add(total_fees_sats)
        .ok_or_else(accumulated_fee_out_of_range)?;
    let coinbase_value_sats = coinbase_transaction
        .outputs
        .iter()
        .map(|output| output.value.to_sats())
        .try_fold(0_i64, |value_sum, value| value_sum.checked_add(value))
        .ok_or_else(|| {
            consensus_error(
                "bad-txns-txouttotal-toolarge",
                Some("total output value out of range".to_string()),
            )
        })?;

    if coinbase_value_sats > reward_limit_sats {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-cb-amount",
            Some(format!(
                "coinbase pays too much (actual={coinbase_value_sats} vs limit={reward_limit_sats})"
            )),
        ));
    }

    Ok(())
}

pub fn check_block_header(header: &BlockHeader) -> Result<(), BlockValidationError> {
    check_header_proof_result(check_proof_of_work(
        block_hash(header).to_byte_array(),
        header.bits,
    ))
}

/// Admit a header's target and hash under the selected consensus powLimit.
/// Contextual difficulty, timestamps and ancestry remain separate checks.
pub fn check_block_header_with_params(
    header: &BlockHeader,
    params: &ConsensusParams,
) -> Result<(), BlockValidationError> {
    check_header_proof_result(check_proof_of_work_with_params(
        block_hash(header).to_byte_array(),
        header.bits,
        params,
    ))
}

fn check_header_proof_result(
    result: Result<bool, CompactTargetError>,
) -> Result<(), BlockValidationError> {
    let valid = result.map_err(|error| {
        block_error(
            BlockValidationResult::InvalidHeader,
            "bad-diffbits",
            Some(error.to_string()),
        )
    })?;
    if !valid {
        return Err(block_error(
            BlockValidationResult::InvalidHeader,
            "high-hash",
            Some("proof of work failed".to_string()),
        ));
    }

    Ok(())
}

pub fn check_block(block: &Block) -> Result<(), BlockValidationError> {
    check_block_header(&block.header)?;
    intrinsic::check_block_view(block)
}

pub fn validate_block(
    block: &Block,
    spent_outputs: &[Vec<SpentOutput>],
) -> Result<(), BlockValidationError> {
    check_block(block)?;

    if spent_outputs.len() != block.transactions.len().saturating_sub(1) {
        return Err(block_error(
            BlockValidationResult::MissingPrev,
            "bad-txns-inputs-missingorspent",
            None,
        ));
    }

    for (transaction, transaction_spent_outputs) in
        block.transactions.iter().skip(1).zip(spent_outputs)
    {
        validate_transaction(transaction, transaction_spent_outputs)
            .map_err(|error| map_transaction_validation_error(transaction, error))?;
    }

    Ok(())
}

pub fn check_block_header_contextual(
    header: &BlockHeader,
    context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    let expected_bits = next_work_required(header, context)?;
    if header.bits != expected_bits {
        return Err(block_error(
            BlockValidationResult::InvalidHeader,
            "bad-diffbits",
            Some("incorrect proof of work".to_string()),
        ));
    }

    if i64::from(header.time) <= context.previous_median_time_past {
        return Err(block_error(
            BlockValidationResult::InvalidHeader,
            "time-too-old",
            Some("block's timestamp is too early".to_string()),
        ));
    }

    if i64::from(header.time)
        > context
            .current_time
            .saturating_add(MAX_FUTURE_BLOCK_TIME_SECONDS)
    {
        return Err(block_error(
            BlockValidationResult::TimeFuture,
            "time-too-new",
            Some("block timestamp too far in the future".to_string()),
        ));
    }

    Ok(())
}

/// Generic fixture validation. Legacy small-height byte pushes remain supported here;
/// selected history validation additionally requires the exact CScript integer prefix.
pub fn check_block_contextual(
    block: &Block,
    context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    check_block_contextual_rules(block, context, |block, height| {
        coinbase_has_height_prefix(block, height)
            || coinbase_has_prefix(block, &serialized_script_num(i64::from(height)))
    })
}

fn check_block_contextual_rules(
    block: &Block,
    context: &BlockValidationContext,
    has_height_prefix: fn(&Block, u32) -> bool,
) -> Result<(), BlockValidationError> {
    check_block(block)?;
    check_block_header_contextual(&block.header, context)?;

    let lock_time_cutoff = if context.consensus_params.enforce_bip113_median_time_past {
        context.previous_median_time_past
    } else {
        i64::from(block.header.time)
    };
    for transaction in &block.transactions {
        if !is_final_transaction(
            transaction,
            context.height,
            lock_time_cutoff,
            &context.consensus_params,
        ) {
            return Err(block_error(
                BlockValidationResult::Consensus,
                "bad-txns-nonfinal",
                Some("non-final transaction".to_string()),
            ));
        }
    }

    if context.consensus_params.enforce_bip34_height_in_coinbase
        && !has_height_prefix(block, context.height)
    {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-cb-height",
            Some("block height mismatch in coinbase".to_string()),
        ));
    }

    check_witness_commitment(block, context)?;

    let stripped_size = serialized_block_size(block, false).map_err(map_codec_error)?;
    let total_size = serialized_block_size(block, true).map_err(map_codec_error)?;
    let weight = stripped_size
        .saturating_mul(WITNESS_SCALE_FACTOR - 1)
        .saturating_add(total_size);
    if weight > MAX_BLOCK_WEIGHT {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-blk-weight",
            Some("weight limit failed".to_string()),
        ));
    }

    Ok(())
}

/// Check a body against the exact selected structural branch and shell clock.
/// This pure check grants no header lineage, coins ownership or acceptance receipt.
pub fn check_block_with_candidate_history(
    block: &Block,
    history: &crate::CandidateHistory,
    shell_observed_now: i64,
) -> Result<(), BlockValidationError> {
    if &block.header != history.candidate() {
        return Err(consensus_error("bad-candidate-context", None));
    }
    history.check_header_rules(shell_observed_now)?;
    check_block_contextual_rules(
        block,
        &history.block_validation_context(shell_observed_now),
        coinbase_has_height_prefix,
    )
}

pub fn validate_block_with_context(
    block: &Block,
    transaction_contexts: &[TransactionValidationContext],
    block_context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    check_block_contextual(block, block_context)?;

    if transaction_contexts.len() != block.transactions.len().saturating_sub(1) {
        return Err(block_error(
            BlockValidationResult::MissingPrev,
            "bad-txns-inputs-missingorspent",
            None,
        ));
    }

    let mut total_fees_sats = 0_i64;
    for (transaction, transaction_context) in
        block.transactions.iter().skip(1).zip(transaction_contexts)
    {
        let fee = validate_transaction_with_context(transaction, transaction_context)
            .map_err(|error| map_transaction_validation_error(transaction, error))?;
        let next_total_fees_sats = total_fees_sats
            .checked_add(fee.to_sats())
            .ok_or_else(accumulated_fee_out_of_range)?;
        if !(0..=MAX_MONEY).contains(&next_total_fees_sats) {
            return Err(accumulated_fee_out_of_range());
        }
        total_fees_sats = next_total_fees_sats;
    }
    enforce_coinbase_reward_limit(
        block,
        block_context.height,
        total_fees_sats,
        &block_context.consensus_params,
    )?;

    let mut sigop_budget = BlockSigopBudget::new();
    for coinbase in block.transactions.iter().take(1) {
        sigop_budget.add_transaction(coinbase, &[], crate::ScriptVerifyFlags::NONE)?;
    }
    for (transaction, transaction_context) in
        block.transactions.iter().skip(1).zip(transaction_contexts)
    {
        let spent_outputs: Vec<_> = transaction_context
            .inputs
            .iter()
            .map(|input| input.spent_output.clone())
            .collect();
        sigop_budget.add_transaction(
            transaction,
            &spent_outputs,
            transaction_context.verify_flags,
        )?;
    }

    Ok(())
}

fn accumulated_fee_out_of_range() -> BlockValidationError {
    consensus_error(
        "bad-txns-accumulated-fee-outofrange",
        Some("accumulated fee in the block out of range".to_string()),
    )
}
#[cfg(test)]
fn split_sigop_cost(
    transaction: &open_bitcoin_primitives::Transaction,
    transaction_context: &TransactionValidationContext,
) -> Result<usize, BlockValidationError> {
    let spent_outputs: Vec<_> = transaction_context
        .inputs
        .iter()
        .map(|input| input.spent_output.clone())
        .collect();
    sigops::split_sigop_cost(
        transaction,
        &spent_outputs,
        transaction_context.verify_flags,
    )
}
fn serialized_block_size(block: &Block, include_witness: bool) -> Result<usize, CodecError> {
    intrinsic::serialized_block_size(block, include_witness)
}

fn compact_size_len(value: u64) -> usize {
    match value {
        0..=252 => 1,
        253..=0xffff => 3,
        0x1_0000..=0xffff_ffff => 5,
        _ => 9,
    }
}

fn map_transaction_validation_error(
    transaction: &open_bitcoin_primitives::Transaction,
    error: TxValidationError,
) -> BlockValidationError {
    let txid = format!(
        "{:?}",
        transaction_txid(transaction).map_or([0_u8; 32], |txid| txid.to_byte_array())
    );
    let debug_message = error.debug_message.map_or_else(
        || format!("transaction {txid} failed validation"),
        |source_debug_message| {
            format!("transaction {txid} failed validation: {source_debug_message}")
        },
    );

    block_error(
        BlockValidationResult::Consensus,
        error.reject_reason,
        Some(debug_message),
    )
}

fn coinbase_has_height_prefix(block: &Block, height: u32) -> bool {
    let expected_prefix = match height {
        1..=16 => vec![0x50 + height as u8],
        _ => serialized_script_num(i64::from(height)),
    };
    coinbase_has_prefix(block, &expected_prefix)
}

fn coinbase_has_prefix(block: &Block, expected_prefix: &[u8]) -> bool {
    let Some(coinbase_input) = block
        .transactions
        .first()
        .and_then(|transaction| transaction.inputs.first())
    else {
        return false;
    };
    let script_sig = coinbase_input.script_sig.as_bytes();

    script_sig.starts_with(expected_prefix)
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    if encoded.last().is_some_and(|byte| (byte & 0x80) != 0) {
        encoded.push(if negative { 0x80 } else { 0x00 });
    } else if negative && let Some(last) = encoded.last_mut() {
        *last |= 0x80;
    }

    let mut script = Vec::with_capacity(encoded.len() + 1);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script
}

fn map_codec_error(error: CodecError) -> BlockValidationError {
    consensus_error("bad-blk-serialization", Some(error.to_string()))
}

fn map_script_error(error: crate::script::ScriptError) -> BlockValidationError {
    consensus_error("bad-blk-script", Some(error.to_string()))
}
fn block_sigop_overflow() -> BlockValidationError {
    consensus_error(
        "bad-blk-sigops",
        Some("out-of-bounds SigOpCount".to_string()),
    )
}

fn consensus_error(
    reason: &'static str,
    maybe_debug_message: Option<String>,
) -> BlockValidationError {
    block_error(
        BlockValidationResult::Consensus,
        reason,
        maybe_debug_message,
    )
}

fn enforce_sigop_cost_limit(sigop_cost: usize) -> Result<(), BlockValidationError> {
    if sigop_cost > MAX_BLOCK_SIGOPS_COST {
        return Err(block_sigop_overflow());
    }
    Ok(())
}

#[cfg(test)]
mod phase167_tests;
#[cfg(test)]
mod tests;
