// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use open_bitcoin_core::primitives::{BlockHash, NetworkMagic};
use open_bitcoin_network::WireNetworkMessage;

use super::*;

#[derive(Debug)]
struct ExactFailingSendSession {
    send_calls: usize,
    maybe_fail_on_call: Option<usize>,
}

impl ExactFailingSendSession {
    fn succeeding() -> Self {
        Self {
            send_calls: 0,
            maybe_fail_on_call: None,
        }
    }

    fn failing_on_call(call: usize) -> Self {
        Self {
            send_calls: 0,
            maybe_fail_on_call: Some(call),
        }
    }
}

impl SyncPeerSession for ExactFailingSendSession {
    fn send(
        &mut self,
        _message: &WireNetworkMessage,
        _magic: NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.send_calls = self.send_calls.saturating_add(1);
        if self.maybe_fail_on_call == Some(self.send_calls) {
            return Err(SyncRuntimeError::Io {
                peer: "phase123-scripted-peer".to_string(),
                message: format!("scripted send failure on call {}", self.send_calls),
            });
        }

        Ok(())
    }

    fn receive(
        &mut self,
        _magic: NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        Ok(SyncPeerReceiveOutcome::Closed)
    }
}

fn block_message(previous_hash_byte: u8, height: u32) -> WireNetworkMessage {
    WireNetworkMessage::Block(build_block(
        BlockHash::from_byte_array([previous_hash_byte; 32]),
        height,
    ))
}

fn runtime_for_write_evidence(test_name: &str) -> (DurableSyncRuntime, PathBuf) {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    (runtime, path)
}

