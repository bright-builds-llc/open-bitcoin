// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::{OpenedAuthoritativeRuntime, open_runtime_store};
use open_bitcoin_node::core::chainstate::PruneMode;

#[path = "automatic_prune/fixtures.rs"]
mod fixtures;
#[path = "automatic_prune/index_protection.rs"]
mod index_protection;
use crate::coins_flush::flush_cycle;
use fixtures::*;
use open_bitcoin_node::core::chainstate::{FlushMode, FlushPolicyTime, PruneLockInfo};
use open_bitcoin_node::{
    PersistMode, WalletRegistry, WalletRescanJob, WalletRescanJobState, WalletRescanRuntime,
};

#[test]
fn automatic_prune_offline_explicit_modes_select_durable_authority() {
    for mode in [
        PruneMode::ManualOnly,
        PruneMode::Automatic { target_mib: 550 },
    ] {
        // Arrange
        let data_dir = temp_store_path("offline-prune");
        let runtime = RuntimeConfig {
            maybe_data_dir: Some(data_dir.clone()),
            prune_mode: mode,
            ..RuntimeConfig::default()
        };
        assert!(!runtime.sync.is_enabled());
        assert!(!runtime.inbound.enabled);

        // Act
        let maybe_store = open_runtime_store(&runtime).expect("open store");
        assert!(
            maybe_store.is_some(),
            "explicit pruning needs durable authority"
        );
        let opened = open_authoritative_network_runtime(&runtime, maybe_store)
            .expect("recovered durable runtime");

        // Assert
        assert!(matches!(opened, OpenedAuthoritativeRuntime::Durable(_)));
        drop(opened);
        remove_dir_if_exists(&data_dir);
    }
}

#[test]
fn automatic_prune_offline_disabled_preserves_transient_selection() {
    // Arrange
    let runtime = RuntimeConfig::default();

    // Act
    let maybe_store = open_runtime_store(&runtime).expect("default store selection");
    let opened =
        open_authoritative_network_runtime(&runtime, maybe_store).expect("default runtime");

    // Assert
    assert!(matches!(opened, OpenedAuthoritativeRuntime::Transient(_)));
}

#[tokio::test]
async fn automatic_prune_offline_keeps_sync_and_listener_workers_disabled() {
    // Arrange
    let temp = TempDir::new("offline-workers");
    let runtime = config(&temp);
    let maybe_store = open_runtime_store(&runtime).expect("store");
    let mut opened = open_authoritative_network_runtime(&runtime, maybe_store.clone())
        .expect("owner")
        .expect_durable();
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &runtime,
        opened.network.clone(),
        maybe_store,
    )
    .expect("context");
    let shared = Arc::new(tokio::sync::Mutex::new(context));

    // Act
    let maybe_sync_worker = crate::start_daemon_sync_worker(
        &runtime,
        Arc::clone(&shared),
        opened.maybe_sync_runtime.take(),
    )
    .expect("sync selection");
    let listener = start_inbound_listener_for_runtime_with_context(&runtime, shared).await;

    // Assert
    assert!(maybe_sync_worker.is_none());
    assert_eq!(listener.state, InboundListenerState::Disabled);
    assert!(listener.bound_endpoints.is_empty());
    assert!(listener.maybe_worker.is_none());
    assert_eq!(
        opened
            .network
            .prune_mode()
            .expect("mode installed before activity"),
        runtime.prune_mode
    );
}

#[test]
fn automatic_prune_offline_without_datadir_retains_transient_selection() {
    // Arrange
    let runtime = RuntimeConfig {
        prune_mode: PruneMode::ManualOnly,
        ..RuntimeConfig::default()
    };

    // Act
    let maybe_store = open_runtime_store(&runtime).expect("no datadir");
    let opened =
        open_authoritative_network_runtime(&runtime, maybe_store).expect("no implicit datadir");

    // Assert
    assert!(matches!(opened, OpenedAuthoritativeRuntime::Transient(_)));
}

