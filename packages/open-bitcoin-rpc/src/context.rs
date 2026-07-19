// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use open_bitcoin_network::{
    InboundListenerConfig, InventoryList, PeerPermissionClassRegistry, WireNetworkMessage,
};
use open_bitcoin_node::LogRetentionPolicy;
use open_bitcoin_node::core::consensus::{
    ConsensusParams, ScriptVerifyFlags, build_compact_block_payload,
};
use open_bitcoin_node::core::primitives::{NetworkMagic, ScriptWitness};
use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_node::network::{
    ManagedBlockSerializationMode, ManagedBlockServeCompletion, ManagedBlockServeCompletionOutcome,
    ManagedBlockServeIntent,
};
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
    path::PathBuf,
    sync::{Arc, mpsc},
    time::Duration,
};

use open_bitcoin_node::{FjallNodeStore, ManagedNetworkHandle, PersistMode, RuntimeMetadata};

use crate::inbound_listener::InboundListenerEvidence;
use crate::{RpcFailure, RpcFailureKind};

mod address_boundary;
mod inbound_status;
mod network;
mod peer_policy;
mod rescan;
mod resource_governance;
#[cfg(test)]
mod tests;
mod wallet_state;

pub use inbound_status::AuthoritativeOperatorSnapshot;
pub use rescan::{WalletFreshnessKind, WalletFreshnessView, WalletRescanExecution};
use wallet_state::WalletState;

pub struct ManagedRpcContext {
    chain: AddressNetwork,
    consensus_params: ConsensusParams,
    verify_flags: ScriptVerifyFlags,
    network: ManagedNetworkHandle,
    permission_classes: PeerPermissionClassRegistry,
    inbound_permission_validation_failures: u32,
    inbound_listener_config: InboundListenerConfig,
    maybe_inbound_listener_evidence: Option<InboundListenerEvidence>,
    maybe_resource_governance_log_dir: Option<PathBuf>,
    resource_governance_log_retention: LogRetentionPolicy,
    resource_governance_log_write_failures: u64,
    maybe_block_source: Option<Arc<dyn DurableBlockSource>>,
    maybe_metrics_store: Option<FjallNodeStore>,
    maybe_runtime_metadata_source: Option<FjallNodeStore>,
    maybe_daemon_sync_control: Option<DaemonSyncControl>,
    wallet_state: WalletState,
}

pub(crate) struct EncodedWireResponse {
    pub(crate) message: WireNetworkMessage,
    pub(crate) bytes: Vec<u8>,
    pub(crate) maybe_block_serve_intent: Option<ManagedBlockServeIntent>,
}

pub(crate) struct InboundWireResponsePlan {
    network_magic: NetworkMagic,
    responses: Vec<WireNetworkMessage>,
    block_serve_intents: Vec<ManagedBlockServeIntent>,
    deferred_not_found_responses: Vec<WireNetworkMessage>,
    maybe_block_source: Option<Arc<dyn DurableBlockSource>>,
    peer_id: u64,
    timestamp: i64,
}

pub(crate) struct ResolvedInboundWireResponses {
    pub(crate) responses: Vec<EncodedWireResponse>,
    pub(crate) immediate_completions: Vec<ManagedBlockServeCompletion>,
    pub(crate) failed: bool,
}

pub(crate) trait DurableBlockSource: Send + Sync {
    fn load_block(
        &self,
        block_hash: open_bitcoin_node::core::primitives::BlockHash,
    ) -> Result<Option<open_bitcoin_node::core::primitives::Block>, open_bitcoin_node::StorageError>;
}

impl DurableBlockSource for FjallNodeStore {
    fn load_block(
        &self,
        block_hash: open_bitcoin_node::core::primitives::BlockHash,
    ) -> Result<Option<open_bitcoin_node::core::primitives::Block>, open_bitcoin_node::StorageError>
    {
        FjallNodeStore::load_block(self, block_hash)
    }
}

pub(super) fn durable_block_source(
    maybe_store: Option<FjallNodeStore>,
) -> Option<Arc<dyn DurableBlockSource>> {
    maybe_store.map(|store| Arc::new(store) as Arc<dyn DurableBlockSource>)
}

impl InboundWireResponsePlan {
    pub(crate) fn resolve(mut self) -> ResolvedInboundWireResponses {
        let mut resolved = ResolvedInboundWireResponses {
            responses: Vec::new(),
            immediate_completions: Vec::new(),
            failed: false,
        };
        for response in core::mem::take(&mut self.responses) {
            resolved.push_encoded(response, None, self.network_magic);
        }
        for intent in core::mem::take(&mut self.block_serve_intents) {
            self.resolve_block_intent(intent, &mut resolved);
        }
        for response in core::mem::take(&mut self.deferred_not_found_responses) {
            resolved.push_encoded(response, None, self.network_magic);
        }
        resolved
    }

