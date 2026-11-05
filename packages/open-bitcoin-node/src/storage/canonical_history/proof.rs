// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/coins.cpp

use super::{
    Refusal,
    records::{CanonicalHistoryRecords, Rows},
};
use crate::storage::{filter_index as filters, fjall_store, validation_history as history};
use open_bitcoin_core::{
    chainstate::{
        BASIC_INDEX_PRUNE_LOCK, BasicFilterInputs, BlockUndo, Chainstate, CoinsCache,
        HistoricalBlockUndo, IndexInputProtection, MemoryCoinsView,
        filter_index::lifecycle::IndexLifecycle,
    },
    consensus::{CanonicalChain, ScriptVerifyFlags, chain_spec::CanonicalGenesisProof},
    primitives::BlockHash,
};
use open_bitcoin_network::{HeaderEntry, HeaderStore};
use std::collections::{HashMap, HashSet};

/// Only full replay can construct this evidence; no raw DTO or marker constructor.
/// ```compile_fail
/// use open_bitcoin_node::storage::canonical_history::ProvedHistoryRecords;
/// fn forge() -> ProvedHistoryRecords { ProvedHistoryRecords::default() }
/// ```
pub(in crate::storage) struct ProvedHistoryRecords {
    genesis: CanonicalGenesisProof,
    headers: HeaderStore,
    replay: Chainstate<MemoryCoinsView>,
    basic: Rows,
    retained_undo: HashMap<BlockHash, BlockUndo>,
    source: [u8; 32],
}

impl ProvedHistoryRecords {
    /// Consume earned owners only for the same complete source and generation.
    pub(in crate::storage) fn into_parts(
        self,
        records: &CanonicalHistoryRecords,
    ) -> Result<
        (
            CanonicalGenesisProof,
            HeaderStore,
            Chainstate<MemoryCoinsView>,
        ),
        Refusal,
    > {
        if self.source != records.source_digest()
            || self.basic != records.basic
            || self.retained_undo != records.snapshot.undo_by_block
        {
            return Err(Refusal::ChangedStore);
        }
        Ok((self.genesis, self.headers, self.replay))
    }
}

pub(super) fn validate_history_records(
    selected: CanonicalChain,
    records: &CanonicalHistoryRecords,
    now: i64,
) -> Result<ProvedHistoryRecords, Refusal> {
    if selected != records.chain {
        return Err(Refusal::InconsistentGenesis);
    }
    let root = records
        .snapshot
        .active_chain
        .first()
        .ok_or(Refusal::MissingRecord)?;
    let body = records
        .bodies
        .get(&root.block_hash)
        .ok_or(Refusal::MissingRecord)?;
    let genesis = CanonicalGenesisProof::prove(selected.spec(), body)
        .map_err(|_| Refusal::InconsistentGenesis)?;
    if records.headers != records.index || records.coverage != history::ValidationCoverage::Complete
    {
        return Err(Refusal::MalformedRecord);
    }
    let mut entries = records.headers.clone();
    entries.sort_by_key(|entry| (entry.height, entry.block_hash));
    let headers = HeaderStore::readmit_canonical_entries(&genesis, entries, now)
        .map_err(|_| Refusal::UnsupportedHistory)?;
    let stored: HashMap<_, _> = records
        .headers
        .iter()
        .map(|entry| (entry.block_hash, entry))
        .collect();
    if stored.len() != records.headers.len()
        || headers.entries().count() != stored.len()
        || headers
            .entries()
            .any(|entry| stored.get(&entry.block_hash).copied() != Some(entry))
    {
        return Err(Refusal::MalformedRecord);
    }
    let coins = CoinsCache::from_parent(MemoryCoinsView::from_coins(
        HashMap::new(),
        Some(genesis.hash()),
    ));
    let mut replay = Chainstate::from_canonical_genesis(&genesis, coins)
        .map_err(|_| Refusal::InconsistentGenesis)?;
    let mut active = HashSet::new();
    for (index, position) in records.snapshot.active_chain.iter().enumerate() {
        if position.height as usize != index || !active.insert(position.block_hash) {
            return Err(Refusal::MalformedRecord);
        }
        let entry = stored
            .get(&position.block_hash)
            .ok_or(Refusal::MissingRecord)?;
        if **entry
            != (HeaderEntry {
                block_hash: position.block_hash,
                header: position.header.clone(),
                height: position.height,
                chain_work: position.chain_work,
            })
        {
            return Err(Refusal::MalformedRecord);
        }
        validate_status(records, position)?;
    }
    let (earned_undo, earned_filters) =
        replay_graph(records, &stored, &headers, &mut replay, &genesis, &active)?;
    let mut snapshot = replay
        .admission_snapshot()
        .map_err(|_| Refusal::MalformedRecord)?;
    // Core disconnect removes orphan undo. Its independently earned physical
    // counterpart is compared separately and never injected into the core.
    snapshot.undo_by_block = earned_undo.clone();
    if snapshot != records.snapshot
        || replay
            .coins_best_block()
            .map_err(|_| Refusal::MalformedRecord)?
            != Some(records.best_block)
    {
        return Err(Refusal::MalformedRecord);
    }
    validate_basic(records, &snapshot, &earned_filters)?;
    Ok(ProvedHistoryRecords {
        genesis,
        headers,
        replay,
        basic: records.basic.clone(),
        retained_undo: earned_undo,
        source: records.source_digest(),
    })
}

