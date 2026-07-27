// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py

use super::*;

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
                category: MempoolRejectionCategory::InternalInvariant,
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
