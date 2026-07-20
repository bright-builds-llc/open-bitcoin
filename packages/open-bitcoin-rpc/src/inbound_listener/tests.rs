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
    DurableSyncRuntime, FjallNodeStore, ManagedNetworkHandle, PersistMode, StorageError,
    StorageNamespace, StorageRecoveryAction, SyncNetwork, SyncRuntimeConfig,
    sync::AnnouncementOutboxRegistry,
};
use open_bitcoin_test_harness::PortReservation;
use tokio::net::TcpStream;

use crate::{
    ManagedRpcContext, RuntimeConfig,
    context::{DurableBlockSource, EncodedWireResponse},
};

use super::{
    InboundListenerEvidence, InboundListenerState, ReadWireMessageOutcome, WriteWireMessageOutcome,
    acknowledge_inbound_response_write, activate_inbound_listener, resolve_inbound_wire_responses,
    start_inbound_accept_loop, start_inbound_accept_loop_with_announcements,
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

#[tokio::test]
async fn phase123_inbound_written_block_increments_served_once() {
    // Arrange
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::for_local_operator(AddressNetwork::Regtest),
    ));
    let mut responses = context
        .lock()
        .await
        .encode_wire_responses(vec![WireNetworkMessage::Block(Block::default())])
        .expect("block response should encode");
    let response = responses.pop().expect("one encoded block response");
    assert!(matches!(response.message, WireNetworkMessage::Block(_)));
    let write_result = Ok(WriteWireMessageOutcome::Written);

    // Act
    assert!(acknowledge_inbound_response_write(&write_result, &response, &context).await);
    let served_count = context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert_eq!(served_count, 1);
}

#[tokio::test]
async fn phase123_enabled_runtime_config_serves_and_acknowledges_inbound_block() {
    // Arrange
    let (mut context, block) = phase123_block_serving_context(true);
    let responses = context
        .receive_inbound_wire_message(123, phase123_block_request(&block), 2)
        .expect("serve enabled block request");
    let response = responses
        .into_iter()
        .find(|response| matches!(response.message, WireNetworkMessage::Block(_)))
        .expect("enabled runtime should produce a typed Block response");
    let context = Arc::new(tokio::sync::Mutex::new(context));
    let write_result = Ok(WriteWireMessageOutcome::Written);

    // Act
    assert!(acknowledge_inbound_response_write(&write_result, &response, &context).await);
    let served_count = context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert_eq!(served_count, 1);
}

#[tokio::test]
async fn phase123_disabled_runtime_config_does_not_serve_inbound_block() {
    // Arrange
    let (mut context, block) = phase123_block_serving_context(false);

    // Act
    let responses = context
        .receive_inbound_wire_message(123, phase123_block_request(&block), 2)
        .expect("handle disabled block request");
    let served_count = context
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert!(
        !responses
            .iter()
            .any(|response| matches!(response.message, WireNetworkMessage::Block(_)))
    );
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn durable_block_serving_survives_restart_without_cache_hydration() {
    // Arrange
    let (context, block, data_dir) = durable_block_serving_context(true);
    let context = Arc::new(tokio::sync::Mutex::new(context));

    // Act
    let responses =
        resolve_inbound_wire_responses(&context, 123, durable_block_requests(&block), 2)
            .await
            .expect("resolve durable block responses");
    for response in &responses {
        let written = Ok(WriteWireMessageOutcome::Written);
        assert!(acknowledge_inbound_response_write(&written, response, &context).await);
    }
    let served_count = context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert_eq!(responses.len(), 3);
    assert!(matches!(responses[0].message, WireNetworkMessage::Block(_)));
    assert!(matches!(responses[1].message, WireNetworkMessage::Block(_)));
    assert!(matches!(
        responses[2].message,
        WireNetworkMessage::CompactBlock(_)
    ));
    assert_eq!(served_count, 3);
    drop(context);
    fs::remove_dir_all(data_dir).expect("remove durable block-serving store");
}

#[tokio::test]
async fn durable_block_serving_missing_body_returns_notfound_without_served_credit() {
    // Arrange
    let (context, block, data_dir) = durable_block_serving_context(false);
    let context = Arc::new(tokio::sync::Mutex::new(context));

    // Act
    let responses =
        resolve_inbound_wire_responses(&context, 123, phase123_block_request(&block), 2)
            .await
            .expect("resolve missing durable block response");
    let served_count = context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert_eq!(responses.len(), 1);
    assert!(matches!(
        responses[0].message,
        WireNetworkMessage::NotFound(_)
    ));
    assert_eq!(served_count, 0);
    drop(context);
    fs::remove_dir_all(data_dir).expect("remove missing durable block-serving store");
}

#[tokio::test]
async fn durable_block_serving_corruption_is_redacted_as_notfound() {
    // Arrange
    let failure = ScriptedDurableBlockFailure::Corruption;

    // Act
    let (message, bytes, served_count) = durable_block_failure_outcome(failure).await;

    // Assert
    assert!(matches!(message, WireNetworkMessage::NotFound(_)));
    assert!(!bytes.windows(7).any(|window| window == b"private"));
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn durable_block_serving_store_error_is_redacted_as_notfound() {
    // Arrange
    let failure = ScriptedDurableBlockFailure::Backend;

    // Act
    let (message, bytes, served_count) = durable_block_failure_outcome(failure).await;

    // Assert
    assert!(matches!(message, WireNetworkMessage::NotFound(_)));
    assert!(!bytes.windows(7).any(|window| window == b"private"));
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn phase123_inbound_rejected_block_does_not_increment_served() {
    // Arrange
    let responses = vec![block_response()];
    let write_results = vec![rejected_write_result()];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn phase123_inbound_write_error_block_does_not_increment_served() {
    // Arrange
    let responses = vec![block_response()];
    let write_results = vec![Err(io::Error::other("scripted write failure"))];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn phase123_inbound_written_non_block_does_not_increment_served() {
    // Arrange
    let responses = vec![non_block_response()];
    let write_results = vec![Ok(WriteWireMessageOutcome::Written)];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn phase123_inbound_partial_batch_counts_successful_block_prefix() {
    // Arrange
    let responses = vec![block_response(), non_block_response(), block_response()];
    let write_results = vec![
        Ok(WriteWireMessageOutcome::Written),
        Ok(WriteWireMessageOutcome::Written),
        Err(io::Error::other("scripted later write failure")),
    ];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 1);
}

#[tokio::test]
async fn phase123_inbound_two_blocks_before_later_failure_count_two() {
    // Arrange
    let responses = vec![block_response(), block_response(), non_block_response()];
    let write_results = vec![
        Ok(WriteWireMessageOutcome::Written),
        Ok(WriteWireMessageOutcome::Written),
        Err(io::Error::other("scripted later write failure")),
    ];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 2);
}

#[tokio::test]
async fn phase123_inbound_encoding_failure_does_not_increment_served() {
    // Arrange
    let context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    let inventory = InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: BlockHash::default().into(),
    };
    let oversized = WireNetworkMessage::Inv(InventoryList::new(vec![inventory; MAX_INV_SIZE + 1]));

    // Act
    let result = context.encode_wire_responses(vec![oversized]);
    let served_count = context
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert!(result.is_err());
    assert_eq!(served_count, 0);
}

fn loopback_config(max_peers: usize) -> InboundListenerConfig {
    loopback_config_with_permission_classes(max_peers, 0, PeerPermissionClassRegistry::default())
}

fn loopback_config_with_permission_classes(
    max_peers: usize,
    reserved_slots: usize,
    permission_classes: PeerPermissionClassRegistry,
) -> InboundListenerConfig {
    InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers,
        reserved_slots,
        allow_public: false,
        permission_classes,
    }
}

async fn running_loopback_listener(
    max_peers: usize,
) -> (
    Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    super::InboundListenerWorker,
    String,
) {
    running_loopback_listener_with_config(loopback_config(max_peers)).await
}

async fn running_loopback_listener_with_config(
    inbound: InboundListenerConfig,
) -> (
    Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    super::InboundListenerWorker,
    String,
) {
    let runtime = RuntimeConfig {
        inbound,
        ..RuntimeConfig::default()
    };
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_runtime_config(&runtime),
    ));
    let activation = activate_inbound_listener(&runtime.inbound).await;
    let endpoint = activation
        .bound_endpoints()
        .first()
        .expect("bound loopback endpoint")
        .bound_endpoint
        .clone();
    let worker = start_inbound_accept_loop(activation, Arc::clone(&context))
        .expect("listener worker should start");
    (context, worker, endpoint)
}

