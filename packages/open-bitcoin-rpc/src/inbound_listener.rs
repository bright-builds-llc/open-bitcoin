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
    INBOUND_MESSAGE_HEADER_LEN, InboundAdmissionDecision, InboundAdmissionRejectionReason,
    InboundEnvelopePolicy, InboundHandshakeState, InboundListenerActivationDiagnostic,
    InboundListenerConfig, InboundListenerEndpoint, InboundPreflightDiagnostic,
    InboundPreflightReason, InboundResourceEvent, ResourceGovernancePolicy, WireNetworkMessage,
    classify_inbound_preflight,
};
use tokio::{net::TcpListener, task::JoinHandle};

use crate::{ManagedRpcContext, context::EncodedWireResponse};

mod resource_runtime;
use resource_runtime::{
    InboundRuntimeCounters, ReadWireMessageOutcome, RuntimeQueuePressureState,
    WriteWireMessageOutcome, next_handshake_state, queue_pressure_event,
    read_wire_message_for_state, resource_event_from_decision, resource_timeout_event,
    write_all_for_state,
};
#[cfg(test)]
use resource_runtime::{read_wire_message, read_wire_message_with_timeout_duration, write_all};

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

#[derive(Clone)]
struct InboundAcceptLoopShared {
    context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    evidence: Arc<Mutex<InboundListenerEvidence>>,
    initial_evidence: InboundListenerEvidence,
    shutdown_requested: Arc<AtomicBool>,
    next_peer_id: Arc<AtomicU64>,
    runtime_counters: Arc<Mutex<InboundRuntimeCounters>>,
    connection_handles: Arc<tokio::sync::Mutex<Vec<JoinHandle<()>>>>,
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
    let runtime_counters = Arc::new(Mutex::new(InboundRuntimeCounters::new(current_timestamp())));
    let shared = InboundAcceptLoopShared {
        context: Arc::clone(&context),
        evidence: Arc::clone(&evidence),
        initial_evidence: activation.evidence.clone(),
        shutdown_requested: Arc::clone(&shutdown_requested),
        next_peer_id: Arc::clone(&next_peer_id),
        runtime_counters: Arc::clone(&runtime_counters),
        connection_handles: Arc::clone(&connection_handles),
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
    shared
        .context
        .lock()
        .await
        .set_inbound_listener_evidence(shared.initial_evidence.clone());
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

        let reconnect_input = shared
            .context
            .lock()
            .await
            .reconnect_suppression_input_for_remote_addr(remote_addr, now_unix_seconds);
        if let Some(event) =
            resource_event_from_decision(resource_policy.decide_reconnect(reconnect_input))
        {
            record_shared_resource_event(&shared.context, &shared.evidence, event).await;
            lock_runtime_counters(&shared.runtime_counters)
                .record_failure(&resource_policy, now_unix_seconds);
            continue;
        }

        let peer_id = shared.next_peer_id.fetch_add(1, Ordering::Relaxed);
        let handle = tokio::spawn(handle_inbound_stream(
            peer_id,
            remote_addr,
            stream,
            Arc::clone(&shared.context),
            Arc::clone(&shared.evidence),
            Arc::clone(&shared.runtime_counters),
        ));
        let mut connection_handles = shared.connection_handles.lock().await;
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
    runtime_counters: Arc<Mutex<InboundRuntimeCounters>>,
) {
    let resource_policy = ResourceGovernancePolicy::default();
    let connected_at_unix_seconds = current_timestamp();
    let mut last_activity_unix_seconds = connected_at_unix_seconds;
    let mut handshake_state = InboundHandshakeState::Accepted;
    let decision = {
        let mut context = context.lock().await;
        context.record_inbound_admission_for_remote_addr(peer_id, remote_addr, false)
    };
    let permission_decision = match decision {
        InboundAdmissionDecision::Admit(record) => {
            lock_evidence(&evidence).record_admitted();
            record.permission_decision
        }
        InboundAdmissionDecision::Reject(rejection) => {
            lock_evidence(&evidence).record_rejected(rejection.reason);
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
            return;
        }
    };
    let mut queue_pressure = RuntimeQueuePressureState::default();

    let envelope_policy =
        InboundEnvelopePolicy::new(context.lock().await.network_info().network_magic);

    'message_loop: loop {
        if let Some(event) = resource_timeout_event(
            &resource_policy,
            connected_at_unix_seconds,
            last_activity_unix_seconds,
            current_timestamp(),
            handshake_state,
        ) {
            record_shared_resource_event(&context, &evidence, event).await;
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
            break;
        }
        queue_pressure.record_pending_read(INBOUND_MESSAGE_HEADER_LEN);
        if let Some(event) = queue_pressure_event(
            &resource_policy,
            &queue_pressure,
            permission_decision.active_effects().to_vec(),
            permission_decision.inactive_effects().to_vec(),
        ) {
            record_shared_resource_event(&context, &evidence, event).await;
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
            break;
        }
        let read_result = read_wire_message_for_state(
            &stream,
            &envelope_policy,
            &resource_policy,
            connected_at_unix_seconds,
            last_activity_unix_seconds,
            handshake_state,
        )
        .await;
        queue_pressure.clear_pending_read();
        let outcome = match read_result {
            Ok(outcome) => outcome,
            Err(_error) => break,
        };
        let parsed = match outcome {
            ReadWireMessageOutcome::Message(parsed) => parsed,
            ReadWireMessageOutcome::Rejected(event) => {
                record_shared_resource_event(&context, &evidence, event).await;
                lock_runtime_counters(&runtime_counters)
                    .record_failure(&resource_policy, current_timestamp());
                break;
            }
        };
        last_activity_unix_seconds = current_timestamp();
        handshake_state = next_handshake_state(handshake_state, &parsed.message);
        lock_evidence(&evidence).record_handshake(&parsed.message);
        let responses = {
            let mut context = context.lock().await;
            context.receive_inbound_wire_message(
                peer_id,
                parsed.message,
                last_activity_unix_seconds,
            )
        };
        let Ok(encoded_responses) = responses else {
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
            break;
        };
        for response in encoded_responses {
            queue_pressure.record_pending_write(response.bytes.len());
            if let Some(event) = queue_pressure_event(
                &resource_policy,
                &queue_pressure,
                permission_decision.active_effects().to_vec(),
                permission_decision.inactive_effects().to_vec(),
            ) {
                record_shared_resource_event(&context, &evidence, event).await;
                lock_runtime_counters(&runtime_counters)
                    .record_failure(&resource_policy, current_timestamp());
                break 'message_loop;
            }
            let write_result = write_all_for_state(
                &stream,
                &response.bytes,
                &resource_policy,
                connected_at_unix_seconds,
                last_activity_unix_seconds,
                handshake_state,
            )
            .await;
            queue_pressure.clear_pending_write();
            acknowledge_inbound_response_write(&write_result, &response, &context).await;
            match write_result {
                Ok(WriteWireMessageOutcome::Written) => {}
                Ok(WriteWireMessageOutcome::Rejected(event)) => {
                    record_shared_resource_event(&context, &evidence, event).await;
                    lock_runtime_counters(&runtime_counters)
                        .record_failure(&resource_policy, current_timestamp());
                    break 'message_loop;
                }
                Err(_error) => {
                    lock_runtime_counters(&runtime_counters)
                        .record_failure(&resource_policy, current_timestamp());
                    break 'message_loop;
                }
            }
        }
    }
    disconnect_admitted_peer(&context, peer_id).await;
}

