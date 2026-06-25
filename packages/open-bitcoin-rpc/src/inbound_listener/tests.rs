// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use std::sync::Arc;

use open_bitcoin_network::{
    InboundListenerConfig, InboundPreflightReason, ParsedNetworkMessage, VersionMessage,
    WireNetworkMessage,
};
use open_bitcoin_node::core::primitives::NetworkMagic;
use open_bitcoin_test_harness::PortReservation;
use tokio::net::TcpStream;

use crate::{ManagedRpcContext, RuntimeConfig};

use super::{InboundListenerState, activate_inbound_listener, start_inbound_accept_loop};

fn loopback_config(max_peers: usize) -> InboundListenerConfig {
    InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers,
        reserved_slots: 0,
        allow_public: false,
    }
}

async fn running_loopback_listener(
    max_peers: usize,
) -> (
    Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    super::InboundListenerWorker,
    String,
) {
    let runtime = RuntimeConfig {
        inbound: loopback_config(max_peers),
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
async fn loopback_inbound_cap_rejection_records_evidence_without_admitting_peer() {
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

    // Act
    let second = TcpStream::connect(&endpoint)
        .await
        .expect("connect cap-rejected loopback peer");
    drop(second);
    for _ in 0..10 {
        if context.lock().await.inbound_admission_info().cap_rejections == 1 {
            break;
        }
        tokio::task::yield_now().await;
    }
    let network_info = context.lock().await.network_info();
    let admission = context.lock().await.inbound_admission_info();
    let evidence = worker.evidence();

    // Assert
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 0);
    assert_eq!(admission.rejected_inbound_peers, 1);
    assert_eq!(admission.cap_rejections, 1);
    assert_eq!(
        evidence.maybe_admission_reject_reason.as_deref(),
        Some("cap_reached")
    );
    worker.shutdown().await;
}
