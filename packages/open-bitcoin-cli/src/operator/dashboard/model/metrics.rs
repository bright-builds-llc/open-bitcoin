// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{MetricKind, status::OpenBitcoinStatusSnapshot};

/// Metric series rendered as dashboard charts.
pub const MAX_DASHBOARD_CHARTS: usize = 8;

pub const DASHBOARD_METRIC_KINDS: [MetricKind; 8] = [
    MetricKind::HeaderHeight,
    MetricKind::DownloadedBlockHeight,
    MetricKind::ConnectedBlockHeight,
    MetricKind::SyncHeight,
    MetricKind::PeerCount,
    MetricKind::MempoolTransactions,
    MetricKind::DiskUsageBytes,
    MetricKind::RpcHealth,
];

pub const INBOUND_DASHBOARD_METRIC_CANDIDATES: [MetricKind; 23] = [
    MetricKind::InboundAdmittedPeerCount,
    MetricKind::InboundRejectedPeerCount,
    MetricKind::InboundCapRejectCount,
    MetricKind::InboundReservedSlotRejectCount,
    MetricKind::InboundDuplicateRejectCount,
    MetricKind::InboundSelfConnectionRejectCount,
    MetricKind::InboundPermissionedAdmitCount,
    MetricKind::InboundProtectedAdmitCount,
    MetricKind::InboundInactivePermissionEffectCount,
    MetricKind::InboundPermissionValidationFailureCount,
    MetricKind::InboundEvictionCandidateCount,
    MetricKind::InboundDisconnectCount,
    MetricKind::InboundActiveBanCount,
    MetricKind::InboundMisbehaviorObservationCount,
    MetricKind::InboundProtectedNoActionCount,
    MetricKind::InboundResourcePressureActiveCount,
    MetricKind::InboundReadQueuePressureCount,
    MetricKind::InboundWriteQueuePressureCount,
    MetricKind::InboundRequestCapReachedCount,
    MetricKind::InboundPayloadRejectedCount,
    MetricKind::InboundTimeoutDisconnectCount,
    MetricKind::InboundChurnRejectedCount,
    MetricKind::InboundReconnectSuppressedCount,
];

pub const RELAY_DASHBOARD_METRIC_CANDIDATES: [MetricKind; 16] = [
    MetricKind::RelayAcceptedCount,
    MetricKind::RelayRejectedCount,
    MetricKind::RelayOrphanedCount,
    MetricKind::RelayRequestedCount,
    MetricKind::RelayServedCount,
    MetricKind::RelayAnnouncedCount,
    MetricKind::RelaySuppressedCount,
    MetricKind::RelayEvictedCount,
    MetricKind::RelayExpiredCount,
    MetricKind::RelayRebroadcastDeferredCount,
    MetricKind::RelayRecoveryRecoveredCount,
    MetricKind::RelayRecoveryDroppedConfirmedCount,
    MetricKind::RelayRecoveryDroppedDuplicateCount,
    MetricKind::RelayRecoveryDroppedMissingParentCount,
    MetricKind::RelayRecoveryDroppedPolicyIncompatibleCount,
    MetricKind::RelayRecoveryDroppedEvictedCount,
];

const OPTIONAL_DASHBOARD_METRIC_KINDS: [MetricKind; 3] = [
    MetricKind::MempoolTransactions,
    MetricKind::DiskUsageBytes,
    MetricKind::RpcHealth,
];

pub(super) fn dashboard_metric_kinds(snapshot: &OpenBitcoinStatusSnapshot) -> Vec<MetricKind> {
    let mut kinds = DASHBOARD_METRIC_KINDS.to_vec();
    let retained_optional = retained_inbound_metric_kinds(snapshot);
    for (slot_kind, optional_kind) in OPTIONAL_DASHBOARD_METRIC_KINDS
        .into_iter()
        .zip(retained_optional)
    {
        if let Some(slot) = kinds.iter_mut().find(|kind| **kind == slot_kind) {
            *slot = optional_kind;
        }
    }

    let mut deduplicated = Vec::with_capacity(MAX_DASHBOARD_CHARTS);
    for kind in kinds {
        if deduplicated.contains(&kind) {
            continue;
        }
        deduplicated.push(kind);
        if deduplicated.len() == MAX_DASHBOARD_CHARTS {
            break;
        }
    }
    deduplicated
}

fn retained_inbound_metric_kinds(snapshot: &OpenBitcoinStatusSnapshot) -> Vec<MetricKind> {
    INBOUND_DASHBOARD_METRIC_CANDIDATES
        .into_iter()
        .chain(RELAY_DASHBOARD_METRIC_CANDIDATES)
        .filter(|kind| {
            snapshot
                .metrics
                .samples
                .iter()
                .any(|sample| sample.kind == *kind)
        })
        .collect()
}
