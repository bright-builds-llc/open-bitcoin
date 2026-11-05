// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn duplicate_version_peer_is_failed_and_replaced_without_progress_credit() {
    // Arrange
    let path = temp_store_path("duplicate-version-replacement");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let duplicate_version_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
    ];
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.23", 18_444),
                SyncPeerAddress::manual("198.51.100.24", 18_445),
            ],
            dns_seeds: Vec::new(),
            max_peer_retries: 0,
            target_outbound_peers: 1,
            max_messages_per_peer: 8,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::new(vec![duplicate_version_script, version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.23", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.24", 18_445)]),
    ]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_191)
        .expect("summary");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let durable_sync_state = metadata.maybe_sync_state.expect("durable sync state");

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.headers_received, 0);
    assert_eq!(summary.blocks_received, 0);
    let rejected = &summary.peer_outcomes[0];
    assert_eq!(rejected.state, PeerSyncState::Failed);
    assert_eq!(
        rejected.maybe_failure_reason,
        Some(PeerFailureReason::Compatibility)
    );
    assert_eq!(rejected.contribution.headers_received, 0);
    assert_eq!(rejected.contribution.blocks_received, 0);
    assert!(
        rejected
            .maybe_error
            .as_ref()
            .is_some_and(|message| { message.contains("duplicate version") })
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert_eq!(
        durable_sync_state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Active)
    );
    assert_eq!(
        durable_sync_state.sync.resource_pressure,
        FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 16,
            max_blocks_in_flight_total: 64,
            max_messages_per_peer: 8,
            max_sync_rounds: 8,
            outbound_peers: 1,
            target_outbound_peers: 1,
        })
    );

    remove_dir_if_exists(&path);
}

#[test]
fn wrong_network_peer_is_failed_without_progress_credit() {
    // Arrange
    let path = temp_store_path("wrong-network-peer");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.25", 18_444)],
            dns_seeds: Vec::new(),
            max_peer_retries: 0,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::with_connect_results(vec![Err(SyncRuntimeError::InvalidMagic {
            expected: SyncNetwork::Regtest.magic().to_bytes(),
            actual: SyncNetwork::Mainnet.magic().to_bytes(),
        })]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.25",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_192)
        .expect("summary");

    // Assert
    assert_eq!(summary.connected_peers, 0);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.headers_received, 0);
    assert_eq!(summary.blocks_received, 0);
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Failed);
    assert_eq!(
        outcome.maybe_failure_reason,
        Some(PeerFailureReason::InvalidMagic)
    );
    assert_eq!(outcome.contribution.messages_processed, 0);
    assert_eq!(outcome.contribution.headers_received, 0);
    assert_eq!(outcome.contribution.blocks_received, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn sync_outcome_captures_peer_capabilities_and_endpoint() {
    // Arrange
    let path = temp_store_path("peer-capabilities");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.30", 18_444)],
            dns_seeds: Vec::new(),
            max_messages_per_peer: 4,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 3,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::WtxidRelay,
        WireNetworkMessage::SendHeaders,
        WireNetworkMessage::Verack,
    ]]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.30",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_199)
        .expect("summary");

    // Assert
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Connected);
    assert_eq!(
        outcome.maybe_resolved_endpoint.as_deref(),
        Some("127.0.0.1:18444")
    );
    let capabilities = outcome
        .maybe_capabilities
        .as_ref()
        .expect("peer capabilities");
    assert!(capabilities.services_bits > 0);
    assert_eq!(capabilities.start_height, 3);
    assert!(capabilities.wtxidrelay);
    assert!(capabilities.prefers_headers);
}

mod lifecycle;

use crate::sync::{
    block_reconcile::{self, PeerRequests},
    progress::PeerProgress,
};
use open_bitcoin_codec::{CompactBlockPayload, PrefilledTransaction, SendCompactMessage};
use open_bitcoin_core::consensus::{transaction_txid, transaction_wtxid};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingActivationConfig, CompactRelayActivationConfig,
};

