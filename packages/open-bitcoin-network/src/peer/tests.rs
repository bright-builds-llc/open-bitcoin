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
use open_bitcoin_codec::{
    BlockTransactions, BlockTransactionsRequest, CompactBlockPayload, PrefilledTransaction,
    SendCompactMessage, ShortId,
};
use open_bitcoin_consensus::{block_hash, check_block_header, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, Hash32, MerkleRoot, MessageCommand, NetworkAddress,
    NetworkMagic, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
    TransactionOutput, Txid, Wtxid,
};

use crate::{
    BanDecision, BanReason, BanScope, BlockInFlightCleanupCause, BlockInFlightCleanupInput,
    BlockRelayActivationPolicy, BlockServingActivationConfig, BlockServingOutcomeLabel,
    BlockServingResourceGateDecision, BlockServingStatusDecision, BlockServingStatusLabel,
    CompactAnnouncementEligibility, CompactAnnouncementEligibilityReason, CompactBlockReceiveFacts,
    CompactDownloadCleanupCause, CompactRelayActivationConfig, CompactRelayCapability,
    CompactRelayPreference, ConnectionRole, DisconnectReason, HeaderStore, HeaderSyncPolicy,
    HeadersMessage, InboundAdmissionRejectionReason, InboundAdmissionSlotClass,
    InboundHandshakeState, InboundPeerRecord, InboundPermissionDecision, InventoryList,
    LocalPeerConfig, NetworkError, PHASE94_MAX_HEADER_LOCATOR_HASHES,
    PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER, PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS,
    PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER, PHASE101_GETDATA_TX_INTERVAL_SECONDS,
    PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER, ParsedPeerPermissionClass, PeerAction,
    PeerBanEntry, PeerConnectionClass, PeerId, PeerManager, PeerPermissionClassRegistry,
    PermissionEffectLabel, RelayActivationConfig, RelayDownloadPolicy, RequestPressureInput,
    ResourceGovernanceDecision, ResourceGovernancePolicy, ServiceFlags, TxDownloadAction,
    TxDownloadSuppressionReason, TxRelayId, WireNetworkMessage, classify_block_inflight_cleanup,
};
use open_bitcoin_primitives::{InventoryType, InventoryVector};

use crate::address::{
    AddressAnnouncement, AddressDecisionLabel, AddressDecisionReason, AddressList,
    AddressNetworkKind, AddressSourceKind, GetAddrResponseDecision, LocalAdvertisementDecision,
    PHASE92_GETADDR_RESPONSE_LIMIT, PHASE92_LEARNED_ADDR_BATCH_LIMIT, RoutabilityClass,
};

use super::DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER;
use super::compact_relay::{CompactAnnouncementAction, PeerCompactAnnouncementInput};

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

fn relay_download_policy(inbound_serving_enabled: bool) -> RelayDownloadPolicy {
    RelayDownloadPolicy {
        activation: RelayActivationConfig { enabled: true },
        inbound_serving_enabled,
    }
}

fn relay_download_manager(inbound_serving_enabled: bool) -> PeerManager {
    PeerManager::with_relay_download_policy(
        local_config(),
        DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER,
        relay_download_policy(inbound_serving_enabled),
    )
}

fn add_relay_outbound_peer(manager: &mut PeerManager, peer_id: PeerId) {
    let _ = manager
        .add_outbound_peer(peer_id, 0)
        .expect("outbound peer should be added");
}

fn compact_announcement_activation(enabled: bool) -> BlockRelayActivationPolicy {
    BlockRelayActivationPolicy {
        compact_relay: CompactRelayActivationConfig { enabled },
        ..BlockRelayActivationPolicy::default()
    }
}

fn compact_announcement_input(
    compact_relay_enabled: bool,
    peer_has_previous_header: bool,
    peer_has_current_header: bool,
    status: BlockServingStatusDecision,
    resource_gate: BlockServingResourceGateDecision,
) -> PeerCompactAnnouncementInput {
    PeerCompactAnnouncementInput {
        activation: compact_announcement_activation(compact_relay_enabled),
        peer_has_previous_header,
        peer_has_current_header,
        status,
        resource_gate,
    }
}

fn compact_available_status() -> BlockServingStatusDecision {
    BlockServingStatusDecision {
        label: BlockServingStatusLabel::Available,
        allow_storage_read: true,
        may_serve_block: true,
    }
}

fn compact_unavailable_status() -> BlockServingStatusDecision {
    BlockServingStatusDecision {
        label: BlockServingStatusLabel::Unavailable,
        allow_storage_read: false,
        may_serve_block: false,
    }
}

fn compact_available_resource_gate() -> BlockServingResourceGateDecision {
    BlockServingResourceGateDecision {
        label: BlockServingOutcomeLabel::BlockServingEligible,
        allow_storage_read: true,
        may_serve_block: true,
        maybe_resource_event: None,
        maybe_cleanup: None,
    }
}

fn compact_limited_resource_gate() -> BlockServingResourceGateDecision {
    BlockServingResourceGateDecision {
        label: BlockServingOutcomeLabel::BlockRequestCapReached,
        allow_storage_read: false,
        may_serve_block: false,
        maybe_resource_event: None,
        maybe_cleanup: None,
    }
}

fn process_high_bandwidth_sendcmpct(manager: &mut PeerManager, peer_id: PeerId) {
    manager
        .handle_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct high-bandwidth should process");
}

fn process_low_bandwidth_sendcmpct(manager: &mut PeerManager, peer_id: PeerId) {
    manager
        .handle_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct low-bandwidth should process");
}

fn complete_outbound_handshake(manager: &mut PeerManager, peer_id: PeerId, start_height: i32) {
    manager
        .handle_message(
            peer_id,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");
    manager
        .handle_message(peer_id, WireNetworkMessage::Verack, 12)
        .expect("verack");
}

fn add_relay_permissioned_inbound_peer(manager: &mut PeerManager, peer_id: PeerId) {
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            peer_id,
            permission_decision(["in", "relay"]),
        ))
        .expect("permissioned inbound peer should be added");
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

fn relay_permission_labels(decision: &InboundPermissionDecision) -> Vec<&'static str> {
    decision
        .relay_permission_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect()
}

#[test]
fn peer_manager_exposes_peer_policy_runtime_state_accessors() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let remote_ip = IpAddr::from([203, 0, 113, 240]);
    let entry = PeerBanEntry {
        scope: BanScope::Address(remote_ip),
        reason: BanReason::Manual,
        created_at_unix_seconds: 100,
        expires_at_unix_seconds: 300,
        source: "peer_manager_test",
    };

    // Act
    let decision = manager
        .peer_policy_runtime_state_mut()
        .record_ban(entry, 150);
    let reconnect = manager
        .peer_policy_runtime_state()
        .reconnect_suppression_input_for_ip(remote_ip, 150);

    // Assert
    assert!(matches!(decision, BanDecision::Active(_)));
    assert!(reconnect.banned);
    assert!(!reconnect.discouraged);
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
fn eviction_decision_selects_unprotected_inbound_candidate() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(34).expect("ordinary peer");
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            35,
            protected_permission_decision(),
        ))
        .expect("protected peer");

    // Act
    let decision = manager.eviction_decision();

    // Assert
    let crate::EvictionDecision::Select(candidate) = decision else {
        panic!("expected eviction candidate");
    };
    assert_eq!(candidate.peer_label, "peer-34");
    assert_eq!(candidate.reason.as_str(), "handshake_stalled");
}

