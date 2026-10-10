// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

//! Point reads require all retained immutable records to have been recovered.

use super::{FjallNodeStore, StorageError, StorageNamespace, codec, index_corruption};
use open_bitcoin_core::{
    chainstate::FilterRecordIdentity,
    codec::MAX_SIZE,
    primitives::{BlockHash, FilterHeader},
};

/// Codec maximum, including the persisted identity envelope; never a smaller
/// response policy that would reject an admitted immutable singleton.
pub const BASIC_FILTER_QUERY_MAX_RECORD_BYTES: usize = MAX_SIZE as usize + codec::RECORD_OVERHEAD;
/// A non-genesis lookup validates exactly two individually bounded envelopes.
pub const BASIC_FILTER_QUERY_MAX_READ_BYTES: usize = 2 * BASIC_FILTER_QUERY_MAX_RECORD_BYTES;
/// Hex filter bytes and the 32-byte header rendered as hex.
pub const BASIC_FILTER_QUERY_MAX_HEX_BYTES: usize = 2 * MAX_SIZE as usize + 64;
// SHA256 currently owns a padded input Vec. Count both row hashes, their header
// hashes, target identity's repeated header hash, and the response copy. These
// are logical byte lengths; allocator capacities/reallocations are not RSS bounds.
const MAX_LOGICAL_COPY_BYTES: usize = 3 * MAX_SIZE as usize + 352;
const MAX_HASH_PADDED_BYTES: usize = 2 * MAX_SIZE as usize + 832;

/// Owned bytes and commitment from one guarded immutable identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicFilterRecordView {
    identity: FilterRecordIdentity,
    bytes: Vec<u8>,
    work: BasicFilterQueryWork,
}

impl BasicFilterRecordView {
    /// Return the checked target identity, including its header.
    pub const fn identity(&self) -> FilterRecordIdentity {
        self.identity
    }
    /// Borrow the exact encoded BASIC filter; no generation takes place.
    pub fn encoded_bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Return the raw commitment header bound to these bytes.
    pub fn filter_header(&self) -> FilterHeader {
        self.identity.filter_header()
    }
    /// Return structural accounting observed during the bounded read.
    pub const fn work(&self) -> BasicFilterQueryWork {
        self.work
    }
}

/// Logical work counts, not allocator/RSS or latency guarantees.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BasicFilterQueryWork {
    pub record_reads: usize,
    pub validated_envelope_bytes: usize,
    pub copied_filter_bytes: usize,
    pub hash_input_copy_bytes: usize,
    pub hash_padded_bytes: usize,
    pub logical_copy_bytes: usize,
    pub response_hex_bytes: usize,
}

/// A borrowed read boundary cannot outlive its shared publication guard.
pub(crate) struct BasicFilterQueryReader<'a> {
    store: &'a FjallNodeStore,
    control: &'a super::PublicationControl,
    pub enabled: bool,
    maybe_generation:
        Option<open_bitcoin_core::chainstate::filter_index::lifecycle::IndexGeneration>,
}

#[cfg(test)]
pub(crate) enum QueryRecordFault {
    CorruptTarget,
    CorruptParent,
    MissingTarget,
    MissingParent,
}

impl BasicFilterQueryReader<'_> {
    pub(crate) fn matches_generation(
        &self,
        generation: open_bitcoin_core::chainstate::filter_index::lifecycle::IndexGeneration,
    ) -> bool {
        self.control.read_integrity
            && self
                .maybe_generation
                .is_none_or(|actual| actual == generation)
    }
    pub(crate) fn maybe_record(
        &self,
        hash: BlockHash,
    ) -> Result<Option<BasicFilterRecordView>, StorageError> {
        self.store
            .basic_filter_point_query_guarded(hash, self.control)
    }
}

