mod difficulty;

use open_bitcoin_codec::{
    CodecError, TransactionEncoding, encode_block_header, encode_transaction,
};
use open_bitcoin_primitives::{Amount, Block, BlockHeader, COIN};

use crate::MAX_BLOCK_SIGOPS_COST;
use crate::MAX_BLOCK_WEIGHT;
use crate::WITNESS_SCALE_FACTOR;
use crate::context::{
    BlockValidationContext, ConsensusParams, SpentOutput, TransactionValidationContext,
    is_final_transaction,
};
use crate::crypto::{
    block_hash, block_merkle_root, check_proof_of_work, double_sha256, transaction_txid,
    transaction_wtxid,
};
use crate::script::{count_legacy_sigops, count_p2sh_sigops, count_witness_sigops};
use crate::transaction::{
    check_transaction, validate_transaction, validate_transaction_with_context,
};
use crate::validation::{
    BlockValidationError, BlockValidationResult, TxValidationError, block_error,
};
use difficulty::next_work_required;

const WITNESS_RESERVED_VALUE_STACK_ITEMS: usize = 1;
const WITNESS_RESERVED_VALUE_SIZE: usize = 32;
const WITNESS_COMMITMENT_PREIMAGE_SIZE: usize = 64;
const WITNESS_COMMITMENT_HEADER: [u8; 6] = [0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed];
const WITNESS_COMMITMENT_HEADER_LEN: usize = WITNESS_COMMITMENT_HEADER.len();
const WITNESS_COMMITMENT_SCRIPT_LEN: usize =
    WITNESS_COMMITMENT_HEADER_LEN + WITNESS_RESERVED_VALUE_SIZE;
const WITNESS_COMMITMENT_START: usize = WITNESS_COMMITMENT_HEADER_LEN;
const WITNESS_COMMITMENT_END: usize = WITNESS_COMMITMENT_START + WITNESS_RESERVED_VALUE_SIZE;
const HASH_CONCATENATION_SIZE: usize = 64;
const MAX_FUTURE_BLOCK_TIME_SECONDS: i64 = 7_200;

/// Returns the block subsidy for `height` using the configured halving interval.
pub fn block_subsidy(height: u32, consensus_params: &ConsensusParams) -> Amount {
    let halvings = height / consensus_params.subsidy_halving_interval;
    if halvings >= 64 {
        return Amount::ZERO;
    }

    Amount::from_sats((50 * COIN) >> halvings).expect("subsidy remains within range")
}

/// Rejects coinbases that pay more than the current subsidy plus accumulated fees.
pub fn enforce_coinbase_reward_limit(
    block: &Block,
    height: u32,
    total_fees_sats: i64,
    consensus_params: &ConsensusParams,
) -> Result<(), BlockValidationError> {
    let coinbase_transaction = block
        .transactions
        .first()
        .expect("context-free validation guarantees a coinbase transaction");
    let reward_limit_sats = block_subsidy(height, consensus_params)
        .to_sats()
        .checked_add(total_fees_sats)
        .expect("subsidy plus fees must stay within i64");
    let coinbase_value_sats = coinbase_transaction
        .outputs
        .iter()
        .map(|output| output.value.to_sats())
        .try_fold(0_i64, |value_sum, value| value_sum.checked_add(value))
        .expect("context-free validation keeps coinbase outputs in range");

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
    let valid =
        check_proof_of_work(block_hash(header).to_byte_array(), header.bits).map_err(|error| {
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

    let (merkle_root, maybe_mutated) =
        block_merkle_root(&block.transactions).map_err(map_codec_error)?;
    if block.header.merkle_root != merkle_root {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-txnmrklroot",
            Some("hashMerkleRoot mismatch".to_string()),
        ));
    }
    if maybe_mutated {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-txns-duplicate",
            Some("duplicate transaction".to_string()),
        ));
    }

    if block.transactions.is_empty() {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-blk-length",
            Some("size limits failed".to_string()),
        ));
    }

    let stripped_size = serialized_block_size(block, false).map_err(map_codec_error)?;
    if block
        .transactions
        .len()
        .saturating_mul(WITNESS_SCALE_FACTOR)
        > MAX_BLOCK_WEIGHT
        || stripped_size.saturating_mul(WITNESS_SCALE_FACTOR) > MAX_BLOCK_WEIGHT
    {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-blk-length",
            Some("size limits failed".to_string()),
        ));
    }

    if !block.transactions[0].is_coinbase() {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-cb-missing",
            Some("first tx is not coinbase".to_string()),
        ));
    }
    if block
        .transactions
        .iter()
        .skip(1)
        .any(|transaction| transaction.is_coinbase())
    {
        return Err(block_error(
            BlockValidationResult::Consensus,
            "bad-cb-multiple",
            Some("more than one coinbase".to_string()),
        ));
    }

    for transaction in &block.transactions {
        check_transaction(transaction).map_err(|error| {
            block_error(
                BlockValidationResult::Consensus,
                error.reject_reason,
                error.debug_message.clone(),
            )
        })?;
    }

    let mut sigops = 0_usize;
    for transaction in &block.transactions {
        for input in &transaction.inputs {
            sigops += count_legacy_sigops(&input.script_sig).map_err(map_script_error)?;
        }
        for output in &transaction.outputs {
            sigops += count_legacy_sigops(&output.script_pubkey).map_err(map_script_error)?;
        }
    }
    enforce_sigop_cost_limit(sigops.saturating_mul(WITNESS_SCALE_FACTOR))?;

    Ok(())
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