#[test]
fn misbehavior_decision_respects_protected_inbound_peer() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            36,
            protected_permission_decision(),
        ))
        .expect("protected peer");

    // Act
    let decision = manager
        .misbehavior_decision(36, crate::MisbehaviorKind::MalformedMessage, 500, 100)
        .expect("misbehavior decision");

    // Assert
    assert_eq!(
        decision.response,
        crate::MisbehaviorResponse::ProtectedNoAction,
    );
    assert_eq!(decision.response.as_str(), "protected_no_action");
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
        super::inventory_state::resource_limit_disconnect_actions_from_decision(
            ResourceGovernanceDecision::Backpressure(event.clone()),
        )
        .expect("backpressure event should map to disconnect action");
    let misbehavior_actions =
        super::inventory_state::resource_limit_disconnect_actions_from_decision(
            ResourceGovernanceDecision::RecordMisbehavior(event),
        )
        .expect("misbehavior event should map to disconnect action");

    // Assert
    assert_resource_limit_disconnect(&backpressure_actions);
    assert_resource_limit_disconnect(&misbehavior_actions);
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
    let identities = manager.identities();

    // Assert
    assert_eq!(
        endpoint_keys,
        BTreeSet::from(["127.0.0.1:18442".to_string()])
    );
    assert_eq!(counters.current_inbound_peers, 1);
    assert_eq!(counters.current_reserved_inbound_peers, 1);
    assert_eq!(counters.current_outbound_peers, 1);
    assert_eq!(peer_ids, BTreeSet::from([40, 41, 42]));
    assert_eq!(identities, peer_ids);
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
        super::inventory_state::permission_effect_vectors(peer);
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
fn deferred_relay_commands_remain_absent_from_peer_message_surface() {
    // Arrange
    let deferred_commands = [
        "mempool",
        "getcfilters",
        "cfilter",
        "getcfheaders",
        "getcfcheckpt",
        "filterload",
        "filteradd",
        "filterclear",
    ];

    for command in deferred_commands {
        let command = MessageCommand::new(command).expect("test command should be valid");

        // Act
        let result = WireNetworkMessage::decode_payload(&command, &[]);

        // Assert
        assert!(matches!(
            result,
            Err(NetworkError::UnknownCommand(rejected)) if rejected == command.as_str()
        ));
    }
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
fn scoped_relay_permission_effects_remain_policy_only_for_transaction_paths() {
    // Arrange
    let permission_decision = permission_decision(["in", "relay", "forcerelay", "mempool"]);
    assert!(active_permission_labels(&permission_decision).is_empty());
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
        Vec::<&'static str>::new(),
    );
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let mut manager = relay_download_manager(true);
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
    let wtxid_getdata_actions = manager
        .handle_message(
            91,
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: wtxid.into(),
            }])),
            6,
        )
        .expect("witness transaction getdata");

    // Assert
    assert_transaction_relay_request(&txid_inventory_actions, 91, TxRelayId::Txid(txid));
    assert!(wtxidrelay_actions.is_empty());
    assert_transaction_relay_request(&wtxid_inventory_actions, 91, TxRelayId::Wtxid(wtxid));
    assert_eq!(
        tx_actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::ReceivedTxCleanup {
                peer_id: 91,
                txid,
                wtxid,
            }),
            PeerAction::ReceivedTransaction(transaction),
        ],
    );
    assert_eq!(
        getdata_actions,
        vec![PeerAction::ServeInventory(vec![InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: txid.into(),
        }])]
    );
    assert_eq!(
        wtxid_getdata_actions,
        vec![PeerAction::ServeInventory(vec![InventoryVector {
            inventory_type: InventoryType::WitnessTransaction,
            object_hash: wtxid.into(),
        }])]
    );
}

#[test]
fn peer_manager_transaction_relay_default_off_download_suppresses_without_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(226).expect("peer");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");

    // Act
    let actions = manager
        .handle_message(
            226,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            1,
        )
        .expect("default-off transaction inventory");

    // Assert
    assert_transaction_relay_suppression(
        &actions,
        226,
        TxRelayId::Txid(txid),
        TxDownloadSuppressionReason::RelayDisabled,
    );
    let snapshot = manager.transaction_request_snapshot(226);
    assert_eq!(snapshot.candidate_count, 0);
    assert_eq!(snapshot.in_flight_count, 0);
}

#[test]
fn peer_manager_transaction_relay_enabled_outbound_and_permissioned_inbound_schedule_downloads() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 227);
    add_relay_permissioned_inbound_peer(&mut manager, 228);
    let txid = txid_from_byte(106);
    let wtxid = open_bitcoin_primitives::Wtxid::from_byte_array([107; 32]);
    manager
        .handle_message(228, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");

    // Act
    let txid_actions = manager
        .handle_message(
            227,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            2,
        )
        .expect("outbound txid inventory");
    let wtxid_actions = manager
        .handle_message(
            228,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(wtxid))),
            3,
        )
        .expect("permissioned inbound wtxid inventory");

    // Assert
    assert_transaction_relay_request(&txid_actions, 227, TxRelayId::Txid(txid));
    assert_transaction_relay_request(&wtxid_actions, 228, TxRelayId::Wtxid(wtxid));
}

#[test]
fn peer_manager_transaction_relay_enabled_inbound_classes_require_scoped_relay_permission() {
    // Arrange
    let mut manager = relay_download_manager(true);
    manager
        .add_inbound_peer(229)
        .expect("ordinary inbound peer");
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            230,
            protected_permission_decision(),
        ))
        .expect("protected inbound peer");
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            231,
            permission_decision(["in", "download"]),
        ))
        .expect("permissioned inbound peer without relay scope");

    // Act
    let ordinary_actions = manager
        .handle_message(
            229,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(
                txid_from_byte(108),
            ))),
            1,
        )
        .expect("ordinary inbound inventory");
    let protected_actions = manager
        .handle_message(
            230,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(
                txid_from_byte(109),
            ))),
            2,
        )
        .expect("protected inbound inventory");
    let no_relay_actions = manager
        .handle_message(
            231,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(
                txid_from_byte(110),
            ))),
            3,
        )
        .expect("permissioned inbound without relay inventory");

    // Assert
    assert_transaction_relay_suppression(
        &ordinary_actions,
        229,
        TxRelayId::Txid(txid_from_byte(108)),
        TxDownloadSuppressionReason::PermissionRequired,
    );
    assert_transaction_relay_suppression(
        &protected_actions,
        230,
        TxRelayId::Txid(txid_from_byte(109)),
        TxDownloadSuppressionReason::ProtectedNotRelay,
    );
    assert_transaction_relay_suppression(
        &no_relay_actions,
        231,
        TxRelayId::Txid(txid_from_byte(110)),
        TxDownloadSuppressionReason::PermissionRequired,
    );
}

#[test]
fn peer_manager_transaction_relay_inbound_serving_disabled_blocks_permissioned_downloads() {
    // Arrange
    let mut manager = relay_download_manager(false);
    add_relay_permissioned_inbound_peer(&mut manager, 232);
    let txid = txid_from_byte(111);

    // Act
    let actions = manager
        .handle_message(
            232,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            1,
        )
        .expect("permissioned inbound inventory");

    // Assert
    assert_transaction_relay_suppression(
        &actions,
        232,
        TxRelayId::Txid(txid),
        TxDownloadSuppressionReason::InboundServingRequired,
    );
}

#[test]
fn peer_manager_transaction_relay_ineligible_first_announcement_does_not_block_eligible_second() {
    // Arrange
    let mut manager = relay_download_manager(true);
    manager
        .add_inbound_peer(233)
        .expect("ordinary inbound peer");
    add_relay_outbound_peer(&mut manager, 234);
    let relay_id = TxRelayId::Txid(txid_from_byte(112));

    // Act
    let first_actions = manager
        .handle_message(
            233,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            1,
        )
        .expect("ordinary inbound inventory");
    let second_actions = manager
        .handle_message(
            234,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            2,
        )
        .expect("outbound inventory");

    // Assert
    assert_transaction_relay_suppression(
        &first_actions,
        233,
        relay_id,
        TxDownloadSuppressionReason::PermissionRequired,
    );
    assert_transaction_relay_request(&second_actions, 234, relay_id);
    assert_eq!(manager.transaction_request_snapshot(233).candidate_count, 0);
    assert_eq!(manager.transaction_request_snapshot(233).in_flight_count, 0);
    assert_eq!(manager.transaction_request_snapshot(234).in_flight_count, 1);
}

#[test]
fn peer_manager_transaction_relay_txid_inv_emits_typed_request_action() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 201);
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");

    // Act
    let actions = manager
        .handle_message(
            201,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            1,
        )
        .expect("txid inventory");

    // Assert
    assert_transaction_relay_request(&actions, 201, TxRelayId::Txid(txid));
}

#[test]
fn peer_manager_transaction_relay_wtxid_inv_emits_typed_request_action() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 202);
    manager
        .handle_message(202, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");

    // Act
    let actions = manager
        .handle_message(
            202,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(wtxid))),
            2,
        )
        .expect("wtxid inventory");

    // Assert
    assert_transaction_relay_request(&actions, 202, TxRelayId::Wtxid(wtxid));
}

#[test]
fn peer_manager_transaction_relay_mismatch_emits_suppression_without_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(203).expect("txid-only peer");
    manager.add_inbound_peer(204).expect("wtxid peer");
    manager
        .handle_message(204, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");

    // Act
    let txid_to_wtxid_peer = manager
        .handle_message(
            204,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            2,
        )
        .expect("mismatched txid inventory");
    let wtxid_to_txid_peer = manager
        .handle_message(
            203,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(wtxid))),
            3,
        )
        .expect("mismatched wtxid inventory");

    // Assert
    assert_transaction_relay_identity_mismatch(&txid_to_wtxid_peer, 204);
    assert_transaction_relay_identity_mismatch(&wtxid_to_txid_peer, 203);
    assert_eq!(manager.transaction_request_snapshot(204).in_flight_count, 0);
    assert_eq!(manager.transaction_request_snapshot(203).in_flight_count, 0);
}

#[test]
fn peer_manager_transaction_relay_duplicate_inv_suppresses_second_getdata_but_keeps_fallback() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 205);
    add_relay_outbound_peer(&mut manager, 206);
    let transaction = open_bitcoin_primitives::Transaction::default();
    let relay_id = TxRelayId::Txid(transaction_txid(&transaction).expect("txid"));

    // Act
    let first_actions = manager
        .handle_message(
            205,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            1,
        )
        .expect("first inventory");
    let duplicate_actions = manager
        .handle_message(
            206,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            2,
        )
        .expect("duplicate inventory");
    let fallback_actions =
        manager.expire_transaction_requests(1 + PHASE101_GETDATA_TX_INTERVAL_SECONDS);

    // Assert
    assert_transaction_relay_request(&first_actions, 205, relay_id);
    assert_transaction_relay_duplicate(&duplicate_actions, 206, relay_id);
    assert_eq!(
        fallback_actions,
        vec![
            (
                205,
                PeerAction::TransactionRelay(TxDownloadAction::RequestExpired {
                    peer_id: 205,
                    relay_id,
                }),
            ),
            (
                206,
                PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                    peer_id: 206,
                    relay_id,
                }),
            ),
        ],
    );
}

