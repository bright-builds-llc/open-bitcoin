// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

//! Private daemon ownership for periodic and shutdown mempool checkpoints.

use core::fmt;
use std::{sync::mpsc, thread, time::Duration};

use open_bitcoin_mempool::PolicyTime;
use open_bitcoin_node::{
    FjallNodeStore, ManagedNetworkHandle, StorageError,
    network::{MempoolCheckpointCoordinator, MempoolCheckpointError, MempoolCheckpointOutcome},
};

use super::current_timestamp_unix_seconds;
#[cfg(test)]
use open_bitcoin_node::PersistMode;

pub(super) const MEMPOOL_CHECKPOINT_INTERVAL: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CheckpointWait {
    Elapsed,
    Shutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CheckpointDrive {
    Periodic,
    Shutdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DaemonCheckpointError {
    ProducerJoin,
    WorkerSignal,
    WorkerJoin,
    ShutdownCheckpoint,
    CleanMarker,
}

impl fmt::Display for DaemonCheckpointError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = match self {
            Self::ProducerJoin => "producer-join",
            Self::WorkerSignal => "worker-signal",
            Self::WorkerJoin => "worker-join",
            Self::ShutdownCheckpoint => "shutdown-checkpoint",
            Self::CleanMarker => "clean-marker",
        };
        write!(
            formatter,
            "open-bitcoind checkpoint shutdown failed: {class}"
        )
    }
}

impl std::error::Error for DaemonCheckpointError {}

pub(super) struct MempoolCheckpointWorker {
    shutdown_sender: mpsc::Sender<()>,
    join_handle: thread::JoinHandle<Result<MempoolCheckpointOutcome, MempoolCheckpointError>>,
    #[cfg(test)]
    store: FjallNodeStore,
}

impl MempoolCheckpointWorker {
    pub(super) fn shutdown_settle(self) -> Result<(), DaemonCheckpointError> {
        let signal_result = self
            .shutdown_sender
            .send(())
            .map_err(|_| DaemonCheckpointError::WorkerSignal);
        let join_result = self
            .join_handle
            .join()
            .map_err(|_| DaemonCheckpointError::WorkerJoin)
            .and_then(|outcome| {
                outcome
                    .map(|_| ())
                    .map_err(|_| DaemonCheckpointError::ShutdownCheckpoint)
            });
        if signal_result.is_err()
            && let Err(error) = &join_result
        {
            eprintln!("open-bitcoind checkpoint settlement also failed: {error}");
        }
        signal_result?;
        join_result
    }

    #[cfg(test)]
    pub(super) fn shutdown_and_mark_clean(self) -> Result<(), DaemonCheckpointError> {
        let store = self.store.clone();
        settle_and_mark_clean(
            move || self.shutdown_settle(),
            move || store.mark_clean_shutdown(PersistMode::Sync),
        )
    }
}

pub(super) type ShutdownResult = Result<(), Box<dyn std::error::Error>>;

pub(super) fn settle_daemon_shutdown<Sync, Coins, Retry, Checkpoint, Clean>(
    serve_result: ShutdownResult,
    sync: Sync,
    coins: Coins,
    retry: Retry,
    checkpoint: Checkpoint,
    mark_clean: Clean,
) -> ShutdownResult
where
    Sync: FnOnce() -> ShutdownResult,
    Coins: FnOnce() -> ShutdownResult,
    Retry: FnOnce() -> ShutdownResult,
    Checkpoint: FnOnce() -> ShutdownResult,
    Clean: FnOnce() -> Result<(), StorageError>,
{
    // Evaluate every stop/join before propagating errors. Retry is a producer,
    // so it must quiesce before capturing the final mempool generation.
    let results = [sync(), coins(), retry(), checkpoint(), serve_result];
    let mut maybe_error = None;
    for result in results {
        if let Err(error) = result {
            eprintln!("open-bitcoind shutdown settlement failed: {error}");
            if maybe_error.is_none() {
                maybe_error = Some(error);
            }
        }
    }
    if let Some(error) = maybe_error {
        return Err(error);
    }
    mark_clean().map_err(|_| DaemonCheckpointError::CleanMarker.into())
}

