// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_consensus::{check_block_header, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader, Hash32, MerkleRoot, NetworkMagic};

use crate::{
    ConnectionRole, HeaderStore, HeaderSyncPolicy, HeadersMessage, InventoryList, LocalPeerConfig,
    PeerAction, PeerManager, ServiceFlags, WireNetworkMessage,
};
use open_bitcoin_primitives::{InventoryType, InventoryVector};

fn local_config() -> LocalPeerConfig {
    LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: super::super::message::zero_address(),
        nonce: 7,
        relay: true,
        user_agent: "/open-bitcoin:test/".to_string(),
    }
}

fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: 1_231_006_500 + nonce,
        bits: 0x207f_ffff,
        nonce,
    }
}

fn mined_header(previous_block_hash: BlockHash, seed: u32) -> BlockHeader {
    let mut header = header(previous_block_hash, seed);
    let nonce = (0..=u32::MAX)
        .find(|nonce| {
            header.nonce = *nonce;
            check_block_header(&header).is_ok()
        })
        .expect("expected nonce at easy target");
    header.nonce = nonce;
    header
}

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
fn announce_transaction_uses_wtxidrelay_when_peer_negotiates_it() {
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(4).expect("peer");
    manager
        .handle_message(
            4,
            WireNetworkMessage::Version(crate::VersionMessage::default()),
            20,
        )
        .expect("version");
    manager
        .handle_message(4, WireNetworkMessage::WtxidRelay, 20)
        .expect("wtxidrelay");

    let transaction = open_bitcoin_primitives::Transaction::default();
    let announcement = manager
        .announce_transaction(4, &transaction)
        .expect("announce")
        .expect("message");

    assert!(matches!(
        announcement,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory[0].inventory_type == InventoryType::WitnessTransaction
    ));
}

#[test]
fn helper_methods_and_unknown_peer_errors_are_covered() {
    let mut manager = PeerManager::new(local_config());
    assert!(manager.peer_state(99).is_none());
    assert_eq!(
        manager
            .peer_requested_blocks(99)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .remove_peer(99)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::Version(Default::default()), 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .request_ping(99, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 1),
        transactions: Vec::new(),
    };
    assert_eq!(
        manager
            .announce_block(99, &block)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .announce_transaction(99, &open_bitcoin_primitives::Transaction::default())
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::Verack, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );

    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 2);
    let position = ChainPosition::new(genesis, 0, 1, 0);
    manager.seed_local_chain(std::slice::from_ref(&position));
    manager.note_local_position(&position);
    manager
        .note_local_transaction(&open_bitcoin_primitives::Transaction::default())
        .expect("local transaction");
    assert_eq!(manager.header_store().best_height(), 0);

    let mut restored_headers = HeaderStore::default();
    restored_headers.seed_from_chain(std::slice::from_ref(&position));
    let mut restored_manager = PeerManager::new(local_config());
    restored_manager.seed_header_store(restored_headers);
    assert_eq!(restored_manager.header_store().best_height(), 0);
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

#[test]
fn ping_block_announcement_and_duplicate_add_paths_are_exercised() {
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(5).expect("peer");
    assert_eq!(
        manager
            .add_inbound_peer(5)
            .expect_err("duplicate peer")
            .to_string(),
        "peer already exists: 5",
    );
    assert_eq!(
        manager
            .add_outbound_peer(5, 1)
            .expect_err("duplicate peer")
            .to_string(),
        "peer already exists: 5",
    );

    let ping = manager.request_ping(5, 123).expect("ping");
    assert_eq!(ping, WireNetworkMessage::Ping { nonce: 123 });
    manager
        .handle_message(5, WireNetworkMessage::Pong { nonce: 123 }, 1)
        .expect("pong");
    assert!(
        manager
            .peer_state(5)
            .expect("state")
            .last_ping_nonce
            .is_none()
    );

    let block = Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 4),
        transactions: Vec::new(),
    };
    let inv_message = manager
        .announce_block(5, &block)
        .expect("announce")
        .expect("inv");
    assert!(matches!(
        inv_message,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory[0].inventory_type == InventoryType::Block
    ));

    manager
        .handle_message(5, WireNetworkMessage::SendHeaders, 2)
        .expect("sendheaders");
    let headers_message = manager
        .announce_block(5, &block)
        .expect("announce")
        .expect("headers");
    assert!(matches!(
        headers_message,
        WireNetworkMessage::Headers(HeadersMessage { headers }) if headers.len() == 1
    ));

    let transaction = open_bitcoin_primitives::Transaction::default();
    let announcement = manager
        .announce_transaction(5, &transaction)
        .expect("announce")
        .expect("message");
    assert!(matches!(
        announcement,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory[0].inventory_type == InventoryType::Transaction
    ));

    manager.remove_peer(5).expect("remove peer");
    assert!(manager.peer_state(5).is_none());
}

