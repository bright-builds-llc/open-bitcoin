// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/merkle.cpp

use open_bitcoin_codec::CodecError;
use open_bitcoin_codec::block::wire::BlockView;
use open_bitcoin_codec::transaction::{TransactionEncoding, encode_transaction_view};
use open_bitcoin_primitives::MerkleRoot;

use super::witness::{WitnessMalleationError, check_witness_malleation};
use crate::crypto::{block_merkle_root_view, double_sha256, transaction_wtxid_view};
use crate::transaction::intrinsic::is_coinbase;

pub(super) fn is_block_mutated(
    block: &impl BlockView,
    check_witness_root: bool,
) -> Result<bool, CodecError> {
    let (root, mutated) = block_merkle_root_view(block.transactions())?;
    if root != block.header().merkle_root || mutated {
        return Ok(true);
    }
    if block
        .transactions()
        .first()
        .is_none_or(|transaction| !is_coinbase(transaction))
    {
        for transaction in block.transactions() {
            if encode_transaction_view(transaction, TransactionEncoding::WithoutWitness)?.len()
                == 64
            {
                return Ok(true);
            }
        }
        return Ok(false);
    }
    match check_witness_malleation(block, check_witness_root) {
        Ok(()) => Ok(false),
        Err(WitnessMalleationError::Invalid(_)) => Ok(true),
        Err(WitnessMalleationError::Encoding(error)) => Err(error),
    }
}

pub(super) fn witness_merkle_root(block: &impl BlockView) -> Result<MerkleRoot, CodecError> {
    const HASH_SIZE: usize = 32;
    const HASH_CONCATENATION_SIZE: usize = 64;
    if block.transactions().is_empty() {
        return Ok(MerkleRoot::from_byte_array([0_u8; HASH_SIZE]));
    }
    let mut level = Vec::with_capacity(block.transactions().len());
    level.push([0_u8; HASH_SIZE]);
    for transaction in block.transactions().iter().skip(1) {
        level.push(transaction_wtxid_view(transaction)?.to_byte_array());
    }
    while level.len() > 1 {
        if level.len() % 2 == 1
            && let Some(last_hash) = level.last().copied()
        {
            level.push(last_hash);
        }
        let mut next_level = Vec::with_capacity(level.len() / 2);
        for pair in level.chunks_exact(2) {
            let mut concatenated = [0_u8; HASH_CONCATENATION_SIZE];
            concatenated[..HASH_SIZE].copy_from_slice(&pair[0]);
            concatenated[HASH_SIZE..].copy_from_slice(&pair[1]);
            next_level.push(double_sha256(&concatenated));
        }
        level = next_level;
    }
    Ok(MerkleRoot::from_byte_array(
        level.first().copied().unwrap_or([0_u8; HASH_SIZE]),
    ))
}

#[cfg(test)]
mod tests;
