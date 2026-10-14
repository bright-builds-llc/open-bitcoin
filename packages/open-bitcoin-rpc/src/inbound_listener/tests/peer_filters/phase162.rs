// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Mature accepted Fjall branches at the real CF executor, without codec pressure.

use super::*;
use crate::inbound_listener::connection_runtime::filter_serving::{
    InboundResponseBatch, SocketFilterWriter,
};
use open_bitcoin_network::{InboundAdmissionDecision, InboundPermissionDecision};
use open_bitcoin_node::chainstate::BasicFilterStartupMode;
use open_bitcoin_node::core as core_types;
use open_bitcoin_node::core::{
    chainstate::{FlushMode, FlushPolicyTime},
    consensus::ScriptVerifyFlags,
};
use open_bitcoin_node::network::filter_transport::{FilterTransportError, PreparedFilterResponse};
use serde_json::Value;
use std::path::Path;

mod recipe {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../testdata/phase162/recipe.rs"
    ));
}
mod child {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../testdata/phase162/process.rs"
    ));
}
const PEER: u64 = 162_009;

fn oracle(recipe: &recipe::Recipe) -> Value {
    let blocks: Vec<_> = recipe
        .blocks
        .iter()
        .enumerate()
        .map(|(height, block)| {
            let raw = core_types::codec::encode_block(block).expect("raw recipe block");
            serde_json::json!({"name":format!("active-{height}"), "previous_name":if height==0 {
            String::new() } else { format!("active-{}",height-1) },
            "raw_block":raw.iter().map(|byte|format!("{byte:02x}")).collect::<String>()})
        })
        .collect();
    let input = serde_json::json!({"recipe_id":recipe::RECIPE_ID,"network_magic":"fabfb5da","blocks":blocks});
    let output = child::run(
        std::process::Command::new("bun")
            .arg("scripts/phase162-filter-oracle.ts")
            .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")),
        serde_json::to_vec(&input).expect("bounded raw JSON"),
        child::ORACLE_LIMITS,
    )
    .expect("bounded raw oracle");
    assert!(output.reaped && output.status.success() && output.pid > 1);
    assert!(output.stderr.len() <= child::ORACLE_LIMITS.stderr_bytes);
    serde_json::from_slice(&output.stdout).expect("independent oracle JSON")
}

