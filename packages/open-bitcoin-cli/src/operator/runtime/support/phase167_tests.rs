// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp

use super::*;
use crate::operator::{OperatorCli, runtime::execute_operator_cli_with_default_data_dir};
use clap::Parser;
use open_bitcoin_node::{
    DurableSyncRuntime, FjallChainstateStore, FjallCoinsView, FjallNodeStore, PersistMode,
    RuntimeMetadata, SyncNetwork, SyncRuntimeConfig,
    chainstate::BasicFilterStartupMode,
    core::{
        codec::encode_block,
        consensus::{CanonicalChain, block_hash, block_merkle_root, check_block_header},
        primitives::{
            Amount, Block, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
            TransactionInput, TransactionOutput,
        },
        wallet::AddressNetwork,
    },
};
use open_bitcoin_rpc::{
    config::{BasicFilterIndexSetting, RuntimeConfig},
    context::ManagedRpcContext,
    dispatch::dispatch,
    method::{RequestParameters, normalize_method_call},
};
use serde_json::json;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

type Context = ManagedRpcContext<FjallChainstateStore, FjallCoinsView>;
static SCENARIO: Mutex<()> = Mutex::new(());
static NEXT_SOURCE: AtomicU64 = AtomicU64::new(0);

struct Source {
    root: PathBuf,
    data: PathBuf,
    config: PathBuf,
}

