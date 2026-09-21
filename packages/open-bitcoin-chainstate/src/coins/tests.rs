// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use std::collections::HashMap;

use open_bitcoin_primitives::{Amount, BlockHash, OutPoint, ScriptBuf, TransactionOutput, Txid};

mod algebra;
mod cache_lookup_and_view;
mod flush;
mod view_errors_size_and_admission;

use super::{
    CoinsBatch, CoinsCache, CoinsCacheEntry, CoinsCacheFlags, CoinsView,
    ESTIMATED_COIN_ENTRY_OVERHEAD_BYTES, MemoryCoinsView,
};
use crate::error::ChainstateError;
use crate::types::{BlockUndo, Coin};
use crate::{Chainstate, ChainstateSnapshot, MemoryBackedChainstate};

struct HintlessView;

impl CoinsView for HintlessView {
    fn get_coin(&self, _outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        Ok(None)
    }

    fn have_coin(&self, _outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        Ok(false)
    }

    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        Ok(None)
    }

    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        Ok(Vec::new())
    }

    fn batch_write(
        &mut self,
        _writes: CoinsBatch,
        _maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        Ok(())
    }
}

fn fixture_outpoint() -> OutPoint {
    OutPoint {
        txid: Txid::from_byte_array([1_u8; 32]),
        vout: 0,
    }
}

fn fixture_coin() -> Coin {
    Coin {
        output: TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        },
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 0,
    }
}

fn fixture_best_block() -> BlockHash {
    BlockHash::from_byte_array([2_u8; 32])
}

fn fixture_outpoint_two() -> OutPoint {
    OutPoint {
        txid: Txid::from_byte_array([3_u8; 32]),
        vout: 1,
    }
}

struct FailingCoinsView;

impl CoinsView for FailingCoinsView {
    fn get_coin(&self, _outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "injected-disk-failure".to_string(),
        })
    }

    fn have_coin(&self, _outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "injected-disk-failure".to_string(),
        })
    }

    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "injected-disk-failure".to_string(),
        })
    }

    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "injected-disk-failure".to_string(),
        })
    }

    fn batch_write(
        &mut self,
        _writes: CoinsBatch,
        _maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        Ok(())
    }
}
