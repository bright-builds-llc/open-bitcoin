use core::fmt;

use open_bitcoin_primitives::OutPoint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletError {
    DescriptorSyntax(String),
    UnsupportedDescriptor(String),
    InvalidHex(String),
    InvalidBase58(String),
    InvalidChecksum,
    InvalidPrivateKey,
    InvalidPublicKey,
    InvalidXOnlyPublicKey,
    NetworkMismatch {
        expected: String,
        actual: String,
    },
    MissingSigningKey(String),
    UnsupportedSigningDescriptor(String),
    DuplicateLabel(String),
    UnknownDescriptor(u32),
    NoRecipients,
    NoSpendableCoins,
    ChangeDescriptorRequired,
    InsufficientFunds {
        needed_sats: i64,
        available_sats: i64,
    },
    Codec(String),
    Script(String),
    Amount(String),
    TaprootTweak,
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DescriptorSyntax(message) => write!(f, "descriptor syntax error: {message}"),
            Self::UnsupportedDescriptor(message) => write!(f, "unsupported descriptor: {message}"),
            Self::InvalidHex(message) => write!(f, "invalid hex: {message}"),
            Self::InvalidBase58(message) => write!(f, "invalid base58: {message}"),
            Self::InvalidChecksum => write!(f, "checksum mismatch"),
            Self::InvalidPrivateKey => write!(f, "invalid private key"),
            Self::InvalidPublicKey => write!(f, "invalid public key"),
            Self::InvalidXOnlyPublicKey => write!(f, "invalid x-only public key"),
            Self::NetworkMismatch { expected, actual } => {
                write!(f, "network mismatch: expected {expected}, got {actual}")
            }
            Self::MissingSigningKey(descriptor) => {
                write!(
                    f,
                    "descriptor cannot sign without private key material: {descriptor}"
                )
            }
            Self::UnsupportedSigningDescriptor(descriptor) => {
                write!(f, "unsupported signing descriptor: {descriptor}")
            }
            Self::DuplicateLabel(label) => write!(f, "duplicate wallet label: {label}"),
            Self::UnknownDescriptor(id) => write!(f, "unknown descriptor id: {id}"),
            Self::NoRecipients => write!(f, "transaction requires at least one recipient"),
            Self::NoSpendableCoins => write!(f, "wallet has no spendable coins"),
            Self::ChangeDescriptorRequired => {
                write!(
                    f,
                    "wallet requires an internal change descriptor for this spend"
                )
            }
            Self::InsufficientFunds {
                needed_sats,
                available_sats,
            } => write!(
                f,
                "insufficient funds: need {needed_sats} sats, have {available_sats} sats"
            ),
            Self::Codec(message) => write!(f, "codec error: {message}"),
            Self::Script(message) => write!(f, "script error: {message}"),
            Self::Amount(message) => write!(f, "amount error: {message}"),
            Self::TaprootTweak => write!(f, "invalid taproot tweak"),
        }
    }
}

impl std::error::Error for WalletError {}

impl From<open_bitcoin_codec::CodecError> for WalletError {
    fn from(value: open_bitcoin_codec::CodecError) -> Self {
        Self::Codec(value.to_string())
    }
}

impl From<open_bitcoin_primitives::ScriptError> for WalletError {
    fn from(value: open_bitcoin_primitives::ScriptError) -> Self {
        Self::Script(value.to_string())
    }
}

impl From<open_bitcoin_primitives::AmountError> for WalletError {
    fn from(value: open_bitcoin_primitives::AmountError) -> Self {
        Self::Amount(value.to_string())
    }
}

impl From<OutPoint> for WalletError {
    fn from(value: OutPoint) -> Self {
        Self::Codec(format!(
            "unknown outpoint {}:{}",
            hex_string(value.txid.as_bytes()),
            value.vout
        ))
    }
}

fn hex_string(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(nibble_to_hex(byte >> 4));
        output.push(nibble_to_hex(byte & 0x0f));
    }
    output
}

const fn nibble_to_hex(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'a' + (value - 10)) as char,
        _ => '?',
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_codec::CodecError;
    use open_bitcoin_primitives::{AmountError, OutPoint, ScriptError, Txid};

    use super::WalletError;

    #[test]
    fn display_covers_every_wallet_error_variant() {
        let errors = vec![
            WalletError::DescriptorSyntax("bad".to_string()),
            WalletError::UnsupportedDescriptor("future".to_string()),
            WalletError::InvalidHex("odd".to_string()),
            WalletError::InvalidBase58("bad".to_string()),
            WalletError::InvalidChecksum,
            WalletError::InvalidPrivateKey,
            WalletError::InvalidPublicKey,
            WalletError::InvalidXOnlyPublicKey,
            WalletError::NetworkMismatch {
                expected: "regtest".to_string(),
                actual: "mainnet".to_string(),
            },
            WalletError::MissingSigningKey("wpkh(...)".to_string()),
            WalletError::UnsupportedSigningDescriptor("combo(...)".to_string()),
            WalletError::DuplicateLabel("receive".to_string()),
            WalletError::UnknownDescriptor(7),
            WalletError::NoRecipients,
            WalletError::NoSpendableCoins,
            WalletError::ChangeDescriptorRequired,
            WalletError::InsufficientFunds {
                needed_sats: 10,
                available_sats: 9,
            },
            WalletError::Codec("oops".to_string()),
            WalletError::Script("bad script".to_string()),
            WalletError::Amount("bad amount".to_string()),
            WalletError::TaprootTweak,
        ];

        for error in errors {
            assert!(!error.to_string().is_empty());
        }
    }

    #[test]
    fn conversion_helpers_preserve_useful_messages() {
        let codec = WalletError::from(CodecError::UnexpectedEof {
            needed: 2,
            remaining: 1,
        });
        let script = WalletError::from(ScriptError::TooLarge(10_001));
        let amount = WalletError::from(AmountError::OutOfRange(-1));
        let outpoint = WalletError::from(OutPoint {
            txid: Txid::from_byte_array([0xaa; 32]),
            vout: 5,
        });

        assert!(codec.to_string().contains("codec error"));
        assert!(script.to_string().contains("script error"));
        assert!(amount.to_string().contains("amount error"));
        assert!(outpoint.to_string().contains(":5"));
        assert_eq!(super::nibble_to_hex(42), '?');
    }
}