fn fixture(path: &Path) -> (FilterListenerFixture, Vec<Vec<u8>>) {
    let sync = SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        dns_seeds: Vec::new(),
        manual_peers: Vec::new(),
        ..Default::default()
    };
    let mut runtime = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(path).expect("fresh Fjall"),
        sync.clone(),
        BasicFilterStartupMode::PreserveSaved,
    )
    .expect("no synthetic root/index");
    let recipe = recipe::Recipe::new(
        sync.network.consensus_params(),
        ScriptVerifyFlags::P2SH,
        102,
    );
    for (height, block) in recipe.blocks.iter().enumerate() {
        runtime
            .store()
            .save_block(block, PersistMode::Sync)
            .expect("actual body");
        let receipt = runtime
            .network_handle()
            .connect_local_block(block, recipe.flags, recipe.params)
            .expect("normal-maturity genuine receipt");
        assert_eq!(receipt.height as usize, height);
        assert_eq!(receipt.block_hash, block_hash(&block.header));
        if height == 0 {
            runtime
                .network_handle()
                .flush_coins(FlushMode::Always, FlushPolicyTime::new(2000), u64::MAX)
                .expect("genuine genesis coins/meta");
            drop(runtime);
            runtime = DurableSyncRuntime::open_configured(
                FjallNodeStore::open(path).expect("cold genesis"),
                sync.clone(),
                BasicFilterStartupMode::Enabled,
            )
            .expect("configured owner after genesis");
        }
    }
    let network = runtime.network_handle();
    for turn in 0..256 {
        let progress = network
            .drive_basic_filter_index_turn()
            .expect("ordinary catch-up");
        if progress.maybe_accepted_lag == Some(0) {
            break;
        }
        assert!(turn < 255, "finite catch-up");
    }
    network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(500_000), u64::MAX)
        .expect("real coins/undo/meta settlement");
    let turn = network
        .drive_basic_filter_index_turn()
        .expect("safe fence promotion");
    let progress = turn.maybe_progress.expect("public executed progress");
    let safe = progress.maybe_safe_durable_endpoint().expect("earned safe");
    assert_eq!(safe.height(), 101);
    assert_eq!(recipe.params.coinbase_maturity, 100);
    assert_eq!(recipe.blocks[101].transactions.len(), 3);
    let undo = runtime
        .store()
        .load_undo(block_hash(&recipe.blocks[101].header))
        .expect("real undo")
        .expect("persisted undo");
    let state = network.chainstate_snapshot().expect("accepted position");
    let inputs = core_types::chainstate::BasicFilterInputs::from_historical(
        &recipe.blocks[101],
        &state.active_chain[101],
        Some(core_types::chainstate::HistoricalBlockUndo {
            block_hash: block_hash(&recipe.blocks[101].header),
            undo: &undo,
        }),
    )
    .expect("actual historical and same-block resolution");
    assert_eq!(
        inputs
            .spent_scripts()
            .map(<[u8]>::to_vec)
            .collect::<Vec<_>>(),
        recipe.spent_scripts[101]
    );
    assert_eq!(recipe.wallet_outpoint.vout, 1);
    assert_eq!(recipe.wallet_script, recipe::wallet_script());
    let branch = recipe.fork(101, 1, 0x62);
    assert_eq!(recipe.fork_spent_scripts(101, &branch).len(), 1);
    let oracle = oracle(&recipe);
    let wire = oracle["wire"].as_array().expect("wire");
    let expected = (99..=101)
        .map(|height| {
            let row = wire
                .iter()
                .find(|row| row["name"] == format!("active-{height}-cfilter"))
                .expect("raw-derived frame");
            let hex = row["expected"]["frame_hex"]
                .as_str()
                .expect("pinned Python frame");
            (0..hex.len())
                .step_by(2)
                .map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).expect("hex"))
                .collect()
        })
        .collect();
    network
        .configure_basic_peer_serving(false)
        .expect("general off, explicit permission only");
    let config = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        peer_block_filters: false,
        inbound: loopback_config_with_permission_classes(
            2,
            1,
            loopback_permission_registry(&["in", "blockfilters"]),
        ),
        ..Default::default()
    };
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_fixture_runtime_config_with_network_handle(
            &config,
            network,
            Some(runtime.store().clone()),
        )
        .expect("same configured durable authority"),
    ));
    (
        FilterListenerFixture {
            path: path.to_owned(),
            blocks: recipe.blocks,
            runtime,
            context,
            config,
        },
        expected,
    )
}

