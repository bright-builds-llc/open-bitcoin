// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_network::{HeadersMessage, VersionMessage, WireNetworkMessage};
use open_bitcoin_node::{DurableSyncRuntime, FjallNodeStore, core::consensus::block_hash};

use crate::{
    error::BenchError,
    registry::{BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, SYNC_RUNTIME_MAPPING},
    runtime_fixtures::{
        ScriptedTransport, TempStoreDir, build_block, header, headers_script, sync_config,
    },
};

const HEADERS_CASE_ID: &str = "sync-runtime.headers-sync";
const BLOCK_CASE_ID: &str = "sync-runtime.block-connect";

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
    let genesis = header(
        open_bitcoin_node::core::primitives::BlockHash::from_byte_array([0; 32]),
        21,
    );
    let child = header(block_hash(&genesis), 22);
    let grandchild = header(block_hash(&child), 23);
    let mut runtime = DurableSyncRuntime::open(
        store,
        open_bitcoin_node::SyncRuntimeConfig {
            max_rounds: 4,
            ..sync_config()
        },
    )
    .map_err(|error| BenchError::case_failed(HEADERS_CASE_ID, error.to_string()))?;
    let mut transport = ScriptedTransport::new(vec![
        headers_script(0, vec![genesis]),
        headers_script(1, vec![child]),
        headers_script(2, vec![grandchild]),
        Vec::new(),
    ]);

    let summary = runtime
        .sync_until_idle(&mut transport, 1_777_225_155)
        .map_err(|error| BenchError::case_failed(HEADERS_CASE_ID, error.to_string()))?;
    if summary.best_header_height != 2 || runtime.snapshot_summary().best_header_height != 2 {
        return Err(BenchError::case_failed(
            HEADERS_CASE_ID,
            "header sync fixture did not advance the durable runtime",
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
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone()],
        }),
        WireNetworkMessage::Block(genesis.clone()),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config())
        .map_err(|error| BenchError::case_failed(BLOCK_CASE_ID, error.to_string()))?;

    let summary = runtime
        .sync_once(&mut transport, i64::from(genesis.header.time))
        .map_err(|error| BenchError::case_failed(BLOCK_CASE_ID, error.to_string()))?;
    if summary.blocks_received != 1 || summary.best_block_height != 0 {
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
