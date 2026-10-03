// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/rpc/transactions.cpp
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use fixtures::*;

mod fixtures;
use open_bitcoin_node::chainstate::FlushPersistSink;
use open_bitcoin_node::core::chainstate::CoinsView;
use open_bitcoin_node::core::chainstate::{BlockUndo, ChainPosition, ChainstateSnapshot, Coin};
use open_bitcoin_node::core::primitives::{
    Amount, Block, BlockHeader, OutPoint, TransactionOutput, Txid,
};
use open_bitcoin_node::core::wallet::{DescriptorRole, Wallet, WalletSnapshot};
use open_bitcoin_node::storage::coins_view::FjallCoinsView;
use open_bitcoin_node::wallet_registry::rescan::{
    WalletRescanEligibilityFailure, guard_durable_wallet_rescan_authority,
    prepare_durable_wallet_rescan, prepare_wallet_rescan_with_probe,
};
use open_bitcoin_node::{
    StorageError, StorageNamespace, StorageRecoveryAction, WalletRegistryError, WalletRescanJob,
};
use open_bitcoin_node::{WalletRegistry, WalletRescanJobState};

const DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)";

#[test]
fn durable_post_prune_midrange_rescan_preserves_wallet_and_fails_job() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-post-prune-midrange");
    let before = saved_wallet(&store);
    prune(&store, &truth, 1);
    let mut context = context(&store, &data_dir);
    // Act
    let result = context.rescan_wallet_range(Some(3), Some(3));
    // Assert
    assert!(
        result.is_err(),
        "missing older creating block must refuse: {result:?}"
    );
    assert_eq!(saved_wallet(&store), before);
    let job = saved_job(&store);
    assert_eq!(job.state, WalletRescanJobState::Failed);
    assert_eq!(job.maybe_scanned_through_height, Some(1));
    assert_eq!(job.next_height, 2);
    assert_eq!(
        job.freshness,
        open_bitcoin_node::WalletRescanFreshness::Partial
    );
    assert_eq!(
        job.maybe_tip_median_time_past,
        before.maybe_tip_median_time_past
    );
    let detail = job.maybe_error.expect("detail");
    assert!(
        detail.contains("creating")
            && detail.contains("height 1")
            && detail.contains(&format!("{:?}", truth.active_chain[1].block_hash))
    );
    cleanup(data_dir, context, store);
}

#[test]
fn durable_retained_payload_midrange_rescan_succeeds() {
    // Arrange
    let (data_dir, store, _) = fixture("rpc-retained-midrange");
    let mut context = context(&store, &data_dir);
    // Act
    let result = context
        .rescan_wallet_range(Some(2), Some(3))
        .expect("retained");
    // Assert
    assert_eq!(result.stop_height, 3);
    let wallet = saved_wallet(&store);
    assert_eq!(wallet.utxos.len(), 2);
    assert_eq!(wallet.maybe_tip_height, Some(3));
    assert_eq!(
        wallet
            .utxos
            .iter()
            .map(|coin| coin.output.value.to_sats())
            .sum::<i64>(),
        60_000
    );
    cleanup(data_dir, context, store);
}

#[test]
fn durable_post_prune_reopen_refuses_poison_leftover() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-prune-reopen-poison");
    let before = saved_wallet(&store);
    prune(&store, &truth, 1);
    let leftover = poison(&truth);
    store
        .save_chainstate_snapshot(&leftover, PersistMode::Sync)
        .expect("poison");
    drop(store);
    let store = FjallNodeStore::open(&data_dir).expect("reopen");
    let mut context = context(&store, &data_dir);
    // Act
    let result = context.rescan_wallet_range(Some(2), Some(3));
    // Assert
    assert!(result.is_err());
    assert_refusal(&store, &before, "creating", 1);
    assert_eq!(
        store.load_chainstate_snapshot().expect("leftover"),
        Some(leftover)
    );
    assert!(
        !store
            .has_block(truth.active_chain[1].block_hash)
            .expect("absent")
    );
    assert!(
        !store
            .has_undo(truth.active_chain[1].block_hash)
            .expect("absent")
    );
    assert!(store.load_have_pruned().expect("marker"));
    cleanup(data_dir, context, store);
}

