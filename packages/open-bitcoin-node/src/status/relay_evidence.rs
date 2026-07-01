// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared sanitized relay and mempool evidence status contracts.

use serde::{Deserialize, Serialize};

/// Default reason when mempool admission evidence has not been projected.
pub const MEMPOOL_ADMISSION_EVIDENCE_UNAVAILABLE_REASON: &str =
    "mempool admission evidence unavailable";

/// Default reason when local submission relay evidence has not been projected.
pub const LOCAL_SUBMISSION_RELAY_EVIDENCE_DEFERRED_REASON: &str =
    "local submission relay evidence not projected";

/// Default reason when relay fanout evidence has not been projected.
pub const RELAY_FANOUT_EVIDENCE_DEFERRED_REASON: &str = "relay fanout evidence not projected";

/// Default reason when relay serving evidence has not been projected.
pub const RELAY_SERVING_EVIDENCE_DEFERRED_REASON: &str = "relay serving evidence not projected";

/// Default reason when rebroadcast evidence has not been projected.
pub const REBROADCAST_EVIDENCE_DEFERRED_REASON: &str = "rebroadcast relay evidence not projected";

/// Stable reason for Open Bitcoin's non-promissory public relay boundary.
pub const PUBLIC_RELAY_INTENTIONALLY_DIFFERENT_REASON: &str =
    "public relay readiness is intentionally not claimed";

/// Explicit state wrapper for relay evidence fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state", content = "value")]
pub enum RelayEvidenceField<T> {
    Implemented(T),
    Unavailable { reason: String },
    Deferred { reason: String },
    IntentionallyDifferent { reason: String },
}

impl<T> RelayEvidenceField<T> {
    pub const fn implemented(value: T) -> Self {
        Self::Implemented(value)
    }

    pub fn unavailable(reason: impl Into<String>) -> Self {
        Self::Unavailable {
            reason: reason.into(),
        }
    }

    pub fn deferred(reason: impl Into<String>) -> Self {
        Self::Deferred {
            reason: reason.into(),
        }
    }

    pub fn intentionally_different(reason: impl Into<String>) -> Self {
        Self::IntentionallyDifferent {
            reason: reason.into(),
        }
    }
}

/// Fixed relay and mempool outcome counters safe for every operator surface.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayEvidenceCounters {
    pub accepted_count: u64,
    pub rejected_count: u64,
    pub orphaned_count: u64,
    pub requested_count: u64,
    pub served_count: u64,
    pub announced_count: u64,
    pub suppressed_count: u64,
    pub evicted_count: u64,
    pub expired_count: u64,
    pub rebroadcast_deferred_count: u64,
}

/// Low-cardinality implemented capability labels for relay evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelayEvidenceCapability {
    MempoolAdmission,
    LocalSubmissionRelay,
    RelayFanout,
    RelayServing,
    Rebroadcast,
    PublicRelayReadiness,
}

/// Implemented evidence value for a relay or mempool capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayCapabilityEvidence {
    pub capability: RelayEvidenceCapability,
}

impl RelayCapabilityEvidence {
    pub const fn new(capability: RelayEvidenceCapability) -> Self {
        Self { capability }
    }
}

/// Shared status contract for sanitized relay and mempool evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayEvidenceStatus {
    #[serde(default = "outcome_counters_default")]
    pub outcome_counters: RelayEvidenceField<RelayEvidenceCounters>,
    #[serde(default = "mempool_admission_unavailable")]
    pub mempool_admission: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "local_submission_deferred")]
    pub local_submission: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "fanout_deferred")]
    pub fanout: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "serving_deferred")]
    pub serving: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "rebroadcast_deferred")]
    pub rebroadcast: RelayEvidenceField<RelayCapabilityEvidence>,
    #[serde(default = "public_relay_intentionally_different")]
    pub public_relay: RelayEvidenceField<RelayCapabilityEvidence>,
}

impl RelayEvidenceStatus {
    pub fn default_unavailable() -> Self {
        Self {
            outcome_counters: outcome_counters_default(),
            mempool_admission: mempool_admission_unavailable(),
            local_submission: local_submission_deferred(),
            fanout: fanout_deferred(),
            serving: serving_deferred(),
            rebroadcast: rebroadcast_deferred(),
            public_relay: public_relay_intentionally_different(),
        }
    }

    pub fn with_counters(counters: RelayEvidenceCounters) -> Self {
        Self {
            outcome_counters: RelayEvidenceField::implemented(counters),
            ..Self::default_unavailable()
        }
    }
}

impl Default for RelayEvidenceStatus {
    fn default() -> Self {
        Self::default_unavailable()
    }
}

fn outcome_counters_default() -> RelayEvidenceField<RelayEvidenceCounters> {
    RelayEvidenceField::implemented(RelayEvidenceCounters::default())
}

fn mempool_admission_unavailable() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::unavailable(MEMPOOL_ADMISSION_EVIDENCE_UNAVAILABLE_REASON)
}

fn local_submission_deferred() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::deferred(LOCAL_SUBMISSION_RELAY_EVIDENCE_DEFERRED_REASON)
}

fn fanout_deferred() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::deferred(RELAY_FANOUT_EVIDENCE_DEFERRED_REASON)
}

fn serving_deferred() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::deferred(RELAY_SERVING_EVIDENCE_DEFERRED_REASON)
}

fn rebroadcast_deferred() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::deferred(REBROADCAST_EVIDENCE_DEFERRED_REASON)
}

fn public_relay_intentionally_different() -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::intentionally_different(PUBLIC_RELAY_INTENTIONALLY_DIFFERENT_REASON)
}
