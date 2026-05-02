// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use core::fmt;

use open_bitcoin_codec::CodecError;
use open_bitcoin_primitives::{BlockHash, MessageCommandError};

pub type PeerId = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisconnectReason {
    DuplicateVersion,
    MissingHeaderAncestor(BlockHash),
}

impl fmt::Display for DisconnectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateVersion => write!(f, "duplicate version message"),
            Self::MissingHeaderAncestor(hash) => {
                write!(f, "missing header ancestor: {:?}", hash.to_byte_array(),)
            }
        }
    }
}

impl std::error::Error for DisconnectReason {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    Codec(CodecError),
    MessageCommand(MessageCommandError),
    UnknownCommand(String),
    InvalidChecksum,
    InvalidUserAgentEncoding,
    InvalidHeader {
        reject_reason: String,
        maybe_debug_message: Option<String>,
    },
    HeadersIncludeTransactions(u64),
    MissingHeaderAncestor(BlockHash),
    PeerAlreadyExists(PeerId),
    UnknownPeer(PeerId),
    DuplicateVersion(PeerId),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Codec(error) => error.fmt(f),
            Self::MessageCommand(error) => error.fmt(f),
            Self::UnknownCommand(command) => write!(f, "unknown network command: {command}"),
            Self::InvalidChecksum => write!(f, "invalid network payload checksum"),
            Self::InvalidUserAgentEncoding => {
                write!(f, "version message user agent is not valid UTF-8")
            }
            Self::InvalidHeader {
                reject_reason,
                maybe_debug_message,
            } => {
                if let Some(debug_message) = maybe_debug_message {
                    write!(f, "invalid header: {reject_reason} ({debug_message})")
                } else {
                    write!(f, "invalid header: {reject_reason}")
                }
            }
            Self::HeadersIncludeTransactions(count) => {
                write!(
                    f,
                    "headers message included non-zero transaction count: {count}"
                )
            }
            Self::MissingHeaderAncestor(hash) => {
                write!(f, "missing header ancestor: {:?}", hash.to_byte_array(),)
            }
            Self::PeerAlreadyExists(peer_id) => write!(f, "peer already exists: {peer_id}"),
            Self::UnknownPeer(peer_id) => write!(f, "unknown peer: {peer_id}"),
            Self::DuplicateVersion(peer_id) => {
                write!(f, "peer {peer_id} sent duplicate version message")
            }
        }
    }
}

impl std::error::Error for NetworkError {}

impl From<CodecError> for NetworkError {
    fn from(value: CodecError) -> Self {
        Self::Codec(value)
    }
}

impl From<MessageCommandError> for NetworkError {
    fn from(value: MessageCommandError) -> Self {
        Self::MessageCommand(value)
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_codec::CodecError;
    use open_bitcoin_primitives::{BlockHash, MessageCommandError};

    use super::{DisconnectReason, NetworkError};

    #[test]
    fn display_messages_are_human_readable() {
        assert_eq!(
            NetworkError::from(CodecError::TrailingData { remaining: 3 }).to_string(),
            "trailing data: 3 bytes",
        );
        assert_eq!(
            NetworkError::from(MessageCommandError::ContainsNul).to_string(),
            "message command contains NUL",
        );
        assert_eq!(
            NetworkError::UnknownCommand("mystery".to_string()).to_string(),
            "unknown network command: mystery",
        );
        assert_eq!(
            NetworkError::InvalidChecksum.to_string(),
            "invalid network payload checksum",
        );
        assert_eq!(
            NetworkError::HeadersIncludeTransactions(1).to_string(),
            "headers message included non-zero transaction count: 1",
        );
        assert_eq!(
            DisconnectReason::MissingHeaderAncestor(BlockHash::from_byte_array([7_u8; 32]))
                .to_string(),
            "missing header ancestor: [7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]",
        );
        assert_eq!(
            DisconnectReason::DuplicateVersion.to_string(),
            "duplicate version message",
        );
        assert_eq!(
            NetworkError::InvalidUserAgentEncoding.to_string(),
            "version message user agent is not valid UTF-8",
        );
        assert_eq!(
            NetworkError::InvalidHeader {
                reject_reason: "bad-diffbits".to_string(),
                maybe_debug_message: Some("incorrect proof of work".to_string()),
            }
            .to_string(),
            "invalid header: bad-diffbits (incorrect proof of work)",
        );
        assert_eq!(
            NetworkError::InvalidHeader {
                reject_reason: "bad-prevblk".to_string(),
                maybe_debug_message: None,
            }
            .to_string(),
            "invalid header: bad-prevblk",
        );
        assert_eq!(
            NetworkError::MissingHeaderAncestor(BlockHash::from_byte_array([3_u8; 32])).to_string(),
            "missing header ancestor: [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]",
        );
        assert_eq!(
            NetworkError::PeerAlreadyExists(5).to_string(),
            "peer already exists: 5",
        );
        assert_eq!(NetworkError::UnknownPeer(6).to_string(), "unknown peer: 6",);
        assert_eq!(
            NetworkError::DuplicateVersion(7).to_string(),
            "peer 7 sent duplicate version message",
        );
    }
}
