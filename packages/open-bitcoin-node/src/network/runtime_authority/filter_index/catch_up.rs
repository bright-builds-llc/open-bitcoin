// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! The single ordinary ordered turn; recovery and full flush stay outside it.

use super::*;
use crate::storage::filter_index::StoredFilterRecord;
use open_bitcoin_core::chainstate::{
    BasicFilterInputs, HistoricalBlockUndo, IndexPrefix,
    filter_index::catch_up::{BasicIndexState, BasicIndexTurnBudget, TurnWork},
};
use std::time::{Duration, Instant};
mod inputs;
#[cfg(test)]
mod tests;

/// Achieved progress and reserved logical work; timings include storage latency.
#[derive(Debug, Clone, Copy)]
pub struct BasicFilterTurnOutcome {
    pub work: TurnWork,
    pub batch_bytes: u64,
    pub persistence_batches: u64,
    pub body_reads: u64,
    pub body_decodes: u64,
    pub undo_borrows: u64,
    pub generations: u64,
    pub reused_records: u64,
    pub authority_hold: Duration,
    pub storage_elapsed: Duration,
    pub maybe_progress:
        Option<open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexProgress>,
    pub maybe_accepted_target:
        Option<open_bitcoin_core::chainstate::filter_index::catch_up::AcceptedIndexTarget>,
    pub maybe_accepted_lag: Option<u64>,
    pub oversized_singleton: bool,
    pub examined_script_items: u64,
    pub examined_script_bytes: u64,
    pub hashed_items: u64,
    pub sorted_items: u64,
    #[cfg(test)]
    pub indexed_point_reads: usize,
}

pub(super) fn production_budget() -> Result<BasicIndexTurnBudget, StorageError> {
    crate::chainstate::basic_filter_turn_budget()
}

impl ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView> {
    #[cfg(test)]
    pub(crate) fn last_basic_filter_turn_for_test(&self) -> BasicFilterTurnOutcome {
        self.maybe_last_basic_filter_turn
            .lock()
            .expect("test observation")
            .expect("actual turn")
    }
    pub(crate) fn initialize_basic_filter_index_owner(
        &self,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        let budget = production_budget().map_err(lifecycle_error)?;
        self.mutate(|network| {
            network
                .chainstate_mut()
                .initialize_basic_index_owner(budget.normal())
        })?
        .map_err(lifecycle_error)
    }

    /// Advance one bounded consecutive turn without caller-selected heights or fences.
    pub fn drive_basic_filter_index_turn(
        &self,
    ) -> Result<BasicFilterTurnOutcome, ManagedNetworkAuthorityError> {
        self.drive_basic_filter_index_turn_with_budget(
            production_budget().map_err(lifecycle_error)?,
        )
    }

    pub(crate) fn drive_basic_filter_index_turn_with_budget(
        &self,
        budget: BasicIndexTurnBudget,
    ) -> Result<BasicFilterTurnOutcome, ManagedNetworkAuthorityError> {
        let outcome = self
            .mutate(|network| {
                let start = Instant::now();
                let manager = network.chainstate_mut();
                #[cfg(test)]
                let reads_before = manager.store().inner().basic_filter_point_reads_for_test();
                let mut failure =
                    crate::chainstate::filter_index::AcceptedBasicIndexFailure::InvalidFacts;
                let result = drive_turn(manager, budget, &mut failure);
                if result.is_err() {
                    manager.note_basic_index_failure(failure);
                }
                result.map(|mut outcome| {
                    outcome.authority_hold = start.elapsed();
                    #[cfg(test)]
                    {
                        outcome.indexed_point_reads = manager
                            .store()
                            .inner()
                            .basic_filter_point_reads_for_test()
                            .checked_sub(reads_before)
                            .expect("no turn-side full-scan reset");
                    }
                    outcome
                })
            })?
            .map_err(lifecycle_error)?;
        #[cfg(test)]
        {
            *self
                .maybe_last_basic_filter_turn
                .lock()
                .map_err(|_| ManagedNetworkAuthorityError::Poisoned)? = Some(outcome);
        }
        Ok(outcome)
    }
}

