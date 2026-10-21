// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use crate::storage::{snapshot_codec, validation_history as history};
use crate::{
    DurableSyncRuntime, FjallNodeStore, StorageNamespace, SyncNetwork, SyncRuntimeConfig,
    chainstate::BasicFilterStartupMode,
};
use open_bitcoin_core::{
    chainstate::CoinsView,
    consensus::{CanonicalChain, chain_proof::ProofWork},
    primitives::{OutPoint, Txid},
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

pub(super) static SCENARIO: Mutex<()> = Mutex::new(());
pub(super) const SELECTORS: [(CanonicalChain, SyncNetwork, &str); 4] = [
    (CanonicalChain::Mainnet, SyncNetwork::Mainnet, "mainnet"),
    (CanonicalChain::Testnet3, SyncNetwork::Testnet, "testnet3"),
    (CanonicalChain::Signet, SyncNetwork::Signet, "signet"),
    (CanonicalChain::Regtest, SyncNetwork::Regtest, "regtest"),
];

pub(super) fn source() -> PathBuf {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("owned fixture nonce");
    fs::canonicalize(std::env::temp_dir())
        .expect("canonical fixture parent")
        .join(format!(
            "phase166-qualification-{:x}",
            u128::from_le_bytes(nonce)
        ))
}

pub(super) fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!(
        "../../../../../docs/parity/fixtures/phase166-genesis-vectors.json"
    ))
    .expect("frozen independent source vectors")
}

pub(super) fn vector(name: &str) -> serde_json::Value {
    vectors()["genesis"]
        .as_array()
        .expect("four source vectors")
        .iter()
        .find(|row| row["chain"] == name)
        .expect("selected vector")
        .clone()
}

pub(super) fn bytes(hex: &str) -> Vec<u8> {
    assert!(hex.len().is_multiple_of(2));
    hex.as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("hex pair"), 16).expect("hex byte")
        })
        .collect()
}

pub(super) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(super) fn runtime(
    source: &Path,
    chain: CanonicalChain,
    network: SyncNetwork,
    existing: bool,
) -> DurableSyncRuntime {
    runtime_mode(
        source,
        chain,
        network,
        existing,
        BasicFilterStartupMode::Enabled,
    )
}

pub(super) fn runtime_mode(
    source: &Path,
    chain: CanonicalChain,
    network: SyncNetwork,
    existing: bool,
    mode: BasicFilterStartupMode,
) -> DurableSyncRuntime {
    let opened = if existing {
        FjallNodeStore::reopen_canonical_genesis_existing(source, chain)
    } else {
        FjallNodeStore::open_canonical_genesis(source, chain)
    }
    .expect("product canonical admission");
    DurableSyncRuntime::open_canonical_genesis_configured(
        opened,
        SyncRuntimeConfig {
            network,
            ..Default::default()
        },
        mode,
    )
    .expect("earned canonical runtime")
}

pub(super) fn assert_runtime(runtime: &DurableSyncRuntime, expected: &serde_json::Value) {
    assert_runtime_mode(runtime, expected, BasicFilterStartupMode::Enabled);
}