#[test]
fn peer_manager_transaction_relay_already_have_and_recent_reject_suppress_requests() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(207).expect("already-have peer");
    manager.add_inbound_peer(208).expect("recent-reject peer");
    let local_transaction = open_bitcoin_primitives::Transaction::default();
    let local_txid = transaction_txid(&local_transaction).expect("txid");
    let rejected_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([88_u8; 32])));
    manager
        .note_local_transaction(&local_transaction)
        .expect("local transaction");
    manager.note_recent_reject(rejected_relay_id);

    // Act
    let already_have_actions = manager
        .handle_message(
            207,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(local_txid))),
            1,
        )
        .expect("already-have inventory");
    let recent_reject_actions = manager
        .handle_message(
            208,
            WireNetworkMessage::Inv(transaction_relay_inventory(rejected_relay_id)),
            2,
        )
        .expect("recent-reject inventory");

    // Assert
    assert_eq!(
        already_have_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressAlreadyHave {
                peer_id: 207,
                relay_id: TxRelayId::Txid(local_txid),
            },
        )],
    );
    assert_eq!(
        recent_reject_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressRecentReject {
                peer_id: 208,
                relay_id: rejected_relay_id,
            },
        )],
    );
}

#[test]
fn peer_manager_orphan_parent_request_uses_transaction_scheduler() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 219);
    let parent_txid = txid_from_byte(101);

    // Act
    let actions = manager
        .request_orphan_parent(219, parent_txid, 10)
        .expect("parent request");

    // Assert
    assert_transaction_relay_request(&actions, 219, TxRelayId::Txid(parent_txid));
    assert_eq!(manager.transaction_request_snapshot(219).in_flight_count, 1);
}

#[test]
fn peer_manager_transaction_relay_orphan_parent_request_uses_relay_download_eligibility() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(235).expect("peer");
    let parent_txid = txid_from_byte(113);

    // Act
    let actions = manager
        .request_orphan_parent(235, parent_txid, 10)
        .expect("parent request");

    // Assert
    assert_transaction_relay_suppression(
        &actions,
        235,
        TxRelayId::Txid(parent_txid),
        TxDownloadSuppressionReason::RelayDisabled,
    );
    assert_eq!(manager.transaction_request_snapshot(235).in_flight_count, 0);
    assert_eq!(manager.transaction_request_snapshot(235).candidate_count, 0);
}

#[test]
fn peer_manager_orphan_parent_request_respects_inflight_cap() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 220);

    // Act
    for index in 0..PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER {
        let parent_txid = txid_from_byte(index as u8);
        let actions = manager
            .request_orphan_parent(220, parent_txid, index as i64)
            .expect("parent request");
        assert_transaction_relay_request(&actions, 220, TxRelayId::Txid(parent_txid));
    }
    let capped_txid = txid_from_byte(250);
    let capped_actions = manager
        .request_orphan_parent(220, capped_txid, 500)
        .expect("capped parent request");

    // Assert
    assert_eq!(
        manager.transaction_request_snapshot(220).in_flight_count,
        PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER
    );
    assert_eq!(
        capped_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressRequestCap {
                peer_id: 220,
                relay_id: TxRelayId::Txid(capped_txid),
            },
        )],
    );
}

#[test]
fn peer_manager_orphan_parent_request_suppresses_already_have_recent_reject_and_mempool_known() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(221).expect("already-have peer");
    manager.add_inbound_peer(222).expect("recent-reject peer");
    manager.add_inbound_peer(223).expect("mempool-known peer");
    let local_transaction = open_bitcoin_primitives::Transaction::default();
    let local_txid = transaction_txid(&local_transaction).expect("txid");
    let rejected_txid = txid_from_byte(102);
    let mempool_txid = txid_from_byte(103);
    manager
        .note_local_transaction(&local_transaction)
        .expect("local transaction");
    manager.note_recent_reject(TxRelayId::Txid(rejected_txid));
    manager.note_mempool_known(TxRelayId::Txid(mempool_txid));

    // Act
    let already_have_actions = manager
        .request_orphan_parent(221, local_txid, 1)
        .expect("already-have parent request");
    let recent_reject_actions = manager
        .request_orphan_parent(222, rejected_txid, 2)
        .expect("recent-reject parent request");
    let mempool_known_actions = manager
        .request_orphan_parent(223, mempool_txid, 3)
        .expect("mempool-known parent request");

    // Assert
    assert_eq!(
        already_have_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressAlreadyHave {
                peer_id: 221,
                relay_id: TxRelayId::Txid(local_txid),
            },
        )],
    );
    assert_eq!(
        recent_reject_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressRecentReject {
                peer_id: 222,
                relay_id: TxRelayId::Txid(rejected_txid),
            },
        )],
    );
    assert_eq!(
        mempool_known_actions,
        vec![PeerAction::TransactionRelay(TxDownloadAction::Suppress {
            peer_id: 223,
            relay_id: TxRelayId::Txid(mempool_txid),
            reason: TxDownloadSuppressionReason::MempoolKnown,
        })],
    );
}

#[test]
fn peer_manager_orphan_parent_request_counts_toward_resource_governance() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_permissioned_inbound_peer(&mut manager, 224);
    let parent_txid = txid_from_byte(104);

    // Act
    let actions = manager
        .request_orphan_parent(224, parent_txid, 10)
        .expect("parent request");
    let snapshot = manager.transaction_request_snapshot(224);
    let candidates = manager.eviction_candidate_inputs();
    let [candidate] = candidates.as_slice() else {
        panic!("expected one eviction candidate");
    };
    let decision = ResourceGovernancePolicy::default().decide_request(RequestPressureInput {
        requested_txids_in_flight: PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER
            .saturating_add(snapshot.in_flight_count),
        ..RequestPressureInput::default()
    });

    // Assert
    assert_transaction_relay_request(&actions, 224, TxRelayId::Txid(parent_txid));
    assert_eq!(snapshot.in_flight_count, 1);
    assert_eq!(candidate.requested_inventory_count, 1);
    assert!(
        matches!(decision, ResourceGovernanceDecision::Disconnect(event) if event.label == "request_cap_reached"),
        "expected orphan parent request to contribute to resource governance cap"
    );
}

#[test]
fn peer_manager_orphan_parent_request_rejects_unknown_peer() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let parent_txid = txid_from_byte(105);

    // Act
    let error = manager
        .request_orphan_parent(225, parent_txid, 10)
        .expect_err("unknown peer should fail");

    // Assert
    assert_eq!(error, NetworkError::UnknownPeer(225));
}

#[test]
fn peer_manager_transaction_relay_notfound_timeout_and_disconnect_cleanup_fallback() {
    // Arrange
    let mut manager = relay_download_manager(true);
    for peer_id in 209..=214 {
        add_relay_outbound_peer(&mut manager, peer_id);
    }
    let notfound_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([91_u8; 32])));
    let timeout_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([92_u8; 32])));
    let disconnect_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([93_u8; 32])));
    seed_duplicate_announcements(&mut manager, 209, 210, notfound_relay_id, 1);
    seed_duplicate_announcements(&mut manager, 211, 212, timeout_relay_id, 10);
    seed_duplicate_announcements(&mut manager, 213, 214, disconnect_relay_id, 20);

    // Act
    let notfound_actions = manager
        .handle_message(
            209,
            WireNetworkMessage::NotFound(transaction_relay_inventory(notfound_relay_id)),
            30,
        )
        .expect("notfound");
    let timeout_actions =
        manager.expire_transaction_requests(10 + PHASE101_GETDATA_TX_INTERVAL_SECONDS);
    let disconnect_actions = manager
        .remove_peer_with_transaction_cleanup(213, 40)
        .expect("disconnect cleanup");

    // Assert
    assert_eq!(
        notfound_actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::NotFoundCleanup {
                peer_id: 209,
                relay_id: notfound_relay_id,
            }),
            PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                peer_id: 210,
                relay_id: notfound_relay_id,
            }),
        ],
    );
    assert_eq!(
        timeout_actions,
        vec![
            (
                211,
                PeerAction::TransactionRelay(TxDownloadAction::RequestExpired {
                    peer_id: 211,
                    relay_id: timeout_relay_id,
                }),
            ),
            (
                212,
                PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                    peer_id: 212,
                    relay_id: timeout_relay_id,
                }),
            ),
        ],
    );
    assert_eq!(
        disconnect_actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::PeerCleanup { peer_id: 213 }),
            PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                peer_id: 214,
                relay_id: disconnect_relay_id,
            }),
        ],
    );
    assert!(manager.peer_state(213).is_none());
}

