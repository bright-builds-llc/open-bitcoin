// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Adapter-facing durable storage contracts for the node shell.

use core::fmt;

use crate::status::{DurableSyncState, SyncControlState, SyncRecoveryCategory};

pub mod fjall_store;
mod lock_probe;
pub mod snapshot_codec;

pub use fjall_store::FjallNodeStore;
pub use lock_probe::{FJALL_LOCK_FILE_NAME, probe_fjall_lock};
pub use snapshot_codec::{MetricsStorageSnapshot, StoredHeaderEntries};

/// Logical storage namespaces later adapters must keep distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageRecoveryAction {
    Restart,
    Reindex,
    Repair,
    RestoreFromBackup,
    FreeDisk,
}

impl StorageRecoveryAction {
    pub const fn recovery_category(self) -> SyncRecoveryCategory {
        match self {
            Self::Restart => SyncRecoveryCategory::StorageBackendFailure,
            Self::Reindex => SyncRecoveryCategory::IncompatibleSchema,
            Self::Repair | Self::RestoreFromBackup => SyncRecoveryCategory::StoreCorruption,
            Self::FreeDisk => SyncRecoveryCategory::ResourceExhaustion,
        }
    }

    pub const fn operator_message(self) -> &'static str {
        match self {
            Self::Restart => "Restart the node and retry the storage operation.",
            Self::Reindex => "Run a reindex so storage can rebuild derived indexes.",
            Self::Repair => "Run the storage repair flow before restarting normal operation.",
            Self::RestoreFromBackup => {
                "Restore the affected datadir or wallet state from a known-good backup."
            }
            Self::FreeDisk => "Free disk space for the selected datadir, then retry sync.",
        }
    }

    pub fn for_backend_message(message: &str) -> Self {
        let lower_message = message.to_ascii_lowercase();
        if contains_storage_pressure_signal(&lower_message) {
            return Self::FreeDisk;
        }

        Self::Restart
    }
}

/// Storage runtime metadata persisted outside pure domain snapshots.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RuntimeMetadata {
    pub node_version: String,
    pub storage_engine: String,
    pub last_clean_shutdown: bool,
    pub maybe_last_recovery_action: Option<StorageRecoveryAction>,
    pub maybe_sync_state: Option<DurableSyncState>,
    pub sync_control: SyncControlState,
}

impl Default for RuntimeMetadata {
    fn default() -> Self {
        Self {
            node_version: env!("CARGO_PKG_VERSION").to_string(),
            storage_engine: "fjall".to_string(),
            last_clean_shutdown: false,
            maybe_last_recovery_action: None,
            maybe_sync_state: None,
            sync_control: SyncControlState::default(),
        }
    }
}

/// Durable marker left behind when storage needs operator recovery.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecoveryMarker {
    pub namespace: StorageNamespace,
    pub action: StorageRecoveryAction,
    pub detail: String,
}

