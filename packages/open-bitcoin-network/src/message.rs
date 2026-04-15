use open_bitcoin_codec::{
    CodecError, MAX_SIZE, encode_block, encode_block_header, encode_block_locator,
    encode_inventory_vector, encode_message_header, encode_network_address, encode_transaction,
    parse_block, parse_block_header, parse_inventory_vector, parse_message_header,
    parse_network_address, parse_transaction, write_compact_size,
};
use open_bitcoin_consensus::crypto::double_sha256;
use open_bitcoin_primitives::{
    Block, BlockHash, BlockHeader, BlockLocator, InventoryVector, MessageCommand, MessageHeader,
    NetworkAddress, NetworkMagic, Transaction,
};

use crate::error::NetworkError;

pub const PROTOCOL_VERSION: i32 = 70_016;
pub const USER_AGENT: &str = "/open-bitcoin:0.1.0/";
pub const MAX_HEADERS_RESULTS: usize = 2_000;
pub const MAX_INV_SIZE: usize = 50_000;
const NETWORK_ADDRESS_LEN: usize = 26;
const BLOCK_HEADER_LEN: usize = 80;
const MESSAGE_HEADER_LEN: usize = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ServiceFlags(u64);

impl ServiceFlags {
    pub const NONE: Self = Self(0);
    pub const NETWORK: Self = Self(1 << 0);
    pub const WITNESS: Self = Self(1 << 3);
    pub const REPLACE_BY_FEE: Self = Self(1 << 26);

    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for ServiceFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ServiceFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionMessage {
    pub version: i32,
    pub services: ServiceFlags,
    pub timestamp: i64,
    pub receiver: NetworkAddress,
    pub sender: NetworkAddress,
    pub nonce: u64,
    pub user_agent: String,
    pub start_height: i32,
    pub relay: bool,
}

impl Default for VersionMessage {
    fn default() -> Self {
        Self {
            version: PROTOCOL_VERSION,
            services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
            timestamp: 0,
            receiver: zero_address(),
            sender: zero_address(),
            nonce: 0,
            user_agent: USER_AGENT.to_string(),
            start_height: -1,
            relay: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalPeerConfig {
    pub magic: NetworkMagic,
    pub services: ServiceFlags,
    pub address: NetworkAddress,
    pub nonce: u64,
    pub relay: bool,
    pub user_agent: String,
}

impl LocalPeerConfig {
    pub fn version_message(&self, timestamp: i64, start_height: i32) -> VersionMessage {
        VersionMessage {
            version: PROTOCOL_VERSION,
            services: self.services,
            timestamp,
            receiver: self.address.clone(),
            sender: self.address.clone(),
            nonce: self.nonce,
            user_agent: self.user_agent.clone(),
            start_height,
            relay: self.relay,
        }
    }
}

impl Default for LocalPeerConfig {
    fn default() -> Self {
        Self {
            magic: NetworkMagic::MAINNET,
            services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
            address: zero_address(),
            nonce: 0,
            relay: true,
            user_agent: USER_AGENT.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InventoryList {
    pub inventory: Vec<InventoryVector>,
}

impl InventoryList {
    pub fn new(inventory: Vec<InventoryVector>) -> Self {
        Self { inventory }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HeadersMessage {
    pub headers: Vec<BlockHeader>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WireNetworkMessage {
    Version(VersionMessage),
    Verack,
    WtxidRelay,
    SendHeaders,
    Ping {
        nonce: u64,
    },
    Pong {
        nonce: u64,
    },
    GetHeaders {
        locator: BlockLocator,
        stop_hash: BlockHash,
    },
    Headers(HeadersMessage),
    Inv(InventoryList),
    GetData(InventoryList),
    NotFound(InventoryList),
    Tx(Transaction),
    Block(Block),
}

impl WireNetworkMessage {
    pub fn command_name(&self) -> &'static str {
        match self {
            Self::Version(_) => "version",
            Self::Verack => "verack",
            Self::WtxidRelay => "wtxidrelay",
            Self::SendHeaders => "sendheaders",
            Self::Ping { .. } => "ping",
            Self::Pong { .. } => "pong",
            Self::GetHeaders { .. } => "getheaders",
            Self::Headers(_) => "headers",
            Self::Inv(_) => "inv",
            Self::GetData(_) => "getdata",
            Self::NotFound(_) => "notfound",
            Self::Tx(_) => "tx",
            Self::Block(_) => "block",
        }
    }

    pub fn encode_payload(&self) -> Result<Vec<u8>, NetworkError> {
        match self {
            Self::Version(message) => encode_version_payload(message),
            Self::Verack | Self::WtxidRelay | Self::SendHeaders => Ok(Vec::new()),
            Self::Ping { nonce } | Self::Pong { nonce } => Ok(nonce.to_le_bytes().to_vec()),
            Self::GetHeaders { locator, stop_hash } => {
                let mut payload = encode_block_locator(locator)?;
                payload.extend_from_slice(stop_hash.as_bytes());
                Ok(payload)
            }
            Self::Headers(message) => encode_headers_payload(message),
            Self::Inv(inventory) | Self::GetData(inventory) | Self::NotFound(inventory) => {
                encode_inventory_payload(inventory)
            }
            Self::Tx(transaction) => encode_transaction(
                transaction,
                open_bitcoin_codec::TransactionEncoding::WithWitness,
            )
            .map_err(NetworkError::from),
            Self::Block(block) => Ok(encode_block(block)?),
        }
    }

    pub fn command(&self) -> Result<MessageCommand, NetworkError> {
        Ok(MessageCommand::new(self.command_name())?)
    }

    pub fn encode_wire(&self, magic: NetworkMagic) -> Result<Vec<u8>, NetworkError> {
        let payload = self.encode_payload()?;
        let checksum = checksum(&payload);
        debug_assert!(payload.len() <= u32::MAX as usize);
        let header = MessageHeader {
            magic,
            command: self.command()?,
            payload_size: payload.len() as u32,
            checksum,
        };
        let mut encoded = encode_message_header(&header);
        encoded.extend_from_slice(&payload);
        Ok(encoded)
    }

    pub fn decode_payload(command: &MessageCommand, payload: &[u8]) -> Result<Self, NetworkError> {
        match command.as_str() {
            "version" => Ok(Self::Version(decode_version_payload(payload)?)),
            "verack" => Ok(Self::Verack),
            "wtxidrelay" => Ok(Self::WtxidRelay),
            "sendheaders" => Ok(Self::SendHeaders),
            "ping" => Ok(Self::Ping {
                nonce: decode_nonce_payload(payload)?,
            }),
            "pong" => Ok(Self::Pong {
                nonce: decode_nonce_payload(payload)?,
            }),
            "getheaders" => decode_getheaders_payload(payload),
            "headers" => Ok(Self::Headers(decode_headers_payload(payload)?)),
            "inv" => Ok(Self::Inv(decode_inventory_payload(payload)?)),
            "getdata" => Ok(Self::GetData(decode_inventory_payload(payload)?)),
            "notfound" => Ok(Self::NotFound(decode_inventory_payload(payload)?)),
            "tx" => Ok(Self::Tx(parse_transaction(payload)?)),
            "block" => Ok(Self::Block(parse_block(payload)?)),
            other => Err(NetworkError::UnknownCommand(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedNetworkMessage {
    pub header: MessageHeader,
    pub message: WireNetworkMessage,
}

impl ParsedNetworkMessage {
    pub fn decode_wire(bytes: &[u8]) -> Result<Self, NetworkError> {
        let Some(header_bytes) = bytes.get(..MESSAGE_HEADER_LEN) else {
            return Err(CodecError::UnexpectedEof {
                needed: MESSAGE_HEADER_LEN,
                remaining: bytes.len(),
            }
            .into());
        };
        let header = parse_message_header(header_bytes)?;
        let expected_payload_len = header.payload_size as usize;
        let payload = bytes
            .get(MESSAGE_HEADER_LEN..)
            .ok_or(CodecError::UnexpectedEof {
                needed: expected_payload_len,
                remaining: 0,
            })?;
        if payload.len() != expected_payload_len {
            return Err(CodecError::LengthOutOfRange {
                field: "payload size",
                value: payload.len() as u64,
            }
            .into());
        }
        if checksum(payload) != header.checksum {
            return Err(NetworkError::InvalidChecksum);
        }
        let message = WireNetworkMessage::decode_payload(&header.command, payload)?;
        Ok(Self { header, message })
    }
}

fn encode_version_payload(message: &VersionMessage) -> Result<Vec<u8>, NetworkError> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&message.version.to_le_bytes());
    payload.extend_from_slice(&message.services.bits().to_le_bytes());
    payload.extend_from_slice(&message.timestamp.to_le_bytes());
    payload.extend_from_slice(&encode_network_address(&message.receiver));
    payload.extend_from_slice(&encode_network_address(&message.sender));
    payload.extend_from_slice(&message.nonce.to_le_bytes());
    write_compact_size(&mut payload, message.user_agent.len() as u64)?;
    payload.extend_from_slice(message.user_agent.as_bytes());
    payload.extend_from_slice(&message.start_height.to_le_bytes());
    payload.push(u8::from(message.relay));
    Ok(payload)
}

fn decode_version_payload(payload: &[u8]) -> Result<VersionMessage, NetworkError> {
    let mut cursor = Cursor::new(payload);
    let version = cursor.read_i32_le()?;
    let services = ServiceFlags::from_bits(cursor.read_u64_le()?);
    let timestamp = cursor.read_i64_le()?;
    let receiver = parse_network_address(cursor.read_slice(NETWORK_ADDRESS_LEN)?)?;
    let sender = parse_network_address(cursor.read_slice(NETWORK_ADDRESS_LEN)?)?;
    let nonce = cursor.read_u64_le()?;
    let user_agent_len = compact_size_to_usize(cursor.read_compact_size()?, "user agent length");
    let user_agent_bytes = cursor.read_slice(user_agent_len)?.to_vec();
    let user_agent =
        String::from_utf8(user_agent_bytes).map_err(|_| NetworkError::InvalidUserAgentEncoding)?;
    let start_height = cursor.read_i32_le()?;
    let relay = if cursor.remaining() == 0 {
        false
    } else {
        cursor.read_u8()? != 0
    };
    cursor.finish()?;
    Ok(VersionMessage {
        version,
        services,
        timestamp,
        receiver,
        sender,
        nonce,
        user_agent,
        start_height,
        relay,
    })
}

fn encode_headers_payload(message: &HeadersMessage) -> Result<Vec<u8>, NetworkError> {
    let mut payload = Vec::new();
    write_compact_size(&mut payload, message.headers.len() as u64)?;
    for header in &message.headers {
        payload.extend_from_slice(&encode_block_header(header));
        write_compact_size(&mut payload, 0)?;
    }
    Ok(payload)
}

fn decode_headers_payload(payload: &[u8]) -> Result<HeadersMessage, NetworkError> {
    let mut cursor = Cursor::new(payload);
    let count = compact_size_to_usize(cursor.read_compact_size()?, "headers count");
    if count > MAX_HEADERS_RESULTS {
        return Err(CodecError::LengthOutOfRange {
            field: "headers count",
            value: count as u64,
        }
        .into());
    }

    let mut headers = Vec::with_capacity(count);
    for _ in 0..count {
        let header = parse_block_header(cursor.read_slice(BLOCK_HEADER_LEN)?)?;
        let transaction_count = cursor.read_compact_size()?;
        if transaction_count != 0 {
            return Err(NetworkError::HeadersIncludeTransactions(transaction_count));
        }
        headers.push(header);
    }
    cursor.finish()?;
    Ok(HeadersMessage { headers })
}

fn encode_inventory_payload(payload: &InventoryList) -> Result<Vec<u8>, NetworkError> {
    let mut encoded = Vec::new();
    write_compact_size(&mut encoded, payload.inventory.len() as u64)?;
    for inventory in &payload.inventory {
        encoded.extend_from_slice(&encode_inventory_vector(inventory));
    }
    Ok(encoded)
}

fn decode_inventory_payload(payload: &[u8]) -> Result<InventoryList, NetworkError> {
    let mut cursor = Cursor::new(payload);
    let count = compact_size_to_usize(cursor.read_compact_size()?, "inventory count");
    if count > MAX_INV_SIZE {
        return Err(CodecError::LengthOutOfRange {
            field: "inventory count",
            value: count as u64,
        }
        .into());
    }

    let mut inventory = Vec::with_capacity(count);
    for _ in 0..count {
        inventory.push(parse_inventory_vector(cursor.read_slice(36)?)?);
    }
    cursor.finish()?;
    Ok(InventoryList { inventory })
}

fn decode_getheaders_payload(payload: &[u8]) -> Result<WireNetworkMessage, NetworkError> {
    let mut cursor = Cursor::new(payload);
    let _dummy_version = cursor.read_i32_le()?;
    let count = compact_size_to_usize(cursor.read_compact_size()?, "locator count");
    let mut hashes = Vec::with_capacity(count);
    for _ in 0..count {
        hashes.push(open_bitcoin_primitives::Hash32::from_byte_array(
            cursor.read_array::<32>()?,
        ));
    }
    let stop_hash = BlockHash::from_byte_array(cursor.read_array::<32>()?);
    cursor.finish()?;
    Ok(WireNetworkMessage::GetHeaders {
        locator: BlockLocator {
            block_hashes: hashes,
        },
        stop_hash,
    })
}

fn decode_nonce_payload(payload: &[u8]) -> Result<u64, NetworkError> {
    let mut cursor = Cursor::new(payload);
    let nonce = cursor.read_u64_le()?;
    cursor.finish()?;
    Ok(nonce)
}

fn checksum(payload: &[u8]) -> [u8; 4] {
    let digest = double_sha256(payload);
    [digest[0], digest[1], digest[2], digest[3]]
}

pub(crate) fn zero_address() -> NetworkAddress {
    NetworkAddress {
        services: 0,
        address_bytes: [0_u8; 16],
        port: 0,
    }
}

#[derive(Debug, Clone)]
struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }

    fn finish(self) -> Result<(), CodecError> {
        if self.remaining() == 0 {
            return Ok(());
        }
        Err(CodecError::TrailingData {
            remaining: self.remaining(),
        })
    }

    fn read_u8(&mut self) -> Result<u8, CodecError> {
        Ok(self.read_array::<1>()?[0])
    }

    fn read_u64_le(&mut self) -> Result<u64, CodecError> {
        Ok(u64::from_le_bytes(self.read_array()?))
    }

    fn read_i32_le(&mut self) -> Result<i32, CodecError> {
        Ok(i32::from_le_bytes(self.read_array()?))
    }

    fn read_i64_le(&mut self) -> Result<i64, CodecError> {
        Ok(i64::from_le_bytes(self.read_array()?))
    }

    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], CodecError> {
        let slice = self.read_slice(N)?;
        let array = <[u8; N]>::try_from(slice).expect("slice length already validated");
        Ok(array)
    }

    fn read_slice(&mut self, len: usize) -> Result<&'a [u8], CodecError> {
        let remaining = self.remaining();
        if remaining < len {
            return Err(CodecError::UnexpectedEof {
                needed: len,
                remaining,
            });
        }
        let start = self.offset;
        self.offset += len;
        Ok(&self.bytes[start..self.offset])
    }

    fn read_compact_size(&mut self) -> Result<u64, CodecError> {
        let first = self.read_u8()?;
        let value = match first {
            value @ 0..=252 => u64::from(value),
            0xfd => {
                let value = u64::from(u16::from_le_bytes(self.read_array()?));
                if value < 253 {
                    return Err(CodecError::NonCanonicalCompactSize { value });
                }
                value
            }
            0xfe => {
                let value = u64::from(u32::from_le_bytes(self.read_array()?));
                if value <= u64::from(u16::MAX) {
                    return Err(CodecError::NonCanonicalCompactSize { value });
                }
                value
            }
            0xff => {
                let value = u64::from_le_bytes(self.read_array()?);
                if value <= u64::from(u32::MAX) {
                    return Err(CodecError::NonCanonicalCompactSize { value });
                }
                value
            }
        };

        if value > MAX_SIZE {
            return Err(CodecError::CompactSizeTooLarge(value));
        }
        Ok(value)
    }
}

fn compact_size_to_usize(value: u64, field: &'static str) -> usize {
    debug_assert!(
        value <= usize::MAX as u64,
        "{field} does not fit into usize"
    );
    value as usize
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{
        Amount, Block, BlockHash, BlockHeader, Hash32, InventoryType, MerkleRoot, MessageCommand,
        NetworkMagic, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput, Txid,
    };

    use super::{
        InventoryList, InventoryVector, LocalPeerConfig, ParsedNetworkMessage, ServiceFlags,
        VersionMessage, WireNetworkMessage, zero_address,
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

    #[test]
    fn wire_message_round_trips_version_and_inventory_payloads() {
        let version = WireNetworkMessage::Version(VersionMessage {
            timestamp: 1_700_000_000,
            nonce: 42,
            services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
            receiver: zero_address(),
            sender: zero_address(),
            user_agent: "/open-bitcoin:test/".to_string(),
            start_height: 7,
            relay: true,
            ..VersionMessage::default()
        });

        let encoded = version
            .encode_wire(NetworkMagic::MAINNET)
            .expect("version message should encode");
        let decoded = ParsedNetworkMessage::decode_wire(&encoded).expect("decode");
        assert_eq!(decoded.message, version);
        assert_eq!(decoded.header.magic, NetworkMagic::MAINNET);

        let inventory = WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: Hash32::from_byte_array([9_u8; 32]),
        }]));
        let encoded_inventory = inventory
            .encode_wire(NetworkMagic::MAINNET)
            .expect("inventory should encode");
        let decoded_inventory =
            ParsedNetworkMessage::decode_wire(&encoded_inventory).expect("inventory decode");
        assert_eq!(decoded_inventory.message, inventory);
        assert_eq!(
            WireNetworkMessage::decode_payload(
                &MessageCommand::new("mystery").expect("command"),
                &[],
            )
            .expect_err("unknown command")
            .to_string(),
            "unknown network command: mystery",
        );
    }

    #[test]
    fn local_peer_config_builds_expected_version_message() {
        let config = LocalPeerConfig {
            magic: NetworkMagic::MAINNET,
            services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
            address: zero_address(),
            nonce: 5,
            relay: false,
            user_agent: "/open-bitcoin:test/".to_string(),
        };

        let version = config.version_message(9, 3);
        assert_eq!(version.timestamp, 9);
        assert_eq!(version.start_height, 3);
        assert!(!version.relay);
        assert_eq!(version.nonce, 5);
    }

    #[test]
    fn service_flags_defaults_and_remaining_payload_variants_are_covered() {
        let mut flags = ServiceFlags::NONE;
        flags |= ServiceFlags::NETWORK;
        flags |= ServiceFlags::WITNESS;
        assert!(flags.contains(ServiceFlags::NETWORK));
        assert!(flags.contains(ServiceFlags::WITNESS));
        assert!(!flags.contains(ServiceFlags::REPLACE_BY_FEE));

        let default_config = LocalPeerConfig::default();
        assert_eq!(default_config.magic, NetworkMagic::MAINNET);
        assert!(default_config.services.contains(ServiceFlags::NETWORK));
        assert!(default_config.services.contains(ServiceFlags::WITNESS));

        let messages = vec![
            WireNetworkMessage::Verack,
            WireNetworkMessage::WtxidRelay,
            WireNetworkMessage::SendHeaders,
            WireNetworkMessage::Ping { nonce: 9 },
            WireNetworkMessage::Pong { nonce: 8 },
            WireNetworkMessage::GetHeaders {
                locator: open_bitcoin_primitives::BlockLocator {
                    block_hashes: vec![Hash32::from_byte_array([2_u8; 32])],
                },
                stop_hash: BlockHash::from_byte_array([3_u8; 32]),
            },
            WireNetworkMessage::Headers(super::HeadersMessage {
                headers: vec![sample_block().header.clone()],
            }),
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: Hash32::from_byte_array([7_u8; 32]),
            }])),
            WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: Hash32::from_byte_array([8_u8; 32]),
            }])),
            WireNetworkMessage::Tx(sample_transaction()),
            WireNetworkMessage::Block(sample_block()),
        ];

        for message in messages {
            let payload = message.encode_payload().expect("payload");
            let decoded =
                WireNetworkMessage::decode_payload(&message.command().expect("command"), &payload)
                    .expect("decode payload");
            assert_eq!(decoded, message);
            let wire = message.encode_wire(NetworkMagic::MAINNET).expect("wire");
            let parsed = ParsedNetworkMessage::decode_wire(&wire).expect("decode wire");
            assert_eq!(parsed.message, message);
        }

        let tx_payload = WireNetworkMessage::Tx(sample_transaction())
            .encode_payload()
            .expect("tx payload");
        assert!(!tx_payload.is_empty());
    }

    #[test]
    fn wire_decode_and_cursor_error_paths_are_exercised() {
        assert_eq!(
            ParsedNetworkMessage::decode_wire(&[0_u8; 3])
                .expect_err("short header must fail")
                .to_string(),
            "unexpected EOF: needed 24 bytes, remaining 3",
        );

        let good = WireNetworkMessage::Ping { nonce: 5 }
            .encode_wire(NetworkMagic::MAINNET)
            .expect("wire");
        let mut bad_size = good.clone();
        bad_size[16..20].copy_from_slice(&(9_u32).to_le_bytes());
        assert_eq!(
            ParsedNetworkMessage::decode_wire(&bad_size)
                .expect_err("payload size mismatch must fail")
                .to_string(),
            "payload size length out of range: 8",
        );

        let mut bad_checksum = good;
        *bad_checksum.last_mut().expect("payload byte") ^= 0x01;
        assert_eq!(
            ParsedNetworkMessage::decode_wire(&bad_checksum)
                .expect_err("checksum mismatch must fail")
                .to_string(),
            "invalid network payload checksum",
        );

        let mut relay_optional =
            super::encode_version_payload(&VersionMessage::default()).expect("payload");
        relay_optional.pop();
        let decoded = super::decode_version_payload(&relay_optional).expect("optional relay");
        assert!(!decoded.relay);

        let invalid_user_agent = {
            let mut payload = Vec::new();
            payload.extend_from_slice(&super::PROTOCOL_VERSION.to_le_bytes());
            payload.extend_from_slice(&0_u64.to_le_bytes());
            payload.extend_from_slice(&0_i64.to_le_bytes());
            payload.extend_from_slice(&open_bitcoin_codec::encode_network_address(&zero_address()));
            payload.extend_from_slice(&open_bitcoin_codec::encode_network_address(&zero_address()));
            payload.extend_from_slice(&0_u64.to_le_bytes());
            open_bitcoin_codec::write_compact_size(&mut payload, 1).expect("compact size");
            payload.push(0xff);
            payload.extend_from_slice(&0_i32.to_le_bytes());
            payload
        };
        assert_eq!(
            super::decode_version_payload(&invalid_user_agent)
                .expect_err("invalid user agent encoding must fail")
                .to_string(),
            "version message user agent is not valid UTF-8",
        );

        let mut cursor = super::Cursor::new(&[0x01, 0x02]);
        assert_eq!(
            cursor
                .read_slice(3)
                .expect_err("read past end must fail")
                .to_string(),
            "unexpected EOF: needed 3 bytes, remaining 2",
        );
        let cursor = super::Cursor::new(&[0x01]);
        assert_eq!(
            cursor
                .finish()
                .expect_err("trailing data must fail")
                .to_string(),
            "trailing data: 1 bytes",
        );
        let mut cursor = super::Cursor::new(&[0xfd, 0x01, 0x00]);
        assert_eq!(
            cursor
                .read_compact_size()
                .expect_err("non-canonical compact size")
                .to_string(),
            "non-canonical compact size for value 1",
        );
        let mut cursor = super::Cursor::new(&[0xfe, 0x01, 0x00, 0x00, 0x00]);
        assert_eq!(
            cursor
                .read_compact_size()
                .expect_err("non-canonical compact size")
                .to_string(),
            "non-canonical compact size for value 1",
        );
        let mut cursor = super::Cursor::new(&[0xfe, 0x00, 0x00, 0x01, 0x00]);
        assert_eq!(cursor.read_compact_size().expect("canonical value"), 65_536);
        let mut cursor =
            super::Cursor::new(&[0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(
            cursor
                .read_compact_size()
                .expect_err("non-canonical compact size")
                .to_string(),
            "non-canonical compact size for value 1",
        );
        let mut cursor = super::Cursor::new(&[0xff, 0, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(
            cursor
                .read_compact_size()
                .expect_err("oversized compact size")
                .to_string(),
            "compact size too large: 4294967296",
        );
    }

    #[test]
    fn decode_helpers_cover_headers_inventory_and_nonce_failures() {
        let mut headers_payload = Vec::new();
        open_bitcoin_codec::write_compact_size(&mut headers_payload, 1).expect("count");
        headers_payload.extend_from_slice(&open_bitcoin_codec::encode_block_header(
            &sample_block().header,
        ));
        open_bitcoin_codec::write_compact_size(&mut headers_payload, 1).expect("txn count");
        assert_eq!(
            super::decode_headers_payload(&headers_payload)
                .expect_err("headers payload with txns must fail")
                .to_string(),
            "headers message included non-zero transaction count: 1",
        );

        let mut too_many_headers = Vec::new();
        open_bitcoin_codec::write_compact_size(
            &mut too_many_headers,
            (super::MAX_HEADERS_RESULTS + 1) as u64,
        )
        .expect("count");
        assert_eq!(
            super::decode_headers_payload(&too_many_headers)
                .expect_err("header count overflow")
                .to_string(),
            "headers count length out of range: 2001",
        );

        let mut too_many_inventory = Vec::new();
        open_bitcoin_codec::write_compact_size(
            &mut too_many_inventory,
            (super::MAX_INV_SIZE + 1) as u64,
        )
        .expect("count");
        assert_eq!(
            super::decode_inventory_payload(&too_many_inventory)
                .expect_err("inventory count overflow")
                .to_string(),
            "inventory count length out of range: 50001",
        );
        let encoded_tx = WireNetworkMessage::Tx(sample_transaction())
            .encode_payload()
            .expect("tx payload");
        assert!(matches!(
            WireNetworkMessage::decode_payload(
                &MessageCommand::new("tx").expect("command"),
                &encoded_tx,
            )
            .expect("decode tx"),
            WireNetworkMessage::Tx(_)
        ));

        assert_eq!(
            super::decode_nonce_payload(&[1, 0, 0, 0, 0, 0, 0, 0, 1])
                .expect_err("trailing nonce payload must fail")
                .to_string(),
            "trailing data: 1 bytes",
        );
    }
}
