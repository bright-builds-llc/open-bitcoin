// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

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
