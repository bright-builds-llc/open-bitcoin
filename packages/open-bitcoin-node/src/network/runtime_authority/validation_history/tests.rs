// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/rpc/mining.cpp

use super::*;
use crate::chainstate::validation_history::tests::phase167::{self as genuine, child, remine};
use open_bitcoin_core::consensus::{CanonicalChain, block_hash};
use open_bitcoin_network::{BodyEligibilityError, HeaderStore};

mod compact_download_cleanup;
mod observation;
mod reopen_bound;
mod resources;

type NativeRows = std::collections::BTreeMap<String, std::collections::BTreeMap<Vec<u8>, Vec<u8>>>;
fn native_rows(store: &FjallNodeStore) -> NativeRows {
    let database = store.database();
    let snapshot = database.snapshot();
    database
        .list_keyspace_names()
        .into_iter()
        .map(|name| {
            assert!(database.keyspace_exists(&name));
            let keyspace = database
                .keyspace(&name, fjall::KeyspaceCreateOptions::default)
                .expect("existing namespace");
            let rows = snapshot
                .iter(&keyspace)
                .map(|guard| {
                    let (key, value) = guard.into_inner().expect("actual row");
                    (key.to_vec(), value.to_vec())
                })
                .collect();
            (name.to_string(), rows)
        })
        .collect()
}

fn units(rows: &NativeRows) -> usize {
    rows.len() + rows.values().map(|rows| rows.len()).sum::<usize>()
}

fn fill_inventory(store: &FjallNodeStore, target: usize) {
    let current = units(&native_rows(store));
    assert!(current <= target);
    for index in current..target {
        store
            .write_raw_for_test(
                crate::StorageNamespace::Wallet,
                &format!("wallet_snapshot:resource-{index}"),
                vec![1],
            )
            .expect("actual unrelated row");
    }
    store
        .inspect_submission_inventory()
        .expect("existing native policy supports baseline");
    assert_eq!(units(&native_rows(store)), target);
}

fn reopen<T>(
    path: &Path,
    runtime: DurableSyncRuntime,
    command: T,
    mode: BasicFilterStartupMode,
) -> (DurableSyncRuntime, CanonicalSubmissionCommand) {
    drop(command);
    drop(runtime);
    let (runtime, maybe_command) =
        DurableSyncRuntime::open_canonical_submission_configured(path, config(), mode)
            .expect("all handles dropped before actual safe configured reopen");
    (runtime, maybe_command.expect("same selected command"))
}

