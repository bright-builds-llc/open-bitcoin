#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented,
        clippy::panic_in_result_fn,
    )
)]
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
    error::Error,
    sync::{Arc, mpsc},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use open_bitcoin_network::{InboundPreflightDiagnostic, InboundPreflightReason};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallNodeStore, ManagedNetworkHandle, SyncLifecycleState, SyncRunSummary,
    SyncRuntimeError, SyncStopReason, TcpPeerTransport, status::inbound_status_unavailable,
};
use open_bitcoin_rpc::{
    DaemonSyncControl, ManagedRpcContext,
    config::{RuntimeConfig, load_runtime_config},
    http,
    inbound_listener::{
        InboundListenerState, InboundListenerWorker, activate_inbound_listener,
        start_inbound_accept_loop,
    },
};

#[path = "open_bitcoind/inbound_metrics.rs"]
mod inbound_metrics;
#[path = "open_bitcoind/sync_seed.rs"]
mod sync_seed;

use inbound_metrics::start_inbound_metrics_worker;
#[cfg(test)]
use sync_seed::{DaemonSyncPreflight, daemon_sync_preflight_message};
use sync_seed::{preflight_daemon_sync, report_daemon_sync_preflight, seed_initial_sync_state};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = load_runtime_config()?;
    if !runtime.rpc_server.enabled {
        return Err("RPC server is disabled by configuration".into());
    }
    let bind_address = runtime.rpc_server.bind_address;
    let auth = runtime.rpc_server.auth.clone();
    let maybe_runtime_store = open_runtime_store(&runtime)?;
    let mut authoritative_runtime =
        open_authoritative_network_runtime(&runtime, maybe_runtime_store.clone())?;
    if let Some(preflight) =
        preflight_daemon_sync(&runtime, authoritative_runtime.maybe_sync_runtime.as_ref())?
    {
        report_daemon_sync_preflight(&preflight);
    }
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &runtime,
        authoritative_runtime.network.clone(),
        maybe_runtime_store.clone(),
    )?;
    let shared_context = Arc::new(tokio::sync::Mutex::new(context));
    let maybe_sync_worker = start_daemon_sync_worker(
        &runtime,
        Arc::clone(&shared_context),
        authoritative_runtime.maybe_sync_runtime.take(),
    )?;
    if let Some(worker) = maybe_sync_worker.as_ref() {
        let mut context = shared_context.lock().await;
        context.set_daemon_sync_control(worker.control.clone());
        context.set_metrics_store(worker.metrics_store.clone());
    }
    let state = http::build_http_state_with_shared_context(auth, Arc::clone(&shared_context))?;
    let listener = tokio::net::TcpListener::bind(bind_address).await?;
    let mut inbound_listener =
        start_inbound_listener_for_runtime_with_context(&runtime, Arc::clone(&shared_context))
            .await;
    let maybe_inbound_metrics_worker =
        start_inbound_metrics_worker(&runtime, Arc::clone(&shared_context), maybe_runtime_store)?;
    if let Some(worker) = maybe_inbound_metrics_worker.as_ref() {
        shared_context
            .lock()
            .await
            .set_metrics_store(worker.metrics_store.clone());
    }
    report_inbound_listener_startup(&inbound_listener);

    let serve_result = axum::serve(listener, http::router(state))
        .with_graceful_shutdown(shutdown_signal())
        .await;
    inbound_listener.shutdown().await;
    if let Some(worker) = maybe_inbound_metrics_worker {
        worker.shutdown();
    }
    if let Some(worker) = maybe_sync_worker {
        worker.shutdown();
    }
    serve_result?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DaemonSyncPreflightError {
    message: String,
}

struct DaemonSyncWorker {
    join_handle: thread::JoinHandle<()>,
    shutdown_sender: mpsc::Sender<()>,
    control: DaemonSyncControl,
    metrics_store: FjallNodeStore,
}

