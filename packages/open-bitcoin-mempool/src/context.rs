// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/mempool_entry.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp

//! Canonical mempool entry metadata and explicit operation inputs.

use crate::{AccountedMempoolMemory, MempoolCapacity};

/// A policy timestamp expressed as signed Unix seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolicyTime(i64);

impl PolicyTime {
    /// Creates a policy timestamp from Unix seconds supplied by an effectful adapter.
    pub const fn new(unix_seconds: i64) -> Self {
        Self(unix_seconds)
    }

    /// Returns the signed Unix-seconds representation.
    pub const fn unix_seconds(self) -> i64 {
        self.0
    }
}

/// Whether an entry's original acceptance time is known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolAcceptanceTime {
    /// The exact original acceptance time.
    Known(PolicyTime),
    /// A legacy record did not preserve its original acceptance time.
    LegacyUnknown,
}

/// The trusted source classification for a mempool entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolOrigin {
    /// Submitted through a local operator-facing path.
    Local,
    /// Received from a network peer.
    Peer,
    /// Reaccepted because of a genuine chain reorganization.
    Reorg,
    /// A recovered legacy record did not preserve its origin.
    RecoveryUnknown,
}

/// Whether initial transaction relay was requested at admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayIntent {
    /// Initial relay was requested.
    Requested,
    /// Initial relay was not requested or cannot be proven.
    NotRequested,
}

/// Canonical privacy-sensitive metadata stored on every accepted entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MempoolEntryMetadata {
    /// Exact acceptance time or an explicit legacy-unknown classification.
    pub accepted_at: MempoolAcceptanceTime,
    /// Trusted admission origin.
    pub origin: MempoolOrigin,
    /// Original initial-relay intent.
    pub relay_intent: RelayIntent,
}

impl MempoolEntryMetadata {
    /// Creates metadata from every canonical source fact.
    pub const fn new(
        accepted_at: MempoolAcceptanceTime,
        origin: MempoolOrigin,
        relay_intent: RelayIntent,
    ) -> Self {
        Self {
            accepted_at,
            origin,
            relay_intent,
        }
    }

    /// Classifies a record whose legacy format preserved none of these facts.
    pub const fn legacy_unknown() -> Self {
        Self::new(
            MempoolAcceptanceTime::LegacyUnknown,
            MempoolOrigin::RecoveryUnknown,
            RelayIntent::NotRequested,
        )
    }

    /// Returns whether this entry may enter local initial-broadcast retry.
    pub const fn is_retry_eligible(self, is_current_member: bool) -> bool {
        is_current_member
            && matches!(self.origin, MempoolOrigin::Local)
            && matches!(self.relay_intent, RelayIntent::Requested)
    }
}

/// Immutable facts required for one admission attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissionContext {
    /// Metadata copied into canonical state only after successful first admission.
    pub metadata: MempoolEntryMetadata,
}

impl AdmissionContext {
    /// Creates an admission context from canonical metadata.
    pub const fn new(metadata: MempoolEntryMetadata) -> Self {
        Self { metadata }
    }

    /// Creates the fail-closed context used only by migration adapters.
    pub const fn legacy_unknown() -> Self {
        Self::new(MempoolEntryMetadata::legacy_unknown())
    }
}

/// Immutable occupancy and time facts for one pressure decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PressureDecisionContext {
    /// Time sampled by the calling shell.
    pub observed_at: PolicyTime,
    /// Accounted usage observed for this decision.
    pub usage: AccountedMempoolMemory,
    /// Configured accounted-memory capacity.
    pub capacity: MempoolCapacity,
}

impl PressureDecisionContext {
    /// Creates a pressure context from all required facts.
    pub const fn new(
        observed_at: PolicyTime,
        usage: AccountedMempoolMemory,
        capacity: MempoolCapacity,
    ) -> Self {
        Self {
            observed_at,
            usage,
            capacity,
        }
    }
}

/// Immutable time and height facts for a connected-block transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockLifecycleContext {
    /// Connection time sampled by the calling shell.
    pub connected_at: PolicyTime,
    /// Height of the connected block.
    pub height: u32,
}

impl BlockLifecycleContext {
    /// Creates a block lifecycle context from all required facts.
    pub const fn new(connected_at: PolicyTime, height: u32) -> Self {
        Self {
            connected_at,
            height,
        }
    }
}

/// Immutable time facts for a chain-reorganization transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReorgLifecycleContext {
    /// Reorganization time sampled by the calling shell.
    pub occurred_at: PolicyTime,
}

impl ReorgLifecycleContext {
    /// Creates a reorganization context from the explicit event time.
    pub const fn new(occurred_at: PolicyTime) -> Self {
        Self { occurred_at }
    }
}