// Retain every Accepted data field; the deterministic recovery anchor is checked separately.
fn stable_state(command: &CanonicalSubmissionCommand) -> impl fmt::Debug + PartialEq + use<> {
    command
        .network
        .read(|network| {
            command.check_owner(network).expect("actual current owner");
            let manager = network.chainstate();
            (
                manager
                    .export_chainstate_snapshot()
                    .expect("complete effective Coins"),
                manager
                    .chainstate()
                    .coins_best_block()
                    .expect("live Coins best"),
                command
                    .store
                    .maybe_basic_filter_state()
                    .expect("stored BASIC"),
                manager.maybe_basic_index_progress().map(|progress| {
                    (
                        progress.generation(),
                        progress.accepted_target(),
                        progress.maybe_processed_endpoint(),
                        progress.maybe_safe_durable_endpoint(),
                        progress.protection(),
                        progress.initially_synchronized(),
                        progress.state(),
                    )
                }),
                manager
                    .chainstate()
                    .active_chain()
                    .iter()
                    .map(|position| {
                        (
                            manager
                                .validation_provenance(position.block_hash)
                                .expect("live provenance"),
                            command
                                .store
                                .validation_provenance(position.block_hash)
                                .expect("durable provenance"),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .expect("actual authority data")
}

fn maybe_basic_anchor(command: &CanonicalSubmissionCommand) -> Option<BlockHash> {
    command
        .network
        .maybe_basic_index_progress()
        .expect("actual BASIC owner")
        .map(|progress| progress.branch_identity())
}

fn admit_only(command: &CanonicalSubmissionCommand, blocks: &[Block]) {
    let now = i64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("shell clock")
            .as_secs(),
    )
    .expect("clock range");
    command
        .network
        .mutate(|network| {
            let mut headers = network.peer_manager.header_store().clone();
            let identities = blocks
                .iter()
                .map(|block| {
                    let inserted = headers
                        .insert_canonical_header_detailed(block.header.clone(), now)
                        .expect("actual admitted header");
                    crate::storage::validation_history::BlockValidationIdentity::new(
                        inserted.block_hash,
                        block.header.previous_block_hash,
                        inserted.height,
                    )
                    .expect("earned identity")
                })
                .collect();
            network.peer_manager.seed_header_store(headers);
            network
                .publish_admitted_header_identities(identities)
                .expect("actual KnownOnly publication");
            network
                .persist_validation_header_snapshot()
                .expect("actual trusted snapshot");
        })
        .expect("actual held authority");
}
mod typed_boundaries;

thread_local! {
    static ABSORB_FAULT: std::cell::Cell<Option<crate::storage::fjall_store::validation_history::HistoryPublicationFault>> = const { std::cell::Cell::new(None) };
    static CHANGE_AFTER_BASIC: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static ADVANCE_AFTER_BASIC: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}
pub(super) fn before_absorb(store: &FjallNodeStore) {
    if let Some(fault) = ABSORB_FAULT.take() {
        store.set_validation_history_fault(fault);
    }
}
pub(super) fn after_basic(command: &CanonicalSubmissionCommand) {
    if CHANGE_AFTER_BASIC.take() {
        command
            .store
            .write_raw_for_test(crate::StorageNamespace::Runtime, "snapshot", vec![1])
            .expect("actual supported external row write");
    }
    if !ADVANCE_AFTER_BASIC.replace(false) {
        return;
    }
    command
        .network
        .mutate(|network| {
            let mut headers = network.peer_manager.header_store().clone();
            let mut identities = Vec::new();
            while headers.entries().count() <= FjallNodeStore::canonical_history_reopen_limit() {
                let parent = headers.best_tip().expect("actual tip");
                let block = child(&parent.header, parent.height + 1);
                let inserted = headers
                    .insert_canonical_header(block.header.clone(), 2_000_000_000)
                    .expect("actual external admission");
                identities.push(
                    crate::storage::validation_history::BlockValidationIdentity::new(
                        inserted.block_hash,
                        block.header.previous_block_hash,
                        inserted.height,
                    )
                    .expect("admitted identity"),
                );
            }
            network.peer_manager.seed_header_store(headers);
            network
                .publish_admitted_header_identities(identities)
                .expect("actual KnownOnly publication");
        })
        .expect("same actual authority");
}

fn directory() -> std::path::PathBuf {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    std::fs::canonicalize(std::env::temp_dir())
        .expect("physical parent")
        .join(format!("phase167-submit-{:x}", u128::from_le_bytes(nonce)))
}
fn config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        ..Default::default()
    }
}
fn wire(block: &Block) -> WireBlock {
    let bytes = open_bitcoin_core::codec::encode_block(block).expect("exact wire");
    let (wire, used) = open_bitcoin_core::codec::block::wire::parse_submission_block_prefix(&bytes)
        .expect("complete decode");
    assert_eq!(used, bytes.len());
    wire
}
fn accepted(outcome: CanonicalSubmissionOutcome) -> ChainPosition {
    outcome.fold(
        |position| position,
        |_| panic!("unexpected duplicate"),
        |reason| panic!("rejected: {reason}"),
        || panic!("inconclusive"),
    )
}

fn accepted_state(command: &CanonicalSubmissionCommand) -> genuine::Accepted {
    command
        .network
        .read(|network| genuine::accepted(network.chainstate(), &command.store))
        .expect("complete accepted state")
}

fn runtime(
    mode: BasicFilterStartupMode,
) -> (
    std::path::PathBuf,
    DurableSyncRuntime,
    CanonicalSubmissionCommand,
) {
    let path = directory();
    let (runtime, command) =
        DurableSyncRuntime::open_canonical_submission_configured(&path, config(), mode)
            .expect("safe factory");
    (path, runtime, command.expect("Regtest command"))
}

fn reject(command: &CanonicalSubmissionCommand, raw: WireBlock, reason: &str) {
    let before = accepted_state(command);
    command
        .submit(raw)
        .expect("conclusive actual validation")
        .fold(
            |_| panic!("false acceptance"),
            |_| panic!("false duplicate"),
            |actual| assert_eq!(actual, reason),
            || panic!("false inconclusive"),
        );
    assert_eq!(accepted_state(command), before);
}

#[test]
fn phase167_local_submission_genesis_seal_and_other_selectors_keep_strict_routing() {
    for network in [
        SyncNetwork::Mainnet,
        SyncNetwork::Testnet,
        SyncNetwork::Signet,
        SyncNetwork::Regtest,
    ] {
        // Arrange
        let path = directory();
        let config = SyncRuntimeConfig {
            network,
            ..Default::default()
        };
        let opened = FjallNodeStore::open_canonical_genesis(&path, selected(&config))
            .expect("actual held seal");
        // Act
        let (runtime, command) = DurableSyncRuntime::open_genesis_submission_configured(
            opened,
            config.clone(),
            BasicFilterStartupMode::Disabled,
        )
        .expect("consumes existing lease");
        // Assert
        assert_eq!(command.is_some(), network == SyncNetwork::Regtest);
        drop(command);
        drop(runtime);
        let (runtime, command) = DurableSyncRuntime::open_canonical_submission_configured(
            &path,
            config,
            BasicFilterStartupMode::Disabled,
        )
        .expect("strict genesis or proved history reopen");
        assert_eq!(command.is_some(), network == SyncNetwork::Regtest);
        close(path, runtime, command);
    }
}

fn manifest(root: &Path) -> std::collections::BTreeMap<std::path::PathBuf, [u8; 32]> {
    fn walk(
        root: &Path,
        at: &Path,
        result: &mut std::collections::BTreeMap<std::path::PathBuf, [u8; 32]>,
    ) {
        for entry in std::fs::read_dir(at).expect("whole inventory") {
            let entry = entry.expect("directory entry");
            let path = entry.path();
            if entry.file_type().expect("kind").is_dir() {
                walk(root, &path, result);
            } else {
                result.insert(
                    path.strip_prefix(root).expect("owned path").to_path_buf(),
                    open_bitcoin_core::consensus::crypto::Sha256::digest(
                        &std::fs::read(path).expect("whole file"),
                    ),
                );
            }
        }
    }
    let mut result = Default::default();
    walk(root, root, &mut result);
    result
}

#[test]
fn phase167_local_submission_factories_duplicate_root_and_real_first_acceptance() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let path = directory();
        let (runtime, command) =
            DurableSyncRuntime::open_canonical_submission_configured(&path, config(), mode)
                .expect("safe factory");
        let command = command.expect("Regtest command");
        let genesis = CanonicalChain::Regtest.spec().genesis_block();
        // Act / Assert
        command
            .submit(wire(&genesis))
            .expect("actual accepted root")
            .fold(
                |_| panic!("not new"),
                |hash| assert_eq!(hash, block_hash(&genesis.header)),
                |_| panic!("not rejected"),
                || panic!("not inconclusive"),
            );
        let block = child(&genesis.header, 1);
        let position = accepted(command.submit(wire(&block)).expect("real local submission"));
        assert_eq!(position.block_hash, block_hash(&block.header));
        assert_eq!(
            runtime
                .network_handle()
                .maybe_chain_tip()
                .expect("actual tip"),
            Some(position)
        );
        drop(command);
        drop(runtime);
        std::fs::remove_dir_all(path).expect("all handles dropped");
    }
}

fn params() -> ConsensusParams {
    let chain = CanonicalChain::Regtest;
    let proof = open_bitcoin_core::consensus::chain_spec::CanonicalGenesisProof::prove(
        chain.spec(),
        &chain.spec().genesis_block(),
    )
    .expect("proved root");
    open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&proof)
}

