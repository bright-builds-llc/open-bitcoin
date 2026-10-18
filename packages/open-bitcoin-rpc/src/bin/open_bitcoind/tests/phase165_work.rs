// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::canonical_chain::{NETWORKS, config, manifest, owned_path};
use super::*;
use open_bitcoin_rpc::config::load_runtime_config_for_args;

static NEXT_SOURCE_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn assert_work_diagnostic(message: &str, selected: &str, decision: &str) {
    assert!(
        message.contains(&format!("selected_network={selected}")),
        "{message}"
    );
    assert!(message.contains("observed_work=unavailable"), "{message}");
    assert!(
        message.contains("scope=identity_only; ordinary_history=false"),
        "{message}"
    );
    assert!(
        message.ends_with(&format!("work_decision={decision}")),
        "{message}"
    );
    assert!(message.len() <= 1024);
    for secret in ["PRIVATE", "rpcpassword", "wallet-secret", "backend-payload"] {
        assert!(!message.contains(secret), "diagnostic leaked {secret}");
    }
}

fn closed_source(shape: &str) -> PathBuf {
    let id = NEXT_SOURCE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let root = owned_path(&format!("phase165-PRIVATE-{id}"));
    fs::create_dir(&root).expect("owned source root");
    if shape == "durable" {
        drop(FjallNodeStore::open(root.join("store")).expect("closed fixture database"));
    }
    for (name, bytes) in [
        (
            "snapshot.json",
            r#"{"version":2,"chain_work":{"Exact":"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"},"backend-payload":"PRIVATE"}"#,
        ),
        ("work-profile", "canonical-exact-work-v1"),
        ("exact-work", "canonical-exact-work-v1"),
        ("wallet.dat", "wallet-secret"),
        (
            "bitcoin.conf",
            "rpcuser=private\nrpcpassword=wallet-secret\n",
        ),
        ("recovery", "malformed backend-payload"),
        ("pending-prune", "block undo intent sentinel"),
        ("basic-index", "malformed BASIC sentinel"),
    ] {
        fs::write(root.join(name), bytes).expect("closed sentinel");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(root.join("wallet.dat"), fs::Permissions::from_mode(0o600))
            .expect("private wallet permissions");
        std::os::unix::fs::symlink("wallet.dat", root.join("wallet-link"))
            .expect("owned sentinel link");
    }
    root
}

#[test]
fn phase165_daemon_loaded_startup_refuses_forged_sources_before_rpc_bind() {
    for shape in ["durable", "malformed"] {
        // Arrange
        let path = closed_source(shape);
        let before = manifest(&path);
        let occupied = std::net::TcpListener::bind("127.0.0.1:0").expect("occupied RPC port");
        for (_, _, selected) in NETWORKS {
            for metrics in [false, true] {
                let args = [
                    format!("-datadir={}", path.display()),
                    format!("-chain={selected}"),
                    "-server=1".to_owned(),
                    "-rpcuser=private".to_owned(),
                    "-rpcpassword=wallet-secret".to_owned(),
                    format!("-rpcport={}", occupied.local_addr().expect("port").port()),
                    format!("-openbitcoininbound={}", u8::from(metrics)),
                    "-blockfilterindex=0".to_owned(),
                ]
                .map(std::ffi::OsString::from);
                // Act
                let runtime =
                    load_runtime_config_for_args(&args, &path).expect("valid startup config");
                let result = crate::open_runtime_store(&runtime);
                // Assert
                let Err(refusal) = result else {
                    panic!("ordinary startup work refusal");
                };
                let message = refusal.to_string();
                assert_work_diagnostic(&message, selected, "existing_unproved");
                assert_eq!(runtime.inbound.enabled, metrics);
                assert!(!runtime.sync.is_enabled());
                assert_eq!(manifest(&path), before);
            }
        }
        remove_dir_if_exists(&path);
    }
}

#[test]
fn phase165_daemon_missing_empty_and_metrics_fallback_keep_closed_shapes() {
    for (address, sync, selected) in NETWORKS {
        for empty in [false, true] {
            // Arrange
            let path = owned_path("phase165-missing-empty");
            if empty {
                fs::create_dir(&path).expect("empty source");
            }
            let maybe_before = empty.then(|| manifest(&path));
            let runtime = config(&path, address, sync, 2);
            let context = Arc::new(tokio::sync::Mutex::new(
                ManagedRpcContext::from_fixture_runtime_config(&RuntimeConfig::default()),
            ));
            // Act
            let startup = crate::open_runtime_store(&runtime);
            let metrics =
                crate::inbound_metrics::start_inbound_metrics_worker(&runtime, context, None);
            // Assert
            let Err(startup_refusal) = startup else {
                panic!("startup refusal");
            };
            let Err(metrics_refusal) = metrics else {
                panic!("metrics refusal");
            };
            for message in [startup_refusal, metrics_refusal] {
                assert_work_diagnostic(&message.to_string(), selected, "bootstrap_unavailable");
            }
            assert_eq!(path.exists(), empty);
            if let Some(before) = maybe_before {
                assert_eq!(manifest(&path), before);
            }
            remove_dir_if_exists(&path);
        }
    }
}

#[test]
fn phase165_daemon_metrics_cannot_promote_forged_exact_work() {
    // Arrange
    let path = closed_source("durable");
    let before = manifest(&path);
    for (address, sync, selected) in NETWORKS {
        let runtime = config(&path, address, sync, 2);
        let context = Arc::new(tokio::sync::Mutex::new(
            ManagedRpcContext::from_fixture_runtime_config(&RuntimeConfig::default()),
        ));
        // Act
        let result = crate::inbound_metrics::start_inbound_metrics_worker(&runtime, context, None);
        // Assert
        let Err(refusal) = result else {
            panic!("metrics work refusal");
        };
        assert_work_diagnostic(&refusal.to_string(), selected, "existing_unproved");
        assert_eq!(manifest(&path), before);
    }
    remove_dir_if_exists(&path);
}
