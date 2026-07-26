// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py

use open_bitcoin_mempool::{
    EffectiveFeeGroupId, ExistingMember, HardMemberFailure, MempoolMemberIdentity, NewlyPresent,
    PackageMemberResult, PackageStatus, PostTrimAbsence, PriorMemberSuccess,
    ReconsiderableMemberFailure, RollingMempoolFeeRate, WellFormedPackage, WitnessAlias,
};
use open_bitcoin_network::ReceivedTransactionProvenance;

use super::*;
use crate::network::admission_bridge::ManagedPeerAdmissionResult;

fn cpfp_pair(parent_input: Txid) -> (Transaction, Transaction) {
    let parent = spend_transaction(parent_input, 499_999_900);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(parent_txid, 499_998_900);
    (parent, child)
}

fn identity(transaction: &Transaction) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: transaction_txid(transaction).expect("txid"),
        wtxid: transaction_wtxid(transaction).expect("wtxid"),
    }
}

fn provenance(delivered_by: u64, announcers: &[u64]) -> ReceivedTransactionProvenance {
    ReceivedTransactionProvenance {
        delivered_by,
        announcers: announcers.to_vec(),
    }
}

fn package_network(
    peer_id: u64,
) -> (
    ManagedPeerNetwork<MemoryChainstateStore>,
    open_bitcoin_core::primitives::Txid,
) {
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
    (
        network,
        transaction_txid(&genesis.transactions[0]).expect("coinbase txid"),
    )
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

#[test]
fn every_feedback_variant_keeps_hard_reconsiderable_and_failed_fingerprint_domains_separate() {
    // Arrange
    let peer_id = 133_035;
    let missing_one = Txid::from_byte_array([0x41; 32]);
    let missing_two = Txid::from_byte_array([0x42; 32]);
    let terminal_members = [
        |requested| {
            PackageMemberResult::FinallyPresent(NewlyPresent {
                requested,
                effective_fee_group_id: EffectiveFeeGroupId::from_u64(1),
            })
        },
        |requested| PackageMemberResult::AlreadyPresent(ExistingMember { requested }),
        |requested| {
            PackageMemberResult::SameTxidDifferentWitness(WitnessAlias {
                requested,
                existing_wtxid: requested.wtxid,
            })
        },
        |requested| {
            PackageMemberResult::HardRejected(HardMemberFailure::Policy {
                requested,
                reason: "hard".to_string(),
            })
        },
        |requested| {
            PackageMemberResult::PostTrimAbsent(PostTrimAbsence {
                requested,
                prior: PriorMemberSuccess::AlreadyPresent,
            })
        },
    ];

    // Act / Assert
    for (index, make_member) in terminal_members.into_iter().enumerate() {
        let mut network = relay_enabled_managed_network(peer_id + index as u64);
        network
            .add_inbound_peer(peer_id + index as u64)
            .expect("peer");
        let child = spend_transaction(Txid::from_byte_array([0x50 + index as u8; 32]), 1_000);
        let requested = identity(&child);
        network
            .process_peer_transaction_admission_with_provenance(
                child.clone(),
                provenance(peer_id + index as u64, &[peer_id + index as u64]),
                20,
                verify_flags(),
                consensus_params(),
            )
            .expect("stage terminal fixture");
        let member = make_member(requested);
        network.apply_package_member_feedback_for_test(
            &member,
            &child,
            &provenance(peer_id + index as u64, &[peer_id + index as u64]),
            21,
        );
        assert_eq!(network.orphan_count(), 0);
        assert_eq!(
            network.peer_manager().hard_reject_contains(requested.wtxid),
            matches!(member, PackageMemberResult::HardRejected(_))
        );
        assert!(
            !network
                .peer_manager()
                .reconsiderable_transaction_contains(requested.wtxid)
        );
    }

    let mut missing_network = relay_enabled_managed_network(peer_id + 10);
    missing_network
        .add_inbound_peer(peer_id + 10)
        .expect("peer");
    let missing_child = spend_transaction(missing_one, 1_000);
    let missing_identity = identity(&missing_child);
    missing_network.apply_package_member_feedback_for_test(
        &PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::MissingInputs {
            requested: missing_identity,
            missing_parents: vec![missing_two, missing_one],
        }),
        &missing_child,
        &provenance(peer_id + 10, &[peer_id + 10, peer_id + 11]),
        30,
    );
    assert_eq!(missing_network.orphan_count(), 1);
    assert_eq!(
        missing_network.peer_manager().orphan_peer_len(peer_id + 11),
        1
    );

    for (index, member) in [
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::PackageFee {
            requested: missing_identity,
            effective_fee_group_id: EffectiveFeeGroupId::from_u64(2),
        }),
        PackageMemberResult::Reconsiderable(ReconsiderableMemberFailure::PackageReplacement {
            requested: missing_identity,
        }),
    ]
    .into_iter()
    .enumerate()
    {
        let mut network = relay_enabled_managed_network(peer_id + 20 + index as u64);
        network.apply_package_member_feedback_for_test(
            &member,
            &missing_child,
            &provenance(peer_id + 20 + index as u64, &[peer_id + 20 + index as u64]),
            40,
        );
        assert!(
            network
                .peer_manager()
                .reconsiderable_transaction_contains(missing_identity.wtxid)
        );
        assert!(
            !network
                .peer_manager()
                .hard_reject_contains(missing_identity.wtxid)
        );
    }

    let mut status_network = relay_enabled_managed_network(peer_id + 30);
    let complete = [0x61; 32];
    let partial = [0x62; 32];
    let failed = [0x63; 32];
    status_network.apply_package_status_feedback_for_test(PackageStatus::Complete, complete);
    status_network.apply_package_status_feedback_for_test(PackageStatus::Partial, partial);
    status_network.apply_package_status_feedback_for_test(PackageStatus::Failed, failed);
    assert!(
        !status_network
            .peer_manager()
            .reconsiderable_package_contains(complete)
    );
    assert!(
        status_network
            .peer_manager()
            .reconsiderable_package_contains(partial)
    );
    assert!(
        status_network
            .peer_manager()
            .reconsiderable_package_contains(failed)
    );
}

