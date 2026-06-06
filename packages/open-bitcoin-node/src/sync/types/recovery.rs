// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp

use crate::status::SyncRecoveryCategory;

use super::{PeerFailureReason, SyncRuntimeError, SyncStopReason};

impl PeerFailureReason {
    pub(crate) const fn recovery_category(&self) -> SyncRecoveryCategory {
        match self {
            Self::ResourceLimit => SyncRecoveryCategory::ResourceExhaustion,
            Self::InvalidData
            | Self::InvalidMagic
            | Self::BlockNotFound
            | Self::MalformedBlock
            | Self::InvalidBlock
            | Self::DuplicateBlock
            | Self::DisconnectedBlock
            | Self::NonExtendingBlock => SyncRecoveryCategory::InvalidPeerData,
            Self::Storage => SyncRecoveryCategory::StorageBackendFailure,
            Self::AddressResolution
            | Self::Compatibility
            | Self::Connect
            | Self::Stall
            | Self::RetryBackoff
            | Self::Network => SyncRecoveryCategory::PublicNetworkUnreachable,
        }
    }
}

impl SyncStopReason {
    pub(crate) const fn recovery_category(self) -> Option<SyncRecoveryCategory> {
        match self {
            Self::OperatorPaused | Self::ShutdownRequested => {
                Some(SyncRecoveryCategory::OperatorCancellation)
            }
            Self::TargetHeaderReached { .. }
            | Self::NoProgress { .. }
            | Self::MaxRoundsReached { .. } => None,
        }
    }
}

impl SyncRuntimeError {
    pub(crate) fn recovery_category(&self) -> SyncRecoveryCategory {
        match self {
            Self::NoPeersConfigured
            | Self::AddressResolution { .. }
            | Self::PeerCompatibility { .. }
            | Self::Io { .. }
            | Self::Network { .. } => SyncRecoveryCategory::PublicNetworkUnreachable,
            Self::InvalidData { .. } | Self::InvalidMagic { .. } => {
                SyncRecoveryCategory::InvalidPeerData
            }
            Self::ResourceLimit { .. } => SyncRecoveryCategory::ResourceExhaustion,
            Self::Storage(error) => error.recovery_category(),
        }
    }
}

#[cfg(test)]
pub(crate) fn recovery_category_from_error_detail(detail: &str) -> Option<SyncRecoveryCategory> {
    let lower_detail = detail.to_ascii_lowercase();

    if lower_detail.contains("schema invalid")
        || lower_detail.contains("invalid schema")
        || lower_detail.contains("schema mismatch")
        || lower_detail.contains("invalid schema version")
    {
        return Some(SyncRecoveryCategory::IncompatibleSchema);
    }
    if lower_detail.contains("storage corruption")
        || lower_detail.contains("corrupt namespace")
        || lower_detail.contains("corruption in")
    {
        return Some(SyncRecoveryCategory::StoreCorruption);
    }
    if contains_lock_signal(&lower_detail) {
        return Some(SyncRecoveryCategory::StorageLockContention);
    }
    if lower_detail.contains("backend")
        || lower_detail.contains("unavailable namespace")
        || lower_detail.contains("storage failure")
        || lower_detail.contains("interrupted write")
    {
        return Some(SyncRecoveryCategory::StorageBackendFailure);
    }
    if lower_detail.contains("resource limit") || lower_detail.contains("resource") {
        return Some(SyncRecoveryCategory::ResourceExhaustion);
    }
    if lower_detail.contains("invalid data")
        || lower_detail.contains("malformed block")
        || lower_detail.contains("invalid block")
        || lower_detail.contains("duplicate_block")
        || lower_detail.contains("disconnected_block")
        || lower_detail.contains("non_extending_block")
    {
        return Some(SyncRecoveryCategory::InvalidPeerData);
    }

    None
}

#[cfg(test)]
fn contains_lock_signal(lower_detail: &str) -> bool {
    contains_ascii_word(lower_detail, "lock")
        || contains_ascii_word(lower_detail, "locked")
        || lower_detail.contains("contention")
}

#[cfg(test)]
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

#[cfg(test)]
const fn is_ascii_word_boundary(byte: u8) -> bool {
    !byte.is_ascii_alphanumeric()
}

#[cfg(test)]
mod tests {
    use crate::{
        status::SyncRecoveryCategory,
        storage::{SchemaVersion, StorageError, StorageNamespace, StorageRecoveryAction},
    };

    use super::{
        super::{PeerFailureReason, SyncRuntimeError, SyncStopReason},
        recovery_category_from_error_detail,
    };

