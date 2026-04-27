// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Node-owned storage DTOs for durable snapshot persistence.

use std::collections::HashMap;

use open_bitcoin_core::{
    chainstate::{BlockUndo, ChainPosition, ChainstateSnapshot, Coin, TxUndo},
    consensus::block_hash,
    primitives::{
        Amount, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, TransactionOutput, Txid,
    },
};
use open_bitcoin_network::HeaderEntry;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{
    RecoveryMarker, RuntimeMetadata, SchemaVersion, StorageError, StorageNamespace,
    StorageRecoveryAction, metrics::MetricSample,
};

mod wallet;

pub(crate) use wallet::{
    decode_selected_wallet, decode_wallet_registry_snapshot, decode_wallet_rescan_job,
    decode_wallet_snapshot, encode_selected_wallet, encode_wallet_registry_snapshot,
    encode_wallet_rescan_job, encode_wallet_snapshot,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct VersionedSnapshot<T> {
    schema_version: u32,
    payload: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct BlockHeaderDto {
    version: i32,
    previous_block_hash: [u8; 32],
    merkle_root: [u8; 32],
    time: u32,
    bits: u32,
    nonce: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ChainPositionDto {
    block_hash: [u8; 32],
    header: BlockHeaderDto,
    height: u32,
    chain_work: u128,
    median_time_past: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct OutPointDto {
    txid: [u8; 32],
    vout: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TransactionOutputDto {
    value_sats: i64,
    script_pubkey: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CoinDto {
    output: TransactionOutputDto,
    is_coinbase: bool,
    created_height: u32,
    created_median_time_past: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct UtxoRecordDto {
    outpoint: OutPointDto,
    coin: CoinDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TxUndoDto {
    restored_inputs: Vec<CoinDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct BlockUndoDto {
    transactions: Vec<TxUndoDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct BlockUndoRecordDto {
    block_hash: [u8; 32],
    undo: BlockUndoDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ChainstateSnapshotDto {
    active_chain: Vec<ChainPositionDto>,
    utxos: Vec<UtxoRecordDto>,
    undo_by_block: Vec<BlockUndoRecordDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HeaderEntryDto {
    block_hash: [u8; 32],
    header: BlockHeaderDto,
    height: u32,
    chain_work: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HeaderEntriesDto {
    entries: Vec<HeaderEntryDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricsStorageSnapshot {
    pub samples: Vec<MetricSample>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredHeaderEntries {
    pub entries: Vec<HeaderEntry>,
}

pub(crate) fn encode_chainstate_snapshot(
    snapshot: &ChainstateSnapshot,
) -> Result<Vec<u8>, StorageError> {
    encode_versioned(
        StorageNamespace::Chainstate,
        &ChainstateSnapshotDto::from(snapshot),
    )
}

pub(crate) fn decode_chainstate_snapshot(bytes: &[u8]) -> Result<ChainstateSnapshot, StorageError> {
    let dto: ChainstateSnapshotDto = decode_versioned(StorageNamespace::Chainstate, bytes)?;
    dto.try_into()
}

pub(crate) fn encode_header_entries(entries: &[HeaderEntry]) -> Result<Vec<u8>, StorageError> {
    encode_header_entries_for(StorageNamespace::Headers, entries)
}

pub(crate) fn encode_block_index_entries(entries: &[HeaderEntry]) -> Result<Vec<u8>, StorageError> {
    encode_header_entries_for(StorageNamespace::BlockIndex, entries)
}

pub(crate) fn decode_block_index_entries(
    bytes: &[u8],
) -> Result<StoredHeaderEntries, StorageError> {
    decode_header_entries_for(StorageNamespace::BlockIndex, bytes)
}

fn encode_header_entries_for(
    namespace: StorageNamespace,
    entries: &[HeaderEntry],
) -> Result<Vec<u8>, StorageError> {
    let dto = HeaderEntriesDto {
        entries: entries.iter().map(HeaderEntryDto::from).collect(),
    };
    encode_versioned(namespace, &dto)
}

pub(crate) fn decode_header_entries(bytes: &[u8]) -> Result<StoredHeaderEntries, StorageError> {
    decode_header_entries_for(StorageNamespace::Headers, bytes)
}

fn decode_header_entries_for(
    namespace: StorageNamespace,
    bytes: &[u8],
) -> Result<StoredHeaderEntries, StorageError> {
    let dto: HeaderEntriesDto = decode_versioned(namespace, bytes)?;
    let entries = dto
        .entries
        .into_iter()
        .map(HeaderEntry::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(StoredHeaderEntries { entries })
}

pub(crate) fn encode_runtime_metadata(metadata: &RuntimeMetadata) -> Result<Vec<u8>, StorageError> {
    encode_versioned(StorageNamespace::Runtime, metadata)
}

pub(crate) fn decode_runtime_metadata(bytes: &[u8]) -> Result<RuntimeMetadata, StorageError> {
    decode_versioned(StorageNamespace::Runtime, bytes)
}

pub(crate) fn encode_recovery_marker(marker: &RecoveryMarker) -> Result<Vec<u8>, StorageError> {
    encode_versioned(StorageNamespace::Runtime, marker)
}

pub(crate) fn decode_recovery_marker(bytes: &[u8]) -> Result<RecoveryMarker, StorageError> {
    decode_versioned(StorageNamespace::Runtime, bytes)
}

pub(crate) fn encode_metrics_snapshot(
    snapshot: &MetricsStorageSnapshot,
) -> Result<Vec<u8>, StorageError> {
    encode_versioned(StorageNamespace::Metrics, snapshot)
}

pub(crate) fn decode_metrics_snapshot(
    bytes: &[u8],
) -> Result<MetricsStorageSnapshot, StorageError> {
    decode_versioned(StorageNamespace::Metrics, bytes)
}

fn encode_versioned<T: Serialize>(
    namespace: StorageNamespace,
    payload: &T,
) -> Result<Vec<u8>, StorageError> {
    let snapshot = VersionedSnapshot {
        schema_version: SchemaVersion::CURRENT.get(),
        payload,
    };
    serde_json::to_vec_pretty(&snapshot).map_err(|error| StorageError::BackendFailure {
        namespace,
        message: format!("failed to encode storage snapshot: {error}"),
        action: StorageRecoveryAction::Restart,
    })
}

fn decode_versioned<T: DeserializeOwned>(
    namespace: StorageNamespace,
    bytes: &[u8],
) -> Result<T, StorageError> {
    let snapshot: VersionedSnapshot<T> =
        serde_json::from_slice(bytes).map_err(|error| corruption(namespace, error))?;
    let actual = SchemaVersion::new(snapshot.schema_version)?;
    if actual != SchemaVersion::CURRENT {
        return Err(StorageError::schema_mismatch(
            SchemaVersion::CURRENT,
            actual,
        ));
    }

    Ok(snapshot.payload)
}

fn corruption(namespace: StorageNamespace, detail: impl std::fmt::Display) -> StorageError {
    StorageError::Corruption {
        namespace,
        detail: detail.to_string(),
        action: StorageRecoveryAction::Repair,
    }
}

impl From<&BlockHeader> for BlockHeaderDto {
    fn from(header: &BlockHeader) -> Self {
        Self {
            version: header.version,
            previous_block_hash: header.previous_block_hash.to_byte_array(),
            merkle_root: header.merkle_root.to_byte_array(),
            time: header.time,
            bits: header.bits,
            nonce: header.nonce,
        }
    }
}

impl From<BlockHeaderDto> for BlockHeader {
    fn from(dto: BlockHeaderDto) -> Self {
        Self {
            version: dto.version,
            previous_block_hash: BlockHash::from_byte_array(dto.previous_block_hash),
            merkle_root: MerkleRoot::from_byte_array(dto.merkle_root),
            time: dto.time,
            bits: dto.bits,
            nonce: dto.nonce,
        }
    }
}

impl From<&ChainPosition> for ChainPositionDto {
    fn from(position: &ChainPosition) -> Self {
        Self {
            block_hash: position.block_hash.to_byte_array(),
            header: BlockHeaderDto::from(&position.header),
            height: position.height,
            chain_work: position.chain_work,
            median_time_past: position.median_time_past,
        }
    }
}

impl TryFrom<ChainPositionDto> for ChainPosition {
    type Error = StorageError;

    fn try_from(dto: ChainPositionDto) -> Result<Self, Self::Error> {
        let stored_hash = BlockHash::from_byte_array(dto.block_hash);
        let header = BlockHeader::from(dto.header);
        let position = ChainPosition::new(header, dto.height, dto.chain_work, dto.median_time_past);
        if position.block_hash != stored_hash {
            return Err(corruption(
                StorageNamespace::BlockIndex,
                "stored block hash does not match header",
            ));
        }

        Ok(position)
    }
}

impl From<&OutPoint> for OutPointDto {
    fn from(outpoint: &OutPoint) -> Self {
        Self {
            txid: outpoint.txid.to_byte_array(),
            vout: outpoint.vout,
        }
    }
}

impl From<OutPointDto> for OutPoint {
    fn from(dto: OutPointDto) -> Self {
        Self {
            txid: Txid::from_byte_array(dto.txid),
            vout: dto.vout,
        }
    }
}

impl From<&TransactionOutput> for TransactionOutputDto {
    fn from(output: &TransactionOutput) -> Self {
        Self {
            value_sats: output.value.to_sats(),
            script_pubkey: output.script_pubkey.as_bytes().to_vec(),
        }
    }
}

impl TryFrom<TransactionOutputDto> for TransactionOutput {
    type Error = StorageError;

    fn try_from(dto: TransactionOutputDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: Amount::from_sats(dto.value_sats)
                .map_err(|error| corruption(StorageNamespace::Chainstate, error))?,
            script_pubkey: ScriptBuf::from_bytes(dto.script_pubkey)
                .map_err(|error| corruption(StorageNamespace::Chainstate, error))?,
        })
    }
}

impl From<&Coin> for CoinDto {
    fn from(coin: &Coin) -> Self {
        Self {
            output: TransactionOutputDto::from(&coin.output),
            is_coinbase: coin.is_coinbase,
            created_height: coin.created_height,
            created_median_time_past: coin.created_median_time_past,
        }
    }
}

impl TryFrom<CoinDto> for Coin {
    type Error = StorageError;

    fn try_from(dto: CoinDto) -> Result<Self, Self::Error> {
        Ok(Self {
            output: dto.output.try_into()?,
            is_coinbase: dto.is_coinbase,
            created_height: dto.created_height,
            created_median_time_past: dto.created_median_time_past,
        })
    }
}

impl From<&ChainstateSnapshot> for ChainstateSnapshotDto {
    fn from(snapshot: &ChainstateSnapshot) -> Self {
        let utxos = snapshot
            .utxos
            .iter()
            .map(|(outpoint, coin)| UtxoRecordDto {
                outpoint: OutPointDto::from(outpoint),
                coin: CoinDto::from(coin),
            })
            .collect();
        let undo_by_block = snapshot
            .undo_by_block
            .iter()
            .map(|(block_hash, undo)| BlockUndoRecordDto {
                block_hash: block_hash.to_byte_array(),
                undo: BlockUndoDto::from(undo),
            })
            .collect();

        Self {
            active_chain: snapshot
                .active_chain
                .iter()
                .map(ChainPositionDto::from)
                .collect(),
            utxos,
            undo_by_block,
        }
    }
}

impl TryFrom<ChainstateSnapshotDto> for ChainstateSnapshot {
    type Error = StorageError;

    fn try_from(dto: ChainstateSnapshotDto) -> Result<Self, Self::Error> {
        let active_chain = dto
            .active_chain
            .into_iter()
            .map(ChainPosition::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let utxos = dto
            .utxos
            .into_iter()
            .map(|record| {
                Ok((
                    OutPoint::from(record.outpoint),
                    Coin::try_from(record.coin)?,
                ))
            })
            .collect::<Result<HashMap<_, _>, StorageError>>()?;
        let undo_by_block = dto
            .undo_by_block
            .into_iter()
            .map(|record| {
                Ok((
                    BlockHash::from_byte_array(record.block_hash),
                    BlockUndo::try_from(record.undo)?,
                ))
            })
            .collect::<Result<HashMap<_, _>, StorageError>>()?;

        Ok(Self::new(active_chain, utxos, undo_by_block))
    }
}

impl From<&TxUndo> for TxUndoDto {
    fn from(undo: &TxUndo) -> Self {
        Self {
            restored_inputs: undo.restored_inputs.iter().map(CoinDto::from).collect(),
        }
    }
}

impl TryFrom<TxUndoDto> for TxUndo {
    type Error = StorageError;

    fn try_from(dto: TxUndoDto) -> Result<Self, Self::Error> {
        Ok(Self {
            restored_inputs: dto
                .restored_inputs
                .into_iter()
                .map(Coin::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl From<&BlockUndo> for BlockUndoDto {
    fn from(undo: &BlockUndo) -> Self {
        Self {
            transactions: undo.transactions.iter().map(TxUndoDto::from).collect(),
        }
    }
}

impl TryFrom<BlockUndoDto> for BlockUndo {
    type Error = StorageError;

    fn try_from(dto: BlockUndoDto) -> Result<Self, Self::Error> {
        Ok(Self {
            transactions: dto
                .transactions
                .into_iter()
                .map(TxUndo::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl From<&HeaderEntry> for HeaderEntryDto {
    fn from(entry: &HeaderEntry) -> Self {
        Self {
            block_hash: entry.block_hash.to_byte_array(),
            header: BlockHeaderDto::from(&entry.header),
            height: entry.height,
            chain_work: entry.chain_work,
        }
    }
}

impl TryFrom<HeaderEntryDto> for HeaderEntry {
    type Error = StorageError;

    fn try_from(dto: HeaderEntryDto) -> Result<Self, Self::Error> {
        let stored_hash = BlockHash::from_byte_array(dto.block_hash);
        let header = BlockHeader::from(dto.header);
        if block_hash(&header) != stored_hash {
            return Err(corruption(
                StorageNamespace::Headers,
                "stored header hash does not match header",
            ));
        }

        Ok(Self {
            block_hash: stored_hash,
            header,
            height: dto.height,
            chain_work: dto.chain_work,
        })
    }
}
#[cfg(test)]
mod tests;