async fn acknowledge_inbound_response_write(
    write_result: &io::Result<WriteWireMessageOutcome>,
    response: &EncodedWireResponse,
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
) {
    let Ok(WriteWireMessageOutcome::Written) = write_result else {
        return;
    };

    context
        .lock()
        .await
        .acknowledge_wire_message_written(&response.message);
}

async fn disconnect_admitted_peer(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    peer_id: u64,
) {
    let mut context = context.lock().await;
    if let Err(_error) = context.disconnect_peer(peer_id) {
        // The message loop may already have removed the peer, for example after
        // a runtime self-connection rejection.
    }
}

fn lock_evidence(
    evidence: &Arc<Mutex<InboundListenerEvidence>>,
) -> std::sync::MutexGuard<'_, InboundListenerEvidence> {
    match evidence.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn lock_runtime_counters(
    counters: &Arc<Mutex<InboundRuntimeCounters>>,
) -> std::sync::MutexGuard<'_, InboundRuntimeCounters> {
    match counters.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

async fn record_shared_resource_event(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    evidence: &Arc<Mutex<InboundListenerEvidence>>,
    event: InboundResourceEvent,
) {
    lock_evidence(evidence).record_resource_event(event.clone());
    context.lock().await.record_inbound_resource_event(event);
}

fn current_timestamp() -> i64 {
    let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return 0;
    };
    i64::try_from(duration.as_secs()).unwrap_or(i64::MAX)
}

#[cfg(test)]
mod tests;
