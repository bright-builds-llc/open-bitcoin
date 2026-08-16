// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/test/functional/mempool_unbroadcast.py

//! Receive-independent maintenance ticks walk only the local unbroadcast set.

use open_bitcoin_core::consensus::{block_hash, transaction_txid, transaction_wtxid};
use open_bitcoin_core::primitives::{BlockHash, Transaction, Txid, Wtxid};
use open_bitcoin_mempool::{
    AdmissionContext, MempoolMemberIdentity, PolicyConfig, PolicyTime, PreparedMempoolTransition,
    RelayIntent,
};
use open_bitcoin_network::{
    RelayActivationConfig, RetryDecisionContext, RetryJitterSeconds, WireNetworkMessage,
    next_retry_due_unix_seconds,
};

use super::{build_block, consensus_params, local_config, spend_transaction, verify_flags};
use crate::MemoryChainstateStore;
use crate::network::lifecycle_projection::LifecycleProjectionPlan;
use crate::network::{ManagedNetworkHandle, ManagedPeerNetwork};
use crate::storage::MempoolSnapshot;

fn relay_enabled_network(nonce: u64) -> (ManagedPeerNetwork<MemoryChainstateStore>, Vec<Txid>) {
    let mut network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        PolicyConfig::default(),
        RelayActivationConfig { enabled: true },
        true,
    );
    let genesis = build_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    let coinbase_txids = vec![
        transaction_txid(&genesis.transactions[0]).expect("genesis coinbase"),
        transaction_txid(&spendable.transactions[0]).expect("spendable coinbase"),
    ];
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");
    (network, coinbase_txids)
}

fn apply_prepared(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    core: PreparedMempoolTransition,
) {
    let plan = LifecycleProjectionPlan::prepare(network, network.authority_epoch(), core)
        .expect("projection should prepare");
    let sealed = network
        .validate_prepared_lifecycle(plan)
        .expect("current projection should validate");
    network
        .commit_sealed_lifecycle(sealed)
        .expect("current projection should apply");
}

fn member_identity(transaction: &Transaction) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: transaction_txid(transaction).expect("txid"),
        wtxid: transaction_wtxid(transaction).expect("wtxid"),
    }
}

fn due_context(observed_at_unix_seconds: i64, jitter: u64) -> RetryDecisionContext {
    RetryDecisionContext::new(
        observed_at_unix_seconds,
        RetryJitterSeconds::new(jitter).expect("valid jitter"),
    )
}

fn synthetic_identity(index: u8) -> MempoolMemberIdentity {
    let mut txid_bytes = [0_u8; 32];
    txid_bytes[0] = index;
    let mut wtxid_bytes = [0_u8; 32];
    wtxid_bytes[0] = index;
    wtxid_bytes[1] = 1;
    MempoolMemberIdentity {
        txid: Txid::from_byte_array(txid_bytes),
        wtxid: Wtxid::from_byte_array(wtxid_bytes),
    }
}

#[test]
fn maintenance_tick_walks_only_unbroadcast_members() {
    // Arrange
    let (mut network, coinbase_txids) = relay_enabled_network(136_601);
    let local_tx = spend_transaction(coinbase_txids[0], 499_999_000);
    let local_member = member_identity(&local_tx);
    network
        .submit_local_transaction_outcome_at(
            local_tx,
            verify_flags(),
            consensus_params(),
            100,
            RelayIntent::Requested,
        )
        .expect("local accept");
    network.add_inbound_peer(136_602).expect("peer");
    let peer_tx = spend_transaction(coinbase_txids[1], 499_998_000);
    let peer_member = member_identity(&peer_tx);
    let peer_prepared = network
        .mempool
        .prepare_transaction_with_context(
            &network.chainstate,
            peer_tx,
            verify_flags(),
            consensus_params(),
            AdmissionContext::peer(PolicyTime::new(100)),
        )
        .expect("peer admission should prepare");
    apply_prepared(&mut network, peer_prepared);
    let handle = ManagedNetworkHandle::from_network_fixture(network);

    // Act
    let outcome = handle
        .maintenance_tick(due_context(1_000, 0))
        .expect("due tick");

    // Assert
    assert_eq!(outcome.prepared_count, 1);
    assert_eq!(outcome.inspected_count, 1);
    assert!(handle.retry_timer_for_test().expect("timer").0.is_some());
    let snapshot = handle.authority_snapshot_for_test().expect("snapshot");
    assert!(snapshot.unbroadcast_members().contains(&local_member));
    assert!(!snapshot.unbroadcast_members().contains(&peer_member));
}

