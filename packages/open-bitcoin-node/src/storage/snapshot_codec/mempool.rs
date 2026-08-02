// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use std::collections::BTreeSet;

use open_bitcoin_core::{
    codec::{TransactionEncoding, encode_transaction, parse_transaction},
    consensus::{transaction_txid as canonical_txid, transaction_wtxid},
    primitives::{Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    MempoolAcceptanceTime, MempoolEntryMetadata, MempoolMemberIdentity, MempoolOrigin, PolicyTime,
    RelayIntent, transaction_weight_and_virtual_size,
};
use serde::{Deserialize, Serialize};

use super::{corruption, decode_versioned, encode_versioned};
use crate::storage::mempool_snapshot::{
    CapturedMempoolGeneration, MAX_MEMPOOL_SNAPSHOT_RECORDS,
    MAX_MEMPOOL_SNAPSHOT_UNBROADCAST_MEMBERS, MempoolSnapshotError, MempoolSnapshotFormatVersion,
};
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};
use crate::{StorageError, StorageNamespace};

const MAX_MEMPOOL_SNAPSHOT_ENCODED_BYTES: usize = 64 * 1024 * 1024;
const MAX_MEMPOOL_SNAPSHOT_TRANSACTION_BYTES: usize = 4 * 1024 * 1024;
const MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES: usize = 64 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MempoolSnapshotDecodeLimits {
    pub(crate) max_encoded_bytes: usize,
    pub(crate) max_records: usize,
    pub(crate) max_unbroadcast_members: usize,
    pub(crate) max_transaction_bytes: usize,
    pub(crate) max_total_transaction_bytes: usize,
}

