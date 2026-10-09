// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Bounded consecutive publication. Recovery remains the complete-history proof.

use super::publication::{checkpoint_fault, protection_fault, records_fault};
use super::{
    BasicFilterAppendProof, FjallNodeStore, StorageError, StorageNamespace, codec, index_corruption,
};
use crate::storage::fjall_store::prune::{PRUNE_LOCKS_KEY, encode_prune_locks};
use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, FilterCheckpoint, FilterRecordIdentity, IndexPrefix,
    filter_index::catch_up::{
        BASIC_INDEX_MAX_CANDIDATES, BASIC_INDEX_MAX_ENCODED_BYTES,
        BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES, TurnWork,
    },
};

/// Private prepared bytes cannot be altered or moved to another store/frontier.
pub(crate) struct PreparedBasicFilterAppend {
    proof: BasicFilterAppendProof,
    records: Vec<(String, Vec<u8>)>,
    projections: Vec<(String, Vec<u8>)>,
    processed: FilterCheckpoint,
    safe_checkpoint: FilterCheckpoint,
    work: TurnWork,
}

/// Achieved facts only: an error never returns publication progress.
#[derive(Debug)]
pub(crate) struct BasicFilterAppendOutcome {
    pub processed: FilterCheckpoint,
    pub safe_checkpoint: FilterCheckpoint,
    pub protection: open_bitcoin_core::chainstate::IndexInputProtection,
    pub work: TurnWork,
    pub batch_bytes: u64,
}

impl FjallNodeStore {
    /// Bound envelopes before encoding; prove each immediate edge exactly once.
    pub(crate) fn prepare_basic_filter_append(
        &self,
        proof: BasicFilterAppendProof,
        records: &[codec::StoredFilterRecord],
    ) -> Result<super::PreparedBasicFilterAppend, StorageError> {
        self.prepare_basic_filter_append_guarded_positions(proof, records, None)
    }

    pub(crate) fn prepare_basic_filter_replacement_append(
        &self,
        proof: BasicFilterAppendProof,
        positions: crate::chainstate::ValidatedBasicFilterAppendPositions<'_>,
        records: &[codec::StoredFilterRecord],
    ) -> Result<PreparedBasicFilterAppend, StorageError> {
        self.prepare_basic_filter_append_guarded_positions(proof, records, Some(positions))
    }

