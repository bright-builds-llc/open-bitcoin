// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/merkle.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::{consensus_error, enforce_sigop_cost_limit, map_codec_error, sigops};
use crate::crypto::block_merkle_root_view;
use crate::transaction::intrinsic::{check_transaction_view, is_coinbase};
use crate::validation::{BlockValidationError, BlockValidationResult, block_error};
use crate::{ConsensusParams, MAX_BLOCK_WEIGHT, WITNESS_SCALE_FACTOR};
use open_bitcoin_codec::block::wire::{BlockView, WireBlock};
use open_bitcoin_codec::transaction::encode_transaction_view;
use open_bitcoin_codec::{CodecError, TransactionEncoding, encode_block_header};

/// Pure intrinsic checks over complete raw bytes, never a canonical admission seal.
/// Callers must hold selected-network authority; Signet challenge validation is separate.
pub fn check_submission_wire_block(
    block: &WireBlock,
    params: &ConsensusParams,
) -> Result<(), BlockValidationError> {
    super::check_block_header_with_params(&block.header, params)?;
    check_block_view(block)
}

pub(super) fn check_block_view(block: &impl BlockView) -> Result<(), BlockValidationError> {
    let (root, mutated) = block_merkle_root_view(block.transactions()).map_err(map_codec_error)?;
    if block.header().merkle_root != root {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-txnmrklroot",
            Some("hashMerkleRoot mismatch".to_string()),
        ));
    }
    if mutated {
        return Err(block_error(
            BlockValidationResult::Mutated,
            "bad-txns-duplicate",
            Some("duplicate transaction".to_string()),
        ));
    }
    if block.transactions().is_empty() {
        return Err(length_error());
    }
    let stripped = serialized_block_size(block, false).map_err(map_codec_error)?;
    if block
        .transactions()
        .len()
        .saturating_mul(WITNESS_SCALE_FACTOR)
        > MAX_BLOCK_WEIGHT
        || stripped.saturating_mul(WITNESS_SCALE_FACTOR) > MAX_BLOCK_WEIGHT
    {
        return Err(length_error());
    }
    if !is_coinbase(&block.transactions()[0]) {
        return Err(consensus_error(
            "bad-cb-missing",
            Some("first tx is not coinbase".to_string()),
        ));
    }
    if block.transactions().iter().skip(1).any(is_coinbase) {
        return Err(consensus_error(
            "bad-cb-multiple",
            Some("more than one coinbase".to_string()),
        ));
    }
    for transaction in block.transactions() {
        check_transaction_view(transaction)
            .map_err(|error| consensus_error(error.reject_reason, error.debug_message))?;
    }
    enforce_sigop_cost_limit(sigops::legacy_block_sigop_cost(block)?)?;
    Ok(())
}
fn length_error() -> BlockValidationError {
    consensus_error("bad-blk-length", Some("size limits failed".to_string()))
}

pub(super) fn serialized_block_size(
    block: &impl BlockView,
    include_witness: bool,
) -> Result<usize, CodecError> {
    let mut size = encode_block_header(block.header()).len();
    size += super::compact_size_len(block.transactions().len() as u64);
    let encoding = if include_witness {
        TransactionEncoding::WithWitness
    } else {
        TransactionEncoding::WithoutWitness
    };
    for transaction in block.transactions() {
        size += encode_transaction_view(transaction, encoding)?.len();
    }
    Ok(size)
}

#[cfg(test)]
mod tests;
