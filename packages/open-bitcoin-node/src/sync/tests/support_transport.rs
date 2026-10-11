// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;
use std::{
    net::{TcpListener, TcpStream},
    time::Duration,
};

#[derive(Debug, Clone)]
pub(super) struct ScriptedTransport {
    scripts: VecDeque<Result<Vec<WireNetworkMessage>, SyncRuntimeError>>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
    fail_connect: bool,
}

impl ScriptedTransport {
    pub(super) fn new(scripts: Vec<Vec<WireNetworkMessage>>) -> Self {
        Self {
            scripts: scripts.into_iter().map(Ok).collect(),
            sent: Rc::new(RefCell::new(Vec::new())),
            fail_connect: false,
        }
    }

    pub(super) fn with_connect_results(
        scripts: Vec<Result<Vec<WireNetworkMessage>, SyncRuntimeError>>,
    ) -> Self {
        Self {
            scripts: scripts.into(),
            sent: Rc::new(RefCell::new(Vec::new())),
            fail_connect: false,
        }
    }

    pub(super) fn failing() -> Self {
        Self {
            scripts: VecDeque::new(),
            sent: Rc::new(RefCell::new(Vec::new())),
            fail_connect: true,
        }
    }

    pub(super) fn sent_messages(&self) -> Vec<WireNetworkMessage> {
        self.sent.borrow().clone()
    }
}

#[derive(Debug, Clone)]
pub(super) struct ScriptedSession {
    pub(super) inbound: VecDeque<WireNetworkMessage>,
    pub(super) sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
}

#[derive(Debug, Clone)]
pub(super) struct ErrorAfterMessagesTransport {
    scripts: VecDeque<Vec<WireNetworkMessage>>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
    error: SyncRuntimeError,
    errors_remaining: Rc<RefCell<usize>>,
}

#[derive(Debug, Clone)]
pub(super) struct ErrorAfterMessagesSession {
    inbound: VecDeque<WireNetworkMessage>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
    maybe_error: Option<SyncRuntimeError>,
}

#[derive(Debug, Clone)]
pub(super) struct ScriptedResolver {
    results: VecDeque<Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError>>,
}

impl ScriptedResolver {
    pub(super) fn new(
        results: Vec<Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError>>,
    ) -> Self {
        Self {
            results: results.into(),
        }
    }
}

impl SyncPeerResolver for ScriptedResolver {
    fn resolve(
        &mut self,
        peer: &SyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError> {
        self.results.pop_front().unwrap_or_else(|| {
            Ok(vec![ResolvedSyncPeerAddress::new(
                peer.clone(),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), peer.port),
            )])
        })
    }
}

impl SyncTransport for ScriptedTransport {
    type Session = ScriptedSession;

    fn connect(
        &mut self,
        peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        if self.fail_connect {
            return Err(SyncRuntimeError::Io {
                peer: peer.label(),
                message: "scripted connect failure".to_string(),
            });
        }

        let inbound = self.scripts.pop_front().unwrap_or_else(|| Ok(Vec::new()))?;
        Ok(ScriptedSession {
            inbound: inbound.into(),
            sent: Rc::clone(&self.sent),
        })
    }
}

impl ErrorAfterMessagesTransport {
    pub(super) fn new(
        scripts: Vec<Vec<WireNetworkMessage>>,
        error: SyncRuntimeError,
        errors_remaining: usize,
    ) -> Self {
        Self {
            scripts: scripts.into(),
            sent: Rc::new(RefCell::new(Vec::new())),
            error,
            errors_remaining: Rc::new(RefCell::new(errors_remaining)),
        }
    }

    pub(super) fn sent_messages(&self) -> Vec<WireNetworkMessage> {
        self.sent.borrow().clone()
    }
}

impl SyncTransport for ErrorAfterMessagesTransport {
    type Session = ErrorAfterMessagesSession;

    fn connect(
        &mut self,
        _peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        let inbound = self.scripts.pop_front().unwrap_or_default();
        let mut errors_remaining = self.errors_remaining.borrow_mut();
        let maybe_error = if *errors_remaining == 0 {
            None
        } else {
            *errors_remaining -= 1;
            Some(self.error.clone())
        };
        Ok(ErrorAfterMessagesSession {
            inbound: inbound.into(),
            sent: Rc::clone(&self.sent),
            maybe_error,
        })
    }
}

pub(super) fn resolved_manual_peer(host: &str, port: u16) -> ResolvedSyncPeerAddress {
    ResolvedSyncPeerAddress::new(
        SyncPeerAddress::manual(host, port),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port),
    )
}

