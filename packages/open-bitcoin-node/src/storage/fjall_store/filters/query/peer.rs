// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Immediate bounded reads inside the existing immutable publication boundary.

use super::*;
use open_bitcoin_core::chainstate::filter_index::lifecycle::IndexLifecycle;
use open_bitcoin_network::basic_peer_resource::{
    BASIC_PEER_MAX_FRAME_BYTES, BasicPeerResourceRefusal, BasicPeerServeAxis as Axis,
    BasicPeerServeBudget, cfilter_frame_bytes,
};

/// Absence is returned as None; storage and local admission never become success.
#[derive(Debug)]
pub(crate) enum BasicPeerReadError {
    Disabled,
    Resource(BasicPeerResourceRefusal),
    WirePayload,
    Storage(StorageError),
}
impl From<StorageError> for BasicPeerReadError {
    fn from(error: StorageError) -> Self {
        Self::Storage(error)
    }
}
impl From<BasicPeerResourceRefusal> for BasicPeerReadError {
    fn from(error: BasicPeerResourceRefusal) -> Self {
        Self::Resource(error)
    }
}
impl std::fmt::Display for BasicPeerReadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Disabled => "BASIC peer backend disabled",
            Self::Resource(BasicPeerResourceRefusal(_axis)) => "BASIC peer resource refused",
            Self::WirePayload => "BASIC peer payload refused",
            Self::Storage(_error) => "BASIC peer storage failure",
        })
    }
}
impl std::error::Error for BasicPeerReadError {}

impl FjallNodeStore {
    /// One publication guard for the whole batch, without RPC readiness or repair.
    pub(crate) fn with_basic_peer_query<T>(
        &self,
        budget: &mut BasicPeerServeBudget,
        inspect: impl FnOnce(
            &BasicFilterQueryReader<'_>,
            &mut BasicPeerServeBudget,
        ) -> Result<T, BasicPeerReadError>,
    ) -> Result<T, BasicPeerReadError> {
        let mut control = self.filter_publication_guard()?;
        let read_fault = Cell::new(None);
        let result = (|| {
            budget.charge(Axis::MetadataReads, 1)?;
            self.count_filter_integrity_read();
            let maybe_lifecycle = observe_read_result(
                &read_fault,
                BasicFilterReadFault::Backend,
                self.block_index
                    .get(codec::ownership::OWNER_KEY)
                    .map_err(|error| {
                        super::super::super::backend_failure(StorageNamespace::BlockIndex, error)
                    }),
            )?
            .map(|bytes| {
                observe_read_result(
                    &read_fault,
                    BasicFilterReadFault::Integrity,
                    codec::ownership::decode_owner(&bytes),
                )
            })
            .transpose()?;
            let enabled = match maybe_lifecycle {
                Some(IndexLifecycle::Active { .. }) => true,
                Some(_) => false,
                None => {
                    budget.charge(Axis::MetadataReads, 1)?;
                    self.count_filter_integrity_read();
                    observe_read_result(
                        &read_fault,
                        BasicFilterReadFault::Backend,
                        self.block_index.get(codec::STATE_KEY).map_err(|error| {
                            super::super::super::backend_failure(
                                StorageNamespace::BlockIndex,
                                error,
                            )
                        }),
                    )?
                    .map(|bytes| {
                        observe_read_result(
                            &read_fault,
                            BasicFilterReadFault::Integrity,
                            codec::decode_state(&bytes),
                        )
                    })
                    .transpose()?
                    .is_some()
                }
            };
            if !enabled {
                return Err(BasicPeerReadError::Disabled);
            }
            inspect(
                &BasicFilterQueryReader {
                    store: self,
                    control: &control,
                    read_fault: &read_fault,
                    enabled,
                    maybe_generation: maybe_lifecycle.map(|owner| owner.generation()),
                },
                budget,
            )
        })();
        let observed = control.observe_read_fault(read_fault.get());
        result.and_then(|value| observed.map(|()| value).map_err(Into::into))
    }
}

