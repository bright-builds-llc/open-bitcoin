// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

//! ManagedPeerNetwork GOV-03 proofs: ReceivedBlock multi-peer volatile clear,
//! disconnect/timeout/reorg-restart cleanup stay volatile-only, and Phase 121 /
//! package/filter public-default isolation.

use open_bitcoin_codec::{
    BlockTransactions, BlockTransactionsRequest, CompactBlockPayload, PrefilledTransaction,
    SendCompactMessage,
};
use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{Block, BlockHash, Transaction, Txid},
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{
    BlockRelayActivationPolicy, COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS,
    CompactDownloadCleanupCause, CompactRelayActivationConfig, RelayActivationConfig,
    WireNetworkMessage,
};

use super::{
    build_block, compact_relay_enabled_managed_network, consensus_params, local_config,
    mine_header, spend_transaction, verify_flags,
};
use crate::network::PeerEmission;
use crate::status::FieldAvailability;
use crate::status::relay_evidence::RelayEvidenceField;
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

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

fn assert_in_flight(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
    expected_hash: BlockHash,
) {
    let maybe_state = network.peer_manager().compact_download_peer_state(peer_id);
    let state = maybe_state.expect("compact download state");
    assert!(
        state.in_flight.contains_key(&expected_hash),
        "peer {peer_id} must hold in_flight for {expected_hash:?}"
    );
}

fn assert_no_in_flight_hash(
    network: &ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
    expected_hash: BlockHash,
) {
    let maybe_state = network.peer_manager().compact_download_peer_state(peer_id);
    let Some(state) = maybe_state else {
        return;
    };
    assert!(
        !state.in_flight.contains_key(&expected_hash),
        "peer {peer_id} must not retain in_flight for {expected_hash:?}"
    );
}

fn start_shared_compact_on_peer(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
    payload: CompactBlockPayload,
    start_time: i64,
) {
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
}

#[test]
fn received_block_clears_matching_in_flight_across_peers() {
    // Arrange — peers A and B both hold compact in_flight for the same hash H
    let mut network = compact_relay_enabled_managed_network(120_401);
    let peer_a = 120_401;
    let peer_b = 120_402;
    let (_genesis, spendable) = tip_chain(&mut network);
    let absent = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced = build_block_with_transactions(block_hash(&spendable.header), 2, vec![absent]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_from_block(&announced, 17);
    start_shared_compact_on_peer(&mut network, peer_a, payload.clone(), 1_000);
    start_shared_compact_on_peer(&mut network, peer_b, payload, 1_000);
    assert_in_flight(&network, peer_a, announced_hash);
    assert_in_flight(&network, peer_b, announced_hash);
    let connected_before = match network.block_relay_evidence_status().cleanup {
        FieldAvailability::Available(cleanup) => cleanup.compact_download_block_connected_count,
        FieldAvailability::Unavailable { .. } => 0,
    };

    // Act — complete compact via BlockTxn on peer A; ReceivedBlock path must clear peer B too
    let connect_time = i64::from(announced.header.time);
    let result = network
        .receive_message(
            peer_a,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: announced_hash,
                transactions: vec![announced.transactions[1].clone()],
            }),
            connect_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("completing compact download must succeed");
    let _ = result;

    // Assert — both peers lose matching volatile slots; BlockConnected evidence moves
    assert_no_in_flight_hash(&network, peer_a, announced_hash);
    assert_no_in_flight_hash(&network, peer_b, announced_hash);
    let connected_after = match network.block_relay_evidence_status().cleanup {
        FieldAvailability::Available(cleanup) => cleanup.compact_download_block_connected_count,
        FieldAvailability::Unavailable { reason } => {
            panic!("cleanup evidence unavailable after ReceivedBlock: {reason}")
        }
    };
    assert!(
        connected_after > connected_before,
        "BlockConnected cleanup evidence must increment; before={connected_before} after={connected_after}"
    );
}

