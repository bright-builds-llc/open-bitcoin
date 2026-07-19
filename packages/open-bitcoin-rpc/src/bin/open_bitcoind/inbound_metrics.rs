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
    sync::{Arc, mpsc},
    thread,
    time::Duration,
};

use open_bitcoin_node::{
    FjallNodeStore, MetricRetentionPolicy, PersistMode, inbound_metric_samples,
    relay_metric_samples,
};
use open_bitcoin_rpc::{ManagedRpcContext, config::RuntimeConfig};

use super::{DaemonSyncPreflightError, current_timestamp_unix_seconds};

pub(super) struct InboundMetricsWorker {
    pub(super) join_handle: thread::JoinHandle<()>,
    pub(super) shutdown_sender: mpsc::Sender<()>,
    pub(super) metrics_store: FjallNodeStore,
}

impl InboundMetricsWorker {
    pub(super) fn shutdown(self) {
        let _ = self.shutdown_sender.send(());
        if let Err(error) = self.join_handle.join() {
            eprintln!("open-bitcoind inbound metrics worker shutdown join failed: {error:?}");
        }
    }
}

pub(super) fn start_inbound_metrics_worker(
    runtime: &RuntimeConfig,
    shared_context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    maybe_store: Option<FjallNodeStore>,
) -> Result<Option<InboundMetricsWorker>, DaemonSyncPreflightError> {
    if !runtime.inbound.enabled || runtime.sync.is_enabled() {
        return Ok(None);
    }

    let Some(data_dir) = runtime.maybe_data_dir.as_ref() else {
        return Ok(None);
    };

    let store = match maybe_store {
        Some(store) => store,
        None => FjallNodeStore::open(data_dir).map_err(|error| {
            DaemonSyncPreflightError::new(format!(
                "open-bitcoind inbound metrics failed to open durable store at \"{}\": {error}",
                data_dir.display()
            ))
        })?,
    };
    let metrics_store = store.clone();
    let retention = MetricRetentionPolicy::default();
    let persist_mode = runtime.sync.runtime.persist_mode;
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();

    Ok(Some(InboundMetricsWorker {
        join_handle: thread::spawn(move || {
            inbound_metrics_worker(
                store,
                retention,
                persist_mode,
                shared_context,
                shutdown_receiver,
            )
        }),
        shutdown_sender,
        metrics_store,
    }))
}

fn inbound_metrics_worker(
    store: FjallNodeStore,
    retention: MetricRetentionPolicy,
    persist_mode: PersistMode,
    shared_context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    shutdown_receiver: mpsc::Receiver<()>,
) {
    loop {
        persist_inbound_metrics_once(&store, retention, persist_mode, Arc::clone(&shared_context));
        let wait_seconds = retention.sample_interval_seconds.max(1);
        match shutdown_receiver.recv_timeout(Duration::from_secs(wait_seconds)) {
            Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }
}

fn persist_inbound_metrics_once(
    store: &FjallNodeStore,
    retention: MetricRetentionPolicy,
    persist_mode: PersistMode,
    shared_context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
) {
    let timestamp = current_timestamp_unix_seconds();
    let context = shared_context.blocking_lock();
    let maybe_snapshot = context.authoritative_operator_snapshot().ok();
    drop(context);
    let Some(snapshot) = maybe_snapshot else {
        return;
    };
    let timestamp = u64::try_from(timestamp).unwrap_or(0);
    let inbound = snapshot.inbound().clone();
    let relay = snapshot.relay().clone();
    let mut samples = inbound_metric_samples(&inbound, timestamp);
    samples.extend(relay_metric_samples(&relay, timestamp));
    if samples.is_empty() {
        return;
    }
    if let Err(error) = store.append_metric_samples(&samples, retention, timestamp, persist_mode) {
        eprintln!("open-bitcoind inbound metrics persistence failed: {error}");
    }
}