#[test]
fn maintenance_tick_respects_inspect_256_and_prepare_32_with_leftover_cursor() {
    // Arrange
    let (network, _coinbase_txids) = relay_enabled_network(136_603);
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    let identities: Vec<_> = (1_u8..=40).map(synthetic_identity).collect();
    handle
        .insert_unbroadcast_identities_for_test(identities.clone())
        .expect("insert identities");

    // Act
    let first = handle
        .maintenance_tick(due_context(1_000, 0))
        .expect("first due tick");
    let cursor_after_first = handle.retry_timer_for_test().expect("first cursor").1;
    let second = handle
        .maintenance_tick(due_context(1_700, 0))
        .expect("second due tick");

    // Assert
    assert_eq!(first.prepared_count, 32);
    assert_eq!(first.inspected_count, 40);
    assert!(first.leftover_unattempted_count >= 8);
    assert_eq!(
        cursor_after_first,
        identities.get(31).copied(),
        "cursor is last prepared so leftovers are next when inspect covers the set"
    );
    assert_eq!(second.prepared_count, 32);
    assert_eq!(
        handle.retry_timer_for_test().expect("second cursor").1,
        identities.get(23).copied(),
        "second tick starts after the first prepare window and includes leftover identities"
    );
}

#[test]
fn maintenance_tick_not_due_is_a_noop() {
    // Arrange
    let (network, _coinbase_txids) = relay_enabled_network(136_604);
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    handle
        .insert_unbroadcast_identities_for_test([synthetic_identity(1)])
        .expect("insert identity");
    let first = handle
        .maintenance_tick(due_context(1_000, 0))
        .expect("first tick remints");
    let cursor_after_first = handle.retry_timer_for_test().expect("timer");

    // Act
    let second = handle
        .maintenance_tick(due_context(1_000, 0))
        .expect("not-due tick");

    // Assert
    assert_eq!(first.prepared_count, 1);
    assert_eq!(second.prepared_count, 0);
    assert_eq!(second.inspected_count, 0);
    assert!(second.emissions.is_empty());
    assert_eq!(
        handle.retry_timer_for_test().expect("timer"),
        cursor_after_first
    );
}

#[test]
fn maintenance_tick_remints_due_time_from_injected_context() {
    // Arrange
    let (network, _coinbase_txids) = relay_enabled_network(136_605);
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    let context = due_context(2_000, 10);

    // Act
    let outcome = handle.maintenance_tick(context).expect("tick remints");

    // Assert
    assert_eq!(
        outcome.maybe_next_due_unix_seconds,
        next_retry_due_unix_seconds(context)
    );
    assert_eq!(
        handle.retry_timer_for_test().expect("timer").0,
        next_retry_due_unix_seconds(context)
    );
}

#[test]
fn recovery_install_clears_due_time_and_cursor() {
    // Arrange
    let (network, _coinbase_txids) = relay_enabled_network(136_606);
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    handle
        .set_retry_timer_for_test(Some(1_000), Some(synthetic_identity(7)))
        .expect("set timer");
    let snapshot = MempoolSnapshot::from_legacy_v1(Vec::new());
    let prepared = handle
        .prepare_mempool_recovery_at(
            &snapshot,
            verify_flags(),
            consensus_params(),
            PolicyTime::from_unix_seconds(6_000),
        )
        .expect("prepare recovery");

    // Act
    handle
        .install_mempool_recovery(prepared)
        .expect("install recovery");

    // Assert
    assert_eq!(handle.retry_timer_for_test().expect("timer"), (None, None));
}

#[test]
fn local_accept_plus_drain_prepares_first_hop_inv_without_advancing_ten_minutes() {
    // Arrange
    let (mut network, coinbase_txids) = relay_enabled_network(136_607);
    network
        .connect_outbound_peer(136_608, 1)
        .expect("eligible peer");
    let transaction = spend_transaction(coinbase_txids[1], 499_999_000);
    let handle = ManagedNetworkHandle::from_network_fixture(network);
    handle
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            50,
            RelayIntent::Requested,
        )
        .expect("local accept");

    // Act
    let emissions = handle
        .drain_tx_fanout_emissions(50)
        .expect("first-hop drain");

    // Assert
    assert!(
        emissions.iter().any(|emission| {
            matches!(emission.message(), WireNetworkMessage::Inv(_))
                && emission.is_transaction_inventory()
        }),
        "first-hop INV should be prepared at admission time"
    );
}

#[test]
fn maintenance_tick_does_not_call_receive_message() {
    // Arrange
    let source = include_str!("../runtime_authority/maintenance.rs");

    // Act / Assert
    assert!(!source.contains("receive_message"));
    assert!(!source.contains("receive_message_for_durable_serving"));
    assert!(!source.contains("DurableSyncRuntime"));
}
