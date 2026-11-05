// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

//! Detached row observations confer no work, acceptance or publication authority.

use super::{Namespaces, inventory, source_digest};
use crate::storage::fjall_store::inspection::{InspectedRecords, require_existing_namespaces};
use crate::storage::fjall_store::{
    block_key,
    bounded_rows::{self, RowFamily},
    tracked::MutationStamp,
};
use crate::storage::validation_history::history_corruption;
use crate::storage::{coins_codec, snapshot_codec, validation_history as history};
use crate::{FjallNodeStore, StorageError};
use open_bitcoin_core::{chainstate::ChainstateSnapshot, primitives::Block};
use open_bitcoin_network::HeaderEntry;

type AchievedCheck<'a> = Box<dyn FnOnce(&FjallNodeStore) -> Result<(), StorageError> + 'a>;

fn validate_rows(rows: &Namespaces) -> Result<(), StorageError> {
    crate::storage::fjall_store::inspection::require_existing_namespace_catalog(
        rows.keys().map(String::as_str),
    )?;
    InspectedRecords::check_submission_budget(rows)?;
    inventory(rows).map_err(|_| history_corruption("unsupported native inventory"))
}

/// Same-store observation; fields and construction are private.
pub(crate) struct SubmissionInventory<'a> {
    store: &'a FjallNodeStore,
    catalog: Vec<String>,
    stamp: MutationStamp,
    rows: Namespaces,
    source: [u8; 32],
}

fn catalog(store: &FjallNodeStore) -> Vec<String> {
    let mut catalog: Vec<_> = store
        .database()
        .list_keyspace_names()
        .into_iter()
        .map(|name| name.to_string())
        .collect();
    catalog.sort();
    catalog
}

impl FjallNodeStore {
    /// Borrow one bounded actual inventory; this never opens or repairs a database.
    pub(crate) fn inspect_submission_inventory(
        &self,
    ) -> Result<SubmissionInventory<'_>, StorageError> {
        let stamp = self
            .database()
            .stamp()
            .map_err(|_| history_corruption("native inventory observation unavailable"))?;
        let before_catalog = catalog(self);
        let records = InspectedRecords::read_existing(self.database())?;
        require_existing_namespaces(&records)?;
        let rows = records
            .namespaces()
            .map(|name| {
                Ok((
                    name.to_owned(),
                    records
                        .maybe_records(name)
                        .ok_or_else(|| history_corruption("missing inventory namespace"))?
                        .clone(),
                ))
            })
            .collect::<Result<Namespaces, StorageError>>()?;
        validate_rows(&rows)?;
        #[cfg(test)]
        tests::during_capture(self);
        let source = source_digest(&rows);
        if catalog(self) != before_catalog {
            return Err(history_corruption(
                "native inventory changed during capture",
            ));
        }
        self.database()
            .check_stamp(&stamp)
            .map_err(|_| history_corruption("native inventory changed during capture"))?;
        Ok(SubmissionInventory {
            store: self,
            catalog: before_catalog,
            stamp,
            rows,
            source,
        })
    }
}

impl<'a> SubmissionInventory<'a> {
    /// Consume only actual captured rows, with the original logical source still current.
    pub(crate) fn into_observed_rows(self) -> Result<Namespaces, StorageError> {
        if source_digest(&self.rows) != self.source {
            return Err(history_corruption(
                "prospective rows are not the observed native source",
            ));
        }
        self.check_current_source()?;
        Ok(self.rows)
    }

    fn namespace_mut(&mut self, name: &str) -> Result<&mut super::Rows, StorageError> {
        self.rows
            .get_mut(name)
            .ok_or_else(|| history_corruption("missing prospective namespace"))
    }

    fn header_rows(&mut self, headers: &[HeaderEntry]) -> Result<(), StorageError> {
        bounded_rows::project_replacement(
            self.namespace_mut("headers")?,
            RowFamily::Headers,
            snapshot_codec::encode_header_entries(headers)?,
        )?;
        bounded_rows::project_replacement(
            self.namespace_mut("block_index")?,
            RowFamily::BlockIndex,
            snapshot_codec::encode_block_index_entries(headers)?,
        )
    }

    fn validation_row(
        &mut self,
        identity: history::BlockValidationIdentity,
        status: history::StoredValidationStatus,
    ) -> Result<(), StorageError> {
        let rows = self.namespace_mut("block_index")?;
        let coverage = rows
            .get(history::COVERAGE_KEY.as_bytes())
            .ok_or_else(|| history_corruption("missing prospective coverage"))?;
        if history::decode_coverage(coverage)? != history::ValidationCoverage::Complete {
            return Err(history_corruption(
                "unproved prospective validation coverage",
            ));
        }
        let key = history::record_key(identity.hash());
        let maybe_old = rows
            .get(key.as_bytes())
            .map(|bytes| history::decode_record(&key, bytes))
            .transpose()?;
        if status == history::StoredValidationStatus::KnownOnly
            && maybe_old.is_some_and(|record| {
                record.status() == history::StoredValidationStatus::ScriptsValid
            })
        {
            if maybe_old.is_some_and(|record| record.identity() != identity) {
                return Err(history_corruption(
                    "conflicting prospective accepted identity",
                ));
            }
            return Ok(());
        }
        let next = history::ValidationHistoryRecord::new(identity, status);
        history::validate_upgrade(maybe_old, next)?;
        rows.insert(key.into_bytes(), history::encode_record(next));
        rows.insert(
            history::COVERAGE_KEY.as_bytes().to_vec(),
            history::encode_coverage(history::ValidationCoverage::Complete).to_vec(),
        );
        Ok(())
    }