#[test]
fn durable_post_prune_later_request_rechecks_partial_success_before_and_after_reopen() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-prune-partial-resume");
    let mut rpc = context(&store, &data_dir);
    let first = rpc
        .rescan_wallet_range(Some(0), Some(1))
        .expect("first range");
    assert_eq!(
        first.freshness.freshness,
        super::super::WalletFreshnessKind::Partial
    );
    let before = saved_wallet(&store);
    let checkpoint = saved_job(&store);
    prune(&store, &truth, 1);
    // Act
    let result = rpc.rescan_wallet_range(Some(3), Some(3));
    // Assert
    assert!(result.is_err());
    assert_refusal(&store, &before, "creating", 1);
    let failed = saved_job(&store);
    assert_eq!(failed.next_height, checkpoint.next_height);
    assert_eq!(
        failed.freshness,
        open_bitcoin_node::WalletRescanFreshness::Partial
    );
    drop(rpc);
    drop(store);
    let reopened = FjallNodeStore::open(&data_dir).expect("reopen");
    let mut rpc = context(&reopened, &data_dir);
    let result = rpc.rescan_wallet_range(Some(2), Some(3));
    assert!(result.is_err());
    assert_refusal(&reopened, &before, "creating", 1);
    assert_eq!(saved_job(&reopened).next_height, checkpoint.next_height);
    assert_eq!(
        saved_job(&reopened).freshness,
        open_bitcoin_node::WalletRescanFreshness::Partial
    );
    drop(rpc);
    drop(reopened);
    fs::remove_dir_all(data_dir).expect("cleanup");
}

#[test]
fn durable_post_prune_in_range_absence_preserves_wallet() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-prune-in-range");
    let before = saved_wallet(&store);
    prune(&store, &truth, 2);
    let mut context = context(&store, &data_dir);
    // Act
    let result = context.rescan_wallet_range(Some(2), Some(3));
    // Assert
    assert!(result.is_err());
    assert_refusal(&store, &before, "requested", 2);
    cleanup(data_dir, context, store);
}

#[test]
fn durable_unrelated_pruned_coin_does_not_block_wallet_rescan() {
    // Arrange
    let (data_dir, store, mut truth) = fixture("rpc-unrelated-pruned");
    for coin in truth
        .utxos
        .values_mut()
        .filter(|coin| coin.created_height == 1)
    {
        coin.output.script_pubkey =
            open_bitcoin_node::core::primitives::ScriptBuf::from_bytes(vec![0x51])
                .expect("unrelated");
    }
    store
        .seed_coins_from_snapshot(&truth)
        .expect("updated coins");
    prune(&store, &truth, 1);
    let mut context = context(&store, &data_dir);
    // Act
    context
        .rescan_wallet_range(Some(2), Some(3))
        .expect("unrelated excluded");
    // Assert
    let wallet = saved_wallet(&store);
    assert_eq!(wallet.utxos.len(), 1);
    assert_eq!(wallet.utxos[0].created_height, 3);
    assert_eq!(wallet.utxos[0].output.value.to_sats(), 35_000);
    assert_eq!(
        store
            .wallet_scan_chainstate_snapshot()
            .expect("truth")
            .expect("coins")
            .utxos
            .len(),
        2
    );
    cleanup(data_dir, context, store);
}

