// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use crate::{RelayEligibilityDecision, RelayEligibilityReason};

use super::*;

fn eligible_relay() -> RelayEligibilityDecision {
    RelayEligibilityDecision {
        eligible: true,
        reason: RelayEligibilityReason::Eligible,
        relay_permission_effects: Vec::new(),
        version_message_relay: true,
    }
}

fn disabled_relay() -> RelayEligibilityDecision {
    RelayEligibilityDecision {
        eligible: false,
        reason: RelayEligibilityReason::Disabled,
        relay_permission_effects: Vec::new(),
        version_message_relay: false,
    }
}

fn relay_disabled_after_eligibility() -> RelayEligibilityDecision {
    RelayEligibilityDecision {
        eligible: true,
        reason: RelayEligibilityReason::Eligible,
        relay_permission_effects: Vec::new(),
        version_message_relay: false,
    }
}

fn peer(peer_id: PeerId, peer_mode: TxRelayPeerMode) -> TxFanoutPeerInput {
    TxFanoutPeerInput {
        peer_id,
        peer_mode,
        relay_eligibility: eligible_relay(),
        origin_peer: false,
        already_have: false,
        recent_reject: false,
        in_flight: false,
        mempool_known: false,
    }
}

fn admission(byte: u8, outcome: TxFanoutAdmissionOutcome) -> TxFanoutAdmission {
    TxFanoutAdmission {
        txid: txid(byte),
        wtxid: wtxid(byte.saturating_add(1)),
        outcome,
    }
}

pub(super) fn tx_fanout_policy_honors_identity_and_limits() {
    // Arrange
    let mut queue = TxFanoutQueue::new(TxFanoutPolicy {
        max_queue_per_peer: 1,
        max_drain_per_peer: 16,
        min_interval_seconds: 1,
    });
    let peers = [
        peer(1, TxRelayPeerMode::TxidOnly),
        peer(2, TxRelayPeerMode::WtxidRelay),
    ];
    let first = admission(10, TxFanoutAdmissionOutcome::Accepted);
    let second = admission(12, TxFanoutAdmissionOutcome::Replaced);

    // Act
    let first_actions = queue.enqueue_admission(first, &peers);
    let second_actions = queue.enqueue_admission(second, &peers);
    let txid_actions = queue.drain_peer(1, 100);
    let wtxid_actions = queue.drain_peer(2, 100);

    // Assert
    assert!(first_actions.is_empty());
    assert_eq!(
        second_actions,
        [
            TxFanoutAction::QueueCap {
                peer_id: 1,
                relay_id: TxRelayId::Txid(second.txid),
            },
            TxFanoutAction::QueueCap {
                peer_id: 2,
                relay_id: TxRelayId::Wtxid(second.wtxid),
            },
        ],
    );
    assert_eq!(
        txid_actions,
        [TxFanoutAction::Announce {
            peer_id: 1,
            relay_id: TxRelayId::Txid(first.txid),
        }],
    );
    assert_eq!(
        wtxid_actions,
        [TxFanoutAction::Announce {
            peer_id: 2,
            relay_id: TxRelayId::Wtxid(first.wtxid),
        }],
    );
    assert_eq!(
        TxFanoutAction::QueueCap {
            peer_id: 1,
            relay_id: TxRelayId::Txid(second.txid)
        }
        .as_str(),
        "queue_cap"
    );
    assert_eq!(queue.snapshot().queued_count, 0);
}

pub(super) fn tx_fanout_policy_suppresses_origin_and_ineligible_peers() {
    // Arrange
    let mut queue = TxFanoutQueue::default();
    let relay = admission(20, TxFanoutAdmissionOutcome::Accepted);
    let mut origin = peer(1, TxRelayPeerMode::TxidOnly);
    origin.origin_peer = true;
    let mut ineligible = peer(2, TxRelayPeerMode::TxidOnly);
    ineligible.relay_eligibility = disabled_relay();
    let mut recent_reject = peer(3, TxRelayPeerMode::TxidOnly);
    recent_reject.recent_reject = true;
    let mut in_flight = peer(4, TxRelayPeerMode::TxidOnly);
    in_flight.in_flight = true;
    let mut mempool_known = peer(5, TxRelayPeerMode::TxidOnly);
    mempool_known.mempool_known = true;

    // Act
    let actions = queue.enqueue_admission(
        relay,
        &[origin, ineligible, recent_reject, in_flight, mempool_known],
    );
    let reasons = actions
        .iter()
        .map(|action| match action {
            TxFanoutAction::Suppress { reason, .. } => reason.as_str(),
            _ => action.as_str(),
        })
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        reasons,
        [
            "origin_peer",
            "not_relay_eligible",
            "recent_reject",
            "in_flight",
            "mempool_known",
        ],
    );
    assert_eq!(queue.snapshot().queued_count, 0);
}

