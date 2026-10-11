// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

//! Constant-size retained immutable-row observation, separate from index authority.

pub(crate) use super::publication::BasicFilterReadFault;
use super::{FjallNodeStore, PublicationControl, StorageError, codec, index_corruption};
use crate::status::{BasicIndexUnavailableReason, BasicRetentionBasis, BasicRetentionMeasurement};
use open_bitcoin_core::chainstate::filter_index::catch_up::{
    BASIC_INDEX_MAX_CANDIDATES, BASIC_INDEX_MAX_ENCODED_BYTES,
    BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES,
};

/// Observation budgets do not change mandatory integrity/recovery limits.
#[derive(Clone, Copy)]
pub(super) struct RetentionObservationLimits {
    pub rows: u64,
    pub key_value_bytes: u64,
}

impl Default for RetentionObservationLimits {
    fn default() -> Self {
        Self {
            rows: 2_000_000,
            key_value_bytes: 256 * 1024 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RetainedBasicFilterObservation {
    #[default]
    Incomplete,
    Failed,
    BudgetExceeded,
    Complete(BasicRetentionMeasurement),
}

impl RetainedBasicFilterObservation {
    /// Reuse admitted, validated unique missing-row buffers without copying payloads.
    /// Unknown starting totals cannot be reconstructed from a partial delta.
    pub(super) fn checked_append(
        self,
        records: &[(String, Vec<u8>)],
    ) -> Result<Self, StorageError> {
        let mut next = self;
        for (key, bytes) in records {
            let payload = bytes
                .len()
                .checked_sub(codec::RECORD_OVERHEAD)
                .ok_or_else(|| index_corruption("BASIC retention invalid envelope"))?;
            next.observe_lengths(
                key.len() as u64,
                bytes.len() as u64,
                payload as u64,
                RetentionObservationLimits {
                    rows: u64::MAX,
                    key_value_bytes: u64::MAX,
                },
            );
            if matches!(self, Self::Complete(_)) && matches!(next, Self::Failed) {
                return Err(index_corruption("BASIC retention arithmetic overflow"));
            }
        }
        Ok(next)
    }

    pub(super) const fn empty() -> Self {
        Self::Complete(BasicRetentionMeasurement {
            retained_record_count: 0,
            encoded_filter_payload_bytes: 0,
            retained_record_key_value_bytes: 0,
            basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
        })
    }

    /// Borrow validated primary-row lengths only; parent/projection reads never count.
    pub(super) fn observe_lengths(
        &mut self,
        key: u64,
        value: u64,
        payload: u64,
        limits: RetentionObservationLimits,
    ) {
        let Self::Complete(previous) = *self else {
            return;
        };
        let Some((count, payload, bytes)) = previous
            .retained_record_count
            .checked_add(1)
            .zip(previous.encoded_filter_payload_bytes.checked_add(payload))
            .zip(
                key.checked_add(value)
                    .and_then(|n| previous.retained_record_key_value_bytes.checked_add(n)),
            )
            .map(|((count, payload), bytes)| (count, payload, bytes))
        else {
            *self = Self::Failed;
            return;
        };
        if count > limits.rows || bytes > limits.key_value_bytes {
            *self = Self::BudgetExceeded;
            return;
        }
        *self = Self::Complete(BasicRetentionMeasurement {
            retained_record_count: count,
            encoded_filter_payload_bytes: payload,
            retained_record_key_value_bytes: bytes,
            ..previous
        });
    }

    pub(crate) fn measurement(
        self,
    ) -> Result<BasicRetentionMeasurement, BasicIndexUnavailableReason> {
        match self {
            Self::Complete(value) => Ok(value),
            Self::Incomplete => Err(BasicIndexUnavailableReason::RetentionIncomplete),
            Self::Failed => Err(BasicIndexUnavailableReason::RetentionFailed),
            Self::BudgetExceeded => Err(BasicIndexUnavailableReason::RetentionBudgetExceeded),
        }
    }
}

impl PublicationControl {
    /// Refuse effects and revoke a total that cannot represent complete growth.
    pub(super) fn retention_after_append(
        &mut self,
        starting: RetainedBasicFilterObservation,
        records: &[(String, Vec<u8>)],
    ) -> Result<RetainedBasicFilterObservation, StorageError> {
        starting.checked_append(records).inspect_err(|_| {
            self.retention = RetainedBasicFilterObservation::Failed;
        })
    }

    /// Only complete exclusive validation or known empty initialization earns trust.
    pub(super) fn install_retention(&mut self, retained: RetainedBasicFilterObservation) {
        self.retention = retained;
    }
}

impl FjallNodeStore {
    /// One publication lifetime, two fixed metadata values, and no record/lock walks.
    pub(crate) fn basic_filter_operator_observation(
        &self,
    ) -> Result<BasicFilterOperatorObservation, BasicIndexUnavailableReason> {
        self.basic_filter_operator_observation_with_limits(8, 4096)
    }

    pub(crate) fn basic_filter_operator_observation_with_limits(
        &self,
        reads: u64,
        bytes: u64,
    ) -> Result<BasicFilterOperatorObservation, BasicIndexUnavailableReason> {
        let control = self
            .filter_publication_guard()
            .map_err(|_| BasicIndexUnavailableReason::Backend)?;
        #[cfg(test)]
        let mut control = control;
        #[cfg(test)]
        {
            control.retention_inspections += 1;
        }
        let retention = if control.read_integrity {
            control.retention.measurement()
        } else {
            Err(match control.retention {
                RetainedBasicFilterObservation::Failed => {
                    BasicIndexUnavailableReason::RetentionFailed
                }
                _ => BasicIndexUnavailableReason::RetentionIncomplete,
            })
        };
        let retained_availability = if control.read_integrity {
            match control.retention {
                RetainedBasicFilterObservation::Complete(value) => {
                    Ok(if value.retained_record_count == 0 {
                        crate::status::BasicRetainedAvailability::NoneSeen
                    } else {
                        crate::status::BasicRetainedAvailability::Observed
                    })
                }
                RetainedBasicFilterObservation::BudgetExceeded => {
                    Ok(crate::status::BasicRetainedAvailability::Observed)
                }
                _ => Err(BasicIndexUnavailableReason::RetentionFailed),
            }
        } else {
            Err(BasicIndexUnavailableReason::RetentionIncomplete)
        };
        let mut work = BasicOperatorObservationWork::default();
        let authority = (|| {
            let mut read = |key: &str| {
                if work.metadata_reads >= reads {
                    return Err(BasicOperatorObservationFailure::Budget);
                }
                work.metadata_reads += 1;
                let maybe_value = self
                    .block_index
                    .get(key)
                    .map_err(|_| BasicOperatorObservationFailure::Backend)?;
                if let Some(value) = &maybe_value {
                    let next_bytes = work
                        .metadata_bytes
                        .checked_add(value.len() as u64)
                        .ok_or(BasicOperatorObservationFailure::Budget)?;
                    if next_bytes > bytes {
                        return Err(BasicOperatorObservationFailure::Budget);
                    }
                    work.metadata_bytes = next_bytes;
                }
                Ok(maybe_value)
            };
            let maybe_state = read(codec::STATE_KEY)?
                .map(|value| codec::decode_state(value.as_ref()))
                .transpose()
                .map_err(|_| BasicOperatorObservationFailure::Integrity)?;
            let maybe_lifecycle = read(codec::ownership::OWNER_KEY)?
                .map(|value| codec::ownership::decode_owner(value.as_ref()))
                .transpose()
                .map_err(|_| BasicOperatorObservationFailure::Integrity)?;
            let maybe_identity = control
                .maybe_append_identity
                .filter(|id| id.revision == control.revision);
            if let Some(id) = maybe_identity {
                let Some(state) = maybe_state else {
                    return Err(BasicOperatorObservationFailure::Integrity);
                };
                let maybe_endpoint = match id.owner.checkpoint().checkpoint().prefix() {
                    open_bitcoin_core::chainstate::IndexPrefix::Empty => None,
                    open_bitcoin_core::chainstate::IndexPrefix::Committed(record) => {
                        Some((record.height(), record.block_hash()))
                    }
                };
                if maybe_lifecycle != Some(id.owner.lifecycle())
                    || state.maybe_endpoint != maybe_endpoint
                    || state.protection != id.owner.saved_protection()
                    || state.fence_height != id.owner.checkpoint().fence_height()
                    || state.fence_hash != id.owner.checkpoint().fence_hash()
                    || id.owner.checkpoint().checkpoint() != id.safe_checkpoint
                {
                    return Err(BasicOperatorObservationFailure::Integrity);
                }
            }
            Ok(BasicFilterOperatorAuthority {
                integrity: control.read_integrity,
                suspended: control.maybe_reorg_suspension.is_some(),
                maybe_lifecycle,
                maybe_publication: maybe_identity.map(|id| {
                    (
                        id.generation,
                        id.branch_identity,
                        id.processed,
                        id.safe_checkpoint,
                        id.owner
                            .maybe_effective_protection()
                            .unwrap_or(id.owner.saved_protection()),
                    )
                }),
                has_state: maybe_state.is_some(),
            })
        })();
        Ok(BasicFilterOperatorObservation {
            maybe_read_fault: control.maybe_read_fault,
            maybe_protection: control.maybe_read_fault_protection,
            retention,
            retained_availability,
            authority,
            work,
        })
    }

    /// Copy previously earned evidence without reading any stored row.
    /// Existing recovery tests use this narrow adapter to the production capture.
    #[cfg(test)]
    pub(crate) fn basic_filter_retention(
        &self,
    ) -> Result<BasicRetentionMeasurement, BasicIndexUnavailableReason> {
        self.basic_filter_operator_observation()
            .map_err(|_| BasicIndexUnavailableReason::RetentionFailed)?
            .retention
    }

    #[cfg(test)]
    pub(crate) fn basic_operator_namespaces_for_test(&self) -> Vec<Vec<(Vec<u8>, Vec<u8>)>> {
        use crate::storage::StorageNamespace::*;
        [
            Headers, BlockIndex, Chainstate, Coins, Wallet, Metrics, Mempool, Runtime, Schema,
        ]
        .into_iter()
        .map(|namespace| {
            self.keyspace(namespace)
                .iter()
                .map(|entry| {
                    let (key, value) = entry.into_inner().expect("namespace evidence");
                    (key.as_ref().to_vec(), value.as_ref().to_vec())
                })
                .collect()
        })
        .collect()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BasicOperatorObservationWork {
    pub metadata_reads: u64,
    pub metadata_bytes: u64,
}

pub(crate) struct BasicFilterOperatorObservation {
    pub maybe_read_fault: Option<super::publication::BasicFilterReadFault>,
    pub maybe_protection: Option<open_bitcoin_core::chainstate::IndexInputProtection>,
    pub retention: Result<BasicRetentionMeasurement, BasicIndexUnavailableReason>,
    pub retained_availability:
        Result<crate::status::BasicRetainedAvailability, BasicIndexUnavailableReason>,
    pub authority: Result<BasicFilterOperatorAuthority, BasicOperatorObservationFailure>,
    pub work: BasicOperatorObservationWork,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BasicOperatorObservationFailure {
    Budget,
    Backend,
    Integrity,
}

pub(crate) type BasicFilterOperatorPublication = (
    open_bitcoin_core::chainstate::filter_index::lifecycle::IndexGeneration,
    open_bitcoin_core::primitives::BlockHash,
    open_bitcoin_core::chainstate::FilterCheckpoint,
    open_bitcoin_core::chainstate::FilterCheckpoint,
    open_bitcoin_core::chainstate::IndexInputProtection,
);

pub(crate) struct BasicFilterOperatorAuthority {
    pub integrity: bool,
    pub suspended: bool,
    pub maybe_lifecycle:
        Option<open_bitcoin_core::chainstate::filter_index::lifecycle::IndexLifecycle>,
    pub maybe_publication: Option<BasicFilterOperatorPublication>,
    pub has_state: bool,
}

/// Shared finite admission ceiling for both immutable publication paths.
pub(super) fn check_envelope_bounds(
    records: &[codec::StoredFilterRecord],
) -> Result<(), StorageError> {
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
