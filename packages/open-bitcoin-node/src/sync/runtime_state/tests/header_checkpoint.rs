// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/headerssync.cpp

use super::*;
use crate::chainstate::FlushPersistSink;
use crate::storage::{StorageError, StorageNamespace};
use crate::{BasicFilterBranchLimit, BasicFilterBranchOutcome, SyncNetwork};
use open_bitcoin_core::chainstate::{
    AnchoredBlock, FlushMode, FlushPolicyTime, PruneMode, PrunePlan,
};
use open_bitcoin_mempool::{PolicyTime, ReorgLifecycleContext};
use open_bitcoin_network::policy::{BranchWorkLimit, FilterRequest, FilterRequestInvalid};

mod faults;
use faults::{settle, snapshots, stale_runtime};

fn runtime(name: &str) -> (std::path::PathBuf, DurableSyncRuntime) {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let unique = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let (dir, runtime) = super::runtime(&format!("{name}-{unique}"));
    runtime
        .network_handle()
        .adopt_live_fixture_owner(open_bitcoin_core::chainstate::Chainstate::new())
        .expect("genuine empty fixture source");
    (dir, runtime)
}

fn live_source(
    store: crate::FjallNodeStore,
    blocks: &[open_bitcoin_core::primitives::Block],
) -> crate::ManagedChainstate<crate::FjallChainstateStore, crate::FjallCoinsView> {
    use crate::chainstate::FlushLifecycle;
    use open_bitcoin_core::chainstate::Chainstate;
    let mut core = Chainstate::from_parent(store.coins_view(), vec![], Default::default(), None);
    Chainstate::new()
        .rebind_into(&mut core)
        .expect("genuine empty transfer");
    let now = FlushPolicyTime::new(0);
    let mut source = crate::ManagedChainstate::from_chainstate(
        crate::FjallChainstateStore::from_store(store.clone()),
        core,
        FlushLifecycle::ready(now, now, 0, false),
    );
    for block in blocks {
        source
            .connect_block(
                block,
                open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                ScriptVerifyFlags::P2SH,
                SyncNetwork::Regtest.consensus_params(),
            )
            .expect("actual normal-maturity prefix acceptance");
        store
            .save_block(block, crate::PersistMode::Sync)
            .expect("actual retained prefix body");
    }
    source
        .flush_with_mode(FlushMode::Always, now, u64::MAX)
        .expect("same genuine prefix flush");
    source
}

#[test]
fn phase162_header_checkpoint_due_periodic_manual_and_automatic_caller_wiring() {
    // Arrange
    for caller in ["periodic", "manual", "automatic"] {
        let (dir, runtime, stale) = stale_runtime();
        let mut handle = runtime.network_handle();
        let before = runtime.store().snapshot_attempts();
        // Act: the short fixture has no legal retention candidate; these test dispatch only.
        let execution = match caller {
            "periodic" => {
                handle
                    .set_coins_next_write(FlushPolicyTime::new(0))
                    .expect("due");
                handle.flush_coins(FlushMode::Periodic, FlushPolicyTime::new(30_000), u64::MAX)
            }
            "manual" => handle.flush_applying_prune_plan(
                FlushMode::Always,
                FlushPolicyTime::new(30_000),
                u64::MAX,
                &PrunePlan::default(),
                &[],
            ),
            _ => {
                handle
                    .set_prune_mode(PruneMode::Automatic { target_mib: 550 })
                    .expect("automatic");
                handle
                    .set_prune_network(SyncNetwork::Regtest)
                    .expect("threshold");
                handle.flush_coins(FlushMode::Always, FlushPolicyTime::new(30_000), u64::MAX)
            }
        }
        .expect("shared checkpoint");
        // Assert
        assert!(execution.wrote_coins);
        assert!(execution.deleted_block_hashes.is_empty());
        assert_eq!(
            runtime.store().snapshot_attempts(),
            before.map(|value| value + 1)
        );
        assert!(
            runtime
                .store()
                .load_header_store()
                .expect("snapshot")
                .expect("saved")
                .entry(&stale)
                .is_some()
        );
        drop(handle);
        drop(runtime);
        std::fs::remove_dir_all(dir).expect("cleanup");
    }
}

