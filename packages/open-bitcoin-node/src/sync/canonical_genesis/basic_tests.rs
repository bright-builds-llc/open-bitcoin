// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

use super::*;
use crate::storage::fjall_store::filters::FilterPublicationFault as Fault;
use crate::{FjallNodeStore, StorageNamespace, storage::filter_index};
use open_bitcoin_core::{chainstate::CoinsView, primitives::OutPoint};
use std::path::{Path, PathBuf};

const SELECTORS: [(CanonicalChain, SyncNetwork, &str); 4] = [
    (CanonicalChain::Mainnet, SyncNetwork::Mainnet, "mainnet"),
    (CanonicalChain::Testnet3, SyncNetwork::Testnet, "testnet3"),
    (CanonicalChain::Signet, SyncNetwork::Signet, "signet"),
    (CanonicalChain::Regtest, SyncNetwork::Regtest, "regtest"),
];

fn source() -> PathBuf {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).expect("fixture nonce");
    std::fs::canonicalize(std::env::temp_dir())
        .expect("canonical temp parent")
        .join(format!("phase166-basic-{:x}", u128::from_le_bytes(nonce)))
}

fn runtime(
    source: &Path,
    chain: CanonicalChain,
    network: SyncNetwork,
    mode: BasicFilterStartupMode,
) -> DurableSyncRuntime {
    let opened =
        FjallNodeStore::open_canonical_genesis(source, chain).expect("proved canonical store");
    DurableSyncRuntime::open_canonical_genesis_configured(
        opened,
        SyncRuntimeConfig {
            network,
            ..Default::default()
        },
        mode,
    )
    .expect("canonical runtime")
}

fn vector(name: &str) -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../../docs/parity/fixtures/phase166-genesis-vectors.json"
    ))
    .expect("frozen independent vectors");
    fixture["genesis"]
        .as_array()
        .expect("four vectors")
        .iter()
        .find(|row| row["chain"] == name)
        .expect("selector vector")
        .clone()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn assert_genesis_filter(runtime: &DurableSyncRuntime, expected: &serde_json::Value) {
    let hash = runtime
        .network_handle()
        .maybe_chain_tip()
        .expect("tip")
        .expect("genesis")
        .block_hash;
    let record = runtime
        .store
        .load_basic_filter_record(hash)
        .expect("saved record")
        .expect("genesis indexed");
    let identity = record.identity();
    assert_eq!(identity.height(), 0);
    let index = runtime
        .store
        .database()
        .keyspace(
            StorageNamespace::BlockIndex.as_str(),
            fjall::KeyspaceCreateOptions::default,
        )
        .expect("existing namespace");
    assert_eq!(
        index.prefix(filter_index::RECORD_PREFIX).count(),
        1,
        "exactly one immutable genesis record"
    );
    assert_eq!(
        index.prefix(filter_index::ACTIVE_PREFIX).count(),
        1,
        "exactly one active height"
    );
    assert_eq!(
        hex(identity.block_hash().as_bytes()),
        expected["block_hash_raw"]
    );
    assert_eq!(hex(record.encoded_bytes()), expected["basic"]["encoded"]);
    assert_eq!(
        hex(identity.filter_hash().as_bytes()),
        expected["basic"]["hash_raw"]
    );
    assert_eq!(
        hex(identity.filter_header().as_bytes()),
        expected["basic"]["header_raw"]
    );
    assert_eq!(
        hex(identity.previous_header().as_bytes()),
        expected["basic"]["predecessor_raw"]
    );
    let body = runtime
        .store
        .load_block(hash)
        .expect("stored body")
        .expect("complete genesis");
    assert_eq!(
        hex(body.transactions[0].outputs[0].script_pubkey.as_bytes()),
        expected["output_script_hex"]
    );
    assert!(
        runtime
            .store
            .coins_view()
            .get_coin(&OutPoint {
                txid: open_bitcoin_core::consensus::transaction_txid(&body.transactions[0])
                    .expect("txid"),
                vout: 0
            })
            .expect("genesis coin lookup")
            .is_none()
    );
    assert!(
        runtime
            .store
            .coins_view()
            .complete_snapshot()
            .expect("coins")
            .coins
            .is_empty()
    );
    let progress = runtime
        .network_handle()
        .maybe_basic_index_progress()
        .expect("owner")
        .expect("accepted BASIC owner");
    assert_eq!(progress.maybe_processed_endpoint(), Some(identity));
    assert_eq!(progress.maybe_safe_durable_endpoint(), Some(identity));
    assert_eq!(progress.current_lag(), 0);
    assert_eq!(
        runtime
            .store
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint")
            .expect("durable genesis")
            .prefix(),
        open_bitcoin_core::chainstate::IndexPrefix::Committed(identity)
    );
}

