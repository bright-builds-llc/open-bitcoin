// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

//! Tokio runtime adapter for opt-in inbound peer serving.

use std::{
    io,
    net::SocketAddr,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_network::{
    InboundAdmissionDecision, InboundAdmissionRejectionReason, InboundListenerActivationDiagnostic,
    InboundListenerConfig, InboundListenerEndpoint, InboundPreflightDiagnostic,
    InboundPreflightReason, ParsedNetworkMessage, WireNetworkMessage, classify_inbound_preflight,
};
use tokio::{net::TcpListener, task::JoinHandle};

use crate::ManagedRpcContext;

const MESSAGE_HEADER_LEN: usize = 24;
const PAYLOAD_SIZE_OFFSET: usize = 16;
const PAYLOAD_SIZE_LEN: usize = 4;
const MAX_INBOUND_RUNTIME_PAYLOAD_BYTES: usize = 32 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InboundListenerState {
    Disabled,
    Blocked,
    Listening,
}

impl InboundListenerState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Blocked => "blocked",
            Self::Listening => "listening",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundInboundEndpoint {
    pub configured_endpoint: String,
    pub bound_endpoint: String,
}

#[derive(Debug)]
struct BoundInboundListener {
    listener: TcpListener,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundListenerEvidence {
    pub listener_state: String,
    pub preflight_reason: String,
    pub bound_endpoints: Vec<String>,
    pub admitted_inbound_peers: usize,
    pub rejected_inbound_peers: usize,
    pub maybe_admission_reject_reason: Option<String>,
    pub maybe_latest_admission_event: Option<String>,
}

impl InboundListenerEvidence {
    fn from_activation(activation: &InboundListenerActivation) -> Self {
        Self {
            listener_state: activation.state.as_str().to_string(),
            preflight_reason: activation.preflight_reason.as_str().to_string(),
            bound_endpoints: activation
                .bound_endpoints
                .iter()
                .map(|endpoint| endpoint.bound_endpoint.clone())
                .collect(),
            admitted_inbound_peers: 0,
            rejected_inbound_peers: 0,
            maybe_admission_reject_reason: None,
            maybe_latest_admission_event: Some(activation.preflight_reason.as_str().to_string()),
        }
    }

    fn record_admitted(&mut self) {
        self.admitted_inbound_peers += 1;
        self.maybe_latest_admission_event = Some("admitted".to_string());
    }

    fn record_rejected(&mut self, reason: InboundAdmissionRejectionReason) {
        self.rejected_inbound_peers += 1;
        self.maybe_admission_reject_reason = Some(reason.as_str().to_string());
        self.maybe_latest_admission_event = Some(reason.as_str().to_string());
    }

    fn record_handshake(&mut self, message: &WireNetworkMessage) {
        self.maybe_latest_admission_event = Some(message.command_name().to_string());
    }
}

#[derive(Debug)]
pub struct InboundListenerActivation {
    state: InboundListenerState,
    preflight_reason: InboundPreflightReason,
    diagnostics: Vec<InboundPreflightDiagnostic>,
    bound_endpoints: Vec<BoundInboundEndpoint>,
    listeners: Vec<BoundInboundListener>,
    evidence: InboundListenerEvidence,
}

impl InboundListenerActivation {
    pub fn state(&self) -> InboundListenerState {
        self.state
    }

    pub fn preflight_reason(&self) -> InboundPreflightReason {
        self.preflight_reason
    }

    pub fn diagnostics(&self) -> &[InboundPreflightDiagnostic] {
        &self.diagnostics
    }

    pub fn bound_endpoints(&self) -> &[BoundInboundEndpoint] {
        &self.bound_endpoints
    }

    pub fn evidence(&self) -> &InboundListenerEvidence {
        &self.evidence
    }

    pub fn latest_admission_event(&self) -> Option<&str> {
        self.evidence.maybe_latest_admission_event.as_deref()
    }

    fn inactive(
        state: InboundListenerState,
        reason: InboundPreflightReason,
        diagnostics: Vec<InboundPreflightDiagnostic>,
    ) -> Self {
        let mut activation = Self {
            state,
            preflight_reason: reason,
            diagnostics,
            bound_endpoints: Vec::new(),
            listeners: Vec::new(),
            evidence: InboundListenerEvidence {
                listener_state: state.as_str().to_string(),
                preflight_reason: reason.as_str().to_string(),
                bound_endpoints: Vec::new(),
                admitted_inbound_peers: 0,
                rejected_inbound_peers: 0,
                maybe_admission_reject_reason: None,
                maybe_latest_admission_event: Some(reason.as_str().to_string()),
            },
        };
        activation.evidence = InboundListenerEvidence::from_activation(&activation);
        activation
    }

    fn listening(
        diagnostics: Vec<InboundPreflightDiagnostic>,
        bound_endpoints: Vec<BoundInboundEndpoint>,
        listeners: Vec<BoundInboundListener>,
    ) -> Self {
        let mut activation = Self {
            state: InboundListenerState::Listening,
            preflight_reason: InboundPreflightReason::Ready,
            diagnostics,
            bound_endpoints,
            listeners,
            evidence: InboundListenerEvidence {
                listener_state: InboundListenerState::Listening.as_str().to_string(),
                preflight_reason: InboundPreflightReason::Ready.as_str().to_string(),
                bound_endpoints: Vec::new(),
                admitted_inbound_peers: 0,
                rejected_inbound_peers: 0,
                maybe_admission_reject_reason: None,
                maybe_latest_admission_event: Some(
                    InboundPreflightReason::Ready.as_str().to_string(),
                ),
            },
        };
        activation.evidence = InboundListenerEvidence::from_activation(&activation);
        activation
    }
}

#[derive(Debug)]
pub struct InboundListenerWorker {
    handles: Vec<JoinHandle<()>>,
    connection_handles: Arc<tokio::sync::Mutex<Vec<JoinHandle<()>>>>,
    evidence: Arc<Mutex<InboundListenerEvidence>>,
    shutdown_requested: Arc<AtomicBool>,
}

impl InboundListenerWorker {
    pub fn evidence(&self) -> InboundListenerEvidence {
        lock_evidence(&self.evidence).clone()
    }

    pub async fn shutdown(self) {
        self.shutdown_requested.store(true, Ordering::Relaxed);
        for handle in self.handles {
            handle.abort();
            let _ = handle.await;
        }
        let mut connection_handles = self.connection_handles.lock().await;
        for handle in connection_handles.drain(..) {
            handle.abort();
            let _ = handle.await;
        }
    }
}

pub async fn activate_inbound_listener(
    config: &InboundListenerConfig,
) -> InboundListenerActivation {
    let plan = classify_inbound_preflight(config);
    if !plan.should_attempt_bind() {
        let state = match plan.reason() {
            InboundPreflightReason::Disabled => InboundListenerState::Disabled,
            _ => InboundListenerState::Blocked,
        };
        return InboundListenerActivation::inactive(
            state,
            plan.reason(),
            plan.diagnostics().to_vec(),
        );
    }

    let mut listeners = Vec::with_capacity(plan.ready_endpoints().len());
    let mut bound_endpoints = Vec::with_capacity(plan.ready_endpoints().len());
    for endpoint in plan.ready_endpoints() {
        match bind_endpoint(endpoint).await {
            Ok((bound_endpoint, listener)) => {
                listeners.push(BoundInboundListener { listener });
                bound_endpoints.push(bound_endpoint);
            }
            Err(diagnostic) => {
                return InboundListenerActivation::inactive(
                    InboundListenerState::Blocked,
                    diagnostic.reason,
                    vec![diagnostic],
                );
            }
        }
    }

    InboundListenerActivation::listening(plan.diagnostics().to_vec(), bound_endpoints, listeners)
}

pub fn start_inbound_accept_loop(
    activation: InboundListenerActivation,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
) -> Option<InboundListenerWorker> {
    if activation.state != InboundListenerState::Listening {
        return None;
    }

    let evidence = Arc::new(Mutex::new(activation.evidence.clone()));
    let shutdown_requested = Arc::new(AtomicBool::new(false));
    let connection_handles = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let next_peer_id = Arc::new(AtomicU64::new(1));
    let mut handles = Vec::with_capacity(activation.listeners.len());

    for bound_listener in activation.listeners {
        handles.push(tokio::spawn(accept_loop(
            bound_listener,
            Arc::clone(&context),
            Arc::clone(&evidence),
            Arc::clone(&shutdown_requested),
            Arc::clone(&next_peer_id),
            Arc::clone(&connection_handles),
        )));
    }

    Some(InboundListenerWorker {
        handles,
        connection_handles,
        evidence,
        shutdown_requested,
    })
}

async fn bind_endpoint(
    endpoint: &InboundListenerEndpoint,
) -> Result<(BoundInboundEndpoint, TcpListener), InboundPreflightDiagnostic> {
    let listener = TcpListener::bind(endpoint.address)
        .await
        .map_err(|error| activation_bind_diagnostic(endpoint, &error))?;
    let local_addr = listener.local_addr().map_err(|error| {
        InboundListenerActivationDiagnostic::bind_unavailable(endpoint, error.to_string())
            .into_preflight_diagnostic()
    })?;
    Ok((
        BoundInboundEndpoint {
            configured_endpoint: endpoint.normalized.clone(),
            bound_endpoint: local_addr.to_string(),
        },
        listener,
    ))
}

fn activation_bind_diagnostic(
    endpoint: &InboundListenerEndpoint,
    error: &io::Error,
) -> InboundPreflightDiagnostic {
    match error.kind() {
        io::ErrorKind::AddrInUse => {
            InboundListenerActivationDiagnostic::already_bound(endpoint, error.to_string())
        }
        _ => InboundListenerActivationDiagnostic::bind_unavailable(endpoint, error.to_string()),
    }
    .into_preflight_diagnostic()
}

async fn accept_loop(
    bound_listener: BoundInboundListener,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    evidence: Arc<Mutex<InboundListenerEvidence>>,
    shutdown_requested: Arc<AtomicBool>,
    next_peer_id: Arc<AtomicU64>,
    connection_handles: Arc<tokio::sync::Mutex<Vec<JoinHandle<()>>>>,
) {
    loop {
        if shutdown_requested.load(Ordering::Relaxed) {
            break;
        }
        let Ok((stream, remote_addr)) = bound_listener.listener.accept().await else {
            break;
        };
        let peer_id = next_peer_id.fetch_add(1, Ordering::Relaxed);
        let handle = tokio::spawn(handle_inbound_stream(
            peer_id,
            remote_addr,
            stream,
            Arc::clone(&context),
            Arc::clone(&evidence),
        ));
        let mut connection_handles = connection_handles.lock().await;
        connection_handles.retain(|handle| !handle.is_finished());
        connection_handles.push(handle);
    }
}

async fn handle_inbound_stream(
    peer_id: u64,
    remote_addr: SocketAddr,
    stream: tokio::net::TcpStream,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    evidence: Arc<Mutex<InboundListenerEvidence>>,
) {
    let decision = {
        let mut context = context.lock().await;
        context.record_inbound_admission_for_remote_addr(peer_id, remote_addr, false)
    };
    match decision {
        InboundAdmissionDecision::Admit(_record) => {
            lock_evidence(&evidence).record_admitted();
        }
        InboundAdmissionDecision::Reject(rejection) => {
            lock_evidence(&evidence).record_rejected(rejection.reason);
            return;
        }
    }

    while let Ok(bytes) = read_wire_message(&stream).await {
        let Ok(parsed) = ParsedNetworkMessage::decode_wire(&bytes) else {
            break;
        };
        lock_evidence(&evidence).record_handshake(&parsed.message);
        let responses = {
            let mut context = context.lock().await;
            context.receive_inbound_wire_message(peer_id, parsed.message, current_timestamp())
        };
        let Ok(encoded_responses) = responses else {
            break;
        };
        for response in encoded_responses {
            if write_all(&stream, &response).await.is_err() {
                return;
            }
        }
    }
}

async fn read_wire_message(stream: &tokio::net::TcpStream) -> io::Result<Vec<u8>> {
    let mut header = [0_u8; MESSAGE_HEADER_LEN];
    read_exact(stream, &mut header).await?;
    let payload_len = payload_len_from_header(&header)?;
    let mut encoded = header.to_vec();
    let mut payload = vec![0_u8; payload_len];
    read_exact(stream, &mut payload).await?;
    encoded.extend_from_slice(&payload);
    Ok(encoded)
}

fn payload_len_from_header(header: &[u8; MESSAGE_HEADER_LEN]) -> io::Result<usize> {
    let mut payload_len = [0_u8; PAYLOAD_SIZE_LEN];
    payload_len
        .copy_from_slice(&header[PAYLOAD_SIZE_OFFSET..PAYLOAD_SIZE_OFFSET + PAYLOAD_SIZE_LEN]);
    let payload_len = u32::from_le_bytes(payload_len) as usize;
    if payload_len > MAX_INBOUND_RUNTIME_PAYLOAD_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "inbound wire message payload exceeds runtime listener bound",
        ));
    }
    Ok(payload_len)
}

async fn read_exact(stream: &tokio::net::TcpStream, buffer: &mut [u8]) -> io::Result<()> {
    let mut offset = 0;
    while offset < buffer.len() {
        stream.readable().await?;
        match stream.try_read(&mut buffer[offset..]) {
            Ok(0) => return Err(io::Error::from(io::ErrorKind::UnexpectedEof)),
            Ok(read) => offset += read,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

async fn write_all(stream: &tokio::net::TcpStream, buffer: &[u8]) -> io::Result<()> {
    let mut offset = 0;
    while offset < buffer.len() {
        stream.writable().await?;
        match stream.try_write(&buffer[offset..]) {
            Ok(0) => return Err(io::Error::from(io::ErrorKind::WriteZero)),
            Ok(written) => offset += written,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn lock_evidence(
    evidence: &Arc<Mutex<InboundListenerEvidence>>,
) -> std::sync::MutexGuard<'_, InboundListenerEvidence> {
    match evidence.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn current_timestamp() -> i64 {
    let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return 0;
    };
    i64::try_from(duration.as_secs()).unwrap_or(i64::MAX)
}

#[cfg(test)]
mod tests;
