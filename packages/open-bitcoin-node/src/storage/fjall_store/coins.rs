// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.h
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp

//! Schema 1→2 coins migrate, leftover-non-authority hydrate, and Fjall undo records.

use std::{collections::HashMap, str};

use open_bitcoin_core::{
    chainstate::{
        BlockUndo, ChainstateError, ChainstateSnapshot, Coin, CoinsBatch, CoinsCacheEntry,
        CoinsView, RecoveryDecision, decide_recovery,
    },
    primitives::{BlockHash, OutPoint},
};

use super::{
    FjallNodeStore, PersistMode, SCHEMA_VERSION_KEY, SNAPSHOT_KEY, SchemaVersion, StorageError,
    StorageNamespace, StorageRecoveryAction, backend_failure, corruption,
};
use crate::storage::coins_codec::{
    DB_COIN, decode_coin_key, decode_coin_value, encode_best_block_key, encode_coin_key,
    encode_coin_value, estimated_encoded_bytes,
};
use crate::storage::coins_view::FjallCoinsView;
use crate::storage::snapshot_codec::{
    HydratedChainMeta, decode_block_undo, decode_chain_meta, encode_block_undo, encode_chain_meta,
};

const UNDO_KEY_PREFIX: &str = "undo:";
const CHAIN_META_KEY: &str = "chain_meta";

impl FjallNodeStore {
    pub fn save_chain_meta(
        &self,
        active_chain: &[open_bitcoin_core::chainstate::ChainPosition],
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let meta_bytes = encode_chain_meta(active_chain, None)?;
        self.put_bytes(
            StorageNamespace::Chainstate,
            CHAIN_META_KEY,
            meta_bytes,
            mode,
        )
    }

    pub fn save_undo(
        &self,
        block_hash: BlockHash,
        undo: &BlockUndo,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let bytes = encode_block_undo(undo)?;
        self.put_bytes(
            StorageNamespace::Chainstate,
            &undo_key(block_hash),
            bytes,
            mode,
        )
    }

    pub fn load_undo(&self, block_hash: BlockHash) -> Result<Option<BlockUndo>, StorageError> {
        self.get_bytes(StorageNamespace::Chainstate, &undo_key(block_hash))?
            .map(|bytes| decode_block_undo(&bytes))
            .transpose()
    }

    pub fn hydrate_chainstate_for_open(&self) -> Result<Option<ChainstateSnapshot>, StorageError> {
        let view = FjallCoinsView::from_store(self);
        let heads = view.head_blocks().map_err(map_heads_error)?;
        if matches!(
            decide_recovery(heads.len()),
            RecoveryDecision::InterruptedTwoHeads
        ) {
            return Err(interrupted_coins_write());
        }

        let leftover_present = self.leftover_snapshot_present()?;
        let coins_empty = self.coins_keyspace_is_empty()?;
        if leftover_present && coins_empty {
            // Surface malformed leftover as chainstate decode corruption first.
            let _decoded_leftover = self.load_chainstate_snapshot()?;
            return Err(leftover_empty_coins_corruption());
        }
        if coins_empty {
            return Ok(None);
        }

        let utxos = self.scan_coin_records()?;
        let undo_by_block = self.load_all_undo_records()?;
        let (active_chain, maybe_confirmed_txid_counts) = self.load_hydrate_chain_meta()?;
        let mut snapshot = ChainstateSnapshot::new(active_chain, utxos, undo_by_block);
        snapshot.maybe_confirmed_txid_counts = maybe_confirmed_txid_counts;
        Ok(Some(snapshot))
    }

    pub(crate) fn ensure_schema_and_migrate_coins(&self) -> Result<(), StorageError> {
        let Some(bytes) = self.get_bytes(StorageNamespace::Schema, SCHEMA_VERSION_KEY)? else {
            return self.write_schema_version(SchemaVersion::CURRENT);
        };
        let actual = parse_schema_version(&bytes)?;
        match actual.get() {
            1 => self.ensure_schema_one(actual),
            2 => self.ensure_schema_two(),
            _ => Err(StorageError::schema_mismatch(
                SchemaVersion::CURRENT,
                actual,
            )),
        }
    }

