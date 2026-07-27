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
fn managed_admission_bridge_resource_caps_preserved_under_orphan_burst() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(622, 6, PolicyConfig::default());
    network.with_orphan_policy(test_orphan_policy(3, 3));
    network.add_inbound_peer(622).expect("peer");
    let orphan_children = coinbase_txids
        .iter()
        .take(5)
        .map(|coinbase_txid| parent_and_child(*coinbase_txid).1)
        .collect::<Vec<_>>();

    // Act
    let results = orphan_children
        .into_iter()
        .enumerate()
        .map(|(index, child)| {
            network
                .process_peer_transaction_admission(
                    622,
                    child,
                    600 + index as i64,
                    verify_flags(),
                    consensus_params(),
                )
                .expect("orphan burst")
        })
        .collect::<Vec<_>>();
    let request_snapshot = network.peer_manager().transaction_request_snapshot(622);
    let evicted_count = results
        .iter()
        .flat_map(|result| result.reconsidered.iter())
        .filter(|outcome| matches!(outcome, MempoolOutcome::Evicted { .. }))
        .count();

    // Assert
    assert_eq!(network.orphan_count(), 3);
    assert!(evicted_count >= 2);
    assert!(request_snapshot.in_flight_count <= PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER);
}

#[test]
fn managed_admission_bridge_orphaned_peer_tx_feeds_compact_extra_txn() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(119_301, 2, PolicyConfig::default());
    network.connect_outbound_peer(119_301, 0).expect("peer");
    let (_parent, child) = parent_and_child(coinbase_txids[0]);
    assert_eq!(network.compact_extra_txn_len(), 0);

    // Act
    let result = network
        .process_peer_transaction_admission(119_301, child, 10, verify_flags(), consensus_params())
        .expect("orphan outcome");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Orphaned { .. }));
    assert_eq!(
        network.compact_extra_txn_len(),
        1,
        "orphaned staged body must push into CompactExtraTxnBuffer"
    );
}

#[test]
fn managed_admission_bridge_rejected_peer_tx_feeds_compact_extra_txn_gated() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(119_302, 3, PolicyConfig::default());
    network.add_inbound_peer(119_302).expect("peer");
    let rejected = low_fee_spend(coinbase_txids[1]);
    assert_eq!(network.compact_extra_txn_len(), 0);

    // Act
    let result = network
        .process_peer_transaction_admission(
            119_302,
            rejected,
            20,
            verify_flags(),
            consensus_params(),
        )
        .expect("rejected");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Rejected { .. }));
    assert_eq!(
        network.compact_extra_txn_len(),
        1,
        "rejected body under per-tx size gate must push_gated into CompactExtraTxnBuffer"
    );
}

#[test]
fn managed_admission_bridge_replaced_victims_feed_compact_extra_txn() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(119_303, 2, PolicyConfig::default());
    network.add_inbound_peer(119_303).expect("peer");
    let original = spend_transaction(coinbase_txids[0], 499_999_000);
    let replacement = spend_transaction(coinbase_txids[0], 499_996_000);
    network
        .process_peer_transaction_admission(
            119_303,
            original,
            30,
            verify_flags(),
            consensus_params(),
        )
        .expect("original");
    let len_before_replace = network.compact_extra_txn_len();

    // Act
    let result = network
        .process_peer_transaction_admission(
            119_303,
            replacement,
            31,
            verify_flags(),
            consensus_params(),
        )
        .expect("replacement");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Replaced { .. }));
    assert!(
        network.compact_extra_txn_len() > len_before_replace,
        "replaced victim bodies must push into CompactExtraTxnBuffer before demotion"
    );
}
