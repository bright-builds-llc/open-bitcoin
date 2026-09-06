// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::{
    chainstate::{
        ChainstateError, Coin, CoinsBatch, CoinsCache, CoinsView, FlushMode, FlushPolicyTime,
    },
    primitives::{Amount, BlockHash, OutPoint, ScriptBuf, TransactionOutput, Txid},
};

use crate::chainstate::flush_lifecycle::{
    FlushLifecycle, FlushPersistSink, default_coins_cache_byte_limit,
};
use crate::storage::{StorageError, StorageNamespace, StorageRecoveryAction};

struct FailingCoinsView {
    error: ChainstateError,
}

impl CoinsView for FailingCoinsView {
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
        Err(self.error.clone())
    }
}

struct SucceedingSink;

impl FlushPersistSink for SucceedingSink {
    fn persist_block(
        &mut self,
        _block: &open_bitcoin_core::primitives::Block,
    ) -> Result<(), StorageError> {
        Ok(())
    }

    fn persist_undo(
        &mut self,
        _hash: BlockHash,
        _undo: &open_bitcoin_core::chainstate::BlockUndo,
    ) -> Result<(), StorageError> {
        Ok(())
    }

    fn persist_header_entries(
        &mut self,
        _entries: &[open_bitcoin_network::HeaderEntry],
    ) -> Result<(), StorageError> {
        Ok(())
    }
}

fn sample_coin() -> Coin {
    Coin {
        output: TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        },
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 1,
    }
}

fn expect_error<T>(result: Result<T, StorageError>, message: &str) -> StorageError {
    match result {
        Ok(_) => panic!("{message}: expected error"),
        Err(error) => error,
    }
}

#[test]
fn execute_flush_maps_coins_write_errors() {
    // Arrange
    let cases = [
        ChainstateError::InterruptedWrite { heads: Vec::new() },
        ChainstateError::CoinsStorage {
            detail: "coins backend".to_string(),
        },
        ChainstateError::MissingTip,
    ];

    for error in cases {
        let mut lifecycle = FlushLifecycle::ready_for_test(
            default_coins_cache_byte_limit(),
            0,
            FlushPolicyTime::from_unix_seconds(1),
            false,
        );
        let mut cache = CoinsCache::from_parent(FailingCoinsView {
            error: error.clone(),
        });
        cache
            .add_coin(
                OutPoint {
                    txid: Txid::from_byte_array([0x33; 32]),
                    vout: 0,
                },
                sample_coin(),
                true,
            )
            .expect("dirty overlay");

        // Act
        let mapped = expect_error(
            lifecycle.execute_flush(
                &mut SucceedingSink,
                &mut cache,
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(1),
                u64::MAX,
                &[],
                &[],
                &[],
            ),
            "mapped coins write",
        );

        // Assert
        match error {
            ChainstateError::InterruptedWrite { .. } => {
                assert!(matches!(
                    mapped,
                    StorageError::InterruptedWrite {
                        namespace: StorageNamespace::Coins,
                        action: StorageRecoveryAction::Reindex,
                    }
                ));
            }
            ChainstateError::CoinsStorage { .. } => {
                assert!(matches!(
                    mapped,
                    StorageError::Corruption {
                        namespace: StorageNamespace::Coins,
                        ref detail,
                        action: StorageRecoveryAction::Repair,
                    } if detail == "coins backend"
                ));
            }
            ChainstateError::MissingTip => {
                assert!(matches!(
                    mapped,
                    StorageError::Corruption {
                        namespace: StorageNamespace::Coins,
                        action: StorageRecoveryAction::Repair,
                        ..
                    }
                ));
            }
            other => panic!("unexpected fixture {other:?}"),
        }
    }
}
