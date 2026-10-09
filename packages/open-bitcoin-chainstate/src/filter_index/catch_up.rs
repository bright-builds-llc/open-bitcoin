// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Ordered progress and admission facts; these values confer no storage authority.

mod budget;
mod reorg;
pub use budget::{
    BASIC_INDEX_MAX_CANDIDATES, BASIC_INDEX_MAX_ENCODED_BYTES,
    BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES, BasicIndexTurnBudget, TurnAdmission, TurnBudgetError,
    TurnWork,
};
pub use reorg::BasicIndexReplacementFacts;

use core::fmt;
use open_bitcoin_primitives::BlockHash;

use super::lifecycle::{IndexGeneration, IndexLifecycle};
use super::{
    FilterCheckpoint, FilterRecordIdentity, IndexInputProtection, IndexPrefix,
    VerifiedChainstateFence, verify_filter_record_predecessor,
};

/// Accepted validated chain tip, distinct from generated rows and durable coins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcceptedIndexTarget {
    height: u32,
    block_hash: BlockHash,
}

impl AcceptedIndexTarget {
    /// Capture validated acceptance facts. This constructor does not validate a block.
    pub const fn new(height: u32, block_hash: BlockHash) -> Self {
        Self { height, block_hash }
    }
    /// Return the accepted height.
    pub const fn height(self) -> u32 {
        self.height
    }
    /// Return the accepted block identity.
    pub const fn block_hash(self) -> BlockHash {
        self.block_hash
    }
}

/// Work stops explicitly without discarding accepted targets or safe protection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicIndexPause {
    Persistence,
    MissingHistory,
    Invalidated,
}

/// Operational status is independent of initial completion and current lag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicIndexState {
    Active,
    Paused(BasicIndexPause),
    Disabled,
}

/// Constant-size progress. Adapters prove complete recovered prefix ancestry first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicIndexProgress {
    generation: IndexGeneration,
    branch_identity: BlockHash,
    accepted_target: AcceptedIndexTarget,
    maybe_processed_endpoint: Option<FilterRecordIdentity>,
    maybe_safe_durable_endpoint: Option<FilterRecordIdentity>,
    protection: IndexInputProtection,
    initially_synchronized: bool,
    state: BasicIndexState,
}

/// Immutable preparation snapshot; no public fields or storage permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicIndexPreparedTurn {
    expected: BasicIndexProgress,
}

/// A checkpoint endpoint matched against already-verified durable chainstate.
/// Complete prefix/projection verification remains the adapter's responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedIndexDurability {
    generation: IndexGeneration,
    branch_identity: BlockHash,
    maybe_endpoint: Option<FilterRecordIdentity>,
}

impl ValidatedIndexDurability {
    /// Check durable endpoint correspondence without rescanning or copying history.
    pub fn new(
        generation: IndexGeneration,
        branch_identity: BlockHash,
        checkpoint: FilterCheckpoint,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<Self, BasicIndexCatchUpError> {
        let maybe_endpoint = match checkpoint.prefix() {
            IndexPrefix::Empty => None,
            IndexPrefix::Committed(record) => {
                if !fence
                    .maybe_position(record.height())
                    .is_some_and(|position| {
                        position.block_hash == record.block_hash()
                            && position.previous_block_hash() == record.parent_hash()
                    })
                {
                    return Err(BasicIndexCatchUpError::InvalidDurability);
                }
                Some(record)
            }
        };
        Ok(Self {
            generation,
            branch_identity,
            maybe_endpoint,
        })
    }
}

impl BasicIndexProgress {
    /// Parse recovered facts; rows ahead of recovered durability are processed only.
    /// `branch_identity` is the adapter's stable branch incarnation, never height alone.
    pub fn new(
        generation: IndexGeneration,
        branch_identity: BlockHash,
        accepted_target: AcceptedIndexTarget,
        maybe_processed_endpoint: Option<FilterRecordIdentity>,
        maybe_safe_durable_endpoint: Option<FilterRecordIdentity>,
        protection: IndexInputProtection,
    ) -> Result<Self, BasicIndexCatchUpError> {
        check_endpoint_order(maybe_processed_endpoint, maybe_safe_durable_endpoint)?;
        if maybe_processed_endpoint.is_some_and(|record| {
            record.height() > accepted_target.height
                || (record.height() == accepted_target.height
                    && record.block_hash() != accepted_target.block_hash)
        }) {
            return Err(BasicIndexCatchUpError::InvalidProgress);
        }
        if !protection.covers(prefix(maybe_safe_durable_endpoint).input_protection()) {
            return Err(BasicIndexCatchUpError::WeakProtection);
        }
        let initially_synchronized = maybe_processed_endpoint.is_some_and(|record| {
            record.height() == accepted_target.height
                && record.block_hash() == accepted_target.block_hash
        });
        Ok(Self {
            generation,
            branch_identity,
            accepted_target,
            maybe_processed_endpoint,
            maybe_safe_durable_endpoint,
            protection,
            initially_synchronized,
            state: BasicIndexState::Active,
        })
    }