impl RecoveryMarker {
    pub fn new(
        namespace: StorageNamespace,
        action: StorageRecoveryAction,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            namespace,
            action,
            detail: detail.into(),
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
    RecoveryMarkerCorruption {
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

    pub fn recovery_category(&self) -> SyncRecoveryCategory {
        match self {
            Self::InvalidSchemaVersion { .. } | Self::SchemaMismatch { .. } => {
                SyncRecoveryCategory::IncompatibleSchema
            }
            Self::Corruption { .. } | Self::RecoveryMarkerCorruption { .. } => {
                SyncRecoveryCategory::StoreCorruption
            }
            Self::UnavailableNamespace { .. } | Self::InterruptedWrite { .. } => {
                SyncRecoveryCategory::StorageBackendFailure
            }
            Self::BackendFailure {
                message, action, ..
            } => {
                if contains_storage_lock_signal(message) {
                    return SyncRecoveryCategory::StorageLockContention;
                }

                action.recovery_category()
            }
        }
    }

    pub const fn recovery_action(&self) -> Option<StorageRecoveryAction> {
        match self {
            Self::InvalidSchemaVersion { .. }
            | Self::SchemaMismatch { .. }
            | Self::UnavailableNamespace { .. } => None,
            Self::Corruption { action, .. }
            | Self::RecoveryMarkerCorruption { action, .. }
            | Self::InterruptedWrite { action, .. }
            | Self::BackendFailure { action, .. } => Some(*action),
        }
    }
}

fn contains_storage_lock_signal(message: &str) -> bool {
    let lower_message = message.to_ascii_lowercase();
    contains_ascii_word(&lower_message, "lock")
        || contains_ascii_word(&lower_message, "locked")
        || lower_message.contains("contention")
}

fn contains_storage_pressure_signal(lower_message: &str) -> bool {
    lower_message.contains("no space left on device")
        || lower_message.contains("enospc")
        || lower_message.contains("disk full")
        || lower_message.contains("low disk")
        || lower_message.contains("storage pressure")
}

fn contains_ascii_word(haystack: &str, needle: &str) -> bool {
    haystack.match_indices(needle).any(|(start, _)| {
        let end = start + needle.len();
        let before_is_boundary =
            start == 0 || is_ascii_word_boundary(haystack.as_bytes()[start - 1]);
        let after_is_boundary =
            end == haystack.len() || is_ascii_word_boundary(haystack.as_bytes()[end]);
        before_is_boundary && after_is_boundary
    })
}

const fn is_ascii_word_boundary(byte: u8) -> bool {
    !byte.is_ascii_alphanumeric()
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
            Self::RecoveryMarkerCorruption {
                namespace,
                detail,
                action,
            } => write!(
                f,
                "recovery marker corruption in {}: {detail}; {}",
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
    use crate::status::SyncRecoveryCategory;

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
            StorageRecoveryAction::FreeDisk,
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

    #[test]
    fn storage_recovery_category_maps_schema_corruption_lock_and_backend_states() {
        // Arrange
        let schema_error = StorageError::schema_mismatch(
            SchemaVersion::new(2).expect("nonzero schema version"),
            SchemaVersion::new(1).expect("nonzero schema version"),
        );
        let corruption_error = StorageError::Corruption {
            namespace: StorageNamespace::BlockIndex,
            detail: "checksum mismatch".to_string(),
            action: StorageRecoveryAction::Repair,
        };
        let lock_error = StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            message: "database lock held by another process".to_string(),
            action: StorageRecoveryAction::Restart,
        };
        let locked_error = StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            message: "store locked by another process".to_string(),
            action: StorageRecoveryAction::Restart,
        };
        let contention_error = StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            message: "contention while opening the runtime store".to_string(),
            action: StorageRecoveryAction::Restart,
        };
        let backend_error = StorageError::BackendFailure {
            namespace: StorageNamespace::Headers,
            message: "flush failed".to_string(),
            action: StorageRecoveryAction::Restart,
        };
        let block_backend_error = StorageError::BackendFailure {
            namespace: StorageNamespace::Headers,
            message: "block flush failed".to_string(),
            action: StorageRecoveryAction::Restart,
        };
        let interrupted_write = StorageError::InterruptedWrite {
            namespace: StorageNamespace::Headers,
            action: StorageRecoveryAction::Restart,
        };

        // Act
        let mapped_categories = [
            schema_error.recovery_category(),
            corruption_error.recovery_category(),
            lock_error.recovery_category(),
            locked_error.recovery_category(),
            contention_error.recovery_category(),
            backend_error.recovery_category(),
            block_backend_error.recovery_category(),
            interrupted_write.recovery_category(),
        ];
        let action_categories = [
            StorageRecoveryAction::Restart.recovery_category(),
            StorageRecoveryAction::Reindex.recovery_category(),
            StorageRecoveryAction::Repair.recovery_category(),
            StorageRecoveryAction::RestoreFromBackup.recovery_category(),
        ];

        // Assert
        assert_eq!(
            mapped_categories,
            [
                SyncRecoveryCategory::IncompatibleSchema,
                SyncRecoveryCategory::StoreCorruption,
                SyncRecoveryCategory::StorageLockContention,
                SyncRecoveryCategory::StorageLockContention,
                SyncRecoveryCategory::StorageLockContention,
                SyncRecoveryCategory::StorageBackendFailure,
                SyncRecoveryCategory::StorageBackendFailure,
                SyncRecoveryCategory::StorageBackendFailure,
            ]
        );
        assert_eq!(
            action_categories,
            [
                SyncRecoveryCategory::StorageBackendFailure,
                SyncRecoveryCategory::IncompatibleSchema,
                SyncRecoveryCategory::StoreCorruption,
                SyncRecoveryCategory::StoreCorruption,
            ]
        );
    }

    #[test]
    fn storage_recovery_category_maps_low_disk_and_storage_pressure() {
        // Arrange
        let pressure_messages = [
            "no space left on device while flushing block index",
            "ENOSPC while writing chainstate",
            "disk full during runtime metadata write",
            "low disk warning from backend",
            "storage pressure reported by adapter",
        ];
        let free_disk_error = StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            message: "storage pressure reported by adapter".to_string(),
            action: StorageRecoveryAction::FreeDisk,
        };

        // Act
        let pressure_actions = pressure_messages.map(StorageRecoveryAction::for_backend_message);
        let block_flush_action = StorageRecoveryAction::for_backend_message("block flush failed");
        let serialized_action =
            serde_json::to_value(StorageRecoveryAction::FreeDisk).expect("recovery action json");
        let category = free_disk_error.recovery_category();
        let action_category = StorageRecoveryAction::FreeDisk.recovery_category();
        let operator_message = StorageRecoveryAction::FreeDisk.operator_message();

        // Assert
        assert!(
            pressure_actions
                .iter()
                .all(|action| *action == StorageRecoveryAction::FreeDisk)
        );
        assert_eq!(block_flush_action, StorageRecoveryAction::Restart);
        assert_eq!(
            serialized_action,
            serde_json::Value::String("free_disk".to_string())
        );
        assert_eq!(category, SyncRecoveryCategory::ResourceExhaustion);
        assert_eq!(action_category, SyncRecoveryCategory::ResourceExhaustion);
        assert_eq!(
            operator_message,
            "Free disk space for the selected datadir, then retry sync."
        );
    }
}
