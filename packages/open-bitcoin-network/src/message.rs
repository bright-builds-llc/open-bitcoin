// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netbase.h

mod cursor;

use open_bitcoin_codec::{
    BlockTransactions, BlockTransactionsRequest, CodecError, CompactBlockPayload,
    SendCompactMessage, decode_block_transactions_payload, decode_compact_block_payload,
    decode_get_block_transactions_payload, decode_send_compact_payload, encode_block,
    encode_block_header, encode_block_locator, encode_block_transactions_payload,
    encode_compact_block_payload, encode_get_block_transactions_payload, encode_inventory_vector,
    encode_message_header, encode_network_address, encode_send_compact_payload, encode_transaction,
    parse_block, parse_block_header, parse_inventory_vector, parse_message_header,
    parse_network_address, parse_transaction, write_compact_size,
};
use open_bitcoin_consensus::crypto::double_sha256;
use open_bitcoin_primitives::{
    Block, BlockHash, BlockHeader, BlockLocator, InventoryVector, MessageCommand, MessageHeader,
    NetworkAddress, NetworkMagic, Transaction,
};

use crate::address::{AddressAnnouncement, AddressList, PHASE92_ADDR_BATCH_LIMIT};
use crate::error::NetworkError;
use cursor::{Cursor, compact_size_to_usize};

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

    pub fn version_message_with_sender_policy(
        &self,
        timestamp: i64,
        start_height: i32,
        maybe_sender: Option<NetworkAddress>,
    ) -> VersionMessage {
        let mut message = self.version_message(timestamp, start_height);
        message.sender = maybe_sender.unwrap_or_else(zero_address);
        message
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
    SendCompact(SendCompactMessage),
    CompactBlock(CompactBlockPayload),
    GetBlockTxn(BlockTransactionsRequest),
    BlockTxn(BlockTransactions),
    GetAddr,
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
    Addr(AddressList),
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
            Self::SendCompact(_) => "sendcmpct",
            Self::CompactBlock(_) => "cmpctblock",
            Self::GetBlockTxn(_) => "getblocktxn",
            Self::BlockTxn(_) => "blocktxn",
            Self::GetAddr => "getaddr",
            Self::Ping { .. } => "ping",
            Self::Pong { .. } => "pong",
            Self::GetHeaders { .. } => "getheaders",
            Self::Headers(_) => "headers",
            Self::Addr(_) => "addr",
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
            Self::Verack | Self::WtxidRelay | Self::SendHeaders | Self::GetAddr => Ok(Vec::new()),
            Self::SendCompact(message) => Ok(encode_send_compact_payload(message)),
            Self::CompactBlock(message) => Ok(encode_compact_block_payload(message)?),
            Self::GetBlockTxn(message) => Ok(encode_get_block_transactions_payload(message)?),
            Self::BlockTxn(message) => Ok(encode_block_transactions_payload(message)?),
            Self::Ping { nonce } | Self::Pong { nonce } => Ok(nonce.to_le_bytes().to_vec()),
            Self::GetHeaders { locator, stop_hash } => {
                let mut payload = encode_block_locator(locator)?;
                payload.extend_from_slice(stop_hash.as_bytes());
                Ok(payload)
            }
            Self::Headers(message) => encode_headers_payload(message),
            Self::Addr(addresses) => encode_addr_payload(addresses),
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
            "verack" => decode_empty_message(payload, Self::Verack),
            "wtxidrelay" => decode_empty_message(payload, Self::WtxidRelay),
            "sendheaders" => decode_empty_message(payload, Self::SendHeaders),
            "sendcmpct" => Ok(Self::SendCompact(decode_send_compact_payload(payload)?)),
            "cmpctblock" => Ok(Self::CompactBlock(decode_compact_block_payload(payload)?)),
            #[rustfmt::skip]
            "getblocktxn" => Ok(Self::GetBlockTxn(decode_get_block_transactions_payload(payload)?)),
            "blocktxn" => Ok(Self::BlockTxn(decode_block_transactions_payload(payload)?)),
            "getaddr" => decode_empty_message(payload, Self::GetAddr),
            "ping" => Ok(Self::Ping {
                nonce: decode_nonce_payload(payload)?,
            }),
            "pong" => Ok(Self::Pong {
                nonce: decode_nonce_payload(payload)?,
            }),
            "getheaders" => decode_getheaders_payload(payload),
            "headers" => Ok(Self::Headers(decode_headers_payload(payload)?)),
            "addr" => Ok(Self::Addr(decode_addr_payload(payload)?)),
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

fn encode_addr_payload(address_list: &AddressList) -> Result<Vec<u8>, NetworkError> {
    validate_addr_count(address_list.addresses.len())?;

    let mut encoded = Vec::new();
    write_compact_size(&mut encoded, address_list.addresses.len() as u64)?;
    for announcement in &address_list.addresses {
        encoded.extend_from_slice(&announcement.time_unix_seconds.to_le_bytes());
        encoded.extend_from_slice(&encode_network_address(&announcement.address));
    }
    Ok(encoded)
}

fn decode_addr_payload(payload: &[u8]) -> Result<AddressList, NetworkError> {
    let mut cursor = Cursor::new(payload);
    let count = compact_size_to_usize(cursor.read_compact_size()?, "addr count");
    validate_addr_count(count)?;

    let mut addresses = Vec::with_capacity(count);
    for _ in 0..count {
        let time_unix_seconds = cursor.read_u32_le()?;
        let address = parse_network_address(cursor.read_slice(NETWORK_ADDRESS_LEN)?)?;
        addresses.push(AddressAnnouncement {
            time_unix_seconds,
            address,
        });
    }
    cursor.finish()?;
    Ok(AddressList { addresses })
}

fn validate_addr_count(count: usize) -> Result<(), NetworkError> {
    if count <= PHASE92_ADDR_BATCH_LIMIT {
        return Ok(());
    }

    Err(CodecError::LengthOutOfRange {
        field: "addr count",
        value: count as u64,
    }
    .into())
}

fn decode_empty_payload(payload: &[u8]) -> Result<(), NetworkError> {
    Cursor::new(payload).finish()?;
    Ok(())
}

fn decode_empty_message(
    payload: &[u8],
    message: WireNetworkMessage,
) -> Result<WireNetworkMessage, NetworkError> {
    decode_empty_payload(payload)?;
    Ok(message)
}

fn encode_inventory_payload(payload: &InventoryList) -> Result<Vec<u8>, NetworkError> {
    validate_inventory_count(payload.inventory.len())?;

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
    validate_inventory_count(count)?;

    let mut inventory = Vec::with_capacity(count);
    for _ in 0..count {
        let vector_bytes = cursor.read_slice(InventoryVector::SERIALIZED_LEN)?;
        let vector = parse_inventory_vector(vector_bytes)?;
        inventory.push(vector);
    }
    cursor.finish()?;
    Ok(InventoryList { inventory })
}

fn validate_inventory_count(count: usize) -> Result<(), NetworkError> {
    if count <= MAX_INV_SIZE {
        return Ok(());
    }

    Err(CodecError::LengthOutOfRange {
        field: "inventory count",
        value: count as u64,
    }
    .into())
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

#[cfg(test)]
mod tests;
