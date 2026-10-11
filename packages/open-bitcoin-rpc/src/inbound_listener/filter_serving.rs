// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Request-bound full-frame transport, with explicit terminal suffix retirement.

use super::super::resource_runtime::WriteWireMessageOutcome;
use open_bitcoin_network::WireNetworkMessage;
use open_bitcoin_node::network::filter_transport::{
    FilterTransportDisposition, FilterTransportError, FilterWrittenPrefix, PreparedFilterResponse,
};
use open_bitcoin_node::network::{
    EffectAbort, PeerEmission, PeerEmissionReceipt, PeerEmissionWriteCapability,
};
use open_bitcoin_node::{ChainstateStore, ManagedNetworkHandle, core::chainstate::CoinsView};
use std::collections::VecDeque;
use std::{future::Future, io};

pub(in crate::inbound_listener) struct InboundResponseBatch<'a, S: ChainstateStore, V: CoinsView> {
    pub steps: std::collections::VecDeque<crate::context::InboundWireStep>,
    pub network: &'a ManagedNetworkHandle<S, V>,
    pub evidence: &'a std::sync::Arc<std::sync::Mutex<super::super::InboundListenerEvidence>>,
}

impl<S: ChainstateStore, V: CoinsView> Drop for InboundResponseBatch<'_, S, V> {
    fn drop(&mut self) {
        for step in self.steps.drain(..) {
            match step {
                crate::context::InboundWireStep::Encoded(response) => {
                    drop(GuardedEncodedResponse {
                        response,
                        network: self.network,
                        evidence: self.evidence,
                    });
                }
                crate::context::InboundWireStep::Filter(response) => {
                    if response.cancel().is_err() {
                        record_cleanup_failure(self.evidence);
                    }
                }
            }
        }
    }
}

pub(in crate::inbound_listener) struct GuardedEncodedResponse<'a, S: ChainstateStore, V: CoinsView>
{
    pub response: Box<crate::context::EncodedWireResponse>,
    pub network: &'a ManagedNetworkHandle<S, V>,
    pub evidence: &'a std::sync::Arc<std::sync::Mutex<super::super::InboundListenerEvidence>>,
}
impl<S: ChainstateStore, V: CoinsView> std::ops::Deref for GuardedEncodedResponse<'_, S, V> {
    type Target = crate::context::EncodedWireResponse;
    fn deref(&self) -> &Self::Target {
        &self.response
    }
}
impl<S: ChainstateStore, V: CoinsView> std::ops::DerefMut for GuardedEncodedResponse<'_, S, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.response
    }
}
impl<S: ChainstateStore, V: CoinsView> Drop for GuardedEncodedResponse<'_, S, V> {
    fn drop(&mut self) {
        if !abort_encoded_response(&mut self.response, self.network) {
            record_cleanup_failure(self.evidence);
        }
    }
}

pub(super) fn abort_encoded_response<S: ChainstateStore, V: CoinsView>(
    response: &mut crate::context::EncodedWireResponse,
    network: &ManagedNetworkHandle<S, V>,
) -> bool {
    let mut clean = true;
    if let Some(capability) = response.maybe_tx_write_capability.take() {
        clean &= network.abort_peer_emission(capability).is_ok();
    }
    if let Some(intent) = response.maybe_block_serve_intent.take() {
        clean &= network
            .complete_block_serve(&intent.completion(
                open_bitcoin_node::network::ManagedBlockServeCompletionOutcome::TransportFailed,
            ))
            .is_ok();
    }
    clean
}

fn record_cleanup_failure(
    evidence: &std::sync::Arc<std::sync::Mutex<super::super::InboundListenerEvidence>>,
) {
    super::super::resource_runtime::lock_evidence(evidence).record_resource_event(
        open_bitcoin_network::InboundResourceEvent {
            outcome: "cleanup_failed".to_string(),
            reason: "inbound unwritten capability retirement failed".to_string(),
            label: "inbound_cleanup".to_string(),
            source: "source_runtime_write".to_string(),
            message: "inbound_resource_governance".to_string(),
            next_action: "disconnect".to_string(),
        },
    );
}

#[derive(Debug)]
pub(in crate::inbound_listener) enum FilterWriteFailure {
    Transport(FilterTransportError),
    Write(io::Error),
    Rejected(open_bitcoin_network::InboundResourceEvent),
}

#[derive(Debug)]
pub(in crate::inbound_listener) struct InboundFilterFailure {
    pub cause: FilterWriteFailure,
    pub maybe_cleanup: Option<FilterTransportError>,
    pub maybe_progress_failure: Option<FilterTransportError>,
    pub progress: FilterWrittenPrefix,
}

