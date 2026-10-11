// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp

use super::*;
use open_bitcoin_core::chainstate::PruneMode;
use open_bitcoin_network::{PeerAction, VersionMessage};

#[test]
fn phase161_operator_authority_memory_keeps_configuration_without_live_backend() {
    // Arrange
    let network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(161_401),
        PolicyConfig::default(),
    );
    // Act
    let snapshot = network.operator_snapshot();
    // Assert
    assert_eq!(
        snapshot.basic_index().configured_peer_serving,
        crate::status::FieldAvailability::available(false)
    );
    assert!(matches!(
        snapshot.basic_index().configured_indexing,
        crate::status::FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        snapshot.basic_index().maybe_processed_height,
        crate::status::FieldAvailability::Unavailable { .. }
    ));
}

#[test]
fn phase160_services_prune_update_preserves_configured_filter_bit() {
    // Arrange
    let mut config = local_config(160_201);
    config.services |= ServiceFlags::COMPACT_FILTERS;
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        config,
        PolicyConfig::default(),
    );

    // Act
    network.set_prune_mode(PruneMode::ManualOnly);

    // Assert
    assert!(
        network
            .local_services()
            .contains(ServiceFlags::COMPACT_FILTERS)
    );
    assert!(
        network
            .local_services()
            .contains(ServiceFlags::NETWORK_LIMITED)
    );
    assert!(network.local_services().contains(ServiceFlags::WITNESS));
    assert!(!network.local_services().contains(ServiceFlags::NETWORK));
}

fn version_services(actions: &[PeerAction]) -> ServiceFlags {
    actions
        .iter()
        .find_map(|action| match action {
            PeerAction::Send(WireNetworkMessage::Version(version)) => Some(version.services),
            _ => None,
        })
        .expect("local version")
}

fn receive_inbound_version(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
) -> ServiceFlags {
    let actions = network
        .peer_manager_mut()
        .handle_message(
            peer_id,
            WireNetworkMessage::Version(VersionMessage {
                nonce: 999,
                ..Default::default()
            }),
            1,
        )
        .expect("inbound version");
    version_services(&actions)
}

#[test]
fn phase160_services_general_serving_both_directions_before_catchup() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(160_202),
        PolicyConfig::default(),
    );
    network.set_basic_filter_serving(true);
    network.add_inbound_peer(1).expect("inbound peer");

    // Act
    let outbound = network
        .peer_manager_mut()
        .add_outbound_peer(2, 1)
        .expect("outbound peer");
    let inbound_services = receive_inbound_version(&mut network, 1);

    // Assert
    assert_eq!(network.peer_manager().header_store().best_height(), -1);
    assert!(inbound_services.contains(ServiceFlags::COMPACT_FILTERS));
    assert_eq!(version_services(&outbound), inbound_services);
    for peer_id in [1, 2] {
        let peer = network.peer_manager().peer_state(peer_id).expect("peer");
        assert_eq!(peer.own_advertised_services, inbound_services);
        assert!(!peer.sync_started);
    }
}

