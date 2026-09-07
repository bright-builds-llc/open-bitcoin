// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

//! Private daemon ownership for Periodic and Always coins flushes.

use core::fmt;
use std::{sync::mpsc, thread, time::Duration};

use open_bitcoin_node::{
    FjallNodeStore, ManagedNetworkAuthorityError, ManagedNetworkHandle,
    chainstate::{FlushExecution, probe_disk_free_bytes},
    core::chainstate::{FlushMode, FlushPolicyTime},
};

use super::checkpoint::{CheckpointWait, DaemonCheckpointError, settle_and_mark_clean};
use super::current_timestamp_unix_seconds;

pub(super) const PERIODIC_WRITE_MIN_SECS: u64 = 50 * 60;
pub(super) const PERIODIC_WRITE_MAX_SECS: u64 = 70 * 60;

const TICK_SECS: u64 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CoinsFlushError {
    Authority,
}

impl fmt::Display for CoinsFlushError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("open-bitcoind coins flush failed: authority")
    }
}

impl std::error::Error for CoinsFlushError {}

impl From<ManagedNetworkAuthorityError> for CoinsFlushError {
    fn from(_error: ManagedNetworkAuthorityError) -> Self {
        Self::Authority
    }
}

pub(super) struct CoinsFlushWorker {
    shutdown_sender: mpsc::Sender<()>,
    join_handle: thread::JoinHandle<Result<(), CoinsFlushError>>,
}

impl CoinsFlushWorker {
    pub(super) fn shutdown_always(self) -> Result<(), DaemonCheckpointError> {
        self.shutdown_sender
            .send(())
            .map_err(|_| DaemonCheckpointError::WorkerSignal)?;
        let join_handle = self.join_handle;
        settle_and_mark_clean(
            move || {
                join_handle
                    .join()
                    .map_err(|_| DaemonCheckpointError::WorkerJoin)?
                    .map_err(|_| DaemonCheckpointError::ShutdownCheckpoint)?;
                Ok(())
            },
            || Ok(()),
        )
    }
}

pub(super) fn start_coins_flush_worker<S, V>(
    handle: ManagedNetworkHandle<S, V>,
    maybe_store: Option<FjallNodeStore>,
) -> Option<CoinsFlushWorker>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let store = maybe_store?;
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();
    let join_handle = thread::spawn(move || {
        let _store = store;
        coins_flush_worker_loop(handle, move |duration| {
            match shutdown_receiver.recv_timeout(duration) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => CheckpointWait::Shutdown,
                Err(mpsc::RecvTimeoutError::Timeout) => CheckpointWait::Elapsed,
            }
        })
    });

    Some(CoinsFlushWorker {
        shutdown_sender,
        join_handle,
    })
}

pub(super) fn resample_periodic_next_write(now: u64) -> Result<FlushPolicyTime, std::io::Error> {
    let mut bytes = [0_u8; 8];
    getrandom::fill(&mut bytes).map_err(|error| std::io::Error::other(error.to_string()))?;
    let span = PERIODIC_WRITE_MAX_SECS - PERIODIC_WRITE_MIN_SECS;
    let offset = u64::from_le_bytes(bytes) % (span + 1);
    Ok(FlushPolicyTime::from_unix_seconds(
        now.saturating_add(PERIODIC_WRITE_MIN_SECS + offset),
    ))
}

fn coins_flush_worker_loop<S, V, Wait>(
    handle: ManagedNetworkHandle<S, V>,
    mut wait: Wait,
) -> Result<(), CoinsFlushError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
    Wait: FnMut(Duration) -> CheckpointWait,
{
    loop {
        match wait(Duration::from_secs(TICK_SECS)) {
            CheckpointWait::Elapsed => drive_periodic(&handle),
            CheckpointWait::Shutdown => return drive_always(&handle),
        }
    }
}

fn drive_periodic<S, V>(handle: &ManagedNetworkHandle<S, V>)
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    match flush_now(handle, FlushMode::Periodic) {
        Ok(execution) => resample_after_periodic_write(handle, execution),
        Err(error) => eprintln!("open-bitcoind periodic coins flush failed: {error}"),
    }
}

fn drive_always<S, V>(handle: &ManagedNetworkHandle<S, V>) -> Result<(), CoinsFlushError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    flush_now(handle, FlushMode::Always).map(|_| ())
}

fn flush_now<S, V>(
    handle: &ManagedNetworkHandle<S, V>,
    mode: FlushMode,
) -> Result<FlushExecution, CoinsFlushError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let now = current_flush_policy_time();
    let disk_free_bytes = probe_disk_free_bytes(std::path::Path::new("."));
    handle
        .flush_coins(mode, now, disk_free_bytes)
        .map_err(CoinsFlushError::from)
}

fn resample_after_periodic_write<S, V>(
    handle: &ManagedNetworkHandle<S, V>,
    execution: FlushExecution,
) where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    if !execution.wrote_coins {
        return;
    }
    let now = current_flush_policy_time().unix_seconds();
    let Ok(next_write) = resample_periodic_next_write(now) else {
        return;
    };
    let _ = handle.set_coins_next_write(next_write);
}

fn current_flush_policy_time() -> FlushPolicyTime {
    FlushPolicyTime::from_unix_seconds(u64::try_from(current_timestamp_unix_seconds()).unwrap_or(0))
}

#[cfg(test)]
#[path = "coins_flush/tests.rs"]
mod tests;