fn lifetime_fixture(label: &str) -> (PathBuf, DurableSyncRuntime, Block) {
    let path = temp_store_path(label);
    let runtime = DurableSyncRuntime::open_with_block_relay_activation(
        FjallNodeStore::open(&path).expect("owned store"),
        sync_config(),
        BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
    )
    .expect("runtime");
    let genesis = build_block(BlockHash::default(), 0);
    crate::sync::tests::support_runtime::initialize_fixture_runtime_genesis(&runtime, &genesis);
    let mut previous = genesis;
    let mut maybe_spendable = None;
    for height in 1..=100 {
        let block = build_block(block_hash(&previous.header), height);
        runtime
            .network
            .connect_local_block(&block, runtime.verify_flags, runtime.consensus_params)
            .expect("real ordinary history");
        if height == 1 {
            maybe_spendable = Some(block.transactions[0].clone());
        }
        previous = block;
    }
    let spendable = maybe_spendable.expect("actual first-height output");
    let mut block = build_block(block_hash(&previous.header), 101);
    block.transactions.push(Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: transaction_txid(&spendable).expect("txid"),
                vout: 0,
            },
            script_sig: script(&[]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(25).expect("amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    });
    block.header.merkle_root = block_merkle_root(&block.transactions)
        .expect("actual merkle")
        .0;
    mine_header(&mut block);
    (path, runtime, block)
}

fn lifetime_compact(block: &Block, complete: bool) -> CompactBlockPayload {
    let nonce = 17;
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&block.header, nonce);
    CompactBlockPayload {
        header: block.header.clone(),
        nonce,
        short_ids: if complete {
            Vec::new()
        } else {
            vec![open_bitcoin_core::consensus::compact_short_id_for_wtxid(
                selector,
                &transaction_wtxid(&block.transactions[1]).expect("wtxid"),
            )]
        },
        prefilled_transactions: block
            .transactions
            .iter()
            .take(if complete { 2 } else { 1 })
            .map(|transaction| PrefilledTransaction {
                index_delta: 0,
                transaction: transaction.clone(),
            })
            .collect(),
    }
}

fn lifetime_handshake(runtime: &DurableSyncRuntime, peer: PeerId) {
    runtime
        .network
        .connect_outbound_peer(peer, 0)
        .expect("actual peer");
    for message in version_verack_script(101)
        .into_iter()
        .chain([WireNetworkMessage::SendCompact(SendCompactMessage {
            announce: true,
            version: 2,
        })])
    {
        runtime
            .network
            .receive_sync_message(
                peer,
                message,
                0,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("actual handshake");
    }
}

fn lifetime_register(runtime: &mut DurableSyncRuntime, peer: PeerId, hashes: &[BlockHash]) {
    let outbound = runtime
        .network
        .request_missing_blocks(peer, hashes)
        .expect("actual network membership");
    let expected = getdata_block_hashes(&outbound);
    assert!(expected.iter().all(|hash| {
        runtime
            .network
            .peer_requested_blocks(peer)
            .expect("retained")
            .contains(hash)
    }));
    block_reconcile::handoff_requested_messages(
        runtime,
        peer,
        outbound
            .into_iter()
            .map(|message| (peer, message))
            .collect(),
    )
    .expect("actual runtime grant");
    assert!(
        expected
            .iter()
            .all(|hash| runtime.inflight_blocks.contains(hash))
    );
}

fn lifetime_admit_headers(
    runtime: &DurableSyncRuntime,
    headers: &[open_bitcoin_core::primitives::BlockHeader],
) {
    let peer = 98;
    lifetime_handshake(runtime, peer);
    for header in headers {
        runtime
            .network
            .receive_sync_message(
                peer,
                WireNetworkMessage::Headers(open_bitcoin_network::HeadersMessage {
                    headers: vec![header.clone()],
                }),
                i64::from(header.time) + 1,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("normal peer header ingress preserves admitted proof");
    }
    runtime
        .network
        .disconnect_peer(peer)
        .expect("temporary header peer removed before measured baseline");
}

use std::cell::Cell;

fn owned_peer(runtime: &mut DurableSyncRuntime, peer: PeerId) -> PeerRequests {
    let (token, outbound) = runtime
        .network
        .connect_sync_peer(peer, 0)
        .expect("actual admission");
    outbound.expect("actual version");
    for message in version_verack_script(101) {
        runtime
            .network
            .receive_sync_message(
                peer,
                message,
                0,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("actual handshake");
    }
    let mut requests = PeerRequests::new(token);
    requests.renew(runtime, &[]).expect("actual observation");
    requests
}

struct CompactSession {
    outcomes: VecDeque<SyncPeerReceiveOutcome>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
    network: crate::DurableNetworkHandle,
    remove_on_getdata: bool,
    hash: BlockHash,
    manager_only: bool,
}

impl SyncPeerSession for CompactSession {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.sent.borrow_mut().push(message.clone());
        if self.remove_on_getdata && !getdata_block_hashes(std::slice::from_ref(message)).is_empty()
        {
            self.remove_on_getdata = false;
            self.network.disconnect_peer(1)?;
        }
        Ok(())
    }
    fn receive(
        &mut self,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        let outcome = self
            .outcomes
            .pop_front()
            .unwrap_or(SyncPeerReceiveOutcome::Closed);
        if self.manager_only
            && matches!(
                &outcome,
                SyncPeerReceiveOutcome::Message(
                    WireNetworkMessage::Block(_) | WireNetworkMessage::NotFound(_)
                )
            )
        {
            self.manager_only = false;
            let issued = self.network.request_missing_blocks(1, &[self.hash])?;
            assert!(getdata_block_hashes(&issued).contains(&self.hash));
            assert!(self.network.peer_requested_blocks(1)?.contains(&self.hash));
            // Deliberately omit runtime handoff: manager membership is not a grant.
        }
        if getdata_block_hashes(&self.sent.borrow()).contains(&self.hash) {
            if matches!(
                &outcome,
                SyncPeerReceiveOutcome::Message(WireNetworkMessage::Ping { .. })
            ) {
                assert!(self.network.peer_requested_blocks(1)?.contains(&self.hash));
            }
            if matches!(
                &outcome,
                SyncPeerReceiveOutcome::Message(WireNetworkMessage::BlockTxn(_))
            ) || matches!(&outcome, SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(block)) if block.short_ids.is_empty())
            {
                assert!(self.network.peer_requested_blocks(1)?.contains(&self.hash));
            }
        }
        Ok(outcome)
    }
}

struct MeterSession<T> {
    inner: T,
    reads: Rc<Cell<usize>>,
}

impl<T: SyncPeerSession> SyncPeerSession for MeterSession<T> {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.inner.send(message, magic)
    }
    fn receive(
        &mut self,
        magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        self.reads.set(self.reads.get() + 1);
        self.inner.receive(magic)
    }
}

struct ReplacedOutboxSession {
    inbound: VecDeque<WireNetworkMessage>,
    network: crate::DurableNetworkHandle,
    store: FjallNodeStore,
    outboxes: crate::sync::AnnouncementOutboxRegistry,
    block: Block,
    flags: open_bitcoin_core::consensus::ScriptVerifyFlags,
    params: open_bitcoin_core::consensus::ConsensusParams,
    expected: Rc<RefCell<Vec<WireNetworkMessage>>>,
    registration: Rc<RefCell<Option<crate::sync::AnnouncementOutboxNotification>>>,
    replaced: bool,
}

impl SyncPeerSession for ReplacedOutboxSession {
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
        if let Some(message) = self.inbound.pop_front() {
            return Ok(SyncPeerReceiveOutcome::Message(message));
        }
        if !self.replaced {
            self.outboxes.unregister_peer(&self.network, 1)?;
            let registration = self.outboxes.register_peer(1)?;
            self.network
                .connect_local_block(&self.block, self.flags, self.params)?;
            self.store.save_block(&self.block, PersistMode::Sync)?;
            let snapshots = self.outboxes.snapshots()?;
            let outcomes = self
                .network
                .prepare_block_announcements(&self.block, &snapshots)?;
            *self.expected.borrow_mut() = outcomes
                .iter()
                .filter_map(|outcome| {
                    let crate::network::AnnouncementPreparationOutcome::Ready(emission) = outcome
                    else {
                        return None;
                    };
                    Some(emission.message().clone())
                })
                .collect();
            assert!(!self.expected.borrow().is_empty());
            self.outboxes.enqueue_prepared(&self.network, outcomes)?;
            *self.registration.borrow_mut() = Some(registration);
            self.replaced = true;
        }
        Ok(SyncPeerReceiveOutcome::Closed)
    }
}

