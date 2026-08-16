// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/test/functional/mempool_unbroadcast.py

//! GETDATA TX-response receipts clear unbroadcast only after a successful write.
//!
//! Open Bitcoin records first-hop delivery after the TX bytes are written, not when
//! a send buffer is prepared. Classify-time EligibleServe does not mutate membership.

use open_bitcoin_core::consensus::{block_hash, transaction_txid, transaction_wtxid};
use open_bitcoin_core::primitives::{BlockHash, InventoryType, InventoryVector, Transaction, Txid};
use open_bitcoin_mempool::{
    AdmissionContext, MempoolLifecycleDelta, MempoolMemberIdentity, MempoolRetryClear,
    MempoolRetryClearCause, PackageMemberResult, PolicyConfig, PolicyTime, PreparedMempoolTransition,
    RelayIntent, SubmissionPackage, SubmitPackageCommand, WellFormedPackage,
};
use open_bitcoin_network::{
    InventoryList, PeerId, RelayActivationConfig, WireNetworkMessage,
};

use super::{
    build_block, consensus_params, local_config, spend_transaction, verify_flags,
};
use crate::network::lifecycle_projection::{
    LifecycleCommand, LifecycleProjectionPlan, PeerRelayPreparationRequest,
};
use crate::network::runtime_authority::{LifecycleCommandResult, apply_lifecycle_command};
use crate::network::{EffectAbort, EffectCompletion, ManagedPeerNetwork, PeerEmission};
use crate::MemoryChainstateStore;

fn relay_enabled_network() -> (ManagedPeerNetwork<MemoryChainstateStore>, Txid) {
    let mut network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config(136_040),
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

fn admit_local(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    transaction: Transaction,
    accepted_at: i64,
) {
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            accepted_at,
            RelayIntent::Requested,
        )
        .expect("local admission should apply");
}

fn tx_inventory(txid: Txid) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: txid.into(),
    }])
}

fn prepare_tx_response(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: PeerId,
    transaction: Transaction,
    member: MempoolMemberIdentity,
) -> crate::network::PeerEmissionWriteCapability {
    let capability = match apply_lifecycle_command(
        network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(peer_id)),
    )
    .expect("peer effect should prepare")
    {
        LifecycleCommandResult::RelayPrepared(capability) => capability,
        _ => panic!("relay preparation returned the wrong command result"),
    };
    let emission = PeerEmission::try_new_tx_response(
        peer_id,
        WireNetworkMessage::Tx(transaction),
        member,
        capability,
    )
    .expect("tx response emission");
    emission.into_parts().2
}

fn complete_emission(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    capability: crate::network::PeerEmissionWriteCapability,
) -> EffectCompletion {
    match apply_lifecycle_command(
        network,
        LifecycleCommand::CompletePeerEmission(capability.acknowledge_write()),
    )
    .expect("peer emission should complete")
    {
        LifecycleCommandResult::PeerEffectCompleted(completion) => completion,
        _ => panic!("peer emission completion returned the wrong command result"),
    }
}