#[test]
fn inventory_requests_and_notfound_paths_cover_tx_and_block_modes() {
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(6).expect("peer");
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::Inv(InventoryList::default()), 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );

    let txid_inv = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: Hash32::from_byte_array([2_u8; 32]),
    }]);
    let txid_actions = manager
        .handle_message(6, WireNetworkMessage::Inv(txid_inv), 1)
        .expect("txid inventory");
    assert!(matches!(
        txid_actions.as_slice(),
        [PeerAction::Send(WireNetworkMessage::GetData(_))]
    ));

    manager
        .handle_message(6, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::WtxidRelay, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::SendHeaders, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::Pong { nonce: 1 }, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    let wtxid_inv = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::WitnessTransaction,
        object_hash: Hash32::from_byte_array([3_u8; 32]),
    }]);
    let wtxid_actions = manager
        .handle_message(6, WireNetworkMessage::Inv(wtxid_inv), 2)
        .expect("wtxid inventory");
    assert!(matches!(
        wtxid_actions.as_slice(),
        [PeerAction::Send(WireNetworkMessage::GetData(_))]
    ));
    let ignored_inventory = manager
        .handle_message(
            6,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::CompactBlock,
                object_hash: Hash32::from_byte_array([4_u8; 32]),
            }])),
            2,
        )
        .expect("ignored inventory");
    assert!(ignored_inventory.is_empty());

    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 5);
    manager.seed_local_chain(&[ChainPosition::new(genesis.clone(), 0, 1, 0)]);
    let next = mined_header(open_bitcoin_consensus::block_hash(&genesis), 6);
    manager
        .handle_message(
            6,
            WireNetworkMessage::Headers(crate::HeadersMessage {
                headers: vec![next.clone()],
            }),
            3,
        )
        .expect("headers");

    let not_found = InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Hash32::from_byte_array([2_u8; 32]),
        },
        InventoryVector {
            inventory_type: InventoryType::WitnessTransaction,
            object_hash: Hash32::from_byte_array([3_u8; 32]),
        },
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: open_bitcoin_consensus::block_hash(&next).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::CompactBlock,
            object_hash: Hash32::from_byte_array([4_u8; 32]),
        },
    ]);
    manager
        .handle_message(6, WireNetworkMessage::NotFound(not_found), 4)
        .expect("notfound");
    let peer = manager.peer_state(6).expect("peer");
    assert!(peer.requested_txids.is_empty());
    assert!(peer.requested_wtxids.is_empty());
    assert!(peer.requested_blocks.is_empty());
}

#[test]
fn received_tx_and_block_clear_requested_inventory() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(8).expect("peer");

    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");

    manager
        .handle_message(
            8,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }])),
            1,
        )
        .expect("txid inventory");
    manager
        .handle_message(8, WireNetworkMessage::WtxidRelay, 2)
        .expect("wtxidrelay");
    manager
        .handle_message(
            8,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: wtxid.into(),
            }])),
            3,
        )
        .expect("wtxid inventory");

    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 7);
    manager.seed_local_chain(&[ChainPosition::new(genesis.clone(), 0, 1, 0)]);
    let next = mined_header(open_bitcoin_consensus::block_hash(&genesis), 8);
    manager
        .handle_message(
            8,
            WireNetworkMessage::Headers(crate::HeadersMessage {
                headers: vec![next.clone()],
            }),
            4,
        )
        .expect("headers");

    // Act
    manager
        .handle_message(8, WireNetworkMessage::Tx(transaction), 5)
        .expect("transaction");
    manager
        .handle_message(
            8,
            WireNetworkMessage::Block(Block {
                header: next,
                transactions: Vec::new(),
            }),
            6,
        )
        .expect("block");

    // Assert
    let peer = manager.peer_state(8).expect("peer");
    assert!(peer.requested_txids.is_empty());
    assert!(peer.requested_wtxids.is_empty());
    assert!(peer.requested_blocks.is_empty());
}

