// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Adapter-facing durable storage contracts for the node shell.

use core::fmt;

/// Logical storage namespaces later adapters must keep distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StorageNamespace {
    Headers,
    BlockIndex,
    Chainstate,
    Wallet,
    Metrics,
    Runtime,
    Schema,
}

impl StorageNamespace {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Headers => "headers",
            Self::BlockIndex => "block_index",
            Self::Chainstate => "chainstate",
            Self::Wallet => "wallet",
            Self::Metrics => "metrics",
            Self::Runtime => "runtime",
            Self::Schema => "schema",
        }
    }
}

/// Nonzero schema version recorded by durable storage adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaVersion(u32);

impl SchemaVersion {
    pub const CURRENT: Self = Self(1);

    pub fn new(version: u32) -> Result<Self, StorageError> {
        if version == 0 {
            return Err(StorageError::InvalidSchemaVersion { version });
        }

        Ok(Self(version))
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Persistence strength requested by a storage operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistMode {
    Buffered,
    Flush,
    Sync,
}

impl PersistMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Buffered => "buffered",
            Self::Flush => "flush",
            Self::Sync => "sync",
        }
    }
}

/// Operator-visible recovery action suggested by a storage failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageRecoveryAction {
    Restart,
    Reindex,
    Repair,
    RestoreFromBackup,
}

impl StorageRecoveryAction {
    pub const fn operator_message(self) -> &'static str {
        match self {
            Self::Restart => "Restart the node and retry the storage operation.",
            Self::Reindex => "Run a reindex so storage can rebuild derived indexes.",
            Self::Repair => "Run the storage repair flow before restarting normal operation.",
            Self::RestoreFromBackup => {
                "Restore the affected datadir or wallet state from a known-good backup."
            }
        }
    }
}

/// Typed storage errors produced by future durable adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    InvalidSchemaVersion {
        version: u32,
    },
    SchemaMismatch {
        expected: SchemaVersion,
        actual: SchemaVersion,
    },
    Corruption {
        namespace: StorageNamespace,
        detail: String,
        action: StorageRecoveryAction,
    },
    UnavailableNamespace {
        namespace: StorageNamespace,
    },
    InterruptedWrite {
        namespace: StorageNamespace,
        action: StorageRecoveryAction,
    },
    BackendFailure {
        namespace: StorageNamespace,
        message: String,
        action: StorageRecoveryAction,
    },
}

impl StorageError {
    pub const fn schema_mismatch(expected: SchemaVersion, actual: SchemaVersion) -> Self {
        Self::SchemaMismatch { expected, actual }
    }

    pub const fn recovery_action(&self) -> Option<StorageRecoveryAction> {
        match self {
            Self::InvalidSchemaVersion { .. }
            | Self::SchemaMismatch { .. }
            | Self::UnavailableNamespace { .. } => None,
            Self::Corruption { action, .. }
            | Self::InterruptedWrite { action, .. }
            | Self::BackendFailure { action, .. } => Some(*action),
        }
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchemaVersion { version } => {
                write!(f, "invalid storage schema version: {version}")
            }
            Self::SchemaMismatch { expected, actual } => write!(
                f,
                "storage schema mismatch: expected {}, found {}",
                expected.get(),
                actual.get()
            ),
            Self::Corruption {
                namespace,
                detail,
                action,
            } => write!(
                f,
                "storage corruption in {}: {detail}; {}",
                namespace.as_str(),
                action.operator_message()
            ),
            Self::UnavailableNamespace { namespace } => {
                write!(f, "storage namespace unavailable: {}", namespace.as_str())
            }
            Self::InterruptedWrite { namespace, action } => write!(
                f,
                "interrupted write in {}; {}",
                namespace.as_str(),
                action.operator_message()
            ),
            Self::BackendFailure {
                namespace,
                message,
                action,
            } => write!(
                f,
                "storage backend failure in {}: {message}; {}",
                namespace.as_str(),
                action.operator_message()
            ),
        }
    }
}

impl std::error::Error for StorageError {}

#[cfg(test)]
mod tests {
    use super::{SchemaVersion, StorageError, StorageNamespace, StorageRecoveryAction};

    #[test]
    fn storage_namespace_names_are_stable() {
        // Arrange
        let namespaces = [
            (StorageNamespace::Headers, "headers"),
            (StorageNamespace::BlockIndex, "block_index"),
            (StorageNamespace::Chainstate, "chainstate"),
            (StorageNamespace::Wallet, "wallet"),
            (StorageNamespace::Metrics, "metrics"),
            (StorageNamespace::Runtime, "runtime"),
            (StorageNamespace::Schema, "schema"),
        ];

        // Act / Assert
        for (namespace, expected_name) in namespaces {
            assert_eq!(namespace.as_str(), expected_name);
        }
    }

    #[test]
    fn storage_recovery_actions_have_operator_messages() {
        // Arrange
        let actions = [
            StorageRecoveryAction::Restart,
            StorageRecoveryAction::Reindex,
            StorageRecoveryAction::Repair,
            StorageRecoveryAction::RestoreFromBackup,
        ];

        // Act / Assert
        for action in actions {
            assert!(!action.operator_message().is_empty());
        }
    }

    #[test]
    fn schema_mismatch_exposes_expected_and_actual_versions() {
        // Arrange
        let expected = SchemaVersion::new(2).expect("nonzero schema version");
        let actual = SchemaVersion::new(1).expect("nonzero schema version");

        // Act
        let error = StorageError::schema_mismatch(expected, actual);

        // Assert
        assert_eq!(
            error.to_string(),
            "storage schema mismatch: expected 2, found 1"
        );
        assert_eq!(error.recovery_action(), None);
    }
}
