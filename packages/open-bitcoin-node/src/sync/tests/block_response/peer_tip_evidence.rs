// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase69_peer_agreement_classifies_agrees_behind_disagrees_and_no_evidence() {
    // Arrange
    let path = temp_store_path("phase69-peer-agreement");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let best_tip = build_block(block_hash(&child.header), 2);
    let child_hash = block_hash(&child.header);
    let best_tip_hash = block_hash(&best_tip.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1), (&best_tip, 2)],
        &[(&genesis, 0), (&child, 1), (&best_tip, 2)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut agrees = peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_444),
        PeerSyncState::Connected,
        1,
        None,
        None,
    );
    agrees.maybe_tip_height = Some(2);
    agrees.maybe_tip_hash = Some(block_hash_hex(best_tip_hash));
    agrees.maybe_tip_work = Some("3".to_string());
    agrees.maybe_last_activity_unix_seconds = Some(u64::from(best_tip.header.time));
    let mut behind = peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_445),
        PeerSyncState::Connected,
        1,
        None,
        None,
    );
    behind.maybe_tip_height = Some(1);
    behind.maybe_tip_hash = Some(block_hash_hex(child_hash));
    behind.maybe_tip_work = Some("2".to_string());
    behind.maybe_last_activity_unix_seconds = Some(u64::from(best_tip.header.time));
    let mut disagrees = peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_446),
        PeerSyncState::Connected,
        1,
        None,
        None,
    );
    disagrees.maybe_tip_height = Some(2);
    disagrees.maybe_tip_hash = Some("aa".repeat(32));
    disagrees.maybe_tip_work = Some("3".to_string());
    disagrees.maybe_last_activity_unix_seconds = Some(u64::from(best_tip.header.time));
    let no_evidence = peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_447),
        PeerSyncState::Connected,
        1,
        None,
        None,
    );
    let mut summary = SyncRunSummary::empty(2, 2, 4);
    summary.connected_peers = 4;
    summary.peer_outcomes = vec![agrees, behind, disagrees, no_evidence];

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            None,
            i64::from(best_tip.header.time) + 30,
        )
        .expect("durable status");

    // Assert
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.source, BestKnownTipSource::HeaderStore);
    assert_eq!(best_known_tip.height, 2);
    assert_eq!(best_known_tip.block_hash, block_hash_hex(best_tip_hash));
    assert_eq!(
        best_known_tip
            .peer_agreement
            .iter()
            .map(|row| row.status)
            .collect::<Vec<_>>(),
        vec![
            PeerTipAgreementStatus::Agrees,
            PeerTipAgreementStatus::Behind,
            PeerTipAgreementStatus::Disagrees,
            PeerTipAgreementStatus::NoEvidence,
        ]
    );
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase69_peer_tip_observation_uses_peer_terminal_header_not_global_best() {
    // Arrange
    let path = temp_store_path("phase69-peer-terminal-header");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let global_child = build_block(block_hash(&genesis.header), 1);
    let global_tip = build_block(block_hash(&global_child.header), 2);
    let peer_terminal = build_branch_block(block_hash(&genesis.header), 1, 200);
    let global_tip_hash = block_hash(&global_tip.header);
    let peer_terminal_hash = block_hash(&peer_terminal.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&global_child, 1), (&global_tip, 2)],
        &[(&genesis, 0)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    assert_eq!(
        runtime
            .network
            .peer_manager_snapshot()
            .expect("authoritative peer-manager snapshot")
            .header_store()
            .best_tip()
            .map(|entry| entry.block_hash),
        Some(global_tip_hash)
    );
    let mut transport =
        ScriptedTransport::new(vec![headers_script(2, vec![peer_terminal.header.clone()])]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(peer_terminal.header.time))
        .expect("sync summary");
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            None,
            i64::from(peer_terminal.header.time),
        )
        .expect("durable status");

    // Assert
    assert_eq!(summary.headers_received, 1);
    assert_eq!(
        runtime
            .network
            .peer_manager_snapshot()
            .expect("authoritative peer-manager snapshot")
            .header_store()
            .best_tip()
            .map(|entry| entry.block_hash),
        Some(global_tip_hash)
    );
    let outcome = summary.peer_outcomes.first().expect("peer outcome");
    assert_eq!(outcome.maybe_tip_height, Some(1));
    assert_eq!(
        outcome.maybe_tip_hash,
        Some(block_hash_hex(peer_terminal_hash))
    );
    assert_eq!(outcome.maybe_tip_work, Some("2".to_string()));
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.block_hash, block_hash_hex(global_tip_hash));
    assert_eq!(
        best_known_tip.peer_agreement.first().map(|row| row.status),
        Some(PeerTipAgreementStatus::Behind)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn sync_progress_omits_block_hashes_when_unavailable() {
    // Arrange
    let path = temp_store_path("sync-progress-no-hashes");
    remove_dir_if_exists(&path);
    let header_only_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[HeaderEntry {
                    block_hash: block_hash(&header_only_block.header),
                    header: header_only_block.header.clone(),
                    height: 0,
                    chain_work: 1,
                }],
                PersistMode::Sync,
            )
            .expect("save header");
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime.snapshot_summary();
    let status = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_181)
        .expect("durable status");
    let encoded = serde_json::to_value(&status.sync.sync_progress).expect("sync progress json");

    // Assert
    assert_eq!(
        status.sync.sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 0,
            block_height: 0,
            downloaded_block_height: 0,
            connected_block_height: 0,
            validated_active_chain_height: 0,
            maybe_downloaded_block_hash: None,
            maybe_connected_block_hash: None,
            maybe_validated_active_chain_hash: None,
            maybe_validated_active_chain_work: None,
            progress_ratio: 1.0,
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        })
    );
    assert!(encoded["value"]["maybe_downloaded_block_hash"].is_null());
    assert!(encoded["value"]["maybe_connected_block_hash"].is_null());
    assert!(encoded["value"]["maybe_validated_active_chain_hash"].is_null());
    assert!(encoded["value"]["maybe_validated_active_chain_work"].is_null());

    remove_dir_if_exists(&path);
}
