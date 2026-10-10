// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp

//! Additive validation provenance, not Knots disk-format compatibility.
//! Decoded identities describe records; they never authorize scripts-valid writes.

use super::{FjallNodeStore, StorageError, StorageNamespace, StorageRecoveryAction};
use open_bitcoin_core::primitives::BlockHash;

pub(crate) const RECORD_PREFIX: &str = "validated_block:v1:";
pub(crate) const COVERAGE_KEY: &str = "validated_coverage:v1";
pub(crate) const RECORD_BYTES: usize = 70;
pub(crate) const MAX_VALIDATION_IDENTITIES: usize =
    open_bitcoin_core::chainstate::filter_index::catch_up::BASIC_INDEX_MAX_CANDIDATES as usize;
const KEY_BYTES: usize = RECORD_PREFIX.len() + 64;
const MAX_BATCH_BYTES: usize = MAX_VALIDATION_IDENTITIES * (RECORD_BYTES + KEY_BYTES);

/// Checked read identity. Possession alone grants no publication authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BlockValidationIdentity {
    hash: BlockHash,
    parent_hash: BlockHash,
    height: u32,
}

impl BlockValidationIdentity {
    pub(crate) fn new(
        hash: BlockHash,
        parent_hash: BlockHash,
        height: u32,
    ) -> Result<Self, StorageError> {
        if hash == BlockHash::from_byte_array([0; 32])
            || hash == parent_hash
            || (height == 0) != (parent_hash == BlockHash::from_byte_array([0; 32]))
        {
            return Err(history_corruption("invalid validation-history identity"));
        }
        Ok(Self {
            hash,
            parent_hash,
            height,
        })
    }

