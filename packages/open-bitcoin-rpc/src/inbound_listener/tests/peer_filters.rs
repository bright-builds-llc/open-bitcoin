// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

use super::super::connection_runtime::filter_serving::{
    FilterWriteFailure, InboundFilterFailure, InboundFilterWriter, execute_inbound_filter,
};
use super::listener_fixtures::*;
use super::*;
use open_bitcoin_network::{
    CFCheckpt, CFHeaders, CFilter, GetCFCheckpt, GetCFHeaders, GetCFilters, ServiceFlags,
};
use open_bitcoin_node::network::filter_transport::FilterTransportSnapshot;

async fn receive_frame(stream: &TcpStream) -> Vec<u8> {
    async fn exact(stream: &TcpStream, buffer: &mut [u8]) {
        let mut offset = 0;
        while offset < buffer.len() {
            stream.readable().await.expect("readable socket");
            match stream.try_read(&mut buffer[offset..]) {
                Ok(0) => panic!("closed incomplete frame"),
                Ok(read) => offset += read,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("socket read: {error}"),
            }
        }
    }
    let mut bytes = vec![0; 24];
    exact(stream, &mut bytes).await;
    let len = parse_message_header(&bytes)
        .expect("frame header")
        .payload_size as usize;
    assert!(len <= 4_000_000);
    bytes.resize(24 + len, 0);
    exact(stream, &mut bytes[24..]).await;
    bytes
}

async fn handshake(stream: &TcpStream) -> ServiceFlags {
    send_regtest(
        stream,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 160_901,
            ..Default::default()
        }),
    )
    .await;
    let bytes = receive_frame(stream).await;
    let WireNetworkMessage::Version(version) = ParsedNetworkMessage::decode_wire(&bytes)
        .expect("actual version")
        .message
    else {
        panic!("version first");
    };
    loop {
        let ack = receive_frame(stream).await;
        if ParsedNetworkMessage::decode_wire(&ack)
            .expect("actual handshake frame")
            .message
            == WireNetworkMessage::Verack
        {
            break;
        }
    }
    send_regtest(stream, WireNetworkMessage::Verack).await;
    send_regtest(stream, WireNetworkMessage::Ping { nonce: 160_900 }).await;
    loop {
        let message = ParsedNetworkMessage::decode_wire(&receive_frame(stream).await)
            .expect("handshake drain")
            .message;
        if message == (WireNetworkMessage::Pong { nonce: 160_900 }) {
            break;
        }
        assert!(!matches!(
            message,
            WireNetworkMessage::CFilter(_)
                | WireNetworkMessage::CFHeaders(_)
                | WireNetworkMessage::CFCheckpt(_)
        ));
    }
    version.services
}

async fn send_regtest(stream: &TcpStream, message: WireNetworkMessage) {
    let bytes = message
        .encode_wire(SyncNetwork::Regtest.magic())
        .expect("regtest frame");
    write_all(stream, &bytes).await.expect("remote write");
}

#[tokio::test]
async fn phase160_inbound_transport_actual_listener_all_three_exact_families() {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let network = fixture.runtime.network_handle();
    let progress = network.maybe_basic_index_summary().expect("summary");
    assert!(!progress.expect("enabled index").synced);
    let (worker, endpoint) = fixture.listener().await;
    let stream = TcpStream::connect(endpoint).await.expect("actual peer");
    // Act
    let services = handshake(&stream).await;
    let pending = network
        .encode_pending_network_message(160_099, &WireNetworkMessage::Ping { nonce: 9 })
        .expect("ordinary pending buffer on the actual listener's CF ledger");
    let records: Vec<_> = (0..3)
        .map(|height| {
            match network
                .basic_filter_query(fixture.hash(height))
                .expect("persisted query")
            {
                open_bitcoin_node::network::BasicFilterQuery::Found(record) => record,
                other => panic!("indexed prefix: {other:?}"),
            }
        })
        .collect();
    send_regtest(
        &stream,
        WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type: 0,
            start_height: 0,
            stop_hash: fixture.hash(2),
        }),
    )
    .await;
    let mut actual = Vec::new();
    for _ in 0..3 {
        actual.push(receive_frame(&stream).await);
    }
    let expected: Vec<_> = records
        .iter()
        .map(|record| {
            WireNetworkMessage::CFilter(CFilter {
                filter_type: 0,
                block_hash: record.identity().block_hash(),
                filter: record.encoded_bytes().to_vec(),
            })
            .encode_wire(SyncNetwork::Regtest.magic())
            .expect("expected cfilter")
        })
        .collect();
    // Assert
    assert_eq!(actual, expected);
    assert!(services.contains(ServiceFlags::COMPACT_FILTERS));
    assert!(services.contains(ServiceFlags::NETWORK_LIMITED));
    assert!(!services.contains(ServiceFlags::NETWORK));
    for (request, expected) in [
        (
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: 1,
                stop_hash: fixture.hash(2),
            }),
            WireNetworkMessage::CFHeaders(CFHeaders {
                filter_type: 0,
                stop_hash: fixture.hash(2),
                previous_filter_header: records[0].identity().filter_header().into(),
                filter_hashes: records[1..]
                    .iter()
                    .map(|record| record.identity().filter_hash().into())
                    .collect(),
            }),
        ),
        (
            WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                filter_type: 0,
                stop_hash: fixture.hash(2),
            }),
            WireNetworkMessage::CFCheckpt(CFCheckpt {
                filter_type: 0,
                stop_hash: fixture.hash(2),
                filter_headers: Vec::new(),
            }),
        ),
    ] {
        send_regtest(&stream, request).await;
        assert_eq!(
            receive_frame(&stream).await,
            expected
                .encode_wire(SyncNetwork::Regtest.magic())
                .expect("expected exact family")
        );
    }
    drop(stream);
    worker.shutdown().await;
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("ordinary owner survives actual filter writes")
            .generic_pending_bytes,
        pending.bytes().len() as u64
    );
    drop(pending);
    assert_eq!(
        network.basic_filter_transport_snapshot().expect("settled"),
        FilterTransportSnapshot::default()
    );
    drop(network);
    fixture.cleanup();
}