async fn running_loopback_listener_with_announcements() -> (
    Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    super::InboundListenerWorker,
    String,
    AnnouncementOutboxRegistry,
    ManagedNetworkHandle,
) {
    let runtime = RuntimeConfig {
        inbound: loopback_config(2),
        block_serving: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
        ..RuntimeConfig::default()
    };
    let network = ManagedNetworkHandle::transient_runtime(
        NetworkMagic::MAINNET,
        8_333,
        runtime.relay,
        runtime.block_serving,
        true,
    );
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_runtime_config_with_network_handle(&runtime, network.clone(), None)
            .expect("compose announcement listener context"),
    ));
    let outboxes = AnnouncementOutboxRegistry::default();
    let activation = activate_inbound_listener(&runtime.inbound).await;
    let endpoint = activation
        .bound_endpoints()
        .first()
        .expect("bound announcement loopback endpoint")
        .bound_endpoint
        .clone();
    let worker = start_inbound_accept_loop_with_announcements(
        activation,
        Arc::clone(&context),
        outboxes.clone(),
        network.clone(),
    )
    .expect("announcement listener worker should start");
    (context, worker, endpoint, outboxes, network)
}

fn loopback_permission_registry(permissions: &[&str]) -> PeerPermissionClassRegistry {
    PeerPermissionClassRegistry::new([ParsedPeerPermissionClass::parse(
        "loopback-permission",
        ["127.0.0.1"],
        permissions.iter().copied(),
    )
    .expect("loopback permission class should parse")])
}

fn listener_evidence(bound_endpoints: &[&str]) -> InboundListenerEvidence {
    InboundListenerEvidence {
        listener_state: "listening".to_string(),
        preflight_reason: "ready".to_string(),
        bound_endpoints: bound_endpoints
            .iter()
            .map(|endpoint| (*endpoint).to_string())
            .collect(),
        admitted_inbound_peers: 0,
        rejected_inbound_peers: 0,
        resource_rejections: 0,
        timeout_disconnects: 0,
        churn_rejections: 0,
        reconnect_suppressions: 0,
        maybe_admission_reject_reason: None,
        maybe_latest_admission_event: Some("ready".to_string()),
        maybe_latest_resource_event: None,
    }
}

fn peer_policy_entry(scope: BanScope, expires_at_unix_seconds: i64) -> PeerBanEntry {
    PeerBanEntry {
        scope,
        reason: BanReason::Manual,
        created_at_unix_seconds: 100,
        expires_at_unix_seconds,
        source: "runtime_test",
    }
}

