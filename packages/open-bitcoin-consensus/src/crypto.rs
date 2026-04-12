mod ripemd160;
mod sha256;

use core::cmp::Ordering;

use open_bitcoin_codec::{
    CodecError, TransactionEncoding, encode_block_header, encode_transaction,
};
use open_bitcoin_primitives::{BlockHash, BlockHeader, MerkleRoot, Transaction, Txid, Wtxid};

pub use ripemd160::Ripemd160;
pub use sha256::Sha256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactTargetError {
    Negative,
    Overflow,
    Zero,
}

impl core::fmt::Display for CompactTargetError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Negative => write!(f, "compact target encodes a negative value"),
            Self::Overflow => write!(f, "compact target overflows 256 bits"),
            Self::Zero => write!(f, "compact target must be non-zero"),
        }
    }
}

impl std::error::Error for CompactTargetError {}

pub fn double_sha256(bytes: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(bytes);
    Sha256::digest(&first)
}

pub fn hash160(bytes: &[u8]) -> [u8; 20] {
    let sha = Sha256::digest(bytes);
    Ripemd160::digest(&sha)
}

pub fn transaction_txid(transaction: &Transaction) -> Result<Txid, CodecError> {
    let encoded = encode_transaction(transaction, TransactionEncoding::WithoutWitness)?;
    Ok(Txid::from_byte_array(double_sha256(&encoded)))
}

pub fn transaction_wtxid(transaction: &Transaction) -> Result<Wtxid, CodecError> {
    let encoding = if transaction.has_witness() {
        TransactionEncoding::WithWitness
    } else {
        TransactionEncoding::WithoutWitness
    };
    let encoded = encode_transaction(transaction, encoding)?;
    Ok(Wtxid::from_byte_array(double_sha256(&encoded)))
}

pub fn block_hash(header: &BlockHeader) -> BlockHash {
    BlockHash::from_byte_array(double_sha256(&encode_block_header(header)))
}

pub fn block_merkle_root(transactions: &[Transaction]) -> Result<(MerkleRoot, bool), CodecError> {
    if transactions.is_empty() {
        return Ok((MerkleRoot::from_byte_array([0_u8; 32]), false));
    }

    let mut level = Vec::with_capacity(transactions.len());
    for transaction in transactions {
        level.push(transaction_txid(transaction)?.to_byte_array());
    }

    let mut maybe_mutated = false;
    while level.len() > 1 {
        if level.len() % 2 == 1 {
            let last_hash = *level.last().expect("non-empty merkle level");
            level.push(last_hash);
        }

        let mut next_level = Vec::with_capacity(level.len() / 2);
        for pair in level.chunks_exact(2) {
            if pair[0] == pair[1] {
                maybe_mutated = true;
            }

            let mut concatenated = [0_u8; 64];
            concatenated[..32].copy_from_slice(&pair[0]);
            concatenated[32..].copy_from_slice(&pair[1]);
            next_level.push(double_sha256(&concatenated));
        }
        level = next_level;
    }

    Ok((MerkleRoot::from_byte_array(level[0]), maybe_mutated))
}

pub fn compact_target_bytes(bits: u32) -> Result<[u8; 32], CompactTargetError> {
    let exponent = (bits >> 24) as usize;
    let mantissa = bits & 0x007f_ffff;

    if (bits & 0x0080_0000) != 0 {
        return Err(CompactTargetError::Negative);
    }
    if mantissa == 0 {
        return Err(CompactTargetError::Zero);
    }
    if exponent > 34
        || (mantissa > 0x0000_ffff && exponent > 32)
        || (mantissa > 0x0000_00ff && exponent > 33)
    {
        return Err(CompactTargetError::Overflow);
    }

    let mut target = [0_u8; 32];
    if exponent <= 3 {
        let value = mantissa >> (8 * (3 - exponent));
        for (index, slot) in target.iter_mut().take(exponent).enumerate() {
            *slot = ((value >> (8 * index)) & 0xff) as u8;
        }
        return Ok(target);
    }

    let offset = exponent - 3;
    for shift in 0..3 {
        let byte = ((mantissa >> (8 * shift)) & 0xff) as u8;
        let index = offset + shift;
        if byte == 0 && index >= target.len() {
            continue;
        }
        if let Some(slot) = target.get_mut(index) {
            *slot = byte;
        }
    }

    Ok(target)
}

pub fn check_proof_of_work(hash: [u8; 32], bits: u32) -> Result<bool, CompactTargetError> {
    let target = compact_target_bytes(bits)?;
    Ok(compare_little_endian(&hash, &target) != Ordering::Greater)
}