impl Default for MempoolSnapshotDecodeLimits {
    fn default() -> Self {
        Self {
            max_encoded_bytes: MAX_MEMPOOL_SNAPSHOT_ENCODED_BYTES,
            max_records: MAX_MEMPOOL_SNAPSHOT_RECORDS,
            max_unbroadcast_members: MAX_MEMPOOL_SNAPSHOT_UNBROADCAST_MEMBERS,
            max_transaction_bytes: MAX_MEMPOOL_SNAPSHOT_TRANSACTION_BYTES,
            max_total_transaction_bytes: MAX_MEMPOOL_SNAPSHOT_TOTAL_TRANSACTION_BYTES,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MempoolSnapshotV2Dto {
    format_version: u32,
    captured_generation: u64,
    captured_at_unix_seconds: i64,
    records: Vec<MempoolSnapshotV2RecordDto>,
    unbroadcast_members: Vec<MempoolMemberIdentityDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MempoolSnapshotV2RecordDto {
    transaction: Vec<u8>,
    accepted_at_unix_seconds: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MempoolMemberIdentityDto {
    txid: [u8; 32],
    wtxid: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MempoolSnapshotV1Dto {
    records: Vec<MempoolSnapshotV1RecordDto>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MempoolOriginV1Dto {
    Local,
    Peer,
    Reorg,
    RecoveryUnknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MempoolSnapshotV1RecordDto {
    txid: [u8; 32],
    wtxid: [u8; 32],
    transaction: Vec<u8>,
    fee_sats: i64,
    virtual_size: usize,
    #[serde(
        default,
        rename = "accepted_at_unix_seconds",
        skip_serializing_if = "Option::is_none"
    )]
    maybe_accepted_at_unix_seconds: Option<i64>,
    #[serde(default, rename = "origin", skip_serializing_if = "Option::is_none")]
    maybe_origin: Option<MempoolOriginV1Dto>,
    #[serde(
        default,
        rename = "relay_requested",
        skip_serializing_if = "Option::is_none"
    )]
    maybe_relay_requested: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum MempoolSnapshotPayloadDto {
    CurrentV2(MempoolSnapshotV2Dto),
    LegacyV1(MempoolSnapshotV1Dto),
}

pub(crate) fn encode_mempool_snapshot(snapshot: &MempoolSnapshot) -> Result<Vec<u8>, StorageError> {
    if snapshot.format_version().is_some() {
        return encode_versioned(
            StorageNamespace::Mempool,
            &MempoolSnapshotV2Dto::try_from(snapshot)?,
        );
    }

    // Temporary Wave 1 facade: the sole production capture cannot provide v2
    // provenance until Plan 04. Keep this branch exact and remove it with that migration.
    encode_versioned(
        StorageNamespace::Mempool,
        &MempoolSnapshotV1Dto::try_from(snapshot)?,
    )
}

pub(crate) fn decode_mempool_snapshot(bytes: &[u8]) -> Result<MempoolSnapshot, StorageError> {
    decode_mempool_snapshot_with_limits(bytes, MempoolSnapshotDecodeLimits::default())
}

pub(crate) fn decode_mempool_snapshot_with_limits(
    bytes: &[u8],
    limits: MempoolSnapshotDecodeLimits,
) -> Result<MempoolSnapshot, StorageError> {
    if bytes.len() > limits.max_encoded_bytes {
        return Err(snapshot_failure(
            MempoolSnapshotError::ResourceBoundExceeded,
        ));
    }

    let payload: MempoolSnapshotPayloadDto =
        decode_versioned(StorageNamespace::Mempool, bytes).map_err(map_decode_failure)?;
    preflight_payload(&payload, limits)?;

    match payload {
        MempoolSnapshotPayloadDto::CurrentV2(dto) => dto.try_into(),
        MempoolSnapshotPayloadDto::LegacyV1(dto) => dto.try_into(),
    }
}

fn preflight_payload(
    payload: &MempoolSnapshotPayloadDto,
    limits: MempoolSnapshotDecodeLimits,
) -> Result<(), StorageError> {
    match payload {
        MempoolSnapshotPayloadDto::CurrentV2(dto) => {
            preflight_counts(dto.records.len(), dto.unbroadcast_members.len(), limits)?;
            preflight_transactions(
                dto.records.iter().map(|record| record.transaction.len()),
                limits,
            )
        }
        MempoolSnapshotPayloadDto::LegacyV1(dto) => {
            preflight_counts(dto.records.len(), 0, limits)?;
            preflight_transactions(
                dto.records.iter().map(|record| record.transaction.len()),
                limits,
            )
        }
    }
}

fn preflight_counts(
    record_count: usize,
    unbroadcast_count: usize,
    limits: MempoolSnapshotDecodeLimits,
) -> Result<(), StorageError> {
    if record_count > limits.max_records || unbroadcast_count > limits.max_unbroadcast_members {
        return Err(snapshot_failure(
            MempoolSnapshotError::ResourceBoundExceeded,
        ));
    }
    Ok(())
}

fn preflight_transactions(
    transaction_lengths: impl Iterator<Item = usize>,
    limits: MempoolSnapshotDecodeLimits,
) -> Result<(), StorageError> {
    let mut total_transaction_bytes = 0_usize;
    for transaction_bytes in transaction_lengths {
        if transaction_bytes > limits.max_transaction_bytes {
            return Err(snapshot_failure(
                MempoolSnapshotError::ResourceBoundExceeded,
            ));
        }
        total_transaction_bytes = total_transaction_bytes
            .checked_add(transaction_bytes)
            .ok_or_else(|| snapshot_failure(MempoolSnapshotError::ResourceBoundExceeded))?;
    }
    if total_transaction_bytes > limits.max_total_transaction_bytes {
        return Err(snapshot_failure(
            MempoolSnapshotError::ResourceBoundExceeded,
        ));
    }
    Ok(())
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
        let MempoolAcceptanceTime::Known(accepted_at) = record.acceptance_time else {
            return Err(snapshot_failure(MempoolSnapshotError::StructuralCorruption));
        };
        Ok(Self {
            transaction: encode_canonical_transaction(&record.transaction)?,
            accepted_at_unix_seconds: accepted_at.unix_seconds(),
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
                    MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(
                        record.accepted_at_unix_seconds,
                    )),
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
            CapturedMempoolGeneration::new(dto.captured_generation),
            PolicyTime::from_unix_seconds(dto.captured_at_unix_seconds),
            records,
            unbroadcast_members,
        )
        .map_err(snapshot_failure)
    }
}

impl TryFrom<&MempoolSnapshot> for MempoolSnapshotV1Dto {
    type Error = StorageError;

    fn try_from(snapshot: &MempoolSnapshot) -> Result<Self, Self::Error> {
        let records = snapshot
            .records
            .iter()
            .map(MempoolSnapshotV1RecordDto::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { records })
    }
}

impl TryFrom<&MempoolSnapshotRecord> for MempoolSnapshotV1RecordDto {
    type Error = StorageError;

    fn try_from(record: &MempoolSnapshotRecord) -> Result<Self, Self::Error> {
        let (maybe_accepted_at_unix_seconds, maybe_origin, maybe_relay_requested) =
            encode_v1_entry_metadata(record.metadata)?;
        Ok(Self {
            txid: record.txid.to_byte_array(),
            wtxid: record.wtxid.to_byte_array(),
            transaction: encode_canonical_transaction(&record.transaction)?,
            fee_sats: record.fee_sats,
            virtual_size: record.virtual_size,
            maybe_accepted_at_unix_seconds,
            maybe_origin,
            maybe_relay_requested,
        })
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
    let (_, virtual_size) = transaction_weight_and_virtual_size(&transaction)
        .map_err(|_| snapshot_failure(MempoolSnapshotError::StructuralCorruption))?;
    let metadata = MempoolEntryMetadata::new(
        acceptance_time,
        MempoolOrigin::RecoveryUnknown,
        RelayIntent::NotRequested,
    );

    Ok(MempoolSnapshotRecord {
        transaction,
        acceptance_time,
        txid,
        wtxid,
        fee_sats: 0,
        virtual_size,
        metadata,
    })
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

type EncodedV1Metadata = (Option<i64>, Option<MempoolOriginV1Dto>, Option<bool>);

fn encode_v1_entry_metadata(
    metadata: MempoolEntryMetadata,
) -> Result<EncodedV1Metadata, StorageError> {
    if metadata == MempoolEntryMetadata::legacy_unknown() {
        return Ok((None, None, None));
    }
    let MempoolAcceptanceTime::Known(accepted_at) = metadata.accepted_at else {
        return Err(snapshot_failure(MempoolSnapshotError::StructuralCorruption));
    };
    Ok((
        Some(accepted_at.unix_seconds()),
        Some(MempoolOriginV1Dto::from(metadata.origin)),
        Some(matches!(metadata.relay_intent, RelayIntent::Requested)),
    ))
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

fn map_decode_failure(error: StorageError) -> StorageError {
    match error {
        StorageError::InvalidSchemaVersion { .. } | StorageError::SchemaMismatch { .. } => error,
        _ => snapshot_failure(MempoolSnapshotError::DecodeFailure),
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

impl From<MempoolOrigin> for MempoolOriginV1Dto {
    fn from(origin: MempoolOrigin) -> Self {
        match origin {
            MempoolOrigin::Local => Self::Local,
            MempoolOrigin::Peer => Self::Peer,
            MempoolOrigin::Reorg => Self::Reorg,
            MempoolOrigin::RecoveryUnknown => Self::RecoveryUnknown,
        }
    }
}
