// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use core::net::IpAddr;
use std::collections::BTreeSet;

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_consensus::{check_block_header, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Block, BlockHash, BlockHeader, Hash32, MerkleRoot, NetworkAddress, NetworkMagic,
};

use crate::{
    ConnectionRole, DisconnectReason, HeaderStore, HeaderSyncPolicy, HeadersMessage,
    InboundAdmissionRejectionReason, InboundAdmissionSlotClass, InboundHandshakeState,
    InboundPeerRecord, InboundPermissionDecision, InventoryList, LocalPeerConfig,
    ParsedPeerPermissionClass, PeerAction, PeerConnectionClass, PeerId, PeerManager,
    PeerPermissionClassRegistry, ServiceFlags, WireNetworkMessage,
};
use open_bitcoin_primitives::{InventoryType, InventoryVector};

use crate::address::{
    AddressAnnouncement, AddressDecisionLabel, AddressDecisionReason, AddressList,
    AddressNetworkKind, AddressSourceKind, GetAddrResponseDecision, LocalAdvertisementDecision,
    PHASE92_GETADDR_RESPONSE_LIMIT, PHASE92_LEARNED_ADDR_BATCH_LIMIT, RoutabilityClass,
};

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

fn protected_permission_decision() -> InboundPermissionDecision {
    let class =
        ParsedPeerPermissionClass::parse("protected-test", ["203.0.113.7"], ["in", "forceinbound"])
            .expect("protected class");
    let address: IpAddr = "203.0.113.7".parse().expect("test address");
    PeerPermissionClassRegistry::new([class]).resolve_inbound(address)
}

fn permission_decision(
    tokens: impl IntoIterator<Item = &'static str>,
) -> InboundPermissionDecision {
    let class = ParsedPeerPermissionClass::parse("phase91-test", ["203.0.113.91"], tokens)
        .expect("permission class");
    let address: IpAddr = "203.0.113.91".parse().expect("test address");
    PeerPermissionClassRegistry::new([class]).resolve_inbound(address)
}

fn permissioned_inbound_record(
    peer_id: PeerId,
    permission_decision: InboundPermissionDecision,
) -> InboundPeerRecord {
    InboundPeerRecord {
        peer_id,
        remote_endpoint: format!("127.0.0.1:{peer_id}"),
        slot_class: permission_decision.slot_class(),
        connection_class: permission_decision.connection_class(),
        permission_decision,
        handshake_state: InboundHandshakeState::Accepted,
        maybe_remote_nonce: None,
        observed_inbound_peers: 0,
        observed_outbound_peers: 1,
    }
}

fn active_permission_labels(decision: &InboundPermissionDecision) -> Vec<&'static str> {
    decision
        .active_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect()
}

fn inactive_permission_labels(decision: &InboundPermissionDecision) -> Vec<&'static str> {
    decision
        .inactive_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect()
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
fn inbound_peer_record_stores_endpoint_and_starts_handshaking() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let record = InboundPeerRecord {
        peer_id: 31,
        remote_endpoint: "127.0.0.1:18444".to_string(),
        slot_class: InboundAdmissionSlotClass::Reserved,
        connection_class: PeerConnectionClass::ProtectedInbound,
        permission_decision: protected_permission_decision(),
        handshake_state: InboundHandshakeState::Accepted,
        maybe_remote_nonce: None,
        observed_inbound_peers: 0,
        observed_outbound_peers: 2,
    };

    // Act
    manager
        .add_inbound_peer_record(record)
        .expect("inbound record should be stored");

    // Assert
    let peer = manager.peer_state(31).expect("peer state");
    assert_eq!(peer.role, ConnectionRole::Inbound);
    assert_eq!(
        peer.maybe_inbound_record
            .as_ref()
            .expect("inbound record")
            .remote_endpoint,
        "127.0.0.1:18444",
    );
    assert_eq!(
        peer.maybe_inbound_record
            .as_ref()
            .expect("inbound record")
            .slot_class,
        InboundAdmissionSlotClass::Reserved,
    );
    assert_eq!(
        peer.maybe_inbound_record
            .as_ref()
            .expect("inbound record")
            .handshake_state,
        InboundHandshakeState::Handshaking,
    );
}

