// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/block.h
// - packages/bitcoin-knots/src/core_read.cpp
// - packages/bitcoin-knots/src/serialize.h

use crate::transaction::wire::{TransactionView, WireTransaction};
use crate::{CodecError, primitives::Reader};
use open_bitcoin_primitives::{Block, BlockHeader, Transaction};

/// Complete serialized block data without a canonical seal or validity claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireBlock {
    pub header: BlockHeader,
    pub transactions: Vec<WireTransaction>,
}

/// Borrowed serialization facts, never block authority.
pub trait BlockView {
    type Transaction: TransactionView;
    fn header(&self) -> &BlockHeader;
    fn transactions(&self) -> &[Self::Transaction];
}
impl BlockView for Block {
    type Transaction = Transaction;
    fn header(&self) -> &BlockHeader {
        &self.header
    }
    fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
}
impl BlockView for WireBlock {
    type Transaction = WireTransaction;
    fn header(&self) -> &BlockHeader {
        &self.header
    }
    fn transactions(&self) -> &[WireTransaction] {
        &self.transactions
    }
}
impl WireBlock {
    /// Fallibly move exact scripts and witness into the monetary domain.
    pub fn try_into_block(self) -> Result<Block, CodecError> {
        Ok(Block {
            header: self.header,
            transactions: self
                .transactions
                .into_iter()
                .map(WireTransaction::try_into_transaction)
                .collect::<Result<_, _>>()?,
        })
    }
}

/// Decode a complete first block, returning its exact consumed prefix length.
pub fn parse_submission_block_prefix(bytes: &[u8]) -> Result<(WireBlock, usize), CodecError> {
    let mut reader = Reader::new(bytes);
    let (header, transactions) =
        super::parse_block_parts::<crate::transaction::SubmissionWire>(&mut reader)?;
    Ok((
        WireBlock {
            header,
            transactions,
        },
        bytes.len() - reader.remaining(),
    ))
}

/// Decode exactly one serialized domain block, preserving script bytes.
pub fn parse_serialized_block(bytes: &[u8]) -> Result<Block, CodecError> {
    let mut reader = Reader::new(bytes);
    let (header, transactions) =
        super::parse_block_parts::<crate::transaction::DomainMode<true>>(&mut reader)?;
    reader.finish()?;
    Ok(Block {
        header,
        transactions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_bitcoin_primitives::{BlockHash, MerkleRoot};

    #[test]
    fn phase167_submission_wire_codec_block_strict_and_prefix_control() {
        // Arrange
        let block = WireBlock {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0; 32]),
                merkle_root: MerkleRoot::from_byte_array([0; 32]),
                time: 0,
                bits: 0,
                nonce: 0,
            },
            transactions: vec![],
        };
        let bytes = super::super::encode_block_view(&block).expect("encode block");
        let mut suffix = bytes.clone();
        suffix.push(1);
        // Act
        let (decoded, offset) = parse_submission_block_prefix(&suffix).expect("complete prefix");
        // Assert
        assert_eq!(decoded, block);
        assert_eq!(offset, bytes.len());
        assert_eq!(
            parse_serialized_block(&suffix),
            Err(CodecError::TrailingData { remaining: 1 })
        );
        assert!(parse_serialized_block(&bytes).is_ok());
        for length in 0..bytes.len() {
            assert!(parse_submission_block_prefix(&bytes[..length]).is_err());
        }
    }
}