#[test]
fn automatic_prune_genuine_ordinary_retention_and_reopen() {
    // Arrange: seed consistent sparse chain truth before opening the production owner.
    let started = std::time::Instant::now();
    let temp = TempDir::new("genuine-retention");
    let mut runtime = config(&temp);
    runtime.block_serving.block_serving.enabled = true;
    let store = open_runtime_store(&runtime)
        .expect("open")
        .expect("durable");
    let truth = seed(&store);
    let opened = open_authoritative_network_runtime(&runtime, Some(store.clone()))
        .expect("recovery before readiness")
        .expect_durable();
    let handle = opened.network.clone();
    handle
        .set_coins_next_write(FlushPolicyTime::new(10_000))
        .expect("future coins deadline");
    handle
        .replace_prune_lock(PruneLockInfo {
            name: "retained-wallet".into(),
            height_first: 520,
            height_last: 520,
        })
        .expect("committed lock and ten-block buffer");
    for p in &truth.active_chain {
        cache_small(&handle, p);
    }
    let mut rpc = ManagedRpcContext::from_runtime_config_with_network_handle(
        &runtime,
        handle.clone(),
        Some(store.clone()),
    )
    .expect("same authority RPC");
    rpc.set_request_wallet_name(Some("alpha".into()));
    rpc.rescan_wallet_range(Some(714), Some(714))
        .expect("retained control succeeds");
    let retained_tip = &truth.active_chain[4];
    let retained_job = WalletRescanJob::new(
        "alpha",
        retained_tip.block_hash,
        retained_tip.height,
        retained_tip.height,
        Some(714),
    )
    .expect("retained node job");
    WalletRegistry::load(&store)
        .expect("registry")
        .save_rescan_job(&store, retained_job, PersistMode::Sync)
        .expect("retained node checkpoint");
    let retained_node = WalletRescanRuntime::open(store.clone(), PersistMode::Sync)
        .expect("retained node control succeeds");
    assert_eq!(
        store
            .load_wallet_rescan_job("alpha")
            .expect("retained job")
            .expect("completed")
            .state,
        WalletRescanJobState::Complete
    );
    drop(retained_node);
    let before = saved_wallet(&store);
    let under_target = flush_cycle(
        &handle,
        FlushMode::Periodic,
        FlushPolicyTime::new(10),
        u64::MAX,
    )
    .expect("real under-target control");
    assert!(!under_target.wrote_coins);
    assert!(under_target.deleted_block_hashes.is_empty());
    assert!(!store.load_have_pruned().expect("unearned marker"));
    let initial_bytes = populate_threshold(&store, &truth);

    // Act: the daemon's ordinary Periodic cycle, without a manual prune call.
    let execution = flush_cycle(
        &handle,
        FlushMode::Periodic,
        FlushPolicyTime::new(100),
        u64::MAX,
    )
    .expect("ordinary automatic checkpoint");

    // Assert: actual paired deletes earn counters; protected/nonactive usage remains.
    assert!(
        execution.wrote_coins,
        "prune forces coins despite future deadline"
    );
    let expected: Vec<_> = [0, 1, 3]
        .into_iter()
        .map(|i| truth.active_chain[i].block_hash)
        .collect();
    assert_eq!(execution.deleted_block_hashes, expected);
    assert_deleted(&store, &handle, &truth, &[0, 1, 3]);
    for i in [2, 4, 5] {
        let hash = truth.active_chain[i].block_hash;
        assert!(store.has_block(hash).expect("protected body"));
        assert!(store.has_undo(hash).expect("protected undo"));
        assert!(handle.cached_block_present(hash).expect("protected cache"));
    }
    let retained = store
        .retained_payload_usage(&truth.active_chain)
        .expect("post usage");
    assert!(retained.current_usage_bytes > TARGET);
    assert!(retained.current_usage_bytes < initial_bytes);
    let counts = store.load_operator_support_counts().expect("earned counts");
    assert_eq!(counts.successful_batch_count, 1);
    assert_eq!(counts.pruned_height_count, 3);
    assert_eq!(counts.maybe_last_prune_height, Some(713));
    assert!(store.load_have_pruned().expect("earned marker"));
    assert_serving_and_operator(&mut rpc, &store, &handle, &truth);
    assert_wallet_refusals(&mut rpc, &store, &truth, &before);
    let repeated = flush_cycle(
        &handle,
        FlushMode::Periodic,
        FlushPolicyTime::new(160),
        u64::MAX,
    )
    .expect("repeat");
    assert!(repeated.deleted_block_hashes.is_empty());
    assert_eq!(
        store
            .load_operator_support_counts()
            .expect("no duplicate counts"),
        counts
    );
    let durable_truth = store
        .wallet_scan_chainstate_snapshot()
        .expect("checkpoint")
        .expect("coins truth");
    assert_eq!(durable_truth.active_chain, truth.active_chain);
    assert_eq!(durable_truth.utxos, truth.utxos);
    let mut poison = truth.clone();
    poison.active_chain.last_mut().expect("tip").height = 99;
    store
        .save_chainstate_snapshot(&poison, PersistMode::Sync)
        .expect("poison leftover is not authority");
    drop(rpc);
    drop(handle);
    drop(opened);

    assert_later_error_and_reopen(&runtime, store, &truth);
    eprintln!(
        "genuine retention completed: logical_before={initial_bytes} logical_after={} deletes=4 batches=2 duration={:?}",
        retained.current_usage_bytes,
        started.elapsed()
    );
}

