use crate::hash::{BlockHash, MerkleRoot};
use crate::transaction::Transaction;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BlockHeader {
    pub version: i32,
    pub previous_block_hash: BlockHash,
    pub merkle_root: MerkleRoot,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
}

impl BlockHeader {
    pub fn is_null(&self) -> bool {
        self.bits == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[cfg(test)]
mod tests {
    use crate::hash::{BlockHash, MerkleRoot};

    use super::BlockHeader;

    #[test]
    fn block_header_is_null_when_bits_are_zero() {
        let null_header = BlockHeader::default();
        let non_null_header = BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([1_u8; 32]),
            time: 1,
            bits: 1,
            nonce: 1,
        };

        assert!(null_header.is_null());
        assert!(!non_null_header.is_null());
    }
}