#[test]
fn phase166_basic_enabled_uses_full_genesis_and_four_independent_vectors() {
    for (chain, network, name) in SELECTORS {
        // Arrange
        let source = source();
        let expected = vector(name);
        // Act
        let runtime = runtime(&source, chain, network, BasicFilterStartupMode::Enabled);
        // Assert
        assert_genesis_filter(&runtime, &expected);
        drop(runtime);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

#[test]
fn phase166_basic_disabled_and_preserve_absence_create_no_index_rows_or_owner() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::PreserveSaved,
    ] {
        for (chain, network, _) in SELECTORS {
            // Arrange
            let source = source();
            // Act
            let runtime = runtime(&source, chain, network, mode);
            // Assert
            assert!(
                runtime
                    .network_handle()
                    .maybe_basic_index_progress()
                    .expect("owner absence")
                    .is_none()
            );
            assert!(
                runtime
                    .store
                    .maybe_basic_filter_state()
                    .expect("state absence")
                    .is_none()
            );
            assert!(
                runtime
                    .store
                    .maybe_basic_filter_lifecycle_for_test()
                    .expect("saved absence")
                    .is_none()
            );
            assert!(
                runtime
                    .store
                    .maybe_basic_filter_checkpoint()
                    .expect("checkpoint absence")
                    .is_none()
            );
            assert_eq!(
                runtime
                    .store
                    .basic_filter_artifacts()
                    .expect("artifact absence"),
                (false, false)
            );
            assert!(
                !runtime
                    .store
                    .load_prune_locks()
                    .expect("locks")
                    .iter()
                    .any(|lock| lock.name == open_bitcoin_core::chainstate::BASIC_INDEX_PRUNE_LOCK)
            );
            assert!(
                runtime
                    .store
                    .database()
                    .keyspace(
                        StorageNamespace::BlockIndex.as_str(),
                        fjall::KeyspaceCreateOptions::default
                    )
                    .expect("existing namespace")
                    .get(filter_index::ownership::OWNER_KEY)
                    .expect("owner bytes absence")
                    .is_none()
            );
            drop(runtime);
            std::fs::remove_dir_all(source).expect("cleanup");
        }
    }
}