fn drive_turn(
    manager: &mut crate::ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
    budget: BasicIndexTurnBudget,
    failure: &mut crate::chainstate::filter_index::AcceptedBasicIndexFailure,
) -> Result<BasicFilterTurnOutcome, StorageError> {
    *failure = match manager.maybe_basic_index_failure() {
        Some(crate::chainstate::filter_index::AcceptedBasicIndexFailure::Persistence) => {
            crate::chainstate::filter_index::AcceptedBasicIndexFailure::Persistence
        }
        _ => crate::chainstate::filter_index::AcceptedBasicIndexFailure::Invalidated,
    };
    let mut outcome = BasicFilterTurnOutcome {
        work: TurnWork::default(),
        batch_bytes: 0,
        persistence_batches: 0,
        body_reads: 0,
        body_decodes: 0,
        undo_borrows: 0,
        generations: 0,
        reused_records: 0,
        authority_hold: Duration::ZERO,
        storage_elapsed: Duration::ZERO,
        maybe_progress: manager.maybe_basic_index_progress(),
        maybe_accepted_target: manager.maybe_basic_index_accepted_target(),
        maybe_accepted_lag: None,
        oversized_singleton: false,
        examined_script_items: 0,
        examined_script_bytes: 0,
        hashed_items: 0,
        sorted_items: 0,
        #[cfg(test)]
        indexed_point_reads: 0,
    };
    let Some(progress) = outcome.maybe_progress else {
        return Ok(outcome);
    };
    if progress.state() == BasicIndexState::Disabled {
        return Ok(outcome);
    }
    let Some(accepted) = outcome.maybe_accepted_target else {
        return Ok(outcome);
    };
    outcome.maybe_accepted_lag = Some(
        u64::from(accepted.height()) + 1
            - progress
                .maybe_processed_endpoint()
                .map_or(0, |id| u64::from(id.height()) + 1)
                .min(u64::from(accepted.height()) + 1),
    );
    if accepted != progress.accepted_target() {
        return Err(index_corruption(
            "BASIC accepted target has pending publication",
        ));
    }
    let store = manager.store().inner().clone();
    let Some(proof) =
        store.maybe_basic_filter_append_proof_with_budget(inputs::acquisition_limit(budget))?
    else {
        return Err(index_corruption("BASIC turn authority unavailable"));
    };
    let endpoint = |cp: open_bitcoin_core::chainstate::FilterCheckpoint| match cp.prefix() {
        IndexPrefix::Empty => None,
        IndexPrefix::Committed(id) => Some(id),
    };
    if proof.generation() != progress.generation()
        || proof.branch_identity() != progress.branch_identity()
        || endpoint(proof.processed()) != progress.maybe_processed_endpoint()
        || endpoint(proof.safe_checkpoint()) != progress.maybe_safe_durable_endpoint()
    {
        return Err(index_corruption("stale BASIC ordered owner"));
    }
    let captured_work = proof.preparation_work();
    *failure = crate::chainstate::filter_index::AcceptedBasicIndexFailure::InvalidFacts;
    let GeneratedRecords {
        records,
        work: total_work,
        maximum,
    } = generate_records(
        manager,
        progress,
        budget,
        &mut outcome,
        failure,
        captured_work,
    )?;
    let mut work = subtract(total_work, captured_work)?;
    // Blocks and encoded output were reserved before generation. The adapter
    // counts their actual selected/output totals once, so do not merge twice.
    work.blocks = 0;
    work.encoded_bytes = 0;
    work = work
        .checked_add(TurnWork {
            cloned_bytes: records.len() as u64
                * std::mem::size_of::<open_bitcoin_core::chainstate::FilterRecordIdentity>() as u64,
            ..TurnWork::default()
        })
        .map_err(index_corruption)?;
    let remaining = subtract(maximum, work)?;
    let proof = proof.with_remaining_budget(remaining)?;
    // Resume preserves accepted state and retained facts; no recovery or raw reseed.
    if manager.maybe_basic_index_failure().is_some() {
        manager
            .resume_basic_index_owner(proof.generation(), proof.branch_identity())
            .map_err(index_corruption)?;
    }
    let current = manager
        .maybe_basic_index_progress()
        .ok_or_else(|| index_corruption("missing BASIC owner"))?;
    let prepared_progress = current.prepare_turn().map_err(index_corruption)?;
    let identities: Vec<_> = records.iter().map(StoredFilterRecord::identity).collect();
    let mut trial = current;
    if !identities.is_empty() {
        trial
            .complete_turn(prepared_progress, &identities)
            .map_err(index_corruption)?;
    }
    let prepared = if records.is_empty() {
        // Checkpoint-only promotion has no forward projection; the actual own
        // coins/metadata fence remains required by the shared append adapter.
        store.prepare_basic_filter_append(proof, &records)?
    } else {
        let positions = manager.authorize_basic_filter_append_positions(&proof, &records)?;
        store.prepare_basic_filter_replacement_append(proof, positions, &records)?
    };
    *failure = crate::chainstate::filter_index::AcceptedBasicIndexFailure::Persistence;
    let storage_start = Instant::now();
    let achieved = store.complete_basic_filter_append(prepared)?;
    outcome.storage_elapsed = storage_start.elapsed();
    if endpoint(achieved.processed) != trial.maybe_processed_endpoint() {
        return Err(index_corruption(
            "BASIC achieved processed endpoint differs",
        ));
    }
    if !identities.is_empty() {
        manager
            .complete_basic_index_turn(prepared_progress, &identities)
            .map_err(index_corruption)?;
    }
    manager
        .confirm_basic_index_checkpoint(achieved.safe_checkpoint, achieved.protection)
        .map_err(index_corruption)?;
    outcome.work = work.checked_add(achieved.work).map_err(index_corruption)?;
    outcome.batch_bytes = achieved.batch_bytes;
    outcome.persistence_batches = u64::from(achieved.batch_bytes != 0);
    outcome.maybe_progress = manager.maybe_basic_index_progress();
    outcome.maybe_accepted_lag = outcome.maybe_progress.map(|progress| {
        u64::from(accepted.height()) + 1
            - progress
                .maybe_processed_endpoint()
                .map_or(0, |id| u64::from(id.height()) + 1)
                .min(u64::from(accepted.height()) + 1)
    });
    Ok(outcome)
}