#[test]
fn peer_manager_transaction_relay_received_transaction_cleanup_waits_for_admission() {
    // Arrange
    let mut manager = relay_download_manager(true);
    for peer_id in 215..=217 {
        add_relay_outbound_peer(&mut manager, peer_id);
    }
    manager
        .handle_message(217, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    manager
        .handle_message(
            215,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            2,
        )
        .expect("request inventory");

    // Act
    let received_actions = manager
        .handle_message(215, WireNetworkMessage::Tx(transaction.clone()), 3)
        .expect("received transaction");
    let txid_again = manager
        .handle_message(
            216,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            4,
        )
        .expect("txid inventory after receipt");
    let wtxid_again = manager
        .handle_message(
            217,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(wtxid))),
            5,
        )
        .expect("wtxid inventory after receipt");

    // Assert
    assert_eq!(
        received_actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::ReceivedTxCleanup {
                peer_id: 215,
                txid,
                wtxid,
            }),
            PeerAction::ReceivedTransaction(transaction),
        ],
    );
    assert_eq!(
        txid_again,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::RequestGetData {
                peer_id: 216,
                relay_id: TxRelayId::Txid(txid),
            },
        )],
    );
    assert_eq!(
        wtxid_again,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::RequestGetData {
                peer_id: 217,
                relay_id: TxRelayId::Wtxid(wtxid),
            },
        )],
    );
}

#[test]
fn peer_manager_transaction_relay_received_transaction_mismatch_does_not_satisfy_unrelated_request()
{
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 218);
    let requested_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([94_u8; 32])));
    manager
        .handle_message(
            218,
            WireNetworkMessage::Inv(transaction_relay_inventory(requested_relay_id)),
            1,
        )
        .expect("request inventory");
    let unrelated_transaction = open_bitcoin_primitives::Transaction::default();

    // Act
    let actions = manager
        .handle_message(218, WireNetworkMessage::Tx(unrelated_transaction), 2)
        .expect("unrelated transaction");

    // Assert
    assert_transaction_relay_identity_mismatch(&actions, 218);
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::ReceivedTransaction(_)))
    );
    assert_eq!(manager.transaction_request_snapshot(218).in_flight_count, 1);
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
fn phase112_bip152_wire_messages_are_peer_noops() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(112_003, 0).expect("peer");
    let block_hash = BlockHash::from_byte_array([112_u8; 32]);
    let messages = [
        WireNetworkMessage::SendCompact(SendCompactMessage {
            announce: true,
            version: 2,
        }),
        WireNetworkMessage::CompactBlock(CompactBlockPayload {
            header: header(BlockHash::from_byte_array([0_u8; 32]), 112),
            nonce: 1,
            short_ids: vec![ShortId::from_wire_bytes([1, 2, 3, 4, 5, 6])],
            prefilled_transactions: Vec::new(),
        }),
        WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
            block_hash,
            index_deltas: vec![0],
        }),
        WireNetworkMessage::BlockTxn(BlockTransactions {
            block_hash,
            transactions: Vec::new(),
        }),
    ];

    for message in messages {
        // Act
        let actions = manager
            .handle_message(112_003, message, 1)
            .expect("BIP152 message should be accepted");

        // Assert
        assert!(actions.is_empty());
    }
}

#[test]
fn phase113_sendcmpct_version2_high_bandwidth_updates_peer_compact_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_001, 0).expect("peer");

    // Act
    let actions = manager
        .handle_message(
            113_001,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct high-bandwidth should process");

    // Assert
    assert!(actions.is_empty());
    let compact = &manager.peer_state(113_001).expect("peer").compact_relay;
    assert_eq!(
        compact.capability,
        CompactRelayCapability::Supported { version: 2 }
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn phase113_sendcmpct_version2_low_bandwidth_updates_peer_compact_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_002, 0).expect("peer");

    // Act
    let actions = manager
        .handle_message(
            113_002,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct low-bandwidth should process");

    // Assert
    assert!(actions.is_empty());
    let compact = &manager.peer_state(113_002).expect("peer").compact_relay;
    assert_eq!(
        compact.capability,
        CompactRelayCapability::Supported { version: 2 }
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn phase113_sendcmpct_high_to_low_clears_high_bandwidth_preference() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_003, 0).expect("peer");

    // Act
    manager
        .handle_message(
            113_003,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct high-bandwidth should process");
    manager
        .handle_message(
            113_003,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 2,
            }),
            2,
        )
        .expect("sendcmpct low-bandwidth should process");

    // Assert
    let compact = &manager.peer_state(113_003).expect("peer").compact_relay;
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn phase113_sendcmpct_low_to_high_clears_low_bandwidth_preference() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_004, 0).expect("peer");

    // Act
    manager
        .handle_message(
            113_004,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct low-bandwidth should process");
    manager
        .handle_message(
            113_004,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            2,
        )
        .expect("sendcmpct high-bandwidth should process");

    // Assert
    let compact = &manager.peer_state(113_004).expect("peer").compact_relay;
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn phase113_sendcmpct_unsupported_version_records_evidence_without_disconnect() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_005, 0).expect("peer");

    // Act
    let actions = manager
        .handle_message(
            113_005,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 3,
            }),
            1,
        )
        .expect("unsupported sendcmpct should process without disconnecting");

    // Assert
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::Disconnect(_)))
    );
    let compact = &manager.peer_state(113_005).expect("peer").compact_relay;
    assert_eq!(
        compact.capability,
        CompactRelayCapability::Unsupported { version: 3 }
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Unknown
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Unknown
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
    assert_eq!(compact.maybe_unsupported_version, Some(3));
}

#[test]
fn phase113_unsupported_sendcmpct_does_not_clear_existing_version2_capability() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_006, 0).expect("peer");

    // Act
    manager
        .handle_message(
            113_006,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct high-bandwidth should process");
    let unsupported_actions = manager
        .handle_message(
            113_006,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 1,
            }),
            2,
        )
        .expect("unsupported sendcmpct should process");

    // Assert
    assert!(
        !unsupported_actions
            .iter()
            .any(|action| matches!(action, PeerAction::Disconnect(_)))
    );
    let compact = &manager.peer_state(113_006).expect("peer").compact_relay;
    assert_eq!(
        compact.capability,
        CompactRelayCapability::Supported { version: 2 }
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
    assert_eq!(compact.maybe_unsupported_version, Some(1));
}

#[test]
fn phase113_transaction_relay_messages_do_not_activate_compact_relay_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(113_007).expect("peer");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");

    // Act
    let wtxidrelay_actions = manager
        .handle_message(113_007, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay should process");
    let inventory_actions = manager
        .handle_message(
            113_007,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }])),
            2,
        )
        .expect("transaction inventory should process");

    // Assert
    assert!(wtxidrelay_actions.is_empty());
    assert!(!inventory_actions.iter().any(|action| {
        matches!(
            action,
            PeerAction::Send(WireNetworkMessage::CompactBlock(_))
        )
    }));
    let peer = manager.peer_state(113_007).expect("peer");
    assert!(peer.remote_wtxidrelay);
    assert_eq!(
        peer.compact_relay.capability,
        CompactRelayCapability::Unknown
    );
    assert_eq!(
        peer.compact_relay.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

fn phase115_seed_header_chain(manager: &mut PeerManager, headers: &[BlockHeader]) {
    let mut store = HeaderStore::default();
    for header in headers {
        let _ = store
            .insert_header(header.clone())
            .expect("header should insert");
    }
    manager.seed_header_store(store);
}

fn phase115_coinbase_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(vec![0x01, 0x02]).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(50_000_000_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn phase115_sample_transaction(previous_txid_byte: u8) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([previous_txid_byte; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::from_bytes(Vec::new()).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x01, 0x02]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51, 0xac]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn phase115_compact_payload_with_missing_short_id() -> (CompactBlockPayload, Transaction, Wtxid) {
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

    (payload, missing, wtxid)
}

fn phase115_prepare_compact_download_manager(
    peer_id: PeerId,
) -> (PeerManager, CompactBlockPayload, Transaction, BlockHash) {
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let (payload, missing, _) = phase115_compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);
    (manager, payload, missing, block_hash)
}

#[test]
fn phase115_handle_compact_block_download_with_activation_enabled() {
    let peer_id = 115_001;
    let (mut manager, payload, _, block_hash) = phase115_prepare_compact_download_manager(peer_id);

    let actions = manager
        .handle_compact_block_download(peer_id, payload, CompactBlockReceiveFacts::default(), 1_000)
        .expect("compact block should process");

    assert_eq!(actions.len(), 1);
    assert!(matches!(
        actions[0],
        PeerAction::Send(WireNetworkMessage::GetBlockTxn(_))
    ));
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    assert!(download_state.in_flight.contains_key(&block_hash));
    assert_eq!(
        manager.block_relay_activation_policy(),
        compact_announcement_activation(true)
    );
}

#[test]
fn phase115_expire_compact_download_timeouts_requests_full_blocks() {
    let peer_id = 115_002;
    let (mut manager, payload, _, block_hash) = phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_compact_block_download(peer_id, payload, CompactBlockReceiveFacts::default(), 100)
        .expect("compact block should start download");

    let actions = manager
        .expire_compact_download_timeouts(100 + crate::COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1);

    assert_eq!(actions.len(), 1);
    assert!(matches!(
        &actions[0],
        (
            returned_peer_id,
            PeerAction::Send(WireNetworkMessage::GetData(inventory))
        ) if *returned_peer_id == peer_id
            && inventory.inventory.len() == 1
            && inventory.inventory[0].inventory_type == InventoryType::Block
            && inventory.inventory[0].object_hash == block_hash.into()
    ));
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    assert!(download_state.in_flight.is_empty());
}

#[test]
fn phase115_handle_block_transactions_message_completes_download() {
    let peer_id = 115_003;
    let (mut manager, payload, missing, block_hash) =
        phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_compact_block_download(peer_id, payload, CompactBlockReceiveFacts::default(), 1_000)
        .expect("compact block should start download");

    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash,
                transactions: vec![missing],
            }),
            1_001,
        )
        .expect("blocktxn should process");

    assert_eq!(actions.len(), 1);
    assert!(matches!(actions[0], PeerAction::ReceivedBlock(_)));
    assert!(
        manager.compact_download_peer_state(peer_id).is_none()
            || manager
                .compact_download_peer_state(peer_id)
                .expect("download state")
                .in_flight
                .is_empty()
    );
}