#[test]
fn newest_failed_fingerprint_falls_back_once_to_older_eligible_child() {
    // Arrange
    let peer_id = 133_040;
    let (mut network, coinbase_txid) = package_network(peer_id);
    let parent = spend_transaction(coinbase_txid, 499_999_900);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let older_high_child = spend_transaction(parent_txid, 499_998_900);
    let newer_low_child = spend_transaction(parent_txid, 499_999_800);
    network
        .process_peer_transaction_admission_with_provenance(
            parent.clone(),
            provenance(peer_id, &[peer_id]),
            50,
            verify_flags(),
            consensus_params(),
        )
        .expect("reconsiderable parent");
    for (timestamp, child) in [(51, older_high_child.clone()), (52, newer_low_child)] {
        network
            .process_peer_transaction_admission_with_provenance(
                child,
                provenance(peer_id, &[peer_id]),
                timestamp,
                verify_flags(),
                consensus_params(),
            )
            .expect("stage child");
    }
    crate::ManagedMempool::reset_package_submit_probe_for_test();

    // Act
    let first = network
        .process_peer_transaction_admission_with_provenance(
            parent.clone(),
            provenance(peer_id, &[peer_id]),
            53,
            verify_flags(),
            consensus_params(),
        )
        .expect("newest failed candidate");
    let second = network
        .process_peer_transaction_admission_with_provenance(
            parent,
            provenance(peer_id, &[peer_id]),
            54,
            verify_flags(),
            consensus_params(),
        )
        .expect("older fallback candidate");

    // Assert
    let ManagedPeerAdmissionResult::Package(first) = first else {
        panic!("expected newest candidate package");
    };
    let ManagedPeerAdmissionResult::Package(second) = second else {
        panic!("expected older fallback package");
    };
    assert_eq!(first.submitted.report.status(), &PackageStatus::Failed);
    assert_eq!(second.submitted.report.status(), &PackageStatus::Complete);
    assert_ne!(
        first.submitted.report.fingerprint(),
        second.submitted.report.fingerprint()
    );
    assert_eq!(crate::ManagedMempool::package_submit_count_for_test(), 2);
    assert_eq!(
        second.submitted.report.members()[1]
            .requested_identity()
            .wtxid,
        transaction_wtxid(&older_high_child).expect("older child wtxid")
    );
}

