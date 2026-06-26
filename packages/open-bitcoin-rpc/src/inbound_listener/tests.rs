// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use std::sync::Arc;

use open_bitcoin_network::{
    InboundAdmissionSlotClass, InboundListenerConfig, InboundPreflightReason, ParsedNetworkMessage,
    ParsedPeerPermissionClass, PeerConnectionClass, PeerPermissionClassRegistry, VersionMessage,
    WireNetworkMessage,
};
use open_bitcoin_node::core::primitives::NetworkMagic;
use open_bitcoin_node::status::{FieldAvailability, InboundPeerServingStatus};
use open_bitcoin_test_harness::PortReservation;
use tokio::net::TcpStream;

use crate::{ManagedRpcContext, RuntimeConfig};

use super::{
    InboundListenerEvidence, InboundListenerState, activate_inbound_listener,
    start_inbound_accept_loop,
};

fn loopback_config(max_peers: usize) -> InboundListenerConfig {
    loopback_config_with_permission_classes(max_peers, 0, PeerPermissionClassRegistry::default())
}

fn loopback_config_with_permission_classes(
    max_peers: usize,
    reserved_slots: usize,
    permission_classes: PeerPermissionClassRegistry,
) -> InboundListenerConfig {
    InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers,
        reserved_slots,
        allow_public: false,
        permission_classes,
    }
}

async fn running_loopback_listener(
    max_peers: usize,
) -> (
    Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    super::InboundListenerWorker,
    String,
) {
    running_loopback_listener_with_config(loopback_config(max_peers)).await
}

async fn running_loopback_listener_with_config(
    inbound: InboundListenerConfig,
) -> (
    Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    super::InboundListenerWorker,
    String,
) {
    let runtime = RuntimeConfig {
        inbound,
        ..RuntimeConfig::default()
    };
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_runtime_config(&runtime),
    ));
    let activation = activate_inbound_listener(&runtime.inbound).await;
    let endpoint = activation
        .bound_endpoints()
        .first()
        .expect("bound loopback endpoint")
        .bound_endpoint
        .clone();
    let worker = start_inbound_accept_loop(activation, Arc::clone(&context))
        .expect("listener worker should start");
    (context, worker, endpoint)
}

fn loopback_permission_registry(permissions: &[&str]) -> PeerPermissionClassRegistry {
    PeerPermissionClassRegistry::new([ParsedPeerPermissionClass::parse(
        "loopback-permission",
        ["127.0.0.1"],
        permissions.iter().copied(),
    )
    .expect("loopback permission class should parse")])
}

fn listener_evidence(bound_endpoints: &[&str]) -> InboundListenerEvidence {
    InboundListenerEvidence {
        listener_state: "listening".to_string(),
        preflight_reason: "ready".to_string(),
        bound_endpoints: bound_endpoints
            .iter()
            .map(|endpoint| (*endpoint).to_string())
            .collect(),
        admitted_inbound_peers: 0,
        rejected_inbound_peers: 0,
        maybe_admission_reject_reason: None,
        maybe_latest_admission_event: Some("ready".to_string()),
    }
}

fn inbound_status(context: &ManagedRpcContext) -> InboundPeerServingStatus {
    match context.current_inbound_status() {
        FieldAvailability::Available(status) => status,
        FieldAvailability::Unavailable { reason } => {
            panic!("expected inbound status to be available, got {reason}")
        }
    }
}

async fn send_message(stream: &TcpStream, message: WireNetworkMessage) {
    let encoded = message
        .encode_wire(NetworkMagic::MAINNET)
        .expect("encode wire message");
    super::write_all(stream, &encoded)
        .await
        .expect("write wire message");
}

async fn receive_message(stream: &TcpStream) -> WireNetworkMessage {
    let bytes = super::read_wire_message(stream)
        .await
        .expect("read wire message");
    ParsedNetworkMessage::decode_wire(&bytes)
        .expect("decode wire message")
        .message
}

