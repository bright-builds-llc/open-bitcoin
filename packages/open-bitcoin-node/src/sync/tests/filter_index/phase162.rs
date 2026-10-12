// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real acceptance adapter for the shared normal-maturity history.

use super::*;
use crate::chainstate::BasicFilterStartupMode;
use core_types::{
    chainstate::{FlushMode, FlushPolicyTime},
    consensus::ScriptVerifyFlags,
};
use open_bitcoin_core as core_types;

pub(in crate::sync::tests) mod recipe {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../testdata/phase162/recipe.rs"
    ));
}

mod process {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../testdata/phase162/process.rs"
    ));
}

mod corruption;
mod missing_history;
mod publication_faults;
mod recovery;

pub(in crate::sync::tests) struct AcceptedRecipe {
    pub path: PathBuf,
    pub runtime: DurableSyncRuntime,
    pub recipe: recipe::Recipe,
    pub receipts: Vec<ChainPosition>,
}

impl AcceptedRecipe {
    pub fn new(count: u32) -> Self {
        Self::at(
            reserved_filter_path("phase162-normal-maturity"),
            count,
            BasicFilterStartupMode::Enabled,
        )
    }

    fn at(path: PathBuf, count: u32, mode: BasicFilterStartupMode) -> Self {
        let mut runtime = DurableSyncRuntime::open_configured(
            FjallNodeStore::open(&path).expect("empty actual Fjall"),
            sync_config(),
            BasicFilterStartupMode::PreserveSaved,
        )
        .expect("actual configured runtime");
        let recipe = recipe::Recipe::new(runtime.consensus_params, runtime.verify_flags, count);
        assert_eq!(recipe.params, SyncNetwork::Regtest.consensus_params());
        assert_eq!(recipe.flags, runtime.verify_flags);
        let mut receipts = Vec::with_capacity(count as usize);
        for block in &recipe.blocks {
            runtime
                .store()
                .save_block(block, PersistMode::Sync)
                .expect("ordinary body persistence");
            let receipt = runtime
                .network_handle()
                .connect_local_block(block, recipe.flags, recipe.params)
                .expect("normal-maturity accepted connect receipt");
            assert_eq!(receipt.height as usize, receipts.len());
            assert_eq!(receipt.block_hash, block_hash(&block.header));
            receipts.push(receipt);
            if receipts.len() == 1 {
                runtime
                    .network_handle()
                    .flush_coins(
                        FlushMode::Always,
                        FlushPolicyTime::from_unix_seconds(2000),
                        u64::MAX,
                    )
                    .expect("accepted genesis durability");
                drop(runtime);
                runtime = DurableSyncRuntime::open_configured(
                    FjallNodeStore::open(&path).expect("accepted genesis all-handle-close reopen"),
                    sync_config(),
                    mode,
                )
                .expect("configured owner after genuine genesis");
                assert_eq!(
                    runtime
                        .network_handle()
                        .maybe_basic_index_progress()
                        .expect("configured owner read")
                        .is_some(),
                    mode == BasicFilterStartupMode::Enabled
                );
            }
        }
        Self {
            path,
            runtime,
            recipe,
            receipts,
        }
    }

    pub fn finish(&self) -> u32 {
        for turns in 1..=256 {
            let turn = self
                .runtime
                .network_handle()
                .drive_basic_filter_index_turn()
                .expect("ordinary bounded probe catch-up turn");
            if turn.maybe_accepted_lag == Some(0) {
                assert_eq!(
                    turn.maybe_progress
                        .expect("real enabled owner")
                        .current_lag(),
                    0
                );
                return turns;
            }
        }
        panic!("bounded normal-maturity probe catch-up exhausted");
    }

    pub fn flush(&self) {
        let receipt = self
            .runtime
            .network_handle()
            .flush_coins(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(500_000),
                u64::MAX,
            )
            .expect("actual coins/undo/metadata flush");
        assert!(receipt.wrote_coins);
        self.runtime
            .network_handle()
            .drive_basic_filter_index_turn()
            .expect("durable fence promotion");
    }

    pub fn cleanup(self) {
        let Self { path, runtime, .. } = self;
        drop(runtime);
        // A successful second owner confirms no original database handle remains.
        drop(FjallNodeStore::open(&path).expect("all-handle-close reopen"));
        fs::remove_dir_all(&path).expect("actual datadir cleanup");
        assert!(!path.exists());
    }
}

