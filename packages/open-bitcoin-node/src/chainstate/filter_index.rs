// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Accepted facts belong to the ordered owner, independently of later persistence.

use super::{ChainstateStore, ManagedChainstate};
use open_bitcoin_core::{
    chainstate::{
        BasicFilterInputs, BlockUndo, ChainPosition, CoinsView, FilterRecordIdentity,
        HistoricalBlockUndo, StagedChainstateConnect,
        filter_index::catch_up::{
            AcceptedIndexTarget, BasicIndexCatchUpError, BasicIndexPause, BasicIndexPreparedTurn,
            BasicIndexProgress, TurnWork,
        },
    },
    primitives::{Block, Transaction, TransactionInput, TransactionOutput},
};

impl ManagedChainstate<crate::FjallChainstateStore, crate::storage::FjallCoinsView> {
    /// Seal a bounded borrowed next-height range; public snapshots carry no authority.
    pub(crate) fn authorize_basic_filter_append_positions<'a>(
        &'a self,
        proof: &crate::storage::fjall_store::filters::BasicFilterAppendProof,
        records: &[crate::storage::filter_index::StoredFilterRecord],
    ) -> Result<super::fjall_store::ValidatedBasicFilterAppendPositions<'a>, crate::StorageError>
    {
        let lineage = self.maybe_validated_lineage.as_ref().ok_or_else(|| {
            crate::storage::filter_index::index_corruption("missing BASIC validated append lineage")
        })?;
        let positions = lineage.authorize_append_positions(
            proof,
            self.chainstate.active_chain(),
            records.len(),
        )?;
        proof.admit_work(positions.work())?;
        positions.validate_for(self.store.inner(), proof, records)?;
        Ok(positions)
    }
    /// Attach only to the sealed recovered manager and its genuine current store proof.
    pub(crate) fn initialize_basic_index_owner(
        &mut self,
        maximum_work: TurnWork,
    ) -> Result<(), crate::StorageError> {
        if self.maybe_validated_lineage.is_none() {
            return Ok(());
        }
        let Some(proof) = self
            .store
            .inner()
            .maybe_basic_filter_append_proof_with_budget(maximum_work)?
        else {
            return Err(crate::storage::filter_index::index_corruption(
                "missing recovered BASIC proof",
            ));
        };
        let tip = self.chainstate.tip().ok_or_else(|| {
            crate::storage::filter_index::index_corruption("missing recovered BASIC tip")
        })?;
        if proof.durable_tip() != (tip.height, tip.block_hash) {
            return Err(crate::storage::filter_index::index_corruption(
                "recovered BASIC tip differs",
            ));
        }
        let endpoint =
            |checkpoint: open_bitcoin_core::chainstate::FilterCheckpoint| match checkpoint.prefix()
            {
                open_bitcoin_core::chainstate::IndexPrefix::Empty => None,
                open_bitcoin_core::chainstate::IndexPrefix::Committed(id) => Some(id),
            };
        let progress = BasicIndexProgress::new(
            proof.generation(),
            proof.branch_identity(),
            AcceptedIndexTarget::new(tip.height, tip.block_hash),
            endpoint(proof.processed()),
            endpoint(proof.safe_checkpoint()),
            proof.protection(),
        )
        .map_err(crate::storage::filter_index::index_corruption)?;
        self.install_basic_index_owner(progress)
            .map_err(crate::storage::filter_index::index_corruption)
    }
}

/// Logical owned allocations, including container elements; never an RSS promise.
pub(crate) const MAX_ACCEPTED_FACT_BYTES: u64 = 384 * 1024 * 1024;
pub(crate) const MAX_ACCEPTED_FACT_ITEMS: u64 = 1_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AcceptedBasicIndexFailure {
    Persistence,
    RequiredBody,
    RequiredUndo,
    FactLimit,
    InvalidFacts,
    Invalidated,
}