impl SyncPeerSession for ScriptedSession {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.sent.borrow_mut().push(message.clone());
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        Ok(self.inbound.pop_front().map_or(
            SyncPeerReceiveOutcome::Closed,
            SyncPeerReceiveOutcome::Message,
        ))
    }
}

impl SyncPeerSession for ErrorAfterMessagesSession {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.sent.borrow_mut().push(message.clone());
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        let maybe_message = self.inbound.pop_front();
        if let Some(message) = maybe_message {
            return Ok(SyncPeerReceiveOutcome::Message(message));
        }
        if let Some(error) = self.maybe_error.take() {
            return Err(error);
        }
        Ok(SyncPeerReceiveOutcome::Closed)
    }
}

pub(super) fn loopback() -> (
    super::super::tcp::TcpPeerSession,
    TcpStream,
    ResolvedSyncPeerAddress,
) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let endpoint = listener.local_addr().expect("endpoint");
    let peer = ResolvedSyncPeerAddress::new(
        SyncPeerAddress::manual("127.0.0.1", endpoint.port()),
        endpoint,
    );
    let session = TcpPeerTransport
        .connect(&peer, &sync_config())
        .expect("connect");
    let (socket, _) = listener.accept().expect("accept");
    socket
        .set_read_timeout(Some(Duration::from_secs(10)))
        .expect("timeout");
    (session, socket, peer)
}

pub(super) fn fill_pending(
    handle: &crate::DurableNetworkHandle,
) -> Vec<crate::network::filter_transport::GenericPendingBuffer> {
    let mut buffers = Vec::new();
    loop {
        let snapshot = handle.basic_filter_transport_snapshot().expect("pressure");
        let remaining =
            8 * 1024 * 1024 - snapshot.generic_pending_bytes - snapshot.active_frame_bytes;
        if remaining < 512 {
            break;
        }
        let witness_len = ((remaining / 2).saturating_sub(160)).min(3_999_000) as usize;
        let message = WireNetworkMessage::Tx(Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::default(),
                sequence: u32::MAX,
                witness: ScriptWitness::new(vec![vec![0x51; witness_len]]),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::ZERO,
                script_pubkey: ScriptBuf::default(),
            }],
            lock_time: 0,
        });
        buffers.push(
            handle
                .encode_pending_network_message(1, &message)
                .expect("conservative actual encoder peak"),
        );
    }
    while let Ok(buffer) = handle.encode_pending_network_message(1, &WireNetworkMessage::Verack) {
        buffers.push(buffer);
    }
    assert!(
        handle
            .encode_pending_network_message(1, &WireNetworkMessage::Ping { nonce: 1 })
            .is_err()
    );
    assert!(buffers.len() < 128);
    buffers
}

pub(super) fn expected_filter_frames(
    fixture: &super::filter_index::peer_fixtures::PeerFilterFixture,
    magic: open_bitcoin_core::primitives::NetworkMagic,
) -> Vec<Vec<u8>> {
    let records: Vec<_> = (0..=2)
        .map(|height| {
            let crate::network::BasicFilterQuery::Found(record) = fixture
                .runtime
                .network
                .basic_filter_query(fixture.hash(height))
                .expect("persisted record")
            else {
                panic!("indexed prefix")
            };
            record
        })
        .collect();
    let expected_messages = [
        WireNetworkMessage::CFilter(open_bitcoin_network::CFilter {
            filter_type: 0,
            block_hash: fixture.hash(1),
            filter: records[1].encoded_bytes().to_vec(),
        }),
        WireNetworkMessage::CFilter(open_bitcoin_network::CFilter {
            filter_type: 0,
            block_hash: fixture.hash(2),
            filter: records[2].encoded_bytes().to_vec(),
        }),
        WireNetworkMessage::CFHeaders(open_bitcoin_network::CFHeaders {
            filter_type: 0,
            stop_hash: fixture.hash(2),
            previous_filter_header: records[0].filter_header().into(),
            filter_hashes: records[1..]
                .iter()
                .map(|record| record.identity().filter_hash().into())
                .collect(),
        }),
        WireNetworkMessage::CFCheckpt(open_bitcoin_network::CFCheckpt {
            filter_type: 0,
            stop_hash: fixture.hash(2),
            filter_headers: Vec::new(),
        }),
    ];
    expected_messages
        .iter()
        .map(|message| {
            message
                .encode_wire(magic)
                .expect("expected persisted record framing")
        })
        .collect()
}
