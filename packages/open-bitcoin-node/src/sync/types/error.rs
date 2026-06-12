// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp

use std::fmt;

use open_bitcoin_core::chainstate::ChainstateError;
use open_bitcoin_network::NetworkError;

use crate::{
    ManagedNetworkError, StorageError,
    status::{HealthSignal, HealthSignalLevel},
};

use super::projection::storage_health_message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncRuntimeError {
    NoPeersConfigured,
    AddressResolution { peer: String, message: String },
    PeerCompatibility { message: String },
    Io { peer: String, message: String },
    InvalidData { message: String },
    InvalidMagic { expected: [u8; 4], actual: [u8; 4] },
    Network { message: String },
    ResourceLimit { message: String },
    Storage(StorageError),
}

impl fmt::Display for SyncRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPeersConfigured => write!(f, "no sync peers configured"),
            Self::AddressResolution { peer, message } => {
                write!(f, "failed to resolve sync peer {peer}: {message}")
            }
            Self::PeerCompatibility { message } => {
                write!(f, "sync peer compatibility failure: {message}")
            }
            Self::Io { peer, message } => write!(f, "sync I/O failure for {peer}: {message}"),
            Self::InvalidData { message } => write!(f, "sync invalid data: {message}"),
            Self::InvalidMagic { expected, actual } => write!(
                f,
                "network magic mismatch: expected {expected:?}, got {actual:?}"
            ),
            Self::Network { message } => write!(f, "sync network failure: {message}"),
            Self::ResourceLimit { message } => write!(f, "sync resource limit: {message}"),
            Self::Storage(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for SyncRuntimeError {}

impl SyncRuntimeError {
    pub fn health_signal(&self) -> HealthSignal {
        match self {
            Self::AddressResolution { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync address resolution failed: inspect peer configuration".to_string(),
            },
            Self::PeerCompatibility { message } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: format!(
                    "sync peer compatibility failure: {message}; inspect peer protocol behavior"
                ),
            },
            Self::Io { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync I/O failure: inspect peer connectivity".to_string(),
            },
            Self::InvalidData { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync peer sent invalid data: inspect peer compatibility".to_string(),
            },
            Self::InvalidMagic { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync network magic mismatch: inspect peer network".to_string(),
            },
            Self::Network { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync network failure: inspect peer connectivity".to_string(),
            },
            Self::ResourceLimit { message } => HealthSignal {
                level: HealthSignalLevel::Warn,
                source: "sync".to_string(),
                message: format!("sync resource limit reached: {message}"),
            },
            Self::Storage(error) => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "storage".to_string(),
                message: storage_health_message(error),
            },
            Self::NoPeersConfigured => HealthSignal {
                level: HealthSignalLevel::Warn,
                source: "sync".to_string(),
                message: "sync has no configured peers".to_string(),
            },
        }
    }
}

impl From<StorageError> for SyncRuntimeError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

impl From<ManagedNetworkError> for SyncRuntimeError {
    fn from(value: ManagedNetworkError) -> Self {
        match value {
            ManagedNetworkError::Network(error) => Self::from(error),
            ManagedNetworkError::Chainstate(error) if chainstate_error_is_peer_data(&error) => {
                Self::InvalidData {
                    message: error.to_string(),
                }
            }
            other => Self::Network {
                message: other.to_string(),
            },
        }
    }
}

impl From<NetworkError> for SyncRuntimeError {
    fn from(value: NetworkError) -> Self {
        match value {
            NetworkError::DuplicateVersion(_) => Self::PeerCompatibility {
                message: value.to_string(),
            },
            NetworkError::InvalidHeader { .. }
            | NetworkError::HeadersIncludeTransactions(_)
            | NetworkError::MissingHeaderAncestor(_) => Self::InvalidData {
                message: value.to_string(),
            },
            _ => Self::Network {
                message: value.to_string(),
            },
        }
    }
}

fn chainstate_error_is_peer_data(error: &ChainstateError) -> bool {
    matches!(
        error,
        ChainstateError::MissingCoin { .. }
            | ChainstateError::InvalidGenesisParent { .. }
            | ChainstateError::InvalidTipExtension { .. }
            | ChainstateError::OutputOverwrite { .. }
            | ChainstateError::BlockValidation { .. }
            | ChainstateError::TransactionValidation { .. }
    )
}
