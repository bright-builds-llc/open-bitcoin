// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::{
    DurableSyncRuntime, PeerContribution, PeerFailureReason, PeerRetryState, PeerSyncOutcome,
    PeerSyncState, ResolvedSyncPeerAddress, SyncRunSummary, SyncRuntimeError, SyncStopReason,
    progress, tip,
};
use crate::SyncLifecycleState;

impl DurableSyncRuntime {
    pub(super) fn maybe_target_header_stop_reason(
        &self,
        summary: &SyncRunSummary,
    ) -> Option<SyncStopReason> {
        let target_header_height = self.config.maybe_target_header_height?;
        if summary.best_header_height < target_header_height {
            return None;
        }
        Some(SyncStopReason::TargetHeaderReached {
            target_header_height,
            best_header_height: summary.best_header_height,
        })
    }

    pub(super) fn maybe_current_at_best_known_tip_stop_reason(
        &self,
        timestamp: i64,
    ) -> Option<SyncStopReason> {
        tip::current_at_best_known_tip_stop_reason_from_evidence(
            self.network.peer_manager().header_store().best_tip(),
            self.connected_block(),
            u64::try_from(timestamp).unwrap_or(0),
            self.config.tip_freshness_threshold_seconds,
        )
    }

    pub(super) fn record_until_idle_stop(
        &self,
        summary: &mut SyncRunSummary,
        stop_reason: SyncStopReason,
        timestamp: i64,
    ) -> Result<(), SyncRuntimeError> {
        summary.maybe_stop_reason = Some(stop_reason);
        summary.health_signals.push(stop_reason.health_signal());
        let state = self.durable_sync_state_from_summary(
            summary,
            SyncLifecycleState::Active,
            summary.latest_error_message(),
            timestamp,
        )?;
        self.persist_durable_sync_state(state)?;
        Ok(())
    }

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