#[test]
fn phase115_cleanup_all_compact_downloads() {
    let peer_a = 115_004;
    let peer_b = 115_005;
    let (mut manager, payload_a, _, _) = phase115_prepare_compact_download_manager(peer_a);
    let (payload_b, _, _) = {
        let (_, payload, missing, block_hash) = phase115_prepare_compact_download_manager(peer_b);
        (payload, missing, block_hash)
    };

    let _ = manager
        .handle_compact_block_download(
            peer_a,
            payload_a,
            CompactBlockReceiveFacts::default(),
            1_000,
        )
        .expect("peer a compact block");
    manager.add_outbound_peer(peer_b, 0).expect("peer b");
    complete_outbound_handshake(&mut manager, peer_b, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_b);
    let _ = manager
        .handle_compact_block_download(
            peer_b,
            payload_b,
            CompactBlockReceiveFacts::default(),
            1_000,
        )
        .expect("peer b compact block");

    assert_eq!(
        manager
            .cleanup_compact_download_for_peer(peer_a, CompactDownloadCleanupCause::Timeout)
            .expect("peer a cleanup"),
        1
    );
    manager.cleanup_all_compact_downloads(CompactDownloadCleanupCause::PeerDisconnect);
    assert!(
        manager
            .compact_download_peer_state(peer_a)
            .is_none_or(|state| state.in_flight.is_empty())
    );
    assert!(
        manager
            .compact_download_peer_state(peer_b)
            .is_none_or(|state| state.in_flight.is_empty())
    );
}

#[test]
fn phase115_on_compact_download_block_connected_clears_matching_in_flight() {
    let peer_id = 115_006;
    let (mut manager, payload, _, connected_hash) =
        phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_compact_block_download(peer_id, payload, CompactBlockReceiveFacts::default(), 1_000)
        .expect("compact block");
    manager.on_compact_download_block_connected(connected_hash);
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    assert!(!download_state.in_flight.contains_key(&connected_hash));
}

fn phase119_compact_payload_with_one_matched_and_one_missing()
-> (CompactBlockPayload, Transaction, Wtxid, Transaction, Wtxid) {
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let header = mined_header(block_hash(&tip), 3);
    let coinbase = phase115_coinbase_transaction();
    let matched = phase115_sample_transaction(0x31);
    let still_missing = phase115_sample_transaction(0x32);
    let matched_wtxid = transaction_wtxid(&matched).expect("matched wtxid");
    let missing_wtxid = transaction_wtxid(&still_missing).expect("missing wtxid");
    let selector = open_bitcoin_codec::short_id_selector_from_header_and_nonce(&header, 42);
    let matched_short_id =
        open_bitcoin_consensus::compact_short_id_for_wtxid(selector, &matched_wtxid);
    let missing_short_id =
        open_bitcoin_consensus::compact_short_id_for_wtxid(selector, &missing_wtxid);

    let payload = CompactBlockPayload {
        header,
        nonce: 42,
        short_ids: vec![matched_short_id, missing_short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };

    (
        payload,
        matched,
        matched_wtxid,
        still_missing,
        missing_wtxid,
    )
}

#[test]
fn peer_manager_on_mempool_transaction_removed_clears_matching_partial_slots() {
    // Arrange
    let peer_id = 119_001;
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let (payload, matched, matched_wtxid, _, _) =
        phase119_compact_payload_with_one_matched_and_one_missing();
    let block_hash = block_hash(&payload.header);
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);

    let facts = CompactBlockReceiveFacts {
        candidates: &[(&matched_wtxid, &matched)],
        extra: &[],
    };
    let _ = manager
        .handle_compact_block_download(peer_id, payload, facts, 1_000)
        .expect("compact block with one candidate match");

    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    let in_flight = download_state
        .in_flight
        .get(&block_hash)
        .expect("in-flight partial");
    assert!(in_flight.partial.is_transaction_available(1));
    assert!(!in_flight.partial.is_transaction_available(2));

    let unrelated_wtxid = Wtxid::from_byte_array([0xaa; 32]);

    // Act — unrelated wtxid leaves matched slot unchanged
    manager.on_mempool_transaction_removed(&unrelated_wtxid);
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    let in_flight = download_state
        .in_flight
        .get(&block_hash)
        .expect("in-flight partial");
    assert!(in_flight.partial.is_transaction_available(1));

    // Act — matching wtxid clears the volatile slot
    manager.on_mempool_transaction_removed(&matched_wtxid);

    // Assert
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    let in_flight = download_state
        .in_flight
        .get(&block_hash)
        .expect("in-flight partial");
    assert!(!in_flight.partial.is_transaction_available(1));
    assert_eq!(in_flight.partial.missing_transaction_indexes(), vec![1, 2]);
}

#[test]
fn phase115_cleanup_compact_download_for_peer_without_state_is_noop() {
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(115_007, 0).expect("peer");

    assert_eq!(
        manager
            .cleanup_compact_download_for_peer(115_007, CompactDownloadCleanupCause::Timeout)
            .expect("cleanup should succeed"),
        0
    );
}

#[test]
fn phase115_block_transactions_without_download_state_is_ignored() {
    let peer_id = 115_008;
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 0);

    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: BlockHash::from_byte_array([0x88; 32]),
                transactions: Vec::new(),
            }),
            1,
        )
        .expect("blocktxn without download state");

    assert!(actions.is_empty());
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
        .handle_compact_block_download(peer_id, payload, CompactBlockReceiveFacts::default(), 1_000)
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
        .handle_compact_block_download(peer_id, payload, CompactBlockReceiveFacts::default(), 1_000)
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
        .handle_compact_block_download(peer_id, payload, CompactBlockReceiveFacts::default(), 1_000)
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

#[test]
fn phase113_compact_announcement_all_gates_allow_compact_block() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_021, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_021);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_021, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_announced");
    assert_eq!(
        manager
            .peer_state(113_021)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Eligible
    );
}

#[test]
fn phase113_compact_announcement_disabled_local_activation_uses_inventory_fallback() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_022, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_022);
    let input = compact_announcement_input(
        false,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_022, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceInventory
    );
    assert_eq!(decision.reason.as_str(), "compact_relay_disabled");
    assert_eq!(
        manager
            .peer_state(113_022)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::LocalActivationDisabled,
        }
    );
}

#[test]
fn phase113_compact_announcement_missing_previous_header_uses_headers_fallback() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_023, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_023);
    let input = compact_announcement_input(
        true,
        false,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_023, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::AnnounceHeaders);
    assert_eq!(
        decision.reason.as_str(),
        "compact_header_continuity_missing"
    );
    assert_eq!(
        manager
            .peer_state(113_023)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::HeaderContinuityMissing,
        }
    );
}

#[test]
fn phase113_compact_announcement_unavailable_block_suppresses() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_024, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_024);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_unavailable_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_024, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::Suppress);
    assert_eq!(decision.reason.as_str(), "compact_block_unavailable");
    assert_eq!(
        manager
            .peer_state(113_024)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::BlockUnavailable,
        }
    );
}

#[test]
fn phase113_compact_announcement_resource_limit_suppresses() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_025, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_025);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_limited_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_025, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::Suppress);
    assert_eq!(decision.reason.as_str(), "compact_resource_limited");
    assert_eq!(
        manager
            .peer_state(113_025)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::ResourceLimited,
        }
    );
}