    fn prepare_basic_filter_append_guarded_positions(
        &self,
        proof: BasicFilterAppendProof,
        records: &[codec::StoredFilterRecord],
        maybe_positions: Option<crate::chainstate::ValidatedBasicFilterAppendPositions<'_>>,
    ) -> Result<PreparedBasicFilterAppend, StorageError> {
        check_envelope_bounds(records)?;
        let Some(maximum_work) = proof.maybe_maximum_work() else {
            return Err(index_corruption(
                "BASIC append requires budgeted acquisition",
            ));
        };
        let mut work = self.check_basic_filter_append_proof(&proof)?;
        let replacement = maybe_positions.is_some();
        if proof.identity.projection_authority
            == super::ownership::BasicFilterProjectionAuthority::ValidatedReorg
            && !replacement
            && records
                .iter()
                .any(|record| match proof.processed().prefix() {
                    IndexPrefix::Empty => true,
                    IndexPrefix::Committed(id) => record.identity().height() > id.height(),
                })
        {
            return Err(index_corruption(
                "BASIC replacement requires accepted positions",
            ));
        }
        if let Some(positions) = maybe_positions {
            let cost = positions.work();
            charge_append_work(&mut work, cost, Some(maximum_work))?;
            positions.validate_for(self, &proof, records)?;
        }
        if !work.fits(maximum_work) {
            return Err(index_corruption("BASIC append work budget"));
        }
        let processed = proof.processed();
        let mut prepared = PreparedBasicFilterAppend {
            safe_checkpoint: proof.safe_checkpoint(),
            processed,
            proof,
            records: Vec::with_capacity(records.len()),
            projections: Vec::with_capacity(records.len()),
            work: TurnWork::default(),
        };
        let mut maybe_previous = match records.first() {
            None => None,
            Some(record) if record.identity().height() == 0 => None,
            Some(record) => {
                let id = record.identity();
                if let IndexPrefix::Committed(endpoint) = processed.prefix()
                    && endpoint.height().checked_add(1) == Some(id.height())
                {
                    Some(endpoint)
                } else {
                    Some(self.read_basic_filter_local_identity(
                        id.parent_hash(),
                        &mut work,
                        Some(maximum_work),
                    )?)
                }
            }
        };
        for record in records {
            let id = record.identity();
            let maybe_next = match prepared.processed.prefix() {
                IndexPrefix::Empty => Some(0),
                IndexPrefix::Committed(endpoint) => endpoint.height().checked_add(1),
            };
            if maybe_next.is_some_and(|next| id.height() > next) {
                return Err(index_corruption("BASIC append skipped height"));
            }
            let key = codec::record_key(id.block_hash());
            let size = record.encoded_bytes().len() as u64 + codec::RECORD_OVERHEAD as u64;
            charge_append_work(
                &mut work,
                TurnWork {
                    blocks: 1,
                    record_operations: 1,
                    encoded_bytes: size,
                    cloned_bytes: size,
                    ..TurnWork::default()
                },
                Some(maximum_work),
            )?;
            let bytes = codec::encode_record(record);
            codec::parse_record(&key, &bytes)?;
            charge_append_work(
                &mut work,
                TurnWork {
                    record_operations: 1,
                    ..TurnWork::default()
                },
                Some(maximum_work),
            )?;
            open_bitcoin_core::chainstate::filter_index::verify_filter_record_predecessor(
                id.height(),
                id.parent_hash(),
                id.previous_header(),
                maybe_previous
                    .as_ref()
                    .map(|previous: &FilterRecordIdentity| {
                        (
                            previous.height(),
                            previous.block_hash(),
                            previous.filter_header(),
                        )
                    }),
            )
            .map_err(index_corruption)?;
            match self.maybe_bounded_basic_filter_append_row(&key, &mut work, Some(maximum_work))? {
                Some(existing) => {
                    if existing != bytes {
                        return Err(index_corruption("conflicting immutable BASIC record"));
                    }
                }
                None if maybe_next != Some(id.height()) => {
                    return Err(index_corruption("missing BASIC retry row"));
                }
                None => prepared.records.push((key, bytes)),
            }
            let projection_key = codec::active_key(id.height());
            let maybe_projection = self.maybe_basic_filter_append_projection(
                id.height(),
                &mut work,
                Some(maximum_work),
            )?;
            if !replacement
                && proof_identity_is_recovered(&prepared.proof)
                && maybe_projection.is_some()
                && maybe_next == Some(id.height())
            {
                return Err(index_corruption(
                    "BASIC recovered projection advance requires accepted positions",
                ));
            }
            if maybe_projection.is_some_and(|hash| hash != id.block_hash())
                && !(replacement && maybe_next == Some(id.height()))
            {
                return Err(index_corruption("conflicting BASIC suffix projection"));
            }
            if maybe_projection.is_none() && maybe_next != Some(id.height()) {
                return Err(index_corruption("missing BASIC retry projection"));
            }
            if maybe_projection != Some(id.block_hash()) {
                charge_append_work(
                    &mut work,
                    TurnWork {
                        projection_operations: 1,
                        cloned_bytes: 38,
                        ..TurnWork::default()
                    },
                    Some(maximum_work),
                )?;
                prepared.projections.push((
                    projection_key,
                    codec::encode_projection(id.height(), id.block_hash()),
                ));
            }
            if maybe_next == Some(id.height()) {
                prepared.processed = FilterCheckpoint::new(IndexPrefix::Committed(id));
            }
            maybe_previous = Some(id);
        }
        prepared.safe_checkpoint =
            self.earned_basic_filter_safe_checkpoint(&prepared, records, &mut work)?;
        prepared.work = work;
        Ok(prepared)
    }

