// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::listener_fixtures::*;
use super::*;

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
    let admission = context
        .lock()
        .await
        .inbound_admission_info()
        .expect("authoritative inbound admission");

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
    let admission = context
        .lock()
        .await
        .inbound_admission_info()
        .expect("authoritative inbound admission");

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
async fn permissioned_loopback_inbound_uses_ordinary_capacity_with_scoped_filter_evidence() {
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
    let admission = context
        .lock()
        .await
        .inbound_admission_info()
        .expect("authoritative inbound admission");
    let network_info = context
        .lock()
        .await
        .network_info()
        .expect("authoritative network info");

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
    assert_eq!(admission.inactive_permission_effect_observations, 2);
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
    let network_info = context
        .lock()
        .await
        .network_info()
        .expect("authoritative network info");

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
async fn idle_inbound_peer_wakes_for_queued_announcement_and_credits_once() {
    // Arrange
    let (_context, worker, endpoint, outboxes, network) =
        running_loopback_listener_with_announcements().await;
    let stream = TcpStream::connect(&endpoint)
        .await
        .expect("connect announcement loopback peer");
    send_message(
        &stream,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 128,
            ..VersionMessage::default()
        }),
    )
    .await;
    for _ in 0..4 {
        let _ = receive_message(&stream).await;
    }
    send_message(&stream, WireNetworkMessage::Verack).await;
    let compact_offer = receive_any_message(&stream).await;
    assert!(matches!(compact_offer, WireNetworkMessage::SendCompact(_)));
    let snapshots = outboxes.snapshots().expect("registered inbound outbox");
    let peer_id = snapshots
        .first()
        .expect("one registered inbound outbox")
        .peer_id();
    let block = Block::default();
    let outcomes = network
        .prepare_block_announcements(&block, &snapshots)
        .expect("prepare idle inbound announcement");

    // Act
    outboxes
        .enqueue_prepared(outcomes)
        .expect("enqueue idle inbound announcement");
    let announcement = tokio::time::timeout(Duration::from_secs(1), receive_any_message(&stream))
        .await
        .expect("idle inbound peer should wake without another socket message");
    tokio::task::yield_now().await;
    let evidence = serde_json::to_value(
        network
            .block_relay_evidence_status()
            .expect("announcement evidence"),
    )
    .expect("serialize announcement evidence");

    // Assert
    assert!(matches!(announcement, WireNetworkMessage::Inv(_)));
    assert_eq!(
        evidence["announcement"]["value"]["compact_inventory_fallback_count"],
        1
    );
    assert!(
        outboxes
            .take_peer_emissions(peer_id)
            .expect("drained inbound outbox")
            .is_empty()
    );
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
        if context
            .lock()
            .await
            .network_info()
            .is_ok_and(|info| info.inbound_peers == 1)
        {
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
    let network_info = context
        .lock()
        .await
        .network_info()
        .expect("authoritative network info");
    let admission = context
        .lock()
        .await
        .inbound_admission_info()
        .expect("authoritative inbound admission");
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