#[test]
fn phase113_compact_announcement_refreshes_eligibility_across_high_low_high_toggles() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_026, 0).expect("peer");
    let input = || {
        compact_announcement_input(
            true,
            true,
            false,
            compact_available_status(),
            compact_available_resource_gate(),
        )
    };

    // Act
    process_high_bandwidth_sendcmpct(&mut manager, 113_026);
    let high_decision = manager
        .decide_compact_announcement_for_peer(113_026, input())
        .expect("high bandwidth decision");
    process_low_bandwidth_sendcmpct(&mut manager, 113_026);
    let low_decision = manager
        .decide_compact_announcement_for_peer(113_026, input())
        .expect("low bandwidth decision");
    process_high_bandwidth_sendcmpct(&mut manager, 113_026);
    let restored_high_decision = manager
        .decide_compact_announcement_for_peer(113_026, input())
        .expect("restored high bandwidth decision");

    // Assert
    assert_eq!(
        high_decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        low_decision.reason.as_str(),
        "compact_high_bandwidth_not_requested"
    );
    assert_eq!(
        low_decision.eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::HighBandwidthNotRequested,
        }
    );
    assert_eq!(
        restored_high_decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        manager
            .peer_state(113_026)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Eligible
    );
}

#[test]
fn phase113_compact_announcement_preserves_supported_preference_after_unsupported_sendcmpct() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_027, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_027);
    manager
        .handle_message(
            113_027,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 1,
            }),
            2,
        )
        .expect("unsupported sendcmpct should process");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_027, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_announced");
    let compact = &manager.peer_state(113_027).expect("peer").compact_relay;
    assert_eq!(compact.maybe_unsupported_version, Some(1));
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
}

#[test]
fn phase113_low_bandwidth_compact_peer_uses_headers_fallback_for_direct_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_031, 0).expect("peer");
    manager
        .handle_message(113_031, WireNetworkMessage::SendHeaders, 1)
        .expect("sendheaders should process");
    process_low_bandwidth_sendcmpct(&mut manager, 113_031);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_031, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::AnnounceHeaders);
    assert_eq!(
        decision.reason.as_str(),
        "compact_high_bandwidth_not_requested"
    );
    let compact = &manager.peer_state(113_031).expect("peer").compact_relay;
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
}

#[test]
fn phase113_low_bandwidth_compact_peer_uses_inventory_fallback_without_sendheaders() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_032, 0).expect("peer");
    process_low_bandwidth_sendcmpct(&mut manager, 113_032);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_032, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceInventory
    );
    assert_eq!(
        decision.reason.as_str(),
        "compact_high_bandwidth_not_requested"
    );
    let compact = &manager.peer_state(113_032).expect("peer").compact_relay;
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
}

#[test]
fn phase113_high_to_low_toggle_never_announces_compact() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_033, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_033);
    process_low_bandwidth_sendcmpct(&mut manager, 113_033);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_033, input)
        .expect("compact announcement decision");

    // Assert
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        decision.reason.as_str(),
        "compact_high_bandwidth_not_requested"
    );
    let compact = &manager.peer_state(113_033).expect("peer").compact_relay;
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
}

#[test]
fn phase113_low_to_high_toggle_all_gates_allow_compact_block() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_034, 0).expect("peer");
    process_low_bandwidth_sendcmpct(&mut manager, 113_034);
    process_high_bandwidth_sendcmpct(&mut manager, 113_034);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_034, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_announced");
    let compact = &manager.peer_state(113_034).expect("peer").compact_relay;
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
}

#[test]
fn phase113_high_low_high_toggle_refreshes_recorded_eligibility() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_035, 0).expect("peer");
    let input = || {
        compact_announcement_input(
            true,
            true,
            false,
            compact_available_status(),
            compact_available_resource_gate(),
        )
    };

    // Act
    process_high_bandwidth_sendcmpct(&mut manager, 113_035);
    manager
        .decide_compact_announcement_for_peer(113_035, input())
        .expect("high-bandwidth decision");
    let high_eligibility = manager
        .peer_state(113_035)
        .expect("peer")
        .compact_relay
        .announcement_eligibility;
    process_low_bandwidth_sendcmpct(&mut manager, 113_035);
    manager
        .decide_compact_announcement_for_peer(113_035, input())
        .expect("low-bandwidth decision");
    let low_eligibility = manager
        .peer_state(113_035)
        .expect("peer")
        .compact_relay
        .announcement_eligibility;
    process_high_bandwidth_sendcmpct(&mut manager, 113_035);
    manager
        .decide_compact_announcement_for_peer(113_035, input())
        .expect("restored high-bandwidth decision");
    let restored_eligibility = manager
        .peer_state(113_035)
        .expect("peer")
        .compact_relay
        .announcement_eligibility;

    // Assert
    assert_eq!(high_eligibility, CompactAnnouncementEligibility::Eligible);
    assert_eq!(
        low_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::HighBandwidthNotRequested,
        }
    );
    assert_eq!(
        restored_eligibility,
        CompactAnnouncementEligibility::Eligible
    );
}

#[test]
fn phase113_unsupported_compact_version_without_supported_preference_uses_inventory_fallback_without_disconnect()
 {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_036, 0).expect("peer");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let actions = manager
        .handle_message(
            113_036,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 3,
            }),
            1,
        )
        .expect("unsupported sendcmpct should process");
    let decision = manager
        .decide_compact_announcement_for_peer(113_036, input)
        .expect("compact announcement decision");

    // Assert
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::Disconnect(_)))
    );
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceInventory
    );
    assert_eq!(decision.reason.as_str(), "compact_unsupported_version");
}

#[test]
fn phase113_unsupported_compact_version_after_supported_high_bandwidth_still_announces_compact() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_037, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_037);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let actions = manager
        .handle_message(
            113_037,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 1,
            }),
            2,
        )
        .expect("unsupported sendcmpct should process");
    let decision = manager
        .decide_compact_announcement_for_peer(113_037, input)
        .expect("compact announcement decision");

    // Assert
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::Disconnect(_)))
    );
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_announced");
    let compact = &manager.peer_state(113_037).expect("peer").compact_relay;
    assert_eq!(compact.maybe_unsupported_version, Some(1));
}

#[test]
fn phase113_peer_already_has_current_header_uses_headers_fallback() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_038, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_038);
    let input = compact_announcement_input(
        true,
        true,
        true,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_038, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::AnnounceHeaders);
    assert_eq!(decision.reason.as_str(), "compact_peer_already_has_header");
}

#[test]
fn phase113_missing_header_or_unavailable_block_never_announces_compact() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_039, 0).expect("peer");
    manager.add_outbound_peer(113_040, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_039);
    process_high_bandwidth_sendcmpct(&mut manager, 113_040);
    let missing_header_input = compact_announcement_input(
        true,
        false,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );
    let unavailable_block_input = compact_announcement_input(
        true,
        true,
        false,
        compact_unavailable_status(),
        compact_available_resource_gate(),
    );

    // Act
    let missing_header_decision = manager
        .decide_compact_announcement_for_peer(113_039, missing_header_input)
        .expect("missing-header compact announcement decision");
    let unavailable_block_decision = manager
        .decide_compact_announcement_for_peer(113_040, unavailable_block_input)
        .expect("unavailable-block compact announcement decision");

    // Assert
    assert_ne!(
        missing_header_decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        missing_header_decision.reason.as_str(),
        "compact_header_continuity_missing"
    );
    assert_ne!(
        unavailable_block_decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        unavailable_block_decision.action,
        CompactAnnouncementAction::Suppress
    );
    assert_eq!(
        unavailable_block_decision.reason.as_str(),
        "compact_block_unavailable"
    );
}

#[test]
fn phase113_wtxidrelay_does_not_activate_compact_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_041, 0).expect("peer");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let actions = manager
        .handle_message(113_041, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay should process");
    let decision = manager
        .decide_compact_announcement_for_peer(113_041, input)
        .expect("compact announcement decision");

    // Assert
    assert!(actions.is_empty());
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_peer_not_negotiated");
    assert_eq!(
        manager
            .peer_state(113_041)
            .expect("peer")
            .compact_relay
            .capability,
        CompactRelayCapability::Unknown
    );
}

#[test]
fn phase113_block_serving_enabled_without_compact_relay_does_not_announce_compact() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_042, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_042);
    let input = PeerCompactAnnouncementInput {
        activation: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig::default(),
        },
        peer_has_previous_header: true,
        peer_has_current_header: false,
        status: compact_available_status(),
        resource_gate: compact_available_resource_gate(),
    };

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_042, input)
        .expect("compact announcement decision");

    // Assert
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_relay_disabled");
}

#[test]
fn phase113_download_permission_does_not_grant_compact_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            113_043,
            permission_decision(["in", "download"]),
        ))
        .expect("download-permission inbound peer");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_043, input)
        .expect("compact announcement decision");

    // Assert
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_peer_not_negotiated");
}

#[test]
fn phase113_protected_inbound_permission_does_not_grant_compact_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            113_044,
            protected_permission_decision(),
        ))
        .expect("protected inbound peer");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_044, input)
        .expect("compact announcement decision");

    // Assert
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_peer_not_negotiated");
}

#[test]
fn phase113_default_activation_policy_suppresses_compact_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_045, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_045);
    let input = PeerCompactAnnouncementInput {
        activation: BlockRelayActivationPolicy::default(),
        peer_has_previous_header: true,
        peer_has_current_header: false,
        status: compact_available_status(),
        resource_gate: compact_available_resource_gate(),
    };

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_045, input)
        .expect("compact announcement decision");

    // Assert
    assert!(!BlockRelayActivationPolicy::default().compact_relay.enabled);
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_relay_disabled");
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
    assert_eq!(evidence.learned_address_rejection_count, 0);
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
    assert_eq!(evidence.learned_address_rejection_count, 7);
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
    assert_eq!(
        evidence.learned_address_rejection_count,
        PHASE92_LEARNED_ADDR_BATCH_LIMIT + 1,
    );
    assert!(evidence.learned_address_rejections.is_empty());
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

