// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/rpc/transactions.cpp
// - packages/bitcoin-knots/src/coins.cpp

use super::*;
use crate::storage::coins_codec::{
    encode_best_block_key, encode_head_blocks_key, encode_head_blocks_value,
};
use crate::storage::coins_view::FjallCoinsView;
use crate::wallet_registry::rescan::prepare_durable_wallet_rescan;
use crate::{StorageError, StorageNamespace, StorageRecoveryAction, WalletRescanJob};
use open_bitcoin_core::chainstate::{BlockUndo, CoinsView};

fn wallet_snapshot(store: &FjallNodeStore) -> open_bitcoin_core::wallet::WalletSnapshot {
    WalletRegistry::load(store)
        .expect("registry")
        .wallet_snapshot("alpha")
        .expect("wallet")
        .clone()
}

fn pending_job(store: &FjallNodeStore, truth: &ChainstateSnapshot) -> WalletRescanJob {
    let tip = truth.tip().expect("tip");
    let mut job =
        WalletRescanJob::new("alpha", tip.block_hash, tip.height, 2, Some(1)).expect("job");
    job.maybe_tip_median_time_past = Some(1_700_000_001);
    WalletRegistry::load(store)
        .expect("registry")
        .save_rescan_job(store, job.clone(), PersistMode::Sync)
        .expect("pending job");
    job
}

fn assert_preserved_checkpoint(before: &WalletRescanJob, after: &WalletRescanJob) {
    assert_eq!(after.state, WalletRescanJobState::Failed);
    assert_eq!(after.next_height, before.next_height);
    assert_eq!(
        after.maybe_scanned_through_height,
        before.maybe_scanned_through_height
    );
    assert_eq!(
        after.maybe_tip_median_time_past,
        before.maybe_tip_median_time_past
    );
    assert_eq!(after.target_tip_hash, before.target_tip_hash);
    assert_eq!(after.target_tip_height, before.target_tip_height);
    assert_eq!(after.freshness, before.freshness);
}

#[test]
fn post_prune_resume_rechecks_earlier_creating_payload_after_reopen() {
    // Arrange
    let (path, store, truth) = funded_store("post-prune-resume");
    let empty = wallet_with_ranged_descriptor();
    WalletRegistry::load(&store)
        .expect("registry")
        .save_wallet(&store, "alpha", &empty, PersistMode::Sync)
        .expect("reset prior");
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");
    let prior_job = runtime.enqueue_rescan("alpha").expect("first chunk");
    assert_eq!(prior_job.state, WalletRescanJobState::Scanning);
    assert_eq!(prior_job.maybe_scanned_through_height, Some(1));
    let before = wallet_snapshot(runtime.store());
    paired_prune(runtime.store(), &truth, 1);
    runtime
        .store()
        .save_chainstate_snapshot(&poison_leftover_snapshot(), PersistMode::Sync)
        .expect("poison leftover");
    drop(runtime);

    // Act
    let reopened = FjallNodeStore::open(&path).expect("reopen store");
    let result = WalletRescanRuntime::open_with_chunk_size(reopened, PersistMode::Sync, 2);

    // Assert
    assert!(result.is_err(), "automatic resume refuses");
    drop(result);
    let durable = FjallNodeStore::open(&path).expect("reopen durable failed evidence");
    assert_eq!(wallet_snapshot(&durable), before);
    let job = durable
        .load_wallet_rescan_job("alpha")
        .expect("load")
        .expect("job");
    assert_preserved_checkpoint(&prior_job, &job);
    let detail = job.maybe_error.expect("detail");
    assert!(
        detail.contains("creating") && detail.contains("height 1"),
        "{detail}"
    );
    assert_eq!(
        durable.load_chainstate_snapshot().expect("leftover"),
        Some(poison_leftover_snapshot())
    );
    assert!(durable.load_have_pruned().expect("marker"));
    assert!(
        !durable
            .has_block(truth.active_chain[1].block_hash)
            .expect("block")
    );
    assert!(
        !durable
            .has_undo(truth.active_chain[1].block_hash)
            .expect("undo")
    );
    assert_eq!(
        durable
            .wallet_scan_chainstate_snapshot()
            .expect("truth")
            .expect("coins")
            .utxos,
        truth.utxos
    );
    remove_dir_if_exists(&path);
}