#[test]
fn simple_inbound_helper_creates_compatible_inbound_record() {
    // Arrange
    let mut manager = PeerManager::new(local_config());

    // Act
    manager.add_inbound_peer(32).expect("peer should be added");

    // Assert
    let peer = manager.peer_state(32).expect("peer state");
    assert_eq!(peer.role, ConnectionRole::Inbound);
    assert_eq!(
        peer.maybe_inbound_record
            .as_ref()
            .expect("compatible inbound record")
            .peer_id,
        32,
    );
    assert_eq!(
        peer.maybe_inbound_record
            .as_ref()
            .expect("compatible inbound record")
            .slot_class,
        InboundAdmissionSlotClass::Ordinary,
    );
    assert_eq!(
        peer.maybe_inbound_record
            .as_ref()
            .expect("compatible inbound record")
            .handshake_state,
        InboundHandshakeState::Handshaking,
    );
}

#[test]
fn inbound_self_connection_version_rejects_without_establishing_peer() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(33).expect("peer should be added");

    // Act
    let actions = manager
        .handle_message(
            33,
            WireNetworkMessage::Version(crate::VersionMessage {
                nonce: local_config().nonce,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("self connection should be rejected as an action");

    // Assert
    assert_eq!(
        actions,
        vec![PeerAction::Disconnect(DisconnectReason::SelfConnection)],
    );
    let peer = manager.peer_state(33).expect("peer state");
    let inbound_record = peer.maybe_inbound_record.as_ref().expect("inbound record");
    assert_eq!(
        inbound_record.handshake_state,
        InboundHandshakeState::Disconnected,
    );
    assert_eq!(
        inbound_record.maybe_remote_nonce,
        Some(local_config().nonce)
    );
    assert_eq!(
        peer.maybe_inbound_rejection_reason,
        Some(InboundAdmissionRejectionReason::SelfConnection),
    );
    assert!(!peer.remote_version_received);
    assert!(!peer.local_verack_sent);
}

#[test]
fn inbound_handshake_uses_existing_peer_action_flow() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(34).expect("peer should be added");

    // Act
    let actions = manager
        .handle_message(
            34,
            WireNetworkMessage::Version(crate::VersionMessage {
                nonce: 99,
                start_height: 3,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version should process");

    // Assert
    assert_eq!(
        actions,
        vec![
            PeerAction::Send(WireNetworkMessage::Version(
                local_config().version_message(11, -1)
            )),
            PeerAction::Send(WireNetworkMessage::WtxidRelay),
            PeerAction::Send(WireNetworkMessage::Verack),
            PeerAction::Send(WireNetworkMessage::SendHeaders),
        ],
    );
    let peer = manager.peer_state(34).expect("peer state");
    assert!(peer.remote_version_received);
    assert!(peer.local_version_sent);
    assert!(peer.local_verack_sent);
    assert_eq!(
        peer.maybe_inbound_record
            .as_ref()
            .expect("inbound record")
            .maybe_remote_nonce,
        Some(99),
    );
    assert_eq!(
        peer.maybe_inbound_record
            .as_ref()
            .expect("inbound record")
            .handshake_state,
        InboundHandshakeState::Handshaking,
    );
}

#[test]
fn inbound_counters_and_endpoint_keys_ignore_disconnected_records() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_outbound_peer(40, 10)
        .expect("outbound peer should be added");
    manager
        .add_inbound_peer_record(InboundPeerRecord {
            peer_id: 41,
            remote_endpoint: "127.0.0.1:18441".to_string(),
            slot_class: InboundAdmissionSlotClass::Ordinary,
            connection_class: PeerConnectionClass::OrdinaryInbound,
            permission_decision: InboundPermissionDecision::ordinary(),
            handshake_state: InboundHandshakeState::Accepted,
            maybe_remote_nonce: None,
            observed_inbound_peers: 0,
            observed_outbound_peers: 1,
        })
        .expect("ordinary inbound peer should be added");
    manager
        .add_inbound_peer_record(InboundPeerRecord {
            peer_id: 42,
            remote_endpoint: "127.0.0.1:18442".to_string(),
            slot_class: InboundAdmissionSlotClass::Reserved,
            connection_class: PeerConnectionClass::ProtectedInbound,
            permission_decision: protected_permission_decision(),
            handshake_state: InboundHandshakeState::Accepted,
            maybe_remote_nonce: None,
            observed_inbound_peers: 1,
            observed_outbound_peers: 1,
        })
        .expect("reserved inbound peer should be added");
    manager
        .handle_message(
            41,
            WireNetworkMessage::Version(crate::VersionMessage {
                nonce: local_config().nonce,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("self connection should be represented as a disconnect action");

    // Act
    let endpoint_keys = manager.inbound_endpoint_keys();
    let counters = manager.inbound_admission_counters();
    let peer_ids = manager.peer_ids();

    // Assert
    assert_eq!(
        endpoint_keys,
        BTreeSet::from(["127.0.0.1:18442".to_string()])
    );
    assert_eq!(counters.current_inbound_peers, 1);
    assert_eq!(counters.current_reserved_inbound_peers, 1);
    assert_eq!(counters.current_outbound_peers, 1);
    assert_eq!(peer_ids, BTreeSet::from([40, 41, 42]));
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
fn relay_permission_labels_remain_inactive_for_transaction_paths() {
    // Arrange
    let permission_decision = permission_decision(["in", "relay", "forcerelay", "mempool"]);
    assert!(active_permission_labels(&permission_decision).is_empty());
    assert_eq!(
        inactive_permission_labels(&permission_decision),
        vec!["inactive_relay", "inactive_forcerelay", "inactive_mempool"]
    );
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let mut manager = PeerManager::new(local_config());
    manager
        .add_inbound_peer_record(permissioned_inbound_record(91, permission_decision))
        .expect("permissioned inbound peer should be added");

    // Act
    let txid_inventory_actions = manager
        .handle_message(
            91,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }])),
            1,
        )
        .expect("transaction inventory");
    let wtxidrelay_actions = manager
        .handle_message(91, WireNetworkMessage::WtxidRelay, 2)
        .expect("wtxidrelay");
    let wtxid_inventory_actions = manager
        .handle_message(
            91,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: wtxid.into(),
            }])),
            3,
        )
        .expect("witness transaction inventory");
    let tx_actions = manager
        .handle_message(91, WireNetworkMessage::Tx(transaction.clone()), 4)
        .expect("transaction");
    let getdata_actions = manager
        .handle_message(
            91,
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }])),
            5,
        )
        .expect("getdata");

    // Assert
    let [PeerAction::Send(WireNetworkMessage::GetData(txid_inventory))] =
        txid_inventory_actions.as_slice()
    else {
        panic!("expected txid getdata action");
    };
    assert_eq!(txid_inventory.inventory.len(), 1);
    assert_eq!(
        txid_inventory.inventory[0].inventory_type,
        InventoryType::Transaction
    );
    assert!(wtxidrelay_actions.is_empty());
    let [PeerAction::Send(WireNetworkMessage::GetData(wtxid_inventory))] =
        wtxid_inventory_actions.as_slice()
    else {
        panic!("expected wtxid getdata action");
    };
    assert_eq!(wtxid_inventory.inventory.len(), 1);
    assert_eq!(
        wtxid_inventory.inventory[0].inventory_type,
        InventoryType::WitnessTransaction
    );
    assert_eq!(
        tx_actions,
        vec![PeerAction::ReceivedTransaction(transaction)]
    );
    assert_eq!(
        getdata_actions,
        vec![PeerAction::ServeInventory(vec![InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: txid.into(),
        }])]
    );
}

