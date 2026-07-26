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

pub(super) fn received_transaction_cleanup_waits_for_admission_before_already_have() {
    // Arrange
    let mut scheduler = scheduler();
    let txid = txid(22);
    let wtxid = wtxid(23);
    let _ = scheduler.record_announcement(announcement(
        22,
        TxRelayId::Txid(txid).to_inventory_vector(),
        TxRelayPeerMode::TxidOnly,
        0,
    ));
    let _ = scheduler.record_announcement(announcement(
        23,
        TxRelayId::Wtxid(wtxid).to_inventory_vector(),
        TxRelayPeerMode::WtxidRelay,
        0,
    ));

    // Act
    let cleanup = scheduler.record_received_transaction(22, txid, wtxid);
    let txid_again = scheduler.record_announcement(announcement(
        24,
        TxRelayId::Txid(txid).to_inventory_vector(),
        TxRelayPeerMode::TxidOnly,
        1,
    ));
    let wtxid_again = scheduler.record_announcement(announcement(
        25,
        TxRelayId::Wtxid(wtxid).to_inventory_vector(),
        TxRelayPeerMode::WtxidRelay,
        1,
    ));

    // Assert
    assert_eq!(
        cleanup,
        [TxDownloadAction::ReceivedTxCleanup {
            peer_id: 22,
            txid,
            wtxid,
        }],
    );
    assert_eq!(scheduler.snapshot().already_have_count, 0);
    assert_eq!(txid_again, [request(24, TxRelayId::Txid(txid))]);
    assert_eq!(wtxid_again, [request(25, TxRelayId::Wtxid(wtxid))]);
}

pub(super) fn different_deliverer_receipt_unions_and_orders_announcers() {
    // Arrange
    let mut scheduler = scheduler();
    let txid = txid(30);
    let wtxid = wtxid(31);
    for (peer_id, relay_id, peer_mode) in [
        (30, TxRelayId::Txid(txid), TxRelayPeerMode::TxidOnly),
        (40, TxRelayId::Txid(txid), TxRelayPeerMode::TxidOnly),
        (20, TxRelayId::Wtxid(wtxid), TxRelayPeerMode::WtxidRelay),
        (10, TxRelayId::Wtxid(wtxid), TxRelayPeerMode::WtxidRelay),
    ] {
        let _ = scheduler.record_announcement(announcement(
            peer_id,
            relay_id.to_inventory_vector(),
            peer_mode,
            0,
        ));
    }

    // Act
    let receipt = scheduler.record_received_transaction_with_provenance(99, txid, wtxid);

    // Assert
    assert_eq!(
        receipt.maybe_provenance,
        Some(ReceivedTransactionProvenance {
            delivered_by: 99,
            announcers: vec![10, 20, 30, 40, 99],
        }),
    );
}

pub(super) fn receipt_provenance_deduplicates_delivered_by_announcer() {
    // Arrange
    let mut scheduler = scheduler();
    let txid = txid(32);
    let wtxid = wtxid(33);
    let _ = scheduler.record_announcement(announcement(
        32,
        TxRelayId::Txid(txid).to_inventory_vector(),
        TxRelayPeerMode::TxidOnly,
        0,
    ));
    let _ = scheduler.record_announcement(announcement(
        32,
        TxRelayId::Wtxid(wtxid).to_inventory_vector(),
        TxRelayPeerMode::WtxidRelay,
        0,
    ));

    // Act
    let receipt = scheduler.record_received_transaction_with_provenance(32, txid, wtxid);

    // Assert
    assert_eq!(
        receipt.maybe_provenance,
        Some(ReceivedTransactionProvenance {
            delivered_by: 32,
            announcers: vec![32],
        }),
    );
}
