// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::chainstate::PruneMode;
use open_bitcoin_network::{PeerAction, advertised_service_flags};

use super::*;

fn assert_limited_services(services: ServiceFlags) {
    assert!(services.contains(ServiceFlags::NETWORK_LIMITED));
    assert!(services.contains(ServiceFlags::WITNESS));
    assert!(!services.contains(ServiceFlags::NETWORK));
    assert_eq!(services, advertised_service_flags(PruneMode::ManualOnly));
}

fn assert_full_history_services(services: ServiceFlags) {
    assert!(services.contains(ServiceFlags::NETWORK));
    assert!(services.contains(ServiceFlags::WITNESS));
    assert!(!services.contains(ServiceFlags::NETWORK_LIMITED));
    assert_eq!(services, advertised_service_flags(PruneMode::Disabled));
}

fn maybe_version_services(messages: &[WireNetworkMessage]) -> Option<ServiceFlags> {
    messages.iter().find_map(|message| match message {
        WireNetworkMessage::Version(version) => Some(version.services),
        _ => None,
    })
}

fn maybe_outbound_version_services(actions: &[PeerAction]) -> Option<ServiceFlags> {
    actions.iter().find_map(|action| match action {
        PeerAction::Send(WireNetworkMessage::Version(version)) => Some(version.services),
        _ => None,
    })
}

#[test]
fn manual_only_advertises_limited_service_while_have_pruned_is_false() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(149_201);

    // Act
    network.set_serving_have_pruned(false);
    network.set_prune_mode(PruneMode::ManualOnly);
    let services = network.local_services();

    // Assert
    assert_limited_services(services);
}

#[test]
fn automatic_mode_advertises_limited_service_while_have_pruned_is_false() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(149_202);

    // Act
    network.set_serving_have_pruned(false);
    network.set_prune_mode(PruneMode::Automatic { target_mib: 550 });
    let services = network.local_services();

    // Assert
    assert_limited_services(services);
}

#[test]
fn disabled_mode_advertises_full_history_without_network_limited() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(149_203);

    // Act
    network.set_serving_have_pruned(true);
    network.set_prune_mode(PruneMode::Disabled);
    let services = network.local_services();

    // Assert
    assert_full_history_services(services);
}

#[test]
fn add_outbound_peer_sends_limited_version_after_manual_mode() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(149_204);
    network.set_serving_have_pruned(false);
    network.set_prune_mode(PruneMode::ManualOnly);

    // Act
    let actions = network
        .peer_manager_mut()
        .add_outbound_peer(149_204, 1)
        .expect("add outbound peer");
    let services = maybe_outbound_version_services(&actions).expect("version message");

    // Assert
    assert_limited_services(services);
}

#[test]
fn fresh_local_config_network_sends_full_history_until_prune_mode() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(149_205),
        PolicyConfig::default(),
    );

    // Act
    let messages = network
        .connect_outbound_peer(149_205, 1)
        .expect("connect outbound peer");
    let services = maybe_version_services(&messages).expect("version message");

    // Assert
    assert_full_history_services(services);
}

#[test]
fn transient_runtime_advertises_disabled_service_flags() {
    // Arrange
    let handle = crate::ManagedNetworkHandle::transient_runtime(
        NetworkMagic::MAINNET,
        8333,
        RelayActivationConfig::default(),
        BlockRelayActivationPolicy::default(),
        false,
    );

    // Act
    let info = handle.network_info().expect("network info");

    // Assert
    assert_eq!(
        info.local_services_bits,
        advertised_service_flags(PruneMode::Disabled).bits()
    );
    assert_ne!(
        info.local_services_bits,
        ServiceFlags::NETWORK_LIMITED.bits()
    );
}