#[test]
fn durable_probe_error_saves_failed_without_changing_wallet() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-probe-error");
    let before = saved_wallet(&store);
    let mut context = context(&store, &data_dir);
    // Act
    let result = context.rescan_wallet_range_with(
        Some(2),
        Some(3),
        guard_durable_wallet_rescan_authority,
        |_, wallet, snapshot, start, stop| {
            prepare_wallet_rescan_with_probe(wallet, snapshot, start, stop, |_| {
                Err(backend_error(StorageNamespace::BlockIndex))
            })
        },
        save_failure,
    );
    // Assert
    let error = result.expect_err("read error");
    assert_refusal(&store, &before, "creating", 1);
    let detail = saved_job(&store).maybe_error.expect("detail");
    assert!(
        detail.contains("BackendFailure")
            && detail.contains(&format!("{:?}", truth.active_chain[1].block_hash))
    );
    assert!(!detail.contains("private") && !detail.contains("secret"));
    assert!(
        !error
            .maybe_detail
            .expect("RPC detail")
            .message
            .contains("private")
    );
    cleanup(data_dir, context, store);
}

#[test]
fn durable_missing_creating_metadata_saves_failed_without_changing_wallet() {
    // Arrange
    let (data_dir, store, _) = fixture("rpc-missing-creating-metadata");
    let before = saved_wallet(&store);
    let mut context = context(&store, &data_dir);
    // Act
    let result = context.rescan_wallet_range_with(
        Some(2),
        Some(3),
        guard_durable_wallet_rescan_authority,
        |store, wallet, snapshot, start, stop| {
            let mut missing = snapshot.clone();
            missing.active_chain.retain(|position| position.height != 1);
            prepare_durable_wallet_rescan(store, wallet, &missing, start, stop)
        },
        save_failure,
    );
    // Assert
    assert!(result.is_err());
    assert_refusal(&store, &before, "creating", 1);
    assert!(
        !saved_job(&store)
            .maybe_error
            .expect("detail")
            .contains("hash")
    );
    cleanup(data_dir, context, store);
}

#[test]
fn durable_failed_save_error_propagates_without_changing_wallet() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-failed-save-error");
    let before = saved_wallet(&store);
    prune(&store, &truth, 1);
    let mut context = context(&store, &data_dir);
    // Act
    let result = context.rescan_wallet_range_with(
        Some(2),
        Some(3),
        guard_durable_wallet_rescan_authority,
        prepare_durable_wallet_rescan,
        |_, _, job| {
            assert_eq!(job.state, WalletRescanJobState::Failed);
            Err(WalletRegistryError::Storage(
                StorageError::UnavailableNamespace {
                    namespace: StorageNamespace::Wallet,
                },
            ))
        },
    );
    // Assert
    let error = result.expect_err("save error propagated");
    assert!(
        error
            .maybe_detail
            .expect("detail")
            .message
            .contains("wallet")
    );
    assert_eq!(saved_wallet(&store), before);
    assert_eq!(saved_job(&store).state, WalletRescanJobState::Pending);
    cleanup(data_dir, context, store);
}

#[test]
fn durable_changed_target_failed_save_keeps_pending_freshness_partial() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-changed-target-failed-save");
    let mut context = context(&store, &data_dir);
    context
        .rescan_wallet_range(Some(0), Some(1))
        .expect("first range");
    let checkpoint = saved_job(&store);
    assert_eq!(checkpoint.state, WalletRescanJobState::Complete);
    assert_eq!(
        checkpoint.freshness,
        open_bitcoin_node::WalletRescanFreshness::Fresh
    );
    let before = saved_wallet(&store);
    prune(&store, &truth, 1);
    // Act
    let result = context.rescan_wallet_range_with(
        Some(2),
        Some(3),
        guard_durable_wallet_rescan_authority,
        prepare_durable_wallet_rescan,
        |_, _, job| {
            assert_eq!(job.state, WalletRescanJobState::Failed);
            Err(WalletRegistryError::Storage(
                StorageError::UnavailableNamespace {
                    namespace: StorageNamespace::Wallet,
                },
            ))
        },
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(saved_wallet(&store), before);
    let pending = saved_job(&store);
    assert_eq!(pending.state, WalletRescanJobState::Pending);
    assert_eq!(
        pending.freshness,
        open_bitcoin_node::WalletRescanFreshness::Partial
    );
    assert_eq!(pending.target_tip_height, 3);
    assert_eq!(
        pending.target_tip_hash,
        truth.tip().expect("tip").block_hash
    );
    assert_eq!(pending.next_height, checkpoint.next_height);
    assert_eq!(
        pending.maybe_scanned_through_height,
        checkpoint.maybe_scanned_through_height
    );
    assert_eq!(
        pending.maybe_tip_median_time_past,
        checkpoint.maybe_tip_median_time_past
    );
    let view = context.wallet_freshness().expect("freshness");
    assert!(view.scanning);
    assert_eq!(view.freshness, super::super::WalletFreshnessKind::Partial);
    assert_eq!(view.maybe_target_height, Some(3));
    assert_eq!(view.maybe_scanned_through_height, Some(1));
    cleanup(data_dir, context, store);
}

