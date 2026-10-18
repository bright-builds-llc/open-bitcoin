// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_node::FjallNodeStore;
use std::{
    collections::BTreeMap,
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant, SystemTime},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    kind: fs::FileType,
    bytes: Vec<u8>,
    length: u64,
    modified: SystemTime,
    readonly: bool,
    maybe_link: Option<PathBuf>,
    mode: u32,
}

fn manifest(root: &Path) -> BTreeMap<PathBuf, Entry> {
    fn visit(root: &Path, path: &Path, entries: &mut BTreeMap<PathBuf, Entry>) {
        let metadata = fs::symlink_metadata(path).expect("owned metadata");
        let kind = metadata.file_type();
        #[cfg(unix)]
        let mode = {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode()
        };
        #[cfg(not(unix))]
        let mode = 0;
        entries.insert(
            path.strip_prefix(root)
                .expect("relative path")
                .to_path_buf(),
            Entry {
                kind,
                bytes: if kind.is_file() {
                    fs::read(path).expect("closed bytes")
                } else {
                    Vec::new()
                },
                length: metadata.len(),
                modified: metadata.modified().expect("mtime"),
                readonly: metadata.permissions().readonly(),
                maybe_link: kind
                    .is_symlink()
                    .then(|| fs::read_link(path).expect("link target")),
                mode,
            },
        );
        if kind.is_dir() {
            for child in fs::read_dir(path).expect("owned directory") {
                visit(root, &child.expect("owned child").path(), entries);
            }
        }
    }
    let mut entries = BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir()
            .canonicalize()
            .expect("physical temp parent")
            .join(format!(
                "phase165-daemon-startup-{}-{}",
                std::process::id(),
                NEXT_ID.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir(&root).expect("owned root");
        Self(root)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove owned source");
    }
}

fn run_bounded(command: &mut Command) -> Output {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("normal daemon process");
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if child.try_wait().expect("owned child status").is_some() {
            return child.wait_with_output().expect("daemon output");
        }
        if Instant::now() >= deadline {
            child.kill().expect("terminate only timed-out owned daemon");
            let output = child.wait_with_output().expect("reap timed-out daemon");
            panic!(
                "normal daemon exceeded 15-second refusal deadline: {:?}",
                output.status
            );
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn phase165_normal_daemon_main_refuses_before_rpc_bind_or_source_effects() {
    for shape in ["missing", "empty", "forged", "unsafe"] {
        // Arrange
        let fixture = Fixture::new();
        let root = &fixture.0;
        let source = root.join("source");
        if shape == "forged" {
            drop(FjallNodeStore::open(&source).expect("closed raw source fixture"));
            for (name, bytes) in [
                ("snapshot.json", "malformed PRIVATE-backend-payload"),
                ("work-profile", "canonical-exact-work-v1"),
                ("exact-work", "canonical-exact-work-v1"),
                ("wallet.dat", "PRIVATE-wallet-secret"),
                ("prune-intent", "block undo sentinel"),
                ("basic-index", "malformed BASIC recovery"),
            ] {
                fs::write(source.join(name), bytes).expect("closed source sentinels");
            }
        } else if shape == "empty" {
            fs::create_dir(&source).expect("empty source");
        }
        if shape == "unsafe" {
            fs::create_dir(root.join("target")).expect("owned link target");
            fs::write(root.join("target/wallet.dat"), "PRIVATE-wallet-secret")
                .expect("target sentinel");
            #[cfg(unix)]
            std::os::unix::fs::symlink("target", &source).expect("source alias");
            #[cfg(not(unix))]
            fs::write(&source, "PRIVATE unsafe file").expect("unsafe regular file");
        }
        let conf = root.join("bitcoin.conf");
        fs::write(
            &conf,
            "rpcuser=private\nrpcpassword=PRIVATE-wallet-secret\n",
        )
        .expect("valid config");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&conf, fs::Permissions::from_mode(0o600))
                .expect("private config mode");
            std::os::unix::fs::symlink("bitcoin.conf", root.join("config-link"))
                .expect("config sentinel link");
        }
        let before = manifest(root);
        let occupied = TcpListener::bind("127.0.0.1:0").expect("occupied RPC port");
        for selector in ["mainnet", "testnet3", "signet", "regtest"] {
            for metrics in [false, true] {
                let mut command = Command::new(env!("CARGO_BIN_EXE_open-bitcoind"));
                command
                    .env_clear()
                    .env("HOME", root)
                    .current_dir(root)
                    .args([
                        format!("-conf={}", conf.display()),
                        format!("-chain={selector}"),
                        "-server=1".to_owned(),
                        "-rpcbind=127.0.0.1".to_owned(),
                        format!(
                            "-rpcport={}",
                            occupied.local_addr().expect("RPC port").port()
                        ),
                        "-blockfilterindex=0".to_owned(),
                        format!("-openbitcoininbound={}", u8::from(metrics)),
                        "-openbitcoinlisten=127.0.0.1:0".to_owned(),
                    ]);
                if shape != "missing" {
                    command.arg(format!("-datadir={}", source.display()));
                }
                // Act
                let output = run_bounded(&mut command);
                // Assert
                assert!(!output.status.success());
                assert!(output.stdout.is_empty());
                let message = String::from_utf8(output.stderr).expect("UTF-8 diagnostic");
                let decision = match shape {
                    "missing" | "empty" => "bootstrap_unavailable",
                    "unsafe" => "unsafe_path",
                    _ => "existing_unproved",
                };
                assert!(
                    message.contains(&format!("selected_network={selector}")),
                    "{message}"
                );
                assert!(message.contains("observed_work=unavailable"), "{message}");
                assert!(
                    message.contains("scope=identity_only; ordinary_history=false"),
                    "{message}"
                );
                assert!(
                    message.contains(&format!("work_decision={decision}")),
                    "{message}"
                );
                assert!(message.len() <= 1100);
                assert!(
                    !message.contains(&root.display().to_string()),
                    "source path leaked"
                );
                for secret in [
                    "PRIVATE",
                    "wallet-secret",
                    "backend-payload",
                    "Address already in use",
                ] {
                    assert!(!message.contains(secret), "{message}");
                }
                assert_eq!(
                    manifest(root),
                    before,
                    "{shape} {selector} metrics={metrics}"
                );
            }
        }
    }
}
