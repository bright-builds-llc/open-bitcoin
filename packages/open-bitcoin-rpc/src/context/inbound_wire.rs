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

use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
    sync::Arc,
};

use open_bitcoin_network::{InventoryList, WireNetworkMessage};
use open_bitcoin_node::FjallNodeStore;
use open_bitcoin_node::core::chainstate::{CoinsView, MemoryCoinsView};
use open_bitcoin_node::core::consensus::build_compact_block_payload;
use open_bitcoin_node::core::primitives::ScriptWitness;
use open_bitcoin_node::network::filter_transport::{GenericPendingBuffer, PreparedFilterResponse};
use open_bitcoin_node::network::{
    ManagedBlockSerializationMode, ManagedBlockServeCompletion, ManagedBlockServeCompletionOutcome,
    ManagedBlockServeIntent, ManagedInboundResponsePlanItem, PeerEmissionWriteCapability,
};
use open_bitcoin_node::{ChainstateStore, ManagedNetworkHandle, MemoryChainstateStore};

use super::ManagedRpcContext;

pub(crate) struct EncodedWireResponse {
    pub(crate) message: WireNetworkMessage,
    pub(crate) bytes: GenericPendingBuffer,
    pub(crate) maybe_block_serve_intent: Option<ManagedBlockServeIntent>,
    pub(crate) maybe_tx_write_capability: Option<PeerEmissionWriteCapability>,
}

pub(crate) enum InboundWireStep {
    Encoded(Box<EncodedWireResponse>),
    Filter(PreparedFilterResponse),
}

pub(crate) struct InboundWireResponsePlan<S = MemoryChainstateStore, V: CoinsView = MemoryCoinsView>
{
    pub(super) network: ManagedNetworkHandle<S, V>,
    pub(crate) responses: Vec<ManagedInboundResponsePlanItem>,
    pub(super) maybe_block_source: Option<Arc<dyn DurableBlockSource>>,
    pub(super) peer_id: u64,
    pub(super) timestamp: i64,
}

pub(crate) struct ResolvedInboundWireResponses {
    pub(crate) responses: Vec<InboundWireStep>,
    pub(crate) immediate_completions: Vec<ManagedBlockServeCompletion>,
    pub(crate) tx_write_aborts: Vec<PeerEmissionWriteCapability>,
    pub(crate) failed: bool,
}

pub(crate) trait DurableBlockSource: Send + Sync {
    fn has_block(
        &self,
        block_hash: open_bitcoin_node::core::primitives::BlockHash,
    ) -> Result<bool, open_bitcoin_node::StorageError>;

    fn load_block(
        &self,
        block_hash: open_bitcoin_node::core::primitives::BlockHash,
    ) -> Result<Option<open_bitcoin_node::core::primitives::Block>, open_bitcoin_node::StorageError>;

    fn load_have_pruned(&self) -> Result<bool, open_bitcoin_node::StorageError>;

    /// Whether durable undo exists for `block_hash`.
    ///
    /// The default is false so in-memory callers that do not store undo stay
    /// incomplete until a store override answers.
    fn has_undo(
        &self,
        _block_hash: open_bitcoin_node::core::primitives::BlockHash,
    ) -> Result<bool, open_bitcoin_node::StorageError> {
        Ok(false)
    }
}

impl DurableBlockSource for FjallNodeStore {
    fn has_block(
        &self,
        block_hash: open_bitcoin_node::core::primitives::BlockHash,
    ) -> Result<bool, open_bitcoin_node::StorageError> {
        FjallNodeStore::has_block(self, block_hash)
    }

    fn load_block(
        &self,
        block_hash: open_bitcoin_node::core::primitives::BlockHash,
    ) -> Result<Option<open_bitcoin_node::core::primitives::Block>, open_bitcoin_node::StorageError>
    {
        FjallNodeStore::load_block(self, block_hash)
    }

    fn load_have_pruned(&self) -> Result<bool, open_bitcoin_node::StorageError> {
        FjallNodeStore::load_have_pruned(self)
    }

    fn has_undo(
        &self,
        block_hash: open_bitcoin_node::core::primitives::BlockHash,
    ) -> Result<bool, open_bitcoin_node::StorageError> {
        FjallNodeStore::has_undo(self, block_hash)
    }
}

pub(crate) fn durable_block_source(
    maybe_store: Option<FjallNodeStore>,
) -> Option<Arc<dyn DurableBlockSource>> {
    maybe_store.map(|store| Arc::new(store) as Arc<dyn DurableBlockSource>)
}

