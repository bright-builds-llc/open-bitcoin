// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

use super::*;

struct PartialWriter<'a> {
    fixture: &'a FilterListenerFixture,
    stream: &'a TcpStream,
    fail_on: usize,
    calls: usize,
    invalidate: bool,
}

impl InboundFilterWriter for PartialWriter<'_> {
    async fn write(&mut self, bytes: &[u8]) -> io::Result<WriteWireMessageOutcome> {
        // These acquisitions execute inside the production executor's write await.
        let context = self
            .fixture
            .context
            .try_lock()
            .expect("context free during socket write");
        self.fixture
            .runtime
            .network_handle()
            .network_info()
            .expect("authority free during socket write");
        drop(context);
        self.calls += 1;
        if self.calls == self.fail_on {
            write_all(self.stream, &bytes[..13]).await?;
            return Err(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "forced partial frame",
            ));
        }
        write_all(self.stream, bytes).await?;
        if self.invalidate {
            self.fixture
                .runtime
                .network_handle()
                .configure_basic_peer_serving(false)
                .expect("real lifecycle invalidation");
        }
        Ok(WriteWireMessageOutcome::Written)
    }
}

async fn partial_failure(fail_on: usize) {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let response = fixture.prepared().await;
    let network = fixture.runtime.network_handle();
    let (client, server) = tcp_pair().await;
    // Act
    let failure = execute_inbound_filter(
        response,
        &network,
        PartialWriter {
            fixture: &fixture,
            stream: &server,
            fail_on,
            calls: 0,
            invalidate: false,
        },
    )
    .await
    .expect_err("forced partial terminal");
    // Assert
    assert!(
        matches!(failure.cause, FilterWriteFailure::Write(ref error) if error.kind() == io::ErrorKind::ConnectionReset)
    );
    assert_eq!(failure.progress.messages, (fail_on - 1) as u64);
    assert!(failure.maybe_cleanup.is_none());
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("all suffix released"),
        FilterTransportSnapshot::default()
    );
    if fail_on == 2 {
        let frame = receive_frame(&client).await;
        assert_eq!(failure.progress.bytes, frame.len() as u64);
        assert!(matches!(
            ParsedNetworkMessage::decode_wire(&frame)
                .expect("fully written prefix")
                .message,
            WireNetworkMessage::CFilter(_)
        ));
    } else {
        assert_eq!(failure.progress.bytes, 0);
    }
    let mut partial = [0_u8; 13];
    client.readable().await.expect("actual partial bytes");
    assert_eq!(
        client
            .try_read(&mut partial)
            .expect("actual partial prefix"),
        13
    );
    drop(network);
    fixture.cleanup();
}

#[tokio::test]
async fn phase160_inbound_executor_partial_first_frame_earns_zero() {
    partial_failure(1).await;
}

#[tokio::test]
async fn phase160_inbound_transport_partial_second_frame_keeps_only_complete_prefix() {
    partial_failure(2).await;
}

#[tokio::test]
async fn phase160_inbound_executor_late_full_frame_is_physical_only() {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let response = fixture.prepared().await;
    let network = fixture.runtime.network_handle();
    let (client, server) = tcp_pair().await;
    // Act
    let failure = execute_inbound_filter(
        response,
        &network,
        PartialWriter {
            fixture: &fixture,
            stream: &server,
            fail_on: usize::MAX,
            calls: 0,
            invalidate: true,
        },
    )
    .await
    .expect_err("invalidated full write");
    // Assert
    assert_eq!(failure.progress.messages, 1);
    assert_eq!(
        failure.progress.bytes,
        receive_frame(&client).await.len() as u64
    );
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("retired suffix"),
        FilterTransportSnapshot::default()
    );
    drop(network);
    fixture.cleanup();
}

struct BlockedWriter<'a> {
    fixture: &'a FilterListenerFixture,
    entered: &'a tokio::sync::Notify,
}
impl InboundFilterWriter for BlockedWriter<'_> {
    async fn write(&mut self, _bytes: &[u8]) -> io::Result<WriteWireMessageOutcome> {
        let guard = self
            .fixture
            .context
            .try_lock()
            .expect("independent context acquisition during pending write");
        self.fixture
            .runtime
            .network_handle()
            .network_info()
            .expect("independent authority acquisition during pending write");
        drop(guard);
        self.entered.notify_one();
        std::future::pending().await
    }
}