#[test]
fn post_prune_in_range_missing_payload_preserves_wallet() {
    // Arrange
    let (path, store, truth) = funded_store("post-prune-in-range");
    let before = wallet_snapshot(&store);
    paired_prune(&store, &truth, 2);
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");
    // Act
    let result = runtime.enqueue_rescan("alpha");
    // Assert
    assert!(result.is_err());
    assert_eq!(wallet_snapshot(runtime.store()), before);
    let job = runtime
        .store()
        .load_wallet_rescan_job("alpha")
        .expect("load")
        .expect("job");
    assert_eq!(job.state, WalletRescanJobState::Failed);
    assert_eq!(job.next_height, 2);
    assert_eq!(job.maybe_scanned_through_height, Some(1));
    let detail = job.maybe_error.expect("detail");
    assert!(
        detail.contains("requested") && detail.contains("height 2"),
        "{detail}"
    );
    remove_dir_if_exists(&path);
}

#[test]
fn retained_creating_payload_midrange_rescan_succeeds() {
    // Arrange
    let (path, store, _) = funded_store("retained-midrange");
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");
    // Act
    let job = runtime
        .enqueue_rescan("alpha")
        .expect("retained creating payload");
    // Assert
    assert_eq!(job.state, WalletRescanJobState::Complete);
    assert_eq!(job.maybe_scanned_through_height, Some(3));
    let wallet = wallet_snapshot(runtime.store());
    assert_eq!(wallet.utxos.len(), 2);
    assert_eq!(wallet.maybe_tip_height, Some(3));
    assert_eq!(wallet.maybe_tip_median_time_past, Some(1_700_000_003));
    assert_eq!(
        wallet
            .utxos
            .iter()
            .map(|utxo| utxo.output.value.to_sats())
            .sum::<i64>(),
        60_000
    );
    remove_dir_if_exists(&path);
}

#[test]
fn unrelated_old_pruned_coin_does_not_block_node_rescan() {
    // Arrange
    let (path, store, mut truth) = funded_store("unrelated-pruned");
    for coin in truth
        .utxos
        .values_mut()
        .filter(|coin| coin.created_height == 1)
    {
        coin.output.script_pubkey = ScriptBuf::from_bytes(vec![0x51]).expect("unrelated script");
    }
    store
        .seed_coins_from_snapshot(&truth)
        .expect("unrelated durable coin");
    paired_prune(&store, &truth, 1);
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");
    // Act
    let job = runtime
        .enqueue_rescan("alpha")
        .expect("unrelated coin excluded");
    // Assert
    assert_eq!(job.state, WalletRescanJobState::Complete);
    let wallet = wallet_snapshot(runtime.store());
    assert_eq!(wallet.utxos.len(), 1);
    assert_eq!(wallet.utxos[0].created_height, 3);
    assert_eq!(wallet.utxos[0].output.value.to_sats(), 35_000);
    assert_eq!(
        runtime
            .store()
            .wallet_scan_chainstate_snapshot()
            .expect("authority")
            .expect("coins")
            .utxos
            .len(),
        2
    );
    remove_dir_if_exists(&path);
}

#[test]
fn interrupted_two_heads_refuses_loader_preparation_and_existing_job() {
    // Arrange
    let (path, store, truth) = funded_store("interrupted-wallet-authority");
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");
    let before = wallet_snapshot(runtime.store());
    let prior_job = pending_job(runtime.store(), &truth);
    runtime
        .store()
        .save_chainstate_snapshot(&poison_leftover_snapshot(), PersistMode::Sync)
        .expect("leftover");
    let view = FjallCoinsView::from_store(runtime.store());
    view.delete_raw_bytes(&encode_best_block_key())
        .expect("remove B");
    view.write_raw_bytes(
        &encode_head_blocks_key(),
        encode_head_blocks_value(&[
            truth.active_chain[3].block_hash,
            truth.active_chain[2].block_hash,
        ])
        .expect("heads"),
    )
    .expect("plant actual H");
    let interrupted = StorageError::InterruptedWrite {
        namespace: StorageNamespace::Coins,
        action: StorageRecoveryAction::Reindex,
    };
    // Act
    let loader = runtime.store().wallet_scan_chainstate_snapshot();
    let preparation = prepare_durable_wallet_rescan(
        runtime.store(),
        &Wallet::from_snapshot(before.clone()),
        &truth,
        2,
        3,
    );
    let advance = runtime.advance_wallet_rescan("alpha");
    // Assert
    assert_eq!(loader.expect_err("loader refused"), interrupted);
    assert_eq!(
        preparation.expect_err("preparation refused").error,
        crate::WalletRegistryError::Storage(interrupted.clone())
    );
    assert_eq!(
        advance.expect_err("advance refused"),
        crate::WalletRegistryError::Storage(interrupted)
    );
    assert_eq!(wallet_snapshot(runtime.store()), before);
    let job = runtime
        .store()
        .load_wallet_rescan_job("alpha")
        .expect("load")
        .expect("job");
    assert_preserved_checkpoint(&prior_job, &job);
    assert!(
        job.maybe_error
            .expect("detail")
            .contains("authority: InterruptedWrite")
    );
    assert_eq!(
        runtime
            .store()
            .load_chainstate_snapshot()
            .expect("poison remains"),
        Some(poison_leftover_snapshot())
    );
    drop(view);
    drop(runtime);
    let reopened = FjallNodeStore::open(&path).expect("reopen marker state");
    assert_eq!(wallet_snapshot(&reopened), before);
    assert_eq!(
        reopened
            .load_wallet_rescan_job("alpha")
            .expect("job")
            .expect("present")
            .state,
        WalletRescanJobState::Failed
    );
    remove_dir_if_exists(&path);
}