#[test]
fn reconnect_suppression_uses_matching_remote_policy_state() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .record_peer_policy_ban(
            peer_policy_entry(BanScope::Address(IpAddr::from([127, 0, 0, 2])), 300),
            150,
        )
        .expect("authoritative peer policy");

    // Act
    let reconnect = context
        .reconnect_suppression_input_for_remote_addr(
            "127.0.0.2:18444".parse().expect("valid remote addr"),
            150,
        )
        .expect("authoritative reconnect state");

    // Assert
    assert!(reconnect.banned);
    assert!(!reconnect.discouraged);
}

#[test]
fn reconnect_suppression_ignores_non_matching_remote_policy_state() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .record_peer_policy_ban(
            peer_policy_entry(BanScope::Address(IpAddr::from([127, 0, 0, 2])), 300),
            150,
        )
        .expect("authoritative peer policy");

    // Act
    let reconnect = context
        .reconnect_suppression_input_for_remote_addr(
            "127.0.0.3:18444".parse().expect("valid remote addr"),
            150,
        )
        .expect("authoritative reconnect state");

    // Assert
    assert!(!reconnect.banned);
    assert!(!reconnect.discouraged);
}

#[test]
fn listener_records_scoped_banned_reconnect_suppression() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .record_peer_policy_ban(
            peer_policy_entry(BanScope::Address(IpAddr::from([127, 0, 0, 1])), 300),
            150,
        )
        .expect("authoritative peer policy");
    let mut evidence = listener_evidence(&["127.0.0.1:18444"]);
    let reconnect = context
        .reconnect_suppression_input_for_remote_addr(
            "127.0.0.1:18444".parse().expect("valid remote addr"),
            150,
        )
        .expect("authoritative reconnect state");
    let event = match ResourceGovernancePolicy::default().decide_reconnect(reconnect) {
        ResourceGovernanceDecision::Disconnect(event) => event,
        other => panic!("expected reconnect_suppressed_banned event, got {other:?}"),
    };

    // Act
    evidence.record_resource_event(event);

    // Assert
    assert_eq!(evidence.reconnect_suppressions, 1);
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .expect("latest resource event")
            .label,
        "reconnect_suppressed_banned"
    );
}

#[test]
fn listener_records_scoped_discouraged_reconnect_suppression() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .record_peer_policy_discouragement(
            peer_policy_entry(BanScope::Address(IpAddr::from([127, 0, 0, 1])), 300),
            150,
        )
        .expect("authoritative peer policy");
    let mut evidence = listener_evidence(&["127.0.0.1:18444"]);
    let reconnect = context
        .reconnect_suppression_input_for_remote_addr(
            "127.0.0.1:18444".parse().expect("valid remote addr"),
            150,
        )
        .expect("authoritative reconnect state");
    let event = match ResourceGovernancePolicy::default().decide_reconnect(reconnect) {
        ResourceGovernanceDecision::Backpressure(event) => event,
        other => panic!("expected reconnect_suppressed_discouraged event, got {other:?}"),
    };

    // Act
    evidence.record_resource_event(event);

    // Assert
    assert_eq!(evidence.reconnect_suppressions, 1);
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .expect("latest resource event")
            .label,
        "reconnect_suppressed_discouraged"
    );
}

fn inbound_status(context: &ManagedRpcContext) -> InboundPeerServingStatus {
    match context.current_inbound_status() {
        FieldAvailability::Available(status) => status,
        FieldAvailability::Unavailable { reason } => {
            panic!("expected inbound status to be available, got {reason}")
        }
    }
}

async fn send_message(stream: &TcpStream, message: WireNetworkMessage) {
    let encoded = message
        .encode_wire(NetworkMagic::MAINNET)
        .expect("encode wire message");
    super::write_all(stream, &encoded)
        .await
        .expect("write wire message");
}

async fn receive_message(stream: &TcpStream) -> WireNetworkMessage {
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let outcome = super::read_wire_message(stream, &policy)
        .await
        .expect("read wire message");
    match outcome {
        ReadWireMessageOutcome::Message(parsed) => parsed.message,
        ReadWireMessageOutcome::Rejected(event) => {
            panic!("expected inbound response message, got {}", event.label)
        }
    }
}

async fn receive_any_message(stream: &TcpStream) -> WireNetworkMessage {
    let mut buffered = Vec::new();
    loop {
        if buffered.len() >= INBOUND_MESSAGE_HEADER_LEN {
            let header = parse_message_header(&buffered[..INBOUND_MESSAGE_HEADER_LEN])
                .expect("response header should decode");
            let frame_len = INBOUND_MESSAGE_HEADER_LEN + header.payload_size as usize;
            if buffered.len() >= frame_len {
                return ParsedNetworkMessage::decode_wire(&buffered[..frame_len])
                    .expect("response should decode")
                    .message;
            }
        }

        stream
            .readable()
            .await
            .expect("response stream should become readable");
        let mut bytes = [0_u8; 4_096];
        match stream.try_read(&mut bytes) {
            Ok(0) => panic!("listener closed before a complete response"),
            Ok(count) => buffered.extend_from_slice(&bytes[..count]),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => panic!("response read failed: {error}"),
        }
    }
}

async fn tcp_pair() -> (TcpStream, TcpStream) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind loopback test listener");
    let endpoint = listener.local_addr().expect("loopback listener address");
    let client = TcpStream::connect(endpoint)
        .await
        .expect("connect loopback test client");
    let (server, _) = listener.accept().await.expect("accept loopback test peer");
    (client, server)
}