    /// Forecast only the actual earned header and its permitted KnownOnly publication.
    pub(crate) fn project_admitted_headers(
        &mut self,
        headers: &[HeaderEntry],
        identity: history::BlockValidationIdentity,
    ) -> Result<(), StorageError> {
        self.header_rows(headers)?;
        self.validation_row(identity, history::StoredValidationStatus::KnownOnly)?;
        validate_rows(&self.rows)
    }

    /// Encode complete genuine staged state; this neither absorbs nor publishes it.
    pub(crate) fn project_prepared_connect(
        &mut self,
        block: &Block,
        snapshot: &ChainstateSnapshot,
        headers: &[HeaderEntry],
    ) -> Result<(), StorageError> {
        let position = snapshot
            .tip()
            .ok_or_else(|| history_corruption("missing prospective accepted tip"))?;
        if position.header != block.header || snapshot.maybe_confirmed_txid_counts.is_none() {
            return Err(history_corruption("prospective block/state mismatch"));
        }
        self.header_rows(headers)?;
        let coins = self.namespace_mut("coins")?;
        if coins.contains_key(coins_codec::encode_head_blocks_key().as_slice()) {
            return Err(history_corruption("pending prospective Coins write"));
        }
        coins.retain(|key, _| !key.starts_with(&[coins_codec::DB_COIN]));
        for (outpoint, coin) in &snapshot.utxos {
            coins.insert(
                coins_codec::encode_coin_key(outpoint).to_vec(),
                coins_codec::encode_coin_value(coin)?,
            );
        }
        coins.insert(
            coins_codec::encode_best_block_key().to_vec(),
            coins_codec::encode_best_block_value(position.block_hash),
        );
        coins.remove(coins_codec::encode_head_blocks_key().as_slice());
        bounded_rows::project_replacement(
            self.namespace_mut("chainstate")?,
            RowFamily::ChainMeta,
            snapshot_codec::encode_chain_meta(
                &snapshot.active_chain,
                snapshot.maybe_confirmed_txid_counts.as_ref(),
            )?,
        )?;
        for (hash, undo) in &snapshot.undo_by_block {
            let (key, bytes) = FjallNodeStore::submission_undo_row(*hash, undo)?;
            self.namespace_mut("chainstate")?
                .insert(key.into_bytes(), bytes);
        }
        self.namespace_mut("block_index")?.insert(
            block_key(position.block_hash).into_bytes(),
            open_bitcoin_core::codec::encode_block(block)
                .map_err(|error| history_corruption(error.to_string()))?,
        );
        self.validation_row(
            history::BlockValidationIdentity::new(
                position.block_hash,
                position.previous_block_hash(),
                position.height,
            )?,
            history::StoredValidationStatus::ScriptsValid,
        )?;
        validate_rows(&self.rows)
    }

    /// Add exact BASIC bytes without obtaining a future append proof or fence.
    pub(crate) fn project_basic(
        &mut self,
        position: &open_bitcoin_core::chainstate::ChainPosition,
        maybe_inputs: Option<&open_bitcoin_core::chainstate::BasicFilterInputs<'_>>,
        maybe_progress: Option<
            open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexProgress,
        >,
    ) -> Result<(), StorageError> {
        let rows = self
            .store
            .submission_basic_rows(position, maybe_inputs, maybe_progress)?;
        self.namespace_mut("block_index")?.extend(
            rows.into_iter()
                .map(|(key, bytes)| (key.into_bytes(), bytes)),
        );
        validate_rows(&self.rows)
    }

    /// Validate the current source tuple immediately before consumer effects.
    pub(crate) fn check_current_source(&self) -> Result<(), StorageError> {
        self.store
            .database()
            .check_stamp(&self.stamp)
            .map_err(|_| history_corruption("stale native inventory source"))?;
        if catalog(self.store) != self.catalog {
            return Err(history_corruption("stale native inventory source"));
        }
        let current = self.store.inspect_submission_inventory()?;
        if catalog(self.store) != self.catalog {
            return Err(history_corruption("stale native inventory source"));
        }
        if current.source_digest() != self.source_digest() {
            return Err(history_corruption("changed native inventory bytes"));
        }
        self.store
            .database()
            .check_stamp(&self.stamp)
            .map_err(|_| history_corruption("stale native inventory source"))?;
        Ok(())
    }

    /// Inspect a data digest, never an acceptance seal.
    pub(crate) fn source_digest(&self) -> [u8; 32] {
        self.source
    }

    /// Compare a fresh achieved capture after expected own writes.
    pub(crate) fn compare_achieved(&self, store: &FjallNodeStore) -> Result<(), StorageError> {
        if !self.store.shares_validation_history_store(store) {
            return Err(history_corruption("foreign achieved inventory store"));
        }
        let achieved = store.inspect_submission_inventory()?;
        if achieved.rows != self.rows {
            return Err(history_corruption(
                "projected and achieved native rows differ",
            ));
        }
        Ok(())
    }

    /// Carry only detached expected bytes and the original store borrow through effects.
    pub(crate) fn into_achieved_check(self) -> AchievedCheck<'a> {
        Box::new(move |store| self.compare_achieved(store))
    }
}

#[cfg(test)]
mod tests;
