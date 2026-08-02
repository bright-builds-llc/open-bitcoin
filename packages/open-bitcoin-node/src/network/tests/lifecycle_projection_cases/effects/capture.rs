// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::BTreeSet;

use open_bitcoin_core::consensus::{transaction_txid, transaction_wtxid};

use super::*;

#[test]
fn snapshot_capture_uses_one_authoritative_generation_time_and_unbroadcast_set() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let local_transaction = spend_transaction(coinbase_txid, 499_999_000);
    let local_txid = transaction_txid(&local_transaction).expect("local txid");
    let local_core = network
        .mempool
        .prepare_transaction_with_context(
            &network.chainstate,
            local_transaction,
            verify_flags(),
            consensus_params(),
            AdmissionContext::local(PolicyTime::new(135_040), RelayIntent::Requested),
        )
        .expect("local transaction should prepare");
    apply_prepared(&mut network, local_core);
    let peer_transaction = spend_transaction(local_txid, 499_998_000);
    let peer_member = open_bitcoin_mempool::MempoolMemberIdentity {
        txid: transaction_txid(&peer_transaction).expect("peer txid"),
        wtxid: transaction_wtxid(&peer_transaction).expect("peer wtxid"),
    };
    let peer_core = network
        .mempool
        .prepare_transaction_with_context(
            &network.chainstate,
            peer_transaction,
            verify_flags(),
            consensus_params(),
            AdmissionContext::peer(PolicyTime::new(135_041)),
        )
        .expect("peer transaction should prepare");
    apply_prepared(&mut network, peer_core);
    network.unbroadcast_members = BTreeSet::from([peer_member]);
    let captured_generation = network.lifecycle_generation;
    let captured_at = PolicyTime::new(135_042);

    // Act
    let prepared = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareSnapshot(SnapshotPreparationRequest::new(
            captured_at,
            CheckpointTrigger::Shutdown,
        )),
    )
    .expect("snapshot should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::SnapshotPrepared(prepared) => {
            prepared
        }
        _ => panic!("snapshot preparation returned the wrong command result"),
    };

    // Assert
    assert_eq!(prepared.snapshot().records.len(), 2);
    assert_eq!(
        prepared
            .snapshot()
            .captured_generation()
            .map(|value| value.raw()),
        Some(captured_generation.raw())
    );
    assert_eq!(prepared.snapshot().captured_at(), Some(captured_at));
    assert_eq!(
        prepared.snapshot().unbroadcast_members(),
        &BTreeSet::from([peer_member])
    );
    assert_eq!(prepared.checkpoint_trigger(), CheckpointTrigger::Shutdown);
}

#[test]
fn public_facades_prepare_and_complete_both_families() {
    // Arrange
    let handle = ManagedNetworkHandle::from_network_fixture(network_fixture());

    // Act
    let peer_receipt = handle
        .prepare_peer_relay_effect(134_082)
        .expect("peer effect should prepare")
        .acknowledge_write();
    let snapshot_receipt = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_000), CheckpointTrigger::Periodic)
        .expect("snapshot should prepare")
        .into_parts()
        .1
        .acknowledge_write();
    let peer_completion = handle
        .complete_peer_effect(peer_receipt)
        .expect("peer completion should dispatch");
    let snapshot_completion = handle
        .complete_snapshot_write(snapshot_receipt)
        .expect("snapshot completion should dispatch");

    // Assert
    assert_eq!(peer_completion, EffectCompletion::Applied);
    assert_eq!(snapshot_completion, EffectCompletion::Applied);
}