impl BasicFilterQueryReader<'_> {
    /// Prepare every captured identity or expose no successful prefix.
    pub(crate) fn maybe_peer_records(
        &self,
        identities: &[open_bitcoin_network::policy::FilterBranchIdentity],
        budget: &mut BasicPeerServeBudget,
        filter_frame: bool,
    ) -> Result<Option<Vec<BasicFilterRecordView>>, BasicPeerReadError> {
        if identities.len() as u64 > budget.remaining(Axis::RecordReads) {
            return Err(BasicPeerResourceRefusal(Axis::RecordReads).into());
        }
        let container_bytes = (identities.len() as u64)
            .checked_mul(std::mem::size_of::<BasicFilterRecordView>() as u64)
            .ok_or(BasicPeerResourceRefusal(Axis::LogicalCopyBytes))?;
        budget.charge(Axis::LogicalCopyBytes, container_bytes)?;
        let mut records = Vec::with_capacity(identities.len());
        for identity in identities {
            let Some(record) = self.maybe_peer_record(identity.hash, budget, filter_frame)? else {
                return Ok(None);
            };
            let actual = record.identity();
            if actual.height() != identity.height || actual.parent_hash() != identity.parent {
                return Err(
                    index_corruption("BASIC peer record differs from captured branch").into(),
                );
            }
            records.push(record);
        }
        Ok(Some(records))
    }

    /// The caller owns family framing admission for headers/checkpoints; filters
    /// charge one full frame before parsing or copying their target payload.
    pub(crate) fn maybe_peer_record(
        &self,
        hash: BlockHash,
        budget: &mut BasicPeerServeBudget,
        filter_frame: bool,
    ) -> Result<Option<BasicFilterRecordView>, BasicPeerReadError> {
        if !self.control.read_integrity {
            return Err(
                index_corruption("BASIC read integrity requires exclusive recovery").into(),
            );
        }
        #[cfg(test)]
        if self.control.maybe_fault == Some(super::super::FilterPublicationFault::BeforeQueryRead) {
            self.read_fault.set(Some(BasicFilterReadFault::Backend));
            return Err(StorageError::BackendFailure {
                namespace: StorageNamespace::BlockIndex,
                message: "/private/phase160-backend-read-marker".into(),
                action: crate::StorageRecoveryAction::Restart,
            }
            .into());
        }
        let key = codec::record_key(hash);
        let Some(bytes) = self.peer_row(&key, budget)? else {
            return Ok(None);
        };
        check_envelope(bytes.len())?;
        let encoded_len = observe_read_result(
            self.read_fault,
            BasicFilterReadFault::Integrity,
            checked_encoded_length(bytes.len()),
        )?;
        if filter_frame {
            let frame = cfilter_frame_bytes(encoded_len as u64)
                .filter(|bytes| *bytes <= BASIC_PEER_MAX_FRAME_BYTES)
                .ok_or(BasicPeerReadError::WirePayload)?;
            budget.charge(Axis::EncodedSuffixBytes, frame)?;
        }
        let mut work = BasicFilterQueryWork {
            copied_filter_bytes: encoded_len,
            response_hex_bytes: checked_response_hex_bytes(encoded_len)?,
            ..Default::default()
        };
        charge_envelope(budget, &mut work, bytes.len(), true)?;
        let target = observe_read_result(
            self.read_fault,
            BasicFilterReadFault::Integrity,
            codec::parse_record(&key, &bytes),
        )?;
        let identity = if target.height == 0 {
            observe_read_result(
                self.read_fault,
                BasicFilterReadFault::Integrity,
                target.verify_parent(None),
            )?;
            observe_read_result(
                self.read_fault,
                BasicFilterReadFault::Integrity,
                target.checkpoint_identity(None),
            )?
        } else {
            let parent_key = codec::record_key(target.parent);
            let parent_bytes = self.peer_row(&parent_key, budget)?;
            let parent_bytes = observe_read_result(
                self.read_fault,
                BasicFilterReadFault::Integrity,
                parent_bytes.ok_or_else(|| index_corruption("missing BASIC query predecessor")),
            )?;
            check_envelope(parent_bytes.len())?;
            observe_read_result(
                self.read_fault,
                BasicFilterReadFault::Integrity,
                checked_encoded_length(parent_bytes.len()),
            )?;
            charge_envelope(budget, &mut work, parent_bytes.len(), false)?;
            let parent = observe_read_result(
                self.read_fault,
                BasicFilterReadFault::Integrity,
                codec::parse_record(&parent_key, &parent_bytes),
            )?;
            if parent.height == 0 {
                observe_read_result(
                    self.read_fault,
                    BasicFilterReadFault::Integrity,
                    parent.verify_parent(None),
                )?;
            }
            observe_read_result(
                self.read_fault,
                BasicFilterReadFault::Integrity,
                target.verify_parent(Some(&parent)),
            )?;
            observe_read_result(
                self.read_fault,
                BasicFilterReadFault::Integrity,
                target.checkpoint_identity(Some(&parent)),
            )?
        };
        Ok(Some(BasicFilterRecordView {
            identity,
            bytes: bytes[codec::RECORD_OVERHEAD..].to_vec(),
            work,
        }))
    }

    fn peer_row(
        &self,
        key: &str,
        budget: &mut BasicPeerServeBudget,
    ) -> Result<Option<fjall::Slice>, BasicPeerReadError> {
        budget.charge(Axis::RecordReads, 1)?;
        self.store.count_filter_integrity_read();
        observe_read_result(
            self.read_fault,
            BasicFilterReadFault::Backend,
            self.store.block_index.get(key).map_err(|error| {
                super::super::super::backend_failure(StorageNamespace::BlockIndex, error)
            }),
        )
        .map_err(Into::into)
    }
}

fn checked_encoded_length(length: usize) -> Result<usize, StorageError> {
    check_envelope(length)?;
    length
        .checked_sub(codec::RECORD_OVERHEAD)
        .ok_or_else(|| index_corruption("truncated BASIC query envelope"))
}

fn charge_envelope(
    budget: &mut BasicPeerServeBudget,
    work: &mut BasicFilterQueryWork,
    length: usize,
    target: bool,
) -> Result<(), BasicPeerReadError> {
    let encoded = checked_encoded_length(length)?;
    let mut next = *work;
    next.record_reads += 1;
    next.validated_envelope_bytes += length;
    charge_hash_work(&mut next, encoded, target)?;
    budget.charge(Axis::ValidatedEnvelopeBytes, length as u64)?;
    budget.charge(Axis::HeaderOperations, if target { 2 } else { 1 })?;
    budget.charge(
        Axis::HashPaddedBytes,
        (next.hash_padded_bytes - work.hash_padded_bytes) as u64,
    )?;
    // The target output copy is reserved together with its hash work, before parse.
    let copied = if target { next.copied_filter_bytes } else { 0 };
    budget.charge(
        Axis::LogicalCopyBytes,
        (next.hash_input_copy_bytes - work.hash_input_copy_bytes + copied) as u64,
    )?;
    *work = next;
    Ok(())
}

#[cfg(test)]
mod tests;
