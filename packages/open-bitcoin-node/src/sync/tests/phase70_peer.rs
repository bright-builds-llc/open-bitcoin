// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

fn rotation_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        manual_peers: vec![
            SyncPeerAddress::manual("127.0.0.1", 18_444),
            SyncPeerAddress::manual("127.0.0.1", 18_445),
        ],
        dns_seeds: Vec::new(),
        target_outbound_peers: 1,
        max_peer_retries: 0,
        retry_backoff_ms: 10_000,
        max_messages_per_peer: 8,
        ..sync_config()
    }
}

fn outcome_with_reason(summary: &SyncRunSummary, reason: PeerFailureReason) -> &PeerSyncOutcome {
    summary
        .peer_outcomes
        .iter()
        .find(|outcome| outcome.maybe_failure_reason.as_ref() == Some(&reason))
        .expect("peer outcome with expected failure reason")
}

fn assert_reason_without_block_credit(summary: &SyncRunSummary, reason: PeerFailureReason) {
    let outcome = outcome_with_reason(summary, reason);
    assert_eq!(outcome.contribution.blocks_received, 0);
}

fn assert_first_peer_backoff(runtime: &DurableSyncRuntime) {
    assert!(runtime.peer_backoff.contains_key("127.0.0.1:18444"));
}

fn persist_previous_active_chain_credit(
    runtime: &mut DurableSyncRuntime,
    observed_at_unix_seconds: i64,
) -> ProgressCreditEvidence {
    let mut previous_summary = runtime.snapshot_summary();
    previous_summary.messages_processed = 3;
    previous_summary.headers_received = 1;
    previous_summary.blocks_received = 1;
    previous_summary
        .peer_outcomes
        .push(peer_outcome_with_contribution(
            SyncPeerAddress::manual("127.0.0.1", 18_444),
            PeerSyncState::Connected,
            1,
            None,
            PeerContribution {
                messages_processed: 3,
                headers_received: 1,
                blocks_received: 1,
            },
        ));
    let previous_state = runtime
        .durable_sync_state_for_summary(
            &previous_summary,
            SyncLifecycleState::Active,
            None,
            observed_at_unix_seconds,
        )
        .expect("previous durable status");
    let previous_credit = available_progress_credit(&previous_state).clone();
    runtime
        .persist_durable_sync_state(previous_state)
        .expect("persist previous status");
    previous_credit
}

mod backoff_rotation;
mod failure_classification;
mod fallback_rotation;
