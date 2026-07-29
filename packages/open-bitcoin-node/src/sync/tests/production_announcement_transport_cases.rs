// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/validationinterface.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{BIP152_COMPACT_BLOCKS_VERSION, SendCompactMessage};
use open_bitcoin_core::{
    consensus::block_hash,
    primitives::{Block, BlockHash, NetworkMagic},
};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingActivationConfig, CompactRelayActivationConfig,
    HeadersMessage, PHASE94_MAX_PEER_QUEUED_MESSAGES, WireNetworkMessage,
};

use super::*;
use crate::{
    block_relay_metric_samples,
    logging::block_relay_log_record,
    network::{PeerEmission, PeerOutboxSnapshot},
};

const TRANSPORT_TIMESTAMP: i64 = 1_784_523_200;

#[derive(Debug, Default)]
struct RecordingAnnouncementSession {
    sent: Vec<WireNetworkMessage>,
    maybe_fail_on_call: Option<usize>,
    maybe_failure: Option<SyncRuntimeError>,
    send_calls: usize,
}

impl RecordingAnnouncementSession {
    fn failing_on_call(call: usize) -> Self {
        Self::failing_on_call_with(
            call,
            SyncRuntimeError::Io {
                peer: "redacted-test-peer".to_string(),
                message: "scripted announcement write failure".to_string(),
            },
        )
    }

    fn failing_on_call_with(call: usize, failure: SyncRuntimeError) -> Self {
        Self {
            maybe_fail_on_call: Some(call),
            maybe_failure: Some(failure),
            ..Self::default()
        }
    }
}

impl SyncPeerSession for RecordingAnnouncementSession {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        _magic: NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.send_calls = self.send_calls.saturating_add(1);
        if self.maybe_fail_on_call == Some(self.send_calls) {
            return Err(self
                .maybe_failure
                .clone()
                .expect("scripted failure must accompany a failing call"));
        }
        self.sent.push(message.clone());
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        Ok(SyncPeerReceiveOutcome::Closed)
    }
}

fn open_production_announcement_runtime(test_name: &str) -> (DurableSyncRuntime, PathBuf) {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open_with_block_relay_activation(
        store,
        sync_config(),
        BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
    )
    .expect("production announcement runtime");
    (runtime, path)
}

fn complete_remote_handshake(runtime: &mut DurableSyncRuntime, peer_id: PeerId) {
    connect_runtime_peer(runtime, peer_id, 1);
}

