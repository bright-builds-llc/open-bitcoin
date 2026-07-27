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
fn outbound_handshake_negotiates_verack_sendheaders_and_wtxidrelay() {
    let mut manager = PeerManager::new(local_config());
    let outbound = manager
        .add_outbound_peer(11, 10)
        .expect("peer should be added");
    assert!(matches!(
        outbound.as_slice(),
        [PeerAction::Send(WireNetworkMessage::Version(_))]
    ));

    let version_actions = manager
        .handle_message(
            11,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 3,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version should process");
    assert_eq!(
        version_actions,
        vec![
            PeerAction::Send(WireNetworkMessage::WtxidRelay),
            PeerAction::Send(WireNetworkMessage::Verack),
            PeerAction::Send(WireNetworkMessage::SendHeaders),
        ],
    );

    let verack_actions = manager
        .handle_message(11, WireNetworkMessage::Verack, 12)
        .expect("verack should process");
    assert!(matches!(
        verack_actions.as_slice(),
        [PeerAction::Send(WireNetworkMessage::GetHeaders { .. })]
    ));

    let ping_actions = manager
        .handle_message(11, WireNetworkMessage::Ping { nonce: 99 }, 13)
        .expect("ping should process");
    assert_eq!(
        ping_actions,
        vec![PeerAction::Send(WireNetworkMessage::Pong { nonce: 99 })],
    );
    assert_eq!(
        manager.peer_state(11).expect("state").role,
        ConnectionRole::Outbound,
    );
}

#[test]
fn resource_limit_disconnect_action_is_available_for_request_cap_tests() {
    // Arrange
    let decision = ResourceGovernancePolicy::default().decide_request(RequestPressureInput {
        inventory_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
        ..RequestPressureInput::default()
    });
    let ResourceGovernanceDecision::Disconnect(event) = decision else {
        panic!("expected request-cap disconnect decision");
    };
    let actions = vec![PeerAction::ResourceGovernanceDisconnect(event)];

    // Act / Assert
    assert_resource_limit_disconnect(&actions);
}

#[test]
fn resource_limit_disconnect_mapping_preserves_all_resource_events() {
    // Arrange
    let decision = ResourceGovernancePolicy::default().decide_request(RequestPressureInput {
        inventory_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
        ..RequestPressureInput::default()
    });
    let ResourceGovernanceDecision::Disconnect(event) = decision else {
        panic!("expected request-cap disconnect decision");
    };

    // Act
    let backpressure_actions =
        super::super::inventory_state::resource_limit_disconnect_actions_from_decision(
            ResourceGovernanceDecision::Backpressure(event.clone()),
        )
        .expect("backpressure event should map to disconnect action");
    let misbehavior_actions =
        super::super::inventory_state::resource_limit_disconnect_actions_from_decision(
            ResourceGovernanceDecision::RecordMisbehavior(event),
        )
        .expect("misbehavior event should map to disconnect action");

    // Assert
    assert_resource_limit_disconnect(&backpressure_actions);
    assert_resource_limit_disconnect(&misbehavior_actions);
}

#[test]
fn block_inventory_triggers_getheaders_then_getdata_for_missing_blocks() {
    let mut manager = PeerManager::new(local_config());
    let genesis_header = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis_hash = open_bitcoin_consensus::block_hash(&genesis_header);
    manager.seed_local_chain(&[ChainPosition::new(genesis_header.clone(), 0, 1, 0)]);
    manager.add_outbound_peer(2, 10).expect("peer");
    manager
        .handle_message(
            2,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 0,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");
    manager
        .handle_message(2, WireNetworkMessage::Verack, 12)
        .expect("verack");

    let next_header = mined_header(genesis_hash, 2);
    let block_inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: open_bitcoin_consensus::block_hash(&next_header).into(),
    }]);
    let inventory_actions = manager
        .handle_message(2, WireNetworkMessage::Inv(block_inventory), 13)
        .expect("inventory");
    assert!(inventory_actions.iter().any(|action| matches!(
        action,
        PeerAction::Send(WireNetworkMessage::GetHeaders { .. })
    )));

    let header_actions = manager
        .handle_message(
            2,
            WireNetworkMessage::Headers(crate::HeadersMessage {
                headers: vec![next_header.clone()],
            }),
            14,
        )
        .expect("headers");
    assert!(
        header_actions
            .iter()
            .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::GetData(_))))
    );
    assert!(
        manager
            .peer_state(2)
            .expect("peer")
            .requested_blocks
            .contains(&open_bitcoin_consensus::block_hash(&next_header))
    );
}

