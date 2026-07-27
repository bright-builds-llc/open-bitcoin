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
fn managed_admission_bridge_local_submission_uses_same_outcome_contract() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(620, 2, PolicyConfig::default());
    let accepted = spend_transaction(coinbase_txids[0], 499_999_000);
    let accepted_txid = txid(&accepted);
    let orphan_parent = spend_transaction(coinbase_txids[0], 499_998_000);
    let orphan = spend_transaction(txid(&orphan_parent), 499_997_000);

    // Act
    let accepted_outcome = network
        .submit_local_transaction_outcome_at(
            accepted.clone(),
            verify_flags(),
            consensus_params(),
            40,
            RelayIntent::NotRequested,
        )
        .expect("accepted outcome");
    let duplicate_outcome = network
        .submit_local_transaction_outcome_at(
            accepted,
            verify_flags(),
            consensus_params(),
            41,
            RelayIntent::NotRequested,
        )
        .expect("duplicate outcome");
    let orphan_outcome = network
        .submit_local_transaction_outcome_at(
            orphan,
            verify_flags(),
            consensus_params(),
            42,
            RelayIntent::NotRequested,
        )
        .expect("orphan outcome");

    // Assert
    assert!(
        matches!(accepted_outcome, MempoolOutcome::Accepted { txid, .. } if txid == accepted_txid)
    );
    assert!(
        matches!(duplicate_outcome, MempoolOutcome::Duplicate { txid } if txid == accepted_txid)
    );
    assert!(matches!(orphan_outcome, MempoolOutcome::Orphaned { .. }));
    assert_eq!(network.orphan_count(), 0);
}

#[test]
fn managed_admission_bridge_local_not_requested_preserves_explicit_time() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(629, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);

    // Act
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            45,
            RelayIntent::NotRequested,
        )
        .expect("explicit local admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("accepted explicit transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(45))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
}

#[test]
fn managed_admission_bridge_local_requested_admission_preserves_metadata() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(626, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);

    // Act
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            50,
            RelayIntent::Requested,
        )
        .expect("local admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("accepted local transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(50))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::Requested);
}

#[test]
fn managed_admission_bridge_local_not_requested_admission_preserves_metadata() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(627, 2, PolicyConfig::default());
    network
        .connect_outbound_peer(630, 1)
        .expect("eligible peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);

    // Act
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            50,
            RelayIntent::NotRequested,
        )
        .expect("local admission without relay");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("accepted local transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(50))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::NotRequested);
    assert_eq!(network.relay_fanout_info().queued_transactions, 0);
}

#[test]
fn managed_admission_bridge_local_duplicate_preserves_first_metadata() {
    // Arrange
    let (mut network, coinbase_txids) =
        relay_enabled_network_with_chain(628, 2, PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let transaction_txid = txid(&transaction);
    network
        .submit_local_transaction_outcome_at(
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            50,
            RelayIntent::Requested,
        )
        .expect("first local admission");

    // Act
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            51,
            RelayIntent::NotRequested,
        )
        .expect("duplicate local admission");

    // Assert
    let metadata = network
        .mempool()
        .mempool()
        .entry(&transaction_txid)
        .expect("original local transaction")
        .metadata;
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(50))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::Requested);
}

#[test]
fn managed_admission_bridge_explicit_local_submission_preserves_outcome_contract() {
    // Arrange
    let (mut network, coinbase_txids) = network_with_chain(621, 3, PolicyConfig::default());
    let accepted = spend_transaction(coinbase_txids[0], 499_999_000);
    let accepted_txid = txid(&accepted);
    let (parent, orphan) = parent_and_child(coinbase_txids[1]);

    // Act
    let accepted_outcome = network
        .submit_local_transaction_outcome_at(
            accepted,
            verify_flags(),
            consensus_params(),
            52,
            RelayIntent::NotRequested,
        )
        .expect("explicit accepted outcome");
    let orphan_outcome = network
        .submit_local_transaction_outcome_at(
            orphan,
            verify_flags(),
            consensus_params(),
            53,
            RelayIntent::NotRequested,
        )
        .expect("explicit orphan outcome");

    // Assert
    assert!(matches!(
        accepted_outcome,
        MempoolOutcome::Accepted { txid, .. } if txid == accepted_txid
    ));
    assert!(matches!(orphan_outcome, MempoolOutcome::Orphaned { .. }));
    assert_not_stored(&network, txid(&parent));
}
