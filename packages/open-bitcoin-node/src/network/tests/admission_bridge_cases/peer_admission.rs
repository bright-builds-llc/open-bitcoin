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
fn managed_admission_bridge_peer_tx_uses_download_boundary_before_mempool() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(610, 2, PolicyConfig::default());
    network.connect_outbound_peer(610, 0).expect("peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let inventory = transaction_relay_inventory(&transaction);

    // Act
    let inventory_outbound = network
        .receive_message(
            610,
            WireNetworkMessage::Inv(inventory.clone()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("inventory")
        .outbound;
    let count_before_tx = network.mempool_info().transaction_count;
    let result = network
        .receive_sync_message(
            610,
            WireNetworkMessage::Tx(transaction.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("tx");

    // Assert
    assert_getdata(&inventory_outbound, inventory);
    assert_eq!(count_before_tx, 0);
    assert!(result.outbound.is_empty());
    assert!(result.targeted_outbound.is_empty());
    assert_mempool_contains(&network, txid(&transaction));
}

#[test]
fn managed_admission_bridge_peer_missing_parent_stages_orphan_and_requests_parent() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(611, 2, PolicyConfig::default());
    network.connect_outbound_peer(611, 0).expect("peer");
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let parent_txid = txid(&parent);

    // Act
    let result = network
        .process_peer_transaction_admission(
            611,
            child.clone(),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("orphan outcome");

    // Assert
    assert!(matches!(
        result.outcome,
        MempoolOutcome::Orphaned {
            missing_parents,
            ..
        } if missing_parents == vec![parent_txid]
    ));
    assert_eq!(network.orphan_count(), 1);
    assert_targeted_getdata(&result.targeted_outbound, 611, txid_inventory(parent_txid));
    assert_not_stored(&network, txid(&child));
}

#[test]
fn managed_admission_bridge_parent_acceptance_reconsiders_child() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(612, 2, PolicyConfig::default());
    network.add_inbound_peer(612).expect("peer");
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let parent_txid = txid(&parent);
    let child_txid = txid(&child);
    network
        .process_peer_transaction_admission(612, child, 10, verify_flags(), consensus_params())
        .expect("stage child");

    // Act
    let result = network
        .process_peer_transaction_admission(612, parent, 11, verify_flags(), consensus_params())
        .expect("accept parent");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Accepted { txid, .. } if txid == parent_txid));
    assert!(result.reconsidered.iter().any(
        |outcome| matches!(outcome, MempoolOutcome::Accepted { txid, .. } if *txid == child_txid)
    ));
    assert_eq!(network.orphan_count(), 0);
    assert_mempool_contains(&network, parent_txid);
    assert_mempool_contains(&network, child_txid);
}

#[test]
fn managed_admission_bridge_peer_admission_preserves_receive_metadata() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(623, 2, PolicyConfig::default());
    network.add_inbound_peer(623).expect("peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);

    // Act
    network
        .process_peer_transaction_admission(
            623,
            transaction,
            42,
            verify_flags(),
            consensus_params(),
        )
        .expect("peer admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("accepted peer transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(42))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Peer);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}

#[test]
fn managed_admission_bridge_reconsidered_orphan_uses_reconsideration_metadata() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(624, 2, PolicyConfig::default());
    network.add_inbound_peer(624).expect("peer");
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let child_txid = txid(&child);
    network
        .process_peer_transaction_admission(624, child, 41, verify_flags(), consensus_params())
        .expect("stage child");

    // Act
    network
        .process_peer_transaction_admission(624, parent, 43, verify_flags(), consensus_params())
        .expect("accept parent and reconsider child");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&child_txid)
        .expect("reconsidered child")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(43))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Peer);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}

#[test]
fn managed_admission_bridge_peer_duplicate_preserves_first_metadata() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(625, 2, PolicyConfig::default());
    network.add_inbound_peer(625).expect("peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    network
        .process_peer_transaction_admission(
            625,
            transaction.clone(),
            42,
            verify_flags(),
            consensus_params(),
        )
        .expect("first admission");

    // Act
    network
        .process_peer_transaction_admission(
            625,
            transaction,
            44,
            verify_flags(),
            consensus_params(),
        )
        .expect("duplicate admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("original peer transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(42))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Peer);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}
