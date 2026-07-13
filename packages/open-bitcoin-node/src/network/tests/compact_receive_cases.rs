// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

//! Live CompactBlock receive proofs for Phase 119 injected-path RCN-02/RCN-03/GOV-04.
//!
//! Duplicate `blocktxn` response misbehavior remains Phase 115 coverage
//! (`CompactBlockTxnMisbehavior::DuplicateResponse`); this module proves duplicate
//! short-id payload handling on the injected receive path instead.

use open_bitcoin_codec::{CompactBlockPayload, PrefilledTransaction, SendCompactMessage};
use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, transaction_txid, transaction_wtxid},
    primitives::{Block, BlockHash, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{MempoolOutcome, PolicyConfig};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, CompactRelayActivationConfig, RelayActivationConfig,
    WireNetworkMessage,
};

use super::{
    build_block, compact_relay_enabled_managed_network, consensus_params, local_config,
    mine_header, spend_transaction, verify_flags,
};
use crate::status::relay_evidence::RelayEvidenceField;
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn txid(transaction: &Transaction) -> Txid {
    transaction_txid(transaction).expect("txid")
}

fn wtxid(transaction: &Transaction) -> Wtxid {
    transaction_wtxid(transaction).expect("wtxid")
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

fn compact_payload_matched_and_missing(
    announced: &Block,
    matched: &Transaction,
    missing: &Transaction,
    nonce: u64,
) -> CompactBlockPayload {
    let matched_wtxid = transaction_wtxid(matched).expect("matched wtxid");
    let missing_wtxid = transaction_wtxid(missing).expect("missing wtxid");
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&announced.header, nonce);
    let matched_short_id =
        open_bitcoin_core::consensus::compact_short_id_for_wtxid(selector, &matched_wtxid);
    let missing_short_id =
        open_bitcoin_core::consensus::compact_short_id_for_wtxid(selector, &missing_wtxid);

    CompactBlockPayload {
        header: announced.header.clone(),
        nonce,
        short_ids: vec![matched_short_id, missing_short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: announced.transactions[0].clone(),
        }],
    }
}

fn compact_payload_with_duplicate_short_ids(announced: &Block, nonce: u64) -> CompactBlockPayload {
    let sample = spend_transaction(txid(&announced.transactions[0]), 1);
    let sample_wtxid = transaction_wtxid(&sample).expect("sample wtxid");
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&announced.header, nonce);
    let colliding =
        open_bitcoin_core::consensus::compact_short_id_for_wtxid(selector, &sample_wtxid);

    CompactBlockPayload {
        header: announced.header.clone(),
        nonce,
        short_ids: vec![colliding, colliding],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: announced.transactions[0].clone(),
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

#[test]
fn live_compact_receive_uses_mempool_candidates_not_empty_facts() {
    // Arrange — tip + mempool tx whose short ID appears in the CompactBlock
    let mut network = compact_relay_enabled_managed_network(119_201);
    let peer_id = 119_201;
    let (_genesis, spendable) = tip_chain(&mut network);

    let mempool_tx = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![mempool_tx.clone()]);
    let payload = compact_payload_from_block(&announced, 42);

    let outcome = network
        .submit_local_transaction_outcome(mempool_tx, verify_flags(), consensus_params())
        .expect("admit mempool tx");
    assert!(matches!(outcome, MempoolOutcome::Accepted { .. }));

    handshake_and_sendcmpct(&mut network, peer_id);

    // Act — live ManagedPeerNetwork receive must inject mempool candidates
    let receive_time = i64::from(announced.header.time) + 60;
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            receive_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("live compact receive")
        .outbound;

    // Assert — injected facts reconstruct without GetBlockTxn for the mempool-supplied short ID.
    // Empty-facts live receive would always request every unmatched short-id index.
    let live_requests_missing = outbound
        .iter()
        .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_)));
    assert!(
        !live_requests_missing,
        "live CompactBlock receive with mempool candidates must not fall back to empty-facts GetBlockTxn; outbound={outbound:?}"
    );
}

