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

pub struct CoinsOverlay {
    entries: HashMap<OutPoint, CoinsCacheEntry>,
    maybe_best_block: Option<BlockHash>,
}

impl CoinsOverlay {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            maybe_best_block: None,
        }
    }

    pub fn peek_coin<V: CoinsView>(
        &self,
        parent: &V,
        outpoint: &OutPoint,
    ) -> Result<Option<Coin>, ChainstateError> {
        if let Some(entry) = self.entries.get(outpoint) {
            return Ok(entry.maybe_coin().cloned());
        }
        parent.get_coin(outpoint)
    }

    pub fn add_coin<V: CoinsView>(
        &mut self,
        parent: &V,
        outpoint: OutPoint,
        coin: Coin,
        possible_overwrite: bool,
    ) -> Result<(), ChainstateError> {
        add_coin_into(
            self,
            |peeked| parent.get_coin(peeked),
            outpoint,
            coin,
            possible_overwrite,
        )
    }

    pub fn spend_coin<V: CoinsView>(
        &mut self,
        parent: &V,
        outpoint: &OutPoint,
    ) -> Result<Coin, ChainstateError> {
        spend_coin_into(self, |peeked| parent.get_coin(peeked), outpoint)
    }

    pub fn into_dirty_batch(self) -> CoinsBatch {
        CoinsBatch {
            entries: self
                .entries
                .into_iter()
                .filter(|(_, entry)| entry.is_dirty())
                .collect(),
        }
    }

    pub fn set_best_block(&mut self, block_hash: BlockHash) {
        self.maybe_best_block = Some(block_hash);
    }

    fn get(&self, outpoint: &OutPoint) -> Option<&CoinsCacheEntry> {
        self.entries.get(outpoint)
    }

    fn insert(&mut self, outpoint: OutPoint, entry: CoinsCacheEntry) {
        self.entries.insert(outpoint, entry);
    }

    fn contains(&self, outpoint: &OutPoint) -> bool {
        self.entries.contains_key(outpoint)
    }

    fn remove(&mut self, outpoint: &OutPoint) {
        self.entries.remove(outpoint);
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    fn dirty_batch(&self) -> CoinsBatch {
        CoinsBatch {
            entries: self
                .entries
                .iter()
                .filter(|(_, entry)| entry.is_dirty())
                .map(|(outpoint, entry)| (outpoint.clone(), entry.clone()))
                .collect(),
        }
    }
}

impl Default for CoinsOverlay {
    fn default() -> Self {
        Self::new()
    }
}

fn add_coin_into(
    overlay: &mut CoinsOverlay,
    peek_beneath: impl Fn(&OutPoint) -> Result<Option<Coin>, ChainstateError>,
    outpoint: OutPoint,
    coin: Coin,
    possible_overwrite: bool,
) -> Result<(), ChainstateError> {
    let fresh = if let Some(existing) = overlay.get(&outpoint) {
        if existing.maybe_coin().is_some() {
            if !possible_overwrite {
                return Err(ChainstateError::OutputOverwrite { outpoint });
            }
            false
        } else {
            !existing.is_dirty()
        }
    } else {
        let maybe_parent_coin = peek_beneath(&outpoint)?;
        if maybe_parent_coin.is_some() && !possible_overwrite {
            return Err(ChainstateError::OutputOverwrite { outpoint });
        }
        maybe_parent_coin.is_none()
    };

    let entry = if fresh {
        CoinsCacheEntry::unspent_fresh_dirty(coin)
    } else {
        CoinsCacheEntry::unspent_dirty(coin)
    };
    overlay.insert(outpoint, entry);
    Ok(())
}

fn spend_coin_into(
    overlay: &mut CoinsOverlay,
    peek_beneath: impl Fn(&OutPoint) -> Result<Option<Coin>, ChainstateError>,
    outpoint: &OutPoint,
) -> Result<Coin, ChainstateError> {
    if let Some(existing) = overlay.get(outpoint) {
        let Some(coin) = existing.maybe_coin().cloned() else {
            return Err(ChainstateError::MissingCoin {
                outpoint: outpoint.clone(),
            });
        };
        let erase_fresh = existing.is_fresh();
        if erase_fresh {
            overlay.remove(outpoint);
        } else {
            overlay.insert(outpoint.clone(), CoinsCacheEntry::spent_dirty());
        }
        return Ok(coin);
    }

    let Some(coin) = peek_beneath(outpoint)? else {
        return Err(ChainstateError::MissingCoin {
            outpoint: outpoint.clone(),
        });
    };
    overlay.insert(
        outpoint.clone(),
        CoinsCacheEntry::unspent_clean(coin.clone()),
    );
    overlay.insert(outpoint.clone(), CoinsCacheEntry::spent_dirty());
    Ok(coin)
}