    /// Compare the captured capability again under the single publication guard.
    pub(crate) fn complete_basic_filter_append(
        &self,
        mut prepared: PreparedBasicFilterAppend,
    ) -> Result<super::BasicFilterAppendOutcome, StorageError> {
        let mut control = self.filter_publication_guard()?;
        if control.maybe_reorg_suspension.is_some() {
            return Err(index_corruption("BASIC append suspended for reorg preview"));
        }
        let mut identity = self.check_basic_filter_append_proof_guarded_counted(
            &prepared.proof,
            &control,
            &mut prepared.work,
        )?;
        if prepared.records.is_empty()
            && prepared.projections.is_empty()
            && prepared.processed == identity.processed
            && prepared.safe_checkpoint == identity.safe_checkpoint
        {
            return Ok(BasicFilterAppendOutcome {
                processed: identity.processed,
                safe_checkpoint: identity.safe_checkpoint,
                protection: identity
                    .owner
                    .maybe_effective_protection()
                    .unwrap_or(identity.owner.saved_protection()),
                work: prepared.work,
                batch_bytes: 0,
            });
        }
        let protection = if prepared.safe_checkpoint == identity.safe_checkpoint {
            identity
                .owner
                .maybe_effective_protection()
                .unwrap_or(identity.owner.saved_protection())
        } else {
            prepared.safe_checkpoint.input_protection()
        };
        let maximum_work = prepared.proof.maybe_maximum_work();
        let mut locks = self.load_basic_filter_append_locks(&mut prepared.work, maximum_work)?;
        locks.retain(|lock| lock.name != BASIC_INDEX_PRUNE_LOCK);
        if let Some(lock) = protection.maybe_prune_lock() {
            locks.push(lock);
        }
        let state = codec::StoredFilterState {
            maybe_endpoint: maybe_checkpoint_endpoint(prepared.safe_checkpoint),
            fence_height: identity.durable_height,
            fence_hash: identity.durable_hash,
            protection,
        };
        let count = locks.len() as u64;
        let lock_size = locks.iter().try_fold(4_u64, |size, lock| {
            size.checked_add(lock.name.len() as u64 + 10)
                .ok_or_else(|| index_corruption("BASIC lock-map encoded overflow"))
        })?;
        // Encoding includes another duplicate-name check; reserve its comparison
        // ceiling and output bytes before constructing the lock-map buffer.
        charge_append_work(
            &mut prepared.work,
            super::ownership::basic_filter_lock_map_cost(count, lock_size)?,
            maximum_work,
        )?;
        charge_append_work(
            &mut prepared.work,
            TurnWork {
                checkpoint_operations: 3,
                cloned_bytes: 80,
                ..TurnWork::default()
            },
            maximum_work,
        )?;
        let state_bytes = codec::encode_state(state);
        let lock_bytes = encode_prune_locks(&locks)?;
        // Construct achieved ownership before effects so no fallible reconstruction follows commit.
        let owner = open_bitcoin_core::chainstate::filter_index::lifecycle::EffectiveIndexOwnership::maybe_from_artifacts(
            Some(identity.owner.lifecycle()),
            Some(open_bitcoin_core::chainstate::filter_index::lifecycle::IndexCheckpointIdentity::new(
                prepared.safe_checkpoint, identity.durable_height, identity.durable_hash)),
            Some(protection), locks.iter().find(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK), false, false,
        ).map_err(index_corruption)?.ok_or_else(|| index_corruption("absent achieved BASIC owner"))?;
        let mut batch_bytes =
            (state_bytes.len() + lock_bytes.len() + codec::STATE_KEY.len() + PRUNE_LOCKS_KEY.len())
                as u64;
        charge_append_work(
            &mut prepared.work,
            TurnWork {
                record_operations: prepared.records.len() as u64,
                projection_operations: prepared.projections.len() as u64,
                cloned_bytes: batch_bytes
                    + prepared
                        .records
                        .iter()
                        .chain(&prepared.projections)
                        .map(|(key, bytes)| (key.len() + bytes.len()) as u64)
                        .sum::<u64>(),
                ..TurnWork::default()
            },
            maximum_work,
        )?;
        let mut batch = self
            .db
            .batch()
            .durability(Some(fjall::PersistMode::SyncAll));
        records_fault(&mut control)?;
        for (key, bytes) in prepared.records {
            batch_bytes += (key.len() + bytes.len()) as u64;
            batch.insert(&self.block_index, key, bytes);
        }
        for (key, bytes) in prepared.projections {
            batch_bytes += (key.len() + bytes.len()) as u64;
            batch.insert(&self.block_index, key, bytes);
        }
        checkpoint_fault(&mut control)?;
        batch.insert(&self.block_index, codec::STATE_KEY, state_bytes);
        protection_fault(&mut control)?;
        batch.insert(&self.block_index, PRUNE_LOCKS_KEY, lock_bytes);
        self.finish_basic_filter_batch(batch, &mut control)?;
        identity.owner = owner;
        identity.processed = prepared.processed;
        identity.safe_checkpoint = prepared.safe_checkpoint;
        identity.revision = control.revision;
        control.maybe_append_identity = Some(identity);
        Ok(BasicFilterAppendOutcome {
            processed: identity.processed,
            safe_checkpoint: identity.safe_checkpoint,
            protection: identity
                .owner
                .maybe_effective_protection()
                .unwrap_or(identity.owner.saved_protection()),
            work: prepared.work,
            batch_bytes,
        })
    }

