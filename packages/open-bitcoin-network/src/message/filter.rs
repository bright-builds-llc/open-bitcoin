// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/protocol.cpp
// - packages/bitcoin-knots/test/functional/test_framework/messages.py

use open_bitcoin_codec::{CodecError, write_compact_size};
use open_bitcoin_primitives::{BlockHash, Hash32};

use super::cursor::Cursor;
use crate::NetworkError;

/// Pinned maximum complete CF message payload, excluding the 24-byte frame header.
pub const MAX_CF_PAYLOAD_BYTES: usize = 4_000_000;

/// The three CF handlers whose extraction failures are nonfatal after frame validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CFRequestKind {
    Filters,
    Headers,
    Checkpoints,
}

impl CFRequestKind {
    pub(super) const fn command_name(self) -> &'static str {
        match self {
            Self::Filters => "getcfilters",
            Self::Headers => "getcfheaders",
            Self::Checkpoints => "getcfcheckpt",
        }
    }

    pub(super) const fn payload_len(self) -> usize {
        match self {
            Self::Filters | Self::Headers => 37,
            Self::Checkpoints => 33,
        }
    }
}

/// Raw ranged filter request; filter type authorization belongs to serving policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCFilters {
    pub filter_type: u8,
    pub start_height: u32,
    pub stop_hash: BlockHash,
}

/// Raw ranged filter-hash request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCFHeaders {
    pub filter_type: u8,
    pub start_height: u32,
    pub stop_hash: BlockHash,
}

/// Raw filter checkpoint request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetCFCheckpt {
    pub filter_type: u8,
    pub stop_hash: BlockHash,
}

/// Persisted serialized filter bytes for one block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CFilter {
    pub filter_type: u8,
    pub block_hash: BlockHash,
    pub filter: Vec<u8>,
}

/// Filter hashes with the header immediately preceding the requested range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CFHeaders {
    pub filter_type: u8,
    pub stop_hash: BlockHash,
    pub previous_filter_header: Hash32,
    pub filter_hashes: Vec<Hash32>,
}

/// Filter headers at the pinned checkpoint interval.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CFCheckpt {
    pub filter_type: u8,
    pub stop_hash: BlockHash,
    pub filter_headers: Vec<Hash32>,
}

pub(super) fn encode_ranged(filter_type: u8, start_height: u32, stop_hash: BlockHash) -> Vec<u8> {
    let mut payload = Vec::with_capacity(37);
    payload.push(filter_type);
    payload.extend_from_slice(&start_height.to_le_bytes());
    payload.extend_from_slice(stop_hash.as_bytes());
    payload
}

pub(super) fn encode_checkpoint(request: &GetCFCheckpt) -> Vec<u8> {
    let mut payload = Vec::with_capacity(33);
    payload.push(request.filter_type);
    payload.extend_from_slice(request.stop_hash.as_bytes());
    payload
}

pub(super) fn decode_ranged(payload: &[u8]) -> Result<GetCFilters, NetworkError> {
    let mut cursor = Cursor::new(payload);
    let request = GetCFilters {
        filter_type: cursor.read_u8()?,
        start_height: cursor.read_u32_le()?,
        stop_hash: BlockHash::from_byte_array(cursor.read_array()?),
    };
    // The pinned request handlers extract these fields without requiring stream exhaustion.
    Ok(request)
}

pub(super) fn decode_checkpoint(payload: &[u8]) -> Result<GetCFCheckpt, NetworkError> {
    let mut cursor = Cursor::new(payload);
    let request = GetCFCheckpt {
        filter_type: cursor.read_u8()?,
        stop_hash: BlockHash::from_byte_array(cursor.read_array()?),
    };
    // Only requests accept handler trailing bytes; response parsers remain strict.
    Ok(request)
}

impl CFilter {
    pub(super) fn encode(&self) -> Result<Vec<u8>, NetworkError> {
        let len = response_len(33, self.filter.len(), 1)?;
        let mut payload = Vec::with_capacity(len);
        payload.push(self.filter_type);
        payload.extend_from_slice(self.block_hash.as_bytes());
        write_compact_size(&mut payload, self.filter.len() as u64)?;
        payload.extend_from_slice(&self.filter);
        Ok(payload)
    }

    pub(super) fn decode(payload: &[u8]) -> Result<Self, NetworkError> {
        validate_payload_len(payload.len())?;
        let mut cursor = Cursor::new(payload);
        let filter_type = cursor.read_u8()?;
        let block_hash = BlockHash::from_byte_array(cursor.read_array()?);
        let count = read_count(&mut cursor, 33, 1)?;
        let filter = cursor.read_slice(count)?.to_vec();
        cursor.finish()?;
        Ok(Self {
            filter_type,
            block_hash,
            filter,
        })
    }
}

