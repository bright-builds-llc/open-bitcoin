// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp

//! Only genuine managed absorption can seal retained validation facts.

#[cfg(test)]
pub(crate) mod tests;

use super::*;
use crate::storage::validation_history::{
    AcceptedValidationBatch, BlockValidationIdentity, ValidationProvenance, history_corruption,
    validate_batch_size,
};

/// Private-field evidence minted only after the actual managed absorption.
pub(crate) struct AcceptedValidationReceipt {
    store: crate::FjallNodeStore,
    identities: Vec<BlockValidationIdentity>,
}

impl AcceptedValidationReceipt {
    pub(crate) fn into_parts(self) -> (crate::FjallNodeStore, Vec<BlockValidationIdentity>) {
        (self.store, self.identities)
    }
}

pub(super) struct PreparedValidationAcceptance {
    store: crate::FjallNodeStore,
    identities: Vec<BlockValidationIdentity>,
    maybe_reorg_lineage: Option<ReorgValidationLineage>,
}

#[derive(PartialEq, Eq)]
struct ReorgValidationLineage {
    maybe_old: Option<(u32, BlockHash)>,
    maybe_ancestor: Option<(u32, BlockHash)>,
}

impl PreparedValidationAcceptance {
    fn seal_connect(self, accepted: &ChainPosition) -> AcceptedValidationReceipt {
        assert!(
            self.identities.len() == 1
                && self.identities[0].hash() == accepted.block_hash
                && self.identities[0].height() == accepted.height
                && self.identities[0].parent_hash() == accepted.previous_block_hash(),
            "genuine connect receipt must match its own stage"
        );
        AcceptedValidationReceipt {
            store: self.store,
            identities: self.identities,
        }
    }