#[test]
fn durable_unknown_checkpoint_refusal_keeps_scanning_freshness() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-unknown-checkpoint-freshness");
    let mut registry = WalletRegistry::load(&store).expect("registry");
    let mut wallet = registry.wallet("alpha").expect("wallet");
    wallet
        .rescan_chainstate(&ChainstateSnapshot::new(
            Vec::new(),
            Default::default(),
            Default::default(),
        ))
        .expect("empty prior");
    registry
        .save_wallet(&store, "alpha", &wallet, PersistMode::Sync)
        .expect("save prior");
    let before = saved_wallet(&store);
    prune(&store, &truth, 1);
    let mut context = context(&store, &data_dir);
    // Act
    let result = context.rescan_wallet_range(Some(2), Some(3));
    // Assert
    assert!(result.is_err());
    assert_eq!(saved_wallet(&store), before);
    let failed = saved_job(&store);
    assert_eq!(failed.state, WalletRescanJobState::Failed);
    assert_eq!(
        failed.freshness,
        open_bitcoin_node::WalletRescanFreshness::Scanning
    );
    assert_eq!(failed.maybe_scanned_through_height, None);
    assert_eq!(failed.maybe_tip_median_time_past, None);
    assert_eq!(failed.next_height, 0);
    cleanup(data_dir, context, store);
}

#[test]
fn durable_retained_checkpoint_at_target_refusal_keeps_fresh_metadata() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-checkpoint-at-target-refusal");
    let mut context = context(&store, &data_dir);
    context
        .rescan_wallet_range(Some(0), Some(1))
        .expect("first range");
    let before = saved_wallet(&store);
    let checkpoint = saved_job(&store);
    prune(&store, &truth, 1);
    // Act
    let result = context.rescan_wallet_range(Some(1), Some(1));
    // Assert
    assert!(result.is_err());
    assert_refusal(&store, &before, "creating", 1);
    let failed = saved_job(&store);
    assert_eq!(
        failed.freshness,
        open_bitcoin_node::WalletRescanFreshness::Fresh
    );
    assert_eq!(failed.target_tip_height, 1);
    assert_eq!(failed.target_tip_hash, checkpoint.target_tip_hash);
    assert_eq!(failed.next_height, checkpoint.next_height);
    cleanup(data_dir, context, store);
}

#[test]
fn durable_authority_head_check_failure_saves_failed_without_changing_wallet() {
    // Arrange / Act / Assert: typed interruption enters the production refusal handler.
    authority_failure_case(
        "rpc-head-refusal",
        StorageError::InterruptedWrite {
            namespace: StorageNamespace::Coins,
            action: StorageRecoveryAction::Reindex,
        },
        "InterruptedWrite",
    );
}

#[test]
fn durable_authority_read_failure_saves_safe_failed_without_changing_wallet() {
    // Arrange / Act / Assert: backend text never reaches persisted detail.
    authority_failure_case(
        "rpc-authority-read-refusal",
        backend_error(StorageNamespace::Coins),
        "BackendFailure",
    );
}