type RetainedEvidence = (
    HashMap<BlockHash, BlockUndo>,
    HashMap<BlockHash, filters::StoredFilterRecord>,
);

fn replay_graph(
    records: &CanonicalHistoryRecords,
    stored: &HashMap<BlockHash, &HeaderEntry>,
    headers: &HeaderStore,
    replay: &mut Chainstate<MemoryCoinsView>,
    genesis: &CanonicalGenesisProof,
    active: &HashSet<BlockHash>,
) -> Result<RetainedEvidence, Refusal> {
    let children = accepted_children(records, stored, genesis.hash())?;
    let mut earned_undo = HashMap::new();
    let mut earned_filters = HashMap::new();
    let mut seen = HashSet::new();
    let mut events = vec![ReplayEvent::Connect(genesis.hash())];
    let params =
        open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(genesis);
    while let Some(event) = events.pop() {
        match event {
            ReplayEvent::Disconnect(hash) => {
                replay
                    .disconnect_tip(records.bodies.get(&hash).ok_or(Refusal::MissingRecord)?)
                    .map_err(|_| Refusal::UnsupportedHistory)?;
            }
            ReplayEvent::Connect(hash) => {
                if !seen.insert(hash) || seen.len() > super::records::MAX_HISTORY_BLOCKS {
                    return Err(Refusal::UnsupportedHistory);
                }
                let entry = stored.get(&hash).ok_or(Refusal::MissingRecord)?;
                let body = records.bodies.get(&hash).ok_or(Refusal::MissingRecord)?;
                if hash != genesis.hash() {
                    let context = headers
                        .canonical_header_validation_context(&body.header)
                        .map_err(|_| Refusal::UnsupportedHistory)?;
                    replay
                        .admit_canonical_header(&body.header, context.current_time)
                        .map_err(|_| Refusal::UnsupportedHistory)?;
                    replay
                        .connect_block_with_current_time(
                            body,
                            entry.chain_work,
                            context.current_time,
                            ScriptVerifyFlags::NONE,
                            params,
                        )
                        .map_err(|_| Refusal::UnsupportedHistory)?;
                }
                let position = replay.tip().ok_or(Refusal::MissingRecord)?;
                if position.block_hash != hash
                    || position.header != entry.header
                    || position.height != entry.height
                    || position.chain_work != entry.chain_work
                    || headers.median_time_past(hash) != Some(position.median_time_past)
                {
                    return Err(Refusal::MalformedRecord);
                }
                validate_status(records, position)?;
                let maybe_undo = replay.undo_by_block().get(&hash);
                if let Some(undo) = maybe_undo {
                    if records.snapshot.undo_by_block.get(&hash) != Some(undo) {
                        return Err(Refusal::MalformedRecord);
                    }
                    earned_undo.insert(hash, undo.clone());
                } else if position.height != 0 {
                    return Err(Refusal::MissingRecord);
                }
                let inputs = BasicFilterInputs::from_historical(
                    body,
                    position,
                    maybe_undo.map(|undo| HistoricalBlockUndo {
                        block_hash: hash,
                        undo,
                    }),
                )
                .map_err(|_| Refusal::InconsistentIndex)?;
                let maybe_previous = earned_filters
                    .get(&position.previous_block_hash())
                    .map(filters::StoredFilterRecord::identity);
                let filter = filters::StoredFilterRecord::generate(
                    &inputs,
                    position,
                    maybe_previous.as_ref(),
                )
                .map_err(|_| Refusal::InconsistentIndex)?;
                earned_filters.insert(hash, filter);
                let mut next = children.get(&hash).cloned().unwrap_or_default();
                next.sort_by_key(|child| (active.contains(child), *child));
                for child in next.into_iter().rev() {
                    if !active.contains(&child) {
                        events.push(ReplayEvent::Disconnect(child));
                    }
                    events.push(ReplayEvent::Connect(child));
                }
                if events.len() > 2 * super::records::MAX_HISTORY_BLOCKS {
                    return Err(Refusal::UnsupportedHistory);
                }
            }
        }
    }
    let accepted = records
        .validation
        .values()
        .filter(|record| record.status() == history::StoredValidationStatus::ScriptsValid)
        .count();
    if seen.len() != accepted || earned_undo != records.snapshot.undo_by_block {
        return Err(Refusal::MissingRecord);
    }
    Ok((earned_undo, earned_filters))
}

