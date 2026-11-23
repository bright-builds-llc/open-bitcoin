// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use super::{
    Refusal, proof,
    records::{CanonicalHistoryRecords, Namespaces, Rows},
};
use crate::SyncNetwork;
use crate::storage::{
    filter_index,
    fjall_store::{
        self, block_key,
        inspection::{InspectedRecords, require_existing_namespaces},
        prune_proof_undo_key,
    },
    validation_history::ValidationProvenance,
};
#[cfg(test)]
use crate::{
    FjallNodeStore,
    storage::fjall_store::{inspection::LogicalInventory, tracked::RecordSource},
};
use open_bitcoin_core::{
    chainstate::{
        Chainstate, MemoryCoinsView, filter_index::lifecycle::IndexLifecycle,
        height_forbidden_by_any_lock, height_inside_keep_window,
    },
    consensus::{CanonicalChain, chain_spec::CanonicalGenesisProof},
    primitives::BlockHash,
};
use open_bitcoin_network::HeaderStore;

#[cfg(test)]
pub(crate) struct RegtestReplayArchive(Namespaces);

pub(in crate::storage) struct PrunedRegtestProjection {
    pub(crate) genesis: CanonicalGenesisProof,
    #[cfg(test)]
    pub(crate) deleted: Vec<BlockHash>,
    pub(crate) headers: HeaderStore,
    pub(crate) replay: Chainstate<MemoryCoinsView>,
}

pub(in crate::storage) fn prove_pruned_inspection(
    archive: &InspectedRecords,
    retained: &InspectedRecords,
) -> Result<PrunedRegtestProjection, Refusal> {
    prove_pruned_regtest(
        &observed_rows(archive)?,
        &observed_rows(retained)?,
        observation_time()?,
    )
}

fn observation_time() -> Result<i64, Refusal> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Refusal::StorageUnavailable)?;
    i64::try_from(now.as_secs()).map_err(|_| Refusal::StorageUnavailable)
}

#[cfg(test)]
impl FjallNodeStore {
    pub(crate) fn capture_regtest_replay_archive_for_test(
        &self,
    ) -> Result<RegtestReplayArchive, Refusal> {
        read_proof_rows(self).map(RegtestReplayArchive)
    }

    pub(crate) fn check_pruned_regtest_mutations_for_test(
        &self,
        archive: &RegtestReplayArchive,
    ) -> Result<(), Refusal> {
        let retained = read_proof_rows(self)?;
        let now = observation_time()?;
        let projection = prove_pruned_regtest(&archive.0, &retained, now)?;
        assert_projection_refusals(&archive.0, &retained, now, &projection.deleted);
        Ok(())
    }
}

#[cfg(test)]
impl RegtestReplayArchive {
    pub(crate) fn write_for_test(&self, path: &std::path::Path) {
        let db = fjall::Database::builder(path)
            .open()
            .expect("owned archive fixture");
        let mut batch = db.batch().durability(Some(fjall::PersistMode::SyncAll));
        for (name, rows) in &self.0 {
            let space = db
                .keyspace(name, fjall::KeyspaceCreateOptions::default)
                .expect("archive namespace");
            for (key, value) in rows {
                batch.insert(&space, key.as_slice(), value.as_slice());
            }
        }
        batch.commit().expect("complete independent replay input");
    }
}

#[cfg(test)]
fn read_proof_rows(store: &FjallNodeStore) -> Result<Namespaces, Refusal> {
    let db = store.database();
    let stamp = db.stamp().map_err(|_| Refusal::ChangedStore)?;
    let records = db
        .read_existing_records(LogicalInventory::RegtestPruneProof)
        .map_err(|_| Refusal::StorageUnavailable)?;
    let rows = observed_rows(&records)?;
    db.check_stamp(&stamp).map_err(|_| Refusal::ChangedStore)?;
    Ok(rows)
}

