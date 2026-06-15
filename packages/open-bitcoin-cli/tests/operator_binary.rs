// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use open_bitcoin_node::{
    DurableSyncState, FjallNodeStore, PersistMode, RuntimeMetadata, WalletRegistry,
    core::wallet::{AddressNetwork, DescriptorRole, Wallet},
    status::{
        BestKnownTipSource, BestKnownTipStatus, FieldAvailability, NoProgressDiagnosis, PeerCounts,
        PeerStatus, PeerTipAgreement, PeerTipAgreementStatus, StayCurrentStatus,
        SyncAttemptCounters, SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState,
        SyncProgress, SyncProgressSignal, SyncResourcePressure, SyncStatus, SyncStopReasonStatus,
        TipFreshnessStatus,
    },
};
use serde_json::{Value, json};

static NEXT_SANDBOX_ID: AtomicU64 = AtomicU64::new(0);
const RECEIVE_DESCRIPTOR: &str = "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)";
const CHANGE_DESCRIPTOR: &str = "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))";

struct TestSandbox {
    home: PathBuf,
}

impl TestSandbox {
    fn new(label: &str) -> Self {
        let home = std::env::temp_dir().join(format!(
            "open-bitcoin-operator-binary-{label}-{}",
            NEXT_SANDBOX_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&home).expect("sandbox");
        Self { home }
    }

    fn child(&self, relative: &str) -> PathBuf {
        self.home.join(relative)
    }
}

impl Drop for TestSandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.home);
    }
}

#[test]
fn open_bitcoin_status_json_succeeds_for_stopped_node() {
    // Arrange
    let sandbox = TestSandbox::new("stopped-json");
    let data_dir = sandbox.child("open-data");
    let core_dir = sandbox.child(".bitcoin");
    fs::create_dir_all(&data_dir).expect("open datadir");
    fs::create_dir_all(&core_dir).expect("core datadir");
    fs::write(core_dir.join("bitcoin.conf"), "regtest=1\n").expect("core config");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "status",
            "--format",
            "json",
        ],
    );

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("status json");
    for field in [
        "node",
        "config",
        "service",
        "sync",
        "peers",
        "mempool",
        "wallet",
        "logs",
        "metrics",
        "health_signals",
        "build",
    ] {
        assert!(decoded.get(field).is_some(), "missing {field}");
    }
    assert_eq!(decoded["node"]["state"], "stopped");
    assert_eq!(decoded["sync"]["network"]["state"], "unavailable");
    let rendered = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(rendered.contains(core_dir.join("bitcoin.conf").to_str().expect("core path")));
    assert!(rendered.contains("uncertain"));
}

#[test]
fn open_bitcoin_status_json_uses_fake_running_rpc() {
    // Arrange
    let sandbox = TestSandbox::new("running-json");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let server = FakeRpcServer::start();
    fs::write(
        data_dir.join("bitcoin.conf"),
        format!(
            "rpcconnect=127.0.0.1\nrpcport={}\nrpcuser=alice\nrpcpassword=secret\n",
            server.address.port()
        ),
    )
    .expect("bitcoin.conf");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "status",
            "--format",
            "json",
        ],
    );

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("status json");
    assert_eq!(decoded["node"]["state"], "running");
    assert_eq!(decoded["sync"]["network"]["value"], "regtest");
    assert_eq!(decoded["sync"]["chain_tip"]["value"]["height"], 144);
    assert_eq!(decoded["peers"]["peer_counts"]["value"]["outbound"], 5);
    assert_eq!(decoded["mempool"]["transactions"]["value"], 12);
    assert_eq!(decoded["wallet"]["trusted_balance_sats"]["value"], 50_000);
}

#[test]
fn open_bitcoin_sync_pause_and_resume_update_durable_control_state() {
    // Arrange
    let sandbox = TestSandbox::new("sync-control");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let store = FjallNodeStore::open(&data_dir).expect("store");
    store
        .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
        .expect("save runtime metadata");
    drop(store);

    // Act
    let pause_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "pause",
        ],
    );
    let paused_status = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "json",
            "sync",
            "status",
        ],
    );
    let resume_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "resume",
        ],
    );
    let resumed_status = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "json",
            "sync",
            "status",
        ],
    );

    // Assert
    assert_success(&pause_output);
    assert_success(&resume_output);
    let paused: Value = serde_json::from_slice(&paused_status.stdout).expect("paused status");
    let resumed: Value = serde_json::from_slice(&resumed_status.stdout).expect("resumed status");
    assert_eq!(paused["sync_control"]["paused"], json!(true));
    assert_eq!(resumed["sync_control"]["paused"], json!(false));
}

#[test]
fn sync_control_uses_live_rpc_when_datadir_store_is_locked() {
    // Arrange
    let sandbox = TestSandbox::new("sync-control-live-rpc");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let server = FakeRpcServer::start();
    write_rpc_conf(&data_dir, server.address.port());
    let store = FjallNodeStore::open(&data_dir).expect("store");
    store
        .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
        .expect("save runtime metadata");
    let _store_guard = store;

    // Act
    let status_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "json",
            "sync",
            "status",
        ],
    );
    let pause_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "pause",
        ],
    );
    let resume_output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "resume",
        ],
    );

    // Assert
    assert_success(&status_output);
    assert_success(&pause_output);
    assert_success(&resume_output);
    let status: Value = serde_json::from_slice(&status_output.stdout).expect("status json");
    assert_eq!(status["sync_control"]["paused"], json!(false));
    assert_no_store_lock_error(&status_output);
    assert_no_store_lock_error(&pause_output);
    assert_no_store_lock_error(&resume_output);
    let requests = server.requests().join("\n");
    assert!(requests.contains("openbitcoinsyncstatus"));
    assert!(requests.contains("openbitcoinsyncpause"));
    assert!(requests.contains("openbitcoinsyncresume"));
}

#[test]
fn sync_control_auth_failure_does_not_fallback_to_store() {
    // Arrange
    let sandbox = TestSandbox::new("sync-control-auth");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let server = FakeRpcServer::start_unauthorized();
    write_rpc_conf(&data_dir, server.address.port());
    let store = FjallNodeStore::open(&data_dir).expect("store");
    store
        .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
        .expect("save runtime metadata");
    let _store_guard = store;

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "sync",
            "pause",
        ],
    );

    // Assert
    assert_failure(&output);
    assert_no_store_lock_error(&output);
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("RPC authentication failed for operator sync command"));
    let requests = server.requests().join("\n");
    assert!(requests.contains("openbitcoinsyncpause"));
}

#[test]
fn open_bitcoin_soak_start_writes_durable_ledger_and_reports() {
    // Arrange
    let sandbox = TestSandbox::new("soak-phase75-start");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    seed_phase72_runtime_metadata(&data_dir, false);

    // Act
    let output = run_open_bitcoin_vec(&sandbox, soak_start_args(&data_dir, "soak-1700000000-0001"));

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("soak start json");
    let run_dir = data_dir.join("soak/runs/soak-1700000000-0001");
    let index_path = data_dir.join("soak/run-index.json");
    let events_path = run_dir.join("events.jsonl");
    let report_json_path = run_dir.join("report.json");
    let report_markdown_path = run_dir.join("report.md");
    assert_eq!(decoded["run_id"], json!("soak-1700000000-0001"));
    assert_eq!(
        decoded["ledger_path"],
        json!(events_path.display().to_string())
    );
    assert_eq!(
        decoded["json_report_path"],
        json!(report_json_path.display().to_string())
    );
    assert_eq!(
        decoded["markdown_report_path"],
        json!(report_markdown_path.display().to_string())
    );
    assert!(index_path.exists());
    assert!(events_path.exists());
    assert!(report_json_path.exists());
    assert!(report_markdown_path.exists());
    let index: Value =
        serde_json::from_str(&fs::read_to_string(index_path).expect("run index json"))
            .expect("run index");
    assert_eq!(index["runs"][0]["run_id"], json!("soak-1700000000-0001"));
    let events = read_jsonl_values(&events_path);
    assert_eq!(events[0]["event"]["kind"], json!("started"));
    assert!(
        events
            .iter()
            .any(|event| event["event"]["kind"] == json!("checkpoint"))
    );
    let report: Value =
        serde_json::from_str(&fs::read_to_string(report_json_path).expect("report json"))
            .expect("report");
    assert_eq!(report["is_projection"], json!(true));
    assert_eq!(report["run_id"], json!("soak-1700000000-0001"));
}

