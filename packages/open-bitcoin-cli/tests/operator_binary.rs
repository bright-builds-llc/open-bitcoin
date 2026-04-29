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
    FjallNodeStore, PersistMode, WalletRegistry,
    core::wallet::{AddressNetwork, DescriptorRole, Wallet},
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

struct FakeRpcServer {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<String>>>,
    stop: std::sync::mpsc::Sender<()>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl FakeRpcServer {
    fn start() -> Self {
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
                    Ok((stream, _)) => handle_rpc_connection(stream, &request_log),
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

impl Drop for FakeRpcServer {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.join().expect("server thread");
        }
    }
}

fn handle_rpc_connection(mut stream: TcpStream, requests: &Arc<Mutex<Vec<String>>>) {
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("read timeout");
    let request = read_http_request(&mut stream);
    let request_text = String::from_utf8_lossy(&request).into_owned();
    requests
        .lock()
        .expect("request log")
        .push(request_text.clone());
    let result = if request_text.contains("getnetworkinfo") {
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
    } else if request_text.contains("getblockchaininfo") {
        json!({
            "chain": "regtest",
            "blocks": 144,
            "headers": 150,
            "bestblockhash": "00aabb",
            "verificationprogress": 0.96,
            "initialblockdownload": false,
            "warnings": []
        })
    } else if request_text.contains("getmempoolinfo") {
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
    } else if request_text.contains("buildandsigntransaction") {
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
    } else if request_text.contains("sendtoaddress") {
        json!("bb".repeat(32))
    } else if request_text.contains("getwalletinfo") {
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
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    stream
        .write_all(response.as_bytes())
        .expect("write response");
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
                if !buffer.is_empty() || Instant::now() > deadline {
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) => panic!("read request: {error}"),
        }
        if buffer.windows(4).any(|window| window == b"\r\n\r\n") && buffer.ends_with(b"}") {
            break;
        }
        if Instant::now() > deadline {
            break;
        }
    }
    buffer
}
