// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;
use crate::storage::{
    CanonicalBootstrapRefusal as Refusal,
    canonical_bootstrap::{
        self as bootstrap, ProvedGenesisRecords, records::CanonicalGenesisRecords,
    },
    chain_compatibility::{DatadirObservation, inspect_datadir_without_backend},
};
use open_bitcoin_core::{
    chainstate::ChainPosition,
    consensus::chain_spec::{CanonicalChain, CanonicalGenesisProof},
};

/// Exclusive proved genesis storage; no raw store access is exposed.
pub struct OpenedCanonicalGenesis {
    store: FjallNodeStore,
    recovered: RecoveredCanonicalGenesis,
}

/// Unforgeable evidence bound to one live database instance and write generation.
pub(crate) struct RecoveredCanonicalGenesis {
    publication: Arc<Mutex<filters::PublicationControl>>,
    sequence: u64,
    proof: ProvedGenesisRecords,
}

impl OpenedCanonicalGenesis {
    /// Complete proved genesis, including exact work; confers no history support.
    pub fn genesis(&self) -> &CanonicalGenesisProof {
        self.recovered.proof.genesis()
    }

    /// Proved height-zero active position.
    pub fn position(&self) -> &ChainPosition {
        self.recovered.proof.position()
    }

    /// Consume the sealed owner at the managed-runtime boundary.
    pub(crate) fn into_parts(self) -> Result<(FjallNodeStore, RecoveredCanonicalGenesis), Refusal> {
        self.recovered.check(&self.store)?;
        Ok((self.store, self.recovered))
    }
}

impl RecoveredCanonicalGenesis {
    /// Recheck the original instance, every intervening write and complete facts.
    pub(crate) fn check(&self, store: &FjallNodeStore) -> Result<&CanonicalGenesisProof, Refusal> {
        if !Arc::ptr_eq(&self.publication, &store.filter_publication) {
            return Err(Refusal::ChangedStore);
        }
        let _guard = store.filter_publication_guard().map_err(unavailable)?;
        if store.db.seqno() != self.sequence
            || prove_live(store, self.proof.genesis().chain())? != self.proof
            || store.db.seqno() != self.sequence
        {
            return Err(Refusal::ChangedStore);
        }
        Ok(self.proof.genesis())
    }
}

impl FjallNodeStore {
    /// Initialize absent storage or prove existing canonical genesis storage.
    /// Cooperative application openers are excluded throughout inspection and
    /// handoff. Arbitrary direct-engine/filesystem writers are outside this contract.
    pub fn open_canonical_genesis(
        path: impl AsRef<Path>,
        selected: CanonicalChain,
    ) -> Result<OpenedCanonicalGenesis, Refusal> {
        open_selected(path.as_ref(), selected, true)
    }

    /// Prove existing canonical storage without initializing absent storage.
    pub fn reopen_canonical_genesis_existing(
        path: impl AsRef<Path>,
        selected: CanonicalChain,
    ) -> Result<OpenedCanonicalGenesis, Refusal> {
        open_selected(path.as_ref(), selected, false)
    }
}

fn open_selected(
    path: &Path,
    selected: CanonicalChain,
    initialize: bool,
) -> Result<OpenedCanonicalGenesis, Refusal> {
    let lease = open_lease::OperationLease::acquire(path).map_err(unavailable)?;
    match inspect_datadir_without_backend(lease.source()) {
        DatadirObservation::Missing | DatadirObservation::EmptyDirectory if initialize => {
            let rows = bootstrap::prepare_genesis_rows(selected)?;
            let store = FjallNodeStore::open_with_lease(lease).map_err(unavailable)?;
            // All data encodings and logical proof precede the first insertion.
            let mut batch = store.db.batch().durability(Some(FjallPersistMode::SyncAll));
            for (namespace, records) in rows {
                let keyspace = match namespace {
                    "schema" => &store.schema,
                    "headers" => &store.headers,
                    "block_index" => &store.block_index,
                    "chainstate" => &store.chainstate,
                    "coins" => &store.coins,
                    _ => return Err(Refusal::UnsupportedRecord),
                };
                for (key, value) in records {
                    batch.insert(keyspace, key, value);
                }
            }
            #[cfg(test)]
            fault(Fault::BeforeCommit)?;
            #[cfg(test)]
            let before = store.db.seqno();
            batch.commit().map_err(|_| Refusal::CommitUncertain)?;
            #[cfg(test)]
            LAST_BATCH_SEQUENCES.set(Some((before, store.db.seqno())));
            #[cfg(test)]
            fault(Fault::AfterCommit)?;
            let proof = prove_live(&store, selected)?;
            seal(store, proof)
        }
        DatadirObservation::Missing | DatadirObservation::EmptyDirectory => {
            Err(Refusal::MissingRecord)
        }
        DatadirObservation::ExistingUnproved => reopen(lease, selected),
        _ => Err(Refusal::StorageUnavailable),
    }
}

