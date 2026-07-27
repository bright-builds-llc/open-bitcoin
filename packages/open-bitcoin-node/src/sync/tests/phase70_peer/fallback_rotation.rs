// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase70_notfound_releases_inflight_and_rotates_to_second_peer() {
    // Arrange
    let path = temp_store_path("phase70-peer-notfound-rotation");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            notfound_for_block(child_hash),
        ],
        version_verack_script(1),
    ]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(child.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.peer_outcomes.len(), 2);
    assert_reason_without_block_credit(&summary, PeerFailureReason::BlockNotFound);
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == child_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(1).is_err());
    assert_first_peer_backoff(&runtime);

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_stale_inflight_cleanup_preserves_prior_credit_and_rotates_peer() {
    // Arrange
    let path = temp_store_path("phase78-stale-inflight-prior-credit");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
    runtime
        .inflight_blocks
        .insert(BlockHash::from_byte_array([78_u8; 32]));
    let previous_credit =
        persist_previous_active_chain_credit(&mut runtime, i64::from(child.header.time));
    assert_eq!(
        serialized_label(RejectedProgressActivityKind::InFlightRequest),
        "in_flight_request"
    );
    assert_rejected_activity(
        &previous_credit,
        RejectedProgressActivityKind::InFlightRequest,
    );
    let observed_at_unix_seconds = i64::from(child.header.time) + 10_000;
    let mut transport = ScriptedTransport::new(vec![Vec::new(), version_verack_script(1)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, observed_at_unix_seconds)
        .expect("sync with stale in-flight and replacement peer");
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            None,
            observed_at_unix_seconds,
        )
        .expect("durable stale in-flight status");

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Stalled);
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert_progress_credit_unavailable(&state);
    let last_work = available_last_useful_work(&state);
    assert_eq!(
        last_work.kind,
        ProgressCreditKind::ValidatedDurableActiveChain
    );
    assert_eq!(last_work.credited_validated_active_chain_height, 1);
    assert_rejected_activity(last_work, RejectedProgressActivityKind::InFlightRequest);
    assert_eq!(
        state.sync.no_progress_diagnosis,
        FieldAvailability::available(NoProgressDiagnosis::StaleInflightCleanup)
    );
    let last_peer_contribution = available_last_peer_contribution(&state);
    assert_eq!(
        last_peer_contribution.kind,
        PeerContributionKind::MessagesOnly
    );
    let stall = available_stall_diagnosis(&state);
    assert_eq!(
        serialized_label(stall.stalled_subsystem),
        "slow_or_stalled_peers"
    );
    assert_eq!(
        stall.stalled_subsystem,
        StalledSubsystem::SlowOrStalledPeers
    );
    assert_first_peer_backoff(&runtime);

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_no_credit_peer_rotation_keeps_last_peer_contribution_without_credit() {
    // Arrange
    let path = temp_store_path("phase78-no-credit-peer-rotation");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
    let previous_credit =
        persist_previous_active_chain_credit(&mut runtime, i64::from(genesis.header.time));
    assert_eq!(previous_credit.credited_validated_active_chain_height, 0);
    let mut transport = ScriptedTransport::new(vec![
        vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            notfound_for_block(child_hash),
        ],
        version_verack_script(1),
    ]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(child.header.time))
        .expect("sync with no-credit peer rotation");
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time),
        )
        .expect("durable no-credit rotation status");

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::BlockNotFound)
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert_progress_credit_unavailable(&state);
    assert_eq!(
        available_last_useful_work(&state).credited_validated_active_chain_height,
        0
    );
    let last_peer_contribution = available_last_peer_contribution(&state);
    assert_eq!(
        last_peer_contribution.peer,
        SyncPeerAddress::manual("127.0.0.1", 18_445).label()
    );
    assert_eq!(
        last_peer_contribution.kind,
        PeerContributionKind::MessagesOnly
    );
    let stall = available_stall_diagnosis(&state);
    assert_eq!(
        serialized_label(stall.stalled_subsystem),
        "slow_or_stalled_peers"
    );
    assert_eq!(
        stall.stalled_subsystem,
        StalledSubsystem::SlowOrStalledPeers
    );
    assert_first_peer_backoff(&runtime);

    remove_dir_if_exists(&path);
}