    fn resolve_block_intent(
        &self,
        intent: ManagedBlockServeIntent,
        resolved: &mut ResolvedInboundWireResponses,
    ) {
        let maybe_block = self
            .maybe_block_source
            .as_ref()
            .map(|source| source.load_block(intent.block_hash()));
        let block = match maybe_block {
            Some(Ok(Some(block))) => block,
            Some(Ok(None)) | Some(Err(_)) | None => {
                resolved
                    .immediate_completions
                    .push(intent.completion(ManagedBlockServeCompletionOutcome::LookupUnavailable));
                resolved.push_encoded(
                    WireNetworkMessage::NotFound(InventoryList::new(vec![
                        intent.request().clone(),
                    ])),
                    None,
                    self.network_magic,
                );
                return;
            }
        };
        let maybe_response = block_serve_response(
            block,
            intent.serialization_mode(),
            request_scoped_compact_nonce(self.peer_id, self.timestamp, &intent),
        );
        let Some(response) = maybe_response else {
            resolved
                .immediate_completions
                .push(intent.completion(ManagedBlockServeCompletionOutcome::TransportFailed));
            resolved.failed = true;
            return;
        };
        resolved.push_encoded(response, Some(intent), self.network_magic);
    }
}

impl ResolvedInboundWireResponses {
    fn push_encoded(
        &mut self,
        message: WireNetworkMessage,
        maybe_block_serve_intent: Option<ManagedBlockServeIntent>,
        network_magic: NetworkMagic,
    ) {
        match message.encode_wire(network_magic) {
            Ok(bytes) => self.responses.push(EncodedWireResponse {
                message,
                bytes,
                maybe_block_serve_intent,
            }),
            Err(_) => {
                if let Some(intent) = maybe_block_serve_intent {
                    self.immediate_completions.push(
                        intent.completion(ManagedBlockServeCompletionOutcome::TransportFailed),
                    );
                }
                self.failed = true;
            }
        }
    }
}

fn block_serve_response(
    mut block: open_bitcoin_node::core::primitives::Block,
    mode: ManagedBlockSerializationMode,
    compact_nonce: u64,
) -> Option<WireNetworkMessage> {
    match mode {
        ManagedBlockSerializationMode::Block => {
            for transaction in &mut block.transactions {
                for input in &mut transaction.inputs {
                    input.witness = ScriptWitness::default();
                }
            }
            Some(WireNetworkMessage::Block(block))
        }
        ManagedBlockSerializationMode::WitnessBlock => Some(WireNetworkMessage::Block(block)),
        ManagedBlockSerializationMode::CompactBlock => {
            build_compact_block_payload(&block, compact_nonce)
                .ok()
                .map(WireNetworkMessage::CompactBlock)
        }
    }
}

fn request_scoped_compact_nonce(
    peer_id: u64,
    timestamp: i64,
    intent: &ManagedBlockServeIntent,
) -> u64 {
    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(peer_id);
    hasher.write_i64(timestamp);
    hasher.write(intent.block_hash().as_bytes());
    hasher.finish()
}

pub(crate) async fn resolve_inbound_wire_responses(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    peer_id: u64,
    message: WireNetworkMessage,
    timestamp: i64,
) -> Option<Vec<EncodedWireResponse>> {
    let plan = {
        let mut context = context.lock().await;
        context
            .prepare_inbound_wire_message(peer_id, message, timestamp)
            .ok()?
    };
    let mut resolved = plan.resolve();
    if resolved.failed {
        resolved.immediate_completions.extend(
            resolved
                .responses
                .iter()
                .filter_map(|response| response.maybe_block_serve_intent.as_ref())
                .map(|intent| {
                    intent.completion(ManagedBlockServeCompletionOutcome::TransportFailed)
                }),
        );
    }
    {
        let context = context.lock().await;
        context
            .complete_block_serves(&resolved.immediate_completions)
            .ok()?;
    }
    (!resolved.failed).then_some(resolved.responses)
}

