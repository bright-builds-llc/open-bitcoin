// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::collections::HashMap;

use open_bitcoin_primitives::{BlockHash, OutPoint};

use super::{CoinsBatch, CoinsCacheEntry, CoinsView};
use crate::error::ChainstateError;
use crate::types::Coin;

pub struct CoinsCache<V: CoinsView> {
    parent: V,
    overlay: HashMap<OutPoint, CoinsCacheEntry>,
    maybe_best_block: Option<BlockHash>,
}

impl<V: CoinsView> CoinsCache<V> {
    pub fn from_parent(parent: V) -> Self {
        let maybe_best_block = parent.best_block();
        Self {
            parent,
            overlay: HashMap::new(),
            maybe_best_block,
        }
    }

    pub fn contains_in_cache(&self, outpoint: &OutPoint) -> bool {
        self.overlay.contains_key(outpoint)
    }

    pub fn have_coin_in_cache(&self, outpoint: &OutPoint) -> bool {
        self.overlay
            .get(outpoint)
            .is_some_and(|entry| entry.maybe_coin().is_some())
    }

    pub fn fetch_coin(
        &mut self,
        outpoint: &OutPoint,
    ) -> Result<Option<&CoinsCacheEntry>, ChainstateError> {
        if self.overlay.contains_key(outpoint) {
            return Ok(self.overlay.get(outpoint));
        }

        let Some(coin) = self.parent.get_coin(outpoint)? else {
            return Ok(None);
        };

        self.overlay
            .insert(outpoint.clone(), CoinsCacheEntry::unspent_clean(coin));
        Ok(self.overlay.get(outpoint))
    }

    pub fn parent(&self) -> &V {
        &self.parent
    }

    #[cfg(test)]
    pub(crate) fn insert_entry_for_test(&mut self, outpoint: OutPoint, entry: CoinsCacheEntry) {
        self.overlay.insert(outpoint, entry);
    }
}

impl<V: CoinsView> CoinsView for CoinsCache<V> {
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        if let Some(entry) = self.overlay.get(outpoint) {
            return Ok(entry.maybe_coin().cloned());
        }
        self.parent.get_coin(outpoint)
    }

    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        if let Some(entry) = self.overlay.get(outpoint) {
            return Ok(entry.maybe_coin().is_some());
        }
        self.parent.have_coin(outpoint)
    }

    fn best_block(&self) -> Option<BlockHash> {
        self.maybe_best_block.or_else(|| self.parent.best_block())
    }

    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        for (outpoint, entry) in writes.entries {
            self.overlay.insert(outpoint, entry);
        }
        if let Some(best_block) = maybe_best_block {
            self.maybe_best_block = Some(best_block);
        }
        Ok(())
    }
}
