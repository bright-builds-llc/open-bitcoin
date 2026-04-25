// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/block.h
// - packages/bitcoin-knots/src/primitives/block.cpp
// - packages/bitcoin-knots/src/serialize.h
// - packages/bitcoin-knots/src/streams.h

use open_bitcoin_primitives::{Block, BlockHash, BlockHeader, MerkleRoot};

use crate::compact_size::{compact_size_to_usize, read_compact_size, write_compact_size};
use crate::error::CodecError;
use crate::primitives::{Reader, write_i32_le, write_u32_le};
use crate::transaction::{TransactionEncoding, encode_transaction, parse_transaction_from_reader};

pub fn parse_block_header(bytes: &[u8]) -> Result<BlockHeader, CodecError> {
    let mut reader = Reader::new(bytes);
    let header = parse_block_header_from_reader(&mut reader)?;
    reader.finish()?;
    Ok(header)
}

pub(crate) fn parse_block_header_from_reader(
    reader: &mut Reader<'_>,
) -> Result<BlockHeader, CodecError> {
    Ok(BlockHeader {
        version: reader.read_i32_le()?,
        previous_block_hash: BlockHash::from_byte_array(reader.read_array::<32>()?),
        merkle_root: MerkleRoot::from_byte_array(reader.read_array::<32>()?),
        time: reader.read_u32_le()?,
        bits: reader.read_u32_le()?,
        nonce: reader.read_u32_le()?,
    })
}

pub fn encode_block_header(header: &BlockHeader) -> Vec<u8> {
    let mut out = Vec::new();
    write_i32_le(&mut out, header.version);
    out.extend_from_slice(header.previous_block_hash.as_bytes());
    out.extend_from_slice(header.merkle_root.as_bytes());
    write_u32_le(&mut out, header.time);
    write_u32_le(&mut out, header.bits);
    write_u32_le(&mut out, header.nonce);
    out
}

pub fn parse_block(bytes: &[u8]) -> Result<Block, CodecError> {
    let mut reader = Reader::new(bytes);
    let block = parse_block_from_reader(&mut reader)?;
    reader.finish()?;
    Ok(block)
}

fn parse_block_from_reader(reader: &mut Reader<'_>) -> Result<Block, CodecError> {
    let header = parse_block_header_from_reader(reader)?;
    let count = compact_size_to_usize(read_compact_size(reader)?, "block transaction count");
    let mut transactions = Vec::with_capacity(count);
    for _ in 0..count {
        transactions.push(parse_transaction_from_reader(reader, true)?);
    }
    Ok(Block {
        header,
        transactions,
    })
}

pub fn encode_block(block: &Block) -> Result<Vec<u8>, CodecError> {
    let mut out = encode_block_header(&block.header);
    write_compact_size(&mut out, block.transactions.len() as u64)?;
    for transaction in &block.transactions {
        let encoded_transaction =
            encode_transaction(transaction, TransactionEncoding::WithWitness)?;
        out.extend_from_slice(&encoded_transaction);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{
        Amount, OutPoint, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid,
    };

    use crate::test_support::decode_hex;

    use super::{
        Block, BlockHash, BlockHeader, MerkleRoot, encode_block, parse_block, parse_block_header,
    };

    const GENESIS_BLOCK_HEADER_HEX: &str = include_str!("../testdata/block_header.hex");

    #[test]
    fn block_header_round_trips() {
        let bytes = decode_hex(GENESIS_BLOCK_HEADER_HEX);
        let header = parse_block_header(&bytes).expect("fixture should decode");
        let reencoded = super::encode_block_header(&header);

        assert_eq!(reencoded, bytes);
    }

    #[test]
    fn block_round_trips_single_transaction() {
        let transaction = Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([3_u8; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: Default::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(5).expect("valid amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            }],
            lock_time: 0,
        };
        let block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: MerkleRoot::from_byte_array([1_u8; 32]),
                time: 2,
                bits: 3,
                nonce: 4,
            },
            transactions: vec![transaction],
        };

        let encoded = encode_block(&block).expect("block should encode");
        let decoded = parse_block(&encoded).expect("block should decode");

        assert_eq!(decoded, block);
    }

    #[test]
    fn block_rejects_trailing_bytes() {
        let mut bytes = decode_hex(GENESIS_BLOCK_HEADER_HEX);
        bytes.push(0x00);

        let error = parse_block_header(&bytes).expect_err("trailing bytes must be rejected");
        assert_eq!(error.to_string(), "trailing data: 1 bytes");
    }
}