/// Hermetic raw evidence only: scripts and commitments cannot enter this boundary.
pub(in crate::sync::tests) fn oracle_input(
    recipe: &recipe::Recipe,
    maybe_branch: Option<(u32, &[Block])>,
) -> serde_json::Value {
    let raw = |name: String, previous: String, block: &Block| {
        let bytes = core_types::codec::encode_block(block).expect("raw protocol block");
        serde_json::json!({ "name": name, "previous_name": previous,
            "raw_block": bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>() })
    };
    let mut blocks: Vec<_> = recipe
        .blocks
        .iter()
        .enumerate()
        .map(|(height, block)| {
            raw(
                format!("active-{height}"),
                if height == 0 {
                    String::new()
                } else {
                    format!("active-{}", height - 1)
                },
                block,
            )
        })
        .collect();
    if let Some((ancestor, branch)) = maybe_branch {
        for (offset, block) in branch.iter().enumerate() {
            blocks.push(raw(
                format!("fork-{}", ancestor as usize + offset + 1),
                if offset == 0 {
                    format!("active-{ancestor}")
                } else {
                    format!("fork-{}", ancestor as usize + offset)
                },
                block,
            ));
        }
    }
    serde_json::json!({ "recipe_id": recipe::RECIPE_ID, "network_magic": "fabfb5da", "blocks": blocks })
}

pub(in crate::sync::tests) fn independent_oracle(input: &serde_json::Value) -> serde_json::Value {
    use std::process::Command;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = process::run(
        Command::new("bun")
            .arg("scripts/phase162-filter-oracle.ts")
            .current_dir(root),
        serde_json::to_vec(input).expect("raw recipe JSON"),
        process::ORACLE_LIMITS,
    )
    .expect("bounded outer raw-oracle process");
    serde_json::from_slice(&output.stdout).expect("independent raw evidence JSON")
}

#[test]
fn phase162_recipe_largest_active_prefix_and_extending_fork_reach_oracle() {
    // Arrange: recipe-only boundary proof; no additional Fjall history.
    let recipe = recipe::Recipe::new(
        SyncNetwork::Regtest.consensus_params(),
        ScriptVerifyFlags::P2SH,
        1100,
    );
    let branch = recipe.fork(1098, 2, 0x62);
    // Act
    let oracle = independent_oracle(&oracle_input(&recipe, Some((1098, &branch))));
    // Assert
    assert_eq!(oracle["cases"].as_array().expect("cases").len(), 1102);
    assert_eq!(oracle["cases"][1101]["name"], "fork-1100");
    let spent: Vec<Vec<String>> = recipe
        .fork_spent_scripts(1098, &branch)
        .iter()
        .map(|scripts| {
            scripts
                .iter()
                .map(|script| script.iter().map(|byte| format!("{byte:02x}")).collect())
                .collect()
        })
        .collect();
    for (offset, scripts) in spent.iter().enumerate() {
        assert_eq!(
            oracle["cases"][1100 + offset]["spent"],
            serde_json::json!(scripts)
        );
    }
}

fn footprint(path: &Path) -> u64 {
    use std::os::unix::fs::MetadataExt;
    fs::read_dir(path)
        .expect("footprint directory")
        .map(|entry| {
            let entry = entry.expect("footprint entry");
            let metadata = entry.metadata().expect("footprint metadata");
            if metadata.is_dir() {
                footprint(&entry.path())
            } else {
                metadata.blocks() * 512
            }
        })
        .sum()
}

