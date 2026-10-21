// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::SystemTime,
};

use open_bitcoin_node::{
    FjallNodeStore, PersistMode, RuntimeMetadata, core::consensus::CanonicalChain,
};

use super::*;
pub(crate) mod phase165_work;
mod phase166_genesis;
use crate::operator::runtime::{OperatorRuntimeError, support::execute_offline_sync_command};
use crate::operator::{
    NetworkSelection, OperatorOutputFormat, SyncArgs, SyncCommand,
    config::{
        OPEN_BITCOIN_NETWORK_ENV, OperatorConfigRequest, OperatorConfigResolution,
        OperatorConfigRoots, resolve_operator_config,
    },
};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);
pub(crate) const CHAINS: [CanonicalChain; 4] = [
    CanonicalChain::Mainnet,
    CanonicalChain::Testnet3,
    CanonicalChain::Signet,
    CanonicalChain::Regtest,
];
pub(crate) const NETWORKS: [NetworkSelection; 4] = [
    NetworkSelection::Mainnet,
    NetworkSelection::Testnet,
    NetworkSelection::Signet,
    NetworkSelection::Regtest,
];
pub(crate) const SHAPES: [&str; 7] = [
    "missing",
    "empty",
    "durable",
    "malformed",
    "canonical_looking",
    "file",
    "symlink",
];

/// Closed owned sources only; raw opening arranges evidence, never the operator attempt.
pub(crate) struct SourceFixture {
    pub(crate) root: PathBuf,
    pub(crate) source: PathBuf,
}

