// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

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
