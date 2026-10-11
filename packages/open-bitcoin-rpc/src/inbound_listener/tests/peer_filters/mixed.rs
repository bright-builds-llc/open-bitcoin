// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! One configured authority, concrete inbound executor and concrete outbound TCP loop.

use super::*;
use open_bitcoin_node::network::filter_transport::GenericPendingBuffer;
use open_bitcoin_node::{
    ResolvedSyncPeerAddress, SyncPeerReceiveOutcome, SyncPeerSession, SyncRuntimeError,
    SyncTransport,
};
use std::io::{Read, Write};
use std::sync::{Barrier, mpsc};

const LIMIT: u64 = 8 * 1024 * 1024;

fn pressure(
    handle: &open_bitcoin_node::DurableNetworkHandle,
    target: u64,
) -> Vec<GenericPendingBuffer> {
    let initial = handle
        .basic_filter_transport_snapshot()
        .expect("initial")
        .generic_pending_bytes;
    let mut buffers = Vec::new();
    loop {
        let current = handle
            .basic_filter_transport_snapshot()
            .expect("actual ledger")
            .generic_pending_bytes
            - initial;
        let remaining = target - current;
        if remaining == 0 {
            break;
        }
        let message = if remaining > 256 {
            let mut length = ((remaining / 2).saturating_sub(100)).min(3_999_000) as usize;
            loop {
                let tx = WireNetworkMessage::Tx(Transaction {
                    version: 2,
                    inputs: vec![TransactionInput {
                        previous_output: OutPoint::null(),
                        script_sig: ScriptBuf::default(),
                        sequence: u32::MAX,
                        witness: ScriptWitness::new(vec![vec![0x51; length]]),
                    }],
                    outputs: vec![TransactionOutput {
                        value: Amount::ZERO,
                        script_pubkey: ScriptBuf::default(),
                    }],
                    lock_time: 0,
                });
                let size = tx
                    .encode_wire(SyncNetwork::Regtest.magic())
                    .expect("test size control")
                    .len() as u64;
                if (remaining - size).is_multiple_of(8) {
                    break tx;
                }
                length -= 1;
            }
        } else if remaining.is_multiple_of(24) {
            WireNetworkMessage::Verack
        } else {
            assert!(remaining >= 32, "representable final residue {remaining}");
            WireNetworkMessage::Ping { nonce: 1 }
        };
        buffers.push(
            handle
                .encode_pending_network_message(160_201, &message)
                .expect("real stage-peak admission"),
        );
        assert!(buffers.len() < 128);
    }
    assert_eq!(
        handle
            .basic_filter_transport_snapshot()
            .expect("exact pressure")
            .generic_pending_bytes
            - initial,
        target
    );
    buffers
}