fn observed_rows(records: &InspectedRecords) -> Result<Namespaces, Refusal> {
    require_existing_namespaces(records).map_err(|_| Refusal::UnsupportedRecord)?;
    records
        .namespaces()
        .map(|name| {
            Ok((
                name.to_owned(),
                records
                    .maybe_records(name)
                    .ok_or(Refusal::MissingRecord)?
                    .clone(),
            ))
        })
        .collect()
}

fn prove_pruned_regtest(
    archive: &Namespaces,
    retained: &Namespaces,
    now: i64,
) -> Result<PrunedRegtestProjection, Refusal> {
    let records = CanonicalHistoryRecords::parse_pruned_regtest_archive(archive)?;
    let proof = proof::validate_history_records(CanonicalChain::Regtest, &records, now)?;
    let deleted = check_retained_rows(archive, retained, &records)?;
    let (genesis, mut headers, mut replay) = proof.into_parts(&records)?;
    let params =
        open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&genesis);
    for position in &records.snapshot.active_chain {
        let block = records
            .bodies
            .get(&position.block_hash)
            .ok_or(Refusal::MissingRecord)?;
        headers
            .check_body_eligibility(block, params, now)
            .map_err(|_| Refusal::UnsupportedHistory)?;
    }
    for hash in &deleted {
        replay.forget_undo(*hash);
        headers.mark_body_unavailable(*hash);
    }
    Ok(PrunedRegtestProjection {
        genesis,
        #[cfg(test)]
        deleted,
        headers,
        replay,
    })
}

fn check_retained_rows(
    archive: &Namespaces,
    retained: &Namespaces,
    records: &CanonicalHistoryRecords,
) -> Result<Vec<BlockHash>, Refusal> {
    let tip = records
        .snapshot
        .active_chain
        .last()
        .ok_or(Refusal::MissingRecord)?;
    if tip.height < SyncNetwork::Regtest.prune_after_height() {
        return Err(Refusal::UnsupportedHistory);
    }
    if records.headers.iter().any(|entry| {
        entry.chain_work > tip.chain_work
            && records
                .validation
                .get(&entry.block_hash)
                .is_some_and(|record| record.provenance() == ValidationProvenance::ScriptsValid)
    }) {
        return Err(Refusal::UnsupportedHistory);
    }
    let saved = filter_index::decode_state(
        records
            .basic
            .get(filter_index::STATE_KEY.as_bytes())
            .ok_or(Refusal::InconsistentIndex)?,
    )
    .map_err(|_| Refusal::InconsistentIndex)?;
    let owner = filter_index::ownership::decode_owner(
        records
            .basic
            .get(filter_index::ownership::OWNER_KEY.as_bytes())
            .ok_or(Refusal::InconsistentIndex)?,
    )
    .map_err(|_| Refusal::InconsistentIndex)?;
    if !matches!(owner, IndexLifecycle::Active { .. })
        || saved.maybe_endpoint != Some((tip.height, tip.block_hash))
        || (saved.fence_height, saved.fence_hash) != (tip.height, tip.block_hash)
    {
        return Err(Refusal::InconsistentIndex);
    }
    let locks = fjall_store::decode_prune_locks(
        records
            .basic
            .get(fjall_store::PRUNE_LOCKS_KEY.as_bytes())
            .ok_or(Refusal::InconsistentIndex)?,
    )
    .map_err(|_| Refusal::InconsistentIndex)?;

    for name in ["schema", "headers", "coins"] {
        if namespace(archive, name)? != namespace(retained, name)? {
            return Err(Refusal::MalformedRecord);
        }
    }
    let mut expected_index = namespace(archive, "block_index")?.clone();
    let mut expected_chainstate = namespace(archive, "chainstate")?.clone();
    let mut actual_index = namespace(retained, "block_index")?.clone();
    let actual_chainstate = namespace(retained, "chainstate")?;
    if actual_index
        .remove(fjall_store::HAVE_PRUNED_KEY.as_bytes())
        .as_deref()
        != Some(&[1])
    {
        return Err(Refusal::UnsupportedHistory);
    }
    // Support counters are telemetry. Actual mate absence below proves the projection.
    actual_index.remove(fjall_store::PRUNE_SUMMARY_KEY.as_bytes());
    let mut deleted = Vec::new();
    for position in records.snapshot.active_chain.iter().skip(1) {
        let body_key = block_key(position.block_hash);
        let undo_key = prune_proof_undo_key(position.block_hash);
        if actual_index.contains_key(body_key.as_bytes()) {
            continue;
        }
        if actual_chainstate.contains_key(undo_key.as_bytes())
            || height_inside_keep_window(tip.height, position.height)
            || height_forbidden_by_any_lock(position.height, &locks)
            || saved
                .protection
                .check_prune_intent(position.height)
                .is_err()
            || expected_index.remove(body_key.as_bytes()).is_none()
            || expected_chainstate.remove(undo_key.as_bytes()).is_none()
        {
            return Err(Refusal::UnsupportedHistory);
        }
        deleted.push(position.block_hash);
    }
    if deleted.is_empty()
        || actual_index != expected_index
        || *actual_chainstate != expected_chainstate
    {
        return Err(Refusal::MalformedRecord);
    }
    Ok(deleted)
}

