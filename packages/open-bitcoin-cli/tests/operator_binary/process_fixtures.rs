use super::*;

pub(super) static NEXT_SANDBOX_ID: AtomicU64 = AtomicU64::new(0);
pub(super) static SANDBOX_MUTEX: Mutex<()> = Mutex::new(());
pub(super) const RECEIVE_DESCRIPTOR: &str =
    "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)";
pub(super) const CHANGE_DESCRIPTOR: &str =
    "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))";

pub(super) struct TestSandbox {
    pub(super) home: PathBuf,
    pub(super) _guard: MutexGuard<'static, ()>,
}

impl TestSandbox {
    pub(super) fn new(label: &str) -> Self {
        let guard = match SANDBOX_MUTEX.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let home = std::env::temp_dir().join(format!(
            "open-bitcoin-operator-binary-{label}-{}",
            NEXT_SANDBOX_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&home).expect("sandbox");
        Self {
            home,
            _guard: guard,
        }
    }

    pub(super) fn child(&self, relative: &str) -> PathBuf {
        self.home.join(relative)
    }
}

impl Drop for TestSandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.home);
    }
}

pub(super) fn onboard_args<'a>(
    data_dir: &'a Path,
    config_path: &'a Path,
    extra: &'a [&'a str],
) -> Vec<&'a str> {
    let mut args = vec![
        "--network",
        "regtest",
        "--datadir",
        data_dir.to_str().expect("datadir"),
        "--config",
        config_path.to_str().expect("config"),
        "onboard",
        "--non-interactive",
        "--approve-write",
        "--detect-existing",
    ];
    args.extend_from_slice(extra);
    args
}

pub(super) fn soak_start_args<'a>(data_dir: &'a Path, run_id: &'a str) -> Vec<&'a str> {
    vec![
        "--datadir",
        data_dir.to_str().expect("datadir"),
        "--network",
        "regtest",
        "--format",
        "json",
        "soak",
        "start",
        "--elapsed-time-seconds",
        "60",
        "--checkpoint-interval-seconds",
        "15",
        "--target-height",
        "840004",
        "--peer-policy",
        "daemon-configured",
        "--disk-budget-bytes",
        "1048576",
        "--stop-condition",
        "target-height",
        "--run-id",
        run_id,
    ]
}

pub(super) fn soak_target_height_start_args<'a>(
    data_dir: &'a Path,
    run_id: &'a str,
) -> Vec<&'a str> {
    vec![
        "--datadir",
        data_dir.to_str().expect("datadir"),
        "--network",
        "regtest",
        "--format",
        "json",
        "soak",
        "start",
        "--elapsed-time-seconds",
        "60",
        "--checkpoint-interval-seconds",
        "15",
        "--target-height",
        "840004",
        "--peer-policy",
        "daemon-configured",
        "--disk-budget-bytes",
        "1048576",
        "--stop-condition",
        "target-height",
        "--run-id",
        run_id,
    ]
}

pub(super) fn run_open_bitcoin<const N: usize>(sandbox: &TestSandbox, args: [&str; N]) -> Output {
    run_open_bitcoin_vec(sandbox, args.to_vec())
}

pub(super) fn run_open_bitcoin_vec(sandbox: &TestSandbox, args: Vec<&str>) -> Output {
    Command::new(env!("CARGO_BIN_EXE_open-bitcoin"))
        .args(args)
        .env("HOME", &sandbox.home)
        .output()
        .expect("run open-bitcoin")
}

pub(super) fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