pub(super) fn start_mempool_checkpoint_worker<S, V>(
    handle: ManagedNetworkHandle<S, V>,
    maybe_store: Option<FjallNodeStore>,
) -> Option<MempoolCheckpointWorker>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let store = maybe_store?;
    let worker_store = store.clone();
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();
    let join_handle = thread::spawn(move || {
        checkpoint_worker_loop(
            handle,
            worker_store,
            move |duration| match shutdown_receiver.recv_timeout(duration) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => CheckpointWait::Shutdown,
                Err(mpsc::RecvTimeoutError::Timeout) => CheckpointWait::Elapsed,
            },
            current_policy_time,
        )
    });

    Some(MempoolCheckpointWorker {
        shutdown_sender,
        join_handle,
        #[cfg(test)]
        store,
    })
}

pub(super) fn checkpoint_worker_loop<S, V, Wait, Now>(
    handle: ManagedNetworkHandle<S, V>,
    store: FjallNodeStore,
    mut wait: Wait,
    mut now: Now,
) -> Result<MempoolCheckpointOutcome, MempoolCheckpointError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
    Wait: FnMut(Duration) -> CheckpointWait,
    Now: FnMut() -> PolicyTime,
{
    let coordinator = MempoolCheckpointCoordinator::new();

    checkpoint_worker_loop_with(
        &mut wait,
        move |drive| match drive {
            CheckpointDrive::Periodic => coordinator.periodic_tick(&handle, &store, &mut now),
            CheckpointDrive::Shutdown => coordinator.settle_shutdown(&handle, &store, &mut now),
        },
        |error| {
            eprintln!(
                "open-bitcoind periodic mempool checkpoint failed: {}",
                checkpoint_failure_class(error)
            );
        },
    )
}

pub(super) fn checkpoint_worker_loop_with<Wait, Drive, OnPeriodicError, Outcome, Error>(
    mut wait: Wait,
    mut drive: Drive,
    mut on_periodic_error: OnPeriodicError,
) -> Result<Outcome, Error>
where
    Wait: FnMut(Duration) -> CheckpointWait,
    Drive: FnMut(CheckpointDrive) -> Result<Outcome, Error>,
    OnPeriodicError: FnMut(&Error),
{
    loop {
        match wait(MEMPOOL_CHECKPOINT_INTERVAL) {
            CheckpointWait::Elapsed => {
                if let Err(error) = drive(CheckpointDrive::Periodic) {
                    on_periodic_error(&error);
                }
            }
            CheckpointWait::Shutdown => return drive(CheckpointDrive::Shutdown),
        }
    }
}

#[cfg(test)]
pub(super) fn settle_and_mark_clean<Settle, MarkClean>(
    settle: Settle,
    mark_clean: MarkClean,
) -> Result<(), DaemonCheckpointError>
where
    Settle: FnOnce() -> Result<(), DaemonCheckpointError>,
    MarkClean: FnOnce() -> Result<(), StorageError>,
{
    settle()?;
    mark_clean().map_err(|_| DaemonCheckpointError::CleanMarker)
}

fn current_policy_time() -> PolicyTime {
    PolicyTime::new(current_timestamp_unix_seconds())
}

pub(super) fn checkpoint_failure_class(error: &MempoolCheckpointError) -> &'static str {
    match error {
        MempoolCheckpointError::CoordinatorPoisoned => "coordinator",
        MempoolCheckpointError::Authority(_) => "authority",
        MempoolCheckpointError::Execution(_) => "execution",
        MempoolCheckpointError::CompletionDispatch(_) => "completion-dispatch",
        MempoolCheckpointError::AbortDispatch { .. } => "abort-dispatch",
        MempoolCheckpointError::ShutdownNotCurrent { .. } => "not-current",
    }
}