enum ReplayEvent {
    Connect(BlockHash),
    Disconnect(BlockHash),
}

fn accepted_children(
    records: &CanonicalHistoryRecords,
    stored: &HashMap<BlockHash, &HeaderEntry>,
    root: BlockHash,
) -> Result<HashMap<BlockHash, Vec<BlockHash>>, Refusal> {
    let mut children: HashMap<BlockHash, Vec<BlockHash>> = HashMap::new();
    for record in records.validation.values() {
        let identity = record.identity();
        if let Some(entry) = stored.get(&identity.hash()) {
            if identity.height() != entry.height
                || identity.parent_hash() != entry.header.previous_block_hash
            {
                return Err(Refusal::MalformedRecord);
            }
        } else if record.status() == history::StoredValidationStatus::ScriptsValid {
            return Err(Refusal::MissingRecord);
        }
        if record.status() != history::StoredValidationStatus::ScriptsValid
            || identity.hash() == root
        {
            continue;
        }
        if records
            .validation
            .get(&identity.parent_hash())
            .is_none_or(|parent| parent.status() != history::StoredValidationStatus::ScriptsValid)
        {
            return Err(Refusal::MissingRecord);
        }
        children
            .entry(identity.parent_hash())
            .or_default()
            .push(identity.hash());
    }
    Ok(children)
}

fn validate_status(
    records: &CanonicalHistoryRecords,
    position: &open_bitcoin_core::chainstate::ChainPosition,
) -> Result<(), Refusal> {
    let record = records
        .validation
        .get(&position.block_hash)
        .ok_or(Refusal::MissingRecord)?;
    let id = record.identity();
    if record.status() != history::StoredValidationStatus::ScriptsValid
        || record.provenance() != history::ValidationProvenance::ScriptsValid
        || id.hash() != position.block_hash
        || id.parent_hash() != position.previous_block_hash()
        || id.height() != position.height
    {
        return Err(Refusal::MalformedRecord);
    }
    Ok(())
}

