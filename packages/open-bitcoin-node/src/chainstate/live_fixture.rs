// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp

//! Explicit fixture transfer of a genuine live owner, never raw recovery acceptance.

use super::{
    Chainstate, ChainstateError, FjallChainstateStore, HashMap, ManagedChainstate, map_persist,
};
use crate::storage::coins_view::FjallCoinsView;
use open_bitcoin_core::chainstate::CoinsView;

impl ManagedChainstate<FjallChainstateStore, FjallCoinsView> {
    /// Fixture-only enrichment after a real flush. Production metadata still
    /// stores unknown counts; these rows themselves carry no work authority.
    pub(crate) fn persist_missing_live_fixture_counts(&mut self) -> Result<(), ChainstateError> {
        self.chainstate.validate_live_origin()?;
        let store = self.store.inner().clone();
        if !self.chainstate.coins().parent().belongs_to(&store)
            || self.has_pending_validation_history()
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        let _guard = store.filter_publication_guard().map_err(map_persist)?;
        let current = self.chainstate.overlay_snapshot();
        let Some(counts) = current.maybe_confirmed_txid_counts.as_ref() else {
            return Err(ChainstateError::WorkRebindMismatch);
        };
        let (positions, maybe_saved_counts) =
            store.load_chain_meta_for_open().map_err(map_persist)?;
        let live_coins = self.chainstate.coins().complete_snapshot()?;
        let physical_coins = store.coins_view().complete_snapshot()?;
        if positions != current.active_chain
            || live_coins != physical_coins
            || !physical_coins.head_blocks.is_empty()
            || physical_coins.maybe_best_block != self.chainstate.tip().map(|tip| tip.block_hash)
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        if let Some(saved) = maybe_saved_counts {
            return if saved == *counts {
                Ok(())
            } else {
                Err(ChainstateError::WorkRebindMismatch)
            };
        }
        let bytes = crate::storage::snapshot_codec::encode_chain_meta(&positions, Some(counts))
            .map_err(map_persist)?;
        store
            .write_raw_for_test(crate::StorageNamespace::Chainstate, "chain_meta", bytes)
            .map_err(map_persist)
    }

    pub(crate) fn adopt_live_fixture_owner(
        &mut self,
        source: Chainstate,
    ) -> Result<(), ChainstateError> {
        let store = self.store.inner().clone();
        if !self.chainstate.coins().parent().belongs_to(&store)
            || self.has_pending_validation_history()
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        let _guard = store.filter_publication_guard().map_err(map_persist)?;
        let (positions, maybe_counts) = store.load_chain_meta_for_open().map_err(map_persist)?;
        let undo = store.load_all_undo_records().map_err(map_persist)?;
        let current = self.chainstate.overlay_snapshot();
        let empty_counts = positions.is_empty()
            && undo.is_empty()
            && maybe_counts.as_ref().is_none_or(HashMap::is_empty)
            && current
                .maybe_confirmed_txid_counts
                .as_ref()
                .is_none_or(HashMap::is_empty);
        if positions != current.active_chain
            || undo != current.undo_by_block
            || (!empty_counts && maybe_counts != current.maybe_confirmed_txid_counts)
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        source.rebind_into(&mut self.chainstate)
    }
}

#[cfg(test)]
impl ManagedChainstate<FjallChainstateStore, FjallCoinsView> {
    /// Attach BASIC to the same genuine live fixture after its actual full flush.
    /// No core source, body history, durable work or ordinary support is imported.
    #[cfg(test)]
    pub(crate) fn attach_fixture_basic_owner_after_flush(
        &mut self,
        params: open_bitcoin_core::consensus::ConsensusParams,
    ) -> Result<(), ChainstateError> {
        use open_bitcoin_core::chainstate::{
            IndexPrefix,
            filter_index::catch_up::{AcceptedIndexTarget, BasicIndexProgress},
        };
        self.chainstate.bind_live_work_source(params)?;
        let store = self.store.inner().clone();
        if !self.chainstate.coins().parent().belongs_to(&store)
            || self.has_pending_validation_history()
            || self.maybe_validated_lineage.is_some()
            || self.maybe_basic_index_owner.is_some()
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        let proof = store
            .maybe_basic_filter_append_proof()
            .map_err(map_persist)?
            .ok_or(ChainstateError::WorkRebindMismatch)?;
        let control = store.filter_publication_guard().map_err(map_persist)?;
        let current = self.chainstate.overlay_snapshot();
        let (positions, counts) = store.load_chain_meta_for_open().map_err(map_persist)?;
        let physical = store.coins_view().complete_snapshot()?;
        if positions != current.active_chain
            || counts != current.maybe_confirmed_txid_counts
            || store.load_all_undo_records().map_err(map_persist)? != current.undo_by_block
            || self.chainstate.coins().complete_snapshot()? != physical
            || !physical.head_blocks.is_empty()
            || physical.maybe_best_block != self.chainstate.tip().map(|tip| tip.block_hash)
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        let tip = self.chainstate.tip().ok_or(ChainstateError::MissingTip)?;
        let lineage = super::fjall_store::ValidatedChainstateLineage::from_fixture_flush_proof(
            &store, &proof, tip, &control,
        )
        .map_err(map_persist)?;
        let endpoint =
            |checkpoint: open_bitcoin_core::chainstate::FilterCheckpoint| match checkpoint.prefix()
            {
                IndexPrefix::Empty => None,
                IndexPrefix::Committed(identity) => Some(identity),
            };
        let progress = BasicIndexProgress::new(
            proof.generation(),
            proof.branch_identity(),
            AcceptedIndexTarget::new(tip.height, tip.block_hash),
            endpoint(proof.processed()),
            endpoint(proof.safe_checkpoint()),
            proof.protection(),
        )
        .map_err(|_| ChainstateError::WorkRebindMismatch)?;
        self.install_basic_index_owner(progress)
            .map_err(|_| ChainstateError::WorkRebindMismatch)?;
        self.maybe_validated_lineage = Some(lineage);
        Ok(())
    }
}