#[test]
fn probe_error_persists_failed_without_backend_path_and_preserves_checkpoint() {
    // Arrange
    let (path, store, truth) = funded_store("probe-error");
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");
    let before = wallet_snapshot(runtime.store());
    let prior_job = pending_job(runtime.store(), &truth);
    let error = StorageError::BackendFailure {
        namespace: StorageNamespace::BlockIndex,
        message: "/private/wallet/path".into(),
        action: StorageRecoveryAction::Repair,
    };
    // Act
    let result = runtime.advance_with_probe_and_failure_save(
        "alpha",
        |_| Err(error.clone()),
        |registry, job| registry.save_rescan_job(runtime.store(), job, PersistMode::Sync),
    );
    // Assert
    assert_eq!(
        result.expect_err("probe refused"),
        crate::WalletRegistryError::Storage(error)
    );
    assert_eq!(wallet_snapshot(runtime.store()), before);
    let job = runtime
        .store()
        .load_wallet_rescan_job("alpha")
        .expect("load")
        .expect("job");
    assert_preserved_checkpoint(&prior_job, &job);
    let detail = job.maybe_error.expect("safe detail");
    assert!(detail.contains("creating") && detail.contains("height 1") && detail.contains("hash"));
    assert!(!detail.contains("private"));
    remove_dir_if_exists(&path);
}

#[test]
fn failure_save_error_reaches_caller_without_wallet_write() {
    // Arrange
    let (path, store, truth) = funded_store("failure-save-error");
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");
    let before = wallet_snapshot(runtime.store());
    let prior_job = pending_job(runtime.store(), &truth);
    let save_error = crate::WalletRegistryError::Storage(StorageError::BackendFailure {
        namespace: StorageNamespace::Wallet,
        message: "failed job persistence".into(),
        action: StorageRecoveryAction::Repair,
    });
    // Act
    let result = runtime.advance_with_probe_and_failure_save(
        "alpha",
        |_| Ok(false),
        |_, _| Err(save_error.clone()),
    );
    // Assert
    assert_eq!(result.expect_err("save refusal visible"), save_error);
    assert_eq!(wallet_snapshot(runtime.store()), before);
    assert_eq!(
        runtime
            .store()
            .load_wallet_rescan_job("alpha")
            .expect("load")
            .expect("job"),
        prior_job
    );
    remove_dir_if_exists(&path);
}

#[test]
fn completed_and_failed_jobs_return_without_authority_reads() {
    // Arrange
    let (path, store, truth) = funded_store("inactive-job");
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");
    let mut job = pending_job(runtime.store(), &truth);
    let view = FjallCoinsView::from_store(runtime.store());
    view.delete_raw_bytes(&encode_best_block_key())
        .expect("remove B");
    view.write_raw_bytes(
        &encode_head_blocks_key(),
        encode_head_blocks_value(&[
            truth.active_chain[3].block_hash,
            truth.active_chain[2].block_hash,
        ])
        .expect("heads"),
    )
    .expect("plant H");
    // Act / Assert
    for state in [WalletRescanJobState::Complete, WalletRescanJobState::Failed] {
        job.state = state;
        WalletRegistry::load(runtime.store())
            .expect("registry")
            .save_rescan_job(runtime.store(), job.clone(), PersistMode::Sync)
            .expect("save inactive");
        assert_eq!(
            runtime.advance_wallet_rescan("alpha").expect("job first"),
            job
        );
    }
    remove_dir_if_exists(&path);
}