async fn wait_for_inbound_peers(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    expected: usize,
) {
    for _ in 0..100 {
        if context.lock().await.network_info().inbound_peers == expected {
            return;
        }
        tokio::task::yield_now().await;
    }
}

async fn wait_for_reserved_slot_rejections(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    expected: usize,
) {
    for _ in 0..100 {
        if context
            .lock()
            .await
            .inbound_admission_info()
            .reserved_slot_rejections
            == expected
        {
            return;
        }
        tokio::task::yield_now().await;
    }
}

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
    context.set_inbound_listener_evidence(listener_evidence(&["127.0.0.1:18444"]));
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
    context.set_inbound_listener_evidence(listener_evidence(&["8.8.8.8:8333"]));
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
    context.set_inbound_listener_evidence(listener_evidence(&["not-a-socket-address"]));
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

#[tokio::test]
async fn enabled_loopback_zero_port_binds_without_public_network_dependency() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
        permission_classes: Default::default(),
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

#[tokio::test]
async fn ordinary_loopback_inbound_cannot_consume_reserved_capacity() {
    // Arrange
    let config =
        loopback_config_with_permission_classes(2, 1, PeerPermissionClassRegistry::default());
    let (context, worker, endpoint) = running_loopback_listener_with_config(config).await;
    let first = TcpStream::connect(&endpoint)
        .await
        .expect("connect first ordinary loopback peer");
    wait_for_inbound_peers(&context, 1).await;

    // Act
    let second = TcpStream::connect(&endpoint)
        .await
        .expect("connect second ordinary loopback peer");
    drop(second);
    wait_for_reserved_slot_rejections(&context, 1).await;
    let admission = context.lock().await.inbound_admission_info();

    // Assert
    assert_eq!(admission.ordinary_inbound_admits, 1);
    assert_eq!(admission.permissioned_inbound_admits, 0);
    assert_eq!(admission.protected_inbound_admits, 0);
    assert_eq!(admission.reserved_inbound_admits, 0);
    assert_eq!(admission.rejected_inbound_peers, 1);
    assert_eq!(admission.reserved_slot_rejections, 1);
    drop(first);
    worker.shutdown().await;
}

#[tokio::test]
async fn protected_loopback_inbound_consumes_reserved_capacity() {
    // Arrange
    let config = loopback_config_with_permission_classes(
        2,
        1,
        loopback_permission_registry(&["in", "noban", "forceinbound"]),
    );
    let (context, worker, endpoint) = running_loopback_listener_with_config(config).await;

    // Act
    let first = TcpStream::connect(&endpoint)
        .await
        .expect("connect protected loopback peer");
    wait_for_inbound_peers(&context, 1).await;
    let permission_decision = context
        .lock()
        .await
        .permission_decision_for_remote_addr("127.0.0.1:50000".parse().expect("remote address"));
    let admission = context.lock().await.inbound_admission_info();

    // Assert
    assert_eq!(
        permission_decision.connection_class(),
        PeerConnectionClass::ProtectedInbound
    );
    assert_eq!(
        permission_decision.slot_class(),
        InboundAdmissionSlotClass::Reserved
    );
    assert_eq!(admission.ordinary_inbound_admits, 0);
    assert_eq!(admission.permissioned_inbound_admits, 0);
    assert_eq!(admission.protected_inbound_admits, 1);
    assert_eq!(admission.reserved_inbound_admits, 1);
    assert_eq!(admission.active_permission_effect_observations, 4);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
    drop(first);
    worker.shutdown().await;
}

