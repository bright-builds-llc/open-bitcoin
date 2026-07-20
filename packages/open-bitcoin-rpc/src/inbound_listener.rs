// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

//! Tokio runtime adapter for opt-in inbound peer serving.

use std::{
    io,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use open_bitcoin_network::{
    InboundAdmissionRejectionReason, InboundListenerActivationDiagnostic, InboundListenerConfig,
    InboundListenerEndpoint, InboundPreflightDiagnostic, InboundPreflightReason,
    InboundResourceEvent, ResourceGovernancePolicy, WireNetworkMessage, classify_inbound_preflight,
};
use open_bitcoin_node::{
    ManagedNetworkHandle, PeerIdentityAuthority, sync::AnnouncementOutboxRegistry,
};
use tokio::{net::TcpListener, task::JoinHandle};

use crate::ManagedRpcContext;

mod connection_runtime;
mod resource_runtime;
#[cfg(test)]
use crate::context::resolve_inbound_wire_responses;
#[cfg(test)]
use connection_runtime::acknowledge_inbound_response_write;
use connection_runtime::handle_inbound_stream;
use resource_runtime::{
    InboundRuntimeCounters, current_timestamp, lock_evidence, lock_runtime_counters,
    record_shared_resource_event, resource_event_from_decision,
};
#[cfg(test)]
use resource_runtime::{
    ReadWireMessageOutcome, RuntimeQueuePressureState, WriteWireMessageOutcome,
    queue_pressure_event, read_wire_message, read_wire_message_with_timeout_duration,
    resource_timeout_event, write_all,
};

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
    pub resource_rejections: usize,
    pub timeout_disconnects: usize,
    pub churn_rejections: usize,
    pub reconnect_suppressions: usize,
    pub maybe_admission_reject_reason: Option<String>,
    pub maybe_latest_admission_event: Option<String>,
    pub maybe_latest_resource_event: Option<InboundResourceEvent>,
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
            resource_rejections: 0,
            timeout_disconnects: 0,
            churn_rejections: 0,
            reconnect_suppressions: 0,
            maybe_admission_reject_reason: None,
            maybe_latest_admission_event: Some(activation.preflight_reason.as_str().to_string()),
            maybe_latest_resource_event: None,
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

    pub(crate) fn record_resource_event(&mut self, event: InboundResourceEvent) {
        match event.next_action.as_str() {
            "payload_rejected" => {
                self.resource_rejections += 1;
            }
            "timeout_disconnect" => {
                self.timeout_disconnects += 1;
            }
            "churn_rejected" => {
                self.churn_rejections += 1;
            }
            "reconnect_suppressed" => {
                self.reconnect_suppressions += 1;
            }
            _ => {}
        }
        self.maybe_latest_resource_event = Some(event);
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
                resource_rejections: 0,
                timeout_disconnects: 0,
                churn_rejections: 0,
                reconnect_suppressions: 0,
                maybe_admission_reject_reason: None,
                maybe_latest_admission_event: Some(reason.as_str().to_string()),
                maybe_latest_resource_event: None,
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
                resource_rejections: 0,
                timeout_disconnects: 0,
                churn_rejections: 0,
                reconnect_suppressions: 0,
                maybe_admission_reject_reason: None,
                maybe_latest_admission_event: Some(
                    InboundPreflightReason::Ready.as_str().to_string(),
                ),
                maybe_latest_resource_event: None,
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
    shutdown_notify: Arc<tokio::sync::Notify>,
}

impl InboundListenerWorker {
    pub fn evidence(&self) -> InboundListenerEvidence {
        lock_evidence(&self.evidence).clone()
    }

    pub async fn shutdown(self) {
        self.shutdown_requested.store(true, Ordering::Relaxed);
        self.shutdown_notify.notify_waiters();
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

#[derive(Clone)]
struct InboundAcceptLoopShared {
    context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    evidence: Arc<Mutex<InboundListenerEvidence>>,
    initial_evidence: InboundListenerEvidence,
    shutdown_requested: Arc<AtomicBool>,
    shutdown_notify: Arc<tokio::sync::Notify>,
    peer_identity_authority: PeerIdentityAuthority,
    runtime_counters: Arc<Mutex<InboundRuntimeCounters>>,
    connection_handles: Arc<tokio::sync::Mutex<Vec<JoinHandle<()>>>>,
    maybe_announcement_transport: Option<InboundAnnouncementTransport>,
}

#[derive(Clone)]
struct InboundAnnouncementTransport {
    outboxes: AnnouncementOutboxRegistry,
    network: ManagedNetworkHandle,
}

#[derive(Clone)]
struct InboundConnectionControl {
    shutdown_requested: Arc<AtomicBool>,
    shutdown_notify: Arc<tokio::sync::Notify>,
    maybe_announcement_transport: Option<InboundAnnouncementTransport>,
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
    start_inbound_accept_loop_inner(activation, context, PeerIdentityAuthority::default(), None)
}

pub fn start_inbound_accept_loop_with_announcements(
    activation: InboundListenerActivation,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    peer_identity_authority: PeerIdentityAuthority,
    outboxes: AnnouncementOutboxRegistry,
    network: ManagedNetworkHandle,
) -> Option<InboundListenerWorker> {
    start_inbound_accept_loop_inner(
        activation,
        context,
        peer_identity_authority,
        Some(InboundAnnouncementTransport { outboxes, network }),
    )
}

fn start_inbound_accept_loop_inner(
    activation: InboundListenerActivation,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    peer_identity_authority: PeerIdentityAuthority,
    maybe_announcement_transport: Option<InboundAnnouncementTransport>,
) -> Option<InboundListenerWorker> {
    if activation.state != InboundListenerState::Listening {
        return None;
    }

    let evidence = Arc::new(Mutex::new(activation.evidence.clone()));
    let shutdown_requested = Arc::new(AtomicBool::new(false));
    let shutdown_notify = Arc::new(tokio::sync::Notify::new());
    let connection_handles = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let runtime_counters = Arc::new(Mutex::new(InboundRuntimeCounters::new(current_timestamp())));
    let shared = InboundAcceptLoopShared {
        context: Arc::clone(&context),
        evidence: Arc::clone(&evidence),
        initial_evidence: activation.evidence.clone(),
        shutdown_requested: Arc::clone(&shutdown_requested),
        shutdown_notify: Arc::clone(&shutdown_notify),
        peer_identity_authority,
        runtime_counters: Arc::clone(&runtime_counters),
        connection_handles: Arc::clone(&connection_handles),
        maybe_announcement_transport,
    };
    let mut handles = Vec::with_capacity(activation.listeners.len());

    for bound_listener in activation.listeners {
        handles.push(tokio::spawn(accept_loop(bound_listener, shared.clone())));
    }

    Some(InboundListenerWorker {
        handles,
        connection_handles,
        evidence,
        shutdown_requested,
        shutdown_notify,
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

async fn accept_loop(bound_listener: BoundInboundListener, shared: InboundAcceptLoopShared) {
    if shared
        .context
        .lock()
        .await
        .set_inbound_listener_evidence(shared.initial_evidence.clone())
        .is_err()
    {
        return;
    }
    let resource_policy = ResourceGovernancePolicy::default();
    loop {
        if shared.shutdown_requested.load(Ordering::Relaxed) {
            break;
        }
        let Ok((stream, remote_addr)) = bound_listener.listener.accept().await else {
            break;
        };
        let now_unix_seconds = current_timestamp();
        let maybe_churn_event = {
            let mut counters = lock_runtime_counters(&shared.runtime_counters);
            let churn_input =
                counters.record_connection_attempt(&resource_policy, now_unix_seconds);
            resource_event_from_decision(resource_policy.decide_churn(churn_input)).or_else(|| {
                let failure_input =
                    counters.repeated_failure_input(&resource_policy, now_unix_seconds);
                resource_event_from_decision(resource_policy.decide_repeated_failure(failure_input))
            })
        };
        if let Some(event) = maybe_churn_event {
            record_shared_resource_event(&shared.context, &shared.evidence, event).await;
            lock_runtime_counters(&shared.runtime_counters)
                .record_failure(&resource_policy, now_unix_seconds);
            continue;
        }

        let Ok(reconnect_input) = shared
            .context
            .lock()
            .await
            .reconnect_suppression_input_for_remote_addr(remote_addr, now_unix_seconds)
        else {
            lock_runtime_counters(&shared.runtime_counters)
                .record_failure(&resource_policy, now_unix_seconds);
            continue;
        };
        if let Some(event) =
            resource_event_from_decision(resource_policy.decide_reconnect(reconnect_input))
        {
            record_shared_resource_event(&shared.context, &shared.evidence, event).await;
            lock_runtime_counters(&shared.runtime_counters)
                .record_failure(&resource_policy, now_unix_seconds);
            continue;
        }

        let Ok(peer_id) = shared.peer_identity_authority.allocate() else {
            lock_runtime_counters(&shared.runtime_counters)
                .record_failure(&resource_policy, now_unix_seconds);
            continue;
        };
        let connection_control = InboundConnectionControl {
            shutdown_requested: Arc::clone(&shared.shutdown_requested),
            shutdown_notify: Arc::clone(&shared.shutdown_notify),
            maybe_announcement_transport: shared.maybe_announcement_transport.clone(),
        };
        let handle = tokio::spawn(handle_inbound_stream(
            peer_id,
            remote_addr,
            stream,
            Arc::clone(&shared.context),
            Arc::clone(&shared.evidence),
            Arc::clone(&shared.runtime_counters),
            connection_control,
        ));
        let mut connection_handles = shared.connection_handles.lock().await;
        connection_handles.retain(|handle| !handle.is_finished());
        connection_handles.push(handle);
    }
}

#[cfg(test)]
mod tests;