#[test]
fn disconnect_cleanup_clears_volatile_compact_state_only() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(120_403);
    let peer_id = 120_403;
    let (_genesis, spendable) = tip_chain(&mut network);
    let absent = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced = build_block_with_transactions(block_hash(&spendable.header), 2, vec![absent]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_from_block(&announced, 19);
    start_shared_compact_on_peer(&mut network, peer_id, payload, 2_000);
    assert_in_flight(&network, peer_id, announced_hash);
    let tip_before = network.maybe_chain_tip().expect("tip before disconnect");
    let chain_len_before = network.chainstate_snapshot().active_chain.len();
    let durable_blocks_before = network.blocks_by_hash.len();
    let spendable_hash = block_hash(&spendable.header);
    assert!(network.blocks_by_hash.contains_key(&spendable_hash));

    // Act
    network
        .disconnect_peer(peer_id)
        .expect("disconnect peer with compact in_flight");

    // Assert — peer compact state gone; durable blocks remain queryable
    assert!(
        network
            .peer_manager()
            .compact_download_peer_state(peer_id)
            .is_none_or(|state| state.in_flight.is_empty())
    );
    assert_eq!(
        network
            .maybe_chain_tip()
            .expect("tip after disconnect")
            .block_hash,
        tip_before.block_hash
    );
    assert_eq!(
        network.chainstate_snapshot().active_chain.len(),
        chain_len_before
    );
    assert_eq!(network.blocks_by_hash.len(), durable_blocks_before);
    assert!(network.blocks_by_hash.contains_key(&spendable_hash));
}

#[test]
fn timeout_cleanup_leaves_durable_store_unchanged() {
    // Arrange — reinforce Plan 01 volatile-only timeout cleanup on ManagedPeerNetwork
    let mut network = compact_relay_enabled_managed_network(120_405);
    let peer_id = 120_405;
    let (_genesis, spendable) = tip_chain(&mut network);
    let absent = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced = build_block_with_transactions(block_hash(&spendable.header), 2, vec![absent]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_from_block(&announced, 21);
    start_shared_compact_on_peer(&mut network, peer_id, payload, 3_000);
    assert_in_flight(&network, peer_id, announced_hash);
    let tip_before = network.maybe_chain_tip().expect("tip before timeout");
    let chain_len_before = network.chainstate_snapshot().active_chain.len();
    let durable_blocks_before = network.blocks_by_hash.len();

    // Act
    let _expired = network
        .expire_compact_download_timeouts(3_000 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1)
        .expect("expire compact downloads");

    // Assert
    assert_no_in_flight_hash(&network, peer_id, announced_hash);
    assert_eq!(
        network
            .maybe_chain_tip()
            .expect("tip after timeout")
            .block_hash,
        tip_before.block_hash
    );
    assert_eq!(
        network.chainstate_snapshot().active_chain.len(),
        chain_len_before
    );
    assert_eq!(network.blocks_by_hash.len(), durable_blocks_before);
}

#[test]
fn reorg_restart_cleanup_clears_only_volatile_in_flight() {
    // Arrange — Phase 120 wrapper over PeerManager cleanup_all (Phase 115 coverage must stay green)
    let mut network = compact_relay_enabled_managed_network(120_406);
    let peer_id = 120_406;
    let (_genesis, spendable) = tip_chain(&mut network);
    let absent = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced = build_block_with_transactions(block_hash(&spendable.header), 2, vec![absent]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_from_block(&announced, 23);
    start_shared_compact_on_peer(&mut network, peer_id, payload, 4_000);
    assert_in_flight(&network, peer_id, announced_hash);
    let tip_before = network.maybe_chain_tip().expect("tip before reorg cleanup");
    let chain_len_before = network.chainstate_snapshot().active_chain.len();
    let durable_blocks_before = network.blocks_by_hash.len();
    let spendable_hash = block_hash(&spendable.header);

    // Act — reorg then restart cleanup paths clear only compact in_flight maps
    network
        .peer_manager_mut()
        .cleanup_all_compact_downloads(CompactDownloadCleanupCause::Reorg);
    assert_no_in_flight_hash(&network, peer_id, announced_hash);

    // Re-seed one slot, then RuntimeRestart
    let payload_restart = compact_payload_from_block(&announced, 23);
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload_restart),
            4_100,
            verify_flags(),
            consensus_params(),
        )
        .expect("re-seed compact after reorg clear")
        .outbound;
    assert!(
        outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_)))
    );
    assert_in_flight(&network, peer_id, announced_hash);
    network
        .peer_manager_mut()
        .cleanup_all_compact_downloads(CompactDownloadCleanupCause::RuntimeRestart);

    // Assert — volatile cleared; durable chainstate/block blobs unchanged
    assert_no_in_flight_hash(&network, peer_id, announced_hash);
    assert_eq!(
        network
            .maybe_chain_tip()
            .expect("tip after restart cleanup")
            .block_hash,
        tip_before.block_hash
    );
    assert_eq!(
        network.chainstate_snapshot().active_chain.len(),
        chain_len_before
    );
    assert_eq!(network.blocks_by_hash.len(), durable_blocks_before);
    assert!(network.blocks_by_hash.contains_key(&spendable_hash));
}