    #[test]
    fn sync_recovery_category_maps_peer_failure_reason_groups() {
        // Arrange
        let invalid_peer_data_reasons = [
            PeerFailureReason::InvalidData,
            PeerFailureReason::InvalidMagic,
            PeerFailureReason::BlockNotFound,
            PeerFailureReason::MalformedBlock,
            PeerFailureReason::InvalidBlock,
            PeerFailureReason::DuplicateBlock,
            PeerFailureReason::DisconnectedBlock,
            PeerFailureReason::NonExtendingBlock,
        ];
        let public_network_reasons = [
            PeerFailureReason::AddressResolution,
            PeerFailureReason::Compatibility,
            PeerFailureReason::Connect,
            PeerFailureReason::Stall,
            PeerFailureReason::RetryBackoff,
            PeerFailureReason::Network,
        ];

        // Act
        let resource_category = PeerFailureReason::ResourceLimit.recovery_category();
        let storage_category = PeerFailureReason::Storage.recovery_category();
        let invalid_categories = invalid_peer_data_reasons.map(|reason| reason.recovery_category());
        let public_network_categories =
            public_network_reasons.map(|reason| reason.recovery_category());

        // Assert
        assert_eq!(resource_category, SyncRecoveryCategory::ResourceExhaustion);
        assert_eq!(
            storage_category,
            SyncRecoveryCategory::StorageBackendFailure
        );
        assert!(
            invalid_categories
                .iter()
                .all(|category| *category == SyncRecoveryCategory::InvalidPeerData)
        );
        assert!(
            public_network_categories
                .iter()
                .all(|category| *category == SyncRecoveryCategory::PublicNetworkUnreachable)
        );
    }

    #[test]
    fn sync_recovery_category_maps_operator_stop_reasons() {
        // Arrange
        let stop_reasons = [
            SyncStopReason::OperatorPaused,
            SyncStopReason::ShutdownRequested,
        ];

        // Act
        let categories = stop_reasons.map(SyncStopReason::recovery_category);

        // Assert
        assert_eq!(
            categories,
            [
                Some(SyncRecoveryCategory::OperatorCancellation),
                Some(SyncRecoveryCategory::OperatorCancellation),
            ]
        );
        assert_eq!(
            SyncRecoveryCategory::OperatorCancellation.as_str(),
            "operator_cancellation"
        );
    }

    #[test]
    fn sync_recovery_category_maps_runtime_error_groups_and_storage_precedence() {
        // Arrange
        let network_errors = [
            SyncRuntimeError::NoPeersConfigured,
            SyncRuntimeError::AddressResolution {
                peer: "seed.bitcoin.sipa.be".to_string(),
                message: "dns unavailable".to_string(),
            },
            SyncRuntimeError::PeerCompatibility {
                message: "duplicate version".to_string(),
            },
            SyncRuntimeError::Io {
                peer: "203.0.113.1:8333".to_string(),
                message: "timed out".to_string(),
            },
            SyncRuntimeError::Network {
                message: "connection reset".to_string(),
            },
        ];
        let invalid_errors = [
            SyncRuntimeError::InvalidData {
                message: "malformed block".to_string(),
            },
            SyncRuntimeError::InvalidMagic {
                expected: [0xf9, 0xbe, 0xb4, 0xd9],
                actual: [0x0b, 0x11, 0x09, 0x07],
            },
        ];
        let resource_error = SyncRuntimeError::ResourceLimit {
            message: "resource limit reached".to_string(),
        };
        let storage_error = SyncRuntimeError::Storage(StorageError::schema_mismatch(
            SchemaVersion::new(2).expect("nonzero schema version"),
            SchemaVersion::new(1).expect("nonzero schema version"),
        ));

        // Act
        let network_categories = network_errors.map(|error| error.recovery_category());
        let invalid_categories = invalid_errors.map(|error| error.recovery_category());
        let resource_category = resource_error.recovery_category();
        let storage_category = storage_error.recovery_category();

        // Assert
        assert!(
            network_categories
                .iter()
                .all(|category| *category == SyncRecoveryCategory::PublicNetworkUnreachable)
        );
        assert!(
            invalid_categories
                .iter()
                .all(|category| *category == SyncRecoveryCategory::InvalidPeerData)
        );
        assert_eq!(resource_category, SyncRecoveryCategory::ResourceExhaustion);
        assert_eq!(storage_category, SyncRecoveryCategory::IncompatibleSchema);
    }

    #[test]
    fn sync_recovery_category_from_error_detail_maps_known_detail_facts() {
        // Arrange
        let cases = [
            (
                "schema invalid in durable metadata",
                SyncRecoveryCategory::IncompatibleSchema,
            ),
            (
                "storage corruption in block index",
                SyncRecoveryCategory::StoreCorruption,
            ),
            (
                "runtime store locked by another process",
                SyncRecoveryCategory::StorageLockContention,
            ),
            (
                "interrupted write in headers namespace",
                SyncRecoveryCategory::StorageBackendFailure,
            ),
            (
                "peer hit resource limit",
                SyncRecoveryCategory::ResourceExhaustion,
            ),
            (
                "malformed block payload",
                SyncRecoveryCategory::InvalidPeerData,
            ),
        ];

        // Act / Assert
        for (detail, expected_category) in cases {
            assert_eq!(
                recovery_category_from_error_detail(detail),
                Some(expected_category)
            );
        }
        assert_eq!(recovery_category_from_error_detail("no known signal"), None);
    }

    #[test]
    fn sync_recovery_category_uses_storage_error_lock_mapping() {
        // Arrange
        let runtime_error = SyncRuntimeError::Storage(StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            message: "store lock held by another process".to_string(),
            action: StorageRecoveryAction::Restart,
        });

        // Act
        let category = runtime_error.recovery_category();

        // Assert
        assert_eq!(category, SyncRecoveryCategory::StorageLockContention);
    }
}