fn verack_header(magic: NetworkMagic) -> [u8; INBOUND_MESSAGE_HEADER_LEN] {
    let encoded = WireNetworkMessage::Verack
        .encode_wire(magic)
        .expect("encode verack message");
    encoded[..INBOUND_MESSAGE_HEADER_LEN]
        .try_into()
        .expect("encoded message should include header")
}

fn unsupported_command_header() -> [u8; INBOUND_MESSAGE_HEADER_LEN] {
    let mut header = verack_header(NetworkMagic::MAINNET);
    header[4..16].fill(0);
    header[4..11].copy_from_slice(b"mempool");
    header
}

async fn read_rejected_header(header: [u8; INBOUND_MESSAGE_HEADER_LEN]) -> String {
    // Arrange
    let (client, server) = tcp_pair().await;
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);

    // Act
    super::write_all(&client, &header)
        .await
        .expect("write header under test");
    let outcome = super::read_wire_message(&server, &policy)
        .await
        .expect("read rejected header");

    // Assert
    match outcome {
        ReadWireMessageOutcome::Rejected(event) => event.label,
        ReadWireMessageOutcome::Message(_) => {
            panic!("expected inbound envelope policy to reject header")
        }
    }
}

async fn wait_for_inbound_peers(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    expected: usize,
) {
    for _ in 0..100 {
        if context
            .lock()
            .await
            .network_info()
            .is_ok_and(|info| info.inbound_peers == expected)
        {
            return;
        }
        tokio::task::yield_now().await;
    }
}

async fn wait_for_reserved_slot_rejections(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    expected: usize,
) {
    for _ in 0..100 {
        if context
            .lock()
            .await
            .inbound_admission_info()
            .is_ok_and(|info| info.reserved_slot_rejections == expected)
        {
            return;
        }
        tokio::task::yield_now().await;
    }
}

#[tokio::test]
async fn disabled_runtime_reports_disabled_without_bound_endpoints() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: false,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
        permission_classes: Default::default(),
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Disabled);
    assert_eq!(
        activation.preflight_reason(),
        InboundPreflightReason::Disabled
    );
    assert!(activation.bound_endpoints().is_empty());
    assert_eq!(
        activation
            .latest_admission_event()
            .expect("listener activation should record a latest event"),
        "disabled"
    );
}

#[tokio::test]
async fn invalid_endpoint_reports_typed_diagnostic_before_bind() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["not-a-socket-address".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
        permission_classes: Default::default(),
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Blocked);
    assert_eq!(
        activation.preflight_reason(),
        InboundPreflightReason::InvalidEndpoint
    );
    assert!(activation.bound_endpoints().is_empty());
    assert_eq!(
        activation.diagnostics()[0].maybe_endpoint.as_deref(),
        Some("not-a-socket-address")
    );
}

#[tokio::test]
async fn unsafe_public_endpoint_reports_typed_diagnostic_before_bind() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["0.0.0.0:18444".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
        permission_classes: Default::default(),
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Blocked);
    assert_eq!(
        activation.preflight_reason(),
        InboundPreflightReason::UnsafeEndpoint
    );
    assert!(activation.bound_endpoints().is_empty());
    assert_eq!(
        activation.diagnostics()[0].maybe_endpoint.as_deref(),
        Some("0.0.0.0:18444")
    );
}

#[tokio::test]
async fn held_loopback_address_reports_bind_failure_with_next_action() {
    // Arrange
    let held = PortReservation::localhost().expect("held loopback port");
    let config = InboundListenerConfig {
        enabled: true,
        listen_addresses: vec![held.address().to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
        permission_classes: Default::default(),
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Blocked);
    assert!(matches!(
        activation.preflight_reason(),
        InboundPreflightReason::AlreadyBound | InboundPreflightReason::BindUnavailable
    ));
    assert!(activation.bound_endpoints().is_empty());
    assert_eq!(
        activation.diagnostics()[0].maybe_endpoint.as_deref(),
        Some(held.address().to_string().as_str())
    );
    assert!(!activation.diagnostics()[0].next_action.is_empty());
}

#[test]
fn loopback_listener_evidence_is_suppressed_for_public_advertisement() {
    // Arrange
    let runtime = RuntimeConfig {
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:18444".to_string()],
            max_peers: 8,
            reserved_slots: 0,
            allow_public: false,
            permission_classes: Default::default(),
        },
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);

    // Act
    context
        .set_inbound_listener_evidence(listener_evidence(&["127.0.0.1:18444"]))
        .expect("authoritative listener evidence");
    let status = inbound_status(&context);

    // Assert
    assert!(status.local_advertisement_candidates.is_empty());
    assert_eq!(status.suppressed_advertisements.len(), 1);
    assert_eq!(
        status.suppressed_advertisements[0].label,
        "advertise_suppressed"
    );
    assert_eq!(
        status.suppressed_advertisements[0].reason,
        "not_publicly_routable"
    );
}

#[test]
fn public_literal_listener_evidence_can_be_advertisement_candidate_when_allowed() {
    // Arrange
    let runtime = RuntimeConfig {
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["8.8.8.8:8333".to_string()],
            max_peers: 8,
            reserved_slots: 0,
            allow_public: true,
            permission_classes: Default::default(),
        },
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);

    // Act
    context
        .set_inbound_listener_evidence(listener_evidence(&["8.8.8.8:8333"]))
        .expect("authoritative listener evidence");
    let status = inbound_status(&context);

    // Assert
    assert_eq!(status.local_advertisement_candidates.len(), 1);
    assert_eq!(
        status.local_advertisement_candidates[0].source,
        "source_local_listener"
    );
    assert_eq!(
        status.local_advertisement_candidates[0].network_kind,
        "ipv4"
    );
    assert_eq!(
        status.local_advertisement_candidates[0].routability,
        "publicly_routable"
    );
    assert_eq!(status.local_advertisement_candidates[0].port, 8333);
    assert!(status.suppressed_advertisements.is_empty());
}