pub(crate) async fn acknowledge_encoded_wire_response(
    was_written: bool,
    response: &EncodedWireResponse,
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
) -> bool {
    let context = context.lock().await;
    let Some(intent) = response.maybe_block_serve_intent.as_ref() else {
        return !was_written
            || context
                .acknowledge_wire_message_written(&response.message)
                .is_ok();
    };
    let outcome = if was_written {
        ManagedBlockServeCompletionOutcome::Written
    } else {
        ManagedBlockServeCompletionOutcome::TransportFailed
    };
    context
        .complete_block_serves(&[intent.completion(outcome)])
        .is_ok()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaemonSyncControlAction {
    Status,
    Pause,
    Resume,
}

#[derive(Debug)]
pub struct DaemonSyncControlRequest {
    action: DaemonSyncControlAction,
    response_sender: mpsc::Sender<Result<RuntimeMetadata, String>>,
}

impl DaemonSyncControlRequest {
    pub const fn action(&self) -> DaemonSyncControlAction {
        self.action
    }

    pub fn respond(self, result: Result<RuntimeMetadata, String>) {
        let _ = self.response_sender.send(result);
    }
}

#[derive(Debug)]
pub struct DaemonSyncControlReceiver {
    receiver: mpsc::Receiver<DaemonSyncControlRequest>,
}

impl DaemonSyncControlReceiver {
    pub fn try_recv(&self) -> Result<DaemonSyncControlRequest, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    pub fn recv_timeout(
        &self,
        timeout: Duration,
    ) -> Result<DaemonSyncControlRequest, mpsc::RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonSyncControlError {
    message: String,
}

impl DaemonSyncControlError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl core::fmt::Display for DaemonSyncControlError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for DaemonSyncControlError {}

#[derive(Clone)]
pub struct DaemonSyncControl {
    backend: DaemonSyncControlBackend,
}

#[derive(Clone)]
enum DaemonSyncControlBackend {
    Channel {
        sender: mpsc::Sender<DaemonSyncControlRequest>,
        response_timeout: Duration,
    },
    Store {
        store: FjallNodeStore,
        persist_mode: PersistMode,
    },
}

impl DaemonSyncControl {
    pub fn store_backed(store: FjallNodeStore, persist_mode: PersistMode) -> Self {
        Self {
            backend: DaemonSyncControlBackend::Store {
                store,
                persist_mode,
            },
        }
    }

    pub fn channel() -> (Self, DaemonSyncControlReceiver) {
        let (sender, receiver) = mpsc::channel();
        (
            Self {
                backend: DaemonSyncControlBackend::Channel {
                    sender,
                    response_timeout: Duration::from_secs(30),
                },
            },
            DaemonSyncControlReceiver { receiver },
        )
    }

    pub fn status(&self) -> Result<RuntimeMetadata, DaemonSyncControlError> {
        self.request(DaemonSyncControlAction::Status)
    }

    pub fn pause(&self) -> Result<RuntimeMetadata, DaemonSyncControlError> {
        self.request(DaemonSyncControlAction::Pause)
    }

    pub fn resume(&self) -> Result<RuntimeMetadata, DaemonSyncControlError> {
        self.request(DaemonSyncControlAction::Resume)
    }

    fn request(
        &self,
        action: DaemonSyncControlAction,
    ) -> Result<RuntimeMetadata, DaemonSyncControlError> {
        match &self.backend {
            DaemonSyncControlBackend::Channel {
                sender,
                response_timeout,
            } => request_daemon_sync_channel(sender, *response_timeout, action),
            DaemonSyncControlBackend::Store {
                store,
                persist_mode,
            } => request_daemon_sync_store(store, *persist_mode, action),
        }
    }
}

fn request_daemon_sync_channel(
    sender: &mpsc::Sender<DaemonSyncControlRequest>,
    response_timeout: Duration,
    action: DaemonSyncControlAction,
) -> Result<RuntimeMetadata, DaemonSyncControlError> {
    let (response_sender, response_receiver) = mpsc::channel();
    sender
        .send(DaemonSyncControlRequest {
            action,
            response_sender,
        })
        .map_err(|_| DaemonSyncControlError::new("daemon sync control is unavailable"))?;
    response_receiver
        .recv_timeout(response_timeout)
        .map_err(|_| DaemonSyncControlError::new("daemon sync control timed out"))?
        .map_err(DaemonSyncControlError::new)
}

fn request_daemon_sync_store(
    store: &FjallNodeStore,
    persist_mode: PersistMode,
    action: DaemonSyncControlAction,
) -> Result<RuntimeMetadata, DaemonSyncControlError> {
    let mut metadata = load_daemon_sync_metadata(store)?;
    match action {
        DaemonSyncControlAction::Status => Ok(metadata),
        DaemonSyncControlAction::Pause => {
            metadata.sync_control.paused = true;
            save_daemon_sync_metadata(store, persist_mode, &metadata)?;
            Ok(metadata)
        }
        DaemonSyncControlAction::Resume => {
            metadata.sync_control.paused = false;
            save_daemon_sync_metadata(store, persist_mode, &metadata)?;
            Ok(metadata)
        }
    }
}

fn load_daemon_sync_metadata(
    store: &FjallNodeStore,
) -> Result<RuntimeMetadata, DaemonSyncControlError> {
    store
        .load_runtime_metadata()
        .map_err(|error| DaemonSyncControlError::new(error.to_string()))
        .map(|maybe_metadata| maybe_metadata.unwrap_or_default())
}

fn save_daemon_sync_metadata(
    store: &FjallNodeStore,
    persist_mode: PersistMode,
    metadata: &RuntimeMetadata,
) -> Result<(), DaemonSyncControlError> {
    store
        .save_runtime_metadata(metadata, persist_mode)
        .map_err(|error| DaemonSyncControlError::new(error.to_string()))
}

impl core::fmt::Debug for ManagedRpcContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let wallet_mode = match &self.wallet_state {
            WalletState::Local(_) => "local",
            WalletState::DurableNamedRegistry { .. } => "durable",
        };
        f.debug_struct("ManagedRpcContext")
            .field("chain", &self.chain)
            .field("consensus_params", &self.consensus_params)
            .field("verify_flags", &self.verify_flags)
            .field(
                "has_runtime_metadata_source",
                &self.maybe_runtime_metadata_source.is_some(),
            )
            .field(
                "has_daemon_sync_control",
                &self.maybe_daemon_sync_control.is_some(),
            )
            .field(
                "has_resource_governance_log_dir",
                &self.maybe_resource_governance_log_dir.is_some(),
            )
            .field(
                "resource_governance_log_write_failures",
                &self.resource_governance_log_write_failures,
            )
            .field("wallet_mode", &wallet_mode)
            .finish()
    }
}