#[test]
fn live_compact_receive_sync_message_also_injects_mempool_candidates() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(119_202);
    let peer_id = 119_202;
    let (_genesis, spendable) = tip_chain(&mut network);

    let mempool_tx = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![mempool_tx.clone()]);
    let payload = compact_payload_from_block(&announced, 42);
    let outcome = network
        .submit_local_transaction_outcome(mempool_tx, verify_flags(), consensus_params())
        .expect("admit mempool tx");
    assert!(matches!(outcome, MempoolOutcome::Accepted { .. }));
    handshake_and_sendcmpct(&mut network, peer_id);

    // Act
    let receive_time = i64::from(announced.header.time) + 60;
    let result = network
        .receive_sync_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            receive_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("live compact sync receive");

    // Assert
    let live_requests_missing = result
        .outbound
        .iter()
        .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_)));
    assert!(
        !live_requests_missing,
        "receive_sync_message CompactBlock path must inject mempool candidates; outbound={:?}",
        result.outbound
    );
}

#[test]
fn phase119_live_receive_with_mempool_candidates_reconstructs_or_requests_missing() {
    // Arrange — one mempool match leaves fewer missing indexes than empty-facts (which miss all)
    let mut network = compact_relay_enabled_managed_network(119_210);
    let peer_id = 119_210;
    let (genesis, spendable) = tip_chain(&mut network);
    let matched = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let still_missing = spend_transaction(txid(&genesis.transactions[0]), 499_998_000);
    let announced =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![matched.clone()]);
    let payload = compact_payload_matched_and_missing(&announced, &matched, &still_missing, 7);
    assert!(matches!(
        network
            .submit_local_transaction_outcome(matched, verify_flags(), consensus_params())
            .expect("admit"),
        MempoolOutcome::Accepted { .. }
    ));
    handshake_and_sendcmpct(&mut network, peer_id);

    // Act
    let receive_time = i64::from(announced.header.time) + 60;
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            receive_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("live receive")
        .outbound;

    // Assert — Ready path requests only the unmatched short-id index (not both)
    let getblocktxn = outbound.iter().find_map(|message| match message {
        WireNetworkMessage::GetBlockTxn(request) => Some(request),
        _ => None,
    });
    let request =
        getblocktxn.expect("injected path with one missing short-id must request GetBlockTxn");
    let indexes = open_bitcoin_codec::expand_block_transaction_indexes(request)
        .expect("expand getblocktxn indexes");
    assert_eq!(
        indexes,
        vec![2],
        "empty-facts would request every short-id index; injected match must leave only index 2 missing"
    );
}

#[test]
fn phase119_live_receive_short_id_collision_is_typed() {
    // Arrange — duplicate short IDs in payload → ShortIdCollision → Fallback GetData
    let mut network = compact_relay_enabled_managed_network(119_211);
    let peer_id = 119_211;
    let (_genesis, spendable) = tip_chain(&mut network);
    let announced = build_block(block_hash(&spendable.header), 2, 500_000_000);
    let payload = compact_payload_with_duplicate_short_ids(&announced, 9);
    handshake_and_sendcmpct(&mut network, peer_id);

    // Act
    let receive_time = i64::from(announced.header.time) + 60;
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            receive_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("live receive collision")
        .outbound;

    // Assert — typed failure maps to full-block fallback, not silent Completed success
    assert!(
        outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetData(_))),
        "ShortIdCollision on injected path must yield typed Fallback GetData; outbound={outbound:?}"
    );
    assert!(
        !outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_))),
        "collision must not silently proceed as Ready/GetBlockTxn"
    );
}

