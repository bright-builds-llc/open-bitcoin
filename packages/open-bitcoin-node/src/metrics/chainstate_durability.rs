// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use crate::status::{
    CacheSizeLabel, ChainstateDurabilityEvidence, CoinsRecoveryOutcome, FieldAvailability,
    LastFlushReasonLabel, WriteKindLabel,
};

use super::{MetricKind, MetricSample};

/// Project chainstate-durability evidence into seven fixed low-cardinality series.
///
/// Cache-size is the current snapshot occupancy class (`FlushMode::None`), not
/// the last write. Unavailable fields emit no samples so occupancy is not
/// fabricated as 0/0/0.
pub fn chainstate_durability_metric_samples(
    durability: &FieldAvailability<ChainstateDurabilityEvidence>,
    timestamp_unix_seconds: u64,
) -> Vec<MetricSample> {
    let FieldAvailability::Available(evidence) = durability else {
        return Vec::new();
    };

    vec![
        MetricSample::new(
            MetricKind::ChainstateDurabilityCacheSizeClass,
            cache_size_class(evidence.cache_size),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::ChainstateDurabilityLastFlushReasonClass,
            last_flush_reason_class(evidence.last_flush_reason),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::ChainstateDurabilityWriteKindClass,
            write_kind_class(evidence.write_kind),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::ChainstateDurabilityRecoveryClass,
            recovery_class(evidence.recovery_outcome),
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::ChainstateDurabilityAvailableCount,
            evidence.available_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::ChainstateDurabilityUnavailableCount,
            evidence.unavailable_count as f64,
            timestamp_unix_seconds,
        ),
        MetricSample::new(
            MetricKind::ChainstateDurabilityIndexKnownWithoutPayloadCount,
            evidence.index_known_without_payload_count as f64,
            timestamp_unix_seconds,
        ),
    ]
}

fn cache_size_class(cache_size: CacheSizeLabel) -> f64 {
    match cache_size {
        CacheSizeLabel::Ok => 0.0,
        CacheSizeLabel::Large => 1.0,
        CacheSizeLabel::Critical => 2.0,
    }
}

fn last_flush_reason_class(reason: LastFlushReasonLabel) -> f64 {
    match reason {
        LastFlushReasonLabel::None => 0.0,
        LastFlushReasonLabel::Needed => 1.0,
        LastFlushReasonLabel::Periodic => 2.0,
        LastFlushReasonLabel::Always => 3.0,
        LastFlushReasonLabel::FailedDisk => 4.0,
    }
}

fn write_kind_class(write_kind: WriteKindLabel) -> f64 {
    match write_kind {
        WriteKindLabel::None => 0.0,
        WriteKindLabel::Flush => 1.0,
        WriteKindLabel::Sync => 2.0,
        WriteKindLabel::RefuseDiskSpace => 3.0,
    }
}

fn recovery_class(outcome: CoinsRecoveryOutcome) -> f64 {
    match outcome {
        CoinsRecoveryOutcome::Consistent => 0.0,
        CoinsRecoveryOutcome::Replayed => 1.0,
        CoinsRecoveryOutcome::Interrupted => 2.0,
        CoinsRecoveryOutcome::FailClosed => 3.0,
    }
}