#[test]
fn open_bitcoin_soak_stop_rejects_terminal_verdict() {
    // Arrange
    let sandbox = TestSandbox::new("soak-phase75-stop");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    seed_phase72_runtime_metadata(&data_dir, false);
    let start = run_open_bitcoin_vec(&sandbox, soak_start_args(&data_dir, "soak-1700000000-0001"));
    assert_success(&start);

    // Act
    let stop = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--network",
            "regtest",
            "--format",
            "json",
            "soak",
            "stop",
            "--run-id",
            "soak-1700000000-0001",
            "--reason",
            "operator-stop",
        ],
    );

    // Assert
    assert_failure(&stop);
    let stderr = String::from_utf8(stop.stderr).expect("stderr utf8");
    assert!(stderr.contains("already has a terminal verdict"));
    let run_dir = data_dir.join("soak/runs/soak-1700000000-0001");
    let events = read_jsonl_values(&run_dir.join("events.jsonl"));
    assert!(!events.iter().any(|event| {
        event["event"]["kind"] == json!("verdict")
            && event["event"]["outcome"] == json!("operator_stop")
    }));
    let report: Value =
        serde_json::from_str(&fs::read_to_string(run_dir.join("report.json")).expect("report"))
            .expect("report json");
    assert_eq!(report["verdict"]["outcome"], json!("clean_completion"));
}

#[test]
fn open_bitcoin_soak_report_is_projection_without_ledger_append() {
    // Arrange
    let sandbox = TestSandbox::new("soak-phase75-report");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    seed_phase72_runtime_metadata(&data_dir, false);
    let start = run_open_bitcoin_vec(&sandbox, soak_start_args(&data_dir, "soak-1700000000-0001"));
    assert_success(&start);
    let events_path = data_dir.join("soak/runs/soak-1700000000-0001/events.jsonl");
    let before_count = read_jsonl_values(&events_path).len();

    // Act
    let report = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--network",
            "regtest",
            "--format",
            "json",
            "soak",
            "report",
            "--run-id",
            "soak-1700000000-0001",
        ],
    );
    let after_count = read_jsonl_values(&events_path).len();

    // Assert
    assert_success(&report);
    assert_eq!(before_count, after_count);
    let decoded: Value = serde_json::from_slice(&report.stdout).expect("soak report json");
    assert_eq!(decoded["run_id"], json!("soak-1700000000-0001"));
    assert_eq!(
        decoded["ledger_path"],
        json!(events_path.display().to_string())
    );
}

#[test]
fn open_bitcoin_soak_resume_refuses_clean_completion() {
    // Arrange
    let sandbox = TestSandbox::new("soak-phase75-clean-resume");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    seed_phase72_runtime_metadata(&data_dir, false);
    let start = run_open_bitcoin_vec(
        &sandbox,
        soak_target_height_start_args(&data_dir, "soak-1700000000-0001"),
    );
    assert_success(&start);
    let report: Value = serde_json::from_str(
        &fs::read_to_string(data_dir.join("soak/runs/soak-1700000000-0001/report.json"))
            .expect("report"),
    )
    .expect("report json");
    assert_eq!(report["verdict"]["outcome"], json!("clean_completion"));

    // Act
    let resume = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--network",
            "regtest",
            "--format",
            "json",
            "soak",
            "resume",
            "--run-id",
            "soak-1700000000-0001",
            "--checkpoint-interval-seconds",
            "15",
        ],
    );

    // Assert
    assert_failure(&resume);
    let stderr = String::from_utf8(resume.stderr).expect("stderr utf8");
    assert!(stderr.contains("clean_completion"));
}

#[test]
fn http_request_complete_waits_for_lowercase_content_length_body() {
    // Arrange
    let headers = b"POST / HTTP/1.1\r\ncontent-length: 5\r\n\r\n";
    let incomplete_request = [headers.as_slice(), b"abc"].concat();
    let complete_request = [headers.as_slice(), b"abcde"].concat();

    // Act
    let incomplete_is_complete = http_request_complete(&incomplete_request);
    let complete_is_complete = http_request_complete(&complete_request);

    // Assert
    assert!(!incomplete_is_complete);
    assert!(complete_is_complete);
}

#[test]
fn fake_rpc_server_responses_declare_connection_close() {
    // Arrange
    let server = FakeRpcServer::start();
    let request_body = r#"{"jsonrpc":"2.0","method":"getblockchaininfo","params":[],"id":1}"#;
    let request = format!(
        "POST / HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: {}\r\n\r\n{}",
        request_body.len(),
        request_body
    );
    let mut stream = TcpStream::connect(server.address).expect("connect fake rpc");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("read timeout");

    // Act
    stream.write_all(request.as_bytes()).expect("write request");
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("read fake rpc response");

    // Assert
    assert!(
        response.contains("\r\nConnection: close\r\n"),
        "response did not declare connection close: {response}"
    );
}

#[test]
fn open_bitcoin_status_human_no_color_is_support_oriented() {
    // Arrange
    let sandbox = TestSandbox::new("human");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "status",
            "--format",
            "human",
            "--no-color",
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    for label in [
        "Daemon:", "Version:", "Datadir:", "Config:", "Network:", "Chain:", "Sync:", "Peers:",
        "Mempool:", "Wallet:", "Service:", "Logs:", "Metrics:", "Health:",
    ] {
        assert!(stdout.contains(label), "missing {label}");
    }
    assert!(!stdout.contains("\u{1b}["));
}

#[test]
fn open_bitcoin_dashboard_json_is_snapshot_and_ansi_free() {
    // Arrange
    let sandbox = TestSandbox::new("dashboard-json");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "json",
            "dashboard",
        ],
    );

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("dashboard json");
    assert_eq!(decoded["node"]["state"], "stopped");
    assert_eq!(decoded["metrics"]["samples"], json!([]));
    let rendered = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(!rendered.contains("\u{1b}["));
    assert!(!rendered.contains("dashboard command is deferred"));
}

#[test]
fn open_bitcoin_dashboard_human_non_tty_uses_snapshot_sections() {
    // Arrange
    let sandbox = TestSandbox::new("dashboard-human");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "human",
            "--no-color",
            "dashboard",
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    for label in [
        "Open Bitcoin Dashboard",
        "## Node",
        "## Sync and Peers",
        "## Mempool and Wallet",
        "## Service",
        "## Logs and Health",
        "## Charts",
        "## Actions",
    ] {
        assert!(stdout.contains(label), "missing {label}");
    }
    assert!(!stdout.contains("\u{1b}["));
    assert!(!stdout.contains("dashboard command is deferred"));
}

#[test]
fn open_bitcoin_onboard_non_interactive_is_idempotent() {
    // Arrange
    let sandbox = TestSandbox::new("onboard");
    let data_dir = sandbox.child("open-data");
    let config_path = data_dir.join("open-bitcoin.jsonc");
    let core_dir = sandbox.child(".bitcoin");
    fs::create_dir_all(&core_dir).expect("core datadir");
    fs::write(core_dir.join("bitcoin.conf"), "regtest=1\n").expect("core config");

    // Act
    let first = run_open_bitcoin_vec(&sandbox, onboard_args(&data_dir, &config_path, &[]));
    let first_contents = fs::read_to_string(&config_path).expect("first config");
    let second = run_open_bitcoin_vec(&sandbox, onboard_args(&data_dir, &config_path, &[]));
    let second_contents = fs::read_to_string(&config_path).expect("second config");
    let forced = run_open_bitcoin_vec(
        &sandbox,
        onboard_args(&data_dir, &config_path, &["--force-overwrite"]),
    );

    // Assert
    assert_success(&first);
    assert_success(&second);
    assert_success(&forced);
    assert_eq!(first_contents, second_contents);
    assert!(first_contents.contains("\"onboarding\""));
    assert!(first_contents.contains("\"wizard_answers\""));
    assert!(first_contents.contains("\"network\""));
    assert!(first_contents.contains("\"datadir\""));
    assert!(first_contents.contains("\"metrics\""));
    assert!(first_contents.contains("\"logs\""));
    assert!(first_contents.contains("\"migration\""));
    let first_stdout = String::from_utf8(first.stdout).expect("stdout utf8");
    let second_stdout = String::from_utf8(second.stdout).expect("stdout utf8");
    assert!(first_stdout.contains(core_dir.join("bitcoin.conf").to_str().expect("core path")));
    assert!(first_stdout.contains("confidence="));
    assert!(second_stdout.contains("left unchanged"));
    assert!(!data_dir.join("bitcoin.conf").exists());
}

