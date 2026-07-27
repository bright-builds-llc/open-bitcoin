// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netbase.h

use open_bitcoin_codec::{
    encode_send_compact_payload, BlockTransactions, BlockTransactionsRequest, CompactBlockPayload,
    PrefilledTransaction, SendCompactMessage, ShortId, BIP152_COMPACT_BLOCKS_VERSION,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, Hash32, InventoryType, MerkleRoot, MessageCommand,
    NetworkAddress, NetworkMagic, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid,
};

use crate::address::{AddressAnnouncement, AddressList, PHASE92_ADDR_BATCH_LIMIT};

use super::{
    zero_address, InventoryList, InventoryVector, LocalPeerConfig, ParsedNetworkMessage,
    ServiceFlags, VersionMessage, WireNetworkMessage,
};

fn sample_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([4_u8; 32]),
                vout: 1,
            },
            script_sig: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x01]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(42).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn sample_block() -> Block {
    Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([1_u8; 32]),
            time: 2,
            bits: 0x207f_ffff,
            nonce: 3,
        },
        transactions: vec![sample_transaction()],
    }
}

fn sample_compact_block_payload() -> CompactBlockPayload {
    CompactBlockPayload {
        header: sample_block().header,
        nonce: 0x0807_0605_0403_0201,
        short_ids: vec![
            ShortId::from_wire_bytes([1, 2, 3, 4, 5, 6]),
            ShortId::from_wire_bytes([7, 8, 9, 10, 11, 12]),
        ],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: sample_transaction(),
        }],
    }
}

fn decode_hex(input: &str) -> Vec<u8> {
    let trimmed = input.trim();
    assert_eq!(trimmed.len() % 2, 0, "hex fixtures must use full bytes");
    let chars = trimmed.chars().collect::<Vec<_>>();
    chars
        .chunks(2)
        .map(|pair| {
            let high = pair[0].to_digit(16).expect("fixture should be hex");
            let low = pair[1].to_digit(16).expect("fixture should be hex");
            ((high << 4) | low) as u8
        })
        .collect()
}

fn malformed_empty_cmpctblock_payload() -> Vec<u8> {
    let mut payload = open_bitcoin_codec::encode_block_header(&sample_block().header);
    payload.extend_from_slice(&1_u64.to_le_bytes());
    open_bitcoin_codec::write_compact_size(&mut payload, 0).expect("short id count");
    open_bitcoin_codec::write_compact_size(&mut payload, 0).expect("prefilled count");
    payload
}

fn malformed_getblocktxn_overflow_payload() -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(BlockHash::from_byte_array([16_u8; 32]).as_bytes());
    open_bitcoin_codec::write_compact_size(&mut payload, 1).expect("index count");
    open_bitcoin_codec::write_compact_size(&mut payload, u64::from(u16::MAX) + 1)
        .expect("index delta");
    payload
}

fn malformed_blocktxn_superfluous_witness_payload() -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(BlockHash::from_byte_array([17_u8; 32]).as_bytes());
    open_bitcoin_codec::write_compact_size(&mut payload, 1).expect("transaction count");
    payload.extend_from_slice(&decode_hex(
        "0200000000010102020202020202020202020202020202020202020202020202020202020202020100000000ffffffff012a0000000000000001510000000000",
    ));
    payload
}

fn sample_peer_address(port: u16) -> NetworkAddress {
    NetworkAddress {
        services: (ServiceFlags::NETWORK | ServiceFlags::WITNESS).bits(),
        address_bytes: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xcb, 0x00,
            0x71, 0x0a,
        ],
        port,
    }
}

mod address_message_cases;
mod compact_protocol_cases;
mod version_cases;
mod wire_codec_cases;