fn reopen(
    lease: open_lease::OperationLease,
    selected: CanonicalChain,
) -> Result<OpenedCanonicalGenesis, Refusal> {
    let mut maybe_refusal = None;
    let expected = inspection::with_inspection(&lease, |records| {
        prove_detached(records, selected).map_err(|refusal| {
            maybe_refusal = Some(refusal);
            corruption(
                StorageNamespace::Runtime,
                "canonical bootstrap proof refused",
            )
        })
    })
    .map_err(|_| maybe_refusal.unwrap_or(Refusal::StorageUnavailable))?;
    // The inspection helper returned only after checked cleanup and source recheck.
    // Its temporary source-engine lock is now released; our app lease never was.
    let store = FjallNodeStore::open_with_lease(lease).map_err(unavailable)?;
    #[cfg(test)]
    if MAYBE_FAULT.get() == Some(Fault::OriginalProof) {
        MAYBE_FAULT.set(None);
        store
            .schema
            .insert(bootstrap::records::IDENTITY_KEY, b"changed")
            .map_err(unavailable)?;
    }
    #[cfg(test)]
    if MAYBE_FAULT.get() == Some(Fault::DifferentProof) {
        MAYBE_FAULT.set(None);
        store
            .block_index
            .insert(
                PRUNE_LOCKS_KEY,
                encode_prune_locks(&[]).map_err(unavailable)?,
            )
            .map_err(unavailable)?;
    }
    let actual = prove_live(&store, selected)?;
    if actual != expected {
        return Err(Refusal::ChangedStore);
    }
    // No schema migration, history hydration, pruning or index recovery preceded proof.
    seal(store, actual)
}

fn prove_detached(
    records: &inspection::InspectedRecords,
    selected: CanonicalChain,
) -> Result<ProvedGenesisRecords, Refusal> {
    bootstrap::validate_genesis_records(selected, &CanonicalGenesisRecords::decode(records)?)
}

fn prove_live(
    store: &FjallNodeStore,
    selected: CanonicalChain,
) -> Result<ProvedGenesisRecords, Refusal> {
    prove_detached(
        &inspection::InspectedRecords::read_existing(&store.db).map_err(unavailable)?,
        selected,
    )
}

fn seal(
    store: FjallNodeStore,
    proof: ProvedGenesisRecords,
) -> Result<OpenedCanonicalGenesis, Refusal> {
    // Complete current coverage was already proved. This initializes only its
    // existing in-memory invalidation owner, never ordinary-history admission.
    store.recover_validation_history().map_err(unavailable)?;
    if prove_live(&store, proof.genesis().chain())? != proof {
        return Err(Refusal::ChangedStore);
    }
    let recovered = RecoveredCanonicalGenesis {
        publication: Arc::clone(&store.filter_publication),
        sequence: store.db.seqno(),
        proof,
    };
    Ok(OpenedCanonicalGenesis { store, recovered })
}

fn unavailable<T>(_: T) -> Refusal {
    Refusal::StorageUnavailable
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Fault {
    BeforeCommit,
    AfterCommit,
    OriginalProof,
    DifferentProof,
}

#[cfg(test)]
thread_local! {
    static MAYBE_FAULT: std::cell::Cell<Option<Fault>> = const { std::cell::Cell::new(None) };
    static LAST_BATCH_SEQUENCES: std::cell::Cell<Option<(u64, u64)>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn fault(point: Fault) -> Result<(), Refusal> {
    if MAYBE_FAULT.get() != Some(point) {
        return Ok(());
    }
    MAYBE_FAULT.set(None);
    Err(if point == Fault::AfterCommit {
        Refusal::CommitUncertain
    } else {
        Refusal::StorageUnavailable
    })
}

#[cfg(test)]
mod tests;