#[test]
fn invalid_runtime_bound_evidence_is_suppressed_without_falling_back_to_configured_public_address()
{
    // Arrange
    let runtime = RuntimeConfig {
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["8.8.8.8:8333".to_string()],
            max_peers: 8,
            reserved_slots: 0,
            allow_public: true,
            permission_classes: Default::default(),
        },
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);

    // Act
    context
        .set_inbound_listener_evidence(listener_evidence(&["not-a-socket-address"]))
        .expect("authoritative listener evidence");
    let status = inbound_status(&context);

    // Assert
    assert!(status.local_advertisement_candidates.is_empty());
    assert_eq!(status.suppressed_advertisements.len(), 1);
    assert_eq!(
        status.suppressed_advertisements[0].label,
        "advertise_suppressed"
    );
    assert_eq!(
        status.suppressed_advertisements[0].reason,
        "unsupported_address_network"
    );
}

#[tokio::test]
async fn enabled_loopback_zero_port_binds_without_public_network_dependency() {
    // Arrange
    let config = InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers: 8,
        reserved_slots: 0,
        allow_public: false,
        permission_classes: Default::default(),
    };

    // Act
    let activation = activate_inbound_listener(&config).await;

    // Assert
    assert_eq!(activation.state(), InboundListenerState::Listening);
    assert_eq!(activation.preflight_reason(), InboundPreflightReason::Ready);
    let endpoint = activation
        .bound_endpoints()
        .first()
        .expect("loopback endpoint should bind");
    assert!(endpoint.bound_endpoint.starts_with("127.0.0.1:"));
    assert_ne!(endpoint.bound_endpoint, "127.0.0.1:0");
}

#[tokio::test]
async fn ordinary_loopback_inbound_cannot_consume_reserved_capacity() {
    // Arrange
    let config =
        loopback_config_with_permission_classes(2, 1, PeerPermissionClassRegistry::default());
    let (context, worker, endpoint) = running_loopback_listener_with_config(config).await;
    let first = TcpStream::connect(&endpoint)
        .await
        .expect("connect first ordinary loopback peer");
    wait_for_inbound_peers(&context, 1).await;

    // Act
    let second = TcpStream::connect(&endpoint)
        .await
        .expect("connect second ordinary loopback peer");
    drop(second);
    wait_for_reserved_slot_rejections(&context, 1).await;
    let admission = context
        .lock()
        .await
        .inbound_admission_info()
        .expect("authoritative inbound admission");

    // Assert
    assert_eq!(admission.ordinary_inbound_admits, 1);
    assert_eq!(admission.permissioned_inbound_admits, 0);
    assert_eq!(admission.protected_inbound_admits, 0);
    assert_eq!(admission.reserved_inbound_admits, 0);
    assert_eq!(admission.rejected_inbound_peers, 1);
    assert_eq!(admission.reserved_slot_rejections, 1);
    drop(first);
    worker.shutdown().await;
}

#[tokio::test]
async fn protected_loopback_inbound_consumes_reserved_capacity() {
    // Arrange
    let config = loopback_config_with_permission_classes(
        2,
        1,
        loopback_permission_registry(&["in", "noban", "forceinbound"]),
    );
    let (context, worker, endpoint) = running_loopback_listener_with_config(config).await;

    // Act
    let first = TcpStream::connect(&endpoint)
        .await
        .expect("connect protected loopback peer");
    wait_for_inbound_peers(&context, 1).await;
    let permission_decision = context
        .lock()
        .await
        .permission_decision_for_remote_addr("127.0.0.1:50000".parse().expect("remote address"));
    let admission = context
        .lock()
        .await
        .inbound_admission_info()
        .expect("authoritative inbound admission");

    // Assert
    assert_eq!(
        permission_decision.connection_class(),
        PeerConnectionClass::ProtectedInbound
    );
    assert_eq!(
        permission_decision.slot_class(),
        InboundAdmissionSlotClass::Reserved
    );
    assert_eq!(admission.ordinary_inbound_admits, 0);
    assert_eq!(admission.permissioned_inbound_admits, 0);
    assert_eq!(admission.protected_inbound_admits, 1);
    assert_eq!(admission.reserved_inbound_admits, 1);
    assert_eq!(admission.active_permission_effect_observations, 4);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
    drop(first);
    worker.shutdown().await;
}

