// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.h
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp

//! Compact first-party coins keys and values for the dedicated coins keyspace.

use open_bitcoin_codec::{
    primitives::{Reader, write_i64_le},
    read_compact_size, write_compact_size,
};
use open_bitcoin_core::{
    chainstate::Coin,
    primitives::{Amount, BlockHash, OutPoint, ScriptBuf, TransactionOutput, Txid},
};

use super::{StorageError, StorageNamespace, StorageRecoveryAction};

pub const DB_COIN: u8 = b'C';
pub const DB_BEST_BLOCK: u8 = b'B';
pub const DB_HEAD_BLOCKS: u8 = b'H';
pub const DEFAULT_COINS_DB_BATCH_BYTES: usize = 64 << 20;

#[cfg(test)]
mod tests;

fn coins_corruption(detail: impl core::fmt::Display) -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::Coins,
        detail: detail.to_string(),
        action: StorageRecoveryAction::Repair,
    }
}

pub fn decode_coin_key(key: &[u8]) -> Result<OutPoint, StorageError> {
    if key.len() != 37 || key.first().copied() != Some(DB_COIN) {
        return Err(coins_corruption("coin key must be C + txid + le vout"));
    }
    let txid_bytes: [u8; 32] = key[1..33]
        .try_into()
        .map_err(|_| coins_corruption("coin key txid slice"))?;
    let vout_bytes: [u8; 4] = key[33..37]
        .try_into()
        .map_err(|_| coins_corruption("coin key vout slice"))?;
    Ok(OutPoint {
        txid: Txid::from_byte_array(txid_bytes),
        vout: u32::from_le_bytes(vout_bytes),
    })
}

pub fn encode_coin_key(outpoint: &OutPoint) -> [u8; 37] {
    let mut key = [0_u8; 37];
    key[0] = DB_COIN;
    key[1..33].copy_from_slice(outpoint.txid.as_bytes());
    key[33..37].copy_from_slice(&outpoint.vout.to_le_bytes());
    key
}

pub fn encode_best_block_key() -> [u8; 1] {
    [DB_BEST_BLOCK]
}

pub fn encode_head_blocks_key() -> [u8; 1] {
    [DB_HEAD_BLOCKS]
}

pub fn encode_coin_value(coin: &Coin) -> Result<Vec<u8>, StorageError> {
    let mut out = Vec::new();
    let code = (u64::from(coin.created_height) << 1) | u64::from(coin.is_coinbase);
    write_compact_size(&mut out, code).map_err(coins_corruption)?;
    write_i64_le(&mut out, coin.output.value.to_sats());
    let script = coin.output.script_pubkey.as_bytes();
    let script_len = u64::try_from(script.len()).map_err(coins_corruption)?;
    write_compact_size(&mut out, script_len).map_err(coins_corruption)?;
    out.extend_from_slice(script);
    write_i64_le(&mut out, coin.created_median_time_past);
    Ok(out)
}

pub fn decode_coin_value(bytes: &[u8]) -> Result<Coin, StorageError> {
    let mut reader = Reader::new(bytes);
    let code = read_compact_size(&mut reader).map_err(coins_corruption)?;
    let height = code >> 1;
    if height > u64::from(u32::MAX) {
        return Err(coins_corruption("created_height exceeds u32"));
    }
    let created_height = u32::try_from(height).map_err(coins_corruption)?;
    let is_coinbase = code & 1 == 1;
    let sats = reader.read_i64_le().map_err(coins_corruption)?;
    let value = Amount::from_sats(sats).map_err(coins_corruption)?;
    let script_len = read_compact_size(&mut reader).map_err(coins_corruption)?;
    let script_len = usize::try_from(script_len).map_err(coins_corruption)?;
    let script_bytes = reader.read_vec(script_len).map_err(coins_corruption)?;
    let script_pubkey = ScriptBuf::from_bytes(script_bytes).map_err(coins_corruption)?;
    let created_median_time_past = reader.read_i64_le().map_err(coins_corruption)?;
    reader.finish().map_err(coins_corruption)?;
    Ok(Coin {
        output: TransactionOutput {
            value,
            script_pubkey,
        },
        is_coinbase,
        created_height,
        created_median_time_past,
    })
}

pub fn encode_best_block_value(block_hash: BlockHash) -> Vec<u8> {
    block_hash.as_bytes().to_vec()
}

pub fn decode_best_block_value(bytes: &[u8]) -> Result<BlockHash, StorageError> {
    let mut reader = Reader::new(bytes);
    let hash = reader.read_array::<32>().map_err(coins_corruption)?;
    reader.finish().map_err(coins_corruption)?;
    Ok(BlockHash::from_byte_array(hash))
}

pub fn encode_head_blocks_value(heads: &[BlockHash]) -> Result<Vec<u8>, StorageError> {
    let mut out = Vec::new();
    let count = u64::try_from(heads.len()).map_err(coins_corruption)?;
    write_compact_size(&mut out, count).map_err(coins_corruption)?;
    for hash in heads {
        out.extend_from_slice(hash.as_bytes());
    }
    Ok(out)
}

pub fn decode_head_blocks_value(bytes: &[u8]) -> Result<Vec<BlockHash>, StorageError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }

    let mut reader = Reader::new(bytes);
    let count = read_compact_size(&mut reader).map_err(coins_corruption)?;
    if count != 2 {
        return Err(coins_corruption(format!(
            "head_blocks count must be 2, got {count}"
        )));
    }

    let mut heads = Vec::with_capacity(2);
    for _ in 0..count {
        let hash = reader.read_array::<32>().map_err(coins_corruption)?;
        heads.push(BlockHash::from_byte_array(hash));
    }
    reader.finish().map_err(coins_corruption)?;
    Ok(heads)
}

pub fn estimated_encoded_bytes(key: &[u8], maybe_value: Option<&[u8]>) -> usize {
    key.len() + maybe_value.map_or(0, <[u8]>::len)
}