struct AuthoritativeNetworkRuntime {
    network: ManagedNetworkHandle,
    maybe_sync_runtime: Option<DurableSyncRuntime>,
}

#[derive(Debug)]
struct InboundDaemonListener {
    state: InboundListenerState,
    preflight_reason: InboundPreflightReason,
    bound_endpoints: Vec<String>,
    diagnostics: Vec<InboundPreflightDiagnostic>,
    maybe_worker: Option<InboundListenerWorker>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DaemonSyncLoopPolicy {
    sleep_duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DaemonSyncLoopDecision {
    RetryAfter(Duration),
    Paused(Duration),
    Stopped,
    Failed(Duration),
}

impl DaemonSyncLoopPolicy {
    fn from_runtime(sync_runtime: &DurableSyncRuntime) -> Self {
        Self {
            sleep_duration: Duration::from_millis(
                sync_runtime.config().retry_backoff_ms.max(1_000),
            ),
        }
    }
}

impl DaemonSyncLoopDecision {
    fn sleep_duration(self) -> Duration {
        match self {
            Self::RetryAfter(duration) | Self::Paused(duration) | Self::Failed(duration) => {
                duration
            }
            Self::Stopped => Duration::from_millis(0),
        }
    }
}

impl DaemonSyncWorker {
    fn shutdown(self) {
        let _ = self.shutdown_sender.send(());
        if let Err(error) = self.join_handle.join() {
            eprintln!("open-bitcoind daemon sync worker shutdown join failed: {error:?}");
        }
    }
}

impl InboundDaemonListener {
    async fn shutdown(&mut self) {
        let Some(worker) = self.maybe_worker.take() else {
            return;
        };
        worker.shutdown().await;
    }
}

impl DaemonSyncPreflightError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl core::fmt::Display for DaemonSyncPreflightError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for DaemonSyncPreflightError {}

fn open_authoritative_network_runtime(
    runtime: &RuntimeConfig,
    maybe_store: Option<FjallNodeStore>,
) -> Result<AuthoritativeNetworkRuntime, DaemonSyncPreflightError> {
    if let Some(store) = maybe_store {
        let sync_runtime = DurableSyncRuntime::open_with_runtime_activation(
            store,
            runtime.sync.runtime.clone(),
            runtime.relay,
            runtime.block_serving,
            runtime.inbound.enabled,
        )
        .map_err(|error| {
            DaemonSyncPreflightError::new(format!(
                "open-bitcoind failed to construct authoritative network runtime: {error}"
            ))
        })?;
        return Ok(AuthoritativeNetworkRuntime {
            network: sync_runtime.network_handle(),
            maybe_sync_runtime: Some(sync_runtime),
        });
    }

    if runtime.sync.is_enabled() {
        return Err(DaemonSyncPreflightError::new(
            "open-bitcoind mainnet sync activation requires an existing datadir; set -datadir=<path> or create the default Bitcoin datadir before enabling -openbitcoinsync=mainnet-ibd.",
        ));
    }

    Ok(AuthoritativeNetworkRuntime {
        network: ManagedNetworkHandle::transient_runtime(
            runtime.sync.runtime.network.magic(),
            runtime.sync.runtime.network.default_port(),
            runtime.relay,
            runtime.block_serving,
            runtime.inbound.enabled,
        ),
        maybe_sync_runtime: None,
    })
}

fn report_inbound_listener_startup(listener: &InboundDaemonListener) {
    eprintln!("{}", inbound_listener_startup_message(listener));
}

fn open_runtime_store(
    runtime: &RuntimeConfig,
) -> Result<Option<FjallNodeStore>, DaemonSyncPreflightError> {
    if !runtime.sync.is_enabled() && !runtime.inbound.enabled {
        return Ok(None);
    }

    let Some(data_dir) = runtime.maybe_data_dir.as_ref() else {
        return Ok(None);
    };

    FjallNodeStore::open(data_dir).map(Some).map_err(|error| {
        DaemonSyncPreflightError::new(format!(
            "open-bitcoind runtime failed to open durable store at \"{}\": {error}",
            data_dir.display()
        ))
    })
}

#[cfg(test)]
async fn start_inbound_listener_for_runtime(runtime: &RuntimeConfig) -> InboundDaemonListener {
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_runtime_config(runtime),
    ));
    start_inbound_listener_for_runtime_with_context(runtime, context).await
}

