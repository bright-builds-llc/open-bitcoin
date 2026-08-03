// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

//! Single-flight durable mempool checkpoint orchestration.

use core::fmt;
use std::sync::Mutex;

use open_bitcoin_mempool::PolicyTime;

use crate::storage::{FjallNodeStore, fjall_store::SnapshotWriteExecutionError};

use super::{
    CheckpointTrigger, EffectCompletion, ManagedNetworkAuthorityError, ManagedNetworkHandle,
    PreparedSnapshotWrite, SnapshotWriteReceipt,
};

const INTERNAL_EVIDENCE_INTERVAL_SECONDS: u64 = 1;
const MAX_WRITES_PER_CALL: u8 = 2;

#[derive(Debug)]
enum CheckpointCoordinatorState {
    Idle,
    Persisting,
    AchievedAwaitingCompletion(SnapshotWriteReceipt),
}

enum FlightClaim {
    Coalesced,
    Claimed(Option<SnapshotWriteReceipt>),
}

/// One bounded result from driving the checkpoint state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolCheckpointOutcome {
    /// No dirty generation required a periodic checkpoint.
    SkippedClean,
    /// Another caller already owns the active flight.
    Coalesced,
    /// One flight completed, including at most one immediate follow-up write.
    Completed {
        writes_started: u8,
        maybe_last_completion: Option<EffectCompletion>,
    },
}

/// Typed failure while driving one mempool checkpoint flight.
#[derive(Debug)]
pub enum MempoolCheckpointError {
    CoordinatorPoisoned,
    Authority(ManagedNetworkAuthorityError),
    Execution(SnapshotWriteExecutionError),
    CompletionDispatch(ManagedNetworkAuthorityError),
    ShutdownNotCurrent {
        current_generation: u64,
        maybe_last_durable_generation: Option<u64>,
    },
}

impl fmt::Display for MempoolCheckpointError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CoordinatorPoisoned => {
                formatter.write_str("mempool checkpoint coordinator is unavailable")
            }
            Self::Authority(error) => error.fmt(formatter),
            Self::Execution(error) => error.fmt(formatter),
            Self::CompletionDispatch(error) => {
                write!(formatter, "checkpoint completion dispatch failed: {error}")
            }
            Self::ShutdownNotCurrent {
                current_generation,
                maybe_last_durable_generation,
            } => write!(
                formatter,
                "shutdown checkpoint is not current: durable generation {maybe_last_durable_generation:?}, current generation {current_generation}"
            ),
        }
    }
}

impl std::error::Error for MempoolCheckpointError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Authority(error) | Self::CompletionDispatch(error) => Some(error),
            Self::Execution(error) => Some(error),
            Self::CoordinatorPoisoned | Self::ShutdownNotCurrent { .. } => None,
        }
    }
}

/// Imperative-shell owner for one coalesced mempool checkpoint flight.
#[derive(Debug)]
pub struct MempoolCheckpointCoordinator {
    state: Mutex<CheckpointCoordinatorState>,
}

impl Default for MempoolCheckpointCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl MempoolCheckpointCoordinator {
    pub const fn new() -> Self {
        Self {
            state: Mutex::new(CheckpointCoordinatorState::Idle),
        }
    }

    /// Drives a periodic checkpoint, skipping clean generations.
    pub fn periodic_tick<Now>(
        &self,
        handle: &ManagedNetworkHandle,
        store: &FjallNodeStore,
        mut now: Now,
    ) -> Result<MempoolCheckpointOutcome, MempoolCheckpointError>
    where
        Now: FnMut() -> PolicyTime,
    {
        self.run_with(
            handle,
            CheckpointTrigger::Periodic,
            &mut now,
            &mut |prepared, now| {
                store.execute_prepared_mempool_snapshot_write(handle, prepared, now)
            },
        )
    }

    /// Forces shutdown checkpoint progress after mutation producers have quiesced.
    pub fn settle_shutdown<Now>(
        &self,
        handle: &ManagedNetworkHandle,
        store: &FjallNodeStore,
        mut now: Now,
    ) -> Result<MempoolCheckpointOutcome, MempoolCheckpointError>
    where
        Now: FnMut() -> PolicyTime,
    {
        let outcome = self.run_with(
            handle,
            CheckpointTrigger::Shutdown,
            &mut now,
            &mut |prepared, now| {
                store.execute_prepared_mempool_snapshot_write(handle, prepared, now)
            },
        )?;
        let evidence = handle
            .checkpoint_evidence(now(), INTERNAL_EVIDENCE_INTERVAL_SECONDS)
            .map_err(MempoolCheckpointError::Authority)?;
        if evidence.maybe_last_durable_generation != Some(evidence.current_generation) {
            return Err(MempoolCheckpointError::ShutdownNotCurrent {
                current_generation: evidence.current_generation,
                maybe_last_durable_generation: evidence.maybe_last_durable_generation,
            });
        }
        Ok(outcome)
    }