#[tokio::test]
async fn phase160_inbound_transport_permission_versions_before_catchup() {
    for (tokens, expected) in [
        (vec!["in", "all"], true),
        (vec!["in", "blockfilters"], true),
        (vec!["in"], false),
    ] {
        // Arrange
        let fixture = FilterListenerFixture::new(false, &tokens);
        let (worker, endpoint) = fixture.listener().await;
        let stream = TcpStream::connect(endpoint)
            .await
            .expect("actual permissioned peer");
        // Act
        let services = handshake(&stream).await;
        // Assert
        assert_eq!(services.contains(ServiceFlags::COMPACT_FILTERS), expected);
        assert!(services.contains(ServiceFlags::NETWORK_LIMITED));
        assert!(
            !fixture
                .runtime
                .network_handle()
                .maybe_basic_index_summary()
                .expect("actual index")
                .expect("enabled")
                .synced
        );
        assert_eq!(
            fixture
                .runtime
                .network_handle()
                .network_info()
                .expect("global grant off")
                .local_services_bits
                & ServiceFlags::COMPACT_FILTERS.bits(),
            0
        );
        if !expected {
            send_regtest(
                &stream,
                WireNetworkMessage::GetCFilters(GetCFilters {
                    filter_type: 0,
                    start_height: 0,
                    stop_hash: fixture.hash(2),
                }),
            )
            .await;
            tokio::time::timeout(Duration::from_secs(1), async {
                let mut bytes = [0; 1];
                loop {
                    stream.readable().await.expect("closed socket readiness");
                    match stream.try_read(&mut bytes) {
                        Ok(0) => break,
                        Err(error) if error.kind() == io::ErrorKind::WouldBlock => continue,
                        Err(error)
                            if matches!(
                                error.kind(),
                                io::ErrorKind::ConnectionReset
                                    | io::ErrorKind::ConnectionAborted
                                    | io::ErrorKind::NotConnected
                            ) =>
                        {
                            break;
                        }
                        other => panic!(
                            "unadvertised capability must disconnect without a response: {other:?}"
                        ),
                    }
                }
            })
            .await
            .expect("unadvertised request disconnects");
        }
        drop(stream);
        worker.shutdown().await;
        fixture.cleanup();
    }
}

#[tokio::test]
async fn phase160_inbound_transport_missing_later_record_emits_no_prefix() {
    // Arrange
    let fixture = FilterListenerFixture::new(true, &["in"]);
    let (worker, endpoint) = fixture.listener().await;
    let stream = TcpStream::connect(endpoint).await.expect("actual peer");
    handshake(&stream).await;
    // Act
    send_regtest(
        &stream,
        WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type: 0,
            start_height: 0,
            stop_hash: fixture.hash(39),
        }),
    )
    .await;
    send_regtest(&stream, WireNetworkMessage::Ping { nonce: 160_999 }).await;
    let frame = receive_frame(&stream).await;
    // Assert
    assert_eq!(
        ParsedNetworkMessage::decode_wire(&frame)
            .expect("sentinel frame")
            .message,
        WireNetworkMessage::Pong { nonce: 160_999 }
    );
    drop(stream);
    worker.shutdown().await;
    assert_eq!(
        fixture
            .runtime
            .network_handle()
            .basic_filter_transport_snapshot()
            .expect("no leaked unavailable request"),
        FilterTransportSnapshot::default()
    );
    fixture.cleanup();
}

