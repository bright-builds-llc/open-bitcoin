// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;
use crate::storage::fjall_store::filters::FilterPublicationFault as Fault;
use crate::storage::validation_history::{COVERAGE_KEY, ValidationCoverage, encode_coverage};
use crate::{PersistMode, StorageNamespace};
use open_bitcoin_core::consensus::chain_proof::ProofWork;
use serde_json::{Value, json};
use std::fs::{self, File};
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

const CHILD_CASE: &str = "OPEN_BITCOIN_CREC04_CHILD";
const CHILD_DATADIR: &str = "OPEN_BITCOIN_CREC04_DATADIR";
const REPORT: &str = "OPEN_BITCOIN_CREC04_REPORT";
const CLI: &str = "OPEN_BITCOIN_CREC04_CLI";
const INTERRUPTED: i32 = 75;
const CASES: [(&str, Option<Fault>, Option<usize>); 5] = [
    ("before-body", Some(Fault::BeforeBody), Some(1)),
    ("before-chain-meta", Some(Fault::BeforeChainMeta), None),
    ("after-basic-commit", Some(Fault::AfterCommit), Some(2)),
    ("inconsistent-coins-tip", None, None),
    ("inconsistent-header-work", None, None),
];

fn hex(bytes: impl IntoIterator<Item = u8>) -> String {
    bytes
        .into_iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn corpus_blocks() -> [Block; 2] {
    let input: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../testdata/phase167/regtest-history.json"
    )))
    .expect("canonical Regtest corpus");
    [1, 2].map(|height| {
        let input_hex = input["rows"][height]["block_hex"]
            .as_str()
            .expect("wire block");
        let bytes = (0..input_hex.len())
            .step_by(2)
            .map(|offset| {
                u8::from_str_radix(&input_hex[offset..offset + 2], 16).expect("fixture hex")
            })
            .collect::<Vec<_>>();
        open_bitcoin_core::codec::block::wire::parse_serialized_block(&bytes)
            .expect("complete canonical block")
    })
}

#[test]
#[ignore = "spawned in a disposable process by the CREC-04 recovery matrix"]
fn crec04_fixture_child() {
    let name = std::env::var(CHILD_CASE).expect("owned fixture case");
    let path =
        std::path::PathBuf::from(std::env::var_os(CHILD_DATADIR).expect("owned fixture path"));
    assert!(
        path.is_absolute() && !path.exists(),
        "fresh owned fixture path"
    );
    let (_, fault, _) = CASES
        .iter()
        .find(|case| case.0 == name)
        .expect("bounded case");
    let blocks = corpus_blocks();
    let (runtime, command) = DurableSyncRuntime::open_canonical_submission_configured(
        &path,
        config(),
        BasicFilterStartupMode::Enabled,
    )
    .expect("ordinary fresh canonical submission");
    let command = command.expect("Regtest command");
    accepted(
        command
            .submit(wire(&blocks[0]))
            .expect("earned first checkpoint"),
    );
    if let Some(fault) = fault {
        command.store.set_basic_filter_fault(*fault);
        let error = command
            .submit(wire(&blocks[1]))
            .expect_err("injected failure before RPC success");
        assert!(
            error
                .to_string()
                .contains("injected BASIC publication failure"),
            "{error}"
        );
        // End the fixture process with every runtime handle still in scope. The
        // ordinary reopen must use committed bytes after OS close, without Drop.
        std::process::exit(INTERRUPTED);
    }
    accepted(
        command
            .submit(wire(&blocks[1]))
            .expect("earned second checkpoint"),
    );
    drop(command);
    drop(runtime);

    let store = FjallNodeStore::open(&path).expect("exclusive disposable mutation owner");
    let before = native_rows(&store);
    match name.as_str() {
        "inconsistent-coins-tip" => store
            .write_raw_for_test(
                StorageNamespace::Coins,
                "B",
                block_hash(&blocks[0].header).to_byte_array().to_vec(),
            )
            .expect("replace only the coins best-block hash"),
        "inconsistent-header-work" => {
            let mut entries = store
                .load_header_entries()
                .expect("paired headers")
                .expect("present")
                .entries;
            let tip = entries
                .iter_mut()
                .find(|entry| entry.height == 2)
                .expect("known tip header");
            tip.chain_work = ProofWork::ZERO;
            store
                .save_header_entries(&entries, PersistMode::Sync)
                .expect("replace paired typed work");
            store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    COVERAGE_KEY,
                    encode_coverage(ValidationCoverage::Complete).to_vec(),
                )
                .expect("preserve original coverage so work is the inconsistent input");
        }
        _ => panic!("unsupported closed-record case"),
    }
    let after = native_rows(&store);
    let changed: Vec<_> = before
        .iter()
        .filter_map(|(namespace, rows)| (*rows != after[namespace]).then_some(namespace.as_str()))
        .collect();
    assert_eq!(
        changed,
        if name == "inconsistent-coins-tip" {
            vec!["coins"]
        } else {
            vec!["block_index", "headers"]
        }
    );
}

