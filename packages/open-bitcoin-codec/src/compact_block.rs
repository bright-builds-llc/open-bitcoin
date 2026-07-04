// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py
// - packages/bitcoin-knots/test/functional/test_framework/messages.py

use crate::error::CodecError;
use open_bitcoin_primitives::{BlockHash, BlockHeader, Transaction};

use crate::block::{encode_block_header, parse_block_header_from_reader};
use crate::compact_size::{compact_size_to_usize, read_compact_size, write_compact_size};
use crate::primitives::{Reader, write_u64_le};
use crate::transaction::{TransactionEncoding, encode_transaction, parse_transaction_from_reader};

pub const BIP152_COMPACT_BLOCKS_VERSION: u64 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SendCompactMessage {
    pub announce: bool,
    pub version: u64,
}

pub fn encode_send_compact_payload(message: &SendCompactMessage) -> Vec<u8> {
    let mut out = Vec::with_capacity(9);
    out.push(u8::from(message.announce));
    write_u64_le(&mut out, message.version);
    out
}

pub fn decode_send_compact_payload(payload: &[u8]) -> Result<SendCompactMessage, CodecError> {
    let mut reader = Reader::new(payload);
    let announce = reader.read_u8()? != 0;
    let version = reader.read_u64_le()?;
    reader.finish()?;

    Ok(SendCompactMessage { announce, version })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortId([u8; 6]);

impl ShortId {
    pub const fn from_wire_bytes(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    pub const fn as_wire_bytes(&self) -> &[u8; 6] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefilledTransaction {
    pub index_delta: u64,
    pub transaction: Transaction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactBlockPayload {
    pub header: BlockHeader,
    pub nonce: u64,
    pub short_ids: Vec<ShortId>,
    pub prefilled_transactions: Vec<PrefilledTransaction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTransactionsRequest {
    pub block_hash: BlockHash,
    pub index_deltas: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTransactions {
    pub block_hash: BlockHash,
    pub transactions: Vec<Transaction>,
}

pub fn encode_compact_block_payload(payload: &CompactBlockPayload) -> Result<Vec<u8>, CodecError> {
    validate_compact_block_structure(payload)?;

    let mut out = encode_block_header(&payload.header);
    write_u64_le(&mut out, payload.nonce);
    write_compact_size(&mut out, payload.short_ids.len() as u64)?;
    for short_id in &payload.short_ids {
        out.extend_from_slice(short_id.as_wire_bytes());
    }
    write_compact_size(&mut out, payload.prefilled_transactions.len() as u64)?;
    for prefilled in &payload.prefilled_transactions {
        write_compact_size(&mut out, prefilled.index_delta)?;
        let encoded_transaction =
            encode_transaction(&prefilled.transaction, TransactionEncoding::WithWitness)?;
        out.extend_from_slice(&encoded_transaction);
    }

    Ok(out)
}

pub fn decode_compact_block_payload(bytes: &[u8]) -> Result<CompactBlockPayload, CodecError> {
    let mut reader = Reader::new(bytes);
    let header = parse_block_header_from_reader(&mut reader)?;
    let nonce = reader.read_u64_le()?;
    let short_id_count = read_bounded_compact_block_count(&mut reader, "short id count")?;
    let mut short_ids = Vec::with_capacity(short_id_count);
    for _ in 0..short_id_count {
        short_ids.push(ShortId::from_wire_bytes(reader.read_array::<6>()?));
    }

    let prefilled_transaction_count =
        read_bounded_compact_block_count(&mut reader, "prefilled transaction count")?;
    validate_compact_block_count(short_id_count as u64, prefilled_transaction_count as u64)?;
    let mut prefilled_transactions = Vec::with_capacity(prefilled_transaction_count);
    for _ in 0..prefilled_transaction_count {
        prefilled_transactions.push(PrefilledTransaction {
            index_delta: read_compact_size(&mut reader)?,
            transaction: parse_transaction_from_reader(&mut reader, true)?,
        });
    }
    reader.finish()?;

    let payload = CompactBlockPayload {
        header,
        nonce,
        short_ids,
        prefilled_transactions,
    };
    validate_compact_block_structure(&payload)?;
    Ok(payload)
}

pub fn encode_get_block_transactions_payload(
    request: &BlockTransactionsRequest,
) -> Result<Vec<u8>, CodecError> {
    expand_block_transaction_indexes(request)?;

    let mut out = Vec::with_capacity(32 + request.index_deltas.len());
    out.extend_from_slice(request.block_hash.as_bytes());
    write_compact_size(&mut out, request.index_deltas.len() as u64)?;
    for index_delta in &request.index_deltas {
        write_compact_size(&mut out, *index_delta)?;
    }

    Ok(out)
}

pub fn decode_get_block_transactions_payload(
    bytes: &[u8],
) -> Result<BlockTransactionsRequest, CodecError> {
    let mut reader = Reader::new(bytes);
    let block_hash = BlockHash::from_byte_array(reader.read_array::<32>()?);
    let index_count = compact_size_to_usize(
        read_compact_size(&mut reader)?,
        "block transaction index count",
    );
    let mut index_deltas = Vec::with_capacity(index_count);
    for _ in 0..index_count {
        index_deltas.push(read_compact_size(&mut reader)?);
    }
    reader.finish()?;

    let request = BlockTransactionsRequest {
        block_hash,
        index_deltas,
    };
    expand_block_transaction_indexes(&request)?;
    Ok(request)
}

pub fn encode_block_transactions_payload(
    response: &BlockTransactions,
) -> Result<Vec<u8>, CodecError> {
    let mut out = Vec::with_capacity(32 + response.transactions.len());
    out.extend_from_slice(response.block_hash.as_bytes());
    write_compact_size(&mut out, response.transactions.len() as u64)?;
    for transaction in &response.transactions {
        let encoded_transaction =
            encode_transaction(transaction, TransactionEncoding::WithWitness)?;
        out.extend_from_slice(&encoded_transaction);
    }

    Ok(out)
}

pub fn decode_block_transactions_payload(bytes: &[u8]) -> Result<BlockTransactions, CodecError> {
    let mut reader = Reader::new(bytes);
    let block_hash = BlockHash::from_byte_array(reader.read_array::<32>()?);
    let transaction_count =
        compact_size_to_usize(read_compact_size(&mut reader)?, "block transaction count");
    let mut transactions = Vec::with_capacity(transaction_count);
    for _ in 0..transaction_count {
        transactions.push(parse_transaction_from_reader(&mut reader, true)?);
    }
    reader.finish()?;

    Ok(BlockTransactions {
        block_hash,
        transactions,
    })
}

pub fn validate_compact_block_structure(payload: &CompactBlockPayload) -> Result<(), CodecError> {
    if payload.short_ids.is_empty() && payload.prefilled_transactions.is_empty() {
        return Err(CodecError::CompactBlockEmpty);
    }

    let transaction_count = validate_compact_block_count(
        payload.short_ids.len() as u64,
        payload.prefilled_transactions.len() as u64,
    )?;
    let positions = expand_prefilled_positions(payload)?;
    for (prefilled_index, position) in positions.iter().copied().enumerate() {
        let max_position = payload.short_ids.len() as u64 + prefilled_index as u64;
        let position = u64::from(position);
        if position > max_position {
            return Err(CodecError::PrefilledTransactionOutOfBounds {
                position,
                transaction_count,
            });
        }
    }

    for prefilled in &payload.prefilled_transactions {
        if prefilled.transaction.inputs.is_empty() || prefilled.transaction.outputs.is_empty() {
            return Err(CodecError::CompactBlockNullPrefilledTransaction);
        }
    }

    Ok(())
}

pub fn expand_prefilled_positions(payload: &CompactBlockPayload) -> Result<Vec<u16>, CodecError> {
    let deltas = payload
        .prefilled_transactions
        .iter()
        .map(|prefilled| prefilled.index_delta)
        .collect::<Vec<_>>();
    expand_differential_indexes(&deltas, "prefilled transaction index")
}

pub fn expand_block_transaction_indexes(
    request: &BlockTransactionsRequest,
) -> Result<Vec<u16>, CodecError> {
    expand_differential_indexes(&request.index_deltas, "block transaction index")
}

pub fn expand_differential_indexes(
    deltas: &[u64],
    _field: &'static str,
) -> Result<Vec<u16>, CodecError> {
    let mut shift = 0_u64;
    let mut indexes = Vec::with_capacity(deltas.len());
    for delta in deltas {
        shift = shift
            .checked_add(*delta)
            .ok_or(CodecError::DifferentialIndexOverflow)?;
        let index = u16::try_from(shift).map_err(|_| CodecError::DifferentialIndexOverflow)?;
        indexes.push(index);
        shift = shift
            .checked_add(1)
            .ok_or(CodecError::DifferentialIndexOverflow)?;
    }
    Ok(indexes)
}

fn read_bounded_compact_block_count(
    reader: &mut Reader<'_>,
    field: &'static str,
) -> Result<usize, CodecError> {
    let count = read_compact_size(reader)?;
    if count > u64::from(u16::MAX) {
        return Err(CodecError::CompactBlockTransactionCountOverflow { count });
    }
    Ok(compact_size_to_usize(count, field))
}

fn validate_compact_block_count(
    short_id_count: u64,
    prefilled_transaction_count: u64,
) -> Result<u64, CodecError> {
    let count = short_id_count
        .checked_add(prefilled_transaction_count)
        .ok_or(CodecError::CompactBlockTransactionCountOverflow { count: u64::MAX })?;
    if count > u64::from(u16::MAX) {
        return Err(CodecError::CompactBlockTransactionCountOverflow { count });
    }
    Ok(count)
}

#[cfg(test)]
mod tests;
