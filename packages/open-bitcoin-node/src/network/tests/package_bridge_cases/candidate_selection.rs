// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py

use super::*;

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

#[test]
fn two_reconsiderable_parents_suppress_multi_parent_package_submission() {
    // Arrange
    let peer_id = 133_042;
    let (mut network, coinbase_txid) = package_network(peer_id);
    let first_parent = spend_transaction(coinbase_txid, 499_999_900);
    let first_parent_txid = transaction_txid(&first_parent).expect("first parent txid");
    network
        .process_peer_transaction_admission_with_provenance(
            first_parent.clone(),
            provenance(peer_id, &[peer_id]),
            80,
            verify_flags(),
            consensus_params(),
        )
        .expect("first reconsiderable parent");
    let second_parent = spend_transaction(Txid::from_byte_array([0x81; 32]), 499_999_800);
    let second_parent_identity = identity(&second_parent);
    network
        .peer_manager
        .record_reconsiderable_transaction(second_parent_identity.wtxid);
    let child = Transaction {
        version: 2,
        inputs: vec![
            TransactionInput {
                previous_output: OutPoint {
                    txid: first_parent_txid,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            },
            TransactionInput {
                previous_output: OutPoint {
                    txid: second_parent_identity.txid,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            },
        ],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(499_998_700).expect("amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    };
    network
        .process_peer_transaction_admission_with_provenance(
            child,
            provenance(peer_id, &[peer_id]),
            81,
            verify_flags(),
            consensus_params(),
        )
        .expect("stage two-parent child");
    crate::ManagedMempool::reset_package_submit_probe_for_test();

    // Act
    let admission = network
        .process_peer_transaction_admission_with_provenance(
            first_parent,
            provenance(peer_id, &[peer_id]),
            82,
            verify_flags(),
            consensus_params(),
        )
        .expect("two-parent suppression");

    // Assert
    assert!(matches!(admission, ManagedPeerAdmissionResult::Suppressed));
    assert_eq!(crate::ManagedMempool::package_submit_count_for_test(), 0);
    assert_eq!(network.orphan_count(), 1);
}