impl InboundFilterFailure {
    pub(super) fn resource_event(&self) -> open_bitcoin_network::InboundResourceEvent {
        let reason = match &self.cause {
            FilterWriteFailure::Transport(error) => format!("{error}"),
            FilterWriteFailure::Write(error) => format!("socket {:?}", error.kind()),
            FilterWriteFailure::Rejected(event) => event.reason.clone(),
        };
        open_bitcoin_network::InboundResourceEvent {
            outcome: "transport_failed".to_string(),
            reason: format!(
                "{reason}; complete frames={}; bytes={}; cleanup={:?}; progress_failure={:?}",
                self.progress.messages,
                self.progress.bytes,
                self.maybe_cleanup,
                self.maybe_progress_failure
            ),
            label: "basic_filter_transport".to_string(),
            source: "source_runtime_write".to_string(),
            message: "inbound_resource_governance".to_string(),
            next_action: "disconnect".to_string(),
        }
    }
}

pub(in crate::inbound_listener) trait InboundFilterWriter:
    Send
{
    fn write(
        &mut self,
        bytes: &[u8],
    ) -> impl Future<Output = io::Result<WriteWireMessageOutcome>> + Send;
}

pub(in crate::inbound_listener) struct SocketFilterWriter<'a> {
    pub stream: &'a tokio::net::TcpStream,
    pub policy: &'a open_bitcoin_network::ResourceGovernancePolicy,
    pub connected: i64,
    pub activity: i64,
    pub handshake: open_bitcoin_network::InboundHandshakeState,
    pub queue: &'a mut super::super::resource_runtime::RuntimeQueuePressureState,
    pub active: Vec<open_bitcoin_network::PermissionEffectLabel>,
    pub inactive: Vec<open_bitcoin_network::InactivePermissionEffectLabel>,
    pub shutdown: &'a tokio::sync::Notify,
}

impl InboundFilterWriter for SocketFilterWriter<'_> {
    async fn write(&mut self, bytes: &[u8]) -> io::Result<WriteWireMessageOutcome> {
        use super::super::resource_runtime::{filter_queue_pressure_event, write_all_for_state};
        self.queue.record_pending_write(bytes.len());
        let result = if let Some(event) = filter_queue_pressure_event(
            self.policy,
            self.queue,
            self.active.clone(),
            self.inactive.clone(),
        ) {
            Ok(WriteWireMessageOutcome::Rejected(event))
        } else {
            tokio::select! {
                result = write_all_for_state(self.stream, bytes, self.policy, self.connected, self.activity, self.handshake) => result,
                () = self.shutdown.notified() => Err(io::Error::from(io::ErrorKind::Interrupted)),
            }
        };
        self.queue.clear_pending_write();
        result
    }
}

