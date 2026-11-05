// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Independent BASIC expectations for the stopped Plan 48 process fixture.

use super::Fixture;
use open_bitcoin_node::{
    DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig,
    chainstate::BasicFilterStartupMode,
    core::{
        chainstate::{
            ChainstateSnapshot, IndexInputProtection, coins::CompleteCoinsSnapshot,
            filter_index::catch_up::BasicIndexProgress,
        },
        consensus::block_hash,
        primitives::{Block, BlockHash, OutPoint, Txid},
    },
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

pub(super) type Rows = BTreeMap<String, BTreeMap<Vec<u8>, Vec<u8>>>;
pub(super) type Provenance = BTreeMap<
    BlockHash,
    (
        open_bitcoin_node::network::BasicBlockValidationProvenance,
        open_bitcoin_node::network::BasicBlockValidationProvenance,
    ),
>;
pub(super) type Facts = (
    (ChainstateSnapshot, Option<BlockHash>, CompleteCoinsSnapshot),
    Option<BasicIndexProgress>,
    Option<(
        Option<(u32, BlockHash)>,
        u32,
        BlockHash,
        IndexInputProtection,
    )>,
    Rows,
    Provenance,
);
pub(super) fn stopped(fixture: &Fixture, hash: BlockHash) -> (Facts, Block) {
    // Called only after Daemon::stop consumed/reaped the child. No concurrent owner.
    let (runtime, command) = DurableSyncRuntime::open_canonical_submission_configured(
        fixture.0.join("source"),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
        BasicFilterStartupMode::PreserveSaved,
    )
    .expect("stopped-state safe full reproof");
    let command = command.expect("Regtest command");
    let facts = command
        .observe_accepted_state(&[hash])
        .expect("actual stopped reconstructed facts")
        .fold(|coins, progress, stored, rows, provenance| {
            (coins, progress, stored, rows, provenance)
        });
    let body = runtime
        .store()
        .load_block(hash)
        .expect("actual retained bytes")
        .expect("retained body");
    assert_eq!(facts.3.len(), 9);
    drop(command);
    drop(runtime); // All observation handles gone before restart.
    (facts, body)
}
pub(super) fn stable(facts: &Facts) -> impl std::fmt::Debug + PartialEq + use<> {
    (
        facts.0.clone(),
        facts.1.map(|p| {
            (
                p.generation(),
                p.accepted_target(),
                p.maybe_processed_endpoint(),
                p.maybe_safe_durable_endpoint(),
                p.protection(),
                p.initially_synchronized(),
                p.state(),
            )
        }),
        facts.2,
        facts.3.clone(),
        facts.4.clone(),
    )
}

/// Feed only serialized test-owned coinbase blocks to the pinned Knots Python oracle.
pub(super) fn oracle_case(block: &Block, height: u32) -> Value {
    assert_eq!(block.transactions.len(), 1, "single coinbase block");
    assert_eq!(block.transactions[0].inputs.len(), 1, "no spent scripts");
    assert_eq!(
        block.transactions[0].inputs[0].previous_output,
        OutPoint::null()
    );
    let script = block.transactions[0].outputs[0].script_pubkey.as_bytes();
    assert!(
        !script.is_empty() && script[0] != 0x6a,
        "selected BASIC output"
    );
    let mut case = json!({
        "name": format!("height-{height}"),
        "previous_name": if height == 0 { String::new() } else { format!("height-{}", height - 1) },
        "raw_block": super::hex(&open_bitcoin_codec::encode_block(block).expect("raw block")),
        "block_hash_raw": super::hex(block_hash(&block.header).as_bytes()),
        "outputs": [],
        "spent": [],
        "predecessor_raw": "00".repeat(32),
    });
    if (302..=303).contains(&height) {
        case["include_coinbase_commitment"] = json!(true);
    }
    case
}

/// One bounded batch uses pinned Knots CBlock/SipHash/hash256, not production BASIC code.
pub(super) fn oracle(fixture: &Fixture, cases: &[Value]) -> Vec<Value> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root");
    assert_eq!(cases.len(), 304, "one complete oracle batch");
    let input_path = fixture.0.join("basic-oracle.input");
    let input = serde_json::to_vec(&json!({"cases": cases})).expect("oracle input JSON");
    assert!(input.len() <= 2_000_000, "bounded oracle input");
    let mut input_file = fs::File::create(&input_path).expect("oracle input");
    input_file.write_all(&input).expect("complete oracle input");
    input_file.sync_all().expect("durable oracle input");
    drop(input_file);
    let stdin = fs::File::open(&input_path).expect("oracle input for child");
    let stdout_path = fixture.0.join("basic-oracle.stdout");
    let stderr_path = fixture.0.join("basic-oracle.stderr");
    let stdout = fs::File::create(&stdout_path).expect("oracle output");
    let stderr = fs::File::create(&stderr_path).expect("oracle diagnostic");
    let mut child = Command::new("python3")
        .arg(root.join("scripts/basic-filter-oracle.py"))
        .current_dir(&root)
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .stdin(Stdio::from(stdin))
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()
        .expect("pinned test-only oracle");
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        if let Some(status) = child.try_wait().expect("oracle status") {
            assert!(
                status.success(),
                "oracle: {}",
                bounded_diagnostic(&stderr_path)
            );
            break;
        }
        if Instant::now() >= deadline {
            child.kill().expect("kill owned oracle on timeout");
            child.wait().expect("reap timed-out oracle");
            panic!("bounded BASIC oracle timeout");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    child.wait().expect("reap completed oracle");
    let bytes = fs::read(stdout_path).expect("oracle result");
    assert!(bytes.len() <= 2_000_000, "bounded oracle result");
    let value: Value = serde_json::from_slice(&bytes).expect("oracle JSON");
    let rows = value["cases"].as_array().expect("oracle cases");
    assert_eq!(rows.len(), cases.len());
    assert_eq!(rows.len(), 304);
    for (height, row) in rows.iter().enumerate() {
        assert_eq!(row["name"], cases[height]["name"]);
        assert_eq!(row["block_hash_raw"], cases[height]["block_hash_raw"]);
        if height > 0 {
            assert_eq!(row["predecessor_raw"], rows[height - 1]["header_raw"]);
        }
    }
    assert_eq!(rows[0]["encoded"], "014756c0");
    assert_eq!(
        rows[0]["header_raw"],
        "2b5adc66021d5c775f630efd91518cf6ce3e9f525bbf54d9f0d709451e305e48"
    );
    rows.clone()
}

fn bounded_diagnostic(path: &Path) -> String {
    let mut bytes = Vec::new();
    fs::File::open(path)
        .expect("oracle diagnostic")
        .take(16_384)
        .read_to_end(&mut bytes)
        .expect("bounded oracle diagnostic");
    String::from_utf8_lossy(&bytes).into_owned()
}

fn raw(case: &Value, field: &str) -> Vec<u8> {
    let hex = case[field].as_str().expect("oracle hex field");
    assert!(hex.len().is_multiple_of(2), "complete raw hex");
    hex.as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII hex"), 16).expect("hex byte")
        })
        .collect()
}

