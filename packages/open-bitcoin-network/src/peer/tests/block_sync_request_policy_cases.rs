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
fn headers_to_block_requests_never_exceed_phase94_inflight_cap() {
    // Arrange
    assert_phase94_block_cap_matches_peer_default();
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(106).expect("inbound peer");
    let headers = header_chain(PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER + 1);

    // Act
    let actions = manager
        .handle_headers_with_policy(
            106,
            HeadersMessage { headers },
            HeaderSyncPolicy::HeadersAndBlocks,
            |store: &mut HeaderStore, header: &BlockHeader| store.insert_header(header.clone()),
        )
        .expect("headers should be processed");

    // Assert
    let [PeerAction::Send(WireNetworkMessage::GetData(inventory))] = actions.as_slice() else {
        panic!("expected capped getdata action");
    };
    assert_eq!(
        inventory.inventory.len(),
        PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
    );
    assert_eq!(
        manager
            .peer_state(106)
            .expect("peer state")
            .requested_blocks
            .len(),
        PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
    );
}

#[test]
fn phase110_block_inflight_default_matches_phase94_request_cap() {
    // Arrange / Act / Assert
    assert_phase94_block_cap_matches_peer_default();
}

#[test]
fn request_pressure_input_records_bounded_permission_effect_evidence() {
    // Arrange
    let permission_decision =
        permission_decision(["in", "download", "addr", "noban", "forceinbound"]);
    let mut manager = PeerManager::new(local_config());
    manager
        .add_inbound_peer_record(permissioned_inbound_record(107, permission_decision))
        .expect("permissioned inbound peer should be added");
    let peer = manager.peer_state(107).expect("peer state");
    let (active_permission_effects, inactive_permission_effects) =
        super::super::inventory_state::permission_effect_vectors(peer);
    let input = RequestPressureInput {
        inventory_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
        active_permission_effects: active_permission_effects.clone(),
        inactive_permission_effects,
        ..RequestPressureInput::default()
    };

    // Act
    let policy_decision = ResourceGovernancePolicy::default().decide_request(input.clone());
    let actions = manager
        .handle_message(
            107,
            WireNetworkMessage::Inv(transaction_inventory(
                PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
            )),
            1,
        )
        .expect("permissioned over-cap inventory should disconnect");

    // Assert
    assert!(
        input
            .active_permission_effects
            .contains(&PermissionEffectLabel::DownloadServingPolicyInput)
    );
    assert!(
        input
            .active_permission_effects
            .contains(&PermissionEffectLabel::AddressResponsePolicyInput)
    );
    assert!(
        input
            .active_permission_effects
            .contains(&PermissionEffectLabel::EvictionPolicyProtected)
    );
    assert!(
        input
            .active_permission_effects
            .contains(&PermissionEffectLabel::MisbehaviorPolicyProtected)
    );
    assert!(
        input
            .active_permission_effects
            .contains(&PermissionEffectLabel::AdmissionProtected)
    );
    let ResourceGovernanceDecision::Disconnect(event) = policy_decision else {
        panic!("expected request-cap disconnect decision");
    };
    assert_eq!(event.label, "request_cap_reached");
    assert_resource_limit_disconnect(&actions);
}

#[test]
fn headers_only_policy_continues_headers_without_requesting_blocks() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let genesis_header = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let next_header = mined_header(open_bitcoin_consensus::block_hash(&genesis_header), 2);
    manager.add_outbound_peer(13, 10).expect("peer");
    manager
        .handle_message(
            13,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 2,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");

    // Act
    let actions = manager
        .handle_headers_with_policy(
            13,
            HeadersMessage {
                headers: vec![genesis_header, next_header],
            },
            HeaderSyncPolicy::HeadersOnly,
            |headers: &mut HeaderStore, header: &BlockHeader| headers.insert_header(header.clone()),
        )
        .expect("headers");

    // Assert
    assert!(actions.iter().any(|action| matches!(
        action,
        PeerAction::Send(WireNetworkMessage::GetHeaders { .. })
    )));
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::GetData(_))))
    );
    assert!(
        manager
            .peer_state(13)
            .expect("peer")
            .requested_blocks
            .is_empty()
    );
}

