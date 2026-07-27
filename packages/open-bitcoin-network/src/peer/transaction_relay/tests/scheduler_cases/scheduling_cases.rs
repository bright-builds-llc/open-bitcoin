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

use super::*;

pub(super) fn duplicate_announcement_retains_fallback_candidate_without_second_request() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(4);

    // Act
    let first_actions = announce_txid(&mut scheduler, 4, 4, 0);
    let duplicate_actions = announce_txid(&mut scheduler, 5, 4, 1);

    // Assert
    assert_eq!(first_actions, [request(4, relay_id)]);
    assert_eq!(duplicate_actions, [duplicate(5, relay_id)]);
    assert_eq!(scheduler.snapshot().candidate_count, 1);
    assert_eq!(scheduler.peer_snapshot(5).candidate_count, 1);
}

pub(super) fn orphan_parent_request_suppresses_duplicate_pending_parent_with_fallback() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(5);

    // Act
    let first_actions = scheduler.request_parent(parent_request(5, relay_id, 0));
    let duplicate_actions = scheduler.request_parent(parent_request(6, relay_id, 1));
    let fallback_actions = scheduler.expire_and_schedule(60);

    // Assert
    assert_eq!(first_actions, [request(5, relay_id)]);
    assert_eq!(duplicate_actions, [duplicate(6, relay_id)]);
    assert_eq!(scheduler.snapshot().in_flight_count, 1);
    assert_eq!(scheduler.peer_snapshot(6).candidate_count, 0);
    assert_eq!(scheduler.peer_snapshot(6).in_flight_count, 1);
    assert_eq!(
        fallback_actions,
        [expect_expired(5, relay_id), fallback(6, relay_id)]
    );
}

pub(super) fn inflight_cap_suppresses_additional_ready_requests() {
    // Arrange
    let mut scheduler = scheduler();

    // Act
    let first_actions = announce_txid(&mut scheduler, 9, 9, 0);
    let capped_actions = announce_txid(&mut scheduler, 9, 10, 1);

    // Assert
    assert_eq!(first_actions.len(), 1);
    assert_eq!(capped_actions, [request_cap(9, txid_relay(10))]);
    assert_eq!(
        scheduler.peer_snapshot(9),
        TxPeerRequestSnapshot {
            peer_id: 9,
            candidate_count: 0,
            in_flight_count: 1,
        },
    );
}