#[tokio::test]
async fn permissioned_loopback_inbound_uses_ordinary_capacity_with_scoped_filter_evidence() {
    // Arrange
    let config = loopback_config_with_permission_classes(
        2,
        1,
        loopback_permission_registry(&[
            "in",
            "download",
            "addr",
            "relay",
            "forcerelay",
            "mempool",
            "bloomfilter",
            "blockfilters",
        ]),
    );
    let (context, worker, endpoint) = running_loopback_listener_with_config(config).await;
    let first = TcpStream::connect(&endpoint)
        .await
        .expect("connect first permissioned loopback peer");
    wait_for_inbound_peers(&context, 1).await;

    // Act
    let second = TcpStream::connect(&endpoint)
        .await
        .expect("connect second permissioned loopback peer");
    drop(second);
    wait_for_reserved_slot_rejections(&context, 1).await;
    let permission_decision = context
        .lock()
        .await
        .permission_decision_for_remote_addr("127.0.0.1:50000".parse().expect("remote address"));
    let admission = context
        .lock()
        .await
        .inbound_admission_info()
        .expect("authoritative inbound admission");
    let network_info = context
        .lock()
        .await
        .network_info()
        .expect("authoritative network info");

    // Assert
    assert_eq!(
        permission_decision.connection_class(),
        PeerConnectionClass::PermissionedInbound
    );
    assert_eq!(
        permission_decision.slot_class(),
        InboundAdmissionSlotClass::Ordinary
    );
    assert_eq!(admission.ordinary_inbound_admits, 0);
    assert_eq!(admission.permissioned_inbound_admits, 1);
    assert_eq!(admission.protected_inbound_admits, 0);
    assert_eq!(admission.reserved_inbound_admits, 0);
    assert_eq!(admission.active_permission_effect_observations, 2);
    assert_eq!(admission.inactive_permission_effect_observations, 2);
    assert_eq!(admission.reserved_slot_rejections, 1);
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 0);
    drop(first);
    worker.shutdown().await;
}

#[tokio::test]
async fn loopback_inbound_peer_handshake_increments_inbound_without_outbound() {
    // Arrange
    let (context, worker, endpoint) = running_loopback_listener(2).await;
    let stream = TcpStream::connect(&endpoint)
        .await
        .expect("connect loopback inbound listener");
    let remote_version = VersionMessage {
        nonce: 42,
        ..VersionMessage::default()
    };

    // Act
    send_message(&stream, WireNetworkMessage::Version(remote_version)).await;
    let responses = [
        receive_message(&stream).await,
        receive_message(&stream).await,
        receive_message(&stream).await,
        receive_message(&stream).await,
    ];
    send_message(&stream, WireNetworkMessage::Verack).await;
    let network_info = context
        .lock()
        .await
        .network_info()
        .expect("authoritative network info");

    // Assert
    assert!(matches!(responses[0], WireNetworkMessage::Version(_)));
    assert!(matches!(responses[1], WireNetworkMessage::WtxidRelay));
    assert!(matches!(responses[2], WireNetworkMessage::Verack));
    assert!(matches!(responses[3], WireNetworkMessage::SendHeaders));
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 0);
    worker.shutdown().await;
}

#[tokio::test]
async fn idle_inbound_peer_wakes_for_queued_announcement_and_credits_once() {
    // Arrange
    let (_context, worker, endpoint, outboxes, network) =
        running_loopback_listener_with_announcements().await;
    let stream = TcpStream::connect(&endpoint)
        .await
        .expect("connect announcement loopback peer");
    send_message(
        &stream,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 128,
            ..VersionMessage::default()
        }),
    )
    .await;
    for _ in 0..4 {
        let _ = receive_message(&stream).await;
    }
    send_message(&stream, WireNetworkMessage::Verack).await;
    let compact_offer = receive_any_message(&stream).await;
    assert!(matches!(compact_offer, WireNetworkMessage::SendCompact(_)));
    let snapshots = outboxes.snapshots().expect("registered inbound outbox");
    let peer_id = snapshots
        .first()
        .expect("one registered inbound outbox")
        .peer_id();
    let block = Block::default();
    let outcomes = network
        .prepare_block_announcements(&block, &snapshots)
        .expect("prepare idle inbound announcement");

    // Act
    outboxes
        .enqueue_prepared(outcomes)
        .expect("enqueue idle inbound announcement");
    let announcement = tokio::time::timeout(Duration::from_secs(1), receive_any_message(&stream))
        .await
        .expect("idle inbound peer should wake without another socket message");
    tokio::task::yield_now().await;
    let evidence = serde_json::to_value(
        network
            .block_relay_evidence_status()
            .expect("announcement evidence"),
    )
    .expect("serialize announcement evidence");

    // Assert
    assert!(matches!(announcement, WireNetworkMessage::Inv(_)));
    assert_eq!(
        evidence["announcement"]["value"]["compact_inventory_fallback_count"],
        1
    );
    assert!(
        outboxes
            .take_peer_emissions(peer_id)
            .expect("drained inbound outbox")
            .is_empty()
    );
    worker.shutdown().await;
}

#[tokio::test]
async fn dropped_loopback_inbound_releases_capacity_for_next_peer() {
    // Arrange
    let (context, worker, endpoint) = running_loopback_listener(1).await;
    let first = TcpStream::connect(&endpoint)
        .await
        .expect("connect first loopback peer");
    send_message(
        &first,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 43,
            ..VersionMessage::default()
        }),
    )
    .await;
    for _ in 0..100 {
        if context
            .lock()
            .await
            .network_info()
            .is_ok_and(|info| info.inbound_peers == 1)
        {
            break;
        }
        tokio::task::yield_now().await;
    }
    drop(first);
    wait_for_inbound_peers(&context, 0).await;

    // Act
    let second = TcpStream::connect(&endpoint)
        .await
        .expect("connect next loopback peer after drop");
    send_message(
        &second,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 44,
            ..VersionMessage::default()
        }),
    )
    .await;
    wait_for_inbound_peers(&context, 1).await;
    let network_info = context
        .lock()
        .await
        .network_info()
        .expect("authoritative network info");
    let admission = context
        .lock()
        .await
        .inbound_admission_info()
        .expect("authoritative inbound admission");
    let evidence = worker.evidence();

    // Assert
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 0);
    assert_eq!(admission.admitted_inbound_peers, 2);
    assert_eq!(admission.rejected_inbound_peers, 0);
    assert_eq!(admission.cap_rejections, 0);
    assert_eq!(evidence.maybe_admission_reject_reason, None);
    drop(second);
    worker.shutdown().await;
}