impl FjallNodeStore {
    /// Simulate external backend tampering underneath an already earned epoch.
    /// This deliberately neither invalidates nor readmits any authority.
    #[cfg(test)]
    pub(crate) fn inject_query_record_fault_for_test(
        &self,
        hash: BlockHash,
        fault: QueryRecordFault,
    ) -> Result<(), StorageError> {
        let _control = self.filter_publication_guard()?;
        let target_key = codec::record_key(hash);
        let target = self
            .block_index
            .get(&target_key)
            .map_err(|error| super::super::backend_failure(StorageNamespace::BlockIndex, error))?
            .ok_or_else(|| index_corruption("fault requires an existing target"))?;
        let parent = codec::parse_record(&target_key, &target)?.parent;
        let key = match fault {
            QueryRecordFault::CorruptTarget | QueryRecordFault::MissingTarget => target_key,
            QueryRecordFault::CorruptParent | QueryRecordFault::MissingParent => {
                codec::record_key(parent)
            }
        };
        match fault {
            QueryRecordFault::MissingTarget | QueryRecordFault::MissingParent => {
                self.block_index.remove(key).map_err(|error| {
                    super::super::backend_failure(StorageNamespace::BlockIndex, error)
                })?;
            }
            QueryRecordFault::CorruptTarget | QueryRecordFault::CorruptParent => {
                let mut bytes = self
                    .block_index
                    .get(&key)
                    .map_err(|error| {
                        super::super::backend_failure(StorageNamespace::BlockIndex, error)
                    })?
                    .ok_or_else(|| index_corruption("fault requires an existing record"))?
                    .to_vec();
                bytes[102] ^= 1;
                self.block_index.insert(key, bytes).map_err(|error| {
                    super::super::backend_failure(StorageNamespace::BlockIndex, error)
                })?;
            }
        }
        self.persist(StorageNamespace::BlockIndex, crate::PersistMode::Sync)
    }

    #[cfg(test)]
    pub(crate) fn basic_filter_publication_lock_available_for_test(&self) -> bool {
        self.filter_publication.try_lock().is_ok()
    }