    fn ensure_schema_one(&self, _actual: SchemaVersion) -> Result<(), StorageError> {
        let leftover_present = self.leftover_snapshot_present()?;
        let coins_empty = self.coins_keyspace_is_empty()?;
        if leftover_present && coins_empty {
            self.migrate_schema_1_coins()?;
            return self.write_schema_version(SchemaVersion::CURRENT);
        }
        if leftover_present && !coins_empty {
            let Some(leftover) = self.load_chainstate_snapshot_with_confirmation_migration()?
            else {
                return Err(mixed_schema_one_corruption());
            };
            if !self.leftover_matches_consistent_coins(&leftover)? {
                return Err(mixed_schema_one_corruption());
            }
            self.finish_schema_one_undo_and_chain_meta(&leftover)?;
        }
        self.write_schema_version(SchemaVersion::CURRENT)
    }

    fn ensure_schema_two(&self) -> Result<(), StorageError> {
        let heads = FjallCoinsView::from_store(self)
            .head_blocks()
            .map_err(map_heads_error)?;
        if matches!(
            decide_recovery(heads.len()),
            RecoveryDecision::InterruptedTwoHeads
        ) {
            return Ok(());
        }
        let leftover_present = self.leftover_snapshot_present()?;
        let coins_empty = self.coins_keyspace_is_empty()?;
        if leftover_present && coins_empty {
            return Err(leftover_empty_coins_corruption());
        }
        Ok(())
    }

    fn leftover_matches_consistent_coins(
        &self,
        leftover: &ChainstateSnapshot,
    ) -> Result<bool, StorageError> {
        let view = FjallCoinsView::from_store(self);
        match view.head_blocks() {
            Ok(_) => {}
            Err(error) => return Err(map_heads_error(error)),
        }
        let Some(best) = view.best_block().map_err(map_heads_error)? else {
            return Ok(false);
        };
        let Some(tip) = leftover.active_chain.last() else {
            return Ok(false);
        };
        if best != tip.block_hash {
            return Ok(false);
        }
        Ok(self.scan_coin_records()? == leftover.utxos)
    }

    fn finish_schema_one_undo_and_chain_meta(
        &self,
        leftover: &ChainstateSnapshot,
    ) -> Result<(), StorageError> {
        for (block_hash, undo) in &leftover.undo_by_block {
            self.save_undo(*block_hash, undo, PersistMode::Sync)?;
        }
        let meta_bytes = encode_chain_meta(
            &leftover.active_chain,
            leftover.maybe_confirmed_txid_counts.as_ref(),
        )?;
        self.put_bytes(
            StorageNamespace::Chainstate,
            CHAIN_META_KEY,
            meta_bytes,
            PersistMode::Sync,
        )
    }

    fn migrate_schema_1_coins(&self) -> Result<(), StorageError> {
        let Some(leftover) = self.load_chainstate_snapshot_with_confirmation_migration()? else {
            return Ok(());
        };

        if let Some(tip) = leftover.active_chain.last() {
            self.write_migrated_coins_with_tip(&leftover.utxos, tip.block_hash)?;
        } else {
            self.write_migrated_coins_without_tip(&leftover.utxos)?;
        }
        self.finish_schema_one_undo_and_chain_meta(&leftover)
    }

    /// Copies leftover snapshot UTXOs into coins so a schema-2 reopen can hydrate.
    /// Production persist still writes leftover only; Phase 142 owns write-site cutover.
    pub fn seed_coins_from_leftover_for_reopen(&self) -> Result<(), StorageError> {
        if !self.leftover_snapshot_present()? {
            return Ok(());
        }
        let Some(leftover) = self.load_chainstate_snapshot()? else {
            return Ok(());
        };
        self.seed_coins_from_snapshot(&leftover)
    }

    /// Replaces the coins keyspace from a snapshot so persist can write coins before leftover.
    pub fn seed_coins_from_snapshot(
        &self,
        snapshot: &ChainstateSnapshot,
    ) -> Result<(), StorageError> {
        if let Some(tip) = snapshot.active_chain.last() {
            self.write_migrated_coins_with_tip(&snapshot.utxos, tip.block_hash)?;
        } else if snapshot.utxos.is_empty() {
            self.write_migrated_coins_with_tip(
                &HashMap::new(),
                BlockHash::from_byte_array([0_u8; 32]),
            )?;
        } else {
            self.write_migrated_coins_without_tip(&snapshot.utxos)?;
        }
        for (block_hash, undo) in &snapshot.undo_by_block {
            self.save_undo(*block_hash, undo, PersistMode::Sync)?;
        }
        let meta_bytes = encode_chain_meta(
            &snapshot.active_chain,
            snapshot.maybe_confirmed_txid_counts.as_ref(),
        )?;
        self.put_bytes(
            StorageNamespace::Chainstate,
            CHAIN_META_KEY,
            meta_bytes,
            PersistMode::Sync,
        )
    }

