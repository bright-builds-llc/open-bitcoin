use core::fmt;

use open_bitcoin_primitives::{AmountError, HashLengthError, MessageCommandError, ScriptError};

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
                write!(
                    f,
                    "unexpected EOF: needed {needed} bytes, remaining {remaining}"
                )
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

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{AmountError, HashLengthError, MessageCommandError, ScriptError};

    use super::CodecError;

    #[test]
    fn display_messages_are_human_readable() {
        assert_eq!(
            CodecError::UnexpectedEof {
                needed: 4,
                remaining: 2,
            }
            .to_string(),
            "unexpected EOF: needed 4 bytes, remaining 2",
        );
        assert_eq!(
            CodecError::CompactSizeTooLarge(33_554_433).to_string(),
            "compact size too large: 33554433",
        );
        assert_eq!(
            CodecError::LengthOutOfRange {
                field: "locator",
                value: 99,
            }
            .to_string(),
            "locator length out of range: 99",
        );
        assert_eq!(
            CodecError::NonCanonicalCompactSize { value: 1 }.to_string(),
            "non-canonical compact size for value 1",
        );
        assert_eq!(
            CodecError::TrailingData { remaining: 3 }.to_string(),
            "trailing data: 3 bytes",
        );
        assert_eq!(
            CodecError::InvalidWitnessFlag(2).to_string(),
            "invalid witness flag: 2",
        );
        assert_eq!(
            CodecError::SuperfluousWitnessRecord.to_string(),
            "superfluous witness record",
        );
        assert_eq!(
            CodecError::from(AmountError::OutOfRange(-1)).to_string(),
            "amount out of range: -1",
        );
        assert_eq!(
            CodecError::from(HashLengthError {
                expected: 32,
                actual: 31,
            })
            .to_string(),
            "invalid hash length: expected 32, got 31",
        );
        assert_eq!(
            CodecError::from(ScriptError::TooLarge(10_001)).to_string(),
            "script too large: 10001",
        );
        assert_eq!(
            CodecError::from(MessageCommandError::ContainsNul).to_string(),
            "message command contains NUL",
        );
    }
}
