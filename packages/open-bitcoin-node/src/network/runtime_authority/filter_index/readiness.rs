// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

//! Finite captured-frontier waits; only the ordinary owner earns completion.

use std::{
    collections::BTreeMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

use super::query::{BasicFilterQueryError, BasicFilterReadFrontier, ReadRequest};

/// Maximum outstanding BASIC read barriers per shared authority.
pub const MAX_BASIC_FILTER_WAITERS: usize = 64;

/// Typed resource, lifecycle and owner failures; none means indexing absence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicFilterReadFailure {
    Capacity,
    CounterExhausted,
    Invalidated,
    OwnerFailed,
    OwnerStopped,
    AuthorityUnavailable,
    AlreadyCompleted,
}

impl std::fmt::Display for BasicFilterReadFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "BASIC read readiness: {self:?}")
    }
}
impl std::error::Error for BasicFilterReadFailure {}

/// A notification proof, never filter data or permission to skip a final read.
#[derive(Debug)]
pub struct BasicFilterReadCompletion {
    pub(super) frontier: BasicFilterReadFrontier,
    pub(super) request: ReadRequest,
}

#[derive(Debug)]
struct Waiter {
    frontier: BasicFilterReadFrontier,
    maybe_result: Option<Result<(), BasicFilterReadFailure>>,
    maybe_waker: Option<Waker>,
}

#[derive(Debug, Default)]
struct Registry {
    next_id: u64,
    stopped: bool,
    waiters: BTreeMap<u64, Waiter>,
}

/// Handles own this object; barriers own only its registry. Last-handle drop
/// therefore settles outstanding waits even when a caller retains a barrier.
#[derive(Debug, Default)]
pub(in crate::network) struct ReadinessOwner {
    registry: Arc<Mutex<Registry>>,
}

impl ReadinessOwner {
    pub(super) fn register(
        &self,
        frontier: BasicFilterReadFrontier,
        request: ReadRequest,
        maybe_result: Option<Result<(), BasicFilterReadFailure>>,
    ) -> Result<BasicFilterReadBarrier, BasicFilterReadFailure> {
        let mut registry = self
            .registry
            .lock()
            .map_err(|_| BasicFilterReadFailure::AuthorityUnavailable)?;
        if registry.stopped {
            return Err(BasicFilterReadFailure::OwnerStopped);
        }
        if registry.waiters.len() >= MAX_BASIC_FILTER_WAITERS {
            return Err(BasicFilterReadFailure::Capacity);
        }
        let id = registry
            .next_id
            .checked_add(1)
            .ok_or(BasicFilterReadFailure::CounterExhausted)?;
        registry.next_id = id;
        registry.waiters.insert(
            id,
            Waiter {
                frontier,
                maybe_result,
                maybe_waker: None,
            },
        );
        Ok(BasicFilterReadBarrier {
            frontier,
            request,
            id,
            registry: Arc::clone(&self.registry),
            finished: false,
        })
    }

    /// Called while authority is held; returned wakers MUST run after release.
    pub(in crate::network) fn collect(
        &self,
        mut inspect: impl FnMut(BasicFilterReadFrontier) -> Option<Result<(), BasicFilterReadFailure>>,
    ) -> Vec<Waker> {
        // Poll/cancel contend only with bounded in-memory operations. Inspect
        // publication outside this mutex, while still holding the authority.
        let (frontiers, initially_poisoned) = {
            let (registry, poisoned) = match self.registry.lock() {
                Ok(registry) => (registry, false),
                Err(poison) => (poison.into_inner(), true),
            };
            let frontiers = registry
                .waiters
                .iter()
                .filter(|(_, waiter)| waiter.maybe_result.is_none())
                .map(|(id, waiter)| (*id, waiter.frontier))
                .collect::<Vec<_>>();
            (frontiers, poisoned)
        };
        let results = frontiers
            .into_iter()
            .map(|(id, frontier)| (id, inspect(frontier)))
            .collect::<Vec<_>>();
        let (mut registry, poisoned) = match self.registry.lock() {
            Ok(registry) => (registry, initially_poisoned),
            Err(poison) => (poison.into_inner(), true),
        };
        let mut wakes = Vec::new();
        for (id, maybe_result) in results {
            let Some(waiter) = registry.waiters.get_mut(&id) else {
                continue;
            };
            if waiter.maybe_result.is_some() {
                continue;
            }
            waiter.maybe_result = if poisoned {
                Some(Err(BasicFilterReadFailure::AuthorityUnavailable))
            } else {
                maybe_result
            };
            if waiter.maybe_result.is_some()
                && let Some(waker) = waiter.maybe_waker.take()
            {
                wakes.push(waker);
            }
        }
        wakes
    }