#[test]
fn filter_permission_labels_remain_inactive_without_service_bits_or_compact_blocks() {
    // Arrange
    let permission_decision = permission_decision(["in", "all"]);
    assert_eq!(
        relay_permission_labels(&permission_decision),
        vec![
            "transaction_relay_policy_input",
            "force_relay_policy_input",
            "mempool_policy_input",
        ],
    );
    assert_eq!(
        inactive_permission_labels(&permission_decision),
        vec!["inactive_bloomfilter", "inactive_blockfilters"]
    );
    let config = local_config();
    assert_eq!(
        config.services,
        ServiceFlags::NETWORK | ServiceFlags::WITNESS
    );
    assert_eq!(
        config.version_message(1, 0).services,
        ServiceFlags::NETWORK | ServiceFlags::WITNESS
    );
    let mut manager = PeerManager::new(config);
    manager
        .add_inbound_peer_record(permissioned_inbound_record(92, permission_decision))
        .expect("permissioned inbound peer should be added");
    let compact_block_inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::CompactBlock,
        object_hash: Hash32::from_byte_array([9_u8; 32]),
    }]);
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 9),
        transactions: Vec::new(),
    };

    // Act
    let compact_block_actions = manager
        .handle_message(92, WireNetworkMessage::Inv(compact_block_inventory), 1)
        .expect("compact block inventory");
    let announcement = manager
        .announce_block(92, &block)
        .expect("block announcement")
        .expect("announcement");

    // Assert
    assert!(compact_block_actions.is_empty());
    assert!(matches!(
        announcement,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory.len() == 1 && inventory[0].inventory_type == InventoryType::Block
    ));
}

