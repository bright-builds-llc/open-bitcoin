// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

fn assert_peer_reason_without_block_credit(summary: &SyncRunSummary, reason: PeerFailureReason) {
    let outcome = summary
        .peer_outcomes
        .iter()
        .find(|outcome| outcome.maybe_failure_reason.as_ref() == Some(&reason))
        .expect("peer outcome with block response failure reason");
    assert_eq!(outcome.contribution.blocks_received, 0);
}

mod connection_progress;
mod peer_tip_evidence;
mod response_failures;