#[test]
fn filter_permission_labels_remain_inactive_without_service_bits_or_compact_blocks() {
    // Arrange
    let permission_decision = permission_decision(["in", "all"]);
    assert_eq!(
        inactive_permission_labels(&permission_decision),
        vec![
            "inactive_relay",
            "inactive_forcerelay",
            "inactive_mempool",
            "inactive_bloomfilter",
            "inactive_blockfilters",
        ]
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
fn inbound_addr_messages_update_learned_address_evidence_without_actions() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(93).expect("peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let addresses = AddressList {
        addresses: vec![
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(8, 8, 8, 8, 8333),
            ),
            address_announcement(
                now_unix_seconds,
                public_ipv6_network_address("2606:4700:4700::1111", 8333),
            ),
        ],
    };

    // Act
    let addr_actions = manager
        .handle_message(
            93,
            WireNetworkMessage::Addr(addresses),
            now_unix_seconds as i64,
        )
        .expect("addr should be learned");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert!(addr_actions.is_empty());
    assert_eq!(evidence.learned_address_entries.len(), 2);
    assert!(evidence.learned_address_entries.iter().all(|entry| {
        entry.source == AddressSourceKind::InboundAddr
            && entry.routability == RoutabilityClass::PubliclyRoutable
            && entry.persistence_eligible
    }));
    assert!(evidence.learned_address_rejections.is_empty());
    assert_eq!(
        evidence
            .maybe_latest_address_decision
            .expect("latest decision")
            .label
            .as_str(),
        "learned_accepted",
    );
}

