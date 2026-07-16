// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use std::{
    cell::RefCell,
    collections::VecDeque,
    io::{self, Read},
    rc::Rc,
};

use open_bitcoin_codec::{
    BIP152_COMPACT_BLOCKS_VERSION, CompactBlockPayload, PrefilledTransaction, SendCompactMessage,
};
use open_bitcoin_core::consensus::{block_hash, block_merkle_root, transaction_wtxid};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingActivationConfig,
    COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS, CompactRelayActivationConfig,
};

use super::*;
use crate::sync::tcp::{ReadStageOutcome, read_stage};

#[derive(Debug)]
enum ReadAction {
    Bytes(Vec<u8>),
    Error(io::ErrorKind),
    Eof,
}

#[derive(Debug)]
struct ScriptedReader {
    actions: VecDeque<ReadAction>,
}

impl ScriptedReader {
    fn new(actions: Vec<ReadAction>) -> Self {
        Self {
            actions: actions.into(),
        }
    }
}

impl Read for ScriptedReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match self.actions.pop_front().unwrap_or(ReadAction::Eof) {
            ReadAction::Bytes(bytes) => {
                let copied = bytes.len().min(buffer.len());
                buffer[..copied].copy_from_slice(&bytes[..copied]);
                Ok(copied)
            }
            ReadAction::Error(kind) => Err(io::Error::from(kind)),
            ReadAction::Eof => Ok(0),
        }
    }
}

#[derive(Debug)]
struct TimingTransport {
    maybe_outcomes: Option<VecDeque<SyncPeerReceiveOutcome>>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
}