impl<S: ChainstateStore, V: CoinsView> InboundWireResponsePlan<S, V> {
    pub(crate) fn resolve(mut self) -> ResolvedInboundWireResponses {
        let mut resolved = ResolvedInboundWireResponses {
            responses: Vec::new(),
            immediate_completions: Vec::new(),
            tx_write_aborts: Vec::new(),
            failed: false,
        };
        for response in core::mem::take(&mut self.responses) {
            match response {
                ManagedInboundResponsePlanItem::Immediate(message) => {
                    resolved.push_encoded(message, None, None, &self.network, self.peer_id);
                }
                ManagedInboundResponsePlanItem::DurableBlock(intent) => {
                    self.resolve_block_intent(intent, &mut resolved);
                }
                ManagedInboundResponsePlanItem::PreparedTxServe(emission) => {
                    let (_, message, capability) = emission.into_parts();
                    resolved.push_encoded(
                        message,
                        None,
                        Some(capability),
                        &self.network,
                        self.peer_id,
                    );
                }
                ManagedInboundResponsePlanItem::PreparedFilterServe(response) => {
                    resolved.responses.push(InboundWireStep::Filter(*response))
                }
                ManagedInboundResponsePlanItem::FilterServeUnavailable(_) => {}
                ManagedInboundResponsePlanItem::FilterRequestIntent(_) => resolved.failed = true,
            }
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
                    None,
                    &self.network,
                    self.peer_id,
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
        resolved.push_encoded(response, Some(intent), None, &self.network, self.peer_id);
    }
}

impl ResolvedInboundWireResponses {
    fn push_encoded<S: ChainstateStore, V: CoinsView>(
        &mut self,
        message: WireNetworkMessage,
        maybe_block_serve_intent: Option<ManagedBlockServeIntent>,
        maybe_tx_write_capability: Option<PeerEmissionWriteCapability>,
        network: &ManagedNetworkHandle<S, V>,
        peer_id: u64,
    ) {
        match network.encode_pending_network_message(peer_id, &message) {
            Ok(bytes) => {
                self.responses
                    .push(InboundWireStep::Encoded(Box::new(EncodedWireResponse {
                        message,
                        bytes,
                        maybe_block_serve_intent,
                        maybe_tx_write_capability,
                    })))
            }
            Err(_) => {
                if let Some(intent) = maybe_block_serve_intent {
                    self.immediate_completions.push(
                        intent.completion(ManagedBlockServeCompletionOutcome::TransportFailed),
                    );
                }
                if let Some(capability) = maybe_tx_write_capability {
                    self.tx_write_aborts.push(capability);
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

pub(crate) async fn resolve_inbound_wire_responses<S, V>(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext<S, V>>>,
    peer_id: u64,
    message: WireNetworkMessage,
    timestamp: i64,
) -> Option<Vec<InboundWireStep>>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let (plan, network) = {
        let mut context = context.lock().await;
        let plan = context
            .prepare_inbound_wire_message(peer_id, message, timestamp)
            .ok()?;
        (plan, context.basic_filter_network_handle())
    };
    let mut resolved = plan.resolve();
    if resolved.failed {
        resolved.immediate_completions.extend(
            resolved
                .responses
                .iter()
                .filter_map(|step| match step {
                    InboundWireStep::Encoded(response) => {
                        response.maybe_block_serve_intent.as_ref()
                    }
                    InboundWireStep::Filter(_) => None,
                })
                .map(|intent| {
                    intent.completion(ManagedBlockServeCompletionOutcome::TransportFailed)
                }),
        );
        resolved
            .tx_write_aborts
            .extend(resolved.responses.iter_mut().filter_map(|step| match step {
                InboundWireStep::Encoded(response) => response.maybe_tx_write_capability.take(),
                InboundWireStep::Filter(_) => None,
            }));
    }
    {
        let mut cleanup_failed = false;
        for completion in &resolved.immediate_completions {
            cleanup_failed |= network.complete_block_serve(completion).is_err();
        }
        for capability in resolved.tx_write_aborts {
            cleanup_failed |= network.abort_peer_emission(capability).is_err();
        }
        resolved.failed |= cleanup_failed;
    }
    if resolved.failed {
        for step in resolved.responses {
            if let InboundWireStep::Filter(response) = step {
                // A terminal registry failure remains visible through its snapshot.
                resolved.failed |= response.cancel().is_err();
            }
        }
        return None;
    }
    Some(resolved.responses)
}

pub(crate) async fn acknowledge_encoded_wire_response<S, V>(
    was_written: bool,
    response: &mut EncodedWireResponse,
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext<S, V>>>,
) -> bool
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let context = context.lock().await;
    if let Some(capability) = response.maybe_tx_write_capability.take() {
        return if was_written {
            context
                .complete_peer_emission(capability.acknowledge_write())
                .is_ok()
        } else {
            context.abort_peer_emission(capability).is_ok()
        };
    }
    let Some(intent) = response.maybe_block_serve_intent.take() else {
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
