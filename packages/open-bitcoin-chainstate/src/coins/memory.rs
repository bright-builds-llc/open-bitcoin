// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::collections::HashMap;

use open_bitcoin_primitives::{BlockHash, OutPoint};

use super::{CoinsBatch, CoinsView};
use crate::error::ChainstateError;
use crate::types::Coin;

pub struct MemoryCoinsView {
    coins: HashMap<OutPoint, Coin>,
    maybe_best_block: Option<BlockHash>,
}

impl MemoryCoinsView {
    pub fn from_coins(coins: HashMap<OutPoint, Coin>, maybe_best_block: Option<BlockHash>) -> Self {
        Self {
            coins,
            maybe_best_block,
        }
    }

    pub fn contains_outpoint(&self, outpoint: &OutPoint) -> bool {
        self.coins.contains_key(outpoint)
    }

    pub fn unspent_coins(&self) -> HashMap<OutPoint, Coin> {
        self.coins.clone()
    }
}

impl CoinsView for MemoryCoinsView {
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        Ok(self.coins.get(outpoint).cloned())
    }

    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        Ok(self.coins.contains_key(outpoint))
    }

    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        Ok(self.maybe_best_block)
    }

    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        Ok(Vec::new())
    }

    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        for (outpoint, entry) in writes.entries {
            match entry.maybe_coin() {
                Some(coin) => {
                    self.coins.insert(outpoint, coin.clone());
                }
                None => {
                    self.coins.remove(&outpoint);
                }
            }
        }
        if let Some(best_block) = maybe_best_block {
            self.maybe_best_block = Some(best_block);
        }
        Ok(())
    }
}