    /// Return accepted work even after a later persistence failure.
    pub const fn accepted_target(self) -> AcceptedIndexTarget {
        self.accepted_target
    }
    /// Return the checked lifecycle incarnation captured by prepared work.
    pub const fn generation(self) -> IndexGeneration {
        self.generation
    }
    /// Return the stable branch incarnation supplied by validated adapters.
    pub const fn branch_identity(self) -> BlockHash {
        self.branch_identity
    }
    /// Return the generated contiguous endpoint, which may be ahead of coins.
    pub const fn maybe_processed_endpoint(self) -> Option<FilterRecordIdentity> {
        self.maybe_processed_endpoint
    }
    /// Return only confirmed recoverable progress.
    pub const fn maybe_safe_durable_endpoint(self) -> Option<FilterRecordIdentity> {
        self.maybe_safe_durable_endpoint
    }
    /// Return currently retained conservative inputs.
    pub const fn protection(self) -> IndexInputProtection {
        self.protection
    }
    /// Return the historical initial-completion latch.
    pub const fn initially_synchronized(self) -> bool {
        self.initially_synchronized
    }
    /// Return operational state independently of lag.
    pub const fn state(self) -> BasicIndexState {
        self.state
    }
    /// Count missing rows, including genesis, without u32 wraparound.
    pub fn current_lag(self) -> u64 {
        u64::from(self.accepted_target.height) + 1
            - self
                .maybe_processed_endpoint
                .map_or(0, |r| u64::from(r.height()) + 1)
    }
    /// Choose only the next consecutive height, or none when caught up.
    pub fn maybe_next_height(self) -> Result<Option<u32>, BasicIndexCatchUpError> {
        if self.current_lag() == 0 {
            return Ok(None);
        }
        self.maybe_processed_endpoint.map_or(Ok(Some(0)), |r| {
            r.height()
                .checked_add(1)
                .map(Some)
                .ok_or(BasicIndexCatchUpError::HeightExhausted)
        })
    }
    /// Observe only an already-validated direct child; rejected inputs never call this.
    /// This transition must precede the adapter's fallible persistence operation.
    pub fn observe_validated_connect(
        &mut self,
        target: AcceptedIndexTarget,
        parent: BlockHash,
    ) -> Result<(), BasicIndexCatchUpError> {
        if self.state == BasicIndexState::Disabled {
            return Err(BasicIndexCatchUpError::NotActive);
        }
        let next = self
            .accepted_target
            .height
            .checked_add(1)
            .ok_or(BasicIndexCatchUpError::HeightExhausted)?;
        if target.height != next || parent != self.accepted_target.block_hash {
            return Err(BasicIndexCatchUpError::NonConsecutive);
        }
        self.accepted_target = target;
        Ok(())
    }
    /// Capture an immutable frontier. Accepted growth may proceed during preparation.
    pub fn prepare_turn(self) -> Result<BasicIndexPreparedTurn, BasicIndexCatchUpError> {
        if self.state != BasicIndexState::Active {
            return Err(BasicIndexCatchUpError::NotActive);
        }
        Ok(BasicIndexPreparedTurn { expected: self })
    }
    /// Atomically reduce a bounded consecutive suffix after achieved publication.
    /// Store-bound proof and byte integrity must be checked by the adapter first.
    pub fn complete_turn(
        &mut self,
        prepared: BasicIndexPreparedTurn,
        records: &[FilterRecordIdentity],
    ) -> Result<(), BasicIndexCatchUpError> {
        self.check_prepared(prepared)?;
        if records.is_empty() || records.len() > BASIC_INDEX_MAX_CANDIDATES as usize {
            return Err(BasicIndexCatchUpError::InvalidTurn);
        }
        let mut next = *self;
        for record in records {
            if record.height() > prepared.expected.accepted_target.height {
                return Err(BasicIndexCatchUpError::StaleWork);
            }
            if next.maybe_next_height()? != Some(record.height()) {
                return Err(BasicIndexCatchUpError::NonConsecutive);
            }
            verify_filter_record_predecessor(
                record.height(),
                record.parent_hash(),
                record.previous_header(),
                next.maybe_processed_endpoint
                    .map(|r| (r.height(), r.block_hash(), r.filter_header())),
            )
            .map_err(|_| BasicIndexCatchUpError::NonConsecutive)?;
            for target in [prepared.expected.accepted_target, self.accepted_target] {
                if record.height() == target.height && record.block_hash() != target.block_hash {
                    return Err(BasicIndexCatchUpError::StaleWork);
                }
            }
            next.maybe_processed_endpoint = Some(*record);
        }
        next.initially_synchronized |= next.current_lag() == 0;
        *self = next;
        Ok(())
    }
    /// Advance only verified achieved durability; failures leave this method uncalled.
    pub fn confirm_safe_durable(
        &mut self,
        validated: ValidatedIndexDurability,
        protection: IndexInputProtection,
    ) -> Result<(), BasicIndexCatchUpError> {
        self.confirm_achieved_checkpoint(
            validated.generation,
            validated.branch_identity,
            FilterCheckpoint::new(prefix(validated.maybe_endpoint)),
            protection,
        )
    }
    /// Reduce adapter-achieved checkpoint facts without rescanning history.
    /// This pure method grants no storage authority: callers must separately
    /// consume an authenticated achieved publication before applying these facts.
    pub fn confirm_achieved_checkpoint(
        &mut self,
        generation: IndexGeneration,
        branch_identity: BlockHash,
        checkpoint: FilterCheckpoint,
        protection: IndexInputProtection,
    ) -> Result<(), BasicIndexCatchUpError> {
        let maybe_endpoint = match checkpoint.prefix() {
            IndexPrefix::Empty => None,
            IndexPrefix::Committed(id) => Some(id),
        };
        if self.state != BasicIndexState::Active
            || self.generation != generation
            || self.branch_identity != branch_identity
        {
            return Err(BasicIndexCatchUpError::StaleWork);
        }
        check_endpoint_order(self.maybe_processed_endpoint, maybe_endpoint)?;
        check_endpoint_order(maybe_endpoint, self.maybe_safe_durable_endpoint)?;
        if !protection.covers(prefix(maybe_endpoint).input_protection()) {
            return Err(BasicIndexCatchUpError::WeakProtection);
        }
        self.maybe_safe_durable_endpoint = maybe_endpoint;
        self.protection = protection;
        Ok(())
    }
    /// Preserve all progress/protection while suspending work for an explicit reason.
    pub fn pause(&mut self, reason: BasicIndexPause) {
        if self.state != BasicIndexState::Disabled {
            self.state = BasicIndexState::Paused(reason);
        }
    }
    /// Resume after the adapter has re-proven unchanged storage and branch authority.
    pub fn resume(
        &mut self,
        generation: IndexGeneration,
        branch_identity: BlockHash,
    ) -> Result<(), BasicIndexCatchUpError> {
        if self.state == BasicIndexState::Disabled
            || generation != self.generation
            || branch_identity != self.branch_identity
        {
            return Err(BasicIndexCatchUpError::StaleWork);
        }
        self.state = BasicIndexState::Active;
        Ok(())
    }
    /// Invalidate work with checked generation advancement before ownership release.
    pub fn disable(&mut self) -> Result<(), BasicIndexCatchUpError> {
        if self.state == BasicIndexState::Disabled {
            return Ok(());
        }
        let lifecycle = IndexLifecycle::Active {
            generation: self.generation,
        }
        .disable()
        .map_err(|_| BasicIndexCatchUpError::GenerationExhausted)?;
        self.generation = lifecycle.generation();
        self.state = BasicIndexState::Disabled;
        Ok(())
    }
    fn check_prepared(
        self,
        prepared: BasicIndexPreparedTurn,
    ) -> Result<(), BasicIndexCatchUpError> {
        let expected = prepared.expected;
        if self.state != BasicIndexState::Active
            || self.generation != expected.generation
            || self.branch_identity != expected.branch_identity
            || self.maybe_processed_endpoint != expected.maybe_processed_endpoint
            || self.maybe_safe_durable_endpoint != expected.maybe_safe_durable_endpoint
            || self.protection != expected.protection
        {
            return Err(BasicIndexCatchUpError::StaleWork);
        }
        Ok(())
    }
}

fn prefix(maybe_endpoint: Option<FilterRecordIdentity>) -> IndexPrefix {
    maybe_endpoint.map_or(IndexPrefix::Empty, IndexPrefix::Committed)
}

fn check_endpoint_order(
    maybe_later: Option<FilterRecordIdentity>,
    maybe_earlier: Option<FilterRecordIdentity>,
) -> Result<(), BasicIndexCatchUpError> {
    if let Some(earlier) = maybe_earlier {
        let Some(later) = maybe_later else {
            return Err(BasicIndexCatchUpError::InvalidProgress);
        };
        if earlier.height() > later.height()
            || (earlier.height() == later.height() && earlier != later)
        {
            return Err(BasicIndexCatchUpError::InvalidProgress);
        }
        if earlier.height().checked_add(1) == Some(later.height())
            && (later.parent_hash() != earlier.block_hash()
                || later.previous_header() != earlier.filter_header())
        {
            return Err(BasicIndexCatchUpError::InvalidProgress);
        }
    }
    Ok(())
}

/// Explicit refusal categories without payloads or paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicIndexCatchUpError {
    InvalidProgress,
    InvalidDurability,
    WeakProtection,
    NonConsecutive,
    StaleWork,
    NotActive,
    HeightExhausted,
    GenerationExhausted,
    InvalidTurn,
}

impl fmt::Display for BasicIndexCatchUpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidProgress => "incompatible BASIC progress endpoints",
            Self::InvalidDurability => "BASIC endpoint differs from verified durable chainstate",
            Self::WeakProtection => "insufficient BASIC input protection",
            Self::NonConsecutive => "non-consecutive BASIC work",
            Self::StaleWork => "stale BASIC generation, branch or frontier",
            Self::NotActive => "BASIC owner is not active",
            Self::HeightExhausted => "BASIC height space exhausted",
            Self::GenerationExhausted => "BASIC generation space exhausted",
            Self::InvalidTurn => "BASIC turn must contain one through 128 records",
        })
    }
}

impl std::error::Error for BasicIndexCatchUpError {}

#[cfg(test)]
mod tests;