#[test]
fn multiple_parent_and_grandchild_candidates_are_excluded_without_fanout_or_serving_projection() {
    // Arrange
    let peer_id = 133_041;
    let (mut network, coinbase_txid) = package_network(peer_id);
    let parent = spend_transaction(coinbase_txid, 499_999_900);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    network
        .process_peer_transaction_admission_with_provenance(
            parent.clone(),
            provenance(peer_id, &[peer_id]),
            60,
            verify_flags(),
            consensus_params(),
        )
        .expect("reconsiderable parent");
    let unrelated_parent = Txid::from_byte_array([0x71; 32]);
    let multi_parent_child = Transaction {
        version: 2,
        inputs: vec![
            TransactionInput {
                previous_output: OutPoint {
                    txid: parent_txid,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            },
            TransactionInput {
                previous_output: OutPoint {
                    txid: unrelated_parent,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            },
        ],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(499_998_900).expect("amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    };
    network
        .process_peer_transaction_admission_with_provenance(
            multi_parent_child,
            provenance(peer_id, &[peer_id]),
            61,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage multi-parent child");
    crate::ManagedMempool::reset_package_submit_probe_for_test();

    // Act
    let suppressed = network
        .process_peer_transaction_admission_with_provenance(
            parent,
            provenance(peer_id, &[peer_id]),
            62,
            verify_flags(),
            consensus_params(),
        )
        .expect("multi-parent exclusion");

    // Assert
    assert!(matches!(suppressed, ManagedPeerAdmissionResult::Suppressed));
    assert_eq!(crate::ManagedMempool::package_submit_count_for_test(), 0);

    let grandchild_peer = peer_id + 1;
    let (mut grandchild_network, grandchild_coinbase) = package_network(grandchild_peer);
    let (parent, child) = cpfp_pair(grandchild_coinbase);
    let child_txid = transaction_txid(&child).expect("child txid");
    let grandchild = spend_transaction(child_txid, 499_997_900);
    grandchild_network
        .process_peer_transaction_admission_with_provenance(
            grandchild,
            provenance(grandchild_peer, &[grandchild_peer]),
            70,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage grandchild");
    grandchild_network
        .process_peer_transaction_admission_with_provenance(
            child.clone(),
            provenance(grandchild_peer, &[grandchild_peer]),
            71,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage direct child");
    let serving_before = grandchild_network.relay_serving_info();
    let fanout_before = grandchild_network.relay_fanout_info();
    let admission = grandchild_network
        .process_peer_transaction_admission_with_provenance(
            parent,
            provenance(grandchild_peer, &[grandchild_peer]),
            72,
            verify_flags(),
            consensus_params(),
        )
        .expect("direct child package");
    let ManagedPeerAdmissionResult::Package(package) = admission else {
        panic!("expected direct parent-child package");
    };
    assert_eq!(
        package.submitted.report.members()[1]
            .requested_identity()
            .wtxid,
        transaction_wtxid(&child).expect("child wtxid")
    );
    assert_eq!(grandchild_network.orphan_count(), 1);
    assert_eq!(grandchild_network.relay_serving_info(), serving_before);
    assert_eq!(grandchild_network.relay_fanout_info(), fanout_before);
}
