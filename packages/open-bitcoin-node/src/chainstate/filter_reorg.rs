// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::filter_index::AcceptedBasicFilterFacts;
use super::*;
pub(crate) mod preflight;
use crate::storage::fjall_store::filters::PreparedBasicFilterReorg;
use open_bitcoin_core::chainstate::filter_index::catch_up::{
    AcceptedIndexTarget, BasicIndexProgress, BasicIndexReplacementFacts,
};
use open_bitcoin_core::chainstate::{AcceptedChainstateReorg, FilterCheckpoint, IndexPrefix};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BasicIndexReorgState {
    Ordinary,
    PreviewFrozen,
    AcceptedReplacement {
        maybe_target: Option<(u32, BlockHash)>,
    },
    DurablyFencedReplacement {
        maybe_target: Option<(u32, BlockHash)>,
    },
}

pub(super) struct PreparedIndexReorg {
    storage: PreparedBasicFilterReorg,
    maybe_trial: Option<BasicIndexProgress>,
    maybe_facts: Option<AcceptedBasicFilterFacts>,
    work: open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork,
    #[cfg(test)]
    component_work: [open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork; 3],
}

impl PreparedIndexReorg {
    #[cfg(test)]
    pub(super) fn work(&self) -> open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork {
        self.work
    }
    #[cfg(test)]
    pub(super) fn component_work(
        &self,
    ) -> [open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork; 4] {
        [
            self.component_work[0],
            self.component_work[1],
            self.component_work[2],
            self.work,
        ]
    }
    fn validate_work_bound(&self) -> Result<(), StorageError> {
        use open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork;
        let budget = preflight::production_budget()?;
        let source = TurnWork {
            blocks: budget.normal().blocks,
            ..budget.absolute_singleton()
        };
        let capture = TurnWork {
            body_bytes: super::filter_index::MAX_ACCEPTED_FACT_BYTES,
            undo_bytes: super::filter_index::MAX_ACCEPTED_FACT_BYTES,
            cloned_bytes: super::filter_index::MAX_ACCEPTED_FACT_BYTES,
            script_items: super::filter_index::MAX_ACCEPTED_FACT_ITEMS,
            script_bytes: super::filter_index::MAX_ACCEPTED_FACT_BYTES,
            ..Default::default()
        };
        let maximum = PreparedBasicFilterReorg::maximum_work()
            .checked_add(source)
            .and_then(|work| work.checked_add(capture))
            .map_err(crate::storage::filter_index::index_corruption)?;
        if !self.work.fits(maximum) {
            return Err(crate::storage::filter_index::index_corruption(
                "BASIC composed reorg preparation budget",
            ));
        }
        Ok(())
    }
}

fn maybe_endpoint(
    checkpoint: FilterCheckpoint,
) -> Option<open_bitcoin_core::chainstate::FilterRecordIdentity> {
    match checkpoint.prefix() {
        IndexPrefix::Empty => None,
        IndexPrefix::Committed(id) => Some(id),
    }
}

