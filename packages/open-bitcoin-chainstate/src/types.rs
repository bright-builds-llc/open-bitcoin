use std::collections::HashMap;

use open_bitcoin_consensus::{SpentOutput, block_hash};
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader, OutPoint, TransactionOutput};

use crate::error::ChainstateError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coin {
    pub output: TransactionOutput,
    pub is_coinbase: bool,
    pub created_height: u32,
    pub created_median_time_past: i64,
}

impl Coin {
    pub fn as_spent_output(&self) -> SpentOutput {
        SpentOutput {
            value: self.output.value,
            script_pubkey: self.output.script_pubkey.clone(),
            is_coinbase: self.is_coinbase,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TxUndo {
    pub restored_inputs: Vec<Coin>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BlockUndo {
    pub transactions: Vec<TxUndo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainPosition {
    pub block_hash: BlockHash,
    pub header: BlockHeader,
    pub height: u32,
    pub chain_work: u128,
    pub median_time_past: i64,
}

impl ChainPosition {
    pub fn new(header: BlockHeader, height: u32, chain_work: u128, median_time_past: i64) -> Self {
        let block_hash = block_hash(&header);

        Self {
            block_hash,
            header,
            height,
            chain_work,
            median_time_past,
        }
    }

    pub fn genesis(
        header: BlockHeader,
        chain_work: u128,
        median_time_past: i64,
    ) -> Result<Self, ChainstateError> {
        let position = Self::new(header, 0, chain_work, median_time_past);
        if position.header.previous_block_hash.to_byte_array() != [0_u8; 32] {
            return Err(ChainstateError::InvalidGenesisParent {
                block_hash: position.block_hash,
            });
        }

        Ok(position)
    }

    pub fn previous_block_hash(&self) -> BlockHash {
        self.header.previous_block_hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainstateSnapshot {
    pub active_chain: Vec<ChainPosition>,
    pub utxos: HashMap<OutPoint, Coin>,
    pub undo_by_block: HashMap<BlockHash, BlockUndo>,
}

impl ChainstateSnapshot {
    pub fn new(
        active_chain: Vec<ChainPosition>,
        utxos: HashMap<OutPoint, Coin>,
        undo_by_block: HashMap<BlockHash, BlockUndo>,
    ) -> Self {
        Self {
            active_chain,
            utxos,
            undo_by_block,
        }
    }

    pub fn tip(&self) -> Option<&ChainPosition> {
        self.active_chain.last()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnchoredBlock {
    pub block: Block,
    pub chain_work: u128,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChainTransition {
    pub disconnected: Vec<ChainPosition>,
    pub connected: Vec<ChainPosition>,
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{BlockHash, BlockHeader};

    use super::ChainPosition;

    #[test]
    fn previous_block_hash_returns_the_parent_hash() {
        let previous_block_hash = BlockHash::from_byte_array([5_u8; 32]);
        let header = BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root: Default::default(),
            time: 1,
            bits: 1,
            nonce: 1,
        };
        let position = ChainPosition::new(header, 2, 3, 4);

        assert_eq!(position.previous_block_hash(), previous_block_hash);
    }
}
