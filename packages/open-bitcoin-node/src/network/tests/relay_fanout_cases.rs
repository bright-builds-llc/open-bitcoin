// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{Block, BlockHash, InventoryType, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{MempoolOutcome, MempoolRejectionCategory, PolicyConfig};
use open_bitcoin_network::{
    InboundAdmissionDecision, InventoryList, RelayActivationConfig, WireNetworkMessage,
};

use super::{
    build_block, consensus_params, local_config, mine_header, permissioned_inbound_request,
    spend_transaction, verify_flags,
};
use crate::status::relay_evidence::RelayEvidenceField;
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn wtxid(transaction: &Transaction) -> Wtxid {
    transaction_wtxid(transaction).expect("wtxid")
}

fn relay_enabled_network(
    nonce: u64,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Vec<Txid>, Block) {
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

    (network, coinbase_txids, spendable)
}

fn admit_relay_peer(network: &mut ManagedPeerNetwork<MemoryChainstateStore>, peer_id: u64) {
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        peer_id,
        "127.0.0.1:18448",
        &["in", "relay", "mempool"],
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
}

fn build_block_with_transactions(
    previous_block_hash: BlockHash,
    height: u32,
    extra_transactions: Vec<Transaction>,
) -> Block {
    let mut block = build_block(previous_block_hash, height, 500_000_000);
    block.transactions.extend(extra_transactions);
    let (merkle_root, maybe_mutated) = block_merkle_root(&block.transactions).expect("merkle root");
    assert!(!maybe_mutated);
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);
    block
}

fn assert_single_inv_for_peer(
    targeted: &[(u64, WireNetworkMessage)],
    peer_id: u64,
    inventory_type: InventoryType,
) {
    assert_eq!(targeted.len(), 1);
    let (actual_peer_id, message) = &targeted[0];
    assert_eq!(*actual_peer_id, peer_id);
    assert!(matches!(
        message,
        WireNetworkMessage::Inv(InventoryList { inventory }) if inventory.len() == 1
            && inventory[0].inventory_type == inventory_type
    ));
}

#[test]
fn managed_fanout_announces_wtxid_to_wtxidrelay_peer() {
    // Arrange
    let (mut network, coinbase_txids, _spendable) = relay_enabled_network(900);
    admit_relay_peer(&mut network, 900);
    network
        .connect_outbound_peer(901, 1)
        .expect("eligible peer");
    network
        .receive_message(
            901,
            WireNetworkMessage::WtxidRelay,
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxidrelay");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let expected_wtxid = wtxid(&transaction);

    // Act
    let result = network
        .receive_sync_message(
            900,
            WireNetworkMessage::Tx(transaction),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("accepted peer transaction");

    // Assert
    assert!(result.outbound.is_empty());
    assert_single_inv_for_peer(
        &result.targeted_outbound,
        901,
        InventoryType::WitnessTransaction,
    );
    let WireNetworkMessage::Inv(InventoryList { inventory }) = &result.targeted_outbound[0].1
    else {
        panic!("expected inv");
    };
    assert_eq!(inventory[0].object_hash, expected_wtxid.into());
}

#[test]
fn managed_fanout_suppresses_origin_ineligible_and_recent_reject_peers() {
    // Arrange
    let (mut network, coinbase_txids, _spendable) = relay_enabled_network(901);
    admit_relay_peer(&mut network, 901);
    network
        .connect_outbound_peer(902, 1)
        .expect("eligible peer");
    network.add_inbound_peer(903).expect("ineligible peer");
    network
        .connect_outbound_peer(904, 1)
        .expect("recent-reject peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let rejected = MempoolOutcome::Rejected {
        txid: txid(&transaction),
        wtxid: wtxid(&transaction),
        category: MempoolRejectionCategory::Validation,
    };
    network
        .relay_fanout
        .record_admission_outcome(Some(904), &rejected, &[]);

    // Act
    let result = network
        .receive_sync_message(
            901,
            WireNetworkMessage::Tx(transaction),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("accepted peer transaction");

    // Assert
    assert!(result.outbound.is_empty());
    assert_single_inv_for_peer(&result.targeted_outbound, 902, InventoryType::Transaction);
    let info = network.relay_fanout_info();
    assert_eq!(info.queued_transactions, 0);
    let reasons = info
        .latest_actions
        .iter()
        .filter_map(|action| action.reason)
        .collect::<Vec<_>>();
    assert_eq!(
        reasons,
        vec!["origin_peer", "not_relay_eligible", "recent_reject"],
    );
    assert!(
        info.latest_actions
            .iter()
            .any(|action| action.label == "announce")
    );
    let status = network.relay_evidence_status();
    let RelayEvidenceField::Implemented(counters) = &status.outcome_counters else {
        panic!("expected implemented relay evidence counters");
    };
    assert_eq!(counters.announced_count, 1);
    assert_eq!(counters.suppressed_count, 3);
    let json = serde_json::to_string(&status).expect("relay evidence json");
    assert!(!json.contains("origin_peer"));
    assert!(!json.contains("not_relay_eligible"));
    assert!(!json.contains("recent_reject"));
}

#[test]
fn managed_lifecycle_cleanup_removes_serving_and_fanout_state() {
    // Arrange
    let (mut network, coinbase_txids, spendable) = relay_enabled_network(905);
    admit_relay_peer(&mut network, 905);
    network
        .connect_outbound_peer(906, 1)
        .expect("eligible peer");
    let transaction = spend_transaction(coinbase_txids[0], 499_999_000);
    let accepted_txid = txid(&transaction);
    let result = network
        .receive_sync_message(
            905,
            WireNetworkMessage::Tx(transaction.clone()),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("accepted peer transaction");
    assert_single_inv_for_peer(&result.targeted_outbound, 906, InventoryType::Transaction);
    assert_eq!(network.relay_serving_info().serveable_transactions, 1);
    assert_eq!(network.relay_fanout_info().known_transactions, 1);
    assert!(matches!(
        network
            .relay_fanout
            .record_admission_outcome(
                None,
                &MempoolOutcome::Accepted {
                    txid: accepted_txid,
                    wtxid: wtxid(&transaction),
                    evicted: Vec::new(),
                },
                &[],
            )
            .as_slice(),
        []
    ));
    let block = build_block_with_transactions(block_hash(&spendable.header), 2, vec![transaction]);

    // Act
    network
        .connect_local_block(&block, verify_flags(), consensus_params())
        .expect("connect confirming block");

    // Assert
    assert_eq!(network.relay_serving_info().serveable_transactions, 0);
    let info = network.relay_fanout_info();
    assert_eq!(info.known_transactions, 0);
    assert!(
        info.latest_actions
            .iter()
            .any(|action| action.label == "cleanup" && action.reason == Some("confirmed"))
    );
}