    fn earned_basic_filter_safe_checkpoint(
        &self,
        prepared: &PreparedBasicFilterAppend,
        records: &[codec::StoredFilterRecord],
        work: &mut TurnWork,
    ) -> Result<FilterCheckpoint, StorageError> {
        let (height, hash) = prepared.proof.durable_tip();
        if prepared.proof.durable_displaced() {
            return Ok(prepared.safe_checkpoint);
        }
        let IndexPrefix::Committed(processed) = prepared.processed.prefix() else {
            return Ok(prepared.safe_checkpoint);
        };
        if processed.height() < height {
            return Ok(prepared.safe_checkpoint);
        }
        let maybe_candidate = records
            .iter()
            .find(|record| record.identity().height() == height);
        let endpoint = match maybe_candidate {
            Some(record) => record.identity(),
            None => {
                if self.maybe_basic_filter_append_projection(
                    height,
                    work,
                    prepared.proof.maybe_maximum_work(),
                )? != Some(hash)
                {
                    return Err(index_corruption(
                        "BASIC safe endpoint projection differs from durable tip",
                    ));
                }
                self.read_basic_filter_local_identity(
                    hash,
                    work,
                    prepared.proof.maybe_maximum_work(),
                )?
            }
        };
        if endpoint.height() != height || endpoint.block_hash() != hash {
            return Err(index_corruption(
                "BASIC safe endpoint differs from durable tip",
            ));
        }
        Ok(FilterCheckpoint::new(IndexPrefix::Committed(endpoint)))
    }

    pub(super) fn read_basic_filter_local_identity(
        &self,
        hash: open_bitcoin_core::primitives::BlockHash,
        work: &mut TurnWork,
        maybe_maximum_work: Option<TurnWork>,
    ) -> Result<FilterRecordIdentity, StorageError> {
        let key = codec::record_key(hash);
        let bytes = self
            .maybe_bounded_basic_filter_append_row(&key, work, maybe_maximum_work)?
            .ok_or_else(|| index_corruption("missing BASIC local endpoint record"))?;
        charge_append_work(
            work,
            TurnWork {
                record_operations: 1,
                ..TurnWork::default()
            },
            maybe_maximum_work,
        )?;
        let parsed = codec::parse_record(&key, &bytes)?;
        let maybe_parent_bytes = if parsed.height == 0 {
            None
        } else {
            let parent_bytes = self
                .maybe_bounded_basic_filter_append_row(
                    &codec::record_key(parsed.parent),
                    work,
                    maybe_maximum_work,
                )?
                .ok_or_else(|| index_corruption("missing BASIC predecessor record"))?;
            Some(parent_bytes)
        };
        let maybe_parent = maybe_parent_bytes
            .as_ref()
            .map(|bytes| {
                charge_append_work(
                    work,
                    TurnWork {
                        record_operations: 1,
                        ..TurnWork::default()
                    },
                    maybe_maximum_work,
                )?;
                codec::parse_record_fields(&codec::record_key(parsed.parent), bytes)
            })
            .transpose()?;
        charge_append_work(
            work,
            TurnWork {
                record_operations: 1,
                ..TurnWork::default()
            },
            maybe_maximum_work,
        )?;
        parsed.checkpoint_identity(maybe_parent.as_ref())
    }