async fn admitted(
    fixture: &FilterListenerFixture,
    server: &TcpStream,
) -> InboundPermissionDecision {
    let mut context = fixture.context.lock().await;
    let InboundAdmissionDecision::Admit(record) = context
        .record_inbound_admission_for_remote_addr(
            PEER,
            server.peer_addr().expect("actual peer endpoint"),
            false,
        )
        .expect("actual admission")
    else {
        panic!("admitted")
    };
    let capability = record.permission_decision.basic_filter_capability();
    assert!(capability.explicit_permission_requested && capability.effective_permission);
    for message in [
        WireNetworkMessage::Version(VersionMessage {
            nonce: PEER,
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
    ] {
        drop(
            context
                .prepare_inbound_wire_message(PEER, message, 50_000)
                .expect("actual handshake state")
                .resolve(),
        );
    }
    let peers = fixture
        .runtime
        .network_handle()
        .peer_manager_snapshot()
        .expect("actual own-Version effects");
    let record = peers
        .peer_state(PEER)
        .expect("handshaken peer")
        .maybe_inbound_record
        .as_ref()
        .expect("inbound record");
    assert!(!record.permission_decision.active_effects().is_empty());
    record.permission_decision.clone()
}

async fn prepared(
    fixture: &FilterListenerFixture,
    server: &TcpStream,
) -> (PreparedFilterResponse, InboundPermissionDecision) {
    let permissions = admitted(fixture, server).await;
    let mut responses = fixture
        .context
        .lock()
        .await
        .prepare_inbound_wire_message(
            PEER,
            WireNetworkMessage::GetCFilters(GetCFilters {
                filter_type: 0,
                start_height: 99,
                stop_hash: fixture.hash(101),
            }),
            50_001,
        )
        .expect("actual permission-gated CF request")
        .resolve();
    assert!(!responses.failed);
    assert_eq!(responses.responses.len(), 1);
    let InboundWireStep::Filter(response) =
        responses.responses.pop().expect("earned complete range")
    else {
        panic!("CF")
    };
    (response, permissions)
}

struct PartialWriter<'a> {
    fixture: &'a FilterListenerFixture,
    stream: &'a TcpStream,
    fail_on: usize,
    partial: usize,
    calls: usize,
    invalidate: bool,
}
impl InboundFilterWriter for PartialWriter<'_> {
    async fn write(&mut self, bytes: &[u8]) -> io::Result<WriteWireMessageOutcome> {
        let context = self
            .fixture
            .context
            .try_lock()
            .expect("context released across writer await");
        let handle = self.fixture.runtime.network_handle();
        handle
            .network_info()
            .expect("authority free during awaited phase162_physical_bytes write");
        let snapshot = handle
            .basic_filter_transport_snapshot()
            .expect("actual active charge");
        assert_eq!(snapshot.active_frame_bytes, bytes.len() as u64);
        assert!(snapshot.suffix_bytes >= snapshot.active_frame_bytes);
        assert_eq!(
            (snapshot.generic_descriptors, snapshot.generic_pending_bytes),
            (2, 64)
        );
        drop(context);
        self.calls += 1;
        if self.calls == self.fail_on {
            write_all(self.stream, &bytes[..self.partial]).await?;
            return Err(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "actual partial CF frame",
            ));
        }
        write_all(self.stream, bytes).await?;
        if self.invalidate {
            handle
                .disconnect_peer(PEER)
                .expect("actual old session invalidation");
            admitted(self.fixture, self.stream).await;
        }
        Ok(WriteWireMessageOutcome::Written)
    }
}

async fn partial_case(position: usize, partial: usize, invalidate: bool) {
    // Arrange
    let cleanup = Phase162WriteCleanup::new();
    let (fixture, expected) = fixture(&cleanup.0);
    let network = fixture.runtime.network_handle();
    let (client, server) = tcp_pair().await;
    let (response, _permissions) = prepared(&fixture, &server).await;
    let evidence = Arc::new(std::sync::Mutex::new(listener_evidence(&[])));
    let batch = InboundResponseBatch {
        steps: phase162_generic_suffix(&fixture).await.into(),
        network: &network,
        evidence: &evidence,
    };
    // Act
    let result = execute_inbound_filter(
        response,
        &network,
        PartialWriter {
            fixture: &fixture,
            stream: &server,
            fail_on: position,
            partial,
            calls: 0,
            invalidate,
        },
    )
    .await;
    drop(server);
    let bytes = phase162_physical_bytes(&client, 1024).await;
    drop(client);
    drop(batch);
    network.disconnect_peer(PEER).expect("actual peer cleanup");
    // Assert
    let failed = result.expect_err("incomplete/stale range cannot succeed");
    let complete = usize::from(position == 2 || invalidate);
    phase162_assert_prefix(&failed, &expected, complete);
    let mut wanted = expected[..complete].concat();
    if !invalidate {
        wanted.extend_from_slice(&expected[complete][..partial]);
    }
    assert_eq!(
        bytes, wanted,
        "entire independent phase162_physical_bytes wire prefix"
    );
    if invalidate {
        assert!(matches!(
            failed.cause,
            FilterWriteFailure::Transport(FilterTransportError::Invalidated)
        ));
    } else {
        assert!(
            matches!(failed.cause,FilterWriteFailure::Write(ref error) if error.kind()==io::ErrorKind::ConnectionReset)
        );
    }
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("all CF/generic suffixes dropped"),
        FilterTransportSnapshot::default()
    );
    eprintln!(
        "phase162 inbound partial position={position} partial={partial} epoch={invalidate}: physical={} complete_frames={complete} complete_bytes={} current_range=false resources=0",
        bytes.len(),
        failed.progress.bytes
    );
    drop(network);
    drop(fixture);
    drop(cleanup);
}

