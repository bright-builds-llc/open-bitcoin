// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/blockfilter.h
// - packages/bitcoin-knots/src/undo.h
// - packages/bitcoin-knots/src/validation.cpp

//! Complete historical script projection from validated block/position/undo evidence.

use core::fmt;

use open_bitcoin_consensus::{
    BasicFilter, BasicFilterEncodingError, CodecError, FilterHeaderError, FilterHeaderPredecessor,
    block_hash, block_merkle_root,
};
use open_bitcoin_primitives::{Block, BlockHash, FilterHeader};

use crate::{BlockUndo, ChainPosition};

/// Undo tagged by the identity under which the retained authority stored it.
#[derive(Debug, Clone, Copy)]
pub struct HistoricalBlockUndo<'a> {
    pub block_hash: BlockHash,
    pub undo: &'a BlockUndo,
}

/// Borrowed complete facts; construction checks correspondence, not coin authenticity.
///
/// Callers must retain a validated block/position and authoritative undo. A fabricated
/// same-shape undo can pass these checks; validation staging supplies stronger provenance.
#[derive(Debug)]
pub struct BasicFilterInputs<'a> {
    block: &'a Block,
    position: &'a ChainPosition,
    maybe_undo: Option<&'a BlockUndo>,
}

impl<'a> BasicFilterInputs<'a> {
    /// Bind the transaction body to its validated header and refuse incomplete history.
    /// No current-coins view participates in historical script reconstruction.
    pub fn from_historical(
        block: &'a Block,
        position: &'a ChainPosition,
        maybe_history: Option<HistoricalBlockUndo<'a>>,
    ) -> Result<Self, BasicFilterInputError> {
        if block.header != position.header || block_hash(&block.header) != position.block_hash {
            return Err(BasicFilterInputError::BlockIdentity);
        }
        let null_parent = block.header.previous_block_hash == BlockHash::default();
        if (position.height == 0) != null_parent {
            return Err(BasicFilterInputError::GenesisPosition);
        }
        let Some(coinbase) = block.transactions.first() else {
            return Err(BasicFilterInputError::EmptyBlock);
        };
        if !coinbase.is_coinbase() || coinbase.outputs.is_empty() {
            return Err(BasicFilterInputError::CoinbaseStructure);
        }
        for (index, transaction) in block.transactions.iter().enumerate().skip(1) {
            if transaction.inputs.is_empty()
                || transaction.outputs.is_empty()
                || transaction
                    .inputs
                    .iter()
                    .any(|input| input.previous_output.is_null())
            {
                return Err(BasicFilterInputError::TransactionStructure { index });
            }
        }
        if position.height == 0 && block.transactions.len() != 1 {
            return Err(BasicFilterInputError::GenesisSpends);
        }
        let (merkle_root, mutated) =
            block_merkle_root(&block.transactions).map_err(BasicFilterInputError::BodyEncoding)?;
        if mutated {
            return Err(BasicFilterInputError::MutatedBody);
        }
        if merkle_root != block.header.merkle_root {
            return Err(BasicFilterInputError::BodyMerkleMismatch);
        }
        let maybe_undo = match maybe_history {
            Some(history) => {
                if history.block_hash != position.block_hash {
                    return Err(BasicFilterInputError::UndoIdentity);
                }
                let expected = block.transactions.len() - 1;
                if history.undo.transactions.len() != expected {
                    return Err(BasicFilterInputError::TransactionCount {
                        expected,
                        actual: history.undo.transactions.len(),
                    });
                }
                for (index, (transaction, undo)) in block
                    .transactions
                    .iter()
                    .skip(1)
                    .zip(&history.undo.transactions)
                    .enumerate()
                {
                    if undo.restored_inputs.len() != transaction.inputs.len() {
                        return Err(BasicFilterInputError::InputCount {
                            index: index + 1,
                            expected: transaction.inputs.len(),
                            actual: undo.restored_inputs.len(),
                        });
                    }
                }
                Some(history.undo)
            }
            None if position.height == 0 => None,
            None => return Err(BasicFilterInputError::MissingUndo),
        };
        Ok(Self {
            block,
            position,
            maybe_undo,
        })
    }

