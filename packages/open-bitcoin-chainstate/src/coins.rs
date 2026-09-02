// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

mod cache;
mod flush;
mod memory;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

use open_bitcoin_primitives::{BlockHash, OutPoint};

use crate::error::ChainstateError;
use crate::types::Coin;

pub use cache::{CoinsCache, CoinsOverlay};
pub use flush::{
    COIN_WRITE_GUARD_BYTES_PER_ENTRY, CoinsCacheSizeState, FlushDecision, FlushDecisionFacts,
    FlushMode, FlushPolicyInput, FlushPolicyTime, LARGE_CACHE_DENOMINATOR,
    LARGE_CACHE_HEADROOM_BYTES, LARGE_CACHE_NUMERATOR, LastFlushReason, decide_flush,
};
pub use memory::MemoryCoinsView;

pub trait CoinsView {
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError>;
    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError>;
    fn best_block(&self) -> Option<BlockHash>;
    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError>;
}

pub struct CoinsBatch {
    pub entries: HashMap<OutPoint, CoinsCacheEntry>,
}

/// Knots SanityCheck valid attr masks (DIRTY=1, FRESH=2, spent=4):
/// 0 unspent clean, 1 unspent DIRTY, 3 unspent FRESH|DIRTY, 5 spent DIRTY,
/// 6 spent FRESH-not-DIRTY. Invalid: 2 FRESH-only unspent, 4 spent clean,
/// 7 spent FRESH|DIRTY.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoinsCacheFlags {
    UnspentClean,
    UnspentDirty,
    UnspentFreshDirty,
    SpentDirty,
    SpentFresh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoinsCacheEntry {
    maybe_coin: Option<Coin>,
    flags: CoinsCacheFlags,
}

impl CoinsCacheEntry {
    pub fn unspent_clean(coin: Coin) -> Self {
        Self {
            maybe_coin: Some(coin),
            flags: CoinsCacheFlags::UnspentClean,
        }
    }

    pub fn unspent_dirty(coin: Coin) -> Self {
        Self {
            maybe_coin: Some(coin),
            flags: CoinsCacheFlags::UnspentDirty,
        }
    }

    pub fn unspent_fresh_dirty(coin: Coin) -> Self {
        Self {
            maybe_coin: Some(coin),
            flags: CoinsCacheFlags::UnspentFreshDirty,
        }
    }

    pub fn spent_dirty() -> Self {
        Self {
            maybe_coin: None,
            flags: CoinsCacheFlags::SpentDirty,
        }
    }

    pub fn spent_fresh() -> Self {
        Self {
            maybe_coin: None,
            flags: CoinsCacheFlags::SpentFresh,
        }
    }

    pub fn try_from_flags(
        maybe_coin: Option<Coin>,
        dirty: bool,
        fresh: bool,
    ) -> Result<Self, ChainstateError> {
        match (dirty, fresh, maybe_coin) {
            (false, false, Some(coin)) => Ok(Self::unspent_clean(coin)),
            (true, false, Some(coin)) => Ok(Self::unspent_dirty(coin)),
            (true, true, Some(coin)) => Ok(Self::unspent_fresh_dirty(coin)),
            (true, false, None) => Ok(Self::spent_dirty()),
            (false, true, None) => Ok(Self::spent_fresh()),
            (dirty, fresh, maybe_coin) => Err(ChainstateError::InvalidCacheEntry {
                dirty,
                fresh,
                spent: maybe_coin.is_none(),
            }),
        }
    }

    pub fn maybe_coin(&self) -> Option<&Coin> {
        self.maybe_coin.as_ref()
    }

    pub fn flags(&self) -> CoinsCacheFlags {
        self.flags
    }

    pub const fn is_dirty(&self) -> bool {
        matches!(
            self.flags,
            CoinsCacheFlags::UnspentDirty
                | CoinsCacheFlags::UnspentFreshDirty
                | CoinsCacheFlags::SpentDirty
        )
    }

    pub const fn is_fresh(&self) -> bool {
        matches!(
            self.flags,
            CoinsCacheFlags::UnspentFreshDirty | CoinsCacheFlags::SpentFresh
        )
    }

    pub fn is_spent(&self) -> bool {
        self.maybe_coin.is_none()
    }
}