#[test]
fn open_bitcoin_migrate_plan_is_dry_run_only_for_detected_source_install() {
    // Arrange
    let sandbox = TestSandbox::new("migrate-plan");
    let target_data_dir = sandbox.child("open-data");
    let source_data_dir = sandbox.child(".bitcoin");
    let source_wallet_dir = source_data_dir.join("wallets/main");
    fs::create_dir_all(&source_wallet_dir).expect("source wallet dir");
    fs::write(source_data_dir.join("bitcoin.conf"), "regtest=1\n").expect("source config");
    fs::write(source_data_dir.join(".cookie"), "__cookie__:secret\n").expect("source cookie");
    fs::write(source_wallet_dir.join("wallet.dat"), "legacy wallet bytes").expect("source wallet");

    #[cfg(target_os = "macos")]
    let source_service_path = {
        let path = sandbox.child("Library/LaunchAgents/org.bitcoin.bitcoind.plist");
        fs::create_dir_all(path.parent().expect("launchagents parent")).expect("launchagents");
        fs::write(
            &path,
            format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
<plist version=\"1.0\">\n\
<dict>\n\
    <key>Label</key>\n\
    <string>org.bitcoin.bitcoind</string>\n\
    <key>ProgramArguments</key>\n\
    <array>\n\
        <string>/usr/local/bin/bitcoind</string>\n\
        <string>-conf</string>\n\
        <string>{}</string>\n\
        <string>-datadir</string>\n\
        <string>{}</string>\n\
    </array>\n\
    <key>RunAtLoad</key>\n\
    <true/>\n\
</dict>\n\
</plist>\n",
                source_data_dir.join("bitcoin.conf").display(),
                source_data_dir.display()
            ),
        )
        .expect("launchd service");
        path
    };

    #[cfg(target_os = "linux")]
    let source_service_path = {
        let path = sandbox.child(".config/systemd/user/bitcoind.service");
        fs::create_dir_all(path.parent().expect("systemd parent")).expect("systemd");
        fs::write(
            &path,
            format!(
                "[Service]\nExecStart=/usr/bin/bitcoind -conf={} -datadir={}\n",
                source_data_dir.join("bitcoin.conf").display(),
                source_data_dir.display()
            ),
        )
        .expect("systemd service");
        path
    };

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let source_service_path = {
        let path = sandbox.child("services/bitcoind.service");
        fs::create_dir_all(path.parent().expect("service parent")).expect("service dir");
        fs::write(
            &path,
            format!(
                "[Service]\nExecStart=/usr/bin/bitcoind -conf={} -datadir={}\n",
                source_data_dir.join("bitcoin.conf").display(),
                source_data_dir.display()
            ),
        )
        .expect("service file");
        path
    };

    let before_config = fs::read(source_data_dir.join("bitcoin.conf")).expect("before config");
    let before_cookie = fs::read(source_data_dir.join(".cookie")).expect("before cookie");
    let before_wallet = fs::read(source_wallet_dir.join("wallet.dat")).expect("before wallet");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--no-color",
            "--datadir",
            target_data_dir.to_str().expect("target datadir"),
            "migrate",
            "plan",
            "--source-datadir",
            source_data_dir.to_str().expect("source datadir"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Migration plan (dry run only)"));
    assert!(stdout.contains("Benefits:"));
    assert!(stdout.contains("Backup requirements:"));
    assert!(
        stdout.contains(
            source_data_dir
                .join("bitcoin.conf")
                .to_str()
                .expect("config path")
        )
    );
    assert!(stdout.contains(source_service_path.to_str().expect("service path")));
    assert!(
        stdout.contains(
            source_wallet_dir
                .join("wallet.dat")
                .to_str()
                .expect("wallet path")
        )
    );
    assert!(stdout.contains("mig-dry-run-only-switch-over"));
    assert!(!stdout.contains("__cookie__:secret"));
    assert!(!stdout.contains("legacy wallet bytes"));
    assert_eq!(
        fs::read(source_data_dir.join("bitcoin.conf")).expect("after config"),
        before_config
    );
    assert_eq!(
        fs::read(source_data_dir.join(".cookie")).expect("after cookie"),
        before_cookie
    );
    assert_eq!(
        fs::read(source_wallet_dir.join("wallet.dat")).expect("after wallet"),
        before_wallet
    );
}

#[test]
fn open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots() {
    // Arrange
    let sandbox = TestSandbox::new("migrate-custom-source");
    let target_data_dir = sandbox.child("open-data");
    let source_data_dir = sandbox.child("custom-source/bitcoin-core-datadir");
    let source_wallet_dir = source_data_dir.join("wallets/main");
    fs::create_dir_all(&source_wallet_dir).expect("source wallet dir");
    fs::write(source_data_dir.join("bitcoin.conf"), "regtest=1\n").expect("source config");
    fs::write(source_data_dir.join(".cookie"), "__cookie__:secret\n").expect("source cookie");
    fs::write(source_wallet_dir.join("wallet.dat"), "legacy wallet bytes").expect("source wallet");

    #[cfg(target_os = "macos")]
    let source_service_path = {
        let path = sandbox.child("Library/LaunchAgents/org.bitcoin.bitcoind.plist");
        fs::create_dir_all(path.parent().expect("launchagents parent")).expect("launchagents");
        fs::write(&path, "<plist></plist>\n").expect("launchd service");
        path
    };

    #[cfg(target_os = "linux")]
    let source_service_path = {
        let path = sandbox.child(".config/systemd/user/bitcoind.service");
        fs::create_dir_all(path.parent().expect("systemd parent")).expect("systemd");
        fs::write(&path, "[Service]\nExecStart=bitcoind\n").expect("systemd service");
        path
    };

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let source_service_path = {
        let path = sandbox.child("services/bitcoind.service");
        fs::create_dir_all(path.parent().expect("service parent")).expect("service dir");
        fs::write(&path, "service unsupported\n").expect("service file");
        path
    };

    let before_config = fs::read(source_data_dir.join("bitcoin.conf")).expect("before config");
    let before_cookie = fs::read(source_data_dir.join(".cookie")).expect("before cookie");
    let before_wallet = fs::read(source_wallet_dir.join("wallet.dat")).expect("before wallet");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--no-color",
            "--datadir",
            target_data_dir.to_str().expect("target datadir"),
            "migrate",
            "plan",
            "--source-datadir",
            source_data_dir.to_str().expect("source datadir"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Migration plan (dry run only)"));
    assert!(stdout.contains(source_data_dir.to_str().expect("source datadir")));
    assert!(
        stdout.contains(
            source_data_dir
                .join("bitcoin.conf")
                .to_str()
                .expect("config path")
        )
    );
    assert!(!stdout.contains(source_service_path.to_str().expect("service path")));
    assert!(stdout.contains("could not be confidently tied to the selected source install"));
    assert!(
        stdout.contains(
            source_wallet_dir
                .join("wallet.dat")
                .to_str()
                .expect("wallet path")
        )
    );
    assert!(!stdout.contains("__cookie__:secret"));
    assert!(!stdout.contains("legacy wallet bytes"));
    assert_eq!(
        fs::read(source_data_dir.join("bitcoin.conf")).expect("after config"),
        before_config
    );
    assert_eq!(
        fs::read(source_data_dir.join(".cookie")).expect("after cookie"),
        before_cookie
    );
    assert_eq!(
        fs::read(source_wallet_dir.join("wallet.dat")).expect("after wallet"),
        before_wallet
    );
}

#[test]
fn open_bitcoin_config_paths_reports_sources() {
    // Arrange
    let sandbox = TestSandbox::new("config-paths");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "config",
            "paths",
            "--format",
            "human",
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Config:"));
    assert!(stdout.contains("Bitcoin config:"));
    assert!(stdout.contains("Datadir:"));
    assert!(stdout.contains("Logs:"));
    assert!(stdout.contains("Metrics:"));
    assert!(stdout.contains("cli_flags > environment > open_bitcoin_jsonc"));
}

#[test]
fn open_bitcoin_support_bundle_writes_redacted_json_and_markdown() {
    // Arrange
    let sandbox = TestSandbox::new("support-bundle");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let server = FakeRpcServer::start();
    fs::write(
        data_dir.join("bitcoin.conf"),
        format!(
            "regtest=1\nrpcconnect=127.0.0.1\nrpcport={}\nrpcuser=alice\nrpcpassword=super-secret-password\n",
            server.address.port()
        ),
    )
    .expect("bitcoin.conf");
    fs::write(data_dir.join(".cookie"), "__cookie__:super-secret-cookie\n").expect("cookie");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Support evidence written"));
    assert!(stdout.contains("support-evidence.json"));
    assert!(stdout.contains("support-evidence.md"));
    let json_path = output_dir.join("support-evidence.json");
    let markdown_path = output_dir.join("support-evidence.md");
    let json_text = fs::read_to_string(&json_path).expect("support json");
    let markdown = fs::read_to_string(&markdown_path).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["status"]["node"]["state"], "running");
    assert!(decoded.get("config").is_some());
    assert!(decoded.get("store_health").is_some());
    assert!(decoded.get("redaction").is_some());
    assert_eq!(decoded["live_smoke"]["state"], "unavailable");
    assert!(markdown.contains("# Open Bitcoin Support Evidence"));
    for rendered in [&stdout, &json_text, &markdown] {
        assert_absent(rendered, "super-secret-password");
        assert_absent(rendered, "super-secret-cookie");
    }
}