pub(super) fn assert_runtime_mode(
    runtime: &DurableSyncRuntime,
    expected: &serde_json::Value,
    mode: BasicFilterStartupMode,
) {
    let handle = runtime.network_handle();
    let tip = handle
        .maybe_chain_tip()
        .expect("tip read")
        .expect("genesis tip");
    let work = bytes(
        expected["proof_le"]
            .as_str()
            .expect("independent work bytes"),
    );
    assert_eq!(tip.height, 0);
    assert_eq!(hex(tip.block_hash.as_bytes()), expected["block_hash_raw"]);
    assert_eq!(tip.chain_work.to_le_bytes().as_slice(), work);
    assert_eq!(tip.chain_work.to_hex(), expected["proof_hex"]);
    let headers = handle.best_chain_entries().expect("earned header owner");
    assert_eq!(headers.len(), 1);
    assert_eq!(headers[0].chain_work.to_le_bytes().as_slice(), work);
    assert_eq!(headers[0].block_hash, tip.block_hash);
    for entries in [
        runtime
            .store()
            .load_header_entries()
            .expect("stored headers"),
        runtime
            .store()
            .load_block_index_entries()
            .expect("stored index"),
    ] {
        let entries = entries.expect("complete stored projection").entries;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].chain_work.to_le_bytes().as_slice(), work);
        assert_eq!(entries[0].block_hash, tip.block_hash);
    }
    let (active, _) = runtime
        .store()
        .load_chain_meta_for_open()
        .expect("stored active metadata");
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].chain_work.to_le_bytes().as_slice(), work);
    assert_eq!(active[0].block_hash, tip.block_hash);
    let coins = runtime.store().coins_view();
    assert_eq!(coins.best_block().expect("coins B"), Some(tip.block_hash));
    assert!(coins.head_blocks().expect("H absence").is_empty());
    assert!(
        coins
            .complete_snapshot()
            .expect("complete coins")
            .coins
            .is_empty()
    );
    let txid: [u8; 32] = bytes(expected["txid_raw"].as_str().expect("fixed genesis txid"))
        .try_into()
        .expect("32-byte txid");
    assert!(
        coins
            .get_coin(&OutPoint {
                txid: Txid::from_byte_array(txid),
                vout: 0
            })
            .expect("actual outpoint lookup")
            .is_none()
    );
    let block = runtime
        .store()
        .load_block(tip.block_hash)
        .expect("body read")
        .expect("complete body");
    assert_eq!(
        hex(&open_bitcoin_core::codec::encode_block(&block).expect("body encoding")),
        expected["block_hex"]
    );
    assert!(!handle.relay_activation_enabled().expect("relay off"));
    if matches!(mode, BasicFilterStartupMode::Disabled) {
        assert!(
            handle
                .maybe_basic_index_progress()
                .expect("no owner")
                .is_none()
        );
        assert!(
            runtime
                .store()
                .maybe_basic_filter_checkpoint()
                .expect("no checkpoint")
                .is_none()
        );
        assert_eq!(
            runtime
                .store()
                .basic_filter_artifacts()
                .expect("no artifacts"),
            (false, false)
        );
        return;
    }
    let record = runtime
        .store()
        .load_basic_filter_record(tip.block_hash)
        .expect("filter read")
        .expect("canonical genesis filter");
    assert_eq!(hex(record.encoded_bytes()), expected["basic"]["encoded"]);
    assert_eq!(
        hex(record.identity().filter_hash().as_bytes()),
        expected["basic"]["hash_raw"]
    );
    assert_eq!(
        hex(record.identity().filter_header().as_bytes()),
        expected["basic"]["header_raw"]
    );
    assert_eq!(
        hex(record.identity().previous_header().as_bytes()),
        expected["basic"]["predecessor_raw"]
    );
    let progress = handle
        .maybe_basic_index_progress()
        .expect("index owner")
        .expect("accepted progress");
    assert_eq!(
        progress.maybe_safe_durable_endpoint(),
        Some(record.identity())
    );
    assert_eq!(progress.current_lag(), 0);
}

pub(super) fn numeric_work(value: &serde_json::Value) -> ProofWork {
    ProofWork::from_hex(value.as_str().expect("fixed numerical hex")).expect("256-bit number")
}

pub(super) fn keyspace(store: &FjallNodeStore, namespace: StorageNamespace) -> fjall::Keyspace {
    store
        .database()
        .keyspace(namespace.as_str(), fjall::KeyspaceCreateOptions::default)
        .expect("existing namespace")
}

pub(super) fn rewrite_json(
    store: &FjallNodeStore,
    namespace: StorageNamespace,
    key: &str,
    change: impl FnOnce(&mut serde_json::Value),
) {
    let space = keyspace(store, namespace);
    let bytes = space
        .get(key)
        .expect("read existing JSON")
        .expect("existing JSON");
    let mut value = serde_json::from_slice(&bytes).expect("current JSON");
    change(&mut value);
    space
        .insert(key, serde_json::to_vec(&value).expect("mutated JSON"))
        .expect("test-only corruption");
}

#[derive(PartialEq, Eq)]
pub(super) struct PhysicalEntry {
    kind: &'static str,
    length: u64,
    bytes: Vec<u8>,
    maybe_link: Option<PathBuf>,
    modified: std::time::SystemTime,
    #[cfg(unix)]
    mode: u32,
}

pub(super) fn physical_manifest(source: &Path) -> BTreeMap<PathBuf, PhysicalEntry> {
    fn visit(root: &Path, current: &Path, entries: &mut BTreeMap<PathBuf, PhysicalEntry>) {
        let metadata = fs::symlink_metadata(current).expect("fixture metadata");
        let kind = metadata.file_type();
        entries.insert(
            current
                .strip_prefix(root)
                .expect("relative fixture")
                .to_path_buf(),
            PhysicalEntry {
                kind: if kind.is_file() {
                    "file"
                } else if kind.is_dir() {
                    "directory"
                } else if kind.is_symlink() {
                    "symlink"
                } else {
                    "special"
                },
                length: metadata.len(),
                bytes: if kind.is_file() {
                    fs::read(current).expect("fixture bytes")
                } else {
                    vec![]
                },
                maybe_link: if kind.is_symlink() {
                    Some(fs::read_link(current).expect("fixture target"))
                } else {
                    None
                },
                modified: metadata.modified().expect("fixture mtime"),
                #[cfg(unix)]
                mode: {
                    use std::os::unix::fs::MetadataExt;
                    metadata.mode()
                },
            },
        );
        if kind.is_dir() {
            for entry in fs::read_dir(current).expect("fixture directory") {
                visit(root, &entry.expect("fixture entry").path(), entries);
            }
        }
    }
    let mut entries = BTreeMap::new();
    visit(source, source, &mut entries);
    entries
}