#[tokio::test]
async fn phase160_inbound_executor_cancelled_blocked_write_retains_charge_until_drop() {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let response = fixture.prepared().await;
    let network = fixture.runtime.network_handle();
    let entered = tokio::sync::Notify::new();
    // Act
    {
        let execution = execute_inbound_filter(
            response,
            &network,
            BlockedWriter {
                fixture: &fixture,
                entered: &entered,
            },
        );
        tokio::pin!(execution);
        tokio::select! { result = &mut execution => panic!("unexpected completion: {result:?}"), () = entered.notified() => {} }
        let before = network
            .basic_filter_transport_snapshot()
            .expect("live handed owner");
        assert_eq!(before.requests, 1);
        assert!(before.active_frame_bytes > 0);
        network
            .configure_basic_peer_serving(false)
            .expect("revoke blocked response");
        let revoked = network
            .basic_filter_transport_snapshot()
            .expect("live handed bytes remain charged");
        assert_eq!(revoked.active_frame_bytes, before.active_frame_bytes);
        assert_eq!(revoked.requests, 1);
        assert_eq!(revoked.suffix_bytes, revoked.active_frame_bytes);
        assert!(fixture.context.try_lock().is_ok());
        network
            .configure_basic_peer_serving(true)
            .expect("new grant");
        let refused = fixture
            .context
            .lock()
            .await
            .prepare_inbound_wire_message(
                160_009,
                WireNetworkMessage::GetCFilters(GetCFilters {
                    filter_type: 0,
                    start_height: 0,
                    stop_hash: fixture.hash(2),
                }),
                3,
            )
            .expect("later request")
            .resolve();
        assert!(
            refused.responses.is_empty(),
            "handed old owner still consumes the peer slot"
        );
    }
    // Assert
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("actual owner drop frees quota"),
        FilterTransportSnapshot::default()
    );
    drop(network);
    fixture.cleanup();
}

#[tokio::test]
async fn phase160_inbound_executor_local_queue_refusal_releases_entire_filter_suffix() {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let response = fixture.prepared().await;
    let network = fixture.runtime.network_handle();
    let (_client, server) = tcp_pair().await;
    let policy = ResourceGovernancePolicy::default();
    let mut queue = RuntimeQueuePressureState::default();
    queue.record_aggregate_queued_messages(policy.max_aggregate_queued_messages);
    let shutdown = tokio::sync::Notify::new();
    // Act
    let failure = execute_inbound_filter(
        response,
        &network,
        crate::inbound_listener::connection_runtime::filter_serving::SocketFilterWriter {
            stream: &server,
            policy: &policy,
            connected: 1,
            activity: 1,
            handshake: InboundHandshakeState::Established,
            queue: &mut queue,
            active: Vec::new(),
            inactive: Vec::new(),
            shutdown: &shutdown,
        },
    )
    .await
    .expect_err("real socket executor queue rejection");
    // Assert
    assert!(matches!(failure.cause, FilterWriteFailure::Rejected(_)));
    assert_eq!(failure.progress.messages, 0);
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("all retired"),
        FilterTransportSnapshot::default()
    );
    drop(network);
    fixture.cleanup();
}

fn fill_socket_send_buffer(stream: &TcpStream) {
    let padding = [0_u8; 65_536];
    for _ in 0..256 {
        match stream.try_write(&padding) {
            Ok(0) => panic!("unexpected write zero"),
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => return,
            Err(error) => panic!("fill socket: {error}"),
        }
    }
    panic!("bounded loopback send buffer did not fill");
}

#[tokio::test]
async fn phase160_inbound_transport_expired_socket_deadline_aborts_all_unwritten_frames() {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let response = fixture.prepared().await;
    let network = fixture.runtime.network_handle();
    let (_client, server) = tcp_pair().await;
    let policy = ResourceGovernancePolicy {
        idle_peer_timeout_seconds: 0,
        ..Default::default()
    };
    let mut queue = RuntimeQueuePressureState::default();
    let shutdown = tokio::sync::Notify::new();
    // Act
    let failure = execute_inbound_filter(
        response,
        &network,
        crate::inbound_listener::connection_runtime::filter_serving::SocketFilterWriter {
            stream: &server,
            policy: &policy,
            connected: 1,
            activity: 1,
            handshake: InboundHandshakeState::Established,
            queue: &mut queue,
            active: Vec::new(),
            inactive: Vec::new(),
            shutdown: &shutdown,
        },
    )
    .await
    .expect_err("expired actual socket executor deadline");
    // Assert
    assert!(
        matches!(failure.cause, FilterWriteFailure::Rejected(ref event) if event.label == "idle_peer")
    );
    assert_eq!(failure.progress.messages, 0);
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("retired all suffix"),
        FilterTransportSnapshot::default()
    );
    drop(network);
    fixture.cleanup();
}

#[test]
fn phase160_inbound_executor_filter_exception_keeps_generic_local_limit() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut queue = RuntimeQueuePressureState::default();
    queue.record_pending_write(PHASE94_MAX_PEER_WRITE_QUEUE_BYTES + 1);
    // Act / Assert
    assert!(queue_pressure_event(&policy, &queue, Vec::new(), Vec::new()).is_some());
    assert!(
        crate::inbound_listener::resource_runtime::filter_queue_pressure_event(
            &policy,
            &queue,
            Vec::new(),
            Vec::new()
        )
        .is_none()
    );
    queue.record_aggregate_queued_messages(policy.max_aggregate_queued_messages);
    assert!(
        crate::inbound_listener::resource_runtime::filter_queue_pressure_event(
            &policy,
            &queue,
            Vec::new(),
            Vec::new()
        )
        .is_some()
    );
}

