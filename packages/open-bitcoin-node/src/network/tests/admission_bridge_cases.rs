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
    primitives::{
        Amount, BlockHash, InventoryType, InventoryVector, OutPoint, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput, Txid,
    },
};
use open_bitcoin_mempool::{
    FinalMempoolMembership, MempoolAcceptanceTime, MempoolOrigin, MempoolOutcome,
    MempoolRemovalCause, MempoolRemovalRole, PolicyConfig, PolicyTime, RelayIntent,
};
use open_bitcoin_network::{
    InventoryList, OrphanPolicy, PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER,
    RelayActivationConfig, WireNetworkMessage,
};

use super::{
    assert_getdata, assert_targeted_getdata, build_block, consensus_params, local_config,
    p2sh_script, script, spend_transaction, transaction_relay_inventory, verify_flags,
};
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn txid_inventory(txid: Txid) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: txid.into(),
    }])
}

fn network_with_chain(
    nonce: u64,
    block_count: u32,
    mempool_config: PolicyConfig,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Vec<Txid>) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(nonce),
        mempool_config,
    );
    let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
    let mut coinbase_txids = Vec::new();

    for height in 0..block_count {
        let block = build_block(previous_hash, height, 500_000_000);
        coinbase_txids.push(txid(&block.transactions[0]));
        previous_hash = block_hash(&block.header);
        network
            .connect_local_block(&block, verify_flags(), consensus_params())
            .expect("connect fixture block");
    }

    (network, coinbase_txids)
}

fn relay_enabled_network_with_chain(
    nonce: u64,
    block_count: u32,
    mempool_config: PolicyConfig,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Vec<Txid>) {
    let mut network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        mempool_config,
        RelayActivationConfig { enabled: true },
        true,
    );
    let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
    let mut coinbase_txids = Vec::new();

    for height in 0..block_count {
        let block = build_block(previous_hash, height, 500_000_000);
        coinbase_txids.push(txid(&block.transactions[0]));
        previous_hash = block_hash(&block.header);
        network
            .connect_local_block(&block, verify_flags(), consensus_params())
            .expect("connect fixture block");
    }

    (network, coinbase_txids)
}

fn parent_and_child(previous_txid: Txid) -> (Transaction, Transaction) {
    let parent = spend_transaction(previous_txid, 499_999_000);
    let child = spend_transaction(txid(&parent), 499_998_000);
    (parent, child)
}

fn two_input_child(first_parent: Txid, second_parent: Txid, output_value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![
            TransactionInput {
                previous_output: OutPoint {
                    txid: first_parent,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            },
            TransactionInput {
                previous_output: OutPoint {
                    txid: second_parent,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            },
        ],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(output_value).expect("valid amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn low_fee_spend(previous_txid: Txid) -> Transaction {
    spend_transaction(previous_txid, 499_999_999)
}

fn test_orphan_policy(max_total_orphans: usize, max_orphans_per_peer: usize) -> OrphanPolicy {
    test_orphan_policy_with_reconsideration_cap(max_total_orphans, max_orphans_per_peer, 8)
}

fn test_orphan_policy_with_reconsideration_cap(
    max_total_orphans: usize,
    max_orphans_per_peer: usize,
    max_reconsiderations_per_parent: usize,
) -> OrphanPolicy {
    OrphanPolicy {
        max_total_orphans,
        max_orphans_per_peer,
        max_announcers_per_orphan: 8,
        max_retained_bytes: open_bitcoin_network::PHASE133_MAX_ORPHAN_RETAINED_BYTES,
        orphan_ttl_seconds: 1,
        max_reconsiderations_per_parent,
    }
}

fn assert_mempool_contains(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    expected_txid: Txid,
) {
    assert!(network.mempool().mempool().entry(&expected_txid).is_some());
}

fn assert_not_stored(network: &ManagedPeerNetwork<MemoryChainstateStore>, rejected_txid: Txid) {
    assert!(!network.transactions_by_txid.contains_key(&rejected_txid));
}

mod local_submission;
mod orphan_reconsideration;
mod peer_admission;
mod replacement_cleanup;
mod resource_and_compact;
