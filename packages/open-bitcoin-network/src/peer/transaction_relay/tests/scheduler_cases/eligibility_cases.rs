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

pub(super) fn identity_mismatch_suppresses_without_candidate_or_inflight_state() {
    // Arrange
    let mut scheduler = scheduler();
    let non_transaction_inventory = InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: Hash32::from_byte_array([3; 32]),
    };

    // Act
    let mismatch_actions = scheduler.record_announcement(announcement(
        3,
        txid_inventory(3),
        TxRelayPeerMode::WtxidRelay,
        0,
    ));
    let non_transaction_actions = scheduler.record_announcement(announcement(
        3,
        non_transaction_inventory,
        TxRelayPeerMode::TxidOnly,
        0,
    ));

    // Assert
    assert_eq!(
        mismatch_actions,
        [TxDownloadAction::SuppressIdentityMismatch {
            peer_id: 3,
            reason: TxDownloadSuppressionReason::IdentityMismatch,
        }],
    );
    assert_eq!(
        non_transaction_actions,
        [TxDownloadAction::SuppressIdentityMismatch {
            peer_id: 3,
            reason: TxDownloadSuppressionReason::NotTransactionInventory,
        }],
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 0,
            already_have_count: 0,
        },
    );
}

pub(super) fn disabled_relay_suppresses_announcement_without_request_state() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(22);

    // Act
    let actions = scheduler.record_announcement(with_relay_eligibility(
        announcement(22, txid_inventory(22), TxRelayPeerMode::TxidOnly, 0),
        relay_disabled(),
    ));

    // Assert
    assert_eq!(
        actions,
        [suppress(
            22,
            relay_id,
            TxDownloadSuppressionReason::RelayDisabled,
        )],
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 0,
            already_have_count: 0,
        },
    );
}

pub(super) fn ineligible_relay_suppressions_are_typed_without_request_state() {
    // Arrange
    let relay_id = txid_relay(23);
    let cases = [
        (
            inbound_serving_required(),
            TxDownloadSuppressionReason::InboundServingRequired,
        ),
        (
            permission_required(),
            TxDownloadSuppressionReason::PermissionRequired,
        ),
        (
            protected_not_relay(),
            TxDownloadSuppressionReason::ProtectedNotRelay,
        ),
    ];

    for (relay_eligibility, expected_reason) in cases {
        let mut scheduler = scheduler();

        // Act
        let actions = scheduler.record_announcement(with_relay_eligibility(
            announcement(23, txid_inventory(23), TxRelayPeerMode::TxidOnly, 0),
            relay_eligibility,
        ));

        // Assert
        assert_eq!(actions, [suppress(23, relay_id, expected_reason)]);
        assert_eq!(
            scheduler.snapshot(),
            TxDownloadSnapshot {
                candidate_count: 0,
                in_flight_count: 0,
                already_have_count: 0,
            },
        );
    }
}

pub(super) fn ineligible_eligible_reason_maps_to_not_relay_eligible() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(26);

    // Act
    let actions = scheduler.record_announcement(with_relay_eligibility(
        announcement(26, txid_inventory(26), TxRelayPeerMode::TxidOnly, 0),
        ineligible_with_eligible_reason(),
    ));

    // Assert
    assert_eq!(
        actions,
        [suppress(
            26,
            relay_id,
            TxDownloadSuppressionReason::NotRelayEligible,
        )],
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 0,
            already_have_count: 0,
        },
    );
}

pub(super) fn disabled_parent_request_suppresses_without_request_state() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(24);

    // Act
    let actions = scheduler.request_parent(parent_with_relay_eligibility(
        parent_request(24, relay_id, 0),
        relay_disabled(),
    ));

    // Assert
    assert_eq!(
        actions,
        [suppress(
            24,
            relay_id,
            TxDownloadSuppressionReason::RelayDisabled,
        )],
    );
    assert_eq!(
        scheduler.snapshot(),
        TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 0,
            already_have_count: 0,
        },
    );
}

pub(super) fn ineligible_first_announcement_does_not_block_eligible_second_announcer() {
    // Arrange
    let mut scheduler = scheduler();
    let relay_id = txid_relay(25);

    // Act
    let first_actions = scheduler.record_announcement(with_relay_eligibility(
        announcement(25, txid_inventory(25), TxRelayPeerMode::TxidOnly, 0),
        permission_required(),
    ));
    let second_actions = announce_txid(&mut scheduler, 26, 25, 1);

    // Assert
    assert_eq!(
        first_actions,
        [suppress(
            25,
            relay_id,
            TxDownloadSuppressionReason::PermissionRequired,
        )],
    );
    assert_eq!(second_actions, [request(26, relay_id)]);
    assert_eq!(scheduler.peer_snapshot(25).candidate_count, 0);
    assert_eq!(scheduler.peer_snapshot(25).in_flight_count, 0);
    assert_eq!(scheduler.peer_snapshot(26).in_flight_count, 1);
}