#[test]
fn inbound_addr_rejections_are_recorded_without_disconnect_actions() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(94).expect("peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let accepted = address_announcement(
        now_unix_seconds,
        public_ipv4_network_address(8, 8, 4, 4, 8333),
    );
    manager
        .handle_message(
            94,
            WireNetworkMessage::Addr(AddressList {
                addresses: vec![accepted.clone()],
            }),
            now_unix_seconds as i64,
        )
        .expect("seed address should be learned");
    let rejected_addresses = AddressList {
        addresses: vec![
            address_announcement(now_unix_seconds, public_ipv4_network_address(8, 8, 8, 8, 0)),
            address_announcement(
                now_unix_seconds - crate::PHASE92_MAX_ADDR_AGE_SECONDS - 1,
                public_ipv4_network_address(8, 8, 8, 8, 8333),
            ),
            address_announcement(
                now_unix_seconds + crate::PHASE92_MAX_FUTURE_SKEW_SECONDS + 1,
                public_ipv4_network_address(1, 1, 1, 1, 8333),
            ),
            accepted,
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(127, 0, 0, 1, 8333),
            ),
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(10, 0, 0, 1, 8333),
            ),
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(192, 0, 2, 1, 8333),
            ),
        ],
    };

    // Act
    let actions = manager
        .handle_message(
            94,
            WireNetworkMessage::Addr(rejected_addresses),
            now_unix_seconds as i64,
        )
        .expect("addr rejections should be evidence only");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert!(actions.is_empty());
    assert_eq!(evidence.learned_address_entries.len(), 1);
    assert_eq!(
        evidence
            .learned_address_rejections
            .iter()
            .map(|decision| decision.label.as_str())
            .collect::<Vec<_>>(),
        vec![
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
            "learned_rejected",
        ],
    );
    assert_eq!(
        evidence
            .learned_address_rejections
            .iter()
            .map(|decision| decision.reason.as_str())
            .collect::<Vec<_>>(),
        vec![
            "invalid_port",
            "stale_or_future",
            "stale_or_future",
            "duplicate_address",
            "not_publicly_routable",
            "not_publicly_routable",
            "not_publicly_routable",
        ],
    );
}