#[test]
fn phase162_header_checkpoint_admitted_header_ahead_and_successor_survive_without_scripts_authority()
 {
    // Arrange
    let (dir, runtime) = runtime("phase162-header-ahead");
    let genesis = block(BlockHash::default(), 0, 0x51);
    headers(&runtime, vec![genesis.header.clone()]).expect("admitted PoW");
    // Act
    let handle = runtime.network_handle();
    handle
        .connect_local_block(
            &genesis,
            ScriptVerifyFlags::P2SH,
            SyncNetwork::Regtest.consensus_params(),
        )
        .expect("genuine genesis acceptance");
    settle(&runtime).expect("normal accepted header checkpoint");
    let child = block(block_hash(&genesis.header), 1, 0x51);
    headers(&runtime, vec![child.header.clone()]).expect("actual successor header before body");
    handle
        .connect_local_block(
            &child,
            ScriptVerifyFlags::P2SH,
            SyncNetwork::Regtest.consensus_params(),
        )
        .expect("successor after saved snapshot");
    let ahead = block(block_hash(&child.header), 2, 0x51);
    let ahead_hash = block_hash(&ahead.header);
    headers(&runtime, vec![ahead.header]).expect("admitted ahead");
    settle(&runtime).expect("ordinary successor settlement");
    // Assert
    assert_eq!(
        runtime
            .store()
            .validation_provenance(ahead_hash)
            .expect("header only"),
        ValidationProvenance::NeverConnected
    );
    drop(handle);
    drop(runtime);
    let reopened = crate::FjallNodeStore::open(&dir).expect("exclusive reopen");
    assert!(
        reopened
            .load_header_store()
            .expect("headers")
            .expect("snapshot")
            .entry(&ahead_hash)
            .is_some()
    );
    assert_eq!(
        reopened
            .validation_provenance(ahead_hash)
            .expect("not scripts"),
        ValidationProvenance::NeverConnected
    );
    assert_eq!(
        reopened.coins_view().best_block().expect("B"),
        Some(block_hash(&child.header))
    );
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase162_header_checkpoint_failed_admission_is_typed_and_never_reaches_writer() {
    // Arrange
    let (dir, runtime) = runtime("phase162-unadmitted");
    runtime
        .store()
        .set_validation_history_fault(HistoryPublicationFault::BeforeCommit);
    let genesis = block(BlockHash::default(), 0, 0x51);
    assert!(headers(&runtime, vec![genesis.header]).is_err());
    let live = runtime
        .network_handle()
        .peer_manager_snapshot()
        .expect("live rejected publication");
    let snapshot = crate::network::validation_history::TrustedHeaderSnapshot::maybe_capture(
        runtime.store().clone(),
        live.header_store(),
    )
    .expect("live prefix");
    // Act
    let error = snapshot.persist().expect_err("unadmitted cannot publish");
    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::BlockIndex,
            ..
        }
    ));
    assert_eq!(runtime.store().snapshot_attempts(), [0, 1, 1, 0]);
    assert!(
        runtime
            .store()
            .load_header_entries()
            .expect("none")
            .is_none()
    );
    assert!(settle(&runtime).is_err());
    drop(snapshot);
    drop(runtime);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase162_header_checkpoint_empty_and_legacy_keep_coverage_and_saved_knowledge() {
    // Arrange
    let (dir, runtime) = runtime("phase162-empty-legacy");
    runtime
        .store()
        .set_validation_history_fault(HistoryPublicationFault::SnapshotBeforeCommitFailure);
    // Act
    assert!(
        settle(&runtime).is_err(),
        "existing empty-coins refusal remains unchanged"
    );
    // Assert
    assert_eq!(runtime.store().snapshot_attempts(), [1, 1, 0, 0]);
    assert!(
        runtime
            .store()
            .load_header_entries()
            .expect("still no snapshot")
            .is_none()
    );
    let genesis = block(BlockHash::default(), 0, 0x51);
    runtime
        .network_handle()
        .connect_local_block(
            &genesis,
            ScriptVerifyFlags::P2SH,
            SyncNetwork::Regtest.consensus_params(),
        )
        .expect("genuine accepted header");
    assert!(
        settle(&runtime).is_err(),
        "empty capture did not consume fault"
    );
    settle(&runtime).expect("retry");
    let before = snapshots(runtime.store());
    runtime
        .store()
        .invalidate_validation_coverage()
        .expect("legacy control");
    settle(&runtime).expect("legacy remains persistable");
    assert_eq!(snapshots(runtime.store()), before);
    assert!(
        !runtime
            .store()
            .recovered_validation_history()
            .expect("legacy")
            .is_complete()
            .expect("coverage")
    );
    drop(runtime);
    let reopened = crate::FjallNodeStore::open(&dir).expect("exclusive reopen");
    assert_eq!(snapshots(&reopened), before);
    assert!(
        !reopened
            .recovered_validation_history()
            .expect("legacy cold")
            .is_complete()
            .expect("coverage")
    );
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase162_header_checkpoint_unsupported_memory_store_preserves_existing_flush() {
    // Arrange
    let network = crate::ManagedPeerNetwork::new(
        crate::MemoryChainstateStore::default(),
        open_bitcoin_network::LocalPeerConfig::default(),
        open_bitcoin_mempool::PolicyConfig::default(),
    );
    let handle = crate::ManagedNetworkHandle::from_network_fixture(network);
    // Act / Assert
    assert!(
        handle
            .flush_coins(FlushMode::Always, FlushPolicyTime::new(0), u64::MAX)
            .expect("unsupported snapshot store is no-op")
            .wrote_coins
    );
}

#[test]
fn phase162_header_checkpoint_selected_pressure_uses_real_durable_handle() {
    // Arrange: normal initialization supplies pressure; no runtime or authority is reacquired.
    let (dir, initial) = runtime("phase162-selected-pressure");
    drop(initial);
    let store = crate::FjallNodeStore::open(&dir).expect("exclusive pressure open");
    let now = FlushPolicyTime::new(0);
    let (lifecycle, view, cache) = crate::chainstate::initialize(
        &store,
        now,
        FlushPolicyTime::new(u64::MAX),
        0,
        true,
        u64::MAX,
    )
    .expect("pressure initialization");
    drop(view);
    let mut chainstate = open_bitcoin_core::chainstate::Chainstate::from_coins_cache(
        cache,
        Vec::new(),
        Default::default(),
        None,
    );
    open_bitcoin_core::chainstate::Chainstate::new()
        .rebind_into(&mut chainstate)
        .expect("genuine empty pressure source");
    let managed = crate::ManagedChainstate::from_recovered_chainstate(
        crate::FjallChainstateStore::from_store(store.clone()),
        chainstate,
        lifecycle,
    )
    .expect("recovered manager");
    let network = crate::ManagedPeerNetwork::from_initialized_chainstate(
        managed,
        open_bitcoin_network::LocalPeerConfig::default(),
        open_bitcoin_mempool::PolicyConfig::default(),
        16,
        Default::default(),
        Default::default(),
        false,
    );
    let handle = crate::ManagedNetworkHandle::from_network_fixture(network);
    let genesis = block(BlockHash::default(), 0, 0x51);
    handle
        .connect_local_block(
            &genesis,
            ScriptVerifyFlags::P2SH,
            SyncNetwork::Regtest.consensus_params(),
        )
        .expect("genuine pressure acceptance");
    let before = store.snapshot_attempts();
    // Act
    let execution = handle
        .flush_coins(FlushMode::IfNeeded, now, u64::MAX)
        .expect("selected pressure flush");
    // Assert
    assert!(execution.wrote_coins);
    assert_eq!(store.snapshot_attempts(), before.map(|value| value + 1));
    assert!(
        store
            .load_header_store()
            .expect("snapshot")
            .expect("saved")
            .entry(&block_hash(&genesis.header))
            .is_some()
    );
    assert_eq!(
        store
            .validation_provenance(block_hash(&genesis.header))
            .expect("accepted"),
        ValidationProvenance::ScriptsValid
    );
    drop(handle);
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase162_header_checkpoint_old_missing_stale_header_remains_refused_without_backfill() {
    // Arrange: model a pre-fix store after genuine acceptance and normal settlement.
    let (dir, runtime, stale) = stale_runtime();
    settle(&runtime).expect("settled active metadata");
    runtime
        .store()
        .remove_header_snapshot_for_test()
        .expect("old missing snapshot control");
    drop(runtime);
    // Act
    let reopened = DurableSyncRuntime::open(
        crate::FjallNodeStore::open(&dir).expect("exclusive"),
        crate::SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
    )
    .expect("active metadata fallback");
    let handle = reopened.network_handle();
    handle.configure_basic_peer_serving(true).expect("BASIC");
    handle.connect_outbound_peer(2, 10_000).expect("peer");
    let capture = handle
        .capture_basic_filter_branch(
            2,
            FilterRequest::Filters {
                filter_type: 0,
                start_height: 1,
                stop_hash: stale,
            },
            SyncNetwork::Regtest.consensus_params(),
            BasicFilterBranchLimit {
                branch: BranchWorkLimit {
                    ancestry_steps: 16,
                    identity_bytes: 4096,
                },
                metadata_units: 1024,
            },
        )
        .expect("capture");
    // Assert
    assert_eq!(
        reopened
            .store()
            .validation_provenance(stale)
            .expect("accepted ledger survives"),
        ValidationProvenance::ScriptsValid
    );
    assert_eq!(
        capture,
        BasicFilterBranchOutcome::Invalid(FilterRequestInvalid::UnknownOrDisallowedStop)
    );
    assert!(
        handle
            .peer_manager_snapshot()
            .expect("headers")
            .header_store()
            .entry(&stale)
            .is_none()
    );
    assert!(
        reopened
            .store()
            .load_header_entries()
            .expect("no repair")
            .is_none()
    );
    drop(handle);
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase162_header_checkpoint_reopen() {
    let params = SyncNetwork::Regtest.consensus_params();
    let request_limit = BasicFilterBranchLimit {
        branch: BranchWorkLimit {
            ancestry_steps: 16,
            identity_bytes: 4096,
        },
        metadata_units: 1024,
    };
    // Arrange
    let (dir, runtime, stale_hash) = stale_runtime();
    let handle = runtime.network_handle();
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: stale_hash,
    };

    // Act
    handle
        .flush_coins(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(20_002),
            u64::MAX,
        )
        .expect("replacement durable coins fence");
    let live = handle.peer_manager_snapshot().expect("live headers");
    let live_headers = live.header_store();
    let best = live_headers
        .maybe_best_proof_header()
        .expect("derived best proof");
    let stale = live_headers
        .maybe_proof_facts(stale_hash)
        .expect("derived stale proof");
    let exact_age = open_bitcoin_core::consensus::chain_proof::proof_equivalent_time(
        best.cumulative,
        stale.cumulative,
        best.block_proof,
        params.pow_target_spacing_seconds,
    )
    .expect("exact signed proof age");
    let timestamp_age = i64::from(best.timestamp) - i64::from(stale.timestamp);
    let live_capture = handle
        .capture_basic_filter_branch(2, request, params, request_limit)
        .expect("live capture");

    assert!(matches!(
        live_capture,
        BasicFilterBranchOutcome::Authorized(_)
    ));
    assert!(timestamp_age < 2_592_000 && exact_age < 2_592_000);

    let maybe_saved = runtime
        .store()
        .load_header_store()
        .expect("saved header snapshot");

    assert!(
        maybe_saved
            .expect("normal snapshot")
            .entry(&stale_hash)
            .is_some()
    );
    drop(handle);
    drop(runtime);
    let reopened = DurableSyncRuntime::open(
        crate::FjallNodeStore::open(&dir).expect("all-handle store reopen"),
        crate::SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
    )
    .expect("cold runtime");
    let handle = reopened.network_handle();
    assert!(
        handle
            .best_chain_entries()
            .expect("unproved cold import")
            .is_empty()
    );
    handle
        .try_admit_fixture_headers(
            open_bitcoin_network::HeaderStore::from_entries(
                handle.header_entries().expect("raw saved bytes"),
            )
            .expect("complete actual ancestry"),
            i64::MAX,
            params,
        )
        .expect("explicit model readmission; cold reopening alone grants no proof");
    handle
        .configure_basic_peer_serving(true)
        .expect("explicit cold capability");
    handle
        .connect_outbound_peer(2, 10_000)
        .expect("cold advertising peer");
    let cold = handle.peer_manager_snapshot().expect("cold headers");
    let cold_headers = cold.header_store();
    let cold_capture = handle
        .capture_basic_filter_branch(2, request, params, request_limit)
        .expect("cold capture");
    let provenance = reopened
        .store()
        .validation_provenance(stale_hash)
        .expect("cold provenance");

    // Assert: ordinary settlement retains exact admitted stale proof and provenance.
    assert_eq!(provenance, ValidationProvenance::ScriptsValid);
    assert!(cold_headers.entry(&stale_hash).is_some());
    assert_eq!(cold_headers.maybe_proof_facts(stale_hash), Some(stale));
    assert_eq!(cold_headers.maybe_best_proof_header(), Some(best));
    assert_eq!(timestamp_age, 100);
    assert_eq!(exact_age, 600);
    assert!(matches!(
        cold_capture,
        BasicFilterBranchOutcome::Authorized(_)
    ));
    drop(handle);
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("all handles dropped before cleanup");
}