#[tokio::test]
async fn oversized_header_returns_payload_oversized_before_payload_allocation() {
    // Arrange
    let mut header = verack_header(NetworkMagic::MAINNET);
    let oversized_len = (PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES as u32)
        .saturating_add(1)
        .to_le_bytes();
    header[16..20].copy_from_slice(&oversized_len);

    // Act
    let label = read_rejected_header(header).await;

    // Assert
    assert_eq!(label, "payload_oversized");
}

#[tokio::test]
async fn wrong_magic_returns_wrong_network_magic_and_closes_message_loop() {
    // Arrange
    let regtest_magic = NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]);
    let header = verack_header(regtest_magic);

    // Act
    let label = read_rejected_header(header).await;

    // Assert
    assert_eq!(label, "wrong_network_magic");
}

#[tokio::test]
async fn unsupported_command_records_evidence_without_receive_inbound_wire_message() {
    // Arrange
    let (context, worker, endpoint) = running_loopback_listener(2).await;
    let stream = TcpStream::connect(&endpoint)
        .await
        .expect("connect loopback inbound listener");
    let header = unsupported_command_header();

    // Act
    super::write_all(&stream, &header)
        .await
        .expect("write unsupported command header");
    for _ in 0..100 {
        if worker
            .evidence()
            .maybe_latest_resource_event
            .as_ref()
            .is_some_and(|event| event.label == "unsupported_command")
        {
            break;
        }
        tokio::task::yield_now().await;
    }
    let evidence = worker.evidence();
    let network_info = context
        .lock()
        .await
        .network_info()
        .expect("authoritative network info");

    // Assert
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .expect("resource event should be recorded")
            .label,
        "unsupported_command"
    );
    assert_eq!(
        evidence.maybe_latest_admission_event.as_deref(),
        Some("admitted")
    );
    assert_eq!(network_info.outbound_peers, 0);
    worker.shutdown().await;
}

#[test]
fn record_resource_event_counts_timeout_churn_and_reconnect_actions() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut evidence = listener_evidence(&["127.0.0.1:18444"]);
    let timeout = super::resource_timeout_event(
        &policy,
        10,
        10,
        10 + PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS + 1,
        InboundHandshakeState::Handshaking,
    )
    .expect("slow_handshake timeout event");
    let churn = match policy.decide_churn(open_bitcoin_network::ConnectionChurnInput {
        window_started_unix_seconds: 10,
        now_unix_seconds: 10,
        connection_attempts_in_window: PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW + 1,
    }) {
        ResourceGovernanceDecision::Backpressure(event) => event,
        other => panic!("expected connection_churn_limited event, got {other:?}"),
    };
    let reconnect = match policy.decide_reconnect(ReconnectSuppressionInput {
        banned: true,
        discouraged: false,
    }) {
        ResourceGovernanceDecision::Disconnect(event) => event,
        other => panic!("expected reconnect_suppressed event, got {other:?}"),
    };

    // Act
    evidence.record_resource_event(timeout);
    evidence.record_resource_event(churn);
    evidence.record_resource_event(reconnect);

    // Assert
    assert_eq!(evidence.timeout_disconnects, 1);
    assert_eq!(evidence.churn_rejections, 1);
    assert_eq!(evidence.reconnect_suppressions, 1);
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .expect("latest resource event")
            .label,
        "reconnect_suppressed_banned"
    );
}

#[test]
fn resource_timeout_event_distinguishes_slow_handshake_and_idle_peer() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();

    // Act
    let slow_handshake = super::resource_timeout_event(
        &policy,
        100,
        100,
        100 + policy.slow_handshake_timeout_seconds + 1,
        InboundHandshakeState::Accepted,
    )
    .expect("slow_handshake timeout");
    let idle_peer = super::resource_timeout_event(
        &policy,
        100,
        200,
        200 + policy.idle_peer_timeout_seconds + 1,
        InboundHandshakeState::Established,
    )
    .expect("idle_peer timeout");

    // Assert
    assert_eq!(slow_handshake.label, "slow_handshake");
    assert_eq!(slow_handshake.next_action, "timeout_disconnect");
    assert_eq!(idle_peer.label, "idle_peer");
    assert_eq!(idle_peer.next_action, "timeout_disconnect");
}

#[test]
fn runtime_window_counters_limit_churn_and_repeated_failures() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut counters = super::InboundRuntimeCounters::new(1_000);

    // Act
    let mut latest_churn = ResourceGovernanceDecision::Accept;
    for _ in 0..=PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW {
        let input = counters.record_connection_attempt(&policy, 1_000);
        latest_churn = policy.decide_churn(input);
    }
    for _ in 0..=PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW {
        counters.record_failure(&policy, 1_000);
    }
    let repeated_failure =
        policy.decide_repeated_failure(counters.repeated_failure_input(&policy, 1_000));

    // Assert
    let ResourceGovernanceDecision::Backpressure(churn_event) = latest_churn else {
        panic!("expected connection_churn_limited backpressure");
    };
    assert_eq!(churn_event.label, "connection_churn_limited");
    let ResourceGovernanceDecision::Backpressure(failure_event) = repeated_failure else {
        panic!("expected repeated_failure_limited backpressure");
    };
    assert_eq!(failure_event.label, "repeated_failure_limited");
    assert_eq!(failure_event.next_action, "churn_rejected");
}

