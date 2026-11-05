// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/pow.cpp

use std::{
    fs,
    path::{Path, PathBuf},
};

use open_bitcoin_core::{
    chainstate::{FlushMode, FlushPolicyTime},
    consensus::{CanonicalChain, block_hash},
    primitives::Block,
};
use open_bitcoin_network::{HeadersMessage, WireNetworkMessage};
use serde_json::Value;

use super::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig};
use crate::storage::validation_history::ValidationProvenance;
use crate::{FjallNodeStore, PersistMode, chainstate::BasicFilterStartupMode};

pub(super) struct Source(PathBuf);

impl Source {
    pub(super) fn new() -> Self {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned ordinary test nonce");
        Self(
            std::env::temp_dir()
                .canonicalize()
                .expect("physical temp parent")
                .join(format!(
                    "phase167-ordinary-{:x}",
                    u128::from_le_bytes(nonce)
                )),
        )
    }

    pub(super) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        if self.0.exists() {
            fs::remove_dir_all(&self.0).expect("ordinary test handles all dropped");
        }
    }
}

pub(super) fn corpus() -> Value {
    serde_json::from_str(include_str!(
        "../../../../testdata/phase167/regtest-history.json"
    ))
    .expect("frozen serialized ordinary corpus")
}

pub(super) fn expectations() -> Value {
    serde_json::from_str(include_str!(
        "../../../../../docs/parity/fixtures/phase167-regtest-expectations.json"
    ))
    .expect("independent source-derived ordinary expectations")
}

pub(super) fn row(source: &Value, height: usize) -> &Value {
    let item = &source["rows"][height];
    assert_eq!(item["height"].as_u64(), Some(height as u64));
    item
}

pub(super) fn named_negative<'a>(source: &'a Value, name: &str) -> &'a Value {
    source["negatives"]
        .as_array()
        .expect("negative rows")
        .iter()
        .find(|item| item["name"] == name)
        .expect("named frozen negative")
}

pub(super) fn bytes(hex: &str) -> Vec<u8> {
    assert!(hex.len().is_multiple_of(2), "complete hex fixture bytes");
    hex.as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let hi = (pair[0] as char).to_digit(16).expect("hex high");
            let lo = (pair[1] as char).to_digit(16).expect("hex low");
            ((hi << 4) | lo) as u8
        })
        .collect()
}

pub(super) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(super) fn block(source_row: &Value) -> Block {
    let encoded = source_row["block_hex"].as_str().expect("serialized body");
    let block = open_bitcoin_codec::parse_block(&bytes(encoded)).expect("canonical block bytes");
    assert_eq!(
        hex(block_hash(&block.header).as_bytes()),
        source_row["block_hash_raw"]
            .as_str()
            .expect("independent block identity"),
    );
    block
}

pub(super) fn assert_block_boundary(block: &Block, expected: &Value) {
    let height = expected["height"].as_u64().expect("independent height");
    let subsidy_sats = expected["subsidy_sats"]
        .as_str()
        .expect("independent subsidy")
        .parse::<i64>()
        .expect("subsidy amount");
    let fees_sats = expected["fees_sats"]
        .as_str()
        .expect("independent fees")
        .parse::<i64>()
        .expect("fee amount");
    assert_eq!(subsidy_sats, 5_000_000_000_i64 >> (height / 150));
    let observed_fees = block
        .transactions
        .iter()
        .skip(1)
        .map(|spend| {
            assert_eq!(spend.inputs.len(), 1, "isolated source-bound spend");
            2_000_000_000_i64
                - spend
                    .outputs
                    .iter()
                    .map(|output| output.value.to_sats())
                    .sum::<i64>()
        })
        .sum::<i64>();
    assert_eq!(observed_fees, fees_sats);
    let coinbase_sats = expected["coinbase_sats"]
        .as_str()
        .expect("independent coinbase")
        .parse::<i64>()
        .expect("coinbase amount");
    assert_eq!(coinbase_sats, subsidy_sats + fees_sats);
    assert_eq!(
        block.transactions[0]
            .outputs
            .iter()
            .map(|output| output.value.to_sats())
            .sum::<i64>(),
        coinbase_sats
    );
    assert_eq!(
        block.header.bits as u64,
        expected["bits"].as_u64().expect("selected bits")
    );
    assert_eq!(
        hex(block_hash(&block.header).as_bytes()),
        expected["block_hash_raw"]
            .as_str()
            .expect("source-derived hash")
    );
}

