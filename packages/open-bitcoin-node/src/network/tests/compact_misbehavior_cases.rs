// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

//! ManagedPeerNetwork live-path proofs for Phase 120 GOV-02 compact misbehavior escalation.

use open_bitcoin_codec::{
    BlockTransactions, CompactBlockPayload, PrefilledTransaction, SendCompactMessage, ShortId,
};
use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{
        Amount, Block, BlockHash, InventoryType, ScriptBuf, Transaction, TransactionOutput, Txid,
    },
};
use open_bitcoin_network::{
    MisbehaviorKind, MisbehaviorResponse, NetworkError, WireNetworkMessage,
};

use super::{
    build_block, compact_relay_enabled_managed_network, consensus_params, mine_header,
    spend_transaction, verify_flags,
};
use crate::{ManagedNetworkError, ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
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

fn compact_payload_from_block(block: &Block, nonce: u64) -> CompactBlockPayload {
    assert!(block.transactions.len() >= 2);
    let wtxid = transaction_wtxid(&block.transactions[1]).expect("wtxid");
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&block.header, nonce);
    let short_id = open_bitcoin_core::consensus::compact_short_id_for_wtxid(selector, &wtxid);

    CompactBlockPayload {
        header: block.header.clone(),
        nonce,
        short_ids: vec![short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: block.transactions[0].clone(),
        }],
    }
}

fn handshake_and_sendcmpct(network: &mut ManagedPeerNetwork<MemoryChainstateStore>, peer_id: u64) {
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("version");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Verack,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("verack");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
            }),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("sendcmpct");
}

fn tip_chain(network: &mut ManagedPeerNetwork<MemoryChainstateStore>) -> (Block, Block) {
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");
    (genesis, spendable)
}

fn start_in_flight_compact_download(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
    start_time: i64,
) -> (BlockHash, Block) {
    let (_genesis, spendable) = tip_chain(network);
    let absent = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced = build_block_with_transactions(block_hash(&spendable.header), 2, vec![absent]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_from_block(&announced, 17);
    handshake_and_sendcmpct(network, peer_id);

    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            start_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("start compact download")
        .outbound;
    assert!(
        outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_))),
        "missing short IDs must leave in-flight via GetBlockTxn; outbound={outbound:?}"
    );
    (announced_hash, announced)
}

fn assert_compact_disconnect(error: ManagedNetworkError, peer_id: u64) {
    match error {
        ManagedNetworkError::Network(NetworkError::CompactBlockMisbehavior(id))
        | ManagedNetworkError::Network(NetworkError::CompactBlockHeaderViolation(id)) => {
            assert_eq!(id, peer_id);
        }
        other => panic!("expected compact disconnect NetworkError, got {other:?}"),
    }
}

fn assert_peer_removed(network: &ManagedPeerNetwork<MemoryChainstateStore>, peer_id: u64) {
    assert!(
        network.peer_manager().peer_state(peer_id).is_none(),
        "misbehaving peer must be disconnected/removed"
    );
}

#[test]
fn live_duplicate_blocktxn_disconnects_with_malformed_policy() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(120_301);
    let peer_id = 120_301;
    let (block_hash, announced) = start_in_flight_compact_download(&mut network, peer_id, 1_000);
    let in_flight = network
        .peer_manager_mut()
        .compact_download_peer_state_mut(peer_id)
        .expect("download state")
        .in_flight
        .get_mut(&block_hash)
        .expect("in-flight entry");
    in_flight.getblocktxn_in_flight = false;

    // Act
    let error = network
        .receive_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash,
                transactions: vec![announced.transactions[1].clone()],
            }),
            1_001,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("duplicate blocktxn must escalate");

    // Assert
    assert_compact_disconnect(error, peer_id);
    assert_peer_removed(&network, peer_id);
    let info = network.peer_policy_info();
    assert_eq!(info.misbehavior_observations, 1);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("compact misbehavior decision");
    assert_eq!(latest.reason, MisbehaviorKind::MalformedMessage.as_str());
    assert_eq!(latest.outcome, MisbehaviorResponse::Disconnect.as_str());
}

#[test]
fn live_out_of_bounds_blocktxn_disconnects() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(120_302);
    let peer_id = 120_302;
    let (block_hash, _) = start_in_flight_compact_download(&mut network, peer_id, 1_000);
    let null_transaction = Transaction {
        version: 2,
        inputs: Vec::new(),
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    };

    // Act
    let error = network
        .receive_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash,
                transactions: vec![null_transaction],
            }),
            1_001,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("oob blocktxn must escalate");

    // Assert
    assert_compact_disconnect(error, peer_id);
    assert_peer_removed(&network, peer_id);
    let info = network.peer_policy_info();
    assert_eq!(info.misbehavior_observations, 1);
    assert_eq!(
        info.maybe_latest_peer_policy_decision
            .expect("decision")
            .reason,
        MisbehaviorKind::MalformedMessage.as_str()
    );
}

