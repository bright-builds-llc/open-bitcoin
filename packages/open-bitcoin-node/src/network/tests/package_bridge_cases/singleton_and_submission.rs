// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py

use super::*;

#[test]
fn singleton_policy_failures_preserve_exact_rejection_categories() {
    // Arrange
    let requested = identity(&spend_transaction(Txid::from_byte_array([0x40; 32]), 1_000));
    let categories = [
        MempoolRejectionCategory::Validation,
        MempoolRejectionCategory::NonStandard,
        MempoolRejectionCategory::ConflictNotAllowed,
        MempoolRejectionCategory::LimitExceeded,
        MempoolRejectionCategory::InternalInvariant,
    ];

    // Act / Assert
    for category in categories {
        let transition =
            crate::network::admission_bridge::singleton_transition_from_hard_failure_for_test(
                HardMemberFailure::Policy {
                    requested,
                    category,
                    reason: "typed policy failure".to_string(),
                },
            )
            .expect("singleton transition");

        assert_eq!(
            transition.outcome,
            MempoolOutcome::Rejected {
                txid: requested.txid,
                wtxid: requested.wtxid,
                category,
            }
        );
    }
}

#[test]
fn child_first_neutral_candidate_has_one_submit_exact_report_and_fingerprint_with_no_projection() {
    // Arrange
    let peer_id = 133_031;
    let mut network = relay_enabled_managed_network(peer_id);
    network.add_inbound_peer(peer_id).expect("peer");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");
    network
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            5_000,
        )))
        .expect("rolling floor");
    let (parent, child) =
        cpfp_pair(transaction_txid(&genesis.transactions[0]).expect("coinbase txid"));
    let expected_package =
        WellFormedPackage::try_from(vec![parent.clone(), child.clone()]).expect("checked pair");
    let expected_fingerprint = *expected_package.fingerprint();
    let child_admission = network
        .process_peer_transaction_admission_with_provenance(
            child.clone(),
            ReceivedTransactionProvenance {
                delivered_by: peer_id,
                announcers: vec![peer_id],
            },
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("child orphan");
    assert!(matches!(
        child_admission,
        ManagedPeerAdmissionResult::Singleton(ref result)
            if matches!(result.outcome, open_bitcoin_mempool::MempoolOutcome::Orphaned { .. })
    ));
    let serving_before = network.relay_serving_info();
    let fanout_before = network.relay_fanout_info();
    let compact_before = network.compact_extra_txn_len();
    let stored_by_txid_before = network.transactions_by_txid.clone();
    let stored_by_wtxid_before = network.transactions_by_wtxid.clone();
    crate::ManagedMempool::reset_package_submit_probe_for_test();

    // Act
    let admission = network
        .process_peer_transaction_admission_with_provenance(
            parent.clone(),
            ReceivedTransactionProvenance {
                delivered_by: peer_id,
                announcers: vec![peer_id],
            },
            11,
            verify_flags(),
            consensus_params(),
        )
        .expect("package admission");

    // Assert
    let ManagedPeerAdmissionResult::Package(package_admission) = admission else {
        panic!("expected exact package admission");
    };
    let captured = crate::ManagedMempool::take_last_submitted_package_for_test()
        .expect("managed adapter captured submitted truth");
    assert_eq!(crate::ManagedMempool::package_submit_count_for_test(), 1);
    assert_eq!(package_admission.origins, [peer_id; 2]);
    assert_eq!(package_admission.submitted, captured);
    assert_eq!(
        package_admission.submitted.report.fingerprint(),
        &expected_fingerprint
    );
    assert_eq!(
        package_admission
            .submitted
            .report
            .members()
            .iter()
            .map(PackageMemberResult::requested_identity)
            .collect::<Vec<_>>(),
        expected_package
            .members()
            .map(|transaction| {
                let txid = transaction_txid(transaction).expect("txid");
                let wtxid = transaction_wtxid(transaction).expect("wtxid");
                open_bitcoin_mempool::MempoolMemberIdentity { txid, wtxid }
            })
            .collect::<Vec<_>>()
    );
    assert_eq!(package_admission.submitted.delta.admitted.len(), 2);
    assert_eq!(network.relay_serving_info(), serving_before);
    assert_eq!(network.relay_fanout_info(), fanout_before);
    assert_eq!(network.compact_extra_txn_len(), compact_before);
    assert_eq!(network.transactions_by_txid, stored_by_txid_before);
    assert_eq!(network.transactions_by_wtxid, stored_by_wtxid_before);
    assert_eq!(
        network.orphan_count(),
        0,
        "finally-present package feedback must retire the staged child"
    );
}

#[test]
fn parent_first_retransmission_and_different_deliverer_use_qualifying_announcer_not_wrong_peer() {
    // Arrange
    let qualifying_peer = 133_032;
    let body_peer = 133_033;
    let wrong_peer = 133_034;
    let mut network = relay_enabled_managed_network(qualifying_peer);
    for peer_id in [qualifying_peer, body_peer, wrong_peer] {
        network.add_inbound_peer(peer_id).expect("peer");
    }
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");
    network
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            5_000,
        )))
        .expect("rolling floor");
    let (parent, child) =
        cpfp_pair(transaction_txid(&genesis.transactions[0]).expect("coinbase txid"));
    let first_parent = network
        .process_peer_transaction_admission_with_provenance(
            parent.clone(),
            provenance(qualifying_peer, &[qualifying_peer]),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("low-fee parent");
    assert!(matches!(
        first_parent,
        ManagedPeerAdmissionResult::Singleton(_)
    ));
    network
        .process_peer_transaction_admission_with_provenance(
            child,
            provenance(body_peer, &[body_peer, qualifying_peer]),
            11,
            verify_flags(),
            consensus_params(),
        )
        .expect("announcer-qualified child");
    crate::ManagedMempool::reset_package_submit_probe_for_test();

    // Act
    let wrong = network
        .process_peer_transaction_admission_with_provenance(
            parent.clone(),
            provenance(wrong_peer, &[wrong_peer]),
            12,
            verify_flags(),
            consensus_params(),
        )
        .expect("wrong-peer parent");
    let qualifying = network
        .process_peer_transaction_admission_with_provenance(
            parent,
            provenance(qualifying_peer, &[qualifying_peer]),
            13,
            verify_flags(),
            consensus_params(),
        )
        .expect("qualifying parent retransmission");

    // Assert
    assert!(!matches!(wrong, ManagedPeerAdmissionResult::Package(_)));
    let ManagedPeerAdmissionResult::Package(package) = qualifying else {
        panic!("retained announcer must qualify the package");
    };
    assert_eq!(package.origins, [qualifying_peer; 2]);
    assert_eq!(crate::ManagedMempool::package_submit_count_for_test(), 1);
    assert_eq!(network.orphan_count(), 0);
}
