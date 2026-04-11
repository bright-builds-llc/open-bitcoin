use core::fmt;

use open_bitcoin_primitives::{
    AmountError,
    HashLengthError,
    MessageCommandError,
    ScriptError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodecError {
    UnexpectedEof { needed: usize, remaining: usize },
    LengthOutOfRange { field: &'static str, value: u64 },
    CompactSizeTooLarge(u64),
    NonCanonicalCompactSize { value: u64 },
    TrailingData { remaining: usize },
    InvalidWitnessFlag(u8),
    SuperfluousWitnessRecord,
    Amount(AmountError),
    HashLength(HashLengthError),
    Script(ScriptError),
    MessageCommand(MessageCommandError),
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof { needed, remaining } => {
                write!(f, "unexpected EOF: needed {needed} bytes, remaining {remaining}")
            }
            Self::LengthOutOfRange { field, value } => {
                write!(f, "{field} length out of range: {value}")
            }
            Self::CompactSizeTooLarge(value) => write!(f, "compact size too large: {value}"),
            Self::NonCanonicalCompactSize { value } => {
                write!(f, "non-canonical compact size for value {value}")
            }
            Self::TrailingData { remaining } => write!(f, "trailing data: {remaining} bytes"),
            Self::InvalidWitnessFlag(flag) => write!(f, "invalid witness flag: {flag}"),
            Self::SuperfluousWitnessRecord => write!(f, "superfluous witness record"),
            Self::Amount(error) => error.fmt(f),
            Self::HashLength(error) => error.fmt(f),
            Self::Script(error) => error.fmt(f),
            Self::MessageCommand(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CodecError {}

impl From<AmountError> for CodecError {
    fn from(value: AmountError) -> Self {
        Self::Amount(value)
    }
}

impl From<HashLengthError> for CodecError {
    fn from(value: HashLengthError) -> Self {
        Self::HashLength(value)
    }
}

impl From<ScriptError> for CodecError {
    fn from(value: ScriptError) -> Self {
        Self::Script(value)
    }
}

impl From<MessageCommandError> for CodecError {
    fn from(value: MessageCommandError) -> Self {
        Self::MessageCommand(value)
    }
}
