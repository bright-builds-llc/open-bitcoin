// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

const FIXTURE_ENFORCE_SEGWIT: bool = true;

#[test]
fn phase120_compact_block_no_matching_in_flight_blocktxn_stays_silent() {
    // Arrange
    let peer_id = 120_205;
    let (mut manager, payload, _, expected_block_hash) =
        phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("compact block should start download");

    // Act
    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: BlockHash::from_byte_array([0x77; 32]),
                transactions: Vec::new(),
            }),
            1,
        )
        .expect("stray blocktxn");

    // Assert
    assert!(actions.is_empty());
    assert!(
        manager
            .compact_download_peer_state(peer_id)
            .expect("download state")
            .in_flight
            .contains_key(&expected_block_hash)
    );
}

#[test]
fn phase120_compact_block_unexpected_block_hash_disconnects_peer() {
    let peer_id = 120_206;
    let (mut manager, payload, missing, block_hash) =
        phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("compact block should start download");

    // Key in_flight under a different hash than the partial's block hash so apply reports
    // UnexpectedBlockHash (GOV-02 unexpected blocktxn / non-matching path).
    let lookup_hash = BlockHash::from_byte_array([0xde; 32]);
    let download_state = manager
        .compact_download_peer_state_mut(peer_id)
        .expect("download state");
    let in_flight = download_state
        .in_flight
        .remove(&block_hash)
        .expect("in-flight entry");
    download_state.in_flight.insert(lookup_hash, in_flight);

    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: lookup_hash,
                transactions: vec![missing],
            }),
            1_001,
        )
        .expect("unexpected hash blocktxn should process");

    assert_eq!(
        actions,
        vec![PeerAction::Disconnect(
            DisconnectReason::CompactBlockMisbehavior
        )]
    );
}

#[test]
fn phase167_compact_wrong_partial_hash_without_held_blocktxn_disconnects_first() {
    // Arrange
    let peer_id = 120_207;
    let (mut manager, payload, missing, original_hash) =
        phase115_prepare_compact_download_manager(peer_id);
    manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("compact request");
    let lookup_hash = BlockHash::from_byte_array([0xdf; 32]);
    let state = manager
        .compact_download_peer_state_mut(peer_id)
        .expect("download state");
    let mut in_flight = state
        .in_flight
        .remove(&original_hash)
        .expect("original partial");
    in_flight.getblocktxn_in_flight = false;
    state.in_flight.insert(lookup_hash, in_flight);

    // Act
    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: lookup_hash,
                transactions: vec![missing],
            }),
            1_001,
        )
        .expect("malformed partial handled");

    // Assert
    assert_eq!(
        actions,
        vec![PeerAction::Disconnect(
            DisconnectReason::CompactBlockMisbehavior
        )]
    );
    assert!(manager.block_request_peer_order(lookup_hash).is_empty());
    assert_eq!(
        manager.block_request_peer_order(original_hash),
        vec![peer_id]
    );
}

#[test]
fn phase167_compact_missing_held_partial_header_refuses_without_new_authority() {
    // Arrange
    let peer_id = 120_211;
    let other_peer = 120_212;
    let (mut manager, payload, missing, hash) = phase115_prepare_compact_download_manager(peer_id);
    let other_hash = BlockHash::from_byte_array([0xd1; 32]);
    manager
        .add_outbound_peer(other_peer, 0)
        .expect("unrelated peer");
    assert!(
        manager
            .register_block_request(other_peer, other_hash, None, 2)
            .expect("unrelated ordinary request")
    );
    manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("selected compact request");
    manager
        .compact_download_peer_state_mut(peer_id)
        .expect("selected state")
        .in_flight
        .get_mut(&hash)
        .expect("selected partial")
        .partial = crate::PartialCompactBlock::new();

    // Act
    let error = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: hash,
                transactions: vec![missing],
            }),
            1_001,
        )
        .expect_err("missing held partial header must refuse");

    // Assert
    assert!(
        matches!(error, NetworkError::InvalidHeader { reject_reason, .. }
        if reject_reason == "missing-compact-request-binding")
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![peer_id]);
    assert_eq!(
        manager.block_request_peer_order(other_hash),
        vec![other_peer]
    );
    assert!(
        manager
            .compact_download_peer_state(peer_id)
            .is_some_and(|state| state.in_flight.contains_key(&hash))
    );
}