struct GeneratedRecords {
    records: Vec<StoredFilterRecord>,
    work: TurnWork,
    maximum: TurnWork,
}

fn generate_records(
    manager: &crate::ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
    progress: open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexProgress,
    budget: BasicIndexTurnBudget,
    outcome: &mut BasicFilterTurnOutcome,
    failure: &mut crate::chainstate::filter_index::AcceptedBasicIndexFailure,
    captured_work: TurnWork,
) -> Result<GeneratedRecords, StorageError> {
    let store = manager.store().inner();
    let mut records = Vec::new();
    let mut work = captured_work;
    let mut maybe_previous = progress.maybe_processed_endpoint();
    let mut maximum = budget.normal();
    if !work.fits(maximum) {
        return Err(index_corruption("BASIC fixed preparation budget"));
    }
    let mut examined_items = 0;
    let mut examined_bytes = 0;
    if let Some(first) = progress.maybe_next_height().map_err(index_corruption)? {
        for height in first..=progress.accepted_target().height() {
            if records.len() as u64 >= maximum.blocks {
                break;
            }
            let position = manager
                .chainstate()
                .active_chain()
                .get(height as usize)
                .ok_or_else(|| index_corruption("missing BASIC canonical position"))?;
            let reusable = store.maybe_basic_filter_reusable_record(
                position,
                &mut work,
                maximum,
                |work, cost| {
                    Ok(choose_work(work, cost, budget, &mut maximum, outcome)?.then_some(maximum))
                },
            )?;
            if reusable.is_deferred() {
                break;
            }
            if let Some(record) = reusable.maybe_record() {
                work = work
                    .checked_add(TurnWork {
                        blocks: 1,
                        checkpoint_operations: 3,
                        ..Default::default()
                    })
                    .map_err(index_corruption)?;
                if !work.fits(maximum) {
                    return Err(index_corruption("BASIC reuse ancestry budget"));
                }
                let id = record.identity();
                open_bitcoin_core::chainstate::filter_index::verify_filter_record_predecessor(
                    id.height(),
                    id.parent_hash(),
                    id.previous_header(),
                    maybe_previous.as_ref().map(|previous| {
                        (
                            previous.height(),
                            previous.block_hash(),
                            previous.filter_header(),
                        )
                    }),
                )
                .map_err(index_corruption)?;
                maybe_previous = Some(id);
                records.push(record);
                outcome.reused_records += 1;
                continue;
            }
            let maybe_facts = manager
                .maybe_accepted_basic_facts()
                .filter(|facts| facts.position().height == height);
            let inputs;
            let maybe_body;
            if let Some(facts) = maybe_facts {
                let mut candidate_work = facts
                    .work()
                    .checked_add(TurnWork {
                        checkpoint_operations: 3,
                        ..Default::default()
                    })
                    .map_err(index_corruption)?;
                let items = candidate_work.script_items;
                let bytes = candidate_work.script_bytes;
                maybe_body = None;
                // Reservation precedes body/merkle validation and generation.
                reserve_generation(&mut candidate_work)?;
                if !choose_work(&mut work, candidate_work, budget, &mut maximum, outcome)? {
                    break;
                }
                if facts.position().block_hash != position.block_hash
                    || facts.position().height != position.height
                    || facts.position().previous_block_hash() != position.previous_block_hash()
                {
                    return Err(index_corruption(
                        "BASIC accepted facts differ from canonical position",
                    ));
                }
                examined_items += items;
                examined_bytes += bytes;
                inputs = facts.inputs().map_err(index_corruption)?;
            } else {
                *failure = crate::chainstate::filter_index::AcceptedBasicIndexFailure::RequiredUndo;
                let maybe_undo = if height == 0 {
                    None
                } else {
                    Some(
                        manager
                            .chainstate()
                            .undo_by_block()
                            .get(&position.block_hash)
                            .ok_or_else(|| index_corruption("missing BASIC required undo"))?,
                    )
                };
                let undo_work = inputs::undo_work(maybe_undo, &mut work, maximum)?;
                let remaining_body = if records.is_empty() {
                    budget.absolute_singleton().body_bytes
                } else {
                    maximum.body_bytes.saturating_sub(work.body_bytes)
                };
                work = work
                    .checked_add(TurnWork {
                        record_operations: 1,
                        ..TurnWork::default()
                    })
                    .map_err(index_corruption)?;
                if !work.fits(maximum) {
                    return Err(index_corruption("BASIC source probe budget"));
                }
                outcome.body_reads += 1;
                *failure = crate::chainstate::filter_index::AcceptedBasicIndexFailure::RequiredBody;
                maybe_body = store.maybe_basic_filter_turn_body(
                    position.block_hash,
                    remaining_body,
                    records.is_empty(),
                    |body_work| {
                        let mut candidate_work =
                            body_work.checked_add(undo_work).map_err(index_corruption)?;
                        reserve_generation(&mut candidate_work)?;
                        let admitted =
                            choose_work(&mut work, candidate_work, budget, &mut maximum, outcome)?;
                        Ok(admitted)
                    },
                )?;
                let Some((body, observed)) = maybe_body.as_ref() else {
                    break;
                };
                examined_items += observed.script_items + undo_work.script_items;
                examined_bytes += observed.script_bytes + undo_work.script_bytes;
                outcome.body_decodes += 1;
                if maybe_undo.is_some() {
                    outcome.undo_borrows += 1;
                }
                inputs = BasicFilterInputs::from_historical(
                    body,
                    position,
                    maybe_undo.map(|undo| HistoricalBlockUndo {
                        block_hash: position.block_hash,
                        undo,
                    }),
                )
                .map_err(index_corruption)?;
            }
            let _body_lifetime = &maybe_body;
            let record = StoredFilterRecord::generate(&inputs, position, maybe_previous.as_ref())?;
            let mut reader =
                open_bitcoin_core::codec::primitives::Reader::new(record.encoded_bytes());
            let elements = open_bitcoin_core::codec::read_compact_size(&mut reader)
                .map_err(index_corruption)?;
            outcome.hashed_items += elements;
            outcome.sorted_items += elements;
            outcome.generations += 1;
            maybe_previous = Some(record.identity());
            records.push(record);
            if outcome.oversized_singleton {
                break;
            }
        }
    }
    outcome.examined_script_items = examined_items;
    outcome.examined_script_bytes = examined_bytes;
    Ok(GeneratedRecords {
        records,
        work,
        maximum,
    })
}

