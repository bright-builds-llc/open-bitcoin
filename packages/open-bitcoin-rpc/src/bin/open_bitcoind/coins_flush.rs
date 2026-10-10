// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

//! One daemon maintenance owner for bounded BASIC turns and ordinary coins flushes.

use core::fmt;
use std::{sync::mpsc, thread, time::Duration};

use open_bitcoin_node::{
    BasicFilterTurnOutcome, FjallChainstateStore, FjallCoinsView, FjallNodeStore,
    ManagedNetworkAuthorityError, ManagedNetworkHandle, MemoryChainstateStore,
    chainstate::{FlushExecution, probe_disk_free_bytes},
    core::chainstate::{FlushMode, FlushPolicyTime, MemoryCoinsView},
};

use super::checkpoint::{CheckpointWait, DaemonCheckpointError};
use super::current_timestamp_unix_seconds;

pub(super) const PERIODIC_WRITE_MIN_SECS: u64 = 50 * 60;
pub(super) const PERIODIC_WRITE_MAX_SECS: u64 = 70 * 60;

const TICK_SECS: u64 = 1;

pub(super) enum CoinsFlushError {
    Authority,
    BasicFilter(ManagedNetworkAuthorityError),
}

impl fmt::Debug for CoinsFlushError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl fmt::Display for CoinsFlushError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Authority => formatter.write_str("open-bitcoind coins flush failed: authority"),
            Self::BasicFilter(_) => {
                formatter.write_str("open-bitcoind BASIC maintenance failed: index unavailable")
            }
        }
    }
}

impl std::error::Error for CoinsFlushError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BasicFilter(error) => Some(error),
            Self::Authority => None,
        }
    }
}

impl From<ManagedNetworkAuthorityError> for CoinsFlushError {
    fn from(_error: ManagedNetworkAuthorityError) -> Self {
        Self::Authority
    }
}

/// Compile-time adapter: only genuine durable handles can drive the owned index.
pub(super) trait BasicIndexMaintenance {
    fn maybe_drive_index_turn(&self) -> Result<Option<BasicFilterTurnOutcome>, CoinsFlushError>;
}

impl BasicIndexMaintenance for ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView> {
    fn maybe_drive_index_turn(&self) -> Result<Option<BasicFilterTurnOutcome>, CoinsFlushError> {
        self.drive_basic_filter_index_turn()
            .map(Some)
            .map_err(CoinsFlushError::BasicFilter)
    }
}

impl BasicIndexMaintenance for ManagedNetworkHandle<MemoryChainstateStore, MemoryCoinsView> {
    fn maybe_drive_index_turn(&self) -> Result<Option<BasicFilterTurnOutcome>, CoinsFlushError> {
        Ok(None)
    }
}

pub(super) struct CoinsFlushWorker {
    shutdown_sender: mpsc::Sender<()>,
    join_handle: thread::JoinHandle<Result<(), CoinsFlushError>>,
}

impl CoinsFlushWorker {
    pub(super) fn shutdown_always(self) -> Result<(), DaemonCheckpointError> {
        let signal_result = self
            .shutdown_sender
            .send(())
            .map_err(|_| DaemonCheckpointError::WorkerSignal);
        let join_result = self
            .join_handle
            .join()
            .map_err(|_| DaemonCheckpointError::WorkerJoin)
            .and_then(|outcome| outcome.map_err(|_| DaemonCheckpointError::ShutdownCheckpoint));
        if signal_result.is_err()
            && let Err(error) = &join_result
        {
            eprintln!("open-bitcoind coins settlement also failed: {error}");
        }
        signal_result?;
        join_result
    }
}