impl CFHeaders {
    pub(super) fn encode(&self) -> Result<Vec<u8>, NetworkError> {
        let len = response_len(65, self.filter_hashes.len(), 32)?;
        let mut payload = Vec::with_capacity(len);
        payload.push(self.filter_type);
        payload.extend_from_slice(self.stop_hash.as_bytes());
        payload.extend_from_slice(self.previous_filter_header.as_bytes());
        encode_hashes(&mut payload, &self.filter_hashes)?;
        Ok(payload)
    }

    pub(super) fn decode(payload: &[u8]) -> Result<Self, NetworkError> {
        validate_payload_len(payload.len())?;
        let mut cursor = Cursor::new(payload);
        let filter_type = cursor.read_u8()?;
        let stop_hash = BlockHash::from_byte_array(cursor.read_array()?);
        let previous_filter_header = Hash32::from_byte_array(cursor.read_array()?);
        let filter_hashes = decode_hashes(&mut cursor, 65)?;
        cursor.finish()?;
        Ok(Self {
            filter_type,
            stop_hash,
            previous_filter_header,
            filter_hashes,
        })
    }
}

impl CFCheckpt {
    pub(super) fn encode(&self) -> Result<Vec<u8>, NetworkError> {
        let len = response_len(33, self.filter_headers.len(), 32)?;
        let mut payload = Vec::with_capacity(len);
        payload.push(self.filter_type);
        payload.extend_from_slice(self.stop_hash.as_bytes());
        encode_hashes(&mut payload, &self.filter_headers)?;
        Ok(payload)
    }

    pub(super) fn decode(payload: &[u8]) -> Result<Self, NetworkError> {
        validate_payload_len(payload.len())?;
        let mut cursor = Cursor::new(payload);
        let filter_type = cursor.read_u8()?;
        let stop_hash = BlockHash::from_byte_array(cursor.read_array()?);
        let filter_headers = decode_hashes(&mut cursor, 33)?;
        cursor.finish()?;
        Ok(Self {
            filter_type,
            stop_hash,
            filter_headers,
        })
    }
}

fn encode_hashes(payload: &mut Vec<u8>, hashes: &[Hash32]) -> Result<(), NetworkError> {
    write_compact_size(payload, hashes.len() as u64)?;
    for hash in hashes {
        payload.extend_from_slice(hash.as_bytes());
    }
    Ok(())
}

fn decode_hashes(cursor: &mut Cursor<'_>, fixed_len: usize) -> Result<Vec<Hash32>, NetworkError> {
    let count = read_count(cursor, fixed_len, 32)?;
    let mut hashes = Vec::with_capacity(count);
    for _ in 0..count {
        hashes.push(Hash32::from_byte_array(cursor.read_array()?));
    }
    Ok(hashes)
}

fn read_count(
    cursor: &mut Cursor<'_>,
    fixed_len: usize,
    element_len: usize,
) -> Result<usize, NetworkError> {
    let raw_count = cursor.read_compact_size()?;
    let count = usize::try_from(raw_count).map_err(|_| length_error(raw_count))?;
    response_len(fixed_len, count, element_len)?;
    let required = count
        .checked_mul(element_len)
        .ok_or_else(|| length_error(raw_count))?;
    if required > cursor.remaining() {
        return Err(CodecError::UnexpectedEof {
            needed: required,
            remaining: cursor.remaining(),
        }
        .into());
    }
    Ok(count)
}

fn response_len(fixed_len: usize, count: usize, element_len: usize) -> Result<usize, NetworkError> {
    let compact_len = match count {
        0..=252 => 1,
        253..=65_535 => 3,
        65_536..=4_294_967_295 => 5,
        _ => 9,
    };
    let len = count
        .checked_mul(element_len)
        .and_then(|len| len.checked_add(fixed_len))
        .and_then(|len| len.checked_add(compact_len))
        .ok_or_else(|| length_error(count as u64))?;
    validate_payload_len(len)?;
    Ok(len)
}

fn validate_payload_len(len: usize) -> Result<(), NetworkError> {
    if len > MAX_CF_PAYLOAD_BYTES {
        return Err(length_error(len as u64));
    }
    Ok(())
}

fn length_error(value: u64) -> NetworkError {
    CodecError::LengthOutOfRange {
        field: "compact filter payload",
        value,
    }
    .into()
}

#[cfg(test)]
mod tests;