pub(super) fn assert_creating_ancestor_201(block: &Block, expected: &Value) {
    let spend = &block.transactions[1];
    let input = &spend.inputs[0];
    assert_eq!(
        spend.version,
        expected["version"].as_i64().expect("version") as i32
    );
    assert_eq!(
        input.sequence as u64,
        expected["sequence"].as_u64().expect("time-lock sequence")
    );
    assert_eq!(
        format!(
            "{}:{}",
            hex(input.previous_output.txid.as_bytes()),
            input.previous_output.vout
        ),
        expected["reserved_outpoint"]
            .as_str()
            .expect("reserved height1 outpoint")
    );
    assert_eq!(expected["creating_height"].as_u64(), Some(1));
    assert_ne!(
        expected["creating_parent_mtp"],
        expected["candidate_parent_mtp"]
    );
    let units = input.sequence & 0xffff;
    assert_eq!(Some(u64::from(units)), expected["sequence_units"].as_u64());
    let creating_mtp = expected["creating_parent_mtp"]
        .as_i64()
        .expect("independent creating MTP");
    assert_eq!(
        creating_mtp + (i64::from(units) << 9) - 1,
        expected["last_invalid_time"]
            .as_i64()
            .expect("independent lock boundary")
    );
    assert!(
        expected["candidate_parent_mtp"]
            .as_i64()
            .expect("independent parent MTP")
            > creating_mtp + (i64::from(units) << 9) - 1
    );
}

pub(super) fn selected_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        ..Default::default()
    }
}

pub(super) fn open_genesis(source: &Source) -> DurableSyncRuntime {
    let opened = FjallNodeStore::open_canonical_genesis(source.path(), CanonicalChain::Regtest)
        .expect("proved default-regtest genesis");
    let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
        opened,
        selected_config(),
        BasicFilterStartupMode::Enabled,
    )
    .expect("genuine default-regtest owner");
    runtime
        .network
        .connect_outbound_peer(1, 0)
        .expect("socket-free peer");
    runtime
}

pub(super) fn reopen(source: &Source) -> DurableSyncRuntime {
    let runtime = DurableSyncRuntime::reopen_canonical_history_configured(
        source.path(),
        selected_config(),
        BasicFilterStartupMode::PreserveSaved,
    )
    .expect("actual safe copied/original history reproof");
    runtime
        .network
        .connect_outbound_peer(1, 0)
        .expect("reopened socket-free peer");
    runtime
}

pub(super) fn admit_and_connect(runtime: &DurableSyncRuntime, block: &Block) {
    runtime
        .network
        .receive_message(
            1,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![block.header.clone()],
            }),
            i64::MIN,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("real contextual header ingress");
    runtime
        .store
        .save_block(block, PersistMode::Sync)
        .expect("retained serialized body");
    runtime
        .network
        .connect_local_block(block, runtime.verify_flags, runtime.consensus_params)
        .expect("real managed durable acceptance");
}

pub(super) fn assert_scripts_valid(runtime: &DurableSyncRuntime, block: &Block) {
    assert_eq!(
        runtime
            .store
            .validation_provenance(block_hash(&block.header))
            .expect("durable accepted provenance"),
        ValidationProvenance::ScriptsValid,
    );
}

pub(super) fn checkpoint(runtime: &DurableSyncRuntime, height: u32, expected: &Value) {
    runtime
        .network
        .persist_validation_header_snapshot()
        .expect("earned header checkpoint");
    runtime
        .network
        .flush_coins(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(70_000 + u64::from(height)),
            u64::MAX,
        )
        .expect("durable coins checkpoint");
    for _ in 0..=height + 1 {
        let turn = runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("real BASIC turn");
        if turn.maybe_accepted_lag == Some(0) {
            break;
        }
    }
    let progress = runtime
        .network
        .maybe_basic_index_progress()
        .expect("progress")
        .expect("enabled");
    assert_eq!(
        progress
            .maybe_processed_endpoint()
            .expect("processed")
            .height(),
        height
    );
    assert_eq!(
        progress.maybe_processed_endpoint(),
        progress.maybe_safe_durable_endpoint()
    );
    let hash = runtime
        .network
        .chainstate_snapshot()
        .expect("checkpoint state")
        .tip()
        .expect("checkpoint tip")
        .block_hash;
    assert_eq!(
        runtime
            .store
            .basic_filter_projection(height)
            .expect("active filter"),
        hash
    );
    let filter = runtime
        .store
        .load_basic_filter_record(hash)
        .expect("filter record")
        .expect("present");
    assert_eq!(
        hex(filter.encoded_bytes()),
        expected["basic"]["encoded"]
            .as_str()
            .expect("expected BASIC bytes")
    );
    let identity = filter.identity();
    assert_eq!(
        hex(identity.filter_hash().as_bytes()),
        expected["basic"]["hash_raw"]
            .as_str()
            .expect("independent BASIC hash")
    );
    assert_eq!(
        hex(identity.filter_header().as_bytes()),
        expected["basic"]["header_raw"]
            .as_str()
            .expect("independent BASIC header")
    );
    assert_eq!(
        hex(identity.previous_header().as_bytes()),
        expected["basic"]["predecessor_raw"]
            .as_str()
            .expect("independent predecessor")
    );
}