pub(super) fn start_coins_flush_worker<S, V>(
    handle: ManagedNetworkHandle<S, V>,
    maybe_store: Option<FjallNodeStore>,
) -> Option<CoinsFlushWorker>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
    ManagedNetworkHandle<S, V>: BasicIndexMaintenance,
{
    let store = maybe_store?;
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();
    let join_handle = thread::spawn(move || {
        coins_flush_worker_loop(handle, store, move |duration| {
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
    store: FjallNodeStore,
    mut wait: Wait,
) -> Result<(), CoinsFlushError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
    Wait: FnMut(Duration) -> CheckpointWait,
    ManagedNetworkHandle<S, V>: BasicIndexMaintenance,
{
    let mut maybe_index_error = None;
    loop {
        match wait(Duration::from_secs(TICK_SECS)) {
            CheckpointWait::Elapsed => {
                if let Err(error) = maybe_drive_elapsed(&handle, &store) {
                    eprintln!("{error}");
                    if let Err(stop_error) = handle.stop_basic_filter_readiness() {
                        eprintln!(
                            "open-bitcoind BASIC readiness settlement also failed: {stop_error}"
                        );
                    }
                    // A stopped read owner cannot become healthy on a later tick.
                    if maybe_index_error.is_none() {
                        maybe_index_error = Some(error);
                    }
                }
            }
            CheckpointWait::Shutdown => {
                // No further index turn after the stop event. Always retains the
                // saved Active protection; configured disable owns its release.
                let flush_result = drive_always(&handle, &store);
                let stop_result = handle
                    .stop_basic_filter_readiness()
                    .map_err(CoinsFlushError::from);
                if flush_result.is_err()
                    && let Err(error) = &stop_result
                {
                    eprintln!("open-bitcoind BASIC readiness settlement also failed: {error}");
                }
                if let Some(error) = maybe_index_error {
                    if let Err(flush_error) = &flush_result {
                        eprintln!("open-bitcoind coins settlement also failed: {flush_error}");
                    }
                    if flush_result.is_ok()
                        && let Err(stop_error) = &stop_result
                    {
                        eprintln!(
                            "open-bitcoind BASIC readiness settlement also failed: {stop_error}"
                        );
                    }
                    return Err(error);
                }
                flush_result?;
                return stop_result;
            }
        }
    }
}

fn maybe_drive_elapsed<S, V>(
    handle: &ManagedNetworkHandle<S, V>,
    store: &FjallNodeStore,
) -> Result<Option<BasicFilterTurnOutcome>, CoinsFlushError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
    ManagedNetworkHandle<S, V>: BasicIndexMaintenance,
{
    drive_periodic(handle, store)?;
    handle.maybe_drive_index_turn()
}

fn drive_periodic<S, V>(
    handle: &ManagedNetworkHandle<S, V>,
    store: &FjallNodeStore,
) -> Result<(), CoinsFlushError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let execution = flush_now(handle, store, FlushMode::Periodic)?;
    resample_after_periodic_write(handle, execution)
}

fn drive_always<S, V>(
    handle: &ManagedNetworkHandle<S, V>,
    store: &FjallNodeStore,
) -> Result<(), CoinsFlushError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    flush_now(handle, store, FlushMode::Always).map(|_| ())
}

fn flush_now<S, V>(
    handle: &ManagedNetworkHandle<S, V>,
    store: &FjallNodeStore,
    mode: FlushMode,
) -> Result<FlushExecution, CoinsFlushError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let now = current_flush_policy_time();
    let disk_free_bytes = probe_disk_free_bytes(store.datadir());
    flush_cycle(handle, mode, now, disk_free_bytes).map_err(CoinsFlushError::from)
}

pub(super) fn flush_cycle<S, V>(
    handle: &ManagedNetworkHandle<S, V>,
    mode: FlushMode,
    now: FlushPolicyTime,
    disk_free_bytes: u64,
) -> Result<FlushExecution, ManagedNetworkAuthorityError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    handle.flush_coins(mode, now, disk_free_bytes)
}

