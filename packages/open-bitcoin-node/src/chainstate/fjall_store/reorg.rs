// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Tracked acceptance capabilities; no public or test construction path.

use super::*;

pub(super) struct PendingBasicFilterReorg {
    old_identity: (IndexGeneration, BlockHash, u64),
    achieved_identity: (IndexGeneration, BlockHash, u64),
    maybe_old: Option<(u32, BlockHash)>,
    maybe_new: Option<(u32, BlockHash)>,
    maybe_ancestor: Option<(u32, BlockHash)>,
}

/// Only tracked genuine absorption can mint this same-store publication authority.
pub(crate) struct ValidatedBasicFilterReorg {
    store: FjallNodeStore,
    facts: PendingBasicFilterReorg,
    work: open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork,
}

/// Borrowed next-height positions from a genuinely tracked accepted manager.
pub(crate) struct ValidatedBasicFilterAppendPositions<'a> {
    store: FjallNodeStore,
    identity: (IndexGeneration, BlockHash, u64),
    processed: open_bitcoin_core::chainstate::FilterCheckpoint,
    positions: &'a [ChainPosition],
    work: open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork,
}

impl ValidatedBasicFilterAppendPositions<'_> {
    pub(crate) fn work(&self) -> open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork {
        self.work
    }
    pub(crate) fn validate_for(
        &self,
        store: &FjallNodeStore,
        proof: &BasicFilterAppendProof,
        records: &[crate::storage::filter_index::StoredFilterRecord],
    ) -> Result<(), StorageError> {
        if !self.store.shares_basic_filter_store(store)
            || self.identity
                != (
                    proof.generation(),
                    proof.branch_identity(),
                    proof.revision(),
                )
            || self.processed != proof.processed()
            || self.positions.len() != records.len()
        {
            return Err(receipt_error(
                "foreign or changed BASIC replacement positions",
            ));
        }
        for (position, record) in self.positions.iter().zip(records) {
            let id = record.identity();
            if position.height != id.height()
                || position.block_hash != id.block_hash()
                || position.previous_block_hash() != id.parent_hash()
            {
                return Err(receipt_error(
                    "BASIC replacement differs from accepted position",
                ));
            }
        }
        Ok(())
    }
}

impl ValidatedBasicFilterReorg {
    pub(crate) fn work(&self) -> open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork {
        self.work
    }
    pub(crate) fn validate_for(
        &self,
        store: &FjallNodeStore,
        prepared: &PreparedBasicFilterReorg,
    ) -> Result<(), StorageError> {
        if !self.store.shares_basic_filter_store(store)
            || !prepared.belongs_to(store)
            || !self.facts.matches(prepared)
        {
            return Err(receipt_error(
                "foreign or changed BASIC reorg authorization",
            ));
        }
        Ok(())
    }
}

impl PendingBasicFilterReorg {
    fn from_prepared(prepared: &PreparedBasicFilterReorg) -> Self {
        Self {
            old_identity: prepared.old_identity(),
            achieved_identity: prepared.achieved_identity(),
            maybe_old: prepared.maybe_old_endpoint(),
            maybe_new: prepared.maybe_new_endpoint(),
            maybe_ancestor: prepared.maybe_ancestor_endpoint(),
        }
    }
    fn matches(&self, prepared: &PreparedBasicFilterReorg) -> bool {
        self.old_identity == prepared.old_identity()
            && self.achieved_identity == prepared.achieved_identity()
            && self.maybe_old == prepared.maybe_old_endpoint()
            && self.maybe_new == prepared.maybe_new_endpoint()
            && self.maybe_ancestor == prepared.maybe_ancestor_endpoint()
    }
}