#[test]
fn open_bitcoin_support_bundle_includes_phase75_soak_summary() {
    // Arrange
    let sandbox = TestSandbox::new("support-phase75-soak");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    fs::create_dir_all(&data_dir).expect("open datadir");
    fs::write(
        data_dir.join("bitcoin.conf"),
        "regtest=1\nrpcuser=alice\nrpcpassword=phase75-rpc-secret\n",
    )
    .expect("bitcoin.conf");
    fs::write(
        data_dir.join(".cookie"),
        "__cookie__:phase75-cookie-secret\n",
    )
    .expect("cookie");
    seed_phase72_runtime_metadata(&data_dir, false);
    let start = run_open_bitcoin_vec(&sandbox, soak_start_args(&data_dir, "soak-1700000000-0001"));
    assert_success(&start);
    let run_dir = data_dir.join("soak/runs/soak-1700000000-0001");
    let events_path = run_dir.join("events.jsonl");
    let report_json_path = run_dir.join("report.json");
    let report_markdown_path = run_dir.join("report.md");
    fs::write(
        &report_json_path,
        r#"{"raw reports phase75-secret":"wallet material phase75-secret"}"#,
    )
    .expect("overwrite report json with raw report sentinel");
    fs::write(
        &report_markdown_path,
        "raw daemon logs phase75-secret\nRPC credentials phase75-secret\nunbounded peer tables phase75-secret\n",
    )
    .expect("overwrite report markdown with raw report sentinel");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
        ],
    );

    // Assert
    assert_success(&output);
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["soak_evidence"]["state"], "available");
    assert_eq!(
        decoded["soak_evidence"]["maybe_run_id"],
        json!("soak-1700000000-0001")
    );
    assert_eq!(
        decoded["soak_evidence"]["maybe_final_outcome"],
        json!("clean_completion")
    );
    let latest_sequence = decoded["soak_evidence"]["maybe_latest_sequence"]
        .as_u64()
        .expect("latest sequence");
    assert!(latest_sequence > 0);
    assert_eq!(
        decoded["soak_evidence"]["maybe_source_ledger_path"],
        json!(events_path.display().to_string())
    );
    assert_eq!(
        decoded["soak_evidence"]["maybe_json_report_path"],
        json!(report_json_path.display().to_string())
    );
    assert_eq!(
        decoded["soak_evidence"]["maybe_markdown_report_path"],
        json!(report_markdown_path.display().to_string())
    );
    for expected in [
        "## Soak Evidence",
        "State: available",
        "Run: soak-1700000000-0001",
        "Final outcome: clean_completion",
        "Source ledger:",
        "JSON report:",
        "Markdown report:",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
    assert!(markdown.contains(&format!("Latest sequence: {latest_sequence}")));
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "raw ledger",
            "raw daemon logs",
            "raw reports",
            "wallet material",
            "RPC credentials",
            "unbounded peer tables",
            "phase75-rpc-secret",
            "phase75-cookie-secret",
            "phase75-secret",
            "\"kind\":\"started\"",
            "\"kind\":\"checkpoint\"",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn open_bitcoin_support_bundle_includes_phase72_full_sync_evidence_and_typed_verdict() {
    // Arrange
    let sandbox = TestSandbox::new("support-phase72-evidence");
    let data_dir = sandbox.child("open-data");
    let unavailable_data_dir = sandbox.child("open-data-unavailable");
    let output_dir = sandbox.child("support");
    let unavailable_output_dir = sandbox.child("support-unavailable");
    let report_path = sandbox.child("phase72-live-smoke.json");
    let server = FakeRpcServer::start();
    seed_phase72_runtime_metadata(&data_dir, false);
    seed_phase72_runtime_metadata(&unavailable_data_dir, true);
    write_rpc_conf(&data_dir, server.address.port());
    write_rpc_conf(&unavailable_data_dir, server.address.port());
    fs::write(
        &report_path,
        json!({
            "schema_version": 2,
            "result": {
                "status": "progress",
                "rawPeerTable": "rawPeerTable phase72-live-smoke-secret",
                "walletMaterial": "seed phrase phase72-live-smoke-secret"
            },
            "final_status": {
                "validatedActiveChainHeight": 840_004,
                "maybeValidatedActiveChainHash": "1111111111111111111111111111111111111111111111111111111111111111",
                "maybeValidatedActiveChainWork": "840005",
                "bestKnownTip": {
                    "height": 840_004,
                    "blockHash": "1111111111111111111111111111111111111111111111111111111111111111",
                    "work": "840005",
                    "freshness": "fresh",
                    "rawPeerTable": "raw phase72-live-smoke-secret"
                },
                "stayCurrent": "current_at_best_known_tip",
                "stayCurrentNextAction": "Continue monitoring best-known tip freshness.",
                "noProgressDiagnosis": "current_at_best_known_tip",
                "noProgressNextAction": "No operator action required.",
                "latestReorg": {
                    "finalActiveHeight": 840_004,
                    "finalActiveHash": "1111111111111111111111111111111111111111111111111111111111111111",
                    "fullyPersisted": true,
                    "rawLogTail": "raw phase72-live-smoke-secret"
                },
                "reconcileProgress": {
                    "state": "extended_active_chain",
                    "connectedCount": 4,
                    "finalActiveHeight": 840_004,
                    "finalActiveHash": "1111111111111111111111111111111111111111111111111111111111111111"
                },
                "resourcePressure": {
                    "blocksInFlight": 1,
                    "targetOutboundPeers": 4
                },
                "peerContribution": {
                    "connected": 3,
                    "failed": 1,
                    "rawPeerTable": "peer phase72-live-smoke-secret"
                },
                "rpcpassword": "super-secret-password",
                "__cookie__": "super-secret-cookie"
            },
            "daemon": {
                "stdoutTail": "stdoutTail phase72-live-smoke-secret",
                "stderrTail": "stderrTail phase72-live-smoke-secret"
            }
        })
        .to_string(),
    )
    .expect("live smoke report");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
            "--include-live-smoke-report",
            report_path.to_str().expect("report path"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(
        decoded["full_sync_evidence"]["verdict"]["label"],
        json!("sync_to_tip_proven")
    );
    assert_eq!(
        decoded["full_sync_evidence"]["connected_active_chain"]["height"],
        json!(840_004)
    );
    assert_eq!(
        decoded["full_sync_evidence"]["connected_active_chain"]["hash"],
        json!("1111111111111111111111111111111111111111111111111111111111111111")
    );
    assert_eq!(
        decoded["full_sync_evidence"]["connected_active_chain"]["work"],
        json!("840005")
    );
    assert_eq!(
        decoded["full_sync_evidence"]["validated_active_chain"]["height"],
        json!(840_004)
    );
    assert_eq!(
        decoded["full_sync_evidence"]["validated_active_chain"]["hash"],
        json!("1111111111111111111111111111111111111111111111111111111111111111")
    );
    assert_eq!(
        decoded["full_sync_evidence"]["validated_active_chain"]["work"],
        json!("840005")
    );
    for expected in [
        "## Full Sync Evidence",
        "Evidence verdict: sync_to_tip_proven",
        "validated_active_chain_matches_best_known_tip",
        "Connected active chain: height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005",
        "Validated active chain: height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005",
        "Stay-current window:",
        "Peer contribution:",
        "No-progress or reorg events:",
        "Resource pressure:",
        "Recovery:",
    ] {
        assert!(markdown.contains(expected), "missing {expected}");
    }
    for rendered in [&stdout, &json_text, &markdown] {
        for forbidden in [
            "super-secret-password",
            "super-secret-cookie",
            "stdoutTail",
            "stderrTail",
            "rawPeerTable",
            "rawLogTail",
            "seed phrase",
            "walletMaterial",
            "phase72-live-smoke-secret",
        ] {
            assert_absent(rendered, forbidden);
        }
    }

    // Act
    let unavailable_output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            unavailable_data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            unavailable_output_dir.to_str().expect("output dir"),
        ],
    );

    // Assert
    assert_success(&unavailable_output);
    let unavailable_json_text =
        fs::read_to_string(unavailable_output_dir.join("support-evidence.json"))
            .expect("support json");
    let unavailable_markdown =
        fs::read_to_string(unavailable_output_dir.join("support-evidence.md"))
            .expect("support markdown");
    let unavailable: Value = serde_json::from_str(&unavailable_json_text).expect("support json");
    assert_eq!(
        unavailable["full_sync_evidence"]["connected_active_chain"]["maybe_unavailable_reason"],
        json!("connected active-chain hash unavailable")
    );
    assert_eq!(
        unavailable["full_sync_evidence"]["validated_active_chain"]["maybe_unavailable_reason"],
        json!("validated active-chain hash unavailable")
    );
    assert!(unavailable_markdown.contains(
        "Connected active chain: height=840004 hash=Unavailable work=Unavailable; Unavailable: connected active-chain hash unavailable"
    ));
    assert!(unavailable_markdown.contains(
        "Validated active chain: height=840004 hash=Unavailable work=Unavailable; Unavailable: validated active-chain hash unavailable"
    ));
}

