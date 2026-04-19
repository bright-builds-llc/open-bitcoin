use core::fmt;

use crate::hash::Hash32;

pub const MESSAGE_TYPE_SIZE: usize = 12;
pub const BLOCK_LOCATOR_DUMMY_VERSION: i32 = 70_016;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkMagic([u8; 4]);

impl NetworkMagic {
    pub const MAINNET: Self = Self([0xf9, 0xbe, 0xb4, 0xd9]);

    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }

    pub const fn to_bytes(self) -> [u8; 4] {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageCommandError {
    TooLong(usize),
    ContainsNul,
    InvalidWirePadding,
    NonAscii(u8),
}

impl fmt::Display for MessageCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLong(length) => write!(f, "message command too long: {length}"),
            Self::ContainsNul => write!(f, "message command contains NUL"),
            Self::InvalidWirePadding => write!(f, "message command wire padding is invalid"),
            Self::NonAscii(byte) => write!(f, "message command contains non-ascii byte: {byte}"),
        }
    }
}

impl std::error::Error for MessageCommandError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageCommand(String);

impl MessageCommand {
    pub fn new(command: impl AsRef<str>) -> Result<Self, MessageCommandError> {
        let command = command.as_ref();
        if command.len() > MESSAGE_TYPE_SIZE {
            return Err(MessageCommandError::TooLong(command.len()));
        }
        if command.bytes().any(|byte| byte == 0) {
            return Err(MessageCommandError::ContainsNul);
        }
        let Some(invalid_byte) = command.bytes().find(|byte| !byte.is_ascii()) else {
            return Ok(Self(command.to_owned()));
        };
        Err(MessageCommandError::NonAscii(invalid_byte))
    }

    pub fn from_wire_bytes(bytes: [u8; MESSAGE_TYPE_SIZE]) -> Result<Self, MessageCommandError> {
        let first_nul = bytes.iter().position(|byte| *byte == 0);
        let content_len = first_nul.unwrap_or(MESSAGE_TYPE_SIZE);
        if bytes[content_len..].iter().any(|byte| *byte != 0) {
            return Err(MessageCommandError::InvalidWirePadding);
        }
        let content = &bytes[..content_len];
        let Some(invalid_byte) = content.iter().copied().find(|byte| !byte.is_ascii()) else {
            let command = core::str::from_utf8(content)
                .expect("ASCII bytes must be valid UTF-8")
                .to_owned();
            return Self::new(command);
        };
        Err(MessageCommandError::NonAscii(invalid_byte))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_wire_bytes(&self) -> [u8; MESSAGE_TYPE_SIZE] {
        let mut bytes = [0_u8; MESSAGE_TYPE_SIZE];
        bytes[..self.0.len()].copy_from_slice(self.0.as_bytes());
        bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageHeader {
    pub magic: NetworkMagic,
    pub command: MessageCommand,
    pub payload_size: u32,
    pub checksum: [u8; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryType {
    Error,
    Transaction,
    Block,
    FilteredBlock,
    CompactBlock,
    WitnessTransaction,
    WitnessBlock,
    Unknown(u32),
}

impl InventoryType {
    pub const fn from_raw(value: u32) -> Self {
        match value {
            0 => Self::Error,
            1 => Self::Transaction,
            2 => Self::Block,
            3 => Self::FilteredBlock,
            4 => Self::CompactBlock,
            0x4000_0001 => Self::WitnessTransaction,
            0x4000_0002 => Self::WitnessBlock,
            unknown => Self::Unknown(unknown),
        }
    }

    pub const fn to_raw(self) -> u32 {
        match self {
            Self::Error => 0,
            Self::Transaction => 1,
            Self::Block => 2,
            Self::FilteredBlock => 3,
            Self::CompactBlock => 4,
            Self::WitnessTransaction => 0x4000_0001,
            Self::WitnessBlock => 0x4000_0002,
            Self::Unknown(value) => value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryVector {
    pub inventory_type: InventoryType,
    pub object_hash: Hash32,
}

impl InventoryVector {
    pub const SERIALIZED_LEN: usize = core::mem::size_of::<u32>() + Hash32::LEN;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkAddress {
    pub services: u64,
    pub address_bytes: [u8; 16],
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BlockLocator {
    pub block_hashes: Vec<Hash32>,
}

#[cfg(test)]
mod tests {
    use super::{InventoryType, MessageCommand, MessageCommandError, NetworkMagic};

    #[test]
    fn message_command_round_trips_to_wire_bytes() {
        let command = MessageCommand::new("version").expect("valid command");
        let wire = command.to_wire_bytes();

        assert_eq!(
            MessageCommand::from_wire_bytes(wire).expect("valid wire bytes"),
            command,
        );
    }

    #[test]
    fn message_command_rejects_too_long_and_bad_padding() {
        assert_eq!(
            MessageCommand::new("abcdefghijklmn"),
            Err(MessageCommandError::TooLong(14)),
        );

        let mut wire = MessageCommand::new("ping")
            .expect("valid command")
            .to_wire_bytes();
        wire[6] = b'x';
        assert_eq!(
            MessageCommand::from_wire_bytes(wire),
            Err(MessageCommandError::InvalidWirePadding),
        );
    }

    #[test]
    fn inventory_type_preserves_unknown_values() {
        assert_eq!(InventoryType::from_raw(0).to_raw(), 0);
        assert_eq!(InventoryType::from_raw(1).to_raw(), 1);
        assert_eq!(InventoryType::from_raw(2).to_raw(), 2);
        assert_eq!(InventoryType::from_raw(3).to_raw(), 3);
        assert_eq!(InventoryType::from_raw(4).to_raw(), 4);
        assert_eq!(InventoryType::from_raw(0x4000_0001).to_raw(), 0x4000_0001);
        assert_eq!(InventoryType::from_raw(0x4000_0002).to_raw(), 0x4000_0002);
        assert_eq!(InventoryType::from_raw(99).to_raw(), 99);
        assert_eq!(NetworkMagic::MAINNET.to_bytes(), [0xf9, 0xbe, 0xb4, 0xd9]);
        assert_eq!(
            NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]).as_bytes(),
            &[0xfa, 0xbf, 0xb5, 0xda],
        );
    }

    #[test]
    fn message_command_rejects_nul_and_non_ascii_bytes() {
        assert_eq!(
            MessageCommand::new("ver\0sion"),
            Err(MessageCommandError::ContainsNul),
        );
        assert_eq!(
            MessageCommand::new("vérsion"),
            Err(MessageCommandError::NonAscii(195)),
        );
    }

    #[test]
    fn message_command_exposes_plain_text_value() {
        let command = MessageCommand::new("ping").expect("valid command");

        assert_eq!(command.as_str(), "ping");
    }

    #[test]
    fn message_command_error_messages_are_human_readable() {
        assert_eq!(
            MessageCommandError::TooLong(14).to_string(),
            "message command too long: 14",
        );
        assert_eq!(
            MessageCommandError::ContainsNul.to_string(),
            "message command contains NUL",
        );
        assert_eq!(
            MessageCommandError::InvalidWirePadding.to_string(),
            "message command wire padding is invalid",
        );
        assert_eq!(
            MessageCommandError::NonAscii(255).to_string(),
            "message command contains non-ascii byte: 255",
        );
    }

    #[test]
    fn message_command_from_wire_bytes_rejects_non_ascii_bytes() {
        let mut wire = [0_u8; super::MESSAGE_TYPE_SIZE];
        wire[0] = 0xff;

        assert_eq!(
            MessageCommand::from_wire_bytes(wire),
            Err(MessageCommandError::NonAscii(255)),
        );
    }
}