fn compare_little_endian(left: &[u8; 32], right: &[u8; 32]) -> Ordering {
    for index in (0..32).rev() {
        match left[index].cmp(&right[index]) {
            Ordering::Equal => continue,
            ordering => return ordering,
        }
    }

    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use open_bitcoin_codec::parse_block_header;
    use open_bitcoin_primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput,
    };

    use super::{
        CompactTargetError, Ripemd160, Sha256, block_hash, block_merkle_root, check_proof_of_work,
        compact_target_bytes, double_sha256, hash160, transaction_txid, transaction_wtxid,
    };

    const GENESIS_BLOCK_HEADER_HEX: &str =
        include_str!("../../open-bitcoin-codec/testdata/block_header.hex");

    fn decode_hex(input: &str) -> Vec<u8> {
        let trimmed = input.trim();
        let mut bytes = Vec::with_capacity(trimmed.len() / 2);
        let chars: Vec<char> = trimmed.chars().collect();
        for pair in chars.chunks(2) {
            let high = pair[0].to_digit(16).expect("hex fixture");
            let low = pair[1].to_digit(16).expect("hex fixture");
            bytes.push(((high << 4) | low) as u8);
        }
        bytes
    }

    fn sample_transaction() -> Transaction {
        Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::from_bytes(vec![0x01, 0x01]).expect("valid script"),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            }],
            lock_time: 0,
        }
    }

    #[test]
    fn sha256_matches_known_vector() {
        let digest = Sha256::digest(b"abc");

        assert_eq!(
            digest,
            [
                0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
                0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
                0xf2, 0x00, 0x15, 0xad,
            ],
        );
    }

    #[test]
    fn double_sha256_matches_known_vector() {
        let digest = double_sha256(b"a");

        assert_eq!(
            digest,
            [
                0xbf, 0x5d, 0x3a, 0xff, 0xb7, 0x3e, 0xfd, 0x2e, 0xc6, 0xc3, 0x6a, 0xd3, 0x11, 0x2d,
                0xd9, 0x33, 0xef, 0xed, 0x63, 0xc4, 0xe1, 0xcb, 0xff, 0xcf, 0xa8, 0x8e, 0x27, 0x59,
                0xc1, 0x44, 0xf2, 0xd8,
            ],
        );
    }

    #[test]
    fn ripemd160_matches_known_vector() {
        let digest = Ripemd160::digest(b"abc");

        assert_eq!(
            digest,
            [
                0x8e, 0xb2, 0x08, 0xf7, 0xe0, 0x5d, 0x98, 0x7a, 0x9b, 0x04, 0x4a, 0x8e, 0x98, 0xc6,
                0xb0, 0x87, 0xf1, 0x5a, 0x0b, 0xfc,
            ],
        );
    }

    #[test]
    fn hash160_matches_known_vector() {
        let digest = hash160(b"abc");

        assert_eq!(
            digest,
            [
                0xbb, 0x1b, 0xe9, 0x8c, 0x14, 0x24, 0x44, 0xd7, 0xa5, 0x6a, 0xa3, 0x98, 0x1c, 0x39,
                0x42, 0xa9, 0x78, 0xe4, 0xdc, 0x33,
            ],
        );
    }

    #[test]
    fn transaction_ids_distinguish_witness_serialization() {
        let mut transaction = sample_transaction();
        transaction.inputs[0].witness = ScriptWitness::new(vec![vec![0x02]]);

        let txid = transaction_txid(&transaction).expect("txid should encode");
        let wtxid = transaction_wtxid(&transaction).expect("wtxid should encode");

        assert_ne!(txid.to_byte_array(), wtxid.to_byte_array());
    }

    #[test]
    fn merkle_root_marks_duplicate_pairs_as_mutated() {
        let transaction = sample_transaction();
        let (merkle_root, maybe_mutated) =
            block_merkle_root(&[transaction.clone(), transaction]).expect("merkle root");

        assert_ne!(merkle_root.to_byte_array(), [0_u8; 32]);
        assert!(maybe_mutated);
    }

    #[test]
    fn compact_target_rejects_invalid_values() {
        assert_eq!(
            compact_target_bytes(0x0000_0000),
            Err(CompactTargetError::Zero),
        );
        assert_eq!(
            compact_target_bytes(0x0180_0000),
            Err(CompactTargetError::Negative),
        );
        assert_eq!(
            compact_target_bytes(0x2301_0000),
            Err(CompactTargetError::Overflow),
        );
    }

    #[test]
    fn compact_target_display_strings_are_descriptive() {
        assert_eq!(
            CompactTargetError::Negative.to_string(),
            "compact target encodes a negative value",
        );
        assert_eq!(
            CompactTargetError::Overflow.to_string(),
            "compact target overflows 256 bits",
        );
        assert_eq!(
            CompactTargetError::Zero.to_string(),
            "compact target must be non-zero",
        );
    }

    #[test]
    fn transaction_wtxid_without_witness_matches_legacy_txid() {
        let transaction = sample_transaction();

        let txid = transaction_txid(&transaction).expect("txid should encode");
        let wtxid = transaction_wtxid(&transaction).expect("wtxid should encode");

        assert_eq!(txid.to_byte_array(), wtxid.to_byte_array());
    }

    #[test]
    fn empty_merkle_root_returns_zero_hash() {
        let (merkle_root, maybe_mutated) = block_merkle_root(&[]).expect("empty root");

        assert_eq!(merkle_root.to_byte_array(), [0_u8; 32]);
        assert!(!maybe_mutated);
    }

    #[test]
    fn compact_target_handles_small_exponents_and_equal_hashes() {
        let target = compact_target_bytes(0x0300_ffff).expect("small exponent target");
        assert_eq!(target[0], 0xff);
        assert_eq!(target[1], 0xff);

        let wide_target = compact_target_bytes(0x2200_0001).expect("wide target");
        assert!(check_proof_of_work(wide_target, 0x2200_0001).expect("pow result"));
    }

    #[test]
    fn genesis_header_hash_meets_its_claimed_target() {
        let header = parse_block_header(&decode_hex(GENESIS_BLOCK_HEADER_HEX))
            .expect("genesis header fixture should parse");
        let hash = block_hash(&header);

        let valid = check_proof_of_work(hash.to_byte_array(), header.bits)
            .expect("genesis bits should decode");

        assert!(valid);
    }
}