#[test]
fn over_cap_addr_batch_records_batch_rejection_without_partial_inserts() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(95).expect("peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let addresses = (0..=PHASE92_LEARNED_ADDR_BATCH_LIMIT)
        .map(|index| {
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(8, 8, 8, index as u8, 8333),
            )
        })
        .collect();

    // Act
    let actions = manager
        .handle_message(
            95,
            WireNetworkMessage::Addr(AddressList { addresses }),
            now_unix_seconds as i64,
        )
        .expect("over-cap addr should be rejected as evidence");
    let evidence = manager.address_boundary_evidence();
    let latest = evidence
        .maybe_latest_address_decision
        .expect("batch rejection should be latest decision");

    // Assert
    assert!(actions.is_empty());
    assert!(evidence.learned_address_entries.is_empty());
    assert_eq!(latest.label, AddressDecisionLabel::LearnedRejected);
    assert_eq!(latest.reason, AddressDecisionReason::OverCapBatch);
    assert_eq!(latest.label.as_str(), "learned_rejected");
    assert_eq!(latest.reason.as_str(), "over_cap_batch");
}

#[test]
fn outbound_addr_messages_parse_without_response_or_relay_actions() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_outbound_peer(96, 1)
        .expect("outbound peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let addresses = AddressList {
        addresses: vec![address_announcement(
            now_unix_seconds,
            public_ipv4_network_address(8, 8, 8, 8, 8333),
        )],
    };

    // Act
    let actions = manager
        .handle_message(
            96,
            WireNetworkMessage::Addr(addresses),
            now_unix_seconds as i64,
        )
        .expect("outbound addr should parse");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert!(actions.is_empty());
    assert_eq!(evidence.learned_address_entries.len(), 1);
    assert!(evidence.getaddr_responses_served.is_empty());
    assert!(evidence.getaddr_requests_suppressed.is_empty());
}

#[test]
fn addr_unknown_peer_empty_and_local_duplicate_paths_are_evidence_only() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(97).expect("peer should be added");
    let local_address = public_ipv4_network_address(8, 8, 8, 8, 8333);
    manager.local_address_decisions = vec![LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseCandidate,
        reason: AddressDecisionReason::PolicyAccepted,
        source: AddressSourceKind::LocalListener,
        network_kind: AddressNetworkKind::Ipv4,
        routability: RoutabilityClass::PubliclyRoutable,
        services_bits: (ServiceFlags::NETWORK | ServiceFlags::WITNESS).bits(),
        port: local_address.port,
        maybe_wire_address: Some(local_address.clone()),
    }];

    // Act
    let unknown_peer_error = manager
        .handle_message(404, WireNetworkMessage::Addr(AddressList::default()), -1)
        .expect_err("unknown peer should fail");
    let empty_actions = manager
        .handle_message(97, WireNetworkMessage::Addr(AddressList::default()), -1)
        .expect("empty addr should be evidence only");
    let duplicate_actions = manager
        .handle_message(
            97,
            WireNetworkMessage::Addr(AddressList {
                addresses: vec![address_announcement(0, local_address)],
            }),
            -1,
        )
        .expect("duplicate local addr should be evidence only");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert_eq!(unknown_peer_error.to_string(), "unknown peer: 404");
    assert!(empty_actions.is_empty());
    assert!(duplicate_actions.is_empty());
    assert!(evidence.learned_address_entries.is_empty());
    assert_eq!(evidence.learned_address_rejections.len(), 1);
    assert_eq!(
        evidence.learned_address_rejections[0].reason,
        AddressDecisionReason::DuplicateAddress,
    );
    assert_eq!(
        evidence
            .maybe_latest_address_decision
            .expect("local duplicate latest decision")
            .reason
            .as_str(),
        "duplicate_address",
    );
}