#[test]
fn request_missing_blocks_skips_known_hashes_and_tracks_requested_inventory() {
    let mut manager = PeerManager::with_max_blocks_in_flight(local_config(), 1);
    let genesis_header = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let known_hash = open_bitcoin_consensus::block_hash(&genesis_header);
    let missing_header = mined_header(known_hash, 2);
    let missing_hash = open_bitcoin_consensus::block_hash(&missing_header);

    manager.seed_local_chain(&[ChainPosition::new(genesis_header, 0, 1, 0)]);
    manager.add_outbound_peer(21, 10).expect("peer");
    assert_eq!(
        manager
            .request_missing_blocks(21, &[known_hash, missing_hash])
            .expect("pre-handshake"),
        None
    );

    manager
        .handle_message(
            21,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 1,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");
    manager
        .handle_message(21, WireNetworkMessage::Verack, 12)
        .expect("verack");
    manager.note_local_block_hash(known_hash);

    let Some(WireNetworkMessage::GetData(inventory)) = manager
        .request_missing_blocks(21, &[known_hash, missing_hash])
        .expect("request")
    else {
        panic!("expected getdata");
    };
    assert_eq!(inventory.inventory.len(), 1);
    assert_eq!(
        BlockHash::from(inventory.inventory[0].object_hash),
        missing_hash
    );
    assert_eq!(
        manager.peer_requested_blocks(21).expect("requested blocks"),
        vec![missing_hash]
    );
    assert_eq!(
        manager
            .request_missing_blocks(21, &[missing_hash])
            .expect("duplicate request"),
        None
    );
}

#[test]
fn phase110_block_request_missing_blocks_skips_known_duplicates_and_stops_at_cap() {
    // Arrange
    let mut manager = PeerManager::with_max_blocks_in_flight(local_config(), 2);
    let known_hash = BlockHash::from(hash_from_index(110_000));
    let first_missing_hash = BlockHash::from(hash_from_index(110_001));
    let second_missing_hash = BlockHash::from(hash_from_index(110_002));
    let third_missing_hash = BlockHash::from(hash_from_index(110_003));
    manager.note_local_block_hash(known_hash);
    manager.add_outbound_peer(110, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 110, 3);

    // Act
    let Some(WireNetworkMessage::GetData(inventory)) = manager
        .request_missing_blocks(
            110,
            &[
                known_hash,
                first_missing_hash,
                first_missing_hash,
                second_missing_hash,
                third_missing_hash,
            ],
        )
        .expect("request")
    else {
        panic!("expected capped getdata");
    };
    let saturated_request = manager
        .request_missing_blocks(110, &[third_missing_hash])
        .expect("saturated request");

    // Assert
    assert_eq!(inventory.inventory.len(), 2);
    assert_eq!(
        manager
            .peer_requested_blocks(110)
            .expect("requested blocks"),
        vec![first_missing_hash, second_missing_hash]
    );
    assert_eq!(saturated_request, None);
}

#[test]
fn phase110_block_getdata_over_inflight_cap_disconnects_with_request_cap_label() {
    // Arrange
    let mut manager = PeerManager::with_max_blocks_in_flight(
        local_config(),
        PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER + 1,
    );
    manager.add_outbound_peer(111, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 111, 128);
    let requested = (0..=PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER)
        .map(|index| BlockHash::from(hash_from_index(111_000 + index)))
        .collect::<Vec<_>>();
    manager
        .request_missing_blocks(111, &requested)
        .expect("seed over-cap requested blocks")
        .expect("over-cap request message");

    // Act
    let actions = manager
        .handle_message(111, WireNetworkMessage::GetData(block_inventory(1)), 13)
        .expect("getdata should map over-cap in-flight state to disconnect");

    // Assert
    assert_resource_limit_disconnect(&actions);
    assert_eq!(
        manager
            .peer_requested_blocks(111)
            .expect("requested blocks")
            .len(),
        PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER + 1
    );
}

#[test]
fn request_missing_blocks_respects_capacity_and_returns_none_when_only_skips_remain() {
    let mut manager = PeerManager::with_max_blocks_in_flight(local_config(), 2);
    let genesis_header = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let known_hash = open_bitcoin_consensus::block_hash(&genesis_header);
    let first_missing = mined_header(known_hash, 2);
    let first_missing_hash = open_bitcoin_consensus::block_hash(&first_missing);
    let second_missing = mined_header(first_missing_hash, 3);
    let second_missing_hash = open_bitcoin_consensus::block_hash(&second_missing);

    manager.seed_local_chain(&[ChainPosition::new(genesis_header, 0, 1, 0)]);
    manager.add_outbound_peer(22, 10).expect("peer");
    manager
        .handle_message(
            22,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 2,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");
    manager
        .handle_message(22, WireNetworkMessage::Verack, 12)
        .expect("verack");

    let Some(WireNetworkMessage::GetData(first_inventory)) = manager
        .request_missing_blocks(22, &[first_missing_hash, second_missing_hash])
        .expect("first request")
    else {
        panic!("expected getdata");
    };
    assert_eq!(first_inventory.inventory.len(), 2);

    let third_missing = mined_header(second_missing_hash, 4);
    let third_missing_hash = open_bitcoin_consensus::block_hash(&third_missing);
    let mut manager = PeerManager::with_max_blocks_in_flight(local_config(), 2);
    let genesis_header = mined_header(BlockHash::from_byte_array([0_u8; 32]), 11);
    let known_hash = open_bitcoin_consensus::block_hash(&genesis_header);
    manager.seed_local_chain(&[ChainPosition::new(genesis_header, 0, 1, 0)]);
    manager.add_outbound_peer(23, 10).expect("peer");
    manager
        .handle_message(
            23,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 4,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");
    manager
        .handle_message(23, WireNetworkMessage::Verack, 12)
        .expect("verack");

    let Some(WireNetworkMessage::GetData(capped_inventory)) = manager
        .request_missing_blocks(23, &[third_missing_hash])
        .expect("seed request")
    else {
        panic!("expected capped getdata");
    };
    assert_eq!(capped_inventory.inventory.len(), 1);
    assert_eq!(
        BlockHash::from(capped_inventory.inventory[0].object_hash),
        third_missing_hash
    );
    manager.note_local_block_hash(known_hash);
    assert_eq!(
        manager
            .request_missing_blocks(23, &[known_hash, third_missing_hash])
            .expect("skip-only request"),
        None
    );
}

#[test]
fn request_missing_blocks_stops_once_capacity_is_filled() {
    let mut manager = PeerManager::with_max_blocks_in_flight(local_config(), 1);
    let genesis_header = mined_header(BlockHash::from_byte_array([0_u8; 32]), 21);
    let first_missing = mined_header(open_bitcoin_consensus::block_hash(&genesis_header), 22);
    let first_missing_hash = open_bitcoin_consensus::block_hash(&first_missing);
    let second_missing = mined_header(first_missing_hash, 23);
    let second_missing_hash = open_bitcoin_consensus::block_hash(&second_missing);

    manager.seed_local_chain(&[ChainPosition::new(genesis_header, 0, 1, 0)]);
    manager.add_outbound_peer(24, 10).expect("peer");
    manager
        .handle_message(
            24,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 2,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");
    manager
        .handle_message(24, WireNetworkMessage::Verack, 12)
        .expect("verack");

    let Some(WireNetworkMessage::GetData(inventory)) = manager
        .request_missing_blocks(24, &[first_missing_hash, second_missing_hash])
        .expect("request")
    else {
        panic!("expected getdata");
    };
    assert_eq!(inventory.inventory.len(), 1);
    assert_eq!(
        BlockHash::from(inventory.inventory[0].object_hash),
        first_missing_hash
    );
    assert_eq!(
        manager.peer_requested_blocks(24).expect("requested"),
        vec![first_missing_hash]
    );
}