impl TimingTransport {
    fn new(outcomes: Vec<SyncPeerReceiveOutcome>) -> Self {
        Self {
            maybe_outcomes: Some(outcomes.into()),
            sent: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn sent_messages(&self) -> Vec<WireNetworkMessage> {
        self.sent.borrow().clone()
    }
}

#[derive(Debug)]
struct TimingSession {
    outcomes: VecDeque<SyncPeerReceiveOutcome>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
}

#[derive(Debug)]
struct PerpetualIdleSession {
    receive_calls: Rc<RefCell<usize>>,
}

impl SyncTransport for TimingTransport {
    type Session = TimingSession;

    fn connect(
        &mut self,
        _peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        Ok(TimingSession {
            outcomes: self.maybe_outcomes.take().unwrap_or_default(),
            sent: Rc::clone(&self.sent),
        })
    }
}

impl SyncPeerSession for TimingSession {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.sent.borrow_mut().push(message.clone());
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        Ok(self
            .outcomes
            .pop_front()
            .unwrap_or(SyncPeerReceiveOutcome::Closed))
    }
}

impl SyncPeerSession for PerpetualIdleSession {
    fn send(
        &mut self,
        _message: &WireNetworkMessage,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        let call = *self.receive_calls.borrow();
        *self.receive_calls.borrow_mut() = call.saturating_add(1);
        Ok(match call {
            0 => SyncPeerReceiveOutcome::Message(version_message()),
            1 => SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
            _ => SyncPeerReceiveOutcome::Idle,
        })
    }
}

#[test]
fn phase123_tcp_zero_progress_timeout_is_idle() {
    // Arrange
    let mut reader = ScriptedReader::new(vec![ReadAction::Error(io::ErrorKind::TimedOut)]);
    let mut buffer = [0_u8; 2];

    // Act
    let outcome = read_stage(&mut reader, &mut buffer, true).expect("idle outcome");

    // Assert
    assert_eq!(outcome, ReadStageOutcome::Idle);
}

#[test]
fn phase123_tcp_clean_eof_is_closed() {
    // Arrange
    let mut reader = ScriptedReader::new(vec![ReadAction::Eof]);
    let mut buffer = [0_u8; 2];

    // Act
    let outcome = read_stage(&mut reader, &mut buffer, true).expect("closed outcome");

    // Assert
    assert_eq!(outcome, ReadStageOutcome::Closed);
}

#[test]
fn phase123_partial_frame_timeout_is_not_clean_idle() {
    // Arrange
    let mut reader = ScriptedReader::new(vec![
        ReadAction::Bytes(vec![0x01]),
        ReadAction::Error(io::ErrorKind::TimedOut),
    ]);
    let mut buffer = [0_u8; 2];

    // Act
    let result = read_stage(&mut reader, &mut buffer, true);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
}

#[test]
fn phase123_partial_frame_eof_is_not_clean_closed() {
    // Arrange
    let mut reader = ScriptedReader::new(vec![ReadAction::Bytes(vec![0x01]), ReadAction::Eof]);
    let mut buffer = [0_u8; 2];

    // Act
    let result = read_stage(&mut reader, &mut buffer, true);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
}

#[test]
fn phase123_idle_before_timeout_retains_session_without_fallback_or_progress() {
    // Arrange
    let path = temp_store_path("phase123-idle-before-timeout");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let mut transport = TimingTransport::new(vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Closed,
    ]);
    let mut resolver = timing_resolver();
    let clock_calls = Rc::new(RefCell::new(0_usize));
    let clock_call_count = Rc::clone(&clock_calls);
    let mut clock = move || {
        *clock_call_count.borrow_mut() += 1;
        1_000 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS - 1
    };

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 1_000, &mut clock)
        .expect("idle sync summary");

    // Assert
    assert_eq!(*clock_calls.borrow(), 1);
    assert_eq!(summary.messages_processed, 2);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Connected);
    assert!(!transport.sent_messages().iter().any(is_full_block_getdata));
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_idle_after_fake_clock_emits_same_peer_full_block_fallback() {
    // Arrange
    let path = temp_store_path("phase123-idle-after-timeout");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let compact_block = compact_block_fixture(&mut runtime);
    let expected_hash = block_hash(&compact_block.header);
    let mut transport = TimingTransport::new(compact_download_script(&compact_block));
    let mut resolver = timing_resolver();
    let mut clock = || 2_000 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1;

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 2_000, &mut clock)
        .expect("timed-out compact sync summary");

    // Assert
    assert_eq!(summary.messages_processed, 4);
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| { is_full_block_getdata_for_hash(message, expected_hash) })
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_message_after_idle_uses_session_clock_for_compact_timeout() {
    // Arrange
    let path = temp_store_path("phase123-message-after-idle-clock");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let compact_block = compact_block_fixture(&mut runtime);
    let expected_hash = block_hash(&compact_block.header);
    let mut transport = TimingTransport::new(vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Message(send_compact_message()),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(compact_payload(
            &compact_block,
        ))),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Closed,
    ]);
    let sent = Rc::clone(&transport.sent);
    let mut resolver = timing_resolver();
    let compact_received_at = 7_000;
    let mut clock_values = [
        compact_received_at,
        compact_received_at + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS - 1,
        compact_received_at + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1,
    ]
    .into_iter();
    let mut clock = || {
        let now = clock_values.next().expect("scripted clock value");
        if now > compact_received_at + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS {
            assert!(!sent.borrow().iter().any(is_full_block_getdata));
        }
        now
    };

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 6_000, &mut clock)
        .expect("late compact sync summary");

    // Assert
    assert_eq!(summary.messages_processed, 4);
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| is_full_block_getdata_for_hash(message, expected_hash))
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_idle_wake_does_not_consume_message_budget() {
    // Arrange
    let path = temp_store_path("phase123-idle-message-budget");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 2);
    let mut transport = TimingTransport::new(vec![
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Closed,
    ]);
    let mut resolver = timing_resolver();
    let mut clock = || 3_000;

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 3_000, &mut clock)
        .expect("idle budget summary");

    // Assert
    assert_eq!(summary.messages_processed, 2);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Connected);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_perpetual_idle_session_returns_after_bounded_wakes() {
    // Arrange
    let path = temp_store_path("phase123-bounded-idle-session");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let receive_calls = Rc::new(RefCell::new(0_usize));
    let session = PerpetualIdleSession {
        receive_calls: Rc::clone(&receive_calls),
    };
    let peer = resolved_manual_peer("127.0.0.1", 18_444);
    let mut clock = || 3_500;

    // Act
    let progress = runtime
        .sync_connected_peer(session, &peer, 1, 1, 3_500, &mut clock)
        .expect("bounded idle session");

    // Assert
    assert_eq!(*receive_calls.borrow(), 4);
    assert_eq!(progress.state, PeerSyncState::Connected);
    assert_eq!(progress.messages_processed, 2);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_closed_receive_ends_session() {
    // Arrange
    let path = temp_store_path("phase123-closed-session");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let mut transport = TimingTransport::new(vec![SyncPeerReceiveOutcome::Closed]);
    let mut resolver = timing_resolver();
    let mut clock = || 4_000;

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 4_000, &mut clock)
        .expect("closed summary");

    // Assert
    assert_eq!(summary.messages_processed, 0);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Stalled);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_target_mismatch_is_not_written_to_current_session() {
    // Arrange
    let path = temp_store_path("phase123-target-mismatch");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let compact_block = compact_block_fixture(&mut runtime);
    let expected_hash = block_hash(&compact_block.header);
    start_other_peer_compact_download(&mut runtime, 99, &compact_block, 5_000);
    let sent = Rc::new(RefCell::new(Vec::new()));
    let session = TimingSession {
        outcomes: vec![SyncPeerReceiveOutcome::Idle].into(),
        sent: Rc::clone(&sent),
    };
    let peer = resolved_manual_peer("127.0.0.1", 18_444);
    let mut clock = || 5_000 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1;

    // Act
    let result = runtime.sync_connected_peer(session, &peer, 1, 1, 5_000, &mut clock);

    // Assert
    let failure = result.expect_err("target mismatch must fail");
    assert!(matches!(
        failure.error,
        SyncRuntimeError::Network { ref message }
            if message == "compact timeout action target does not match connected session"
    ));
    assert!(
        !sent
            .borrow()
            .iter()
            .any(|message| { is_full_block_getdata_for_hash(message, expected_hash) })
    );
    remove_dir_if_exists(&path);
}

