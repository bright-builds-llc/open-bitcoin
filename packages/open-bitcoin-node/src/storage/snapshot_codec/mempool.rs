// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use open_bitcoin_core::{
    codec::{TransactionEncoding, encode_transaction, parse_transaction},
    consensus::{transaction_txid, transaction_wtxid},
    primitives::{Txid, Wtxid},
};
use open_bitcoin_mempool::{
    MempoolAcceptanceTime, MempoolEntryMetadata, MempoolOrigin, PolicyTime, RelayIntent,
};
use serde::{Deserialize, Serialize};

use super::{corruption, decode_versioned, encode_versioned};
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};
use crate::{StorageError, StorageNamespace};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MempoolSnapshotDto {
    records: Vec<MempoolSnapshotRecordDto>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MempoolOriginDto {
    Local,
    Peer,
    Reorg,
    RecoveryUnknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MempoolSnapshotRecordDto {
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
    maybe_origin: Option<MempoolOriginDto>,
    #[serde(
        default,
        rename = "relay_requested",
        skip_serializing_if = "Option::is_none"
    )]
    maybe_relay_requested: Option<bool>,
}

pub(crate) fn encode_mempool_snapshot(snapshot: &MempoolSnapshot) -> Result<Vec<u8>, StorageError> {
    encode_versioned(
        StorageNamespace::Mempool,
        &MempoolSnapshotDto::try_from(snapshot)?,
    )
}

pub(crate) fn decode_mempool_snapshot(bytes: &[u8]) -> Result<MempoolSnapshot, StorageError> {
    let dto: MempoolSnapshotDto = decode_versioned(StorageNamespace::Mempool, bytes)?;
    dto.try_into()
}

impl TryFrom<&MempoolSnapshot> for MempoolSnapshotDto {
    type Error = StorageError;

    fn try_from(snapshot: &MempoolSnapshot) -> Result<Self, Self::Error> {
        let records = snapshot
            .records
            .iter()
            .map(MempoolSnapshotRecordDto::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { records })
    }
}

impl TryFrom<MempoolSnapshotDto> for MempoolSnapshot {
    type Error = StorageError;

    fn try_from(dto: MempoolSnapshotDto) -> Result<Self, Self::Error> {
        let records = dto
            .records
            .into_iter()
            .map(MempoolSnapshotRecord::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { records })
    }
}

impl TryFrom<&MempoolSnapshotRecord> for MempoolSnapshotRecordDto {
    type Error = StorageError;

    fn try_from(record: &MempoolSnapshotRecord) -> Result<Self, Self::Error> {
        let (maybe_accepted_at_unix_seconds, maybe_origin, maybe_relay_requested) =
            encode_entry_metadata(record.metadata)?;
        Ok(Self {
            txid: record.txid.to_byte_array(),
            wtxid: record.wtxid.to_byte_array(),
            transaction: encode_transaction(&record.transaction, TransactionEncoding::WithWitness)
                .map_err(|error| corruption(StorageNamespace::Mempool, error))?,
            fee_sats: record.fee_sats,
            virtual_size: record.virtual_size,
            maybe_accepted_at_unix_seconds,
            maybe_origin,
            maybe_relay_requested,
        })
    }
}

impl TryFrom<MempoolSnapshotRecordDto> for MempoolSnapshotRecord {
    type Error = StorageError;

    fn try_from(dto: MempoolSnapshotRecordDto) -> Result<Self, Self::Error> {
        let transaction = parse_transaction(&dto.transaction)
            .map_err(|error| corruption(StorageNamespace::Mempool, error))?;
        let txid = Txid::from_byte_array(dto.txid);
        let wtxid = Wtxid::from_byte_array(dto.wtxid);
        let actual_txid = transaction_txid(&transaction)
            .map_err(|error| corruption(StorageNamespace::Mempool, error))?;
        let actual_wtxid = transaction_wtxid(&transaction)
            .map_err(|error| corruption(StorageNamespace::Mempool, error))?;
        if actual_txid != txid {
            return Err(corruption(
                StorageNamespace::Mempool,
                "stored mempool txid does not match transaction",
            ));
        }
        if actual_wtxid != wtxid {
            return Err(corruption(
                StorageNamespace::Mempool,
                "stored mempool wtxid does not match transaction",
            ));
        }
        let metadata = decode_entry_metadata(
            dto.maybe_accepted_at_unix_seconds,
            dto.maybe_origin,
            dto.maybe_relay_requested,
        )?;

        Ok(Self {
            txid,
            wtxid,
            transaction,
            fee_sats: dto.fee_sats,
            virtual_size: dto.virtual_size,
            metadata,
        })
    }
}

type EncodedEntryMetadataFields = (Option<i64>, Option<MempoolOriginDto>, Option<bool>);

fn encode_entry_metadata(
    metadata: MempoolEntryMetadata,
) -> Result<EncodedEntryMetadataFields, StorageError> {
    if metadata == MempoolEntryMetadata::legacy_unknown() {
        return Ok((None, None, None));
    }
    let MempoolAcceptanceTime::Known(accepted_at) = metadata.accepted_at else {
        return Err(corruption(
            StorageNamespace::Mempool,
            "incomplete mempool entry metadata cannot be encoded",
        ));
    };
    Ok((
        Some(accepted_at.unix_seconds()),
        Some(MempoolOriginDto::from(metadata.origin)),
        Some(matches!(metadata.relay_intent, RelayIntent::Requested)),
    ))
}

fn decode_entry_metadata(
    maybe_accepted_at_unix_seconds: Option<i64>,
    maybe_origin: Option<MempoolOriginDto>,
    maybe_relay_requested: Option<bool>,
) -> Result<MempoolEntryMetadata, StorageError> {
    match (
        maybe_accepted_at_unix_seconds,
        maybe_origin,
        maybe_relay_requested,
    ) {
        (Some(accepted_at_unix_seconds), Some(origin), Some(relay_requested)) => {
            Ok(MempoolEntryMetadata::new(
                MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(
                    accepted_at_unix_seconds,
                )),
                MempoolOrigin::from(origin),
                if relay_requested {
                    RelayIntent::Requested
                } else {
                    RelayIntent::NotRequested
                },
            ))
        }
        (None, None, None) => Ok(MempoolEntryMetadata::legacy_unknown()),
        _ => Err(corruption(
            StorageNamespace::Mempool,
            "partial mempool entry metadata is corrupt",
        )),
    }
}

impl From<MempoolOrigin> for MempoolOriginDto {
    fn from(origin: MempoolOrigin) -> Self {
        match origin {
            MempoolOrigin::Local => Self::Local,
            MempoolOrigin::Peer => Self::Peer,
            MempoolOrigin::Reorg => Self::Reorg,
            MempoolOrigin::RecoveryUnknown => Self::RecoveryUnknown,
        }
    }
}

impl From<MempoolOriginDto> for MempoolOrigin {
    fn from(origin: MempoolOriginDto) -> Self {
        match origin {
            MempoolOriginDto::Local => Self::Local,
            MempoolOriginDto::Peer => Self::Peer,
            MempoolOriginDto::Reorg => Self::Reorg,
            MempoolOriginDto::RecoveryUnknown => Self::RecoveryUnknown,
        }
    }
}
