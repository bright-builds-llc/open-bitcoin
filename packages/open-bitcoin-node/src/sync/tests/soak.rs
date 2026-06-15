// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

const SYNTHETIC_SOAK_BLOCKS: usize = 96;
const SYNTHETIC_SOAK_HEADER_TIME: u32 = 1_777_225_300;
const SYNTHETIC_SOAK_CHUNK_SIZE: usize = 9;

fn synthetic_soak_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        manual_peers: vec![
            SyncPeerAddress::manual("127.0.0.1", 18_444),
            SyncPeerAddress::manual("127.0.0.1", 18_445),
        ],
        dns_seeds: Vec::new(),
        target_outbound_peers: 2,
        maybe_target_header_height: Some(95),
        max_blocks_in_flight_total: 4,
        max_messages_per_peer: 12,
        max_rounds: 64,
        max_peer_retries: 0,
        ..sync_config()
    }
}

fn synthetic_soak_blocks() -> Vec<Block> {
    let mut blocks = Vec::with_capacity(SYNTHETIC_SOAK_BLOCKS);
    let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
    for height in 0..SYNTHETIC_SOAK_BLOCKS {
        let mut block = build_block(previous_hash, height as u32);
        block.header.time = SYNTHETIC_SOAK_HEADER_TIME.saturating_add(height as u32);
        mine_header(&mut block);
        previous_hash = block_hash(&block.header);
        blocks.push(block);
    }
    blocks
}

fn header_block_scripts(blocks: &[Block]) -> Vec<Vec<WireNetworkMessage>> {
    let mut scripts = blocks
        .chunks(SYNTHETIC_SOAK_CHUNK_SIZE)
        .enumerate()
        .map(|(chunk_index, chunk)| {
            let end_height = chunk_index
                .saturating_mul(SYNTHETIC_SOAK_CHUNK_SIZE)
                .saturating_add(chunk.len())
                .saturating_sub(1);
            let headers = chunk
                .iter()
                .map(|block| block.header.clone())
                .collect::<Vec<_>>();
            let mut script = headers_script(end_height as i32, headers);
            script.extend(chunk.iter().cloned().map(WireNetworkMessage::Block));
            script
        })
        .collect::<Vec<_>>();
    if scripts.len() % 2 != 0 {
        scripts.push(version_verack_script((SYNTHETIC_SOAK_BLOCKS - 1) as i32));
    }
    scripts
}

fn all_headers(blocks: &[Block]) -> Vec<BlockHeader> {
    blocks
        .iter()
        .map(|block| block.header.clone())
        .collect::<Vec<_>>()
}

