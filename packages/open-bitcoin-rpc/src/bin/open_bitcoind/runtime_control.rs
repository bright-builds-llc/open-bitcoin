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
    sync::mpsc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use open_bitcoin_node::{DurableSyncRuntime, SyncLifecycleState, SyncRuntimeError, SyncStopReason};

pub(super) fn daemon_sync_shutdown_requested(receiver: &mpsc::Receiver<()>) -> bool {
    !matches!(receiver.try_recv(), Err(mpsc::TryRecvError::Empty))
}

pub(super) fn daemon_sync_wait_or_shutdown(
    receiver: &mpsc::Receiver<()>,
    duration: Duration,
) -> bool {
    match receiver.recv_timeout(duration) {
        Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => true,
        Err(mpsc::RecvTimeoutError::Timeout) => false,
    }
}

pub(super) async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        eprintln!("open-bitcoind shutdown signal listener failed: {error}");
    }
}

pub(super) fn persist_daemon_loop_stop(
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

pub(super) fn persist_daemon_loop_failure(
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

pub(super) fn current_timestamp_unix_seconds() -> i64 {
    let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return 0;
    };
    i64::try_from(duration.as_secs()).unwrap_or(i64::MAX)
}