impl ValidatedChainstateLineage {
    pub(in crate::chainstate) fn preflight_required_sources(
        &self,
        staged: &open_bitcoin_core::chainstate::StagedChainstateReorg,
        replacements: &[open_bitcoin_core::chainstate::AnchoredBlock],
        positions: &[ChainPosition],
        undo: &std::collections::HashMap<BlockHash, open_bitcoin_core::chainstate::BlockUndo>,
        processed: open_bitcoin_core::chainstate::FilterCheckpoint,
    ) -> Result<open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork, StorageError> {
        super::super::filter_reorg::preflight::required_sources(
            &self.store,
            staged,
            replacements,
            positions,
            undo,
            processed,
        )
    }
    pub(in crate::chainstate) fn reorg_is_pending(&self) -> bool {
        self.preview_frozen || self.maybe_pending_reorg.is_some()
    }
    pub(in crate::chainstate) fn prepare_reorg_storage(
        &self,
        staged: &open_bitcoin_core::chainstate::StagedChainstateReorg,
    ) -> Result<PreparedBasicFilterReorg, StorageError> {
        if self.preview_frozen
            || self.maybe_pending_reorg.is_some()
            || staged.maybe_old_tip().map(|p| (p.height, p.block_hash)) != self.maybe_accepted
        {
            return Err(receipt_error("untracked BASIC reorg preparation"));
        }
        let proof = self
            .store
            .maybe_basic_filter_append_proof_with_budget(PreparedBasicFilterReorg::maximum_work())?
            .ok_or_else(|| receipt_error("absent BASIC reorg preparation proof"))?;
        self.store.prepare_basic_filter_reorg(&proof, staged)
    }

    pub(in crate::chainstate) fn freeze_index_reorg(
        &mut self,
        prepared: &PreparedBasicFilterReorg,
    ) -> Result<(), StorageError> {
        self.check_index_reorg(prepared)?;
        self.store.suspend_basic_filter_reorg(prepared)?;
        self.preview_frozen = true;
        Ok(())
    }

    pub(in crate::chainstate) fn check_index_reorg(
        &self,
        prepared: &PreparedBasicFilterReorg,
    ) -> Result<(), StorageError> {
        if self.maybe_pending_reorg.is_some()
            || self.maybe_accepted != prepared.maybe_old_endpoint()
            || (self.generation, self.branch)
                != (prepared.old_identity().0, prepared.old_identity().1)
        {
            return Err(receipt_error("changed BASIC manager reorg lineage"));
        }
        self.store.check_basic_filter_reorg_suspension(prepared)
    }

    pub(in crate::chainstate) fn complete_index_reorg(
        &mut self,
        accepted: &AcceptedChainstateReorg,
        prepared: PreparedBasicFilterReorg,
    ) -> Result<(), StorageError> {
        let authorization = self.authorize_reorg(accepted, &prepared)?;
        let achieved = self
            .store
            .complete_basic_filter_reorg(prepared, authorization)?;
        self.confirm_reorg(&achieved)
    }