fn assert_later_error_and_reopen(
    runtime: &RuntimeConfig,
    store: FjallNodeStore,
    truth: &open_bitcoin_node::core::chainstate::ChainstateSnapshot,
) {
    // Replenish one same-tip value, then reopen with real undo and cache before later error.
    save_small(&store, &truth.active_chain[0]);
    drop(store);
    let store = open_runtime_store(runtime)
        .expect("reopen store")
        .expect("store");
    let reopened = open_authoritative_network_runtime(runtime, Some(store.clone()))
        .expect("successful checkpoint reopens")
        .expect_durable();
    cache_small(&reopened.network, &truth.active_chain[0]);
    assert!(
        reopened
            .network
            .chainstate_snapshot()
            .expect("undo before error")
            .undo_by_block
            .contains_key(&truth.active_chain[0].block_hash)
    );
    drop(reopened);
    let (fault_owner, fail_metadata) = metadata_fault_owner(&store);
    cache_small(&fault_owner, &truth.active_chain[0]);
    let failure = flush_cycle(
        &fault_owner,
        FlushMode::Periodic,
        FlushPolicyTime::new(200),
        u64::MAX,
    );
    assert!(
        failure.is_err(),
        "injected metadata persistence failure after committed unlink"
    );
    assert_deleted(&store, &fault_owner, truth, &[0, 1, 3]);
    let after_error = store
        .load_operator_support_counts()
        .expect("committed delete counts");
    assert_eq!(after_error.successful_batch_count, 2);
    assert_eq!(after_error.pruned_height_count, 4);
    fail_metadata.store(false, std::sync::atomic::Ordering::SeqCst);
    flush_cycle(
        &fault_owner,
        FlushMode::Always,
        FlushPolicyTime::new(201),
        u64::MAX,
    )
    .expect("retry");
    assert_deleted(&store, &fault_owner, truth, &[0, 1, 3]);
    assert_eq!(
        store
            .load_operator_support_counts()
            .expect("retry no extra counts"),
        after_error
    );
    drop(fault_owner);
    drop(store);
    let store = open_runtime_store(runtime)
        .expect("final reopen")
        .expect("store");
    let reopened = open_authoritative_network_runtime(runtime, Some(store.clone()))
        .expect("recovered final owner")
        .expect_durable();
    assert_deleted(&store, &reopened.network, truth, &[0, 1, 3]);
    assert_eq!(
        store
            .wallet_scan_chainstate_snapshot()
            .expect("reopened metadata")
            .expect("truth")
            .active_chain,
        truth.active_chain
    );
}

fn assert_serving_and_operator(
    rpc: &mut ManagedRpcContext<
        open_bitcoin_node::FjallChainstateStore,
        open_bitcoin_node::FjallCoinsView,
    >,
    store: &FjallNodeStore,
    handle: &open_bitcoin_node::ManagedNetworkHandle<
        open_bitcoin_node::FjallChainstateStore,
        open_bitcoin_node::FjallCoinsView,
    >,
    truth: &open_bitcoin_node::core::chainstate::ChainstateSnapshot,
) {
    use open_bitcoin_network::InventoryList;
    use open_bitcoin_node::core::{
        consensus::{ConsensusParams, ScriptVerifyFlags},
        primitives::{BlockHash, InventoryType, InventoryVector},
    };
    use open_bitcoin_rpc::{
        dispatch::dispatch,
        method::{GetBlockchainInfoRequest, MethodCall, OpenBitcoinNetworkStatusRequest},
    };
    handle
        .clone()
        .set_serving_have_pruned(store.load_have_pruned().expect("actual earned marker"))
        .expect("serving projection");
    // This peer is in-memory; no transport or listener is activated.
    handle
        .connect_outbound_peer(153, 1)
        .expect("in-memory peer");
    for hash in [
        truth.active_chain[3].block_hash,
        BlockHash::from_byte_array([0xab; 32]),
    ] {
        let inventory = InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: hash.into(),
        }]);
        let outcome = handle
            .receive_message(
                153,
                WireNetworkMessage::GetData(inventory.clone()),
                2,
                ScriptVerifyFlags::NONE,
                ConsensusParams::default(),
            )
            .expect("serving refusal");
        assert_eq!(
            outcome.outbound,
            vec![WireNetworkMessage::NotFound(inventory)]
        );
    }
    let retained_hash = truth.active_chain[4].block_hash;
    let request = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: retained_hash.into(),
    }]);
    let response = handle
        .receive_message_for_durable_serving(
            153,
            WireNetworkMessage::GetData(request),
            3,
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
            |hash| store.has_block(hash).expect("actual durable presence"),
        )
        .expect("retained serving intent");
    let maybe_intent = response
        .inbound_response_plan
        .into_iter()
        .find_map(|item| match item {
            open_bitcoin_node::network::ManagedInboundResponsePlanItem::DurableBlock(intent) => {
                Some(intent)
            }
            _ => None,
        });
    let intent = maybe_intent.expect("retained body remains eligible");
    // An unrelated lookup failure uses the existing completion effect seam.
    handle
        .complete_block_serve(&intent.completion(
            open_bitcoin_node::network::ManagedBlockServeCompletionOutcome::LookupUnavailable,
        ))
        .expect("unavailable completion");
    let serving = handle
        .block_relay_evidence_status()
        .expect("serving labels")
        .block_serving
        .status;
    let FieldAvailability::Available(serving) = serving else {
        panic!("serving evidence")
    };
    assert_eq!(serving.pruned_count, 1);
    assert_eq!(serving.unavailable_count, 1);
    assert_eq!(serving.unknown_count, 1);
    let blockchain = dispatch(
        rpc,
        MethodCall::GetBlockchainInfo(GetBlockchainInfoRequest::default()),
    )
    .expect("blockchain projection");
    assert_eq!(blockchain["pruned"], true);
    assert_eq!(blockchain["automatic_pruning"], true);
    assert_eq!(blockchain["prune_target_size"], TARGET);
    let status = dispatch(
        rpc,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("operator status");
    assert_eq!(
        status["prune"]["support_counts"]["value"]["pruned_height_count"],
        3
    );
}