#[test]
fn open_bitcoin_support_bundle_summarizes_phase61_resource_recovery_evidence() {
    // Arrange
    let sandbox = TestSandbox::new("support-live-smoke-v2");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    let report_path = sandbox.child("live-smoke.json");
    let cookie_like_text = "__cookie__:live-smoke-secret";
    let wallet_like_text = "seed phrase live-smoke-secret";
    fs::create_dir_all(&data_dir).expect("open datadir");
    fs::write(
        &report_path,
        json!({
            "schema_version": 2,
            "result": {
                "status": "no_progress",
                "progressDetected": false,
                "maybeNoProgressCause": "handshake_failure",
                "nextAction": "Retry after rotating manual peers; rpcpassword=fixture-secret",
                "headerDelta": 0,
                "blockDelta": 0,
                "firstHeaderProgress": {
                    "observedAtUnixSeconds": 1_780_600_100,
                    "before": {
                        "headerHeight": 840_000,
                        "downloadedBlockHeight": 830_000,
                        "secret": "raw before header live-smoke-secret"
                    },
                    "after": {
                        "headerHeight": 840_100,
                        "connectedBlockHeight": 830_001,
                        "secret": "raw after header live-smoke-secret"
                    },
                    "maybePeer": "peer-accepted-headers",
                    "maybeSource": "manual_peer",
                    "maybeResolvedEndpoint": "redacted-endpoint",
                    "rawEndpoint": "192.0.2.10:8333"
                },
                "firstBlockProgress": {
                    "kind": "connected",
                    "height": 840_004,
                    "blockHash": "0000000000000000000b10c400000000000000000000000000000000000000",
                    "observedAtUnixSeconds": 1_780_600_120,
                    "before": {
                        "downloadedBlockHeight": 840_005,
                        "connectedBlockHeight": 840_003,
                        "headerHeight": 840_100
                    },
                    "after": {
                        "downloadedBlockHeight": 840_006,
                        "connectedBlockHeight": 840_004,
                        "headerHeight": 840_100
                    },
                    "maybePeer": "peer-accepted-block",
                    "maybeSource": "manual_peer",
                    "maybeResolvedEndpoint": "redacted-endpoint",
                    "rawEndpoint": "192.0.2.10:8333"
                },
                "restartResumeEvidence": {
                    "restartStatus": "completed",
                    "sameDatadir": {
                        "requestedPathMatched": true,
                        "resolvedPathMatched": true,
                        "rawPath": "/tmp/live-smoke-secret"
                    },
                    "duplicateConnectVerdict": "no_duplicate_connect_observed",
                    "maybePostRestartProgressDelta": {
                        "headerDelta": 0,
                        "downloadedBlockDelta": 1,
                        "connectedBlockDelta": 1,
                        "rawDeltaNote": "live-smoke-secret"
                    },
                    "peerOutcomeSummary": {
                        "connected": 1,
                        "failed": 1,
                        "failureCauses": ["handshake_failure"],
                        "handshook": 1,
                        "skipped": 0,
                        "rawEndpoint": "192.0.2.10:8333"
                    },
                    "recoveryDiagnosis": {
                        "category": "storage_lock_contention",
                        "maybeLastError": null,
                        "maybeNoProgressCause": "handshake_failure",
                        "maybePeerFailureReason": "peer did not provide useful blocks",
                        "maybeStorageRecoveryAction": null,
                        "rawCookie": cookie_like_text
                    },
                    "beforeRestart": {
                        "headerHeight": 840_100,
                        "downloadedBlockHeight": 840_005,
                        "connectedBlockHeight": 840_003,
                        "phase": "blocks",
                        "lifecycle": "active",
                        "maybeLastError": null,
                        "maybeLastSuccessfulProgressUnixSeconds": 1_780_600_100,
                        "maybeDownloadedBlockHash": "0000000000000000000b10c500000000000000000000000000000000000000",
                        "maybeConnectedBlockHash": "0000000000000000000b10c300000000000000000000000000000000000000",
                        "snapshots": ["live-smoke-secret"]
                    },
                    "afterRestart": {
                        "headerHeight": 840_100,
                        "downloadedBlockHeight": 840_006,
                        "connectedBlockHeight": 840_004,
                        "phase": "blocks",
                        "lifecycle": "active",
                        "maybeLastError": null,
                        "maybeLastSuccessfulProgressUnixSeconds": 1_780_600_120,
                        "maybeDownloadedBlockHash": "0000000000000000000b10c600000000000000000000000000000000000000",
                        "maybeConnectedBlockHash": "0000000000000000000b10c400000000000000000000000000000000000000",
                        "walletMaterial": wallet_like_text
                    }
                },
                "message": "raw result message should not be copied"
            },
            "final_status": {
                "headerHeight": 840_100,
                "downloadedBlockHeight": 840_006,
                "connectedBlockHeight": 840_004,
                "blockHeight": 840_004,
                "phase": "blocks",
                "lifecycle": "active",
                "outboundPeers": 2,
                "messagesProcessed": 128,
                "recoveryCategory": "invalid_peer_data",
                "resourcePressure": {
                    "blocksInFlight": 4,
                    "maxHeaderRequestsInFlightPerPeer": 2,
                    "maxHeadersPerMessage": 2_000,
                    "maxBlocksInFlightPerPeer": 16,
                    "maxBlocksInFlightTotal": 64,
                    "maxMessagesPerPeer": 4_096,
                    "maxSyncRounds": 16,
                    "outboundPeers": 2,
                    "targetOutboundPeers": 3,
                    "rawEndpoint": "192.0.2.10:8333",
                    "rawCookie": cookie_like_text
                },
                "maybeLastError": null,
                "maybeLastSuccessfulProgressUnixSeconds": 1_780_600_120,
                "recentPeers": [
                    {
                        "peer": "192.0.2.10:8333",
                        "error": "live-smoke-secret"
                    }
                ]
            },
            "daemon": {
                "stderrTail": "raw daemon stderr tail live-smoke-secret",
                "stdoutTail": "raw daemon stdout tail live-smoke-secret"
            },
            "options": {
                "manualPeers": ["192.0.2.10:8333"],
                "rpcpassword": "live-smoke-secret"
            },
            "snapshots": [
                {
                    "phase": "header_sync",
                    "secret": "raw snapshot live-smoke-secret",
                    "cookie": cookie_like_text,
                    "wallet": wallet_like_text
                }
            ],
            "preflight": {
                "checks": [
                    {
                        "name": "fixture",
                        "ok": false,
                        "message": "raw preflight check live-smoke-secret"
                    }
                ]
            },
            "network_preflight": {
                "endpoint_outcomes": [
                    {
                        "address": "192.0.2.10:8333",
                        "maybeError": "endpoint-table-detail live-smoke-secret",
                        "state": "failed"
                    }
                ]
            }
        })
        .to_string(),
    )
    .expect("live smoke report");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
            "--include-live-smoke-report",
            report_path.to_str().expect("report path"),
        ],
    );

    // Assert
    assert_success(&output);
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["live_smoke"]["state"], "available");
    assert_eq!(decoded["live_smoke"]["summary"]["status"], "no_progress");
    assert_eq!(decoded["live_smoke"]["summary"]["progressDetected"], false);
    assert_eq!(
        decoded["live_smoke"]["summary"]["maybeNoProgressCause"],
        "handshake_failure"
    );
    assert_eq!(decoded["live_smoke"]["summary"]["nextAction"], "[redacted]");
    assert_eq!(decoded["live_smoke"]["summary"]["headerDelta"], 0);
    assert_eq!(decoded["live_smoke"]["summary"]["blockDelta"], 0);
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstHeaderProgress"]["observedAtUnixSeconds"],
        1_780_600_100
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstHeaderProgress"]["before"]["headerHeight"],
        840_000
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstHeaderProgress"]["after"]["headerHeight"],
        840_100
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstBlockProgress"]["kind"],
        "connected"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstBlockProgress"]["height"],
        840_004
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstBlockProgress"]["before"]["downloadedBlockHeight"],
        840_005
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["firstBlockProgress"]["after"]["connectedBlockHeight"],
        840_004
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["restartStatus"],
        "completed"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["sameDatadir"]["requestedPathMatched"],
        true
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["duplicateConnectVerdict"],
        "no_duplicate_connect_observed"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["recoveryDiagnosis"]["category"],
        "storage_lock_contention"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["restartResumeEvidence"]["afterRestart"]["connectedBlockHeight"],
        840_004
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["headerHeight"],
        840_100
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["downloadedBlockHeight"],
        840_006
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["connectedBlockHeight"],
        840_004
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["recoveryCategory"],
        "invalid_peer_data"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["resourcePressure"]["maxBlocksInFlightTotal"],
        64
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["finalStatus"]["resourcePressure"]["targetOutboundPeers"],
        3
    );
    assert_eq!(decoded["store_health"]["state"], "unavailable");
    assert!(decoded["status"]["service"].is_object());
    assert_eq!(
        decoded["status"]["service"]["restart_resume"]["state"],
        "unavailable"
    );
    assert_eq!(decoded["status"]["logs"]["path"]["state"], "unavailable");
    assert_eq!(
        decoded["status"]["metrics"]["availability"]["state"],
        "unavailable"
    );
    assert!(markdown.contains("Progress detected"));
    assert!(markdown.contains("No-progress cause"));
    assert!(markdown.contains("Header delta"));
    assert!(markdown.contains("Block delta"));
    assert!(markdown.contains("First header progress"));
    assert!(markdown.contains("First block progress"));
    assert!(markdown.contains("Restart/resume evidence"));
    assert!(markdown.contains("Recovery diagnosis"));
    assert!(markdown.contains("Recovery category"));
    assert!(markdown.contains("Resource pressure"));
    assert!(markdown.contains("Final status"));
    assert!(markdown.contains("Service lifecycle"));
    assert!(markdown.contains("Service restart/resume"));
    assert!(markdown.contains("Logs path"));
    assert!(markdown.contains("Metrics availability"));
    assert!(markdown.contains("Metrics samples"));
    assert!(markdown.contains("handshake_failure"));
    for rendered in [&json_text, &markdown] {
        assert_absent(rendered, "live-smoke-secret");
        assert_absent(rendered, "fixture-secret");
        assert_absent(rendered, "stdoutTail");
        assert_absent(rendered, "stderrTail");
        assert_absent(rendered, "endpoint_outcomes");
        assert_absent(rendered, "manualPeers");
        assert_absent(rendered, "rpcpassword");
        assert_absent(rendered, "raw daemon stderr tail");
        assert_absent(rendered, "raw daemon stdout tail");
        assert_absent(rendered, "raw snapshot");
        assert_absent(rendered, "raw preflight check");
        assert_absent(rendered, "endpoint-table-detail");
        assert_absent(rendered, "192.0.2.10:8333");
        assert_absent(rendered, cookie_like_text);
        assert_absent(rendered, wallet_like_text);
        assert_absent(rendered, "snapshots");
    }
}

