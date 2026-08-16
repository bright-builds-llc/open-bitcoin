// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/test/functional/mempool_unbroadcast.py

//! Ordinary INV/TX package fanout keeps parent-before-child FIFO on the
//! existing Phase 104 queue. No package wire message is introduced.

use open_bitcoin_core::consensus::{block_hash, transaction_txid, transaction_wtxid};
use open_bitcoin_core::primitives::{BlockHash, InventoryType, InventoryVector, Transaction, Txid};
use open_bitcoin_mempool::{
    AdmissionContext, MempoolLifecycleDelta, MempoolMemberIdentity, PackageMemberResult,
    PolicyConfig, PolicyTime, PreparedMempoolTransition, RelayIntent, SubmissionPackage,
    SubmitPackageCommand, WellFormedPackage,
};
use open_bitcoin_network::{
    InventoryList, PeerId, RelayActivationConfig, TxRelayId, TxRelayPeerMode, WireNetworkMessage,
};

use super::{build_block, consensus_params, local_config, spend_transaction, verify_flags};
use crate::network::lifecycle_projection::LifecycleProjectionPlan;
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn relay_enabled_network(nonce: u64) -> (ManagedPeerNetwork<MemoryChainstateStore>, Txid) {
    let mut network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        PolicyConfig::default(),
        RelayActivationConfig { enabled: true },
        true,
    );
    let genesis = build_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable block");
    (
        network,
        transaction_txid(&spendable.transactions[0]).expect("coinbase txid"),
    )
}

fn apply_prepared(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    core: PreparedMempoolTransition,
) -> MempoolLifecycleDelta {
    let plan = LifecycleProjectionPlan::prepare(network, network.authority_epoch(), core)
        .expect("projection should prepare");
    let sealed = network
        .validate_prepared_lifecycle(plan)
        .expect("current projection should validate");
    network
        .commit_sealed_lifecycle(sealed)
        .expect("current projection should apply")
}

fn member_identity(transaction: &Transaction) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: transaction_txid(transaction).expect("txid"),
        wtxid: transaction_wtxid(transaction).expect("wtxid"),
    }
}

fn parent_child(coinbase_txid: Txid) -> (Transaction, Transaction) {
    let parent = spend_transaction(coinbase_txid, 499_999_000);
    let child = spend_transaction(member_identity(&parent).txid, 499_998_000);
    (parent, child)
}

fn apply_local_package(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    members: Vec<Transaction>,
    accepted_at: i64,
) -> MempoolLifecycleDelta {
    let checked = WellFormedPackage::try_from(members).expect("checked package");
    let package =
        SubmissionPackage::try_from_package(checked, &network.chainstate.chainstate().snapshot())
            .expect("submission package");
    let prepared = network
        .mempool()
        .prepare_package(
            SubmitPackageCommand {
                package,
                context: AdmissionContext::local(
                    PolicyTime::new(accepted_at),
                    RelayIntent::Requested,
                ),
            },
            &network.chainstate.chainstate().snapshot(),
            verify_flags(),
            consensus_params(),
        )
        .expect("package should prepare");
    apply_prepared(network, prepared)
}

fn peer_inventory(
    drained: &[(PeerId, WireNetworkMessage)],
    peer_id: PeerId,
) -> Vec<InventoryVector> {
    drained
        .iter()
        .filter(|(id, _message)| *id == peer_id)
        .flat_map(|(_id, message)| match message {
            WireNetworkMessage::Inv(InventoryList { inventory }) => inventory.clone(),
            _ => Vec::new(),
        })
        .collect()
}

#[test]
fn co_admitted_package_enqueues_parent_before_child() {
    // Arrange
    let peer_id = 136_501;
    let (mut network, coinbase_txid) = relay_enabled_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("eligible peer");
    let (parent, child) = parent_child(coinbase_txid);
    let parent_id = member_identity(&parent);
    let child_id = member_identity(&child);

    // Act
    let delta = apply_local_package(&mut network, vec![parent, child], 100);
    let queued = network.relay_fanout.queued_relay_ids_for_peer(peer_id);
    let drained = network.drain_relay_fanout(200);
    let inventory = peer_inventory(&drained, peer_id);

    // Assert
    assert_eq!(delta.admitted, vec![parent_id, child_id]);
    assert_eq!(
        queued,
        vec![
            TxRelayId::Txid(parent_id.txid),
            TxRelayId::Txid(child_id.txid)
        ]
    );
    assert_eq!(
        inventory,
        vec![
            InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: parent_id.txid.into(),
            },
            InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: child_id.txid.into(),
            },
        ]
    );
}

