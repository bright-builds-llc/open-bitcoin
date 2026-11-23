// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::{
    FjallNodeStore,
    canonical_genesis::HistoryOwners,
    inspection::{self, InspectedRecords, LogicalInventory},
    open_lease::OperationLease,
    tracked::{MutationStamp, RecordSource},
};
use crate::storage::{
    canonical_bootstrap::CanonicalBootstrapRefusal as Refusal,
    canonical_history::pruned::{PrunedRegtestProjection, prove_pruned_inspection},
};
use std::path::Path;

/// A pruned Regtest source paired with a replay of its complete archive.
pub(crate) struct OpenedPrunedRegtest {
    store: FjallNodeStore,
    stamp: MutationStamp,
    retained: InspectedRecords,
    proof: PrunedRegtestProjection,
}

impl FjallNodeStore {
    /// Prove the two closed sources in isolated scratch copies before opening the original.
    /// The archive supplies validation inputs. Its backend and payloads are never attached to the runtime.
    pub(crate) fn open_pruned_regtest_with_archive(
        retained_path: impl AsRef<Path>,
        archive_path: impl AsRef<Path>,
    ) -> Result<OpenedPrunedRegtest, Refusal> {
        // Release the archive lease before acquiring the original lease. Both observations
        // are detached, and replay independently validates every accepted archive input.
        let archive = {
            let lease = OperationLease::acquire(archive_path.as_ref()).map_err(unavailable)?;
            inspection::inspect_prune_proof(&lease).map_err(unavailable)?
        };
        let lease = OperationLease::acquire(retained_path.as_ref()).map_err(unavailable)?;
        let retained = inspection::inspect_prune_proof(&lease).map_err(unavailable)?;
        let proof = prove_pruned_inspection(&archive, &retained)?;
        let store = Self::open_with_lease(lease).map_err(unavailable)?;
        let stamp = store.db.stamp().map_err(|_| Refusal::ChangedStore)?;
        verify_original(&store, &stamp, &retained)?;
        Ok(OpenedPrunedRegtest {
            store,
            stamp,
            retained,
            proof,
        })
    }
}

impl OpenedPrunedRegtest {
    pub(crate) fn into_parts(self) -> Result<(FjallNodeStore, HistoryOwners), Refusal> {
        verify_original(&self.store, &self.stamp, &self.retained)?;
        self.store
            .recover_validation_history()
            .map_err(unavailable)?;
        Ok((
            self.store,
            (self.proof.genesis, self.proof.headers, self.proof.replay),
        ))
    }
}

fn verify_original(
    store: &FjallNodeStore,
    stamp: &MutationStamp,
    expected: &InspectedRecords,
) -> Result<(), Refusal> {
    let guard = store.filter_publication_guard().map_err(unavailable)?;
    store
        .db
        .check_stamp(stamp)
        .map_err(|_| Refusal::ChangedStore)?;
    let actual = store
        .db
        .read_existing_records(LogicalInventory::RegtestPruneProof)
        .map_err(unavailable)?;
    if actual != *expected {
        return Err(Refusal::ChangedStore);
    }
    store
        .db
        .check_stamp(stamp)
        .map_err(|_| Refusal::ChangedStore)?;
    drop(guard);
    Ok(())
}

fn unavailable<T>(_: T) -> Refusal {
    Refusal::StorageUnavailable
}