pub fn check_block_contextual(
    block: &Block,
    context: &BlockValidationContext,
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
        && !coinbase_has_height_prefix(block, context.height)
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

    for (transaction, transaction_context) in
        block.transactions.iter().skip(1).zip(transaction_contexts)
    {
        validate_transaction_with_context(transaction, transaction_context)
            .map_err(|error| map_transaction_validation_error(transaction, error))?;
    }

    let mut sigop_cost = 0_usize;
    for transaction in &block.transactions {
        sigop_cost = sigop_cost.saturating_add(legacy_sigop_cost(transaction)?);
    }
    for (transaction, transaction_context) in
        block.transactions.iter().skip(1).zip(transaction_contexts)
    {
        sigop_cost = sigop_cost.saturating_add(split_sigop_cost(transaction, transaction_context)?);
    }
    enforce_sigop_cost_limit(sigop_cost)?;

    Ok(())
}

fn legacy_sigop_cost(
    transaction: &open_bitcoin_primitives::Transaction,
) -> Result<usize, BlockValidationError> {
    let mut sigops = 0_usize;
    for input in &transaction.inputs {
        sigops = sigops
            .saturating_add(count_legacy_sigops(&input.script_sig).map_err(map_script_error)?);
    }
    for output in &transaction.outputs {
        sigops = sigops
            .saturating_add(count_legacy_sigops(&output.script_pubkey).map_err(map_script_error)?);
    }
    Ok(sigops.saturating_mul(WITNESS_SCALE_FACTOR))
}

fn split_sigop_cost(
    transaction: &open_bitcoin_primitives::Transaction,
    transaction_context: &TransactionValidationContext,
) -> Result<usize, BlockValidationError> {
    let mut sigops = 0_usize;
    for (input, input_context) in transaction.inputs.iter().zip(&transaction_context.inputs) {
        sigops = sigops.saturating_add(
            count_p2sh_sigops(&input.script_sig, &input_context.spent_output.script_pubkey)
                .map_err(map_script_error)?
                .saturating_mul(WITNESS_SCALE_FACTOR),
        );
        sigops = sigops.saturating_add(
            count_witness_sigops(
                &input.script_sig,
                &input_context.spent_output.script_pubkey,
                &input.witness,
                transaction_context.verify_flags,
            )
            .map_err(map_script_error)?,
        );
    }
    Ok(sigops)
}

fn serialized_block_size(block: &Block, include_witness: bool) -> Result<usize, CodecError> {
    let mut size = encode_block_header(&block.header).len();
    size += compact_size_len(block.transactions.len() as u64);
    for transaction in &block.transactions {
        let encoding = if include_witness {
            TransactionEncoding::WithWitness
        } else {
            TransactionEncoding::WithoutWitness
        };
        size += encode_transaction(transaction, encoding)?.len();
    }
    Ok(size)
}

fn compact_size_len(value: u64) -> usize {
    match value {
        0..=252 => 1,
        253..=0xffff => 3,
        0x1_0000..=0xffff_ffff => 5,
        _ => 9,
    }
}

fn check_witness_commitment(
    block: &Block,
    context: &BlockValidationContext,
) -> Result<(), BlockValidationError> {
    let witness_present = block
        .transactions
        .iter()
        .any(|transaction| transaction.has_witness());
    if !context.consensus_params.enforce_segwit {
        if witness_present {
            return Err(block_error(
                BlockValidationResult::Mutated,
                "unexpected-witness",
                Some("unexpected witness data found".to_string()),
            ));
        }

        return Ok(());
    }
    let Some(commitment_index) = witness_commitment_index(block) else {
        if witness_present {
            return Err(block_error(
                BlockValidationResult::Mutated,
                "unexpected-witness",
                Some("unexpected witness data found".to_string()),
            ));
        }

        return Ok(());
    };
    let coinbase_input = block
        .transactions
        .first()
        .and_then(|transaction| transaction.inputs.first())
        .expect("coinbase input must exist after context-free block checks");
    if coinbase_input.witness.stack().len() != WITNESS_RESERVED_VALUE_STACK_ITEMS
        || coinbase_input.witness.stack()[0].len() != WITNESS_RESERVED_VALUE_SIZE
    {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-witness-nonce-size",
            Some("invalid witness reserved value size".to_string()),
        ));
    }

    let witness_root = block_witness_merkle_root(block).map_err(map_codec_error)?;
    let reserved_value = &coinbase_input.witness.stack()[0];
    let mut commitment_preimage = [0_u8; WITNESS_COMMITMENT_PREIMAGE_SIZE];
    commitment_preimage[..WITNESS_RESERVED_VALUE_SIZE].copy_from_slice(witness_root.as_bytes());
    commitment_preimage[WITNESS_RESERVED_VALUE_SIZE..].copy_from_slice(reserved_value);
    let expected_commitment = double_sha256(&commitment_preimage);

    let commitment_script = block.transactions[0].outputs[commitment_index]
        .script_pubkey
        .as_bytes();
    if commitment_script[WITNESS_COMMITMENT_START..WITNESS_COMMITMENT_END] != expected_commitment {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-witness-merkle-match",
            Some("witness merkle commitment mismatch".to_string()),
        ));
    }

    Ok(())
}