    /// Lifecycle and target inspection share the existing publication lock.
    /// The caller resolves known identity before requesting any immutable row.
    pub(crate) fn with_basic_filter_query<T>(
        &self,
        inspect: impl FnOnce(&BasicFilterQueryReader<'_>) -> Result<T, StorageError>,
    ) -> Result<T, StorageError> {
        let control = self.filter_publication_guard()?;
        self.count_filter_integrity_read();
        let maybe_lifecycle = self
            .block_index
            .get(codec::ownership::OWNER_KEY)
            .map_err(|error| super::super::backend_failure(StorageNamespace::BlockIndex, error))?
            .map(|bytes| codec::ownership::decode_owner(&bytes))
            .transpose()?;
        let enabled = match maybe_lifecycle {
            Some(
                open_bitcoin_core::chainstate::filter_index::lifecycle::IndexLifecycle::Active {
                    ..
                },
            ) => true,
            Some(_) => false,
            None => {
                self.count_filter_integrity_read();
                self.block_index
                    .get(codec::STATE_KEY)
                    .map_err(|error| {
                        super::super::backend_failure(StorageNamespace::BlockIndex, error)
                    })?
                    .map(|bytes| codec::decode_state(&bytes))
                    .transpose()?
                    .is_some()
            }
        };
        inspect(&BasicFilterQueryReader {
            store: self,
            control: &control,
            enabled,
            maybe_generation: maybe_lifecycle.map(|lifecycle| lifecycle.generation()),
        })
    }

    #[cfg(test)]
    pub(crate) fn maybe_basic_filter_point_query(
        &self,
        hash: BlockHash,
    ) -> Result<Option<BasicFilterRecordView>, StorageError> {
        let control = self.filter_publication_guard()?;
        self.basic_filter_point_query_guarded(hash, &control)
    }

    fn basic_filter_point_query_guarded(
        &self,
        hash: BlockHash,
        control: &super::PublicationControl,
    ) -> Result<Option<BasicFilterRecordView>, StorageError> {
        if !control.read_integrity {
            return Err(index_corruption(
                "BASIC read integrity requires exclusive recovery",
            ));
        }
        #[cfg(test)]
        if control.maybe_fault == Some(super::FilterPublicationFault::BeforeQueryRead) {
            return Err(StorageError::BackendFailure {
                namespace: StorageNamespace::BlockIndex,
                message: "/private/phase159-backend-read-marker".into(),
                action: crate::StorageRecoveryAction::Restart,
            });
        }
        let key = codec::record_key(hash);
        self.count_filter_integrity_read();
        let Some(bytes) = self
            .block_index
            .get(&key)
            .map_err(|error| super::super::backend_failure(StorageNamespace::BlockIndex, error))?
        else {
            return Ok(None);
        };
        check_envelope(bytes.len())?;
        let encoded_len = bytes
            .len()
            .checked_sub(codec::RECORD_OVERHEAD)
            .ok_or_else(|| index_corruption("truncated BASIC query envelope"))?;
        let mut work = BasicFilterQueryWork {
            record_reads: 1,
            validated_envelope_bytes: bytes.len(),
            copied_filter_bytes: encoded_len,
            response_hex_bytes: checked_response_hex_bytes(encoded_len)?,
            ..Default::default()
        };
        charge_hash_work(&mut work, encoded_len, true)?;
        let target = codec::parse_record(&key, &bytes)?;
        let identity = if target.height == 0 {
            target.verify_parent(None)?;
            target.checkpoint_identity(None)?
        } else {
            let parent_key = codec::record_key(target.parent);
            self.count_filter_integrity_read();
            let parent_bytes = self
                .block_index
                .get(&parent_key)
                .map_err(|error| {
                    super::super::backend_failure(StorageNamespace::BlockIndex, error)
                })?
                .ok_or_else(|| index_corruption("missing BASIC query predecessor"))?;
            check_envelope(parent_bytes.len())?;
            let parent_len = parent_bytes
                .len()
                .checked_sub(codec::RECORD_OVERHEAD)
                .ok_or_else(|| index_corruption("truncated BASIC query parent envelope"))?;
            work.record_reads += 1;
            work.validated_envelope_bytes = work
                .validated_envelope_bytes
                .checked_add(parent_bytes.len())
                .filter(|size| *size <= BASIC_FILTER_QUERY_MAX_READ_BYTES)
                .ok_or_else(|| index_corruption("BASIC query read byte bound"))?;
            charge_hash_work(&mut work, parent_len, false)?;
            let parent = codec::parse_record(&parent_key, &parent_bytes)?;
            if parent.height == 0 {
                parent.verify_parent(None)?;
            }
            target.verify_parent(Some(&parent))?;
            target.checkpoint_identity(Some(&parent))?
        };
        let encoded = &bytes[codec::RECORD_OVERHEAD..];
        Ok(Some(BasicFilterRecordView {
            identity,
            bytes: encoded.to_vec(),
            work,
        }))
    }
}

pub(super) fn charge_hash_work(
    work: &mut BasicFilterQueryWork,
    encoded: usize,
    target: bool,
) -> Result<(), StorageError> {
    // SHA256d(encoded), SHA256d(64-byte header composition), and, for the
    // target, checkpoint_identity's repeated header composition. SHA256d's
    // second digest copies 32 bytes; its padded length is 64 bytes.
    let input = encoded
        .checked_add(if target { 224 } else { 128 })
        .ok_or_else(|| index_corruption("BASIC query hash copy overflow"))?;
    let padded = encoded
        .checked_add(9)
        .and_then(|size| size.checked_add(63))
        .map(|size| size / 64 * 64)
        .and_then(|size| size.checked_add(if target { 448 } else { 256 }))
        .ok_or_else(|| index_corruption("BASIC query hash padding overflow"))?;
    work.hash_input_copy_bytes = work
        .hash_input_copy_bytes
        .checked_add(input)
        .ok_or_else(|| index_corruption("BASIC query hash copy overflow"))?;
    work.hash_padded_bytes = work
        .hash_padded_bytes
        .checked_add(padded)
        .filter(|size| *size <= MAX_HASH_PADDED_BYTES)
        .ok_or_else(|| index_corruption("BASIC query hash padding bound"))?;
    work.logical_copy_bytes = work
        .hash_input_copy_bytes
        .checked_add(work.copied_filter_bytes)
        .filter(|size| *size <= MAX_LOGICAL_COPY_BYTES)
        .ok_or_else(|| index_corruption("BASIC query logical copy bound"))?;
    Ok(())
}

pub(super) fn check_envelope(bytes: usize) -> Result<(), StorageError> {
    if bytes > BASIC_FILTER_QUERY_MAX_RECORD_BYTES {
        return Err(index_corruption("BASIC query envelope byte bound"));
    }
    Ok(())
}

pub(super) fn checked_response_hex_bytes(bytes: usize) -> Result<usize, StorageError> {
    bytes
        .checked_mul(2)
        .and_then(|size| size.checked_add(64))
        .filter(|size| *size <= BASIC_FILTER_QUERY_MAX_HEX_BYTES)
        .ok_or_else(|| index_corruption("BASIC query response expansion bound"))
}