#[test]
fn already_present_parent_is_not_reenqueued_when_child_is_accepted() {
    // Arrange
    let peer_id = 136_502;
    let (mut network, coinbase_txid) = relay_enabled_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("eligible peer");
    let (parent, child) = parent_child(coinbase_txid);
    let parent_id = member_identity(&parent);
    let child_id = member_identity(&child);
    network
        .submit_local_transaction_outcome_at(
            parent.clone(),
            verify_flags(),
            consensus_params(),
            100,
            RelayIntent::Requested,
        )
        .expect("parent singleton");
    let queued_before = network.relay_fanout.queued_relay_ids_for_peer(peer_id);
    assert_eq!(queued_before, vec![TxRelayId::Txid(parent_id.txid)]);
    let checked = WellFormedPackage::try_from(vec![parent, child]).expect("checked package");
    let package =
        SubmissionPackage::try_from_package(checked, &network.chainstate.chainstate().snapshot())
            .expect("submission package");
    let prepared = network
        .mempool()
        .prepare_package(
            SubmitPackageCommand {
                package,
                context: AdmissionContext::local(PolicyTime::new(101), RelayIntent::Requested),
            },
            &network.chainstate.chainstate().snapshot(),
            verify_flags(),
            consensus_params(),
        )
        .expect("child package should prepare");
    assert!(matches!(
        prepared.facts().maybe_package_report().and_then(|report| {
            report
                .members()
                .iter()
                .find(|member| member.requested_identity() == parent_id)
        }),
        Some(PackageMemberResult::AlreadyPresent(_))
    ));

    // Act
    let delta = apply_prepared(&mut network, prepared);
    let queued_after = network.relay_fanout.queued_relay_ids_for_peer(peer_id);

    // Assert
    assert!(!delta.admitted.contains(&parent_id));
    assert_eq!(delta.admitted, vec![child_id]);
    assert_eq!(
        queued_after
            .iter()
            .filter(|id| **id == TxRelayId::Txid(parent_id.txid))
            .count(),
        1
    );
    assert_eq!(
        queued_after,
        vec![
            TxRelayId::Txid(parent_id.txid),
            TxRelayId::Txid(child_id.txid)
        ]
    );
}

#[test]
fn independently_admitted_child_does_not_reannounce_present_parent() {
    // Arrange
    let peer_id = 136_503;
    let (mut network, coinbase_txid) = relay_enabled_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("eligible peer");
    let (parent, child) = parent_child(coinbase_txid);
    let parent_id = member_identity(&parent);
    let child_id = member_identity(&child);
    network
        .submit_local_transaction_outcome_at(
            parent,
            verify_flags(),
            consensus_params(),
            100,
            RelayIntent::Requested,
        )
        .expect("independent parent");
    let queued_before = network.relay_fanout.queued_relay_ids_for_peer(peer_id);
    assert_eq!(queued_before, vec![TxRelayId::Txid(parent_id.txid)]);

    // Act
    network
        .submit_local_transaction_outcome_at(
            child,
            verify_flags(),
            consensus_params(),
            101,
            RelayIntent::Requested,
        )
        .expect("independent child");
    let queued_after = network.relay_fanout.queued_relay_ids_for_peer(peer_id);

    // Assert
    assert_eq!(
        queued_after
            .iter()
            .filter(|id| **id == TxRelayId::Txid(parent_id.txid))
            .count(),
        1
    );
    assert_eq!(
        queued_after,
        vec![
            TxRelayId::Txid(parent_id.txid),
            TxRelayId::Txid(child_id.txid)
        ]
    );
}

#[test]
fn package_fanout_uses_txid_or_wtxid_from_existing_peer_mode() {
    // Arrange
    let txid_peer = 136_504;
    let wtxid_peer = 136_505;
    let (mut network, coinbase_txid) = relay_enabled_network(txid_peer);
    network
        .connect_outbound_peer(txid_peer, 1)
        .expect("txid peer");
    network
        .connect_outbound_peer(wtxid_peer, 1)
        .expect("wtxid peer");
    network
        .receive_message(
            wtxid_peer,
            WireNetworkMessage::WtxidRelay,
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxidrelay");
    let (parent, child) = parent_child(coinbase_txid);
    let parent_id = member_identity(&parent);
    let child_id = member_identity(&child);

    // Act
    apply_local_package(&mut network, vec![parent, child], 100);
    assert_eq!(
        network.relay_fanout.queued_relay_ids_for_peer(txid_peer),
        vec![
            TxRelayId::Txid(parent_id.txid),
            TxRelayId::Txid(child_id.txid)
        ]
    );
    assert_eq!(
        network.relay_fanout.queued_relay_ids_for_peer(wtxid_peer),
        vec![
            TxRelayId::Wtxid(parent_id.wtxid),
            TxRelayId::Wtxid(child_id.wtxid)
        ]
    );
    let drained = network.drain_relay_fanout(200);

    // Assert
    assert_eq!(
        TxRelayPeerMode::TxidOnly.expected_inventory_type(),
        InventoryType::Transaction
    );
    assert_eq!(
        TxRelayPeerMode::WtxidRelay.expected_inventory_type(),
        InventoryType::WitnessTransaction
    );
    assert_eq!(
        peer_inventory(&drained, txid_peer),
        vec![
            InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: parent_id.txid.into(),
            },
            InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: child_id.txid.into(),
            },
        ]
    );
    assert_eq!(
        peer_inventory(&drained, wtxid_peer),
        vec![
            InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: parent_id.wtxid.into(),
            },
            InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: child_id.wtxid.into(),
            },
        ]
    );
}