fn timing_runtime(path: &std::path::Path, max_messages_per_peer: usize) -> DurableSyncRuntime {
    let store = FjallNodeStore::open(path).expect("store");
    DurableSyncRuntime::open_with_block_relay_activation(
        store,
        SyncRuntimeConfig {
            max_messages_per_peer,
            max_peer_retries: 0,
            ..sync_config()
        },
        enabled_block_relay_activation(),
    )
    .expect("runtime")
}

fn timing_resolver() -> ScriptedResolver {
    ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer("127.0.0.1", 18_444)])])
}

fn version_message() -> WireNetworkMessage {
    WireNetworkMessage::Version(VersionMessage::default())
}

fn enabled_block_relay_activation() -> BlockRelayActivationPolicy {
    BlockRelayActivationPolicy {
        block_serving: BlockServingActivationConfig { enabled: true },
        compact_relay: CompactRelayActivationConfig { enabled: true },
    }
}

fn compact_block_fixture(runtime: &mut DurableSyncRuntime) -> Block {
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let spendable = build_block(block_hash(&genesis.header), 1);
    runtime
        .network
        .connect_local_block(&genesis, runtime.verify_flags, runtime.consensus_params)
        .expect("connect genesis");
    runtime
        .network
        .connect_local_block(&spendable, runtime.verify_flags, runtime.consensus_params)
        .expect("connect spendable");

    let mut announced = build_block(block_hash(&spendable.header), 2);
    announced.transactions.push(coinbase_transaction(3, 25));
    let (merkle_root, maybe_mutated) =
        block_merkle_root(&announced.transactions).expect("merkle root");
    assert!(!maybe_mutated);
    announced.header.merkle_root = merkle_root;
    mine_header(&mut announced);
    announced
}

fn compact_download_script(block: &Block) -> Vec<SyncPeerReceiveOutcome> {
    vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Message(send_compact_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(compact_payload(block))),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Closed,
    ]
}

fn send_compact_message() -> WireNetworkMessage {
    WireNetworkMessage::SendCompact(SendCompactMessage {
        announce: true,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    })
}

fn compact_payload(block: &Block) -> CompactBlockPayload {
    let nonce = 17;
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&block.header, nonce);
    let wtxid = transaction_wtxid(&block.transactions[1]).expect("wtxid");
    CompactBlockPayload {
        header: block.header.clone(),
        nonce,
        short_ids: vec![open_bitcoin_core::consensus::compact_short_id_for_wtxid(
            selector, &wtxid,
        )],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: block.transactions[0].clone(),
        }],
    }
}

fn start_other_peer_compact_download(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
    block: &Block,
    timestamp: i64,
) {
    runtime
        .network
        .connect_outbound_peer(peer_id, timestamp)
        .expect("connect other peer");
    for message in [
        version_message(),
        WireNetworkMessage::Verack,
        send_compact_message(),
    ] {
        runtime
            .network
            .receive_sync_message(
                peer_id,
                message,
                timestamp,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("other peer handshake");
    }
    let outbound = runtime
        .network
        .receive_sync_message(
            peer_id,
            WireNetworkMessage::CompactBlock(compact_payload(block)),
            timestamp,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("other peer compact download")
        .outbound;
    assert!(
        outbound
            .iter()
            .any(|message| { matches!(message, WireNetworkMessage::GetBlockTxn(_)) })
    );
}

fn is_full_block_getdata(message: &WireNetworkMessage) -> bool {
    matches!(
        message,
        WireNetworkMessage::GetData(inventory)
            if inventory.inventory.iter().any(|item| item.inventory_type == InventoryType::Block)
    )
}

fn is_full_block_getdata_for_hash(message: &WireNetworkMessage, expected_hash: BlockHash) -> bool {
    matches!(
        message,
        WireNetworkMessage::GetData(inventory)
            if inventory.inventory.iter().any(|item| {
                item.inventory_type == InventoryType::Block
                    && BlockHash::from(item.object_hash) == expected_hash
            })
    )
}
