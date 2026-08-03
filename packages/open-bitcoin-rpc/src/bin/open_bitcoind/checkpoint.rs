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
    FjallNodeStore, ManagedNetworkHandle, PersistMode, StorageError,
    network::{MempoolCheckpointCoordinator, MempoolCheckpointError, MempoolCheckpointOutcome},
};

use super::current_timestamp_unix_seconds;

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
    store: FjallNodeStore,
}

impl MempoolCheckpointWorker {
    pub(super) fn shutdown_and_mark_clean(self) -> Result<(), DaemonCheckpointError> {
        self.shutdown_sender
            .send(())
            .map_err(|_| DaemonCheckpointError::WorkerSignal)?;
        let join_handle = self.join_handle;
        let store = self.store;
        settle_and_mark_clean(
            move || {
                join_handle
                    .join()
                    .map_err(|_| DaemonCheckpointError::WorkerJoin)?
                    .map_err(|_| DaemonCheckpointError::ShutdownCheckpoint)?;
                Ok(())
            },
            move || store.mark_clean_shutdown(PersistMode::Sync),
        )
    }
}

pub(super) fn start_mempool_checkpoint_worker(
    handle: ManagedNetworkHandle,
    maybe_store: Option<FjallNodeStore>,
) -> Option<MempoolCheckpointWorker> {
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
        store,
    })
}

pub(super) fn checkpoint_worker_loop<Wait, Now>(
    handle: ManagedNetworkHandle,
    store: FjallNodeStore,
    mut wait: Wait,
    mut now: Now,
) -> Result<MempoolCheckpointOutcome, MempoolCheckpointError>
where
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