pub(super) fn tx_fanout_policy_reports_rebroadcast_deferred_without_timer() {
    // Arrange
    let accepted = admission(30, TxFanoutAdmissionOutcome::Accepted);
    let replaced = admission(32, TxFanoutAdmissionOutcome::Replaced);
    let mut queue = TxFanoutQueue::new(TxFanoutPolicy {
        max_queue_per_peer: 4,
        max_drain_per_peer: 1,
        min_interval_seconds: 3,
    });
    let peer = peer(7, TxRelayPeerMode::TxidOnly);

    // Act
    let accepted_action = defer_local_rebroadcast(accepted, true, true);
    let replaced_action = defer_local_rebroadcast(replaced, true, true);
    let remote_action = defer_local_rebroadcast(accepted, false, true);
    let disabled_action = defer_local_rebroadcast(accepted, true, false);
    queue.enqueue_admission(accepted, std::slice::from_ref(&peer));
    let first_drain = queue.drain_peer(7, 10);
    queue.enqueue_admission(replaced, std::slice::from_ref(&peer));
    let rate_limited = queue.drain_peer(7, 11);
    let second_drain = queue.drain_peer(7, 13);

    // Assert
    assert_eq!(
        accepted_action,
        Some(TxFanoutAction::RebroadcastDeferred {
            relay_id: TxRelayId::Txid(accepted.txid),
        }),
    );
    assert_eq!(
        accepted_action.map(TxFanoutAction::as_str),
        Some("rebroadcast_deferred"),
    );
    assert_eq!(replaced_action, None);
    assert_eq!(remote_action, None);
    assert_eq!(disabled_action, None);
    assert_eq!(first_drain.len(), 1);
    assert_eq!(
        rate_limited,
        [TxFanoutAction::RateLimit {
            peer_id: 7,
            ready_at_unix_seconds: 13,
        }],
    );
    assert_eq!(
        second_drain,
        [TxFanoutAction::Announce {
            peer_id: 7,
            relay_id: TxRelayId::Txid(replaced.txid),
        }],
    );
}

pub(super) fn tx_fanout_policy_covers_fixed_labels_and_suppression_edges() {
    // Arrange
    let relay = admission(40, TxFanoutAdmissionOutcome::Accepted);
    let zero_txid = TxFanoutAdmission {
        txid: txid(0),
        wtxid: wtxid(41),
        outcome: TxFanoutAdmissionOutcome::Accepted,
    };
    let zero_wtxid = TxFanoutAdmission {
        txid: txid(42),
        wtxid: wtxid(0),
        outcome: TxFanoutAdmissionOutcome::Accepted,
    };
    let mut already_have = peer(1, TxRelayPeerMode::TxidOnly);
    already_have.already_have = true;
    let mut relay_disabled = peer(2, TxRelayPeerMode::TxidOnly);
    relay_disabled.relay_eligibility = relay_disabled_after_eligibility();
    let labels_action_relay = TxRelayId::Txid(relay.txid);
    let labels = [
        TxFanoutAdmissionOutcome::Accepted.as_str(),
        TxFanoutAdmissionOutcome::Replaced.as_str(),
    ];
    let suppression_labels = [
        TxFanoutSuppressionReason::OriginPeer,
        TxFanoutSuppressionReason::AlreadyHave,
        TxFanoutSuppressionReason::RecentReject,
        TxFanoutSuppressionReason::InFlight,
        TxFanoutSuppressionReason::MempoolKnown,
        TxFanoutSuppressionReason::RelayDisabled,
        TxFanoutSuppressionReason::NotRelayEligible,
        TxFanoutSuppressionReason::QueueCapReached,
        TxFanoutSuppressionReason::RateLimited,
        TxFanoutSuppressionReason::IdentityUnavailable,
    ]
    .map(TxFanoutSuppressionReason::as_str);
    let cleanup_labels = [
        TxFanoutCleanupReason::Confirmed,
        TxFanoutCleanupReason::Replaced,
        TxFanoutCleanupReason::Evicted,
        TxFanoutCleanupReason::Expired,
        TxFanoutCleanupReason::PeerDisconnected,
    ]
    .map(TxFanoutCleanupReason::as_str);
    let action_labels = [
        TxFanoutAction::Announce {
            peer_id: 1,
            relay_id: labels_action_relay,
        },
        TxFanoutAction::Suppress {
            peer_id: 1,
            relay_id: labels_action_relay,
            reason: TxFanoutSuppressionReason::AlreadyHave,
        },
        TxFanoutAction::QueueCap {
            peer_id: 1,
            relay_id: labels_action_relay,
        },
        TxFanoutAction::RateLimit {
            peer_id: 1,
            ready_at_unix_seconds: 2,
        },
        TxFanoutAction::Cleanup {
            relay_id: labels_action_relay,
            reason: TxFanoutCleanupReason::Confirmed,
        },
        TxFanoutAction::RebroadcastDeferred {
            relay_id: labels_action_relay,
        },
    ]
    .map(TxFanoutAction::as_str);

    // Act
    let suppressions = TxFanoutQueue::default().enqueue_admission(
        relay,
        &[
            already_have,
            relay_disabled,
            peer(3, TxRelayPeerMode::WtxidRelay),
        ],
    );
    let txid_identity_unavailable = TxFanoutQueue::default()
        .enqueue_admission(zero_txid, &[peer(4, TxRelayPeerMode::TxidOnly)]);
    let wtxid_identity_unavailable = TxFanoutQueue::default()
        .enqueue_admission(zero_wtxid, &[peer(5, TxRelayPeerMode::WtxidRelay)]);

    // Assert
    assert_eq!(labels, ["accepted", "replaced"]);
    assert_eq!(
        suppression_labels,
        [
            "origin_peer",
            "already_have",
            "recent_reject",
            "in_flight",
            "mempool_known",
            "relay_disabled",
            "not_relay_eligible",
            "queue_cap_reached",
            "rate_limited",
            "identity_unavailable",
        ],
    );
    assert_eq!(
        cleanup_labels,
        [
            "confirmed",
            "replaced",
            "evicted",
            "expired",
            "peer_disconnected",
        ],
    );
    assert_eq!(
        action_labels,
        [
            "announce",
            "suppress",
            "queue_cap",
            "rate_limit",
            "cleanup",
            "rebroadcast_deferred",
        ],
    );
    assert_eq!(
        suppressions[..2],
        [
            TxFanoutAction::Suppress {
                peer_id: 1,
                relay_id: TxRelayId::Txid(relay.txid),
                reason: TxFanoutSuppressionReason::AlreadyHave,
            },
            TxFanoutAction::Suppress {
                peer_id: 2,
                relay_id: TxRelayId::Txid(relay.txid),
                reason: TxFanoutSuppressionReason::RelayDisabled,
            },
        ],
    );
    assert!(matches!(
        txid_identity_unavailable.as_slice(),
        [TxFanoutAction::Suppress {
            reason: TxFanoutSuppressionReason::IdentityUnavailable,
            ..
        }]
    ));
    assert!(matches!(
        wtxid_identity_unavailable.as_slice(),
        [TxFanoutAction::Suppress {
            reason: TxFanoutSuppressionReason::IdentityUnavailable,
            ..
        }]
    ));
}

