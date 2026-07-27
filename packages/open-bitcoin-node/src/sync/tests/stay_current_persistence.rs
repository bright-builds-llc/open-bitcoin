// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase69_stale_tip_is_distinct_from_no_progress() {
    // Arrange
    let path = temp_store_path("phase69-stale-tip");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            tip_freshness_threshold_seconds: 1_200,
            ..sync_config()
        },
    )
    .expect("runtime");
    let summary = runtime.snapshot_summary();

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time) + 1_201,
        )
        .expect("durable stale-tip status");

    // Assert
    assert_ne!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::NoProgress)
    );
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::StaleTip)
    );
    assert_eq!(
        state.sync.stay_current_next_action,
        FieldAvailability::available(
            "Refresh peers or wait for fresh peer tip evidence before treating the node as current."
                .to_string(),
        )
    );
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.height, 1);
    assert_eq!(best_known_tip.block_hash, block_hash_hex(child_hash));
    assert_eq!(best_known_tip.freshness, TipFreshnessStatus::Stale);

    remove_dir_if_exists(&path);
}

#[test]
fn phase69_tip_evidence_survives_runtime_reopen() {
    // Arrange
    let path = temp_store_path("phase69-tip-evidence-reopen");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    let expected_child_hash = block_hash_hex(child_hash);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 1,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(1)]);

    // Act
    runtime
        .sync_until_idle(&mut transport, i64::from(child.header.time) + 30)
        .expect("persist tip evidence");
    let persisted_before = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata")
        .maybe_sync_state
        .expect("persisted sync state");
    drop(runtime);
    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let reopened_runtime =
        DurableSyncRuntime::open(reopened_store, sync_config()).expect("reopen runtime");
    let persisted_after = reopened_runtime
        .store()
        .load_runtime_metadata()
        .expect("load reopened runtime metadata")
        .expect("reopened runtime metadata")
        .maybe_sync_state
        .expect("reopened persisted sync state");
    let reopened_summary = reopened_runtime.snapshot_summary();
    let reopened_state = reopened_runtime
        .durable_sync_state_for_summary(
            &reopened_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time) + 30,
        )
        .expect("reopened durable status");

    // Assert
    assert_eq!(
        persisted_before.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    assert_eq!(
        persisted_after.sync.stay_current,
        persisted_before.sync.stay_current
    );
    let FieldAvailability::Available(persisted_tip) = persisted_after.sync.best_known_tip else {
        panic!("persisted best-known tip should be available after reopen");
    };
    assert_eq!(persisted_tip.height, 1);
    assert_eq!(persisted_tip.block_hash, expected_child_hash.clone());
    assert_eq!(persisted_tip.freshness, TipFreshnessStatus::Fresh);
    assert_eq!(reopened_summary.best_header_height, 1);
    assert_eq!(reopened_summary.best_block_height, 1);
    assert_eq!(
        reopened_state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    let FieldAvailability::Available(reopened_tip) = reopened_state.sync.best_known_tip else {
        panic!("reopened best-known tip should be available");
    };
    assert_eq!(reopened_tip.height, 1);
    assert_eq!(reopened_tip.block_hash, expected_child_hash);
    assert_eq!(reopened_tip.freshness, TipFreshnessStatus::Fresh);

    remove_dir_if_exists(&path);
}