fn apply_child_write(
    overlay: &mut CoinsOverlay,
    outpoint: OutPoint,
    child_entry: CoinsCacheEntry,
) -> Result<(), ChainstateError> {
    if child_entry.is_fresh() && child_entry.is_spent() && !overlay.contains(&outpoint) {
        return Ok(());
    }
    if !child_entry.is_dirty() {
        return Ok(());
    }

    let Some(parent_entry) = overlay.get(&outpoint) else {
        overlay.insert(outpoint, dirty_copying_fresh(child_entry));
        return Ok(());
    };
    let parent_is_spent = parent_entry.is_spent();
    let parent_is_fresh = parent_entry.is_fresh();

    if child_entry.is_fresh() && !parent_is_spent {
        return Err(ChainstateError::FreshFlagMisapplied { outpoint });
    }
    if parent_is_fresh && child_entry.is_spent() {
        overlay.remove(&outpoint);
        return Ok(());
    }

    overlay.insert(outpoint, overwrite_dirty_without_fresh(child_entry));
    Ok(())
}

fn dirty_copying_fresh(child_entry: CoinsCacheEntry) -> CoinsCacheEntry {
    match (child_entry.maybe_coin().cloned(), child_entry.is_fresh()) {
        (Some(coin), true) => CoinsCacheEntry::unspent_fresh_dirty(coin),
        (Some(coin), false) => CoinsCacheEntry::unspent_dirty(coin),
        (None, _) => CoinsCacheEntry::spent_dirty(),
    }
}

fn overwrite_dirty_without_fresh(child_entry: CoinsCacheEntry) -> CoinsCacheEntry {
    match child_entry.maybe_coin().cloned() {
        Some(coin) => CoinsCacheEntry::unspent_dirty(coin),
        None => CoinsCacheEntry::spent_dirty(),
    }
}

pub struct CoinsCache<V: CoinsView> {
    parent: V,
    overlay: CoinsOverlay,
}

impl<V: CoinsView> CoinsCache<V> {
    pub fn from_parent(parent: V) -> Self {
        let maybe_best_block = parent.best_block();
        Self {
            parent,
            overlay: CoinsOverlay {
                entries: HashMap::new(),
                maybe_best_block,
            },
        }
    }

    pub fn contains_in_cache(&self, outpoint: &OutPoint) -> bool {
        self.overlay.contains(outpoint)
    }

    pub fn have_coin_in_cache(&self, outpoint: &OutPoint) -> bool {
        self.overlay
            .get(outpoint)
            .is_some_and(|entry| entry.maybe_coin().is_some())
    }

    pub fn cached_entry(&self, outpoint: &OutPoint) -> Option<&CoinsCacheEntry> {
        self.overlay.get(outpoint)
    }

    pub fn fetch_coin(
        &mut self,
        outpoint: &OutPoint,
    ) -> Result<Option<&CoinsCacheEntry>, ChainstateError> {
        if self.overlay.contains(outpoint) {
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

    pub fn add_coin(
        &mut self,
        outpoint: OutPoint,
        coin: Coin,
        possible_overwrite: bool,
    ) -> Result<(), ChainstateError> {
        add_coin_into(
            &mut self.overlay,
            |peeked| CoinsView::get_coin(&self.parent, peeked),
            outpoint,
            coin,
            possible_overwrite,
        )
    }

    pub fn spend_coin(&mut self, outpoint: &OutPoint) -> Result<Coin, ChainstateError> {
        spend_coin_into(
            &mut self.overlay,
            |peeked| CoinsView::get_coin(&self.parent, peeked),
            outpoint,
        )
    }

    pub fn flush(&mut self) -> Result<(), ChainstateError> {
        let batch = self.overlay.dirty_batch();
        let maybe_best_block = self.overlay.maybe_best_block;
        self.parent.batch_write(batch, maybe_best_block)?;
        self.overlay.clear();
        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), ChainstateError> {
        let batch = self.overlay.dirty_batch();
        let maybe_best_block = self.overlay.maybe_best_block;
        self.parent.batch_write(batch, maybe_best_block)?;
        self.overlay.entries.retain(|_, entry| {
            let Some(coin) = entry.maybe_coin().cloned() else {
                return false;
            };
            *entry = CoinsCacheEntry::unspent_clean(coin);
            true
        });
        Ok(())
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
        self.overlay
            .maybe_best_block
            .or_else(|| self.parent.best_block())
    }

    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        for (outpoint, child_entry) in writes.entries {
            apply_child_write(&mut self.overlay, outpoint, child_entry)?;
        }
        if let Some(best_block) = maybe_best_block {
            self.overlay.maybe_best_block = Some(best_block);
        }
        Ok(())
    }
}