    fn write_migrated_coins_with_tip(
        &self,
        utxos: &HashMap<OutPoint, Coin>,
        tip: BlockHash,
    ) -> Result<(), StorageError> {
        let mut view = FjallCoinsView::from_store(self);
        let mut entries = HashMap::new();
        for guard in self.coins.prefix([DB_COIN]) {
            let (key_bytes, _) = guard
                .into_inner()
                .map_err(|error| backend_failure(StorageNamespace::Coins, error))?;
            let outpoint = decode_coin_key(key_bytes.as_ref())?;
            if !utxos.contains_key(&outpoint) {
                entries.insert(outpoint, CoinsCacheEntry::spent_dirty());
            }
        }
        for (outpoint, coin) in utxos {
            entries.insert(
                outpoint.clone(),
                CoinsCacheEntry::unspent_dirty(coin.clone()),
            );
        }
        view.batch_write(CoinsBatch { entries }, Some(tip))
            .map_err(map_heads_error)
    }

    fn write_migrated_coins_without_tip(
        &self,
        utxos: &HashMap<OutPoint, Coin>,
    ) -> Result<(), StorageError> {
        let mut accumulated = 0_usize;
        for (outpoint, coin) in utxos {
            let key = encode_coin_key(outpoint);
            let value = encode_coin_value(coin)?;
            let next_bytes = estimated_encoded_bytes(key.as_slice(), Some(value.as_slice()));
            self.coins
                .insert(key.as_slice(), value)
                .map_err(|error| backend_failure(StorageNamespace::Coins, error))?;
            accumulated += next_bytes;
            if accumulated >= crate::storage::coins_codec::DEFAULT_COINS_DB_BATCH_BYTES {
                self.persist(StorageNamespace::Coins, PersistMode::Buffered)?;
                accumulated = 0;
            }
        }
        self.persist(StorageNamespace::Coins, PersistMode::Buffered)
    }

    fn leftover_snapshot_present(&self) -> Result<bool, StorageError> {
        Ok(self
            .get_bytes(StorageNamespace::Chainstate, SNAPSHOT_KEY)?
            .is_some())
    }

    fn coins_keyspace_is_empty(&self) -> Result<bool, StorageError> {
        if self
            .coins
            .contains_key(encode_best_block_key())
            .map_err(|error| backend_failure(StorageNamespace::Coins, error))?
        {
            return Ok(false);
        }
        Ok(self.coins.prefix([DB_COIN]).next().is_none())
    }

    fn scan_coin_records(&self) -> Result<HashMap<OutPoint, Coin>, StorageError> {
        let mut utxos = HashMap::new();
        for guard in self.coins.prefix([DB_COIN]) {
            let (key_bytes, value_bytes) = guard
                .into_inner()
                .map_err(|error| backend_failure(StorageNamespace::Coins, error))?;
            let outpoint = decode_coin_key(key_bytes.as_ref())?;
            let coin = decode_coin_value(value_bytes.as_ref())?;
            utxos.insert(outpoint, coin);
        }
        Ok(utxos)
    }

    pub(crate) fn load_all_undo_records(
        &self,
    ) -> Result<HashMap<BlockHash, BlockUndo>, StorageError> {
        let mut undo_by_block = HashMap::new();
        for guard in self.chainstate.prefix(UNDO_KEY_PREFIX) {
            let (key_bytes, value_bytes) = guard
                .into_inner()
                .map_err(|error| backend_failure(StorageNamespace::Chainstate, error))?;
            let key = str::from_utf8(key_bytes.as_ref())
                .map_err(|error| corruption(StorageNamespace::Chainstate, error))?;
            let block_hash = parse_undo_key(key)?;
            undo_by_block.insert(block_hash, decode_block_undo(value_bytes.as_ref())?);
        }
        Ok(undo_by_block)
    }