impl Source {
    fn new() -> Self {
        let root = std::env::temp_dir()
            .canonicalize()
            .expect("physical temp parent")
            .join(format!(
                "phase167-cli-history-{}-{}",
                std::process::id(),
                NEXT_SOURCE.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir(&root).expect("owned test root");
        let config = root.join("open-bitcoin.jsonc");
        fs::write(
            &config,
            json!({"onboarding":{"wizard_answers":{"network":"regtest"}}}).to_string(),
        )
        .expect("physical selected config");
        Self {
            data: root.join("data"),
            root,
            config,
        }
    }

    fn runtime_config(&self) -> RuntimeConfig {
        let mut config = RuntimeConfig {
            chain: AddressNetwork::Regtest,
            maybe_data_dir: Some(self.data.clone()),
            block_filter_index: BasicFilterIndexSetting::Basic,
            ..Default::default()
        };
        config.sync.runtime.network = SyncNetwork::Regtest;
        config
    }

    fn cli_status(&self) -> OperatorCommandOutcome {
        let physical_data = self.data.canonicalize().expect("same physical owner path");
        self.cli_status_for(&physical_data, "regtest")
    }

    fn cli_status_for(&self, data_dir: &Path, network: &str) -> OperatorCommandOutcome {
        let cli = OperatorCli::try_parse_from([
            "open-bitcoin",
            "--config",
            self.config.to_str().expect("owned config"),
            "--datadir",
            data_dir.to_str().expect("owned data"),
            "--network",
            network,
            "--format",
            "json",
            "sync",
            "status",
        ])
        .expect("actual operator parser");
        execute_operator_cli_with_default_data_dir(cli, self.root.join("unused"))
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        if self.root.exists() {
            fs::remove_dir_all(&self.root).expect("all test owners dropped");
        }
    }
}

fn source_manifest(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, here: &Path, rows: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(here).expect("owned source listing") {
            let entry = entry.expect("physical entry");
            let path = entry.path();
            let kind = entry.file_type().expect("physical kind");
            assert!(!kind.is_symlink(), "owned fixture has no symlinks");
            if kind.is_dir() {
                visit(root, &path, rows);
            } else {
                rows.insert(
                    path.strip_prefix(root)
                        .expect("owned relative path")
                        .to_path_buf(),
                    fs::read(&path).expect("whole owned file"),
                );
            }
        }
    }
    let mut rows = BTreeMap::new();
    visit(root, root, &mut rows);
    rows
}

fn assert_cli_compatibility_refusal(outcome: OperatorCommandOutcome) {
    assert_ne!(outcome.exit_code.code(), 0);
    let diagnostic: serde_json::Value =
        serde_json::from_str(&outcome.stderr.text).expect("structured CLI refusal");
    assert_eq!(diagnostic["error"], "chain_compatibility_refused");
    assert!(diagnostic["diagnostic"].is_string());
}

#[test]
fn phase167_cli_status_observation_error_is_network_neutral() {
    // Arrange
    let error = OperatorRuntimeError::CanonicalObservationUnavailable;
    // Act
    let rendered = error.to_string();
    // Assert
    assert_eq!(rendered, "canonical status observation unavailable");
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn child(parent: &BlockHeader, height: u32) -> Block {
    let mut number = Vec::new();
    let mut value = height;
    while value > 0 {
        number.push(value as u8);
        value >>= 8;
    }
    if number.last().is_some_and(|byte| byte & 0x80 != 0) {
        number.push(0);
    }
    let mut script = if height <= 16 {
        vec![0x50 + height as u8]
    } else {
        let mut encoded = vec![number.len() as u8];
        encoded.extend(number);
        encoded
    };
    script.push(0);
    let transaction = Transaction {
        version: 1,
        lock_time: height,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(script).expect("height"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000 >> (height / 150)).expect("pinned subsidy"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("fixture true"),
        }],
    };
    let mut block = Block {
        header: BlockHeader {
            version: 4,
            previous_block_hash: block_hash(parent),
            merkle_root: Default::default(),
            time: parent.time + 1,
            bits: parent.bits,
            nonce: 0,
        },
        transactions: vec![transaction],
    };
    block.header.merkle_root = block_merkle_root(&block.transactions)
        .expect("fixture merkle")
        .0;
    block.header.nonce = (0..100_000)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("bounded real regtest work");
    block
}

fn submit(context: &mut Context, block: &Block) {
    let data = hex(&encode_block(block).expect("exact serialized block"));
    let call = normalize_method_call(
        "submitblock",
        RequestParameters::Positional(vec![json!(data)]),
    )
    .expect("actual method registry");
    assert_eq!(
        dispatch(context, call).expect("earned admission"),
        json!(null)
    );
}

#[test]
fn phase167_cli_history_301_descendants_reopen_through_actual_parser() {
    let _scenario = SCENARIO.lock().expect("serial durable scenario");
    // Arrange: the same sealed producer used by RPC accepts actual default-Regtest blocks.
    let source = Source::new();
    let (mut context, runtime) =
        Context::from_canonical_history_runtime_config(&source.runtime_config())
            .expect("paired canonical producer");
    let mut parent = CanonicalChain::Regtest.spec().genesis_header();
    for height in 1..=301 {
        let block = child(&parent, height);
        submit(&mut context, &block);
        parent = block.header;
    }
    let expected_hash = hex(block_hash(&parent).as_bytes());
    let canonical_data_dir = source.data.canonicalize().expect("physical earned store");
    let rpc = dispatch(
        &mut context,
        normalize_method_call("getblockchaininfo", RequestParameters::None)
            .expect("actual blockchain-info registry"),
    )
    .expect("same paired RPC owner status");
    let accepted_tip = runtime
        .network_handle()
        .maybe_chain_tip()
        .expect("same paired durable tip read")
        .expect("accepted descendant tip");
    assert_eq!(accepted_tip.height, 301);
    assert_eq!(hex(accepted_tip.block_hash.as_bytes()), expected_hash);
    assert_eq!(rpc["blocks"], 301);
    let mut display = accepted_tip.block_hash.to_byte_array();
    display.reverse();
    assert_eq!(rpc["bestblockhash"], hex(&display));
    let paired_work = accepted_tip.chain_work.to_hex();
    assert!(
        runtime
            .store()
            .load_block(accepted_tip.block_hash)
            .expect("retained accepted body")
            .is_some(),
        "the paired owner must retain the accepted tip body"
    );
    drop(context);
    drop(runtime);
    // Act: use the actual CLI parser/config route after every writer handle drops.
    let outcome = source.cli_status();
    // Assert: these are runtime-derived accepted fields, not static selector claims.
    assert_eq!(outcome.exit_code.code(), 0, "{}", outcome.stderr.text);
    assert!(outcome.stderr.text.is_empty());
    let output: serde_json::Value =
        serde_json::from_str(&outcome.stdout.text).expect("status JSON");
    let progress = &output["maybe_sync_state"]["sync"]["sync_progress"]["value"];
    assert_eq!(
        source.data.canonicalize().expect("same closed store"),
        canonical_data_dir
    );
    assert_eq!(output["selected_network"], "regtest");
    assert_eq!(progress["validated_active_chain_height"], rpc["blocks"]);
    assert_eq!(progress["maybe_validated_active_chain_hash"], expected_hash);
    let raw_hash = progress["maybe_validated_active_chain_hash"]
        .as_str()
        .expect("raw accepted hash");
    assert_eq!(raw_hash.len(), 64);
    let display_from_cli = (0..32)
        .rev()
        .map(|index| &raw_hash[index * 2..index * 2 + 2])
        .collect::<String>();
    assert_eq!(rpc["bestblockhash"], display_from_cli);
    assert_eq!(progress["maybe_validated_active_chain_work"], paired_work);
    assert_eq!(
        progress["maybe_validated_active_chain_work"],
        format!("{:064x}", 604u64)
    );
    let basic = &output["basic_index"];
    for endpoint in ["accepted_height", "processed_height", "safe_durable_height"] {
        assert_eq!(basic[endpoint]["value"], 301, "BASIC {endpoint}");
    }
    assert_eq!(basic["retained_availability"]["value"], "observed");
    assert!(
        basic["retention"]["value"]["retained_record_count"]
            .as_u64()
            .is_some_and(|count| count >= 302),
        "genesis and every accepted descendant must have retained BASIC evidence"
    );
    assert_eq!(basic["protection"]["state"], "available");
    assert_eq!(
        output["support"]["local_ordinary_implementation"],
        "bounded_default_regtest"
    );
    assert_eq!(
        output["support"]["observed_evidence_class"],
        "complete_retained_body_reopen"
    );
    assert_eq!(output["support"]["retained_body_reopen_bound"], true);
    assert!(
        output["support"]["generic_ordinary_gate_missing"]
            .as_array()
            .is_some_and(|items| !items.is_empty())
    );
    assert_eq!(
        output["support"]["actual_knots_daemon_differential_qualified"],
        false
    );
}

#[test]
fn phase167_cli_history_one_descendant_reports_retained_basic_evidence() {
    let _scenario = SCENARIO.lock().expect("serial durable scenario");
    // Arrange
    let source = Source::new();
    let (mut context, runtime) =
        Context::from_canonical_history_runtime_config(&source.runtime_config())
            .expect("paired canonical producer");
    submit(
        &mut context,
        &child(&CanonicalChain::Regtest.spec().genesis_header(), 1),
    );
    drop(context);
    drop(runtime);
    // Act
    let outcome = source.cli_status();
    // Assert
    assert_eq!(outcome.exit_code.code(), 0, "{}", outcome.stderr.text);
    let output: serde_json::Value =
        serde_json::from_str(&outcome.stdout.text).expect("status JSON");
    let basic = &output["basic_index"];
    for endpoint in ["accepted_height", "processed_height", "safe_durable_height"] {
        assert_eq!(basic[endpoint]["value"], 1, "BASIC {endpoint}");
    }
    assert_eq!(basic["retained_availability"]["value"], "observed");
    assert!(
        basic["retention"]["value"]["retained_record_count"]
            .as_u64()
            .is_some_and(|count| count >= 2)
    );
    assert_eq!(basic["protection"]["state"], "available");
    assert_eq!(
        output["support"]["observed_evidence_class"],
        "complete_retained_body_reopen"
    );
}

#[test]
fn phase167_cli_history_missing_source_does_not_initialize() {
    let _scenario = SCENARIO.lock().expect("serial durable scenario");
    // Arrange
    let missing = Source::new();
    let missing_before = source_manifest(&missing.root);
    // Act / Assert: missing status cannot create a backend.
    assert!(matches!(
        execute_offline_sync_command(
            &missing.data,
            CanonicalChain::Regtest,
            &SyncArgs {
                command: SyncCommand::Status
            },
            OperatorOutputFormat::Json,
        ),
        Err(OperatorRuntimeError::CompatibilityRefused { .. })
    ));
    assert_eq!(source_manifest(&missing.root), missing_before);
    assert_cli_compatibility_refusal(missing.cli_status_for(&missing.data, "regtest"));
    assert_eq!(source_manifest(&missing.root), missing_before);
}

#[test]
fn phase167_cli_history_existing_unproved_store_refuses_before_effects() {
    let _scenario = SCENARIO.lock().expect("serial durable scenario");
    // Arrange: raw metadata cannot substitute for a complete history proof.
    let source = Source::new();
    let store = FjallNodeStore::open(&source.data).expect("unproved owned store");
    store
        .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
        .expect("unproved metadata sentinel");
    drop(store);
    let before = source_manifest(&source.root);
    // Act
    let result = execute_offline_sync_command(
        &source.data,
        CanonicalChain::Regtest,
        &SyncArgs {
            command: SyncCommand::Status,
        },
        OperatorOutputFormat::Json,
    );
    // Assert
    assert!(matches!(
        result,
        Err(OperatorRuntimeError::CompatibilityRefused { .. })
    ));
    assert_eq!(source_manifest(&source.root), before);
    assert_cli_compatibility_refusal(source.cli_status());
    assert_eq!(source_manifest(&source.root), before);
}

#[test]
fn phase167_cli_history_wrong_selector_preserves_source() {
    let _scenario = SCENARIO.lock().expect("serial durable scenario");
    // Arrange
    let source = Source::new();
    let (mut context, runtime) =
        Context::from_canonical_history_runtime_config(&source.runtime_config())
            .expect("paired canonical producer");
    submit(
        &mut context,
        &child(&CanonicalChain::Regtest.spec().genesis_header(), 1),
    );
    drop(context);
    drop(runtime);
    let before = source_manifest(&source.root);
    // Act / Assert: no other selected identity can inherit this Regtest proof.
    let physical_data = source
        .data
        .canonicalize()
        .expect("same physical owner path");
    for (chain, network) in [
        (CanonicalChain::Mainnet, "mainnet"),
        (CanonicalChain::Testnet3, "testnet"),
        (CanonicalChain::Signet, "signet"),
    ] {
        assert!(matches!(
            execute_offline_sync_command(
                &source.data,
                chain,
                &SyncArgs {
                    command: SyncCommand::Status,
                },
                OperatorOutputFormat::Json,
            ),
            Err(OperatorRuntimeError::CompatibilityRefused { .. })
        ));
        assert_eq!(source_manifest(&source.root), before);
        assert_cli_compatibility_refusal(source.cli_status_for(&physical_data, network));
        assert_eq!(source_manifest(&source.root), before);
    }
}

#[test]
fn phase167_cli_genesis_four_selectors_retain_existing_status_route() {
    let _scenario = SCENARIO.lock().expect("serial durable scenario");
    for (chain, network) in [
        (CanonicalChain::Mainnet, SyncNetwork::Mainnet),
        (CanonicalChain::Testnet3, SyncNetwork::Testnet),
        (CanonicalChain::Signet, SyncNetwork::Signet),
        (CanonicalChain::Regtest, SyncNetwork::Regtest),
    ] {
        // Arrange
        let source = Source::new();
        let opened = FjallNodeStore::open_canonical_genesis(&source.data, chain)
            .expect("proved genesis fixture");
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            SyncRuntimeConfig {
                network,
                ..Default::default()
            },
            BasicFilterStartupMode::Enabled,
        )
        .expect("configured genesis fixture");
        drop(runtime);
        // Act
        let result = execute_offline_sync_command(
            &source.data,
            chain,
            &SyncArgs {
                command: SyncCommand::Status,
            },
            OperatorOutputFormat::Json,
        )
        .expect("existing genesis status");
        // Assert
        let value: serde_json::Value =
            serde_json::from_str(&result.stdout.text).expect("status JSON");
        assert_eq!(value["selected_network"], chain.to_string());
        assert_eq!(
            value["support"]["observed_evidence_class"],
            "proved_genesis"
        );
        assert_eq!(
            value["maybe_sync_state"]["sync"]["sync_progress"]["value"]["validated_active_chain_height"],
            0
        );
        assert_eq!(
            value["support"]["local_ordinary_implementation"],
            if chain == CanonicalChain::Regtest {
                "bounded_default_regtest"
            } else {
                "unavailable"
            }
        );
        assert_eq!(
            value["support"]["actual_knots_daemon_differential_qualified"],
            false
        );
    }
}

#[test]
fn phase167_cli_history_refuses_second_owner_while_runtime_is_live() {
    let _scenario = SCENARIO.lock().expect("serial durable scenario");
    // Arrange
    let source = Source::new();
    let (mut context, runtime): (Context, DurableSyncRuntime) =
        Context::from_canonical_history_runtime_config(&source.runtime_config())
            .expect("paired canonical producer");
    submit(
        &mut context,
        &child(&CanonicalChain::Regtest.spec().genesis_header(), 1),
    );
    let before = source_manifest(&source.root);
    // Act
    let result = execute_offline_sync_command(
        &source.data,
        CanonicalChain::Regtest,
        &SyncArgs {
            command: SyncCommand::Status,
        },
        OperatorOutputFormat::Json,
    );
    // Assert
    assert!(matches!(
        result,
        Err(OperatorRuntimeError::CompatibilityRefused { .. })
    ));
    assert_eq!(source_manifest(&source.root), before);
    assert_cli_compatibility_refusal(source.cli_status());
    assert_eq!(source_manifest(&source.root), before);
    drop(context);
    drop(runtime);
}