async fn start_inbound_listener_for_runtime_with_context(
    runtime: &RuntimeConfig,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
) -> InboundDaemonListener {
    let activation = activate_inbound_listener(&runtime.inbound).await;
    let mut state = activation.state();
    let mut preflight_reason = activation.preflight_reason();
    let bound_endpoints = activation
        .bound_endpoints()
        .iter()
        .map(|endpoint| endpoint.bound_endpoint.clone())
        .collect::<Vec<_>>();
    let mut diagnostics = activation.diagnostics().to_vec();
    let authority_available = {
        let mut context = context.lock().await;
        context
            .set_inbound_listener_evidence(activation.evidence().clone())
            .is_ok()
    };
    if !authority_available {
        state = InboundListenerState::Blocked;
        preflight_reason = InboundPreflightReason::BindUnavailable;
        diagnostics.push(InboundPreflightDiagnostic {
            reason: InboundPreflightReason::BindUnavailable,
            maybe_endpoint: None,
            field: "authoritative_network",
            message: "authoritative network state is unavailable".to_string(),
            next_action: "restart open-bitcoind and inspect runtime health".to_string(),
        });
    }
    let maybe_worker = if state == InboundListenerState::Listening && authority_available {
        start_inbound_accept_loop(activation, context)
    } else {
        None
    };

    InboundDaemonListener {
        state,
        preflight_reason,
        bound_endpoints,
        diagnostics,
        maybe_worker,
    }
}

fn inbound_listener_startup_message(listener: &InboundDaemonListener) -> String {
    let bound_endpoint = listener
        .bound_endpoints
        .first()
        .cloned()
        .unwrap_or_else(|| "unavailable".to_string());
    let next_action = listener
        .diagnostics
        .first()
        .map(|diagnostic| diagnostic.next_action.as_str())
        .unwrap_or("no listener action needed");
    format!(
        "open-bitcoind inbound listener startup: inbound_listener_state={} inbound_preflight_reason={} bound_endpoint={} admission_reject_reason=unavailable; opt-in inbound listener/admission {}; next_action=\"{}\"; deferred network participation remains out of scope.",
        listener.state.as_str(),
        listener.preflight_reason.as_str(),
        bound_endpoint,
        inbound_listener_state_description(listener.state),
        next_action
    )
}

fn inbound_listener_state_description(state: InboundListenerState) -> &'static str {
    match state {
        InboundListenerState::Disabled => "is disabled by configuration",
        InboundListenerState::Blocked => "is blocked before socket serving",
        InboundListenerState::Listening => "is active on configured endpoints",
    }
}

fn start_daemon_sync_worker(
    runtime: &RuntimeConfig,
    shared_context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    maybe_sync_runtime: Option<DurableSyncRuntime>,
) -> Result<Option<DaemonSyncWorker>, DaemonSyncPreflightError> {
    if !runtime.sync.is_enabled() {
        return Ok(None);
    }

    let Some(mut sync_runtime) = maybe_sync_runtime else {
        return Err(DaemonSyncPreflightError::new(
            "open-bitcoind daemon sync requires the authoritative durable sync runtime",
        ));
    };
    let sync_config = runtime.sync.runtime.clone();
    let store = sync_runtime.store().clone();
    let control = DaemonSyncControl::store_backed(store.clone(), sync_config.persist_mode);
    let metrics_store = store.clone();
    sync_runtime.set_inbound_metric_status_provider(move || {
        let Ok(context) = shared_context.try_lock() else {
            return inbound_status_unavailable();
        };
        context.current_inbound_status()
    });
    seed_initial_sync_state(&sync_runtime)?;
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();

    Ok(Some(DaemonSyncWorker {
        join_handle: thread::spawn(move || {
            daemon_sync_worker_with_transport(sync_runtime, TcpPeerTransport, shutdown_receiver)
        }),
        shutdown_sender,
        control,
        metrics_store,
    }))
}

