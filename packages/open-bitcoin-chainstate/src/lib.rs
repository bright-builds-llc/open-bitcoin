#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented,
        clippy::panic_in_result_fn,
    )
)]
// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure-core chainstate and UTXO domain models for Open Bitcoin.

pub mod engine;
pub mod error;
pub mod types;

pub use engine::{Chainstate, prefer_candidate_tip};
pub use error::ChainstateError;
pub use types::{
    AnchoredBlock, BlockUndo, ChainPosition, ChainTransition, ChainstateSnapshot, Coin, TxUndo,
};

pub const fn crate_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use open_bitcoin_primitives::{
        Amount, BlockHash, BlockHeader, OutPoint, ScriptBuf, TransactionOutput, Txid,
    };

    use crate::{ChainPosition, ChainstateError, ChainstateSnapshot, Coin, crate_ready};

    fn sample_header(previous_block_hash: BlockHash, time: u32) -> BlockHeader {
        BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root: Default::default(),
            time,
            bits: 0x207f_ffff,
            nonce: 0,
        }
    }

    #[test]
    fn crate_ready_reports_true() {
        assert!(crate_ready());
    }

    #[test]
    fn coin_converts_to_spent_output_without_losing_metadata() {
        // Arrange
        let coin = Coin {
            output: TransactionOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            },
            is_coinbase: true,
            created_height: 7,
            created_median_time_past: 1_000,
        };

        // Act
        let spent_output = coin.as_spent_output();

        // Assert
        assert_eq!(spent_output.value, coin.output.value);
        assert_eq!(spent_output.script_pubkey, coin.output.script_pubkey);
        assert!(spent_output.is_coinbase);
    }

    #[test]
    fn genesis_requires_a_null_previous_block_hash() {
        // Arrange
        let non_genesis_header = sample_header(BlockHash::from_byte_array([9_u8; 32]), 10);

        // Act
        let result = ChainPosition::genesis(non_genesis_header, 1, 10);

        // Assert
        assert!(matches!(
            result,
            Err(ChainstateError::InvalidGenesisParent { .. })
        ));
    }

    #[test]
    fn snapshot_tip_returns_the_last_active_position() {
        // Arrange
        let genesis = ChainPosition::genesis(sample_header(Default::default(), 1), 1, 1)
            .expect("genesis position");
        let tip = ChainPosition::new(sample_header(genesis.block_hash, 2), 1, 2, 2);
        let mut utxos = HashMap::new();
        utxos.insert(
            OutPoint {
                txid: Txid::from_byte_array([3_u8; 32]),
                vout: 0,
            },
            Coin {
                output: TransactionOutput {
                    value: Amount::from_sats(25).expect("valid amount"),
                    script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
                },
                is_coinbase: false,
                created_height: 1,
                created_median_time_past: 2,
            },
        );
        let snapshot =
            ChainstateSnapshot::new(vec![genesis.clone(), tip.clone()], utxos, HashMap::new());

        // Act
        let maybe_tip = snapshot.tip();

        // Assert
        assert_eq!(maybe_tip, Some(&tip));
        assert_eq!(snapshot.active_chain[0], genesis);
    }
}
