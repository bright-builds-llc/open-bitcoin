// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

pub(super) fn relay_download_policy(inbound_serving_enabled: bool) -> RelayDownloadPolicy {
    RelayDownloadPolicy {
        activation: RelayActivationConfig { enabled: true },
        inbound_serving_enabled,
    }
}

pub(super) fn relay_download_manager(inbound_serving_enabled: bool) -> PeerManager {
    PeerManager::with_relay_download_policy(
        local_config(),
        DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER,
        relay_download_policy(inbound_serving_enabled),
    )
}

pub(super) fn add_relay_outbound_peer(manager: &mut PeerManager, peer_id: PeerId) {
    let _ = manager
        .add_outbound_peer(peer_id, 0)
        .expect("outbound peer should be added");
}

pub(super) fn add_relay_permissioned_inbound_peer(manager: &mut PeerManager, peer_id: PeerId) {
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            peer_id,
            permission_decision(["in", "relay"]),
        ))
        .expect("permissioned inbound peer should be added");
}

pub(super) fn protected_permission_decision() -> InboundPermissionDecision {
    let class =
        ParsedPeerPermissionClass::parse("protected-test", ["203.0.113.7"], ["in", "forceinbound"])
            .expect("protected class");
    let address: IpAddr = "203.0.113.7".parse().expect("test address");
    PeerPermissionClassRegistry::new([class]).resolve_inbound(address)
}

pub(super) fn permission_decision(
    tokens: impl IntoIterator<Item = &'static str>,
) -> InboundPermissionDecision {
    let class = ParsedPeerPermissionClass::parse("phase91-test", ["203.0.113.91"], tokens)
        .expect("permission class");
    let address: IpAddr = "203.0.113.91".parse().expect("test address");
    PeerPermissionClassRegistry::new([class]).resolve_inbound(address)
}

pub(super) fn permissioned_inbound_record(
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

pub(super) fn active_permission_labels(decision: &InboundPermissionDecision) -> Vec<&'static str> {
    decision
        .active_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect()
}

pub(super) fn inactive_permission_labels(
    decision: &InboundPermissionDecision,
) -> Vec<&'static str> {
    decision
        .inactive_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect()
}

pub(super) fn relay_permission_labels(decision: &InboundPermissionDecision) -> Vec<&'static str> {
    decision
        .relay_permission_effects()
        .iter()
        .map(|effect| effect.as_str())
        .collect()
}

pub(super) fn assert_transaction_relay_request(
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

pub(super) fn assert_transaction_relay_identity_mismatch(
    actions: &[PeerAction],
    expected_peer_id: PeerId,
) {
    let [PeerAction::TransactionRelay(TxDownloadAction::SuppressIdentityMismatch {
        peer_id, ..
    })] = actions
    else {
        panic!("expected transaction relay identity mismatch, got {actions:?}");
    };
    assert_eq!(*peer_id, expected_peer_id);
}

pub(super) fn assert_transaction_relay_duplicate(
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

pub(super) fn assert_transaction_relay_suppression(
    actions: &[PeerAction],
    expected_peer_id: PeerId,
    expected_relay_id: TxRelayId,
    expected_reason: TxDownloadSuppressionReason,
) {
    let [PeerAction::TransactionRelay(TxDownloadAction::Suppress {
        peer_id,
        relay_id,
        reason,
    })] = actions
    else {
        panic!("expected transaction relay suppression, got {actions:?}");
    };
    assert_eq!(
        (*peer_id, *relay_id, *reason),
        (expected_peer_id, expected_relay_id, expected_reason),
    );
}

pub(super) fn seed_duplicate_announcements(
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

pub(super) fn transaction_relay_inventory(relay_id: TxRelayId) -> InventoryList {
    InventoryList::new(vec![relay_id.to_inventory_vector()])
}

pub(super) fn txid_from_byte(byte: u8) -> Txid {
    Txid::from(Hash32::from_byte_array([byte; 32]))
}

pub(super) fn wtxid_from_byte(byte: u8) -> Wtxid {
    Wtxid::from(Hash32::from_byte_array([byte; 32]))
}