impl SourceFixture {
    pub(crate) fn new(shape: &str) -> Self {
        let root = std::env::temp_dir()
            .canonicalize()
            .expect("real temp root")
            .join(format!(
                "open-bitcoin-cli-canonical-{}-{}",
                std::process::id(),
                NEXT_ID.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir(&root).expect("owned root");
        let source = root.join("source");
        match shape {
            "missing" => {}
            "empty" => fs::create_dir(&source).expect("empty source"),
            "durable" => {
                let store = FjallNodeStore::open(&source).expect("raw fixture");
                store
                    .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
                    .expect("metadata sentinel");
                let mut registry = open_bitcoin_node::WalletRegistry::default();
                registry
                    .create_wallet(
                        &store,
                        "alpha",
                        open_bitcoin_node::core::wallet::Wallet::new(
                            open_bitcoin_node::core::wallet::AddressNetwork::Regtest,
                        ),
                        PersistMode::Sync,
                    )
                    .expect("wallet sentinel");
                registry
                    .set_selected_wallet(&store, "alpha", PersistMode::Sync)
                    .expect("selected wallet sentinel");
                drop(store);
            }
            "malformed" | "canonical_looking" => {
                fs::create_dir(&source).expect("source");
                fs::write(
                    source.join("genesis"),
                    if shape == "canonical_looking" {
                        b"000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
                            .as_slice()
                    } else {
                        b"rpcpassword=secret; malformed backend".as_slice()
                    },
                )
                .expect("sentinel");
            }
            "file" => fs::write(&source, b"unsafe source sentinel").expect("file"),
            "symlink" => {
                let target = root.join("target");
                fs::create_dir(&target).expect("target");
                fs::write(target.join("sentinel"), b"target bytes").expect("target sentinel");
                #[cfg(unix)]
                std::os::unix::fs::symlink(&target, &source).expect("symlink");
            }
            _ => panic!("unknown fixture shape"),
        }
        Self { root, source }
    }

    pub(crate) fn snapshot(&self) -> Vec<PhysicalEntry> {
        physical_manifest(&self.root)
    }
}

impl Drop for SourceFixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).expect("remove owned source");
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PhysicalEntry {
    relative: PathBuf,
    length: u64,
    modified: SystemTime,
    readonly: bool,
    mode: u32,
    kind: &'static str,
    bytes: Vec<u8>,
    maybe_target: Option<PathBuf>,
}

pub(crate) fn physical_manifest(root: &Path) -> Vec<PhysicalEntry> {
    fn visit(root: &Path, path: &Path, entries: &mut Vec<PhysicalEntry>) {
        let meta = fs::symlink_metadata(path).expect("manifest metadata");
        #[cfg(unix)]
        let mode = {
            use std::os::unix::fs::PermissionsExt;
            meta.permissions().mode()
        };
        #[cfg(not(unix))]
        let mode = 0;
        entries.push(PhysicalEntry {
            relative: path.strip_prefix(root).expect("relative").to_path_buf(),
            length: meta.len(),
            modified: meta.modified().expect("mtime"),
            readonly: meta.permissions().readonly(),
            mode,
            kind: if meta.is_dir() {
                "directory"
            } else if meta.is_symlink() {
                "symlink"
            } else {
                "file"
            },
            bytes: if meta.is_file() {
                fs::read(path).expect("manifest bytes")
            } else {
                Vec::new()
            },
            maybe_target: if meta.is_symlink() {
                Some(fs::read_link(path).expect("target"))
            } else {
                None
            },
        });
        if !meta.is_dir() {
            return;
        }
        let mut children = fs::read_dir(path)
            .expect("manifest directory")
            .map(|entry| entry.expect("entry").path())
            .collect::<Vec<_>>();
        children.sort();
        for child in children {
            visit(root, &child, entries);
        }
    }
    let mut entries = Vec::new();
    visit(root, root, &mut entries);
    entries
}

pub(crate) fn serve_rpc_response(
    fixture: &SourceFixture,
    status: u16,
    body: &str,
) -> (OperatorConfigResolution, std::thread::JoinHandle<String>) {
    use std::{
        io::{Read, Write},
        net::TcpListener,
        time::{Duration, Instant},
    };
    let listener = TcpListener::bind("127.0.0.1:0").expect("local test listener");
    let port = listener.local_addr().expect("local address").port();
    listener.set_nonblocking(true).expect("nonblocking");
    let body = body.to_owned();
    let handle = std::thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(error)
                    if error.kind() == std::io::ErrorKind::WouldBlock
                        && Instant::now() < deadline =>
                {
                    std::thread::sleep(Duration::from_millis(5))
                }
                Err(error) => panic!("test RPC accept: {error}"),
            }
        };
        stream
            .set_nonblocking(false)
            .expect("blocking accepted RPC stream");
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .expect("read timeout");
        let mut buffer = [0u8; 8192];
        let count = stream.read(&mut buffer).expect("test RPC request");
        stream.write_all(format!("HTTP/1.1 {status} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).expect("test RPC response");
        String::from_utf8(buffer[..count].to_vec()).expect("request text")
    });
    let conf = fixture.root.join("bitcoin.conf");
    fs::write(
        &conf,
        format!("rpcuser=test\nrpcpassword=test\nrpcport={port}\nregtest=1\n"),
    )
    .expect("test config");
    (
        OperatorConfigResolution {
            maybe_data_dir: Some(fixture.source.clone()),
            maybe_bitcoin_conf_path: Some(conf),
            maybe_network: Some(NetworkSelection::Regtest),
            ..OperatorConfigResolution::default()
        },
        handle,
    )
}

#[test]
fn chain_compatibility_live_rpc_failure_never_downgrades_to_offline() {
    for (status, body, expected) in [
        (401, "{}", "RPC authentication failed"),
        (
            200,
            r#"{"error":{"code":-8,"message":"visible operation refusal"},"id":1}"#,
            "visible operation refusal",
        ),
    ] {
        // Arrange
        let fixture = SourceFixture::new("durable");
        let (resolution, server) = serve_rpc_response(&fixture, status, body);
        let before = fixture.snapshot();
        // Act
        let error = crate::operator::runtime::support::execute_sync_command(
            &SyncArgs {
                command: SyncCommand::Pause,
            },
            OperatorOutputFormat::Json,
            &resolution,
        )
        .expect_err("visible live failure");
        // Assert
        assert!(error.to_string().contains(expected), "{error}");
        assert!(matches!(error, OperatorRuntimeError::InvalidRequest { .. }));
        let request = server.join().expect("RPC server");
        assert!(request.starts_with("POST / HTTP/1.1"));
        assert_eq!(fixture.snapshot(), before);
    }
}

