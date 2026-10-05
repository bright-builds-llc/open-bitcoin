// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Additive BlockIndex v1 envelopes. Integers are little endian; hashes are raw
//! bytes (never display endian). Record order: version/type, height, block,
//! parent, previous header, filter hash/header, u32 byte length, exact bytes.
//! State: version/type, endpoint flag [height/hash], fence height/hash,
//! protection flag [earliest height]. Projection: version/type, height/hash.

use super::{StorageError, StorageNamespace, StorageRecoveryAction};
use open_bitcoin_core::{
    chainstate::{
        BasicFilterInputs, ChainPosition, FilterRecordIdentity, IndexInputProtection,
        filter_index::{verify_filter_record_commitment, verify_filter_record_predecessor},
    },
    codec::{MAX_SIZE, block_filter::validation::validate_basic_filter_encoding},
    consensus::{FilterHeaderPredecessor, crypto::double_sha256},
    primitives::{BlockHash, FilterHash, FilterHeader},
};

pub(crate) const RECORD_PREFIX: &str = "basic_filter:v1:record:";
pub(crate) const ACTIVE_PREFIX: &str = "basic_filter:v1:active:";
pub(crate) const STATE_KEY: &str = "basic_filter:v1:state";
pub(crate) const RECORD_OVERHEAD: usize = 170;

pub(crate) mod ownership;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StoredFilterRecord {
    identity: FilterRecordIdentity,
    bytes: Vec<u8>,
}

/// Borrowed bounded envelope, deliberately not a verified ancestry identity.
pub(crate) struct ParsedFilterRecord<'a> {
    pub(crate) height: u32,
    pub(crate) block: BlockHash,
    pub(crate) parent: BlockHash,
    previous: FilterHeader,
    hash: FilterHash,
    header: FilterHeader,
    encoded: &'a [u8],
}

impl ParsedFilterRecord<'_> {
    pub(crate) fn verify_parent(&self, maybe_parent: Option<&Self>) -> Result<(), StorageError> {
        verify_filter_record_predecessor(
            self.height,
            self.parent,
            self.previous,
            maybe_parent.map(|parent| (parent.height, parent.block, parent.header)),
        )
        .map_err(index_corruption)
    }

    /// Construct a local immutable identity after checking its immediate edge.
    /// This bounded read is not a complete ancestry or release proof.
    pub(crate) fn checkpoint_identity(
        &self,
        maybe_parent: Option<&Self>,
    ) -> Result<FilterRecordIdentity, StorageError> {
        FilterRecordIdentity::new_with_predecessor_facts(
            self.height,
            self.block,
            self.parent,
            self.hash,
            self.header,
            self.previous,
            maybe_parent.map(|parent| (parent.height, parent.block, parent.header)),
        )
        .map_err(index_corruption)
    }
}

impl StoredFilterRecord {
    /// New writes require the historical-input generator and explicit predecessor.
    #[cfg_attr(not(test), allow(dead_code))] // Phase 157 owns activation/catch-up.
    pub(crate) fn generate(
        inputs: &BasicFilterInputs<'_>,
        position: &ChainPosition,
        maybe_predecessor: Option<&FilterRecordIdentity>,
    ) -> Result<Self, StorageError> {
        let predecessor = match maybe_predecessor {
            None => FilterHeaderPredecessor::Genesis,
            Some(previous) => FilterHeaderPredecessor::Previous {
                block_hash: previous.block_hash(),
                height: previous.height(),
                header: previous.filter_header(),
            },
        };
        let (filter, header) = inputs.generate(predecessor).map_err(index_corruption)?;
        if filter.block_hash() != position.block_hash {
            return Err(index_corruption("BASIC input position mismatch"));
        }
        let previous_header = maybe_predecessor
            .map(|p| p.filter_header())
            .unwrap_or_default();
        let identity = FilterRecordIdentity::new(
            position.height,
            position.block_hash,
            position.previous_block_hash(),
            filter.filter_hash(),
            header,
            previous_header,
            maybe_predecessor,
        )
        .map_err(index_corruption)?;
        Ok(Self {
            identity,
            bytes: filter.encoded_bytes().to_vec(),
        })
    }
    pub(crate) fn identity(&self) -> FilterRecordIdentity {
        self.identity
    }
    pub(crate) fn encoded_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StoredFilterState {
    pub(crate) maybe_endpoint: Option<(u32, BlockHash)>,
    pub(crate) fence_height: u32,
    pub(crate) fence_hash: BlockHash,
    pub(crate) protection: IndexInputProtection,
}

pub(crate) fn index_corruption(detail: impl core::fmt::Display) -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::BlockIndex,
        detail: detail.to_string(),
        action: StorageRecoveryAction::Repair,
    }
}

