// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::listener_fixtures::*;
use super::*;

#[tokio::test]
async fn oversized_header_returns_payload_oversized_before_payload_allocation() {
    // Arrange
    let mut header = verack_header(NetworkMagic::MAINNET);
    let oversized_len = (PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES as u32)
        .saturating_add(1)
        .to_le_bytes();
    header[16..20].copy_from_slice(&oversized_len);

    // Act
    let label = read_rejected_header(header).await;

    // Assert
    assert_eq!(label, "payload_oversized");
}

#[tokio::test]
async fn wrong_magic_returns_wrong_network_magic_and_closes_message_loop() {
    // Arrange
    let regtest_magic = NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]);
    let header = verack_header(regtest_magic);

    // Act
    let label = read_rejected_header(header).await;

    // Assert
    assert_eq!(label, "wrong_network_magic");
}

#[tokio::test]
async fn unsupported_command_records_evidence_without_receive_inbound_wire_message() {
    // Arrange
    let (context, worker, endpoint) = running_loopback_listener(2).await;
    let stream = TcpStream::connect(&endpoint)
        .await
        .expect("connect loopback inbound listener");
    let header = unsupported_command_header();

    // Act
    super::write_all(&stream, &header)
        .await
        .expect("write unsupported command header");
    for _ in 0..100 {
        if worker
            .evidence()
            .maybe_latest_resource_event
            .as_ref()
            .is_some_and(|event| event.label == "unsupported_command")
        {
            break;
        }
        tokio::task::yield_now().await;
    }
    let evidence = worker.evidence();
    let network_info = context
        .lock()
        .await
        .network_info()
        .expect("authoritative network info");

    // Assert
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .expect("resource event should be recorded")
            .label,
        "unsupported_command"
    );
    assert_eq!(
        evidence.maybe_latest_admission_event.as_deref(),
        Some("admitted")
    );
    assert_eq!(network_info.outbound_peers, 0);
    worker.shutdown().await;
}

#[test]
fn record_resource_event_counts_timeout_churn_and_reconnect_actions() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut evidence = listener_evidence(&["127.0.0.1:18444"]);
    let timeout = super::resource_timeout_event(
        &policy,
        10,
        10,
        10 + PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS + 1,
        InboundHandshakeState::Handshaking,
    )
    .expect("slow_handshake timeout event");
    let churn = match policy.decide_churn(open_bitcoin_network::ConnectionChurnInput {
        window_started_unix_seconds: 10,
        now_unix_seconds: 10,
        connection_attempts_in_window: PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW + 1,
    }) {
        ResourceGovernanceDecision::Backpressure(event) => event,
        other => panic!("expected connection_churn_limited event, got {other:?}"),
    };
    let reconnect = match policy.decide_reconnect(ReconnectSuppressionInput {
        banned: true,
        discouraged: false,
    }) {
        ResourceGovernanceDecision::Disconnect(event) => event,
        other => panic!("expected reconnect_suppressed event, got {other:?}"),
    };

    // Act
    evidence.record_resource_event(timeout);
    evidence.record_resource_event(churn);
    evidence.record_resource_event(reconnect);

    // Assert
    assert_eq!(evidence.timeout_disconnects, 1);
    assert_eq!(evidence.churn_rejections, 1);
    assert_eq!(evidence.reconnect_suppressions, 1);
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .expect("latest resource event")
            .label,
        "reconnect_suppressed_banned"
    );
}

#[test]
fn resource_timeout_event_distinguishes_slow_handshake_and_idle_peer() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();

    // Act
    let slow_handshake = super::resource_timeout_event(
        &policy,
        100,
        100,
        100 + policy.slow_handshake_timeout_seconds + 1,
        InboundHandshakeState::Accepted,
    )
    .expect("slow_handshake timeout");
    let idle_peer = super::resource_timeout_event(
        &policy,
        100,
        200,
        200 + policy.idle_peer_timeout_seconds + 1,
        InboundHandshakeState::Established,
    )
    .expect("idle_peer timeout");

    // Assert
    assert_eq!(slow_handshake.label, "slow_handshake");
    assert_eq!(slow_handshake.next_action, "timeout_disconnect");
    assert_eq!(idle_peer.label, "idle_peer");
    assert_eq!(idle_peer.next_action, "timeout_disconnect");
}

#[test]
fn runtime_window_counters_limit_churn_and_repeated_failures() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut counters = super::InboundRuntimeCounters::new(1_000);

    // Act
    let mut latest_churn = ResourceGovernanceDecision::Accept;
    for _ in 0..=PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW {
        let input = counters.record_connection_attempt(&policy, 1_000);
        latest_churn = policy.decide_churn(input);
    }
    for _ in 0..=PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW {
        counters.record_failure(&policy, 1_000);
    }
    let repeated_failure =
        policy.decide_repeated_failure(counters.repeated_failure_input(&policy, 1_000));

    // Assert
    let ResourceGovernanceDecision::Backpressure(churn_event) = latest_churn else {
        panic!("expected connection_churn_limited backpressure");
    };
    assert_eq!(churn_event.label, "connection_churn_limited");
    let ResourceGovernanceDecision::Backpressure(failure_event) = repeated_failure else {
        panic!("expected repeated_failure_limited backpressure");
    };
    assert_eq!(failure_event.label, "repeated_failure_limited");
    assert_eq!(failure_event.next_action, "churn_rejected");
}