#[test]
fn inbound_inv_over_tx_request_cap_returns_resource_limit_disconnect() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(104).expect("inbound peer");
    let inventory = transaction_inventory(PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER + 1);

    // Act
    let actions = manager
        .handle_message(104, WireNetworkMessage::Inv(inventory), 1)
        .expect("inventory cap should be handled as a disconnect action");

    // Assert
    assert_resource_limit_disconnect(&actions);
    let snapshot = manager.transaction_request_snapshot(104);
    assert_eq!(snapshot.in_flight_count, 0);
    assert_eq!(snapshot.candidate_count, 0);
}

#[test]
fn inbound_getdata_over_inventory_cap_disconnects_without_serving_inventory() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(105).expect("inbound peer");
    let inventory = block_inventory(PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1);

    // Act
    let actions = manager
        .handle_message(105, WireNetworkMessage::GetData(inventory), 1)
        .expect("getdata cap should be handled as a disconnect action");

    // Assert
    assert_resource_limit_disconnect(&actions);
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::ServeInventory(_)))
    );
}

#[test]
fn phase111_getdata_block_witness_and_compact_inventory_stays_after_request_pressure_gate() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(111_001).expect("inbound peer");
    let txid_hash = hash_from_index(111_004);
    let wtxid_hash = hash_from_index(111_005);
    let inventory = InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: hash_from_index(111_001),
        },
        InventoryVector {
            inventory_type: InventoryType::WitnessBlock,
            object_hash: hash_from_index(111_002),
        },
        InventoryVector {
            inventory_type: InventoryType::CompactBlock,
            object_hash: hash_from_index(111_003),
        },
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: txid_hash,
        },
        InventoryVector {
            inventory_type: InventoryType::WitnessTransaction,
            object_hash: wtxid_hash,
        },
    ]);

    // Act
    let actions = manager
        .handle_message(111_001, WireNetworkMessage::GetData(inventory), 1)
        .expect("mixed getdata should pass request pressure");

    // Assert
    let [PeerAction::ServeInventory(served)] = actions.as_slice() else {
        panic!("expected one ServeInventory action, got {actions:?}");
    };
    let served_types = served
        .iter()
        .map(|item| item.inventory_type)
        .collect::<Vec<_>>();
    assert_eq!(
        served_types,
        vec![
            InventoryType::Block,
            InventoryType::WitnessBlock,
            InventoryType::CompactBlock,
            InventoryType::Transaction,
            InventoryType::WitnessTransaction,
        ]
    );
    assert_eq!(served[3].object_hash, txid_hash);
    assert_eq!(served[4].object_hash, wtxid_hash);
}

#[test]
fn phase111_over_cap_block_witness_compact_getdata_disconnects_before_serve_inventory() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(111_002).expect("inbound peer");
    let inventory =
        phase111_block_witness_compact_inventory(PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1);

    // Act
    let actions = manager
        .handle_message(111_002, WireNetworkMessage::GetData(inventory), 1)
        .expect("over-cap getdata should disconnect before serving");

    // Assert
    assert_resource_limit_disconnect(&actions);
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::ServeInventory(_)))
    );
}

#[test]
fn phase111_compact_block_getdata_does_not_enter_requested_blocks() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(111_003).expect("inbound peer");
    let compact_hash = hash_from_index(111_003);
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::CompactBlock,
        object_hash: compact_hash,
    }]);

    // Act
    let actions = manager
        .handle_message(111_003, WireNetworkMessage::GetData(inventory), 1)
        .expect("compact getdata should remain classified inventory");

    // Assert
    let [PeerAction::ServeInventory(served)] = actions.as_slice() else {
        panic!("expected compact request to stay in ServeInventory, got {actions:?}");
    };
    assert_eq!(
        served,
        &vec![InventoryVector {
            inventory_type: InventoryType::CompactBlock,
            object_hash: compact_hash,
        }]
    );
    assert!(
        manager
            .peer_requested_blocks(111_003)
            .expect("requested blocks")
            .is_empty()
    );
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::Block(_))))
    );
}