#[test]
fn durable_authority_refusal_without_prior_job_does_not_invent_target() {
    // Arrange
    let (data_dir, store, _) = fixture("rpc-authority-no-target");
    let before = saved_wallet(&store);
    let mut context = context(&store, &data_dir);
    // Act
    let result = context.rescan_wallet_range_with(
        Some(2),
        Some(3),
        |_| {
            Err(WalletRescanEligibilityFailure::authority(
                backend_error(StorageNamespace::Coins).into(),
            ))
        },
        prepare_durable_wallet_rescan,
        save_failure,
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(saved_wallet(&store), before);
    assert!(
        store
            .load_wallet_rescan_job("alpha")
            .expect("load")
            .is_none()
    );
    cleanup(data_dir, context, store);
}

#[test]
fn durable_direct_rescan_helper_cannot_bypass_eligibility() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-direct-helper");
    let before = saved_wallet(&store);
    prune(&store, &truth, 1);
    let mut context = context(&store, &data_dir);
    let favorable = ChainstateSnapshot::new(Vec::new(), Default::default(), Default::default());
    // Act
    let result = context.rescan_wallet(&favorable);
    // Assert
    assert!(result.is_err());
    assert_refusal(&store, &before, "creating", 1);
    cleanup(data_dir, context, store);
}

#[test]
fn local_direct_rescan_helper_keeps_supplied_snapshot() {
    // Arrange
    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig::default());
    let snapshot = ChainstateSnapshot::new(
        vec![ChainPosition::new(BlockHeader::default(), 7, 8, 77)],
        Default::default(),
        Default::default(),
    );
    // Act
    context
        .rescan_wallet(&snapshot)
        .expect("local supplied fixture");
    // Assert
    assert_eq!(
        context.wallet_snapshot().expect("wallet").maybe_tip_height,
        Some(7)
    );
    assert_eq!(
        context
            .wallet_snapshot()
            .expect("wallet")
            .maybe_tip_median_time_past,
        Some(77)
    );
}

#[test]
fn durable_missing_stop_position_does_not_fabricate_new_target() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-missing-stop-no-job");
    let before = saved_wallet(&store);
    let mut context = missing_stop_context(&store, &data_dir, &truth);
    // Act
    let error = context
        .rescan_wallet_range(Some(2), Some(2))
        .expect_err("missing target");
    // Assert
    assert_eq!(saved_wallet(&store), before);
    assert!(
        store
            .load_wallet_rescan_job("alpha")
            .expect("load")
            .is_none()
    );
    let detail = error.maybe_detail.expect("detail").message;
    assert!(
        detail.contains("requested") && detail.contains("height 2") && !detail.contains("hash")
    );
    cleanup(data_dir, context, store);
}

#[test]
fn durable_missing_stop_position_fails_identified_job_without_changing_target() {
    // Arrange
    let (data_dir, store, truth) = fixture("rpc-missing-stop-existing-job");
    let before = saved_wallet(&store);
    let mut pending =
        WalletRescanJob::new("alpha", truth.tip().expect("tip").block_hash, 3, 2, Some(1))
            .expect("job");
    pending.maybe_tip_median_time_past = before.maybe_tip_median_time_past;
    save_failure(
        &mut WalletRegistry::load(&store).expect("registry"),
        &store,
        pending.clone(),
    )
    .expect("pending");
    let mut context = missing_stop_context(&store, &data_dir, &truth);
    // Act
    let result = context.rescan_wallet_range(Some(2), Some(2));
    // Assert
    assert!(result.is_err());
    assert_refusal(&store, &before, "requested", 2);
    let failed = saved_job(&store);
    assert_eq!(failed.target_tip_hash, pending.target_tip_hash);
    assert_eq!(failed.target_tip_height, pending.target_tip_height);
    assert_eq!(failed.next_height, pending.next_height);
    cleanup(data_dir, context, store);
}