impl ManagedRpcContext {
    #[cfg(test)]
    pub(crate) fn set_durable_block_source_for_test(
        &mut self,
        source: Arc<dyn DurableBlockSource>,
    ) {
        self.maybe_block_source = Some(source);
    }

    pub(crate) fn prepare_inbound_wire_message(
        &mut self,
        peer_id: u64,
        message: WireNetworkMessage,
        timestamp: i64,
    ) -> Result<InboundWireResponsePlan, open_bitcoin_node::ManagedNetworkAuthorityError> {
        let result = self.network.receive_message_for_durable_serving(
            peer_id,
            message,
            timestamp,
            self.verify_flags,
            self.consensus_params,
        )?;
        let (deferred_not_found_responses, responses) = result
            .outbound
            .into_iter()
            .partition(|response| matches!(response, WireNetworkMessage::NotFound(_)));
        Ok(InboundWireResponsePlan {
            network_magic: network::network_magic(self.chain),
            responses,
            block_serve_intents: result.block_serve_intents,
            deferred_not_found_responses,
            maybe_block_source: self.maybe_block_source.clone(),
            peer_id,
            timestamp,
        })
    }

    pub(crate) fn complete_block_serves(
        &self,
        completions: &[ManagedBlockServeCompletion],
    ) -> Result<(), open_bitcoin_node::ManagedNetworkAuthorityError> {
        for completion in completions {
            self.network.complete_block_serve(completion)?;
        }
        Ok(())
    }

    pub fn set_daemon_sync_control(&mut self, control: DaemonSyncControl) {
        self.maybe_daemon_sync_control = Some(control);
    }

    pub fn daemon_sync_status(&self) -> Result<RuntimeMetadata, RpcFailure> {
        self.daemon_sync_control()?
            .status()
            .map_err(daemon_sync_control_error_to_failure)
    }

    pub fn daemon_sync_pause(&self) -> Result<RuntimeMetadata, RpcFailure> {
        self.daemon_sync_control()?
            .pause()
            .map_err(daemon_sync_control_error_to_failure)
    }

    pub fn daemon_sync_resume(&self) -> Result<RuntimeMetadata, RpcFailure> {
        self.daemon_sync_control()?
            .resume()
            .map_err(daemon_sync_control_error_to_failure)
    }

    fn daemon_sync_control(&self) -> Result<&DaemonSyncControl, RpcFailure> {
        self.maybe_daemon_sync_control.as_ref().ok_or_else(|| {
            RpcFailure::new(
                RpcFailureKind::ClientNotConnected,
                Some(crate::RpcErrorDetail::new(
                    crate::RpcErrorCode::ClientNotConnected,
                    "daemon sync control is unavailable",
                )),
            )
        })
    }
}

fn daemon_sync_control_error_to_failure(error: DaemonSyncControlError) -> RpcFailure {
    RpcFailure::new(
        RpcFailureKind::ClientNotConnected,
        Some(crate::RpcErrorDetail::new(
            crate::RpcErrorCode::ClientNotConnected,
            error.to_string(),
        )),
    )
}