#[tokio::test]
async fn phase162_inbound_write_partial_first_header() {
    partial_case(1, 13, false).await;
}
#[tokio::test]
async fn phase162_inbound_write_partial_second_header() {
    partial_case(2, 13, false).await;
}
#[tokio::test]
async fn phase162_inbound_write_partial_first_payload() {
    partial_case(1, 31, false).await;
}
#[tokio::test]
async fn phase162_inbound_write_partial_second_payload() {
    partial_case(2, 31, false).await;
}
#[tokio::test]
async fn phase162_inbound_write_epoch_handed_full_frame() {
    partial_case(usize::MAX, 0, true).await;
}

async fn socket_case(case: SocketCase) {
    // Arrange
    let cleanup = Phase162WriteCleanup::new();
    let (fixture, expected) = fixture(&cleanup.0);
    let network = fixture.runtime.network_handle();
    let (client, server) = tcp_pair().await;
    let (response, permissions) = prepared(&fixture, &server).await;
    let expired = matches!(case, SocketCase::Expired);
    let policy = ResourceGovernancePolicy {
        idle_peer_timeout_seconds: if expired { 0 } else { 5 },
        ..Default::default()
    };
    let mut queue = RuntimeQueuePressureState::default();
    if matches!(case, SocketCase::Queue) {
        queue.record_aggregate_queued_messages(policy.max_aggregate_queued_messages);
    }
    let baseline = queue.queue_pressure_input(
        permissions.active_effects().to_vec(),
        permissions.inactive_effects().to_vec(),
    );
    let shutdown = tokio::sync::Notify::new();
    let padding = if matches!(case, SocketCase::Shutdown | SocketCase::Disconnect) {
        tokio::time::timeout(Duration::from_secs(3), phase162_saturate(&server))
            .await
            .expect("entire socket priming deadline")
    } else {
        0
    };
    let mut maybe_client = Some(client);
    if matches!(case, SocketCase::Disconnect) {
        drop(maybe_client.take());
        tokio::time::timeout(Duration::from_secs(5), server.readable())
            .await
            .expect("peer close deadline")
            .expect("peer close ready");
        let read = server.try_read(&mut [0; 1]);
        assert!(read.is_err() || matches!(read, Ok(0)));
    }
    if matches!(case, SocketCase::Shutdown) {
        shutdown.notify_one();
    }
    let evidence = Arc::new(std::sync::Mutex::new(listener_evidence(&[])));
    let batch = InboundResponseBatch {
        steps: phase162_generic_suffix(&fixture).await.into(),
        network: &network,
        evidence: &evidence,
    };
    // Act: real policy/queue/shutdown handling, including pending-write retirement on return.
    let result = tokio::time::timeout(
        Duration::from_secs(7),
        execute_inbound_filter(
            response,
            &network,
            SocketFilterWriter {
                stream: &server,
                policy: &policy,
                connected: 50_000,
                activity: 50_001,
                handshake: InboundHandshakeState::Established,
                queue: &mut queue,
                active: permissions.active_effects().to_vec(),
                inactive: permissions.inactive_effects().to_vec(),
                shutdown: &shutdown,
            },
        ),
    )
    .await
    .expect("finite actual writer deadline");
    drop(server);
    let bytes = if let Some(client) = maybe_client {
        let bytes = phase162_physical_bytes(&client, 16 * 1024 * 1024).await;
        drop(client);
        bytes
    } else {
        Vec::new()
    };
    drop(batch);
    network.disconnect_peer(PEER).expect("settled actual peer");
    // Assert
    assert_eq!(
        queue.queue_pressure_input(
            permissions.active_effects().to_vec(),
            permissions.inactive_effects().to_vec()
        ),
        baseline
    );
    if matches!(case, SocketCase::Complete) {
        let progress = result.expect("real complete-frame receipts");
        assert_eq!(progress.messages, 3);
        assert_eq!(progress.bytes, 189);
        assert_eq!(bytes, expected.concat());
    } else {
        let failed = result.expect_err("terminal control cannot claim completed range");
        phase162_assert_prefix(&failed, &expected, 0);
        match case {
            SocketCase::Expired => assert!(
                matches!(failed.cause,FilterWriteFailure::Rejected(ref event) if event.label=="idle_peer")
            ),
            SocketCase::Queue => assert!(matches!(failed.cause, FilterWriteFailure::Rejected(_))),
            SocketCase::Shutdown => assert!(
                matches!(failed.cause,FilterWriteFailure::Write(ref error) if error.kind()==io::ErrorKind::Interrupted)
            ),
            SocketCase::Disconnect => assert!(matches!(failed.cause, FilterWriteFailure::Write(_))),
            SocketCase::Complete => unreachable!(),
        }
        if !matches!(case, SocketCase::Disconnect) {
            assert_eq!(bytes.len(), padding);
            assert!(bytes.iter().all(|byte| *byte == 0));
        }
    }
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("zero CF/generic charges"),
        FilterTransportSnapshot::default()
    );
    eprintln!(
        "phase162 inbound socket {case:?}: priming_bytes={padding} queue_baseline_restored=true resources=0"
    );
    drop(network);
    drop(fixture);
    drop(cleanup);
}

