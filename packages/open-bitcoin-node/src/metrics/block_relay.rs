// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use crate::status::{BlockRelayEvidenceStatus, FieldAvailability};

use super::{MetricKind, MetricSample};

/// Project block-relay evidence counters into fixed low-cardinality metric samples.
pub fn block_relay_metric_samples(
    block_relay: &BlockRelayEvidenceStatus,
    served_count: u64,
    timestamp_unix_seconds: u64,
) -> Vec<MetricSample> {
    let block_serving = &block_relay.block_serving;
    let block_serving_suppressed_count = match &block_serving.status {
        FieldAvailability::Available(counters) => counters.suppressed_count,
        FieldAvailability::Unavailable { .. } => 0,
    };
    let compact_announced_count = match &block_relay.announcement {
        FieldAvailability::Available(counters) => counters.compact_announced_count,
        FieldAvailability::Unavailable { .. } => 0,
    };
    let compact_reconstructed_count = match &block_relay.reconstruction {
        FieldAvailability::Available(counters) => counters.compact_reconstructed_count,
        FieldAvailability::Unavailable { .. } => 0,
    };
    let compact_missing_tx_requested_count = match &block_relay.missing_transaction {
        FieldAvailability::Available(counters) => counters.compact_missing_tx_requested_count,
        FieldAvailability::Unavailable { .. } => 0,
    };
    let compact_fallback_count = match &block_relay.fallback {
        FieldAvailability::Available(counters) => counters.compact_fallback_count,
        FieldAvailability::Unavailable { .. } => 0,
    };
    let compact_malformed_count = match &block_relay.reconstruction {
        FieldAvailability::Available(counters) => counters.compact_malformed_count,
        FieldAvailability::Unavailable { .. } => 0,
    };
    let compact_timeout_count = match &block_relay.fallback {
        FieldAvailability::Available(counters) => counters.compact_timeout_count,
        FieldAvailability::Unavailable { .. } => 0,
    };
    let compact_cleanup_count = match &block_relay.cleanup {
        FieldAvailability::Available(counters) => counters.compact_cleanup_count,
        FieldAvailability::Unavailable { .. } => 0,
    };

    vec![
        MetricSample::new(
            MetricKind::BlockServedCount,
            served_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::BlockServingSuppressedCount,
            block_serving_suppressed_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::CompactAnnouncedCount,
            compact_announced_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::CompactReconstructedCount,
            compact_reconstructed_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::CompactMissingTxRequestedCount,
            compact_missing_tx_requested_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::CompactFallbackCount,
            compact_fallback_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::CompactMalformedCount,
            compact_malformed_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::CompactTimeoutCount,
            compact_timeout_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::CompactCleanupCount,
            compact_cleanup_count as f64,
            timestamp_unix_seconds,
        ),
    ]
}
