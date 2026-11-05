// Parity breadcrumbs:
// - packages/bitcoin-knots/src/undo.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/coins.cpp

//! Lossless untrusted undo carriers. Integrity never grants canonical authority.

use super::*;
use open_bitcoin_core::chainstate::types::{BlockOverwriteUndo, OverwrittenCoin};

// Local storage resource bounds, deliberately above a consensus block's entries.
// These are carrier limits, not claims that a decoded record passed consensus.
const MAX_RECORDS: usize = 4_000_000;
const MAX_BYTES: usize = 256 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct BlockOverwriteUndoDto {
    block_hash: [u8; 32],
    height: u32,
    creating_median_time_past: i64,
    overwritten: Vec<OverwrittenCoinDto>,
    integrity: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct OverwrittenCoinDto {
    transaction_index: u32,
    outpoint: OutPointDto,
    previous_coin: CoinDto,
    new_coin: CoinDto,
}

fn invalid(detail: impl std::fmt::Display) -> StorageError {
    corruption(StorageNamespace::Chainstate, detail)
}

pub(super) fn check_bytes(bytes: &[u8]) -> Result<(), StorageError> {
    if bytes.len() > MAX_BYTES {
        return Err(invalid("undo/snapshot byte bound exceeded"));
    }
    Ok(())
}

pub(super) fn encode_carrier(value: &impl Serialize) -> Result<Vec<u8>, StorageError> {
    let bytes = encode_versioned(StorageNamespace::Chainstate, value)?;
    check_bytes(&bytes)?;
    Ok(bytes)
}

pub(super) fn check_snapshot_counts(dto: &ChainstateSnapshotDto) -> Result<(), StorageError> {
    check_count(dto.active_chain.len())?;
    check_count(dto.utxos.len())?;
    check_count(dto.undo_by_block.len())?;
    if let Some(counts) = &dto.maybe_confirmed_txid_counts {
        check_count(counts.len())?;
    }
    if let Some(txids) = &dto.maybe_confirmed_txids {
        check_count(txids.len())?;
    }
    Ok(())
}

pub(super) fn check_count(count: usize) -> Result<(), StorageError> {
    if count > MAX_RECORDS {
        return Err(invalid("undo/snapshot record bound exceeded"));
    }
    Ok(())
}

fn check_metadata(height: u32, mtp: i64) -> Result<(), StorageError> {
    if height > i32::MAX as u32 || !(0..=i64::from(u32::MAX)).contains(&mtp) {
        return Err(invalid("undo coin height/time out of range"));
    }
    Ok(())
}

fn check_coin(coin: &Coin) -> Result<(), StorageError> {
    check_metadata(coin.created_height, coin.created_median_time_past)
}

pub(super) fn encode_overwrite_records(
    records: &[BlockOverwriteUndo],
) -> Vec<BlockOverwriteUndoDto> {
    records
        .iter()
        .map(|batch| BlockOverwriteUndoDto {
            block_hash: batch.block_hash.to_byte_array(),
            height: batch.height,
            creating_median_time_past: batch.creating_median_time_past,
            integrity: batch.integrity,
            overwritten: batch
                .overwritten
                .iter()
                .map(|entry| OverwrittenCoinDto {
                    transaction_index: entry.transaction_index,
                    outpoint: OutPointDto::from(&entry.outpoint),
                    previous_coin: CoinDto::from(&entry.previous_coin),
                    new_coin: CoinDto::from(&entry.new_coin),
                })
                .collect(),
        })
        .collect()
}

pub(super) fn decode_overwrite_records(
    records: Vec<BlockOverwriteUndoDto>,
) -> Result<Vec<BlockOverwriteUndo>, StorageError> {
    if records.len() > 1 {
        return Err(invalid("duplicate complete overwrite batch"));
    }
    records
        .into_iter()
        .map(|batch| {
            check_count(batch.overwritten.len())?;
            let overwritten = batch
                .overwritten
                .into_iter()
                .map(|entry| {
                    Ok(OverwrittenCoin {
                        transaction_index: entry.transaction_index,
                        outpoint: entry.outpoint.into(),
                        previous_coin: entry.previous_coin.try_into()?,
                        new_coin: entry.new_coin.try_into()?,
                    })
                })
                .collect::<Result<Vec<_>, StorageError>>()?;
            Ok(BlockOverwriteUndo {
                block_hash: BlockHash::from_byte_array(batch.block_hash),
                height: batch.height,
                creating_median_time_past: batch.creating_median_time_past,
                overwritten,
                integrity: batch.integrity,
            })
        })
        .collect()
}

pub(super) fn validate_undo(undo: &BlockUndo) -> Result<(), StorageError> {
    check_count(undo.transactions.len())?;
    let mut count = 0usize;
    for tx in &undo.transactions {
        count = count
            .checked_add(tx.restored_inputs.len())
            .ok_or_else(|| invalid("undo count overflow"))?;
        check_count(count)?;
        for coin in &tx.restored_inputs {
            check_coin(coin)?;
        }
    }
    // An explicit empty outer collection is unknown data, never an empty scan.
    if undo.overwrites.len() > 1 {
        return Err(invalid("duplicate complete overwrite batch"));
    }
    for batch in &undo.overwrites {
        check_metadata(batch.height, batch.creating_median_time_past)?;
        check_count(batch.overwritten.len())?;
        batch.validate_integrity().map_err(invalid)?;
        let mut seen = HashSet::new();
        for entry in &batch.overwritten {
            check_coin(&entry.previous_coin)?;
            check_coin(&entry.new_coin)?;
            if !seen.insert(&entry.outpoint)
                || entry.transaction_index != 0
                || entry.outpoint.vout == u32::MAX
                || !entry.previous_coin.is_coinbase
                || !entry.new_coin.is_coinbase
                || entry.previous_coin.created_height >= batch.height
                || entry.new_coin.created_height != batch.height
                || entry.new_coin.created_median_time_past != batch.creating_median_time_past
                || entry.previous_coin.output != entry.new_coin.output
            {
                return Err(invalid("invalid complete overwrite occurrence"));
            }
        }
    }
    Ok(())
}

pub(super) fn validate_snapshot(snapshot: &ChainstateSnapshot) -> Result<(), StorageError> {
    check_count(snapshot.active_chain.len())?;
    check_count(snapshot.utxos.len())?;
    check_count(snapshot.undo_by_block.len())?;
    let mut positions = HashMap::new();
    for (index, position) in snapshot.active_chain.iter().enumerate() {
        if positions
            .insert(position.block_hash, (index, position))
            .is_some()
        {
            return Err(invalid("duplicate snapshot block position"));
        }
    }
    for (hash, undo) in &snapshot.undo_by_block {
        validate_undo(undo)?;
        for batch in &undo.overwrites {
            let (index, position) = positions
                .get(hash)
                .ok_or_else(|| invalid("overwrite batch missing active position"))?;
            if batch.block_hash != *hash || batch.height != position.height {
                return Err(invalid("overwrite batch snapshot binding mismatch"));
            }
            if let Some(parent) = index
                .checked_sub(1)
                .and_then(|i| snapshot.active_chain.get(i))
                && batch.creating_median_time_past != parent.median_time_past
            {
                return Err(invalid(
                    "overwrite batch creating MTP differs from stored parent",
                ));
            }
        }
    }
    Ok(())
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
        check_count(dto.restored_inputs.len())?;
        Ok(Self {
            restored_inputs: dto
                .restored_inputs
                .into_iter()
                .map(Coin::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
