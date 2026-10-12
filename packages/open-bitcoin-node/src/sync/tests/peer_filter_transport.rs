// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Concrete loopback transport evidence through the outbound production caller.

mod phase162;
mod wire_goldens;

use super::filter_index::peer_fixtures::PeerFilterFixture;
use super::*;
use crate::network::{
    ManagedInboundResponsePlanItem as Item, filter_transport::FilterTransportSnapshot,
};
use open_bitcoin_network::{
    GetCFCheckpt, GetCFHeaders, GetCFilters, ParsedNetworkMessage, ServiceFlags,
};
use std::{
    io::{Read, Write},
    time::Duration,
};

use super::support_transport::fill_pending;
pub(super) use super::support_transport::loopback;

fn filter_request(stop: BlockHash) -> WireNetworkMessage {
    WireNetworkMessage::GetCFilters(GetCFilters {
        filter_type: 0,
        start_height: 1,
        stop_hash: stop,
    })
}

fn prepare(fixture: &PeerFilterFixture) -> Vec<Item> {
    fixture
        .runtime
        .network
        .receive_sync_message(
            1,
            filter_request(fixture.hash(2)),
            50_000,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("receive")
        .inbound_response_plan
}

fn execute(
    fixture: &PeerFilterFixture,
    session: &mut impl SyncPeerSession,
    plans: Vec<Item>,
) -> Result<(), SyncRuntimeError> {
    super::super::session::filter_serving::send_filter_plans(
        &fixture.runtime.network,
        session,
        1,
        fixture.runtime.config.network.magic(),
        plans,
        &mut || false,
    )
}

#[test]
fn phase160_outbound_transport_generic_first_pressure_refuses_filter_before_socket_write() {
    // Arrange
    let fixture = PeerFilterFixture::large();
    let request = WireNetworkMessage::GetCFilters(GetCFilters {
        filter_type: 0,
        start_height: 16,
        stop_hash: fixture.hash(16),
    });
    let plans = fixture
        .runtime
        .network
        .receive_sync_message(
            1,
            request,
            50_000,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("prepared")
        .inbound_response_plan;
    let buffers = fill_pending(&fixture.runtime.network);
    let retained = fixture
        .runtime
        .network
        .basic_filter_transport_snapshot()
        .expect("pending")
        .generic_pending_bytes;
    let (mut session, mut socket, _) = loopback();
    // Act
    let result = execute(&fixture, &mut session, plans);
    drop(session);
    let mut bytes = Vec::new();
    socket
        .read_to_end(&mut bytes)
        .expect("no physical response");
    // Assert
    assert!(result.is_err());
    assert!(bytes.is_empty());
    let snapshot = fixture
        .runtime
        .network
        .basic_filter_transport_snapshot()
        .expect("generic still retained");
    assert_eq!(snapshot.generic_pending_bytes, retained);
    assert_eq!(
        (
            snapshot.requests,
            snapshot.active_frame_bytes,
            snapshot.suffix_bytes
        ),
        (0, 0, 0)
    );
    drop(buffers);
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
fn phase160_outbound_transport_filter_first_write_shares_generic_peak_admission() {
    // Arrange
    let fixture = PeerFilterFixture::large();
    let plans = fixture
        .runtime
        .network
        .receive_sync_message(
            1,
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 16,
                stop_hash: fixture.hash(16),
            }),
            50_000,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("prepared")
        .inbound_response_plan;
    let (mut session, mut socket, _) = loopback();
    let handle = fixture.runtime.network.clone();
    session.control_writes(move |stream, bytes| {
        let buffers = fill_pending(&handle);
        let snapshot = handle
            .basic_filter_transport_snapshot()
            .expect("mixed peak");
        assert_eq!(snapshot.active_frame_bytes, bytes.len() as u64);
        assert!(snapshot.generic_pending_bytes + snapshot.active_frame_bytes <= 8 * 1024 * 1024);
        assert!(snapshot.generic_pending_bytes > 7 * 1024 * 1024);
        drop(buffers);
        assert_eq!(
            handle
                .basic_filter_transport_snapshot()
                .expect("CF retained")
                .active_frame_bytes,
            bytes.len() as u64
        );
        stream.write_all(bytes)
    });
    // Act
    let result = execute(&fixture, &mut session, plans);
    drop(session);
    let mut bytes = Vec::new();
    socket
        .read_to_end(&mut bytes)
        .expect("actual completed filter");
    // Assert
    assert!(result.is_ok());
    assert!(matches!(
        ParsedNetworkMessage::decode_wire(&bytes)
            .expect("exact filter frame")
            .message,
        WireNetworkMessage::CFilter(_)
    ));
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
fn phase160_outbound_transport_actual_caller_all_families() {
    // Arrange
    let mut fixture = PeerFilterFixture::lagging(200, 8);
    fixture
        .runtime
        .network
        .set_prune_mode(open_bitcoin_core::chainstate::PruneMode::ManualOnly)
        .expect("prune mode");
    let index = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("progress")
        .expect("index enabled");
    assert!(
        index.current_lag() > 0,
        "authoritative lag before actual version send"
    );
    let (session, mut socket, peer) = loopback();
    fixture.runtime.config.max_messages_per_peer = 5;
    let magic = fixture.runtime.config.network.magic();
    let expected_frames = super::support_transport::expected_filter_frames(&fixture, magic);
    let requests = [
        WireNetworkMessage::Version(VersionMessage {
            nonce: 999,
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
        filter_request(fixture.hash(2)),
        WireNetworkMessage::GetCFHeaders(GetCFHeaders {
            filter_type: 0,
            start_height: 1,
            stop_hash: fixture.hash(2),
        }),
        WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
            filter_type: 0,
            stop_hash: fixture.hash(2),
        }),
    ];
    let remote = std::thread::spawn(move || {
        for request in requests {
            socket
                .write_all(&request.encode_wire(magic).expect("request"))
                .expect("request write");
        }
        let mut bytes = Vec::new();
        socket.read_to_end(&mut bytes).expect("bounded peer output");
        bytes
    });
    // Act
    let result = fixture
        .runtime
        .sync_connected_peer(session, &peer, 2, 1, 50_000, &mut || 50_001);
    let bytes = remote.join().expect("remote");
    // Assert
    assert!(result.is_ok(), "{result:?}");
    let mut messages = Vec::new();
    let mut filter_frames = Vec::new();
    let mut remainder = bytes.as_slice();
    while !remainder.is_empty() {
        let header =
            open_bitcoin_core::codec::parse_message_header(&remainder[..24]).expect("header");
        let size = 24 + header.payload_size as usize;
        let message = ParsedNetworkMessage::decode_wire(&remainder[..size])
            .expect("frame")
            .message;
        if matches!(
            message,
            WireNetworkMessage::CFilter(_)
                | WireNetworkMessage::CFHeaders(_)
                | WireNetworkMessage::CFCheckpt(_)
        ) {
            filter_frames.push(remainder[..size].to_vec());
        }
        messages.push(message);
        remainder = &remainder[size..];
    }
    assert!(messages.iter().any(|message| matches!(message, WireNetworkMessage::Version(version) if version.services.contains(ServiceFlags::COMPACT_FILTERS) && version.services.contains(ServiceFlags::NETWORK_LIMITED) && !version.services.contains(ServiceFlags::NETWORK))));
    let filters: Vec<_> = messages
        .iter()
        .filter_map(|message| match message {
            WireNetworkMessage::CFilter(filter) => Some(filter),
            _ => None,
        })
        .collect();
    assert_eq!(filters.len(), 2);
    assert_eq!(filters[0].block_hash, fixture.hash(1));
    assert_eq!(filters[1].block_hash, fixture.hash(2));
    assert_eq!(
        filter_frames, expected_frames,
        "same persisted responses and exact framing"
    );
    assert!(messages.iter().any(|message| matches!(message, WireNetworkMessage::CFHeaders(headers) if headers.filter_hashes.len() == 2)));
    assert!(messages.iter().any(|message| matches!(message, WireNetworkMessage::CFCheckpt(checkpoint) if checkpoint.filter_headers.is_empty())));
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
fn phase160_outbound_transport_partial_first_frame_no_complete_credit() {
    failed_frame_case(1);
}

#[test]
fn phase160_outbound_transport_partial_second_frame_keeps_prior_prefix() {
    failed_frame_case(2);
}

fn failed_frame_case(fail_on: usize) {
    // Arrange
    let fixture = PeerFilterFixture::new(16);
    let (mut session, mut socket, _) = loopback();
    let plans = prepare(&fixture);
    let mut calls = 0usize;
    let expected_first_len = match &plans[0] {
        Item::PreparedFilterServe(response) => response.progress().expect("prefix").bytes,
        _ => panic!("prepared"),
    };
    assert_eq!(expected_first_len, 0);
    session.control_writes(move |stream, bytes| {
        calls += 1;
        if calls == fail_on {
            stream.write_all(&bytes[..7])?;
            return Err(io::Error::other("injected partial socket write"));
        }
        stream.write_all(bytes)
    });
    // Act
    let Item::PreparedFilterServe(response) = plans.into_iter().next().expect("plan") else {
        panic!("prepared")
    };
    let (result, prefix) = super::super::session::filter_serving::send_response(
        &fixture.runtime.network,
        &mut session,
        1,
        fixture.runtime.config.network.magic(),
        *response,
        &mut || false,
    );
    drop(session);
    let mut bytes = Vec::new();
    socket
        .read_to_end(&mut bytes)
        .expect("actual physical prefix");
    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert_eq!(
        prefix.expect("truthful prefix").progress.messages,
        fail_on as u64 - 1
    );
    assert_eq!(
        prefix.expect("truthful bytes").progress.bytes,
        bytes.len() as u64 - 7
    );
    if fail_on == 1 {
        assert_eq!(bytes.len(), 7);
    } else {
        let header =
            open_bitcoin_core::codec::parse_message_header(&bytes[..24]).expect("first full frame");
        assert_eq!(bytes.len(), 24 + header.payload_size as usize + 7);
    }
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
fn phase160_outbound_executor_lifecycle_during_write_retains_handed_owner() {
    // Arrange
    let fixture = PeerFilterFixture::new(16);
    let plans = prepare(&fixture);
    let (mut session, mut socket, _) = loopback();
    let handle = fixture.runtime.network.clone();
    let stop = fixture.hash(2);
    let params = fixture.runtime.consensus_params;
    session.control_writes(move |stream, bytes| {
        let handle = handle.clone();
        let frame_len = bytes.len() as u64;
        // The writer is blocked on this worker, which must acquire shared authority.
        let (sender, receiver) = std::sync::mpsc::channel();
        let worker = std::thread::spawn(move || {
            let before = handle.basic_filter_transport_snapshot().expect("handed");
            assert_eq!(before.active_frame_bytes, frame_len);
            assert!(before.suffix_bytes >= frame_len);
            handle
                .configure_basic_peer_serving(false)
                .expect("invalidate while writer blocked");
            let during = handle.basic_filter_transport_snapshot().expect("retained");
            assert_eq!(
                (
                    during.requests,
                    during.suffix_bytes,
                    during.active_frame_bytes
                ),
                (1, frame_len, frame_len)
            );
            handle
                .configure_basic_peer_serving(true)
                .expect("restore grant");
            let retry = handle
                .prepare_basic_filter_response(
                    1,
                    open_bitcoin_network::policy::FilterRequest::Filters {
                        filter_type: 0,
                        start_height: 1,
                        stop_hash: stop,
                    },
                    params,
                )
                .expect("reentry");
            assert!(matches!(
                retry,
                crate::network::filter_transport::BasicFilterPrepareOutcome::Unavailable(_)
            ));
            sender.send(()).expect("release writer");
        });
        receiver
            .recv_timeout(Duration::from_secs(2))
            .expect("authority lock must be available");
        worker.join().expect("worker");
        stream.write_all(bytes)
    });
    let Item::PreparedFilterServe(response) = plans.into_iter().next().expect("plan") else {
        panic!("prepared")
    };
    // Act
    let (result, prefix) = super::super::session::filter_serving::send_response(
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
    assert_eq!(prefix.expect("prefix").progress.messages, 1);
    assert_eq!(prefix.expect("prefix").progress.bytes, bytes.len() as u64);
    assert_eq!(
        prefix.expect("prefix").disposition,
        crate::network::filter_transport::FilterTransportDisposition::PhysicalPrefixInvalidated
    );
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
fn phase160_outbound_executor_cancel_during_write_earns_only_physical_prefix() {
    // Arrange
    use std::sync::atomic::{AtomicBool, Ordering};
    let fixture = PeerFilterFixture::new(16);
    let plans = prepare(&fixture);
    let (mut session, mut socket, _) = loopback();
    let cancelled = Arc::new(AtomicBool::new(false));
    let writer_cancelled = Arc::clone(&cancelled);
    session.control_writes(move |stream, bytes| {
        writer_cancelled.store(true, Ordering::Release);
        stream.write_all(bytes)
    });
    let Item::PreparedFilterServe(response) = plans.into_iter().next().expect("plan") else {
        panic!("prepared")
    };
    // Act
    let (result, prefix) = super::super::session::filter_serving::send_response(
        &fixture.runtime.network,
        &mut session,
        1,
        fixture.runtime.config.network.magic(),
        *response,
        &mut || cancelled.load(Ordering::Acquire),
    );
    drop(session);
    let mut bytes = Vec::new();
    socket.read_to_end(&mut bytes).expect("physical prefix");
    // Assert
    assert!(result.is_ok());
    assert_eq!(prefix.expect("prefix").progress.messages, 1);
    assert_eq!(prefix.expect("prefix").progress.bytes, bytes.len() as u64);
    assert_eq!(
        prefix.expect("prefix").disposition,
        crate::network::filter_transport::FilterTransportDisposition::PhysicalPrefixInvalidated
    );
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
fn phase160_outbound_executor_generic_batch_retains_suffix_during_real_partial_write() {
    // Arrange
    let mut fixture = PeerFilterFixture::new(16);
    let (mut session, mut socket, _) = loopback();
    let handle = fixture.runtime.network.clone();
    session.control_writes(move |stream, bytes| {
        let snapshot = handle
            .basic_filter_transport_snapshot()
            .expect("shared ledger");
        assert_eq!(
            (snapshot.generic_descriptors, snapshot.generic_pending_bytes),
            (2, 64)
        );
        assert!(
            handle.peer_manager_snapshot().is_ok(),
            "authority released before physical write"
        );
        stream.write_all(&bytes[..7])?;
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "injected physical timeout",
        ))
    });
    // Act
    let result = fixture.runtime.send_all(
        &mut session,
        1,
        &[
            WireNetworkMessage::Ping { nonce: 1 },
            WireNetworkMessage::Ping { nonce: 2 },
        ],
    );
    drop(session);
    let mut bytes = Vec::new();
    socket.read_to_end(&mut bytes).expect("actual prefix");
    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert_eq!(bytes.len(), 7);
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
fn phase160_outbound_executor_unavailable_emits_nothing_leaked_intent_errors() {
    // Arrange
    let fixture = PeerFilterFixture::new(16);
    let (mut session, mut socket, _) = loopback();
    // Act
    execute(
        &fixture,
        &mut session,
        vec![Item::FilterServeUnavailable(
            crate::network::filter_transport::BasicFilterServeUnavailable::Missing,
        )],
    )
    .expect("no response");
    let result = execute(
        &fixture,
        &mut session,
        vec![Item::FilterRequestIntent(
            open_bitcoin_network::policy::FilterRequest::Filters {
                filter_type: 0,
                start_height: 1,
                stop_hash: fixture.hash(2),
            },
        )],
    );
    drop(session);
    let mut bytes = Vec::new();
    socket.read_to_end(&mut bytes).expect("no physical output");
    // Assert
    assert!(result.is_err());
    assert!(bytes.is_empty());
    fixture.cleanup();
}