fn witness_commitment_index(block: &Block) -> Option<usize> {
    let coinbase_outputs = block.transactions.first()?.outputs.iter().enumerate();
    let mut maybe_commitment_index = None;
    for (index, output) in coinbase_outputs {
        let bytes = output.script_pubkey.as_bytes();
        if has_witness_commitment_header(bytes) {
            maybe_commitment_index = Some(index);
        }
    }
    maybe_commitment_index
}

fn map_transaction_validation_error(
    transaction: &open_bitcoin_primitives::Transaction,
    error: TxValidationError,
) -> BlockValidationError {
    let txid = format!(
        "{:?}",
        transaction_txid(transaction)
            .expect("phase-2 typed transactions should serialize for txid logging")
            .to_byte_array()
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

fn has_witness_commitment_header(bytes: &[u8]) -> bool {
    bytes.len() >= WITNESS_COMMITMENT_SCRIPT_LEN && bytes.starts_with(&WITNESS_COMMITMENT_HEADER)
}

fn block_witness_merkle_root(
    block: &Block,
) -> Result<open_bitcoin_primitives::MerkleRoot, CodecError> {
    if block.transactions.is_empty() {
        return Ok(open_bitcoin_primitives::MerkleRoot::from_byte_array(
            [0_u8; 32],
        ));
    }

    let mut level = Vec::with_capacity(block.transactions.len());
    level.push([0_u8; 32]);
    for transaction in block.transactions.iter().skip(1) {
        level.push(transaction_wtxid(transaction)?.to_byte_array());
    }

    while level.len() > 1 {
        if level.len() % 2 == 1 {
            let last_hash = *level.last().expect("non-empty merkle level");
            level.push(last_hash);
        }

        let mut next_level = Vec::with_capacity(level.len() / 2);
        for pair in level.chunks_exact(2) {
            let mut concatenated = [0_u8; HASH_CONCATENATION_SIZE];
            concatenated[..WITNESS_RESERVED_VALUE_SIZE].copy_from_slice(&pair[0]);
            concatenated[WITNESS_RESERVED_VALUE_SIZE..].copy_from_slice(&pair[1]);
            next_level.push(double_sha256(&concatenated));
        }
        level = next_level;
    }

    Ok(open_bitcoin_primitives::MerkleRoot::from_byte_array(
        level[0],
    ))
}

fn coinbase_has_height_prefix(block: &Block, height: u32) -> bool {
    let Some(coinbase_input) = block
        .transactions
        .first()
        .and_then(|transaction| transaction.inputs.first())
    else {
        return false;
    };
    let expected_prefix = serialized_script_num(height as i64);
    let script_sig = coinbase_input.script_sig.as_bytes();

    script_sig.len() >= expected_prefix.len()
        && script_sig[..expected_prefix.len()] == expected_prefix
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
    } else if negative {
        let last = encoded.last_mut().expect("non-empty because value != 0");
        *last |= 0x80;
    }

    let mut script = Vec::with_capacity(encoded.len() + 1);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script
}

fn map_codec_error(error: CodecError) -> BlockValidationError {
    block_error(
        BlockValidationResult::Consensus,
        "bad-blk-serialization",
        Some(error.to_string()),
    )
}

fn map_script_error(error: crate::script::ScriptError) -> BlockValidationError {
    block_error(
        BlockValidationResult::Consensus,
        "bad-blk-script",
        Some(error.to_string()),
    )
}

fn block_sigop_overflow() -> BlockValidationError {
    block_error(
        BlockValidationResult::Consensus,
        "bad-blk-sigops",
        Some("out-of-bounds SigOpCount".to_string()),
    )
}

fn enforce_sigop_cost_limit(sigop_cost: usize) -> Result<(), BlockValidationError> {
    if sigop_cost > MAX_BLOCK_SIGOPS_COST {
        return Err(block_sigop_overflow());
    }
    Ok(())
}

#[cfg(test)]
mod tests;