    /// Borrow every restored script in transaction/input order, including empty scripts.
    pub fn spent_scripts(&self) -> impl Iterator<Item = &'a [u8]> + '_ {
        self.maybe_undo
            .into_iter()
            .flat_map(|undo| &undo.transactions)
            .flat_map(|undo| &undo.restored_inputs)
            .map(|coin| coin.output.script_pubkey.as_bytes())
    }

    /// Generate BASIC bytes and commit against explicit predecessor branch/height facts.
    pub fn generate(
        &self,
        predecessor: FilterHeaderPredecessor,
    ) -> Result<(BasicFilter, FilterHeader), BasicFilterGenerationError> {
        let outputs: Vec<_> = self
            .block
            .transactions
            .iter()
            .flat_map(|tx| &tx.outputs)
            .map(|output| output.script_pubkey.as_bytes())
            .collect();
        let spent: Vec<_> = self.spent_scripts().collect();
        let filter = BasicFilter::from_script_facts(self.position.block_hash, &outputs, &spent)
            .map_err(BasicFilterGenerationError::Encoding)?;
        let header = filter
            .header(
                self.position.previous_block_hash(),
                self.position.height,
                predecessor,
            )
            .map_err(BasicFilterGenerationError::Header)?;
        Ok((filter, header))
    }
}

/// Refusal while proving complete historical input correspondence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicFilterInputError {
    BlockIdentity,
    BodyEncoding(CodecError),
    MutatedBody,
    BodyMerkleMismatch,
    GenesisPosition,
    EmptyBlock,
    CoinbaseStructure,
    TransactionStructure {
        index: usize,
    },
    GenesisSpends,
    MissingUndo,
    UndoIdentity,
    TransactionCount {
        expected: usize,
        actual: usize,
    },
    InputCount {
        index: usize,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for BasicFilterInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlockIdentity => {
                f.write_str("block header/hash does not match validated position")
            }
            Self::BodyEncoding(source) => write!(f, "block body encoding failed: {source}"),
            Self::MutatedBody => {
                f.write_str("block body contains a mutated transaction merkle tree")
            }
            Self::BodyMerkleMismatch => {
                f.write_str("block body merkle root does not match validated header")
            }
            Self::GenesisPosition => {
                f.write_str("height zero and null genesis parent must correspond")
            }
            Self::EmptyBlock => f.write_str("historical block has no coinbase transaction"),
            Self::CoinbaseStructure => {
                f.write_str("first transaction must be a coinbase with outputs")
            }
            Self::TransactionStructure { index } => {
                write!(f, "transaction {index} has invalid non-coinbase structure")
            }
            Self::GenesisSpends => f.write_str("genesis cannot contain spent inputs"),
            Self::MissingUndo => f.write_str("non-genesis block requires explicit complete undo"),
            Self::UndoIdentity => f.write_str("historical undo identity does not match block"),
            Self::TransactionCount { expected, actual } => write!(
                f,
                "undo transaction count {actual} does not match {expected}"
            ),
            Self::InputCount {
                index,
                expected,
                actual,
            } => write!(
                f,
                "transaction {index} undo input count {actual} does not match {expected}"
            ),
        }
    }
}

impl std::error::Error for BasicFilterInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BodyEncoding(source) => Some(source),
            _ => None,
        }
    }
}

/// Preserve the generator's resource refusal or contextual ancestry error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicFilterGenerationError {
    Encoding(BasicFilterEncodingError),
    Header(FilterHeaderError),
}

impl fmt::Display for BasicFilterGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encoding(source) => source.fmt(f),
            Self::Header(source) => source.fmt(f),
        }
    }
}

impl std::error::Error for BasicFilterGenerationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Encoding(source) => Some(source),
            Self::Header(source) => Some(source),
        }
    }
}

#[cfg(test)]
mod tests;
