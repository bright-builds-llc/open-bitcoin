// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

//! Pure Block → BIP152 `CompactBlockPayload` construction for outbound announce.

use open_bitcoin_codec::{
    CodecError, CompactBlockPayload, PrefilledTransaction, validate_compact_block_structure,
};
use open_bitcoin_primitives::Block;

use crate::crypto::{compact_short_id_for_wtxid, compact_short_id_selector, transaction_wtxid};

#[cfg(test)]
mod tests;

/// Build a Knots-shaped compact-block announce payload for `block`.
///
/// Prefills only the coinbase at absolute index 0 and short-IDs remaining
/// transactions by wtxid. Returns [`CodecError::CompactBlockEmpty`] when
/// `block.transactions` is empty.
pub fn build_compact_block_payload(
    block: &Block,
    nonce: u64,
) -> Result<CompactBlockPayload, CodecError> {
    let Some(coinbase) = block.transactions.first() else {
        return Err(CodecError::CompactBlockEmpty);
    };

    let selector = compact_short_id_selector(&block.header, nonce);
    let mut short_ids = Vec::with_capacity(block.transactions.len().saturating_sub(1));
    for transaction in block.transactions.iter().skip(1) {
        let wtxid = transaction_wtxid(transaction)?;
        short_ids.push(compact_short_id_for_wtxid(selector, &wtxid));
    }

    let payload = CompactBlockPayload {
        header: block.header.clone(),
        nonce,
        short_ids,
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase.clone(),
        }],
    };
    validate_compact_block_structure(&payload)?;
    Ok(payload)
}