#[test]
fn open_bitcoin_support_bundle_preserves_top_level_live_smoke_fallback() {
    // Arrange
    let sandbox = TestSandbox::new("support-live-smoke-fallback");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    let report_path = sandbox.child("live-smoke.json");
    fs::create_dir_all(&data_dir).expect("open datadir");
    fs::write(
        &report_path,
        json!({
            "status": "timeout",
            "maybeNoProgressCause": "handshake_failure",
            "nextAction": "Retry with --manual-peer=HOST[:PORT]",
            "manualPeers": ["192.0.2.10:8333"],
            "manual_peers": ["198.51.100.20:8333"],
            "daemonStderrTail": "live-smoke-secret"
        })
        .to_string(),
    )
    .expect("live smoke report");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
            "--include-live-smoke-report",
            report_path.to_str().expect("report path"),
        ],
    );

    // Assert
    assert_success(&output);
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["live_smoke"]["state"], "available");
    assert_eq!(decoded["live_smoke"]["summary"]["status"], "timeout");
    assert_eq!(
        decoded["live_smoke"]["summary"]["maybeNoProgressCause"],
        "handshake_failure"
    );
    assert_eq!(
        decoded["live_smoke"]["summary"]["nextAction"],
        "Retry with --manual-peer=HOST[:PORT]"
    );
    assert!(markdown.contains("handshake_failure"));
    assert_absent(&json_text, "live-smoke-secret");
    assert_absent(&markdown, "live-smoke-secret");
    for rendered in [&json_text, &markdown] {
        assert_absent(rendered, "manualPeers");
        assert_absent(rendered, "manual_peers");
        assert_absent(rendered, "192.0.2.10:8333");
        assert_absent(rendered, "198.51.100.20:8333");
    }
}

#[test]
fn open_bitcoin_support_bundle_keeps_missing_live_smoke_report_unavailable() {
    // Arrange
    let sandbox = TestSandbox::new("support-live-smoke-missing");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    let report_path = sandbox.child("missing-live-smoke.json");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
            "--include-live-smoke-report",
            report_path.to_str().expect("report path"),
        ],
    );

    // Assert
    assert_success(&output);
    let json_text =
        fs::read_to_string(output_dir.join("support-evidence.json")).expect("support json");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["live_smoke"]["state"], "unavailable");
    assert!(
        decoded["live_smoke"]["reason"]
            .as_str()
            .expect("live smoke reason")
            .contains("does not exist")
    );
}

