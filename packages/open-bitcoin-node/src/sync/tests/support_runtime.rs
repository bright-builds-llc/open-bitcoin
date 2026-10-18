// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

/// Explicit model-fixture initialization/re-admission; raw durable reopen supplies no receipt.
pub(super) fn admit_fixture_runtime_headers(runtime: &DurableSyncRuntime) {
    revalidate_retained_fixture_owner(runtime);
    admit_fixture_runtime_headers_only(runtime);
}

/// Protocol-only admission never promotes the actual core's existing work origin.
pub(super) fn admit_fixture_runtime_headers_only(runtime: &DurableSyncRuntime) {
    let handle = runtime.network_handle();
    let earned_before = handle
        .has_earned_fixture_work()
        .expect("actual source origin");
    let entries = handle.header_entries().expect("raw fixture bytes");
    let headers = if entries.is_empty() {
        let root = build_block(BlockHash::default(), 0);
        let mut headers = open_bitcoin_network::HeaderStore::default();
        headers
            .insert_header(root.header)
            .expect("named mined model root");
        headers
    } else {
        open_bitcoin_network::HeaderStore::from_entries(entries).expect("complete fixture ancestry")
    };
    handle
        .try_admit_fixture_headers(headers, i64::MAX, runtime.consensus_params)
        .expect("actual selected model header admission");
    assert_eq!(
        handle
            .has_earned_fixture_work()
            .expect("unchanged core origin"),
        earned_before
    );
}

pub(super) fn assert_fixture_work_unearned(runtime: &DurableSyncRuntime) {
    assert!(
        !runtime
            .network_handle()
            .has_earned_fixture_work()
            .expect("header/protocol processing never promotes the raw core")
    );
}

/// A new fixture seed contains only complete facts derived from actual validated bodies.
/// The saved bytes remain untrusted until the existing strict live-owner replay succeeds.
pub(super) fn seed_genuine_fixture_history(
    path: &Path,
    headers: &[(&Block, u32)],
    active: &[(&Block, u32)],
) {
    assert!(
        !path.exists(),
        "a genuine fixture seed never repairs an existing raw datadir"
    );
    let params = open_bitcoin_core::consensus::ConsensusParams::default();
    let mut source = open_bitcoin_core::chainstate::Chainstate::new();
    let mut checked_headers = open_bitcoin_network::HeaderStore::default();
    for (block, expected_height) in active {
        let position = source
            .connect_block_with_current_time(
                block,
                open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                i64::from(block.header.time),
                open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
                params,
            )
            .expect("actual full active fixture validation");
        assert_eq!(position.height, *expected_height);
        checked_headers
            .insert_header_with_params(block.header.clone(), &params)
            .expect("actual active header");
    }
    for (block, height) in headers {
        let inserted = checked_headers
            .insert_header_with_params(block.header.clone(), &params)
            .expect("actual retained header target/parent");
        assert_eq!(inserted.height, *height);
    }
    let store = FjallNodeStore::open(path).expect("new fixture store");
    let snapshot = source.snapshot();
    store
        .save_header_entries(
            &checked_headers.entries().cloned().collect::<Vec<_>>(),
            PersistMode::Sync,
        )
        .expect("raw checked fixture header data");
    store
        .save_chainstate_snapshot(&snapshot, PersistMode::Sync)
        .expect("actual complete fixture snapshot");
    store
        .seed_coins_from_snapshot(&snapshot)
        .expect("same actual complete coins/undo/counts fixture");
    for (block, _) in active {
        store
            .save_block(block, PersistMode::Sync)
            .expect("actual retained fixture active body");
    }
}

/// Initializes a named mined model root outside ordinary P2P; canonical guards still apply.
pub(super) fn admit_fixture_runtime_root(runtime: &DurableSyncRuntime, root: BlockHeader) {
    revalidate_retained_fixture_owner(runtime);
    let mut headers = open_bitcoin_network::HeaderStore::default();
    headers
        .insert_header_with_params(root, &runtime.consensus_params)
        .expect("actual named model root");
    runtime
        .network_handle()
        .try_admit_fixture_headers(headers, i64::MAX, runtime.consensus_params)
        .expect("controlled model root admission");
}

