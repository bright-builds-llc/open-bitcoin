// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Pure ownership facts. These public values never authorize storage writes.

use core::fmt;
use open_bitcoin_primitives::BlockHash;

use super::{FilterCheckpoint, FilterIndexError, IndexInputProtection};
use crate::PruneLockInfo;

/// Durable work incarnation, advanced only through checked lifecycle transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexGeneration(u64);

impl IndexGeneration {
    /// Decode a generation; every u64 is valid, but exhaustion cannot advance.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the stable persisted value.
    pub const fn value(self) -> u64 {
        self.0
    }

    fn next(self) -> Result<Self, IndexLifecycleError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(IndexLifecycleError::GenerationExhausted)
    }
}

/// Explicit lifecycle; absent owner bytes never mean Disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexLifecycle {
    Active { generation: IndexGeneration },
    Disabled { generation: IndexGeneration },
}

impl IndexLifecycle {
    /// Return the persisted work incarnation in either mode.
    pub const fn generation(self) -> IndexGeneration {
        match self {
            Self::Active { generation } | Self::Disabled { generation } => generation,
        }
    }

    /// Invalidate active work before the adapter releases retained protection.
    pub fn disable(self) -> Result<Self, IndexLifecycleError> {
        match self {
            Self::Active { generation } => Ok(Self::Disabled {
                generation: generation.next()?,
            }),
            Self::Disabled { .. } => Ok(self),
        }
    }

    /// Require a fresh generation on re-enable; active retries are idempotent.
    /// The adapter must prove history and durably acquire protection first.
    pub fn enable(self) -> Result<Self, IndexLifecycleError> {
        match self {
            Self::Disabled { generation } => Ok(Self::Active {
                generation: generation.next()?,
            }),
            Self::Active { .. } => Ok(self),
        }
    }
}

/// Exact saved frontier and its branch/fence identity, rather than height alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexCheckpointIdentity {
    checkpoint: FilterCheckpoint,
    fence_height: u32,
    fence_hash: BlockHash,
}

impl IndexCheckpointIdentity {
    /// Bind validated saved checkpoint facts; ancestry proof stays in recovery.
    pub const fn new(
        checkpoint: FilterCheckpoint,
        fence_height: u32,
        fence_hash: BlockHash,
    ) -> Self {
        Self {
            checkpoint,
            fence_height,
            fence_hash,
        }
    }

    /// Return the exact immutable endpoint assertion.
    pub const fn checkpoint(self) -> FilterCheckpoint {
        self.checkpoint
    }

    /// Return the saved chainstate fence height.
    pub const fn fence_height(self) -> u32 {
        self.fence_height
    }

    /// Return the saved chainstate fence block hash.
    pub const fn fence_hash(self) -> BlockHash {
        self.fence_hash
    }
}

/// Validated immutable snapshot; Disabled may conservatively retain protection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveIndexOwnership {
    lifecycle: IndexLifecycle,
    checkpoint: IndexCheckpointIdentity,
    saved_protection: IndexInputProtection,
    maybe_effective_protection: Option<IndexInputProtection>,
}

impl EffectiveIndexOwnership {
    /// Validate bounded saved ownership facts, preserving legacy absence/Active.
    /// Full record/projection integrity is separately proven at startup/release.
    pub fn maybe_from_artifacts(
        maybe_lifecycle: Option<IndexLifecycle>,
        maybe_checkpoint: Option<IndexCheckpointIdentity>,
        maybe_saved_protection: Option<IndexInputProtection>,
        maybe_lock: Option<&PruneLockInfo>,
        has_records: bool,
        has_projection: bool,
    ) -> Result<Option<Self>, IndexLifecycleError> {
        let Some(checkpoint) = maybe_checkpoint else {
            if maybe_lifecycle.is_some()
                || maybe_saved_protection.is_some()
                || maybe_lock.is_some()
                || has_records
                || has_projection
            {
                return Err(FilterIndexError::PartialState.into());
            }
            return Ok(None);
        };
        let Some(saved_protection) = maybe_saved_protection else {
            return Err(FilterIndexError::PartialState.into());
        };
        if !saved_protection.covers(checkpoint.checkpoint().input_protection()) {
            return Err(FilterIndexError::WeakProtection.into());
        }
        let lifecycle = maybe_lifecycle.unwrap_or(IndexLifecycle::Active {
            generation: IndexGeneration::new(0),
        });
        let maybe_lock_protection = maybe_lock
            .map(IndexInputProtection::from_saved_lock)
            .transpose()?;
        if maybe_lock_protection.is_some_and(|protection| !protection.covers(saved_protection)) {
            return Err(FilterIndexError::WeakProtection.into());
        }
        let maybe_effective_protection = match lifecycle {
            IndexLifecycle::Active { .. } => {
                if maybe_lock_protection.is_none()
                    && saved_protection != IndexInputProtection::HeightSpaceExhausted
                {
                    return Err(FilterIndexError::WeakProtection.into());
                }
                Some(maybe_lock_protection.unwrap_or(saved_protection))
            }
            IndexLifecycle::Disabled { .. } => maybe_lock_protection,
        };
        Ok(Some(Self {
            lifecycle,
            checkpoint,
            saved_protection,
            maybe_effective_protection,
        }))
    }

    /// Return the explicit mode and work incarnation.
    pub const fn lifecycle(self) -> IndexLifecycle {
        self.lifecycle
    }

    /// Return the full saved frontier and branch identity.
    pub const fn checkpoint(self) -> IndexCheckpointIdentity {
        self.checkpoint
    }

    /// Return the unchanged checkpoint's persisted protection assertion.
    pub const fn saved_protection(self) -> IndexInputProtection {
        self.saved_protection
    }

    /// Return direct deletion protection; absence requires explicit Disabled.
    pub const fn maybe_effective_protection(self) -> Option<IndexInputProtection> {
        self.maybe_effective_protection
    }

    /// Refuse stale generation/frontier/branch or lock facts before publication.
    pub fn check_work(
        self,
        work: IndexWorkIdentity,
        fence_height: u32,
        fence_hash: BlockHash,
    ) -> Result<(), IndexLifecycleError> {
        if !matches!(self.lifecycle, IndexLifecycle::Active { .. })
            || self != work.expected
            || fence_height != work.fence_height
            || fence_hash != work.fence_hash
        {
            return Err(IndexLifecycleError::StaleWork);
        }
        Ok(())
    }
}

/// Prepared facts only; adapters mint private, store-bound capability wrappers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexWorkIdentity {
    expected: EffectiveIndexOwnership,
    fence_height: u32,
    fence_hash: BlockHash,
}

impl IndexWorkIdentity {
    /// Capture exact ownership and preparation fence without granting authority.
    pub const fn new(
        expected: EffectiveIndexOwnership,
        fence_height: u32,
        fence_hash: BlockHash,
    ) -> Self {
        Self {
            expected,
            fence_height,
            fence_hash,
        }
    }
}

/// Explicit lifecycle refusal without persisted payload disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexLifecycleError {
    Integrity(FilterIndexError),
    GenerationExhausted,
    StaleWork,
}

impl From<FilterIndexError> for IndexLifecycleError {
    fn from(error: FilterIndexError) -> Self {
        Self::Integrity(error)
    }
}

impl fmt::Display for IndexLifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integrity(error) => error.fmt(f),
            Self::GenerationExhausted => f.write_str("BASIC index generation exhausted"),
            Self::StaleWork => f.write_str("stale or disabled BASIC index work"),
        }
    }
}

impl std::error::Error for IndexLifecycleError {}