#[tokio::test]
async fn phase160_inbound_transport_all_only_no_backend_emits_no_response() {
    // Arrange
    let (context, worker, endpoint) = running_loopback_listener_with_config(
        loopback_config_with_permission_classes(2, 1, loopback_permission_registry(&["in", "all"])),
    )
    .await;
    let block = phase123_mined_block();
    context
        .lock()
        .await
        .connect_local_block(&block)
        .expect("known permitted stop without index");
    let stream = TcpStream::connect(endpoint)
        .await
        .expect("actual transient peer");
    send_message(
        &stream,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 160_902,
            ..Default::default()
        }),
    )
    .await;
    let WireNetworkMessage::Version(version) =
        ParsedNetworkMessage::decode_wire(&receive_frame(&stream).await)
            .expect("actual version")
            .message
    else {
        panic!("version");
    };
    assert!(version.services.contains(ServiceFlags::COMPACT_FILTERS));
    loop {
        if ParsedNetworkMessage::decode_wire(&receive_frame(&stream).await)
            .expect("actual handshake")
            .message
            == WireNetworkMessage::Verack
        {
            break;
        }
    }
    send_message(&stream, WireNetworkMessage::Verack).await;
    send_message(&stream, WireNetworkMessage::Ping { nonce: 160_900 }).await;
    loop {
        if ParsedNetworkMessage::decode_wire(&receive_frame(&stream).await)
            .expect("handshake drain")
            .message
            == (WireNetworkMessage::Pong { nonce: 160_900 })
        {
            break;
        }
    }
    // Act
    for request in [
        WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type: 0,
            start_height: 0,
            stop_hash: block_hash(&block.header),
        }),
        WireNetworkMessage::GetCFHeaders(GetCFHeaders {
            filter_type: 0,
            start_height: 0,
            stop_hash: block_hash(&block.header),
        }),
        WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
            filter_type: 0,
            stop_hash: block_hash(&block.header),
        }),
    ] {
        send_message(&stream, request).await;
        send_message(&stream, WireNetworkMessage::Ping { nonce: 160_998 }).await;
        // Assert
        assert_eq!(
            ParsedNetworkMessage::decode_wire(&receive_frame(&stream).await)
                .expect("sentinel")
                .message,
            WireNetworkMessage::Pong { nonce: 160_998 }
        );
    }
    drop(stream);
    worker.shutdown().await;
}

mod failures;
mod mixed;
mod phase162;

#[derive(Clone, Copy, Debug)]
enum SocketCase {
    Expired,
    Queue,
    Shutdown,
    Disconnect,
    Complete,
}
async fn phase162_generic_suffix(fixture: &FilterListenerFixture) -> Vec<InboundWireStep> {
    fixture
        .context
        .lock()
        .await
        .encode_wire_responses(vec![
            WireNetworkMessage::Ping { nonce: 1 },
            WireNetworkMessage::Pong { nonce: 1 },
        ])
        .expect("real shared pending encoders")
        .into_iter()
        .map(|response| InboundWireStep::Encoded(Box::new(response)))
        .collect()
}

fn phase162_assert_prefix(failure: &InboundFilterFailure, expected: &[Vec<u8>], complete: usize) {
    assert_eq!(failure.progress.messages, complete as u64);
    assert_eq!(
        failure.progress.bytes,
        expected[..complete].concat().len() as u64
    );
    assert!(failure.maybe_cleanup.is_none() && failure.maybe_progress_failure.is_none());
}

struct Phase162WriteCleanup(PathBuf);
impl Phase162WriteCleanup {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "open-bitcoin-phase162-inbound-write-{}-{}",
            process::id(),
            NEXT_DURABLE_BLOCK_SERVING_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(!path.exists(), "fresh owned path");
        Self(path)
    }
}
impl Drop for Phase162WriteCleanup {
    fn drop(&mut self) {
        drop(FjallNodeStore::open(&self.0).expect("all handles closed before raw reopen"));
        fs::remove_dir_all(&self.0).expect("owned datadir removal");
        assert!(!self.0.exists());
    }
}