#[test]
fn chain_compatibility_offline_controls_preserve_every_source_and_selector() {
    for shape in SHAPES {
        // Arrange
        let fixture = SourceFixture::new(shape);
        let before = fixture.snapshot();
        for chain in CHAINS {
            for command in [SyncCommand::Status, SyncCommand::Pause, SyncCommand::Resume] {
                for format in [OperatorOutputFormat::Human, OperatorOutputFormat::Json] {
                    // Act
                    let error = execute_offline_sync_command(
                        &fixture.source,
                        chain,
                        &SyncArgs {
                            command: command.clone(),
                        },
                        format,
                    )
                    .expect_err("ordinary refusal");
                    // Assert
                    let OperatorRuntimeError::CompatibilityRefused { refusal } = error else {
                        panic!("typed refusal required");
                    };
                    assert_eq!(refusal.selected_chain(), chain);
                    assert!(refusal.to_string().contains("observed_network=unavailable"));
                    assert_eq!(fixture.snapshot(), before, "{shape} {chain}");
                }
            }
        }
    }
}

#[test]
fn chain_compatibility_selection_uses_resolved_precedence_and_default() {
    // Arrange
    let fixture = SourceFixture::new("empty");
    let roots = OperatorConfigRoots::new(fixture.source.clone());
    let env = BTreeMap::from([(OPEN_BITCOIN_NETWORK_ENV.to_owned(), "signet".to_owned())]);
    for (network, chain) in NETWORKS.into_iter().zip(CHAINS) {
        let request = OperatorConfigRequest {
            maybe_network: Some(network),
            ..OperatorConfigRequest::default()
        };
        // Act
        let resolution = resolve_operator_config(&request, &env, &roots).expect("resolution");
        // Assert
        assert_eq!(
            canonical_chain_from_resolution(&resolution).expect("selection"),
            chain
        );
    }
    let resolution = resolve_operator_config(&OperatorConfigRequest::default(), &env, &roots)
        .expect("env resolution");
    assert_eq!(
        canonical_chain_from_resolution(&resolution).expect("env selection"),
        CanonicalChain::Signet
    );
    assert_eq!(
        canonical_chain_from_resolution(&OperatorConfigResolution::default()).expect("default"),
        CanonicalChain::Mainnet
    );
    let config_path = fixture.root.join("open-bitcoin.jsonc");
    for (selector, chain) in ["mainnet", "testnet", "signet", "regtest"]
        .into_iter()
        .zip(CHAINS)
    {
        fs::write(
            &config_path,
            serde_json::json!({"onboarding": {"wizard_answers": {"network": selector}}})
                .to_string(),
        )
        .expect("config selection fixture");
        let request = OperatorConfigRequest {
            maybe_config_path: Some(config_path.clone()),
            ..OperatorConfigRequest::default()
        };
        let resolution =
            resolve_operator_config(&request, &BTreeMap::new(), &roots).expect("JSONC selection");
        assert_eq!(
            canonical_chain_from_resolution(&resolution).expect("configured chain"),
            chain
        );
    }
}

#[test]
fn chain_compatibility_refusal_is_bounded_and_redacted() {
    // Arrange
    let hostile = PathBuf::from(format!("/rpcpassword=secret/{}", "hostile".repeat(4096)));
    for chain in CHAINS {
        // Act
        let refusal = FjallNodeStore::open_ordinary(&hostile, chain)
            .err()
            .expect("refusal");
        let error = OperatorRuntimeError::CompatibilityRefused { refusal };
        let text = error.to_string();
        let json = serde_json::to_string(&refusal).expect("json");
        // Assert
        assert!(text.len() <= 1024 && json.len() <= 1024);
        assert!(text.contains("scope=identity_only"));
        assert!(text.contains("observed_root=unavailable; observed_work=unavailable"));
        assert!(!text.contains("rpcpassword") && !json.contains("hostile"));
    }
}
