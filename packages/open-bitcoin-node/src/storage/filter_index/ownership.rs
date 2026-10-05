// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Additive owner envelope; the existing record/state/projection v1 bytes stay fixed.

use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};

use super::{Cursor, StorageError, index_corruption};

pub(crate) const OWNER_KEY: &str = "basic_filter:v1:owner";
const OWNER_LENGTH: usize = 11;

/// Encode version 1, BASIC type 0, explicit mode, then little-endian generation.
pub(crate) fn encode_owner(owner: IndexLifecycle) -> [u8; OWNER_LENGTH] {
    let mut bytes = [0; OWNER_LENGTH];
    bytes[0] = 1;
    bytes[2] = match owner {
        IndexLifecycle::Active { .. } => 0,
        IndexLifecycle::Disabled { .. } => 1,
    };
    bytes[3..].copy_from_slice(&owner.generation().value().to_le_bytes());
    bytes
}

/// Decode exact bounded bytes; a malformed owner is never interpreted as absence.
pub(crate) fn decode_owner(bytes: &[u8]) -> Result<IndexLifecycle, StorageError> {
    if bytes.len() != OWNER_LENGTH {
        return Err(index_corruption("BASIC owner envelope length"));
    }
    let mut cursor = Cursor::new(bytes);
    cursor.version()?;
    let mode = cursor.byte()?;
    let generation = IndexGeneration::new(u64::from_le_bytes(cursor.array()?));
    cursor.finish()?;
    match mode {
        0 => Ok(IndexLifecycle::Active { generation }),
        1 => Ok(IndexLifecycle::Disabled { generation }),
        _ => Err(index_corruption("unknown BASIC owner mode")),
    }
}