#[test]
fn eligible_serve_classify_does_not_clear_unbroadcast() {
    // Arrange
    let (mut network, coinbase_txid) = relay_enabled_network();
    let peer_id = 136_401;
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = member_identity(&transaction);
    admit_local(&mut network, transaction.clone(), 100);
    assert!(network.unbroadcast_members().contains(&member));

    // Act
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetData(tx_inventory(member.txid)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("getdata")
        .outbound;

    // Assert
    // EligibleServe is semantic and does not clear membership (D-03).
    assert_eq!(outbound, vec![WireNetworkMessage::Tx(transaction)]);
    assert!(network.unbroadcast_members().contains(&member));
    assert_eq!(network.last_transport_written_clear(), None);
}

#[test]
fn fresh_tx_response_write_clears_unbroadcast_as_transport_written() {
    // Arrange
    let (mut network, coinbase_txid) = relay_enabled_network();
    let peer_id = 136_402;
    let parent = spend_transaction(coinbase_txid, 499_999_000);
    let parent_member = member_identity(&parent);
    let child = spend_transaction(parent_member.txid, 499_998_000);
    let child_member = member_identity(&child);
    admit_local(&mut network, parent.clone(), 100);
    assert!(network.unbroadcast_members().contains(&parent_member));
    let write_capability = prepare_tx_response(&mut network, peer_id, parent.clone(), parent_member);

    // Act
    let completion = complete_emission(&mut network, write_capability);

    // Assert
    assert!(matches!(completion, EffectCompletion::Applied));
    assert!(!network.unbroadcast_members().contains(&parent_member));
    assert_eq!(
        network.last_transport_written_clear(),
        Some(MempoolRetryClear {
            member: parent_member,
            cause: MempoolRetryClearCause::TransportWritten,
        })
    );

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
    assert!(!prepared.facts().delta().admitted.contains(&parent_member));
    assert!(prepared.facts().delta().admitted.contains(&child_member));
    assert!(matches!(
        prepared.facts().maybe_package_report().and_then(|report| {
            report
                .members()
                .iter()
                .find(|member| member.requested_identity() == parent_member)
        }),
        Some(PackageMemberResult::AlreadyPresent(_))
    ));
    let prepared = apply_prepared(&mut network, prepared);
    assert!(!network.unbroadcast_members().contains(&parent_member));
    assert!(network.unbroadcast_members().contains(&child_member));
    assert!(
        prepared
            .retry_clears
            .iter()
            .all(|clear| clear.cause != MempoolRetryClearCause::EligibleServe)
    );
}

#[test]
fn stale_tx_response_receipt_does_not_clear_newer_unbroadcast() {
    // Arrange
    let (mut network, coinbase_txid) = relay_enabled_network();
    let peer_id = 136_403;
    let parent = spend_transaction(coinbase_txid, 499_999_000);
    let parent_member = member_identity(&parent);
    let child = spend_transaction(parent_member.txid, 499_998_000);
    let child_member = member_identity(&child);
    admit_local(&mut network, parent.clone(), 100);
    let write_capability = prepare_tx_response(&mut network, peer_id, parent, parent_member);
    admit_local(&mut network, child, 101);
    assert!(network.unbroadcast_members().contains(&parent_member));
    assert!(network.unbroadcast_members().contains(&child_member));

    // Act
    let completion = complete_emission(&mut network, write_capability);

    // Assert
    assert!(matches!(completion, EffectCompletion::AchievedButStale));
    assert!(network.unbroadcast_members().contains(&parent_member));
    assert!(network.unbroadcast_members().contains(&child_member));
    assert_eq!(network.last_transport_written_clear(), None);
}

#[test]
fn tx_inventory_write_does_not_clear_unbroadcast() {
    // Arrange
    let (mut network, coinbase_txid) = relay_enabled_network();
    let peer_id = 136_404;
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = member_identity(&transaction);
    admit_local(&mut network, transaction, 100);
    let capability = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(peer_id)),
    )
    .expect("peer effect should prepare")
    {
        LifecycleCommandResult::RelayPrepared(capability) => capability,
        _ => panic!("relay preparation returned the wrong command result"),
    };
    let emission = PeerEmission::try_new_tx_inventory(
        peer_id,
        WireNetworkMessage::Inv(InventoryList::new(Vec::new())),
        member,
        capability,
    )
    .expect("tx inventory emission");
    let write_capability = emission.into_parts().2;

    // Act
    let completion = complete_emission(&mut network, write_capability);

    // Assert
    assert!(matches!(completion, EffectCompletion::Applied));
    assert!(network.unbroadcast_members().contains(&member));
    assert_eq!(network.last_transport_written_clear(), None);
}

#[test]
fn aborted_tx_response_leaves_unbroadcast_member() {
    // Arrange
    let (mut network, coinbase_txid) = relay_enabled_network();
    let peer_id = 136_405;
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = member_identity(&transaction);
    admit_local(&mut network, transaction.clone(), 100);
    let write_capability = prepare_tx_response(&mut network, peer_id, transaction, member);

    // Act
    let abort = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::AbortPeerEffect(write_capability.into_effect_capability()),
    )
    .expect("peer emission should abort")
    {
        LifecycleCommandResult::PeerEffectAborted(abort) => abort,
        _ => panic!("peer emission abort returned the wrong command result"),
    };

    // Assert
    assert!(matches!(abort, EffectAbort::Aborted));
    assert!(network.unbroadcast_members().contains(&member));
    assert_eq!(network.last_transport_written_clear(), None);
}