fn reserve_generation(work: &mut TurnWork) -> Result<(), StorageError> {
    // Borrowed script lists/BTree nodes, values, txid encoding/merkle layers and
    // GCS buffers plus the generator's owned output copy are reserved pre-work.
    let output = work
        .script_items
        .checked_mul(4)
        .and_then(|n| n.checked_add(179))
        .ok_or_else(|| index_corruption("BASIC generation overflow"))?;
    let copies = work
        .body_bytes
        .checked_mul(4)
        .and_then(|n| n.checked_add(work.script_items * 256))
        .and_then(|n| n.checked_add(output * 4))
        .ok_or_else(|| index_corruption("BASIC generation allocation overflow"))?;
    *work = work
        .checked_add(TurnWork {
            blocks: 1,
            encoded_bytes: output,
            cloned_bytes: copies,
            // Conservative per-element BTree selection and mapped-sort comparisons;
            // report examined/hash/sorted element counts separately from reservation.
            script_items: work
                .script_items
                .checked_mul(256)
                .ok_or_else(|| index_corruption("BASIC item-work overflow"))?,
            script_bytes: work
                .script_bytes
                .checked_mul(256)
                .ok_or_else(|| index_corruption("BASIC byte-work overflow"))?,
            ..TurnWork::default()
        })
        .map_err(index_corruption)?;
    Ok(())
}