fn receive_peer_preference(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
    message: WireNetworkMessage,
) {
    runtime
        .network
        .receive_sync_message(
            peer_id,
            message,
            TRANSPORT_TIMESTAMP,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("record peer announcement preference");
}

fn seed_live_previous_header_fact(
    runtime: &DurableSyncRuntime,
    peer_id: PeerId,
    previous_block_hash: BlockHash,
) {
    let message = WireNetworkMessage::Headers(HeadersMessage {
        headers: vec![build_block(BlockHash::from_byte_array([0x91; 32]), 91).header],
    });
    let effect_capability = runtime
        .network
        .prepare_peer_relay_effect(peer_id)
        .expect("prepare header provenance capability");
    let (_, _, capability) =
        PeerEmission::new(peer_id, message, previous_block_hash, effect_capability)
            .expect("header provenance emission")
            .into_parts();
    runtime
        .network
        .complete_peer_emission(capability.acknowledge_write())
        .expect("seed live peer header fact");
}

fn make_durable_and_prepare(runtime: &mut DurableSyncRuntime, block: &Block) {
    runtime
        .network
        .connect_local_block(block, runtime.verify_flags, runtime.consensus_params)
        .expect("connect local block");
    runtime
        .store()
        .save_block(block, PersistMode::Sync)
        .expect("persist durable block");
    runtime.queue_durable_tip_advanced(block.clone());
    runtime
        .dispatch_pending_durable_tip()
        .expect("prepare durable tip announcements");
}

fn announcement_evidence(runtime: &DurableSyncRuntime) -> serde_json::Value {
    serde_json::to_value(
        runtime
            .network
            .block_relay_evidence_status()
            .expect("block relay evidence"),
    )
    .expect("serialize announcement evidence")
}

fn achieved_announcement_count(evidence: &serde_json::Value) -> u64 {
    [
        "compact_announced_count",
        "compact_headers_fallback_count",
        "compact_inventory_fallback_count",
    ]
    .into_iter()
    .map(|field| {
        evidence["announcement"]["value"][field]
            .as_u64()
            .unwrap_or(0)
    })
    .sum()
}

fn assert_peer_pending_capacity_recovered(runtime: &DurableSyncRuntime, peer_id: PeerId) {
    for _ in 0..PHASE94_MAX_PEER_QUEUED_MESSAGES {
        runtime
            .network
            .prepare_peer_relay_effect(peer_id)
            .expect("all pending peer slots should be reusable after fanout failure");
    }
}

fn assert_node_write_failure_terminal_contract(failure_call: usize, test_name: &str) {
    let (mut runtime, path) = open_production_announcement_runtime(test_name);
    let peer_id = 128_320_u64.saturating_add(failure_call as u64);
    complete_remote_handshake(&mut runtime, peer_id);
    runtime
        .announcement_outboxes
        .register_peer(peer_id)
        .expect("register peer outbox");
    let first = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    make_durable_and_prepare(&mut runtime, &first);
    let second = build_block(block_hash(&first.header), 1);
    make_durable_and_prepare(&mut runtime, &second);
    let third = build_block(block_hash(&second.header), 2);
    make_durable_and_prepare(&mut runtime, &third);
    let achieved_before = achieved_announcement_count(&announcement_evidence(&runtime));
    let mut session = RecordingAnnouncementSession::failing_on_call(failure_call);

    let result = runtime.send_all_for_peer(&mut session, peer_id, &[]);

    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert_eq!(session.sent.len(), failure_call.saturating_sub(1));
    assert_eq!(
        achieved_announcement_count(&announcement_evidence(&runtime)) - achieved_before,
        failure_call.saturating_sub(1) as u64
    );
    assert_peer_pending_capacity_recovered(&runtime, peer_id);
    remove_dir_if_exists(&path);
}

#[test]
fn production_announcement_transport_cases_fanout_uses_live_peer_facts() {
    // Arrange
    let (mut runtime, path) = open_production_announcement_runtime("production-fanout");
    let high_bandwidth_peer = 128_301;
    let headers_peer = 128_302;
    let inventory_peer = 128_303;
    for peer_id in [high_bandwidth_peer, headers_peer, inventory_peer] {
        complete_remote_handshake(&mut runtime, peer_id);
        runtime
            .announcement_outboxes
            .register_peer(peer_id)
            .expect("register peer outbox");
    }
    receive_peer_preference(
        &mut runtime,
        high_bandwidth_peer,
        WireNetworkMessage::SendCompact(SendCompactMessage {
            announce: true,
            version: BIP152_COMPACT_BLOCKS_VERSION,
        }),
    );
    receive_peer_preference(&mut runtime, headers_peer, WireNetworkMessage::SendHeaders);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    seed_live_previous_header_fact(
        &runtime,
        high_bandwidth_peer,
        block.header.previous_block_hash,
    );
    make_durable_and_prepare(&mut runtime, &block);
    let mut high_session = RecordingAnnouncementSession::default();
    let mut headers_session = RecordingAnnouncementSession::default();
    let mut inventory_session = RecordingAnnouncementSession::default();

    // Act
    runtime
        .send_all_for_peer(&mut high_session, high_bandwidth_peer, &[])
        .expect("write compact announcement");
    runtime
        .send_all_for_peer(&mut headers_session, headers_peer, &[])
        .expect("write headers announcement");
    runtime
        .send_all_for_peer(&mut inventory_session, inventory_peer, &[])
        .expect("write inventory announcement");

    // Assert
    assert!(matches!(
        high_session.sent.as_slice(),
        [WireNetworkMessage::CompactBlock(_)]
    ));
    assert!(matches!(
        headers_session.sent.as_slice(),
        [WireNetworkMessage::Headers(_)]
    ));
    assert!(matches!(
        inventory_session.sent.as_slice(),
        [WireNetworkMessage::Inv(_)]
    ));
    let evidence = announcement_evidence(&runtime);
    assert_eq!(
        evidence["announcement"]["value"]["compact_announced_count"],
        1
    );
    assert_eq!(
        evidence["announcement"]["value"]["compact_headers_fallback_count"],
        2
    );
    assert_eq!(
        evidence["announcement"]["value"]["compact_inventory_fallback_count"],
        1
    );
    remove_dir_if_exists(&path);
}

#[test]
fn production_announcement_transport_cases_partial_failure_credits_only_prefix_and_redacts() {
    // Arrange
    let (mut runtime, path) = open_production_announcement_runtime("production-partial-failure");
    let peer_id = 128_311;
    complete_remote_handshake(&mut runtime, peer_id);
    runtime
        .announcement_outboxes
        .register_peer(peer_id)
        .expect("register peer outbox");
    receive_peer_preference(
        &mut runtime,
        peer_id,
        WireNetworkMessage::SendCompact(SendCompactMessage {
            announce: true,
            version: BIP152_COMPACT_BLOCKS_VERSION,
        }),
    );
    let first = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    seed_live_previous_header_fact(&runtime, peer_id, first.header.previous_block_hash);
    make_durable_and_prepare(&mut runtime, &first);
    let second = build_block(block_hash(&first.header), 1);
    make_durable_and_prepare(&mut runtime, &second);
    let third = build_block(block_hash(&second.header), 2);
    make_durable_and_prepare(&mut runtime, &third);
    let mut session = RecordingAnnouncementSession::failing_on_call(2);

    // Act
    let result = runtime.send_all_for_peer(&mut session, peer_id, &[]);
    let status = runtime
        .network
        .block_relay_evidence_status()
        .expect("authoritative block relay status");
    let metrics = block_relay_metric_samples(&status, 0, TRANSPORT_TIMESTAMP as u64);
    let log = block_relay_log_record(&status, 0, TRANSPORT_TIMESTAMP as u64);
    let projected = format!("{metrics:?} {log:?}");

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert!(matches!(
        session.sent.as_slice(),
        [WireNetworkMessage::CompactBlock(_)]
    ));
    assert_eq!(session.send_calls, 2);
    let evidence = announcement_evidence(&runtime);
    assert_eq!(
        evidence["announcement"]["value"]["compact_announced_count"],
        1
    );
    assert_eq!(
        evidence["announcement"]["value"]["compact_headers_fallback_count"],
        1
    );
    assert!(
        runtime
            .announcement_outboxes
            .take_peer_emissions(peer_id)
            .expect("remaining peer emissions")
            .is_empty()
    );
    for sensitive in [
        peer_id.to_string(),
        format!("{:02x?}", block_hash(&first.header).to_byte_array()),
        "download_permission".to_string(),
        "rpc-password=secret".to_string(),
        "transaction-payload-sentinel".to_string(),
    ] {
        assert!(!projected.contains(&sensitive));
    }
    assert_peer_pending_capacity_recovered(&runtime, peer_id);
    remove_dir_if_exists(&path);
}

#[test]
fn production_announcement_transport_cases_first_and_last_failures_restore_capacity() {
    // Arrange / Act / Assert
    assert_node_write_failure_terminal_contract(1, "production-first-failure");
    assert_node_write_failure_terminal_contract(3, "production-last-failure");
}

#[test]
fn production_announcement_transport_cases_enqueue_race_aborts_unqueued_emission() {
    // Arrange
    let (mut runtime, path) = open_production_announcement_runtime("production-enqueue-race");
    let peer_id = 128_331;
    complete_remote_handshake(&mut runtime, peer_id);
    let snapshots = [PeerOutboxSnapshot::new(
        peer_id,
        0,
        PHASE94_MAX_PEER_QUEUED_MESSAGES,
    )];
    let outcomes = runtime
        .network
        .prepare_block_announcements(&Block::default(), &snapshots)
        .expect("prepare raced announcement");

    // Act
    runtime
        .announcement_outboxes
        .enqueue_prepared(&runtime.network, outcomes)
        .expect("abort emission whose outbox disappeared");

    // Assert
    assert_peer_pending_capacity_recovered(&runtime, peer_id);
    remove_dir_if_exists(&path);
}

#[test]
fn production_announcement_transport_cases_unregister_aborts_queued_emission() {
    // Arrange
    let (mut runtime, path) = open_production_announcement_runtime("production-unregister");
    let peer_id = 128_332;
    complete_remote_handshake(&mut runtime, peer_id);
    runtime
        .announcement_outboxes
        .register_peer(peer_id)
        .expect("register peer outbox");
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    make_durable_and_prepare(&mut runtime, &block);

    // Act
    runtime
        .announcement_outboxes
        .unregister_peer(&runtime.network, peer_id)
        .expect("abort queued peer emission");

    // Assert
    assert_peer_pending_capacity_recovered(&runtime, peer_id);
    remove_dir_if_exists(&path);
}

#[test]
fn production_announcement_transport_cases_encode_failure_preserves_only_achieved_prefix() {
    // Arrange
    let (mut runtime, path) = open_production_announcement_runtime("production-encode-failure");
    let peer_id = 128_312;
    complete_remote_handshake(&mut runtime, peer_id);
    runtime
        .announcement_outboxes
        .register_peer(peer_id)
        .expect("register peer outbox");
    receive_peer_preference(
        &mut runtime,
        peer_id,
        WireNetworkMessage::SendCompact(SendCompactMessage {
            announce: true,
            version: BIP152_COMPACT_BLOCKS_VERSION,
        }),
    );
    let first = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    seed_live_previous_header_fact(&runtime, peer_id, first.header.previous_block_hash);
    make_durable_and_prepare(&mut runtime, &first);
    let second = build_block(block_hash(&first.header), 1);
    make_durable_and_prepare(&mut runtime, &second);
    let third = build_block(block_hash(&second.header), 2);
    make_durable_and_prepare(&mut runtime, &third);
    let expected = SyncRuntimeError::Network {
        message: "scripted announcement encode failure".to_string(),
    };
    let mut session = RecordingAnnouncementSession::failing_on_call_with(2, expected.clone());

    // Act
    let result = runtime.send_all_for_peer(&mut session, peer_id, &[]);

    // Assert
    assert_eq!(result, Err(expected));
    assert_eq!(session.send_calls, 2);
    assert!(matches!(
        session.sent.as_slice(),
        [WireNetworkMessage::CompactBlock(_)]
    ));
    let evidence = announcement_evidence(&runtime);
    assert_eq!(
        evidence["announcement"]["value"]["compact_announced_count"],
        1
    );
    assert_eq!(
        evidence["announcement"]["value"]["compact_headers_fallback_count"],
        1
    );
    assert_peer_pending_capacity_recovered(&runtime, peer_id);
    remove_dir_if_exists(&path);
}
