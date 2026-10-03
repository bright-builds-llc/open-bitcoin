// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/rpc/transactions.cpp
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

pub(super) fn saved_job(store: &FjallNodeStore) -> WalletRescanJob {
    store
        .load_wallet_rescan_job("alpha")
        .expect("load")
        .expect("job")
}

pub(super) fn cleanup(data_dir: PathBuf, context: ManagedRpcContext, store: FjallNodeStore) {
    drop(context);
    drop(store);
    fs::remove_dir_all(data_dir).expect("cleanup");
}

pub(super) fn fixture(label: &str) -> (PathBuf, FjallNodeStore, ChainstateSnapshot) {
    let data_dir = test_data_dir(label);
    let store = FjallNodeStore::open(&data_dir).expect("store");
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor("receive", DescriptorRole::External, DESCRIPTOR)
        .expect("descriptor");
    let script = wallet
        .default_receive_address()
        .expect("receive")
        .script_pubkey;
    let positions = (0..=3)
        .map(|height| {
            ChainPosition::new(
                BlockHeader {
                    version: 1,
                    time: 1_700_000_000 + height,
                    nonce: height,
                    bits: 0x207f_ffff,
                    ..BlockHeader::default()
                },
                height,
                u128::from(height) + 1,
                1_700_000_000 + i64::from(height),
            )
        })
        .collect();
    let coins = [(1, 25_000), (3, 35_000)]
        .into_iter()
        .map(|(height, value)| {
            (
                OutPoint {
                    txid: Txid::from_byte_array([height as u8; 32]),
                    vout: 0,
                },
                Coin {
                    output: TransactionOutput {
                        value: Amount::from_sats(value).expect("amount"),
                        script_pubkey: script.clone(),
                    },
                    is_coinbase: false,
                    created_height: height,
                    created_median_time_past: 1_700_000_000 + i64::from(height),
                },
            )
        })
        .collect();
    let truth = ChainstateSnapshot::new(positions, coins, Default::default());
    store.seed_coins_from_snapshot(&truth).expect("coins");
    for position in &truth.active_chain {
        store
            .save_block(
                &Block {
                    header: position.header.clone(),
                    transactions: Vec::new(),
                },
                PersistMode::Sync,
            )
            .expect("block");
        store
            .save_undo(
                position.block_hash,
                &BlockUndo::default(),
                PersistMode::Sync,
            )
            .expect("undo");
    }
    let prior = crate::context::rescan::partial_chainstate_snapshot(&truth, 1);
    wallet.rescan_chainstate(&prior).expect("fund prior wallet");
    WalletRegistry::default()
        .create_wallet(&store, "alpha", wallet, PersistMode::Sync)
        .expect("wallet");
    (data_dir, store, truth)
}

pub(super) fn context(store: &FjallNodeStore, data_dir: &std::path::Path) -> ManagedRpcContext {
    let mut context = ManagedRpcContext::from_runtime_config_with_store(
        &RuntimeConfig {
            chain: AddressNetwork::Regtest,
            maybe_data_dir: Some(data_dir.to_path_buf()),
            ..RuntimeConfig::default()
        },
        Some(store.clone()),
    )
    .expect("context");
    context.set_request_wallet_name(Some("alpha".to_string()));
    assert!(matches!(
        context.wallet_state,
        crate::context::wallet_state::WalletState::DurableNamedRegistry { .. }
    ));
    assert_eq!(
        context.blockchain_snapshot().expect("live authority").utxos,
        store
            .wallet_scan_chainstate_snapshot()
            .expect("durable")
            .expect("coins")
            .utxos
    );
    context
}

pub(super) fn missing_stop_context(
    store: &FjallNodeStore,
    data_dir: &std::path::Path,
    truth: &ChainstateSnapshot,
) -> ManagedRpcContext {
    let mut rpc = context(store, data_dir);
    let mut missing = truth.clone();
    missing.active_chain.retain(|position| position.height != 2);
    rpc.network = open_bitcoin_node::ManagedNetworkHandle::from_network_fixture(
        open_bitcoin_node::ManagedPeerNetwork::new(
            open_bitcoin_node::MemoryChainstateStore::from_snapshot(missing),
            open_bitcoin_network::LocalPeerConfig::default(),
            Default::default(),
        ),
    );
    assert!(
        !rpc.blockchain_snapshot()
            .expect("authority")
            .active_chain
            .iter()
            .any(|position| position.height == 2)
    );
    rpc
}

pub(super) fn saved_wallet(store: &FjallNodeStore) -> WalletSnapshot {
    WalletRegistry::load(store)
        .expect("registry")
        .wallet_snapshot("alpha")
        .expect("wallet")
        .clone()
}

