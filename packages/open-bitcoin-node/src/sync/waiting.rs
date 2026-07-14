// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::{
    DurableSyncRuntime, PeerContribution, PeerFailureReason, PeerRetryState, PeerSyncOutcome,
    PeerSyncState, ResolvedSyncPeerAddress, SyncRunSummary, progress,
};

impl DurableSyncRuntime {
    pub(super) fn record_waiting_outcome(
        &self,
        summary: &mut SyncRunSummary,
        peer: &ResolvedSyncPeerAddress,
        backoff: PeerRetryState,
        timestamp: i64,
    ) {
        let wait_seconds = backoff
            .next_attempt_unix_seconds
            .saturating_sub(timestamp)
            .max(0);
        summary.health_signals.push(progress::waiting_peer_signal());
        summary.peer_outcomes.push(PeerSyncOutcome {
            peer: peer.peer.clone(),
            maybe_resolved_endpoint: Some(peer.endpoint.to_string()),
            network: self.config.network,
            state: PeerSyncState::Waiting,
            attempts: 0,
            contribution: PeerContribution {
                messages_processed: 0,
                headers_received: 0,
                blocks_received: 0,
            },
            maybe_tip_height: None,
            maybe_tip_hash: None,
            maybe_tip_work: None,
            maybe_last_activity_unix_seconds: None,
            maybe_capabilities: None,
            maybe_failure_reason: Some(PeerFailureReason::RetryBackoff),
            maybe_error: Some(format!(
                "retry backoff wait_seconds={wait_seconds} consecutive_failures={} next_attempt_unix_seconds={}",
                backoff.consecutive_failures, backoff.next_attempt_unix_seconds
            )),
        });
    }
}
