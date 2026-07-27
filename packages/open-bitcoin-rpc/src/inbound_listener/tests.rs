// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use std::fs;
use std::io;
use std::net::IpAddr;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use open_bitcoin_codec::parse_message_header;
use open_bitcoin_network::{
    BanReason, BanScope, BlockRelayActivationPolicy, BlockServingActivationConfig,
    CompactRelayActivationConfig, INBOUND_MESSAGE_HEADER_LEN, InboundAdmissionSlotClass,
    InboundEnvelopePolicy, InboundHandshakeState, InboundListenerConfig, InboundPreflightReason,
    InboundResourceEvent, InventoryList, MAX_INV_SIZE, PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW,
    PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES, PHASE94_MAX_PEER_READ_QUEUE_BYTES,
    PHASE94_MAX_PEER_WRITE_QUEUE_BYTES, PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW,
    PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS, ParsedNetworkMessage, ParsedPeerPermissionClass,
    PeerBanEntry, PeerConnectionClass, PeerPermissionClassRegistry, ReconnectSuppressionInput,
    ResourceGovernanceDecision, ResourceGovernancePolicy, VersionMessage, WireNetworkMessage,
};
use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_node::core::{
    consensus::{block_hash, block_merkle_root, check_block_header},
    primitives::{
        Amount, Block, BlockHash, BlockHeader, InventoryType, InventoryVector, NetworkMagic,
        OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_node::status::{FieldAvailability, InboundPeerServingStatus};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallNodeStore, ManagedNetworkHandle, PeerIdentityAuthority, PersistMode,
    StorageError, StorageNamespace, StorageRecoveryAction, SyncNetwork, SyncRuntimeConfig,
    sync::AnnouncementOutboxRegistry,
};
use open_bitcoin_test_harness::PortReservation;
use tokio::net::TcpStream;

use crate::{
    ManagedRpcContext, RuntimeConfig,
    context::{DurableBlockSource, EncodedWireResponse},
};

use super::{
    InboundListenerEvidence, InboundListenerState, InboundListenerWorker, InboundRuntimeCounters,
    ReadWireMessageOutcome, RuntimeQueuePressureState, WriteWireMessageOutcome,
    acknowledge_inbound_response_write, activate_inbound_listener, queue_pressure_event,
    read_wire_message, read_wire_message_with_timeout_duration, resolve_inbound_wire_responses,
    resource_timeout_event, start_inbound_accept_loop,
    start_inbound_accept_loop_with_announcements, write_all,
};

const PHASE123_EASY_BITS: u32 = 0x207f_ffff;
static NEXT_DURABLE_BLOCK_SERVING_DIR: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy)]
enum ScriptedDurableBlockFailure {
    Corruption,
    Backend,
}

struct ScriptedDurableBlockSource {
    failure: ScriptedDurableBlockFailure,
}

impl DurableBlockSource for ScriptedDurableBlockSource {
    fn load_block(&self, _block_hash: BlockHash) -> Result<Option<Block>, StorageError> {
        Err(match self.failure {
            ScriptedDurableBlockFailure::Corruption => StorageError::Corruption {
                namespace: StorageNamespace::BlockIndex,
                detail: "private corruption detail".to_string(),
                action: StorageRecoveryAction::Repair,
            },
            ScriptedDurableBlockFailure::Backend => StorageError::BackendFailure {
                namespace: StorageNamespace::BlockIndex,
                message: "private backend detail".to_string(),
                action: StorageRecoveryAction::Restart,
            },
        })
    }
}

fn block_response() -> EncodedWireResponse {
    EncodedWireResponse {
        message: WireNetworkMessage::Block(Block::default()),
        bytes: Vec::new(),
        maybe_block_serve_intent: None,
    }
}

fn non_block_response() -> EncodedWireResponse {
    EncodedWireResponse {
        message: WireNetworkMessage::Verack,
        bytes: Vec::new(),
        maybe_block_serve_intent: None,
    }
}

fn phase123_block_serving_context(enabled: bool) -> (ManagedRpcContext, Block) {
    let permission_classes = loopback_permission_registry(&["in", "download"]);
    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        inbound: InboundListenerConfig {
            enabled: true,
            max_peers: 2,
            reserved_slots: 1,
            permission_classes,
            ..InboundListenerConfig::default()
        },
        block_serving: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled },
            compact_relay: CompactRelayActivationConfig::default(),
        },
        ..RuntimeConfig::default()
    });
    let block = phase123_mined_block();
    context.connect_local_block(&block).expect("connect block");
    let admission = context
        .record_inbound_admission_for_remote_addr(
            123,
            "127.0.0.1:18444".parse().expect("loopback address"),
            false,
        )
        .expect("authoritative inbound admission");
    assert!(matches!(
        admission,
        open_bitcoin_network::InboundAdmissionDecision::Admit(_)
    ));
    for message in [
        WireNetworkMessage::Version(VersionMessage {
            nonce: 123_456,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
    ] {
        context
            .receive_network_message(123, message, 1)
            .expect("complete inbound handshake");
    }
    (context, block)
}

fn phase123_mined_block() -> Block {
    let script_sig = ScriptBuf::from_bytes(vec![0x00, 0x51]).expect("coinbase script");
    let script_pubkey = ScriptBuf::from_bytes(vec![0x51]).expect("output script");
    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig,
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("coinbase amount"),
            script_pubkey,
        }],
        lock_time: 0,
    };
    let (merkle_root, maybe_mutated) =
        block_merkle_root(core::slice::from_ref(&transaction)).expect("coinbase merkle root");
    assert!(!maybe_mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::default(),
            merkle_root,
            time: 1_231_006_500,
            bits: PHASE123_EASY_BITS,
            nonce: 0,
        },
        transactions: vec![transaction],
    };
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("mined nonce");
    block
}