#[cfg(test)]
impl PreparedChainstateReorg {
    pub(crate) fn maybe_basic_filter_component_work(
        &self,
    ) -> Option<[open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork; 4]> {
        self.maybe_index
            .as_ref()
            .map(PreparedIndexReorg::component_work)
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedChainstate<S, V> {
    pub(super) fn prepare_basic_index_reorg(
        &self,
        staged: &StagedChainstateReorg,
        replacements: &[AnchoredBlock],
    ) -> Result<Option<PreparedIndexReorg>, ChainstateError> {
        let Some(lineage) = self.maybe_validated_lineage.as_ref() else {
            return Ok(None);
        };
        let storage = lineage.prepare_reorg_storage(staged).map_err(map_persist)?;
        let preflight_work = lineage
            .preflight_required_sources(
                staged,
                replacements,
                self.chainstate.active_chain(),
                self.chainstate.undo_by_block(),
                storage.processed(),
            )
            .map_err(map_persist)?;
        let maybe_trial = self
            .maybe_basic_index_owner
            .as_ref()
            .map(|owner| {
                let Some((height, hash)) = storage.maybe_new_endpoint() else {
                    return Ok(None);
                };
                owner
                    .progress
                    .replace_validated_branch(BasicIndexReplacementFacts {
                        expected_generation: storage.old_identity().0,
                        expected_branch_identity: storage.old_identity().1,
                        old_target: owner.progress.accepted_target(),
                        new_target: AcceptedIndexTarget::new(height, hash),
                        maybe_common_ancestor: storage
                            .maybe_ancestor_endpoint()
                            .map(|(h, b)| AcceptedIndexTarget::new(h, b)),
                        maybe_indexed_common: maybe_endpoint(storage.processed()),
                        maybe_shared_safe: maybe_endpoint(storage.safe_checkpoint()),
                        achieved_generation: storage.achieved_identity().0,
                        achieved_branch_identity: storage.achieved_identity().1,
                        protection: storage.protection(),
                    })
                    .map(Some)
                    .map_err(crate::storage::filter_index::index_corruption)
            })
            .transpose()
            .map_err(map_persist)?
            .flatten();
        let maybe_next =
            maybe_endpoint(storage.processed()).map_or(Some(0), |id| id.height().checked_add(1));
        let maybe_first_replacement = staged
            .maybe_common_ancestor()
            .map_or(Some(0), |ancestor| ancestor.height.checked_add(1));
        let maybe_replacement = maybe_next
            .zip(maybe_first_replacement)
            .and_then(|(height, first)| height.checked_sub(first).map(|offset| (height, offset)))
            .and_then(|(height, offset)| {
                Some((
                    replacements.get(usize::try_from(offset).ok()?)?,
                    staged.maybe_position_at_height(height)?,
                ))
            });
        let maybe_facts = if self.maybe_basic_index_owner.is_some() {
            maybe_replacement
                .map(|(anchored, position)| {
                    let undo = staged.maybe_replacement_undo(position.block_hash).ok_or(
                        ChainstateError::CoinsStorage {
                            detail: "missing genuine BASIC replacement undo".to_owned(),
                        },
                    )?;
                    AcceptedBasicFilterFacts::capture_reorg(&anchored.block, position, undo)
                        .map_err(|failure| ChainstateError::CoinsStorage {
                            detail: format!("BASIC replacement capture refused: {failure:?}"),
                        })
                })
                .transpose()?
        } else {
            None
        };
        let work = storage
            .work()
            .checked_add(preflight_work)
            .and_then(|work| {
                work.checked_add(
                    maybe_facts
                        .as_ref()
                        .map_or(Default::default(), AcceptedBasicFilterFacts::work),
                )
            })
            .map_err(|error| map_persist(crate::storage::filter_index::index_corruption(error)))?;
        Ok(Some(PreparedIndexReorg {
            #[cfg(test)]
            component_work: [
                storage.work(),
                preflight_work,
                maybe_facts
                    .as_ref()
                    .map_or(Default::default(), AcceptedBasicFilterFacts::work),
            ],
            storage,
            maybe_trial,
            maybe_facts,
            work,
        }))
    }

    pub(super) fn freeze_basic_index_reorg(
        &mut self,
        prepared: &PreparedChainstateReorg,
    ) -> Result<(), StorageError> {
        if let Some(index) = &prepared.maybe_index {
            index.validate_work_bound()?;
            let lineage = self.maybe_validated_lineage.as_mut().ok_or_else(|| {
                crate::storage::filter_index::index_corruption("lost BASIC preview lineage")
            })?;
            lineage.freeze_index_reorg(&index.storage)?;
        } else {
            self.maybe_validated_lineage = None;
        }
        self.invalidate_basic_index_owner();
        if let Some(owner) = &mut self.maybe_basic_index_owner {
            owner.reorg_state = BasicIndexReorgState::PreviewFrozen;
        }
        Ok(())
    }

    pub(super) fn check_basic_index_reorg(
        &self,
        prepared: &PreparedChainstateReorg,
    ) -> Result<(), StorageError> {
        if let Some(index) = &prepared.maybe_index {
            index.validate_work_bound()?;
            self.maybe_validated_lineage
                .as_ref()
                .ok_or_else(|| {
                    crate::storage::filter_index::index_corruption("lost BASIC prepared lineage")
                })?
                .check_index_reorg(&index.storage)?;
        }
        Ok(())
    }

    pub(super) fn accept_basic_index_reorg(
        &mut self,
        accepted: AcceptedChainstateReorg,
        maybe_index: Option<PreparedIndexReorg>,
    ) -> Result<(), StorageError> {
        self.invalidate_basic_index_owner();
        if let Some(owner) = &mut self.maybe_basic_index_owner {
            owner.reorg_state = BasicIndexReorgState::AcceptedReplacement {
                maybe_target: accepted.maybe_new_endpoint(),
            };
        }
        let Some(index) = maybe_index else {
            self.maybe_validated_lineage = None;
            return Ok(());
        };
        let lineage = self.maybe_validated_lineage.as_mut().ok_or_else(|| {
            crate::storage::filter_index::index_corruption("lost BASIC accepted lineage")
        })?;
        lineage.complete_index_reorg(&accepted, index.storage)?;
        if let Some(owner) = &mut self.maybe_basic_index_owner {
            if let Some(trial) = index.maybe_trial {
                owner.progress = trial;
                owner.maybe_failure = None;
            }
            owner.maybe_facts = index.maybe_facts;
        }
        Ok(())
    }

    pub(super) fn note_basic_index_reorg_durable(&mut self) {
        if let Some(owner) = &mut self.maybe_basic_index_owner
            && let BasicIndexReorgState::AcceptedReplacement { maybe_target } = owner.reorg_state
            && maybe_target == self.chainstate.tip().map(|p| (p.height, p.block_hash))
        {
            owner.reorg_state = BasicIndexReorgState::DurablyFencedReplacement { maybe_target };
        }
    }
}

#[cfg(test)]
mod tests;
