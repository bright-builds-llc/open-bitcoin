// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Checked pre-work accounting; production limits are supplied by measured adapters.

use core::fmt;

/// Existing storage publisher candidate ceiling.
pub const BASIC_INDEX_MAX_CANDIDATES: u64 = 128;
/// Existing codec MAX_SIZE plus 128 immutable record envelopes (170 bytes each).
pub const BASIC_INDEX_MAX_ENCODED_BYTES: u64 = 0x0200_0000 + 128 * 170;
/// One codec-valid filter plus its immutable record envelope.
pub const BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES: u64 = 0x0200_0000 + 170;

/// Reserved upper-bound work, including input copies and all validation/publication.
/// Encoded bytes include immutable envelope overhead. Script counts include all
/// examined inputs/outputs before exclusions/deduplication, not only filter entries.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TurnWork {
    pub blocks: u64,
    pub body_bytes: u64,
    pub undo_bytes: u64,
    pub cloned_bytes: u64,
    pub script_items: u64,
    pub script_bytes: u64,
    pub encoded_bytes: u64,
    pub record_operations: u64,
    pub checkpoint_operations: u64,
    pub projection_operations: u64,
}

impl TurnWork {
    fn counters(self) -> [u64; 10] {
        [
            self.blocks,
            self.body_bytes,
            self.undo_bytes,
            self.cloned_bytes,
            self.script_items,
            self.script_bytes,
            self.encoded_bytes,
            self.record_operations,
            self.checkpoint_operations,
            self.projection_operations,
        ]
    }
    /// Sum reservations without truncating any counter on overflow.
    pub fn checked_add(self, other: Self) -> Result<Self, TurnBudgetError> {
        let mut sums = [0; 10];
        for ((sum, left), right) in sums.iter_mut().zip(self.counters()).zip(other.counters()) {
            *sum = left.checked_add(right).ok_or(TurnBudgetError::Overflow)?;
        }
        let [
            blocks,
            body_bytes,
            undo_bytes,
            cloned_bytes,
            script_items,
            script_bytes,
            encoded_bytes,
            record_operations,
            checkpoint_operations,
            projection_operations,
        ] = sums;
        Ok(Self {
            blocks,
            body_bytes,
            undo_bytes,
            cloned_bytes,
            script_items,
            script_bytes,
            encoded_bytes,
            record_operations,
            checkpoint_operations,
            projection_operations,
        })
    }
    /// Require every count and byte bound to fit, including completion work.
    pub fn fits(self, limit: Self) -> bool {
        self.counters()
            .into_iter()
            .zip(limit.counters())
            .all(|(used, max)| used <= max)
    }
}

/// Injectable normal aggregate and legal absolute singleton ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicIndexTurnBudget {
    normal: TurnWork,
    absolute_singleton: TurnWork,
}

impl BasicIndexTurnBudget {
    /// Reject zero bounds and reservations beyond existing codec ceilings.
    /// Adapters choose absolute input/script bounds from validated legal maxima.
    pub fn new(normal: TurnWork, absolute_singleton: TurnWork) -> Result<Self, TurnBudgetError> {
        if normal.blocks == 0
            || normal.blocks > BASIC_INDEX_MAX_CANDIDATES
            || absolute_singleton.blocks != 1
            || normal.encoded_bytes > BASIC_INDEX_MAX_ENCODED_BYTES
            || absolute_singleton.encoded_bytes > BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES
            || normal.counters().contains(&0)
            || absolute_singleton.counters().contains(&0)
        {
            return Err(TurnBudgetError::InvalidBudget);
        }
        Ok(Self {
            normal,
            absolute_singleton,
        })
    }
    /// Return the normal turn's limits for accounting/measurement.
    pub const fn normal(self) -> TurnWork {
        self.normal
    }
    /// Return the explicit legal singleton bounds.
    pub const fn absolute_singleton(self) -> TurnWork {
        self.absolute_singleton
    }
    /// Admit before hashing/sorting/cloning. Every candidate is one validated block.
    /// `used` can reserve constant preparation/publication work with zero blocks.
    pub fn admit(
        self,
        used: TurnWork,
        candidate: TurnWork,
    ) -> Result<TurnAdmission, TurnBudgetError> {
        if candidate.blocks != 1
            || candidate.body_bytes == 0
            || candidate.encoded_bytes == 0
            || candidate.record_operations == 0
            || candidate.checkpoint_operations == 0
            || candidate.projection_operations == 0
        {
            return Err(TurnBudgetError::InvalidWork);
        }
        if !candidate.fits(self.absolute_singleton) {
            return Err(TurnBudgetError::AbsoluteLimit);
        }
        let total = used.checked_add(candidate)?;
        if !(used.fits(self.normal) || used.blocks == 1 && used.fits(self.absolute_singleton)) {
            return Err(TurnBudgetError::InvalidWork);
        }
        if total.fits(self.normal) {
            return Ok(TurnAdmission::Normal(total));
        }
        if used.blocks != 0 {
            return Ok(TurnAdmission::Yield);
        }
        if !total.fits(self.absolute_singleton) {
            return Err(TurnBudgetError::AbsoluteLimit);
        }
        Ok(TurnAdmission::OversizedSingleton(total))
    }
}

/// A singleton always ends its turn; Yield preserves the previous reservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnAdmission {
    Normal(TurnWork),
    OversizedSingleton(TurnWork),
    Yield,
}

/// Deterministic refusal prevents silent overrun and endless singleton retries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnBudgetError {
    InvalidBudget,
    InvalidWork,
    Overflow,
    AbsoluteLimit,
}

impl fmt::Display for TurnBudgetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidBudget => "invalid BASIC turn budgets",
            Self::InvalidWork => "invalid BASIC work reservation",
            Self::Overflow => "BASIC work accounting overflow",
            Self::AbsoluteLimit => "BASIC singleton exceeds absolute work bounds",
        })
    }
}

impl std::error::Error for TurnBudgetError {}