fn choose_work(
    work: &mut TurnWork,
    candidate: TurnWork,
    budget: BasicIndexTurnBudget,
    maximum: &mut TurnWork,
    outcome: &mut BasicFilterTurnOutcome,
) -> Result<bool, StorageError> {
    if !candidate.fits(budget.absolute_singleton()) {
        return Err(index_corruption("BASIC singleton absolute input bound"));
    }
    let total = work.checked_add(candidate).map_err(index_corruption)?;
    if total.fits(*maximum) {
        *work = total;
        return Ok(true);
    }
    if outcome.generations != 0 || outcome.reused_records != 0 {
        return Ok(false);
    }
    if !total.fits(budget.absolute_singleton()) {
        return Err(index_corruption("BASIC singleton absolute input bound"));
    }
    *maximum = budget.absolute_singleton();
    outcome.oversized_singleton = true;
    *work = total;
    Ok(true)
}

fn subtract(maximum: TurnWork, used: TurnWork) -> Result<TurnWork, StorageError> {
    macro_rules! remaining {
        ($field:ident) => {
            maximum
                .$field
                .checked_sub(used.$field)
                .ok_or_else(|| index_corruption("BASIC total turn budget"))?
        };
    }
    Ok(TurnWork {
        blocks: remaining!(blocks),
        body_bytes: remaining!(body_bytes),
        undo_bytes: remaining!(undo_bytes),
        cloned_bytes: remaining!(cloned_bytes),
        script_items: remaining!(script_items),
        script_bytes: remaining!(script_bytes),
        encoded_bytes: remaining!(encoded_bytes),
        record_operations: remaining!(record_operations),
        checkpoint_operations: remaining!(checkpoint_operations),
        projection_operations: remaining!(projection_operations),
    })
}