/// Compare stopped native rows to Python/Knots output, never to product rows themselves.
pub(super) fn assert_basic_facts(facts: &Facts, block: &Block, expected: &Value, height: u32) {
    let hash = block_hash(&block.header);
    let next_height = height.checked_add(1).expect("bounded test height");
    assert_eq!(expected["name"], format!("height-{height}"));
    assert_eq!(super::hex(hash.as_bytes()), expected["block_hash_raw"]);
    assert_eq!(expected["include_coinbase_commitment"], true);
    let txid = open_bitcoin_node::core::consensus::transaction_txid(&block.transactions[0])
        .expect("product coinbase txid used for Coins and undo outpoint");
    assert_eq!(super::hex(txid.as_bytes()), expected["coinbase_txid_raw"]);
    assert_eq!(
        super::hex(block.header.merkle_root.as_bytes()),
        expected["merkle_root_raw"]
    );
    if height == 302 {
        let oracle_txid = Txid::from_slice(&raw(expected, "coinbase_txid_raw"))
            .expect("32-byte Knots coinbase txid");
        let oracle_outpoint = OutPoint {
            txid: oracle_txid,
            vout: 0,
        };
        assert!(!facts.0.0.utxos.contains_key(&oracle_outpoint));
        assert!(!facts.0.2.coins.contains_key(&oracle_outpoint));
        let undo = &facts.0.0.undo_by_block[&hash];
        assert!(undo.transactions.is_empty());
        assert_eq!(undo.overwrites.len(), 1);
        assert!(undo.overwrites[0].overwritten.is_empty());
    }
    assert_eq!(facts.0.0.tip().expect("stopped tip").height, height);
    assert_eq!(facts.0.0.tip().expect("stopped tip").block_hash, hash);
    let progress = facts.1.expect("durable BASIC progress");
    let processed = progress
        .maybe_processed_endpoint()
        .expect("processed wide body");
    assert_eq!(processed.height(), height);
    assert_eq!(processed.block_hash(), hash);
    assert_eq!(processed.parent_hash(), block.header.previous_block_hash);
    assert_eq!(
        super::hex(processed.previous_header().as_bytes()),
        expected["predecessor_raw"]
    );
    assert_eq!(
        super::hex(processed.filter_hash().as_bytes()),
        expected["hash_raw"]
    );
    assert_eq!(
        super::hex(processed.filter_header().as_bytes()),
        expected["header_raw"]
    );
    assert_eq!(progress.maybe_safe_durable_endpoint(), Some(processed));
    assert_eq!(
        progress.protection(),
        IndexInputProtection::FromHeight(next_height)
    );
    assert_eq!(progress.current_lag(), 0);
    assert_eq!(
        facts.2,
        Some((
            Some((height, hash)),
            height,
            hash,
            IndexInputProtection::FromHeight(next_height)
        ))
    );
    use open_bitcoin_node::network::BasicBlockValidationProvenance::ScriptsValid;
    assert_eq!(facts.4[&hash], (ScriptsValid, ScriptsValid));

    // Manually encode the private storage/filter_index.rs record, projection and state layout.
    let mut record = vec![1, 0];
    record.extend_from_slice(&height.to_le_bytes());
    record.extend_from_slice(hash.as_bytes());
    record.extend_from_slice(block.header.previous_block_hash.as_bytes());
    record.extend_from_slice(&raw(expected, "predecessor_raw"));
    record.extend_from_slice(&raw(expected, "hash_raw"));
    record.extend_from_slice(&raw(expected, "header_raw"));
    let encoded = raw(expected, "encoded");
    record.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
    record.extend_from_slice(&encoded);
    let record_key = format!("basic_filter:v1:record:{}", super::hex(hash.as_bytes()));
    assert_eq!(
        facts.3["block_index"].get(record_key.as_bytes()),
        Some(&record)
    );
    let mut projection = vec![1, 0];
    projection.extend_from_slice(&height.to_le_bytes());
    projection.extend_from_slice(hash.as_bytes());
    let projection_key = format!("basic_filter:v1:active:{height:08x}");
    assert_eq!(
        facts.3["block_index"].get(projection_key.as_bytes()),
        Some(&projection)
    );
    let mut state = vec![1, 0, 1];
    state.extend_from_slice(&height.to_le_bytes());
    state.extend_from_slice(hash.as_bytes());
    state.extend_from_slice(&height.to_le_bytes());
    state.extend_from_slice(hash.as_bytes());
    state.push(0);
    state.extend_from_slice(&next_height.to_le_bytes());
    assert_eq!(
        facts.3["block_index"].get(b"basic_filter:v1:state".as_slice()),
        Some(&state)
    );
    assert_eq!(
        facts.3["block_index"]
            .keys()
            .filter(|k| k.starts_with(b"basic_filter:v1:record:"))
            .count(),
        next_height as usize
    );
    assert_eq!(
        facts.3["block_index"]
            .keys()
            .filter(|k| k.starts_with(b"basic_filter:v1:active:"))
            .count(),
        next_height as usize
    );
}
