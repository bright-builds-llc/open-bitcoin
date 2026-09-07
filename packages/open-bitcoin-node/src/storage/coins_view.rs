// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.h
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp

//! Fjall-backed `CoinsView` parent with Knots two-phase `H`/`B` BatchWrite.

use std::collections::HashMap;

use fjall::PersistMode as FjallPersistMode;
use open_bitcoin_core::{
    chainstate::{ChainstateError, Coin, CoinsBatch, CoinsView},
    primitives::{BlockHash, OutPoint},
};

use super::{
    FjallNodeStore, PersistMode, StorageError, StorageNamespace, StorageRecoveryAction,
    coins_codec::{
        DB_COIN, DEFAULT_COINS_DB_BATCH_BYTES, decode_best_block_value, decode_coin_key,
        decode_coin_value, decode_head_blocks_value, encode_best_block_key,
        encode_best_block_value, encode_coin_key, encode_coin_value, encode_head_blocks_key,
        encode_head_blocks_value, estimated_encoded_bytes,
    },
};

#[cfg(test)]
mod tests;

enum MarkerState {
    Empty,
    Consistent { best_block: BlockHash },
    Interrupted { new: BlockHash, old: BlockHash },
}

/// Disk-backed coins parent. Application consistency is `H` then coins then `B`.
pub struct FjallCoinsView {
    db: fjall::Database,
    coins: fjall::Keyspace,
    #[cfg(test)]
    simulate_crash_after_partial: bool,
    #[cfg(test)]
    partial_buffered_commits: std::cell::Cell<u32>,
}

impl FjallCoinsView {
    pub fn from_store(store: &FjallNodeStore) -> Self {
        Self {
            db: store.database().clone(),
            coins: store.coins_keyspace().clone(),
            #[cfg(test)]
            simulate_crash_after_partial: false,
            #[cfg(test)]
            partial_buffered_commits: std::cell::Cell::new(0),
        }
    }

    pub fn persist_mode_for_final_best_block() -> PersistMode {
        PersistMode::Flush
    }

    #[cfg(test)]
    pub fn write_raw_bytes(&self, key: &[u8], value: Vec<u8>) -> Result<(), StorageError> {
        self.coins
            .insert(key, value)
            .map_err(coins_backend_failure)?;
        self.db
            .persist(FjallPersistMode::SyncAll)
            .map_err(coins_backend_failure)
    }

    #[cfg(test)]
    pub fn read_raw_bytes(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        self.coins_get(key)
    }

    #[cfg(test)]
    pub fn contains_raw_key(&self, key: &[u8]) -> Result<bool, StorageError> {
        self.coins_contains(key)
    }

    #[cfg(test)]
    pub fn set_simulate_crash_after_partial(&mut self, enabled: bool) {
        self.simulate_crash_after_partial = enabled;
    }

    #[cfg(test)]
    pub fn partial_buffered_commits(&self) -> u32 {
        self.partial_buffered_commits.get()
    }

    #[cfg(test)]
    pub fn batch_write_with_limit(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
        cap_bytes: usize,
    ) -> Result<(), ChainstateError> {
        self.batch_write_capped(
            writes,
            maybe_best_block,
            Self::persist_mode_for_final_best_block(),
            cap_bytes,
            false,
        )
    }

    pub fn batch_write_with_persist_mode(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
        final_mode: PersistMode,
    ) -> Result<(), ChainstateError> {
        self.batch_write_capped(
            writes,
            maybe_best_block,
            final_mode,
            DEFAULT_COINS_DB_BATCH_BYTES,
            true,
        )
    }

    #[cfg(test)]
    pub fn delete_raw_bytes(&self, key: &[u8]) -> Result<(), StorageError> {
        self.coins.remove(key).map_err(coins_backend_failure)?;
        self.db
            .persist(FjallPersistMode::SyncAll)
            .map_err(coins_backend_failure)
    }

    fn coins_get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StorageError> {
        self.coins
            .get(key)
            .map(|maybe_bytes| maybe_bytes.map(|bytes| bytes.as_ref().to_vec()))
            .map_err(coins_backend_failure)
    }