#[test]
fn phase111_compact_block_burst_remains_bounded_without_partial_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(111_009).expect("inbound peer");
    let inventory = compact_block_inventory(PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1);

    // Act
    let actions = manager
        .handle_message(111_009, WireNetworkMessage::GetData(inventory), 1)
        .expect("over-cap compact getdata should disconnect before serving");

    // Assert
    assert_resource_limit_disconnect(&actions);
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::ServeInventory(_)))
    );
    assert!(
        manager
            .peer_requested_blocks(111_009)
            .expect("requested blocks")
            .is_empty()
    );
}

#[test]
fn phase111_full_witness_block_cleanup_matrix_uses_phase110_labels() {
    // Arrange
    let released_cases = [
        BlockInFlightCleanupCause::ReceivedBlock,
        BlockInFlightCleanupCause::NotFound,
    ];
    let other_cases = [
        (
            BlockInFlightCleanupCause::PeerDisconnect,
            "block_inflight_cleanup_peer_removed",
        ),
        (
            BlockInFlightCleanupCause::Timeout,
            "block_inflight_cleanup_timeout",
        ),
        (
            BlockInFlightCleanupCause::RuntimeRestart,
            "block_inflight_cleanup_restart",
        ),
    ];

    // Act
    let released_labels = released_cases
        .into_iter()
        .map(|cause| cleanup_label_for(cause, 2, 2, 0))
        .collect::<Vec<_>>();
    let other_labels = other_cases
        .into_iter()
        .map(|(cause, expected)| (cleanup_label_for(cause, 2, 2, 0), expected))
        .collect::<Vec<_>>();
    let still_limited_label = cleanup_label_for(
        BlockInFlightCleanupCause::ReceivedBlock,
        PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER + 1,
        1,
        PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
    );

    // Assert
    assert_eq!(
        released_labels,
        vec![
            "block_inflight_cleanup_released",
            "block_inflight_cleanup_released",
        ],
    );
    for (actual, expected) in other_labels {
        assert_eq!(actual, expected);
    }
    assert_eq!(still_limited_label, "block_inflight_limit_still_reached",);
}

#[test]
fn inbound_getheaders_over_locator_cap_disconnects_without_header_response() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(108).expect("inbound peer");
    let locator = open_bitcoin_primitives::BlockLocator {
        block_hashes: (0..=PHASE94_MAX_HEADER_LOCATOR_HASHES)
            .map(hash_from_index)
            .collect(),
    };

    // Act
    let actions = manager
        .handle_message(
            108,
            WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            },
            1,
        )
        .expect("getheaders cap should be handled as a disconnect action");

    // Assert
    assert_resource_limit_disconnect(&actions);
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::Headers(_))))
    );
}

#[test]
fn headers_response_caps_block_requests_to_in_flight_limit() {
    // Arrange
    let mut manager = PeerManager::with_max_blocks_in_flight(local_config(), 1);
    let genesis_header = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis_hash = open_bitcoin_consensus::block_hash(&genesis_header);
    let first_header = mined_header(genesis_hash, 2);
    let first_hash = open_bitcoin_consensus::block_hash(&first_header);
    let second_header = mined_header(first_hash, 3);
    manager.seed_local_chain(&[ChainPosition::new(genesis_header, 0, 1, 0)]);
    manager.add_outbound_peer(12, 10).expect("peer");

    // Act
    let header_actions = manager
        .handle_message(
            12,
            WireNetworkMessage::Headers(crate::HeadersMessage {
                headers: vec![first_header.clone(), second_header.clone()],
            }),
            14,
        )
        .expect("headers");

    // Assert
    let [PeerAction::Send(WireNetworkMessage::GetData(inventory))] = header_actions.as_slice()
    else {
        panic!("expected one getdata action");
    };
    assert_eq!(inventory.inventory.len(), 1);
    assert_eq!(manager.max_blocks_in_flight_per_peer(), 1);
    assert!(
        manager
            .peer_state(12)
            .expect("peer")
            .requested_blocks
            .contains(&open_bitcoin_consensus::block_hash(&first_header))
    );
    assert!(
        !manager
            .peer_state(12)
            .expect("peer")
            .requested_blocks
            .contains(&open_bitcoin_consensus::block_hash(&second_header))
    );
}