#[test]
fn open_bitcoin_compatibility_harness_writes_json_and_markdown_reports() {
    // Arrange
    let sandbox = TestSandbox::new("compatibility-report");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("compatibility");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--format",
            "json",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "compatibility",
            "harness",
            "--peer-endpoint",
            "203.0.113.10:8333",
            "--scenario",
            "service-bit-mismatch",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout: Value = serde_json::from_slice(&output.stdout).expect("compatibility stdout json");
    assert_eq!(
        stdout["json_path"],
        json!(
            output_dir
                .join("compatibility-harness-report.json")
                .display()
                .to_string()
        )
    );
    assert_eq!(
        stdout["markdown_path"],
        json!(
            output_dir
                .join("compatibility-harness-report.md")
                .display()
                .to_string()
        )
    );
    let json_text = fs::read_to_string(output_dir.join("compatibility-harness-report.json"))
        .expect("compatibility json");
    let markdown = fs::read_to_string(output_dir.join("compatibility-harness-report.md"))
        .expect("compatibility markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("compatibility report json");
    assert_eq!(decoded["peer_endpoint"], "203.0.113.10:8333");
    assert_eq!(decoded["network"], "regtest");
    assert_eq!(decoded["scenario"], "service_bit_mismatch");
    assert_eq!(decoded["diagnosis"], "service_bit_mismatch");
    assert_eq!(decoded["failing_step"]["diagnosis"], "service_bit_mismatch");
    assert_eq!(decoded["transcript_summary"]["useful_progress"], false);
    assert_eq!(
        decoded["negotiated_capabilities"]["required_services"],
        json!(["NODE_NETWORK", "NODE_WITNESS"])
    );
    assert!(decoded["redaction_boundaries"]["omitted"].is_array());
    assert!(markdown.contains("# Open Bitcoin Compatibility Harness Report"));
    assert!(markdown.contains("Peer endpoint: 203.0.113.10:8333"));
    assert!(markdown.contains("Network: regtest"));
    assert!(markdown.contains("Diagnosis: service_bit_mismatch"));
    assert!(markdown.contains("Failing step"));
    assert!(markdown.contains("Negotiated capabilities"));
    assert!(markdown.contains("Redaction boundaries"));
    assert!(markdown.contains("raw wire payloads"));
    for rendered in [&json_text, &markdown] {
        assert_absent(rendered, "super-secret-password");
        assert_absent(rendered, "__cookie__:compat-secret");
        assert_absent(rendered, "seed phrase compat-secret");
        assert_absent(rendered, "raw-payload-hex-secret");
        assert_absent(rendered, "daemon stdout tail secret");
        assert_absent(rendered, "daemon stderr tail secret");
    }
}

#[test]
fn open_bitcoin_compatibility_harness_covers_required_diagnosis_scenarios() {
    // Arrange
    let sandbox = TestSandbox::new("compatibility-scenarios");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let cases = [
        ("compatible", "compatible"),
        ("version-rejected", "version_rejected"),
        ("network-mismatch", "network_mismatch"),
        ("service-bit-mismatch", "service_bit_mismatch"),
        ("unsupported-message-order", "unsupported_message_order"),
        ("timeout", "timeout"),
        ("peer-disconnect", "peer_disconnect"),
        ("malformed-payload", "malformed_payload"),
        ("local-configuration-failure", "local_configuration_failure"),
    ];

    for (scenario, expected_diagnosis) in cases {
        let output_dir = sandbox.child(&format!("compatibility-{scenario}"));

        // Act
        let output = run_open_bitcoin(
            &sandbox,
            [
                "--network",
                "regtest",
                "--datadir",
                data_dir.to_str().expect("datadir"),
                "compatibility",
                "harness",
                "--peer-endpoint",
                "198.51.100.20:8333",
                "--scenario",
                scenario,
                "--output-dir",
                output_dir.to_str().expect("output dir"),
            ],
        );

        // Assert
        assert_success(&output);
        let report_path = output_dir.join("compatibility-harness-report.json");
        let json_text = fs::read_to_string(report_path).expect("compatibility report");
        let decoded: Value = serde_json::from_str(&json_text).expect("compatibility json");
        assert_eq!(decoded["scenario"], expected_diagnosis);
        assert_eq!(decoded["diagnosis"], expected_diagnosis);
    }
}

#[test]
fn open_bitcoin_wallet_send_requires_confirm_and_uses_preview_path() {
    // Arrange
    let sandbox = TestSandbox::new("wallet-send-preview");
    let data_dir = sandbox.child("open-data");
    let server = FakeRpcServer::start();
    seed_managed_wallet(&data_dir, "alpha");
    write_rpc_conf(&data_dir, server.address.port());

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "wallet",
            "--wallet",
            "alpha",
            "send",
            "mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn",
            "12000",
            "--fee-rate-sat-per-kvb",
            "2000",
            "--change-descriptor-id",
            "1",
            "--replaceable",
        ],
    );

    // Assert
    assert_failure(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stdout.contains("Transaction hex:"));
    assert!(stderr.contains("confirmation required"));
    let requests = server.requests();
    assert!(
        requests
            .iter()
            .any(|request| request.contains("POST /wallet/alpha HTTP/1.1"))
    );
    assert!(
        requests
            .iter()
            .any(|request| request.contains("\"buildandsigntransaction\""))
    );
    assert!(
        !requests
            .iter()
            .any(|request| request.contains("\"sendtoaddress\""))
    );
}

#[test]
fn open_bitcoin_wallet_send_confirm_submits_sendtoaddress() {
    // Arrange
    let sandbox = TestSandbox::new("wallet-send-confirm");
    let data_dir = sandbox.child("open-data");
    let server = FakeRpcServer::start();
    seed_managed_wallet(&data_dir, "alpha");
    write_rpc_conf(&data_dir, server.address.port());

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--format",
            "json",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "wallet",
            "--wallet",
            "alpha",
            "send",
            "mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn",
            "12000",
            "--fee-rate-sat-per-kvb",
            "2000",
            "--change-descriptor-id",
            "1",
            "--replaceable",
            "--confirm",
        ],
    );

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("submit json");
    assert_eq!(decoded["wallet"], "alpha");
    assert_eq!(decoded["txid"], json!("bb".repeat(32)));
    let requests = server.requests();
    assert!(
        requests
            .iter()
            .any(|request| request.contains("\"buildandsigntransaction\""))
    );
    assert!(
        requests
            .iter()
            .any(|request| request.contains("\"sendtoaddress\""))
    );
    assert!(
        requests
            .iter()
            .all(|request| request.contains("/wallet/alpha"))
    );
}

#[test]
fn open_bitcoin_wallet_backup_writes_open_bitcoin_export() {
    // Arrange
    let sandbox = TestSandbox::new("wallet-backup-write");
    let data_dir = sandbox.child("open-data");
    let backup_dir = sandbox.child("backups");
    let backup_path = backup_dir.join("alpha-backup.json");
    fs::create_dir_all(&backup_dir).expect("backup dir");
    seed_managed_wallet(&data_dir, "alpha");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "wallet",
            "--wallet",
            "alpha",
            "backup",
            backup_path.to_str().expect("backup path"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Wrote Open Bitcoin wallet backup for alpha"));
    let contents = fs::read_to_string(&backup_path).expect("backup contents");
    let decoded: Value = serde_json::from_str(&contents).expect("backup json");
    assert_eq!(decoded["format"], "open-bitcoin-wallet-backup");
    assert_eq!(decoded["wallet_name"], "alpha");
    assert_eq!(decoded["snapshot"]["network"], "regtest");
    assert_eq!(decoded["snapshot"]["descriptor_count"], 2);
}

#[test]
fn open_bitcoin_wallet_backup_rejects_external_wallet_candidate_paths() {
    // Arrange
    let sandbox = TestSandbox::new("wallet-backup-unsafe");
    let data_dir = sandbox.child("open-data");
    let unsafe_wallet_dir = sandbox.child(".bitcoin/wallets/external");
    let unsafe_backup_path = unsafe_wallet_dir.join("backup.json");
    fs::create_dir_all(&unsafe_wallet_dir).expect("unsafe wallet dir");
    seed_managed_wallet(&data_dir, "alpha");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "wallet",
            "--wallet",
            "alpha",
            "backup",
            unsafe_backup_path.to_str().expect("backup path"),
        ],
    );

    // Assert
    assert_failure(&output);
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("backup destination overlaps detected external wallet candidate"));
}

fn onboard_args<'a>(
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