    fn coins_contains(&self, key: &[u8]) -> Result<bool, StorageError> {
        self.coins.contains_key(key).map_err(coins_backend_failure)
    }

    fn classify_markers(&self) -> Result<MarkerState, StorageError> {
        let maybe_heads = self.coins_get(&encode_head_blocks_key())?;
        let maybe_best = self.coins_get(&encode_best_block_key())?;

        let Some(heads_bytes) = maybe_heads else {
            return match maybe_best {
                Some(best_bytes) => Ok(MarkerState::Consistent {
                    best_block: decode_best_block_value(&best_bytes)?,
                }),
                None => Ok(MarkerState::Empty),
            };
        };

        let heads = decode_head_blocks_value(&heads_bytes)?;
        match (heads.len(), maybe_best) {
            (2, None) => Ok(MarkerState::Interrupted {
                new: heads[0],
                old: heads[1],
            }),
            (2, Some(_)) => Err(coins_corruption(
                "head_blocks still present after best-block write",
            )),
            (0, Some(best_bytes)) => Ok(MarkerState::Consistent {
                best_block: decode_best_block_value(&best_bytes)?,
            }),
            (0, None) => Ok(MarkerState::Empty),
            (count, _) => Err(coins_corruption(format!(
                "unexpected head_blocks count {count}"
            ))),
        }
    }

    fn batch_write_capped(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
        final_mode: PersistMode,
        cap_bytes: usize,
        allow_existing_heads: bool,
    ) -> Result<(), ChainstateError> {
        if !allow_existing_heads
            && self
                .coins_contains(&encode_head_blocks_key())
                .map_err(map_storage)?
        {
            return Err(map_storage(interrupted_write()));
        }

        let Some(new_tip) = maybe_best_block else {
            return Err(ChainstateError::CoinsStorage {
                detail: "batch_write requires a new best-block".into(),
            });
        };

        let old_tip = match self
            .coins_get(&encode_best_block_key())
            .map_err(map_storage)?
        {
            Some(bytes) => decode_best_block_value(&bytes).map_err(map_storage)?,
            None => BlockHash::from_byte_array([0_u8; 32]),
        };

        let best_key = encode_best_block_key();
        let heads_key = encode_head_blocks_key();
        let heads_value = encode_head_blocks_value(&[new_tip, old_tip]).map_err(map_storage)?;
        let best_value = encode_best_block_value(new_tip);

        let mut batch = self.db.batch();
        let mut accumulated = 0_usize;

        batch.remove(&self.coins, best_key.as_slice());
        accumulated += estimated_encoded_bytes(best_key.as_slice(), None);
        batch.insert(&self.coins, heads_key.as_slice(), heads_value.as_slice());
        accumulated += estimated_encoded_bytes(heads_key.as_slice(), Some(heads_value.as_slice()));

        for (outpoint, entry) in writes.entries {
            if !entry.is_dirty() {
                continue;
            }

            let key = encode_coin_key(&outpoint);
            let (maybe_value, next_bytes) = if entry.is_spent() {
                (None, estimated_encoded_bytes(key.as_slice(), None))
            } else {
                let Some(coin) = entry.maybe_coin() else {
                    return Err(map_storage(coins_corruption(
                        "dirty unspent coin is missing a value",
                    )));
                };
                let value = encode_coin_value(coin).map_err(map_storage)?;
                let next_bytes = estimated_encoded_bytes(key.as_slice(), Some(value.as_slice()));
                (Some(value), next_bytes)
            };

            if let Some(value) = maybe_value {
                batch.insert(&self.coins, key.as_slice(), value);
            } else {
                batch.remove(&self.coins, key.as_slice());
            }
            accumulated += next_bytes;

            if accumulated <= cap_bytes {
                continue;
            }

            commit_batch(batch, PersistMode::Buffered).map_err(map_storage)?;
            #[cfg(test)]
            {
                self.partial_buffered_commits
                    .set(self.partial_buffered_commits.get().saturating_add(1));
                if self.simulate_crash_after_partial {
                    return Err(map_storage(interrupted_write()));
                }
            }
            batch = self.db.batch();
            accumulated = 0;
        }

        batch.remove(&self.coins, heads_key.as_slice());
        accumulated += estimated_encoded_bytes(heads_key.as_slice(), None);
        batch.insert(&self.coins, best_key.as_slice(), best_value.as_slice());
        accumulated += estimated_encoded_bytes(best_key.as_slice(), Some(best_value.as_slice()));
        let _counted_final_batch_bytes = accumulated;
        commit_batch(batch, final_mode).map_err(map_storage)
    }
}