#[test]
fn phase75_synthetic_soak_long_run_reaches_target_height_without_public_network() {
    // Arrange
    let path = temp_store_path("phase75-synthetic-soak-long-run");
    remove_dir_if_exists(&path);
    let blocks = synthetic_soak_blocks();
    let config = synthetic_soak_config();
    assert_eq!(blocks.len(), SYNTHETIC_SOAK_BLOCKS);
    assert_eq!(config.manual_peers.len(), 2);
    assert!(config.dns_seeds.is_empty());
    assert_eq!(config.target_outbound_peers, 2);
    assert_eq!(config.maybe_target_header_height, Some(95));
    assert_eq!(config.max_blocks_in_flight_total, 4);
    assert_eq!(config.max_messages_per_peer, 12);
    assert_eq!(config.max_rounds, 64);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, config).expect("runtime");
    let mut transport = ScriptedTransport::new(header_block_scripts(&blocks));
    let mut resolver = ScriptedResolver::new(Vec::new());

    // Act
    let summary = runtime
        .sync_until_idle_with_resolver(
            &mut transport,
            &mut resolver,
            i64::from(blocks[0].header.time),
        )
        .expect("synthetic soak sync");

    // Assert
    assert_eq!(summary.best_header_height, 95);
    assert_eq!(summary.downloaded_block_height, 95);
    assert_eq!(summary.best_block_height, 95);
    assert_eq!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::TargetHeaderReached {
            target_header_height: 95,
            best_header_height: 95,
        })
    );
    assert_eq!(
        summary.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&blocks[95].header)))
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase75_synthetic_soak_reopen_preserves_resume_progress_without_duplicate_getdata() {
    // Arrange
    let path = temp_store_path("phase75-synthetic-soak-reopen-resume");
    remove_dir_if_exists(&path);
    let blocks = synthetic_soak_blocks();
    let config = synthetic_soak_config();
    let mut first_peer_script = headers_script(95, all_headers(&blocks));
    first_peer_script.extend(
        blocks
            .iter()
            .take(9)
            .cloned()
            .map(WireNetworkMessage::Block),
    );
    let mut second_peer_script = version_verack_script(95);
    second_peer_script.extend(
        blocks
            .iter()
            .skip(9)
            .take(10)
            .cloned()
            .map(WireNetworkMessage::Block),
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, config.clone()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, second_peer_script]);
    let mut resolver = ScriptedResolver::new(Vec::new());
    let partial_summary = runtime
        .sync_once_with_resolver(
            &mut transport,
            &mut resolver,
            i64::from(blocks[18].header.time),
        )
        .expect("partial synthetic soak sync");
    drop(runtime);

    // Act
    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let mut reopened_runtime =
        DurableSyncRuntime::open(reopened_store, config).expect("reopened runtime");
    let reopened_summary = reopened_runtime.snapshot_summary();
    let connected_index =
        usize::try_from(reopened_summary.best_block_height).expect("connected height fits usize");
    let connected_hash = block_hash(&blocks[connected_index].header);
    let next_missing_hash = block_hash(&blocks[connected_index + 1].header);
    let mut resume_transport =
        ScriptedTransport::new(vec![version_verack_script(95), version_verack_script(95)]);
    let mut resume_resolver = ScriptedResolver::new(Vec::new());
    let resume_summary = reopened_runtime
        .sync_once_with_resolver(
            &mut resume_transport,
            &mut resume_resolver,
            i64::from(blocks[connected_index + 1].header.time),
        )
        .expect("resume synthetic soak sync");
    let requested_hashes = getdata_block_hashes(&resume_transport.sent_messages());

    // Assert
    assert_eq!(partial_summary.best_header_height, 95);
    assert_eq!(partial_summary.best_block_height, 18);
    assert_eq!(
        reopened_summary.best_header_height,
        partial_summary.best_header_height
    );
    assert_eq!(
        reopened_summary.best_block_height,
        partial_summary.best_block_height
    );
    assert_eq!(
        reopened_summary.maybe_connected_block_hash,
        partial_summary.maybe_connected_block_hash
    );
    assert_eq!(
        reopened_summary.maybe_validated_active_chain_work,
        partial_summary.maybe_validated_active_chain_work
    );
    assert_eq!(
        reopened_summary.maybe_connected_block_hash,
        Some(block_hash_hex(connected_hash))
    );
    assert_eq!(
        reopened_summary.maybe_validated_active_chain_work,
        Some("19".to_string())
    );
    assert!(!requested_hashes.contains(&connected_hash));
    assert!(requested_hashes.contains(&next_missing_hash));
    assert_eq!(
        resume_summary.best_block_height,
        reopened_summary.best_block_height
    );

    drop(reopened_runtime);
    remove_dir_if_exists(&path);
}

#[test]
fn phase75_synthetic_soak_resource_stop_uses_shared_status_evidence() {
    // Arrange
    let path = temp_store_path("phase75-synthetic-soak-resource-stop");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, synthetic_soak_config()).expect("runtime");
    let summary = SyncRunSummary::empty(0, 0, 2);

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            Some("resource limit: storage cache exhausted".to_string()),
            i64::from(SYNTHETIC_SOAK_HEADER_TIME),
        )
        .expect("durable resource-stop status");

    // Assert
    assert_eq!(
        state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::ResourceExhaustion)
    );
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::StorageOrResourceBlocked,
        "Inspect storage health, free disk space for the selected datadir, or increase bounded resource limits.",
    );

    remove_dir_if_exists(&path);
}