fn daemon_sync_worker_with_transport<T: open_bitcoin_node::SyncTransport>(
    mut sync_runtime: DurableSyncRuntime,
    mut transport: T,
    shutdown_receiver: mpsc::Receiver<()>,
) {
    let policy = DaemonSyncLoopPolicy::from_runtime(&sync_runtime);

    loop {
        if daemon_sync_shutdown_requested(&shutdown_receiver) {
            let _ = run_daemon_sync_loop_cycle(
                &mut sync_runtime,
                policy,
                current_timestamp_unix_seconds(),
                true,
                |runtime, _timestamp| Ok(runtime.snapshot_summary()),
            );
            break;
        }

        let mut shutdown_latched = false;
        let decision = run_daemon_sync_loop_cycle(
            &mut sync_runtime,
            policy,
            current_timestamp_unix_seconds(),
            false,
            |runtime, timestamp| {
                let mut clock = current_timestamp_unix_seconds;
                let mut should_cancel = || {
                    shutdown_latched =
                        shutdown_latched || daemon_sync_shutdown_requested(&shutdown_receiver);
                    shutdown_latched
                };
                runtime.sync_until_idle_with_clock_and_cancel(
                    &mut transport,
                    timestamp,
                    &mut clock,
                    &mut should_cancel,
                )
            },
        );
        if shutdown_latched {
            let _ = run_daemon_sync_loop_cycle(
                &mut sync_runtime,
                policy,
                current_timestamp_unix_seconds(),
                true,
                |runtime, _timestamp| Ok(runtime.snapshot_summary()),
            );
            break;
        }
        if matches!(decision, DaemonSyncLoopDecision::Stopped) {
            break;
        }
        if daemon_sync_wait_or_shutdown(&shutdown_receiver, decision.sleep_duration()) {
            let _ = run_daemon_sync_loop_cycle(
                &mut sync_runtime,
                policy,
                current_timestamp_unix_seconds(),
                true,
                |runtime, _timestamp| Ok(runtime.snapshot_summary()),
            );
            break;
        }
    }
}