pub(super) fn assert_failure(output: &Output) {
    assert!(
        !output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

pub(super) fn assert_no_store_lock_error(output: &Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stdout.contains("FjallError"), "stdout={stdout}");
    assert!(!stdout.contains("Locked"), "stdout={stdout}");
    assert!(!stderr.contains("FjallError"), "stderr={stderr}");
    assert!(!stderr.contains("Locked"), "stderr={stderr}");
}

pub(super) fn assert_absent(text: &str, value: &str) {
    assert!(
        !text.contains(value),
        "unexpected sensitive value in {text}"
    );
}

pub(super) fn read_jsonl_values(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .expect("jsonl file")
        .lines()
        .map(|line| serde_json::from_str(line).expect("jsonl value"))
        .collect()
}

pub(super) fn seed_managed_wallet(data_dir: &Path, wallet_name: &str) {
    fs::create_dir_all(data_dir).expect("open datadir");
    let store = FjallNodeStore::open(data_dir).expect("store");
    let mut registry = WalletRegistry::default();
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor("receive", DescriptorRole::External, RECEIVE_DESCRIPTOR)
        .expect("receive descriptor");
    wallet
        .import_descriptor("change", DescriptorRole::Internal, CHANGE_DESCRIPTOR)
        .expect("change descriptor");
    registry
        .create_wallet(&store, wallet_name, wallet, PersistMode::Sync)
        .expect("create wallet");
    registry
        .set_selected_wallet(&store, wallet_name, PersistMode::Sync)
        .expect("select wallet");
}

pub(super) fn write_rpc_conf(data_dir: &Path, rpc_port: u16) {
    fs::write(
        data_dir.join("bitcoin.conf"),
        format!(
            "regtest=1\nrpcconnect=127.0.0.1\nrpcport={rpc_port}\nrpcuser=alice\nrpcpassword=secret\n"
        ),
    )
    .expect("bitcoin.conf");
}

pub(super) fn seed_phase72_runtime_metadata(data_dir: &Path, missing_active_chain: bool) {
    fs::create_dir_all(data_dir).expect("open datadir");
    let store = FjallNodeStore::open(data_dir).expect("store");
    store
        .save_runtime_metadata(
            &RuntimeMetadata {
                maybe_sync_state: Some(phase72_durable_sync_state(missing_active_chain)),
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save runtime metadata");
}

pub(super) fn phase72_durable_sync_state(missing_active_chain: bool) -> DurableSyncState {
    DurableSyncState {
        sync: phase72_sync_status(missing_active_chain),
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 3,
            }),
            recent_peers: FieldAvailability::unavailable("peer telemetry unavailable"),
            inbound: inbound_status_unavailable(),
        },
        health_signals: Vec::new(),
        updated_at_unix_seconds: 1_717_000_020,
    }
}