    pub(super) fn maybe_basic_filter_append_projection(
        &self,
        height: u32,
        work: &mut TurnWork,
        maybe_maximum_work: Option<TurnWork>,
    ) -> Result<Option<open_bitcoin_core::primitives::BlockHash>, StorageError> {
        charge_append_work(
            work,
            TurnWork {
                projection_operations: 1,
                ..TurnWork::default()
            },
            maybe_maximum_work,
        )?;
        #[cfg(test)]
        self.count_filter_integrity_read();
        let key = codec::active_key(height);
        let Some(bytes) = self
            .block_index
            .get(&key)
            .map_err(|error| super::super::backend_failure(StorageNamespace::BlockIndex, error))?
        else {
            return Ok(None);
        };
        charge_append_work(
            work,
            TurnWork {
                projection_operations: 1,
                ..TurnWork::default()
            },
            maybe_maximum_work,
        )?;
        codec::decode_projection(&key, bytes.as_ref()).map(Some)
    }

    fn maybe_bounded_basic_filter_append_row(
        &self,
        key: &str,
        work: &mut TurnWork,
        maybe_maximum_work: Option<TurnWork>,
    ) -> Result<Option<Vec<u8>>, StorageError> {
        charge_append_work(
            work,
            TurnWork {
                record_operations: 1,
                ..TurnWork::default()
            },
            maybe_maximum_work,
        )?;
        #[cfg(test)]
        self.count_filter_integrity_read();
        let Some(bytes) = self
            .block_index
            .get(key)
            .map_err(|error| super::super::backend_failure(StorageNamespace::BlockIndex, error))?
        else {
            return Ok(None);
        };
        if bytes.len() as u64 > BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES {
            return Err(index_corruption("BASIC stored append row byte bound"));
        }
        charge_append_work(
            work,
            TurnWork {
                cloned_bytes: bytes.len() as u64,
                ..TurnWork::default()
            },
            maybe_maximum_work,
        )?;
        Ok(Some(bytes.as_ref().to_vec()))
    }
}

fn proof_identity_is_recovered(proof: &BasicFilterAppendProof) -> bool {
    proof.identity.projection_authority
        == super::ownership::BasicFilterProjectionAuthority::RecoveredPrefix
}

pub(super) fn charge_append_work(
    work: &mut TurnWork,
    cost: TurnWork,
    maybe_maximum_work: Option<TurnWork>,
) -> Result<(), StorageError> {
    let next = work.checked_add(cost).map_err(index_corruption)?;
    if maybe_maximum_work.is_some_and(|maximum| !next.fits(maximum)) {
        return Err(index_corruption("BASIC append work budget"));
    }
    *work = next;
    Ok(())
}

fn maybe_checkpoint_endpoint(
    checkpoint: FilterCheckpoint,
) -> Option<(u32, open_bitcoin_core::primitives::BlockHash)> {
    match checkpoint.prefix() {
        IndexPrefix::Empty => None,
        IndexPrefix::Committed(id) => Some((id.height(), id.block_hash())),
    }
}

fn check_envelope_bounds(records: &[codec::StoredFilterRecord]) -> Result<(), StorageError> {
    if records.len() as u64 > BASIC_INDEX_MAX_CANDIDATES {
        return Err(index_corruption("BASIC append record count bound"));
    }
    let mut total = 0_u64;
    for record in records {
        let envelope = (record.encoded_bytes().len() as u64)
            .checked_add(codec::RECORD_OVERHEAD as u64)
            .ok_or_else(|| index_corruption("BASIC append byte overflow"))?;
        if envelope > BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES {
            return Err(index_corruption("BASIC append singleton byte bound"));
        }
        total = total
            .checked_add(envelope)
            .ok_or_else(|| index_corruption("BASIC append byte overflow"))?;
        if total > BASIC_INDEX_MAX_ENCODED_BYTES {
            return Err(index_corruption("BASIC append aggregate byte bound"));
        }
    }
    Ok(())
}

impl BasicFilterAppendProof {
    pub(super) fn durable_displaced(&self) -> bool {
        self.identity.maybe_replacement.is_some_and(|replacement| {
            replacement.maybe_displaced_fence == Some(self.durable_tip())
        })
    }
}

impl BasicFilterAppendProof {
    pub(crate) fn admit_work(&self, work: TurnWork) -> Result<(), StorageError> {
        if self
            .maybe_maximum_work()
            .is_none_or(|maximum| !work.fits(maximum))
        {
            return Err(index_corruption("BASIC accepted-position work budget"));
        }
        Ok(())
    }
}