#[test]
fn phase110_block_notfound_releases_requested_block_without_clearing_tx_inflight() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 112);
    complete_outbound_handshake(&mut manager, 112, 1);
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    manager
        .handle_message(
            112,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            13,
        )
        .expect("transaction inventory");
    let block_hash = BlockHash::from(hash_from_index(112_000));
    manager
        .request_missing_blocks(112, &[block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(
            112,
            WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::WitnessBlock,
                object_hash: block_hash.into(),
            }])),
            14,
        )
        .expect("notfound");
    let retry = manager
        .request_missing_blocks(112, &[block_hash])
        .expect("retry request");

    // Assert
    assert!(actions.is_empty());
    assert_eq!(manager.transaction_request_snapshot(112).in_flight_count, 1);
    assert!(matches!(retry, Some(WireNetworkMessage::GetData(_))));
}

#[test]
fn phase110_block_response_clears_requested_block_before_received_action() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 113, 1);
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([113_u8; 32]), 1),
        transactions: Vec::new(),
    };
    let block_hash = open_bitcoin_consensus::block_hash(&block.header);
    manager
        .request_missing_blocks(113, &[block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(113, WireNetworkMessage::Block(block), 14)
        .expect("block");

    // Assert
    assert!(matches!(actions.as_slice(), [PeerAction::ReceivedBlock(_)]));
    assert!(
        manager
            .peer_requested_blocks(113)
            .expect("requested blocks")
            .is_empty()
    );
}

#[test]
fn phase110_block_peer_removal_drops_requested_blocks_and_preserves_tx_cleanup() {
    // Arrange
    let mut manager = relay_download_manager(true);
    for peer_id in 114..=115 {
        add_relay_outbound_peer(&mut manager, peer_id);
        complete_outbound_handshake(&mut manager, peer_id, 1);
    }
    let block_hash = BlockHash::from(hash_from_index(114_000));
    manager
        .request_missing_blocks(114, &[block_hash])
        .expect("block request")
        .expect("getdata");
    let txid = TxRelayId::Txid(txid_from_byte(114));
    seed_duplicate_announcements(&mut manager, 114, 115, txid, 20);

    // Act
    let actions = manager
        .remove_peer_with_transaction_cleanup(114, 30)
        .expect("peer cleanup");

    // Assert
    assert_eq!(
        actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::PeerCleanup { peer_id: 114 }),
            PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                peer_id: 115,
                relay_id: txid,
            }),
        ]
    );
    assert!(manager.peer_state(114).is_none());
    assert!(manager.peer_requested_blocks(114).is_err());
}

#[test]
fn phase111_notfound_releases_block_and_witness_block_requested_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(111_104, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 111_104, 2);
    let block_hash = BlockHash::from(hash_from_index(111_104));
    let witness_block_hash = BlockHash::from(hash_from_index(111_105));
    manager
        .request_missing_blocks(111_104, &[block_hash, witness_block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(
            111_104,
            WireNetworkMessage::NotFound(InventoryList::new(vec![
                InventoryVector {
                    inventory_type: InventoryType::Block,
                    object_hash: block_hash.into(),
                },
                InventoryVector {
                    inventory_type: InventoryType::WitnessBlock,
                    object_hash: witness_block_hash.into(),
                },
            ])),
            14,
        )
        .expect("notfound");

    // Assert
    assert!(actions.is_empty());
    assert!(
        manager
            .peer_requested_blocks(111_104)
            .expect("requested blocks")
            .is_empty()
    );
}

#[test]
fn phase111_received_block_releases_requested_block_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(111_105, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 111_105, 1);
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([111_u8; 32]), 105),
        transactions: Vec::new(),
    };
    let block_hash = open_bitcoin_consensus::block_hash(&block.header);
    manager
        .request_missing_blocks(111_105, &[block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(111_105, WireNetworkMessage::Block(block), 14)
        .expect("block");

    // Assert
    assert!(matches!(actions.as_slice(), [PeerAction::ReceivedBlock(_)]));
    assert!(
        manager
            .peer_requested_blocks(111_105)
            .expect("requested blocks")
            .is_empty()
    );
}

#[test]
fn phase111_peer_removal_drops_block_request_state_without_compact_state() {
    // Arrange
    let mut manager = relay_download_manager(true);
    for peer_id in 111_106..=111_107 {
        add_relay_outbound_peer(&mut manager, peer_id);
        complete_outbound_handshake(&mut manager, peer_id, 1);
    }
    let block_hash = BlockHash::from(hash_from_index(111_106));
    let txid = TxRelayId::Txid(txid_from_byte(106));
    manager
        .request_missing_blocks(111_106, &[block_hash])
        .expect("block request")
        .expect("getdata");
    manager
        .handle_message(
            111_106,
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::CompactBlock,
                object_hash: hash_from_index(111_107),
            }])),
            20,
        )
        .expect("compact getdata");
    seed_duplicate_announcements(&mut manager, 111_106, 111_107, txid, 20);

    // Act
    let actions = manager
        .remove_peer_with_transaction_cleanup(111_106, 30)
        .expect("peer cleanup");

    // Assert
    assert_eq!(
        actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::PeerCleanup { peer_id: 111_106 }),
            PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                peer_id: 111_107,
                relay_id: txid,
            }),
        ]
    );
    assert!(manager.peer_state(111_106).is_none());
    assert!(manager.peer_requested_blocks(111_106).is_err());
}

#[test]
fn phase111_compact_notfound_does_not_create_or_release_block_inflight_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(111_108, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 111_108, 1);
    let block_hash = BlockHash::from(hash_from_index(111_108));
    manager
        .request_missing_blocks(111_108, &[block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(
            111_108,
            WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::CompactBlock,
                object_hash: block_hash.into(),
            }])),
            14,
        )
        .expect("compact notfound");

    // Assert
    assert!(actions.is_empty());
    assert_eq!(
        manager
            .peer_requested_blocks(111_108)
            .expect("requested blocks"),
        vec![block_hash]
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

fn announce_with_action_coinbase_block() -> Block {
    Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 4),
        transactions: vec![phase115_coinbase_transaction()],
    }
}

#[test]
fn announce_block_with_action_emits_compact_block_for_valid_coinbase_block() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(50, 0).expect("peer");
    let block = announce_with_action_coinbase_block();
    let compact_nonce = 0x1122_3344_5566_7788_u64;

    // Act
    let message = manager
        .announce_block_with_action(
            50,
            &block,
            CompactAnnouncementAction::AnnounceCompactBlock,
            compact_nonce,
        )
        .expect("announce")
        .expect("message");

    // Assert
    let WireNetworkMessage::CompactBlock(payload) = message else {
        panic!("expected CompactBlock, got {message:?}");
    };
    assert_eq!(payload.header, block.header);
    assert_eq!(payload.nonce, compact_nonce);
    assert_eq!(payload.prefilled_transactions.len(), 1);
    assert_eq!(payload.prefilled_transactions[0].index_delta, 0);
    assert!(payload.short_ids.is_empty());
}

#[test]
fn announce_block_with_action_emits_headers_when_action_is_headers() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(51, 0).expect("peer");
    // Peer prefers inv by default; action must still force Headers.
    assert!(
        !manager
            .peer_state(51)
            .expect("state")
            .remote_prefers_headers
    );
    let block = announce_with_action_coinbase_block();

    // Act
    let message = manager
        .announce_block_with_action(51, &block, CompactAnnouncementAction::AnnounceHeaders, 0)
        .expect("announce")
        .expect("message");

    // Assert
    assert!(matches!(
        message,
        WireNetworkMessage::Headers(HeadersMessage { headers }) if headers.len() == 1
            && headers[0] == block.header
    ));
}

#[test]
fn announce_block_with_action_emits_inventory_when_action_is_inventory() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(52, 0).expect("peer");
    manager
        .handle_message(52, WireNetworkMessage::SendHeaders, 1)
        .expect("sendheaders");
    assert!(
        manager
            .peer_state(52)
            .expect("state")
            .remote_prefers_headers
    );
    let block = announce_with_action_coinbase_block();
    let expected_hash = block_hash(&block.header);

    // Act
    let message = manager
        .announce_block_with_action(52, &block, CompactAnnouncementAction::AnnounceInventory, 0)
        .expect("announce")
        .expect("message");

    // Assert
    assert!(matches!(
        message,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory.len() == 1
            && inventory[0].inventory_type == InventoryType::Block
            && inventory[0].object_hash == expected_hash.into()
    ));
}

#[test]
fn announce_block_with_action_suppress_returns_none() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(53, 0).expect("peer");
    let block = announce_with_action_coinbase_block();

    // Act
    let maybe_message = manager
        .announce_block_with_action(53, &block, CompactAnnouncementAction::Suppress, 0)
        .expect("announce");

    // Assert
    assert!(maybe_message.is_none());
}