    fn run_with<Now, Execute>(
        &self,
        handle: &ManagedNetworkHandle,
        trigger: CheckpointTrigger,
        now: &mut Now,
        execute: &mut Execute,
    ) -> Result<MempoolCheckpointOutcome, MempoolCheckpointError>
    where
        Now: FnMut() -> PolicyTime,
        Execute: FnMut(
            PreparedSnapshotWrite,
            &mut Now,
        ) -> Result<SnapshotWriteReceipt, SnapshotWriteExecutionError>,
    {
        let FlightClaim::Claimed(maybe_retained_receipt) = self.claim_flight()? else {
            return Ok(MempoolCheckpointOutcome::Coalesced);
        };
        let mut maybe_last_completion = None;
        let mut writes_started = 0;
        let mut made_progress = false;

        if let Some(receipt) = maybe_retained_receipt {
            maybe_last_completion = Some(self.complete_or_retain(handle, receipt)?);
            made_progress = true;
        }

        loop {
            let evidence = handle
                .checkpoint_evidence(now(), INTERNAL_EVIDENCE_INTERVAL_SECONDS)
                .map_err(|error| {
                    self.release_with_error(MempoolCheckpointError::Authority(error))
                })?;
            let requires_write = evidence.maybe_dirty_generation.is_some()
                || (trigger == CheckpointTrigger::Shutdown
                    && evidence.maybe_last_durable_generation != Some(evidence.current_generation));
            if !requires_write {
                self.release_flight()?;
                return Ok(if made_progress {
                    MempoolCheckpointOutcome::Completed {
                        writes_started,
                        maybe_last_completion,
                    }
                } else {
                    MempoolCheckpointOutcome::SkippedClean
                });
            }
            if writes_started == MAX_WRITES_PER_CALL {
                self.release_flight()?;
                return Ok(MempoolCheckpointOutcome::Completed {
                    writes_started,
                    maybe_last_completion,
                });
            }

            let prepared = handle
                .prepare_mempool_snapshot_write(now(), trigger)
                .map_err(|error| {
                    self.release_with_error(MempoolCheckpointError::Authority(error))
                })?;
            let receipt = execute(prepared, now).map_err(|error| {
                self.release_with_error(MempoolCheckpointError::Execution(error))
            })?;
            writes_started += 1;
            made_progress = true;
            maybe_last_completion = Some(self.complete_or_retain(handle, receipt)?);
        }
    }

    fn claim_flight(&self) -> Result<FlightClaim, MempoolCheckpointError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| MempoolCheckpointError::CoordinatorPoisoned)?;
        let previous = std::mem::replace(&mut *state, CheckpointCoordinatorState::Persisting);
        match previous {
            CheckpointCoordinatorState::Idle => Ok(FlightClaim::Claimed(None)),
            CheckpointCoordinatorState::Persisting => Ok(FlightClaim::Coalesced),
            CheckpointCoordinatorState::AchievedAwaitingCompletion(receipt) => {
                Ok(FlightClaim::Claimed(Some(receipt)))
            }
        }
    }

    fn complete_or_retain(
        &self,
        handle: &ManagedNetworkHandle,
        receipt: SnapshotWriteReceipt,
    ) -> Result<EffectCompletion, MempoolCheckpointError> {
        match handle.complete_snapshot_write(receipt) {
            Ok(completion) => Ok(completion),
            Err(error) => {
                let (source, receipt) = error.into_parts();
                let mut state = self
                    .state
                    .lock()
                    .map_err(|_| MempoolCheckpointError::CoordinatorPoisoned)?;
                *state = CheckpointCoordinatorState::AchievedAwaitingCompletion(receipt);
                Err(MempoolCheckpointError::CompletionDispatch(source))
            }
        }
    }

    fn release_flight(&self) -> Result<(), MempoolCheckpointError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| MempoolCheckpointError::CoordinatorPoisoned)?;
        *state = CheckpointCoordinatorState::Idle;
        Ok(())
    }

    fn release_with_error(&self, error: MempoolCheckpointError) -> MempoolCheckpointError {
        if self.release_flight().is_err() {
            return MempoolCheckpointError::CoordinatorPoisoned;
        }
        error
    }
}

#[cfg(test)]
mod tests;