    pub(in crate::network) fn stop(&self) -> Vec<Waker> {
        self.registry
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .stopped = true;
        self.collect(|_| Some(Err(BasicFilterReadFailure::OwnerStopped)))
    }

    pub(super) fn is_stopped(&self) -> bool {
        self.registry
            .lock()
            .map_or(true, |registry| registry.stopped)
    }
}

impl Drop for ReadinessOwner {
    fn drop(&mut self) {
        wake_all(self.stop());
    }
}

pub(in crate::network) fn wake_all(wakers: Vec<Waker>) {
    for waker in wakers {
        waker.wake();
    }
}

/// Registered, cancellation-safe std Future over one immutable accepted target.
/// Dropping it returns its slot. It never drives indexing or blocks for progress.
#[derive(Debug)]
pub struct BasicFilterReadBarrier {
    frontier: BasicFilterReadFrontier,
    request: ReadRequest,
    id: u64,
    registry: Arc<Mutex<Registry>>,
    finished: bool,
}

impl PartialEq for BasicFilterReadBarrier {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && Arc::ptr_eq(&self.registry, &other.registry)
    }
}
impl Eq for BasicFilterReadBarrier {}

impl BasicFilterReadBarrier {
    /// Inspect the immutable captured position, never a global accepted sequence.
    pub const fn frontier(&self) -> BasicFilterReadFrontier {
        self.frontier
    }
    /// Inspect the captured branch-local height.
    pub const fn accepted_height(&self) -> u32 {
        self.frontier.accepted_height()
    }
    /// Inspect the captured hash.
    pub const fn accepted_hash(&self) -> open_bitcoin_core::primitives::BlockHash {
        self.frontier.accepted_hash()
    }
}

impl Future for BasicFilterReadBarrier {
    type Output = Result<BasicFilterReadCompletion, BasicFilterQueryError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.finished {
            return Poll::Ready(Err(BasicFilterQueryError::Readiness(
                BasicFilterReadFailure::AlreadyCompleted,
            )));
        }
        // RawWaker clone/drop implementations may reenter the authority. Never
        // invoke those callbacks while the registry or authority is held.
        let new_waker = context.waker().clone();
        let (maybe_result, maybe_old_waker) = {
            let mut registry = match this.registry.lock() {
                Ok(registry) => registry,
                Err(_) => {
                    return Poll::Ready(Err(BasicFilterQueryError::Readiness(
                        BasicFilterReadFailure::AuthorityUnavailable,
                    )));
                }
            };
            let Some(waiter) = registry.waiters.get_mut(&this.id) else {
                return Poll::Ready(Err(BasicFilterQueryError::Readiness(
                    BasicFilterReadFailure::AuthorityUnavailable,
                )));
            };
            match waiter.maybe_result {
                Some(result) => {
                    let Some(waiter) = registry.waiters.remove(&this.id) else {
                        return Poll::Ready(Err(BasicFilterQueryError::Readiness(
                            BasicFilterReadFailure::AuthorityUnavailable,
                        )));
                    };
                    (Some(result), waiter.maybe_waker)
                }
                None => (None, waiter.maybe_waker.replace(new_waker)),
            }
        };
        drop(maybe_old_waker);
        let Some(result) = maybe_result else {
            return Poll::Pending;
        };
        this.finished = true;
        Poll::Ready(
            result
                .map(|()| BasicFilterReadCompletion {
                    frontier: this.frontier,
                    request: this.request,
                })
                .map_err(BasicFilterQueryError::Readiness),
        )
    }
}

impl Drop for BasicFilterReadBarrier {
    fn drop(&mut self) {
        let maybe_waiter = self
            .registry
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .waiters
            .remove(&self.id);
        drop(maybe_waiter);
    }
}

mod owner;
pub(super) use owner::maybe_frontier_result;