    pub(crate) const fn hash(self) -> BlockHash {
        self.hash
    }
    pub(crate) const fn parent_hash(self) -> BlockHash {
        self.parent_hash
    }
    pub(crate) const fn height(self) -> u32 {
        self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ValidationProvenance {
    NeverConnected,
    ScriptsValid,
    UnknownLegacy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StoredValidationStatus {
    KnownOnly,
    ScriptsValid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ValidationCoverage {
    Complete,
    UnknownLegacy,
}

impl ValidationCoverage {
    pub(crate) const fn absent_provenance(self) -> ValidationProvenance {
        match self {
            Self::Complete => ValidationProvenance::NeverConnected,
            Self::UnknownLegacy => ValidationProvenance::UnknownLegacy,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ValidationHistoryRecord {
    identity: BlockValidationIdentity,
    status: StoredValidationStatus,
}

impl ValidationHistoryRecord {
    pub(crate) const fn new(
        identity: BlockValidationIdentity,
        status: StoredValidationStatus,
    ) -> Self {
        Self { identity, status }
    }
    pub(crate) const fn identity(self) -> BlockValidationIdentity {
        self.identity
    }
    pub(crate) const fn status(self) -> StoredValidationStatus {
        self.status
    }
    pub(crate) const fn provenance(self) -> ValidationProvenance {
        match self.status {
            StoredValidationStatus::KnownOnly => ValidationProvenance::NeverConnected,
            StoredValidationStatus::ScriptsValid => ValidationProvenance::ScriptsValid,
        }
    }
}

/// Consume-only accepted publication capability. No raw constructor, deserializer,
/// clone or default exists; the genuine absorb adapter owns the mint seam.
pub(crate) struct AcceptedValidationBatch {
    store: FjallNodeStore,
    identities: Vec<BlockValidationIdentity>,
}

impl AcceptedValidationBatch {
    pub(crate) fn from_absorbed(
        receipt: crate::chainstate::validation_history::AcceptedValidationReceipt,
    ) -> Self {
        let (store, identities) = receipt.into_parts();
        Self { store, identities }
    }
    pub(crate) fn belongs_to(&self, store: &FjallNodeStore) -> bool {
        self.store.shares_validation_history_store(store)
    }
    pub(crate) fn identities(&self) -> &[BlockValidationIdentity] {
        &self.identities
    }
    pub(crate) fn validate(&self) -> Result<(), StorageError> {
        validate_batch_size(self.identities.len())?;
        for (index, identity) in self.identities.iter().enumerate() {
            if self.identities[..index]
                .iter()
                .any(|previous| previous.hash == identity.hash)
            {
                return Err(history_corruption("duplicate accepted validation identity"));
            }
        }
        Ok(())
    }
}

/// Separate sealed evidence for newly admitted trusted headers, never legacy snapshots.
pub(crate) struct AdmittedValidationHeaders {
    store: FjallNodeStore,
    identities: Vec<BlockValidationIdentity>,
}

impl AdmittedValidationHeaders {
    pub(crate) fn from_admitted(
        receipt: crate::network::validation_history::TrustedHeaderAdmission,
    ) -> Vec<Self> {
        let (store, identities) = receipt.into_parts();
        identities
            .chunks(MAX_VALIDATION_IDENTITIES)
            .map(|chunk| Self {
                store: store.clone(),
                identities: chunk.to_vec(),
            })
            .collect()
    }
    pub(crate) fn belongs_to(&self, store: &FjallNodeStore) -> bool {
        self.store.shares_validation_history_store(store)
    }
    pub(crate) fn identities(&self) -> &[BlockValidationIdentity] {
        &self.identities
    }
}

pub(crate) fn validate_batch_size(count: usize) -> Result<(), StorageError> {
    let bytes = count
        .checked_mul(RECORD_BYTES + KEY_BYTES)
        .ok_or_else(|| history_corruption("validation-history byte accounting overflow"))?;
    if count == 0 || count > MAX_VALIDATION_IDENTITIES || bytes > MAX_BATCH_BYTES {
        return Err(history_corruption("validation-history batch bound"));
    }
    Ok(())
}

pub(crate) fn record_key(hash: BlockHash) -> String {
    let mut key = String::with_capacity(KEY_BYTES);
    key.push_str(RECORD_PREFIX);
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in hash.as_bytes() {
        key.push(HEX[(byte >> 4) as usize] as char);
        key.push(HEX[(byte & 15) as usize] as char);
    }
    key
}

pub(crate) fn encode_record(record: ValidationHistoryRecord) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(RECORD_BYTES);
    bytes.extend_from_slice(&[
        1,
        match record.status {
            StoredValidationStatus::KnownOnly => 0,
            StoredValidationStatus::ScriptsValid => 1,
        },
    ]);
    bytes.extend_from_slice(&record.identity.height.to_le_bytes());
    bytes.extend_from_slice(record.identity.hash.as_bytes());
    bytes.extend_from_slice(record.identity.parent_hash.as_bytes());
    bytes
}

pub(crate) fn decode_record(
    key: &str,
    bytes: &[u8],
) -> Result<ValidationHistoryRecord, StorageError> {
    if bytes.len() != RECORD_BYTES || bytes[0] != 1 {
        return Err(history_corruption(
            "unsupported validation-history envelope",
        ));
    }
    let status = match bytes[1] {
        0 => StoredValidationStatus::KnownOnly,
        1 => StoredValidationStatus::ScriptsValid,
        _ => return Err(history_corruption("unsupported validation-history status")),
    };
    let height = u32::from_le_bytes(bytes[2..6].try_into().map_err(history_corruption)?);
    let hash = BlockHash::from_byte_array(bytes[6..38].try_into().map_err(history_corruption)?);
    let parent = BlockHash::from_byte_array(bytes[38..70].try_into().map_err(history_corruption)?);
    let identity = BlockValidationIdentity::new(hash, parent, height)?;
    if record_key(hash) != key {
        return Err(history_corruption("validation-history key/hash mismatch"));
    }
    Ok(ValidationHistoryRecord::new(identity, status))
}

pub(crate) fn validate_upgrade(
    maybe_existing: Option<ValidationHistoryRecord>,
    next: ValidationHistoryRecord,
) -> Result<(), StorageError> {
    let Some(existing) = maybe_existing else {
        return Ok(());
    };
    if existing.identity != next.identity {
        return Err(history_corruption(
            "conflicting validation-history identity",
        ));
    }
    if existing.status == StoredValidationStatus::ScriptsValid
        && next.status == StoredValidationStatus::KnownOnly
    {
        return Err(history_corruption("validation-history downgrade refused"));
    }
    Ok(())
}

pub(crate) fn encode_coverage(coverage: ValidationCoverage) -> [u8; 2] {
    [
        1,
        match coverage {
            ValidationCoverage::Complete => 1,
            ValidationCoverage::UnknownLegacy => 0,
        },
    ]
}

pub(crate) fn decode_coverage(bytes: &[u8]) -> Result<ValidationCoverage, StorageError> {
    match bytes {
        [1, 0] => Ok(ValidationCoverage::UnknownLegacy),
        [1, 1] => Ok(ValidationCoverage::Complete),
        _ => Err(history_corruption(
            "unsupported validation-history coverage",
        )),
    }
}

pub(crate) fn history_corruption(detail: impl std::fmt::Display) -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::BlockIndex,
        detail: detail.to_string(),
        action: StorageRecoveryAction::Repair,
    }
}

#[cfg(test)]
#[path = "fjall_store/validation_history/tests.rs"]
mod store_tests;
#[cfg(test)]
mod tests;