#[test]
fn phase160_services_permission_overlay_stays_local_for_all_and_explicit() {
    // Arrange
    for tokens in [["in", "all"], ["in", "blockfilters"]] {
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config(160_203),
            PolicyConfig::default(),
        );
        network.set_prune_mode(PruneMode::ManualOnly);
        network.set_inbound_admission_policy(InboundAdmissionPolicy::new(8, 2));
        let decision = network.admit_inbound_peer(permissioned_inbound_request(
            1,
            "203.0.113.7:10001",
            &tokens,
        ));
        assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
        network.add_inbound_peer(2).expect("ordinary inbound");
        let base = network.local_services();

        // Act
        let permitted = receive_inbound_version(&mut network, 1);
        let ordinary = receive_inbound_version(&mut network, 2);
        let outbound = network
            .peer_manager_mut()
            .add_outbound_peer(3, 1)
            .expect("outbound");

        // Assert
        assert_eq!(permitted, base | ServiceFlags::COMPACT_FILTERS);
        assert_eq!(ordinary, base);
        assert_eq!(version_services(&outbound), base);
        assert_eq!(network.local_services(), base);
        assert!(permitted.contains(ServiceFlags::NETWORK_LIMITED));
        assert!(!permitted.contains(ServiceFlags::NETWORK));
        assert_eq!(permitted.bits() & (1 << 2), 0);
        let peer = network
            .peer_manager()
            .peer_state(1)
            .expect("permitted peer");
        assert_eq!(peer.own_advertised_services, permitted);
        let permission = &peer
            .maybe_inbound_record
            .as_ref()
            .expect("record")
            .permission_decision;
        assert!(permission.active_effects().contains(
            &open_bitcoin_network::PermissionEffectLabel::BlockFiltersServingPolicyInput
        ));
    }
}

#[test]
fn phase160_services_implicit_and_protected_permissions_never_grant_filters() {
    // Arrange
    let cases: &[&[&str]] = &[
        &["in", "noban"],
        &["in", "download"],
        &["in", "forceinbound"],
        &["in", "mempool", "noban", "addr", "relay"],
    ];
    for tokens in cases {
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config(160_204),
            PolicyConfig::default(),
        );
        network.set_inbound_admission_policy(InboundAdmissionPolicy::new(8, 2));
        assert!(matches!(
            network.admit_inbound_peer(permissioned_inbound_request(
                1,
                "203.0.113.7:10001",
                tokens,
            )),
            InboundAdmissionDecision::Admit(_)
        ));

        // Act
        let services = receive_inbound_version(&mut network, 1);

        // Assert
        assert!(!services.contains(ServiceFlags::COMPACT_FILTERS));
        assert_eq!(services, network.local_services());
    }
}

#[test]
fn phase160_services_prune_and_general_setters_compose_in_both_orders() {
    // Arrange
    for mode in [
        PruneMode::Disabled,
        PruneMode::ManualOnly,
        PruneMode::Automatic { target_mib: 550 },
    ] {
        for prune_first in [false, true] {
            let mut network = ManagedPeerNetwork::new(
                MemoryChainstateStore::default(),
                local_config(160_205),
                PolicyConfig::default(),
            );

            // Act
            if prune_first {
                network.set_prune_mode(mode);
            }
            network.set_basic_filter_serving(true);
            network.set_prune_mode(mode);
            let enabled = network.local_services();
            network.set_basic_filter_serving(false);

            // Assert
            let base = open_bitcoin_network::advertised_service_flags(mode);
            assert_eq!(enabled, base | ServiceFlags::COMPACT_FILTERS);
            assert_eq!(network.local_services(), base);
        }
    }
}

#[test]
fn phase160_services_remote_filter_bit_cannot_authorize_or_rewrite_own_snapshot() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(160_206),
        PolicyConfig::default(),
    );
    network
        .peer_manager_mut()
        .add_outbound_peer(1, 1)
        .expect("outbound");
    let original = network
        .peer_manager()
        .peer_state(1)
        .expect("peer")
        .own_advertised_services;
    network.set_basic_filter_serving(true);

    // Act
    network
        .peer_manager_mut()
        .handle_message(
            1,
            WireNetworkMessage::Version(VersionMessage {
                nonce: 999,
                services: ServiceFlags::NETWORK | ServiceFlags::COMPACT_FILTERS,
                ..Default::default()
            }),
            2,
        )
        .expect("remote version");

    // Assert
    let peer = network.peer_manager().peer_state(1).expect("peer");
    assert_ne!(
        peer.remote_services_bits & ServiceFlags::COMPACT_FILTERS.bits(),
        0
    );
    assert_eq!(peer.own_advertised_services, original);
    assert!(
        !peer
            .own_advertised_services
            .contains(ServiceFlags::COMPACT_FILTERS)
    );
}
