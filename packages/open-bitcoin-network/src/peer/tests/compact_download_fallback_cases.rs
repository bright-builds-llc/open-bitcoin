// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase120_compact_block_no_matching_in_flight_blocktxn_stays_silent() {
    // Arrange
    let peer_id = 120_205;
    let (mut manager, payload, _, expected_block_hash) =
        phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
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
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
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
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
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
    let header = mined_header(block_hash(&tip), 3);
    let payload = CompactBlockPayload {
        header,
        nonce: 5,
        short_ids: Vec::new(),
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: phase115_coinbase_transaction(),
        }],
    };
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);

    let actions = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("prefilled compact block");

    assert_eq!(actions.len(), 1);
    assert!(matches!(actions[0], PeerAction::ReceivedBlock(_)));
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
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("far compact block should fall back");

    assert_eq!(actions.len(), 1);
    assert!(matches!(
        &actions[0],
        PeerAction::Send(WireNetworkMessage::GetData(inventory))
            if inventory.inventory.len() == 1
                && inventory.inventory[0].inventory_type == InventoryType::Block
                && inventory.inventory[0].object_hash == block_hash.into()
    ));
}