    fn seal_reorg(
        self,
        accepted: &open_bitcoin_core::chainstate::AcceptedChainstateReorg,
        transition: &ChainTransition,
    ) -> AcceptedValidationReceipt {
        assert!(
            self.maybe_reorg_lineage
                == Some(ReorgValidationLineage {
                    maybe_old: accepted.maybe_old_endpoint(),
                    maybe_ancestor: accepted.maybe_common_ancestor_endpoint(),
                })
                && self.identities.last().map(|id| (id.height(), id.hash()))
                    == accepted.maybe_new_endpoint()
                && self.identities.len() == transition.connected.len()
                && self
                    .identities
                    .iter()
                    .zip(&transition.connected)
                    .all(|(id, position)| id.hash() == position.block_hash
                        && id.height() == position.height
                        && id.parent_hash() == position.previous_block_hash()),
            "genuine reorg receipt must match every position in its own stage"
        );
        AcceptedValidationReceipt {
            store: self.store,
            identities: self.identities,
        }
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedChainstate<S, V> {
    pub(super) fn admit_validation_acceptance(
        &self,
        positions: &[ChainPosition],
    ) -> Result<Option<PreparedValidationAcceptance>, ChainstateError> {
        self.check_validation_acceptance_available()
            .map_err(map_persist)?;
        let Some(store) = self.store.maybe_validation_history_store() else {
            return Ok(None);
        };
        if positions.is_empty() {
            return Ok(None);
        }
        validate_batch_size(positions.len()).map_err(map_persist)?;
        let identities = positions
            .iter()
            .map(|position| {
                BlockValidationIdentity::new(
                    position.block_hash,
                    position.previous_block_hash(),
                    position.height,
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(map_persist)?;
        Ok(Some(PreparedValidationAcceptance {
            store,
            identities,
            maybe_reorg_lineage: None,
        }))
    }

    pub(super) fn prepare_validation_reorg(
        &self,
        staged: &StagedChainstateReorg,
    ) -> Result<Option<PreparedValidationAcceptance>, ChainstateError> {
        Ok(self
            .admit_validation_acceptance(&staged.transition().connected)?
            .map(|mut history| {
                history.maybe_reorg_lineage = Some(ReorgValidationLineage {
                    maybe_old: staged.maybe_old_tip().map(|p| (p.height, p.block_hash)),
                    maybe_ancestor: staged
                        .maybe_common_ancestor()
                        .map(|p| (p.height, p.block_hash)),
                });
                history
            }))
    }

    fn check_validation_acceptance_available(&self) -> Result<(), StorageError> {
        if self.has_pending_validation_history() {
            return Err(history_corruption(
                "unresolved accepted validation publication",
            ));
        }
        Ok(())
    }

    fn observe_absorbed_validation(&mut self, maybe_receipt: Option<AcceptedValidationReceipt>) {
        self.maybe_pending_validation = maybe_receipt.map(AcceptedValidationBatch::from_absorbed);
    }

    fn publish_pending_validation_history(&mut self) -> Result<(), StorageError> {
        let Some(pending) = self.maybe_pending_validation.as_ref() else {
            return Ok(());
        };
        let store = self
            .store
            .maybe_validation_history_store()
            .ok_or_else(|| history_corruption("lost accepted validation store"))?;
        store.publish_validation_history(pending)?;
        self.maybe_pending_validation = None;
        Ok(())
    }

    pub(crate) fn validation_provenance(
        &self,
        hash: BlockHash,
    ) -> Result<ValidationProvenance, StorageError> {
        if self
            .maybe_pending_validation
            .as_ref()
            .is_some_and(|pending| pending.identities().iter().any(|id| id.hash() == hash))
        {
            return Ok(ValidationProvenance::ScriptsValid);
        }
        match self.store.maybe_validation_history_store() {
            Some(store) => store.validation_provenance(hash),
            None => Ok(ValidationProvenance::UnknownLegacy),
        }
    }

    pub(crate) fn has_pending_validation_history(&self) -> bool {
        self.maybe_pending_validation.is_some()
    }

    /// Read only the sealed live acceptance identity, even before durable rows
    /// and trusted header publication exist. Bodies/filters cannot supply it.
    pub(crate) fn maybe_pending_validation_identity(
        &self,
        hash: BlockHash,
    ) -> Option<BlockValidationIdentity> {
        self.maybe_pending_validation
            .as_ref()?
            .identities()
            .iter()
            .find(|identity| identity.hash() == hash)
            .copied()
    }

    pub(crate) fn commit_prepared_connect(
        &mut self,
        prepared: PreparedChainstateConnect,
    ) -> Result<ChainPosition, open_bitcoin_core::chainstate::ChainstateError> {
        // D-18: absorb stays infallible. Persist after absorb can fail; mempool
        // still applies the patch when this closure returns Err.
        self.check_validation_acceptance_available()
            .map_err(map_persist)?;
        let position = self.chainstate.absorb_staged_connect(prepared.staged);
        self.observe_absorbed_validation(
            prepared
                .maybe_history
                .map(|history| history.seal_connect(&position)),
        );
        self.observe_validated_lineage(&position);
        self.observe_basic_filter_acceptance(&position, prepared.maybe_filter_facts);
        if let Err(error) = self.publish_pending_validation_history() {
            self.note_basic_index_failure(filter_index::AcceptedBasicIndexFailure::Persistence);
            return Err(map_persist(error));
        }
        if let Err(error) = self.persist() {
            self.note_basic_index_failure(filter_index::AcceptedBasicIndexFailure::Persistence);
            return Err(map_persist(error));
        }
        if self.maybe_basic_index_failure().is_some_and(|failure| {
            matches!(
                failure,
                filter_index::AcceptedBasicIndexFailure::FactLimit
                    | filter_index::AcceptedBasicIndexFailure::InvalidFacts
            )
        }) {
            return Err(ChainstateError::CoinsStorage {
                detail: "accepted block; BASIC index input capture paused".to_owned(),
            });
        }
        Ok(position)
    }

    pub(crate) fn commit_prepared_reorg(
        &mut self,
        prepared: PreparedChainstateReorg,
    ) -> Result<ChainTransition, open_bitcoin_core::chainstate::ChainstateError> {
        self.check_validation_acceptance_available()
            .map_err(map_persist)?;
        self.check_basic_index_reorg(&prepared)
            .map_err(map_persist)?;
        let (transition, accepted) = self
            .chainstate
            .absorb_staged_reorg_with_receipt(prepared.staged);
        self.observe_absorbed_validation(
            prepared
                .maybe_history
                .map(|history| history.seal_reorg(&accepted, &transition)),
        );
        let history_result = self.publish_pending_validation_history();
        if let Err(error) = self.accept_basic_index_reorg(accepted, prepared.maybe_index) {
            self.note_basic_index_failure(filter_index::AcceptedBasicIndexFailure::Persistence);
            return Err(map_persist(error));
        }
        if let Err(error) = history_result {
            self.note_basic_index_failure(filter_index::AcceptedBasicIndexFailure::Persistence);
            return Err(map_persist(error));
        }
        if let Err(error) = self.persist() {
            self.note_basic_index_failure(filter_index::AcceptedBasicIndexFailure::Persistence);
            return Err(map_persist(error));
        }
        Ok(transition)
    }

    pub(super) fn flush_window(
        &self,
    ) -> (Vec<(BlockHash, BlockUndo)>, Vec<Block>, Vec<ChainPosition>) {
        let undo_window = self
            .chainstate
            .undo_by_block()
            .iter()
            .map(|(block_hash, undo)| (*block_hash, undo.clone()))
            .collect();
        let active_chain = self.chainstate.active_chain().to_vec();
        (undo_window, Vec::new(), active_chain)
    }
}
