// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/test/functional/mempool_unbroadcast.py

use open_bitcoin_core::consensus::{block_hash, transaction_txid, transaction_wtxid};
use open_bitcoin_core::primitives::{BlockHash, Transaction, Txid};
use open_bitcoin_mempool::{
    AdmissionContext, MempoolLifecycleDelta, MempoolMemberIdentity, MempoolRetryClear,
    MempoolRetryClearCause, PackageMemberResult, PolicyConfig, PolicyTime,
    PreparedMempoolTransition, RelayIntent, SubmissionPackage, SubmitPackageCommand,
    WellFormedPackage,
};

use super::*;
use crate::network::lifecycle_projection::LifecycleProjectionPlan;

fn network_with_spendable_coinbase(
    config: PolicyConfig,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Txid) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(136_021),
        config,
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
    relay_intent: RelayIntent,
) -> MempoolLifecycleDelta {
    let core = network
        .mempool
        .prepare_transaction_with_context(
            &network.chainstate,
            transaction,
            verify_flags(),
            consensus_params(),
            AdmissionContext::local(PolicyTime::new(accepted_at), relay_intent),
        )
        .expect("local admission should prepare");
    apply_prepared(network, core)
}

fn assert_no_eligible_serve(retry_clears: &[MempoolRetryClear]) {
    assert!(
        retry_clears
            .iter()
            .all(|clear| clear.cause != MempoolRetryClearCause::EligibleServe)
    );
}

#[test]
fn local_requested_admission_inserts_unbroadcast_member() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = member_identity(&transaction);

    // Act
    let prepared = admit_local(&mut network, transaction, 100, RelayIntent::Requested);

    // Assert
    assert!(network.unbroadcast_members().contains(&member));
    assert_no_eligible_serve(&prepared.retry_clears);
}

#[test]
fn not_requested_or_peer_admission_does_not_insert_unbroadcast_member() {
    // Arrange
    let (mut not_requested, coinbase_txid) =
        network_with_spendable_coinbase(PolicyConfig::default());
    let not_requested_tx = spend_transaction(coinbase_txid, 499_999_000);
    let not_requested_member = member_identity(&not_requested_tx);
    let (mut peer_network, peer_coinbase) =
        network_with_spendable_coinbase(PolicyConfig::default());
    peer_network.add_inbound_peer(136_022).expect("peer");
    let peer_tx = spend_transaction(peer_coinbase, 499_999_000);
    let peer_member = member_identity(&peer_tx);

    // Act
    let not_requested_prepared = admit_local(
        &mut not_requested,
        not_requested_tx,
        100,
        RelayIntent::NotRequested,
    );
    let peer_prepared = peer_network
        .mempool
        .prepare_transaction_with_context(
            &peer_network.chainstate,
            peer_tx,
            verify_flags(),
            consensus_params(),
            AdmissionContext::peer(PolicyTime::new(100)),
        )
        .expect("peer admission should prepare");
    let peer_prepared = apply_prepared(&mut peer_network, peer_prepared);

    // Assert
    assert!(
        !not_requested
            .unbroadcast_members()
            .contains(&not_requested_member)
    );
    assert!(!peer_network.unbroadcast_members().contains(&peer_member));
    assert_no_eligible_serve(&not_requested_prepared.retry_clears);
    assert_no_eligible_serve(&peer_prepared.retry_clears);
}

#[test]
fn cleared_still_present_parent_is_not_reinserted_by_child_package_admission() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let parent = spend_transaction(coinbase_txid, 499_999_000);
    let parent_member = member_identity(&parent);
    let child = spend_transaction(parent_member.txid, 499_998_000);
    let child_member = member_identity(&child);
    admit_local(&mut network, parent.clone(), 100, RelayIntent::Requested);
    assert!(network.unbroadcast_members.remove(&parent_member));
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

    // Act
    let prepared = apply_prepared(&mut network, prepared);

    // Assert
    assert!(!network.unbroadcast_members().contains(&parent_member));
    assert!(network.unbroadcast_members().contains(&child_member));
    assert_no_eligible_serve(&prepared.retry_clears);
}

#[test]
fn lifecycle_removal_removes_unbroadcast_member() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig {
        mempool_expiry_hours: 1,
        ..PolicyConfig::default()
    });
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = member_identity(&transaction);
    admit_local(&mut network, transaction, 100, RelayIntent::Requested);
    assert!(network.unbroadcast_members().contains(&member));
    let removal = network
        .mempool
        .prepare_expiry(PolicyTime::new(3_701))
        .expect("expiry should prepare");

    // Act
    let prepared = apply_prepared(&mut network, removal);

    // Assert
    assert!(!network.unbroadcast_members().contains(&member));
    assert!(prepared.retry_clears.iter().any(|clear| {
        clear.member == member && clear.cause == MempoolRetryClearCause::LifecycleRemoval
    }));
    assert_no_eligible_serve(&prepared.retry_clears);
}

#[test]
fn empty_admitted_maintenance_facts_do_not_reinsert_cleared_member() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = member_identity(&transaction);
    admit_local(&mut network, transaction, 100, RelayIntent::Requested);
    assert!(network.unbroadcast_members.remove(&member));
    let maintenance = network
        .mempool
        .prepare_expiry(PolicyTime::new(100))
        .expect("empty maintenance should prepare");
    assert!(maintenance.facts().delta().admitted.is_empty());
    assert!(
        network
            .mempool()
            .mempool()
            .entry(&member.txid)
            .is_some_and(|entry| entry.metadata.is_retry_eligible(true))
    );

    // Act
    let prepared = apply_prepared(&mut network, maintenance);

    // Assert
    assert!(!network.unbroadcast_members().contains(&member));
    assert_no_eligible_serve(&prepared.retry_clears);
}