#[test]
fn phase167_local_submission_typed_foreign_store_params_and_incarnation_are_operational() {
    for case in 0..3 {
        // Arrange: both owners really come from safe factories.
        let (path, runtime, mut command) = runtime(BasicFilterStartupMode::Disabled);
        let other_path = directory();
        let (other_runtime, other) = DurableSyncRuntime::open_canonical_submission_configured(
            &other_path,
            config(),
            BasicFilterStartupMode::Disabled,
        )
        .expect("other genuine factory");
        let other = other.expect("other command");
        let before = runtime
            .network_handle()
            .chainstate_snapshot()
            .expect("actual original state");
        if case == 0 {
            command.store = other.store.clone();
        }
        if case == 1 {
            command.params.coinbase_maturity += 1;
        }
        if case == 2 {
            command.incarnation = other.incarnation;
        }
        // Act / Assert
        assert!(
            command
                .submit(wire(&child(
                    &CanonicalChain::Regtest.spec().genesis_header(),
                    1
                )))
                .is_err()
        );
        assert_eq!(
            runtime
                .network_handle()
                .chainstate_snapshot()
                .expect("unchanged actual owner"),
            before
        );
        drop(command);
        drop(runtime);
        drop(other);
        drop(other_runtime);
        std::fs::remove_dir_all(path).expect("all dropped");
        std::fs::remove_dir_all(other_path).expect("other dropped");
    }
}

fn close<T>(path: std::path::PathBuf, runtime: DurableSyncRuntime, command: T) {
    drop(command);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("all handles dropped");
}

