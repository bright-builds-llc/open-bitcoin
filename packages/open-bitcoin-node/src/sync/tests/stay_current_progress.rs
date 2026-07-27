// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;
use crate::sync::block_reconcile;

#[test]
fn phase69_fresh_idle_cycle_reports_current_at_best_known_tip() {
    // Arrange
    let path = temp_store_path("phase69-fresh-idle-at-tip");
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
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 4,
            retry_backoff_ms: 1_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::new(vec![version_verack_script(1), version_verack_script(1)]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, 1_231_006_531)
        .expect("sync until idle at tip");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let state = metadata.maybe_sync_state.expect("persisted sync state");

    // Assert
    assert_eq!(summary.best_header_height, 1);
    assert_eq!(summary.best_block_height, 1);
    assert_eq!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::CurrentAtBestKnownTip {
            best_header_height: 1,
            best_block_height: 1,
        })
    );
    assert!(summary.health_signals.iter().any(|signal| {
        signal.level == HealthSignalLevel::Info
            && signal
                .message
                .contains("current at best-known validated tip")
    }));
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    assert_eq!(
        state.sync.stay_current_next_action,
        FieldAvailability::available(
            "No action required; node is current at the best-known validated tip.".to_string(),
        )
    );
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.height, 1);
    assert_eq!(best_known_tip.block_hash, block_hash_hex(child_hash));
    assert_eq!(best_known_tip.freshness, TipFreshnessStatus::Fresh);
    let FieldAvailability::Available(stop_reason) = state.sync.latest_stop_reason else {
        panic!("latest stop reason should be available");
    };
    assert_eq!(stop_reason.label, "current_at_best_known_tip");

    remove_dir_if_exists(&path);
}