fn assert_deleted(
    store: &FjallNodeStore,
    handle: &open_bitcoin_node::ManagedNetworkHandle<
        impl open_bitcoin_node::ChainstateStore,
        impl open_bitcoin_node::core::chainstate::CoinsView,
    >,
    truth: &open_bitcoin_node::core::chainstate::ChainstateSnapshot,
    indices: &[usize],
) {
    let snapshot = handle.chainstate_snapshot().expect("memory snapshot");
    for &index in indices {
        let hash = truth.active_chain[index].block_hash;
        assert!(!store.has_block(hash).expect("deleted body"));
        assert!(store.load_block(hash).expect("serving lookup").is_none());
        assert!(!store.has_undo(hash).expect("deleted undo"));
        assert!(!handle.cached_block_present(hash).expect("deleted cache"));
        assert!(
            !snapshot.undo_by_block.contains_key(&hash),
            "forget deleted undo"
        );
    }
}

fn assert_wallet_refusals(
    rpc: &mut ManagedRpcContext<
        open_bitcoin_node::FjallChainstateStore,
        open_bitcoin_node::FjallCoinsView,
    >,
    store: &FjallNodeStore,
    truth: &open_bitcoin_node::core::chainstate::ChainstateSnapshot,
    before: &open_bitcoin_node::core::wallet::WalletSnapshot,
) {
    assert!(rpc.rescan_wallet_range(Some(1_001), Some(1_001)).is_err());
    assert_eq!(saved_wallet(store), *before);
    let failed = store
        .load_wallet_rescan_job("alpha")
        .expect("RPC job")
        .expect("failed job");
    assert_eq!(failed.state, WalletRescanJobState::Failed);
    assert!(
        failed
            .maybe_error
            .expect("safe failure detail")
            .contains("creating")
    );
    let tip = truth.tip().expect("tip");
    let job = WalletRescanJob::new(
        "alpha",
        tip.block_hash,
        tip.height,
        tip.height,
        before.maybe_tip_height,
    )
    .expect("node job");
    WalletRegistry::load(store)
        .expect("registry")
        .save_rescan_job(store, job.clone(), PersistMode::Sync)
        .expect("node checkpoint");
    let node = WalletRescanRuntime::open(store.clone(), PersistMode::Sync);
    assert!(
        node.is_err(),
        "node resume must recheck older creating payload"
    );
    assert_eq!(saved_wallet(store), *before);
    let failed = store
        .load_wallet_rescan_job("alpha")
        .expect("node job")
        .expect("failed");
    assert_eq!(failed.state, WalletRescanJobState::Failed);
    assert_eq!(failed.next_height, job.next_height);
    assert_eq!(
        failed.maybe_scanned_through_height,
        job.maybe_scanned_through_height
    );
    assert!(
        failed
            .maybe_error
            .expect("safe detail")
            .contains("creating")
    );
}