fn wait_child(command: &mut Command, log: &Path) -> ExitStatus {
    let stdout = File::create(log).expect("owned test log");
    let stderr = stdout.try_clone().expect("owned test log handle");
    let mut child = command
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .expect("test child");
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Some(status) = child.try_wait().expect("child outcome") {
            return status;
        }
        if Instant::now() >= deadline {
            child.kill().expect("kill owned child");
            child.wait().expect("reap owned child");
            panic!("recovery child timed out, see {}", log.display());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn status_cli(cli: &Path, root: &Path, datadir: &Path, expected: Option<&Value>) -> Value {
    let config = root.join("operator.jsonc");
    fs::write(
        &config,
        r#"{"onboarding":{"wizard_answers":{"network":"regtest"}}}"#,
    )
    .expect("disposable Regtest configuration");
    let log = root.join("sync-status.log");
    let status = wait_child(
        Command::new(cli)
            .arg("--config")
            .arg(config)
            .arg("--datadir")
            .arg(datadir)
            .args(["--network", "regtest", "--format", "json", "sync", "status"]),
        &log,
    );
    let code = status.code().expect("operator exit status");
    let output: Value = serde_json::from_slice(&fs::read(&log).expect("operator output"))
        .unwrap_or_else(|error| panic!("invalid operator response {}: {error}", log.display()));
    if let Some(expected) = expected {
        assert_eq!(code, 0);
        assert_eq!(output["selected_network"], "regtest");
        assert_eq!(
            output["support"]["local_ordinary_implementation"],
            "bounded_default_regtest"
        );
        let progress = &output["maybe_sync_state"]["sync"]["sync_progress"]["value"];
        let raw = progress["maybe_validated_active_chain_hash"]
            .as_str()
            .expect("raw proved tip");
        assert_eq!(raw.len(), 64, "complete CLI hash");
        let display = (0..32)
            .rev()
            .map(|byte| &raw[byte * 2..byte * 2 + 2])
            .collect::<String>();
        assert_eq!(json!(display), expected["tip"]["hash"]);
        assert_eq!(
            progress["validated_active_chain_height"],
            expected["tip"]["height"]
        );
        assert_eq!(
            progress["maybe_validated_active_chain_work"],
            expected["chainwork"]
        );
        assert_eq!(
            output["basic_index"]["safe_durable_height"]["value"],
            expected["tip"]["height"]
        );
        return json!({ "exit_status": code, "tip": expected["tip"], "chainwork": expected["chainwork"] });
    }
    assert_ne!(code, 0);
    assert_eq!(output["error"], "chain_compatibility_refused");
    json!({ "exit_status": code, "error": output["error"] })
}

fn reopened_state(command: &CanonicalSubmissionCommand, height: usize) -> Value {
    let state = command
        .observe_accepted_state(&[])
        .expect("actual recovered authority");
    state.fold(|(snapshot, _, coins), progress, _, _, _| {
        let tip = snapshot.tip().expect("recovered tip");
        let expected = corpus_blocks();
        assert_eq!(tip.height as usize, height);
        assert_eq!(tip.block_hash, block_hash(&expected[height - 1].header));
        assert_eq!(coins.maybe_best_block, Some(tip.block_hash));
        assert!(coins.head_blocks.is_empty());
        let mut actual: Vec<_> = snapshot.utxos.iter().map(|(outpoint, coin)| (
            hex(outpoint.txid.to_byte_array().into_iter().rev()), outpoint.vout,
            coin.output.value.to_sats().to_string(), hex(coin.output.script_pubkey.as_bytes().iter().copied()),
            coin.is_coinbase, coin.created_height,
        )).collect();
        let mut known = Vec::new();
        for (index, block) in expected[..height].iter().enumerate() {
            let [tx] = block.transactions.as_slice() else { panic!("coinbase-only prefix") };
            let txid = open_bitcoin_core::consensus::transaction_txid(tx).expect("fixture txid");
            for (vout, output) in tx.outputs.iter().enumerate() {
                known.push((hex(txid.to_byte_array().into_iter().rev()), u32::try_from(vout).expect("vout"),
                    output.value.to_sats().to_string(), hex(output.script_pubkey.as_bytes().iter().copied()),
                    true, u32::try_from(index + 1).expect("height")));
            }
        }
        actual.sort_unstable();
        known.sort_unstable();
        assert_eq!(actual, known);
        assert_eq!(progress.expect("BASIC progress").maybe_safe_durable_endpoint().expect("safe endpoint").block_hash(), tip.block_hash);
        let digest = open_bitcoin_core::consensus::crypto::Sha256::digest(&serde_json::to_vec(&actual).expect("normalized coins"));
        json!({ "tip": { "height": tip.height, "hash": hex(tip.block_hash.to_byte_array().into_iter().rev()) },
            "chainwork": tip.chain_work.to_hex(), "coins_sha256": hex(digest), "coins_count": actual.len(),
            "basic_height": tip.height })
    })
}

#[test]
fn crec04_reopens_proved_prefix_or_preserves_refused_source() {
    let root = directory();
    fs::create_dir(&root).expect("owned matrix root");
    let maybe_cli = std::env::var_os(CLI).map(std::path::PathBuf::from);
    let maybe_report = std::env::var_os(REPORT);
    assert!(
        maybe_report.is_none() || maybe_cli.is_some(),
        "receipt runs require the ordinary CLI"
    );
    let (_, module) = module_path!()
        .split_once("::")
        .expect("crate-qualified test module");
    let child_test = format!("{module}::crec04_fixture_child");
    let mut cases = Vec::new();
    for (name, fault, maybe_height) in CASES {
        let case = root.join(name);
        fs::create_dir(&case).expect("owned case root");
        let datadir = case.join("data");
        let code = wait_child(
            Command::new(std::env::current_exe().expect("test executable"))
                .args(["--ignored", "--exact", child_test.as_str(), "--nocapture"])
                .env(CHILD_CASE, name)
                .env(CHILD_DATADIR, &datadir),
            &case.join("fixture.log"),
        )
        .code()
        .expect("fixture exit status");
        assert_eq!(
            code,
            if fault.is_some() { INTERRUPTED } else { 0 },
            "fixture log in {}",
            case.display()
        );
        let before = manifest(&datadir);
        assert!(!before.is_empty());
        let opened = DurableSyncRuntime::open_canonical_submission_configured(
            &datadir,
            config(),
            BasicFilterStartupMode::PreserveSaved,
        );
        let mut result = match (maybe_height, opened) {
            (Some(height), Ok((runtime, Some(command)))) => {
                let state = reopened_state(&command, height);
                drop(command);
                drop(runtime);
                json!({ "outcome": "proved_prefix", "state": state })
            }
            (None, Err(error)) => {
                assert!(
                    error.to_string().contains("canonical"),
                    "explicit canonical refusal: {error}"
                );
                assert_eq!(
                    manifest(&datadir),
                    before,
                    "managed refusal must preserve source"
                );
                json!({ "outcome": "refused", "source_unchanged": true })
            }
            (expected, observed) => panic!(
                "unexpected {name} outcome, expected {expected:?}, refusal {:?}",
                observed.err()
            ),
        };
        if let Some(cli) = &maybe_cli {
            result["operator"] = status_cli(cli, &case, &datadir, result.get("state"));
            if maybe_height.is_none() {
                assert_eq!(
                    manifest(&datadir),
                    before,
                    "operator refusal must preserve source"
                );
            }
        }
        result["name"] = json!(name);
        result["fixture_exit_status"] = json!(code);
        cases.push(result);
    }
    if let Some(path) = maybe_report {
        let file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .expect("fresh report file");
        serde_json::to_writer(file, &cases).expect("write recovery observations");
    }
    fs::remove_dir_all(root).expect("remove owned recovery fixtures");
}
