// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use core::fmt;
use std::collections::HashMap;

use open_bitcoin_core::{
    chainstate::ChainstateSnapshot,
    consensus::{block_hash, block_merkle_root, transaction_txid},
};
use open_bitcoin_mempool::PolicyTime;

use super::{FjallNodeStore, SNAPSHOT_KEY};
use crate::network::{
    CheckpointAbortDispatchError, CheckpointPersistenceStrength, EffectAbort, ManagedNetworkHandle,
    PreparedSnapshotWrite, SnapshotWriteAbort, SnapshotWriteAbortError, SnapshotWriteCapability,
    SnapshotWriteFailure, SnapshotWriteReceipt,
};
use crate::status::SyncRecoveryCategory;
use crate::storage::mempool_snapshot::MempoolSnapshotError;
use crate::storage::{
    MempoolSnapshot, PersistMode, StorageError, StorageNamespace, StorageRecoveryAction,
    snapshot_codec,
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

    /// Build the stable finite decode contract owned by the persisted format.
    pub fn for_persisted_input() -> Result<Self, SyncRecoveryCategory> {
        let limits: snapshot_codec::MempoolSnapshotPersistedInputLimits =
            snapshot_codec::persisted_mempool_input_limits()
                .ok_or(SyncRecoveryCategory::ResourceExhaustion)?;
        Ok(Self::new(
            limits.max_encoded_bytes,
            limits.max_records,
            limits.max_unbroadcast_members,
            limits.max_transaction_bytes,
            limits.max_total_transaction_bytes,
        ))
    }

    pub fn into_codec_limits(self) -> snapshot_codec::MempoolSnapshotDecodeLimits {
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

    pub(crate) fn into_abort_dispatch_parts(
        self,
    ) -> Result<
        (
            StorageError,
            crate::network::ManagedNetworkAuthorityError,
            SnapshotWriteAbort,
        ),
        Self,
    > {
        match self {
            Self::AbortDispatch {
                storage_error,
                source,
            } => {
                let (source, abort) = source.into_parts();
                Ok((storage_error, source, abort))
            }
            error => Err(error),
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
    /// Load chainstate and migrate legacy confirmation evidence from durable active-chain blocks.
    pub fn load_chainstate_snapshot_with_confirmation_migration(
        &self,
    ) -> Result<Option<ChainstateSnapshot>, StorageError> {
        let Some(mut snapshot) = self.load_chainstate_snapshot()? else {
            return Ok(None);
        };
        if snapshot.maybe_confirmed_txid_counts.is_some() {
            return Ok(Some(snapshot));
        }

        let mut confirmed_txid_counts = HashMap::new();
        for position in &snapshot.active_chain {
            let Some(block) = self.load_block(position.block_hash)? else {
                return Err(super::corruption(
                    StorageNamespace::Chainstate,
                    format_args!(
                        "missing active-chain block {:?} required for confirmation migration",
                        position.block_hash
                    ),
                ));
            };
            let actual_hash = block_hash(&block.header);
            if actual_hash != position.block_hash {
                return Err(super::corruption(
                    StorageNamespace::Chainstate,
                    format_args!(
                        "active-chain block identity mismatch: expected {:?}, loaded {actual_hash:?}",
                        position.block_hash
                    ),
                ));
            }
            let (actual_merkle_root, maybe_mutated) = block_merkle_root(&block.transactions)
                .map_err(|_| {
                    super::corruption(
                        StorageNamespace::Chainstate,
                        "failed to derive active-chain Merkle commitment during confirmation migration",
                    )
                })?;
            if maybe_mutated {
                return Err(super::corruption(
                    StorageNamespace::Chainstate,
                    "mutated active-chain transaction tree during confirmation migration",
                ));
            }
            if actual_merkle_root != block.header.merkle_root {
                return Err(super::corruption(
                    StorageNamespace::Chainstate,
                    "active-chain block Merkle commitment mismatch during confirmation migration",
                ));
            }
            for transaction in &block.transactions {
                let txid = transaction_txid(transaction).map_err(|error| {
                    super::corruption(
                        StorageNamespace::Chainstate,
                        format_args!(
                            "failed to derive transaction identity during confirmation migration: {error}"
                        ),
                    )
                })?;
                let count = confirmed_txid_counts.entry(txid).or_insert(0_u32);
                *count = count.checked_add(1).ok_or_else(|| {
                    super::corruption(
                        StorageNamespace::Chainstate,
                        format_args!("active-chain occurrence count overflow for {txid:?}"),
                    )
                })?;
            }
        }
        snapshot.maybe_confirmed_txid_counts = Some(confirmed_txid_counts);
        self.save_chainstate_snapshot(&snapshot, PersistMode::Sync)?;
        Ok(Some(snapshot))
    }

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
        load_mempool_snapshot_with(
            limits,
            || {
                self.keyspace(StorageNamespace::Mempool)
                    .size_of(SNAPSHOT_KEY)
                    .map(|maybe_size| maybe_size.map(|size| size as usize))
                    .map_err(|error| super::backend_failure(StorageNamespace::Mempool, error))
            },
            || {
                self.keyspace(StorageNamespace::Mempool)
                    .get(SNAPSHOT_KEY)
                    .map_err(|error| super::backend_failure(StorageNamespace::Mempool, error))
            },
        )
    }

    /// Remove the persisted accepted-mempool snapshot.
    pub fn clear_mempool_snapshot(&self, mode: PersistMode) -> Result<(), StorageError> {
        self.remove_bytes(StorageNamespace::Mempool, SNAPSHOT_KEY, mode)
    }
}

fn load_mempool_snapshot_with<Size, Load, Bytes>(
    limits: MempoolSnapshotDecodeLimits,
    size: Size,
    load: Load,
) -> Result<Option<MempoolSnapshot>, StorageError>
where
    Size: FnOnce() -> Result<Option<usize>, StorageError>,
    Load: FnOnce() -> Result<Option<Bytes>, StorageError>,
    Bytes: AsRef<[u8]>,
{
    let Some(encoded_size) = size()? else {
        return Ok(None);
    };
    if encoded_size > limits.max_encoded_bytes {
        return Err(resource_bound_failure());
    }
    load()?
        .map(|bytes| {
            snapshot_codec::decode_mempool_snapshot_with_limits(
                bytes.as_ref(),
                limits.into_codec_limits(),
            )
        })
        .transpose()
}

fn resource_bound_failure() -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::Mempool,
        detail: MempoolSnapshotError::ResourceBoundExceeded.to_string(),
        action: StorageRecoveryAction::Repair,
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

#[cfg(test)]
mod bounded_load_tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn oversized_value_is_rejected_before_value_loading() {
        // Arrange
        let limits = MempoolSnapshotDecodeLimits::new(8, 1, 1, 8, 8);
        let load_called = Cell::new(false);

        // Act
        let error = load_mempool_snapshot_with(
            limits,
            || Ok(Some(9)),
            || -> Result<Option<&'static [u8]>, StorageError> {
                load_called.set(true);
                Ok(Some(b"ignored"))
            },
        )
        .expect_err("oversized value must fail before loading");

        // Assert
        assert!(!load_called.get());
        assert!(matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::Mempool,
                ..
            }
        ));
    }

    #[test]
    fn persisted_input_limits_use_the_finite_format_contract() {
        // Arrange
        let expected =
            MempoolSnapshotDecodeLimits::new(268_435_456, 220_096, 5_000, 4_194_304, 67_108_864);

        // Act
        let limits =
            MempoolSnapshotDecodeLimits::for_persisted_input().expect("persisted input limits");

        // Assert
        assert_eq!(limits, expected);
    }
}