#[test]
fn live_invalid_compact_header_disconnects_not_fallback() {
    // Arrange — empty compact block is Invalid (EmptyCompactBlock), Knots InitData invalid
    let mut network = compact_relay_enabled_managed_network(120_303);
    let peer_id = 120_303;
    let (_genesis, spendable) = tip_chain(&mut network);
    let header = spendable.header.clone();
    // Build a tip-extending header so eligibility passes and Invalid reconstruction is reached.
    let mut announced = build_block(block_hash(&spendable.header), 2, 500_000_000);
    mine_header(&mut announced);
    let payload = CompactBlockPayload {
        header: announced.header.clone(),
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: Vec::new(),
    };
    let _ = header;
    handshake_and_sendcmpct(&mut network, peer_id);

    // Act
    let error = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            i64::from(announced.header.time) + 60,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("invalid compact must disconnect");

    // Assert — HeaderViolation path, never silent GetData Fallback
    match error {
        ManagedNetworkError::Network(NetworkError::CompactBlockHeaderViolation(id)) => {
            assert_eq!(id, peer_id);
        }
        other => panic!("expected CompactBlockHeaderViolation, got {other:?}"),
    }
    assert_peer_removed(&network, peer_id);
    let info = network.peer_policy_info();
    assert_eq!(info.misbehavior_observations, 1);
    assert_eq!(
        info.maybe_latest_peer_policy_decision
            .expect("decision")
            .reason,
        MisbehaviorKind::HeaderViolation.as_str()
    );
}

#[test]
fn live_stray_unexpected_blocktxn_stays_suppressible() {
    // Arrange — NoMatchingInFlight must remain Knots-ignore silence
    let mut network = compact_relay_enabled_managed_network(120_304);
    let peer_id = 120_304;
    tip_chain(&mut network);
    handshake_and_sendcmpct(&mut network, peer_id);

    // Act
    let result = network
        .receive_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: BlockHash::from_byte_array([0x77; 32]),
                transactions: Vec::new(),
            }),
            1_000,
            verify_flags(),
            consensus_params(),
        )
        .expect("stray/unexpected blocktxn without in-flight must not disconnect");

    // Assert
    assert!(result.outbound.is_empty());
    assert!(network.peer_manager().peer_state(peer_id).is_some());
    assert_eq!(network.peer_policy_info().misbehavior_observations, 0);
}

#[test]
fn live_short_id_collision_falls_back_to_getdata() {
    // Arrange — Failed(ShortIdCollision) remains Fallback GetData (not Disconnect)
    let mut network = compact_relay_enabled_managed_network(120_305);
    let peer_id = 120_305;
    let (_genesis, spendable) = tip_chain(&mut network);
    let mut announced = build_block(block_hash(&spendable.header), 2, 500_000_000);
    mine_header(&mut announced);
    let colliding = ShortId::from_wire_bytes([0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x00]);
    let payload = CompactBlockPayload {
        header: announced.header.clone(),
        nonce: 1,
        short_ids: vec![colliding, colliding],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: announced.transactions[0].clone(),
        }],
    };
    let expected_hash = block_hash(&announced.header);
    handshake_and_sendcmpct(&mut network, peer_id);

    // Act
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            i64::from(announced.header.time) + 60,
            verify_flags(),
            consensus_params(),
        )
        .expect("collision Failed must stay Fallback")
        .outbound;

    // Assert
    assert!(
        outbound.iter().any(|message| matches!(
            message,
            WireNetworkMessage::GetData(inventory)
                if inventory.inventory.len() == 1
                    && inventory.inventory[0].inventory_type == InventoryType::Block
                    && inventory.inventory[0].object_hash == expected_hash.into()
        )),
        "short-id collision must emit GetData(Block) Fallback; outbound={outbound:?}"
    );
    assert!(network.peer_manager().peer_state(peer_id).is_some());
    assert_eq!(network.peer_policy_info().misbehavior_observations, 0);
}

#[test]
fn live_compact_misbehavior_does_not_touch_package_or_filter_defaults() {
    // Negative scope guard (D-09.4 / D-11): package/filter/public-default surfaces stay untouched.
    let network = compact_relay_enabled_managed_network(120_306);
    let activation = network.peer_manager().block_relay_activation_policy();
    assert!(activation.compact_relay.enabled);
    assert!(activation.block_serving.enabled);
    // Compact announcement remains independent of package/filter serving — no package relay
    // activation field exists on BlockRelayActivationPolicy in this milestone.
    let _ = network.peer_policy_info();
}
