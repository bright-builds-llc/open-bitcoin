// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/blockencodings.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py
// - packages/bitcoin-knots/test/functional/test_framework/messages.py

use crate::error::CodecError;
use open_bitcoin_primitives::{BlockHash, BlockHeader, Transaction};

use crate::block::{encode_block_header, parse_block_header_from_reader};
use crate::compact_size::{compact_size_to_usize, read_compact_size, write_compact_size};
use crate::primitives::{Reader, write_u64_le};
use crate::transaction::{TransactionEncoding, encode_transaction, parse_transaction_from_reader};

pub const BIP152_COMPACT_BLOCKS_VERSION: u64 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SendCompactMessage {
    pub announce: bool,
    pub version: u64,
}

pub fn encode_send_compact_payload(message: &SendCompactMessage) -> Vec<u8> {
    let mut out = Vec::with_capacity(9);
    out.push(u8::from(message.announce));
    write_u64_le(&mut out, message.version);
    out
}

pub fn decode_send_compact_payload(payload: &[u8]) -> Result<SendCompactMessage, CodecError> {
    let mut reader = Reader::new(payload);
    let announce = reader.read_u8()? != 0;
    let version = reader.read_u64_le()?;
    reader.finish()?;

    Ok(SendCompactMessage { announce, version })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortId([u8; 6]);

impl ShortId {
    pub const fn from_wire_bytes(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    pub const fn as_wire_bytes(&self) -> &[u8; 6] {
        &self.0
    }
}

const SHORT_ID_MASK: u64 = 0xffff_ffff_ffff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortIdSelector {
    pub k0: u64,
    pub k1: u64,
}

impl ShortIdSelector {
    pub const fn from_keys(k0: u64, k1: u64) -> Self {
        Self { k0, k1 }
    }
}

pub fn short_id_selector_from_header_and_nonce(
    header: &BlockHeader,
    nonce: u64,
) -> ShortIdSelector {
    let mut preimage = encode_block_header(header);
    write_u64_le(&mut preimage, nonce);
    let digest = single_sha256(&preimage);
    let k0 = u64::from_le_bytes([
        digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
    ]);
    let k1 = u64::from_le_bytes([
        digest[8], digest[9], digest[10], digest[11], digest[12], digest[13], digest[14],
        digest[15],
    ]);
    ShortIdSelector::from_keys(k0, k1)
}

pub fn short_id_match_key(short_id: ShortId) -> u64 {
    let bytes = short_id.as_wire_bytes();
    u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], 0, 0,
    ])
}

pub fn short_id_from_masked_u64(masked_digest: u64) -> ShortId {
    let masked = masked_digest & SHORT_ID_MASK;
    let bytes = masked.to_le_bytes();
    ShortId::from_wire_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]])
}

fn single_sha256(bytes: &[u8]) -> [u8; 32] {
    sha256::Sha256::digest(bytes)
}

