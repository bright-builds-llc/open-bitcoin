// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Constant-size replacement policy; adapters separately authenticate ancestry and effects.

use super::*;

/// Separately verified replacement endpoints. These public facts grant no authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicIndexReplacementFacts {
    pub expected_generation: IndexGeneration,
    pub expected_branch_identity: BlockHash,
    pub old_target: AcceptedIndexTarget,
    pub new_target: AcceptedIndexTarget,
    pub maybe_common_ancestor: Option<AcceptedIndexTarget>,
    pub maybe_indexed_common: Option<FilterRecordIdentity>,
    pub maybe_shared_safe: Option<FilterRecordIdentity>,
    pub achieved_generation: IndexGeneration,
    pub achieved_branch_identity: BlockHash,
    pub protection: IndexInputProtection,
}

impl BasicIndexProgress {
    /// Return a checked trial state without mutating progress or granting storage permission.
    /// Complete common-prefix ancestry is verified by the adapter before supplying facts.
    pub fn replace_validated_branch(
        self,
        facts: BasicIndexReplacementFacts,
    ) -> Result<Self, BasicIndexCatchUpError> {
        if self.state == BasicIndexState::Disabled {
            return Err(BasicIndexCatchUpError::NotActive);
        }
        if self.generation != facts.expected_generation
            || self.branch_identity != facts.expected_branch_identity
            || self.accepted_target != facts.old_target
        {
            return Err(BasicIndexCatchUpError::StaleWork);
        }
        let next_generation = self
            .generation
            .value()
            .checked_add(1)
            .ok_or(BasicIndexCatchUpError::GenerationExhausted)?;
        if facts.achieved_generation != IndexGeneration::new(next_generation)
            || facts.achieved_branch_identity != facts.new_target.block_hash()
        {
            return Err(BasicIndexCatchUpError::StaleWork);
        }
        if let Some(ancestor) = facts.maybe_common_ancestor {
            for target in [facts.old_target, facts.new_target] {
                if ancestor.height() > target.height()
                    || (ancestor.height() == target.height() && ancestor != target)
                {
                    return Err(BasicIndexCatchUpError::InvalidProgress);
                }
            }
        }
        check_shared_endpoint(
            self.maybe_processed_endpoint,
            facts.maybe_indexed_common,
            facts.maybe_common_ancestor,
        )?;
        check_shared_endpoint(
            self.maybe_safe_durable_endpoint,
            facts.maybe_shared_safe,
            facts.maybe_common_ancestor,
        )?;
        check_endpoint_order(facts.maybe_indexed_common, facts.maybe_shared_safe)?;
        if !facts.protection.covers(self.protection)
            || !facts
                .protection
                .covers(prefix(facts.maybe_shared_safe).input_protection())
        {
            return Err(BasicIndexCatchUpError::WeakProtection);
        }
        Ok(Self {
            generation: facts.achieved_generation,
            branch_identity: facts.achieved_branch_identity,
            accepted_target: facts.new_target,
            maybe_processed_endpoint: facts.maybe_indexed_common,
            maybe_safe_durable_endpoint: facts.maybe_shared_safe,
            protection: facts.protection,
            initially_synchronized: self.initially_synchronized,
            state: BasicIndexState::Active,
        })
    }
}

fn check_shared_endpoint(
    maybe_old: Option<FilterRecordIdentity>,
    maybe_shared: Option<FilterRecordIdentity>,
    maybe_ancestor: Option<AcceptedIndexTarget>,
) -> Result<(), BasicIndexCatchUpError> {
    let (Some(old), Some(ancestor)) = (maybe_old, maybe_ancestor) else {
        return if maybe_shared.is_none() {
            Ok(())
        } else {
            Err(BasicIndexCatchUpError::InvalidProgress)
        };
    };
    let Some(shared) = maybe_shared else {
        return Err(BasicIndexCatchUpError::InvalidProgress);
    };
    if shared.height() != old.height().min(ancestor.height())
        || (old.height() <= ancestor.height() && shared != old)
        || (shared.height() == ancestor.height() && shared.block_hash() != ancestor.block_hash())
    {
        return Err(BasicIndexCatchUpError::InvalidProgress);
    }
    check_endpoint_order(Some(old), Some(shared))
}