pub(in crate::inbound_listener) async fn execute_inbound_filter<S, V, W>(
    mut response: PreparedFilterResponse,
    network: &ManagedNetworkHandle<S, V>,
    mut write: W,
) -> Result<FilterWrittenPrefix, InboundFilterFailure>
where
    S: ChainstateStore,
    V: CoinsView,
    W: InboundFilterWriter,
{
    let cause = loop {
        match network.basic_filter_response_is_current(&response) {
            Ok(true) => {}
            Ok(false) => break FilterWriteFailure::Transport(FilterTransportError::Invalidated),
            Err(error) => break FilterWriteFailure::Transport(error),
        }
        let frame = match response.next_frame() {
            Ok(Some(frame)) => frame,
            Ok(None) => {
                return response.progress().map_err(|error| InboundFilterFailure {
                    cause: FilterWriteFailure::Transport(error),
                    maybe_cleanup: None,
                    maybe_progress_failure: Some(error),
                    progress: FilterWrittenPrefix::default(),
                });
            }
            Err(error) => break FilterWriteFailure::Transport(error),
        };
        // No context, authority or registry guard survives into this await.
        match write.write(frame.bytes()).await {
            Ok(WriteWireMessageOutcome::Written) => {
                match network.complete_basic_filter_frame(frame.acknowledge_write()) {
                    Ok(completion)
                        if completion.disposition
                            == FilterTransportDisposition::CurrentRangeComplete =>
                    {
                        return Ok(completion.progress);
                    }
                    Ok(completion)
                        if completion.disposition == FilterTransportDisposition::WrittenPrefix => {}
                    Ok(_) => {
                        break FilterWriteFailure::Transport(FilterTransportError::Invalidated);
                    }
                    Err(error) => break FilterWriteFailure::Transport(error),
                }
            }
            Ok(WriteWireMessageOutcome::Rejected(event)) => {
                drop(frame);
                break FilterWriteFailure::Rejected(event);
            }
            Err(error) => {
                drop(frame);
                break FilterWriteFailure::Write(error);
            }
        }
    };
    let (progress, maybe_progress_failure) = match response.progress() {
        Ok(progress) => (progress, None),
        Err(error) => (FilterWrittenPrefix::default(), Some(error)),
    };
    let maybe_cleanup = response.cancel().err();
    Err(InboundFilterFailure {
        cause,
        maybe_cleanup,
        maybe_progress_failure,
        progress,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::inbound_listener) enum InboundEmissionWriteResult {
    Written,
    Rejected,
    Disconnected,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::inbound_listener) enum InboundEmissionExecutionOutcome {
    Complete,
    TargetMismatch,
    EncodeFailed,
    Rejected,
    Disconnected,
    WriteFailed,
    CompletionFailed,
    AbortFailed,
}

pub(in crate::inbound_listener) trait InboundEmissionExecutor {
    fn encode(
        &mut self,
        message: &WireNetworkMessage,
    ) -> Result<open_bitcoin_node::network::filter_transport::GenericPendingBuffer, ()>;

    async fn write(&mut self, bytes: &[u8]) -> InboundEmissionWriteResult;

    fn complete(&mut self, receipt: PeerEmissionReceipt) -> Result<(), ()>;

    fn abort(&mut self, capability: PeerEmissionWriteCapability) -> Result<EffectAbort, ()>;

    fn cancellation_cleanup_failed(&mut self, maybe_cause: Option<InboundEmissionExecutionOutcome>);
}

struct PendingEmissions<'a, E: InboundEmissionExecutor> {
    executor: &'a mut E,
    suffix: VecDeque<PeerEmission>,
    maybe_current: Option<PeerEmissionWriteCapability>,
}
impl<E: InboundEmissionExecutor> PendingEmissions<'_, E> {
    fn retire(&mut self) -> bool {
        let mut failed = false;
        if let Some(current) = self.maybe_current.take() {
            failed |= !matches!(self.executor.abort(current), Ok(EffectAbort::Aborted));
        }
        for emission in self.suffix.drain(..) {
            failed |= !matches!(
                self.executor.abort(emission.into_parts().2),
                Ok(EffectAbort::Aborted)
            );
        }
        !failed
    }
    fn fail(&mut self, cause: InboundEmissionExecutionOutcome) -> InboundEmissionExecutionOutcome {
        if self.retire() {
            cause
        } else {
            self.executor.cancellation_cleanup_failed(Some(cause));
            InboundEmissionExecutionOutcome::AbortFailed
        }
    }
}
impl<E: InboundEmissionExecutor> Drop for PendingEmissions<'_, E> {
    fn drop(&mut self) {
        if !self.retire() {
            self.executor.cancellation_cleanup_failed(None);
        }
    }
}

pub(in crate::inbound_listener) async fn execute_inbound_emissions<E: InboundEmissionExecutor>(
    emissions: Vec<PeerEmission>,
    peer_id: u64,
    executor: &mut E,
) -> InboundEmissionExecutionOutcome {
    let mut pending = PendingEmissions {
        executor,
        suffix: VecDeque::from(emissions),
        maybe_current: None,
    };
    while let Some(emission) = pending.suffix.pop_front() {
        let (target_peer_id, message, capability) = emission.into_parts();
        pending.maybe_current = Some(capability);
        if target_peer_id != peer_id {
            return pending.fail(InboundEmissionExecutionOutcome::TargetMismatch);
        }
        let Ok(bytes) = pending.executor.encode(&message) else {
            return pending.fail(InboundEmissionExecutionOutcome::EncodeFailed);
        };
        let result = pending.executor.write(bytes.bytes()).await;
        drop(bytes);
        match result {
            InboundEmissionWriteResult::Written => {
                let Some(capability) = pending.maybe_current.take() else {
                    return pending.fail(InboundEmissionExecutionOutcome::CompletionFailed);
                };
                if pending
                    .executor
                    .complete(capability.acknowledge_write())
                    .is_err()
                {
                    return pending.fail(InboundEmissionExecutionOutcome::CompletionFailed);
                }
            }
            InboundEmissionWriteResult::Rejected => {
                return pending.fail(InboundEmissionExecutionOutcome::Rejected);
            }
            InboundEmissionWriteResult::Disconnected => {
                return pending.fail(InboundEmissionExecutionOutcome::Disconnected);
            }
            InboundEmissionWriteResult::Failed => {
                return pending.fail(InboundEmissionExecutionOutcome::WriteFailed);
            }
        }
    }
    InboundEmissionExecutionOutcome::Complete
}