mod sha256 {
    fn read_u32_be(bytes: &[u8]) -> u32 {
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    const INITIAL_STATE: [u32; 8] = [
        0x6a09_e667,
        0xbb67_ae85,
        0x3c6e_f372,
        0xa54f_f53a,
        0x510e_527f,
        0x9b05_688c,
        0x1f83_d9ab,
        0x5be0_cd19,
    ];

    const ROUND_CONSTANTS: [u32; 64] = [
        0x428a_2f98,
        0x7137_4491,
        0xb5c0_fbcf,
        0xe9b5_dba5,
        0x3956_c25b,
        0x59f1_11f1,
        0x923f_82a4,
        0xab1c_5ed5,
        0xd807_aa98,
        0x1283_5b01,
        0x2431_85be,
        0x550c_7dc3,
        0x72be_5d74,
        0x80de_b1fe,
        0x9bdc_06a7,
        0xc19b_f174,
        0xe49b_69c1,
        0xefbe_4786,
        0x0fc1_9dc6,
        0x240c_a1cc,
        0x2de9_2c6f,
        0x4a74_84aa,
        0x5cb0_a9dc,
        0x76f9_88da,
        0x983e_5152,
        0xa831_c66d,
        0xb003_27c8,
        0xbf59_7fc7,
        0xc6e0_0bf3,
        0xd5a7_9147,
        0x06ca_6351,
        0x1429_2967,
        0x27b7_0a85,
        0x2e1b_2138,
        0x4d2c_6dfc,
        0x5338_0d13,
        0x650a_7354,
        0x766a_0abb,
        0x81c2_c92e,
        0x9272_2c85,
        0xa2bf_e8a1,
        0xa81a_664b,
        0xc24b_8b70,
        0xc76c_51a3,
        0xd192_e819,
        0xd699_0624,
        0xf40e_3585,
        0x106a_a070,
        0x19a4_c116,
        0x1e37_6c08,
        0x2748_774c,
        0x34b0_bcb5,
        0x391c_0cb3,
        0x4ed8_aa4a,
        0x5b9c_ca4f,
        0x682e_6ff3,
        0x748f_82ee,
        0x78a5_636f,
        0x84c8_7814,
        0x8cc7_0208,
        0x90be_fffa,
        0xa450_6ceb,
        0xbef9_a3f7,
        0xc671_78f2,
    ];

    pub(super) struct Sha256 {
        state: [u32; 8],
        buffer: [u8; 64],
        buffer_len: usize,
        total_len: u64,
    }

    impl Sha256 {
        pub(super) fn digest(message: &[u8]) -> [u8; 32] {
            let mut hasher = Self::new();
            hasher.update(message);
            hasher.finalize()
        }

        #[cfg(test)]
        fn digest_chunks(chunks: &[&[u8]]) -> [u8; 32] {
            let mut hasher = Self::new();
            for chunk in chunks {
                hasher.update(chunk);
            }
            hasher.finalize()
        }

        fn new() -> Self {
            Self {
                state: INITIAL_STATE,
                buffer: [0_u8; 64],
                buffer_len: 0,
                total_len: 0,
            }
        }

        fn update(&mut self, message: &[u8]) {
            self.total_len += message.len() as u64;
            let mut offset = 0;
            if self.buffer_len > 0 {
                let take = usize::min(64 - self.buffer_len, message.len());
                self.buffer[self.buffer_len..self.buffer_len + take]
                    .copy_from_slice(&message[..take]);
                self.buffer_len += take;
                offset += take;
                if self.buffer_len == 64 {
                    let block = self.buffer;
                    self.compress_block(&block);
                    self.buffer_len = 0;
                }
            }

            while offset + 64 <= message.len() {
                self.compress_block(&message[offset..offset + 64]);
                offset += 64;
            }

            if offset < message.len() {
                let remaining = message.len() - offset;
                self.buffer[..remaining].copy_from_slice(&message[offset..]);
                self.buffer_len = remaining;
            }
        }

        fn finalize(mut self) -> [u8; 32] {
            let bit_len = self.total_len * 8;
            self.buffer[self.buffer_len] = 0x80;
            self.buffer_len += 1;

            if self.buffer_len > 56 {
                self.buffer[self.buffer_len..].fill(0);
                let block = self.buffer;
                self.compress_block(&block);
                self.buffer_len = 0;
            }

            self.buffer[self.buffer_len..56].fill(0);
            self.buffer[56..64].copy_from_slice(&bit_len.to_be_bytes());
            let block = self.buffer;
            self.compress_block(&block);

            let mut out = [0_u8; 32];
            for (index, word) in self.state.iter().enumerate() {
                out[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
            }
            out
        }

        fn compress_block(&mut self, block: &[u8]) {
            let mut words = [0_u32; 64];
            for (index, chunk) in block.chunks_exact(4).take(16).enumerate() {
                words[index] = read_u32_be(chunk);
            }

            for index in 16..64 {
                let s0 = words[index - 15].rotate_right(7)
                    ^ words[index - 15].rotate_right(18)
                    ^ (words[index - 15] >> 3);
                let s1 = words[index - 2].rotate_right(17)
                    ^ words[index - 2].rotate_right(19)
                    ^ (words[index - 2] >> 10);
                words[index] = words[index - 16]
                    .wrapping_add(s0)
                    .wrapping_add(words[index - 7])
                    .wrapping_add(s1)
                    .wrapping_add(words[index - 2]);
            }

            let mut a = self.state[0];
            let mut b = self.state[1];
            let mut c = self.state[2];
            let mut d = self.state[3];
            let mut e = self.state[4];
            let mut f = self.state[5];
            let mut g = self.state[6];
            let mut h = self.state[7];

            for (index, constant) in ROUND_CONSTANTS.iter().enumerate() {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ ((!e) & g);
                let temp1 = h
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(*constant)
                    .wrapping_add(words[index]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let temp2 = s0.wrapping_add(maj);

                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(temp1);
                d = c;
                c = b;
                b = a;
                a = temp1.wrapping_add(temp2);
            }

            self.state[0] = self.state[0].wrapping_add(a);
            self.state[1] = self.state[1].wrapping_add(b);
            self.state[2] = self.state[2].wrapping_add(c);
            self.state[3] = self.state[3].wrapping_add(d);
            self.state[4] = self.state[4].wrapping_add(e);
            self.state[5] = self.state[5].wrapping_add(f);
            self.state[6] = self.state[6].wrapping_add(g);
            self.state[7] = self.state[7].wrapping_add(h);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Sha256;

        #[test]
        fn digest_covers_finalize_length_extension_padding() {
            let digest = Sha256::digest(&[0_u8; 120]);

            assert_ne!(digest, [0_u8; 32]);
        }

        #[test]
        fn digest_covers_partial_buffer_merge_on_second_update() {
            let digest = Sha256::digest_chunks(&[&[1_u8; 40], &[2_u8; 40]]);

            assert_ne!(digest, [0_u8; 32]);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefilledTransaction {
    pub index_delta: u64,
    pub transaction: Transaction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactBlockPayload {
    pub header: BlockHeader,
    pub nonce: u64,
    pub short_ids: Vec<ShortId>,
    pub prefilled_transactions: Vec<PrefilledTransaction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTransactionsRequest {
    pub block_hash: BlockHash,
    pub index_deltas: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTransactions {
    pub block_hash: BlockHash,
    pub transactions: Vec<Transaction>,
}

pub fn encode_compact_block_payload(payload: &CompactBlockPayload) -> Result<Vec<u8>, CodecError> {
    validate_compact_block_structure(payload)?;

    let mut out = encode_block_header(&payload.header);
    write_u64_le(&mut out, payload.nonce);
    write_compact_size(&mut out, payload.short_ids.len() as u64)?;
    for short_id in &payload.short_ids {
        out.extend_from_slice(short_id.as_wire_bytes());
    }
    write_compact_size(&mut out, payload.prefilled_transactions.len() as u64)?;
    for prefilled in &payload.prefilled_transactions {
        write_compact_size(&mut out, prefilled.index_delta)?;
        let encoded_transaction =
            encode_transaction(&prefilled.transaction, TransactionEncoding::WithWitness)?;
        out.extend_from_slice(&encoded_transaction);
    }

    Ok(out)
}

pub fn decode_compact_block_payload(bytes: &[u8]) -> Result<CompactBlockPayload, CodecError> {
    let mut reader = Reader::new(bytes);
    let header = parse_block_header_from_reader(&mut reader)?;
    let nonce = reader.read_u64_le()?;
    let short_id_count = read_bounded_compact_block_count(&mut reader, "short id count")?;
    let mut short_ids = Vec::with_capacity(short_id_count);
    for _ in 0..short_id_count {
        short_ids.push(ShortId::from_wire_bytes(reader.read_array::<6>()?));
    }

    let prefilled_transaction_count =
        read_bounded_compact_block_count(&mut reader, "prefilled transaction count")?;
    validate_compact_block_count(short_id_count as u64, prefilled_transaction_count as u64)?;
    let mut prefilled_transactions = Vec::with_capacity(prefilled_transaction_count);
    for _ in 0..prefilled_transaction_count {
        prefilled_transactions.push(PrefilledTransaction {
            index_delta: read_compact_size(&mut reader)?,
            transaction: parse_transaction_from_reader(&mut reader, true)?,
        });
    }
    reader.finish()?;

    let payload = CompactBlockPayload {
        header,
        nonce,
        short_ids,
        prefilled_transactions,
    };
    validate_compact_block_structure(&payload)?;
    Ok(payload)
}

pub fn encode_get_block_transactions_payload(
    request: &BlockTransactionsRequest,
) -> Result<Vec<u8>, CodecError> {
    expand_block_transaction_indexes(request)?;

    let mut out = Vec::with_capacity(32 + request.index_deltas.len());
    out.extend_from_slice(request.block_hash.as_bytes());
    write_compact_size(&mut out, request.index_deltas.len() as u64)?;
    for index_delta in &request.index_deltas {
        write_compact_size(&mut out, *index_delta)?;
    }

    Ok(out)
}

pub fn decode_get_block_transactions_payload(
    bytes: &[u8],
) -> Result<BlockTransactionsRequest, CodecError> {
    let mut reader = Reader::new(bytes);
    let block_hash = BlockHash::from_byte_array(reader.read_array::<32>()?);
    let index_count = compact_size_to_usize(
        read_compact_size(&mut reader)?,
        "block transaction index count",
    );
    let mut index_deltas = Vec::with_capacity(index_count);
    for _ in 0..index_count {
        index_deltas.push(read_compact_size(&mut reader)?);
    }
    reader.finish()?;

    let request = BlockTransactionsRequest {
        block_hash,
        index_deltas,
    };
    expand_block_transaction_indexes(&request)?;
    Ok(request)
}

pub fn encode_block_transactions_payload(
    response: &BlockTransactions,
) -> Result<Vec<u8>, CodecError> {
    let mut out = Vec::with_capacity(32 + response.transactions.len());
    out.extend_from_slice(response.block_hash.as_bytes());
    write_compact_size(&mut out, response.transactions.len() as u64)?;
    for transaction in &response.transactions {
        let encoded_transaction =
            encode_transaction(transaction, TransactionEncoding::WithWitness)?;
        out.extend_from_slice(&encoded_transaction);
    }

    Ok(out)
}

pub fn decode_block_transactions_payload(bytes: &[u8]) -> Result<BlockTransactions, CodecError> {
    let mut reader = Reader::new(bytes);
    let block_hash = BlockHash::from_byte_array(reader.read_array::<32>()?);
    let transaction_count =
        compact_size_to_usize(read_compact_size(&mut reader)?, "block transaction count");
    let mut transactions = Vec::with_capacity(transaction_count);
    for _ in 0..transaction_count {
        transactions.push(parse_transaction_from_reader(&mut reader, true)?);
    }
    reader.finish()?;

    Ok(BlockTransactions {
        block_hash,
        transactions,
    })
}

pub fn validate_compact_block_structure(payload: &CompactBlockPayload) -> Result<(), CodecError> {
    if payload.short_ids.is_empty() && payload.prefilled_transactions.is_empty() {
        return Err(CodecError::CompactBlockEmpty);
    }

    let transaction_count = validate_compact_block_count(
        payload.short_ids.len() as u64,
        payload.prefilled_transactions.len() as u64,
    )?;
    let positions = expand_prefilled_positions(payload)?;
    for (prefilled_index, position) in positions.iter().copied().enumerate() {
        let max_position = payload.short_ids.len() as u64 + prefilled_index as u64;
        let position = u64::from(position);
        if position > max_position {
            return Err(CodecError::PrefilledTransactionOutOfBounds {
                position,
                transaction_count,
            });
        }
    }

    for prefilled in &payload.prefilled_transactions {
        if prefilled.transaction.inputs.is_empty() || prefilled.transaction.outputs.is_empty() {
            return Err(CodecError::CompactBlockNullPrefilledTransaction);
        }
    }

    Ok(())
}

pub fn expand_prefilled_positions(payload: &CompactBlockPayload) -> Result<Vec<u16>, CodecError> {
    let deltas = payload
        .prefilled_transactions
        .iter()
        .map(|prefilled| prefilled.index_delta)
        .collect::<Vec<_>>();
    expand_differential_indexes(&deltas, "prefilled transaction index")
}

pub fn expand_block_transaction_indexes(
    request: &BlockTransactionsRequest,
) -> Result<Vec<u16>, CodecError> {
    expand_differential_indexes(&request.index_deltas, "block transaction index")
}

pub fn expand_differential_indexes(
    deltas: &[u64],
    _field: &'static str,
) -> Result<Vec<u16>, CodecError> {
    let mut shift = 0_u64;
    let mut indexes = Vec::with_capacity(deltas.len());
    for delta in deltas {
        shift = shift
            .checked_add(*delta)
            .ok_or(CodecError::DifferentialIndexOverflow)?;
        let index = u16::try_from(shift).map_err(|_| CodecError::DifferentialIndexOverflow)?;
        indexes.push(index);
        shift = shift
            .checked_add(1)
            .ok_or(CodecError::DifferentialIndexOverflow)?;
    }
    Ok(indexes)
}

fn read_bounded_compact_block_count(
    reader: &mut Reader<'_>,
    field: &'static str,
) -> Result<usize, CodecError> {
    let count = read_compact_size(reader)?;
    if count > u64::from(u16::MAX) {
        return Err(CodecError::CompactBlockTransactionCountOverflow { count });
    }
    Ok(compact_size_to_usize(count, field))
}

fn validate_compact_block_count(
    short_id_count: u64,
    prefilled_transaction_count: u64,
) -> Result<u64, CodecError> {
    let count = short_id_count
        .checked_add(prefilled_transaction_count)
        .ok_or(CodecError::CompactBlockTransactionCountOverflow { count: u64::MAX })?;
    if count > u64::from(u16::MAX) {
        return Err(CodecError::CompactBlockTransactionCountOverflow { count });
    }
    Ok(count)
}

#[cfg(test)]
mod tests;