#[tokio::test]
async fn permissioned_loopback_inbound_uses_ordinary_capacity_with_inactive_effect_evidence() {
    // Arrange
    let config = loopback_config_with_permission_classes(
        2,
        1,
        loopback_permission_registry(&[
            "in",
            "download",
            "addr",
            "relay",
            "forcerelay",
            "mempool",
            "bloomfilter",
            "blockfilters",
        ]),
    );
    let (context, worker, endpoint) = running_loopback_listener_with_config(config).await;
    let first = TcpStream::connect(&endpoint)
        .await
        .expect("connect first permissioned loopback peer");
    wait_for_inbound_peers(&context, 1).await;

    // Act
    let second = TcpStream::connect(&endpoint)
        .await
        .expect("connect second permissioned loopback peer");
    drop(second);
    wait_for_reserved_slot_rejections(&context, 1).await;
    let permission_decision = context
        .lock()
        .await
        .permission_decision_for_remote_addr("127.0.0.1:50000".parse().expect("remote address"));
    let admission = context.lock().await.inbound_admission_info();
    let network_info = context.lock().await.network_info();

    // Assert
    assert_eq!(
        permission_decision.connection_class(),
        PeerConnectionClass::PermissionedInbound
    );
    assert_eq!(
        permission_decision.slot_class(),
        InboundAdmissionSlotClass::Ordinary
    );
    assert_eq!(admission.ordinary_inbound_admits, 0);
    assert_eq!(admission.permissioned_inbound_admits, 1);
    assert_eq!(admission.protected_inbound_admits, 0);
    assert_eq!(admission.reserved_inbound_admits, 0);
    assert_eq!(admission.active_permission_effect_observations, 2);
    assert_eq!(admission.inactive_permission_effect_observations, 5);
    assert_eq!(admission.reserved_slot_rejections, 1);
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 0);
    drop(first);
    worker.shutdown().await;
}

#[tokio::test]
async fn loopback_inbound_peer_handshake_increments_inbound_without_outbound() {
    // Arrange
    let (context, worker, endpoint) = running_loopback_listener(2).await;
    let stream = TcpStream::connect(&endpoint)
        .await
        .expect("connect loopback inbound listener");
    let remote_version = VersionMessage {
        nonce: 42,
        ..VersionMessage::default()
    };

    // Act
    send_message(&stream, WireNetworkMessage::Version(remote_version)).await;
    let responses = [
        receive_message(&stream).await,
        receive_message(&stream).await,
        receive_message(&stream).await,
        receive_message(&stream).await,
    ];
    send_message(&stream, WireNetworkMessage::Verack).await;
    let network_info = context.lock().await.network_info();

    // Assert
    assert!(matches!(responses[0], WireNetworkMessage::Version(_)));
    assert!(matches!(responses[1], WireNetworkMessage::WtxidRelay));
    assert!(matches!(responses[2], WireNetworkMessage::Verack));
    assert!(matches!(responses[3], WireNetworkMessage::SendHeaders));
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 0);
    worker.shutdown().await;
}

#[tokio::test]
async fn dropped_loopback_inbound_releases_capacity_for_next_peer() {
    // Arrange
    let (context, worker, endpoint) = running_loopback_listener(1).await;
    let first = TcpStream::connect(&endpoint)
        .await
        .expect("connect first loopback peer");
    send_message(
        &first,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 43,
            ..VersionMessage::default()
        }),
    )
    .await;
    for _ in 0..100 {
        if context.lock().await.network_info().inbound_peers == 1 {
            break;
        }
        tokio::task::yield_now().await;
    }
    drop(first);
    wait_for_inbound_peers(&context, 0).await;

    // Act
    let second = TcpStream::connect(&endpoint)
        .await
        .expect("connect next loopback peer after drop");
    send_message(
        &second,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 44,
            ..VersionMessage::default()
        }),
    )
    .await;
    wait_for_inbound_peers(&context, 1).await;
    let network_info = context.lock().await.network_info();
    let admission = context.lock().await.inbound_admission_info();
    let evidence = worker.evidence();

    // Assert
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 0);
    assert_eq!(admission.admitted_inbound_peers, 2);
    assert_eq!(admission.rejected_inbound_peers, 0);
    assert_eq!(admission.cap_rejections, 0);
    assert_eq!(evidence.maybe_admission_reject_reason, None);
    drop(second);
    worker.shutdown().await;
}
