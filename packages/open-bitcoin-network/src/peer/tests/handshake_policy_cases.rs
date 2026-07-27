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
    assert_eq!(version.sender, super::super::super::message::zero_address());
    assert_eq!(evidence.suppressed_advertisements.len(), 1);
}