#[test]
fn permissioned_inbound_getaddr_serves_once_and_records_repeated_suppression() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let now_unix_seconds = 1_700_000_000;
    let local_address = public_ipv4_network_address(9, 9, 9, 9, 8333);
    manager.set_local_address_decisions(vec![local_advertisement_candidate(local_address.clone())]);
    manager.add_inbound_peer(98).expect("seed peer");
    let learned_addresses = (0..(PHASE92_GETADDR_RESPONSE_LIMIT + 2))
        .map(|index| {
            address_announcement(
                now_unix_seconds,
                public_ipv4_network_address(11, 0, 0, (index + 1) as u8, 8333),
            )
        })
        .collect();
    manager
        .handle_message(
            98,
            WireNetworkMessage::Addr(AddressList {
                addresses: learned_addresses,
            }),
            now_unix_seconds as i64,
        )
        .expect("seed learned addresses");
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            99,
            permission_decision(["in", "addr"]),
        ))
        .expect("permissioned addr peer");

    // Act
    let first_actions = manager
        .handle_message(99, WireNetworkMessage::GetAddr, now_unix_seconds as i64)
        .expect("first getaddr should be served");
    let second_actions = manager
        .handle_message(99, WireNetworkMessage::GetAddr, now_unix_seconds as i64)
        .expect("second getaddr should be suppressed");
    let evidence = manager.address_boundary_evidence();

    // Assert
    let [PeerAction::Send(WireNetworkMessage::Addr(response))] = first_actions.as_slice() else {
        panic!("expected getaddr addr response");
    };
    assert_eq!(response.addresses.len(), PHASE92_GETADDR_RESPONSE_LIMIT);
    assert_eq!(response.addresses[0].address, local_address);
    assert!(second_actions.is_empty());
    let GetAddrResponseDecision::Served {
        label,
        reason,
        entries,
    } = &evidence.getaddr_responses_served[0]
    else {
        panic!("expected getaddr served evidence");
    };
    assert_eq!(label.as_str(), "getaddr_served");
    assert_eq!(*reason, AddressDecisionReason::PolicyAccepted);
    assert_eq!(entries.len(), PHASE92_GETADDR_RESPONSE_LIMIT);
    let GetAddrResponseDecision::Suppressed { label, reason } =
        &evidence.getaddr_requests_suppressed[0]
    else {
        panic!("expected getaddr suppressed evidence");
    };
    assert_eq!(label.as_str(), "getaddr_suppressed");
    assert_eq!(reason.as_str(), "already_served");
}

#[test]
fn getaddr_suppression_records_permission_and_outbound_reasons() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_inbound_peer(100)
        .expect("ordinary inbound peer");
    manager
        .add_outbound_peer(101, 1)
        .expect("outbound peer should be added");

    // Act
    let ordinary_actions = manager
        .handle_message(100, WireNetworkMessage::GetAddr, 2)
        .expect("ordinary inbound getaddr should be suppressed");
    let outbound_actions = manager
        .handle_message(101, WireNetworkMessage::GetAddr, 3)
        .expect("outbound getaddr should be suppressed");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert!(ordinary_actions.is_empty());
    assert!(outbound_actions.is_empty());
    assert_eq!(
        evidence
            .getaddr_requests_suppressed
            .iter()
            .map(|decision| match decision {
                GetAddrResponseDecision::Suppressed { reason, .. } => reason.as_str(),
                GetAddrResponseDecision::Served { .. } => "unexpected_served",
            })
            .collect::<Vec<_>>(),
        vec!["permission_policy_denied", "not_inbound"],
    );
}

#[test]
fn inbound_version_response_uses_sender_policy_and_suppressed_advertisements_keep_zero_sender() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.set_local_address_decisions(vec![local_advertisement_suppressed(
        public_ipv4_network_address(12, 0, 0, 1, 8333),
        AddressDecisionReason::PermissionPolicyDenied,
    )]);
    manager.add_inbound_peer(102).expect("inbound peer");

    // Act
    let version_actions = manager
        .handle_message(
            102,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 0,
                ..crate::VersionMessage::default()
            }),
            10,
        )
        .expect("version should process");
    let evidence = manager.address_boundary_evidence();

    // Assert
    assert_no_addr_actions(&version_actions);
    let [PeerAction::Send(WireNetworkMessage::Version(version)), ..] = version_actions.as_slice()
    else {
        panic!("expected local version response");
    };
    assert_eq!(version.sender, super::super::message::zero_address());
    assert_eq!(evidence.suppressed_advertisements.len(), 1);
}

