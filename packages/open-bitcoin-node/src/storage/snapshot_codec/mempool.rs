// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use std::collections::BTreeSet;
use std::mem::size_of;

use open_bitcoin_core::{
    codec::{TransactionEncoding, encode_transaction, parse_transaction},
    consensus::{transaction_txid as canonical_txid, transaction_wtxid},
    primitives::{OutPoint, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    MempoolAcceptanceTime, MempoolEntryMetadata, MempoolMemberIdentity, MempoolOrigin, PolicyTime,
    RelayIntent,
};
use serde::{Deserialize, Serialize, Serializer};

use super::{corruption, encode_versioned};
use crate::storage::mempool_snapshot::{
    CapturedMempoolGeneration, MAX_MEMPOOL_SNAPSHOT_UNBROADCAST_MEMBERS, MempoolSnapshotError,
    MempoolSnapshotFormatVersion,
};
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};
use crate::{StorageError, StorageNamespace};

mod decode;

const MAX_MEMPOOL_SNAPSHOT_ENCODED_BYTES: usize = 256 * 1024 * 1024;
const MAX_MEMPOOL_SNAPSHOT_TRANSACTION_BYTES: usize = 4 * 1024 * 1024;
const MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES: usize = 64 * 1024 * 1024;
const ENCODED_ENVELOPE_OVERHEAD_BYTES: usize = 1_048_576;
const ENCODED_RECORD_OVERHEAD_BYTES: usize = 512;
const ENCODED_UNBROADCAST_MEMBER_OVERHEAD_BYTES: usize = 4_096;
const HEX_CHARS_PER_TRANSACTION_BYTE: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MempoolSnapshotPersistedInputLimits {
    pub(crate) max_encoded_bytes: usize,
    pub(crate) max_records: usize,
    pub(crate) max_unbroadcast_members: usize,
    pub(crate) max_transaction_bytes: usize,
    pub(crate) max_total_transaction_bytes: usize,
    pub(crate) max_input_edges: usize,
    pub(crate) max_input_edges_per_record: usize,
}

