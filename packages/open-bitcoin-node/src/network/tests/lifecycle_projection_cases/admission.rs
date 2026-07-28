// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::consensus::transaction_wtxid;
use open_bitcoin_mempool::{MempoolOutcome, PackageStatus, RelayIntent};
use open_bitcoin_network::ReceivedTransactionProvenance;

use super::*;
use crate::network::admission_bridge::ManagedPeerAdmissionResult;

mod partial_package;

fn admitted_projection(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    member: MempoolMemberIdentity,
    expected_unbroadcast: BTreeSet<MempoolMemberIdentity>,
) -> ExpectedProjection {
    let members = BTreeSet::from([member]);
    ExpectedProjection {
        canonical_members: members.clone(),
        serving_members: members.clone(),
        fanout_members: members.clone(),
        peer_known_members: members,
        peer: network.peer_manager.mempool_lifecycle_snapshot(),
        compact_members: BTreeSet::new(),
        unbroadcast_members: expected_unbroadcast,
        authority_epoch: AuthorityEpoch::INITIAL,
        lifecycle_generation: LifecycleGeneration::INITIAL
            .checked_next()
            .expect("test generation should advance"),
        dirty_generation: Some(
            LifecycleGeneration::INITIAL
                .checked_next()
                .expect("test generation should advance"),
        ),
        evidence: LifecycleEvidenceSnapshot {
            committed_transitions: 1,
            admitted_members: 1,
            ..LifecycleEvidenceSnapshot::default()
        },
        reconciliation_counts: [0; 7],
    }
}

fn package_projection(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    members: BTreeSet<MempoolMemberIdentity>,
) -> ExpectedProjection {
    ExpectedProjection {
        canonical_members: members.clone(),
        serving_members: members.clone(),
        fanout_members: members.clone(),
        peer_known_members: members,
        peer: network.peer_manager.mempool_lifecycle_snapshot(),
        compact_members: BTreeSet::new(),
        unbroadcast_members: BTreeSet::new(),
        authority_epoch: AuthorityEpoch::INITIAL,
        lifecycle_generation: LifecycleGeneration::INITIAL
            .checked_next()
            .expect("test generation should advance"),
        dirty_generation: Some(
            LifecycleGeneration::INITIAL
                .checked_next()
                .expect("test generation should advance"),
        ),
        evidence: LifecycleEvidenceSnapshot {
            committed_transitions: 1,
            admitted_members: 2,
            ..LifecycleEvidenceSnapshot::default()
        },
        reconciliation_counts: [0; 7],
    }
}

#[test]
fn local_requested_admission_preserves_outcome_and_projects_every_target() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = MempoolMemberIdentity {
        txid: transaction_txid(&transaction).expect("txid"),
        wtxid: transaction_wtxid(&transaction).expect("wtxid"),
    };

    // Act
    let outcome = network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            100,
            RelayIntent::Requested,
        )
        .expect("local admission should succeed");

    // Assert
    assert!(matches!(
        outcome,
        MempoolOutcome::Accepted { txid, wtxid, .. }
            if txid == member.txid && wtxid == member.wtxid
    ));
    assert_complete_projection(
        &network,
        &admitted_projection(&network, member, BTreeSet::from([member])),
    );
}

#[test]
fn peer_admission_preserves_outcome_and_projects_every_target() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    network.add_inbound_peer(134_061).expect("peer");
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = MempoolMemberIdentity {
        txid: transaction_txid(&transaction).expect("txid"),
        wtxid: transaction_wtxid(&transaction).expect("wtxid"),
    };

    // Act
    let result = network
        .process_peer_transaction_admission(
            134_061,
            transaction,
            100,
            verify_flags(),
            consensus_params(),
        )
        .expect("peer admission should succeed");

    // Assert
    assert!(matches!(
        result.outcome,
        MempoolOutcome::Accepted { txid, wtxid, .. }
            if txid == member.txid && wtxid == member.wtxid
    ));
    assert_complete_projection(
        &network,
        &admitted_projection(&network, member, BTreeSet::new()),
    );
}

#[test]
fn empty_local_admission_preserves_every_target_and_generation() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let parent = spend_transaction(coinbase_txid, 499_999_000);
    let orphan = spend_transaction(transaction_txid(&parent).expect("parent txid"), 499_998_000);
    let baseline = ExpectedProjection {
        canonical_members: BTreeSet::new(),
        serving_members: BTreeSet::new(),
        fanout_members: BTreeSet::new(),
        peer_known_members: BTreeSet::new(),
        peer: network.peer_manager.mempool_lifecycle_snapshot(),
        compact_members: BTreeSet::new(),
        unbroadcast_members: BTreeSet::new(),
        authority_epoch: AuthorityEpoch::INITIAL,
        lifecycle_generation: LifecycleGeneration::INITIAL,
        dirty_generation: None,
        evidence: LifecycleEvidenceSnapshot::default(),
        reconciliation_counts: [0; 7],
    };

    // Act
    let outcome = network
        .submit_local_transaction_outcome_at(
            orphan,
            verify_flags(),
            consensus_params(),
            100,
            RelayIntent::Requested,
        )
        .expect("orphan attempt should return an outcome");

    // Assert
    assert!(matches!(outcome, MempoolOutcome::Orphaned { .. }));
    assert_complete_projection(&network, &baseline);
}

#[test]
fn full_package_projects_parent_first_final_membership_across_every_target() {
    // Arrange
    let peer_id = 134_062;
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    network.add_inbound_peer(peer_id).expect("peer");
    network
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            5_000,
        )))
        .expect("rolling floor");
    let parent = spend_transaction(coinbase_txid, 499_999_900);
    let parent_identity = MempoolMemberIdentity {
        txid: transaction_txid(&parent).expect("parent txid"),
        wtxid: transaction_wtxid(&parent).expect("parent wtxid"),
    };
    let child = spend_transaction(parent_identity.txid, 499_998_900);
    let child_identity = MempoolMemberIdentity {
        txid: transaction_txid(&child).expect("child txid"),
        wtxid: transaction_wtxid(&child).expect("child wtxid"),
    };
    let child_attempt = network
        .process_peer_transaction_admission_with_provenance(
            child,
            ReceivedTransactionProvenance {
                delivered_by: peer_id,
                announcers: vec![peer_id],
            },
            100,
            verify_flags(),
            consensus_params(),
        )
        .expect("child should stage as an orphan");
    assert!(matches!(
        child_attempt,
        ManagedPeerAdmissionResult::Singleton(ref result)
            if matches!(result.outcome, MempoolOutcome::Orphaned { .. })
    ));

    // Act
    let admission = network
        .process_peer_transaction_admission_with_provenance(
            parent,
            ReceivedTransactionProvenance {
                delivered_by: peer_id,
                announcers: vec![peer_id],
            },
            101,
            verify_flags(),
            consensus_params(),
        )
        .expect("parent-child package should be admitted");

    // Assert
    let ManagedPeerAdmissionResult::Package(package) = admission else {
        panic!("expected package admission");
    };
    assert_eq!(package.submitted.report.status(), &PackageStatus::Complete);
    assert_eq!(
        package
            .submitted
            .report
            .members()
            .iter()
            .map(|member| member.requested_identity())
            .collect::<Vec<_>>(),
        vec![parent_identity, child_identity]
    );
    assert_eq!(
        network
            .peer_manager
            .mempool_lifecycle_snapshot()
            .accepted_packages,
        1
    );
    assert_complete_projection(
        &network,
        &package_projection(&network, BTreeSet::from([parent_identity, child_identity])),
    );
}
