// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared sanitized block-relay evidence status contracts.

use serde::{Deserialize, Serialize};

use super::{
    FieldAvailability,
    block_serving::{
        BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON, BlockServingEvidenceStatus,
        BlockServingStatusCounters,
    },
};

/// Aggregate compact-relay negotiation counters safe for operator status surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactRelayNegotiationCounters {
    pub version2_high_bandwidth_count: u64,
    pub version2_low_bandwidth_count: u64,
    pub unsupported_version_count: u64,
}

/// Aggregate compact announcement counters safe for operator status surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactRelayAnnouncementCounters {
    pub compact_announced_count: u64,
    pub compact_headers_fallback_count: u64,
    pub compact_inventory_fallback_count: u64,
    pub compact_suppressed_count: u64,
}

/// Aggregate compact reconstruction counters safe for operator status surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactRelayReconstructionCounters {
    pub compact_reconstructed_count: u64,
    pub compact_reconstruction_failed_count: u64,
    pub compact_malformed_count: u64,
}

/// Aggregate missing-transaction counters safe for operator status surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactRelayMissingTransactionCounters {
    pub compact_missing_tx_requested_count: u64,
    pub compact_missing_tx_suppressed_count: u64,
}

/// Aggregate fallback counters safe for operator status surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactRelayFallbackCounters {
    pub compact_fallback_count: u64,
    pub compact_timeout_count: u64,
}

/// Aggregate in-flight compact download counters safe for operator status surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactRelayInFlightCounters {
    pub in_flight_count: u64,
    pub getblocktxn_in_flight_count: u64,
    pub peers_with_in_flight_count: u64,
}

/// Aggregate cleanup counters safe for operator status surfaces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactRelayCleanupCounters {
    pub compact_cleanup_count: u64,
    pub compact_download_peer_disconnect_count: u64,
    pub compact_download_timeout_count: u64,
    pub compact_download_reorg_count: u64,
    pub compact_download_restart_count: u64,
    pub compact_download_block_connected_count: u64,
}

/// Shared status contract for sanitized block-serving and compact-relay evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockRelayEvidenceStatus {
    #[serde(default = "block_serving_default")]
    pub block_serving: BlockServingEvidenceStatus,
    #[serde(default = "available_default")]
    pub negotiation: FieldAvailability<CompactRelayNegotiationCounters>,
    #[serde(default = "available_default")]
    pub announcement: FieldAvailability<CompactRelayAnnouncementCounters>,
    #[serde(default = "available_default")]
    pub reconstruction: FieldAvailability<CompactRelayReconstructionCounters>,
    #[serde(default = "available_default")]
    pub missing_transaction: FieldAvailability<CompactRelayMissingTransactionCounters>,
    #[serde(default = "available_default")]
    pub fallback: FieldAvailability<CompactRelayFallbackCounters>,
    #[serde(default = "available_default")]
    pub in_flight: FieldAvailability<CompactRelayInFlightCounters>,
    #[serde(default = "available_default")]
    pub cleanup: FieldAvailability<CompactRelayCleanupCounters>,
}

impl BlockRelayEvidenceStatus {
    pub fn default_unavailable() -> Self {
        Self {
            block_serving: BlockServingEvidenceStatus::default_unavailable(),
            negotiation: available_default(),
            announcement: available_default(),
            reconstruction: available_default(),
            missing_transaction: available_default(),
            fallback: available_default(),
            in_flight: available_default(),
            cleanup: available_default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_components(
        block_serving: BlockServingEvidenceStatus,
        negotiation: CompactRelayNegotiationCounters,
        announcement: CompactRelayAnnouncementCounters,
        reconstruction: CompactRelayReconstructionCounters,
        missing_transaction: CompactRelayMissingTransactionCounters,
        fallback: CompactRelayFallbackCounters,
        in_flight: CompactRelayInFlightCounters,
        cleanup: CompactRelayCleanupCounters,
    ) -> Self {
        Self {
            block_serving,
            negotiation: FieldAvailability::available(negotiation),
            announcement: FieldAvailability::available(announcement),
            reconstruction: FieldAvailability::available(reconstruction),
            missing_transaction: FieldAvailability::available(missing_transaction),
            fallback: FieldAvailability::available(fallback),
            in_flight: FieldAvailability::available(in_flight),
            cleanup: FieldAvailability::available(cleanup),
        }
    }
}

impl Default for BlockRelayEvidenceStatus {
    fn default() -> Self {
        Self::default_unavailable()
    }
}

fn block_serving_default() -> BlockServingEvidenceStatus {
    BlockServingEvidenceStatus::default_unavailable()
}

fn available_default<T: Default>() -> FieldAvailability<T> {
    FieldAvailability::available(T::default())
}

pub fn block_serving_default_available_counters() -> BlockServingEvidenceStatus {
    BlockServingEvidenceStatus {
        activation: FieldAvailability::unavailable(BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON),
        eligibility: FieldAvailability::available(Default::default()),
        status: FieldAvailability::available(BlockServingStatusCounters::default()),
    }
}

#[cfg(test)]
mod tests;