fn phase123_block_request(block: &Block) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash(&block.header).into(),
    }]))
}

fn durable_block_serving_context(persist_block: bool) -> (ManagedRpcContext, Block, PathBuf) {
    let data_dir = std::env::temp_dir().join(format!(
        "open-bitcoin-durable-block-serving-{}-{}",
        process::id(),
        NEXT_DURABLE_BLOCK_SERVING_DIR.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&data_dir);
    let store = FjallNodeStore::open(&data_dir).expect("open durable block-serving store");
    let runtime_config = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        inbound: InboundListenerConfig {
            enabled: true,
            max_peers: 2,
            reserved_slots: 1,
            permission_classes: loopback_permission_registry(&["in", "download"]),
            ..InboundListenerConfig::default()
        },
        block_serving: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
        ..RuntimeConfig::default()
    };
    let block = phase123_mined_block();
    let mut seed_context = ManagedRpcContext::from_runtime_config(&runtime_config);
    seed_context
        .connect_local_block(&block)
        .expect("connect durable block fixture");
    store
        .save_chainstate_snapshot(
            &seed_context
                .blockchain_snapshot()
                .expect("snapshot durable block fixture"),
            PersistMode::Sync,
        )
        .expect("persist durable chainstate");
    if persist_block {
        store
            .save_block(&block, PersistMode::Sync)
            .expect("persist durable block");
    }
    drop(seed_context);

    let sync_runtime = DurableSyncRuntime::open_with_runtime_activation(
        store.clone(),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            dns_seeds: Vec::new(),
            ..SyncRuntimeConfig::default()
        },
        runtime_config.relay,
        runtime_config.block_serving,
        true,
    )
    .expect("reopen durable runtime without block cache hydration");
    let mut context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &runtime_config,
        sync_runtime.network_handle(),
        Some(store),
    )
    .expect("compose durable block-serving context");
    context
        .record_inbound_admission_for_remote_addr(
            123,
            "127.0.0.1:18444".parse().expect("loopback address"),
            false,
        )
        .expect("admit durable block-serving peer");
    for message in [
        WireNetworkMessage::Version(VersionMessage {
            nonce: 123_456,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
    ] {
        context
            .receive_network_message(123, message, 1)
            .expect("complete durable block-serving handshake");
    }
    (context, block, data_dir)
}

fn durable_block_requests(block: &Block) -> WireNetworkMessage {
    let object_hash = block_hash(&block.header).into();
    WireNetworkMessage::GetData(InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash,
        },
        InventoryVector {
            inventory_type: InventoryType::WitnessBlock,
            object_hash,
        },
        InventoryVector {
            inventory_type: InventoryType::CompactBlock,
            object_hash,
        },
    ]))
}

async fn durable_block_failure_outcome(
    failure: ScriptedDurableBlockFailure,
) -> (WireNetworkMessage, Vec<u8>, u64) {
    let (mut context, block, data_dir) = durable_block_serving_context(true);
    context.set_durable_block_source_for_test(Arc::new(ScriptedDurableBlockSource { failure }));
    let context = Arc::new(tokio::sync::Mutex::new(context));
    let mut responses =
        resolve_inbound_wire_responses(&context, 123, phase123_block_request(&block), 2)
            .await
            .expect("map durable block source failure");
    let response = responses.pop().expect("one redacted response");
    let served_count = context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count");
    drop(context);
    fs::remove_dir_all(data_dir).expect("remove failed durable block-serving store");
    (response.message, response.bytes, served_count)
}

fn rejected_write_result() -> io::Result<WriteWireMessageOutcome> {
    Ok(WriteWireMessageOutcome::Rejected(InboundResourceEvent {
        outcome: "resource_governance".to_string(),
        reason: "scripted rejection".to_string(),
        label: "scripted_rejection".to_string(),
        source: "source_runtime_write".to_string(),
        message: "inbound_resource_governance".to_string(),
        next_action: "timeout_disconnect".to_string(),
    }))
}

async fn acknowledged_block_count(
    responses: Vec<EncodedWireResponse>,
    write_results: Vec<io::Result<WriteWireMessageOutcome>>,
) -> u64 {
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::for_local_operator(AddressNetwork::Regtest),
    ));
    for (response, write_result) in responses.iter().zip(write_results.iter()) {
        assert!(acknowledge_inbound_response_write(write_result, response, &context).await);
    }
    context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count")
}

mod admission_and_handshake;
mod block_serving;
mod envelope_and_resource;
mod listener_fixtures;
mod preflight_and_advertisement;
mod reconnect_policy;

use listener_fixtures::loopback_permission_registry;