fn validate_basic(
    records: &CanonicalHistoryRecords,
    snapshot: &open_bitcoin_core::chainstate::ChainstateSnapshot,
    earned: &HashMap<BlockHash, filters::StoredFilterRecord>,
) -> Result<(), Refusal> {
    let rows = &records.basic;
    fn invalid<T>(_: T) -> Refusal {
        Refusal::InconsistentIndex
    }
    let maybe_lock_bytes = rows.get(fjall_store::PRUNE_LOCKS_KEY.as_bytes());
    let locks = maybe_lock_bytes
        .map(|bytes| fjall_store::decode_prune_locks(bytes))
        .transpose()
        .map_err(invalid)?
        .unwrap_or_default();
    if let Some(bytes) = maybe_lock_bytes
        && fjall_store::encode_prune_locks(&locks).map_err(invalid)? != *bytes
    {
        return Err(Refusal::InconsistentIndex);
    }
    if locks
        .iter()
        .any(|lock| lock.height_first > lock.height_last)
    {
        return Err(Refusal::InconsistentIndex);
    }
    let mut basic_locks = locks
        .iter()
        .filter(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK);
    let maybe_lock = basic_locks.next();
    if basic_locks.next().is_some() {
        return Err(Refusal::InconsistentIndex);
    }
    let Some(bytes) = rows.get(filters::STATE_KEY.as_bytes()) else {
        if rows
            .keys()
            .any(|key| key.as_slice() != fjall_store::PRUNE_LOCKS_KEY.as_bytes())
            || maybe_lock.is_some()
        {
            return Err(Refusal::InconsistentIndex);
        }
        return Ok(());
    };
    let state = filters::decode_state(bytes).map_err(invalid)?;
    let owner = filters::ownership::decode_owner(
        rows.get(filters::ownership::OWNER_KEY.as_bytes())
            .ok_or(Refusal::InconsistentIndex)?,
    )
    .map_err(invalid)?;
    if snapshot
        .active_chain
        .get(state.fence_height as usize)
        .is_none_or(|position| position.block_hash != state.fence_hash)
    {
        return Err(Refusal::InconsistentIndex);
    }
    let count = match state.maybe_endpoint {
        None => 0,
        Some((height, hash)) => {
            if snapshot
                .active_chain
                .get(height as usize)
                .is_none_or(|position| position.block_hash != hash)
            {
                return Err(Refusal::InconsistentIndex);
            }
            height as usize + 1
        }
    };
    let protection = IndexInputProtection::FromHeight(
        u32::try_from(count).map_err(|_| Refusal::InconsistentIndex)?,
    );
    if !state.protection.covers(protection) {
        return Err(Refusal::InconsistentIndex);
    }
    let maybe_saved = maybe_lock
        .map(IndexInputProtection::from_saved_lock)
        .transpose()
        .map_err(invalid)?;
    if maybe_saved.is_some_and(|lock| !lock.covers(state.protection))
        || matches!(owner, IndexLifecycle::Active { .. }) && maybe_saved.is_none()
    {
        return Err(Refusal::InconsistentIndex);
    }
    let record_count = rows
        .keys()
        .filter(|key| key.starts_with(filters::RECORD_PREFIX.as_bytes()))
        .count();
    if rows.len() != 2 + usize::from(maybe_lock_bytes.is_some()) + count + record_count {
        return Err(Refusal::InconsistentIndex);
    }
    for (key, bytes) in rows
        .iter()
        .filter(|(key, _)| key.starts_with(filters::RECORD_PREFIX.as_bytes()))
    {
        let key = std::str::from_utf8(key).map_err(invalid)?;
        let parsed = filters::parse_record(key, bytes).map_err(invalid)?;
        let expected = earned
            .get(&parsed.block)
            .ok_or(Refusal::InconsistentIndex)?;
        let maybe_previous = earned
            .get(&parsed.parent)
            .map(filters::StoredFilterRecord::identity);
        if parsed.height > 0 && !rows.contains_key(filters::record_key(parsed.parent).as_bytes()) {
            return Err(Refusal::InconsistentIndex);
        }
        let saved = filters::decode_record(key, bytes, maybe_previous.as_ref()).map_err(invalid)?;
        if &saved != expected {
            return Err(Refusal::InconsistentIndex);
        }
    }
    for position in snapshot.active_chain.iter().take(count) {
        if !rows.contains_key(filters::record_key(position.block_hash).as_bytes()) {
            return Err(Refusal::InconsistentIndex);
        }
        let key = filters::active_key(position.height);
        let projection = filters::decode_projection(
            &key,
            rows.get(key.as_bytes()).ok_or(Refusal::InconsistentIndex)?,
        )
        .map_err(invalid)?;
        if projection != position.block_hash {
            return Err(Refusal::InconsistentIndex);
        }
    }
    Ok(())
}
