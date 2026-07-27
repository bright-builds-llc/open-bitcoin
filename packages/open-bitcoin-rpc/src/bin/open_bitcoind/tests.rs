// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use open_bitcoin_network::{
    InboundListenerConfig, InboundPreflightReason, VersionMessage, WireNetworkMessage,
};
use open_bitcoin_node::{
    DurableSyncRuntime, FieldAvailability, FjallNodeStore, MetricKind, ResolvedSyncPeerAddress,
    SyncLifecycleState, SyncPeerAddress, SyncPeerReceiveOutcome, SyncPeerSession, SyncRunSummary,
    SyncRuntimeConfig, SyncRuntimeError, SyncStopReason, SyncTransport,
};
use open_bitcoin_rpc::inbound_listener::InboundListenerState;
use open_bitcoin_rpc::{
    ManagedRpcContext,
    config::{DaemonSyncConfig, RuntimeConfig},
};

use super::{
    DaemonSyncLoopDecision, DaemonSyncLoopPolicy, DaemonSyncPreflight, InboundDaemonListener,
    daemon_sync_preflight_message, daemon_sync_worker_with_transport,
    inbound_listener_startup_message, open_authoritative_network_runtime, preflight_daemon_sync,
    run_daemon_sync_loop_cycle, start_inbound_listener_for_runtime,
    start_inbound_listener_for_runtime_with_context, start_inbound_metrics_worker,
};

fn temp_store_path(label: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!(
        "open-bitcoind-sync-preflight-{label}-{}-{timestamp}",
        std::process::id()
    ))
}

fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

fn test_sync_runtime(label: &str) -> DurableSyncRuntime {
    let data_dir = temp_store_path(label);
    remove_dir_if_exists(&data_dir);
    let store = FjallNodeStore::open(&data_dir).expect("test store");
    DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            dns_seeds: Vec::new(),
            manual_peers: Vec::new(),
            retry_backoff_ms: 25,
            ..SyncRuntimeConfig::default()
        },
    )
    .expect("test sync runtime")
}

#[derive(Debug)]
struct SilentPeerTransport {
    receive_calls: Arc<AtomicUsize>,
}

#[derive(Debug)]
struct SilentPeerSession {
    receive_calls: Arc<AtomicUsize>,
}

impl SyncTransport for SilentPeerTransport {
    type Session = SilentPeerSession;

    fn connect(
        &mut self,
        _peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        Ok(SilentPeerSession {
            receive_calls: Arc::clone(&self.receive_calls),
        })
    }
}

impl SyncPeerSession for SilentPeerSession {
    fn send(
        &mut self,
        _message: &WireNetworkMessage,
        _magic: open_bitcoin_node::core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_node::core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        let call = self.receive_calls.fetch_add(1, Ordering::SeqCst);
        Ok(match call {
            0 => SyncPeerReceiveOutcome::Message(WireNetworkMessage::Version(
                VersionMessage::default(),
            )),
            1 => SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
            _ => {
                thread::sleep(Duration::from_millis(25));
                SyncPeerReceiveOutcome::Idle
            }
        })
    }
}

fn silent_peer_sync_runtime(label: &str) -> DurableSyncRuntime {
    let data_dir = temp_store_path(label);
    remove_dir_if_exists(&data_dir);
    let store = FjallNodeStore::open(&data_dir).expect("test store");
    DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("127.0.0.1", 18_444)],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            max_peer_retries: 0,
            retry_backoff_ms: 25,
            ..SyncRuntimeConfig::default()
        },
    )
    .expect("silent-peer sync runtime")
}

#[path = "tests/daemon_sync.rs"]
mod daemon_sync;
#[path = "tests/inbound_runtime.rs"]
mod inbound_runtime;