pub(crate) fn record_key(hash: BlockHash) -> String {
    let mut key = String::from(RECORD_PREFIX);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in hash.as_bytes() {
        key.push(HEX[(byte >> 4) as usize] as char);
        key.push(HEX[(byte & 15) as usize] as char);
    }
    key
}
pub(crate) fn active_key(height: u32) -> String {
    format!("{ACTIVE_PREFIX}{height:08x}")
}

pub(crate) fn encode_record(record: &StoredFilterRecord) -> Vec<u8> {
    let mut bytes = vec![1, 0];
    let id = record.identity;
    bytes.extend_from_slice(&id.height().to_le_bytes());
    for hash in [
        id.block_hash().as_bytes(),
        id.parent_hash().as_bytes(),
        id.previous_header().as_bytes(),
        id.filter_hash().as_bytes(),
        id.filter_header().as_bytes(),
    ] {
        bytes.extend_from_slice(hash);
    }
    bytes.extend_from_slice(&(record.bytes.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&record.bytes);
    bytes
}

/// Cheap envelope validation also supplies parent identity without hashing or allocation.
pub(crate) fn record_parent(key: &str, bytes: &[u8]) -> Result<(u32, BlockHash), StorageError> {
    let mut cursor = record_cursor(bytes)?;
    let height = cursor.u32()?;
    let hash = BlockHash::from_byte_array(cursor.array()?);
    if record_key(hash) != key {
        return Err(index_corruption("BASIC record key identity"));
    }
    let parent = BlockHash::from_byte_array(cursor.array()?);
    Ok((height, parent))
}

fn record_cursor(bytes: &[u8]) -> Result<Cursor<'_>, StorageError> {
    if bytes.len() > MAX_SIZE as usize + RECORD_OVERHEAD {
        return Err(index_corruption("BASIC record byte bound"));
    }
    let mut cursor = Cursor::new(bytes);
    cursor.version()?;
    Ok(cursor)
}

pub(crate) fn decode_record(
    key: &str,
    bytes: &[u8],
    maybe_predecessor: Option<&FilterRecordIdentity>,
) -> Result<StoredFilterRecord, StorageError> {
    let parsed = parse_record(key, bytes)?;
    let identity = FilterRecordIdentity::new(
        parsed.height,
        parsed.block,
        parsed.parent,
        parsed.hash,
        parsed.header,
        parsed.previous,
        maybe_predecessor,
    )
    .map_err(index_corruption)?;
    Ok(StoredFilterRecord {
        identity,
        bytes: parsed.encoded.to_vec(),
    })
}

/// Validate bounded encoding and own commitments without allocating filter bytes.
pub(crate) fn parse_record<'a>(
    key: &str,
    bytes: &'a [u8],
) -> Result<ParsedFilterRecord<'a>, StorageError> {
    let parsed = parse_record_fields(key, bytes)?;
    validate_basic_filter_encoding(parsed.encoded).map_err(index_corruption)?;
    if FilterHash::from_byte_array(double_sha256(parsed.encoded)) != parsed.hash {
        return Err(index_corruption("BASIC byte hash commitment"));
    }
    verify_filter_record_commitment(
        parsed.height,
        parsed.parent,
        parsed.hash,
        parsed.header,
        parsed.previous,
    )
    .map_err(index_corruption)?;
    Ok(parsed)
}

/// Cheap envelope bounds for parent/projection references during a
/// complete scan. Every row separately receives full byte/hash validation.
pub(crate) fn parse_record_fields<'a>(
    key: &str,
    bytes: &'a [u8],
) -> Result<ParsedFilterRecord<'a>, StorageError> {
    let mut cursor = record_cursor(bytes)?;
    let height = cursor.u32()?;
    let block = BlockHash::from_byte_array(cursor.array()?);
    let parent = BlockHash::from_byte_array(cursor.array()?);
    let previous = FilterHeader::from_byte_array(cursor.array()?);
    let hash = FilterHash::from_byte_array(cursor.array()?);
    let header = FilterHeader::from_byte_array(cursor.array()?);
    let len = cursor.u32()? as usize;
    if len > MAX_SIZE as usize {
        return Err(index_corruption("BASIC encoded length bound"));
    }
    let encoded = cursor.take(len)?;
    cursor.finish()?;
    if record_key(block) != key {
        return Err(index_corruption("BASIC record key identity"));
    }
    Ok(ParsedFilterRecord {
        height,
        block,
        parent,
        previous,
        hash,
        header,
        encoded,
    })
}