#[test]
fn phase166_basic_preserve_saved_restores_actual_accepted_owner_after_all_handle_reopen() {
    for (chain, network, name) in SELECTORS {
        // Arrange
        let source = source();
        let first = runtime(&source, chain, network, BasicFilterStartupMode::Enabled);
        assert_genesis_filter(&first, &vector(name));
        let before_progress = first
            .network_handle()
            .maybe_basic_index_progress()
            .expect("initial progress")
            .expect("initial owner");
        drop(first);
        // Act
        let reopened = runtime(
            &source,
            chain,
            network,
            BasicFilterStartupMode::PreserveSaved,
        );
        // Assert
        assert_genesis_filter(&reopened, &vector(name));
        assert_eq!(
            reopened
                .network_handle()
                .maybe_basic_index_progress()
                .expect("recovered progress")
                .expect("recovered owner")
                .generation(),
            before_progress.generation(),
            "PreserveSaved retains the saved Active generation"
        );
        drop(reopened);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

#[test]
fn phase166_basic_enabled_reopen_retains_exact_genesis_and_generation() {
    for (chain, network, name) in SELECTORS {
        // Arrange
        let source = source();
        let first = runtime(&source, chain, network, BasicFilterStartupMode::Enabled);
        let generation = first
            .network_handle()
            .maybe_basic_index_progress()
            .expect("progress")
            .expect("owner")
            .generation();
        drop(first);
        // Act
        let reopened = runtime(&source, chain, network, BasicFilterStartupMode::Enabled);
        // Assert
        assert_genesis_filter(&reopened, &vector(name));
        assert_eq!(
            reopened
                .network_handle()
                .maybe_basic_index_progress()
                .expect("progress")
                .expect("owner")
                .generation(),
            generation
        );
        drop(reopened);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

#[test]
fn phase166_basic_saved_disable_preserves_rows_and_reenable_uses_current_generation() {
    // Arrange
    let source = source();
    let first = runtime(
        &source,
        CanonicalChain::Regtest,
        SyncNetwork::Regtest,
        BasicFilterStartupMode::Enabled,
    );
    let first_progress = first
        .network_handle()
        .maybe_basic_index_progress()
        .expect("progress")
        .expect("owner");
    let record = first
        .store
        .load_basic_filter_record(first_progress.accepted_target().block_hash())
        .expect("record")
        .expect("genesis");
    drop(first);
    // Act
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::PreserveSaved,
    ] {
        let disabled = runtime(&source, CanonicalChain::Regtest, SyncNetwork::Regtest, mode);
        // Assert
        assert!(
            disabled
                .network_handle()
                .maybe_basic_index_progress()
                .expect("no accepted owner")
                .is_none()
        );
        assert_eq!(
            disabled
                .store
                .load_basic_filter_record(record.identity().block_hash())
                .expect("retained record"),
            Some(record.clone())
        );
        assert!(matches!(
            disabled
                .store
                .maybe_basic_filter_lifecycle_for_test()
                .expect("saved disabled"),
            Some(
                open_bitcoin_core::chainstate::filter_index::lifecycle::IndexLifecycle::Disabled { .. }
            )
        ));
        drop(disabled);
    }
    let enabled = runtime(
        &source,
        CanonicalChain::Regtest,
        SyncNetwork::Regtest,
        BasicFilterStartupMode::Enabled,
    );
    assert_genesis_filter(&enabled, &vector("regtest"));
    assert!(
        enabled
            .network_handle()
            .maybe_basic_index_progress()
            .expect("progress")
            .expect("owner")
            .generation()
            .value()
            > first_progress.generation().value()
    );
    drop(enabled);
    std::fs::remove_dir_all(source).expect("cleanup");
}

fn fault_reopen(stage: basic::Stage, faults: &[Fault]) {
    for &fault in faults {
        // Arrange
        let source = source();
        let opened = FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Regtest)
            .expect("proved core genesis");
        basic::MAYBE_FAULT.set(Some((stage, fault)));
        // Act
        let result = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                ..Default::default()
            },
            BasicFilterStartupMode::Enabled,
        );
        // Assert
        assert!(result.is_err(), "actual {fault:?} must return no runtime");
        assert!(
            basic::MAYBE_FAULT.get().is_none(),
            "selected injection stage reached"
        );
        let reopened = runtime(
            &source,
            CanonicalChain::Regtest,
            SyncNetwork::Regtest,
            BasicFilterStartupMode::Enabled,
        );
        assert_genesis_filter(&reopened, &vector("regtest"));
        drop(reopened);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

#[test]
fn phase166_basic_enable_faults_never_return_partial_runtime() {
    fault_reopen(
        basic::Stage::Configure,
        &[Fault::BeforeEnable, Fault::AfterEnable],
    );
}

#[test]
fn phase166_basic_append_faults_recover_only_complete_genesis() {
    fault_reopen(
        basic::Stage::Append,
        &[
            Fault::BeforeRecords,
            Fault::BeforeCheckpoint,
            Fault::BeforeProtection,
        ],
    );
}

#[test]
fn phase166_basic_syncall_faults_publish_no_accepted_partial_checkpoint() {
    fault_reopen(
        basic::Stage::Append,
        &[Fault::BeforeCommit, Fault::AfterCommit],
    );
}

#[test]
fn phase166_basic_saved_generation_change_fences_prepared_work() {
    // Arrange
    let source = source();
    let runtime = runtime(
        &source,
        CanonicalChain::Regtest,
        SyncNetwork::Regtest,
        BasicFilterStartupMode::Enabled,
    );
    let old = runtime
        .store
        .maybe_basic_filter_append_proof()
        .expect("old proof")
        .expect("active");
    let generation = old.generation();
    // Act
    runtime
        .network_handle()
        .disable_basic_filter_index()
        .expect("disable");
    runtime
        .network_handle()
        .enable_basic_filter_index()
        .expect("enable");
    // Assert
    assert_ne!(
        runtime
            .store
            .maybe_basic_filter_append_proof()
            .expect("new proof")
            .expect("active")
            .generation(),
        generation
    );
    assert!(
        runtime.store.check_basic_filter_append_proof(&old).is_err(),
        "old checkpoint generation cannot publish"
    );
    assert!(
        runtime
            .network_handle()
            .drive_basic_filter_index_turn()
            .is_err(),
        "stale ordered owner refuses current generation"
    );
    drop(old);
    drop(runtime);
    let reopened = runtime_for_regtest(&source);
    assert_genesis_filter(&reopened, &vector("regtest"));
    drop(reopened);
    std::fs::remove_dir_all(source).expect("cleanup");
}

fn runtime_for_regtest(source: &Path) -> DurableSyncRuntime {
    runtime(
        source,
        CanonicalChain::Regtest,
        SyncNetwork::Regtest,
        BasicFilterStartupMode::Enabled,
    )
}