pub(crate) fn persisted_mempool_input_limits() -> Option<MempoolSnapshotPersistedInputLimits> {
    let max_unbroadcast_members = MAX_MEMPOOL_SNAPSHOT_UNBROADCAST_MEMBERS;
    let encoded_transaction_bytes =
        MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES.checked_mul(HEX_CHARS_PER_TRANSACTION_BYTE)?;
    let encoded_unbroadcast_bytes =
        max_unbroadcast_members.checked_mul(ENCODED_UNBROADCAST_MEMBER_OVERHEAD_BYTES)?;
    let encoded_record_budget = MAX_MEMPOOL_SNAPSHOT_ENCODED_BYTES
        .checked_sub(encoded_transaction_bytes)?
        .checked_sub(encoded_unbroadcast_bytes)?
        .checked_sub(ENCODED_ENVELOPE_OVERHEAD_BYTES)?;
    let max_records = encoded_record_budget.checked_div(ENCODED_RECORD_OVERHEAD_BYTES)?;
    let minimum_input_bytes = OutPoint::SERIALIZED_LEN
        .checked_add(1)?
        .checked_add(size_of::<u32>())?;
    let max_input_edges =
        MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES.checked_div(minimum_input_bytes)?;
    let max_input_edges_per_record =
        MAX_MEMPOOL_SNAPSHOT_TRANSACTION_BYTES.checked_div(minimum_input_bytes)?;
    let limits = MempoolSnapshotPersistedInputLimits {
        max_encoded_bytes: MAX_MEMPOOL_SNAPSHOT_ENCODED_BYTES,
        max_records,
        max_unbroadcast_members,
        max_transaction_bytes: MAX_MEMPOOL_SNAPSHOT_TRANSACTION_BYTES,
        max_total_transaction_bytes: MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES,
        max_input_edges,
        max_input_edges_per_record,
    };
    if encoded_size_upper_bound(
        limits.max_total_transaction_bytes,
        limits.max_records,
        limits.max_unbroadcast_members,
    ) != Some(limits.max_encoded_bytes)
    {
        return None;
    }
    Some(limits)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MempoolSnapshotDecodeLimits {
    pub(crate) max_encoded_bytes: usize,
    pub(crate) max_records: usize,
    pub(crate) max_unbroadcast_members: usize,
    pub(crate) max_transaction_bytes: usize,
    pub(crate) max_total_transaction_bytes: usize,
}

impl MempoolSnapshotDecodeLimits {
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
}

#[cfg(test)]
impl Default for MempoolSnapshotDecodeLimits {
    fn default() -> Self {
        let limits = persisted_mempool_input_limits()
            .expect("fixed mempool persisted-input arithmetic must remain representable");
        Self {
            max_encoded_bytes: limits.max_encoded_bytes,
            max_records: limits.max_records,
            max_unbroadcast_members: limits.max_unbroadcast_members,
            max_transaction_bytes: limits.max_transaction_bytes,
            max_total_transaction_bytes: limits.max_total_transaction_bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MempoolSnapshotV2Dto {
    format_version: u32,
    captured_generation: u64,
    captured_at_unix_seconds: i64,
    records: Vec<MempoolSnapshotV2RecordDto>,
    unbroadcast_members: Vec<MempoolMemberIdentityDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MempoolSnapshotV2RecordDto {
    #[serde(serialize_with = "serialize_hex_transaction")]
    transaction: Vec<u8>,
    accepted_at_unix_seconds: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MempoolMemberIdentityDto {
    txid: [u8; 32],
    wtxid: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MempoolSnapshotV1Dto {
    records: Vec<MempoolSnapshotV1RecordDto>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum MempoolOriginV1Dto {
    Local,
    Peer,
    Reorg,
    RecoveryUnknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MempoolSnapshotV1RecordDto {
    txid: [u8; 32],
    wtxid: [u8; 32],
    transaction: Vec<u8>,
    fee_sats: i64,
    virtual_size: usize,
    maybe_accepted_at_unix_seconds: Option<i64>,
    maybe_origin: Option<MempoolOriginV1Dto>,
    maybe_relay_requested: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum MempoolSnapshotPayloadDto {
    CurrentV2(MempoolSnapshotV2Dto),
    LegacyV1(MempoolSnapshotV1Dto),
}

pub(crate) fn encode_mempool_snapshot(snapshot: &MempoolSnapshot) -> Result<Vec<u8>, StorageError> {
    let dto = MempoolSnapshotV2Dto::try_from(snapshot)?;
    let total_transaction_bytes = dto.records.iter().try_fold(0_usize, |total, record| {
        total
            .checked_add(record.transaction.len())
            .ok_or_else(|| snapshot_failure(MempoolSnapshotError::ResourceBoundExceeded))
    })?;
    let max_encoded_bytes = encoded_size_upper_bound(
        total_transaction_bytes,
        dto.records.len(),
        dto.unbroadcast_members.len(),
    )
    .ok_or_else(|| snapshot_failure(MempoolSnapshotError::ResourceBoundExceeded))?;
    let bytes = encode_versioned(StorageNamespace::Mempool, &dto)?;
    if bytes.len() > max_encoded_bytes {
        return Err(snapshot_failure(
            MempoolSnapshotError::ResourceBoundExceeded,
        ));
    }
    Ok(bytes)
}

pub(crate) fn encoded_size_upper_bound(
    max_total_transaction_bytes: usize,
    max_records: usize,
    max_unbroadcast_members: usize,
) -> Option<usize> {
    max_total_transaction_bytes
        .checked_mul(HEX_CHARS_PER_TRANSACTION_BYTE)?
        .checked_add(max_records.checked_mul(ENCODED_RECORD_OVERHEAD_BYTES)?)?
        .checked_add(
            max_unbroadcast_members.checked_mul(ENCODED_UNBROADCAST_MEMBER_OVERHEAD_BYTES)?,
        )?
        .checked_add(ENCODED_ENVELOPE_OVERHEAD_BYTES)
}

#[cfg(test)]
pub(crate) fn decode_mempool_snapshot(bytes: &[u8]) -> Result<MempoolSnapshot, StorageError> {
    decode_mempool_snapshot_with_limits(bytes, MempoolSnapshotDecodeLimits::default())
}

pub fn decode_mempool_snapshot_with_limits(
    bytes: &[u8],
    limits: MempoolSnapshotDecodeLimits,
) -> Result<MempoolSnapshot, StorageError> {
    if bytes.len() > limits.max_encoded_bytes {
        return Err(snapshot_failure(
            MempoolSnapshotError::ResourceBoundExceeded,
        ));
    }

    let payload = decode::decode_bounded_versioned(bytes, limits)?;

    match payload {
        MempoolSnapshotPayloadDto::CurrentV2(dto) => dto.try_into(),
        MempoolSnapshotPayloadDto::LegacyV1(dto) => dto.try_into(),
    }
}

fn serialize_hex_transaction<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len().saturating_mul(2));
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    serializer.serialize_str(&encoded)
}

impl TryFrom<&MempoolSnapshot> for MempoolSnapshotV2Dto {
    type Error = StorageError;

    fn try_from(snapshot: &MempoolSnapshot) -> Result<Self, Self::Error> {
        let format_version = snapshot
            .format_version()
            .ok_or_else(|| snapshot_failure(MempoolSnapshotError::StructuralCorruption))?;
        let captured_generation = snapshot
            .captured_generation()
            .ok_or_else(|| snapshot_failure(MempoolSnapshotError::StructuralCorruption))?;
        let captured_at = snapshot
            .captured_at()
            .ok_or_else(|| snapshot_failure(MempoolSnapshotError::StructuralCorruption))?;
        let canonical_members = snapshot
            .records
            .iter()
            .map(|record| canonical_member_identity(&record.transaction))
            .collect::<Result<BTreeSet<_>, _>>()?;
        if canonical_members.len() != snapshot.records.len()
            || !snapshot.unbroadcast_members().is_subset(&canonical_members)
        {
            return Err(snapshot_failure(MempoolSnapshotError::IdentityMismatch));
        }
        if snapshot.records.iter().any(|record| {
            matches!(
                record.acceptance_time,
                MempoolAcceptanceTime::Known(accepted_at) if accepted_at > captured_at
            )
        }) {
            return Err(snapshot_failure(MempoolSnapshotError::StructuralCorruption));
        }
        let records = snapshot
            .records
            .iter()
            .map(MempoolSnapshotV2RecordDto::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let unbroadcast_members = snapshot
            .unbroadcast_members()
            .iter()
            .copied()
            .map(MempoolMemberIdentityDto::from)
            .collect();

        Ok(Self {
            format_version: format_version.get(),
            captured_generation: captured_generation.raw(),
            captured_at_unix_seconds: captured_at.unix_seconds(),
            records,
            unbroadcast_members,
        })
    }
}

impl TryFrom<&MempoolSnapshotRecord> for MempoolSnapshotV2RecordDto {
    type Error = StorageError;

    fn try_from(record: &MempoolSnapshotRecord) -> Result<Self, Self::Error> {
        let accepted_at_unix_seconds = match record.acceptance_time {
            MempoolAcceptanceTime::Known(accepted_at) => Some(accepted_at.unix_seconds()),
            MempoolAcceptanceTime::LegacyUnknown => None,
        };
        Ok(Self {
            transaction: encode_canonical_transaction(&record.transaction)?,
            accepted_at_unix_seconds,
        })
    }
}

impl TryFrom<MempoolSnapshotV2Dto> for MempoolSnapshot {
    type Error = StorageError;

    fn try_from(dto: MempoolSnapshotV2Dto) -> Result<Self, Self::Error> {
        MempoolSnapshotFormatVersion::try_from(dto.format_version).map_err(snapshot_failure)?;
        let records = dto
            .records
            .into_iter()
            .map(|record| {
                let transaction = decode_canonical_transaction(&record.transaction)?;
                MempoolSnapshotRecord::try_from_canonical(
                    transaction,
                    match record.accepted_at_unix_seconds {
                        Some(accepted_at) => {
                            MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(accepted_at))
                        }
                        None => MempoolAcceptanceTime::LegacyUnknown,
                    },
                )
                .map_err(snapshot_failure)
            })
            .collect::<Result<Vec<_>, StorageError>>()?;
        let unbroadcast_count = dto.unbroadcast_members.len();
        let unbroadcast_members = dto
            .unbroadcast_members
            .into_iter()
            .map(MempoolMemberIdentity::from)
            .collect::<BTreeSet<_>>();
        if unbroadcast_count != unbroadcast_members.len() {
            return Err(snapshot_failure(MempoolSnapshotError::IdentityMismatch));
        }

        MempoolSnapshot::try_new_current(
            CapturedMempoolGeneration::try_new(dto.captured_generation)
                .map_err(snapshot_failure)?,
            PolicyTime::from_unix_seconds(dto.captured_at_unix_seconds),
            records,
            unbroadcast_members,
        )
        .map_err(snapshot_failure)
    }
}

impl TryFrom<MempoolSnapshotV1Dto> for MempoolSnapshot {
    type Error = StorageError;

    fn try_from(dto: MempoolSnapshotV1Dto) -> Result<Self, Self::Error> {
        let records = dto
            .records
            .into_iter()
            .map(decode_v1_record)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::from_legacy_v1(records))
    }
}

fn decode_v1_record(
    dto: MempoolSnapshotV1RecordDto,
) -> Result<MempoolSnapshotRecord, StorageError> {
    let transaction = decode_canonical_transaction(&dto.transaction)?;
    let txid = Txid::from_byte_array(dto.txid);
    let wtxid = Wtxid::from_byte_array(dto.wtxid);
    let actual_txid = canonical_txid(&transaction)
        .map_err(|_| snapshot_failure(MempoolSnapshotError::StructuralCorruption))?;
    let actual_wtxid = transaction_wtxid(&transaction)
        .map_err(|_| snapshot_failure(MempoolSnapshotError::StructuralCorruption))?;
    if actual_txid != txid || actual_wtxid != wtxid {
        return Err(snapshot_failure(MempoolSnapshotError::IdentityMismatch));
    }
    let acceptance_time = decode_v1_acceptance_time(
        dto.maybe_accepted_at_unix_seconds,
        dto.maybe_origin,
        dto.maybe_relay_requested,
    )?;
    let metadata = MempoolEntryMetadata::new(
        acceptance_time,
        MempoolOrigin::RecoveryUnknown,
        RelayIntent::NotRequested,
    );

    MempoolSnapshotRecord::try_from_compatibility(
        transaction,
        txid,
        wtxid,
        dto.fee_sats,
        dto.virtual_size,
        metadata,
    )
    .map_err(snapshot_failure)
}

fn encode_canonical_transaction(transaction: &Transaction) -> Result<Vec<u8>, StorageError> {
    encode_transaction(transaction, TransactionEncoding::WithWitness)
        .map_err(|_| snapshot_failure(MempoolSnapshotError::StructuralCorruption))
}

fn decode_canonical_transaction(bytes: &[u8]) -> Result<Transaction, StorageError> {
    parse_transaction(bytes).map_err(|_| snapshot_failure(MempoolSnapshotError::DecodeFailure))
}

fn canonical_member_identity(
    transaction: &Transaction,
) -> Result<MempoolMemberIdentity, StorageError> {
    let txid = canonical_txid(transaction)
        .map_err(|_| snapshot_failure(MempoolSnapshotError::StructuralCorruption))?;
    let wtxid = transaction_wtxid(transaction)
        .map_err(|_| snapshot_failure(MempoolSnapshotError::StructuralCorruption))?;
    Ok(MempoolMemberIdentity { txid, wtxid })
}

fn decode_v1_acceptance_time(
    maybe_accepted_at_unix_seconds: Option<i64>,
    maybe_origin: Option<MempoolOriginV1Dto>,
    maybe_relay_requested: Option<bool>,
) -> Result<MempoolAcceptanceTime, StorageError> {
    match (
        maybe_accepted_at_unix_seconds,
        maybe_origin,
        maybe_relay_requested,
    ) {
        (Some(accepted_at), Some(_), Some(_)) => Ok(MempoolAcceptanceTime::Known(
            PolicyTime::from_unix_seconds(accepted_at),
        )),
        (None, None, None) => Ok(MempoolAcceptanceTime::LegacyUnknown),
        // Phase 130 compatibility anchor: partial mempool entry metadata is corrupt.
        _ => Err(snapshot_failure(MempoolSnapshotError::StructuralCorruption)),
    }
}

fn snapshot_failure(error: MempoolSnapshotError) -> StorageError {
    corruption(StorageNamespace::Mempool, error)
}

impl From<MempoolMemberIdentity> for MempoolMemberIdentityDto {
    fn from(identity: MempoolMemberIdentity) -> Self {
        Self {
            txid: identity.txid.to_byte_array(),
            wtxid: identity.wtxid.to_byte_array(),
        }
    }
}

impl From<MempoolMemberIdentityDto> for MempoolMemberIdentity {
    fn from(dto: MempoolMemberIdentityDto) -> Self {
        Self {
            txid: Txid::from_byte_array(dto.txid),
            wtxid: Wtxid::from_byte_array(dto.wtxid),
        }
    }
}
