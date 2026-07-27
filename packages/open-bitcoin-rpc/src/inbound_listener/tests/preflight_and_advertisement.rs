// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::listener_fixtures::*;
use super::*;

#[tokio::test]
async fn disabled_runtime_reports_disabled_without_bound_endpoints() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: false,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
        permission_classes: Default::default(),
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Disabled);
    assert_eq!(
        activation.preflight_reason(),
        InboundPreflightReason::Disabled
    );
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
        permission_classes: Default::default(),
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
        permission_classes: Default::default(),
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
        permission_classes: Default::default(),
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

#[test]
fn loopback_listener_evidence_is_suppressed_for_public_advertisement() {
    // Arrange
    let runtime = RuntimeConfig {
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:18444".to_string()],
            max_peers: 8,
            reserved_slots: 0,
            allow_public: false,
            permission_classes: Default::default(),
        },
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);

    // Act
    context
        .set_inbound_listener_evidence(listener_evidence(&["127.0.0.1:18444"]))
        .expect("authoritative listener evidence");
    let status = inbound_status(&context);

    // Assert
    assert!(status.local_advertisement_candidates.is_empty());
    assert_eq!(status.suppressed_advertisements.len(), 1);
    assert_eq!(
        status.suppressed_advertisements[0].label,
        "advertise_suppressed"
    );
    assert_eq!(
        status.suppressed_advertisements[0].reason,
        "not_publicly_routable"
    );
}

#[test]
fn public_literal_listener_evidence_can_be_advertisement_candidate_when_allowed() {
    // Arrange
    let runtime = RuntimeConfig {
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["8.8.8.8:8333".to_string()],
            max_peers: 8,
            reserved_slots: 0,
            allow_public: true,
            permission_classes: Default::default(),
        },
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);

    // Act
    context
        .set_inbound_listener_evidence(listener_evidence(&["8.8.8.8:8333"]))
        .expect("authoritative listener evidence");
    let status = inbound_status(&context);

    // Assert
    assert_eq!(status.local_advertisement_candidates.len(), 1);
    assert_eq!(
        status.local_advertisement_candidates[0].source,
        "source_local_listener"
    );
    assert_eq!(
        status.local_advertisement_candidates[0].network_kind,
        "ipv4"
    );
    assert_eq!(
        status.local_advertisement_candidates[0].routability,
        "publicly_routable"
    );
    assert_eq!(status.local_advertisement_candidates[0].port, 8333);
    assert!(status.suppressed_advertisements.is_empty());
}

#[test]
fn invalid_runtime_bound_evidence_is_suppressed_without_falling_back_to_configured_public_address()
{
    // Arrange
    let runtime = RuntimeConfig {
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["8.8.8.8:8333".to_string()],
            max_peers: 8,
            reserved_slots: 0,
            allow_public: true,
            permission_classes: Default::default(),
        },
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);

    // Act
    context
        .set_inbound_listener_evidence(listener_evidence(&["not-a-socket-address"]))
        .expect("authoritative listener evidence");
    let status = inbound_status(&context);

    // Assert
    assert!(status.local_advertisement_candidates.is_empty());
    assert_eq!(status.suppressed_advertisements.len(), 1);
    assert_eq!(
        status.suppressed_advertisements[0].label,
        "advertise_suppressed"
    );
    assert_eq!(
        status.suppressed_advertisements[0].reason,
        "unsupported_address_network"
    );
}