#[test]
fn ordinary_peer_flows_do_not_send_unsolicited_addr_messages() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.set_local_address_decisions(vec![local_advertisement_candidate(
        public_ipv4_network_address(13, 0, 0, 1, 8333),
    )]);
    manager.add_inbound_peer(103).expect("inbound peer");

    // Act
    let version_actions = manager
        .handle_message(
            103,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 0,
                ..crate::VersionMessage::default()
            }),
            10,
        )
        .expect("version should process");
    let verack_actions = manager
        .handle_message(103, WireNetworkMessage::Verack, 11)
        .expect("verack should process");
    let ping_actions = manager
        .handle_message(103, WireNetworkMessage::Ping { nonce: 7 }, 12)
        .expect("ping should process");
    let inv_actions = manager
        .handle_message(
            103,
            WireNetworkMessage::Inv(InventoryList::new(Vec::new())),
            13,
        )
        .expect("inventory should process");
    let headers_actions = manager
        .handle_message(
            103,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: Vec::new(),
            }),
            14,
        )
        .expect("headers should process");

    // Assert
    for actions in [
        version_actions,
        verack_actions,
        ping_actions,
        inv_actions,
        headers_actions,
    ] {
        assert_no_addr_actions(&actions);
    }
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
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::GetAddr, 1)
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

fn address_announcement(time_unix_seconds: u64, address: NetworkAddress) -> AddressAnnouncement {
    AddressAnnouncement {
        time_unix_seconds: time_unix_seconds as u32,
        address,
    }
}

fn public_ipv4_network_address(a: u8, b: u8, c: u8, d: u8, port: u16) -> NetworkAddress {
    NetworkAddress {
        services: ServiceFlags::NETWORK.bits(),
        address_bytes: ipv4_mapped_address_bytes([a, b, c, d]),
        port,
    }
}

fn public_ipv6_network_address(raw_address: &str, port: u16) -> NetworkAddress {
    let address: core::net::Ipv6Addr = raw_address.parse().expect("test IPv6 should parse");
    NetworkAddress {
        services: ServiceFlags::NETWORK.bits(),
        address_bytes: address.octets(),
        port,
    }
}

fn local_advertisement_candidate(address: NetworkAddress) -> LocalAdvertisementDecision {
    LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseCandidate,
        reason: AddressDecisionReason::PolicyAccepted,
        source: AddressSourceKind::LocalListener,
        network_kind: AddressNetworkKind::Ipv4,
        routability: RoutabilityClass::PubliclyRoutable,
        services_bits: address.services,
        port: address.port,
        maybe_wire_address: Some(address),
    }
}

fn local_advertisement_suppressed(
    address: NetworkAddress,
    reason: AddressDecisionReason,
) -> LocalAdvertisementDecision {
    LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseSuppressed,
        reason,
        source: AddressSourceKind::LocalListener,
        network_kind: AddressNetworkKind::Ipv4,
        routability: RoutabilityClass::PubliclyRoutable,
        services_bits: address.services,
        port: address.port,
        maybe_wire_address: None,
    }
}

fn assert_no_addr_actions(actions: &[PeerAction]) {
    assert!(
        actions
            .iter()
            .all(|action| !matches!(action, PeerAction::Send(WireNetworkMessage::Addr(_)))),
    );
}

fn ipv4_mapped_address_bytes(octets: [u8; 4]) -> [u8; 16] {
    let mut bytes = [0_u8; 16];
    bytes[..12].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff]);
    bytes[12..].copy_from_slice(&octets);
    bytes
}