#[test]
fn phase120_preserves_phase115_block_connected_cleanup_coverage() {
    // Arrange / Act / Assert — named Phase 115 regression wrapper (must remain green)
    // The PeerManager unit proof lives in open-bitcoin-network; this node-level wrapper
    // fails the phase if that coverage regresses by re-asserting the same contract via
    // ManagedPeerNetwork's peer_manager surface.
    let mut network = compact_relay_enabled_managed_network(120_407);
    let peer_id = 120_407;
    let (_genesis, spendable) = tip_chain(&mut network);
    let absent = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced = build_block_with_transactions(block_hash(&spendable.header), 2, vec![absent]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_from_block(&announced, 29);
    start_shared_compact_on_peer(&mut network, peer_id, payload, 5_000);
    assert_in_flight(&network, peer_id, announced_hash);

    network
        .peer_manager_mut()
        .on_compact_download_block_connected(announced_hash);

    assert_no_in_flight_hash(&network, peer_id, announced_hash);
}

#[test]
fn phase120_package_filter_and_phase121_surfaces_untouched() {
    // Arrange — default ManagedPeerNetwork (no compact/package/filter activation)
    let network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(120_408),
        PolicyConfig::default(),
    );

    // Assert — public defaults remain off; Phase 120 did not activate package/filter surfaces
    assert!(!RelayActivationConfig::default().enabled);
    let block_relay_defaults = BlockRelayActivationPolicy::default();
    assert!(!block_relay_defaults.block_serving.enabled);
    assert!(!block_relay_defaults.compact_relay.enabled);
    assert!(!CompactRelayActivationConfig::default().enabled);

    let relay = network.relay_evidence_status();
    let RelayEvidenceField::Implemented(activation) = relay.activation else {
        panic!("expected implemented relay activation evidence");
    };
    assert!(
        !activation.enabled,
        "transaction relay must stay default-off after Phase 120"
    );

    let encoded = serde_json::to_value(network.block_relay_evidence_status()).expect("evidence");
    assert_eq!(
        encoded["block_serving"]["activation"]["state"],
        "unavailable"
    );
    let encoded_text = encoded.to_string();
    assert!(
        !encoded_text.contains("package_relay")
            && !encoded_text.contains("bloom_filter")
            && !encoded_text.contains("compact_filter"),
        "package/filter surfaces must remain untouched; evidence={encoded_text}"
    );

    // Phase 121 isolation — this plan must not call DurableSyncRuntime metric/log
    // projection helpers (negative assertion; git diff gate verified at plan completion).
    let this_source = include_str!("compact_cleanup_cases.rs");
    let phase121_persist = concat!("persist_", "metrics(");
    let phase121_log = concat!("block_relay_", "log_record(");
    assert!(
        !this_source.contains(phase121_persist) && !this_source.contains(phase121_log),
        "Phase 120 cleanup proofs must not invoke Phase 121 metric/log projection helpers"
    );
}

#[test]
fn phase122_disconnect_drops_compact_announcement_provenance_for_reconnected_peer() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(122_401);
    let peer_id = 122_401;
    let (_genesis, spendable) = tip_chain(&mut network);
    handshake_and_sendcmpct(&mut network, peer_id);
    let spendable_hash = block_hash(&spendable.header);
    let message = network
        .announce_block(peer_id, &spendable)
        .expect("announce block")
        .expect("compact message");
    assert!(matches!(message, WireNetworkMessage::CompactBlock(_)));
    let (_, _, receipt) = PeerEmission::new(peer_id, message, spendable_hash)
        .expect("compact emission")
        .into_parts();
    network
        .complete_peer_emission(receipt)
        .expect("complete compact write");
    assert!(
        network
            .peer_manager()
            .peer_state(peer_id)
            .expect("peer")
            .compact_announcements
            .contains(&spendable_hash)
    );

    // Act
    network.disconnect_peer(peer_id).expect("disconnect peer");
    network
        .connect_outbound_peer(peer_id, 3)
        .expect("reconnect peer");
    let result = network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
                block_hash: spendable_hash,
                index_deltas: vec![0],
            }),
            4,
            verify_flags(),
            consensus_params(),
        )
        .expect("reconnected request");

    // Assert
    assert!(result.outbound.is_empty());
    assert!(
        network
            .peer_manager()
            .peer_state(peer_id)
            .expect("reconnected peer")
            .compact_announcements
            .is_empty()
    );
}