#[test]
fn phase162_recipe_cost_probe_real_receipts_undo_coins_and_cleanup() {
    // Arrange: this single pressure-free database runs before fixture multiplication.
    let started = std::time::Instant::now();
    let fixture = AcceptedRecipe::new(1001);
    // Act
    let turns = fixture.finish();
    fixture.flush();
    let store = fixture.runtime.store();
    let usage = store
        .retained_payload_usage(&fixture.receipts)
        .expect("measured logical payload");
    let physical_bytes = footprint(&fixture.path);
    let branch = fixture.recipe.fork(998, 4, 0x62);
    let oracle = independent_oracle(&oracle_input(&fixture.recipe, Some((998, &branch))));
    let oracle_cases = oracle["cases"].as_array().expect("oracle cases");
    let hex = |bytes: &[u8]| {
        bytes
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    };
    for (height, position) in fixture.receipts.iter().enumerate() {
        let record = store
            .load_basic_filter_record(position.block_hash)
            .expect("actual immutable record read")
            .expect("catch-up persisted record");
        assert_eq!(oracle_cases[height]["encoded"], hex(record.encoded_bytes()));
        assert_eq!(
            oracle_cases[height]["hash_raw"],
            hex(record.identity().filter_hash().as_bytes())
        );
        assert_eq!(
            oracle_cases[height]["header_raw"],
            hex(record.identity().filter_header().as_bytes())
        );
    }
    // Assert: source-derived outpoints match genuinely persisted validation undo.
    for (height, position) in fixture.receipts.iter().enumerate().skip(1) {
        let undo = store
            .load_undo(position.block_hash)
            .expect("actual persisted undo")
            .expect("retained undo");
        let inputs = BasicFilterInputs::from_historical(
            &fixture.recipe.blocks[height],
            position,
            Some(HistoricalBlockUndo {
                block_hash: position.block_hash,
                undo: &undo,
            }),
        )
        .expect("actual undo and same-block resolution");
        assert_eq!(
            inputs
                .spent_scripts()
                .map(<[u8]>::to_vec)
                .collect::<Vec<_>>(),
            fixture.recipe.spent_scripts[height]
        );
        let expected_hex: Vec<_> = inputs.spent_scripts().map(hex).collect();
        assert_eq!(
            oracle_cases[height]["spent"],
            serde_json::json!(expected_hex)
        );
    }
    let tip = fixture.receipts.last().expect("accepted tip").block_hash;
    assert_eq!(
        store.coins_view().best_block().expect("durable coins B"),
        Some(tip)
    );
    assert_eq!(
        store
            .load_chain_meta_for_open()
            .expect("metadata")
            .0
            .last()
            .expect("tip")
            .block_hash,
        tip
    );
    let coin = store
        .coins_view()
        .get_coin(&fixture.recipe.wallet_outpoint)
        .expect("wallet coin read")
        .expect("unspent descriptor funding");
    assert_eq!(coin.created_height, 20);
    assert_eq!(coin.output.script_pubkey, fixture.recipe.wallet_script);
    assert_eq!(coin.output.value.to_sats(), recipe::WALLET_SATS);
    let progress = fixture
        .runtime
        .network_handle()
        .maybe_basic_index_progress()
        .expect("actual index owner")
        .expect("enabled index");
    assert_eq!(
        progress
            .maybe_processed_endpoint()
            .expect("processed endpoint")
            .height(),
        1000
    );
    assert_eq!(
        progress
            .maybe_safe_durable_endpoint()
            .expect("coins-fenced durable endpoint")
            .height(),
        1000
    );
    let final_record = store
        .load_basic_filter_record(tip)
        .expect("final record read")
        .expect("actual endpoint");
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("persisted checkpoint"),
        Some(checkpoint(&final_record))
    );
    assert!(physical_bytes > 0 && usage.current_usage_bytes > 0);
    let elapsed_ms = started.elapsed().as_millis();
    fixture.cleanup();
    eprintln!(
        "phase162 cost probe: recipe={} blocks=1001 catch_up_turns={turns} elapsed_ms={elapsed_ms} logical_body_undo_bytes={} physical_allocated_bytes={physical_bytes} cleanup=true",
        recipe::RECIPE_ID,
        usage.current_usage_bytes
    );
}

#[test]
fn phase162_recipe_continuous_normal_maturity_and_deterministic_fork() {
    // Arrange
    let params = SyncNetwork::Regtest.consensus_params();
    let flags = ScriptVerifyFlags::P2SH;
    // Act
    let recipe = recipe::Recipe::new(params, flags, 1001);
    let branch = recipe.fork(998, 4, 0x62);
    // Assert
    assert_eq!(recipe.blocks.len(), 1001);
    assert!(
        recipe
            .blocks
            .windows(2)
            .all(|pair| pair[1].header.previous_block_hash == block_hash(&pair[0].header))
    );
    assert!(
        recipe.blocks[..101]
            .iter()
            .all(|block| block.transactions.len() == 1)
    );
    assert_eq!(branch, recipe.fork(998, 4, 0x62));
    assert_ne!(
        block_hash(&branch[1].header),
        block_hash(&recipe.blocks[1000].header)
    );
    assert_eq!(
        branch[0].header.previous_block_hash,
        block_hash(&recipe.blocks[998].header)
    );
    assert_eq!(
        recipe.blocks[101].transactions[1].inputs[0]
            .previous_output
            .txid,
        core_types::consensus::transaction_txid(&recipe.blocks[1].transactions[0])
            .expect("funding txid")
    );
    assert_eq!(recipe.wallet_outpoint.vout, 1);
    assert_eq!(recipe.fork_spent_scripts(998, &branch).len(), 4);
}