fn metered_socket(
    runtime: &DurableSyncRuntime,
    route: usize,
) -> (
    MeterSession<crate::sync::tcp::TcpPeerSession>,
    std::net::TcpStream,
    ResolvedSyncPeerAddress,
    Rc<Cell<usize>>,
) {
    use std::io::Write;
    let (inner, mut server, peer) = crate::sync::tests::support_transport::loopback();
    let reads = Rc::new(Cell::new(0));
    let session = MeterSession {
        inner,
        reads: Rc::clone(&reads),
    };
    let message = WireNetworkMessage::Version(VersionMessage::default());
    let mut frame = message
        .encode_wire(runtime.config.network.magic())
        .expect("actual frame");
    if route == 3 {
        frame[20] ^= 1;
    }
    server.write_all(&frame).expect("real socket bytes");
    if route != 3 {
        let next = if route == 4 {
            WireNetworkMessage::Verack
                .encode_wire(runtime.config.network.magic())
                .expect("actual verack")
        } else {
            frame
        };
        server.write_all(&next).expect("second actual frame");
    }
    server
        .shutdown(std::net::Shutdown::Write)
        .expect("actual EOF");
    (session, server, peer, reads)
}

fn assert_metered_result(
    route: usize,
    result: Result<PeerProgress, Box<crate::sync::progress::PeerFailure>>,
) {
    match route {
        0 | 4 => {
            let progress = result.expect("actual message-budget exit");
            assert_eq!(progress.messages_processed, if route == 4 { 2 } else { 1 });
            assert_eq!(progress.state, PeerSyncState::Connected);
            assert!(progress.maybe_capabilities.is_some());
        }
        1 | 3 => {
            let progress = result
                .expect_err("real denial/decode failure")
                .maybe_progress
                .expect("original progress");
            assert_eq!(progress.messages_processed, 0);
            assert!(progress.maybe_last_activity_unix_seconds.is_none());
        }
        _ => {
            let failure = result.expect_err("actual duplicate-version protocol error");
            assert!(matches!(
                failure.error,
                SyncRuntimeError::PeerCompatibility { .. }
            ));
            assert_eq!(
                failure
                    .maybe_progress
                    .expect("authorized progress")
                    .messages_processed,
                2
            );
        }
    }
}