#[tokio::test]
async fn phase160_inbound_executor_actual_socket_disconnect_releases_suffix() {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let response = fixture.prepared().await;
    let network = fixture.runtime.network_handle();
    let (client, server) = tcp_pair().await;
    server.writable().await.expect("initial socket readiness");
    fill_socket_send_buffer(&server);
    drop(client);
    server.readable().await.expect("actual peer close ready");
    let mut probe = [0; 1];
    let read = server.try_read(&mut probe);
    assert!(read.is_err() || matches!(read, Ok(0)));
    let policy = ResourceGovernancePolicy::default();
    let mut queue = RuntimeQueuePressureState::default();
    let shutdown = tokio::sync::Notify::new();
    // Act
    let failure = execute_inbound_filter(
        response,
        &network,
        crate::inbound_listener::connection_runtime::filter_serving::SocketFilterWriter {
            stream: &server,
            policy: &policy,
            connected: 1,
            activity: 1,
            handshake: InboundHandshakeState::Established,
            queue: &mut queue,
            active: Vec::new(),
            inactive: Vec::new(),
            shutdown: &shutdown,
        },
    )
    .await
    .expect_err("real closed socket");
    // Assert
    assert!(matches!(failure.cause, FilterWriteFailure::Write(_)));
    assert_eq!(failure.progress.messages, 0);
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("all suffix retired"),
        FilterTransportSnapshot::default()
    );
    drop(network);
    fixture.cleanup();
}

#[tokio::test]
async fn phase160_inbound_executor_generic_suffix_guards_survive_until_batch_drop() {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let network = fixture.runtime.network_handle();
    let responses = fixture
        .context
        .lock()
        .await
        .encode_wire_responses(vec![
            WireNetworkMessage::Verack,
            WireNetworkMessage::Ping { nonce: 1 },
            WireNetworkMessage::Pong { nonce: 1 },
        ])
        .expect("actual shared guarded encoder");
    let bytes: u64 = responses
        .iter()
        .map(|response| response.bytes.bytes().len() as u64)
        .sum();
    let evidence = Arc::new(std::sync::Mutex::new(listener_evidence(&[])));
    // Act
    let mut batch =
        crate::inbound_listener::connection_runtime::filter_serving::InboundResponseBatch {
            steps: responses
                .into_iter()
                .map(|response| InboundWireStep::Encoded(Box::new(response)))
                .collect(),
            network: &network,
            evidence: &evidence,
        };
    let InboundWireStep::Encoded(response) = batch.steps.pop_front().expect("first owned buffer")
    else {
        panic!("ordinary step");
    };
    let response =
        crate::inbound_listener::connection_runtime::filter_serving::GuardedEncodedResponse {
            response,
            network: &network,
            evidence: &evidence,
        };
    // Assert
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("same configured ledger")
            .generic_pending_bytes,
        bytes
    );
    drop(batch);
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("handed buffer remains")
            .generic_pending_bytes,
        response.bytes.bytes().len() as u64
    );
    drop(response);
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("all actual buffers dropped"),
        FilterTransportSnapshot::default()
    );
    drop(network);
    fixture.cleanup();
}

#[tokio::test]
async fn phase160_inbound_executor_generic_after_cf_obeys_shared_scratch_peak() {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let mut response = fixture.prepared().await;
    let network = fixture.runtime.network_handle();
    let mut baseline = phase123_mined_block();
    baseline.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![1; 400_000]]);
    let baseline = network
        .encode_pending_network_message(160_010, &WireNetworkMessage::Block(baseline))
        .expect("real pending generic allocation");
    let frame = response
        .next_frame()
        .expect("atomic CF admission with existing generic")
        .expect("first frame");
    let before = network
        .basic_filter_transport_snapshot()
        .expect("mixed active classes");
    let mut large = phase123_mined_block();
    large.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![1; 3_999_700]]);
    // Act
    let refused =
        network.encode_pending_network_message(160_011, &WireNetworkMessage::Block(large));
    // Assert
    assert!(matches!(
        refused,
        Err(open_bitcoin_node::network::filter_transport::FilterTransportError::Capacity(_))
    ));
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("no untracked fallback"),
        before
    );
    assert_eq!(before.generic_pending_bytes, baseline.bytes().len() as u64);
    assert_eq!(before.active_frame_bytes, frame.bytes().len() as u64);
    drop(frame);
    drop(response);
    drop(baseline);
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("actual allocations freed"),
        FilterTransportSnapshot::default()
    );
    drop(network);
    fixture.cleanup();
}
