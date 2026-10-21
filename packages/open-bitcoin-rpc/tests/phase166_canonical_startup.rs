// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

static PROCESS_SCENARIO: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir()
            .canonicalize()
            .expect("physical temp")
            .join(format!(
                "phase166-daemon-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("clock")
                    .as_nanos()
            ));
        fs::create_dir(&root).expect("owned root");
        fs::create_dir(root.join("source")).expect("explicit empty datadir");
        Self(root)
    }

    fn command(&self, selector: &str, port: u16) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_open-bitcoind"));
        command
            .env_clear()
            .env("HOME", &self.0)
            .current_dir(&self.0)
            .args([
                format!("-datadir={}", self.0.join("source").display()),
                format!("-chain={selector}"),
                "-server=1".to_owned(),
                "-rpcbind=127.0.0.1".to_owned(),
                format!("-rpcport={port}"),
                "-rpcuser=private".to_owned(),
                "-rpcpassword=secret".to_owned(),
                "-blockfilterindex=1".to_owned(),
            ]);
        command
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("owned cleanup");
    }
}

struct Daemon(Child);
impl Drop for Daemon {
    fn drop(&mut self) {
        if self.0.try_wait().expect("child status").is_none() {
            self.0.kill().expect("stop owned daemon");
        }
        self.0.wait().expect("reap owned daemon");
    }
}