    pub(crate) fn load_chain_meta_for_open(&self) -> Result<HydratedChainMeta, StorageError> {
        if let Some(bytes) = self.get_bytes(StorageNamespace::Chainstate, CHAIN_META_KEY)? {
            return decode_chain_meta(&bytes);
        }
        Ok((Vec::new(), Some(HashMap::new())))
    }

    fn load_hydrate_chain_meta(&self) -> Result<HydratedChainMeta, StorageError> {
        if let Some(bytes) = self.get_bytes(StorageNamespace::Chainstate, CHAIN_META_KEY)? {
            return decode_chain_meta(&bytes);
        }
        let Some(leftover) = self.load_chainstate_snapshot()? else {
            return Ok((Vec::new(), None));
        };
        Ok((leftover.active_chain, leftover.maybe_confirmed_txid_counts))
    }

    fn write_schema_version(&self, version: SchemaVersion) -> Result<(), StorageError> {
        self.put_bytes(
            StorageNamespace::Schema,
            SCHEMA_VERSION_KEY,
            version.get().to_string().into_bytes(),
            PersistMode::Sync,
        )
    }
}

fn undo_key(block_hash: BlockHash) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut key = String::with_capacity(UNDO_KEY_PREFIX.len() + 64);
    key.push_str(UNDO_KEY_PREFIX);
    for byte in block_hash.as_bytes() {
        key.push(HEX[(byte >> 4) as usize] as char);
        key.push(HEX[(byte & 0x0f) as usize] as char);
    }
    key
}

fn parse_undo_key(key: &str) -> Result<BlockHash, StorageError> {
    let Some(hex) = key.strip_prefix(UNDO_KEY_PREFIX) else {
        return Err(corruption(
            StorageNamespace::Chainstate,
            "undo key missing undo: prefix",
        ));
    };
    if hex.len() != 64 {
        return Err(corruption(
            StorageNamespace::Chainstate,
            "undo key must be undo: plus 64 hex chars",
        ));
    }
    let mut bytes = [0_u8; 32];
    for (index, chunk) in hex.as_bytes().chunks(2).enumerate() {
        let digit = str::from_utf8(chunk)
            .map_err(|error| corruption(StorageNamespace::Chainstate, error))?;
        let parsed = u8::from_str_radix(digit, 16)
            .map_err(|error| corruption(StorageNamespace::Chainstate, error))?;
        let Some(slot) = bytes.get_mut(index) else {
            return Err(corruption(
                StorageNamespace::Chainstate,
                "undo key hex overflow",
            ));
        };
        *slot = parsed;
    }
    Ok(BlockHash::from_byte_array(bytes))
}

fn parse_schema_version(bytes: &[u8]) -> Result<SchemaVersion, StorageError> {
    let text =
        str::from_utf8(bytes).map_err(|error| corruption(StorageNamespace::Schema, error))?;
    let version = text
        .parse::<u32>()
        .map_err(|error| corruption(StorageNamespace::Schema, error))?;
    SchemaVersion::new(version)
}

fn map_heads_error(error: ChainstateError) -> StorageError {
    match error {
        ChainstateError::InterruptedWrite { .. } => interrupted_coins_write(),
        ChainstateError::CoinsStorage { detail } => StorageError::Corruption {
            namespace: StorageNamespace::Coins,
            detail,
            action: StorageRecoveryAction::Repair,
        },
        other => StorageError::Corruption {
            namespace: StorageNamespace::Coins,
            detail: other.to_string(),
            action: StorageRecoveryAction::Repair,
        },
    }
}

fn interrupted_coins_write() -> StorageError {
    StorageError::InterruptedWrite {
        namespace: StorageNamespace::Coins,
        action: StorageRecoveryAction::Reindex,
    }
}

fn leftover_empty_coins_corruption() -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::Coins,
        detail: "schema 2 leftover snapshot with empty coins must not remigrate".to_string(),
        action: StorageRecoveryAction::RestoreFromBackup,
    }
}

fn mixed_schema_one_corruption() -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::Coins,
        detail: "schema 1 leftover snapshot mixed with existing coins".to_string(),
        action: StorageRecoveryAction::RestoreFromBackup,
    }
}
