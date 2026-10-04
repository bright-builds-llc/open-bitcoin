// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Immutable BASIC identities and conservative durable resume authority.

use core::fmt;
use std::collections::HashSet;

use open_bitcoin_consensus::{block_hash, compute_filter_header};
use open_bitcoin_primitives::{BlockHash, FilterHash, FilterHeader};

use crate::{ChainPosition, PRUNE_LOCK_BUFFER, PruneLockInfo};

pub mod recovery;
pub use recovery::{FilterRecoveryPlan, FilterRecoveryScan};

/// Reserved internal owner; public lock CRUD is outside this module's scope.
pub const BASIC_INDEX_PRUNE_LOCK: &str = "__open_bitcoin_basic_index";

/// Verify an envelope's own commitments without claiming verified ancestry.
/// Adapters must separately verify every direct predecessor in a complete scan.
pub fn verify_filter_record_commitment(
    height: u32,
    parent_hash: BlockHash,
    filter_hash: FilterHash,
    filter_header: FilterHeader,
    previous_header: FilterHeader,
) -> Result<(), FilterIndexError> {
    if height == 0
        && (parent_hash != BlockHash::default() || previous_header != FilterHeader::default())
    {
        return Err(FilterIndexError::Genesis);
    }
    if compute_filter_header(filter_hash, previous_header) != filter_header {
        return Err(FilterIndexError::HeaderCommitment);
    }
    Ok(())
}

/// Verify one decreasing-height edge without constructing an ancestry identity.
/// Complete row scans prove every edge; these raw facts cannot mint an identity.
pub fn verify_filter_record_predecessor(
    height: u32,
    parent_hash: BlockHash,
    previous_header: FilterHeader,
    maybe_predecessor: Option<(u32, BlockHash, FilterHeader)>,
) -> Result<(), FilterIndexError> {
    if height == 0 {
        if parent_hash != BlockHash::default()
            || previous_header != FilterHeader::default()
            || maybe_predecessor.is_some()
        {
            return Err(FilterIndexError::Genesis);
        }
        return Ok(());
    }
    let Some((previous_height, previous_hash, header)) = maybe_predecessor else {
        return Err(FilterIndexError::Predecessor);
    };
    if previous_height != height - 1 || previous_hash != parent_hash || header != previous_header {
        return Err(FilterIndexError::Predecessor);
    }
    Ok(())
}

/// Immutable commitments checked against the preceding immutable identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilterRecordIdentity {
    height: u32,
    block_hash: BlockHash,
    parent_hash: BlockHash,
    filter_hash: FilterHash,
    filter_header: FilterHeader,
    previous_header: FilterHeader,
}

impl FilterRecordIdentity {
    /// Parse contextual commitments; the adapter separately proves byte hash equality.
    pub fn new(
        height: u32,
        block_hash: BlockHash,
        parent_hash: BlockHash,
        filter_hash: FilterHash,
        filter_header: FilterHeader,
        previous_header: FilterHeader,
        maybe_predecessor: Option<&Self>,
    ) -> Result<Self, FilterIndexError> {
        verify_filter_record_predecessor(
            height,
            parent_hash,
            previous_header,
            maybe_predecessor
                .map(|previous| (previous.height, previous.block_hash, previous.filter_header)),
        )?;
        verify_filter_record_commitment(
            height,
            parent_hash,
            filter_hash,
            filter_header,
            previous_header,
        )?;
        Ok(Self {
            height,
            block_hash,
            parent_hash,
            filter_hash,
            filter_header,
            previous_header,
        })
    }

    /// Return the immutable block height.
    pub fn height(&self) -> u32 {
        self.height
    }
    /// Return the immutable block identity.
    pub fn block_hash(&self) -> BlockHash {
        self.block_hash
    }
    /// Return the immutable branch parent.
    pub fn parent_hash(&self) -> BlockHash {
        self.parent_hash
    }
    /// Return the committed encoded-byte hash.
    pub fn filter_hash(&self) -> FilterHash {
        self.filter_hash
    }
    /// Return the contextual filter header.
    pub fn filter_header(&self) -> FilterHeader {
        self.filter_header
    }
    /// Return the predecessor's filter header.
    pub fn previous_header(&self) -> FilterHeader {
        self.previous_header
    }
}

/// An absent prefix differs from a committed genesis filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexPrefix {
    Empty,
    Committed(FilterRecordIdentity),
}

impl IndexPrefix {
    /// Derive required input without height wraparound.
    pub fn input_protection(self) -> IndexInputProtection {
        match self {
            Self::Empty => IndexInputProtection::FromHeight(0),
            Self::Committed(identity) => match identity.height.checked_add(1) {
                Some(next) => IndexInputProtection::FromHeight(next),
                None => IndexInputProtection::HeightSpaceExhausted,
            },
        }
    }
}

/// Saved cursor assertion; recovery must prove its entire projection before use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilterCheckpoint {
    prefix: IndexPrefix,
}

impl FilterCheckpoint {
    /// Bind the persisted state to an explicit empty or committed identity.
    pub fn new(prefix: IndexPrefix) -> Self {
        Self { prefix }
    }
    /// Return the claimed immutable endpoint.
    pub fn prefix(self) -> IndexPrefix {
        self.prefix
    }
    /// Return the inputs required by this checkpoint.
    pub fn input_protection(self) -> IndexInputProtection {
        self.prefix.input_protection()
    }
}