fn rpc(port: u16, method: &str, params: serde_json::Value) -> serde_json::Value {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("local RPC");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("bounded read");
    let body =
        serde_json::json!({"jsonrpc":"2.0", "id":1, "method":method, "params":params}).to_string();
    write!(stream, "POST / HTTP/1.1\r\nHost: localhost\r\nAuthorization: Basic cHJpdmF0ZTpzZWNyZXQ=\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).expect("RPC request");
    let mut response = String::new();
    stream.read_to_string(&mut response).expect("RPC response");
    let (_, json) = response.split_once("\r\n\r\n").expect("HTTP response");
    serde_json::from_str(json).expect("JSON response")
}

#[test]
fn phase166_configured_daemon_initializes_and_reopens_all_selectors() {
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    let vectors: serde_json::Value = serde_json::from_str(include_str!(
        "../../../docs/parity/fixtures/phase166-genesis-vectors.json"
    ))
    .expect("independent vectors");
    for selector in ["mainnet", "testnet3", "signet", "regtest"] {
        // Arrange
        let fixture = Fixture::new();
        for _ in 0..2 {
            let reservation = TcpListener::bind("127.0.0.1:0").expect("local port");
            let port = reservation.local_addr().expect("address").port();
            drop(reservation);
            let mut daemon = Daemon(
                fixture
                    .command(selector, port)
                    .stdout(Stdio::null())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("product daemon"),
            );
            let deadline = Instant::now() + Duration::from_secs(45);
            // Act
            loop {
                if TcpStream::connect(("127.0.0.1", port)).is_ok() {
                    break;
                }
                if let Some(status) = daemon.0.try_wait().expect("child status") {
                    let mut message = String::new();
                    daemon
                        .0
                        .stderr
                        .take()
                        .expect("stderr")
                        .read_to_string(&mut message)
                        .expect("message");
                    panic!("configured {selector} daemon exited {status}: {message}");
                }
                assert!(Instant::now() < deadline, "bounded daemon startup");
                std::thread::sleep(Duration::from_millis(20));
            }
            let info = rpc(port, "getblockchaininfo", serde_json::json!([]));
            // Assert
            assert!(info["error"].is_null(), "{info}");
            assert_eq!(info["result"]["blocks"], 0, "{info}");
            assert_eq!(info["result"]["headers"], 0, "{info}");
            let vector = vectors["genesis"]
                .as_array()
                .expect("genesis vectors")
                .iter()
                .find(|row| row["chain"] == selector)
                .expect("selected vector");
            assert_eq!(
                info["result"]["bestblockhash"],
                vector["block_hash_display"]
            );
            let filter = rpc(
                port,
                "getblockfilter",
                serde_json::json!([info["result"]["bestblockhash"], "basic"]),
            );
            assert!(filter["error"].is_null(), "{filter}");
            assert_eq!(filter["result"]["filter"], vector["basic"]["encoded"]);
            assert_eq!(
                filter["result"]["header"],
                vector["basic"]["header_display"]
            );
            #[cfg(unix)]
            {
                assert!(
                    Command::new("/bin/kill")
                        .args(["-INT", &daemon.0.id().to_string()])
                        .status()
                        .expect("graceful signal")
                        .success()
                );
                let deadline = Instant::now() + Duration::from_secs(15);
                loop {
                    if let Some(status) = daemon.0.try_wait().expect("shutdown") {
                        if !status.success() {
                            let mut message = String::new();
                            daemon
                                .0
                                .stderr
                                .take()
                                .expect("stderr")
                                .read_to_string(&mut message)
                                .expect("shutdown diagnostic");
                            panic!("daemon shutdown {status}: {message}");
                        }
                        break;
                    }
                    assert!(Instant::now() < deadline, "bounded graceful shutdown");
                    std::thread::sleep(Duration::from_millis(20));
                }
            }
            drop(daemon);
            let store = open_bitcoin_node::FjallNodeStore::open(fixture.0.join("source"))
                .expect("closed durable observation");
            let snapshot = store
                .load_header_entries()
                .expect("saved exact headers")
                .expect("genesis headers");
            assert_eq!(snapshot.entries.len(), 1);
            assert_eq!(snapshot.entries[0].chain_work.to_hex(), vector["proof_hex"]);
            drop(store);
        }
    }
}

fn bounded_output(command: &mut Command) -> std::process::Output {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("owned daemon");
    let deadline = Instant::now() + Duration::from_secs(45);
    loop {
        if child.try_wait().expect("child status").is_some() {
            return child.wait_with_output().expect("closed child");
        }
        if Instant::now() >= deadline {
            child.kill().expect("terminate owned overdue child");
            child.wait().expect("reap owned child");
            panic!("bounded daemon refusal exceeded");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PhysicalEntry {
    relative: PathBuf,
    kind: fs::FileType,
    bytes: Vec<u8>,
    modified: std::time::SystemTime,
    mode: u32,
    maybe_link: Option<PathBuf>,
}

fn manifest(root: &std::path::Path) -> Vec<PhysicalEntry> {
    fn visit(root: &std::path::Path, current: &std::path::Path, entries: &mut Vec<PhysicalEntry>) {
        let metadata = fs::symlink_metadata(current).expect("metadata");
        #[cfg(unix)]
        let mode = {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode()
        };
        #[cfg(not(unix))]
        let mode = 0;
        entries.push(PhysicalEntry {
            relative: current
                .strip_prefix(root)
                .expect("relative source")
                .to_path_buf(),
            kind: metadata.file_type(),
            bytes: if metadata.is_file() {
                fs::read(current).expect("source bytes")
            } else {
                Vec::new()
            },
            modified: metadata.modified().expect("mtime"),
            mode,
            maybe_link: metadata
                .is_symlink()
                .then(|| fs::read_link(current).expect("link")),
        });
        if metadata.is_dir() {
            let mut children = fs::read_dir(current)
                .expect("source directory")
                .map(|entry| entry.expect("entry").path())
                .collect::<Vec<_>>();
            children.sort();
            for child in children {
                visit(root, &child, entries);
            }
        }
    }
    let mut entries = Vec::new();
    visit(root, root, &mut entries);
    entries
}

fn assert_redacted(message: &str, selector: &str) {
    assert!(
        message.contains(&format!("selected_network={selector}")),
        "{message}"
    );
    assert!(message.contains("observed_work=unavailable"), "{message}");
    assert!(message.contains("ordinary_history=false"), "{message}");
    assert!(message.contains("candidate_history"), "{message}");
    assert!(message.len() <= 1100, "{message}");
    for private in [
        "PRIVATE",
        "secret",
        "Address already in use",
        "phase166-daemon-",
    ] {
        assert!(!message.contains(private), "{message}");
    }
}

#[test]
fn phase166_daemon_activation_refuses_before_source_or_socket_effects() {
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    for shape in ["empty", "raw", "forged", "unsafe"] {
        let fixture = Fixture::new();
        let source = fixture.0.join("source");
        if shape == "raw" {
            drop(open_bitcoin_node::FjallNodeStore::open(&source).expect("closed unproved engine"));
        }
        if shape == "forged" {
            fs::write(
                source.join("snapshot.json"),
                "PRIVATE legacy synthetic-work malformed",
            )
            .expect("sentinel");
        }
        if shape == "unsafe" {
            fs::remove_dir(&source).expect("empty source");
            fs::create_dir(fixture.0.join("target")).expect("target");
            fs::write(fixture.0.join("target/wallet.dat"), "PRIVATE secret")
                .expect("wallet sentinel");
            #[cfg(unix)]
            std::os::unix::fs::symlink("target", &source).expect("unsafe source");
            #[cfg(not(unix))]
            fs::write(&source, "PRIVATE unsafe file").expect("unsafe source");
        }
        let before = manifest(&fixture.0);
        let occupied = TcpListener::bind("127.0.0.1:0").expect("occupied RPC port");
        for selector in ["mainnet", "testnet3", "signet", "regtest"] {
            for request in [
                "-openbitcoinsync=mainnet-ibd",
                "-openbitcoininbound=1",
                "-openbitcoinrelay=1",
                "-openbitcoinblockserving=1",
                "-openbitcoincompactrelay=1",
            ] {
                // Act
                let output = bounded_output(
                    fixture
                        .command(selector, occupied.local_addr().expect("port").port())
                        .arg(request),
                );
                // Assert
                assert!(!output.status.success());
                assert!(output.stdout.is_empty());
                let message = String::from_utf8(output.stderr).expect("bounded diagnostic");
                if request == "-openbitcoinsync=mainnet-ibd" && selector != "mainnet" {
                    assert!(
                        message.contains("requires -chain=main or -main"),
                        "{message}"
                    );
                    assert!(message.len() <= 1100);
                    assert!(
                        !message.contains("PRIVATE")
                            && !message.contains("secret")
                            && !message.contains("phase166-daemon-")
                    );
                } else {
                    assert_redacted(&message, selector);
                }
                assert_eq!(manifest(&fixture.0), before, "{shape} {selector} {request}");
            }
        }
    }
}

#[test]
fn phase166_daemon_no_trigger_and_unproved_local_sources_remain_unchanged() {
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    for unproved in [false, true] {
        let fixture = Fixture::new();
        if unproved {
            drop(
                open_bitcoin_node::FjallNodeStore::open(fixture.0.join("source"))
                    .expect("raw fixture"),
            );
        }
        let before = manifest(&fixture.0);
        for selector in ["mainnet", "testnet3", "signet", "regtest"] {
            let mut command = fixture.command(selector, 0);
            if !unproved {
                // Remove the explicit index trigger without changing other config parsing.
                command = Command::new(env!("CARGO_BIN_EXE_open-bitcoind"));
                command
                    .env_clear()
                    .env("HOME", &fixture.0)
                    .current_dir(&fixture.0)
                    .args([
                        format!("-datadir={}", fixture.0.join("source").display()),
                        format!("-chain={selector}"),
                        "-server=1".to_owned(),
                        "-rpcuser=private".to_owned(),
                        "-rpcpassword=secret".to_owned(),
                    ]);
            }
            let output = bounded_output(&mut command);
            assert!(!output.status.success());
            assert_redacted(
                &String::from_utf8(output.stderr).expect("message"),
                selector,
            );
            assert_eq!(manifest(&fixture.0), before);
        }
    }
}

#[test]
fn phase166_daemon_foreign_proved_sources_refuse_with_physical_preservation() {
    let _scenario = PROCESS_SCENARIO.lock().expect("process scenario");
    for stored in ["mainnet", "testnet3", "signet", "regtest"] {
        // Arrange: product startup publishes complete local genesis before occupied bind.
        let fixture = Fixture::new();
        let occupied = TcpListener::bind("127.0.0.1:0").expect("occupied RPC port");
        let port = occupied.local_addr().expect("port").port();
        let setup = bounded_output(&mut fixture.command(stored, port));
        assert!(!setup.status.success());
        assert!(
            String::from_utf8(setup.stderr)
                .expect("setup error")
                .contains("Address already in use")
        );
        let before = manifest(&fixture.0);
        for selected in ["mainnet", "testnet3", "signet", "regtest"] {
            if stored == selected {
                continue;
            }
            // Act
            let output = bounded_output(&mut fixture.command(selected, port));
            // Assert
            assert!(!output.status.success());
            assert_redacted(
                &String::from_utf8(output.stderr).expect("refusal"),
                selected,
            );
            assert_eq!(manifest(&fixture.0), before);
        }
    }
}