#[test]
fn phase167_compact_detached_ordinary_request_refuses_unbound_blocktxn() {
    // Arrange
    let peer_id = 120_208;
    let (mut manager, payload, missing, hash) = phase115_prepare_compact_download_manager(peer_id);
    assert!(
        manager
            .register_block_request(peer_id, hash, None, 2)
            .expect("ordinary request")
    );
    manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("selected compact request");
    manager.detach_compact_block_request(peer_id, hash);

    // Act
    let error = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: hash,
                transactions: vec![missing],
            }),
            1_001,
        )
        .expect_err("detached compact rules cannot authorize BlockTxn");

    // Assert
    assert!(
        matches!(error, NetworkError::InvalidHeader { reject_reason, .. }
        if reject_reason == "missing-compact-request-binding")
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![peer_id]);
    assert_eq!(
        manager.maybe_block_request_enforce_segwit(peer_id, hash),
        None
    );
}

#[test]
fn phase167_compact_fixture_rules_refuse_after_canonical_owner_is_seeded() {
    use open_bitcoin_consensus::chain_spec::{CanonicalChain, CanonicalGenesisProof};

    // Arrange
    let peer_id = 120_209;
    let (mut manager, payload, missing, hash) = phase115_prepare_compact_download_manager(peer_id);
    manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("fixture request before canonical owner");
    let spec = CanonicalChain::Regtest.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("proved root");
    manager
        .seed_header_store(HeaderStore::from_canonical_genesis(&proof).expect("canonical store"));

    // Act
    let error = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: hash,
                transactions: vec![missing],
            }),
            1_001,
        )
        .expect_err("stale fixture rules cannot cross canonical owner");

    // Assert
    assert!(
        matches!(error, NetworkError::InvalidHeader { reject_reason, .. }
        if reject_reason == "compact-request-context-mismatch")
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![peer_id]);
}

#[test]
fn phase167_compact_bad_blocktxn_requests_full_block_without_received_block() {
    // Arrange
    let peer_id = 120_210;
    let (mut manager, payload, mut missing, hash) =
        phase115_prepare_compact_download_manager(peer_id);
    manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("selected compact request");
    missing.lock_time += 1;

    // Act
    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: hash,
                transactions: vec![missing],
            }),
            1_001,
        )
        .expect("bad BlockTxn falls back");

    // Assert
    assert!(
        matches!(actions.as_slice(), [PeerAction::Send(WireNetworkMessage::GetData(list))]
        if list.inventory.len() == 1 && list.inventory[0].object_hash == hash.into())
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![peer_id]);
    assert!(
        manager
            .compact_download_peer_state(peer_id)
            .is_some_and(|state| !state.in_flight.contains_key(&hash))
    );
}

#[test]
fn phase115_compact_download_without_sendcmpct_is_suppressed() {
    let peer_id = 115_009;
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let header = mined_header(block_hash(&tip), 3);
    let coinbase = phase115_coinbase_transaction();
    let missing = phase115_sample_transaction(0x22);
    let wtxid = transaction_wtxid(&missing).expect("wtxid");
    let selector = open_bitcoin_codec::short_id_selector_from_header_and_nonce(&header, 42);
    let short_id = open_bitcoin_consensus::compact_short_id_for_wtxid(selector, &wtxid);
    let payload = CompactBlockPayload {
        header,
        nonce: 42,
        short_ids: vec![short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);

    let actions = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("compact block without sendcmpct");

    assert!(actions.is_empty());
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state entry");
    assert!(download_state.in_flight.is_empty());
    assert!(matches!(
        manager
            .peer_state(peer_id)
            .expect("peer")
            .compact_relay
            .capability,
        CompactRelayCapability::Unknown
    ));
}

#[test]
fn phase115_prefilled_compact_block_completes_without_getblocktxn() {
    let peer_id = 115_011;
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let body = crate::test_block_commitments::coherent_body(
        mined_header(block_hash(&tip), 3),
        vec![phase115_coinbase_transaction()],
        true,
    );
    let header = body.header.clone();
    let payload = CompactBlockPayload {
        header,
        nonce: 5,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: body.transactions[0].clone(),
        }],
    };
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);

    let actions = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("prefilled compact block");

    assert_eq!(actions.len(), 1);
    assert!(matches!(&actions[0], PeerAction::ReceivedBlock(received) if received == &body));
    crate::test_block_commitments::assert_coherent(&body);
}

#[test]
fn phase115_ineligible_compact_block_falls_back_to_full_block_fetch() {
    let peer_id = 115_012;
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let unknown_tip = mined_header(block_hash(&genesis), 2);
    let header = mined_header(block_hash(&unknown_tip), 3);
    let payload = CompactBlockPayload {
        header,
        nonce: 42,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: phase115_coinbase_transaction(),
        }],
    };
    let block_hash = block_hash(&payload.header);
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 1);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);

    let actions = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("far compact block should fall back");

    assert_eq!(actions.len(), 1);
    assert!(matches!(
        &actions[0],
        PeerAction::Send(WireNetworkMessage::GetData(inventory))
            if inventory.inventory.len() == 1
                && inventory.inventory[0].inventory_type == InventoryType::WitnessBlock
                && inventory.inventory[0].object_hash == block_hash.into()
    ));
}