fn namespace<'a>(all: &'a Namespaces, name: &str) -> Result<&'a Rows, Refusal> {
    all.get(name).ok_or(Refusal::MissingRecord)
}

#[cfg(test)]
fn assert_projection_refusals(
    archive: &Namespaces,
    retained: &Namespaces,
    now: i64,
    deleted: &[BlockHash],
) {
    let records =
        CanonicalHistoryRecords::parse_pruned_regtest_archive(archive).expect("complete archive");
    let mut changed = retained.clone();
    let key = prune_proof_undo_key(deleted[0]).into_bytes();
    changed
        .get_mut("chainstate")
        .expect("chainstate")
        .insert(key.clone(), archive["chainstate"][&key].clone());
    assert_eq!(
        check_retained_rows(archive, &changed, &records),
        Err(Refusal::UnsupportedHistory)
    );
    let mut changed = retained.clone();
    let tip = records
        .snapshot
        .active_chain
        .last()
        .expect("proved active tip");
    let last_prunable = open_bitcoin_core::chainstate::last_prunable_height(tip.height);
    let kept = records.snapshot.active_chain[last_prunable as usize + 1].block_hash;
    changed
        .get_mut("block_index")
        .expect("block index")
        .remove(block_key(kept).as_bytes())
        .expect("kept body");
    changed
        .get_mut("chainstate")
        .expect("chainstate")
        .remove(prune_proof_undo_key(kept).as_bytes())
        .expect("kept undo");
    assert_eq!(
        check_retained_rows(archive, &changed, &records),
        Err(Refusal::UnsupportedHistory)
    );

    let mut changed = retained.clone();
    let checkpoint = 1_000;
    let active = records.snapshot.active_chain[checkpoint as usize].block_hash;
    let stale = records
        .headers
        .iter()
        .find(|entry| entry.height == checkpoint && entry.block_hash != active)
        .expect("actual displaced checkpoint");
    changed
        .get_mut("block_index")
        .expect("block index")
        .insert(
            filter_index::active_key(checkpoint).into_bytes(),
            filter_index::encode_projection(checkpoint, stale.block_hash),
        )
        .expect("coherent BASIC projection");
    assert_eq!(
        check_retained_rows(archive, &changed, &records),
        Err(Refusal::MalformedRecord)
    );
    let mut changed = retained.clone();
    changed
        .get_mut("coins")
        .expect("coins")
        .remove(b"B".as_slice())
        .expect("coins best block");
    assert_eq!(
        check_retained_rows(archive, &changed, &records),
        Err(Refusal::MalformedRecord)
    );
    let mut changed = archive.clone();
    let key = block_key(deleted[0]).into_bytes();
    changed
        .get_mut("block_index")
        .expect("archive index")
        .remove(&key)
        .expect("replay input");
    assert!(matches!(
        prove_pruned_regtest(&changed, retained, now),
        Err(Refusal::MissingRecord)
    ));
}
