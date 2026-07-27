// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use super::*;

#[test]
fn managed_admission_bridge_duplicate_and_rejected_peer_txs_do_not_store_transaction() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(617, 3, PolicyConfig::default());
    network.add_inbound_peer(617).expect("peer");
    let accepted = spend_transaction(coinbase_txids[0], 499_999_000);
    let rejected = low_fee_spend(coinbase_txids[1]);
    let accepted_txid = txid(&accepted);
    let rejected_txid = txid(&rejected);
    network
        .process_peer_transaction_admission(
            617,
            accepted.clone(),
            300,
            verify_flags(),
            consensus_params(),
        )
        .expect("accepted");
    let stored_count = network.transactions_by_txid.len();

    // Act
    let duplicate = network
        .process_peer_transaction_admission(617, accepted, 301, verify_flags(), consensus_params())
        .expect("duplicate");
    let rejected_result = network
        .process_peer_transaction_admission(617, rejected, 302, verify_flags(), consensus_params())
        .expect("rejected");

    // Assert
    assert!(
        matches!(duplicate.outcome, MempoolOutcome::Duplicate { txid } if txid == accepted_txid)
    );
    assert!(
        matches!(rejected_result.outcome, MempoolOutcome::Rejected { txid, .. } if txid == rejected_txid)
    );
    assert_eq!(network.transactions_by_txid.len(), stored_count);
    assert_not_stored(&network, rejected_txid);
}

#[test]
fn managed_admission_bridge_replacement_removes_replaced_indexes() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(618, 2, PolicyConfig::default());
    network.add_inbound_peer(618).expect("peer");
    let original = spend_transaction(coinbase_txids[0], 499_999_000);
    let replacement = spend_transaction(coinbase_txids[0], 499_996_000);
    let original_txid = txid(&original);
    let replacement_txid = txid(&replacement);
    network
        .process_peer_transaction_admission(618, original, 400, verify_flags(), consensus_params())
        .expect("original");

    // Act
    let result = network
        .process_peer_transaction_admission(
            618,
            replacement,
            401,
            verify_flags(),
            consensus_params(),
        )
        .expect("replacement");

    // Assert
    assert!(matches!(
        result.outcome,
        MempoolOutcome::Replaced { txid, ref replaced, .. }
            if txid == replacement_txid && replaced == &vec![original_txid]
    ));
    assert_not_stored(&network, original_txid);
    assert_mempool_contains(&network, replacement_txid);
    assert!(network.transactions_by_txid.contains_key(&replacement_txid));
    assert!(result.delta.removed.iter().any(|removal| {
        removal.member.txid == original_txid
            && removal.cause == MempoolRemovalCause::Replacement
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert!(result.delta.final_membership.iter().any(|state| {
        state.member.txid == original_txid && state.membership == FinalMempoolMembership::Absent
    }));
    assert!(result.delta.final_membership.iter().any(|state| {
        state.member.txid == replacement_txid && state.membership == FinalMempoolMembership::Present
    }));
}

#[test]
fn managed_admission_bridge_disconnect_cleans_peer_orphans_and_request_state() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(619, 2, PolicyConfig::default());
    for peer_id in 619..=621 {
        network
            .connect_outbound_peer(peer_id, 0)
            .expect("relay peer");
    }
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let child_inventory = transaction_relay_inventory(&child);
    network
        .receive_message(
            619,
            WireNetworkMessage::Inv(child_inventory.clone()),
            500,
            verify_flags(),
            consensus_params(),
        )
        .expect("delivering peer inventory");
    network
        .receive_message(
            620,
            WireNetworkMessage::Inv(child_inventory.clone()),
            501,
            verify_flags(),
            consensus_params(),
        )
        .expect("second announcer inventory");
    network
        .receive_sync_message(
            619,
            WireNetworkMessage::Tx(child),
            502,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage child through receipt provenance");
    let before_disconnect = network.peer_manager().transaction_request_snapshot(619);

    // Act
    let first_disconnect = network
        .disconnect_peer_at(619, 503)
        .expect("disconnect cleanup");
    let after_disconnect = network.peer_manager().transaction_request_snapshot(619);
    let late_inventory = network
        .receive_message(
            621,
            WireNetworkMessage::Inv(child_inventory),
            504,
            verify_flags(),
            consensus_params(),
        )
        .expect("late orphan announcer");
    network
        .disconnect_peer_at(620, 505)
        .expect("second announcer disconnect");

    // Assert
    assert_eq!(before_disconnect.in_flight_count, 1);
    assert!(first_disconnect.is_empty());
    assert_eq!(after_disconnect.in_flight_count, 0);
    assert!(late_inventory.outbound.is_empty());
    assert_eq!(network.orphan_count(), 1);
    network
        .disconnect_peer_at(621, 506)
        .expect("late announcer disconnect");
    assert_eq!(network.orphan_count(), 0);
    assert_eq!(network.network_info().connected_peers, 0);
    assert_not_stored(&network, txid(&parent));
}
