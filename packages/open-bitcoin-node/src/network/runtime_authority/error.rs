// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use std::fmt;

use crate::sync::SyncRuntimeError;

use super::{ManagedNetworkAuthorityError, ManagedNetworkError};

impl fmt::Display for ManagedNetworkAuthorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Poisoned => formatter.write_str("authoritative network state is unavailable"),
            Self::LifecycleEffect(message) => formatter.write_str(message),
            Self::Operation(error) => error.fmt(formatter),
            Self::MaintenanceTick(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ManagedNetworkAuthorityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Poisoned | Self::LifecycleEffect(_) | Self::MaintenanceTick(_) => None,
            Self::Operation(error) => Some(error),
        }
    }
}

impl From<ManagedNetworkError> for ManagedNetworkAuthorityError {
    fn from(value: ManagedNetworkError) -> Self {
        Self::Operation(value)
    }
}

impl From<ManagedNetworkAuthorityError> for SyncRuntimeError {
    fn from(value: ManagedNetworkAuthorityError) -> Self {
        match value {
            ManagedNetworkAuthorityError::Poisoned => Self::Network {
                message: "authoritative network state is unavailable".to_string(),
            },
            ManagedNetworkAuthorityError::LifecycleEffect(message) => Self::Network { message },
            ManagedNetworkAuthorityError::Operation(error) => Self::from(error),
            ManagedNetworkAuthorityError::MaintenanceTick(error) => Self::Network {
                message: error.to_string(),
            },
        }
    }
}
