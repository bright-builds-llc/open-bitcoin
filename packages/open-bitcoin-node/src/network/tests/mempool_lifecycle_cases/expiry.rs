// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn expire_mempool_removes_aged_entry_and_updates_serving() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(703),
        PolicyConfig {
            mempool_expiry_hours: 1,
            ..PolicyConfig::default()
        },
    );
    let genesis = build_block_with_transactions(BlockHash::from_byte_array([0_u8; 32]), 0, vec![]);
    let spendable = build_block_with_transactions(block_hash(&genesis.header), 1, vec![]);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");
    let transaction = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let transaction_txid = txid(&transaction);
    let transaction_wtxid = wtxid(&transaction);
    let accepted_at = 100_i64;
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            accepted_at,
            RelayIntent::NotRequested,
        )
        .expect("admit aged local transaction");
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&transaction_txid)
            .is_some()
    );
    assert!(network.transactions_by_txid.contains_key(&transaction_txid));

    // Act
    let now = PolicyTime::new(accepted_at + 3_600 + 1);
    let delta = network.expire_mempool(now).expect("expire through network");

    // Assert
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&transaction_txid)
            .is_none()
    );
    assert!(!network.transactions_by_txid.contains_key(&transaction_txid));
    assert!(
        !network
            .transactions_by_wtxid
            .contains_key(&transaction_wtxid)
    );
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == transaction_txid
            && removal.cause == MempoolRemovalCause::Expiry
            && removal.role == MempoolRemovalRole::Direct
    }));
    assert_lifecycle_authority(&network, 2);
}

#[test]
fn expire_mempool_authority_hook_removes_aged_entry() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(704),
        PolicyConfig {
            mempool_expiry_hours: 1,
            ..PolicyConfig::default()
        },
    );
    let genesis = build_block_with_transactions(BlockHash::from_byte_array([0_u8; 32]), 0, vec![]);
    let spendable = build_block_with_transactions(block_hash(&genesis.header), 1, vec![]);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");
    let transaction = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let transaction_txid = txid(&transaction);
    let accepted_at = 100_i64;
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            accepted_at,
            RelayIntent::NotRequested,
        )
        .expect("admit aged local transaction");
    let handle = ManagedNetworkHandle::from_network_fixture(network);

    // Act
    let delta = handle
        .expire_mempool(PolicyTime::new(accepted_at + 3_600 + 1))
        .expect("expire through ManagedNetworkHandle");

    // Assert
    assert!(
        handle
            .mempool_entry_metadata(&transaction_txid)
            .expect("read metadata")
            .is_none()
    );
    assert!(delta.removed.iter().any(|removal| {
        removal.member.txid == transaction_txid
            && removal.cause == MempoolRemovalCause::Expiry
            && removal.role == MempoolRemovalRole::Direct
    }));
}
