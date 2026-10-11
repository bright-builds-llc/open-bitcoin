// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Barrier-raced production Pong and inbound CF ownership.

use super::*;

struct RaceTransport {
    barrier: Arc<Barrier>,
    ready: mpsc::Sender<()>,
    entered: mpsc::Sender<()>,
    maybe_release: Option<mpsc::Receiver<()>>,
}
struct RaceSession<S> {
    inner: S,
    barrier: Arc<Barrier>,
    ready: mpsc::Sender<()>,
    entered: mpsc::Sender<()>,
    maybe_release: Option<mpsc::Receiver<()>>,
}
impl SyncTransport for RaceTransport {
    type Session = RaceSession<<open_bitcoin_node::TcpPeerTransport as SyncTransport>::Session>;
    fn connect(
        &mut self,
        peer: &ResolvedSyncPeerAddress,
        config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        Ok(RaceSession {
            inner: open_bitcoin_node::TcpPeerTransport.connect(peer, config)?,
            barrier: Arc::clone(&self.barrier),
            ready: self.ready.clone(),
            entered: self.entered.clone(),
            maybe_release: self.maybe_release.take(),
        })
    }
}
impl<S: SyncPeerSession> SyncPeerSession for RaceSession<S> {
    fn send_encoded(&mut self, bytes: &[u8], magic: NetworkMagic) -> Result<(), SyncRuntimeError> {
        if matches!(
            ParsedNetworkMessage::decode_wire(bytes)
                .expect("actual encoded generic")
                .message,
            WireNetworkMessage::Pong { nonce: 160_203 }
        ) {
            self.entered.send(()).expect("Pong retained owner");
            self.maybe_release
                .take()
                .expect("one Pong")
                .recv_timeout(Duration::from_secs(10))
                .expect("bounded blocked Pong");
        }
        self.inner.send_encoded(bytes, magic)
    }
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        magic: NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.inner.send(message, magic)
    }
    fn receive(&mut self, magic: NetworkMagic) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        let result = self.inner.receive(magic)?;
        if matches!(
            result,
            SyncPeerReceiveOutcome::Message(WireNetworkMessage::Ping { nonce: 160_203 })
        ) {
            self.ready
                .send(())
                .expect("right before actual generic admission");
            self.barrier.wait();
        }
        Ok(result)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase160_peer_evidence_cross_direction_barrier_race_has_one_physical_winner() {
    for _ in 0..8 {
        // Arrange: both actual paths can admit alone, never together at this headroom.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("remote");
        let mut fixture = FilterListenerFixture::new_with_peer(
            true,
            &["in"],
            Some(listener.local_addr().expect("endpoint")),
        );
        drop(fixture.prepared().await);
        let plan = fixture
            .context
            .lock()
            .await
            .prepare_inbound_wire_message(
                160_009,
                WireNetworkMessage::GetCFilters(GetCFilters {
                    filter_type: 0,
                    start_height: 0,
                    stop_hash: fixture.hash(0),
                }),
                2,
            )
            .expect("single frame")
            .resolve();
        let InboundWireStep::Filter(response) =
            plan.responses.into_iter().next().expect("response")
        else {
            panic!("filter")
        };
        let handle = fixture.runtime.network_handle();
        let (client, server) = tcp_pair().await;
        let open_bitcoin_node::BasicFilterQuery::Found(record) =
            handle.basic_filter_query(fixture.hash(0)).expect("row")
        else {
            panic!("row")
        };
        let cf_len = WireNetworkMessage::CFilter(CFilter {
            filter_type: 0,
            block_hash: fixture.hash(0),
            filter: record.encoded_bytes().to_vec(),
        })
        .encode_wire(SyncNetwork::Regtest.magic())
        .expect("exact frame size")
        .len() as u64;
        let barrier = Arc::new(Barrier::new(2));
        let (ready_tx, ready_rx) = mpsc::channel();
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let cf_entered = tokio::sync::Notify::new();
        let cf_release = tokio::sync::Notify::new();
        let mut execution = Box::pin(execute_inbound_filter(
            response,
            &handle,
            HeldInbound {
                stream: &server,
                entered: &cf_entered,
                release: &cf_release,
                first: true,
            },
        ));
        let mut cf_won = false;
        let mut generic_won = false;
        // Act: real received Ping gates the production Pong factory immediately before admission.
        tokio::task::block_in_place(|| {
            std::thread::scope(|scope| {
                let gate = Arc::clone(&barrier);
                let worker = scope.spawn(|| {
                    fixture.runtime.sync_once(
                        &mut RaceTransport {
                            barrier: gate,
                            ready: ready_tx,
                            entered: entered_tx,
                            maybe_release: Some(release_rx),
                        },
                        50_000,
                    )
                });
                let (mut remote, _) = listener.accept().expect("real connection");
                remote
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .expect("bounded read");
                for message in [
                    WireNetworkMessage::Version(VersionMessage {
                        nonce: 160_204,
                        ..Default::default()
                    }),
                    WireNetworkMessage::Verack,
                    WireNetworkMessage::Ping { nonce: 160_203 },
                ] {
                    remote
                        .write_all(
                            &message
                                .encode_wire(SyncNetwork::Regtest.magic())
                                .expect("real incoming"),
                        )
                        .expect("remote write");
                }
                // Wait until the handshake's generic owners have settled and the Ping reaches the gate.
                let own_version = read_std_frame(&mut remote);
                assert!(matches!(
                    ParsedNetworkMessage::decode_wire(&own_version)
                        .expect("version")
                        .message,
                    WireNetworkMessage::Version(_)
                ));
                loop {
                    let bytes = read_std_frame(&mut remote);
                    if ParsedNetworkMessage::decode_wire(&bytes)
                        .expect("ack")
                        .message
                        == WireNetworkMessage::Verack
                    {
                        break;
                    }
                }
                ready_rx
                    .recv_timeout(Duration::from_secs(5))
                    .expect("prior actual generic buffers settled");
                let buffers = pressure(&handle, LIMIT - (cf_len + 32 - 1));
                barrier.wait();
                tokio::runtime::Handle::current().block_on(async {
                tokio::select! {result=&mut execution=>{assert!(result.is_err());},()=cf_entered.notified()=>{cf_won=true;}}
            });
                if entered_rx.recv_timeout(Duration::from_secs(1)).is_ok() {
                    generic_won = true;
                }
                let snapshot = handle
                    .basic_filter_transport_snapshot()
                    .expect("atomic mixed totals");
                assert!(snapshot.generic_pending_bytes + snapshot.active_frame_bytes <= LIMIT);
                assert_ne!(cf_won, generic_won, "exactly one retained physical owner");
                if generic_won {
                    release_tx.send(()).expect("complete Pong");
                }
                if cf_won {
                    cf_release.notify_one();
                    tokio::runtime::Handle::current().block_on(async {
                        assert_eq!((&mut execution).await.expect("real CF winner").messages, 1);
                    });
                }
                drop(buffers);
                remote
                    .shutdown(std::net::Shutdown::Write)
                    .expect("bounded session");
                let mut rest = Vec::new();
                remote
                    .read_to_end(&mut rest)
                    .expect("actual outgoing suffix");
                let mut pongs = 0;
                let mut remaining = rest.as_slice();
                while !remaining.is_empty() {
                    let size = 24
                        + parse_message_header(&remaining[..24])
                            .expect("header")
                            .payload_size as usize;
                    if ParsedNetworkMessage::decode_wire(&remaining[..size])
                        .expect("actual frame")
                        .message
                        == (WireNetworkMessage::Pong { nonce: 160_203 })
                    {
                        pongs += 1;
                    }
                    remaining = &remaining[size..];
                }
                assert_eq!(pongs, usize::from(generic_won));
                worker
                    .join()
                    .expect("worker")
                    .expect("production loop summary");
            })
        });
        if cf_won {
            receive_frame(&client).await;
        }
        drop(execution);
        assert_eq!(
            handle
                .basic_filter_transport_snapshot()
                .expect("settled both directions"),
            FilterTransportSnapshot::default()
        );
        drop(handle);
        fixture.cleanup();
    }
}

fn read_std_frame(stream: &mut std::net::TcpStream) -> Vec<u8> {
    let mut bytes = vec![0; 24];
    stream.read_exact(&mut bytes).expect("actual remote header");
    let size = parse_message_header(&bytes).expect("header").payload_size as usize;
    assert!(size <= 4_000_000);
    bytes.resize(24 + size, 0);
    stream
        .read_exact(&mut bytes[24..])
        .expect("actual remote payload");
    bytes
}