fn resample_after_periodic_write<S, V>(
    handle: &ManagedNetworkHandle<S, V>,
    execution: FlushExecution,
) -> Result<(), CoinsFlushError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    if !execution.wrote_coins {
        return Ok(());
    }
    let now = current_flush_policy_time().unix_seconds();
    let next_write = resample_periodic_next_write(now).unwrap_or_else(|_| {
        FlushPolicyTime::from_unix_seconds(now.saturating_add(PERIODIC_WRITE_MIN_SECS))
    });
    handle
        .set_coins_next_write(next_write)
        .map_err(CoinsFlushError::from)
}

fn current_flush_policy_time() -> FlushPolicyTime {
    FlushPolicyTime::from_unix_seconds(u64::try_from(current_timestamp_unix_seconds()).unwrap_or(0))
}

#[cfg(test)]
#[path = "coins_flush/tests.rs"]
mod tests;

#[cfg(test)]
mod phase159_tests {
    use super::*;
    use crate::tests::filter_index::fixtures::{History, next_block, params};
    use open_bitcoin_node::core::consensus::{ScriptVerifyFlags, block_hash};
    use open_bitcoin_node::{BasicFilterQuery, BasicFilterQueryError, BasicFilterReadFailure};
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll, Waker},
    };

    #[test]
    fn phase159_filter_rpc_maintenance_actual_exit_settles_pending() {
        for fail in [false, true] {
            // Arrange
            let history = History::new(2);
            drop(history.seed());
            let config = history.config(Some("-blockfilterindex=1"));
            let store = crate::open_runtime_store(&config)
                .expect("store")
                .expect("durable");
            let opened = crate::open_authoritative_network_runtime(&config, Some(store.clone()))
                .expect("owner")
                .expect_durable();
            let handle = opened.network.clone();
            let next = next_block(history.blocks.last(), 2);
            handle
                .connect_local_block(&next, ScriptVerifyFlags::P2SH, params())
                .expect("genuine accepted lag");
            store
                .save_block(&next, open_bitcoin_node::PersistMode::Sync)
                .expect("ordinary body");
            let BasicFilterQuery::Pending(mut barrier) = handle
                .basic_filter_query(block_hash(&next.header))
                .expect("capture")
            else {
                panic!("pending");
            };
            assert!(
                Pin::new(&mut barrier)
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            if fail {
                handle
                    .set_coins_next_write(FlushPolicyTime::new(u64::MAX))
                    .expect("not due");
                store
                    .coins_view()
                    .batch_write_with_persist_mode(
                        open_bitcoin_node::core::chainstate::CoinsBatch {
                            entries: Default::default(),
                        },
                        Some(
                            open_bitcoin_node::core::primitives::BlockHash::from_byte_array(
                                [99; 32],
                            ),
                        ),
                        open_bitcoin_node::PersistMode::Sync,
                    )
                    .expect("injected invalid durable marker");
            }
            let mut ticks = 0;
            // Act
            let result = coins_flush_worker_loop(handle.clone(), store, |_| {
                ticks += 1;
                if fail && ticks == 1 {
                    CheckpointWait::Elapsed
                } else {
                    CheckpointWait::Shutdown
                }
            });
            // Assert
            assert_eq!(result.is_err(), fail);
            assert!(matches!(
                Pin::new(&mut barrier).poll(&mut Context::from_waker(Waker::noop())),
                Poll::Ready(Err(BasicFilterQueryError::Readiness(
                    BasicFilterReadFailure::OwnerStopped | BasicFilterReadFailure::OwnerFailed
                )))
            ));
        }
    }

    #[test]
    fn phase159_filter_rpc_maintenance_error_diagnostics_are_redacted() {
        // Arrange
        let error = CoinsFlushError::BasicFilter(ManagedNetworkAuthorityError::LifecycleEffect(
            "/private/backend-marker".to_owned(),
        ));
        // Act / Assert
        assert!(!format!("{error:?} {error}").contains("backend-marker"));
        assert!(
            std::error::Error::source(&error).is_some(),
            "internal typed cause retained"
        );
    }
}
