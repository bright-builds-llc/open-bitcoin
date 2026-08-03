// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use core::fmt;

use open_bitcoin_mempool::{PolicyConfig, PolicyTime};

use super::{FjallNodeStore, SNAPSHOT_KEY};
use crate::network::{
    CheckpointAbortDispatchError, CheckpointPersistenceStrength, EffectAbort, ManagedNetworkHandle,
    PreparedSnapshotWrite, SnapshotWriteAbort, SnapshotWriteAbortError, SnapshotWriteCapability,
    SnapshotWriteFailure, SnapshotWriteReceipt,
};
use crate::status::SyncRecoveryCategory;
use crate::storage::{
    MempoolSnapshot, PersistMode, StorageError, StorageNamespace, snapshot_codec,
};

/// Explicit resource limits for one persisted mempool snapshot load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MempoolSnapshotDecodeLimits {
    max_encoded_bytes: usize,
    max_records: usize,
    max_unbroadcast_members: usize,
    max_transaction_bytes: usize,
    max_total_transaction_bytes: usize,
}

impl MempoolSnapshotDecodeLimits {
    const MAX_UNBROADCAST_MEMBERS: usize = 5_000;
    const ENVELOPE_OVERHEAD_BYTES: usize = 1_048_576;

    /// Build a caller-owned bounded decode contract without implicit defaults.
    pub const fn new(
        max_encoded_bytes: usize,
        max_records: usize,
        max_unbroadcast_members: usize,
        max_transaction_bytes: usize,
        max_total_transaction_bytes: usize,
    ) -> Self {
        Self {
            max_encoded_bytes,
            max_records,
            max_unbroadcast_members,
            max_transaction_bytes,
            max_total_transaction_bytes,
        }
    }

    /// Derive bounded startup decode limits from the policy used by the live mempool.
    pub fn from_policy(policy: &PolicyConfig) -> Result<Self, SyncRecoveryCategory> {
        let record_capacity = policy.mempool_capacity.as_usize();
        let max_encoded_bytes = record_capacity
            .checked_mul(4)
            .and_then(|bytes| bytes.checked_add(Self::ENVELOPE_OVERHEAD_BYTES))
            .ok_or(SyncRecoveryCategory::ResourceExhaustion)?;

        Ok(Self::new(
            max_encoded_bytes,
            record_capacity,
            record_capacity.min(Self::MAX_UNBROADCAST_MEMBERS),
            policy.max_standard_tx_weight,
            record_capacity,
        ))
    }

    fn codec_limits(self) -> snapshot_codec::MempoolSnapshotDecodeLimits {
        snapshot_codec::MempoolSnapshotDecodeLimits {
            max_encoded_bytes: self.max_encoded_bytes,
            max_records: self.max_records,
            max_unbroadcast_members: self.max_unbroadcast_members,
            max_transaction_bytes: self.max_transaction_bytes,
            max_total_transaction_bytes: self.max_total_transaction_bytes,
        }
    }
}

/// Failure while carrying one snapshot capability to a truthful terminal state.
#[derive(Debug)]
pub enum SnapshotWriteExecutionError {
    /// Encoding failed and the exact reservation was aborted.
    Encode(StorageError),
    /// Durable storage failed and the exact reservation was aborted.
    Storage(StorageError),
    /// Pre-achievement abort construction rejected an impossible failure class.
    AbortConstruction {
        storage_error: StorageError,
        source: SnapshotWriteAbortError,
    },
    /// Encoding or storage failed and the exact abort could not dispatch.
    AbortDispatch {
        storage_error: StorageError,
        source: CheckpointAbortDispatchError,
    },
    /// Encoding or storage failed but authority rejected the owned exact abort.
    AbortRejected {
        failure: SnapshotWriteFailure,
        storage_error: StorageError,
        classification: EffectAbort,
    },
}

impl SnapshotWriteExecutionError {
    pub const fn failure(&self) -> SnapshotWriteFailure {
        match self {
            Self::Encode(_) => SnapshotWriteFailure::Encode,
            Self::Storage(_) => SnapshotWriteFailure::Storage,
            Self::AbortConstruction { .. } | Self::AbortDispatch { .. } => {
                SnapshotWriteFailure::AbortDispatch
            }
            Self::AbortRejected { failure, .. } => *failure,
        }
    }
}

impl fmt::Display for SnapshotWriteExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encode(error) => error.fmt(formatter),
            Self::Storage(error) => error.fmt(formatter),
            Self::AbortConstruction {
                storage_error,
                source,
            } => write!(
                formatter,
                "snapshot persistence failed ({storage_error}); exact abort construction failed: {source}"
            ),
            Self::AbortDispatch { storage_error, .. } => write!(
                formatter,
                "snapshot persistence failed ({storage_error}); exact abort dispatch failed"
            ),
            Self::AbortRejected {
                storage_error,
                classification,
                ..
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
            Self::Encode(error) | Self::Storage(error) => Some(error),
            Self::AbortConstruction { source, .. } => Some(source),
            Self::AbortDispatch { source, .. } => Some(source),
            Self::AbortRejected { storage_error, .. } => Some(storage_error),
        }
    }
}