/// Recovered coins plus full compatible durable active ancestry, borrowed in place.
#[derive(Debug, PartialEq, Eq)]
pub struct VerifiedChainstateFence<'a> {
    positions: &'a [ChainPosition],
}

impl<'a> VerifiedChainstateFence<'a> {
    /// Prove metadata ancestry and require its tip to equal recovered coins B.
    pub fn new(
        maybe_coins_best_block: Option<BlockHash>,
        maybe_positions: Option<&'a [ChainPosition]>,
    ) -> Result<Self, FilterIndexError> {
        let Some(coins_best_block) = maybe_coins_best_block else {
            return Err(FilterIndexError::MissingCoins);
        };
        let Some(positions) = maybe_positions.filter(|positions| !positions.is_empty()) else {
            return Err(FilterIndexError::MissingMetadata);
        };
        let mut identities = HashSet::new();
        let mut parent = BlockHash::default();
        for (height, position) in positions.iter().enumerate() {
            if position.height as usize != height
                || block_hash(&position.header) != position.block_hash
                || position.previous_block_hash() != parent
                || !identities.insert(position.block_hash)
            {
                return Err(FilterIndexError::MetadataAncestry);
            }
            parent = position.block_hash;
        }
        if parent != coins_best_block {
            return Err(FilterIndexError::CoinsMetadataMismatch);
        }
        Ok(Self { positions })
    }

    /// Borrow the proven nonempty metadata tip.
    pub fn tip(&self) -> &'a ChainPosition {
        &self.positions[self.positions.len() - 1]
    }
    /// Borrow recovered authority at a height, or absence above its tip.
    pub fn maybe_position(&self, height: u32) -> Option<&'a ChainPosition> {
        self.positions.get(height as usize)
    }
}

/// Conservative earliest required input, or explicit exhaustion of height space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexInputProtection {
    FromHeight(u32),
    HeightSpaceExhausted,
}

impl IndexInputProtection {
    /// Parse a reserved lock only if ordinary lock arithmetic cannot overflow.
    pub fn from_saved_lock(lock: &PruneLockInfo) -> Result<Self, FilterIndexError> {
        if lock.name != BASIC_INDEX_PRUNE_LOCK
            || lock.height_first > lock.height_last
            || lock.height_last != u32::MAX - PRUNE_LOCK_BUFFER
        {
            return Err(FilterIndexError::MalformedProtection);
        }
        Ok(Self::FromHeight(lock.height_first))
    }

    /// Render a range safe for the existing buffered helper; high starts strengthen.
    pub fn maybe_prune_lock(self) -> Option<PruneLockInfo> {
        let Self::FromHeight(first) = self else {
            return None;
        };
        let last = u32::MAX - PRUNE_LOCK_BUFFER;
        Some(PruneLockInfo {
            name: BASIC_INDEX_PRUNE_LOCK.to_owned(),
            height_first: first.min(last),
            height_last: last,
        })
    }

    /// Return whether this saved protection covers every input required by `required`.
    pub fn covers(self, required: Self) -> bool {
        match (self, required) {
            (_, Self::HeightSpaceExhausted) => true,
            (Self::FromHeight(saved), Self::FromHeight(earliest)) => saved <= earliest,
            (Self::HeightSpaceExhausted, Self::FromHeight(_)) => false,
        }
    }

    /// Refuse deletion at or above required input, including heights zero and one.
    pub fn check_prune_intent(self, height: u32) -> Result<(), FilterIndexError> {
        if let Self::FromHeight(earliest) = self
            && height >= earliest
        {
            return Err(FilterIndexError::UnsafePruneIntent);
        }
        Ok(())
    }
}

/// Explicit integrity and progress refusals without path or payload disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterIndexError {
    Genesis,
    Predecessor,
    HeaderCommitment,
    MissingCoins,
    MissingMetadata,
    MetadataAncestry,
    CoinsMetadataMismatch,
    MalformedProtection,
    WeakProtection,
    PartialState,
    Projection,
    Checkpoint,
    NoCommonGenesis,
    UnsafePruneIntent,
}

impl fmt::Display for FilterIndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Genesis => "invalid BASIC genesis identity",
            Self::Predecessor => "missing or incompatible BASIC predecessor",
            Self::HeaderCommitment => "invalid BASIC header commitment",
            Self::MissingCoins => "recovered coins best block is absent",
            Self::MissingMetadata => "durable active metadata is absent or empty",
            Self::MetadataAncestry => "durable active metadata ancestry is incompatible",
            Self::CoinsMetadataMismatch => "coins best block differs from durable metadata tip",
            Self::MalformedProtection => "reserved BASIC protection range is malformed",
            Self::WeakProtection => "saved BASIC protection is absent or weaker than checkpoint",
            Self::PartialState => "BASIC state is absent but index artifacts exist",
            Self::Projection => "BASIC active projection is incomplete or incompatible",
            Self::Checkpoint => "BASIC checkpoint differs from projected endpoint",
            Self::NoCommonGenesis => "BASIC index has no common recovered genesis",
            Self::UnsafePruneIntent => "prune intent targets required BASIC input",
        })
    }
}

impl std::error::Error for FilterIndexError {}

#[cfg(test)]
mod tests;
