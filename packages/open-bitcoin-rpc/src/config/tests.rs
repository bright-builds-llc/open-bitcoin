// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use std::{
    ffi::OsString,
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use open_bitcoin_node::{SyncNetwork, SyncPeerAddress, core::wallet::AddressNetwork};

use open_bitcoin_network::{
    InboundAdmissionSlotClass, InboundPreflightReason, PeerConnectionClass, PermissionEffectLabel,
    classify_inbound_preflight,
};

use super::{
    ConfigPrecedence, ConfigSource, DEFAULT_COOKIE_FILE_NAME, DaemonSyncMode, OpenBitcoinConfig,
    RpcAuthConfig, RuntimeConfig, WalletRuntimeConfig, WalletRuntimeScope,
    load_runtime_config_for_args, parse_open_bitcoin_jsonc_config,
};

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-rpc-config-tests-{label}-{}",
            NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&directory).expect("test directory");
        Self { path: directory }
    }

    fn child(&self, relative: &str) -> PathBuf {
        self.path.join(relative)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn cli_arg(name: &str, value: &Path) -> OsString {
    OsString::from(format!("-{name}={}", value.display()))
}

fn os(value: &str) -> OsString {
    OsString::from(value)
}

mod baseline_and_auth;
mod inbound_cli;
mod inbound_jsonc;
mod precedence_and_scope;
mod runtime_activation;
mod sync_configuration;