pub(crate) fn encode_state(state: StoredFilterState) -> Vec<u8> {
    let mut bytes = vec![1, 0];
    match state.maybe_endpoint {
        None => bytes.push(0),
        Some((height, hash)) => {
            bytes.push(1);
            bytes.extend_from_slice(&height.to_le_bytes());
            bytes.extend_from_slice(hash.as_bytes());
        }
    }
    bytes.extend_from_slice(&state.fence_height.to_le_bytes());
    bytes.extend_from_slice(state.fence_hash.as_bytes());
    match state.protection {
        IndexInputProtection::FromHeight(height) => {
            bytes.push(0);
            bytes.extend_from_slice(&height.to_le_bytes());
        }
        IndexInputProtection::HeightSpaceExhausted => bytes.push(1),
    }
    bytes
}

pub(crate) fn decode_state(bytes: &[u8]) -> Result<StoredFilterState, StorageError> {
    let mut cursor = Cursor::new(bytes);
    cursor.version()?;
    let maybe_endpoint = match cursor.byte()? {
        0 => None,
        1 => Some((cursor.u32()?, BlockHash::from_byte_array(cursor.array()?))),
        _ => return Err(index_corruption("BASIC state endpoint flag")),
    };
    let fence_height = cursor.u32()?;
    let fence_hash = BlockHash::from_byte_array(cursor.array()?);
    let protection = match cursor.byte()? {
        0 => IndexInputProtection::FromHeight(cursor.u32()?),
        1 => IndexInputProtection::HeightSpaceExhausted,
        _ => return Err(index_corruption("BASIC state protection flag")),
    };
    cursor.finish()?;
    Ok(StoredFilterState {
        maybe_endpoint,
        fence_height,
        fence_hash,
        protection,
    })
}

pub(crate) fn encode_projection(height: u32, hash: BlockHash) -> Vec<u8> {
    let mut bytes = vec![1, 0];
    bytes.extend_from_slice(&height.to_le_bytes());
    bytes.extend_from_slice(hash.as_bytes());
    bytes
}
pub(crate) fn decode_projection(key: &str, bytes: &[u8]) -> Result<BlockHash, StorageError> {
    let mut cursor = Cursor::new(bytes);
    cursor.version()?;
    let height = cursor.u32()?;
    let hash = BlockHash::from_byte_array(cursor.array()?);
    cursor.finish()?;
    if active_key(height) != key {
        return Err(index_corruption("BASIC projection key height"));
    }
    Ok(hash)
}

struct Cursor<'a> {
    rest: &'a [u8],
}
impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { rest: bytes }
    }
    fn take(&mut self, len: usize) -> Result<&'a [u8], StorageError> {
        if self.rest.len() < len {
            return Err(index_corruption("truncated BASIC envelope"));
        }
        let (head, tail) = self.rest.split_at(len);
        self.rest = tail;
        Ok(head)
    }
    fn array<const N: usize>(&mut self) -> Result<[u8; N], StorageError> {
        self.take(N)?
            .try_into()
            .map_err(|_| index_corruption("truncated BASIC field"))
    }
    fn byte(&mut self) -> Result<u8, StorageError> {
        Ok(self.take(1)?[0])
    }
    fn u32(&mut self) -> Result<u32, StorageError> {
        Ok(u32::from_le_bytes(self.array()?))
    }
    fn version(&mut self) -> Result<(), StorageError> {
        if self.byte()? != 1 {
            return Err(index_corruption("unknown BASIC envelope version"));
        }
        if self.byte()? != 0 {
            return Err(index_corruption("unknown BASIC filter type"));
        }
        Ok(())
    }
    fn finish(self) -> Result<(), StorageError> {
        if !self.rest.is_empty() {
            return Err(index_corruption("trailing BASIC envelope bytes"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
