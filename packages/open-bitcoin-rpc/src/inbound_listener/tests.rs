// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use open_bitcoin_network::{InboundListenerConfig, InboundPreflightReason};
use open_bitcoin_test_harness::PortReservation;

use super::{InboundListenerState, activate_inbound_listener};

#[tokio::test]
async fn disabled_runtime_reports_disabled_without_bound_endpoints() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: false,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Disabled);
    assert_eq!(activation.preflight_reason(), InboundPreflightReason::Disabled);
    assert!(activation.bound_endpoints().is_empty());
    assert_eq!(
        activation
            .latest_admission_event()
            .expect("listener activation should record a latest event"),
        "disabled"
    );
}

#[tokio::test]
async fn invalid_endpoint_reports_typed_diagnostic_before_bind() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["not-a-socket-address".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Blocked);
    assert_eq!(
        activation.preflight_reason(),
        InboundPreflightReason::InvalidEndpoint
    );
    assert!(activation.bound_endpoints().is_empty());
    assert_eq!(
        activation.diagnostics()[0].maybe_endpoint.as_deref(),
        Some("not-a-socket-address")
    );
}

#[tokio::test]
async fn unsafe_public_endpoint_reports_typed_diagnostic_before_bind() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["0.0.0.0:18444".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Blocked);
    assert_eq!(
        activation.preflight_reason(),
        InboundPreflightReason::UnsafeEndpoint
    );
    assert!(activation.bound_endpoints().is_empty());
    assert_eq!(
        activation.diagnostics()[0].maybe_endpoint.as_deref(),
        Some("0.0.0.0:18444")
    );
}

#[tokio::test]
async fn held_loopback_address_reports_bind_failure_with_next_action() {
    // Arrange
    let held = PortReservation::localhost().expect("held loopback port");
    let config = InboundListenerConfig {
        enabled: true,
        listen_addresses: vec![held.address().to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Blocked);
    assert!(matches!(
        activation.preflight_reason(),
        InboundPreflightReason::AlreadyBound | InboundPreflightReason::BindUnavailable
    ));
    assert!(activation.bound_endpoints().is_empty());
    assert_eq!(
        activation.diagnostics()[0].maybe_endpoint.as_deref(),
        Some(held.address().to_string().as_str())
    );
    assert!(!activation.diagnostics()[0].next_action.is_empty());
}

#[tokio::test]
async fn enabled_loopback_zero_port_binds_without_public_network_dependency() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Listening);
    assert_eq!(activation.preflight_reason(), InboundPreflightReason::Ready);
    let endpoint = activation
        .bound_endpoints()
        .first()
        .expect("loopback endpoint should bind");
    assert!(endpoint.bound_endpoint.starts_with("127.0.0.1:"));
    assert_ne!(endpoint.bound_endpoint, "127.0.0.1:0");
}