struct ControlledTransport {
    entered: mpsc::Sender<u64>,
    release: mpsc::Receiver<()>,
}
struct ControlledSession<S> {
    inner: S,
    entered: mpsc::Sender<u64>,
    maybe_release: Option<mpsc::Receiver<()>>,
}
impl SyncTransport for ControlledTransport {
    type Session =
        ControlledSession<<open_bitcoin_node::TcpPeerTransport as SyncTransport>::Session>;
    fn connect(
        &mut self,
        peer: &ResolvedSyncPeerAddress,
        config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        let inner = open_bitcoin_node::TcpPeerTransport.connect(peer, config)?;
        // One bounded connection per fixture, no scripted send/default decoder path.
        let (_, replacement) = mpsc::channel();
        let release = std::mem::replace(&mut self.release, replacement);
        Ok(ControlledSession {
            inner,
            entered: self.entered.clone(),
            maybe_release: Some(release),
        })
    }
}
impl<S: SyncPeerSession> SyncPeerSession for ControlledSession<S> {
    fn send_encoded(&mut self, bytes: &[u8], magic: NetworkMagic) -> Result<(), SyncRuntimeError> {
        if let Some(release) = self.maybe_release.take() {
            self.entered
                .send(bytes.len() as u64)
                .expect("actual outgoing owner entered");
            release
                .recv_timeout(Duration::from_secs(10))
                .expect("bounded blocked production write");
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
        self.inner.receive(magic)
    }
}

struct ObservedInbound<'a> {
    stream: &'a TcpStream,
    handle: open_bitcoin_node::DurableNetworkHandle,
    expected: u64,
}
impl InboundFilterWriter for ObservedInbound<'_> {
    async fn write(&mut self, bytes: &[u8]) -> io::Result<WriteWireMessageOutcome> {
        let snapshot = self
            .handle
            .basic_filter_transport_snapshot()
            .expect("shared internal totals");
        assert_eq!(snapshot.active_frame_bytes, bytes.len() as u64);
        assert_eq!(
            snapshot.generic_pending_bytes + snapshot.active_frame_bytes,
            self.expected
        );
        self.handle
            .network_info()
            .expect("authority free during write");
        write_all(self.stream, bytes).await?;
        Ok(WriteWireMessageOutcome::Written)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase160_peer_evidence_cross_direction_generic_blocked_exact_and_one_over() {
    for one_over in [false, true] {
        // Arrange: outbound public sync loop and inbound preparation share the SAME handle.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("outbound remote");
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
            .expect("single exact frame")
            .resolve();
        let InboundWireStep::Filter(response) =
            plan.responses.into_iter().next().expect("one response")
        else {
            panic!("filter")
        };
        let handle = fixture.runtime.network_handle();
        let cf_len = response.progress().expect("zero prefix");
        assert_eq!(cf_len.messages, 0);
        let record = handle
            .basic_filter_query(fixture.hash(0))
            .expect("persisted");
        let open_bitcoin_node::BasicFilterQuery::Found(record) = record else {
            panic!("record")
        };
        let cf_len = WireNetworkMessage::CFilter(CFilter {
            filter_type: 0,
            block_hash: fixture.hash(0),
            filter: record.encoded_bytes().to_vec(),
        })
        .encode_wire(SyncNetwork::Regtest.magic())
        .expect("expected exact size")
        .len() as u64;
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let (client, server) = tcp_pair().await;
        // Act: the actual production generic write is paused with its buffer live.
        tokio::task::block_in_place(|| {
            std::thread::scope(|scope| {
                let worker = scope.spawn(|| {
                    fixture.runtime.sync_once(
                        &mut ControlledTransport {
                            entered: entered_tx,
                            release: release_rx,
                        },
                        50_000,
                    )
                });
                let (mut remote, _) = listener.accept().expect("actual outbound socket");
                remote
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .expect("remote timeout");
                let generic_len = entered_rx
                    .recv_timeout(Duration::from_secs(5))
                    .expect("blocked actual generic");
                assert_eq!(
                    handle
                        .basic_filter_transport_snapshot()
                        .expect("live generic")
                        .generic_pending_bytes,
                    generic_len
                );
                let buffers = pressure(&handle, LIMIT - generic_len - cf_len + u64::from(one_over));
                // Barrier sits immediately before the actual shared reservation.
                let barrier = Barrier::new(1);
                barrier.wait();
                let future = execute_inbound_filter(
                    response,
                    &handle,
                    ObservedInbound {
                        stream: &server,
                        handle: handle.clone(),
                        expected: LIMIT,
                    },
                );
                let result = tokio::runtime::Handle::current().block_on(future);
                if one_over {
                    assert!(result.is_err());
                    assert_eq!(result.expect_err("refusal").progress.messages, 0);
                } else {
                    assert_eq!(result.expect("complete frame").messages, 1);
                }
                drop(buffers);
                release_tx.send(()).expect("settle generic");
                remote
                    .shutdown(std::net::Shutdown::Write)
                    .expect("bounded loop termination");
                let mut bytes = Vec::new();
                remote
                    .read_to_end(&mut bytes)
                    .expect("actual outgoing frame");
                assert!(matches!(
                    ParsedNetworkMessage::decode_wire(&bytes)
                        .expect("unchanged actual encoded bytes")
                        .message,
                    WireNetworkMessage::Version(_)
                ));
                worker
                    .join()
                    .expect("production sync worker")
                    .expect("bounded production loop");
            })
        });
        if !one_over {
            receive_frame(&client).await;
        }
        assert_eq!(
            handle
                .basic_filter_transport_snapshot()
                .expect("all buffers settled"),
            FilterTransportSnapshot::default()
        );
        drop(handle);
        fixture.cleanup();
    }
}

struct HeldInbound<'a> {
    stream: &'a TcpStream,
    entered: &'a tokio::sync::Notify,
    release: &'a tokio::sync::Notify,
    first: bool,
}
impl InboundFilterWriter for HeldInbound<'_> {
    async fn write(&mut self, bytes: &[u8]) -> io::Result<WriteWireMessageOutcome> {
        if self.first {
            self.first = false;
            self.entered.notify_one();
            self.release.notified().await;
        }
        write_all(self.stream, bytes).await?;
        Ok(WriteWireMessageOutcome::Written)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase160_peer_evidence_cross_direction_cf_blocked_generic_peak_exact_and_one_over() {
    for one_over in [false, true] {
        // Arrange: concrete inbound writer holds the handed CF plus real queued suffix.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("outbound remote");
        let mut fixture = FilterListenerFixture::new_with_peer(
            true,
            &["in"],
            Some(listener.local_addr().expect("endpoint")),
        );
        let response = fixture.prepared().await;
        let handle = fixture.runtime.network_handle();
        let (client, server) = tcp_pair().await;
        let entered = tokio::sync::Notify::new();
        let release = tokio::sync::Notify::new();
        let mut execution = Box::pin(execute_inbound_filter(
            response,
            &handle,
            HeldInbound {
                stream: &server,
                entered: &entered,
                release: &release,
                first: true,
            },
        ));
        tokio::select! {result=&mut execution=>panic!("unexpected completion:{result:?}"),()=entered.notified()=>{}}
        let held = handle
            .basic_filter_transport_snapshot()
            .expect("handed real frame");
        assert_eq!(held.requests, 1);
        assert!(held.active_frame_bytes > 0);
        assert!(held.suffix_bytes > held.active_frame_bytes);
        assert!(fixture.context.try_lock().is_ok());
        handle.network_info().expect("authority available");
        let version = handle
            .connect_outbound_peer(160_202, 50_000)
            .expect("independent actual own-version size")
            .into_iter()
            .find(|message| matches!(message, WireNetworkMessage::Version(_)))
            .expect("own version");
        let frame = version
            .encode_wire(SyncNetwork::Regtest.magic())
            .expect("known serializer length");
        let payload = frame.len() as u64 - 24;
        let peak = (payload + ((payload + 9).div_ceil(64) * 64)).max(2 * payload + 24 + 7);
        let final_len = frame.len() as u64;
        drop(frame);
        let buffers = pressure(
            &handle,
            LIMIT - held.active_frame_bytes - peak + u64::from(one_over),
        );
        // Act: public outbound production loop reserves its real encoder peak atomically.
        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        tokio::task::block_in_place(|| {
            std::thread::scope(|scope| {
                let worker = scope.spawn(|| {
                    fixture.runtime.sync_once(
                        &mut ControlledTransport {
                            entered: entered_tx,
                            release: release_rx,
                        },
                        50_000,
                    )
                });
                let (mut remote, _) = listener.accept().expect("real outbound connection");
                remote
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .expect("bounded remote");
                if !one_over {
                    assert_eq!(
                        entered_rx
                            .recv_timeout(Duration::from_secs(5))
                            .expect("admitted at exact peak"),
                        final_len
                    );
                    let snapshot = handle
                        .basic_filter_transport_snapshot()
                        .expect("scratch already dropped, final live");
                    assert_eq!(
                        snapshot.generic_pending_bytes + snapshot.active_frame_bytes,
                        LIMIT - peak + final_len
                    );
                    release_tx.send(()).expect("settle real generic write");
                }
                remote
                    .shutdown(std::net::Shutdown::Write)
                    .expect("bounded production loop");
                let mut actual = Vec::new();
                remote.read_to_end(&mut actual).expect("physical output");
                if one_over {
                    assert!(actual.is_empty(), "no normal shortened success");
                } else {
                    assert!(matches!(
                        ParsedNetworkMessage::decode_wire(&actual)
                            .expect("real generic frame")
                            .message,
                        WireNetworkMessage::Version(_)
                    ));
                }
                worker
                    .join()
                    .expect("sync worker")
                    .expect("bounded loop summary");
            })
        });
        // Assert: refused peak retains live CF/suffix; final frames alone would have fit.
        let snapshot = handle
            .basic_filter_transport_snapshot()
            .expect("CF still blocked");
        assert_eq!(snapshot.active_frame_bytes, held.active_frame_bytes);
        assert_eq!(snapshot.suffix_bytes, held.suffix_bytes);
        assert!(snapshot.generic_pending_bytes + held.active_frame_bytes + final_len < LIMIT);
        drop(buffers);
        release.notify_one();
        assert_eq!(
            execution
                .await
                .expect("settled complete inbound range")
                .messages,
            3
        );
        for _ in 0..3 {
            receive_frame(&client).await;
        }
        assert_eq!(
            handle
                .basic_filter_transport_snapshot()
                .expect("zero settled resources"),
            FilterTransportSnapshot::default()
        );
        drop(handle);
        fixture.cleanup();
    }
}

mod race;