pub(super) fn assert_tip(runtime: &DurableSyncRuntime, expected: &Value) {
    let state = runtime
        .network
        .chainstate_snapshot()
        .expect("actual accepted state");
    let tip = state.tip().expect("earned tip");
    assert_eq!(
        tip.height as u64,
        expected["height"].as_u64().expect("expected height")
    );
    assert_eq!(
        hex(tip.block_hash.as_bytes()),
        expected["block_hash_raw"]
            .as_str()
            .expect("expected raw hash")
    );
    assert_eq!(
        tip.chain_work.to_hex(),
        expected["chain_work_hex"]
            .as_str()
            .expect("independent work")
    );
    assert_eq!(
        tip.median_time_past,
        expected["median_time_past"]
            .as_i64()
            .expect("independent median")
    );
}

pub(super) fn accepted_snapshot(
    runtime: &DurableSyncRuntime,
) -> open_bitcoin_core::chainstate::ChainstateSnapshot {
    runtime
        .network
        .chainstate_snapshot()
        .expect("complete accepted state")
}

pub(super) fn basic_state(
    runtime: &DurableSyncRuntime,
) -> Option<crate::storage::filter_index::StoredFilterState> {
    runtime
        .store
        .maybe_basic_filter_state()
        .expect("BASIC state")
}

pub(super) fn assert_frozen_utxos(runtime: &DurableSyncRuntime, expected: &Value) {
    // Arrange: independently serialized ledger facts include every live outpoint.
    let snapshot = accepted_snapshot(runtime);
    let mut actual: Vec<Value> = snapshot
        .utxos
        .iter()
        .map(|(outpoint, coin)| {
            serde_json::json!({
                "sats": coin.output.value.to_sats().to_string(),
                "script": hex(coin.output.script_pubkey.as_bytes()),
                "txid": hex(outpoint.txid.as_bytes()),
                "vout": outpoint.vout,
                "coinbase": coin.is_coinbase,
                "height": coin.created_height,
                "creating_parent_mtp": coin.created_median_time_past,
            })
        })
        .collect();
    let mut independent = expected["utxos"]
        .as_array()
        .expect("complete frozen outpoints")
        .clone();
    let key = |entry: &Value| {
        (
            entry["txid"]
                .as_str()
                .expect("raw transaction identity")
                .to_owned(),
            entry["vout"].as_u64().expect("output index"),
        )
    };
    actual.sort_by_key(&key);
    independent.sort_by_key(&key);
    // Assert: complete coins, values, scripts, heights and creating MTP match.
    assert_eq!(actual, independent);
    assert_eq!(
        snapshot.utxos.len() as u64,
        expected["utxo_count"]
            .as_u64()
            .expect("frozen complete count")
    );
}

pub(super) fn physical_manifest(root: &Path) -> Vec<(PathBuf, u64, [u8; 32])> {
    fn visit(root: &Path, directory: &Path, rows: &mut Vec<(PathBuf, u64, [u8; 32])>) {
        let mut entries: Vec<_> = fs::read_dir(directory)
            .expect("physical inventory directory")
            .map(|entry| entry.expect("physical inventory entry"))
            .collect();
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let path = entry.path();
            let kind = fs::symlink_metadata(&path)
                .expect("physical entry metadata")
                .file_type();
            assert!(
                !kind.is_symlink(),
                "hostile source snapshot has no symlink proxy"
            );
            if kind.is_dir() {
                visit(root, &path, rows);
            } else {
                assert!(kind.is_file(), "only regular source files are inventoried");
                let bytes = fs::read(&path).expect("complete physical file bytes");
                rows.push((
                    path.strip_prefix(root)
                        .expect("relative physical path")
                        .to_path_buf(),
                    bytes.len() as u64,
                    open_bitcoin_core::consensus::crypto::Sha256::digest(&bytes),
                ));
            }
        }
    }
    let mut rows = Vec::new();
    visit(root, root, &mut rows);
    rows
}

pub(super) fn corrupt_untrusted_work_after_closed_runtime(source: &Source) {
    let store = FjallNodeStore::open(source.path()).expect("test-owned closed history store");
    let mut entries = store
        .load_header_entries()
        .expect("stored headers")
        .expect("history rows")
        .entries;
    let tip = entries
        .iter_mut()
        .max_by_key(|entry| entry.height)
        .expect("ordinary tip");
    assert!(tip.height >= 150);
    tip.chain_work = open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO;
    store
        .save_header_entries(&entries, PersistMode::Sync)
        .expect("test-owned deliberately inconsistent header work");
    drop(store);
}
