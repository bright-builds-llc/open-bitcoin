// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use core::fmt;

use open_bitcoin_consensus::{BlockValidationError, TxValidationError};
use open_bitcoin_primitives::{BlockHash, OutPoint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainstateError {
    MissingCoin {
        outpoint: OutPoint,
    },
    MissingTip,
    MissingUndo {
        block_hash: BlockHash,
    },
    InvalidGenesisParent {
        block_hash: BlockHash,
    },
    InvalidTipExtension {
        expected_previous: BlockHash,
        actual_previous: BlockHash,
    },
    OutputOverwrite {
        outpoint: OutPoint,
    },
    DisconnectBlockMismatch {
        expected_tip: BlockHash,
        actual_block: BlockHash,
    },
    UndoMismatch {
        expected_transactions: usize,
        actual_transactions: usize,
    },
    RestoredCoinOverwrite {
        outpoint: OutPoint,
    },
    DisconnectSpentOutputMismatch {
        outpoint: OutPoint,
    },
    DisconnectPastGenesis {
        requested: usize,
        available: usize,
    },
    BlockValidation {
        source: BlockValidationError,
    },
    TransactionValidation {
        source: TxValidationError,
    },
    Serialization {
        context: &'static str,
        reason: String,
    },
}

impl fmt::Display for ChainstateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCoin { outpoint } => {
                write!(
                    f,
                    "missing coin for outpoint {:?}:{}",
                    outpoint.txid, outpoint.vout
                )
            }
            Self::MissingTip => write!(f, "chainstate has no active tip"),
            Self::MissingUndo { block_hash } => {
                write!(f, "missing undo data for block {:?}", block_hash)
            }
            Self::InvalidGenesisParent { block_hash } => {
                write!(
                    f,
                    "genesis position for block {:?} must use a null previous block hash",
                    block_hash
                )
            }
            Self::InvalidTipExtension {
                expected_previous,
                actual_previous,
            } => write!(
                f,
                "invalid tip extension: expected previous block {:?}, got {:?}",
                expected_previous, actual_previous
            ),
            Self::OutputOverwrite { outpoint } => {
                write!(
                    f,
                    "refusing to overwrite existing unspent output {:?}:{}",
                    outpoint.txid, outpoint.vout
                )
            }
            Self::DisconnectBlockMismatch {
                expected_tip,
                actual_block,
            } => write!(
                f,
                "disconnect block mismatch: expected tip {:?}, got {:?}",
                expected_tip, actual_block
            ),
            Self::UndoMismatch {
                expected_transactions,
                actual_transactions,
            } => write!(
                f,
                "undo data mismatch: expected {expected_transactions} transaction undos, got {actual_transactions}",
            ),
            Self::RestoredCoinOverwrite { outpoint } => write!(
                f,
                "disconnect attempted to restore coin into occupied outpoint {:?}:{}",
                outpoint.txid, outpoint.vout
            ),
            Self::DisconnectSpentOutputMismatch { outpoint } => write!(
                f,
                "disconnect found mismatched created output at {:?}:{}",
                outpoint.txid, outpoint.vout
            ),
            Self::DisconnectPastGenesis {
                requested,
                available,
            } => write!(
                f,
                "cannot disconnect {requested} blocks from a chain with only {available} active positions"
            ),
            Self::BlockValidation { source } => write!(f, "{source}"),
            Self::TransactionValidation { source } => write!(f, "{source}"),
            Self::Serialization { context, reason } => {
                write!(f, "{context} serialization failed: {reason}")
            }
        }
    }
}

impl std::error::Error for ChainstateError {}

#[cfg(test)]
mod tests {
    use open_bitcoin_consensus::{BlockValidationResult, TxValidationResult, ValidationError};
    use open_bitcoin_primitives::{BlockHash, OutPoint, Txid};

    use super::ChainstateError;

    fn sample_outpoint() -> OutPoint {
        OutPoint {
            txid: Txid::from_byte_array([7_u8; 32]),
            vout: 3,
        }
    }

    #[test]
    fn display_messages_cover_every_error_variant() {
        let outpoint = sample_outpoint();
        let block_hash = BlockHash::from_byte_array([9_u8; 32]);
        let cases = [
            ChainstateError::MissingCoin {
                outpoint: outpoint.clone(),
            },
            ChainstateError::MissingTip,
            ChainstateError::MissingUndo { block_hash },
            ChainstateError::InvalidGenesisParent { block_hash },
            ChainstateError::InvalidTipExtension {
                expected_previous: block_hash,
                actual_previous: BlockHash::from_byte_array([1_u8; 32]),
            },
            ChainstateError::OutputOverwrite {
                outpoint: outpoint.clone(),
            },
            ChainstateError::DisconnectBlockMismatch {
                expected_tip: block_hash,
                actual_block: BlockHash::from_byte_array([2_u8; 32]),
            },
            ChainstateError::UndoMismatch {
                expected_transactions: 2,
                actual_transactions: 1,
            },
            ChainstateError::RestoredCoinOverwrite {
                outpoint: outpoint.clone(),
            },
            ChainstateError::DisconnectSpentOutputMismatch {
                outpoint: outpoint.clone(),
            },
            ChainstateError::DisconnectPastGenesis {
                requested: 4,
                available: 1,
            },
            ChainstateError::BlockValidation {
                source: ValidationError::new(
                    BlockValidationResult::Consensus,
                    "bad-block",
                    Some("details".to_string()),
                ),
            },
            ChainstateError::TransactionValidation {
                source: ValidationError::new(
                    TxValidationResult::Consensus,
                    "bad-tx",
                    Some("details".to_string()),
                ),
            },
            ChainstateError::Serialization {
                context: "txid derivation",
                reason: "bad compact size".to_string(),
            },
        ];

        for error in cases {
            assert!(!error.to_string().is_empty());
        }
    }

    #[test]
    fn validation_errors_forward_their_display_text() {
        let block_error = ChainstateError::BlockValidation {
            source: ValidationError::new(
                BlockValidationResult::Consensus,
                "bad-block",
                Some("details".to_string()),
            ),
        };
        let tx_error = ChainstateError::TransactionValidation {
            source: ValidationError::new(
                TxValidationResult::Consensus,
                "bad-tx",
                Some("details".to_string()),
            ),
        };

        assert_eq!(block_error.to_string(), "bad-block (details)");
        assert_eq!(tx_error.to_string(), "bad-tx (details)");
    }
}
