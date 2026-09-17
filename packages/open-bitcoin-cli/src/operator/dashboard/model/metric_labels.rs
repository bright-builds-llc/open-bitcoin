// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::MetricKind;

pub(super) fn metric_label(kind: MetricKind) -> &'static str {
    match kind {
        MetricKind::SyncHeight => "Sync height",
        MetricKind::HeaderHeight => "Header height",
        MetricKind::DownloadedBlockHeight => "Downloaded block height",
        MetricKind::ConnectedBlockHeight => "Connected block height",
        MetricKind::ValidatedActiveChainHeight => "Validated active-chain height",
        MetricKind::PeerCount => "Peers",
        MetricKind::MempoolTransactions => "Mempool tx",
        MetricKind::WalletTrustedBalanceSats => "Wallet sats",
        MetricKind::DiskUsageBytes => "Disk bytes",
        MetricKind::RpcHealth => "RPC health",
        MetricKind::ServiceRestarts => "Service restarts",
        MetricKind::InboundAdmittedPeerCount => "Inbound admits",
        MetricKind::InboundRejectedPeerCount => "Inbound rejects",
        MetricKind::InboundCapRejectCount => "Inbound cap rejects",
        MetricKind::InboundReservedSlotRejectCount => "Inbound reserved rejects",
        MetricKind::InboundDuplicateRejectCount => "Inbound duplicate rejects",
        MetricKind::InboundSelfConnectionRejectCount => "Inbound self-connection rejects",
        MetricKind::InboundPermissionedAdmitCount => "Inbound permissioned admits",
        MetricKind::InboundProtectedAdmitCount => "Inbound protected admits",
        MetricKind::InboundInactivePermissionEffectCount => "Inbound inactive permission effects",
        MetricKind::InboundPermissionValidationFailureCount => {
            "Inbound permission validation failures"
        }
        MetricKind::InboundEvictionCandidateCount => "Inbound eviction candidates",
        MetricKind::InboundDisconnectCount => "Inbound disconnects",
        MetricKind::InboundActiveBanCount => "Inbound active bans",
        MetricKind::InboundMisbehaviorObservationCount => "Inbound misbehavior observations",
        MetricKind::InboundProtectedNoActionCount => "Inbound protected no-actions",
        MetricKind::InboundResourcePressureActiveCount => "Inbound resource pressure",
        MetricKind::InboundReadQueuePressureCount => "Inbound read queue pressure",
        MetricKind::InboundWriteQueuePressureCount => "Inbound write queue pressure",
        MetricKind::InboundRequestCapReachedCount => "Inbound request cap reached",
        MetricKind::InboundPayloadRejectedCount => "Inbound payload rejects",
        MetricKind::InboundTimeoutDisconnectCount => "Inbound timeout disconnects",
        MetricKind::InboundChurnRejectedCount => "Inbound churn rejects",
        MetricKind::InboundReconnectSuppressedCount => "Inbound reconnect suppressions",
        MetricKind::RelayAcceptedCount => "Relay accepted",
        MetricKind::RelayRejectedCount => "Relay rejected",
        MetricKind::RelayOrphanedCount => "Relay orphaned",
        MetricKind::RelayRequestedCount => "Relay requested",
        MetricKind::RelayServedCount => "Relay served",
        MetricKind::RelayAnnouncedCount => "Relay announced",
        MetricKind::RelaySuppressedCount => "Relay suppressed",
        MetricKind::RelayEvictedCount => "Relay evicted",
        MetricKind::RelayExpiredCount => "Relay expired",
        MetricKind::RelayRebroadcastDeferredCount => "Relay rebroadcast deferred",
        MetricKind::RelayRecoveryRecoveredCount => "Relay recovery recovered",
        MetricKind::RelayRecoveryDroppedConfirmedCount => "Relay recovery dropped confirmed",
        MetricKind::RelayRecoveryDroppedDuplicateCount => "Relay recovery dropped duplicate",
        MetricKind::RelayRecoveryDroppedMissingParentCount => {
            "Relay recovery dropped missing parent"
        }
        MetricKind::RelayRecoveryDroppedPolicyIncompatibleCount => {
            "Relay recovery dropped policy incompatible"
        }
        MetricKind::RelayRecoveryDroppedExpiredCount => "Relay recovery dropped expired",
        MetricKind::RelayRecoveryDroppedEvictedCount => "Relay recovery dropped evicted",
        MetricKind::BlockServedCount => "Block served",
        MetricKind::BlockServingSuppressedCount => "Block serving suppressed",
        MetricKind::CompactAnnouncedCount => "Compact announced",
        MetricKind::CompactReconstructedCount => "Compact reconstructed",
        MetricKind::CompactMissingTxRequestedCount => "Compact missing tx requested",
        MetricKind::CompactFallbackCount => "Compact fallback",
        MetricKind::CompactMalformedCount => "Compact malformed",
        MetricKind::CompactTimeoutCount => "Compact timeout",
        MetricKind::CompactCleanupCount => "Compact cleanup",
        MetricKind::MempoolVirtualSize => "Virtual size",
        MetricKind::MempoolAccountedUsage => "Accounted usage",
        MetricKind::MempoolAccountedCapacity => "Accounted capacity",
        MetricKind::MempoolStaticRelayFloor => "Static relay floor",
        MetricKind::MempoolRollingMempoolFloor => "Rolling mempool floor",
        MetricKind::MempoolEffectiveAdmissionFloor => "Effective admission floor",
        MetricKind::MempoolIncrementalRelayFee => "Incremental relay fee",
        MetricKind::MempoolPressureRemovalCount => "Pressure removals",
        MetricKind::MempoolCheckpointOverdue => "Checkpoint overdue",
        MetricKind::MempoolRecoveryRecoveredCount => "Mempool recovery recovered",
        MetricKind::MempoolRetryEligible => "Retry eligible",
        MetricKind::MempoolRetryCleared => "Retry cleared",
        MetricKind::MempoolAdmissionAccepted => "Admission accepted",
        MetricKind::MempoolAdmissionStillPresent => "Admission still present",
        MetricKind::ChainstateDurabilityCacheSizeClass => "Chainstate durability cache-size class",
        MetricKind::ChainstateDurabilityLastFlushReasonClass => {
            "Chainstate durability last-flush-reason class"
        }
        MetricKind::ChainstateDurabilityWriteKindClass => "Chainstate durability write-kind class",
        MetricKind::ChainstateDurabilityRecoveryClass => "Chainstate durability recovery class",
        MetricKind::ChainstateDurabilityAvailableCount => "Chainstate durability available count",
        MetricKind::ChainstateDurabilityUnavailableCount => {
            "Chainstate durability unavailable count"
        }
        MetricKind::ChainstateDurabilityIndexKnownWithoutPayloadCount => {
            "Chainstate durability index-known-without-payload count"
        }
    }
}