fn filter_response_for_evidence() -> (
    super::filter_index::peer_fixtures::PeerFilterFixture,
    Box<crate::network::filter_transport::PreparedFilterResponse>,
) {
    let fixture = super::filter_index::peer_fixtures::PeerFilterFixture::new(16);
    let mut plans = fixture
        .runtime
        .network
        .receive_sync_message(
            1,
            WireNetworkMessage::GetCFilters(open_bitcoin_network::GetCFilters {
                filter_type: 0,
                start_height: 1,
                stop_hash: fixture.hash(2),
            }),
            50_000,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("prepare")
        .inbound_response_plan;
    let crate::network::ManagedInboundResponsePlanItem::PreparedFilterServe(response) =
        plans.pop().expect("plan")
    else {
        panic!("prepared")
    };
    (fixture, response)
}

#[test]
fn phase160_outbound_executor_complete_tcp_range_earns_exact_current_evidence() {
    // Arrange
    use std::io::Read;
    let (fixture, response) = filter_response_for_evidence();
    let (mut session, mut socket, _) = super::peer_filter_transport::loopback();
    // Act
    let (result, completion) = super::super::session::filter_serving::send_response(
        &fixture.runtime.network,
        &mut session,
        1,
        fixture.runtime.config.network.magic(),
        *response,
        &mut || false,
    );
    drop(session);
    let mut bytes = Vec::new();
    socket.read_to_end(&mut bytes).expect("complete range");
    // Assert
    assert!(result.is_ok());
    let completion = completion.expect("factual completion");
    assert_eq!(
        completion.disposition,
        crate::network::filter_transport::FilterTransportDisposition::CurrentRangeComplete
    );
    assert_eq!(completion.progress.messages, 2);
    assert_eq!(completion.progress.bytes, bytes.len() as u64);
    assert_eq!(
        fixture
            .runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        crate::network::filter_transport::FilterTransportSnapshot::default()
    );
    fixture.cleanup();
}

#[test]
fn phase160_outbound_executor_wrong_target_cannot_write_or_earn_receipt() {
    // Arrange
    use std::io::Read;
    let (fixture, response) = filter_response_for_evidence();
    let (mut session, mut socket, _) = super::peer_filter_transport::loopback();
    // Act
    let (result, completion) = super::super::session::filter_serving::send_response(
        &fixture.runtime.network,
        &mut session,
        2,
        fixture.runtime.config.network.magic(),
        *response,
        &mut || false,
    );
    drop(session);
    let mut bytes = Vec::new();
    socket.read_to_end(&mut bytes).expect("no response");
    // Assert
    assert!(result.is_err());
    assert!(bytes.is_empty());
    let completion = completion.expect("factual completion");
    assert_eq!(
        completion.disposition,
        crate::network::filter_transport::FilterTransportDisposition::Aborted
    );
    assert_eq!(completion.progress.messages, 0);
    assert_eq!(completion.progress.bytes, 0);
    assert_eq!(
        fixture
            .runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        crate::network::filter_transport::FilterTransportSnapshot::default()
    );
    fixture.cleanup();
}

#[test]
fn phase160_outbound_exact_send_tcp_preserves_supplied_bytes() {
    // Arrange
    use std::io::Read;
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("listener");
    let endpoint = listener.local_addr().expect("address");
    let peer = ResolvedSyncPeerAddress {
        peer: SyncPeerAddress::manual("127.0.0.1", endpoint.port()),
        endpoint,
    };
    let mut transport = TcpPeerTransport;
    let mut session = transport.connect(&peer, &sync_config()).expect("connect");
    let (mut socket, _) = listener.accept().expect("accept");
    socket
        .set_read_timeout(Some(std::time::Duration::from_secs(2)))
        .expect("bounded read");
    // Deliberately not a decodable message: the physical seam must write this exact slice.
    let supplied = [0x51u8; 37];

    // Act
    session
        .send_encoded(&supplied, sync_config().network.magic())
        .expect("write");
    let mut received = [0u8; 37];
    socket.read_exact(&mut received).expect("read");

    // Assert
    assert_eq!(received, supplied);
}

#[test]
fn phase160_outbound_executor_disconnect_during_write_keeps_physical_only_prefix() {
    // Arrange
    use crate::network::{
        ManagedInboundResponsePlanItem as Item,
        filter_transport::{FilterTransportDisposition, FilterTransportSnapshot},
    };
    use std::io::{Read, Write};
    let fixture = super::filter_index::peer_fixtures::PeerFilterFixture::new(16);
    let mut plans = fixture
        .runtime
        .network
        .receive_sync_message(
            1,
            WireNetworkMessage::GetCFilters(open_bitcoin_network::GetCFilters {
                filter_type: 0,
                start_height: 1,
                stop_hash: fixture.hash(2),
            }),
            50_000,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("prepare")
        .inbound_response_plan;
    let Item::PreparedFilterServe(response) = plans.pop().expect("plan") else {
        panic!("prepared")
    };
    let (mut session, mut socket, _) = super::peer_filter_transport::loopback();
    let handle = fixture.runtime.network.clone();
    session.control_writes(move |stream, bytes| {
        handle
            .disconnect_peer(1)
            .expect("disconnect during blocked write");
        let retained = handle
            .basic_filter_transport_snapshot()
            .expect("handed owner");
        assert_eq!(
            (
                retained.requests,
                retained.suffix_bytes,
                retained.active_frame_bytes
            ),
            (1, bytes.len() as u64, bytes.len() as u64)
        );
        stream.write_all(bytes)
    });
    // Act
    let (result, completion) = super::super::session::filter_serving::send_response(
        &fixture.runtime.network,
        &mut session,
        1,
        fixture.runtime.config.network.magic(),
        *response,
        &mut || false,
    );
    drop(session);
    let mut bytes = Vec::new();
    socket.read_to_end(&mut bytes).expect("physical prefix");
    // Assert
    assert!(result.is_err());
    let completion = completion.expect("factual completion");
    assert_eq!(
        completion.disposition,
        FilterTransportDisposition::PhysicalPrefixInvalidated
    );
    assert_eq!(completion.progress.messages, 1);
    assert_eq!(completion.progress.bytes, bytes.len() as u64);
    assert_eq!(
        fixture
            .runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        FilterTransportSnapshot::default()
    );
    fixture.cleanup();
}

#[test]
fn phase160_outbound_executor_generic_admission_failure_drops_entire_encoded_batch() {
    // Arrange
    use std::io::Read;
    let (mut runtime, path) = runtime_for_write_evidence("phase160-batch-admission");
    let (mut session, mut socket, _) = super::peer_filter_transport::loopback();
    let messages = vec![WireNetworkMessage::Ping { nonce: 1 }; 129];
    // Act
    let result = runtime.send_all(&mut session, 1, &messages);
    drop(session);
    let mut bytes = Vec::new();
    socket.read_to_end(&mut bytes).expect("no output");
    // Assert
    assert!(result.is_err());
    assert!(bytes.is_empty());
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        crate::network::filter_transport::FilterTransportSnapshot::default()
    );
    drop(runtime);
    remove_dir_if_exists(&path);
}

#[test]
fn phase160_outbound_executor_generic_cancel_drops_encoded_suffix() {
    // Arrange
    use std::io::{Read, Write};
    use std::sync::atomic::{AtomicBool, Ordering};
    let (mut runtime, path) = runtime_for_write_evidence("phase160-generic-cancel");
    let (mut session, mut socket, _) = super::peer_filter_transport::loopback();
    let cancelled = Arc::new(AtomicBool::new(false));
    let writer_cancelled = Arc::clone(&cancelled);
    let handle = runtime.network.clone();
    session.control_writes(move |stream, bytes| {
        let retained = handle
            .basic_filter_transport_snapshot()
            .expect("queued owners");
        assert_eq!(
            (retained.generic_descriptors, retained.generic_pending_bytes),
            (2, 64)
        );
        writer_cancelled.store(true, Ordering::Release);
        stream.write_all(bytes)
    });
    // Act
    let result = runtime.send_all_for_peer_with_cancel(
        &mut session,
        1,
        &[
            WireNetworkMessage::Ping { nonce: 1 },
            WireNetworkMessage::Ping { nonce: 2 },
        ],
        &mut || cancelled.load(Ordering::Acquire),
    );
    drop(session);
    let mut bytes = Vec::new();
    socket.read_to_end(&mut bytes).expect("physical prefix");
    // Assert
    assert!(result.is_ok());
    assert_eq!(bytes.len(), 32);
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        crate::network::filter_transport::FilterTransportSnapshot::default()
    );
    drop(runtime);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_block_write_success_increments_served_once() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-block-write-success");
    let mut session = ExactFailingSendSession::succeeding();
    let messages = [block_message(1, 1)];

    // Act
    let result = runtime.send_all(&mut session, 1, &messages);

    // Assert
    assert!(result.is_ok());
    assert_eq!(
        runtime
            .network
            .block_served_write_count()
            .expect("authoritative block write count"),
        1
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_block_write_failure_does_not_increment_served() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-block-write-failure");
    let mut session = ExactFailingSendSession::failing_on_call(1);
    let messages = [block_message(2, 1)];

    // Act
    let result = runtime.send_all(&mut session, 1, &messages);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert_eq!(
        runtime
            .network
            .block_served_write_count()
            .expect("authoritative block write count"),
        0
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_non_block_write_does_not_increment_served() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-non-block-write");
    let mut session = ExactFailingSendSession::succeeding();
    let messages = [WireNetworkMessage::Ping { nonce: 123 }];

    // Act
    let result = runtime.send_all(&mut session, 1, &messages);

    // Assert
    assert!(result.is_ok());
    assert_eq!(
        runtime
            .network
            .block_served_write_count()
            .expect("authoritative block write count"),
        0
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_partial_batch_counts_each_successful_block_before_failure() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-partial-batch");
    let mut session = ExactFailingSendSession::failing_on_call(3);
    let messages = [
        block_message(3, 1),
        WireNetworkMessage::Ping { nonce: 123 },
        block_message(4, 2),
    ];

    // Act
    let result = runtime.send_all(&mut session, 1, &messages);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert_eq!(
        runtime
            .network
            .block_served_write_count()
            .expect("authoritative block write count"),
        1
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_two_successful_blocks_before_later_failure_count_two() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-two-block-prefix");
    let mut session = ExactFailingSendSession::failing_on_call(3);
    let messages = [
        block_message(5, 1),
        block_message(6, 2),
        WireNetworkMessage::Ping { nonce: 123 },
    ];

    // Act
    let result = runtime.send_all(&mut session, 1, &messages);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert_eq!(
        runtime
            .network
            .block_served_write_count()
            .expect("authoritative block write count"),
        2
    );
    remove_dir_if_exists(&path);
}