impl FjallNodeStore {
    pub fn coins_view(&self) -> FjallCoinsView {
        FjallCoinsView::from_store(self)
    }
}

impl CoinsView for FjallCoinsView {
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        let maybe_bytes = map_optional_read(self.coins_get(&encode_coin_key(outpoint)))?;
        let Some(bytes) = maybe_bytes else {
            return Ok(None);
        };
        decode_coin_value(&bytes).map(Some).map_err(map_storage)
    }

    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        self.coins_contains(&encode_coin_key(outpoint))
            .map_err(map_storage)
    }

    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        match self.classify_markers().map_err(map_storage)? {
            MarkerState::Empty | MarkerState::Interrupted { .. } => Ok(None),
            MarkerState::Consistent { best_block } => Ok(Some(best_block)),
        }
    }

    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        match self.classify_markers().map_err(map_storage)? {
            MarkerState::Empty | MarkerState::Consistent { .. } => Ok(Vec::new()),
            MarkerState::Interrupted { new, old } => Ok(vec![new, old]),
        }
    }

    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        self.batch_write_capped(
            writes,
            maybe_best_block,
            Self::persist_mode_for_final_best_block(),
            DEFAULT_COINS_DB_BATCH_BYTES,
            false,
        )
    }

    fn collect_unspent_hint(&self) -> HashMap<OutPoint, Coin> {
        let mut utxos = HashMap::new();
        for guard in self.coins.prefix([DB_COIN]) {
            let Ok((key_bytes, value_bytes)) = guard.into_inner() else {
                return HashMap::new();
            };
            let Ok(outpoint) = decode_coin_key(key_bytes.as_ref()) else {
                return HashMap::new();
            };
            let Ok(coin) = decode_coin_value(value_bytes.as_ref()) else {
                return HashMap::new();
            };
            utxos.insert(outpoint, coin);
        }
        utxos
    }
}

fn map_storage(error: StorageError) -> ChainstateError {
    match error {
        StorageError::InterruptedWrite { .. } => {
            ChainstateError::InterruptedWrite { heads: Vec::new() }
        }
        other => ChainstateError::CoinsStorage {
            detail: other.to_string(),
        },
    }
}

fn map_optional_read<T>(
    result: Result<Option<T>, StorageError>,
) -> Result<Option<T>, ChainstateError> {
    match result {
        Ok(maybe_value) => Ok(maybe_value),
        Err(error) => Err(map_storage(error)),
    }
}

fn commit_batch(batch: fjall::OwnedWriteBatch, mode: PersistMode) -> Result<(), StorageError> {
    let batch = match fjall_persist_mode(mode) {
        Some(fjall_mode) => batch.durability(Some(fjall_mode)),
        None => batch,
    };
    batch.commit().map_err(coins_backend_failure)
}

fn fjall_persist_mode(mode: PersistMode) -> Option<FjallPersistMode> {
    match mode {
        PersistMode::Buffered => None,
        PersistMode::Flush => Some(FjallPersistMode::Buffer),
        PersistMode::Sync => Some(FjallPersistMode::SyncAll),
    }
}

fn interrupted_write() -> StorageError {
    StorageError::InterruptedWrite {
        namespace: StorageNamespace::Coins,
        action: StorageRecoveryAction::Reindex,
    }
}

fn coins_corruption(detail: impl core::fmt::Display) -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::Coins,
        detail: detail.to_string(),
        action: StorageRecoveryAction::Repair,
    }
}

fn coins_backend_failure(error: fjall::Error) -> StorageError {
    let message = error.to_string();
    StorageError::BackendFailure {
        namespace: StorageNamespace::Coins,
        message,
        action: StorageRecoveryAction::for_backend_message(&error.to_string()),
    }
}