#[test]
fn announce_block_with_action_unknown_peer_returns_error() {
    // Arrange
    let manager = PeerManager::new(local_config());
    let block = announce_with_action_coinbase_block();

    // Act
    let error = manager
        .announce_block_with_action(99, &block, CompactAnnouncementAction::AnnounceHeaders, 0)
        .expect_err("unknown peer");

    // Assert
    assert_eq!(error, NetworkError::UnknownPeer(99));
}

#[test]
fn announce_block_with_action_construction_failure_falls_back_to_inv() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(54, 0).expect("peer");
    assert!(
        !manager
            .peer_state(54)
            .expect("state")
            .remote_prefers_headers
    );
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 4),
        transactions: Vec::new(),
    };

    // Act
    let message = manager
        .announce_block_with_action(
            54,
            &block,
            CompactAnnouncementAction::AnnounceCompactBlock,
            7,
        )
        .expect("announce")
        .expect("fallback message");

    // Assert
    assert!(matches!(
        message,
        WireNetworkMessage::Inv(InventoryList { ref inventory })
        if inventory.len() == 1 && inventory[0].inventory_type == InventoryType::Block
    ));
    assert!(!matches!(message, WireNetworkMessage::CompactBlock(_)));
}

#[test]
fn announce_block_with_action_construction_failure_falls_back_to_headers() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(55, 0).expect("peer");
    manager
        .handle_message(55, WireNetworkMessage::SendHeaders, 1)
        .expect("sendheaders");
    assert!(
        manager
            .peer_state(55)
            .expect("state")
            .remote_prefers_headers
    );
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 4),
        transactions: Vec::new(),
    };

    // Act
    let message = manager
        .announce_block_with_action(
            55,
            &block,
            CompactAnnouncementAction::AnnounceCompactBlock,
            7,
        )
        .expect("announce")
        .expect("fallback message");

    // Assert
    assert!(matches!(
        message,
        WireNetworkMessage::Headers(HeadersMessage { ref headers }) if headers.len() == 1
    ));
    assert!(!matches!(message, WireNetworkMessage::CompactBlock(_)));
}

#[test]
fn inventory_requests_and_notfound_paths_cover_tx_and_block_modes() {
    let mut manager = relay_download_manager(true);
    add_relay_permissioned_inbound_peer(&mut manager, 6);
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
    let txid_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([2_u8; 32])));
    let txid_actions = manager
        .handle_message(6, WireNetworkMessage::Inv(txid_inv), 1)
        .expect("txid inventory");
    assert_transaction_relay_request(&txid_actions, 6, txid_relay_id);

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
    let wtxid_relay_id = TxRelayId::Wtxid(open_bitcoin_primitives::Wtxid::from(
        Hash32::from_byte_array([3_u8; 32]),
    ));
    let wtxid_actions = manager
        .handle_message(6, WireNetworkMessage::Inv(wtxid_inv), 2)
        .expect("wtxid inventory");
    assert_transaction_relay_request(&wtxid_actions, 6, wtxid_relay_id);
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
    let snapshot = manager.transaction_request_snapshot(6);
    assert_eq!(snapshot.in_flight_count, 0);
    assert_eq!(snapshot.candidate_count, 0);
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
    let snapshot = manager.transaction_request_snapshot(8);
    assert_eq!(snapshot.in_flight_count, 0);
    assert_eq!(snapshot.candidate_count, 0);
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
        [
            PeerAction::TransactionRelay(TxDownloadAction::ReceivedTxCleanup { .. }),
            PeerAction::ReceivedTransaction(_),
        ]
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
    let _ = txid;
    let _ = wtxid;
    let snapshot = manager.transaction_request_snapshot(7);
    assert_eq!(snapshot.in_flight_count, 0);
    assert_eq!(snapshot.candidate_count, 0);
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

fn assert_resource_limit_disconnect(actions: &[PeerAction]) {
    let [PeerAction::ResourceGovernanceDisconnect(event)] = actions else {
        panic!("expected resource-governance disconnect action, got {actions:?}");
    };
    assert_eq!(event.label, "request_cap_reached");
    assert_eq!(event.next_action, "request_cap_reached");
}

fn assert_transaction_relay_request(
    actions: &[PeerAction],
    expected_peer_id: PeerId,
    expected_relay_id: TxRelayId,
) {
    let [PeerAction::TransactionRelay(TxDownloadAction::RequestGetData { peer_id, relay_id })] =
        actions
    else {
        panic!("expected transaction relay request action, got {actions:?}");
    };
    assert_eq!((*peer_id, *relay_id), (expected_peer_id, expected_relay_id));
}

fn assert_transaction_relay_identity_mismatch(actions: &[PeerAction], expected_peer_id: PeerId) {
    let [
        PeerAction::TransactionRelay(TxDownloadAction::SuppressIdentityMismatch {
            peer_id, ..
        }),
    ] = actions
    else {
        panic!("expected transaction relay identity mismatch, got {actions:?}");
    };
    assert_eq!(*peer_id, expected_peer_id);
}

fn assert_transaction_relay_duplicate(
    actions: &[PeerAction],
    expected_peer_id: PeerId,
    expected_relay_id: TxRelayId,
) {
    let [PeerAction::TransactionRelay(TxDownloadAction::SuppressDuplicate { peer_id, relay_id })] =
        actions
    else {
        panic!("expected transaction relay duplicate suppression, got {actions:?}");
    };
    assert_eq!((*peer_id, *relay_id), (expected_peer_id, expected_relay_id));
}

fn assert_transaction_relay_suppression(
    actions: &[PeerAction],
    expected_peer_id: PeerId,
    expected_relay_id: TxRelayId,
    expected_reason: TxDownloadSuppressionReason,
) {
    let [
        PeerAction::TransactionRelay(TxDownloadAction::Suppress {
            peer_id,
            relay_id,
            reason,
        }),
    ] = actions
    else {
        panic!("expected transaction relay suppression, got {actions:?}");
    };
    assert_eq!(
        (*peer_id, *relay_id, *reason),
        (expected_peer_id, expected_relay_id, expected_reason),
    );
}

fn seed_duplicate_announcements(
    manager: &mut PeerManager,
    first_peer_id: PeerId,
    fallback_peer_id: PeerId,
    relay_id: TxRelayId,
    timestamp: i64,
) {
    manager
        .handle_message(
            first_peer_id,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            timestamp,
        )
        .expect("first transaction announcement");
    manager
        .handle_message(
            fallback_peer_id,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            timestamp + 1,
        )
        .expect("fallback transaction announcement");
}

fn transaction_relay_inventory(relay_id: TxRelayId) -> InventoryList {
    InventoryList::new(vec![relay_id.to_inventory_vector()])
}

fn txid_from_byte(byte: u8) -> Txid {
    Txid::from(Hash32::from_byte_array([byte; 32]))
}

#[rustfmt::skip]
fn assert_phase94_block_cap_matches_peer_default() {
    assert_eq!(PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER, DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER);
}

fn transaction_inventory(count: usize) -> InventoryList {
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: hash_from_index(index),
            })
            .collect(),
    )
}

fn block_inventory(count: usize) -> InventoryList {
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: hash_from_index(index),
            })
            .collect(),
    )
}

fn compact_block_inventory(count: usize) -> InventoryList {
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: InventoryType::CompactBlock,
                object_hash: hash_from_index(111_500 + index),
            })
            .collect(),
    )
}

fn phase111_block_witness_compact_inventory(count: usize) -> InventoryList {
    let inventory_types = [
        InventoryType::Block,
        InventoryType::WitnessBlock,
        InventoryType::CompactBlock,
    ];
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: inventory_types[index % inventory_types.len()],
                object_hash: hash_from_index(111_000 + index),
            })
            .collect(),
    )
}

fn cleanup_label_for(
    cause: BlockInFlightCleanupCause,
    blocks_in_flight_before: usize,
    released_blocks: usize,
    remaining_blocks_in_flight: usize,
) -> &'static str {
    classify_block_inflight_cleanup(&BlockInFlightCleanupInput {
        cause,
        blocks_in_flight_before,
        released_blocks,
        remaining_blocks_in_flight,
        max_blocks_in_flight_per_peer: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
        max_blocks_in_flight_total: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
    })
    .label
    .as_str()
}

fn header_chain(count: usize) -> Vec<BlockHeader> {
    let mut headers = Vec::new();
    let mut previous = BlockHash::from_byte_array([0_u8; 32]);
    for index in 0..count {
        let next = header(previous, index as u32 + 1);
        previous = open_bitcoin_consensus::block_hash(&next);
        headers.push(next);
    }
    headers
}

fn hash_from_index(index: usize) -> Hash32 {
    let mut bytes = [0_u8; 32];
    bytes[..8].copy_from_slice(&(index as u64).to_le_bytes());
    Hash32::from_byte_array(bytes)
}

fn ipv4_mapped_address_bytes(octets: [u8; 4]) -> [u8; 16] {
    let mut bytes = [0_u8; 16];
    bytes[..12].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff]);
    bytes[12..].copy_from_slice(&octets);
    bytes
}