#[test]
fn phase167_local_submission_typed_header_and_raw_amount_reasons_survive_real_command() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    block.header.version = 3;
    remine(&mut block);
    // Act / Assert
    reject(&command, wire(&block), "bad-version(0x00000003)");
    let mut future = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    future.header.time = u32::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_secs()
            + 7201,
    )
    .expect("future timestamp");
    remine(&mut future);
    reject(&command, wire(&future), "time-too-new");
    let mut proof = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    while open_bitcoin_core::consensus::block::check_block_header_with_params(
        &proof.header,
        &params(),
    )
    .is_ok()
    {
        proof.header.nonce += 1;
    }
    reject(&command, wire(&proof), "high-hash");
    let mut raw = wire(&child(&CanonicalChain::Regtest.spec().genesis_header(), 1));
    raw.transactions[0].outputs[0].value_sats = -1;
    raw.header.merkle_root =
        open_bitcoin_core::consensus::crypto::block_merkle_root_view(&raw.transactions)
            .expect("real raw base identity")
            .0;
    while open_bitcoin_core::consensus::block::check_block_header_with_params(
        &raw.header,
        &params(),
    )
    .is_err()
    {
        raw.header.nonce += 1;
    }
    reject(&command, raw, "bad-txns-vout-negative");
    assert_eq!(
        command
            .network
            .header_entries()
            .expect("no admission")
            .len(),
        1
    );
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_resource_exact_body_boundary_preserves_maps_and_reopens_next() {
    use open_bitcoin_core::primitives::ScriptBuf;
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(mode);
        for namespace in [
            crate::StorageNamespace::Wallet,
            crate::StorageNamespace::Metrics,
            crate::StorageNamespace::Mempool,
            crate::StorageNamespace::Runtime,
        ] {
            command
                .store
                .write_raw_for_test(namespace, "snapshot", vec![1, 2, 3])
                .expect("unrelated actual rows");
        }
        let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        block.transactions[0].outputs[0].script_pubkey =
            ScriptBuf::from_serialized_bytes(vec![0x51; 65391]);
        remine(&mut block);
        assert_eq!(
            open_bitcoin_core::codec::encode_block(&block)
                .expect("actual encoded body")
                .len(),
            65536
        );
        // Act
        accepted(
            command
                .submit(wire(&block))
                .expect("actual exact supported row boundary"),
        );
        let state = stable_state(&command);
        let incarnation = command.incarnation;
        let maybe_progress = command
            .network
            .maybe_basic_index_progress()
            .expect("actual BASIC progress");
        assert_eq!(
            maybe_progress.is_some(),
            mode == BasicFilterStartupMode::Enabled
        );
        let genesis = block_hash(&CanonicalChain::Regtest.spec().genesis_header());
        let hash = block_hash(&block.header);
        if let Some(progress) = maybe_progress {
            assert_eq!(progress.branch_identity(), genesis);
        }
        let records = [genesis, hash].map(|hash| {
            command
                .store
                .load_basic_filter_record(hash)
                .expect("full BASIC record")
        });
        let rows = native_rows(&command.store);
        assert_eq!(
            rows.keys().map(String::as_str).collect::<Vec<_>>(),
            [
                "block_index",
                "chainstate",
                "coins",
                "headers",
                "mempool",
                "metrics",
                "runtime",
                "schema",
                "wallet"
            ]
        );
        for name in ["wallet", "metrics", "mempool", "runtime"] {
            assert_eq!(rows[name].get(b"snapshot".as_slice()), Some(&vec![1, 2, 3]));
        }
        let coins = command
            .store
            .coins_view()
            .complete_snapshot()
            .expect("complete Coins");
        assert!(coins.coins.is_empty());
        if mode == BasicFilterStartupMode::Enabled {
            assert!(
                records[1]
                    .as_ref()
                    .expect("actual BASIC record")
                    .encoded_bytes()
                    .len()
                    < 65536
            );
        }
        let (runtime, command) = reopen(&path, runtime, command, mode);
        // Assert
        assert_ne!(command.incarnation, incarnation);
        assert_eq!(stable_state(&command), state);
        let maybe_reopened_progress = command
            .network
            .maybe_basic_index_progress()
            .expect("reconstructed BASIC owner");
        assert_eq!(maybe_reopened_progress.is_some(), maybe_progress.is_some());
        if let Some(progress) = maybe_reopened_progress {
            assert_eq!(progress.branch_identity(), hash);
        }
        assert_eq!(
            [genesis, hash].map(|hash| command
                .store
                .load_basic_filter_record(hash)
                .expect("exact full reopened BASIC")),
            records
        );
        assert_eq!(native_rows(&command.store), rows);
        accepted(
            command
                .submit(wire(&child(&block.header, 2)))
                .expect("valid next command"),
        );
        close(path, runtime, command);
    }
}