fn funded_store(test_name: &str) -> (std::path::PathBuf, FjallNodeStore, ChainstateSnapshot) {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut wallet = wallet_with_ranged_descriptor();
    let truth = funded_chainstate(&wallet);
    plant_coins_truth_with_payloads(&store, &truth, &[0, 1, 2, 3]);
    for position in &truth.active_chain {
        store
            .save_undo(
                position.block_hash,
                &BlockUndo::default(),
                PersistMode::Sync,
            )
            .expect("undo");
    }
    let partial = ChainstateSnapshot::new(
        truth.active_chain[..2].to_vec(),
        truth
            .utxos
            .iter()
            .filter(|(_, coin)| coin.created_height <= 1)
            .map(|(outpoint, coin)| (outpoint.clone(), coin.clone()))
            .collect(),
        Default::default(),
    );
    wallet
        .rescan_chainstate(&partial)
        .expect("prior funded wallet");
    WalletRegistry::default()
        .create_wallet(&store, "alpha", wallet, PersistMode::Sync)
        .expect("persist prior wallet");
    (path, store, truth)
}

fn paired_prune(store: &FjallNodeStore, truth: &ChainstateSnapshot, height: u32) {
    let position = truth
        .active_chain
        .iter()
        .find(|position| position.height == height)
        .expect("position");
    assert!(store.has_block(position.block_hash).expect("live block"));
    assert!(store.has_undo(position.block_hash).expect("live undo"));
    store
        .commit_paired_delete(height, position.block_hash)
        .expect("paired prune");
    assert!(!store.has_block(position.block_hash).expect("deleted block"));
    assert!(!store.has_undo(position.block_hash).expect("deleted undo"));
    assert!(store.load_have_pruned().expect("pruned marker"));
    let after = store
        .wallet_scan_chainstate_snapshot()
        .expect("authority")
        .expect("coins");
    assert_eq!(after.utxos, truth.utxos);
    assert_eq!(after.tip(), truth.tip());
    assert_eq!(
        FjallCoinsView::from_store(store)
            .best_block()
            .expect("durable best block"),
        truth.tip().map(|tip| tip.block_hash)
    );
}

#[test]
fn missing_creating_metadata_fails_node_job_without_wallet_write() {
    // Arrange
    let (path, store, truth) = funded_store("missing-creating-metadata");
    let before = wallet_snapshot(&store);
    let positions = truth
        .active_chain
        .iter()
        .filter(|position| position.height != 1)
        .cloned()
        .collect::<Vec<_>>();
    store
        .save_chain_meta(&positions, PersistMode::Sync)
        .expect("missing creating position");
    assert!(
        store
            .has_block(truth.active_chain[1].block_hash)
            .expect("retained payload")
    );
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");
    // Act
    let result = runtime.enqueue_rescan("alpha");
    // Assert
    assert!(result.is_err());
    assert_eq!(wallet_snapshot(runtime.store()), before);
    let job = runtime
        .store()
        .load_wallet_rescan_job("alpha")
        .expect("load")
        .expect("job");
    assert_eq!(job.state, WalletRescanJobState::Failed);
    assert_eq!(job.maybe_scanned_through_height, Some(1));
    let detail = job.maybe_error.expect("detail");
    assert!(
        detail.contains("creating") && detail.contains("height 1"),
        "{detail}"
    );
    assert!(
        !detail.contains("hash"),
        "missing metadata has no known hash"
    );
    assert!(!runtime.store().load_have_pruned().expect("never pruned"));
    remove_dir_if_exists(&path);
}

#[test]
fn post_prune_midrange_rescan_preserves_wallet_and_fails_job() {
    // Arrange
    let (path, store, truth) = funded_store("post-prune-midrange");
    let before = WalletRegistry::load(&store)
        .expect("registry")
        .wallet_snapshot("alpha")
        .expect("prior")
        .clone();
    paired_prune(&store, &truth, 1);
    for position in &truth.active_chain[2..] {
        assert!(
            store
                .has_block(position.block_hash)
                .expect("requested payload retained")
        );
    }
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2).expect("runtime");

    // Act
    let result = runtime.enqueue_rescan("alpha");

    // Assert
    assert!(
        result.is_err(),
        "old adapter admits the earlier pruned creating payload: {result:?}"
    );
    let registry = WalletRegistry::load(runtime.store()).expect("registry");
    assert_eq!(registry.wallet_snapshot("alpha").expect("wallet"), &before);
    let job = registry.rescan_job("alpha").expect("job");
    assert_eq!(job.state, WalletRescanJobState::Failed);
    assert_eq!(job.maybe_scanned_through_height, Some(1));
    assert_eq!(job.next_height, 2);
    let detail = job.maybe_error.as_deref().expect("failure detail");
    assert!(
        detail.contains("creating") && detail.contains("height 1"),
        "{detail}"
    );
    remove_dir_if_exists(&path);
}
