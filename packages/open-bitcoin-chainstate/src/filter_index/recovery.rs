// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Pure streamed recovery decisions; immutable rows are never erased by a plan.

use open_bitcoin_primitives::BlockHash;

use super::{
    FilterCheckpoint, FilterIndexError, FilterRecordIdentity, IndexInputProtection, IndexPrefix,
    VerifiedChainstateFence,
};

/// Recovery outcomes describe projection/state publication, never immutable deletion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterRecoveryPlan {
    LegacyAbsent,
    Keep {
        checkpoint: FilterCheckpoint,
        protection: IndexInputProtection,
    },
    Reconcile {
        checkpoint: FilterCheckpoint,
        protection: IndexInputProtection,
    },
    Refuse(FilterIndexError),
}

impl FilterRecoveryPlan {
    /// Legacy startup is allowed only when all index-owned artifacts are absent.
    pub fn for_absent_state(
        has_records: bool,
        has_projection: bool,
        maybe_protection: Option<IndexInputProtection>,
    ) -> Self {
        if has_records || has_projection || maybe_protection.is_some() {
            return Self::Refuse(FilterIndexError::PartialState);
        }
        Self::LegacyAbsent
    }
}

/// Small reducer over the saved active projection, with one borrowed durable fence.
/// The adapter supplies one verified record and projection identity per step.
#[derive(Debug)]
pub struct FilterRecoveryScan<'a, 'b> {
    saved: FilterCheckpoint,
    protection: IndexInputProtection,
    fence: &'a VerifiedChainstateFence<'b>,
    maybe_last: Option<FilterRecordIdentity>,
    maybe_common: Option<FilterRecordIdentity>,
    maybe_refusal: Option<FilterIndexError>,
    diverged: bool,
}

impl<'a, 'b> FilterRecoveryScan<'a, 'b> {
    /// Reject absent/weak protection before inspecting a possible rewind.
    pub fn new(
        saved: FilterCheckpoint,
        maybe_protection: Option<IndexInputProtection>,
        fence: &'a VerifiedChainstateFence<'b>,
    ) -> Result<Self, FilterIndexError> {
        let Some(protection) =
            maybe_protection.filter(|protection| protection.covers(saved.input_protection()))
        else {
            return Err(FilterIndexError::WeakProtection);
        };
        Ok(Self {
            saved,
            protection,
            fence,
            maybe_last: None,
            maybe_common: None,
            maybe_refusal: None,
            diverged: false,
        })
    }

    /// Validate one consecutive saved-projection record without performing storage I/O.
    pub fn push(
        &mut self,
        record: FilterRecordIdentity,
        projected_hash: BlockHash,
    ) -> Result<(), FilterIndexError> {
        if let Some(error) = self.maybe_refusal {
            return Err(error);
        }
        let result = self.push_validated(record, projected_hash);
        if let Err(error) = result {
            self.maybe_refusal = Some(error);
        }
        result
    }

    fn push_validated(
        &mut self,
        record: FilterRecordIdentity,
        projected_hash: BlockHash,
    ) -> Result<(), FilterIndexError> {
        let IndexPrefix::Committed(endpoint) = self.saved.prefix() else {
            return Err(FilterIndexError::Projection);
        };
        if projected_hash != record.block_hash() || record.height() > endpoint.height() {
            return Err(FilterIndexError::Projection);
        }
        match self.maybe_last {
            None if record.height() != 0 => return Err(FilterIndexError::Projection),
            Some(last)
                if last.height().checked_add(1) != Some(record.height())
                    || last.block_hash() != record.parent_hash()
                    || last.filter_header() != record.previous_header() =>
            {
                return Err(FilterIndexError::Projection);
            }
            _ => {}
        }
        let matches_authority =
            self.fence
                .maybe_position(record.height())
                .is_some_and(|position| {
                    position.block_hash == record.block_hash()
                        && position.previous_block_hash() == record.parent_hash()
                });
        if matches_authority && !self.diverged {
            self.maybe_common = Some(record);
        } else {
            self.diverged = true;
        }
        self.maybe_last = Some(record);
        Ok(())
    }

    /// Require exact saved endpoint and return the verified common indexed prefix.
    pub fn finish(self) -> FilterRecoveryPlan {
        if let Some(error) = self.maybe_refusal {
            return FilterRecoveryPlan::Refuse(error);
        }
        match self.saved.prefix() {
            IndexPrefix::Empty => FilterRecoveryPlan::Keep {
                checkpoint: self.saved,
                protection: self.protection,
            },
            IndexPrefix::Committed(endpoint) => {
                if self.maybe_last != Some(endpoint) {
                    return FilterRecoveryPlan::Refuse(FilterIndexError::Checkpoint);
                }
                let Some(common) = self.maybe_common else {
                    return FilterRecoveryPlan::Refuse(FilterIndexError::NoCommonGenesis);
                };
                if common == endpoint {
                    return FilterRecoveryPlan::Keep {
                        checkpoint: self.saved,
                        protection: self.protection,
                    };
                }
                let checkpoint = FilterCheckpoint::new(IndexPrefix::Committed(common));
                let protection = if self.protection.covers(checkpoint.input_protection()) {
                    self.protection
                } else {
                    checkpoint.input_protection()
                };
                FilterRecoveryPlan::Reconcile {
                    checkpoint,
                    protection,
                }
            }
        }
    }
}
