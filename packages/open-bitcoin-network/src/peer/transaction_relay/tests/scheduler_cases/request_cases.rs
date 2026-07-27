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

pub(super) fn txid_announcement_requests_transaction_inventory() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(1);

    // Act
    let actions = announce_txid(&mut scheduler, 1, 1, 0);

    // Assert
    assert_eq!(actions, [request(1, relay_id)]);
    assert_eq!(
        actions[0].maybe_request_inventory(),
        Some(txid_inventory(1))
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 1,
            already_have_count: 0,
        },
    );
    assert_eq!(scheduler.peer_snapshot(1).in_flight_count, 1);
}

pub(super) fn wtxid_announcement_requests_witness_transaction_inventory() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = wtxid_relay(2);

    // Act
    let actions = announce_wtxid(&mut scheduler, 2, 2, 0);

    // Assert
    assert_eq!(actions, [request(2, relay_id)]);
    assert_eq!(
        actions[0].maybe_request_inventory(),
        Some(wtxid_inventory(2)),
    );
    assert_eq!(scheduler.peer_snapshot(2).in_flight_count, 1);
}

pub(super) fn semantic_reject_facts_suppress_inventory_while_parent_bypasses_reconsiderable() {
    // Arrange
    let mut scheduler = scheduler();

    // Act
    let already_have = scheduler.record_announcement(TxAnnouncementInput {
        local_facts: TxDownloadLocalFacts {
            already_have: true,
            ..TxDownloadLocalFacts::default()
        },
        ..announcement(6, txid_inventory(6), TxRelayPeerMode::TxidOnly, 0)
    });
    let hard_reject = scheduler.record_announcement(TxAnnouncementInput {
        local_facts: TxDownloadLocalFacts {
            hard_rejected: true,
            ..TxDownloadLocalFacts::default()
        },
        ..announcement(7, txid_inventory(7), TxRelayPeerMode::TxidOnly, 0)
    });
    let reconsiderable = scheduler.record_announcement(TxAnnouncementInput {
        local_facts: TxDownloadLocalFacts {
            reconsiderable: true,
            ..TxDownloadLocalFacts::default()
        },
        ..announcement(8, txid_inventory(8), TxRelayPeerMode::TxidOnly, 0)
    });
    let mempool_known = scheduler.record_announcement(TxAnnouncementInput {
        local_facts: TxDownloadLocalFacts {
            mempool_known: true,
            ..TxDownloadLocalFacts::default()
        },
        ..announcement(9, txid_inventory(9), TxRelayPeerMode::TxidOnly, 0)
    });
    let reconsiderable_parent = scheduler.request_parent(TxParentRequestInput {
        local_facts: TxDownloadLocalFacts {
            reconsiderable: true,
            ..TxDownloadLocalFacts::default()
        },
        ..parent_request(10, txid_relay(10), 0)
    });
    let hard_rejected_parent = scheduler.request_parent(TxParentRequestInput {
        local_facts: TxDownloadLocalFacts {
            hard_rejected: true,
            ..TxDownloadLocalFacts::default()
        },
        ..parent_request(11, txid_relay(11), 0)
    });

    // Assert
    assert_eq!(already_have, [expect_already_have(6, txid_relay(6))]);
    assert_eq!(hard_reject, [expect_recent_reject(7, txid_relay(7))]);
    assert_eq!(reconsiderable, [expect_recent_reject(8, txid_relay(8))]);
    assert_eq!(
        mempool_known,
        [TxDownloadAction::Suppress {
            peer_id: 9,
            relay_id: txid_relay(9),
            reason: TxDownloadSuppressionReason::MempoolKnown,
        }],
    );
    assert_eq!(
        mempool_known[0].suppression_reason(),
        Some(TxDownloadSuppressionReason::MempoolKnown),
    );
    assert_eq!(
        reconsiderable_parent,
        [request(10, txid_relay(10))],
        "parent requests must set include_reconsiderable=false",
    );
    assert_eq!(
        hard_rejected_parent,
        [expect_recent_reject(11, txid_relay(11))],
    );
}