#[test]
fn read_queue_pressure_is_decided_before_socket_read() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut queue = super::RuntimeQueuePressureState::default();
    queue.record_pending_read(PHASE94_MAX_PEER_READ_QUEUE_BYTES + 1);

    // Act
    let event = super::queue_pressure_event(&policy, &queue, Vec::new(), Vec::new())
        .expect("read queue pressure event");

    // Assert
    assert_eq!(event.label, "read_queue_pressure");
    assert_eq!(event.next_action, "read_queue_pressure");
}

#[test]
fn write_queue_pressure_skips_socket_write() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut queue = super::RuntimeQueuePressureState::default();
    queue.record_pending_write(PHASE94_MAX_PEER_WRITE_QUEUE_BYTES + 1);

    // Act
    let event = super::queue_pressure_event(&policy, &queue, Vec::new(), Vec::new())
        .expect("write queue pressure event");

    // Assert
    assert_eq!(event.label, "write_queue_pressure");
    assert_eq!(event.next_action, "write_queue_pressure");
}

#[test]
fn aggregate_queue_pressure_records_shared_resource_evidence() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut queue = super::RuntimeQueuePressureState::default();
    queue.record_aggregate_queued_messages(policy.max_aggregate_queued_messages + 1);
    let event = super::queue_pressure_event(&policy, &queue, Vec::new(), Vec::new())
        .expect("aggregate queue pressure event");
    let mut evidence = listener_evidence(&["127.0.0.1:18444"]);
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .set_inbound_listener_evidence(listener_evidence(&["127.0.0.1:18444"]))
        .expect("authoritative listener evidence");

    // Act
    evidence.record_resource_event(event.clone());
    context.record_inbound_resource_event(event);

    // Assert
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .as_ref()
            .expect("listener resource event")
            .label,
        "resource_pressure_active"
    );
    assert_eq!(
        context
            .maybe_inbound_listener_evidence()
            .expect("managed evidence")
            .maybe_latest_resource_event
            .as_ref()
            .expect("managed resource event")
            .label,
        "resource_pressure_active"
    );
}

#[tokio::test]
async fn read_wire_message_returns_timeout_disconnect_without_wall_clock_wait() {
    // Arrange
    let (_client, server) = tcp_pair().await;
    let envelope_policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let resource_policy = ResourceGovernancePolicy::default();

    // Act
    let outcome = super::read_wire_message_with_timeout_duration(
        &server,
        &envelope_policy,
        &resource_policy,
        100,
        100,
        InboundHandshakeState::Handshaking,
        Duration::ZERO,
    )
    .await
    .expect("read timeout should return resource event");

    // Assert
    let ReadWireMessageOutcome::Rejected(event) = outcome else {
        panic!("expected timeout_disconnect resource event");
    };
    assert_eq!(event.label, "slow_handshake");
    assert_eq!(event.next_action, "timeout_disconnect");
}

#[tokio::test(start_paused = true)]
async fn read_wire_message_times_out_across_partial_header_bytes() {
    // Arrange
    let (client, server) = tcp_pair().await;
    let envelope_policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let resource_policy = ResourceGovernancePolicy::default();
    let header = verack_header(NetworkMagic::MAINNET);
    let timeout_duration = Duration::from_secs(5);
    let read_task = tokio::spawn(async move {
        super::read_wire_message_with_timeout_duration(
            &server,
            &envelope_policy,
            &resource_policy,
            100,
            100,
            InboundHandshakeState::Handshaking,
            timeout_duration,
        )
        .await
        .expect("partial read timeout should return resource event")
    });
    tokio::task::yield_now().await;

    // Act
    client.writable().await.expect("client socket writable");
    assert_eq!(
        client
            .try_write(&header[..1])
            .expect("write first header byte"),
        1
    );
    tokio::time::advance(Duration::from_secs(4)).await;
    tokio::task::yield_now().await;
    client
        .writable()
        .await
        .expect("client socket writable again");
    assert_eq!(
        client
            .try_write(&header[1..2])
            .expect("write second header byte"),
        1
    );
    tokio::time::advance(Duration::from_secs(2)).await;
    for _ in 0..10 {
        if read_task.is_finished() {
            break;
        }
        tokio::task::yield_now().await;
    }
    let outcome = read_task.await.expect("read task should join");

    // Assert
    let ReadWireMessageOutcome::Rejected(event) = outcome else {
        panic!("expected timeout_disconnect resource event");
    };
    assert_eq!(event.label, "slow_handshake");
    assert_eq!(event.next_action, "timeout_disconnect");
}

#[test]
fn context_records_inbound_resource_event_for_managed_evidence() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .set_inbound_listener_evidence(listener_evidence(&["127.0.0.1:18444"]))
        .expect("authoritative listener evidence");
    let policy = ResourceGovernancePolicy::default();
    let reconnect = match policy.decide_reconnect(ReconnectSuppressionInput {
        banned: false,
        discouraged: true,
    }) {
        ResourceGovernanceDecision::Backpressure(event) => event,
        other => panic!("expected reconnect_suppressed event, got {other:?}"),
    };

    // Act
    context.record_inbound_resource_event(reconnect);

    // Assert
    let evidence = context
        .maybe_inbound_listener_evidence()
        .expect("managed evidence should be present");
    assert_eq!(evidence.reconnect_suppressions, 1);
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .as_ref()
            .expect("latest resource event")
            .next_action,
        "reconnect_suppressed"
    );
}