#[derive(Clone, Copy)]
pub(super) enum Mutation {
    IdentitySelector,
    IdentityRoot,
    Body,
    JsonByte(StorageNamespace, &'static str, &'static str),
    JsonNumber(StorageNamespace, &'static str, &'static str),
    LegacyWork,
    Coverage,
    ValidationStatus,
    BestBlock,
    MissingBody,
    MissingValidation,
    Missing(StorageNamespace, &'static str),
    Extra(StorageNamespace, &'static str),
    Oversized,
    Descendant,
}

pub(super) fn apply_mutation(raw: &FjallNodeStore, mutation: Mutation) {
    use StorageNamespace::{BlockIndex, Chainstate, Coins, Headers, Schema};
    let hash = CanonicalChain::Regtest.spec().genesis_hash();
    match mutation {
        Mutation::IdentitySelector | Mutation::IdentityRoot => {
            let space = keyspace(raw, Schema);
            let key = super::super::records::IDENTITY_KEY;
            let mut value = space
                .get(key)
                .expect("identity")
                .expect("identity present")
                .to_vec();
            value[if matches!(mutation, Mutation::IdentitySelector) {
                1
            } else {
                2
            }] ^= 1;
            space.insert(key, value).expect("mutated identity");
        }
        Mutation::Body => {
            let space = keyspace(raw, BlockIndex);
            let key = crate::storage::fjall_store::block_key(hash);
            let mut value = space
                .get(&key)
                .expect("body")
                .expect("body present")
                .to_vec();
            *value.last_mut().expect("lock time byte") ^= 1;
            space.insert(key, value).expect("mutated complete body");
        }
        Mutation::JsonByte(namespace, key, pointer) => rewrite_json(raw, namespace, key, |value| {
            let byte = value.pointer_mut(pointer).expect("one-byte field");
            *byte = serde_json::json!(byte.as_u64().expect("u8 field") ^ 1);
        }),
        Mutation::JsonNumber(namespace, key, pointer) => {
            rewrite_json(raw, namespace, key, |value| {
                let number = value.pointer_mut(pointer).expect("number field");
                *number = serde_json::json!(number.as_i64().expect("integer field") + 1);
            })
        }
        Mutation::LegacyWork => rewrite_json(raw, Headers, "snapshot", |value| {
            value["payload"]["entries"][0]["chain_work"] = serde_json::json!(42);
        }),
        Mutation::Coverage => {
            keyspace(raw, BlockIndex)
                .insert(
                    history::COVERAGE_KEY,
                    history::encode_coverage(history::ValidationCoverage::UnknownLegacy),
                )
                .expect("coverage mutation");
        }
        Mutation::ValidationStatus => {
            let key = history::record_key(hash);
            let space = keyspace(raw, BlockIndex);
            let value = space.get(&key).expect("validation row").expect("present");
            let record = history::decode_record(&key, &value).expect("current validation row");
            space
                .insert(
                    key,
                    history::encode_record(history::ValidationHistoryRecord::new(
                        record.identity(),
                        history::StoredValidationStatus::KnownOnly,
                    )),
                )
                .expect("unqualified validation");
        }
        Mutation::BestBlock => {
            let mut value = *hash.as_bytes();
            value[0] ^= 1;
            keyspace(raw, Coins)
                .insert("B", value)
                .expect("one mismatched B");
        }
        Mutation::MissingBody => {
            keyspace(raw, BlockIndex)
                .remove(crate::storage::fjall_store::block_key(hash))
                .expect("missing complete body");
        }
        Mutation::MissingValidation => {
            keyspace(raw, BlockIndex)
                .remove(history::record_key(hash))
                .expect("missing genesis validation");
        }
        Mutation::Missing(namespace, key) => {
            keyspace(raw, namespace)
                .remove(key)
                .expect("one missing required row");
        }
        Mutation::Extra(namespace, key) => {
            keyspace(raw, namespace)
                .insert(key, [1])
                .expect("one unsupported row");
        }
        Mutation::Oversized => {
            let space = keyspace(raw, Headers);
            let mut value = space
                .get("snapshot")
                .expect("header bytes")
                .expect("present")
                .to_vec();
            value.resize(64 * 1024 + 1, b' ');
            space
                .insert("snapshot", value)
                .expect("one-over authority byte limit");
        }
        Mutation::Descendant => {
            let mut entries = raw
                .load_header_entries()
                .expect("headers")
                .expect("present")
                .entries;
            let (mut active, counts) = raw.load_chain_meta_for_open().expect("metadata");
            let mut header = entries[0].header.clone();
            header.previous_block_hash = hash;
            header.time += 600;
            let work = active[0].chain_work.wrapping_add(
                open_bitcoin_core::consensus::chain_proof::block_proof(header.bits)
                    .expect("numerical child proof"),
            );
            let position = open_bitcoin_core::chainstate::ChainPosition::new(
                header.clone(),
                1,
                work,
                i64::from(header.time),
            );
            entries.push(open_bitcoin_network::HeaderEntry {
                block_hash: position.block_hash,
                header,
                height: 1,
                chain_work: work,
            });
            keyspace(raw, Coins)
                .insert("B", position.block_hash.as_bytes())
                .expect("correlated B");
            active.push(position);
            keyspace(raw, Headers)
                .insert(
                    "snapshot",
                    snapshot_codec::encode_header_entries(&entries)
                        .expect("numerical header ancestry"),
                )
                .expect("correlated headers");
            keyspace(raw, BlockIndex)
                .insert(
                    "snapshot",
                    snapshot_codec::encode_block_index_entries(&entries)
                        .expect("numerical block ancestry"),
                )
                .expect("correlated index");
            keyspace(raw, Chainstate)
                .insert(
                    "chain_meta",
                    snapshot_codec::encode_chain_meta(&active, counts.as_ref())
                        .expect("numerical metadata"),
                )
                .expect("correlated active chain");
        }
    }
}

/// Spawn after every parent owner closes, so this child inherits no fixture
/// engine/lease descriptor and must earn its own same-path capability.
pub(super) fn reopen_in_independent_child(
    source: &Path,
    selector: &str,
    mode: BasicFilterStartupMode,
) {
    use std::io::Read;
    let mut child = std::process::Command::new(
        std::env::current_exe().expect("current test executable"),
    )
    .args([
        "--exact",
        "storage::canonical_bootstrap::tests::test_support::__phase166_canonical_reopen_worker",
        "--nocapture",
        "--test-threads=1",
    ])
    .env("PHASE166_QUALIFICATION_SOURCE", source)
    .env("PHASE166_QUALIFICATION_SELECTOR", selector)
    .env(
        "PHASE166_QUALIFICATION_BASIC",
        if matches!(mode, BasicFilterStartupMode::Enabled) {
            "enabled"
        } else {
            "disabled"
        },
    )
    .stdin(std::process::Stdio::null())
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::null())
    .spawn()
    .expect("independent canonical child");
    let started = std::time::Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("child status") {
            let mut output = Vec::new();
            child
                .stdout
                .take()
                .expect("bounded child proof channel")
                .take(4097)
                .read_to_end(&mut output)
                .expect("completed child proof read");
            assert!(
                status.success(),
                "independent child must derive complete canonical authority"
            );
            assert!(output.len() <= 4096, "bounded child response");
            let expected = format!("phase166-child-proof selector={selector} basic={mode:?}");
            assert!(
                std::str::from_utf8(&output)
                    .expect("fixed child protocol")
                    .contains(&expected),
                "zero-discovery children cannot count as reopen evidence"
            );
            return;
        }
        if started.elapsed() >= std::time::Duration::from_secs(45) {
            child.kill().expect("stop overdue owned child");
            child.wait().expect("reap owned child");
            panic!("independent canonical child exceeded test deadline");
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[test]
fn __phase166_canonical_reopen_worker() {
    use std::io::Write;
    let Some(source) = std::env::var_os("PHASE166_QUALIFICATION_SOURCE") else {
        return;
    };
    let selector = std::env::var("PHASE166_QUALIFICATION_SELECTOR").expect("fixed child selector");
    let (chain, network, name) = SELECTORS
        .into_iter()
        .find(|(_, _, name)| *name == selector)
        .expect("existing selector");
    let mode = match std::env::var("PHASE166_QUALIFICATION_BASIC")
        .expect("fixed BASIC mode")
        .as_str()
    {
        "enabled" => BasicFilterStartupMode::Enabled,
        "disabled" => BasicFilterStartupMode::Disabled,
        _ => panic!("unknown child BASIC mode"),
    };
    let reopened = runtime_mode(Path::new(&source), chain, network, true, mode);
    assert_runtime_mode(&reopened, &vector(name), mode);
    drop(reopened);
    // This bounded test protocol marker is emitted only after actual assertions
    // and complete runtime teardown; it is not a diagnostic payload.
    writeln!(
        std::io::stdout().lock(),
        "phase166-child-proof selector={name} basic={mode:?}"
    )
    .expect("child proof marker");
}