pub(super) fn phase72_sync_status(missing_active_chain: bool) -> SyncStatus {
    let maybe_hash = (!missing_active_chain).then(|| "11".repeat(32));
    let maybe_work = (!missing_active_chain).then(|| "840005".to_string());
    SyncStatus {
        network: FieldAvailability::available("mainnet".to_string()),
        chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
        sync_progress: FieldAvailability::available(SyncProgress {
            header_height: 840_004,
            block_height: 840_004,
            downloaded_block_height: 840_004,
            connected_block_height: 840_004,
            validated_active_chain_height: 840_004,
            maybe_downloaded_block_hash: maybe_hash.clone(),
            maybe_connected_block_hash: maybe_hash.clone(),
            maybe_validated_active_chain_hash: maybe_hash,
            maybe_validated_active_chain_work: maybe_work,
            progress_ratio: 1.0,
            messages_processed: 128,
            headers_received: 4,
            blocks_received: 4,
        }),
        lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
        phase: FieldAvailability::available("blocks".to_string()),
        configured_targets: FieldAvailability::available(SyncConfiguredTargets {
            target_outbound_peers: 4,
            maybe_target_header_height: Some(840_004),
        }),
        attempt_counters: FieldAvailability::available(SyncAttemptCounters {
            attempted_peers: 4,
            connected_peers: 3,
            failed_peers: 1,
            max_sync_rounds: 8,
        }),
        progress_signal: FieldAvailability::available(SyncProgressSignal::Steady),
        lag: FieldAvailability::available(SyncLagStatus {
            headers_remaining: 0,
            blocks_remaining: 0,
        }),
        last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_020),
        progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
        expected_progress_window: FieldAvailability::unavailable(
            "expected progress window unavailable",
        ),
        no_progress_threshold: FieldAvailability::unavailable(
            "no-progress threshold evidence unavailable",
        ),
        last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
        last_peer_contribution: FieldAvailability::unavailable(
            "last peer contribution unavailable",
        ),
        stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
        latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
            label: "best_known_tip_reached".to_string(),
            message: "best known tip reached".to_string(),
        }),
        last_error: FieldAvailability::unavailable("sync error unavailable"),
        recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
        recovery_action: FieldAvailability::unavailable(
            "daemon sync recovery guidance unavailable",
        ),
        resource_pressure: FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 1,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 16,
            max_blocks_in_flight_total: 64,
            max_messages_per_peer: 64,
            max_sync_rounds: 8,
            outbound_peers: 4,
            target_outbound_peers: 4,
        }),
        best_known_tip: FieldAvailability::available(BestKnownTipStatus {
            source: BestKnownTipSource::HeaderStore,
            height: 840_004,
            block_hash: "11".repeat(32),
            work: "840005".to_string(),
            block_time_unix_seconds: 1_717_000_010,
            observed_at_unix_seconds: 1_717_000_020,
            freshness: TipFreshnessStatus::Fresh,
            peer_agreement: vec![PeerTipAgreement {
                peer: "peer-1".to_string(),
                maybe_resolved_endpoint: None,
                status: PeerTipAgreementStatus::Agrees,
                maybe_height: Some(840_004),
                maybe_hash: Some("11".repeat(32)),
                maybe_work: Some("840005".to_string()),
                maybe_last_activity_unix_seconds: Some(1_717_000_020),
            }],
        }),
        stay_current: FieldAvailability::available(StayCurrentStatus::InitialCatchUp),
        stay_current_next_action: FieldAvailability::available(
            "Wait for best-known tip catch-up evidence.".to_string(),
        ),
        no_progress_diagnosis: FieldAvailability::available(
            NoProgressDiagnosis::CurrentAtBestKnownTip,
        ),
        no_progress_next_action: FieldAvailability::available(
            "No operator action required.".to_string(),
        ),
        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
        reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
    }
}

pub(super) struct FakeRpcServer {
    pub(super) address: SocketAddr,
    pub(super) requests: Arc<Mutex<Vec<String>>>,
    pub(super) stop: std::sync::mpsc::Sender<()>,
    pub(super) join_handle: Option<thread::JoinHandle<()>>,
}

impl FakeRpcServer {
    pub(super) fn start() -> Self {
        Self::start_with_behavior(FakeRpcBehavior::Normal)
    }

    pub(super) fn start_unauthorized() -> Self {
        Self::start_with_behavior(FakeRpcBehavior::Unauthorized)
    }

    pub(super) fn start_with_behavior(behavior: FakeRpcBehavior) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
        listener.set_nonblocking(true).expect("nonblocking");
        let address = listener.local_addr().expect("addr");
        let requests = Arc::new(Mutex::new(Vec::new()));
        let (stop, stop_rx) = std::sync::mpsc::channel();
        let (ready, ready_rx) = std::sync::mpsc::channel();
        let request_log = Arc::clone(&requests);
        let join_handle = thread::spawn(move || {
            let _ = ready.send(());
            loop {
                if stop_rx.try_recv().is_ok() {
                    break;
                }
                match listener.accept() {
                    Ok((stream, _)) => handle_rpc_connection(stream, &request_log, behavior),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("listener failed: {error}"),
                }
            }
        });
        ready_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("server ready");
        Self {
            address,
            requests,
            stop,
            join_handle: Some(join_handle),
        }
    }

    pub(super) fn requests(&self) -> Vec<String> {
        self.requests.lock().expect("request log").clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FakeRpcBehavior {
    Normal,
    Unauthorized,
}

impl Drop for FakeRpcServer {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.join().expect("server thread");
        }
    }
}
