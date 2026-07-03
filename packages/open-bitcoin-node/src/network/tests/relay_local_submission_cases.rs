// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use open_bitcoin_core::{
    consensus::{block_hash, transaction_txid},
    primitives::{Amount, BlockHash, Transaction, TransactionOutput, Txid},
};
use open_bitcoin_mempool::{MempoolOutcome, PolicyConfig};
use open_bitcoin_network::RelayActivationConfig;

use super::{build_block, consensus_params, local_config, script, spend_transaction, verify_flags};
use crate::status::relay_evidence::RelayEvidenceField;
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn evidence_labels(network: &ManagedPeerNetwork<MemoryChainstateStore>) -> Vec<&'static str> {
    network
        .latest_local_submission_evidence()
        .expect("local relay evidence")
        .labels
        .into_iter()
        .map(|label| label.as_str())
        .collect()
}

fn relay_enabled_network(nonce: u64) -> (ManagedPeerNetwork<MemoryChainstateStore>, Vec<Txid>) {
    let mut network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        PolicyConfig::default(),
        RelayActivationConfig { enabled: true },
        true,
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    let coinbase_txids = vec![
        txid(&genesis.transactions[0]),
        txid(&spendable.transactions[0]),
    ];
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");

    (network, coinbase_txids)
}

fn non_standard_spend(previous_txid: Txid) -> Transaction {
    let mut transaction = spend_transaction(previous_txid, 499_999_000);
    transaction.outputs = vec![TransactionOutput {
        value: Amount::from_sats(499_999_000).expect("amount"),
        script_pubkey: script(&[0x51]),
    }];
    transaction
}

#[test]
fn local_submission_records_queued_internal_relay_evidence() {
    // Arrange
    let (mut network, coinbase_txids) = relay_enabled_network(910);
    network
        .connect_outbound_peer(911, 1)
        .expect("eligible peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);

    // Act
    let outcome = network
        .submit_local_transaction_outcome_at(transaction, verify_flags(), consensus_params(), 20)
        .expect("local accepted outcome");

    // Assert
    assert!(matches!(outcome, MempoolOutcome::Accepted { .. }));
    let evidence = network
        .latest_local_submission_evidence()
        .expect("local evidence");
    assert_eq!(evidence.queued_count, 1);
    assert_eq!(evidence.suppressed_count, 0);
    assert_eq!(
        evidence
            .labels
            .iter()
            .map(|label| label.as_str())
            .collect::<Vec<_>>(),
        vec!["accepted", "queued", "rebroadcast_deferred"],
    );
    assert_eq!(
        evidence.maybe_rebroadcast.map(|label| label.as_str()),
        Some("rebroadcast_deferred"),
    );
    assert_eq!(network.relay_fanout_info().queued_transactions, 1);
    let status = network.relay_evidence_status();
    let RelayEvidenceField::Implemented(counters) = &status.outcome_counters else {
        panic!("expected implemented relay evidence counters");
    };
    assert_eq!(counters.accepted_count, 1);
    assert_eq!(counters.rebroadcast_deferred_count, 1);
    assert_eq!(counters.requested_count, 0);
    assert!(matches!(
        status.local_submission,
        RelayEvidenceField::Implemented(_)
    ));
    assert!(matches!(
        status.rebroadcast,
        RelayEvidenceField::Implemented(_)
    ));
}

#[test]
fn local_submission_duplicate_rejected_or_orphaned_does_not_enqueue_fanout() {
    // Arrange
    let (mut network, coinbase_txids) = relay_enabled_network(912);
    let accepted = spend_transaction(coinbase_txids[0], 499_999_000);
    let rejected = non_standard_spend(coinbase_txids[1]);
    let missing_parent = spend_transaction(coinbase_txids[1], 499_998_000);
    let orphaned = spend_transaction(txid(&missing_parent), 499_997_000);
    network
        .submit_local_transaction_outcome_at(
            accepted.clone(),
            verify_flags(),
            consensus_params(),
            30,
        )
        .expect("accepted setup");
    let queued_before = network.relay_fanout_info().queued_transactions;

    // Act
    let duplicate_outcome = network
        .submit_local_transaction_outcome_at(accepted, verify_flags(), consensus_params(), 31)
        .expect("duplicate outcome");
    let duplicate_labels = evidence_labels(&network);
    let rejected_outcome = network
        .submit_local_transaction_outcome_at(rejected, verify_flags(), consensus_params(), 32)
        .expect("rejected outcome");
    let rejected_labels = evidence_labels(&network);
    let orphaned_outcome = network
        .submit_local_transaction_outcome_at(orphaned, verify_flags(), consensus_params(), 33)
        .expect("orphaned outcome");
    let orphaned_labels = evidence_labels(&network);

    // Assert
    assert!(matches!(
        duplicate_outcome,
        MempoolOutcome::Duplicate { .. }
    ));
    assert!(matches!(rejected_outcome, MempoolOutcome::Rejected { .. }));
    assert!(matches!(orphaned_outcome, MempoolOutcome::Orphaned { .. }));
    assert_eq!(duplicate_labels, vec!["duplicate"]);
    assert_eq!(rejected_labels, vec!["rejected"]);
    assert_eq!(orphaned_labels, vec!["orphaned"]);
    assert_eq!(
        network.relay_fanout_info().queued_transactions,
        queued_before
    );
    assert_eq!(
        network
            .latest_local_submission_evidence()
            .expect("local evidence")
            .queued_count,
        0,
    );
    let status = network.relay_evidence_status();
    let RelayEvidenceField::Implemented(counters) = &status.outcome_counters else {
        panic!("expected implemented relay evidence counters");
    };
    assert_eq!(counters.orphaned_count, 1);
    assert_eq!(counters.accepted_count, 0);
}

#[test]
fn local_submission_records_rebroadcast_deferred_without_timer() {
    // Arrange
    let (mut network, coinbase_txids) = relay_enabled_network(913);
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);

    // Act
    let outcome = network
        .submit_local_transaction_outcome_at(transaction, verify_flags(), consensus_params(), 40)
        .expect("local accepted outcome");

    // Assert
    assert!(matches!(outcome, MempoolOutcome::Accepted { .. }));
    let evidence = network
        .latest_local_submission_evidence()
        .expect("local evidence");
    assert_eq!(
        evidence.maybe_rebroadcast.map(|label| label.as_str()),
        Some("rebroadcast_deferred"),
    );
    assert_eq!(
        evidence
            .labels
            .iter()
            .map(|label| label.as_str())
            .collect::<Vec<_>>(),
        vec!["accepted", "rebroadcast_deferred"],
    );
}