#[tokio::test]
async fn phase162_inbound_write_expired_deadline() {
    socket_case(SocketCase::Expired).await;
}
#[tokio::test]
async fn phase162_inbound_write_queue_refusal() {
    socket_case(SocketCase::Queue).await;
}
#[tokio::test]
async fn phase162_inbound_write_shutdown_returned_cancellation() {
    socket_case(SocketCase::Shutdown).await;
}
#[tokio::test]
async fn phase162_inbound_write_socket_disconnect() {
    socket_case(SocketCase::Disconnect).await;
}
#[tokio::test]
async fn phase162_inbound_write_complete_control() {
    socket_case(SocketCase::Complete).await;
}

struct BlockedWriter<'a> {
    stream: &'a TcpStream,
    entered: &'a tokio::sync::Notify,
}
impl InboundFilterWriter for BlockedWriter<'_> {
    async fn write(&mut self, bytes: &[u8]) -> io::Result<WriteWireMessageOutcome> {
        write_all(self.stream, &bytes[..31]).await?;
        self.entered.notify_one();
        std::future::pending().await
    }
}
#[tokio::test]
async fn phase162_inbound_write_executor_future_drop() {
    // Arrange
    let cleanup = Phase162WriteCleanup::new();
    let (fixture, expected) = fixture(&cleanup.0);
    let network = fixture.runtime.network_handle();
    let (client, server) = tcp_pair().await;
    let (response, _permissions) = prepared(&fixture, &server).await;
    let entered = tokio::sync::Notify::new();
    // Act: a genuinely partial write remains unacknowledged until its executor is dropped.
    let mut execution = Box::pin(execute_inbound_filter(
        response,
        &network,
        BlockedWriter {
            stream: &server,
            entered: &entered,
        },
    ));
    tokio::time::timeout(Duration::from_secs(5),async {
        tokio::select! {result=&mut execution=>panic!("unexpected result: {result:?}"),()=entered.notified()=>{}}
    }).await.expect("handed frame deadline");
    let before = network
        .basic_filter_transport_snapshot()
        .expect("live handed charge");
    assert_eq!(before.requests, 1);
    assert_eq!(before.active_frame_bytes, expected[0].len() as u64);
    assert!(before.suffix_bytes > before.active_frame_bytes);
    network
        .disconnect_peer(PEER)
        .expect("actual old session invalidation while pending");
    admitted(&fixture, &server).await;
    let invalidated = network
        .basic_filter_transport_snapshot()
        .expect("handed bytes retained");
    assert_eq!(invalidated.requests, 1);
    assert_eq!(invalidated.suffix_bytes, before.active_frame_bytes);
    assert_eq!(invalidated.active_frame_bytes, before.active_frame_bytes);
    drop(execution);
    drop(server);
    let bytes = phase162_physical_bytes(&client, 1024).await;
    drop(client);
    network.disconnect_peer(PEER).expect("actual disconnect");
    // Assert: no Written outcome or completion was ever returned for those31bytes.
    assert_eq!(bytes, expected[0][..31]);
    assert_eq!(
        network
            .basic_filter_transport_snapshot()
            .expect("owner dropped"),
        FilterTransportSnapshot::default()
    );
    drop(network);
    drop(fixture);
    drop(cleanup);
}
