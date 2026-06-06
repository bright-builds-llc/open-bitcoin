// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use serde::{Deserialize, Serialize};

/// Stable machine-readable recovery categories for operator status surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncRecoveryCategory {
    CleanShutdown,
    UncleanShutdown,
    IncompatibleSchema,
    StoreCorruption,
    StorageLockContention,
    StorageBackendFailure,
    ResourceExhaustion,
    InvalidPeerData,
    PublicNetworkUnreachable,
    OperatorCancellation,
}

impl SyncRecoveryCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CleanShutdown => "clean_shutdown",
            Self::UncleanShutdown => "unclean_shutdown",
            Self::IncompatibleSchema => "incompatible_schema",
            Self::StoreCorruption => "store_corruption",
            Self::StorageLockContention => "storage_lock_contention",
            Self::StorageBackendFailure => "storage_backend_failure",
            Self::ResourceExhaustion => "resource_exhaustion",
            Self::InvalidPeerData => "invalid_peer_data",
            Self::PublicNetworkUnreachable => "public_network_unreachable",
            Self::OperatorCancellation => "operator_cancellation",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SyncRecoveryCategory;

    #[test]
    fn sync_recovery_category_serializes_stable_labels() {
        // Arrange
        let cases = [
            (SyncRecoveryCategory::CleanShutdown, "clean_shutdown"),
            (SyncRecoveryCategory::UncleanShutdown, "unclean_shutdown"),
            (
                SyncRecoveryCategory::IncompatibleSchema,
                "incompatible_schema",
            ),
            (SyncRecoveryCategory::StoreCorruption, "store_corruption"),
            (
                SyncRecoveryCategory::StorageLockContention,
                "storage_lock_contention",
            ),
            (
                SyncRecoveryCategory::StorageBackendFailure,
                "storage_backend_failure",
            ),
            (
                SyncRecoveryCategory::ResourceExhaustion,
                "resource_exhaustion",
            ),
            (SyncRecoveryCategory::InvalidPeerData, "invalid_peer_data"),
            (
                SyncRecoveryCategory::PublicNetworkUnreachable,
                "public_network_unreachable",
            ),
            (
                SyncRecoveryCategory::OperatorCancellation,
                "operator_cancellation",
            ),
        ];

        // Act / Assert
        for (category, label) in cases {
            assert_eq!(category.as_str(), label);
            assert_eq!(
                serde_json::to_value(category).expect("recovery category json"),
                serde_json::Value::String(label.to_string())
            );
        }
    }
}