#[test]
fn phase69_post_catch_up_new_headers_connect_and_report_stay_current_progress() {
    // Arrange
    let path = temp_store_path("phase69-post-catch-up-new-work");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let grandchild = build_block(block_hash(&child.header), 2);
    let grandchild_hash = block_hash(&grandchild.header);
    let expected_grandchild_hash = block_hash_hex(grandchild_hash);
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
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 2,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![grandchild.header.clone()],
        }),
        WireNetworkMessage::Block(grandchild.clone()),
    ]]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, i64::from(grandchild.header.time))
        .expect("sync post-catch-up work");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let state = metadata.maybe_sync_state.expect("persisted sync state");
    let snapshot = runtime
        .store()
        .load_chainstate_snapshot()
        .expect("load chainstate snapshot")
        .expect("chainstate snapshot");
    let active_tip = snapshot.active_chain.last().expect("active tip");

    // Assert
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 1);
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(summary.best_block_height, 2);
    assert_eq!(summary.downloaded_block_height, 2);
    assert_eq!(
        getdata_block_hashes(&transport.sent_messages()),
        vec![grandchild_hash]
    );
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    let FieldAvailability::Available(progress) = state.sync.sync_progress else {
        panic!("sync progress should be available");
    };
    assert_eq!(progress.header_height, 2);
    assert_eq!(progress.validated_active_chain_height, 2);
    assert_eq!(
        progress.maybe_validated_active_chain_hash,
        Some(expected_grandchild_hash.clone())
    );
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.height, 2);
    assert_eq!(best_known_tip.block_hash, expected_grandchild_hash.clone());
    assert_eq!(best_known_tip.work, "3");
    assert_eq!(best_known_tip.freshness, TipFreshnessStatus::Fresh);
    assert_eq!(
        best_known_tip.peer_agreement.first().map(|row| row.status),
        Some(PeerTipAgreementStatus::Agrees)
    );
    assert_eq!(active_tip.height, 2);
    assert_eq!(active_tip.block_hash, grandchild_hash);
    assert_eq!(active_tip.chain_work, 3);
    assert!(
        runtime
            .store()
            .load_block(grandchild_hash)
            .expect("load grandchild block")
            .is_some()
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase69_headers_only_tip_does_not_report_current() {
    // Arrange
    let path = temp_store_path("phase69-headers-only-not-current");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let grandchild = build_block(block_hash(&child.header), 2);
    let child_hash = block_hash(&child.header);
    let grandchild_hash = block_hash(&grandchild.header);
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
    let mut transport =
        ScriptedTransport::new(vec![headers_script(2, vec![grandchild.header.clone()])]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, i64::from(grandchild.header.time))
        .expect("sync headers-only tip");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let state = metadata.maybe_sync_state.expect("persisted sync state");

    // Assert
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(summary.best_block_height, 1);
    assert!(!matches!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::CurrentAtBestKnownTip { .. })
    ));
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::NoProgress)
    );
    assert_eq!(
        state.sync.stay_current_next_action,
        FieldAvailability::available(
            "Retry sync or inspect peer outcomes; no useful stay-current progress was observed."
                .to_string(),
        )
    );
    let FieldAvailability::Available(progress) = state.sync.sync_progress else {
        panic!("sync progress should be available");
    };
    assert_eq!(progress.header_height, 2);
    assert_eq!(progress.validated_active_chain_height, 1);
    assert_eq!(
        progress.maybe_validated_active_chain_hash,
        Some(block_hash_hex(child_hash))
    );
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.height, 2);
    assert_eq!(best_known_tip.block_hash, block_hash_hex(grandchild_hash));
    assert_eq!(best_known_tip.freshness, TipFreshnessStatus::Fresh);

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_progress_guarantee_projection_rejects_headers_only_as_useful_work() {
    // Arrange
    let path = temp_store_path("phase78-headers-only-progress-guarantee");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let grandchild = build_block(block_hash(&child.header), 2);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let previous_summary = runtime.snapshot_summary();
    let previous_timestamp = u64::from(child.header.time);
    let previous_state = runtime
        .durable_sync_state_for_summary(
            &previous_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time),
        )
        .expect("previous durable status");
    runtime
        .persist_durable_sync_state(previous_state)
        .expect("persist previous state");
    let mut transport =
        ScriptedTransport::new(vec![headers_script(2, vec![grandchild.header.clone()])]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(grandchild.header.time))
        .expect("headers-only sync");
    let state = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata")
        .maybe_sync_state
        .expect("persisted sync state");
    let records = load_structured_log_records(&log_dir);

    // Assert
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 0);
    assert!(matches!(
        state.sync.progress_credit,
        FieldAvailability::Unavailable { .. }
    ));
    let FieldAvailability::Available(last_work) = state.sync.last_useful_work else {
        panic!("previous durable active-chain work should be carried");
    };
    assert_eq!(
        last_work.kind,
        ProgressCreditKind::ValidatedDurableActiveChain
    );
    assert_eq!(last_work.credited_validated_active_chain_height, 1);
    assert_eq!(
        state.sync.last_successful_progress_unix_seconds,
        FieldAvailability::available(previous_timestamp)
    );
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::NoProgress)
    );
    assert!(records.iter().any(|record| {
        record.message.contains("progress_credit=unavailable")
            && record
                .message
                .contains("last_useful_work=validated_durable_active_chain:1")
            && record.message.contains("stalled_subsystem=")
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_branch_competition_does_not_credit_replacement_tip_before_connect() {
    // Arrange
    let path = temp_store_path("phase78-branch-competition-no-credit");
    remove_dir_if_exists(&path);
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
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
    let previous_summary = runtime.snapshot_summary();
    let previous_state = runtime
        .durable_sync_state_for_summary(
            &previous_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(branch_a_two.header.time),
        )
        .expect("previous durable status");
    runtime
        .persist_durable_sync_state(previous_state)
        .expect("persist previous status");

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
        .expect("durable branch competition status");

    // Assert
    assert!(matches!(
        progress,
        SyncReconcileProgress::BranchCompetitionAwaitingBodies { .. }
    ));
    assert_progress_credit_unavailable(&state);
    let last_work = available_last_useful_work(&state);
    assert_eq!(
        last_work.kind,
        ProgressCreditKind::ValidatedDurableActiveChain
    );
    assert_eq!(last_work.credited_validated_active_chain_height, 2);
    assert_eq!(
        state.sync.no_progress_diagnosis,
        FieldAvailability::available(NoProgressDiagnosis::BranchCompetitionAwaitingBodies)
    );
    let stall = available_stall_diagnosis(&state);
    assert_eq!(
        serialized_label(stall.stalled_subsystem),
        "branch_competition_awaiting_bodies"
    );
    assert_eq!(
        stall.stalled_subsystem,
        StalledSubsystem::BranchCompetitionAwaitingBodies
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_current_at_tip_credits_stay_current_useful_work() {
    // Arrange
    let path = temp_store_path("phase78-current-at-tip-credit");
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
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 2,
            retry_backoff_ms: 1_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let previous_summary = runtime.snapshot_summary();
    let previous_state = runtime
        .durable_sync_state_for_summary(
            &previous_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time),
        )
        .expect("previous durable status");
    assert_eq!(
        available_progress_credit(&previous_state).kind,
        ProgressCreditKind::ValidatedDurableActiveChain
    );
    runtime
        .persist_durable_sync_state(previous_state)
        .expect("persist previous status");
    let mut transport =
        ScriptedTransport::new(vec![version_verack_script(1), version_verack_script(1)]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, i64::from(child.header.time) + 1)
        .expect("sync until current at tip");
    let state = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata")
        .maybe_sync_state
        .expect("persisted sync state");

    // Assert
    assert_eq!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::CurrentAtBestKnownTip {
            best_header_height: 1,
            best_block_height: 1,
        })
    );
    let credit = available_progress_credit(&state);
    assert_eq!(credit.kind, ProgressCreditKind::CurrentAtBestKnownTip);
    assert_eq!(credit.credited_validated_active_chain_height, 1);
    assert_eq!(
        credit.credited_validated_active_chain_hash,
        block_hash_hex(child_hash)
    );
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    let stall = available_stall_diagnosis(&state);
    assert_eq!(serialized_label(stall.stalled_subsystem), "at_tip_waiting");
    assert_eq!(stall.stalled_subsystem, StalledSubsystem::AtTipWaiting);

    remove_dir_if_exists(&path);
}