impl FjallNodeStore {
    /// Persist one owned snapshot and terminate its capability through authority.
    ///
    /// Encoding or save failure aborts the exact pre-achievement reservation.
    /// Only a successful save converts the capability into an achieved receipt.
    pub fn execute_prepared_mempool_snapshot_write<Now>(
        &self,
        handle: &ManagedNetworkHandle,
        prepared: PreparedSnapshotWrite,
        now: Now,
    ) -> Result<SnapshotWriteReceipt, SnapshotWriteExecutionError>
    where
        Now: FnMut() -> PolicyTime,
    {
        execute_prepared_mempool_snapshot_write_with(
            handle,
            prepared,
            snapshot_codec::encode_mempool_snapshot,
            |bytes, mode| self.put_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY, bytes, mode),
            now,
        )
    }

    #[cfg(test)]
    pub(crate) fn execute_prepared_mempool_snapshot_write_with<Encode, Save, Now>(
        handle: &ManagedNetworkHandle,
        prepared: PreparedSnapshotWrite,
        encode: Encode,
        save: Save,
        now: Now,
    ) -> Result<SnapshotWriteReceipt, SnapshotWriteExecutionError>
    where
        Encode: FnOnce(&MempoolSnapshot) -> Result<Vec<u8>, StorageError>,
        Save: FnOnce(Vec<u8>, PersistMode) -> Result<(), StorageError>,
        Now: FnMut() -> PolicyTime,
    {
        execute_prepared_mempool_snapshot_write_with(handle, prepared, encode, save, now)
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

    /// Load the accepted-mempool snapshot within caller-supplied resource limits.
    pub fn load_mempool_snapshot_with_limits(
        &self,
        limits: MempoolSnapshotDecodeLimits,
    ) -> Result<Option<MempoolSnapshot>, StorageError> {
        self.get_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY)?
            .map(|bytes| {
                snapshot_codec::decode_mempool_snapshot_with_limits(&bytes, limits.codec_limits())
            })
            .transpose()
    }

    /// Remove the persisted accepted-mempool snapshot.
    pub fn clear_mempool_snapshot(&self, mode: PersistMode) -> Result<(), StorageError> {
        self.remove_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY, mode)
    }
}

fn execute_prepared_mempool_snapshot_write_with<Encode, Save, Now>(
    handle: &ManagedNetworkHandle,
    prepared: PreparedSnapshotWrite,
    encode: Encode,
    save: Save,
    mut now: Now,
) -> Result<SnapshotWriteReceipt, SnapshotWriteExecutionError>
where
    Encode: FnOnce(&MempoolSnapshot) -> Result<Vec<u8>, StorageError>,
    Save: FnOnce(Vec<u8>, PersistMode) -> Result<(), StorageError>,
    Now: FnMut() -> PolicyTime,
{
    let (snapshot, capability) = prepared.into_parts();
    let bytes = match encode(&snapshot) {
        Ok(bytes) => bytes,
        Err(error) => {
            return Err(abort_failed_write(
                handle,
                capability,
                error,
                now(),
                SnapshotWriteFailure::Encode,
            ));
        }
    };
    if let Err(error) = save(bytes, PersistMode::Sync) {
        return Err(abort_failed_write(
            handle,
            capability,
            error,
            now(),
            SnapshotWriteFailure::Storage,
        ));
    }

    Ok(capability.acknowledge_write(now(), CheckpointPersistenceStrength::Sync))
}

fn abort_failed_write(
    handle: &ManagedNetworkHandle,
    capability: SnapshotWriteCapability,
    storage_error: StorageError,
    failed_at: PolicyTime,
    failure: SnapshotWriteFailure,
) -> SnapshotWriteExecutionError {
    let abort = match SnapshotWriteAbort::new(capability, failed_at, failure) {
        Ok(abort) => abort,
        Err(source) => {
            return SnapshotWriteExecutionError::AbortConstruction {
                storage_error,
                source,
            };
        }
    };
    match handle.abort_snapshot_write(abort) {
        Ok(EffectAbort::Aborted) => match failure {
            SnapshotWriteFailure::Encode => SnapshotWriteExecutionError::Encode(storage_error),
            SnapshotWriteFailure::Storage => SnapshotWriteExecutionError::Storage(storage_error),
            SnapshotWriteFailure::AbortDispatch => SnapshotWriteExecutionError::AbortRejected {
                failure,
                storage_error,
                classification: EffectAbort::NotPending,
            },
        },
        Ok(classification) => SnapshotWriteExecutionError::AbortRejected {
            failure,
            storage_error,
            classification,
        },
        Err(source) => SnapshotWriteExecutionError::AbortDispatch {
            storage_error,
            source,
        },
    }
}