#[test]
fn read_queue_pressure_is_decided_before_socket_read() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut queue = super::RuntimeQueuePressureState::default();
    queue.record_pending_read(PHASE94_MAX_PEER_READ_QUEUE_BYTES + 1);

    // Act
    let event = super::queue_pressure_event(&policy, &queue, Vec::new(), Vec::new())
        .expect("read queue pressure event");

    // Assert
    assert_eq!(event.label, "read_queue_pressure");
    assert_eq!(event.next_action, "read_queue_pressure");
}

#[test]
fn write_queue_pressure_skips_socket_write() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut queue = super::RuntimeQueuePressureState::default();
    queue.record_pending_write(PHASE94_MAX_PEER_WRITE_QUEUE_BYTES + 1);

    // Act
    let event = super::queue_pressure_event(&policy, &queue, Vec::new(), Vec::new())
        .expect("write queue pressure event");

    // Assert
    assert_eq!(event.label, "write_queue_pressure");
    assert_eq!(event.next_action, "write_queue_pressure");
}

#[test]
fn aggregate_queue_pressure_records_shared_resource_evidence() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let mut queue = super::RuntimeQueuePressureState::default();
    queue.record_aggregate_queued_messages(policy.max_aggregate_queued_messages + 1);
    let event = super::queue_pressure_event(&policy, &queue, Vec::new(), Vec::new())
        .expect("aggregate queue pressure event");
    let mut evidence = listener_evidence(&["127.0.0.1:18444"]);
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .set_inbound_listener_evidence(listener_evidence(&["127.0.0.1:18444"]))
        .expect("authoritative listener evidence");

    // Act
    evidence.record_resource_event(event.clone());
    context.record_inbound_resource_event(event);

    // Assert
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .as_ref()
            .expect("listener resource event")
            .label,
        "resource_pressure_active"
    );
    assert_eq!(
        context
            .maybe_inbound_listener_evidence()
            .expect("managed evidence")
            .maybe_latest_resource_event
            .as_ref()
            .expect("managed resource event")
            .label,
        "resource_pressure_active"
    );
}

#[tokio::test]
async fn read_wire_message_returns_timeout_disconnect_without_wall_clock_wait() {
    // Arrange
    let (_client, server) = tcp_pair().await;
    let envelope_policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let resource_policy = ResourceGovernancePolicy::default();

    // Act
    let outcome = super::read_wire_message_with_timeout_duration(
        &server,
        &envelope_policy,
        &resource_policy,
        100,
        100,
        InboundHandshakeState::Handshaking,
        Duration::ZERO,
    )
    .await
    .expect("read timeout should return resource event");

    // Assert
    let ReadWireMessageOutcome::Rejected(event) = outcome else {
        panic!("expected timeout_disconnect resource event");
    };
    assert_eq!(event.label, "slow_handshake");
    assert_eq!(event.next_action, "timeout_disconnect");
}

#[tokio::test(start_paused = true)]
async fn read_wire_message_times_out_across_partial_header_bytes() {
    // Arrange
    let (client, server) = tcp_pair().await;
    let envelope_policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let resource_policy = ResourceGovernancePolicy::default();
    let header = verack_header(NetworkMagic::MAINNET);
    let timeout_duration = Duration::from_secs(5);
    let read_task = tokio::spawn(async move {
        super::read_wire_message_with_timeout_duration(
            &server,
            &envelope_policy,
            &resource_policy,
            100,
            100,
            InboundHandshakeState::Handshaking,
            timeout_duration,
        )
        .await
        .expect("partial read timeout should return resource event")
    });
    tokio::task::yield_now().await;

    // Act
    client.writable().await.expect("client socket writable");
    assert_eq!(
        client
            .try_write(&header[..1])
            .expect("write first header byte"),
        1
    );
    tokio::time::advance(Duration::from_secs(4)).await;
    tokio::task::yield_now().await;
    client
        .writable()
        .await
        .expect("client socket writable again");
    assert_eq!(
        client
            .try_write(&header[1..2])
            .expect("write second header byte"),
        1
    );
    tokio::time::advance(Duration::from_secs(2)).await;
    for _ in 0..10 {
        if read_task.is_finished() {
            break;
        }
        tokio::task::yield_now().await;
    }
    let outcome = read_task.await.expect("read task should join");

    // Assert
    let ReadWireMessageOutcome::Rejected(event) = outcome else {
        panic!("expected timeout_disconnect resource event");
    };
    assert_eq!(event.label, "slow_handshake");
    assert_eq!(event.next_action, "timeout_disconnect");
}

#[test]
fn context_records_inbound_resource_event_for_managed_evidence() {
    // Arrange
    let mut context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    context
        .set_inbound_listener_evidence(listener_evidence(&["127.0.0.1:18444"]))
        .expect("authoritative listener evidence");
    let policy = ResourceGovernancePolicy::default();
    let reconnect = match policy.decide_reconnect(ReconnectSuppressionInput {
        banned: false,
        discouraged: true,
    }) {
        ResourceGovernanceDecision::Backpressure(event) => event,
        other => panic!("expected reconnect_suppressed event, got {other:?}"),
    };

    // Act
    context.record_inbound_resource_event(reconnect);

    // Assert
    let evidence = context
        .maybe_inbound_listener_evidence()
        .expect("managed evidence should be present");
    assert_eq!(evidence.reconnect_suppressions, 1);
    assert_eq!(
        evidence
            .maybe_latest_resource_event
            .as_ref()
            .expect("latest resource event")
            .next_action,
        "reconnect_suppressed"
    );
}
