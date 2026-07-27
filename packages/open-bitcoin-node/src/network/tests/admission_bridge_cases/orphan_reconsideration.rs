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
fn managed_admission_bridge_drains_ready_orphans_after_reconsideration_cap() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(620, 2, PolicyConfig::default());
    network.with_orphan_policy(test_orphan_policy_with_reconsideration_cap(10, 10, 1));
    network.add_inbound_peer(620).expect("peer");
    let parent = spend_transaction(coinbase_txids[0], 499_999_000);
    let parent_txid = txid(&parent);
    let children = vec![
        spend_transaction(parent_txid, 499_998_000),
        spend_transaction(parent_txid, 499_997_000),
        spend_transaction(parent_txid, 499_996_000),
    ];
    let child_txids: Vec<_> = children.iter().map(txid).collect();
    for (index, child) in children.into_iter().enumerate() {
        network
            .process_peer_transaction_admission(
                620,
                child,
                20 + index as i64,
                verify_flags(),
                consensus_params(),
            )
            .expect("stage child");
    }
    assert_eq!(network.orphan_count(), 3);

    // Act
    let result = network
        .process_peer_transaction_admission(620, parent, 30, verify_flags(), consensus_params())
        .expect("accept parent");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Accepted { txid, .. } if txid == parent_txid));
    for child_txid in child_txids {
        assert!(
            result
                .reconsidered
                .iter()
                .any(|outcome| outcome.txid() == child_txid),
            "missing reconsidered outcome for child {child_txid:?}"
        );
    }
    assert_eq!(result.reconsidered.len(), 3);
    assert_eq!(network.orphan_count(), 0);
}

#[test]
fn managed_admission_bridge_still_missing_parent_child_remains_staged() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(613, 3, PolicyConfig::default());
    network.add_inbound_peer(613).expect("peer");
    let first_parent = spend_transaction(coinbase_txids[0], 499_999_000);
    let second_parent = spend_transaction(coinbase_txids[1], 499_999_000);
    let first_parent_txid = txid(&first_parent);
    let second_parent_txid = txid(&second_parent);
    let child = two_input_child(first_parent_txid, second_parent_txid, 499_998_000);
    let staged = network
        .process_peer_transaction_admission(
            613,
            child.clone(),
            20,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage child");
    let MempoolOutcome::Orphaned {
        missing_parents, ..
    } = staged.outcome
    else {
        panic!("expected staged child to be orphaned");
    };
    assert!(missing_parents.len() >= 2);
    let requested_parent = missing_parents[0];
    let accepted_parent = if requested_parent == first_parent_txid {
        first_parent
    } else {
        assert_eq!(requested_parent, second_parent_txid);
        second_parent
    };

    // Act
    let result = network
        .process_peer_transaction_admission(
            613,
            accepted_parent,
            21,
            verify_flags(),
            consensus_params(),
        )
        .expect("accept first parent");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Accepted { .. }));
    assert!(result.reconsidered.is_empty());
    assert_eq!(network.orphan_count(), 1);
    assert_not_stored(&network, txid(&child));
}

#[test]
fn managed_admission_bridge_rejected_child_is_removed_from_orphanage() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(614, 2, PolicyConfig::default());
    network.add_inbound_peer(614).expect("peer");
    let parent = spend_transaction(coinbase_txids[0], 499_999_000);
    let child = spend_transaction(txid(&parent), 500_000_000);
    let child_txid = txid(&child);
    network
        .process_peer_transaction_admission(614, child, 30, verify_flags(), consensus_params())
        .expect("stage child");

    // Act
    let result = network
        .process_peer_transaction_admission(614, parent, 31, verify_flags(), consensus_params())
        .expect("accept parent");

    // Assert
    assert!(result.reconsidered.iter().any(
        |outcome| matches!(outcome, MempoolOutcome::Rejected { txid, .. } if *txid == child_txid)
    ));
    assert_eq!(network.orphan_count(), 0);
    assert_not_stored(&network, child_txid);
}

#[test]
fn managed_admission_bridge_orphan_expiry_returns_expired_outcome_without_sleep() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(615, 2, PolicyConfig::default());
    network.with_orphan_policy(test_orphan_policy(4, 4));
    network.add_inbound_peer(615).expect("peer");
    let (parent, child) = parent_and_child(coinbase_txids[0]);
    let child_txid = txid(&child);
    network
        .process_peer_transaction_admission(615, child, 100, verify_flags(), consensus_params())
        .expect("stage child");

    // Act
    let outcomes = network.expire_orphan_transactions(102);

    // Assert
    assert!(outcomes.iter().any(
        |outcome| matches!(outcome, MempoolOutcome::Expired { txid, .. } if *txid == child_txid)
    ));
    assert_eq!(network.orphan_count(), 0);
    assert_not_stored(&network, txid(&parent));
}

#[test]
fn managed_admission_bridge_orphan_cap_eviction_returns_evicted_outcome() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(616, 3, PolicyConfig::default());
    network.with_orphan_policy(test_orphan_policy(1, 1));
    network.add_inbound_peer(616).expect("peer");
    let (first_parent, first_child) = parent_and_child(coinbase_txids[0]);
    let (second_parent, second_child) = parent_and_child(coinbase_txids[1]);
    let first_child_txid = txid(&first_child);

    // Act
    network
        .process_peer_transaction_admission(
            616,
            first_child,
            200,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage first child");
    let result = network
        .process_peer_transaction_admission(
            616,
            second_child,
            201,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage second child");

    // Assert
    assert!(matches!(result.outcome, MempoolOutcome::Orphaned { .. }));
    assert!(
        result
            .reconsidered
            .iter()
            .any(|outcome| matches!(outcome, MempoolOutcome::Evicted { txid, .. } if *txid == first_child_txid))
    );
    assert_eq!(network.orphan_count(), 1);
    assert_not_stored(&network, txid(&first_parent));
    assert_not_stored(&network, txid(&second_parent));
}
