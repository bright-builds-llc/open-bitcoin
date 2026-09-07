// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/test/functional/mempool_unbroadcast.py

//! Shutdown-aware open-bitcoind timer for initial-broadcast retry.
//!
//! This worker is not a public/default relay loop. Relay activation and peer
//! eligibility still gate emissions, and a successful tick does not claim
//! guaranteed propagation.

use core::fmt;
use std::{sync::mpsc, thread, time::Duration};

use open_bitcoin_network::{RetryDecisionContext, RetryJitterSeconds};
use open_bitcoin_node::ManagedNetworkHandle;

use super::current_timestamp_unix_seconds;

/// Initial wait used only before any successful jitter sample.
///
/// This is not a public cadence claim. After a successful tick, the next wait
/// comes from the reminted due time.
const INITIAL_JITTER_UNAVAILABLE_WAIT: Duration = Duration::from_secs(600);
const MINIMUM_RETRY_WAIT: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RetryWait {
    Elapsed,
    Shutdown,
    JitterUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DaemonRetryError {
    WorkerSignal,
    WorkerJoin,
}

impl fmt::Display for DaemonRetryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = match self {
            Self::WorkerSignal => "worker-signal",
            Self::WorkerJoin => "worker-join",
        };
        write!(formatter, "open-bitcoind retry shutdown failed: {class}")
    }
}

impl std::error::Error for DaemonRetryError {}

pub(super) struct InitialBroadcastRetryWorker {
    shutdown_sender: mpsc::Sender<()>,
    join_handle: thread::JoinHandle<Result<(), DaemonRetryError>>,
}

impl InitialBroadcastRetryWorker {
    pub(super) fn shutdown(self) -> Result<(), DaemonRetryError> {
        self.shutdown_sender
            .send(())
            .map_err(|_| DaemonRetryError::WorkerSignal)?;
        self.join_handle
            .join()
            .map_err(|_| DaemonRetryError::WorkerJoin)?
    }
}

/// Starts the receive-independent initial-broadcast retry timer.
///
/// This is initial-broadcast retry for the local unbroadcast set; it is not
/// public/default relay and does not guarantee propagation.
pub(super) fn start_initial_broadcast_retry_worker<S, V>(
    handle: ManagedNetworkHandle<S, V>,
) -> InitialBroadcastRetryWorker
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();
    let join_handle = thread::spawn(move || {
        retry_worker_loop(
            handle,
            move |duration| match shutdown_receiver.recv_timeout(duration) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => RetryWait::Shutdown,
                Err(mpsc::RecvTimeoutError::Timeout) => RetryWait::Elapsed,
            },
            current_timestamp_unix_seconds,
            sample_production_jitter,
        )
    });

    InitialBroadcastRetryWorker {
        shutdown_sender,
        join_handle,
    }
}

pub(super) fn retry_worker_loop<S, V, Wait, Now, Jitter>(
    handle: ManagedNetworkHandle<S, V>,
    mut wait: Wait,
    mut now: Now,
    mut jitter: Jitter,
) -> Result<(), DaemonRetryError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
    Wait: FnMut(Duration) -> RetryWait,
    Now: FnMut() -> i64,
    Jitter: FnMut() -> Option<RetryJitterSeconds>,
{
    let mut next_wait = INITIAL_JITTER_UNAVAILABLE_WAIT;
    loop {
        match wait(next_wait) {
            RetryWait::Shutdown => return Ok(()),
            RetryWait::Elapsed | RetryWait::JitterUnavailable => {}
        }

        let observed_at_unix_seconds = now();
        let Some(sampled_jitter) = jitter() else {
            match jitter_unavailable() {
                RetryWait::Shutdown => return Ok(()),
                RetryWait::Elapsed | RetryWait::JitterUnavailable => continue,
            }
        };
        let context = RetryDecisionContext::new(observed_at_unix_seconds, sampled_jitter);
        match handle.maintenance_tick(context) {
            Ok(outcome) => {
                for emission in outcome.emissions {
                    let capability = emission.into_parts().2;
                    let _ignored = handle.abort_peer_emission(capability);
                }
                next_wait = next_wait_duration(
                    observed_at_unix_seconds,
                    outcome.maybe_next_due_unix_seconds,
                );
            }
            Err(error) => {
                eprintln!("open-bitcoind initial-broadcast retry tick failed: {error}");
            }
        }
    }
}

const fn jitter_unavailable() -> RetryWait {
    RetryWait::JitterUnavailable
}

fn next_wait_duration(now_unix_seconds: i64, maybe_next_due_unix_seconds: Option<i64>) -> Duration {
    let Some(due_at) = maybe_next_due_unix_seconds else {
        return MINIMUM_RETRY_WAIT;
    };
    let delta_seconds = due_at.saturating_sub(now_unix_seconds).max(1);
    u64::try_from(delta_seconds)
        .map(Duration::from_secs)
        .unwrap_or(MINIMUM_RETRY_WAIT)
}

fn sample_production_jitter() -> Option<RetryJitterSeconds> {
    let mut bytes = [0_u8; 8];
    getrandom::fill(&mut bytes).ok()?;
    let raw = u64::from_le_bytes(bytes);
    RetryJitterSeconds::new(raw % 301).ok()
}
