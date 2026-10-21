// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::{
    CanonicalBootstrapRefusal as Refusal,
    records::{CanonicalGenesisRecords, Rows},
};
use crate::storage::{filter_index as filters, fjall_store, validation_history as history};
use open_bitcoin_core::{
    chainstate::{
        BASIC_INDEX_PRUNE_LOCK, BasicFilterInputs, ChainPosition, IndexInputProtection,
        filter_index::lifecycle::IndexLifecycle,
    },
    consensus::{
        chain_proof::ProofWork,
        chain_spec::{CanonicalChain, CanonicalGenesisProof},
    },
};
use open_bitcoin_network::HeaderEntry;

/// Complete data-derived evidence, deliberately unbound to any live store.
#[derive(PartialEq, Eq)]
pub(in crate::storage) struct ProvedGenesisRecords {
    genesis: CanonicalGenesisProof,
    position: ChainPosition,
    basic: Rows,
}

impl ProvedGenesisRecords {
    pub(in crate::storage) fn genesis(&self) -> &CanonicalGenesisProof {
        &self.genesis
    }
    pub(in crate::storage) fn position(&self) -> &ChainPosition {
        &self.position
    }
}

pub(in crate::storage) fn validate_genesis_records(
    selected: CanonicalChain,
    records: &CanonicalGenesisRecords,
) -> Result<ProvedGenesisRecords, Refusal> {
    if records.chain != selected {
        return Err(Refusal::InconsistentGenesis);
    }
    let genesis = CanonicalGenesisProof::prove(selected.spec(), &records.body)
        .map_err(|_| Refusal::InconsistentGenesis)?;
    // Refuse descendants before interpreting their numerical endpoints as roots.
    if records.headers.len() != 1 || records.index.len() != 1 || records.active.len() != 1 {
        return Err(Refusal::UnsupportedHistory);
    }
    if records.identity_hash != genesis.hash()
        || records.best_block != genesis.hash()
        || records.body_key != fjall_store::block_key(genesis.hash())
    {
        return Err(Refusal::InconsistentGenesis);
    }
    // The proved root supplies the continuous height-zero/null-parent ancestry.
    // Compare the full-width accumulation to every stored numerical endpoint.
    let work = ProofWork::ZERO.wrapping_add(genesis.work());
    let position = ChainPosition::genesis(
        genesis.header().clone(),
        work,
        i64::from(genesis.header().time),
    )
    .map_err(|_| Refusal::InconsistentGenesis)?;
    let expected_header = HeaderEntry {
        block_hash: genesis.hash(),
        header: genesis.header().clone(),
        height: 0,
        chain_work: work,
    };
    let identity = records.validation.identity();
    if records.headers[0] != expected_header
        || records.index[0] != expected_header
        || records.active[0] != position
        || records.coverage != history::ValidationCoverage::Complete
        || records.validation.status() != history::StoredValidationStatus::ScriptsValid
        || identity.hash() != genesis.hash()
        || identity.parent_hash() != position.previous_block_hash()
        || identity.height() != 0
    {
        return Err(Refusal::InconsistentGenesis);
    }
    validate_basic(&genesis, &position, &records.basic)?;
    Ok(ProvedGenesisRecords {
        genesis,
        position,
        basic: records.basic.clone(),
    })
}

fn validate_basic(
    genesis: &CanonicalGenesisProof,
    position: &ChainPosition,
    rows: &Rows,
) -> Result<(), Refusal> {
    let maybe_locks = rows.get(fjall_store::PRUNE_LOCKS_KEY.as_bytes());
    let maybe_decoded_locks = maybe_locks
        .map(|bytes| fjall_store::decode_prune_locks(bytes))
        .transpose()
        .map_err(|_| Refusal::InconsistentIndex)?;
    let locks = maybe_decoded_locks.as_deref().unwrap_or(&[]);
    if let Some(bytes) = maybe_locks
        && fjall_store::encode_prune_locks(locks).map_err(|_| Refusal::InconsistentIndex)? != *bytes
    {
        return Err(Refusal::InconsistentIndex);
    }
    for lock in locks {
        if lock.height_first > lock.height_last {
            return Err(Refusal::InconsistentIndex);
        }
    }
    let maybe_lock = locks
        .iter()
        .find(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK);
    let maybe_state = rows.get(filters::STATE_KEY.as_bytes());
    let Some(state_bytes) = maybe_state else {
        if rows
            .keys()
            .any(|key| key.as_slice() != fjall_store::PRUNE_LOCKS_KEY.as_bytes())
            || maybe_lock.is_some()
        {
            return Err(Refusal::InconsistentIndex);
        }
        return Ok(());
    };
    let state = filters::decode_state(state_bytes).map_err(|_| Refusal::InconsistentIndex)?;
    let owner_bytes = rows
        .get(filters::ownership::OWNER_KEY.as_bytes())
        .ok_or(Refusal::InconsistentIndex)?;
    let owner =
        filters::ownership::decode_owner(owner_bytes).map_err(|_| Refusal::InconsistentIndex)?;
    if state.fence_height != 0 || state.fence_hash != genesis.hash() {
        return Err(Refusal::InconsistentIndex);
    }
    let required_protection = match state.maybe_endpoint {
        None => IndexInputProtection::FromHeight(0),
        Some((0, hash)) if hash == genesis.hash() => IndexInputProtection::FromHeight(1),
        _ => return Err(Refusal::UnsupportedHistory),
    };
    if !state.protection.covers(required_protection) {
        return Err(Refusal::InconsistentIndex);
    }
    let maybe_lock_protection = maybe_lock
        .map(IndexInputProtection::from_saved_lock)
        .transpose()
        .map_err(|_| Refusal::InconsistentIndex)?;
    if maybe_lock_protection.is_some_and(|protection| !protection.covers(state.protection))
        || matches!(owner, IndexLifecycle::Active { .. }) && maybe_lock_protection.is_none()
    {
        return Err(Refusal::InconsistentIndex);
    }
    // Exactly the existing state/owner/optional locks plus one canonical pair.
    let expected_count =
        2 + usize::from(maybe_locks.is_some()) + if state.maybe_endpoint.is_some() { 2 } else { 0 };
    if rows.len() != expected_count {
        return Err(Refusal::InconsistentIndex);
    }
    let Some(_) = state.maybe_endpoint else {
        return Ok(());
    };
    let key = filters::record_key(genesis.hash());
    let bytes = rows.get(key.as_bytes()).ok_or(Refusal::InconsistentIndex)?;
    let saved =
        filters::decode_record(&key, bytes, None).map_err(|_| Refusal::InconsistentIndex)?;
    let inputs = BasicFilterInputs::from_historical(genesis.block(), position, None)
        .map_err(|_| Refusal::InconsistentIndex)?;
    let expected = filters::StoredFilterRecord::generate(&inputs, position, None)
        .map_err(|_| Refusal::InconsistentIndex)?;
    if saved != expected {
        return Err(Refusal::InconsistentIndex);
    }
    let key = filters::active_key(0);
    let bytes = rows.get(key.as_bytes()).ok_or(Refusal::InconsistentIndex)?;
    if filters::decode_projection(&key, bytes).map_err(|_| Refusal::InconsistentIndex)?
        != genesis.hash()
    {
        return Err(Refusal::InconsistentIndex);
    }
    Ok(())
}