pub(super) fn prune(store: &FjallNodeStore, truth: &ChainstateSnapshot, height: u32) {
    let hash = truth.active_chain[height as usize].block_hash;
    assert!(store.has_block(hash).expect("present block"));
    assert!(store.has_undo(hash).expect("present undo"));
    store
        .clone()
        .commit_paired_unlink(height, hash)
        .expect("paired unlink");
    assert!(!store.has_block(hash).expect("block absent"));
    assert!(!store.has_undo(hash).expect("undo absent"));
    assert!(store.load_have_pruned().expect("marker"));
    let authority = store
        .wallet_scan_chainstate_snapshot()
        .expect("authority")
        .expect("coins");
    assert_eq!(authority.utxos, truth.utxos);
    assert_eq!(authority.tip(), truth.tip());
    assert_eq!(
        FjallCoinsView::from_store(store)
            .best_block()
            .expect("best block"),
        truth.tip().map(|tip| tip.block_hash)
    );
    for position in truth
        .active_chain
        .iter()
        .filter(|position| position.height != height)
    {
        assert!(
            store
                .has_block(position.block_hash)
                .expect("retained block")
        );
        assert!(store.has_undo(position.block_hash).expect("retained undo"));
    }
}

pub(super) fn assert_refusal(
    store: &FjallNodeStore,
    before: &WalletSnapshot,
    boundary: &str,
    height: u32,
) {
    assert_eq!(saved_wallet(store), *before);
    let job = saved_job(store);
    assert_eq!(job.state, WalletRescanJobState::Failed);
    assert_eq!(job.maybe_scanned_through_height, before.maybe_tip_height);
    assert_eq!(
        job.maybe_tip_median_time_past,
        before.maybe_tip_median_time_past
    );
    let detail = job.maybe_error.expect("detail");
    assert!(
        detail.contains(boundary) && detail.contains(&format!("height {height}")),
        "{detail}"
    );
}

pub(super) fn poison(truth: &ChainstateSnapshot) -> ChainstateSnapshot {
    let mut poison = truth.clone();
    poison.active_chain.last_mut().expect("tip").height = 99;
    for coin in poison.utxos.values_mut() {
        coin.created_height = 99;
        coin.output.value = Amount::from_sats(999_999).expect("amount");
    }
    poison
}

pub(super) fn backend_error(namespace: StorageNamespace) -> StorageError {
    StorageError::BackendFailure {
        namespace,
        message: "/private/datadir/credential=secret".to_string(),
        action: StorageRecoveryAction::Reindex,
    }
}

pub(super) fn save_failure(
    registry: &mut WalletRegistry,
    store: &FjallNodeStore,
    job: WalletRescanJob,
) -> Result<(), WalletRegistryError> {
    registry.save_rescan_job(store, job, PersistMode::Sync)
}

pub(super) fn authority_failure_case(label: &str, error: StorageError, category: &str) {
    let (data_dir, store, truth) = fixture(label);
    let before = saved_wallet(&store);
    let mut pending =
        WalletRescanJob::new("alpha", truth.tip().expect("tip").block_hash, 3, 2, Some(1))
            .expect("job");
    pending.maybe_tip_median_time_past = before.maybe_tip_median_time_past;
    WalletRegistry::load(&store)
        .expect("registry")
        .save_rescan_job(&store, pending.clone(), PersistMode::Sync)
        .expect("pending");
    let mut context = context(&store, &data_dir);
    let result = context.rescan_wallet_range_with(
        Some(3),
        Some(3),
        |_| Err(WalletRescanEligibilityFailure::authority(error.into())),
        prepare_durable_wallet_rescan,
        save_failure,
    );
    let rpc_error = result.expect_err("authority refused");
    let failed = saved_job(&store);
    assert_eq!(failed.state, WalletRescanJobState::Failed);
    assert_eq!(failed.next_height, pending.next_height);
    assert_eq!(
        failed.maybe_scanned_through_height,
        pending.maybe_scanned_through_height
    );
    assert_eq!(
        failed.maybe_tip_median_time_past,
        pending.maybe_tip_median_time_past
    );
    assert_eq!(failed.target_tip_hash, pending.target_tip_hash);
    assert_eq!(failed.target_tip_height, pending.target_tip_height);
    assert_eq!(failed.freshness, pending.freshness);
    let detail = failed.maybe_error.expect("detail");
    assert!(
        detail.contains("authority") && detail.contains(category),
        "{detail}"
    );
    assert!(!detail.contains("private") && !detail.contains("secret"));
    assert!(
        rpc_error
            .maybe_detail
            .expect("RPC category")
            .message
            .contains(category)
    );
    assert_eq!(saved_wallet(&store), before);
    drop(context);
    drop(store);
    let reopened = FjallNodeStore::open(&data_dir).expect("reopen");
    assert_eq!(saved_wallet(&reopened), before);
    assert_eq!(saved_job(&reopened).state, WalletRescanJobState::Failed);
    drop(reopened);
    fs::remove_dir_all(data_dir).expect("cleanup");
}