#[test]
fn getheaders_headers_tx_and_block_paths_are_explicit() {
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(7).expect("peer");

    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 7);
    let genesis_position = ChainPosition::new(genesis.clone(), 0, 1, 0);
    manager.seed_local_chain(std::slice::from_ref(&genesis_position));

    let getheaders_actions = manager
        .handle_message(
            7,
            WireNetworkMessage::GetHeaders {
                locator: open_bitcoin_primitives::BlockLocator::default(),
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            },
            1,
        )
        .expect("getheaders");
    assert!(matches!(
        getheaders_actions.as_slice(),
        [PeerAction::Send(WireNetworkMessage::Headers(HeadersMessage { headers }))]
        if headers.len() == 1
    ));
    assert_eq!(
        manager
            .handle_message(
                99,
                WireNetworkMessage::Headers(crate::HeadersMessage {
                    headers: vec![genesis.clone()],
                }),
                1,
            )
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );

    let missing_parent = mined_header(BlockHash::from_byte_array([8_u8; 32]), 8);
    assert_eq!(
        manager
            .handle_message(
                7,
                WireNetworkMessage::Headers(crate::HeadersMessage {
                    headers: vec![missing_parent],
                }),
                2,
            )
            .expect_err("missing ancestor")
            .to_string(),
        format!(
            "missing header ancestor: {:?}",
            BlockHash::from_byte_array([8_u8; 32]).to_byte_array()
        ),
    );
    let invalid_pow_header = header(genesis_position.block_hash, 99);
    assert_eq!(
        manager
            .handle_message(
                7,
                WireNetworkMessage::Headers(crate::HeadersMessage {
                    headers: vec![invalid_pow_header],
                }),
                2,
            )
            .expect_err("invalid pow")
            .to_string(),
        "invalid header: high-hash (proof of work failed)",
    );
    let empty_headers = manager
        .handle_message(
            7,
            WireNetworkMessage::Headers(crate::HeadersMessage { headers: vec![] }),
            3,
        )
        .expect("empty headers");
    assert!(empty_headers.is_empty());

    let served = manager
        .handle_message(
            7,
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: genesis_position.block_hash.into(),
            }])),
            3,
        )
        .expect("getdata");
    assert!(matches!(served.as_slice(), [PeerAction::ServeInventory(_)]));
    assert_eq!(
        manager
            .handle_message(
                99,
                WireNetworkMessage::NotFound(InventoryList::default()),
                3
            )
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );

    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = open_bitcoin_consensus::transaction_txid(&transaction).expect("txid");
    let wtxid = open_bitcoin_consensus::transaction_wtxid(&transaction).expect("wtxid");
    let tx_actions = manager
        .handle_message(7, WireNetworkMessage::Tx(transaction), 4)
        .expect("tx");
    assert!(matches!(
        tx_actions.as_slice(),
        [PeerAction::ReceivedTransaction(_)]
    ));
    assert_eq!(
        manager
            .handle_message(
                99,
                WireNetworkMessage::Tx(open_bitcoin_primitives::Transaction::default()),
                4,
            )
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    let block = Block {
        header: genesis,
        transactions: Vec::new(),
    };
    let block_hash = open_bitcoin_consensus::block_hash(&block.header);
    let block_actions = manager
        .handle_message(7, WireNetworkMessage::Block(block), 5)
        .expect("block");
    assert!(matches!(
        block_actions.as_slice(),
        [PeerAction::ReceivedBlock(_)]
    ));
    assert_eq!(
        manager
            .handle_message(
                99,
                WireNetworkMessage::Block(Block {
                    header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 10),
                    transactions: Vec::new(),
                }),
                5,
            )
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    let peer = manager.peer_state(7).expect("peer");
    assert!(!peer.requested_txids.contains(&txid));
    assert!(!peer.requested_wtxids.contains(&wtxid));
    assert!(!peer.requested_blocks.contains(&block_hash));
}
