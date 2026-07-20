// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_codec::{BIP152_COMPACT_BLOCKS_VERSION, SendCompactMessage};
use open_bitcoin_core::{
    consensus::block_hash,
    primitives::{Block, BlockHash, NetworkMagic},
};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingActivationConfig, CompactRelayActivationConfig,
    HeadersMessage, WireNetworkMessage,
};

use super::*;
use crate::{block_relay_metric_samples, logging::block_relay_log_record, network::PeerEmission};

const TRANSPORT_TIMESTAMP: i64 = 1_784_523_200;

#[derive(Debug, Default)]
struct RecordingAnnouncementSession {
    sent: Vec<WireNetworkMessage>,
    maybe_fail_on_call: Option<usize>,
    send_calls: usize,
}

impl RecordingAnnouncementSession {
    fn failing_on_call(call: usize) -> Self {
        Self {
            maybe_fail_on_call: Some(call),
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
            return Err(SyncRuntimeError::Io {
                peer: "redacted-test-peer".to_string(),
                message: "scripted announcement write failure".to_string(),
            });
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
    let (_, _, receipt) = PeerEmission::new(peer_id, message, previous_block_hash)
        .expect("header provenance emission")
        .into_parts();
    runtime
        .network
        .complete_peer_emission(receipt)
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
    remove_dir_if_exists(&path);
}