pub(super) fn tx_fanout_policy_cleans_up_duplicates_and_empty_state() {
    // Arrange
    let mut queue = TxFanoutQueue::default();
    let first = admission(50, TxFanoutAdmissionOutcome::Accepted);
    let second = admission(52, TxFanoutAdmissionOutcome::Accepted);
    let peers = [
        peer(10, TxRelayPeerMode::TxidOnly),
        peer(11, TxRelayPeerMode::WtxidRelay),
    ];

    // Act
    let missing_peer_drain = queue.drain_peer(99, 1);
    let missing_relay_cleanup =
        queue.cleanup_relay_id(TxRelayId::Txid(txid(99)), TxFanoutCleanupReason::Expired);
    let missing_peer_cleanup = queue.cleanup_peer(99, TxFanoutCleanupReason::PeerDisconnected);
    let first_enqueue = queue.enqueue_admission(first, &peers);
    let duplicate_enqueue = queue.enqueue_admission(first, &peers);
    let transaction_cleanup =
        queue.cleanup_transaction(first.txid, first.wtxid, TxFanoutCleanupReason::Confirmed);
    let empty_drain = queue.drain_peer(10, 1);
    queue.enqueue_admission(second, &peers);
    let peer_cleanup = queue.cleanup_peer(11, TxFanoutCleanupReason::PeerDisconnected);

    // Assert
    assert!(missing_peer_drain.is_empty());
    assert!(missing_relay_cleanup.is_empty());
    assert!(missing_peer_cleanup.is_empty());
    assert!(first_enqueue.is_empty());
    assert_eq!(
        duplicate_enqueue,
        [
            TxFanoutAction::Suppress {
                peer_id: 10,
                relay_id: TxRelayId::Txid(first.txid),
                reason: TxFanoutSuppressionReason::AlreadyHave,
            },
            TxFanoutAction::Suppress {
                peer_id: 11,
                relay_id: TxRelayId::Wtxid(first.wtxid),
                reason: TxFanoutSuppressionReason::AlreadyHave,
            },
        ],
    );
    assert_eq!(
        transaction_cleanup,
        [
            TxFanoutAction::Cleanup {
                relay_id: TxRelayId::Txid(first.txid),
                reason: TxFanoutCleanupReason::Confirmed,
            },
            TxFanoutAction::Cleanup {
                relay_id: TxRelayId::Wtxid(first.wtxid),
                reason: TxFanoutCleanupReason::Confirmed,
            },
        ],
    );
    assert!(empty_drain.is_empty());
    assert_eq!(
        peer_cleanup,
        [TxFanoutAction::Cleanup {
            relay_id: TxRelayId::Wtxid(second.wtxid),
            reason: TxFanoutCleanupReason::PeerDisconnected,
        }],
    );
}