fn soak_start_args<'a>(data_dir: &'a Path, run_id: &'a str) -> Vec<&'a str> {
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

fn soak_target_height_start_args<'a>(data_dir: &'a Path, run_id: &'a str) -> Vec<&'a str> {
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

fn run_open_bitcoin<const N: usize>(sandbox: &TestSandbox, args: [&str; N]) -> Output {
    run_open_bitcoin_vec(sandbox, args.to_vec())
}

fn run_open_bitcoin_vec(sandbox: &TestSandbox, args: Vec<&str>) -> Output {
    Command::new(env!("CARGO_BIN_EXE_open-bitcoin"))
        .args(args)
        .env("HOME", &sandbox.home)
        .output()
        .expect("run open-bitcoin")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_failure(output: &Output) {
    assert!(
        !output.status.success(),
        "status={:?}\nstdout={}\nstderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_no_store_lock_error(output: &Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stdout.contains("FjallError"), "stdout={stdout}");
    assert!(!stdout.contains("Locked"), "stdout={stdout}");
    assert!(!stderr.contains("FjallError"), "stderr={stderr}");
    assert!(!stderr.contains("Locked"), "stderr={stderr}");
}

fn assert_absent(text: &str, value: &str) {
    assert!(
        !text.contains(value),
        "unexpected sensitive value in {text}"
    );
}

fn read_jsonl_values(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .expect("jsonl file")
        .lines()
        .map(|line| serde_json::from_str(line).expect("jsonl value"))
        .collect()
}

fn seed_managed_wallet(data_dir: &Path, wallet_name: &str) {
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

fn write_rpc_conf(data_dir: &Path, rpc_port: u16) {
    fs::write(
        data_dir.join("bitcoin.conf"),
        format!(
            "regtest=1\nrpcconnect=127.0.0.1\nrpcport={rpc_port}\nrpcuser=alice\nrpcpassword=secret\n"
        ),
    )
    .expect("bitcoin.conf");
}

fn seed_phase72_runtime_metadata(data_dir: &Path, missing_active_chain: bool) {
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

fn phase72_durable_sync_state(missing_active_chain: bool) -> DurableSyncState {
    DurableSyncState {
        sync: phase72_sync_status(missing_active_chain),
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 3,
            }),
            recent_peers: FieldAvailability::unavailable("peer telemetry unavailable"),
        },
        health_signals: Vec::new(),
        updated_at_unix_seconds: 1_717_000_020,
    }
}

fn phase72_sync_status(missing_active_chain: bool) -> SyncStatus {
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

struct FakeRpcServer {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<String>>>,
    stop: std::sync::mpsc::Sender<()>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl FakeRpcServer {
    fn start() -> Self {
        Self::start_with_behavior(FakeRpcBehavior::Normal)
    }

    fn start_unauthorized() -> Self {
        Self::start_with_behavior(FakeRpcBehavior::Unauthorized)
    }

    fn start_with_behavior(behavior: FakeRpcBehavior) -> Self {
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

    fn requests(&self) -> Vec<String> {
        self.requests.lock().expect("request log").clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FakeRpcBehavior {
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

fn handle_rpc_connection(
    mut stream: TcpStream,
    requests: &Arc<Mutex<Vec<String>>>,
    behavior: FakeRpcBehavior,
) {
    stream.set_nonblocking(false).expect("blocking stream");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("read timeout");
    let request = read_http_request(&mut stream);
    let request_text = String::from_utf8_lossy(&request).into_owned();
    let request_methods = json_rpc_methods_from_request(&request);
    requests
        .lock()
        .expect("request log")
        .push(request_text.clone());
    if behavior == FakeRpcBehavior::Unauthorized {
        let response = "HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\nWWW-Authenticate: Basic\r\nConnection: close\r\n\r\n";
        write_http_response(&mut stream, response);
        return;
    }
    let result = if has_rpc_method(&request_methods, "openbitcoinsyncstatus") {
        json!({
            "metadata": fake_runtime_metadata(false)
        })
    } else if has_rpc_method(&request_methods, "openbitcoinsyncpause") {
        json!({
            "metadata": fake_runtime_metadata(true)
        })
    } else if has_rpc_method(&request_methods, "openbitcoinsyncresume") {
        json!({
            "metadata": fake_runtime_metadata(false)
        })
    } else if has_rpc_method(&request_methods, "getnetworkinfo") {
        json!({
            "version": 29300,
            "subversion": "/Satoshi:29.3.0/",
            "protocolversion": 70016,
            "localservices": "0000000000000409",
            "localrelay": true,
            "connections": 7,
            "connections_in": 2,
            "connections_out": 5,
            "relayfee": 1000,
            "incrementalfee": 1000,
            "warnings": []
        })
    } else if has_rpc_method(&request_methods, "getblockchaininfo") {
        json!({
            "chain": "regtest",
            "blocks": 144,
            "headers": 150,
            "bestblockhash": "00aabb",
            "verificationprogress": 0.96,
            "initialblockdownload": false,
            "warnings": []
        })
    } else if has_rpc_method(&request_methods, "getmempoolinfo") {
        json!({
            "size": 12,
            "bytes": 2048,
            "usage": 4096,
            "total_fee_sats": 320,
            "maxmempool": 300000000,
            "mempoolminfee": 1000,
            "minrelaytxfee": 1000,
            "loaded": true
        })
    } else if has_rpc_method(&request_methods, "buildandsigntransaction") {
        json!({
            "transaction_hex": "001122",
            "fee_sats": 220,
            "inputs": [{
                "txid_hex": "aa".repeat(32),
                "vout": 0,
                "descriptor_id": 1,
                "amount_sats": 75000
            }],
            "maybe_change_output_index": 1
        })
    } else if has_rpc_method(&request_methods, "sendtoaddress") {
        json!("bb".repeat(32))
    } else if has_rpc_method(&request_methods, "getwalletinfo") {
        json!({
            "network": "regtest",
            "descriptor_count": 2,
            "utxo_count": 1,
            "maybe_tip_height": 144,
            "walletname": "alpha",
            "freshness": "fresh",
            "scanning": false
        })
    } else {
        json!({
            "mine": {
                "trusted_sats": 50000,
                "untrusted_pending_sats": 0,
                "immature_sats": 0
            }
        })
    };
    let response_body = json!({
        "jsonrpc": "2.0",
        "result": result,
        "id": 1
    })
    .to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    write_http_response(&mut stream, &response);
}

fn write_http_response(stream: &mut TcpStream, response: &str) {
    stream
        .write_all(response.as_bytes())
        .expect("write response");
    stream.flush().expect("flush response");
}

fn json_rpc_methods_from_request(request: &[u8]) -> Vec<String> {
    let Some(header_end) = request
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
    else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_slice::<Value>(&request[header_end..]) else {
        return Vec::new();
    };

    match value {
        Value::Object(object) => object
            .get("method")
            .and_then(Value::as_str)
            .map(|method| vec![method.to_owned()])
            .unwrap_or_default(),
        Value::Array(items) => items
            .iter()
            .filter_map(|item| item.get("method").and_then(Value::as_str))
            .map(str::to_owned)
            .collect(),
        _ => Vec::new(),
    }
}

fn has_rpc_method(methods: &[String], expected: &str) -> bool {
    methods.iter().any(|method| method == expected)
}

fn fake_runtime_metadata(paused: bool) -> Value {
    json!({
        "node_version": "0.1.0",
        "storage_engine": "fjall",
        "last_clean_shutdown": false,
        "maybe_last_recovery_action": null,
        "maybe_sync_state": null,
        "sync_control": {
            "paused": paused
        }
    })
}

fn read_http_request(stream: &mut TcpStream) -> Vec<u8> {
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(bytes_read) => buffer.extend_from_slice(&chunk[..bytes_read]),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                if http_request_complete(&buffer) || Instant::now() > deadline {
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) => panic!("read request: {error}"),
        }
        if http_request_complete(&buffer) {
            break;
        }
        if Instant::now() > deadline {
            break;
        }
    }
    buffer
}

fn http_request_complete(buffer: &[u8]) -> bool {
    let Some(header_end) = buffer
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
    else {
        return false;
    };

    let headers = &buffer[..header_end];
    let Some(content_length) = parse_content_length(headers) else {
        return buffer.len() >= header_end;
    };

    buffer.len() >= header_end + content_length
}

fn parse_content_length(headers: &[u8]) -> Option<usize> {
    std::str::from_utf8(headers).ok().and_then(|text| {
        text.lines().find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("content-length") {
                return value.trim().parse::<usize>().ok();
            }
            None
        })
    })
}
