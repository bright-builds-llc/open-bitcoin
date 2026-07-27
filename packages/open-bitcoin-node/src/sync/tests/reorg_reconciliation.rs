// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;
use crate::sync::block_reconcile;

#[test]
fn reorg_reconcile_context_preserves_explicit_event_time() {
    // Arrange
    let event_time = 80;

    // Act
    let context = block_reconcile::reorg_lifecycle_context(event_time);

    // Assert
    assert_eq!(context.occurred_at, PolicyTime::new(event_time));
}

#[test]
fn phase70_branch_awaiting_bodies_does_not_disconnect_active_chain() {
    // Arrange
    let path = temp_store_path("phase70-branch-awaiting-bodies");
    remove_dir_if_exists(&path);
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
    let branch_b_one_hash = block_hash(&branch_b_one.header);
    save_best_chain_with_active_blocks(
        &path,
        &[
            (&genesis, 0),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
        &[(&genesis, 0), (&branch_a_one, 1), (&branch_a_two, 2)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let progress = block_reconcile::reconcile_and_persist_best_chain(
        &mut runtime,
        i64::from(branch_b_three.header.time),
    )
    .expect("reconcile should wait for missing branch bodies");
    let snapshot = runtime.snapshot_summary();
    let state = runtime
        .durable_sync_state_for_summary(
            &snapshot,
            SyncLifecycleState::Active,
            None,
            i64::from(branch_b_three.header.time),
        )
        .expect("durable reconcile status");

    // Assert
    assert_eq!(
        progress,
        SyncReconcileProgress::BranchCompetitionAwaitingBodies {
            missing_count: 3,
            first_missing_height: 1,
            first_missing_hash: block_hash_hex(branch_b_one_hash),
        }
    );
    assert_eq!(snapshot.best_block_height, 2);
    assert_eq!(
        snapshot.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&branch_a_two.header)))
    );
    assert_eq!(
        state.sync.reconcile_progress,
        FieldAvailability::available(
            SyncReconcileProgressStatus::BranchCompetitionAwaitingBodies {
                common_ancestor_height: 0,
                common_ancestor_hash: block_hash_hex(block_hash(&genesis.header)),
                branch_tip_height: 3,
                branch_tip_hash: block_hash_hex(block_hash(&branch_b_three.header)),
                missing_block_count: 3,
            }
        )
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_reorg_records_bounded_persisted_evidence() {
    // Arrange
    let path = temp_store_path("phase70-branch-reorg-persisted");
    remove_dir_if_exists(&path);
    let (genesis, _branch_a_one, _branch_a_two, _branch_b_one, _branch_b_two, branch_b_three) =
        phase70_save_reorg_ready_branch(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let expected_evidence = SyncReorgEvidence {
        common_ancestor_height: 0,
        common_ancestor_hash: block_hash_hex(block_hash(&genesis.header)),
        disconnected_count: 2,
        connected_count: 3,
        final_active_height: 3,
        final_active_hash: block_hash_hex(block_hash(&branch_b_three.header)),
        fully_persisted: true,
    };

    // Act
    let progress = block_reconcile::reconcile_and_persist_best_chain(
        &mut runtime,
        i64::from(branch_b_three.header.time),
    )
    .expect("reconcile should reorg to complete better branch");
    let snapshot = runtime.snapshot_summary();
    let state = runtime
        .durable_sync_state_for_summary(
            &snapshot,
            SyncLifecycleState::Active,
            None,
            i64::from(branch_b_three.header.time),
        )
        .expect("durable reorg status");
    runtime
        .persist_durable_sync_state(state.clone())
        .expect("persist reorg status");
    drop(runtime);
    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let reopened_runtime =
        DurableSyncRuntime::open(reopened_store, sync_config()).expect("reopen runtime");
    let reopened_summary = reopened_runtime.snapshot_summary();
    let reopened_state = reopened_runtime
        .durable_sync_state_for_summary(
            &reopened_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(branch_b_three.header.time),
        )
        .expect("reopened durable reorg status");

    // Assert
    assert_eq!(
        progress,
        SyncReconcileProgress::ReorgPersisted(expected_evidence.clone())
    );
    assert_eq!(snapshot.best_block_height, 3);
    assert_eq!(
        snapshot.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&branch_b_three.header)))
    );
    assert_eq!(
        state.sync.latest_reorg,
        FieldAvailability::available(expected_evidence.clone())
    );
    assert_eq!(
        state.sync.reconcile_progress,
        FieldAvailability::available(SyncReconcileProgressStatus::ReorgPersisted {
            evidence: expected_evidence.clone(),
        })
    );
    assert_eq!(
        reopened_state.sync.latest_reorg,
        FieldAvailability::available(expected_evidence)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_equal_or_lower_work_side_branch_does_not_replace_active_tip() {
    // Arrange
    let path = temp_store_path("phase70-branch-side-preserved");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let first_branch = build_branch_block(block_hash(&genesis.header), 1, 100);
    let second_branch = build_branch_block(block_hash(&genesis.header), 1, 200);
    let first_hash = block_hash(&first_branch.header);
    let second_hash = block_hash(&second_branch.header);
    let (active_tip, side_tip) = if first_hash > second_hash {
        (first_branch, second_branch)
    } else {
        (second_branch, first_branch)
    };
    let active_tip_hash = block_hash(&active_tip.header);
    let side_tip_hash = block_hash(&side_tip.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&side_tip, 1)],
        &[(&genesis, 0), (&active_tip, 1)],
    );
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_block(&side_tip, PersistMode::Sync)
            .expect("save side branch body");
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let progress = block_reconcile::reconcile_and_persist_best_chain(
        &mut runtime,
        i64::from(side_tip.header.time),
    )
    .expect("reconcile should preserve equal-work side branch");
    let snapshot = runtime.snapshot_summary();
    let state = runtime
        .durable_sync_state_for_summary(
            &snapshot,
            SyncLifecycleState::Active,
            None,
            i64::from(side_tip.header.time),
        )
        .expect("durable side branch status");

    // Assert
    assert_eq!(progress, SyncReconcileProgress::SideBranchPreserved);
    assert_eq!(snapshot.best_block_height, 1);
    assert_eq!(
        snapshot.maybe_connected_block_hash,
        Some(block_hash_hex(active_tip_hash))
    );
    assert_eq!(
        state.sync.reconcile_progress,
        FieldAvailability::available(SyncReconcileProgressStatus::SideBranchPreserved {
            branch_tip_height: 1,
            branch_tip_hash: block_hash_hex(side_tip_hash),
            active_tip_height: 1,
            active_tip_hash: block_hash_hex(active_tip_hash),
        })
    );
    assert!(side_tip_hash < active_tip_hash);

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_missing_active_chain_block_body_is_storage_blocker() {
    // Arrange
    let path = temp_store_path("phase70-missing-active-body");
    remove_dir_if_exists(&path);
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
    save_chain_headers_snapshot_and_blocks(
        &path,
        &[
            (&genesis, 0),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
        &[(&genesis, 0), (&branch_a_one, 1), (&branch_a_two, 2)],
        &[
            (&genesis, 0),
            (&branch_a_one, 1),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let error = block_reconcile::reconcile_best_chain(
        &mut runtime,
        i64::from(branch_b_three.header.time),
    )
    .expect_err("missing active body should block reorg");

    // Assert
    assert!(matches!(
        error,
        SyncRuntimeError::Storage(StorageError::Corruption {
            namespace: StorageNamespace::BlockIndex,
            action: StorageRecoveryAction::Repair,
            ref detail,
        }) if detail.contains("missing durable block body")
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_missing_undo_data_is_storage_blocker() {
    // Arrange
    let path = temp_store_path("phase70-missing-undo");
    remove_dir_if_exists(&path);
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
    save_chain_headers_snapshot_and_blocks(
        &path,
        &[
            (&genesis, 0),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
        &[(&genesis, 0), (&branch_a_one, 1), (&branch_a_two, 2)],
        &[
            (&genesis, 0),
            (&branch_a_one, 1),
            (&branch_a_two, 2),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let error = block_reconcile::reconcile_best_chain(
        &mut runtime,
        i64::from(branch_b_three.header.time),
    )
    .expect_err("missing undo should block reorg");

    // Assert
    assert!(matches!(
        error,
        SyncRuntimeError::Storage(StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            action: StorageRecoveryAction::Repair,
            ref detail,
        }) if detail.contains("missing undo data")
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_malformed_stored_chainstate_is_storage_blocker() {
    // Arrange
    let path = temp_store_path("phase70-malformed-chainstate");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .write_raw_for_test(
            StorageNamespace::Chainstate,
            "snapshot",
            b"{bad-json".to_vec(),
        )
        .expect("write malformed chainstate snapshot");

    // Act
    let error = match DurableSyncRuntime::open(store, sync_config()) {
        Ok(_) => panic!("malformed chainstate should block runtime open"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        SyncRuntimeError::Storage(StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            action: StorageRecoveryAction::Repair,
            ..
        })
    ));

    remove_dir_if_exists(&path);
}