async fn phase162_physical_bytes(stream: &TcpStream, cap: usize) -> Vec<u8> {
    tokio::time::timeout(Duration::from_secs(5), async {
        let mut result = Vec::new();
        loop {
            stream.readable().await.expect("finite remote EOF");
            let mut buffer = [0; 8192];
            match stream.try_read(&mut buffer) {
                Ok(0) => return result,
                Ok(count) => {
                    result.extend_from_slice(&buffer[..count]);
                    assert!(result.len() <= cap);
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("physical read: {error}"),
            }
        }
    })
    .await
    .expect("absolute physical read deadline")
}

async fn phase162_saturate(stream: &TcpStream) -> usize {
    stream.writable().await.expect("initial writable socket");
    let deadline = tokio::time::Instant::now() + Duration::from_secs(2);
    let mut total = 0;
    let mut blocked = 0;
    loop {
        assert!(
            tokio::time::Instant::now() < deadline && total < 16 * 1024 * 1024,
            "bounded saturation"
        );
        match stream.try_write(&[0; 65536]) {
            Ok(0) => panic!("write zero"),
            Ok(count) => {
                total += count;
                blocked = 0;
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                blocked += 1;
                if blocked == 3 {
                    return total;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            Err(error) => panic!("socket priming: {error}"),
        }
    }
}

#[tokio::test]
async fn phase160_peer_evidence_pre_version_and_pre_verack_requests_are_inert() {
    // Arrange: valid and invalid family inputs must wait for completed handshake.
    let fixture = FilterListenerFixture::new(true, &["in"]);
    for after_version in [false, true] {
        let (worker, endpoint) = fixture.listener().await;
        let stream = TcpStream::connect(endpoint)
            .await
            .expect("actual unhandshaken ingress");
        if after_version {
            send_regtest(
                &stream,
                WireNetworkMessage::Version(VersionMessage {
                    nonce: 160_114,
                    ..Default::default()
                }),
            )
            .await;
            loop {
                if ParsedNetworkMessage::decode_wire(&receive_frame(&stream).await)
                    .expect("local handshake")
                    .message
                    == WireNetworkMessage::Verack
                {
                    break;
                }
            }
        }
        // Act: all three valid and all three invalid requests remain silent.
        for filter_type in [0, 255] {
            for request in [
                WireNetworkMessage::GetCFilters(GetCFilters {
                    filter_type,
                    start_height: 0,
                    stop_hash: fixture.hash(2),
                }),
                WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                    filter_type,
                    start_height: 0,
                    stop_hash: fixture.hash(2),
                }),
                WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                    filter_type,
                    stop_hash: fixture.hash(2),
                }),
            ] {
                send_regtest(&stream, request).await;
            }
        }
        // Complete handshake and establish a later physical sentinel to prove the peer survives.
        if !after_version {
            send_regtest(
                &stream,
                WireNetworkMessage::Version(VersionMessage {
                    nonce: 160_115,
                    ..Default::default()
                }),
            )
            .await;
            loop {
                if ParsedNetworkMessage::decode_wire(&receive_frame(&stream).await)
                    .expect("local ack")
                    .message
                    == WireNetworkMessage::Verack
                {
                    break;
                }
            }
        }
        send_regtest(&stream, WireNetworkMessage::Verack).await;
        for unsolicited in [
            WireNetworkMessage::CFilter(CFilter {
                filter_type: 0,
                block_hash: fixture.hash(2),
                filter: vec![0],
            }),
            WireNetworkMessage::CFHeaders(CFHeaders {
                filter_type: 0,
                stop_hash: fixture.hash(2),
                previous_filter_header: Default::default(),
                filter_hashes: vec![],
            }),
            WireNetworkMessage::CFCheckpt(CFCheckpt {
                filter_type: 0,
                stop_hash: fixture.hash(2),
                filter_headers: vec![],
            }),
        ] {
            send_regtest(&stream, unsolicited).await;
        }
        send_regtest(&stream, WireNetworkMessage::Ping { nonce: 160_116 }).await;
        // Assert: no CF bytes precede the sentinel, and invalid pre-handshake inputs did not disconnect.
        loop {
            let message = ParsedNetworkMessage::decode_wire(&receive_frame(&stream).await)
                .expect("ordered post-handshake frame")
                .message;
            assert!(!matches!(
                message,
                WireNetworkMessage::CFilter(_)
                    | WireNetworkMessage::CFHeaders(_)
                    | WireNetworkMessage::CFCheckpt(_)
            ));
            if message == (WireNetworkMessage::Pong { nonce: 160_116 }) {
                break;
            }
        }
        drop(stream);
        worker.shutdown().await;
        assert_eq!(
            fixture
                .runtime
                .network_handle()
                .basic_filter_transport_snapshot()
                .expect("no ignored request owners"),
            FilterTransportSnapshot::default()
        );
    }
    fixture.cleanup();
}
