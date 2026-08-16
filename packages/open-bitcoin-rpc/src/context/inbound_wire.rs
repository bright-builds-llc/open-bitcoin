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
use open_bitcoin_node::core::consensus::build_compact_block_payload;
use open_bitcoin_node::core::primitives::{NetworkMagic, ScriptWitness};
use open_bitcoin_node::network::{
    ManagedBlockSerializationMode, ManagedBlockServeCompletion, ManagedBlockServeCompletionOutcome,
    ManagedBlockServeIntent, ManagedInboundResponsePlanItem, PeerEmission,
    PeerEmissionWriteCapability,
};

use super::ManagedRpcContext;

pub(crate) struct EncodedWireResponse {
    pub(crate) message: WireNetworkMessage,
    pub(crate) bytes: Vec<u8>,
    pub(crate) maybe_block_serve_intent: Option<ManagedBlockServeIntent>,
    pub(crate) maybe_tx_write_capability: Option<PeerEmissionWriteCapability>,
}

pub(crate) struct InboundWireResponsePlan {
    pub(super) network_magic: NetworkMagic,
    pub(super) responses: Vec<ManagedInboundResponsePlanItem>,
    pub(super) maybe_block_source: Option<Arc<dyn DurableBlockSource>>,
    pub(super) peer_id: u64,
    pub(super) timestamp: i64,
}

pub(crate) struct ResolvedInboundWireResponses {
    pub(crate) responses: Vec<EncodedWireResponse>,
    pub(crate) immediate_completions: Vec<ManagedBlockServeCompletion>,
    pub(crate) tx_write_aborts: Vec<PeerEmissionWriteCapability>,
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

pub(crate) fn durable_block_source(
    maybe_store: Option<FjallNodeStore>,
) -> Option<Arc<dyn DurableBlockSource>> {
    maybe_store.map(|store| Arc::new(store) as Arc<dyn DurableBlockSource>)
}

impl InboundWireResponsePlan {
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
                    resolved.push_encoded(message, None, None, self.network_magic);
                }
                ManagedInboundResponsePlanItem::DurableBlock(intent) => {
                    self.resolve_block_intent(intent, &mut resolved);
                }
                ManagedInboundResponsePlanItem::PreparedTxServe(emission) => {
                    resolved.push_prepared_tx_serve(*emission, self.network_magic);
                }
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
        resolved.push_encoded(response, Some(intent), None, self.network_magic);
    }
}

impl ResolvedInboundWireResponses {
    fn push_prepared_tx_serve(&mut self, emission: PeerEmission, network_magic: NetworkMagic) {
        let (_peer_id, message, capability) = emission.into_parts();
        match message.encode_wire(network_magic) {
            Ok(bytes) => self.responses.push(EncodedWireResponse {
                message,
                bytes,
                maybe_block_serve_intent: None,
                maybe_tx_write_capability: Some(capability),
            }),
            Err(_) => {
                self.tx_write_aborts.push(capability);
                self.failed = true;
            }
        }
    }

    fn push_encoded(
        &mut self,
        message: WireNetworkMessage,
        maybe_block_serve_intent: Option<ManagedBlockServeIntent>,
        maybe_tx_write_capability: Option<PeerEmissionWriteCapability>,
        network_magic: NetworkMagic,
    ) {
        match message.encode_wire(network_magic) {
            Ok(bytes) => self.responses.push(EncodedWireResponse {
                message,
                bytes,
                maybe_block_serve_intent,
                maybe_tx_write_capability,
            }),
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
        resolved.tx_write_aborts.extend(
            resolved
                .responses
                .iter_mut()
                .filter_map(|response| response.maybe_tx_write_capability.take()),
        );
    }
    {
        let context = context.lock().await;
        context
            .complete_block_serves(&resolved.immediate_completions)
            .ok()?;
        for capability in resolved.tx_write_aborts {
            context.abort_peer_emission(capability).ok()?;
        }
    }
    (!resolved.failed).then_some(resolved.responses)
}

pub(crate) async fn acknowledge_encoded_wire_response(
    was_written: bool,
    response: &mut EncodedWireResponse,
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
) -> bool {
    if let Some(capability) = response.maybe_tx_write_capability.take() {
        let context = context.lock().await;
        return if was_written {
            context
                .complete_peer_emission(capability.acknowledge_write())
                .is_ok()
        } else {
            context.abort_peer_emission(capability).is_ok()
        };
    }
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