#[test]
fn phase119_live_receive_duplicate_short_ids_are_typed_not_silent() {
    // Explicit D-09.2 duplicate proof on the injected path (distinct from Phase 115
    // DuplicateResponse on blocktxn). Duplicate short IDs must not reconstruct silently.
    let mut network = compact_relay_enabled_managed_network(119_212);
    let peer_id = 119_212;
    let (_genesis, spendable) = tip_chain(&mut network);
    let announced = build_block(block_hash(&spendable.header), 2, 500_000_000);
    let payload = compact_payload_with_duplicate_short_ids(&announced, 11);
    handshake_and_sendcmpct(&mut network, peer_id);

    // Act
    let receive_time = i64::from(announced.header.time) + 60;
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            receive_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("live receive duplicate short ids")
        .outbound;

    // Assert
    let typed_fallback = outbound
        .iter()
        .any(|message| matches!(message, WireNetworkMessage::GetData(_)));
    let silent_success = outbound.is_empty()
        || outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_)));
    assert!(
        typed_fallback && !silent_success,
        "duplicate short IDs must be typed Fallback, not silent success; outbound={outbound:?}"
    );
}

#[test]
fn phase119_live_receive_missing_short_ids_request_getblocktxn() {
    // Arrange — empty mempool so the short ID stays unmatched
    let mut network = compact_relay_enabled_managed_network(119_213);
    let peer_id = 119_213;
    let (_genesis, spendable) = tip_chain(&mut network);
    let absent = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let announced = build_block_with_transactions(block_hash(&spendable.header), 2, vec![absent]);
    let payload = compact_payload_from_block(&announced, 13);
    handshake_and_sendcmpct(&mut network, peer_id);

    // Act
    let receive_time = i64::from(announced.header.time) + 60;
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            receive_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("live receive missing")
        .outbound;

    // Assert
    assert!(
        outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_))),
        "unmatched short IDs on injected path must request GetBlockTxn; outbound={outbound:?}"
    );
}

#[test]
fn phase119_mempool_removal_clears_matched_partial_slot() {
    // Arrange — match via live receive, then remove via connected-block lifecycle
    let mut network = compact_relay_enabled_managed_network(119_214);
    let peer_id = 119_214;
    let (genesis, spendable) = tip_chain(&mut network);
    let matched = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let matched_wtxid = wtxid(&matched);
    let still_missing = spend_transaction(txid(&genesis.transactions[0]), 499_998_000);
    let announced =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![matched.clone()]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_matched_and_missing(&announced, &matched, &still_missing, 15);
    assert!(matches!(
        network
            .submit_local_transaction_outcome(matched, verify_flags(), consensus_params())
            .expect("admit"),
        MempoolOutcome::Accepted { .. }
    ));
    handshake_and_sendcmpct(&mut network, peer_id);
    let receive_time = i64::from(announced.header.time) + 60;
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            receive_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("seed partial")
        .outbound;
    assert!(
        outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_)))
    );
    assert!(
        network
            .peer_manager()
            .compact_download_peer_state(peer_id)
            .and_then(|state| state.in_flight.get(&announced_hash).cloned())
            .is_some_and(|in_flight| in_flight.partial.is_transaction_available(1))
    );

    let conflict = spend_transaction(txid(&spendable.transactions[0]), 499_997_000);
    let conflict_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![conflict]);

    // Act
    network
        .connect_local_block(&conflict_block, verify_flags(), consensus_params())
        .expect("connected-block removal");

    // Assert
    let in_flight = network
        .peer_manager()
        .compact_download_peer_state(peer_id)
        .expect("download state")
        .in_flight
        .get(&announced_hash)
        .expect("partial retained")
        .clone();
    assert!(
        !in_flight.partial.is_transaction_available(1),
        "lifecycle hook must clear matched slot for {matched_wtxid:?}"
    );
}

#[test]
fn phase119_package_filter_surfaces_untouched() {
    // Arrange — default ManagedPeerNetwork (no compact/package/filter activation)
    let network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(119_215),
        PolicyConfig::default(),
    );

    // Assert — public defaults remain off; Phase 119 did not activate package/filter surfaces
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
        "transaction relay must stay default-off after Phase 119"
    );

    // Compact/block-serving evidence stays unavailable until observed — no package/filter keys
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
}
