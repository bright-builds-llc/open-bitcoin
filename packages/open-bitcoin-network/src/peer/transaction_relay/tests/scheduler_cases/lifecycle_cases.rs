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

pub(super) fn txid_delay_waits_until_fake_clock_reaches_ready_time() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(11);
    let _ = announce_wtxid(&mut scheduler, 10, 11, 0);

    // Act
    let delayed = scheduler.record_announcement(announcement(
        11,
        txid_inventory(11),
        TxRelayPeerMode::TxidOnly,
        0,
    ));
    let too_early = scheduler.expire_and_schedule(1);
    let ready = scheduler.expire_and_schedule(2);

    // Assert
    assert!(delayed.is_empty());
    assert!(too_early.is_empty());
    assert_eq!(ready, [request(11, relay_id)]);
}

pub(super) fn non_preferred_peer_delay_waits_until_fake_clock_reaches_ready_time() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(12);

    // Act
    let delayed = scheduler.record_announcement(not_preferred(announcement(
        12,
        txid_inventory(12),
        TxRelayPeerMode::TxidOnly,
        0,
    )));
    let too_early = scheduler.expire_and_schedule(1);
    let ready = scheduler.expire_and_schedule(2);

    // Assert
    assert!(delayed.is_empty());
    assert!(too_early.is_empty());
    assert_eq!(ready, [request(12, relay_id)]);
}

pub(super) fn overloaded_peer_delay_waits_until_fake_clock_reaches_ready_time() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(13);

    // Act
    let delayed = scheduler.record_announcement(overloaded(announcement(
        13,
        txid_inventory(13),
        TxRelayPeerMode::TxidOnly,
        0,
    )));
    let too_early = scheduler.expire_and_schedule(1);
    let ready = scheduler.expire_and_schedule(2);

    // Assert
    assert!(delayed.is_empty());
    assert!(too_early.is_empty());
    assert_eq!(ready, [request(13, relay_id)]);
}

pub(super) fn expiry_fallback_waits_until_fake_clock_reaches_getdata_interval() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(14);
    let _ = announce_txid(&mut scheduler, 14, 14, 0);
    let _ = announce_txid(&mut scheduler, 15, 14, 1);

    // Act
    let too_early = scheduler.expire_and_schedule(59);
    let expired = scheduler.expire_and_schedule(60);

    // Assert
    assert!(too_early.is_empty());
    assert_eq!(
        expired,
        [expect_expired(14, relay_id), fallback(15, relay_id)]
    );
}

pub(super) fn timeout_expires_request_and_falls_back_to_duplicate_announcer() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(16);
    let _ = announce_txid(&mut scheduler, 16, 16, 0);
    let _ = announce_txid(&mut scheduler, 17, 16, 0);

    // Act
    let actions = scheduler.expire_and_schedule(60);

    // Assert
    assert_eq!(
        actions,
        [expect_expired(16, relay_id), fallback(17, relay_id)]
    );
    assert_eq!(scheduler.snapshot().in_flight_count, 1);
    assert_eq!(scheduler.peer_snapshot(17).in_flight_count, 1);
}

pub(super) fn notfound_clears_matching_request_and_falls_back() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(18);
    let _ = announce_txid(&mut scheduler, 18, 18, 0);
    let _ = announce_txid(&mut scheduler, 19, 18, 1);

    // Act
    let actions = scheduler.record_notfound(18, relay_id, 5);

    // Assert
    assert_eq!(
        actions,
        [
            TxDownloadAction::NotFoundCleanup {
                peer_id: 18,
                relay_id,
            },
            fallback(19, relay_id),
        ],
    );
}

pub(super) fn disconnect_cleanup_removes_peer_state_and_falls_back() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(20);
    let _ = announce_txid(&mut scheduler, 20, 20, 0);
    let _ = announce_txid(&mut scheduler, 21, 20, 1);

    // Act
    let actions = scheduler.cleanup_peer(20, 2);

    // Assert
    assert_eq!(
        actions,
        [
            TxDownloadAction::PeerCleanup { peer_id: 20 },
            fallback(21, relay_id),
        ],
    );
    assert_eq!(scheduler.peer_snapshot(20).in_flight_count, 0);
    assert_eq!(scheduler.peer_snapshot(21).in_flight_count, 1);
}