    pub(in crate::chainstate) fn authorize_append_positions<'a>(
        &self,
        proof: &BasicFilterAppendProof,
        positions: &'a [ChainPosition],
        count: usize,
    ) -> Result<ValidatedBasicFilterAppendPositions<'a>, StorageError> {
        use open_bitcoin_core::chainstate::{
            IndexPrefix, filter_index::catch_up::BASIC_INDEX_MAX_CANDIDATES,
        };
        if self.preview_frozen
            || self.maybe_pending_reorg.is_some()
            || positions.last().map(|p| (p.height, p.block_hash)) != self.maybe_accepted
            || (proof.generation(), proof.branch_identity()) != (self.generation, self.branch)
            || count as u64 > BASIC_INDEX_MAX_CANDIDATES
        {
            return Err(receipt_error(
                "untracked BASIC replacement append positions",
            ));
        }
        if proof
            .maybe_replacement_target()
            .is_some_and(|(height, hash)| {
                positions
                    .get(height as usize)
                    .is_none_or(|position| position.height != height || position.block_hash != hash)
            })
        {
            return Err(receipt_error(
                "BASIC replacement target differs from accepted ancestry",
            ));
        }
        let work = self
            .store
            .check_basic_filter_append_proof(proof)?
            .checked_add(
                open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork {
                    checkpoint_operations: count as u64 * 6 + 4,
                    cloned_bytes: std::mem::size_of::<ValidatedBasicFilterAppendPositions<'_>>()
                        as u64,
                    ..Default::default()
                },
            )
            .map_err(|error| receipt_error(&error.to_string()))?;
        let first = match proof.processed().prefix() {
            IndexPrefix::Empty => 0,
            IndexPrefix::Committed(id) => id
                .height()
                .checked_add(1)
                .ok_or_else(|| receipt_error("BASIC replacement height exhausted"))?,
        } as usize;
        let end = first
            .checked_add(count)
            .ok_or_else(|| receipt_error("BASIC replacement count overflow"))?;
        let positions = positions
            .get(first..end)
            .ok_or_else(|| receipt_error("BASIC replacement exceeds accepted chain"))?;
        Ok(ValidatedBasicFilterAppendPositions {
            store: self.store.clone(),
            identity: (
                proof.generation(),
                proof.branch_identity(),
                proof.revision(),
            ),
            processed: proof.processed(),
            positions,
            work,
        })
    }
    pub(in crate::chainstate) fn authorize_reorg(
        &mut self,
        accepted: &AcceptedChainstateReorg,
        prepared: &PreparedBasicFilterReorg,
    ) -> Result<ValidatedBasicFilterReorg, StorageError> {
        if self.maybe_pending_reorg.is_some()
            || !prepared.belongs_to(&self.store)
            || self.maybe_accepted != prepared.maybe_old_endpoint()
            || (self.generation, self.branch)
                != (prepared.old_identity().0, prepared.old_identity().1)
            || accepted.maybe_old_endpoint() != self.maybe_accepted
            || accepted.maybe_new_endpoint() != prepared.maybe_new_endpoint()
            || accepted.maybe_common_ancestor_endpoint() != prepared.maybe_ancestor_endpoint()
        {
            return Err(receipt_error("untracked BASIC accepted reorg"));
        }
        let proof = self
            .store
            .maybe_basic_filter_reorg_proof_with_budget(PreparedBasicFilterReorg::maximum_work())?
            .ok_or_else(|| receipt_error("absent BASIC reorg proof"))?;
        if (
            proof.generation(),
            proof.branch_identity(),
            proof.revision(),
        ) != prepared.old_identity()
        {
            return Err(receipt_error("stale BASIC accepted reorg proof"));
        }
        self.maybe_accepted = accepted.maybe_new_endpoint();
        self.preview_frozen = false;
        self.maybe_pending_reorg = Some(PendingBasicFilterReorg::from_prepared(prepared));
        Ok(ValidatedBasicFilterReorg {
            store: self.store.clone(),
            facts: PendingBasicFilterReorg::from_prepared(prepared),
            work: proof.preparation_work(),
        })
    }

    pub(in crate::chainstate) fn confirm_reorg(
        &mut self,
        achieved: &CompletedBasicFilterReorg,
    ) -> Result<(), StorageError> {
        let prepared = achieved.prepared();
        let pending = self
            .maybe_pending_reorg
            .as_ref()
            .ok_or_else(|| receipt_error("absent BASIC pending reorg"))?;
        if !prepared.belongs_to(&self.store)
            || !pending.matches(prepared)
            || self.maybe_accepted != prepared.maybe_new_endpoint()
        {
            return Err(receipt_error("changed BASIC achieved reorg"));
        }
        let proof = self
            .store
            .maybe_basic_filter_append_proof_with_budget(PreparedBasicFilterReorg::maximum_work())?
            .ok_or_else(|| receipt_error("absent BASIC achieved proof"))?;
        if (
            proof.generation(),
            proof.branch_identity(),
            proof.revision(),
        ) != prepared.achieved_identity()
        {
            return Err(receipt_error("stale BASIC achieved reorg"));
        }
        self.generation = proof.generation();
        self.branch = proof.branch_identity();
        self.maybe_pending_reorg = None;
        Ok(())
    }
}
