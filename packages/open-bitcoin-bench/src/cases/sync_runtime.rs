// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_network::{
    BlockRelayActivationPolicy, HeaderStore, RelayActivationConfig, VersionMessage,
    WireNetworkMessage,
};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallChainstateStore, FjallNodeStore, ManagedChainstate, SyncRuntimeConfig,
    chainstate::{BasicFilterStartupMode, FlushLifecycle},
    core::{
        chainstate::{Chainstate, FlushPolicyTime},
        consensus::{ConsensusParams, block_hash, chain_proof::block_proof},
        primitives::BlockHeader,
    },
};

use crate::{
    error::BenchError,
    registry::{BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, SYNC_RUNTIME_MAPPING},
    runtime_fixtures::{ScriptedTransport, TempStoreDir, build_block, headers_script, sync_config},
};

const HEADERS_CASE_ID: &str = "sync-runtime.headers-sync";
const BLOCK_CASE_ID: &str = "sync-runtime.block-connect";

fn empty_runtime(
    store: FjallNodeStore,
    config: SyncRuntimeConfig,
    root: BlockHeader,
    case: &'static str,
) -> Result<DurableSyncRuntime, BenchError> {
    let mut core = Chainstate::from_parent(store.coins_view(), vec![], Default::default(), None);
    Chainstate::new()
        .rebind_into(&mut core)
        .map_err(|error| BenchError::case_failed(case, error.to_string()))?;
    let now = FlushPolicyTime::from_unix_seconds(0);
    let source = ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(store),
        core,
        FlushLifecycle::ready(now, now, 0, false),
    );
    let params = ConsensusParams::default();
    let runtime = DurableSyncRuntime::from_live_fixture_source(
        source,
        config,
        params,
        BasicFilterStartupMode::Disabled,
        RelayActivationConfig::default(),
        BlockRelayActivationPolicy::default(),
        false,
    )
    .map_err(|error| BenchError::case_failed(case, error.to_string()))?;
    let mut headers = HeaderStore::default();
    headers
        .insert_header_with_params(root, &params)
        .map_err(|error| BenchError::case_failed(case, error.to_string()))?;
    runtime
        .network_handle()
        .try_admit_fixture_headers(headers, 1_777_225_155, params)
        .map_err(|error| BenchError::case_failed(case, error.to_string()))?;
    Ok(runtime)
}

pub const CASES: [BenchCase; 2] = [
    BenchCase {
        id: HEADERS_CASE_ID,
        group: BenchGroupId::SyncRuntime,
        description: "Syncs scripted headers through the durable runtime until idle.",
        measurement: BenchMeasurement {
            focus: "headers_sync",
            fixture: "scripted_transport",
            durability: BenchDurability::Durable,
        },
        knots_mapping: &SYNC_RUNTIME_MAPPING,
        run_once: run_headers_sync_case,
    },
    BenchCase {
        id: BLOCK_CASE_ID,
        group: BenchGroupId::SyncRuntime,
        description: "Downloads, connects, and persists a scripted block through the durable runtime.",
        measurement: BenchMeasurement {
            focus: "block_download_connect",
            fixture: "scripted_transport",
            durability: BenchDurability::Durable,
        },
        knots_mapping: &SYNC_RUNTIME_MAPPING,
        run_once: run_block_connect_case,
    },
];

fn run_headers_sync_case() -> Result<(), BenchError> {
    let temp_dir = TempStoreDir::new("sync-runtime-headers")?;
    let store = FjallNodeStore::open(temp_dir.path())
        .map_err(|error| BenchError::case_failed(HEADERS_CASE_ID, error.to_string()))?;
    let genesis = build_block(
        open_bitcoin_node::core::primitives::BlockHash::from_byte_array([0; 32]),
        0,
    )?
    .header;
    let child = build_block(block_hash(&genesis), 1)?.header;
    let grandchild = build_block(block_hash(&child), 2)?.header;
    let mut runtime = empty_runtime(
        store,
        open_bitcoin_node::SyncRuntimeConfig {
            max_rounds: 4,
            ..sync_config()
        },
        genesis,
        HEADERS_CASE_ID,
    )?;
    let mut transport = ScriptedTransport::new(vec![
        headers_script(1, vec![child]),
        headers_script(2, vec![grandchild.clone()]),
        headers_script(2, vec![grandchild]),
        Vec::new(),
    ]);

    let summary = runtime
        .sync_until_idle(&mut transport, 1_777_225_155)
        .map_err(|error| BenchError::case_failed(HEADERS_CASE_ID, error.to_string()))?;
    if summary.best_header_height != 2
        || runtime.snapshot_summary().best_header_height != 2
        || summary.headers_received != 1
        || summary.messages_processed != 3
        || summary.failed_peers != 0
        || transport
            .sent_messages()
            .iter()
            .filter(|message| matches!(message, WireNetworkMessage::GetHeaders { .. }))
            .count()
            != 2
        || transport
            .sent_messages()
            .iter()
            .filter(|message| matches!(message, WireNetworkMessage::Version(_)))
            .count()
            != 3
        || !matches!(
            summary.maybe_stop_reason,
            Some(open_bitcoin_node::SyncStopReason::NoProgress {
                rounds_completed: 3
            })
        )
    {
        return Err(BenchError::case_failed(
            HEADERS_CASE_ID,
            format!(
                "header sync fixture did not advance the durable runtime: {summary:?}; sent={:?}",
                transport.sent_messages()
            ),
        ));
    }

    Ok(())
}

fn run_block_connect_case() -> Result<(), BenchError> {
    let temp_dir = TempStoreDir::new("sync-runtime-block")?;
    let store = FjallNodeStore::open(temp_dir.path())
        .map_err(|error| BenchError::case_failed(BLOCK_CASE_ID, error.to_string()))?;
    let genesis = build_block(
        open_bitcoin_node::core::primitives::BlockHash::from_byte_array([0; 32]),
        0,
    )?;
    let genesis_hash = block_hash(&genesis.header);
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Block(genesis.clone()),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = empty_runtime(store, sync_config(), genesis.header.clone(), BLOCK_CASE_ID)?;
    if runtime
        .snapshot_summary()
        .maybe_connected_block_hash
        .is_some()
    {
        return Err(BenchError::case_failed(
            BLOCK_CASE_ID,
            "body fixture must start with no active tip",
        ));
    }

    let summary = runtime
        .sync_once(&mut transport, i64::from(genesis.header.time))
        .map_err(|error| BenchError::case_failed(BLOCK_CASE_ID, error.to_string()))?;
    let snapshot = runtime
        .network_handle()
        .chainstate_snapshot()
        .map_err(|error| BenchError::case_failed(BLOCK_CASE_ID, error.to_string()))?;
    let work = block_proof(genesis.header.bits)
        .map_err(|error| BenchError::case_failed(BLOCK_CASE_ID, error.to_string()))?;
    if summary.blocks_received != 1
        || summary.best_block_height != 0
        || snapshot.tip().is_none_or(|tip| {
            tip.block_hash != genesis_hash || tip.height != 0 || tip.chain_work != work
        })
        || summary.failed_peers != 0
    {
        return Err(BenchError::case_failed(
            BLOCK_CASE_ID,
            "block sync fixture did not report the expected block counters",
        ));
    }
    if runtime
        .store()
        .load_block(genesis_hash)
        .map_err(|error| BenchError::case_failed(BLOCK_CASE_ID, error.to_string()))?
        != Some(genesis)
    {
        return Err(BenchError::case_failed(
            BLOCK_CASE_ID,
            "durable runtime did not persist the synced block",
        ));
    }

    Ok(())
}