pub(super) struct AcceptedBasicIndexOwner {
    pub(super) progress: BasicIndexProgress,
    pub(super) maybe_facts: Option<AcceptedBasicFilterFacts>,
    pub(super) maybe_failure: Option<AcceptedBasicIndexFailure>,
    pub(super) reorg_state: super::filter_reorg::BasicIndexReorgState,
}

/// One next-height block, with complete undo copied from genuine staging.
/// Private fields prevent substituting current coins or a same-shape fake undo.
pub(crate) struct AcceptedBasicFilterFacts {
    block: Block,
    undo: BlockUndo,
    position: ChainPosition,
    work: TurnWork,
}

impl AcceptedBasicFilterFacts {
    pub(super) fn capture_reorg(
        block: &Block,
        position: &ChainPosition,
        undo: &BlockUndo,
    ) -> Result<Self, AcceptedBasicIndexFailure> {
        let work = fact_work(block, undo)?;
        BasicFilterInputs::from_historical(
            block,
            position,
            Some(HistoricalBlockUndo {
                block_hash: position.block_hash,
                undo,
            }),
        )
        .map_err(|_| AcceptedBasicIndexFailure::InvalidFacts)?;
        Ok(Self {
            block: clone_basic_body(block),
            undo: undo.clone(),
            position: position.clone(),
            work,
        })
    }
    pub(crate) fn position(&self) -> &ChainPosition {
        &self.position
    }
    pub(crate) const fn work(&self) -> TurnWork {
        self.work
    }
    pub(crate) fn inputs(
        &self,
    ) -> Result<BasicFilterInputs<'_>, open_bitcoin_core::chainstate::BasicFilterInputError> {
        BasicFilterInputs::from_historical(
            &self.block,
            &self.position,
            Some(HistoricalBlockUndo {
                block_hash: self.position.block_hash,
                undo: &self.undo,
            }),
        )
    }

    fn capture(
        block: &Block,
        staged: &StagedChainstateConnect,
    ) -> Result<Self, AcceptedBasicIndexFailure> {
        let work = fact_work(block, &staged.undo)?;
        staged
            .basic_filter_inputs(block)
            .map_err(|_| AcceptedBasicIndexFailure::InvalidFacts)?;
        Ok(Self {
            block: clone_basic_body(block),
            undo: staged.undo.clone(),
            position: staged.position().clone(),
            work,
        })
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedChainstate<S, V> {
    #[cfg(test)]
    pub(crate) fn replace_basic_index_owner_for_test(
        &mut self,
        progress: BasicIndexProgress,
    ) -> Result<(), BasicIndexCatchUpError> {
        self.maybe_basic_index_owner = None;
        self.install_basic_index_owner(progress)
    }
    pub(crate) fn confirm_basic_index_checkpoint(
        &mut self,
        checkpoint: open_bitcoin_core::chainstate::FilterCheckpoint,
        protection: open_bitcoin_core::chainstate::IndexInputProtection,
    ) -> Result<(), BasicIndexCatchUpError> {
        let Some(owner) = self.maybe_basic_index_owner.as_mut() else {
            return Err(BasicIndexCatchUpError::NotActive);
        };
        owner.progress.confirm_achieved_checkpoint(
            owner.progress.generation(),
            owner.progress.branch_identity(),
            checkpoint,
            protection,
        )
    }
    pub(crate) fn resume_basic_index_owner(
        &mut self,
        generation: open_bitcoin_core::chainstate::filter_index::lifecycle::IndexGeneration,
        branch: open_bitcoin_core::primitives::BlockHash,
    ) -> Result<(), BasicIndexCatchUpError> {
        let Some(owner) = self.maybe_basic_index_owner.as_mut() else {
            return Err(BasicIndexCatchUpError::NotActive);
        };
        if owner.reorg_state == super::filter_reorg::BasicIndexReorgState::PreviewFrozen
            || self
                .maybe_validated_lineage
                .as_ref()
                .is_some_and(|lineage| lineage.reorg_is_pending())
        {
            return Err(BasicIndexCatchUpError::StaleWork);
        }
        owner.progress.resume(generation, branch)?;
        owner.maybe_failure = None;
        Ok(())
    }

    /// Installs pure progress only; this grants no store or durable-flush authority.
    pub(crate) fn install_basic_index_owner(
        &mut self,
        progress: BasicIndexProgress,
    ) -> Result<(), BasicIndexCatchUpError> {
        let Some(tip) = self.chainstate.tip() else {
            return Err(BasicIndexCatchUpError::InvalidProgress);
        };
        if self.maybe_basic_index_owner.is_some()
            || progress.accepted_target() != AcceptedIndexTarget::new(tip.height, tip.block_hash)
        {
            return Err(BasicIndexCatchUpError::InvalidProgress);
        }
        self.maybe_basic_index_owner = Some(AcceptedBasicIndexOwner {
            progress,
            maybe_facts: None,
            maybe_failure: None,
            reorg_state: super::filter_reorg::BasicIndexReorgState::Ordinary,
        });
        Ok(())
    }

    pub(crate) fn maybe_basic_index_progress(&self) -> Option<BasicIndexProgress> {
        self.maybe_basic_index_owner
            .as_ref()
            .map(|owner| owner.progress)
    }
    pub(crate) fn maybe_basic_index_failure(&self) -> Option<AcceptedBasicIndexFailure> {
        self.maybe_basic_index_owner
            .as_ref()
            .and_then(|owner| owner.maybe_failure)
    }
    /// Acceptance is visible independently of achieved publication and durable progress.
    pub(crate) fn maybe_basic_index_accepted_target(&self) -> Option<AcceptedIndexTarget> {
        let owner = self.maybe_basic_index_owner.as_ref()?;
        match owner.reorg_state {
            super::filter_reorg::BasicIndexReorgState::Ordinary
            | super::filter_reorg::BasicIndexReorgState::PreviewFrozen => {
                Some(owner.progress.accepted_target())
            }
            super::filter_reorg::BasicIndexReorgState::AcceptedReplacement { maybe_target }
            | super::filter_reorg::BasicIndexReorgState::DurablyFencedReplacement {
                maybe_target,
            } => maybe_target.map(|(height, hash)| AcceptedIndexTarget::new(height, hash)),
        }
    }
    pub(crate) fn maybe_accepted_basic_facts(&self) -> Option<&AcceptedBasicFilterFacts> {
        self.maybe_basic_index_owner
            .as_ref()
            .and_then(|owner| owner.maybe_facts.as_ref())
    }

    /// Apply achieved contiguous publication, then release only consumed live facts.
    pub(crate) fn complete_basic_index_turn(
        &mut self,
        prepared: BasicIndexPreparedTurn,
        records: &[FilterRecordIdentity],
    ) -> Result<(), BasicIndexCatchUpError> {
        let Some(owner) = self.maybe_basic_index_owner.as_mut() else {
            return Err(BasicIndexCatchUpError::NotActive);
        };
        owner.progress.complete_turn(prepared, records)?;
        if owner.maybe_facts.as_ref().is_some_and(|facts| {
            owner
                .progress
                .maybe_processed_endpoint()
                .is_some_and(|record| record.height() >= facts.position.height)
        }) {
            owner.maybe_facts = None;
        }
        Ok(())
    }

    pub(crate) fn note_basic_index_failure(&mut self, failure: AcceptedBasicIndexFailure) {
        let Some(owner) = self.maybe_basic_index_owner.as_mut() else {
            return;
        };
        owner.maybe_failure = Some(failure);
        owner.progress.pause(match failure {
            AcceptedBasicIndexFailure::Persistence => BasicIndexPause::Persistence,
            AcceptedBasicIndexFailure::Invalidated => BasicIndexPause::Invalidated,
            _ => BasicIndexPause::MissingHistory,
        });
    }

    pub(super) fn invalidate_basic_index_owner(&mut self) {
        self.note_basic_index_failure(AcceptedBasicIndexFailure::Invalidated);
        if let Some(owner) = self.maybe_basic_index_owner.as_mut() {
            owner.maybe_facts = None;
        }
    }

    pub(super) fn prepare_basic_filter_facts(
        &self,
        block: &Block,
        staged: &StagedChainstateConnect,
    ) -> Option<Result<AcceptedBasicFilterFacts, AcceptedBasicIndexFailure>> {
        let owner = self.maybe_basic_index_owner.as_ref()?;
        let maybe_next = owner
            .progress
            .maybe_processed_endpoint()
            .map_or(Some(0), |record| record.height().checked_add(1));
        if maybe_next != Some(staged.position().height) {
            return None;
        }
        Some(AcceptedBasicFilterFacts::capture(block, staged))
    }

    pub(super) fn observe_basic_filter_acceptance(
        &mut self,
        position: &ChainPosition,
        maybe_facts: Option<Result<AcceptedBasicFilterFacts, AcceptedBasicIndexFailure>>,
    ) {
        let Some(owner) = self.maybe_basic_index_owner.as_mut() else {
            return;
        };
        if matches!(
            owner.reorg_state,
            super::filter_reorg::BasicIndexReorgState::AcceptedReplacement { .. }
                | super::filter_reorg::BasicIndexReorgState::DurablyFencedReplacement { .. }
        ) {
            owner.reorg_state = super::filter_reorg::BasicIndexReorgState::AcceptedReplacement {
                maybe_target: Some((position.height, position.block_hash)),
            };
        }
        if owner
            .progress
            .observe_validated_connect(
                AcceptedIndexTarget::new(position.height, position.block_hash),
                position.previous_block_hash(),
            )
            .is_err()
        {
            self.invalidate_basic_index_owner();
            return;
        }
        match maybe_facts {
            Some(Ok(facts)) => owner.maybe_facts = Some(facts),
            Some(Err(failure)) => self.note_basic_index_failure(failure),
            None => {}
        }
    }
}

fn add_bytes(counter: &mut u64, bytes: usize) -> Result<(), AcceptedBasicIndexFailure> {
    *counter = counter
        .checked_add(bytes as u64)
        .ok_or(AcceptedBasicIndexFailure::FactLimit)?;
    if *counter > MAX_ACCEPTED_FACT_BYTES {
        return Err(AcceptedBasicIndexFailure::FactLimit);
    }
    Ok(())
}

/// Count before allocating clones, including witness and all restored input scripts.
fn fact_work(block: &Block, undo: &BlockUndo) -> Result<TurnWork, AcceptedBasicIndexFailure> {
    let mut work = TurnWork::default();
    add_bytes(&mut work.body_bytes, std::mem::size_of::<Block>())?;
    for tx in &block.transactions {
        add_bytes(&mut work.body_bytes, std::mem::size_of::<Transaction>())?;
        for input in &tx.inputs {
            add_bytes(
                &mut work.body_bytes,
                std::mem::size_of::<TransactionInput>(),
            )?;
            add_bytes(&mut work.body_bytes, input.script_sig.as_bytes().len())?;
        }
        for output in &tx.outputs {
            add_bytes(
                &mut work.body_bytes,
                std::mem::size_of::<TransactionOutput>(),
            )?;
            add_bytes(&mut work.body_bytes, output.script_pubkey.as_bytes().len())?;
            count_script(&mut work, output.script_pubkey.as_bytes().len())?;
        }
    }
    add_bytes(&mut work.undo_bytes, std::mem::size_of::<BlockUndo>())?;
    for tx in &undo.transactions {
        add_bytes(&mut work.undo_bytes, std::mem::size_of_val(tx))?;
        for coin in &tx.restored_inputs {
            add_bytes(&mut work.undo_bytes, std::mem::size_of_val(coin))?;
            add_bytes(
                &mut work.undo_bytes,
                coin.output.script_pubkey.as_bytes().len(),
            )?;
            count_script(&mut work, coin.output.script_pubkey.as_bytes().len())?;
        }
    }
    work.cloned_bytes = work
        .body_bytes
        .checked_add(work.undo_bytes)
        .and_then(|n| n.checked_add(std::mem::size_of::<ChainPosition>() as u64))
        .ok_or(AcceptedBasicIndexFailure::FactLimit)?;
    if work.cloned_bytes > MAX_ACCEPTED_FACT_BYTES {
        return Err(AcceptedBasicIndexFailure::FactLimit);
    }
    Ok(work)
}

/// Witnesses are already consensus validated and do not participate in txids,
/// header/body binding or BASIC inputs. Preserve every txid-bearing field.
fn clone_basic_body(block: &Block) -> Block {
    Block {
        header: block.header.clone(),
        transactions: block
            .transactions
            .iter()
            .map(|tx| Transaction {
                version: tx.version,
                inputs: tx
                    .inputs
                    .iter()
                    .map(|input| TransactionInput {
                        previous_output: input.previous_output.clone(),
                        script_sig: input.script_sig.clone(),
                        sequence: input.sequence,
                        witness: Default::default(),
                    })
                    .collect(),
                outputs: tx.outputs.clone(),
                lock_time: tx.lock_time,
            })
            .collect(),
    }
}

pub(crate) fn borrowed_undo_work(
    maybe_undo: Option<&BlockUndo>,
) -> Result<TurnWork, crate::StorageError> {
    let mut work = TurnWork::default();
    if let Some(undo) = maybe_undo {
        add_bytes(&mut work.undo_bytes, std::mem::size_of::<BlockUndo>())
            .map_err(|_| crate::storage::filter_index::index_corruption("BASIC undo bound"))?;
        for tx in &undo.transactions {
            add_bytes(&mut work.undo_bytes, std::mem::size_of_val(tx))
                .map_err(|_| crate::storage::filter_index::index_corruption("BASIC undo bound"))?;
            for coin in &tx.restored_inputs {
                add_bytes(&mut work.undo_bytes, std::mem::size_of_val(coin)).map_err(|_| {
                    crate::storage::filter_index::index_corruption("BASIC undo bound")
                })?;
                let len = coin.output.script_pubkey.as_bytes().len();
                add_bytes(&mut work.undo_bytes, len).map_err(|_| {
                    crate::storage::filter_index::index_corruption("BASIC undo bound")
                })?;
                count_script(&mut work, len).map_err(|_| {
                    crate::storage::filter_index::index_corruption("BASIC undo bound")
                })?;
            }
        }
    }
    Ok(work)
}

fn count_script(work: &mut TurnWork, len: usize) -> Result<(), AcceptedBasicIndexFailure> {
    work.script_items = work
        .script_items
        .checked_add(1)
        .ok_or(AcceptedBasicIndexFailure::FactLimit)?;
    if work.script_items > MAX_ACCEPTED_FACT_ITEMS {
        return Err(AcceptedBasicIndexFailure::FactLimit);
    }
    add_bytes(&mut work.script_bytes, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase157_accepted_fact_bytes_refuse_exact_one_over_before_clone() {
        // Arrange
        let mut bytes = MAX_ACCEPTED_FACT_BYTES - 1;
        // Act
        add_bytes(&mut bytes, 1).expect("exact ceiling");
        let result = add_bytes(&mut bytes, 1);
        // Assert
        assert_eq!(result, Err(AcceptedBasicIndexFailure::FactLimit));
    }

    #[test]
    fn phase157_accepted_fact_items_refuse_exact_one_over_before_clone() {
        // Arrange
        let mut work = TurnWork {
            script_items: MAX_ACCEPTED_FACT_ITEMS - 1,
            ..TurnWork::default()
        };
        // Act
        count_script(&mut work, 1).expect("exact ceiling");
        let result = count_script(&mut work, 1);
        // Assert
        assert_eq!(result, Err(AcceptedBasicIndexFailure::FactLimit));
    }

    #[test]
    fn phase157_accepted_fact_bytes_refuse_checked_overflow() {
        // Arrange
        let mut bytes = u64::MAX;
        // Act
        let result = add_bytes(&mut bytes, 1);
        // Assert
        assert_eq!(result, Err(AcceptedBasicIndexFailure::FactLimit));
    }
}