fn run_daemon_sync_loop_cycle<F>(
    sync_runtime: &mut DurableSyncRuntime,
    policy: DaemonSyncLoopPolicy,
    timestamp: i64,
    shutdown_requested: bool,
    run_sync_cycle: F,
) -> DaemonSyncLoopDecision
where
    F: FnOnce(&mut DurableSyncRuntime, i64) -> Result<SyncRunSummary, SyncRuntimeError>,
{
    if shutdown_requested {
        persist_daemon_loop_stop(
            sync_runtime,
            SyncLifecycleState::Stopped,
            SyncStopReason::ShutdownRequested,
            timestamp,
        );
        return DaemonSyncLoopDecision::Stopped;
    }

    match sync_runtime.load_sync_control() {
        Ok(control) if control.paused => {
            persist_daemon_loop_stop(
                sync_runtime,
                SyncLifecycleState::Paused,
                SyncStopReason::OperatorPaused,
                timestamp,
            );
            return DaemonSyncLoopDecision::Paused(policy.sleep_duration);
        }
        Ok(_) => {}
        Err(error) => {
            persist_daemon_loop_failure(sync_runtime, &error, timestamp);
            return DaemonSyncLoopDecision::Failed(policy.sleep_duration);
        }
    }

    let lifecycle = match sync_runtime.store().load_recovery_marker() {
        Ok(Some(_)) => SyncLifecycleState::Recovering,
        Ok(None) => SyncLifecycleState::Active,
        Err(error) => {
            let error = SyncRuntimeError::from(error);
            persist_daemon_loop_failure(sync_runtime, &error, timestamp);
            return DaemonSyncLoopDecision::Failed(policy.sleep_duration);
        }
    };
    match sync_runtime.durable_sync_state(lifecycle, None, timestamp) {
        Ok(state) => {
            if let Err(error) = sync_runtime.persist_durable_sync_state(state) {
                persist_daemon_loop_failure(sync_runtime, &error, timestamp);
                return DaemonSyncLoopDecision::Failed(policy.sleep_duration);
            }
        }
        Err(error) => {
            persist_daemon_loop_failure(sync_runtime, &error, timestamp);
            return DaemonSyncLoopDecision::Failed(policy.sleep_duration);
        }
    }

    match run_sync_cycle(sync_runtime, timestamp) {
        Ok(summary) => {
            let maybe_last_error = summary.latest_error_message();
            match sync_runtime.durable_sync_state_for_summary(
                &summary,
                SyncLifecycleState::Active,
                maybe_last_error,
                timestamp,
            ) {
                Ok(state) => {
                    if let Err(error) = sync_runtime.persist_durable_sync_state(state) {
                        persist_daemon_loop_failure(sync_runtime, &error, timestamp);
                        return DaemonSyncLoopDecision::Failed(policy.sleep_duration);
                    }
                }
                Err(error) => {
                    persist_daemon_loop_failure(sync_runtime, &error, timestamp);
                    return DaemonSyncLoopDecision::Failed(policy.sleep_duration);
                }
            }
            DaemonSyncLoopDecision::RetryAfter(policy.sleep_duration)
        }
        Err(error) => {
            eprintln!("open-bitcoind daemon sync cycle failed: {error}");
            persist_daemon_loop_failure(sync_runtime, &error, timestamp);
            DaemonSyncLoopDecision::Failed(policy.sleep_duration)
        }
    }
}

fn persist_daemon_loop_stop(
    sync_runtime: &DurableSyncRuntime,
    lifecycle: SyncLifecycleState,
    stop_reason: SyncStopReason,
    timestamp: i64,
) {
    let mut summary = sync_runtime.snapshot_summary();
    summary.maybe_stop_reason = Some(stop_reason);
    summary.health_signals.push(stop_reason.health_signal());
    let maybe_last_error = Some(stop_reason.message());
    match sync_runtime.durable_sync_state_for_summary(
        &summary,
        lifecycle,
        maybe_last_error,
        timestamp,
    ) {
        Ok(state) => {
            if let Err(error) = sync_runtime.persist_durable_sync_state(state) {
                eprintln!("open-bitcoind daemon sync stop persistence failed: {error}");
            }
        }
        Err(error) => eprintln!("open-bitcoind daemon sync stop state failed: {error}"),
    }
}

fn persist_daemon_loop_failure(
    sync_runtime: &DurableSyncRuntime,
    error: &SyncRuntimeError,
    timestamp: i64,
) {
    if let Ok(state) = sync_runtime.durable_sync_state(
        SyncLifecycleState::Failed,
        Some(error.to_string()),
        timestamp,
    ) {
        let _ = sync_runtime.persist_durable_sync_state(state);
    }
}

fn daemon_sync_shutdown_requested(receiver: &mpsc::Receiver<()>) -> bool {
    !matches!(receiver.try_recv(), Err(mpsc::TryRecvError::Empty))
}

fn daemon_sync_wait_or_shutdown(receiver: &mpsc::Receiver<()>, duration: Duration) -> bool {
    match receiver.recv_timeout(duration) {
        Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => true,
        Err(mpsc::RecvTimeoutError::Timeout) => false,
    }
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        eprintln!("open-bitcoind shutdown signal listener failed: {error}");
    }
}

fn current_timestamp_unix_seconds() -> i64 {
    let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return 0;
    };
    i64::try_from(duration.as_secs()).unwrap_or(i64::MAX)
}

#[cfg(test)]
#[path = "open_bitcoind/tests.rs"]
mod tests;