/// Positive empty-root models initialize the body explicitly; header-only models do not.
pub(super) fn initialize_fixture_runtime_genesis(runtime: &DurableSyncRuntime, genesis: &Block) {
    revalidate_retained_fixture_owner(runtime);
    let handle = runtime.network_handle();
    assert!(
        handle
            .maybe_chain_tip()
            .expect("empty model state")
            .is_none()
    );
    if handle
        .best_chain_entries()
        .expect("header receipts")
        .is_empty()
    {
        admit_fixture_runtime_root(runtime, genesis.header.clone());
    }
    let root = handle.best_chain_entries().expect("accepted root")[0].clone();
    assert_eq!(root.block_hash, block_hash(&genesis.header));
    handle
        .connect_stored_block(
            genesis,
            root.chain_work,
            i64::from(genesis.header.time),
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("controlled actual model genesis body");
    runtime
        .store()
        .save_block(genesis, PersistMode::Sync)
        .expect("retain actual fixture genesis for complete prefix revalidation");
}

/// Fixture evidence only: actual retained active bodies are revalidated in a new
/// owner. Missing history leaves raw state untrusted; it never earns a reopen receipt.
fn revalidate_retained_fixture_owner(runtime: &DurableSyncRuntime) {
    if runtime
        .network_handle()
        .has_earned_fixture_work()
        .expect("live owner observation")
    {
        return;
    }
    let snapshot = runtime
        .network_handle()
        .chainstate_snapshot()
        .expect("fixture target facts");
    let mut genuine = open_bitcoin_core::chainstate::Chainstate::new();
    for position in &snapshot.active_chain {
        let Some(block) = runtime
            .store()
            .load_block(position.block_hash)
            .expect("fixture body read")
        else {
            return;
        };
        genuine
            .connect_block_with_current_time(
                &block,
                open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                i64::from(block.header.time),
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("actual complete active fixture prefix validation");
    }
    for hash in genuine.undo_by_block().keys().copied().collect::<Vec<_>>() {
        if !snapshot.undo_by_block.contains_key(&hash) {
            genuine.forget_undo(hash);
        }
    }
    assert_eq!(
        genuine.snapshot().maybe_confirmed_txid_counts,
        snapshot.maybe_confirmed_txid_counts,
        "fixture replay requires exact confirmation-count identity"
    );
    runtime
        .network_handle()
        .adopt_live_fixture_owner(genuine)
        .expect("complete matching live fixture state transfer");
}

pub(super) fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-sync-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

pub(super) fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

pub(super) fn sync_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        manual_peers: vec![SyncPeerAddress::manual("127.0.0.1", 18_444)],
        dns_seeds: Vec::new(),
        max_messages_per_peer: 16,
        persist_mode: PersistMode::Sync,
        ..SyncRuntimeConfig::default()
    }
}

pub(super) fn sync_config_with_log_dir(log_dir: &Path) -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        maybe_log_dir: Some(log_dir.to_path_buf()),
        ..sync_config()
    }
}

pub(super) fn two_peer_sync_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        manual_peers: vec![
            SyncPeerAddress::manual("127.0.0.1", 18_444),
            SyncPeerAddress::manual("127.0.0.1", 18_445),
        ],
        target_outbound_peers: 2,
        max_peer_retries: 0,
        ..sync_config()
    }
}

pub(super) fn connect_runtime_peer(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
    start_height: i32,
) {
    runtime
        .network
        .connect_outbound_peer(peer_id, 1_777_225_210)
        .expect("connect peer");
    runtime
        .network
        .receive_sync_message(
            peer_id,
            WireNetworkMessage::Version(VersionMessage {
                start_height,
                ..VersionMessage::default()
            }),
            1_777_225_210,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("receive version");
    runtime
        .network
        .receive_sync_message(
            peer_id,
            WireNetworkMessage::Verack,
            1_777_225_210,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("receive verack");
}

pub(super) fn durable_tip_capture(runtime: &mut DurableSyncRuntime) -> Arc<Mutex<Vec<BlockHash>>> {
    let captured = Arc::new(Mutex::new(Vec::new()));
    let captured_for_sink = Arc::clone(&captured);
    runtime.set_durable_tip_announcement_sink(move |event| {
        captured_for_sink
            .lock()
            .expect("durable tip capture lock")
            .push(block_hash(&event.block().header));
        Ok(())
    });
    captured
}

pub(super) fn version_verack_script(start_height: i32) -> Vec<WireNetworkMessage> {
    vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
    ]
}

pub(super) fn headers_script(
    start_height: i32,
    headers: Vec<BlockHeader>,
) -> Vec<WireNetworkMessage> {
    vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage { headers }),
    ]
}

pub(super) fn load_structured_log_records(log_dir: &Path) -> Vec<StructuredLogRecord> {
    let mut records = Vec::new();
    for entry in fs::read_dir(log_dir).expect("read log directory") {
        let path = entry.expect("read log entry").path();
        if !path.is_file() {
            continue;
        }
        let contents = fs::read_to_string(&path).expect("read structured log file");
        for line in contents.lines().filter(|line| !line.trim().is_empty()) {
            records.push(serde_json::from_str(line).expect("structured log record"));
        }
    }
    records
}

pub(super) fn flush_coins_always(runtime: &DurableSyncRuntime) {
    runtime
        .network_handle()
        .flush_coins(
            open_bitcoin_core::chainstate::FlushMode::Always,
            open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(0),
            u64::MAX,
        )
        .expect("always flush coins B");
    if runtime
        .network_handle()
        .has_earned_fixture_work()
        .expect("genuine live fixture origin")
    {
        runtime
            .network_handle()
            .persist_missing_live_fixture_counts()
            .expect("fixture-enriched counts from actual live owner after forceflush");
    }
}
