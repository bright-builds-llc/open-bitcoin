// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_network::{PeerMempoolLifecycleSnapshot, TxDownloadSnapshot};

use super::*;

fn empty_peer_projection() -> PeerMempoolLifecycleSnapshot {
    PeerMempoolLifecycleSnapshot {
        requests: TxDownloadSnapshot {
            candidate_count: 0,
            in_flight_count: 0,
            already_have_count: 0,
        },
        known_identities: 0,
        orphan_transactions: 0,
        candidate_cursors: 0,
        accepted_packages: 0,
        compact_download_peers: 0,
    }
}

fn empty_expected_projection() -> ExpectedProjection {
    ExpectedProjection {
        canonical_members: BTreeSet::new(),
        serving_members: BTreeSet::new(),
        fanout_members: BTreeSet::new(),
        peer_known_members: BTreeSet::new(),
        peer: empty_peer_projection(),
        compact_members: BTreeSet::new(),
        unbroadcast_members: BTreeSet::new(),
        authority_epoch: AuthorityEpoch::INITIAL,
        lifecycle_generation: LifecycleGeneration::INITIAL,
        dirty_generation: None,
        evidence: LifecycleEvidenceSnapshot::default(),
        reconciliation_counts: [0; 7],
    }
}

fn admitted_network() -> (
    ManagedPeerNetwork<MemoryChainstateStore>,
    MempoolMemberIdentity,
    open_bitcoin_core::primitives::Transaction,
) {
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let member = MempoolMemberIdentity {
        txid: transaction_txid(&transaction).expect("txid"),
        wtxid: transaction_wtxid(&transaction).expect("wtxid"),
    };
    let core = network
        .mempool
        .prepare_transaction_with_context(
            &network.chainstate,
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            AdmissionContext::local(PolicyTime::new(100), RelayIntent::Requested),
        )
        .expect("admission should prepare");
    apply_prepared(&mut network, core);
    (network, member, transaction)
}

#[test]
fn authority_noop_baseline_asserts_every_projection_target() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(134_055),
        PolicyConfig::default(),
    );
    let core = network
        .mempool()
        .prepare_expiry(PolicyTime::new(0))
        .expect("empty expiry should prepare");

    // Act
    apply_prepared(&mut network, core);

    // Assert
    assert_complete_projection(&network, &empty_expected_projection());
}

#[test]
fn admitted_projection_is_complete_and_reconciles_cleanly() {
    // Arrange
    let (network, member, _) = admitted_network();
    let members = BTreeSet::from([member]);
    let generation = LifecycleGeneration::INITIAL
        .checked_next()
        .expect("generation one");

    // Act
    let expected = ExpectedProjection {
        canonical_members: members.clone(),
        serving_members: members.clone(),
        fanout_members: members.clone(),
        peer_known_members: members.clone(),
        peer: PeerMempoolLifecycleSnapshot {
            requests: TxDownloadSnapshot {
                candidate_count: 0,
                in_flight_count: 0,
                already_have_count: 2,
            },
            known_identities: 1,
            orphan_transactions: 0,
            candidate_cursors: 0,
            accepted_packages: 0,
            compact_download_peers: 0,
        },
        compact_members: BTreeSet::new(),
        unbroadcast_members: members,
        authority_epoch: AuthorityEpoch::INITIAL,
        lifecycle_generation: generation,
        dirty_generation: Some(generation),
        evidence: LifecycleEvidenceSnapshot {
            committed_transitions: 1,
            admitted_members: 1,
            ..LifecycleEvidenceSnapshot::default()
        },
        reconciliation_counts: [0; 7],
    };

    // Assert
    assert_complete_projection(&network, &expected);
}

#[test]
fn final_present_admission_clears_stale_compact_extra_body() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let core = network
        .mempool
        .prepare_transaction_with_context(
            &network.chainstate,
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            AdmissionContext::local(PolicyTime::new(100), RelayIntent::Requested),
        )
        .expect("admission should prepare");
    network.compact_extra_txn.push(wtxid, transaction);

    // Act
    apply_prepared(&mut network, core);

    // Assert
    assert!(network.compact_extra_txn.iter_available().next().is_none());
    assert!(network.reconcile_lifecycle_projection().is_clean());
}

#[test]
fn deliberate_divergence_reports_every_fixed_target_without_identity_leaks() {
    // Arrange
    let (network, member, transaction) = admitted_network();

    // Act
    let mut reports = Vec::new();

    let mut serving = network.clone();
    serving.relay_serving.record_status(
        member.txid,
        Some(member.wtxid),
        open_bitcoin_network::TxServingRecordStatus::Stale,
    );
    reports.push(serving.reconcile_lifecycle_projection());

    let mut fanout = network.clone();
    fanout.relay_fanout.cleanup_transactions(
        &[member.txid],
        open_bitcoin_network::TxFanoutCleanupReason::Confirmed,
    );
    reports.push(fanout.reconcile_lifecycle_projection());

    let mut peer = network.clone();
    let teardown = peer
        .peer_manager
        .prepare_transaction_lifecycle(open_bitcoin_network::PeerTransactionLifecycleInput::new(
            Vec::new(),
            vec![open_bitcoin_network::PeerTransactionIdentity::new(
                member.txid,
                member.wtxid,
            )],
            Vec::new(),
        ))
        .expect("peer teardown should prepare");
    peer.peer_manager
        .apply_prepared_transaction_lifecycle(teardown);
    reports.push(peer.reconcile_lifecycle_projection());

    let mut compact = network.clone();
    compact
        .compact_extra_txn
        .push(member.wtxid, transaction.clone());
    reports.push(compact.reconcile_lifecycle_projection());

    let mut unbroadcast = network.clone();
    unbroadcast
        .unbroadcast_members
        .insert(MempoolMemberIdentity {
            txid: Txid::from_byte_array([0x55; 32]),
            wtxid: open_bitcoin_core::primitives::Wtxid::from_byte_array([0x56; 32]),
        });
    reports.push(unbroadcast.reconcile_lifecycle_projection());

    let mut persistence = network.clone();
    persistence.dirty_generation = Some(LifecycleGeneration::INITIAL);
    reports.push(persistence.reconcile_lifecycle_projection());

    let mut evidence = network;
    evidence.lifecycle_evidence = LifecycleEvidenceSnapshot::default();
    reports.push(evidence.reconcile_lifecycle_projection());

    // Assert
    for (index, report) in reports.iter().enumerate() {
        assert_eq!(
            report.labels(),
            LifecycleReconciliationReport::FIXED_TARGET_LABELS
        );
        assert!(report.counts()[index] > 0, "{report:?}");
        assert!(report.counts()[index] <= LifecycleReconciliationReport::MAX_MISMATCH_COUNT);
    }
}

#[test]
fn exact_mismatch_identities_exist_only_in_the_test_contract() {
    // Arrange
    let (mut network, member, transaction) = admitted_network();
    network.compact_extra_txn.push(member.wtxid, transaction);
    network.unbroadcast_members.insert(MempoolMemberIdentity {
        txid: Txid::from_byte_array([0x65; 32]),
        wtxid: open_bitcoin_core::primitives::Wtxid::from_byte_array([0x66; 32]),
    });

    // Act
    let exact = network.reconcile_lifecycle_projection_exact_for_test();

    // Assert
    assert!(exact.compact.contains(&member));
    assert_eq!(exact.unbroadcast.len(), 1);
    assert!(network.reconcile_lifecycle_projection().counts()[3] > 0);
    assert!(network.reconcile_lifecycle_projection().counts()[4] > 0);
}
