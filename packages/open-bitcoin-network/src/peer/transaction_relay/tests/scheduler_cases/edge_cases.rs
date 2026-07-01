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
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use super::*;

#[test]
fn duplicate_fallback_candidate_respects_peer_candidate_cap() {
    // Arrange
    let mut scheduler = scheduler();
    let _ = announce_txid(&mut scheduler, 40, 40, 0);
    for byte in 41..44 {
        assert!(
            scheduler
                .record_announcement(not_preferred(announcement(
                    41,
                    txid_inventory(byte),
                    TxRelayPeerMode::TxidOnly,
                    1,
                )))
                .is_empty()
        );
    }

    // Act
    let actions = announce_txid(&mut scheduler, 41, 40, 2);

    // Assert
    assert_eq!(actions, [duplicate(41, txid_relay(40))]);
    assert_eq!(scheduler.peer_snapshot(41).candidate_count, 3);
}

#[test]
fn duplicate_orphan_parent_fallback_candidate_respects_peer_candidate_cap() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(45);
    let _ = scheduler.request_parent(45, relay_id, 0, TxDownloadLocalFacts::default());
    for byte in 46..49 {
        assert!(
            scheduler
                .record_announcement(not_preferred(announcement(
                    46,
                    txid_inventory(byte),
                    TxRelayPeerMode::TxidOnly,
                    1,
                )))
                .is_empty()
        );
    }

    // Act
    let actions = scheduler.request_parent(46, relay_id, 2, TxDownloadLocalFacts::default());

    // Assert
    assert_eq!(actions, [duplicate(46, relay_id)]);
    assert_eq!(scheduler.peer_snapshot(46).candidate_count, 3);
}

#[test]
fn notfound_missing_or_wrong_peer_is_noop() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(18);
    let _ = announce_txid(&mut scheduler, 18, 18, 0);
    let _ = announce_txid(&mut scheduler, 19, 18, 1);
    let _ = scheduler.record_notfound(18, relay_id, 5);

    // Act
    let wrong_peer = scheduler.record_notfound(18, relay_id, 6);
    let missing_relay = scheduler.record_notfound(18, txid_relay(99), 6);

    // Assert
    assert!(wrong_peer.is_empty());
    assert!(missing_relay.is_empty());
}

#[test]
fn cleanup_peer_handles_empty_and_candidate_only_state() {
    // Arrange
    let mut scheduler = scheduler();
    assert!(
        scheduler
            .record_announcement(not_preferred(announcement(
                30,
                txid_inventory(30),
                TxRelayPeerMode::TxidOnly,
                0,
            )))
            .is_empty()
    );

    // Act
    let candidate_cleanup = scheduler.cleanup_peer(30, 1);
    let empty_cleanup = scheduler.cleanup_peer(30, 1);

    // Assert
    assert_eq!(
        candidate_cleanup,
        [TxDownloadAction::PeerCleanup { peer_id: 30 }],
    );
    assert!(empty_cleanup.is_empty());
}
