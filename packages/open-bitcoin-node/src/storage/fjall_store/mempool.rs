// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use core::fmt;

use super::{FjallNodeStore, SNAPSHOT_KEY};
use crate::network::{
    EffectAbort, EffectCompletion, ManagedNetworkAuthorityError, ManagedNetworkHandle,
    PreparedSnapshotWrite, SnapshotWriteCapability,
};
use crate::storage::{
    MempoolSnapshot, PersistMode, StorageError, StorageNamespace, snapshot_codec,
};

/// Failure while carrying one snapshot capability to a truthful terminal state.
#[derive(Debug)]
pub enum SnapshotWriteExecutionError {
    /// Encoding or durable storage failed and the exact reservation was aborted.
    Storage(StorageError),
    /// Durable storage succeeded, but achieved-effect completion could not dispatch.
    Completion(ManagedNetworkAuthorityError),
    /// Storage failed and the exact pre-achievement abort could not dispatch.
    AbortFailed {
        storage_error: StorageError,
        abort_error: ManagedNetworkAuthorityError,
    },
    /// Storage failed but authority rejected the owned capability's exact abort.
    AbortRejected {
        storage_error: StorageError,
        classification: EffectAbort,
    },
}

impl fmt::Display for SnapshotWriteExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(error) => error.fmt(formatter),
            Self::Completion(error) => {
                write!(
                    formatter,
                    "snapshot persisted but completion failed: {error}"
                )
            }
            Self::AbortFailed {
                storage_error,
                abort_error,
            } => write!(
                formatter,
                "snapshot persistence failed ({storage_error}); exact abort also failed: {abort_error}"
            ),
            Self::AbortRejected {
                storage_error,
                classification,
            } => write!(
                formatter,
                "snapshot persistence failed ({storage_error}); exact abort returned {classification:?}"
            ),
        }
    }
}

impl std::error::Error for SnapshotWriteExecutionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Storage(error) => Some(error),
            Self::Completion(error) => Some(error),
            Self::AbortFailed { abort_error, .. } => Some(abort_error),
            Self::AbortRejected { storage_error, .. } => Some(storage_error),
        }
    }
}

impl FjallNodeStore {
    /// Persist one owned snapshot and terminate its capability through authority.
    ///
    /// Encoding or save failure aborts the exact pre-achievement reservation.
    /// Only a successful save converts the capability into an achieved receipt.
    pub fn execute_prepared_mempool_snapshot_write(
        &self,
        handle: &ManagedNetworkHandle,
        prepared: PreparedSnapshotWrite,
        mode: PersistMode,
    ) -> Result<EffectCompletion, SnapshotWriteExecutionError> {
        execute_prepared_mempool_snapshot_write_with(
            handle,
            prepared,
            mode,
            snapshot_codec::encode_mempool_snapshot,
            |bytes, mode| self.put_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY, bytes, mode),
        )
    }

    #[cfg(test)]
    pub(crate) fn execute_prepared_mempool_snapshot_write_with<Encode, Save>(
        handle: &ManagedNetworkHandle,
        prepared: PreparedSnapshotWrite,
        mode: PersistMode,
        encode: Encode,
        save: Save,
    ) -> Result<EffectCompletion, SnapshotWriteExecutionError>
    where
        Encode: FnOnce(&MempoolSnapshot) -> Result<Vec<u8>, StorageError>,
        Save: FnOnce(Vec<u8>, PersistMode) -> Result<(), StorageError>,
    {
        execute_prepared_mempool_snapshot_write_with(handle, prepared, mode, encode, save)
    }

    /// Persist the accepted-mempool snapshot owned by Open Bitcoin.
    pub fn save_mempool_snapshot(
        &self,
        snapshot: &MempoolSnapshot,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let bytes = snapshot_codec::encode_mempool_snapshot(snapshot)?;
        self.put_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY, bytes, mode)
    }

    /// Load the accepted-mempool snapshot, if present.
    pub fn load_mempool_snapshot(&self) -> Result<Option<MempoolSnapshot>, StorageError> {
        self.get_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY)?
            .map(|bytes| snapshot_codec::decode_mempool_snapshot(&bytes))
            .transpose()
    }

    /// Remove the persisted accepted-mempool snapshot.
    pub fn clear_mempool_snapshot(&self, mode: PersistMode) -> Result<(), StorageError> {
        self.remove_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY, mode)
    }
}

fn execute_prepared_mempool_snapshot_write_with<Encode, Save>(
    handle: &ManagedNetworkHandle,
    prepared: PreparedSnapshotWrite,
    mode: PersistMode,
    encode: Encode,
    save: Save,
) -> Result<EffectCompletion, SnapshotWriteExecutionError>
where
    Encode: FnOnce(&MempoolSnapshot) -> Result<Vec<u8>, StorageError>,
    Save: FnOnce(Vec<u8>, PersistMode) -> Result<(), StorageError>,
{
    let (snapshot, capability) = prepared.into_parts();
    let bytes = match encode(&snapshot) {
        Ok(bytes) => bytes,
        Err(error) => return Err(abort_failed_write(handle, capability, error)),
    };
    if let Err(error) = save(bytes, mode) {
        return Err(abort_failed_write(handle, capability, error));
    }

    handle
        .complete_snapshot_write(capability.acknowledge_write())
        .map_err(SnapshotWriteExecutionError::Completion)
}

fn abort_failed_write(
    handle: &ManagedNetworkHandle,
    capability: SnapshotWriteCapability,
    storage_error: StorageError,
) -> SnapshotWriteExecutionError {
    match handle.abort_snapshot_write(capability) {
        Ok(EffectAbort::Aborted) => SnapshotWriteExecutionError::Storage(storage_error),
        Ok(classification) => SnapshotWriteExecutionError::AbortRejected {
            storage_error,
            classification,
        },
        Err(abort_error) => SnapshotWriteExecutionError::AbortFailed {
            storage_error,
            abort_error,
        },
    }
}
