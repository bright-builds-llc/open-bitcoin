// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/blockfilter.h
// - packages/bitcoin-knots/src/crypto/siphash.cpp

//! Pure BASIC construction from explicit output and historical spent-script facts.

use core::fmt;
use std::collections::BTreeSet;

use open_bitcoin_codec::{
    BasicFilterEncodingError, basic_filter_range, encode_basic_filter_values,
};
use open_bitcoin_primitives::{BlockHash, FilterHash, FilterHeader};

use crate::crypto::{double_sha256, siphash_bytes};

/// Canonical BASIC bytes bound to the block whose raw hash supplies SipHash keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicFilter {
    block_hash: BlockHash,
    encoded: Vec<u8>,
    hash: FilterHash,
}

impl BasicFilter {
    /// Construct from low-level facts; this API does not prove historical completeness.
    ///
    /// Outputs exclude empty and first-byte OP_RETURN scripts. Historical spent
    /// scripts exclude only empties. Raw scripts are deduplicated before mapping;
    /// distinct raw scripts that map to the same value remain distinct elements.
    pub fn from_script_facts(
        block_hash: BlockHash,
        output_scripts: &[&[u8]],
        spent_scripts: &[&[u8]],
    ) -> Result<Self, BasicFilterEncodingError> {
        let mut elements = BTreeSet::new();
        for script in output_scripts
            .iter()
            .copied()
            .filter(|script| !script.is_empty() && script[0] != 0x6a)
            .chain(
                spent_scripts
                    .iter()
                    .copied()
                    .filter(|script| !script.is_empty()),
            )
        {
            elements.insert(script);
            basic_filter_range(elements.len() as u64)?;
        }
        let range = basic_filter_range(elements.len() as u64)?;
        let raw = block_hash.as_bytes();
        let mut word = [0_u8; 8];
        word.copy_from_slice(&raw[..8]);
        let k0 = u64::from_le_bytes(word);
        word.copy_from_slice(&raw[8..16]);
        let k1 = u64::from_le_bytes(word);
        let mut values = reserve_mapped_values(elements.len())?;
        for script in elements {
            let hashed = siphash_bytes(k0, k1, script);
            values.push(((u128::from(hashed) * u128::from(range)) >> 64) as u64);
        }
        values.sort_unstable();
        let encoded = encode_basic_filter_values(&values)?;
        let hash = FilterHash::from_byte_array(double_sha256(&encoded));
        Ok(Self {
            block_hash,
            encoded,
            hash,
        })
    }

    /// Return the raw block identity used to derive this filter's keys.
    pub fn block_hash(&self) -> BlockHash {
        self.block_hash
    }

    /// Borrow the canonical count-prefixed BASIC bytes.
    pub fn encoded_bytes(&self) -> &[u8] {
        &self.encoded
    }

    /// Return SHA256d of the canonical encoded bytes, in raw protocol order.
    pub fn filter_hash(&self) -> FilterHash {
        self.hash
    }

    /// Commit with explicit validated height/parent facts and predecessor identity.
    /// The caller supplies validated chain position; this pure API checks correspondence.
    pub fn header(
        &self,
        parent_hash: BlockHash,
        height: u32,
        predecessor: FilterHeaderPredecessor,
    ) -> Result<FilterHeader, FilterHeaderError> {
        let previous = match predecessor {
            FilterHeaderPredecessor::Genesis => {
                if height != 0 {
                    return Err(FilterHeaderError::GenesisHeight);
                }
                if parent_hash != BlockHash::default() {
                    return Err(FilterHeaderError::GenesisParent);
                }
                FilterHeader::default()
            }
            FilterHeaderPredecessor::Previous {
                block_hash,
                height: previous_height,
                header,
            } => {
                if height == 0 {
                    return Err(FilterHeaderError::PreviousForGenesis);
                }
                if block_hash != parent_hash {
                    return Err(FilterHeaderError::PreviousHash);
                }
                if previous_height != height - 1 {
                    return Err(FilterHeaderError::PreviousHeight);
                }
                header
            }
        };
        Ok(compute_filter_header(self.hash, previous))
    }
}

fn reserve_mapped_values(count: usize) -> Result<Vec<u64>, BasicFilterEncodingError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| BasicFilterEncodingError::AllocationFailed)?;
    Ok(values)
}

/// Explicit genesis or previous branch commitment; omission cannot initialize a suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterHeaderPredecessor {
    Genesis,
    Previous {
        block_hash: BlockHash,
        height: u32,
        header: FilterHeader,
    },
}

/// Identity or height mismatch while constructing a contextual filter header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterHeaderError {
    GenesisHeight,
    GenesisParent,
    PreviousForGenesis,
    PreviousHash,
    PreviousHeight,
}

impl fmt::Display for FilterHeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::GenesisHeight => "genesis predecessor requires height zero",
            Self::GenesisParent => "genesis predecessor requires a null parent",
            Self::PreviousForGenesis => "height zero requires a genesis predecessor",
            Self::PreviousHash => "predecessor block hash does not match parent",
            Self::PreviousHeight => "predecessor height is not the preceding height",
        })
    }
}

impl std::error::Error for FilterHeaderError {}

/// Compose raw commitments without claiming validated ancestry (e.g. sparse vectors).
pub fn compute_filter_header(hash: FilterHash, previous: FilterHeader) -> FilterHeader {
    let mut bytes = [0_u8; 64];
    bytes[..32].copy_from_slice(hash.as_bytes());
    bytes[32..].copy_from_slice(previous.as_bytes());
    FilterHeader::from_byte_array(double_sha256(&bytes))
}

/// Format a filter hash in Bitcoin display order, leaving its raw bytes intact.
pub fn filter_hash_display_hex(hash: FilterHash) -> String {
    display_hex(hash.as_bytes())
}

/// Format a filter header in Bitcoin display order, leaving its raw bytes intact.
pub fn filter_header_display_hex(header: FilterHeader) -> String {
    display_hex(header.as_bytes())
}

fn display_hex(raw: &[u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(64);
    for byte in raw.iter().rev() {
        result.push(char::from(HEX[usize::from(byte >> 4)]));
        result.push(char::from(HEX[usize::from(byte & 15)]));
    }
    result
}

#[cfg(test)]
mod tests;
